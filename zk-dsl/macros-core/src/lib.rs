//! Expansion logic shared by the zk derives and the zolana program derives.
//! Generated paths start at `::spr_program`; a crate that depends only on
//! the zk DSL names it with `extern crate spr_zk_dsl as spr_program;`.

pub mod circuit_type;
pub mod error_name;
pub mod paths;
pub mod proof_input;
pub mod public_inputs;
pub mod r1cs_hook;
mod serialization;
pub mod shape;
pub mod zk_circuit;
