extern crate spr_zk_dsl as spr_program;

use spr_program::{
    circuit::{value, Field},
    conversion::{Allocator, FromCircuit, Placeholder, ProofInput},
};

#[derive(Clone, spr_zk_macros::PublicInputs)]
struct WidePublic {
    amount: u128,
}

#[test]
fn u128_public_input_instantiates_and_round_trips() {
    let input = WidePublic { amount: u128::MAX };
    let circuit = input.instantiate(&Allocator::native()).unwrap();
    assert_eq!(
        value(&circuit.amount.var()).unwrap(),
        Field::from(u128::MAX)
    );
    assert_eq!(u128::from_circuit(&circuit.amount).unwrap(), u128::MAX);
    assert_eq!(WidePublic::placeholder().unwrap().amount, 0);
}
