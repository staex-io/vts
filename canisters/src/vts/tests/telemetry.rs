use agent::{generate_vehicle, upload_firmware};
use candid::Encode;
use ic_agent::Identity;
use k256::ecdsa::{signature::SignerMut, Signature};
use vts::TelemetryType;

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

    let telemetry = vts::TelemetryRequest {
        value: 88,
        t_type: TelemetryType::Gas,
    };
    let telemetry = bincode::encode_to_vec(telemetry, bincode::config::standard()).unwrap();
    let signature: Signature = signing_key.sign(&telemetry);
    let signature = signature.to_vec();
    agent
        .update(&canister_id, "store_telemetry")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&vehicle, &telemetry, &signature).unwrap())
        .call_and_wait()
        .await
        .unwrap();
}
