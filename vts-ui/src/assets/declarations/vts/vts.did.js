export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidSigner' : IDL.Null,
    'Internal' : IDL.Null,
    'NotFound' : IDL.Null,
    'AlreadyExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const UploadFirmwareRequest = IDL.Record({
    'principal' : IDL.Text,
    '_firmware' : IDL.Vec(IDL.Nat8),
    '_arch' : IDL.Text,
  });
  return IDL.Service({
    'create_agreement' : IDL.Func(
        [IDL.Text, IDL.Principal, IDL.Text, IDL.Text],
        [Result],
        [],
      ),
    'request_firmware' : IDL.Func([], [Result_1], []),
    'sign_agreement' : IDL.Func([IDL.Text], [Result_1], []),
    'upload_firmware' : IDL.Func([UploadFirmwareRequest], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
