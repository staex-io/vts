import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'Internal' : null } |
  { 'AlreadyExists' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : Error };
export interface UploadFirmwareRequest {
  'principal' : string,
  '_firmware' : Uint8Array | number[],
  '_arch' : string,
}
export interface _SERVICE {
  'request_firmware' : ActorMethod<[], Result>,
  'upload_firmware' : ActorMethod<[UploadFirmwareRequest], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
