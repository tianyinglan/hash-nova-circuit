use crate::{Prover, Verifier};

const TEST_INIT_VALUE: u64 = 100;
const TEST_STEP_NUM: usize = 4;

#[test]
fn test_recursive_poseidon_simple() {
    let prover = Prover::new(TEST_INIT_VALUE, TEST_STEP_NUM);
    let (snark, vk) = prover.prove().expect("Failed to prove");

    let verifier = Verifier::new(vk);
    let result = verifier.verify(TEST_INIT_VALUE, TEST_STEP_NUM, &snark);
    assert!(result, "Failed to verify");
}
