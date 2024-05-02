use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(CandidType, Deserialize, Default, Debug, PartialEq)]
pub enum Error {
    #[default]
    Internal,
    AlreadyExists,
    NotFound,
    InvalidSigner,
}

pub type VTSResult<T> = Result<T, Error>;

#[derive(CandidType, Deserialize, Debug)]
pub struct Agreement {
    pub name: String,
    pub vh_provider: Principal,
    pub vh_customer: Principal,
    pub state: AgreementState,
    pub conditions: AgreementConditions,
    pub vehicles: Vec<Principal>,
}

impl Storable for Agreement {
    const BOUND: Bound = Bound::Bounded {
        max_size: u32::MAX,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub enum AgreementState {
    Unsigned,
    Signed,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct AgreementConditions {
    pub daily_usage_fee: String,
    pub gas_price: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Vehicle {}

impl Storable for Vehicle {
    const BOUND: Bound = Bound::Bounded {
        max_size: u32::MAX,
        is_fixed_size: false,
    };

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Principal public key as a map key.
    static FIRMWARE_REQUESTS: RefCell<StableBTreeMap<String, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static AGREEMENTS: RefCell<StableBTreeMap<u128, Agreement, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    static AGREEMENT_ID_COUNTER: RefCell<u128> = const { RefCell::new(0) };

    static VEHICLES: RefCell<StableBTreeMap<Principal, Vehicle, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
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

#[derive(CandidType, Deserialize, Debug)]
pub struct UploadFirmwareRequest {
    pub principal: String,
    pub _firmware: Vec<u8>,
    pub _arch: String,
}

#[ic_cdk::update]
fn upload_firmware(req: UploadFirmwareRequest) -> VTSResult<()> {
    // todo: this canister method should be executed only by our gateway.
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().remove(&req.principal));
    Ok(())
}

#[ic_cdk::update]
fn create_agreement(
    name: String,
    vh_customer: Principal,
    daily_usage_fee: String,
    gas_price: String,
) -> VTSResult<u128> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("requested agreement creation by {}", caller);
    AGREEMENTS.with(|agreements| {
        let mut agreements = agreements.borrow_mut();

        let next_agreement_id = AGREEMENT_ID_COUNTER.with(|counter| {
            let mut counter = counter.borrow_mut();
            *counter += 1;
            *counter
        });

        let agreement = Agreement {
            name,
            vh_provider: caller,
            vh_customer,
            state: AgreementState::Unsigned,
            conditions: AgreementConditions {
                // TODO: use decimals to verify money parameters
                daily_usage_fee,
                gas_price,
            },
            vehicles: vec![],
        };

        agreements.insert(next_agreement_id, agreement);

        Ok(next_agreement_id)
    })
}

#[ic_cdk::update]
fn sign_agreement(agreement_id: u128) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("requested agreement signing by {}", caller);

    AGREEMENTS.with(|agreements| {
        let mut agreements = agreements.borrow_mut();

        if let Some(mut agreement) = agreements.get(&agreement_id) {
            if agreement.vh_customer != caller {
                return Err(Error::InvalidSigner);
            }

            match agreement.state {
                AgreementState::Signed => Err(Error::AlreadyExists),
                _ => {
                    agreement.state = AgreementState::Signed;
                    agreements.insert(agreement_id, agreement);
                    Ok(())
                }
            }
        } else {
            Err(Error::NotFound)
        }
    })
}

#[ic_cdk::update]
fn link_vehicle_to_agreement(agreement_id: u128, vehicle_public_key: Principal) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();

    ic_cdk::println!("requested vehicle linking by {}", caller);

    AGREEMENTS.with(|agreements| {
        let mut agreements = agreements.borrow_mut();

        if let Some(mut agreement) = agreements.get(&agreement_id) {
            if agreement.vh_customer != caller {
                return Err(Error::InvalidSigner);
            }

            if agreement.vehicles.contains(&vehicle_public_key) {
                return Err(Error::AlreadyExists);
            }

            let vehicle = Vehicle {};
            VEHICLES.with(|vehicles| {
                let mut vehicles = vehicles.borrow_mut();
                vehicles.insert(vehicle_public_key, vehicle);
            });

            agreement.vehicles.push(vehicle_public_key);
            agreements.insert(agreement_id, agreement);

            Ok(())
        } else {
            Err(Error::NotFound)
        }
    })
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();
