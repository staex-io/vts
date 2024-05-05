import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Agreement {
  'id' : bigint,
  'vehicles' : Array<[Principal, null]>,
  'name' : string,
  'state' : AgreementState,
  'conditions' : AgreementConditions,
  'vh_provider' : Principal,
  'vh_customer' : Principal,
}
export interface AgreementConditions {
  'daily_usage_fee' : string,
  'gas_price' : string,
}
export type AgreementState = { 'Unsigned' : null } |
  { 'Signed' : null };
export type Error = { 'InvalidSigner' : null } |
  { 'Internal' : null } |
  { 'NotFound' : null } |
  { 'AlreadyExists' : null };
export type Result = { 'Ok' : bigint } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Principal } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : Error };
export type Result_3 = { 'Ok' : User } |
  { 'Err' : Error };
export type Result_4 = { 'Ok' : Array<Agreement> } |
  { 'Err' : Error };
export type Result_5 = { 'Ok' : Vehicle } |
  { 'Err' : Error };
export type Result_6 = { 'Ok' : Array<[Principal, null]> } |
  { 'Err' : Error };
export interface User {
  'agreements' : Array<[bigint, null]>,
  'vehicles' : Array<[Principal, null]>,
}
export interface Vehicle {
  'owner' : Principal,
  'arch' : string,
  'firmware' : Uint8Array | number[],
  'identity' : Principal,
}
export interface _SERVICE {
  'create_agreement' : ActorMethod<[string, Principal, string, string], Result>,
  'get_firmware_requests' : ActorMethod<[], Result_1>,
  'get_firmware_requests_by_user' : ActorMethod<[], Result_2>,
  'get_user' : ActorMethod<[], Result_3>,
  'get_user_agreements' : ActorMethod<[], Result_4>,
  'get_vehicle' : ActorMethod<[Principal], Result_5>,
  'get_vehicles_by_agreement' : ActorMethod<[bigint], Result_6>,
  'link_vehicle' : ActorMethod<[bigint, Principal], Result_2>,
  'request_firmware' : ActorMethod<[], Result_2>,
  'sign_agreement' : ActorMethod<[bigint], Result_2>,
  'upload_firmware' : ActorMethod<
    [Principal, Principal, string, Uint8Array | number[]],
    Result_2
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
