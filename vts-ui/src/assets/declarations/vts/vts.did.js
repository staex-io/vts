export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'Internal' : IDL.Null,
    'AlreadyExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const UploadFirmwareRequest = IDL.Record({
    'principal' : IDL.Text,
    '_firmware' : IDL.Vec(IDL.Nat8),
    '_arch' : IDL.Text,
  });
  return IDL.Service({
    'request_firmware' : IDL.Func([], [Result], []),
    'upload_firmware' : IDL.Func([UploadFirmwareRequest], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
