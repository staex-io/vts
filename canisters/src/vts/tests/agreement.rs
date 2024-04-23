use vts::{create_agreement, CanisterState, AgreementState}; // Ensure correct import paths
use candid::Principal;

// Mock the caller environment if necessary (adjust based on your setup and testing framework)
fn setup() -> CanisterState {
    CanisterState::default()
}

#[test]
fn test_agreement_creation_success() {
    let state = setup();
    let principal_provider = Principal::anonymous();
    let principal_customer = Principal::anonymous();

    let agreement_id = create_agreement(
        "New Agreement".to_string(),
        principal_provider,
        principal_customer,
        150.50,
        10.00
    );

    assert!(state.agreements.contains_key(&agreement_id), "Agreement was not added.");
    let agreement = state.agreements.get(&agreement_id).expect("Failed to retrieve agreement.");
    assert_eq!(agreement.name, "New Agreement");
    assert!(matches!(agreement.state, AgreementState::Unsigned));
}

#[test]
fn test_agreement_creation_failure() {
    let _state = setup();
    let result = std::panic::catch_unwind(|| {
        create_agreement(
            "Faulty Agreement".to_string(),
            Principal::anonymous(),
            Principal::anonymous(),
            150.50,
            10.00
        );
    });

    assert!(result.is_err(), "Expected a panic due to invalid float parsing");
}
