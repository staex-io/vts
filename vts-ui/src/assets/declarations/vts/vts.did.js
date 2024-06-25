export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidSigner' : IDL.Null,
    'Internal' : IDL.Null,
    'InvalidSignatureFormat' : IDL.Null,
    'InvalidSignature' : IDL.Null,
    'NotFound' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'AlreadyExists' : IDL.Null,
    'DecodeTelemetry' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  const TelemetryType = IDL.Variant({ 'Gas' : IDL.Null });
  const AggregatedData = IDL.Record({
    'monthly' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
    'yearly' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
    'daily' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat32)),
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(TelemetryType, AggregatedData)),
    'Err' : Error,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : Error });
  const User = IDL.Record({
    'agreements' : IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Null)),
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'email' : IDL.Opt(IDL.Text),
  });
  const Result_4 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const AgreementState = IDL.Variant({
    'Unsigned' : IDL.Null,
    'Signed' : IDL.Null,
  });
  const AgreementConditions = IDL.Record({ 'gas_price' : IDL.Text });
  const Agreement = IDL.Record({
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'name' : IDL.Text,
    'state' : AgreementState,
    'conditions' : AgreementConditions,
    'vh_provider' : IDL.Principal,
    'vh_customer' : IDL.Principal,
  });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Vec(Agreement), 'Err' : Error });
  const AggregationInterval = IDL.Variant({
    'Daily' : IDL.Null,
    'Monthly' : IDL.Null,
    'Yearly' : IDL.Null,
  });
  const Vehicle = IDL.Record({
    'telemetry' : IDL.Vec(
      IDL.Tuple(
        TelemetryType,
        IDL.Vec(
          IDL.Tuple(
            IDL.Int32,
            IDL.Vec(
              IDL.Tuple(
                IDL.Nat8,
                IDL.Vec(IDL.Tuple(IDL.Nat8, IDL.Vec(IDL.Nat))),
              )
            ),
          )
        ),
      )
    ),
    'public_key' : IDL.Vec(IDL.Nat8),
    'owner' : IDL.Principal,
    'arch' : IDL.Text,
    'agreement' : IDL.Opt(IDL.Nat),
    'firmware' : IDL.Vec(IDL.Nat8),
    'accumulated_telemetry' : IDL.Vec(
      IDL.Tuple(
        TelemetryType,
        IDL.Vec(IDL.Tuple(AggregationInterval, AggregatedData)),
      )
    ),
  });
  const Result_6 = IDL.Variant({ 'Ok' : Vehicle, 'Err' : Error });
  const Result_7 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'Err' : Error,
  });
  const StoreTelemetryResponse = IDL.Variant({
    'On' : IDL.Null,
    'Off' : IDL.Null,
  });
  const Result_8 = IDL.Variant({
    'Ok' : StoreTelemetryResponse,
    'Err' : Error,
  });
  return IDL.Service({
    'accumulate_telemetry_data' : IDL.Func([], [Result], []),
    'accumulate_telemetry_data_now' : IDL.Func([], [Result], []),
    'add_admin' : IDL.Func([IDL.Principal], [Result], []),
    'clean_state' : IDL.Func([], [], []),
    'create_agreement' : IDL.Func(
        [IDL.Text, IDL.Principal, IDL.Text],
        [Result_1],
        [],
      ),
    'delete_admin' : IDL.Func([IDL.Principal], [Result], []),
    'delete_pending_invoices' : IDL.Func([IDL.Vec(IDL.Nat)], [], []),
    'delete_user' : IDL.Func([IDL.Principal], [Result], []),
    'fill_predefined_telemetry' : IDL.Func([], [], []),
    'get_aggregated_data' : IDL.Func([IDL.Principal], [Result_2], []),
    'get_firmware_requests' : IDL.Func([], [Result_3], ['query']),
    'get_firmware_requests_by_user' : IDL.Func([], [Result], ['query']),
    'get_user' : IDL.Func([], [Result_4], ['query']),
    'get_user_agreements' : IDL.Func([], [Result_5], ['query']),
    'get_vehicle' : IDL.Func([IDL.Principal], [Result_6], ['query']),
    'get_vehicles_by_agreement' : IDL.Func([IDL.Nat], [Result_7], ['query']),
    'link_vehicle' : IDL.Func([IDL.Nat, IDL.Principal], [Result], []),
    'register_user' : IDL.Func(
        [IDL.Principal, IDL.Opt(IDL.Text)],
        [Result],
        [],
      ),
    'request_firmware' : IDL.Func([], [Result], []),
    'sign_agreement' : IDL.Func([IDL.Nat], [Result], []),
    'store_telemetry' : IDL.Func(
        [IDL.Principal, IDL.Vec(IDL.Nat8), IDL.Vec(IDL.Nat8)],
        [Result_8],
        [],
      ),
    'upload_firmware' : IDL.Func(
        [IDL.Principal, IDL.Vec(IDL.Nat8), IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
