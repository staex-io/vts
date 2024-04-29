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

    let result = 
    create_agreement(&agent, &canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price).await;
    assert!(result.is_err(), "Should return error on duplicate agreement");
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
