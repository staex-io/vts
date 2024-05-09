export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidSigner' : IDL.Null,
    'Internal' : IDL.Null,
    'NotFound' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'AlreadyExists' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : Error });
  const User = IDL.Record({
    'agreements' : IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Null)),
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
  });
  const Result_3 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const AgreementState = IDL.Variant({
    'Unsigned' : IDL.Null,
    'Signed' : IDL.Null,
  });
  const AgreementConditions = IDL.Record({
    'daily_usage_fee' : IDL.Text,
    'gas_price' : IDL.Text,
  });
  const Agreement = IDL.Record({
    'id' : IDL.Nat,
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'name' : IDL.Text,
    'state' : AgreementState,
    'conditions' : AgreementConditions,
    'vh_provider' : IDL.Principal,
    'vh_customer' : IDL.Principal,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Vec(Agreement), 'Err' : Error });
  const Vehicle = IDL.Record({
    'owner' : IDL.Principal,
    'arch' : IDL.Text,
    'agreement' : IDL.Opt(IDL.Nat),
    'firmware' : IDL.Vec(IDL.Nat8),
    'identity' : IDL.Principal,
  });
  const Result_5 = IDL.Variant({ 'Ok' : Vehicle, 'Err' : Error });
  const Result_6 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'Err' : Error,
  });
  return IDL.Service({
    'add_admin' : IDL.Func([IDL.Principal], [Result], []),
    'add_allowed_key' : IDL.Func([IDL.Principal], [Result], []),
    'create_agreement' : IDL.Func(
        [IDL.Text, IDL.Principal, IDL.Text, IDL.Text],
        [Result_1],
        [],
      ),
    'delete_admin' : IDL.Func([IDL.Principal], [Result], []),
    'delete_user' : IDL.Func([IDL.Principal], [Result], []),
    'get_firmware_requests' : IDL.Func([], [Result_2], ['query']),
    'get_firmware_requests_by_user' : IDL.Func([], [Result], ['query']),
    'get_user' : IDL.Func([], [Result_3], ['query']),
    'get_user_agreements' : IDL.Func([], [Result_4], ['query']),
    'get_vehicle' : IDL.Func([IDL.Principal], [Result_5], ['query']),
    'get_vehicles_by_agreement' : IDL.Func([IDL.Nat], [Result_6], ['query']),
    'link_vehicle' : IDL.Func([IDL.Nat, IDL.Principal], [Result], []),
    'register_user' : IDL.Func([IDL.Principal], [Result], []),
    'remove_allowed_key' : IDL.Func([IDL.Principal], [Result], []),
    'request_firmware' : IDL.Func([], [Result], []),
    'sign_agreement' : IDL.Func([IDL.Nat], [Result], []),
    'upload_firmware' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
