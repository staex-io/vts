use std::collections::HashMap;

use agent::{generate_vehicle, upload_firmware};
use candid::{Decode, Encode, Principal};
use ic_agent::Identity;
use k256::ecdsa::{signature::SignerMut, Signature};
use vts::{
    AccumulatedTelemetry, AccumulatedTelemetryMonthy, AccumulatedTelemetryYearly, TelemetryType, VTSResult,
};

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

    let vehicle_principal =
        Principal::from_text("zddkf-v7muw-3zj2q-kwijg-ulgjf-lpj32-t5qvx-5l3yb-rarsi-pq5w6-3ae").unwrap();

    agent
        .update(&canister_id, "fill_predefined_telemetry")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&Principal::anonymous(), &Principal::anonymous(), &vehicle_principal).unwrap())
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
        .with_arg(Encode!(&vehicle_principal).unwrap())
        .call()
        .await
        .unwrap();
    let aggregated_data = Decode!(response.as_slice(), VTSResult<AccumulatedTelemetry>).unwrap().unwrap();

    let expected_aggregated_data = HashMap::from_iter(vec![(
        TelemetryType::Gas,
        HashMap::from_iter(vec![(
            2024,
            AccumulatedTelemetryYearly {
                value: 1387,
                monthy: HashMap::from_iter(vec![(
                    6,
                    AccumulatedTelemetryMonthy {
                        value: 1387,
                        daily: HashMap::from_iter(vec![(15, 182), (16, 52), (17, 1042), (18, 111)]),
                    },
                )]),
            },
        )]),
    )]);

    assert_eq!(aggregated_data, expected_aggregated_data);
}
