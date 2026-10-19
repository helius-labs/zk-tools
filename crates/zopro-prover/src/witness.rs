//! A circuit's inputs by gnark's names, and the witness vector in the key's
//! order.
use std::collections::HashMap;

use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};

use crate::Error;

/// Values by the names gnark gives a circuit's inputs: a struct field's
/// path, with each step and each index joined by an underscore
/// (`Inputs_0_Amount`); an embedded struct's fields keep the embedding
/// struct's path.
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
/// assigned, and nothing the key does not name.
pub fn order(assignment: &Assignment, names: &[String]) -> Result<Vec<Fr>, Error> {
    if let Some(name) = names.iter().find(|name| assignment.get(name).is_none()) {
        return Err(Error::Witness(format!("{name} is not assigned")));
    }
    if assignment.len() != names.len() {
        return Err(Error::Witness(
            "the assignment names inputs the key does not".into(),
        ));
    }
    Ok(names.iter().map(|name| assignment.values[name]).collect())
}

/// A value from its decimal digits or `0x` and hex digits, below the
/// field's modulus.
pub fn parse_value(text: &str) -> Option<Fr> {
    let (digits, base) = text.strip_prefix("0x").map_or((text, 10), |hex| (hex, 16));
    if digits.is_empty() || digits.len() > 80 {
        return None;
    }
    // Big endian bytes, times the base plus the digit for each digit.
    let mut bytes = vec![0u8; 34];
    for c in digits.chars() {
        let mut carry = c.to_digit(base)?;
        for byte in bytes.iter_mut().rev() {
            let v = *byte as u32 * base + carry;
            *byte = v as u8;
            carry = v >> 8;
        }
        if carry != 0 {
            return None;
        }
    }
    let modulus = Fr::MODULUS.to_bytes_be();
    if bytes[..2] != [0, 0] || bytes[2..] >= modulus[..] {
        return None;
    }
    Some(Fr::from_be_bytes_mod_order(&bytes[2..]))
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
