#![cfg(feature = "client")]

use solana_address::Address;
use zolana_hasher::primitives::{hash_bytes, pack_be};
use zolana_zk_dsl::{
    circuit::{
        Assert, Bool, Circuit, CircuitType, CircuitVar, ConstraintSystem, Public, PublicValue,
        Select, SolanaAddress,
    },
    conversion::{to_bytes, Allocator, FromCircuit, Placeholder, ProofInput},
    CircuitError, ZkCircuit,
};
use zolana_zk_gadgets::HashBytes;

#[test]
fn addresses_preserve_spp_bytes_packing_and_hashing() {
    let mut distinct_tail = [0; 32];
    distinct_tail[0] = 0x80;
    distinct_tail[30] = 0x53;
    distinct_tail[31] = 0xff;
    let actual = [[0; 32], [0xff; 32], [7; 32], distinct_tail].map(|bytes| {
        let address = Address::new_from_array(bytes);
        let circuit: SolanaAddress = address.instantiate(&Allocator::native()).unwrap();
        let mut public = Vec::new();
        circuit.public_inputs(&mut public);
        (
            public
                .iter()
                .map(|field| to_bytes(field).unwrap())
                .collect::<Vec<_>>(),
            to_bytes(&circuit.hash_bytes().unwrap()).unwrap(),
            Address::from_circuit(&circuit).unwrap(),
        )
    });
    let expected = [[0; 32], [0xff; 32], [7; 32], distinct_tail].map(|bytes| {
        (
            pack_be::<32, 2>(&bytes).to_vec(),
            hash_bytes(&bytes).unwrap(),
            Address::new_from_array(bytes),
        )
    });
    assert_eq!(
        actual, expected,
        "Solana addresses retain every byte and use the SPP 31+1 byte encoding"
    );
}

#[test]
fn address_selection_and_equality_bind_the_last_byte() {
    let left = Address::new_from_array([1; 32]);
    let mut bytes = [1; 32];
    bytes[31] = 2;
    let right = Address::new_from_array(bytes);
    let actual = [false, true].map(|flag| {
        let cs = ConstraintSystem::new_ref();
        let allocator = Allocator::R1cs(cs.clone());
        let condition = flag.instantiate(&allocator).unwrap();
        let selected = SolanaAddress::select(
            &condition,
            &left.instantiate(&allocator).unwrap(),
            &right.instantiate(&allocator).unwrap(),
        );
        let (expected, wrong) = if flag { (left, right) } else { (right, left) };
        selected
            .assert_equal(
                &SolanaAddress::constant(&expected),
                "the selected address matches",
            )
            .unwrap();
        let satisfied = cs.is_satisfied().unwrap();
        selected
            .assert_equal(
                &SolanaAddress::constant(&wrong),
                "the address differs in its last byte",
            )
            .unwrap();
        (satisfied, cs.is_satisfied().unwrap())
    });
    assert_eq!(
        actual,
        [(true, false); 2],
        "address equality includes all 32 bytes in either selection branch"
    );
}

#[test]
fn conditional_address_equality_checks_only_when_enabled() {
    let left = SolanaAddress::constant(&Address::new_from_array([1; 32]));
    let right = SolanaAddress::constant(&Address::new_from_array([2; 32]));
    assert_eq!(
        (
            left.assert_equal_if(&right, &Bool::constant(false), "addresses match")
                .map_err(|error| error.broken_rule()),
            left.assert_equal_if(&right, &Bool::constant(true), "addresses match")
                .map_err(|error| error.broken_rule()),
        ),
        (Ok(()), Err(Some("addresses match"))),
        "conditional address checks retain the byte wrapper's assertion semantics"
    );
}

#[test]
fn address_hashes_are_shared_by_clones_and_selection_uses_a_fresh_cache() {
    let actual = [false, true].map(|flag| {
        let cs = ConstraintSystem::new_ref();
        let allocator = Allocator::R1cs(cs.clone());
        let left = Address::new_from_array([1; 32])
            .instantiate(&allocator)
            .unwrap();
        let right = Address::new_from_array([2; 32])
            .instantiate(&allocator)
            .unwrap();
        let left_hash = left.hash_bytes().unwrap();
        right.hash_bytes().unwrap();
        let cloned = left.clone();
        let before = cs.num_constraints();
        let clone_hash = cloned.hash_bytes().unwrap();
        let clone_adds_no_rows = cs.num_constraints() == before;
        let selected = SolanaAddress::select(&flag.instantiate(&allocator).unwrap(), &left, &right);
        let selected_hash = selected.hash_bytes().unwrap();
        let before = cs.num_constraints();
        selected.hash_bytes().unwrap();
        let selected_adds_no_rows = cs.num_constraints() == before;
        let expected_hash = hash_bytes(&[if flag { 1 } else { 2 }; 32])
            .unwrap()
            .instantiate(&Allocator::native())
            .unwrap();
        selected_hash
            .assert_equal(&expected_hash, "selection hashes the selected address")
            .unwrap();
        (
            left.is_clone_of(&cloned),
            clone_adds_no_rows,
            clone_hash
                .assert_equal(&left_hash, "clones share the hash")
                .is_ok(),
            !selected.is_clone_of(&left) && !selected.is_clone_of(&right),
            selected_adds_no_rows,
            cs.is_satisfied().unwrap(),
        )
    });
    let expected = [(true, true, true, true, true, true); 2];
    assert_eq!(actual, expected);
}

struct AddressHash<T>(T);
struct AddressHashCircuit<T>(T);

impl<T: CircuitType> CircuitType for AddressHashCircuit<T> {}

impl<T: ProofInput> ProofInput for AddressHash<T> {
    type Circuit = AddressHashCircuit<T::Circuit>;

    fn instantiate(&self, allocator: &Allocator) -> Result<Self::Circuit, CircuitError> {
        Ok(AddressHashCircuit(self.0.instantiate(allocator)?))
    }
}

impl<T: Placeholder> Placeholder for AddressHash<T> {
    fn placeholder() -> Result<Self, CircuitError> {
        Ok(Self(T::placeholder()?))
    }
}

impl<T: CircuitType + HashBytes> Circuit for AddressHashCircuit<T> {
    type Output = Public<CircuitVar>;

    fn circuit(&self) -> Result<Self::Output, CircuitError> {
        Ok(Public(self.0.hash_bytes()?))
    }
}

#[cfg(feature = "setup")]
#[test]
fn the_address_wrapper_preserves_the_existing_r1cs_and_assignment() {
    let bytes = [0xff; 32];
    assert_eq!(
        (
            AddressHash::<Address>::export_r1cs().unwrap(),
            AddressHash(Address::new_from_array(bytes))
                .export_assignment()
                .unwrap(),
        ),
        (
            AddressHash::<zolana_zk_dsl::Bytes<32>>::export_r1cs().unwrap(),
            AddressHash(zolana_zk_dsl::Bytes(bytes))
                .export_assignment()
                .unwrap(),
        ),
        "the address wrapper changes the Rust type without changing the proof representation"
    );
}
