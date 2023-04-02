use super::{utils::build_proof_options, Blake3_256};

#[test]
fn factorial_test_basic_proof_verification() {
    let factorial = Box::new(super::FactorialExample::<Blake3_256>::new(
        16,
        build_proof_options(false),
    ));
    crate::tests::test_basic_proof_verification(factorial);
}

#[test]
fn factorial_test_basic_proof_verification_extension() {
    let factorial = Box::new(super::FactorialExample::<Blake3_256>::new(
        16,
        build_proof_options(true),
    ));
    crate::tests::test_basic_proof_verification(factorial);
}

#[test]
fn factorial_test_basic_proof_verification_fail() {
    let factorial = Box::new(super::FactorialExample::<Blake3_256>::new(
        16,
        build_proof_options(false),
    ));
    crate::tests::test_basic_proof_verification_fail(factorial);
}
