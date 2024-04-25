use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromStr;


#[derive(CandidType, Deserialize, Default, Debug, PartialEq)]
pub enum Error {
    #[default]
    Internal,
    AlreadyExists,
}

pub type VTSResult<T> = Result<T, Error>;

#[derive(CandidType, Deserialize, Debug)]
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

#[derive(Clone, Debug)]
pub struct AgreementConditions {
    pub daily_usage_fee: Decimal,
    pub gas_price: Decimal,
}

#[derive(Clone, Debug)]
pub struct Agreement {
    pub name: String,
    pub vh_provider: Principal,
    pub vh_customer: Principal,
    pub state: AgreementState,
    pub conditions: AgreementConditions,
    pub vehicles: Vec<u128>,
}

#[derive(Clone, CandidType, Deserialize, Debug, PartialEq)]
pub enum AgreementState {
    Unsigned,
    Signed,
}

#[derive(Default)]
pub struct CanisterState {
    pub next_agreement_id: u128,
}


#[derive(Default)]
pub struct AgreementStorage {
    agreements: HashMap<u128, Agreement>,
}

fn get_agreement_storage() -> RefCell<AgreementStorage> {
    thread_local! {
        static AGREEMENT_STORAGE: RefCell<AgreementStorage> = RefCell::new(AgreementStorage::default());
    }
    AGREEMENT_STORAGE.with(|storage| storage)
}



#[ic_cdk::update]
fn create_agreement(name: String, vh_customer: Principal, daily_usage_fee: String, gas_price: String) -> Result<u128, Error> {

    let daily_usage_fee_decimal = Decimal::from_str(&daily_usage_fee);
    let gas_price_decimal = Decimal::from_str(&gas_price);

    let mut agreement_storage = get_agreement_storage().borrow_mut();
    let mut next_agreement_id = agreement_storage.agreements.len() as u128;
    next_agreement_id += 1;

    let agreement = Agreement {
        name,
        vh_provider: ic_cdk::api::caller(),
        vh_customer,
        state: AgreementState::Unsigned,
        conditions: AgreementConditions {
            daily_usage_fee: daily_usage_fee_decimal.unwrap(),
            gas_price: gas_price_decimal.unwrap(),
        },
        vehicles: vec![],
    };

    let mut agreement_storage = get_agreement_storage().borrow_mut();
    agreement_storage.agreements.insert(next_agreement_id, agreement);

    Ok(next_agreement_id)
}