use bellpepper_core::{num::AllocatedNum, ConstraintSystem, SynthesisError};
use ff::PrimeField;
use generic_array::typenum::U2;
use neptune::{
    circuit::poseidon_hash_circuit, circuit::CircuitType,
    poseidon::PoseidonConstants,
};
use nova_snark::traits::circuit::StepCircuit;

/// Poseidon circuit for recursive hash chain
#[derive(Clone, Debug, Default)]
pub struct PoseidonCircuit<Scalar: PrimeField> {
    /// Should be 1 if last step, 0 otherwise. Only contrain final hash value in
    /// last step.
    is_final: Scalar,
    /// Step index
    i: Scalar,
    /// Step value, should be hash value except the init one.
    v: Scalar,
    /// Poseidon constants
    constants: PoseidonConstants<Scalar, U2>,
}

impl<Scalar: PrimeField> StepCircuit<Scalar> for PoseidonCircuit<Scalar> {
    fn arity(&self) -> usize {
        2
    }

    fn synthesize<CS: ConstraintSystem<Scalar>>(
        &self,
        cs: &mut CS,
        z: &[AllocatedNum<Scalar>],
    ) -> Result<Vec<AllocatedNum<Scalar>>, SynthesisError> {
        debug_assert_eq!(z.len(), 2, "Must have inputs of i and v");

        // Get i and i_next.
        let one = AllocatedNum::alloc(cs.namespace(|| "1"), || Ok(1.into()))?;
        let i = AllocatedNum::alloc(cs.namespace(|| "i"), || Ok(self.i))?;
        let i_next = i.add(cs.namespace(|| "i_next = i + 1"), &one)?;
        let i = i.get_variable();

        // Get is_final and v.
        let is_final =
            AllocatedNum::alloc(cs.namespace(|| "is_final"), || {
                Ok(self.is_final)
            })?
            .get_variable();
        let v = AllocatedNum::alloc(cs.namespace(|| "v"), || Ok(self.v))?
            .get_variable();

        // Add contraints.
        cs.enforce(
            || "is_final must be a boolean",
            |lc| lc + CS::one() - is_final,
            |lc| lc + is_final,
            |lc| lc,
        );
        cs.enforce(
            || "(i - z[0]) * (1 - is_final) == 0",
            |lc| lc + i - z[0].get_variable(),
            |lc| lc + CS::one() - is_final,
            |lc| lc,
        );
        cs.enforce(
            || "v == z[1]",
            |lc| lc + v - z[1].get_variable(),
            |lc| lc + CS::one(),
            |lc| lc,
        );

        // Get hash output.
        // TODO: may skip for the last step.
        let output = poseidon_hash_circuit(
            cs,
            CircuitType::Legacy,
            z.to_vec(),
            &self.constants,
        )?;

        Ok(vec![i_next, output])
    }
}

impl<Scalar: PrimeField> PoseidonCircuit<Scalar> {
    pub fn new(is_final: bool, i: u64, v: Scalar) -> Self {
        let is_final = if is_final { 1.into() } else { 0.into() };
        let i = i.into();
        let constants: PoseidonConstants<Scalar, U2> = PoseidonConstants::new();

        Self {
            is_final,
            i,
            v,
            constants,
        }
    }
}
