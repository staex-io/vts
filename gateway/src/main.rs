use std::{fmt::Debug, time::Duration};

use candid::{Decode, Encode};
use ic_agent::{export::Principal, identity::Secp256k1Identity, Agent, Identity};
use log::{debug, error, info, trace, LevelFilter};
use serde::Deserialize;
use tokio::{
    select,
    sync::watch,
    time::{sleep, timeout},
};
use vts::VTSResult;

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
    tokio::spawn(async move { wait_for_firmware_requests(stop_r).await });
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

async fn wait_for_firmware_requests(mut stop_r: watch::Receiver<()>) {
    loop {
        select! {
            _ = stop_r.changed() => {
                trace!("received stop signal, exit tcp server loop");
                return;
            }
            _ = sleep(Duration::from_secs(1)) => {
                if let Err(e) = check_firmware_requests().await {
                    error!("failed to check firmware requests: {:?}", e)
                }
            }
        }
    }
}

async fn check_firmware_requests() -> Res<()> {
    let (agent, canister_id) = init_agent().await?;
    let vh_customer = match get_firmware_request(&agent, canister_id).await? {
        Some(principal) => principal,
        None => {
            debug!("now active firmware requests to build new firmware");
            return Ok(());
        }
    };
    debug!("new firmware request: {vh_customer}, building new firmware");
    let secret_key = k256::SecretKey::random(&mut rand::thread_rng());
    std::fs::write("../firmware/secret_key", secret_key.to_bytes())?;
    let output =
        std::process::Command::new("cargo").args(vec!["build"]).current_dir("../firmware").output()?;
    if !output.status.success() {
        return Err("build firmware error".into());
    }
    let firmware = std::fs::read("../target/debug/firmware")?;
    let vehicle = Secp256k1Identity::from_private_key(secret_key.clone());
    upload_firmware(&agent, canister_id, vh_customer, vehicle.sender()?, firmware).await?;
    debug!("successfully uploaded new firmware for {vh_customer}");
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
    vehicle: Principal,
    firmware: Vec<u8>,
) -> Res<()> {
    let res = agent
        .update(&canister_id, "upload_firmware")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&vh_customer, &vehicle, &std::env::consts::ARCH.to_string(), &firmware)?)
        .call_and_wait()
        .await?;
    Ok(Decode!(res.as_slice(), VTSResult<()>)?.map_err(|_| "failed to upload firmware".to_string())?)
}
