use crate::{Blake3_192, Blake3_256, Example, ExampleOptions, HashFunction, Sha3_256};
use core::marker::PhantomData;
use log::debug;
use std::time::Instant;
use winterfell::{
    crypto::{DefaultRandomCoin, ElementHasher},
    math::{fields::f128::BaseElement, log2, FieldElement},
    ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};

mod air; 
use air::FactorialAir; 
mod utils;
use utils::compute_factorial_term; 
mod prover; 
use prover::FactorialProver; 

#[cfg(test)]
mod tests;

const TRACE_WIDTH: usize = 2; 

pub fn get_example(
    options: &ExampleOptions,
    sequence_length: usize,
) -> Result<Box<dyn Example>, String> {
    let (options, hash_fn) = options.to_proof_options(28, 8); 

    match hash_fn {
        HashFunction::Blake3_192 => Ok(Box::new(FactorialExample::<Blake3_192>::new(
            sequence_length,
            options,
        ))),
        HashFunction::Blake3_256 => Ok(Box::new(FactorialExample::<Blake3_256>::new(
            sequence_length,
            options,
        ))),
        HashFunction::Sha3_256 => Ok(Box::new(FactorialExample::<Sha3_256>::new(
            sequence_length,
            options,
        ))),
        _ => Err("The specified hash function cannot be used with this example.".to_string()),
    }
}

pub struct FactorialExample<H: ElementHasher> {
    options: ProofOptions,
    sequence_length: usize,
    result: BaseElement,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> FactorialExample<H> {
    pub fn new(sequence_length: usize, options: ProofOptions) -> Self {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        // compute the factorial sequence
        let now = Instant::now();
        let result = compute_factorial_term(sequence_length);
        debug!(
            "computed factorial of {} in {} ms",
            sequence_length,
            now.elapsed().as_millis()
        );

        Self {
            options,
            sequence_length,
            result,
            _hasher: PhantomData,
        }
    }
}

// Example implementation
// --------------------------------------------------------------------------------------------
impl<H: ElementHasher> Example for FactorialExample<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    fn prove(&self) -> StarkProof {
        debug!(
            "Generating proof for computing factorial sequence (1 term per step) up to {}th term\n\
            ---------------------",
            self.sequence_length
        );

        // create a prover
        let prover = FactorialProver::<H>::new(self.options.clone());

        // generate execution trace
        let now = Instant::now();
        let trace = prover.build_trace(self.sequence_length);

        let trace_width = trace.width();
        let trace_length = trace.length();
        debug!(
            "Generated execution trace of {} registers and 2^{} steps in {} ms",
            trace_width,
            log2(trace_length),
            now.elapsed().as_millis()
        );

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<FactorialAir, H, DefaultRandomCoin<H>>(proof, self.result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<FactorialAir, H, DefaultRandomCoin<H>>(proof, self.result + BaseElement::ONE)
    }
}
