import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'InvalidSigner' : null } |
  { 'Internal' : null } |
  { 'NotFound' : null } |
  { 'AlreadyExists' : null };
export type Result = { 'Ok' : bigint } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Array<Principal> } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : Error };
export interface UploadFirmwareRequest {
  'principal' : string,
  '_firmware' : Uint8Array | number[],
  '_arch' : string,
}
export interface _SERVICE {
  'create_agreement' : ActorMethod<[string, Principal, string, string], Result>,
  'get_vehicles_by_agreement' : ActorMethod<[bigint], Result_1>,
  'link_vehicle_to_agreement' : ActorMethod<[bigint, Principal], Result_2>,
  'request_firmware' : ActorMethod<[], Result_2>,
  'sign_agreement' : ActorMethod<[bigint], Result_2>,
  'upload_firmware' : ActorMethod<[UploadFirmwareRequest], Result_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
