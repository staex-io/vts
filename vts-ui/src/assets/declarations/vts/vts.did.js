export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidSigner' : IDL.Null,
    'Internal' : IDL.Null,
    'NotFound' : IDL.Null,
    'AlreadyExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const User = IDL.Record({
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
  });
  const Result_3 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const Result_4 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'Err' : Error,
  });
  return IDL.Service({
    'create_agreement' : IDL.Func(
        [IDL.Text, IDL.Principal, IDL.Text, IDL.Text],
        [Result],
        [],
      ),
    'get_firmware_requests' : IDL.Func([], [Result_1], ['query']),
    'get_firmware_requests_by_user' : IDL.Func([], [Result_2], ['query']),
    'get_user' : IDL.Func([], [Result_3], ['query']),
    'get_vehicles_by_agreement' : IDL.Func([IDL.Nat], [Result_4], ['query']),
    'link_vehicle' : IDL.Func([IDL.Nat, IDL.Principal], [Result_2], []),
    'request_firmware' : IDL.Func([], [Result_2], []),
    'sign_agreement' : IDL.Func([IDL.Nat], [Result_2], []),
    'upload_firmware' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result_2],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
