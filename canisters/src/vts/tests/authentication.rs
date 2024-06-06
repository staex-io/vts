use candid::{Decode, Encode, Principal};
use ic_agent::Agent;
use vts::VTSResult;

use crate::agent::init_agent;

mod agent;

#[tokio::test]
async fn test_add_admin() {
    let (agent, canister_id) = init_agent().await;

    let new_admin = agent.get_principal().unwrap();
    let result = add_admin(&agent, canister_id, new_admin).await;
    assert!(result.is_ok(), "should add new admin");
}

#[tokio::test]
async fn test_delete_admin() {
    let (agent, canister_id) = init_agent().await;

    let new_admin = Principal::anonymous();

    let result_1 = add_admin(&agent, canister_id, new_admin).await;
    assert!(result_1.is_ok(), "should add new admin");

    let result_2 = delete_admin(&agent, canister_id, &new_admin).await;
    assert!(result_2.is_ok(), "should delete existing admin");
}

async fn add_admin(agent: &Agent, canister_id: Principal, new_admin: Principal) -> VTSResult<()> {
    let response = agent
        .update(&canister_id, "add_admin")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&new_admin).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<()>).unwrap()
}

async fn delete_admin(agent: &Agent, canister_id: Principal, admin: &Principal) -> VTSResult<()> {
    let response = agent
        .update(&canister_id, "delete_admin")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&admin).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<()>).unwrap()
}
