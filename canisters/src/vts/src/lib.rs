use candid::{CandidType, Deserialize};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

#[derive(Clone, Debug, Default, CandidType, Deserialize, PartialEq)]
pub enum Error {
    #[default]
    Internal,
    AlreadyExists,
}

pub type VTSResult<T> = Result<T, Error>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct UploadFirmwareRequest {
    pub principal: String,
    pub _firmware: Vec<u8>,
    pub _arch: String,
}

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Principal public key as a map key.
    static FIRMWARE_REQUESTS: RefCell<StableBTreeMap<String, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

#[ic_cdk::update]
fn request_firmware() -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("{} is requested firmware", caller);
    FIRMWARE_REQUESTS.with(|requests| {
        if requests.borrow_mut().contains_key(&caller.to_string()) {
            return Err(Error::AlreadyExists);
        }
        requests.borrow_mut().insert(caller.to_string(), ());
        Ok(())
    })?;
    Ok(())
}

#[ic_cdk::update]
fn upload_firmware(req: UploadFirmwareRequest) -> VTSResult<()> {
    // todo: this canister method should be executed only by our gateway.
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().remove(&req.principal));
    Ok(())
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();
