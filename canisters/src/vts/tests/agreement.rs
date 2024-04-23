use vts::{create_agreement, CanisterState, AgreementState};
use candid::Principal;

#[test]
fn test_create_agreement() {
    let name = "Test Agreement".to_string();
    let vh_provider = Principal::anonymous();
    let vh_customer = Principal::anonymous();
    let daily_usage_fee = 10.0;
    let gas_price = 2.0;


    let mut state = CanisterState::default();

    let agreement_id = create_agreement(&mut state, name.clone(), vh_provider.clone(), vh_customer.clone(), daily_usage_fee, gas_price);

    assert_eq!(agreement_id, 0);

    let agreement = state.agreements.get(&agreement_id).expect("Failed to get agreement from state");
    assert_eq!(agreement.name, name);
    assert_eq!(agreement.vh_provider, vh_provider);
    assert_eq!(agreement.vh_customer, vh_customer);
    assert_eq!(agreement.state, AgreementState::Unsigned);
    assert_eq!(agreement.conditions.get_daily_usage_fee(), daily_usage_fee);
    assert_eq!(agreement.conditions.get_gas_price(), gas_price);

    assert!(state.vh_providers.get(&vh_provider).unwrap().contains_key(&agreement_id));
    assert!(state.vh_customers.get(&vh_customer).unwrap().contains_key(&agreement_id));
}