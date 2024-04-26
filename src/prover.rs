use crate::{
    types::{
        CompressedSnark, PoseidonCircuit, PoseidonScalar, PoseidonSnark,
        PublicParams, RecursiveSnark, TrivialCircuit, TrivialScalar,
        TrivialSnark, VerifierKey,
    },
    witness::PoseidonWitness,
};
use anyhow::Result;
use nova_snark::traits::snark::RelaxedR1CSSNARKTrait;

/// Prover for recursive hash chain
#[derive(Debug)]
pub struct Prover {
    init_value: u64,
    step_num: usize,
}

impl Prover {
    pub fn new(init_value: u64, step_num: usize) -> Self {
        Self {
            init_value,
            step_num,
        }
    }

    // Prove recursive hash chain to generate snark and VK.
    pub fn prove(&self) -> Result<(CompressedSnark, VerifierKey)> {
        let trivial_circuit = TrivialCircuit::default();
        let public_params = public_params(&trivial_circuit);

        // Generate recursive snark.
        let recursive_snark =
            self.gen_recursive_snark(&trivial_circuit, &public_params)?;

        // Get PK and VK.
        let (pk, vk) = CompressedSnark::setup(&public_params)?;

        // Generate compressed snark.
        let snark =
            CompressedSnark::prove(&public_params, &pk, &recursive_snark)?;

        Ok((snark, vk))
    }

    fn gen_recursive_snark(
        &self,
        trivial_circuit: &TrivialCircuit,
        public_params: &PublicParams,
    ) -> Result<RecursiveSnark> {
        // Create init inputs.
        let poseidon_z0 = vec![PoseidonScalar::one(), self.init_value.into()];
        let trivial_z0 = vec![TrivialScalar::zero()];

        // Construct recursive poseidon circuits.
        let mut v = poseidon_z0[1];
        let mut poseidon_circuits = vec![];
        for i in 1..=self.step_num as u64 {
            // Set is_final to false for non last step.
            let circuit = PoseidonCircuit::new(false, i, v);
            poseidon_circuits.push(circuit);

            v = PoseidonWitness::new(i, v).hash()?;
        }

        // Add one more and set is_final to true for checking the final hash
        // value. So it has `self.step_num + 1` steps in real.
        let final_circuit = PoseidonCircuit::new(true, 0, v);
        poseidon_circuits.push(final_circuit);

        // Create recursive snark.
        let mut recursive_snark = RecursiveSnark::new(
            public_params,
            &poseidon_circuits[0],
            trivial_circuit,
            &poseidon_z0,
            &trivial_z0,
        )?;

        // Prove for each poseidon step.
        for poseidon_circuit in poseidon_circuits.iter() {
            recursive_snark.prove_step(
                public_params,
                poseidon_circuit,
                trivial_circuit,
            )?;
        }

        // Verify this recursive snark only in debug.
        debug_assert!(
            recursive_snark
                .verify(
                    public_params,
                    self.step_num + 1,
                    &poseidon_z0,
                    &trivial_z0,
                )
                .is_ok()
        );

        Ok(recursive_snark)
    }
}

fn public_params(trivial_circuit: &TrivialCircuit) -> PublicParams {
    PublicParams::setup(
        &PoseidonCircuit::default(),
        trivial_circuit,
        &*PoseidonSnark::ck_floor(),
        &*TrivialSnark::ck_floor(),
    )
}
