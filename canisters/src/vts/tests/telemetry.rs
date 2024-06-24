use agent::{generate_vehicle, upload_firmware};
use candid::{Decode, Encode, Principal};
use ic_agent::Identity;
use k256::ecdsa::{signature::SignerMut, Signature};
use std::collections::HashMap;
use vts::{AggregatedData, TelemetryType};

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

#[tokio::test]
async fn test_get_aggregated_data() {
    let (agent, canister_id) = init_agent().await;
    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    agent.update(&canister_id, "fill_predefined_telemetry").call_and_wait().await.unwrap();

    let vehicle_principal =
        Principal::from_text("zddkf-v7muw-3zj2q-kwijg-ulgjf-lpj32-t5qvx-5l3yb-rarsi-pq5w6-3ae").unwrap();

    agent.update(&canister_id, "accumulate_telemetry_data_now").call_and_wait().await.unwrap();

    let response: Vec<u8> = agent
        .query(&canister_id, "get_aggregated_data")
        .with_arg(Encode!(&vehicle_principal).unwrap())
        .call()
        .await
        .unwrap();
    let aggregated_data: HashMap<TelemetryType, AggregatedData> =
        Decode!(&response, HashMap<TelemetryType, AggregatedData>).unwrap();

    let expected_aggregated_data = AggregatedData {
        daily: vec![
            ("15".to_string(), 182),
            ("16".to_string(), 52),
            ("17".to_string(), 1042),
            ("18".to_string(), 111),
        ]
        .into_iter()
        .collect(),
        monthly: vec![("6".to_string(), 1387)].into_iter().collect(),
        yearly: vec![("2024".to_string(), 1387)].into_iter().collect(),
    };

    assert_eq!(aggregated_data.get(&TelemetryType::Gas).unwrap(), &expected_aggregated_data);
}
