use agent::{generate_vehicle, upload_firmware};
use candid::Encode;
use ic_agent::Identity;
use k256::ecdsa::{signature::SignerMut, Signature};

use crate::agent::{init_agent, register_user};

mod agent;

#[tokio::test]
async fn test_telemetry() {
    let (agent, canister_id) = init_agent().await;
    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    let (mut signing_key, identity) = generate_vehicle();
    let vehicle = identity.sender().unwrap();
    let public_key = identity.public_key().unwrap();

    upload_firmware(&agent, canister_id, agent.get_principal().unwrap(), public_key).await.unwrap();

    let data: Vec<u8> = vec![0, 1, 2];
    let signature: Signature = signing_key.sign(&data);
    let signature = signature.to_vec();
    agent
        .update(&canister_id, "store_telemetry")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&vehicle, &data, &signature).unwrap())
        .call_and_wait()
        .await
        .unwrap();
}
