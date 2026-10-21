extern crate zolana_zk_dsl as zolana_program;

use zolana_program::{
    circuit::{constant, Assert, Circuit},
    conversion::ProofInput,
    CircuitError, Groth16Keys, Groth16Prover, ZkCircuit,
};

#[derive(Clone, ProofInput)]
struct Old {
    x: u64,
    y: u64,
}

impl Circuit for OldCircuit {
    type Output = ();

    fn circuit(&self) -> Result<(), CircuitError> {
        self.x.assert_equal(&self.y, "old rule")
    }
}

#[derive(Clone, ProofInput)]
struct New {
    x: u64,
    y: u64,
}

impl Circuit for NewCircuit {
    type Output = ();

    fn circuit(&self) -> Result<(), CircuitError> {
        self.x.var().assert_equal(&constant(1u64), "new rule")
    }
}

#[test]
fn serialized_key_is_bound_to_its_circuit_constraints() {
    let old = Groth16Prover::<Old>::new_with_test_setup().unwrap();
    let old_bytes = old.keys().to_bytes().unwrap();
    let reloaded = Groth16Keys::from_bytes(&old_bytes).unwrap();
    assert!(Groth16Prover::<New>::new(reloaded).is_err());

    let old_witness = old.prove(&Old { x: 0, y: 0 }).unwrap();
    old.verify(&old_witness).unwrap();
    assert!(New { x: 0, y: 0 }.check_constraints().is_err());
}

#[test]
fn a_key_without_a_circuit_digest_is_rejected() {
    let old = Groth16Prover::<Old>::new_with_test_setup().unwrap();
    let mut bytes = old.keys().to_bytes().unwrap();
    bytes.truncate(bytes.len() - 32);
    assert!(Groth16Keys::from_bytes(&bytes).is_err());
}
