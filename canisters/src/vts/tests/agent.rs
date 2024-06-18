use candid::{Decode, Encode, Principal};
use ic_agent::{identity::Secp256k1Identity, Agent, Identity};
use k256::ecdsa::SigningKey;
use serde::Deserialize;
use vts::VTSResult;

#[derive(Deserialize)]
struct CanisterIds {
    vts: CanisterId,
}

#[derive(Deserialize)]
struct CanisterId {
    local: String,
}

pub async fn init_agent() -> (Agent, Principal) {
    let secret_key = k256::SecretKey::random(&mut rand::thread_rng());
    let identity = Secp256k1Identity::from_private_key(secret_key);
    eprintln!("\nAgent sender is: {:?}", identity.sender().unwrap().to_string());
    let agent = Agent::builder().with_url("http://127.0.0.1:7777").with_identity(identity).build().unwrap();
    agent.fetch_root_key().await.unwrap();
    let canisters_ids: CanisterIds =
        serde_json::from_str(&std::fs::read_to_string("../../.dfx/local/canister_ids.json").unwrap())
            .unwrap();
    let canister_id = Principal::from_text(canisters_ids.vts.local).unwrap();

    // Clean state to not restart dfx node.
    agent
        .update(&canister_id, "clean_state")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&()).unwrap())
        .call_and_wait()
        .await
        .unwrap();

    // Add itself as admin in canister.
    let res = agent
        .update(&canister_id, "add_admin")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&agent.get_principal().unwrap()).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(res.as_slice(), VTSResult<()>).unwrap().unwrap();

    (agent, canister_id)
}

#[allow(dead_code)]
pub async fn register_user(agent: &Agent, canister_id: Principal, user: Principal) {
    let res = agent
        .update(&canister_id, "register_user")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&user).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    let res = Decode!(res.as_slice(), VTSResult<()>).unwrap();
    if let Err(e) = res {
        match e {
            vts::Error::AlreadyExists => (),
            e => panic!("{:?}", e),
        }
    }
}

#[allow(dead_code)]
pub async fn upload_firmware(
    agent: &Agent,
    canister_id: Principal,
    vh_customer: Principal,
    public_key: Vec<u8>,
) -> VTSResult<()> {
    let firmware: Vec<u8> = vec![0, 1, 2];
    let res = agent
        .update(&canister_id, "upload_firmware")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&vh_customer, &public_key, &"arm64".to_string(), &firmware).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(res.as_slice(), VTSResult<()>).unwrap()
}

#[allow(dead_code)]
pub fn generate_vehicle() -> (SigningKey, Secp256k1Identity) {
    let signing_key = k256::ecdsa::SigningKey::random(&mut rand::thread_rng());
    let identity = Secp256k1Identity::from_private_key(signing_key.clone().into());
    (signing_key, identity)
}
