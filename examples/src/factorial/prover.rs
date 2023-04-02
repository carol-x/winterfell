use super::{
    BaseElement, DefaultRandomCoin, ElementHasher, FactorialAir, FieldElement, PhantomData, ProofOptions,
    Prover, Trace, TraceTable, TRACE_WIDTH,
};

pub struct FactorialProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> FactorialProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    /// Builds an execution trace for computing a Fibonacci sequence of the specified length such
    /// that each row advances the sequence by 2 terms.
    pub fn build_trace(&self, sequence_length: usize) -> TraceTable<BaseElement> {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        let mut trace = TraceTable::new(TRACE_WIDTH, sequence_length / 2);
        trace.fill(
            |state| {
                state[0] = BaseElement::ONE;
                state[1] = BaseElement::ONE;
            },
            |_, state| {
                state[0] += BaseElement::ONE;
                state[1] = state[0] * state[1];
            },
        );

        trace
    }
}

impl<H: ElementHasher> Prover for FactorialProver<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    type BaseField = BaseElement;
    type Air = FactorialAir;
    type Trace = TraceTable<BaseElement>;
    type HashFn = H;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        let last_step = trace.length() - 1;
        trace.get(1, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
