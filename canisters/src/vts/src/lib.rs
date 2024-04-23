use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use rust_decimal::Decimal;
use std::cell::RefCell;
use std::collections::HashMap;
use serde::Serialize;

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

#[derive(CandidType, Deserialize, Debug)]
pub struct Provider {
    agreements: Vec<u128>,
}

#[derive(CandidType, Deserialize, Debug)]
struct Customer {
    agreements: Vec<u128>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AgreementConditions {
    #[serde(with = "rust_decimal::serde::str")]
    daily_usage_fee: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    gas_price: Decimal,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Agreement {
    pub name: String,
    pub vh_provider: Principal,
    pub vh_customer: Principal,
    pub state: AgreementState,
    conditions: AgreementConditions,
    pub vehicles: Vec<u128>,
}

#[derive(Clone, CandidType, Deserialize, Debug)]
pub enum AgreementState {
    Unsigned,
    Signed,
}

#[derive(Clone, Default)]
pub struct CanisterState {
    pub vh_providers: HashMap<Principal, HashMap<u128, Agreement>>,
    pub vh_customers: HashMap<Principal, HashMap<u128, Agreement>>,
    pub agreements: HashMap<u128, Agreement>,
    pub vehicles: HashMap<u128, String>,
    pub next_agreement_id: u128,
}

#[ic_cdk_macros::update]
pub fn create_agreement(name: String, vh_provider: Principal, vh_customer: Principal, daily_usage_fee: Decimal, gas_price: Decimal) -> u128 {
    let mut state = CanisterState::default(); // Beispiel zur Verwaltung des Zustands, anpassen nach Bedarf
    let agreement_id = state.next_agreement_id;
    state.next_agreement_id += 1;

    let agreement = Agreement {
        name,
        vh_provider: vh_provider.clone(),
        vh_customer: vh_customer.clone(),
        state: AgreementState::Unsigned,
        conditions: AgreementConditions {
            daily_usage_fee: daily_usage_fee,
            gas_price: gas_price
        },
        vehicles: vec![],
    };

    state.agreements.insert(agreement_id, agreement.clone());
    state.vh_providers.entry(vh_provider).or_default().insert(agreement_id, agreement.clone());
    state.vh_customers.entry(vh_customer).or_default().insert(agreement_id, agreement);

    agreement_id
}
