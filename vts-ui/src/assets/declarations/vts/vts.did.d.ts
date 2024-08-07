import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AccumulatedTelemetryMonthy {
  'value' : bigint,
  'daily' : Array<[number, bigint]>,
}
export interface AccumulatedTelemetryYearly {
  'value' : bigint,
  'monthly' : Array<[number, AccumulatedTelemetryMonthy]>,
}
export interface Agreement {
  'id' : bigint,
  'vehicles' : Array<[Principal, null]>,
  'name' : string,
  'state' : AgreementState,
  'conditions' : AgreementConditions,
  'vh_provider' : Principal,
  'vh_customer' : Principal,
}
export interface AgreementConditions { 'gas_price' : string }
export type AgreementState = { 'Unsigned' : null } |
  { 'Signed' : null };
export type Error = { 'InvalidSigner' : null } |
  { 'Internal' : null } |
  { 'InvalidSignatureFormat' : null } |
  { 'InvalidSignature' : null } |
  { 'NotFound' : null } |
  { 'InvalidData' : null } |
  { 'Unauthorized' : null } |
  { 'AlreadyExists' : null } |
  { 'DecodeTelemetry' : null };
export interface Invoice {
  'id' : bigint,
  'status' : InvoiceStatus,
  'period' : [number, number],
  'agreement' : bigint,
  'total_cost' : bigint,
  'vehicle' : Principal,
}
export type InvoiceStatus = { 'Paid' : null } |
  { 'Unpaid' : null };
export interface PendingInvoice {
  'id' : bigint,
  'vehicle' : Principal,
  'customer_email' : [] | [string],
}
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : Error };
export type Result_10 = { 'Ok' : StoreTelemetryResponse } |
  { 'Err' : Error };
export type Result_2 = {
    'Ok' : Array<[TelemetryType, Array<[number, AccumulatedTelemetryYearly]>]>
  } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : Principal } |
  { 'Err' : Error };
export type Result_4 = { 'Ok' : Invoice } |
  { 'Err' : Error };
export type Result_5 = { 'Ok' : Array<PendingInvoice> } |
  { 'Err' : Error };
export type Result_6 = { 'Ok' : User } |
  { 'Err' : Error };
export type Result_7 = { 'Ok' : Array<Agreement> } |
  { 'Err' : Error };
export type Result_8 = { 'Ok' : Vehicle } |
  { 'Err' : Error };
export type Result_9 = { 'Ok' : Array<[Principal, null]> } |
  { 'Err' : Error };
export type StoreTelemetryResponse = { 'On' : null } |
  { 'Off' : null };
export type TelemetryType = { 'Gas' : null };
export interface User {
  'agreements' : Array<[bigint, null]>,
  'vehicles' : Array<[Principal, null]>,
  'email' : [] | [string],
}
export interface Vehicle {
  'telemetry' : Array<
    [
      TelemetryType,
      Array<[number, Array<[number, Array<[number, Array<bigint>]>]>]>,
    ]
  >,
  'provider' : [] | [Principal],
  'customer' : Principal,
  'public_key' : Uint8Array | number[],
  'arch' : string,
  'agreement' : [] | [bigint],
  'firmware' : Uint8Array | number[],
  'accumulated_telemetry' : Array<
    [TelemetryType, Array<[number, AccumulatedTelemetryYearly]>]
  >,
  'invoices' : Array<bigint>,
  'on_off' : boolean,
}
export interface _SERVICE {
  'accumulate_telemetry_data' : ActorMethod<[], Result>,
  'add_admin' : ActorMethod<[Principal], Result>,
  'clean_state' : ActorMethod<[], undefined>,
  'create_agreement' : ActorMethod<[string, Principal, string], Result_1>,
  'delete_admin' : ActorMethod<[Principal], Result>,
  'delete_paid_invoices' : ActorMethod<[Array<bigint>], undefined>,
  'delete_pending_invoices' : ActorMethod<[Array<bigint>], undefined>,
  'delete_user' : ActorMethod<[Principal], Result>,
  'fill_predefined_telemetry' : ActorMethod<
    [Principal, Principal, string],
    undefined
  >,
  'get_aggregated_data' : ActorMethod<[Principal], Result_2>,
  'get_firmware_requests' : ActorMethod<[], Result_3>,
  'get_firmware_requests_by_user' : ActorMethod<[], Result>,
  'get_invoice' : ActorMethod<[bigint], Result_4>,
  'get_paid_invoices' : ActorMethod<[], Result_5>,
  'get_pending_invoices' : ActorMethod<[], Result_5>,
  'get_user' : ActorMethod<[], Result_6>,
  'get_user_agreements' : ActorMethod<[], Result_7>,
  'get_vehicle' : ActorMethod<[Principal], Result_8>,
  'get_vehicles_by_agreement' : ActorMethod<[bigint], Result_9>,
  'link_vehicle' : ActorMethod<[bigint, Principal], Result>,
  'pay_for_invoice' : ActorMethod<[bigint], Result>,
  'register_user' : ActorMethod<[Principal, [] | [string]], Result>,
  'request_firmware' : ActorMethod<[], Result>,
  'sign_agreement' : ActorMethod<[bigint], Result>,
  'store_telemetry' : ActorMethod<
    [Principal, Uint8Array | number[], Uint8Array | number[]],
    Result_10
  >,
  'turn_on_off_vehicle' : ActorMethod<[Principal, boolean], Result>,
  'upload_firmware' : ActorMethod<
    [Principal, Uint8Array | number[], string, Uint8Array | number[]],
    Result
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
