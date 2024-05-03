export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidSigner' : IDL.Null,
    'Internal' : IDL.Null,
    'NotFound' : IDL.Null,
    'AlreadyExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Principal),
    'Err' : Error,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
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
    'get_vehicles_by_agreement' : IDL.Func([IDL.Nat], [Result_1], ['query']),
    'link_vehicle_to_agreement' : IDL.Func(
        [IDL.Nat, IDL.Principal],
        [Result_2],
        [],
      ),
    'request_firmware' : IDL.Func([], [Result_2], []),
    'sign_agreement' : IDL.Func([IDL.Nat], [Result_2], []),
    'upload_firmware' : IDL.Func([UploadFirmwareRequest], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
