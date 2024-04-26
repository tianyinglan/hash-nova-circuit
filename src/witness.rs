use anyhow::Result;
use ff::PrimeField;
use generic_array::typenum::U2;
use neptune::poseidon::{Poseidon, PoseidonConstants};

#[derive(Debug)]
pub struct PoseidonWitness<Scalar: PrimeField> {
    i: Scalar,
    v: Scalar,
    constants: PoseidonConstants<Scalar, U2>,
}

impl<Scalar: PrimeField> PoseidonWitness<Scalar> {
    pub fn new(i: u64, v: Scalar) -> Self {
        let i = i.into();
        let constants: PoseidonConstants<Scalar, U2> = PoseidonConstants::new();

        Self { i, v, constants }
    }

    // Calculate hash value for i and v.
    pub fn hash(&self) -> Result<Scalar> {
        let mut poseidon = Poseidon::<Scalar, U2>::new(&self.constants);

        poseidon.input(self.i)?;
        poseidon.input(self.v)?;

        Ok(poseidon.hash())
    }
}
