use crate::types::{
    CompressedSnark, PoseidonScalar, TrivialScalar, VerifierKey,
};

/// Verifier for recursive hash chain
pub struct Verifier {
    vk: VerifierKey,
}

impl Verifier {
    pub fn new(vk: VerifierKey) -> Self {
        Self { vk }
    }

    // Verify the snark.
    pub fn verify(
        &self,
        init_value: u64,
        step_num: usize,
        snark: &CompressedSnark,
    ) -> bool {
        // Create init inputs.
        let poseidon_z0 = vec![PoseidonScalar::one(), init_value.into()];
        let trivial_z0 = vec![TrivialScalar::zero()];

        // Verify the snark.
        // Increase step number by 1, since there is a last step to check the
        // final hash value.
        snark
            .verify(&self.vk, step_num + 1, &poseidon_z0, &trivial_z0)
            .is_ok()
    }
}
