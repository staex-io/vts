use candid::{Decode, Encode, Principal};
use ic_agent::Agent;
use vts::{Error, VTSResult};

use crate::agent::{init_agent, register_user, upload_firmware};

mod agent;

#[tokio::test]
async fn test_firmware() {
    let (agent, canister_id) = init_agent().await;

    register_user(&agent, canister_id, agent.get_principal().unwrap()).await;

    let err = get_firmware_requests_by_user(&agent, canister_id).await.unwrap_err();
    assert_eq!(Error::NotFound, err);

    // Request new firmware.
    request_firmware(&agent, canister_id).await.unwrap();

    // User has active firmware request.
    // Canister should return an error about it.
    let err = request_firmware(&agent, canister_id).await.unwrap_err();
    assert_eq!(Error::AlreadyExists, err);

    // Check that now user has active firmware request.
    get_firmware_requests_by_user(&agent, canister_id).await.unwrap();

    // Upload firmware deletes current active user request.
    upload_firmware(&agent, canister_id, agent.get_principal().unwrap(), Principal::anonymous())
        .await
        .unwrap();
    // That's why we can again make new firmware request.
    // Request new firmware.
    request_firmware(&agent, canister_id).await.unwrap();
    // And again user should receive that it has active request.
    let err = request_firmware(&agent, canister_id).await.unwrap_err();
    assert_eq!(Error::AlreadyExists, err);
}

async fn request_firmware(agent: &Agent, canister_id: Principal) -> VTSResult<()> {
    let res = agent
        .update(&canister_id, "request_firmware")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&()).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(res.as_slice(), VTSResult<()>).unwrap()
}

async fn get_firmware_requests_by_user(agent: &Agent, canister_id: Principal) -> VTSResult<()> {
    let res = agent
        .update(&canister_id, "get_firmware_requests_by_user")
        .with_effective_canister_id(canister_id)
        .with_arg(Encode!(&()).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(res.as_slice(), VTSResult<()>).unwrap()
}
