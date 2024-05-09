use std::collections::HashMap;

use candid::{Decode, Encode, Principal};
use ic_agent::Agent;
use vts::{Error, VTSResult};

use crate::agent::init_agent;

mod agent;

#[tokio::test]
async fn test_create_agreement() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = Principal::anonymous();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();
    assert_eq!(1, agreement_id, "First agreement ID should be positive");
}

#[tokio::test]
async fn test_sign_agreement() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = agent.get_principal().unwrap();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();
    sign_agreement(&agent, canister_id, &agreement_id).await.unwrap()
}

#[tokio::test]
async fn test_sign_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;

    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.
    let result = sign_agreement(&agent, canister_id, &nonexistent_agreement_id).await.unwrap_err();
    assert_eq!(Error::NotFound, result);
}

#[tokio::test]
async fn test_sign_agreement_twice() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = agent.get_principal().unwrap();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    let result_first = sign_agreement(&agent, canister_id, &agreement_id).await;
    assert!(result_first.is_ok(), "Should successfully sign the agreement the first time");

    let result_second = sign_agreement(&agent, canister_id, &agreement_id).await.unwrap_err();
    assert_eq!(Error::AlreadyExists, result_second);
}

#[tokio::test]
async fn test_create_duplicate_agreements() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = agent.get_principal().unwrap();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id1 =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    let agreement_id2 =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    assert_ne!(agreement_id1, agreement_id2, "Agreement IDs should be different");
}

#[tokio::test]
async fn test_link_vehicle_to_agreement_success() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = agent.get_principal().unwrap();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();
    let vehicle_public_key = Principal::anonymous();

    let agreement_id =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    let result = link_vehicle(&agent, canister_id, &agreement_id, &vehicle_public_key).await;
    assert!(result.is_ok(), "Should successfully link the vehicle to the agreement");
}

#[tokio::test]
async fn test_link_vehicle_to_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;

    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.
    let vehicle_public_key = Principal::anonymous();
    let result =
        link_vehicle(&agent, canister_id, &nonexistent_agreement_id, &vehicle_public_key).await.unwrap_err();
    assert_eq!(Error::NotFound, result);
}

#[tokio::test]
async fn test_get_vehicles_by_agreement() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = agent.get_principal().unwrap();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();
    let vehicle = Principal::anonymous();

    let agreement_id =
        create_agreement(&agent, canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    let result = link_vehicle(&agent, canister_id, &agreement_id, &vehicle).await;
    assert!(result.is_ok(), "Should successfully link the vehicle to the agreement");

    let vehicles = get_vehicles_by_agreement(&agent, canister_id, &agreement_id).await.unwrap();
    assert_eq!(vehicles.len(), 1, "Should return one vehicle");
    assert_eq!(vehicles.get(&vehicle).unwrap(), &(), "Should return the linked vehicle");
}

#[tokio::test]
async fn test_get_vehicles_by_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;

    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.
    let result = get_vehicles_by_agreement(&agent, canister_id, &nonexistent_agreement_id).await.unwrap_err();
    assert_eq!(Error::NotFound, result);
}

async fn create_agreement(
    agent: &Agent,
    canister_id: Principal,
    name: &str,
    vh_customer: &Principal,
    daily_usage_fee: &str,
    gas_price: &str,
) -> VTSResult<u128> {
    let response = agent
        .update(&canister_id, "create_agreement")
        .with_effective_canister_id(canister_id)
        .with_arg(
            Encode!(&name.to_string(), vh_customer, &daily_usage_fee.to_string(), &gas_price.to_string())
                .unwrap(),
        )
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<u128>).unwrap()
}

async fn sign_agreement(agent: &Agent, canister_id: Principal, agreement_id: &u128) -> VTSResult<()> {
    let response = agent
        .update(&canister_id, "sign_agreement")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&agreement_id).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<()>).unwrap()
}

async fn link_vehicle(
    agent: &Agent,
    canister_id: Principal,
    agreement_id: &u128,
    vehicle: &Principal,
) -> VTSResult<()> {
    let response = agent
        .update(&canister_id, "link_vehicle")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&agreement_id, &vehicle).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<()>).unwrap()
}

async fn get_vehicles_by_agreement(
    agent: &Agent,
    canister_id: Principal,
    agreement_id: &u128,
) -> VTSResult<HashMap<Principal, ()>> {
    let response = agent
        .query(&canister_id, "get_vehicles_by_agreement")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&agreement_id).unwrap())
        .call()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<HashMap<Principal, ()>>).unwrap()
}
