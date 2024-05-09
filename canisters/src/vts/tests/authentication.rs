use candid::{Decode, Encode, Principal};
use ic_agent::Agent;
use vts::{Error, VTSResult};

use crate::agent::init_agent;

mod agent;

#[tokio::test]
async fn test_add_allowed_key() {
    let (agent, canister_id) = init_agent().await;

    let new_key = Principal::anonymous();

    let result1 = add_admin(&agent, canister_id, new_key).await;
    assert!(result1.is_ok(), "Should add new admin");

    let result2 = add_allowed_key(&agent, canister_id, &new_key).await;
    assert_eq!(result2.unwrap_err(), Error::AlreadyExists);
}

#[tokio::test]
async fn test_remove_allowed_key() {
    let (agent, canister_id) = init_agent().await;

    let new_key = Principal::anonymous();

    let result1 = add_admin(&agent, canister_id, new_key).await;
    assert!(result1.is_ok(), "Should add new admin");

    let result2 = add_allowed_key(&agent, canister_id, &new_key).await;
    assert_eq!(result2.unwrap_err(), Error::AlreadyExists);

    let result3 = remove_allowed_key(&agent, canister_id, &new_key).await;
    assert!(result3.is_ok(), "Should remove existing key");
}

#[tokio::test]
async fn test_add_admin() {
    let (agent, canister_id) = init_agent().await;

    let new_key = agent.get_principal().unwrap();

    let result = add_admin(&agent, canister_id, new_key).await;
    assert!(result.is_ok(), "Should add new admin");
}

#[tokio::test]
async fn test_delete_admin() {
    let (agent, canister_id) = init_agent().await;

    let new_admin = Principal::anonymous();

    let result1 = add_admin(&agent, canister_id, new_admin).await;
    assert!(result1.is_ok(), "Should add new admin");

    let result2 = delete_admin(&agent, canister_id, &new_admin).await;
    assert!(result2.is_ok(), "Should delete existing admin");
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

async fn remove_allowed_key(agent: &Agent, canister_id: Principal, key: &Principal) -> VTSResult<()> {
    let response = agent
        .update(&canister_id, "remove_allowed_key")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&key).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(response.as_slice(), VTSResult<()>).unwrap()
}

async fn add_allowed_key(agent: &Agent, canister_id: Principal, key: &Principal) -> VTSResult<()> {
    let response = agent
        .update(&canister_id, "add_allowed_key")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&key).unwrap())
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
