use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};

macro_rules! impl_storable {
    ($struct_name:ident) => {
        impl Storable for $struct_name {
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
    };
}

const ERR_UNAUTHORIZED: &str = "unauthorized";

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static AGREEMENT_ID_COUNTER: RefCell<u128> = const { RefCell::new(0) };

    static FIRMWARE_REQUESTS: RefCell<StableBTreeMap<Principal, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static USERS: RefCell<StableBTreeMap<Principal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    static VEHICLES: RefCell<StableBTreeMap<Principal, Vehicle, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static AGREEMENTS: RefCell<StableBTreeMap<u128, Agreement, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static ADMINS: RefCell<StableBTreeMap<Principal, Admin, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );
}

pub type VTSResult<T> = Result<T, Error>;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(CandidType, Deserialize, Default, Debug, PartialEq)]
pub enum Error {
    #[default]
    Internal,
    AlreadyExists,
    NotFound,
    InvalidSigner,
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthorized => write!(f, "{}", ERR_UNAUTHORIZED),
            _ => write!(f, "unknown"),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        match value.as_str() {
            ERR_UNAUTHORIZED => Self::Unauthorized,
            _ => Self::Internal,
        }
    }
}

#[derive(CandidType, Deserialize, Debug)]
struct Admin {
    public_key: Principal,
}
impl_storable!(Admin);

#[derive(CandidType, Deserialize, Debug)]
enum AgreementState {
    Unsigned,
    Signed,
}

#[derive(CandidType, Deserialize, Debug)]
struct User {
    vehicles: HashMap<Principal, ()>,
    agreements: HashMap<u128, ()>,
}
impl_storable!(User);

#[derive(CandidType, Deserialize, Debug)]
struct Vehicle {
    owner: Principal,
    agreement: Option<u128>,
    identity: Principal,
    arch: String,
    firmware: Vec<u8>,
}
impl_storable!(Vehicle);

#[derive(CandidType, Deserialize, Debug)]
struct Agreement {
    id: u128,
    name: String,
    vh_provider: Principal,
    vh_customer: Principal,
    state: AgreementState,
    conditions: AgreementConditions,
    vehicles: HashMap<Principal, ()>,
}
impl_storable!(Agreement);

#[derive(CandidType, Deserialize, Debug)]
struct AgreementConditions {
    daily_usage_fee: String,
    gas_price: String,
}

#[ic_cdk::update]
fn add_admin(new_admin: Principal) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ADMINS.with(|admins| {
        // If we just deployed canister we can add first admin to it.
        if admins.borrow().is_empty() {
            admins.borrow_mut().insert(caller, Admin { public_key: caller });
            Ok(())
        } else {
            is_admin()?;
            admins.borrow_mut().insert(
                new_admin,
                Admin {
                    public_key: new_admin,
                },
            );
            Ok(())
        }
    })
}

#[ic_cdk::update(guard = is_admin)]
fn delete_admin(admin: Principal) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ADMINS.with(|admins| {
        if !admins.borrow().contains_key(&caller) {
            return Err(Error::Unauthorized);
        }
        if !admins.borrow().contains_key(&admin) {
            return Err(Error::NotFound);
        }
        if admin == caller {
            return Err(Error::InvalidSigner);
        }
        admins.borrow_mut().remove(&admin);
        Ok(())
    })
}

#[ic_cdk::update(guard = is_admin)]
fn register_user(user: Principal) -> VTSResult<()> {
    if USERS.with(|users| users.borrow().contains_key(&user)) {
        return Err(Error::AlreadyExists);
    }

    USERS.with(|users| {
        users.borrow_mut().insert(
            user,
            User {
                vehicles: HashMap::new(),
                agreements: HashMap::new(),
            },
        );
    });

    Ok(())
}

#[ic_cdk::update(guard = is_admin)]
fn delete_user(user: Principal) -> VTSResult<()> {
    // Check if the user to be deleted exists.
    USERS.with(|users| {
        if !users.borrow().contains_key(&user) {
            return Err(Error::NotFound);
        }
        Ok(())
    })?;

    // Remove the user.
    USERS.with(|users| users.borrow_mut().remove(&user));
    Ok(())
}

#[ic_cdk::query(guard = is_user)]
fn get_user() -> VTSResult<User> {
    let caller = ic_cdk::api::caller();
    // is_registered_user(caller)?;
    USERS.with(|users| users.borrow().get(&caller).ok_or(Error::NotFound))
}

#[ic_cdk::query(guard = is_user)]
fn request_firmware() -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("{} is requested firmware", caller);
    FIRMWARE_REQUESTS.with(|requests| {
        if requests.borrow_mut().contains_key(&caller) {
            return Err(Error::AlreadyExists);
        }
        requests.borrow_mut().insert(caller, ());
        Ok(())
    })?;
    Ok(())
}

// This method can return first available firmware request.
#[ic_cdk::query]
fn get_firmware_requests() -> VTSResult<Principal> {
    // todo: this canister method should be executed only by our gateway
    let (identity, _) =
        FIRMWARE_REQUESTS.with(|requests| requests.borrow().first_key_value().ok_or(Error::NotFound))?;
    Ok(identity)
}

// By this method we can check active firmware requests for the particular user.
#[ic_cdk::query(guard = is_user)]
fn get_firmware_requests_by_user() -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    FIRMWARE_REQUESTS.with(|requests| requests.borrow().get(&caller).ok_or(Error::NotFound))?;
    Ok(())
}

#[ic_cdk::update]
fn upload_firmware(
    vh_customer: Principal,
    vehicle: Principal,
    arch: String,
    firmware: Vec<u8>,
) -> VTSResult<()> {
    // todo: this canister method should be executed only by our gateway
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().remove(&vh_customer));
    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(
            vehicle,
            Vehicle {
                owner: vh_customer,
                agreement: None,
                identity: vehicle,
                arch,
                firmware,
            },
        )
    });
    USERS.with(|users| -> VTSResult<()> {
        let mut user = users.borrow_mut().get(&vh_customer).ok_or(Error::NotFound)?;
        user.vehicles.insert(vehicle, ());
        users.borrow_mut().insert(vh_customer, user);
        Ok(())
    })
}

#[ic_cdk::query(guard = is_user)]
fn get_vehicle(vehicle: Principal) -> VTSResult<Vehicle> {
    let caller = ic_cdk::api::caller();
    let vehicle = VEHICLES.with(|vehicles| vehicles.borrow().get(&vehicle).ok_or(Error::NotFound))?;
    if vehicle.owner != caller {
        return Err(Error::InvalidSigner);
    }
    Ok(vehicle)
}

#[ic_cdk::query(guard = is_user)]
fn create_agreement(
    name: String,
    vh_customer: Principal,
    daily_usage_fee: String,
    gas_price: String,
) -> VTSResult<u128> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("requested agreement creation by {}", caller);

    let next_agreement_id = AGREEMENT_ID_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    });

    AGREEMENTS.with(|agreements| {
        let agreement = Agreement {
            id: next_agreement_id,
            name,
            vh_provider: caller,
            vh_customer,
            state: AgreementState::Unsigned,
            conditions: AgreementConditions {
                // todo: use decimals library to verify money parameters
                daily_usage_fee,
                gas_price,
            },
            vehicles: HashMap::new(),
        };
        let mut agreements = agreements.borrow_mut();
        agreements.insert(next_agreement_id, agreement);
    });

    USERS.with(|users| -> Result<(), Error> {
        let mut vh_provider_user = users.borrow_mut().get(&caller).ok_or(Error::NotFound)?;
        let mut vh_customer_user = users.borrow_mut().get(&vh_customer).ok_or(Error::NotFound)?;
        vh_provider_user.agreements.insert(next_agreement_id, ());
        vh_customer_user.agreements.insert(next_agreement_id, ());
        users.borrow_mut().insert(caller, vh_provider_user);
        users.borrow_mut().insert(vh_customer, vh_customer_user);
        Ok(())
    })?;

    Ok(next_agreement_id)
}

#[ic_cdk::query(guard = is_user)]
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

#[ic_cdk::query(guard = is_user)]
fn link_vehicle(agreement_id: u128, vehicle_identity: Principal) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("requested vehicle linking by {}", caller);

    AGREEMENTS.with(|agreements| {
        let mut agreements = agreements.borrow_mut();
        let mut agreement = agreements.get(&agreement_id).ok_or(Error::NotFound)?;

        if agreement.vehicles.contains_key(&vehicle_identity) {
            return Err(Error::AlreadyExists);
        }
        if caller != agreement.vh_customer {
            ic_cdk::println!("vehicle provider tried to link vehicle to its own agreement");
            return Err(Error::InvalidSigner);
        }

        agreement.vehicles.insert(vehicle_identity, ());
        agreements.insert(agreement_id, agreement);

        Ok(())
    })?;

    VEHICLES.with(|vehicles| {
        let mut vehicle = vehicles.borrow_mut().get(&vehicle_identity).ok_or(Error::NotFound)?;

        if caller != vehicle.owner {
            return Err(Error::InvalidSigner);
        }
        if vehicle.agreement.is_some() {
            return Err(Error::AlreadyExists);
        }

        vehicle.agreement = Some(agreement_id);
        vehicles.borrow_mut().insert(vehicle_identity, vehicle);

        Ok(())
    })
}

#[ic_cdk::query(guard = is_user)]
fn get_user_agreements() -> VTSResult<Vec<Agreement>> {
    let caller = ic_cdk::api::caller();
    let user = USERS.with(|users| users.borrow().get(&caller).ok_or(Error::NotFound))?;
    let mut agreements = Vec::with_capacity(user.agreements.len());
    AGREEMENTS.with(|agreements_storage| -> Result<(), Error> {
        let agreements_storage = agreements_storage.borrow();
        for (user_agreement_id, _) in user.agreements {
            let agreement = agreements_storage.get(&user_agreement_id).ok_or(Error::NotFound)?;
            agreements.push(agreement)
        }
        Ok(())
    })?;
    Ok(agreements)
}

#[ic_cdk::query(guard = is_user)]
fn get_vehicles_by_agreement(agreement_id: u128) -> VTSResult<HashMap<Principal, ()>> {
    AGREEMENTS.with(|agreements| {
        let agreements = agreements.borrow();
        let agreement = agreements.get(&agreement_id).ok_or(Error::NotFound)?;
        Ok(agreement.vehicles)
    })
}

fn is_admin() -> Result<(), String> {
    ADMINS.with(|admins| {
        if !admins.borrow().contains_key(&ic_cdk::caller()) {
            return Err(Error::Unauthorized.to_string());
        }
        Ok(())
    })
}

fn is_user() -> Result<(), String> {
    USERS.with(|users| {
        if !users.borrow().contains_key(&ic_cdk::caller()) {
            return Err(Error::Unauthorized.to_string());
        }
        Ok(())
    })
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();
