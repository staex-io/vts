export const idlFactory = ({ IDL }) => {
  const Error = IDL.Variant({
    'InvalidSigner' : IDL.Null,
    'Internal' : IDL.Null,
    'InvalidSignatureFormat' : IDL.Null,
    'InvalidSignature' : IDL.Null,
    'NotFound' : IDL.Null,
    'InvalidData' : IDL.Null,
    'Unauthorized' : IDL.Null,
    'AlreadyExists' : IDL.Null,
    'DecodeTelemetry' : IDL.Null,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : Error });
  const TelemetryType = IDL.Variant({ 'Gas' : IDL.Null });
  const AccumulatedTelemetryMonthy = IDL.Record({
    'value' : IDL.Nat,
    'daily' : IDL.Vec(IDL.Tuple(IDL.Nat8, IDL.Nat)),
  });
  const AccumulatedTelemetryYearly = IDL.Record({
    'value' : IDL.Nat,
    'monthly' : IDL.Vec(IDL.Tuple(IDL.Nat8, AccumulatedTelemetryMonthy)),
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(
      IDL.Tuple(
        TelemetryType,
        IDL.Vec(IDL.Tuple(IDL.Int32, AccumulatedTelemetryYearly)),
      )
    ),
    'Err' : Error,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : Error });
  const Invoice = IDL.Record({
    'id' : IDL.Nat,
    'period' : IDL.Tuple(IDL.Text, IDL.Text),
    'total_cost' : IDL.Nat64,
    'vehicle' : IDL.Principal,
  });
  const Result_4 = IDL.Variant({ 'Ok' : Invoice, 'Err' : Error });
  const PendingInvoice = IDL.Record({
    'id' : IDL.Nat,
    'vehicle' : IDL.Principal,
    'customer_email' : IDL.Opt(IDL.Text),
  });
  const Result_5 = IDL.Variant({
    'Ok' : IDL.Vec(PendingInvoice),
    'Err' : Error,
  });
  const User = IDL.Record({
    'agreements' : IDL.Vec(IDL.Tuple(IDL.Nat, IDL.Null)),
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'email' : IDL.Opt(IDL.Text),
  });
  const Result_6 = IDL.Variant({ 'Ok' : User, 'Err' : Error });
  const AgreementState = IDL.Variant({
    'Unsigned' : IDL.Null,
    'Signed' : IDL.Null,
  });
  const AgreementConditions = IDL.Record({ 'gas_price' : IDL.Text });
  const Agreement = IDL.Record({
    'id' : IDL.Nat,
    'vehicles' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'name' : IDL.Text,
    'state' : AgreementState,
    'conditions' : AgreementConditions,
    'vh_provider' : IDL.Principal,
    'vh_customer' : IDL.Principal,
  });
  const Result_7 = IDL.Variant({ 'Ok' : IDL.Vec(Agreement), 'Err' : Error });
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
        IDL.Vec(IDL.Tuple(IDL.Int32, AccumulatedTelemetryYearly)),
      )
    ),
    'on_off' : IDL.Bool,
  });
  const Result_8 = IDL.Variant({ 'Ok' : Vehicle, 'Err' : Error });
  const Result_9 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Null)),
    'Err' : Error,
  });
  const StoreTelemetryResponse = IDL.Variant({
    'On' : IDL.Null,
    'Off' : IDL.Null,
  });
  const Result_10 = IDL.Variant({
    'Ok' : StoreTelemetryResponse,
    'Err' : Error,
  });
  return IDL.Service({
    'accumulate_telemetry_data' : IDL.Func([], [Result], []),
    'add_admin' : IDL.Func([IDL.Principal], [Result], []),
    'clean_state' : IDL.Func([], [], []),
    'create_agreement' : IDL.Func(
        [IDL.Text, IDL.Principal, IDL.Text],
        [Result_1],
        [],
      ),
    'delete_admin' : IDL.Func([IDL.Principal], [Result], []),
    'delete_paid_invoices' : IDL.Func([IDL.Vec(IDL.Nat)], [], []),
    'delete_pending_invoices' : IDL.Func([IDL.Vec(IDL.Nat)], [], []),
    'delete_user' : IDL.Func([IDL.Principal], [Result], []),
    'fill_predefined_telemetry' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Text],
        [],
        [],
      ),
    'get_aggregated_data' : IDL.Func([IDL.Principal], [Result_2], ['query']),
    'get_firmware_requests' : IDL.Func([], [Result_3], ['query']),
    'get_firmware_requests_by_user' : IDL.Func([], [Result], ['query']),
    'get_invoice' : IDL.Func([IDL.Nat], [Result_4], []),
    'get_paid_invoices' : IDL.Func([], [Result_5], ['query']),
    'get_pending_invoices' : IDL.Func([], [Result_5], ['query']),
    'get_user' : IDL.Func([], [Result_6], ['query']),
    'get_user_agreements' : IDL.Func([], [Result_7], ['query']),
    'get_vehicle' : IDL.Func([IDL.Principal], [Result_8], ['query']),
    'get_vehicles_by_agreement' : IDL.Func([IDL.Nat], [Result_9], ['query']),
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
        [Result_10],
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
