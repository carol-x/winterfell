use winterfell::math::{fields::f128::BaseElement, FieldElement};


pub fn compute_factorial_term<E: FieldElement>(n: usize) -> E {
    let mut t0 = E::ONE;
    let mut t1 = E::ONE;

    for _ in 0..(n - 1) {
        t0 = t0 + E::ONE;
        t1 = t0 * t1;
    }

    t1
}

#[test]
fn test_factorial() {
    let n = 10;
    let expected = compute_factorial_term::<BaseElement>(n);
    let actual = BaseElement::from(3628800u128);
    assert_eq!(expected, actual);
}


#[cfg(test)]
pub fn build_proof_options(use_extension_field: bool) -> winterfell::ProofOptions {
    use winterfell::{FieldExtension, ProofOptions};

    let extension = if use_extension_field {
        FieldExtension::Quadratic
    } else {
        FieldExtension::None
    };
    ProofOptions::new(28, 8, 0, extension, 4, 7)
}