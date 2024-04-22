use candid::{CandidType, Decode, Encode, Principal};
use ic_agent::Agent;
use vts::{Error, UploadFirmwareRequest, VTSResult};

use crate::agent::init_agent;

mod agent;

#[derive(CandidType)]
struct RequestFirmwareRequest {}

#[tokio::test]
async fn test_firmware() {
    let (agent, canister_id) = init_agent().await;

    // Request new firmware.
    request_firmware(&agent, canister_id).await.unwrap();

    // User has active firmware request.
    // Canister should return an error about it.
    let err = request_firmware(&agent, canister_id).await.unwrap_err();
    assert_eq!(Error::AlreadyExists, err);

    // Upload firmware deletes current active user request.
    upload_firmware(&agent, canister_id).await.unwrap();
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
        .with_arg(Encode!(&RequestFirmwareRequest {}).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    Decode!(res.as_slice(), VTSResult<()>).unwrap()
}

async fn upload_firmware(agent: &Agent, canister_id: Principal) -> VTSResult<()> {
    let res = agent
        .update(&canister_id, "upload_firmware")
        .with_effective_canister_id(canister_id)
        .with_arg(
            Encode!(&UploadFirmwareRequest {
                principal: agent.get_principal().unwrap().to_string(),
                _firmware: vec![0, 1, 2],
                _arch: "arm64".to_string(),
            })
            .unwrap(),
        )
        .call_and_wait()
        .await
        .unwrap();
    Decode!(res.as_slice(), VTSResult<()>).unwrap()
}
