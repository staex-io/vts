use candid::{Decode, Encode, Principal};
use ic_agent::Agent;
use vts::VTSResult;

use crate::agent::init_agent;

mod agent;

#[tokio::test]
async fn test_create_agreement() {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = Principal::anonymous();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id_first =
        create_agreement(&agent, &canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();
    assert!(agreement_id_first > 0, "First agreement ID should be positive");
}

#[tokio::test]
async fn test_sign_agreement() {
    let (agent, canister_id) = init_agent().await;
    let vh_customer = agent.get_principal().unwrap();
    let name = "Test Agreement".to_string();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id =
        create_agreement(&agent, &canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    let result = sign_agreement(&agent, &canister_id, &agreement_id).await;
    assert!(result.is_ok(), "Should successfully sign the agreement");
}

#[tokio::test]
async fn test_sign_nonexistent_agreement() {
    let (agent, canister_id) = init_agent().await;
    let nonexistent_agreement_id = 999999; // An ID that doesn't exist.

    let result = sign_agreement(&agent, &canister_id, &nonexistent_agreement_id).await;
    assert!(result.is_err(), "Should fail to sign a nonexistent agreement");
}

#[tokio::test]
async fn test_sign_agreement_twice() {
    let (agent, canister_id) = init_agent().await;
    let vh_customer = agent.get_principal().unwrap();
    let name = "Test Agreement".to_string();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id =
        create_agreement(&agent, &canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price)
            .await
            .unwrap();

    let result_first = sign_agreement(&agent, &canister_id, &agreement_id).await;
    assert!(result_first.is_ok(), "Should successfully sign the agreement the first time");

    let result_second = sign_agreement(&agent, &canister_id, &agreement_id).await;
    assert!(result_second.is_err(), "Should fail to sign the agreement a second time");
}

async fn create_agreement(
    agent: &Agent,
    canister_id: &Principal,
    name: &str,
    vh_customer: &Principal,
    daily_usage_fee: &str,
    gas_price: &str,
) -> VTSResult<u128> {
    let response = agent
        .update(canister_id, "create_agreement")
        .with_effective_canister_id(*canister_id)
        .with_arg(
            Encode!(
                &name.to_string(),
                vh_customer,
                &daily_usage_fee.to_string(),
                &gas_price.to_string()
            )
            .unwrap(),
        )
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<u128>).unwrap()
}

async fn sign_agreement(
    agent: &Agent,
    canister_id: &Principal,
    agreement_id: &u128,
) -> VTSResult<()> {
    let response = agent
        .update(canister_id, "sign_agreement")
        .with_effective_canister_id(*canister_id)
        .with_arg(Encode!(&agreement_id).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<()>).unwrap()
}
