use std::collections::HashMap;

use agent::{generate_vehicle, register_user, upload_firmware};
use candid::{Decode, Encode, Principal};
use ic_agent::{Agent, Identity};
use vts::{Error, VTSResult};

use crate::agent::init_agent;

mod agent;

#[tokio::test]
async fn test_create_agreement() {
    let (agent, canister_id) = init_agent().await;
    let (_, identity) = generate_vehicle();

    let agreement_id = create_agreement(
        &agent,
        canister_id,
        "test",
        Principal::anonymous(),
        "10",
        identity.public_key().unwrap(),
    )
    .await
    .unwrap();
    assert_eq!(1, agreement_id, "first agreement ID should be equal to one");
}

#[tokio::test]
async fn test_sign_agreement() {
    let (agent, canister_id) = init_agent().await;
    let (_, identity) = generate_vehicle();

    let agreement_id = create_agreement(
        &agent,
        canister_id,
        "test",
        agent.get_principal().unwrap(),
        "10",
        identity.public_key().unwrap(),
    )
    .await
    .unwrap();
    sign_agreement(&agent, canister_id, &agreement_id).await.unwrap()
}

#[tokio::test]
async fn test_sign_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;
    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.
    let result = sign_agreement(&agent, canister_id, &nonexistent_agreement_id).await.unwrap_err();
    assert_eq!(Error::NotFound, result);
}

#[tokio::test]
async fn test_sign_agreement_twice() {
    let (agent, canister_id) = init_agent().await;
    let (_, identity) = generate_vehicle();

    let agreement_id = create_agreement(
        &agent,
        canister_id,
        "test",
        agent.get_principal().unwrap(),
        "100",
        identity.public_key().unwrap(),
    )
    .await
    .unwrap();

    let result_first = sign_agreement(&agent, canister_id, &agreement_id).await;
    assert!(result_first.is_ok(), "should successfully sign the agreement the first time");

    let result_second = sign_agreement(&agent, canister_id, &agreement_id).await.unwrap_err();
    assert_eq!(Error::AlreadyExists, result_second);
}

#[tokio::test]
async fn test_create_duplicate_agreements() {
    let (agent, canister_id) = init_agent().await;
    let (_, identity) = generate_vehicle();

    let agreement_id_1 = create_agreement(
        &agent,
        canister_id,
        "test_1",
        agent.get_principal().unwrap(),
        "10",
        identity.public_key().unwrap(),
    )
    .await
    .unwrap();

    let agreement_id_2 = create_agreement(
        &agent,
        canister_id,
        "test_2",
        agent.get_principal().unwrap(),
        "10",
        identity.public_key().unwrap(),
    )
    .await
    .unwrap();

    assert_ne!(agreement_id_1, agreement_id_2, "agreement IDs should be different");
}

#[tokio::test]
async fn test_link_vehicle_to_agreement_success() {
    let (agent, canister_id) = init_agent().await;
    let (_, identity) = generate_vehicle();
    let vehicle = identity.sender().unwrap();
    let public_key = identity.public_key().unwrap();

    let agreement_id =
        create_agreement(&agent, canister_id, "test", agent.get_principal().unwrap(), "10", public_key)
            .await
            .unwrap();

    let result = link_vehicle(&agent, canister_id, &agreement_id, &vehicle).await;
    assert!(result.is_ok(), "should successfully link the vehicle to the agreement");
}

#[tokio::test]
async fn test_link_vehicle_to_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;
    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.
    let vehicle_public_key = Principal::anonymous();
    let result =
        link_vehicle(&agent, canister_id, &nonexistent_agreement_id, &vehicle_public_key).await.unwrap_err();
    assert_eq!(Error::NotFound, result);
}

#[tokio::test]
async fn test_get_vehicles_by_agreement() {
    let (agent, canister_id) = init_agent().await;
    let (_, identity) = generate_vehicle();
    let vehicle = identity.sender().unwrap();
    let public_key = identity.public_key().unwrap();

    let agreement_id =
        create_agreement(&agent, canister_id, "test", agent.get_principal().unwrap(), "10", public_key)
            .await
            .unwrap();

    link_vehicle(&agent, canister_id, &agreement_id, &vehicle).await.unwrap();

    let vehicles = get_vehicles_by_agreement(&agent, canister_id, &agreement_id).await.unwrap();
    assert_eq!(vehicles.len(), 1, "should return one vehicle");
    assert_eq!(vehicles.get(&vehicle).unwrap(), &(), "should return the linked vehicle");
}

#[tokio::test]
async fn test_get_vehicles_by_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;
    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.
    let result = get_vehicles_by_agreement(&agent, canister_id, &nonexistent_agreement_id).await.unwrap_err();
    assert_eq!(Error::NotFound, result);
}

async fn create_agreement(
    agent: &Agent,
    canister_id: Principal,
    name: &str,
    vh_customer: Principal,
    gas_price: &str,
    public_key: Vec<u8>,
) -> VTSResult<u128> {
    register_user(agent, canister_id, agent.get_principal().unwrap()).await;
    register_user(agent, canister_id, vh_customer).await;
    upload_firmware(agent, canister_id, vh_customer, public_key).await.unwrap();

    let response = agent
        .update(&canister_id, "create_agreement")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&name.to_string(), &vh_customer, &gas_price.to_string()).unwrap())
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
