//! The circuit's inputs by gnark's names, and the witness vector in the
//! key's order.
use std::collections::HashMap;

use ark_bn254::Fr;
use ark_ff::PrimeField;

use crate::Error;

/// Values by the names gnark gives a circuit's inputs: a struct field's
/// path, with each step and each index joined by an underscore
/// (`Private_Inputs_0_Utxo_Domain`); an embedded struct's fields keep the
/// embedding struct's path.
#[derive(Default)]
pub struct Assignment {
    values: HashMap<String, Fr>,
}

impl Assignment {
    pub fn set(&mut self, name: String, value: Fr) {
        self.values.insert(name, value);
    }

    /// `prefix_i` for each value.
    pub fn set_all(&mut self, prefix: &str, values: &[Fr]) {
        for (i, value) in values.iter().enumerate() {
            self.set(format!("{prefix}_{i}"), *value);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Fr> {
        self.values.get(name)
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// The names assigned, sorted.
    pub fn names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.values.keys().map(String::as_str).collect();
        names.sort_unstable();
        names
    }
}

/// The assignment's values in the key's order of names: every name
/// assigned, nothing assigned the key does not name (else the request's
/// shape is not the key's).
pub fn order(assignment: &Assignment, names: &[String]) -> Result<Vec<Fr>, Error> {
    if assignment.len() != names.len() {
        return Err(Error::Shape);
    }
    names
        .iter()
        .map(|name| assignment.get(name).copied().ok_or(Error::Shape))
        .collect()
}

/// The kernel's encoding of the witness: four little-endian Montgomery
/// limbs per value.
pub fn encode(values: &[Fr]) -> Vec<u8> {
    let mut out = Vec::with_capacity(values.len() * 32);
    for value in values {
        for limb in value.0.0 {
            out.extend_from_slice(&limb.to_le_bytes());
        }
    }
    out
}

/// A value's decimal digits, as gnark's flattened witnesses print them.
pub fn decimal(value: &Fr) -> String {
    value.into_bigint().to_string()
}
