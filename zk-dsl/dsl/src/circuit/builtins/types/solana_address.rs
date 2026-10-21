use solana_address::Address;

use crate::{
    circuit::{Assert, Bool, Bytes, CircuitDefault, CircuitType, CircuitVar, Select},
    CircuitError,
};

/// A Solana address represented by all 32 checked bytes.
/// Packing uses the protocol's 31-byte big-endian chunks, without a key tag.
#[derive(Clone, Debug, Default)]
pub struct SolanaAddress(Bytes<32>);

impl SolanaAddress {
    pub fn constant(address: &Address) -> Self {
        Self(Bytes::constant(address.as_array()))
    }

    pub fn as_bytes(&self) -> &Bytes<32> {
        &self.0
    }

    pub fn packed(&self) -> Vec<CircuitVar> {
        self.0.packed()
    }
}

impl From<Bytes<32>> for SolanaAddress {
    fn from(bytes: Bytes<32>) -> Self {
        Self(bytes)
    }
}

impl Assert for SolanaAddress {
    #[track_caller]
    fn is_equal(&self, other: &Self) -> Result<Bool, CircuitError> {
        self.0.is_equal(&other.0)
    }

    #[track_caller]
    fn assert_equal(&self, other: &Self, rule: &'static str) -> Result<(), CircuitError> {
        self.0.assert_equal(&other.0, rule)
    }

    #[track_caller]
    fn assert_equal_if(
        &self,
        other: &Self,
        condition: &Bool,
        rule: &'static str,
    ) -> Result<(), CircuitError> {
        self.0.assert_equal_if(&other.0, condition, rule)
    }
}

impl Select for SolanaAddress {
    fn select(condition: &Bool, if_true: &Self, if_false: &Self) -> Self {
        Self(Bytes::select(condition, &if_true.0, &if_false.0))
    }
}

impl CircuitType for SolanaAddress {}

impl CircuitDefault for SolanaAddress {
    fn circuit_default() -> Self {
        Self::default()
    }
}
