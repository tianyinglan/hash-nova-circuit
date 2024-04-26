use nova_snark::{
    provider::{
        ipa_pc,
        mlkzg::{self, Bn256EngineKZG},
        GrumpkinEngine,
    },
    spartan::snark::RelaxedR1CSSNARK,
    traits::Engine,
    CompressedSNARK, RecursiveSNARK,
};

type PoseidonEngine = Bn256EngineKZG;
type TrivialEngine = GrumpkinEngine;

pub(crate) type PoseidonScalar = <PoseidonEngine as Engine>::Scalar;
pub(crate) type TrivialScalar = <TrivialEngine as Engine>::Scalar;

pub(crate) type PoseidonCircuit =
    crate::circuit::PoseidonCircuit<PoseidonScalar>;
pub(crate) type TrivialCircuit =
    nova_snark::traits::circuit::TrivialCircuit<TrivialScalar>;

pub(crate) type PublicParams = nova_snark::PublicParams<
    PoseidonEngine,
    TrivialEngine,
    PoseidonCircuit,
    TrivialCircuit,
>;

pub(crate) type PoseidonSnark =
    RelaxedR1CSSNARK<PoseidonEngine, mlkzg::EvaluationEngine<PoseidonEngine>>;
pub(crate) type TrivialSnark =
    RelaxedR1CSSNARK<TrivialEngine, ipa_pc::EvaluationEngine<TrivialEngine>>;

pub(crate) type RecursiveSnark = RecursiveSNARK<
    PoseidonEngine,
    TrivialEngine,
    PoseidonCircuit,
    TrivialCircuit,
>;

pub(crate) type CompressedSnark = CompressedSNARK<
    PoseidonEngine,
    TrivialEngine,
    PoseidonCircuit,
    TrivialCircuit,
    PoseidonSnark,
    TrivialSnark,
>;

pub(crate) type VerifierKey = nova_snark::VerifierKey<
    PoseidonEngine,
    TrivialEngine,
    PoseidonCircuit,
    TrivialCircuit,
    PoseidonSnark,
    TrivialSnark,
>;
