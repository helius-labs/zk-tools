use std::{cell::OnceCell, rc::Rc};

use solana_address::Address;

use crate::{
    circuit::{
        builtins::field::var::cached, Assert, Bool, Bytes, CircuitDefault, CircuitType, CircuitVar,
        Select,
    },
    CircuitError,
};

/// A Solana address represented by all 32 checked bytes.
/// Packing uses the protocol's 31-byte big-endian chunks, without a key tag.
#[derive(Clone, Debug, Default)]
pub struct SolanaAddress {
    bytes: Bytes<32>,
    hash: Rc<OnceCell<CircuitVar>>,
}

impl SolanaAddress {
    pub fn constant(address: &Address) -> Self {
        Bytes::constant(address.as_array()).into()
    }

    pub fn as_bytes(&self) -> &Bytes<32> {
        &self.bytes
    }

    pub fn packed(&self) -> Vec<CircuitVar> {
        self.bytes.packed()
    }

    /// Shares the address hash between clones without adding constraints again.
    #[doc(hidden)]
    pub fn cached_hash(
        &self,
        hash: impl FnOnce() -> Result<CircuitVar, CircuitError>,
    ) -> Result<CircuitVar, CircuitError> {
        cached(&self.hash, hash)
    }

    #[doc(hidden)]
    pub fn is_clone_of(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.hash, &other.hash)
    }
}

impl From<Bytes<32>> for SolanaAddress {
    fn from(bytes: Bytes<32>) -> Self {
        Self {
            bytes,
            hash: Rc::new(OnceCell::new()),
        }
    }
}

impl Assert for SolanaAddress {
    #[track_caller]
    fn is_equal(&self, other: &Self) -> Result<Bool, CircuitError> {
        self.bytes.is_equal(&other.bytes)
    }

    #[track_caller]
    fn assert_equal(&self, other: &Self, rule: &'static str) -> Result<(), CircuitError> {
        self.bytes.assert_equal(&other.bytes, rule)
    }

    #[track_caller]
    fn assert_equal_if(
        &self,
        other: &Self,
        condition: &Bool,
        rule: &'static str,
    ) -> Result<(), CircuitError> {
        self.bytes.assert_equal_if(&other.bytes, condition, rule)
    }
}

impl Select for SolanaAddress {
    fn select(condition: &Bool, if_true: &Self, if_false: &Self) -> Self {
        Bytes::select(condition, &if_true.bytes, &if_false.bytes).into()
    }
}

impl CircuitType for SolanaAddress {}

impl CircuitDefault for SolanaAddress {
    fn circuit_default() -> Self {
        Self::default()
    }
}
