import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'InvalidSigner' : null } |
  { 'Internal' : null } |
  { 'NotFound' : null } |
  { 'AlreadyExists' : null };
export type Result = { 'Ok' : bigint } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : Error };
export interface UploadFirmwareRequest {
  'principal' : string,
  '_firmware' : Uint8Array | number[],
  '_arch' : string,
}
export interface _SERVICE {
  'create_agreement' : ActorMethod<[string, Principal, string, string], Result>,
  'link_vehicle_to_agreement' : ActorMethod<[bigint, string], Result_1>,
  'request_firmware' : ActorMethod<[], Result_1>,
  'sign_agreement' : ActorMethod<[bigint], Result_1>,
  'upload_firmware' : ActorMethod<[UploadFirmwareRequest], Result_1>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
