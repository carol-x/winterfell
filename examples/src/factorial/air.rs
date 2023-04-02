// Carol's  test 

use super::{BaseElement, FieldElement, ProofOptions, TRACE_WIDTH};
use crate::utils::are_equal;
use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame, TraceInfo, TransitionConstraintDegree,
};

pub struct FactorialAir {
    context: AirContext<BaseElement>, 
    result: BaseElement, 
}

impl Air for FactorialAir {
    type BaseField = BaseElement; 
    type PublicInputs = BaseElement; 

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(trace_info: TraceInfo, pub_inputs: Self::BaseField, options: ProofOptions) -> Self {
        let degree = vec![
            TransitionConstraintDegree::new(1), // keep track of the additive counter
            TransitionConstraintDegree::new(2), // keep track of the actual multiplication 
        ]; 
        assert_eq!(TRACE_WIDTH, trace_info.width());
        FactorialAir {
            context: AirContext::new(trace_info, degree, 3, options), 
            result: pub_inputs, 
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E], 
    ) {
        let current = frame.current();
        let next = frame.next();
        // expected state width is 3 field elements
        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        // constraints of factorial with 3 registers
        result[0] = are_equal(next[0], current[0] + E::ONE);
        result[1] = are_equal(next[1], current[1] * next[0]);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1; 
        vec![
            Assertion::single(0, 0, Self::BaseField::ONE), 
            Assertion::single(1, 0, Self::BaseField::ONE), 
            Assertion::single(1, last_step, self.result),
            ]
    }
}
