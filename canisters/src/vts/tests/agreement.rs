use candid::{CandidType, Encode, Decode, Principal};
use ic_agent::{Agent, AgentError};
use tokio;

use crate::agent::init_agent;

mod agent;

#[derive(CandidType)]
struct CreateAgreementRequest {
    name: String,
    vh_customer: Principal,
    daily_usage_fee: String,
    gas_price: String,
}

#[tokio::test]
async fn test_create_agreement() -> Result<(), Box<dyn std::error::Error>> {
    let (agent, canister_id) = init_agent().await;

    let name = "Test Agreement".to_string();
    let vh_customer = Principal::anonymous();
    let daily_usage_fee = "100".to_string();
    let gas_price = "10".to_string();

    let agreement_id_first = create_agreement(&agent, &canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price).await?;
    assert!(agreement_id_first > 0, "First agreement ID should be positive");

    let result = create_agreement(&agent, &canister_id, &name, &vh_customer, &daily_usage_fee, &gas_price).await;
    assert!(result.is_err(), "Should return error on duplicate agreement");

    Ok(())
}

async fn create_agreement(agent: &Agent, canister_id: &Principal, name: &str, vh_customer: &Principal, daily_usage_fee: &str, gas_price: &str) -> Result<u128, AgentError> {
    let request = CreateAgreementRequest {
        name: name.to_string(),
        vh_customer: *vh_customer,
        daily_usage_fee: daily_usage_fee.to_string(),
        gas_price: gas_price.to_string(),
    };

    let encoded_request = Encode!(&request)?;
    let response = agent.update(canister_id, "create_agreement")
        .with_effective_canister_id(*canister_id)
        .with_arg(encoded_request)
        .call_and_wait()
        .await?;

    let agreement_id: u128 = Decode!(response.as_slice(), u128)?;
    Ok(agreement_id)
}