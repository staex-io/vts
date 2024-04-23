use vts::{create_agreement, CanisterState, AgreementState};

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_create_agreement() {
        let mut state = CanisterState::default();
        let name = "Test Agreement".to_string();
        let vh_provider = Principal::anonymous();
        let vh_customer = Principal::anonymous();
        let daily_usage_fee = "10.0".to_string();
        let gas_price = "2.0".to_string();

        let agreement_id = create_agreement(&mut state, name.clone(), vh_provider.clone(), vh_customer.clone(), daily_usage_fee.clone(), gas_price.clone()).unwrap();

        assert_eq!(agreement_id, 0);
        assert_eq!(state.next_agreement_id, 1);

        let agreement = state.agreements.get(&agreement_id).expect("Failed to get agreement from state");
        assert_eq!(agreement.name, name);
        assert_eq!(agreement.vh_provider, vh_provider);
        assert_eq!(agreement.vh_customer, vh_customer);
        assert_eq!(agreement.state, AgreementState::Unsigned);
        assert_eq!(agreement.conditions.daily_usage_fee, Decimal::from_str(&daily_usage_fee).unwrap());
        assert_eq!(agreement.conditions.gas_price, Decimal::from_str(&gas_price).unwrap());

        assert!(state.vh_providers.get(&vh_provider).unwrap().contains_key(&agreement_id));
        assert!(state.vh_customers.get(&vh_customer).unwrap().contains_key(&agreement_id));
    }
}