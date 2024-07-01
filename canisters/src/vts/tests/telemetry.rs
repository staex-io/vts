use agent::{generate_vehicle, upload_firmware};
use candid::{Decode, Encode, Principal};
use ic_agent::{identity::Secp256k1Identity, Identity};
use k256::ecdsa::{signature::SignerMut, Signature};
use vts::{AccumulatedTelemetry, TelemetryType, VTSResult};

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

    let telemetry = vts::StoreTelemetryRequest {
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

#[tokio::test]
async fn test_get_aggregated_data() {
    let (agent, canister_id) = init_agent().await;
    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    let mut rng = rand::thread_rng();
    let vehicle_secret_key = k256::SecretKey::random(&mut rng);
    let vehicle_identity = Secp256k1Identity::from_private_key(vehicle_secret_key);
    let vehicle_public_key = hex::encode(vehicle_identity.public_key().unwrap());
    let vehicle = vehicle_identity.sender().unwrap();

    agent
        .update(&canister_id, "fill_predefined_telemetry")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&Principal::anonymous(), &Principal::anonymous(), &vehicle_public_key).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    agent
        .update(&canister_id, "accumulate_telemetry_data")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&()).unwrap())
        .call_and_wait()
        .await
        .unwrap();

    let response: Vec<u8> = agent
        .query(&canister_id, "get_aggregated_data")
        .with_arg(Encode!(&vehicle).unwrap())
        .call()
        .await
        .unwrap();
    let aggregated_data = Decode!(response.as_slice(), VTSResult<AccumulatedTelemetry>).unwrap().unwrap();

    aggregated_data.get(&TelemetryType::Gas).unwrap().get(&2023).unwrap();
    aggregated_data.get(&TelemetryType::Gas).unwrap().get(&2024).unwrap();

    assert_eq!(aggregated_data.get(&TelemetryType::Gas).unwrap().get(&2023).unwrap().value, 361);
    assert_eq!(aggregated_data.get(&TelemetryType::Gas).unwrap().get(&2024).unwrap().value, 640);
    assert_eq!(
        aggregated_data.get(&TelemetryType::Gas).unwrap().get(&2024).unwrap().monthly.get(&6).unwrap().value,
        294
    );
    assert_eq!(
        aggregated_data.get(&TelemetryType::Gas).unwrap().get(&2024).unwrap().monthly.get(&7).unwrap().value,
        346
    );
}
