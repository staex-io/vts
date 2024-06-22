use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;

use bincode::{Decode as BDecode, Encode as BEncode};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use k256::pkcs8::DecodePublicKey;

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


    static ADMINS: RefCell<StableBTreeMap<Principal, Admin, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static USERS: RefCell<StableBTreeMap<Principal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );


    static AGREEMENT_ID_COUNTER: RefCell<u128> = const { RefCell::new(0) };
    static AGREEMENTS: RefCell<StableBTreeMap<u128, Agreement, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static FIRMWARE_REQUESTS: RefCell<StableBTreeMap<Principal, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static VEHICLES: RefCell<StableBTreeMap<Principal, Vehicle, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );
}

pub type VTSResult<T> = Result<T, Error>;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(BEncode, BDecode, Debug, PartialEq, Eq, Hash, CandidType, Deserialize, Clone, Copy)]
pub enum TelemetryType {
    Gas,
}

#[derive(CandidType, Deserialize, Default, Debug, PartialEq)]
pub enum Error {
    #[default]
    Internal,
    AlreadyExists,
    NotFound,
    InvalidSigner,
    Unauthorized,
    InvalidSignature,
    InvalidSignatureFormat,
    DecodeTelemetry,
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

#[derive(BEncode, BDecode)]
pub struct TelemetryRequest {
    pub value: u128,
    pub t_type: TelemetryType,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
struct AccumulatedTelemetry {
    daily: HashMap<String, u32>,
    monthly: HashMap<String, u32>,
    yearly: HashMap<String, u32>,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub struct AggregatedData {
    pub yearly: HashMap<String, u32>,
    pub monthly: HashMap<String, u32>,
    pub daily: HashMap<String, u32>,
}

#[derive(CandidType, Deserialize, Debug, PartialEq, Eq, Hash)]
enum AggregationInterval {
    Daily,
    Monthly,
    Yearly,
}

#[derive(CandidType, Deserialize, Debug)]
struct Admin {}
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

#[derive(CandidType, Deserialize)]
struct Vehicle {
    owner: Principal,
    agreement: Option<u128>,
    public_key: Vec<u8>,
    arch: String,
    firmware: Vec<u8>,
    telemetry: Telemetry,
    accumulated_telemetry: AccumTelemetry,
}
impl_storable!(Vehicle);
type Telemetry = HashMap<TelemetryType, HashMap<i32, HashMap<u8, HashMap<u8, Vec<u128>>>>>;
type AccumTelemetry = HashMap<TelemetryType, HashMap<AggregationInterval, AccumulatedTelemetry>>;

#[derive(CandidType, Deserialize, Debug)]
struct Agreement {
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
    gas_price: String,
}

#[ic_cdk::init]
fn init() {
    ic_cdk_timers::set_timer_interval(std::time::Duration::from_secs(86400), || {
        let _ = accumulate_telemetry_data();
    });
}

#[ic_cdk::update]
fn accumulate_telemetry_data_now() -> Result<(), Error> {
    accumulate_telemetry_data()
}

#[ic_cdk::update]
fn accumulate_telemetry_data() -> Result<(), Error> {
    VEHICLES.with(|vehicles| {
        let vehicles = vehicles.borrow_mut();
        for (_, mut vehicle) in vehicles.iter() {
            for (telemetry_type, telemetry_data) in vehicle.telemetry.iter_mut() {
                vehicle.accumulated_telemetry.entry(*telemetry_type).or_insert_with(HashMap::new);
                for (year, year_data) in telemetry_data.iter_mut() {
                    vehicle
                        .accumulated_telemetry
                        .get_mut(&telemetry_type.clone())
                        .unwrap()
                        .entry(AggregationInterval::Yearly)
                        .or_insert_with(|| AccumulatedTelemetry {
                            daily: HashMap::new(),
                            monthly: HashMap::new(),
                            yearly: HashMap::new(),
                        })
                        .yearly
                        .insert((*year).to_string(), 0);
                    for (month, month_data) in year_data.iter_mut() {
                        vehicle
                            .accumulated_telemetry
                            .get_mut(telemetry_type)
                            .unwrap()
                            .entry(AggregationInterval::Monthly)
                            .or_insert_with(|| AccumulatedTelemetry {
                                daily: HashMap::new(),
                                monthly: HashMap::new(),
                                yearly: HashMap::new(),
                            })
                            .monthly
                            .insert((*month).to_string(), 0);
                        for (day, day_data) in month_data.iter_mut() {
                            vehicle
                                .accumulated_telemetry
                                .get_mut(telemetry_type)
                                .unwrap()
                                .entry(AggregationInterval::Daily)
                                .or_insert_with(|| AccumulatedTelemetry {
                                    daily: HashMap::new(),
                                    monthly: HashMap::new(),
                                    yearly: HashMap::new(),
                                })
                                .daily
                                .insert((*day).to_string(), 0);
                            for value in day_data.iter() {
                                vehicle
                                    .accumulated_telemetry
                                    .get_mut(telemetry_type)
                                    .unwrap()
                                    .entry(AggregationInterval::Yearly)
                                    .or_insert_with(|| AccumulatedTelemetry {
                                        daily: HashMap::new(),
                                        monthly: HashMap::new(),
                                        yearly: HashMap::new(),
                                    })
                                    .yearly
                                    .entry((*year).to_string())
                                    .and_modify(|v| *v += *value as u32)
                                    .or_insert(*value as u32);
                                vehicle
                                    .accumulated_telemetry
                                    .get_mut(telemetry_type)
                                    .unwrap()
                                    .entry(AggregationInterval::Monthly)
                                    .or_insert_with(|| AccumulatedTelemetry {
                                        daily: HashMap::new(),
                                        monthly: HashMap::new(),
                                        yearly: HashMap::new(),
                                    })
                                    .monthly
                                    .entry((*month).to_string())
                                    .and_modify(|v| *v += *value as u32)
                                    .or_insert(*value as u32);
                                vehicle
                                    .accumulated_telemetry
                                    .get_mut(telemetry_type)
                                    .unwrap()
                                    .entry(AggregationInterval::Daily)
                                    .or_insert_with(|| AccumulatedTelemetry {
                                        daily: HashMap::new(),
                                        monthly: HashMap::new(),
                                        yearly: HashMap::new(),
                                    })
                                    .daily
                                    .entry((*day).to_string())
                                    .and_modify(|v| *v += *value as u32)
                                    .or_insert(*value as u32);
                            }
                            day_data.clear();
                        }
                        month_data.clear();
                    }
                    year_data.clear();
                }
            }
            VEHICLES.with(|vehicles| vehicles.borrow_mut().insert(vehicle.owner, vehicle));
        }
        Ok(())
    })
}

#[ic_cdk::update(guard = is_user)]
fn get_aggregated_data(vehicle_id: Principal) -> Result<HashMap<TelemetryType, AggregatedData>, Error> {
    VEHICLES.with(|vehicles| {
        let vehicles = vehicles.borrow();
        if let Some(vehicle) = vehicles.get(&vehicle_id) {
            let mut result = HashMap::new();
            for (telemetry_type, intervals) in &vehicle.accumulated_telemetry {
                let aggregated_data = AggregatedData {
                    daily: intervals
                        .get(&AggregationInterval::Daily)
                        .map_or(HashMap::new(), |data| data.daily.clone()),
                    monthly: intervals
                        .get(&AggregationInterval::Monthly)
                        .map_or(HashMap::new(), |data| data.monthly.clone()),
                    yearly: intervals
                        .get(&AggregationInterval::Yearly)
                        .map_or(HashMap::new(), |data| data.yearly.clone()),
                };
                result.insert(*telemetry_type, aggregated_data);
            }
            Ok(result)
        } else {
            Err(Error::NotFound)
        }
    })
}

#[ic_cdk::update]
fn add_admin(new_admin: Principal) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ADMINS.with(|admins| {
        // If we just deployed canister we can add first admin to it.
        if admins.borrow().is_empty() {
            admins.borrow_mut().insert(caller, Admin {});
            Ok(())
        } else {
            is_admin()?;
            admins.borrow_mut().insert(new_admin, Admin {});
            Ok(())
        }
    })
}

#[ic_cdk::update(guard = is_admin)]
fn delete_admin(admin: Principal) -> VTSResult<()> {
    let caller = ic_cdk::api::caller();
    ADMINS.with(|admins| {
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
    USERS.with(|users| users.borrow().get(&caller).ok_or(Error::NotFound))
}

#[ic_cdk::update(guard = is_user)]
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
    public_key: Vec<u8>,
    arch: String,
    firmware: Vec<u8>,
) -> VTSResult<()> {
    // todo: this canister method should be executed only by our gateway
    let vehicle = Principal::self_authenticating(&public_key);
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().remove(&vh_customer));
    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(
            vehicle,
            Vehicle {
                owner: vh_customer,
                agreement: None,
                public_key,
                arch,
                firmware,
                telemetry: HashMap::new(),
                accumulated_telemetry: HashMap::new(),
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

#[ic_cdk::update(guard = is_user)]
fn create_agreement(name: String, vh_customer: Principal, gas_price: String) -> VTSResult<u128> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("requested agreement creation by {}", caller);

    let next_agreement_id = AGREEMENT_ID_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    });

    AGREEMENTS.with(|agreements| {
        let agreement = Agreement {
            name,
            vh_provider: caller,
            vh_customer,
            state: AgreementState::Unsigned,
            conditions: AgreementConditions {
                // todo: use decimals library to verify money parameters
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

#[ic_cdk::update(guard = is_user)]
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

#[ic_cdk::update(guard = is_user)]
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

#[ic_cdk::update]
fn store_telemetry(principal: Principal, data: Vec<u8>, signature: Vec<u8>) -> VTSResult<()> {
    let signature = Signature::from_slice(&signature).map_err(|_| Error::InvalidSignatureFormat)?;
    let mut vehicle = VEHICLES.with(|vehicles| vehicles.borrow().get(&principal).ok_or(Error::NotFound))?;
    let verifying_key =
        VerifyingKey::from_public_key_der(&vehicle.public_key).map_err(|_| Error::Internal)?;
    verifying_key.verify(&data, &signature).map_err(|_| Error::InvalidSignature)?;
    let telemetry: TelemetryRequest = bincode::decode_from_slice(&data, bincode::config::standard())
        .map_err(|_| Error::DecodeTelemetry)?
        .0;
    ic_cdk::println!("received new telemetry: value={}; type={:?}", telemetry.value, telemetry.t_type);
    let timestamp = ic_cdk::api::time();
    let timestamp = time::OffsetDateTime::from_unix_timestamp(timestamp as i64).unwrap();
    vehicle
        .telemetry
        .get_mut(&telemetry.t_type)
        .ok_or(Error::NotFound)?
        .get_mut(&timestamp.year())
        .get_or_insert(&mut HashMap::new())
        .get_mut(&(timestamp.month() as u8))
        .get_or_insert(&mut HashMap::new())
        .get_mut(&timestamp.day())
        .get_or_insert(&mut Vec::new())
        .push(telemetry.value);
    VEHICLES.with(|vehicles| vehicles.borrow_mut().insert(principal, vehicle));
    Ok(())
}

// We use this method only in tests to not restart dfx node.
// And make every test with clean state.
#[cfg(feature = "clean_state")]
#[ic_cdk::update]
fn clean_state() {
    AGREEMENT_ID_COUNTER.set(0);
    FIRMWARE_REQUESTS.with(|firmware_requests| firmware_requests.borrow_mut().clear_new());
    USERS.with(|users| users.borrow_mut().clear_new());
    VEHICLES.with(|vehicles| vehicles.borrow_mut().clear_new());
    AGREEMENTS.with(|agreements| agreements.borrow_mut().clear_new());
    ADMINS.with(|admins| admins.borrow_mut().clear_new());
}

// We use this method only in tests to not restart dfx node.
// To make pre-fill with some data for testing purposes.
#[cfg(feature = "predefined_telemetry")]
#[ic_cdk::update]
fn fill_predefined_telemetry() {
    let vh_provider: Principal =
        Principal::from_text("s76co-mfsqq-uqz5p-jfdh2-z3izx-tnpp7-r5vwe-up6yj-va7ks-5s22x-eqe").unwrap();
    let vh_customer: Principal =
        Principal::from_text("xnufg-sj4kb-rjjc3-73zhk-3msse-3cqb7-qcfgt-kq5lq-s3w5v-mctsx-bae").unwrap();
    let vehicle: Principal =
        Principal::from_text("zddkf-v7muw-3zj2q-kwijg-ulgjf-lpj32-t5qvx-5l3yb-rarsi-pq5w6-3ae").unwrap();

    const AGREEMENT_ID: u128 = 1;

    // Initialize admin.
    ADMINS.with(|admins| admins.borrow_mut().insert(vh_provider, Admin {}));
    // Add customer to users storage.
    USERS.with(|users| {
        users.borrow_mut().insert(
            vh_customer,
            User {
                vehicles: HashMap::from_iter(vec![(vehicle, ())]),
                agreements: HashMap::from_iter(vec![(AGREEMENT_ID, ())]),
            },
        )
    });

    // Initialize agreement.
    AGREEMENTS.with(|agreements| {
        agreements.borrow_mut().insert(
            AGREEMENT_ID,
            Agreement {
                name: String::from("Test Agreement"),
                vh_provider,
                vh_customer,
                state: AgreementState::Signed,
                conditions: AgreementConditions {
                    gas_price: String::from("1.35"),
                },
                vehicles: HashMap::from_iter(vec![(vehicle, ())]),
            },
        )
    });
    AGREEMENT_ID_COUNTER.set(AGREEMENT_ID + 1);

    // Add one pending firmware request.
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().insert(vh_customer, ()));

    // Initialize vehicle.
    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(
            vehicle,
            Vehicle {
                owner: vh_customer,
                agreement: Some(AGREEMENT_ID),
                public_key: Vec::new(),
                arch: String::from("amd64"),
                firmware: Vec::new(),
                telemetry: HashMap::from_iter(vec![(
                    TelemetryType::Gas,
                    HashMap::from_iter(vec![(
                        2024,
                        HashMap::from_iter(vec![(
                            time::Month::June as u8,
                            HashMap::from_iter(vec![
                                (15, vec![96, 86]),
                                (16, vec![52]),
                                (17, vec![991, 51]),
                                (18, vec![71, 23, 17]),
                            ]),
                        )]),
                    )]),
                )]),
                accumulated_telemetry: HashMap::new(),
            },
        )
    });
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
