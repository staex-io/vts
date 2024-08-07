use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;

use bincode::{Decode as BDecode, Encode as BEncode};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::BlockIndex;
use icrc_ledger_types::icrc2::transfer_from::{TransferFromArgs, TransferFromError};
use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use k256::pkcs8::DecodePublicKey;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use time::{Month, OffsetDateTime};

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

const TOKENS_MULTIPLIER: u128 = 1_000_000_000;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));


    static ADMINS: RefCell<StableBTreeMap<Principal, Admin, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static USERS: RefCell<StableBTreeMap<Principal, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );


    static AGREEMENT_ID_COUNTER: RefCell<u128> = const { RefCell::new(0) };
    static AGREEMENTS: RefCell<StableBTreeMap<u128, Agreement, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static FIRMWARE_REQUESTS: RefCell<StableBTreeMap<Principal, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static VEHICLES: RefCell<StableBTreeMap<Principal, Vehicle, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static INVOICE_ID_COUNTER: RefCell<u128> = const { RefCell::new(0) };
    static INVOICES: RefCell<StableBTreeMap<u128, Invoice, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );
    // We need to store pending invoices for gateway.
    // Gateway proceed with pending invoice to send some notification for the user.
    // And delete them from this structure.
    static PENDING_INVOICES: RefCell<StableBTreeMap<u128, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );
    // Same as PENDING_INVOIES.
    static PAID_INVOICES: RefCell<StableBTreeMap<u128, (), Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8))))
    );
}

pub type VTSResult<T> = Result<T, Error>;

pub type AccumulatedTelemetry = HashMap<TelemetryType, HashMap<i32, AccumulatedTelemetryYearly>>;

type Memory = VirtualMemory<DefaultMemoryImpl>;

type Telemetry = HashMap<TelemetryType, HashMap<i32, HashMap<u8, HashMap<u8, Vec<u128>>>>>;

#[derive(BEncode, BDecode, PartialEq, Eq, Hash, CandidType, Deserialize, Debug, Clone, Copy)]
pub enum TelemetryType {
    Gas,
}

#[derive(CandidType, Deserialize, Default, PartialEq, Debug)]
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
    InvalidData,
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

#[derive(CandidType, Deserialize)]
pub enum StoreTelemetryResponse {
    // Vehicle can continue to work.
    On,
    // Vehicle cannot continue to work and should turned off.
    Off,
}

#[derive(CandidType, Deserialize)]
enum AgreementState {
    Unsigned,
    Signed,
}

#[derive(CandidType, Deserialize)]
enum InvoiceStatus {
    Unpaid,
    Paid,
}

#[derive(BEncode, BDecode)]
pub struct StoreTelemetryRequest {
    pub value: u128,
    pub t_type: TelemetryType,
}

#[derive(CandidType, Deserialize)]
pub struct PendingInvoice {
    pub id: u128,
    pub customer_email: Option<String>,
    pub vehicle: Principal,
}

#[derive(CandidType, Deserialize, Default, PartialEq, Debug)]
pub struct AccumulatedTelemetryYearly {
    pub value: u128,
    pub monthly: HashMap<u8, AccumulatedTelemetryMonthy>,
}

#[derive(CandidType, Deserialize, Default, PartialEq, Debug)]
pub struct AccumulatedTelemetryMonthy {
    pub value: u128,
    pub daily: HashMap<u8, u128>,
}

#[derive(CandidType, Deserialize)]
struct Admin {}
impl_storable!(Admin);

#[derive(CandidType, Deserialize)]
struct User {
    vehicles: HashMap<Principal, ()>,
    agreements: HashMap<u128, ()>,
    email: Option<String>,
}
impl_storable!(User);

#[derive(CandidType, Deserialize)]
struct Vehicle {
    provider: Option<Principal>,
    customer: Principal,
    agreement: Option<u128>,
    public_key: Vec<u8>,
    arch: String,
    firmware: Vec<u8>,
    on_off: bool,
    telemetry: Telemetry,
    accumulated_telemetry: AccumulatedTelemetry,
    invoices: Vec<u128>,
}
impl_storable!(Vehicle);

#[derive(CandidType, Deserialize)]
struct Invoice {
    id: u128,
    status: InvoiceStatus,
    vehicle: Principal,
    agreement: u128,
    period: (i32, u8), // year + month
    total_cost: u128,
}
impl_storable!(Invoice);

#[derive(CandidType, Deserialize)]
struct Agreement {
    id: u128, // we need to store it here to be able to use it on frontend
    name: String,
    vh_provider: Principal,
    vh_customer: Principal,
    state: AgreementState,
    conditions: AgreementConditions,
    vehicles: HashMap<Principal, ()>,
}
impl_storable!(Agreement);

#[derive(CandidType, Deserialize)]
struct AgreementConditions {
    gas_price: String,
}

#[ic_cdk::init]
fn init() {
    // Every day or 24h.
    ic_cdk_timers::set_timer_interval(std::time::Duration::from_secs(86400), || {
        if let Err(e) = accumulate_telemetry_data() {
            ic_cdk::println!("failed to accumulate telemetry data: {}", e)
        }
    });
}

#[ic_cdk::query(guard = is_user)]
fn get_invoice(invoice_id: u128) -> Result<Invoice, Error> {
    // todo: restrict by get provider or customer
    INVOICES.with(|invoices| {
        let invoices = invoices.borrow();
        invoices.get(&invoice_id).ok_or(Error::NotFound)
    })
}

#[ic_cdk::update(guard = is_canister)]
fn accumulate_telemetry_data() -> VTSResult<()> {
    ic_cdk::println!("starting to accumulate telemetry data");

    let mut accumulated_telemetry: HashMap<Principal, AccumulatedTelemetry> =
        HashMap::with_capacity(VEHICLES.with(|vehicles| vehicles.borrow().len() as usize));

    VEHICLES.with(|vehicles| -> VTSResult<()> {
        let vehicles = vehicles.borrow();
        for (principal, mut vehicle) in vehicles.iter() {
            for (telemetry_type, telemetry_data) in vehicle.telemetry.iter_mut() {
                vehicle.accumulated_telemetry.entry(*telemetry_type).or_default();
                for (year, year_data) in telemetry_data.iter_mut() {
                    for (month, month_data) in year_data.iter_mut() {
                        for (day, day_data) in month_data.iter_mut() {
                            for value in day_data.iter() {
                                vehicle
                                    .accumulated_telemetry
                                    .get_mut(telemetry_type)
                                    .ok_or(Error::NotFound)?
                                    .entry(*year)
                                    .and_modify(|v| v.value += *value)
                                    .or_insert(AccumulatedTelemetryYearly {
                                        value: *value,
                                        monthly: HashMap::new(),
                                    });
                                vehicle
                                    .accumulated_telemetry
                                    .get_mut(telemetry_type)
                                    .ok_or(Error::NotFound)?
                                    .get_mut(year)
                                    .ok_or(Error::NotFound)?
                                    .monthly
                                    .entry(*month)
                                    .and_modify(|v| v.value += *value)
                                    .or_insert(AccumulatedTelemetryMonthy {
                                        value: *value,
                                        daily: HashMap::new(),
                                    });
                                vehicle
                                    .accumulated_telemetry
                                    .get_mut(telemetry_type)
                                    .ok_or(Error::NotFound)?
                                    .get_mut(year)
                                    .ok_or(Error::NotFound)?
                                    .monthly
                                    .entry(*month)
                                    .or_default()
                                    .daily
                                    .entry(*day)
                                    .and_modify(|v| *v += *value)
                                    .or_insert(*value);
                            }
                            day_data.clear();
                        }
                        month_data.clear();
                    }
                    year_data.clear();
                }
            }
            accumulated_telemetry.insert(principal, vehicle.accumulated_telemetry);
        }

        Ok(())
    })?;

    VEHICLES.with(|vehicles| -> VTSResult<()> {
        let mut vehicles = vehicles.borrow_mut();
        for (v_principal, vat) in accumulated_telemetry {
            let mut vehicle = vehicles.get(&v_principal).ok_or(Error::NotFound)?;
            vehicle.accumulated_telemetry = vat;
            vehicles.insert(v_principal, vehicle);
        }
        Ok(())
    })?;

    // Check if it's the first day of the month.
    let timestamp = ic_cdk::api::time();
    let timestamp =
        OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128).map_err(|_| Error::InvalidData)?;
    if timestamp.day() == 1 {
        let (previous_year, previous_month) = if timestamp.month() == Month::January {
            (timestamp.year() - 1, Month::December)
        } else {
            (timestamp.year(), timestamp.month().previous())
        };
        VEHICLES.with(|vehicles| -> VTSResult<()> {
            let vehicles = vehicles.borrow();
            for (vehicle_id, _) in vehicles.iter() {
                create_invoice(
                    vehicle_id,
                    previous_year,
                    previous_month as u8,
                    &get_aggregated_data(vehicle_id)?,
                )?;
            }
            Ok(())
        })?;
    }

    ic_cdk::println!("accumulating telemetry data is finished");
    Ok(())
}

#[ic_cdk::query(guard = is_user)]
fn get_aggregated_data(vehicle_id: Principal) -> VTSResult<AccumulatedTelemetry> {
    VEHICLES.with(|vehicles| {
        let vehicles = vehicles.borrow();
        let vehicle = vehicles.get(&vehicle_id).ok_or(Error::NotFound)?;
        Ok(vehicle.accumulated_telemetry)
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
fn register_user(user: Principal, email: Option<String>) -> VTSResult<()> {
    if USERS.with(|users| users.borrow().contains_key(&user)) {
        return Err(Error::AlreadyExists);
    }

    USERS.with(|users| {
        users.borrow_mut().insert(
            user,
            User {
                vehicles: HashMap::new(),
                agreements: HashMap::new(),
                email,
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
#[ic_cdk::query(guard = is_gateway)]
fn get_firmware_requests() -> VTSResult<Principal> {
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

#[ic_cdk::update(guard = is_gateway)]
fn upload_firmware(
    vh_customer: Principal,
    public_key: Vec<u8>,
    arch: String,
    firmware: Vec<u8>,
) -> VTSResult<()> {
    let vehicle = Principal::self_authenticating(&public_key);
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().remove(&vh_customer));
    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(
            vehicle,
            Vehicle {
                provider: None,
                customer: vh_customer,
                agreement: None,
                public_key,
                arch,
                firmware,
                telemetry: HashMap::new(),
                on_off: true,
                accumulated_telemetry: HashMap::new(),
                invoices: Vec::new(),
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
    match vehicle.provider {
        Some(provider) => {
            if provider != caller && vehicle.customer != caller {
                return Err(Error::InvalidSigner);
            }
        }
        None => {
            if vehicle.customer != caller {
                return Err(Error::InvalidSigner);
            }
        }
    }
    Ok(vehicle)
}

#[ic_cdk::update(guard = is_user)]
fn create_agreement(name: String, vh_customer: Principal, gas_price: String) -> VTSResult<u128> {
    let caller = ic_cdk::api::caller();
    ic_cdk::println!("requested agreement creation by {}", caller);

    // Veryfy that user passed ok gas price.
    Decimal::from_str(&gas_price).map_err(|_| Error::InvalidData)?;

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
            conditions: AgreementConditions { gas_price },
            vehicles: HashMap::new(),
        };
        let mut agreements = agreements.borrow_mut();
        agreements.insert(next_agreement_id, agreement);
    });

    USERS.with(|users| -> VTSResult<()> {
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

    let vh_provider: Principal = AGREEMENTS.with(|agreements| {
        let mut agreements = agreements.borrow_mut();
        let mut agreement = agreements.get(&agreement_id).ok_or(Error::NotFound)?;
        let vh_provider = agreement.vh_provider;

        if agreement.vehicles.contains_key(&vehicle_identity) {
            return Err(Error::AlreadyExists);
        }
        if caller != agreement.vh_customer {
            ic_cdk::println!("vehicle provider tried to link vehicle to its own agreement");
            return Err(Error::InvalidSigner);
        }

        agreement.vehicles.insert(vehicle_identity, ());
        agreements.insert(agreement_id, agreement);

        Ok(vh_provider)
    })?;

    VEHICLES.with(|vehicles| {
        let mut vehicle = vehicles.borrow_mut().get(&vehicle_identity).ok_or(Error::NotFound)?;

        if caller != vehicle.customer {
            return Err(Error::InvalidSigner);
        }
        if vehicle.agreement.is_some() {
            return Err(Error::AlreadyExists);
        }

        vehicle.agreement = Some(agreement_id);
        vehicle.provider = Some(vh_provider);
        vehicles.borrow_mut().insert(vehicle_identity, vehicle);

        Ok(())
    })?;

    USERS.with(|users| -> VTSResult<()> {
        let mut provider = users.borrow().get(&vh_provider).ok_or(Error::NotFound)?;
        provider.vehicles.insert(vehicle_identity, ());
        users.borrow_mut().insert(vh_provider, provider);
        Ok(())
    })
}

#[ic_cdk::update(guard = is_user)]
async fn pay_for_invoice(id: u128) -> VTSResult<()> {
    let mut invoice = INVOICES.with(|invoices| invoices.borrow().get(&id).ok_or(Error::NotFound))?;
    if let InvoiceStatus::Paid = invoice.status {
        return Ok(());
    }
    let provider = VEHICLES.with(|vehicles| {
        vehicles.borrow().get(&invoice.vehicle).ok_or(Error::NotFound)?.provider.ok_or(Error::NotFound)
    })?;

    let transfer_from_args: TransferFromArgs = TransferFromArgs {
        amount: invoice.total_cost.into(),
        from: Account::from(ic_cdk::caller()),
        to: Account::from(provider),
        memo: None,
        spender_subaccount: None,
        fee: None,
        created_at_time: None,
    };
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        Principal::from_text("ryjl3-tyaaa-aaaaa-aaaba-cai").unwrap(),
        "icrc2_transfer_from",
        (transfer_from_args,),
    )
    .await
    .map_err(|_| Error::Internal)?
    .0
    .map_err(|_| Error::Internal)?;

    invoice.status = InvoiceStatus::Paid;
    INVOICES.with(|invoices| invoices.borrow_mut().insert(id, invoice));
    PAID_INVOICES.with(|invoices| invoices.borrow_mut().insert(id, ()));

    Ok(())
}

#[ic_cdk::query(guard = is_user)]
fn get_user_agreements() -> VTSResult<Vec<Agreement>> {
    let caller = ic_cdk::api::caller();
    let user = USERS.with(|users| users.borrow().get(&caller).ok_or(Error::NotFound))?;
    let mut agreements = Vec::with_capacity(user.agreements.len());
    AGREEMENTS.with(|agreements_storage| -> VTSResult<()> {
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
fn store_telemetry(
    principal: Principal,
    data: Vec<u8>,
    signature: Vec<u8>,
) -> VTSResult<StoreTelemetryResponse> {
    let signature = Signature::from_slice(&signature).map_err(|_| Error::InvalidSignatureFormat)?;
    let mut vehicle = VEHICLES.with(|vehicles| vehicles.borrow().get(&principal).ok_or(Error::NotFound))?;
    let verifying_key =
        VerifyingKey::from_public_key_der(&vehicle.public_key).map_err(|_| Error::Internal)?;
    verifying_key.verify(&data, &signature).map_err(|_| Error::InvalidSignature)?;
    let telemetry: StoreTelemetryRequest = bincode::decode_from_slice(&data, bincode::config::standard())
        .map_err(|_| Error::DecodeTelemetry)?
        .0;
    ic_cdk::println!("received new telemetry: value={}; type={:?}", telemetry.value, telemetry.t_type);
    let timestamp = ic_cdk::api::time();
    let timestamp =
        OffsetDateTime::from_unix_timestamp_nanos(timestamp as i128).map_err(|_| Error::InvalidSigner)?;
    let on_off = vehicle.on_off;
    vehicle
        .telemetry
        .get_mut(&telemetry.t_type)
        .get_or_insert(&mut HashMap::new())
        .get_mut(&timestamp.year())
        .get_or_insert(&mut HashMap::new())
        .get_mut(&(timestamp.month() as u8))
        .get_or_insert(&mut HashMap::new())
        .get_mut(&timestamp.day())
        .get_or_insert(&mut Vec::new())
        .push(telemetry.value);
    VEHICLES.with(|vehicles| vehicles.borrow_mut().insert(principal, vehicle));
    if !on_off {
        return Ok(StoreTelemetryResponse::Off);
    }
    Ok(StoreTelemetryResponse::On)
}

#[ic_cdk::query(guard = is_gateway)]
fn get_pending_invoices() -> VTSResult<Vec<PendingInvoice>> {
    let pending_invoices = PENDING_INVOICES
        .with(|invoices| -> VTSResult<Vec<PendingInvoice>> { prepare_pending_invoices(invoices) })?;
    Ok(pending_invoices)
}

#[ic_cdk::query(guard = is_gateway)]
fn get_paid_invoices() -> VTSResult<Vec<PendingInvoice>> {
    let pending_invoices = PAID_INVOICES
        .with(|invoices| -> VTSResult<Vec<PendingInvoice>> { prepare_pending_invoices(invoices) })?;
    Ok(pending_invoices)
}

#[ic_cdk::update(guard = is_gateway)]
fn delete_paid_invoices(ids: Vec<u128>) {
    PAID_INVOICES.with(|invoices| {
        let mut invoices = invoices.borrow_mut();
        for id in ids {
            invoices.remove(&id);
        }
    });
}

#[ic_cdk::update(guard = is_gateway)]
fn delete_pending_invoices(ids: Vec<u128>) {
    PENDING_INVOICES.with(|invoices| {
        let mut invoices = invoices.borrow_mut();
        for id in ids {
            invoices.remove(&id);
        }
    });
}

#[ic_cdk::update(guard = is_user)]
fn turn_on_off_vehicle(vehicle: Principal, on_off: bool) -> VTSResult<()> {
    VEHICLES.with(|vehicles| -> VTSResult<()> {
        let mut v = vehicles.borrow().get(&vehicle).ok_or(Error::NotFound)?;
        if let Some(provider) = v.provider {
            if provider != ic_cdk::caller() {
                return Err(Error::InvalidSigner);
            }
        } else {
            return Err(Error::InvalidSigner);
        }
        v.on_off = on_off;
        vehicles.borrow_mut().insert(vehicle, v);
        Ok(())
    })
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
fn fill_predefined_telemetry(vh_provider: Principal, vh_customer: Principal, vehicle_public_key_hex: String) {
    const SIGNED_AGREEMENT_ID: u128 = 1;
    const UNSIGNED_AGREEMENT_ID: u128 = 2;
    const PAID_VEHICLE_INVOICE_ID: u128 = 1;
    const UNPAID_VEHICLE_INVOICE_ID: u128 = 2;

    let vehicle_public_key = hex::decode(vehicle_public_key_hex).unwrap();
    let vehicle = Principal::self_authenticating(&vehicle_public_key);

    // Add provider and customer to users storage.
    USERS.with(|users| {
        users.borrow_mut().insert(
            vh_provider,
            User {
                vehicles: HashMap::from_iter(vec![(vehicle, ())]),
                agreements: HashMap::from_iter(vec![(SIGNED_AGREEMENT_ID, ()), (UNSIGNED_AGREEMENT_ID, ())]),
                email: Some(String::from("provider@staex.io")),
            },
        );
        users.borrow_mut().insert(
            vh_customer,
            User {
                vehicles: HashMap::from_iter(vec![(vehicle, ())]),
                agreements: HashMap::from_iter(vec![(SIGNED_AGREEMENT_ID, ()), (UNSIGNED_AGREEMENT_ID, ())]),
                email: Some(String::from("customer@staex.io")),
            },
        );
    });

    // Initialize agreement.
    AGREEMENTS.with(|agreements| {
        agreements.borrow_mut().insert(
            SIGNED_AGREEMENT_ID,
            Agreement {
                id: SIGNED_AGREEMENT_ID,
                name: String::from("Solar Energy GmbH"),
                vh_provider,
                vh_customer,
                state: AgreementState::Signed,
                conditions: AgreementConditions {
                    gas_price: String::from("1.35"),
                },
                vehicles: HashMap::from_iter(vec![(vehicle, ())]),
            },
        );
        agreements.borrow_mut().insert(
            UNSIGNED_AGREEMENT_ID,
            Agreement {
                id: UNSIGNED_AGREEMENT_ID,
                name: String::from("Super Duper Vehicles inc."),
                vh_provider,
                vh_customer,
                state: AgreementState::Unsigned,
                conditions: AgreementConditions {
                    gas_price: String::from("4.71"),
                },
                vehicles: HashMap::new(),
            },
        );
    });
    AGREEMENT_ID_COUNTER.set(UNSIGNED_AGREEMENT_ID);

    // Add one pending firmware request.
    FIRMWARE_REQUESTS.with(|requests| requests.borrow_mut().insert(vh_customer, ()));

    INVOICES.with(|invoices| {
        invoices.borrow_mut().insert(
            PAID_VEHICLE_INVOICE_ID,
            Invoice {
                id: PAID_VEHICLE_INVOICE_ID,
                status: InvoiceStatus::Paid,
                vehicle,
                agreement: SIGNED_AGREEMENT_ID,
                period: (2024, 6),
                total_cost: 67 * TOKENS_MULTIPLIER,
            },
        );
        invoices.borrow_mut().insert(
            UNPAID_VEHICLE_INVOICE_ID,
            Invoice {
                id: UNPAID_VEHICLE_INVOICE_ID,
                status: InvoiceStatus::Unpaid,
                vehicle,
                agreement: SIGNED_AGREEMENT_ID,
                period: (2024, 7),
                total_cost: 23 * TOKENS_MULTIPLIER,
            },
        );
    });
    INVOICE_ID_COUNTER.set(UNPAID_VEHICLE_INVOICE_ID);

    // Initialize vehicle.
    VEHICLES.with(|vehicles| {
        vehicles.borrow_mut().insert(
            vehicle,
            Vehicle {
                provider: Some(vh_provider),
                customer: vh_customer,
                agreement: Some(SIGNED_AGREEMENT_ID),
                public_key: vehicle_public_key,
                arch: String::from("amd64"),
                firmware: Vec::new(),
                telemetry: HashMap::from_iter(vec![(
                    TelemetryType::Gas,
                    HashMap::from_iter(vec![(
                        2023,
                        HashMap::from_iter(vec![(
                            time::Month::June as u8,
                            HashMap::from_iter(vec![(15, vec![96])]),
                        )]),
                    )]),
                )]),
                on_off: true,
                accumulated_telemetry: HashMap::from_iter(vec![(
                    TelemetryType::Gas,
                    HashMap::from_iter(vec![
                        (
                            2023,
                            AccumulatedTelemetryYearly {
                                value: 265,
                                monthly: HashMap::from_iter(vec![(
                                    7,
                                    AccumulatedTelemetryMonthy {
                                        value: 265,
                                        daily: HashMap::from_iter(vec![
                                            (1, 21),
                                            (2, 91),
                                            (4, 62),
                                            (5, 66),
                                            (6, 25),
                                        ]),
                                    },
                                )]),
                            },
                        ),
                        (
                            2024,
                            AccumulatedTelemetryYearly {
                                value: 744,
                                monthly: HashMap::from_iter(vec![
                                    (
                                        5,
                                        AccumulatedTelemetryMonthy {
                                            value: 104,
                                            daily: HashMap::from_iter(vec![]),
                                        },
                                    ),
                                    (
                                        6,
                                        AccumulatedTelemetryMonthy {
                                            value: 294,
                                            daily: HashMap::from_iter(vec![
                                                (2, 52),
                                                (5, 79),
                                                (9, 67),
                                                (12, 51),
                                                (15, 45),
                                            ]),
                                        },
                                    ),
                                    (
                                        7,
                                        AccumulatedTelemetryMonthy {
                                            value: 346,
                                            daily: HashMap::from_iter(vec![
                                                (1, 67),
                                                (2, 99),
                                                (4, 87),
                                                (5, 21),
                                                (6, 72),
                                                (9, 52),
                                                (12, 10),
                                                (15, 19),
                                                (20, 89),
                                            ]),
                                        },
                                    ),
                                ]),
                            },
                        ),
                    ]),
                )]),
                invoices: vec![PAID_VEHICLE_INVOICE_ID, UNPAID_VEHICLE_INVOICE_ID],
            },
        )
    });
}

fn create_invoice(
    vehicle_id: Principal,
    year: i32,
    month: u8,
    aggregated_data: &AccumulatedTelemetry,
) -> VTSResult<()> {
    let existing_invoice = INVOICES.with(|invoices| {
        let invoices = invoices.borrow();
        invoices.iter().any(|invoice| {
            invoice.1.vehicle == vehicle_id && invoice.1.period.0 == year && invoice.1.period.1 == month
        })
    });
    if existing_invoice {
        return Ok(());
    }

    let mut vehicle = VEHICLES
        .with(|vehicles| {
            let vehicles = vehicles.borrow();
            vehicles.get(&vehicle_id)
        })
        .ok_or(Error::NotFound)?;
    let agreement_id = vehicle.agreement.ok_or(Error::NotFound)?;
    let agreement_conditions = AGREEMENTS
        .with(|agreements| {
            let agreements = agreements.borrow();
            agreements.get(&agreement_id).map(|agreement| agreement.conditions)
        })
        .ok_or(Error::NotFound)?;
    let gas_price = Decimal::from_str(&agreement_conditions.gas_price).map_err(|_| Error::InvalidData)?;

    let mut total_cost = Decimal::new(0, 0);
    if let Some(aggregated_data) = aggregated_data.get(&TelemetryType::Gas) {
        for usage in aggregated_data.values().map(|v| Decimal::from_u128(v.value)) {
            let usage = usage.ok_or(Error::Internal)?;
            total_cost += usage * gas_price;
        }
    }

    let invoice_id = INVOICE_ID_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    });
    let invoice = Invoice {
        id: invoice_id,
        status: InvoiceStatus::Unpaid,
        vehicle: vehicle_id,
        agreement: agreement_id,
        period: (year, month),
        total_cost: total_cost.to_u128().ok_or(Error::Internal)?,
    };
    vehicle.invoices.push(invoice_id);

    INVOICES.with(|invoices| invoices.borrow_mut().insert(invoice_id, invoice));
    PENDING_INVOICES.with(|pending| pending.borrow_mut().insert(invoice_id, ()));
    VEHICLES.with(|vehicles| vehicles.borrow_mut().insert(vehicle_id, vehicle));

    Ok(())
}

fn prepare_pending_invoices(
    storage: &RefCell<StableBTreeMap<u128, (), Memory>>,
) -> VTSResult<Vec<PendingInvoice>> {
    let is_no_pending_invoices = storage.borrow().is_empty();
    if is_no_pending_invoices {
        return Ok(vec![]);
    }
    let mut pending_invoices_ids: Vec<u128> = Vec::new();
    let invoices = storage.borrow();
    for invoice in invoices.iter() {
        pending_invoices_ids.push(invoice.0);
    }
    ic_cdk::println!("there are {} pending invoices", pending_invoices_ids.len());
    let pending_invoices: Vec<PendingInvoice> =
        INVOICES.with(|invoices| -> VTSResult<Vec<PendingInvoice>> {
            let mut pending_invoices: Vec<PendingInvoice> = Vec::new();
            for pending_invoice_id in pending_invoices_ids {
                let invoice = invoices.borrow().get(&pending_invoice_id).ok_or(Error::NotFound)?;
                let vehicle = VEHICLES.with(|vehicles| -> VTSResult<Vehicle> {
                    vehicles.borrow().get(&invoice.vehicle).ok_or(Error::NotFound)
                })?;
                let customer: User = USERS.with(|users| -> VTSResult<User> {
                    users.borrow().get(&vehicle.customer).ok_or(Error::NotFound)
                })?;
                pending_invoices.push(PendingInvoice {
                    id: pending_invoice_id,
                    customer_email: customer.email,
                    vehicle: invoice.vehicle,
                });
            }
            Ok(pending_invoices)
        })?;
    Ok(pending_invoices)
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

fn is_gateway() -> Result<(), String> {
    // todo: implement it
    Ok(())
}

fn is_canister() -> Result<(), String> {
    // todo: implement it
    Ok(())
}

// Enable Candid export (see https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid)
ic_cdk::export_candid!();
