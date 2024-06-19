use std::{
    fmt::Debug,
    io::{Cursor, ErrorKind, Write},
    net::SocketAddr,
    time::Duration,
};

use candid::{Decode, Encode};
use gateway_tcp::{Request, Response};
use ic_agent::{export::Principal, identity::Secp256k1Identity, Agent, Identity};
use log::{debug, error, info, trace, LevelFilter};
use serde::Deserialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    select,
    sync::watch,
    time::{sleep, timeout},
};
use vts::{StoreTelemetryResponse, VTSResult};
use zip::write::SimpleFileOptions;

const FIRMWARE_PATH: &str = "../target/debug/firmware";

type Res<T> = Result<T, Error>;

pub(crate) struct Error(String);

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: ToString> From<T> for Error {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone)]
struct State {
    agent: Agent,
    canister_id: Principal,
}

#[derive(Deserialize)]
struct CanisterIds {
    vts: CanisterId,
}

#[derive(Deserialize)]
struct CanisterId {
    local: String,
}

#[tokio::main]
async fn main() -> Res<()> {
    env_logger::builder()
        .filter(None, LevelFilter::Off)
        .filter_module("gateway", LevelFilter::Trace)
        .init();
    let (stop_s, stop_r) = watch::channel(());
    let (agent, canister_id) = init_agent().await?;
    let state = State { agent, canister_id };
    let state_ = state.clone();
    let stop_r_ = stop_r.clone();
    tokio::spawn(async move { wait_for_firmware_requests(state_, stop_r_).await });
    tokio::spawn(async move { start_tcp_server(state, stop_r).await });
    info!("gateway started; waiting for termination signal");
    tokio::signal::ctrl_c().await?;
    debug!("received termination signal");
    stop_s.send(())?;
    match timeout(Duration::from_secs(10), stop_s.closed()).await {
        Ok(_) => info!("everything was stopped successfully"),
        Err(e) => {
            error!("failed to stop everything: {}", e)
        }
    }
    Ok(())
}

async fn wait_for_firmware_requests(state: State, mut stop_r: watch::Receiver<()>) {
    loop {
        select! {
            _ = stop_r.changed() => {
                trace!("received stop signal, exit tcp server loop");
                return;
            }
            _ = sleep(Duration::from_secs(1)) => {
                let state_ = state.clone();
                if let Err(e) = check_firmware_requests(state_).await {
                    error!("failed to check firmware requests: {:?}", e)
                }
            }
        }
    }
}

async fn check_firmware_requests(state: State) -> Res<()> {
    let vh_customer = match get_firmware_request(&state.agent, state.canister_id).await? {
        Some(principal) => principal,
        None => {
            debug!("no active firmware requests to build new firmware");
            return Ok(());
        }
    };
    debug!("new firmware request: {vh_customer}, building new firmware");

    // Generate new secret key for the firmware.
    let secret_key = k256::SecretKey::random(&mut rand::thread_rng());
    std::fs::write("../firmware/secret_key", secret_key.to_bytes())?;

    // Build firmware with newly generated secret key.
    let output =
        std::process::Command::new("cargo").args(vec!["build"]).current_dir("../firmware").output()?;
    if !output.status.success() {
        return Err("build firmware error".into());
    }

    // Sign firmware for macOS.
    sign_firmware(FIRMWARE_PATH)?;

    // Load firmware.
    let firmware = std::fs::read(FIRMWARE_PATH)?;

    // Compress firmware.
    let vehicle = Secp256k1Identity::from_private_key(secret_key.clone());
    let firmware = compress_firmware(vehicle.sender()?, firmware)?;

    upload_firmware(
        &state.agent,
        state.canister_id,
        vh_customer,
        vehicle.public_key().ok_or("identity public key is empty".to_string())?,
        firmware,
    )
    .await?;
    debug!("successfully uploaded new firmware for {vh_customer}: {}", vehicle.sender()?);
    Ok(())
}

async fn init_agent() -> Res<(Agent, Principal)> {
    let identity = Secp256k1Identity::from_pem_file("../canisters/identity.pem")?;
    let agent = Agent::builder().with_url("http://127.0.0.1:7777").with_identity(identity).build()?;
    agent.fetch_root_key().await?;
    let canisters_ids: CanisterIds =
        serde_json::from_str(&std::fs::read_to_string("../canisters/.dfx/local/canister_ids.json")?)?;
    let canister_id = Principal::from_text(canisters_ids.vts.local)?;
    Ok((agent, canister_id))
}

async fn get_firmware_request(agent: &Agent, canister_id: Principal) -> Res<Option<Principal>> {
    let res = agent
        .update(&canister_id, "get_firmware_requests")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&())?)
        .call_and_wait()
        .await?;
    let res = Decode!(res.as_slice(), VTSResult<Principal>)?;
    match res {
        Ok(principal) => Ok(Some(principal)),
        Err(vts::Error::NotFound) => Ok(None),
        Err(_) => Err("failed to decode response".to_string().into()),
    }
}

async fn upload_firmware(
    agent: &Agent,
    canister_id: Principal,
    vh_customer: Principal,
    public_key: Vec<u8>,
    firmware: Vec<u8>,
) -> Res<()> {
    let res = agent
        .update(&canister_id, "upload_firmware")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&vh_customer, &public_key, &std::env::consts::ARCH.to_string(), &firmware)?)
        .call_and_wait()
        .await?;
    Ok(Decode!(res.as_slice(), VTSResult<()>)?.map_err(|_| "failed to upload firmware".to_string())?)
}

fn compress_firmware(vehicle: Principal, firmware: Vec<u8>) -> Res<Vec<u8>> {
    let mut buf = Cursor::new(vec![]);
    let mut zip = zip::ZipWriter::new(&mut buf);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    zip.start_file(format!("{vehicle}.firmware.{}", std::env::consts::ARCH), options)?;
    zip.write_all(&firmware)?;
    zip.finish()?;
    Ok(buf.into_inner())
}

async fn start_tcp_server(state: State, mut stop_r: watch::Receiver<()>) -> Res<()> {
    let tcp_server_address = "127.0.0.1:3322";
    info!("starting tcp server on {}", tcp_server_address);
    let listener = TcpListener::bind(tcp_server_address).await?;
    loop {
        select! {
            _ = stop_r.changed() => {
                trace!("received stop signal, exit tcp server loop");
                return Ok(());
            }
            connection = listener.accept() => {
                if let Ok(connection) = connection {
                    let state_ = state.clone();
                    tokio::spawn(async move {
                        if let Err(e) = process_connection(connection, state_).await {
                            error!("failed to process connection: {}", e.0)
                        }
                    });
                }
            }
        }
    }
}

async fn process_connection(connection: (TcpStream, SocketAddr), state: State) -> Res<()> {
    let (mut stream, addr) = connection;
    trace!("new tcp client connected: {addr}");

    loop {
        let mut buf: Vec<u8> = vec![0; 128];
        let n = stream.read(&mut buf).await;
        let buf = match n {
            Ok(0) => {
                trace!("rpc client disconnected: {}", addr);
                return Ok(());
            }
            Ok(n) => {
                buf.truncate(n);
                // Remove new line if exists.
                if buf.last() == Some(&10) {
                    buf.pop();
                }
                buf
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // This error means that there are no data in socket buffer but it is not closed.
                return Ok(());
            }
            Err(e) => return Err(format!("failed to read from connection: {:?}: {:?}", addr, e).into()),
        };

        let req: Request = bincode::decode_from_slice(&buf, bincode::config::standard())?.0;
        let res = handle_rpc_request(&req, &state).await?;
        let mut buf: Vec<u8> = bincode::encode_to_vec(res, bincode::config::standard())?;
        write(&mut stream, &mut buf).await?;
    }
}

async fn write(stream: &mut TcpStream, buf: &mut Vec<u8>) -> Res<()> {
    // Add new line if not exists.
    if buf.last() != Some(&10) {
        buf.push(10);
    }
    Ok(stream.write_all(buf).await?)
}

async fn handle_rpc_request(req: &Request, state: &State) -> Res<Response> {
    match req {
        Request::StoreTelemetry(telemetry) => {
            let principal = Principal::from_slice(&telemetry.principal);
            let res = state
                .agent
                .update(&state.canister_id, "store_telemetry")
                .with_effective_canister_id(state.canister_id)
                .with_arg(Encode!(&principal, &telemetry.telemetry, &telemetry.signature)?)
                .call_and_wait()
                .await?;
            let res = Decode!(res.as_slice(), StoreTelemetryResponse)?;
            let res = match res {
                StoreTelemetryResponse::On => Response::TurnOn,
                StoreTelemetryResponse::Off => Response::TurnOff,
            };
            Ok(res)
        }
    }
}

fn sign_firmware(filepath: &str) -> Res<()> {
    if std::env::consts::OS != "macos" {
        return Ok(());
    }
    let output = std::process::Command::new("codesign")
        .args(vec!["-f", "-s", "vts-signer", filepath, "--deep"])
        .output()?;
    if !output.status.success() {
        return Err("build firmware error".into());
    }
    Ok(())
}
