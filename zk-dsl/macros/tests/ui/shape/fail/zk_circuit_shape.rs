extern crate spr_zk_dsl as spr_program;

use spr_program::ZkCircuit;

#[derive(Clone, ZkCircuit)]
struct MissingPublic {
    private: u64,
}

#[derive(Clone, ZkCircuit)]
struct ExtraField {
    private: u64,
    public: u64,
    amount: u64,
}

fn main() {}
