//! The transfer requests (`transfer-confidential`, `transfer-ring`,
//! `transfer-ring-authority`) onto the spp_transaction circuits' inputs, as
//! the Go prover's transfer_eddsa_only package did: the same checks, the
//! same defaults, no hashing. The client computes every value.
use zopro_bn254::Fr;
use serde_json::Value;

use crate::{
    Assignment, Circuit,
    request::{self, Error, count, element, elements, is_zero, object, small},
};

const INPUT_TREES: usize = 5;
const STATE_TREE_HEIGHT: usize = 32;
const NULLIFIER_TREE_HEIGHT: usize = 40;
const PUBLIC_SLOTS: usize = 3;
const TREE_INDEX_BITS: u32 = 3;
const UTXO_DOMAIN: u64 = 3;

const REQUEST_FIELDS: &[&str] = &[
    "circuitType",
    "nInputs",
    "nOutputs",
    "inputs",
    "outputs",
    "treeSlots",
    "outputTreeId",
    "externalDataHash",
    "privateTxHash",
    "blindingSeed",
    "publicAssets",
    "publicAmounts",
    "ringProgramId",
    "signerPkHashes",
    "inputFlags",
    "publishedOutputOwnerPkHashes",
    "cacheTreeId",
    "cacheReadHashChain",
    "cacheReadHashes",
    "cacheIsCached",
    "cacheReadIndex",
    "publicInputHash",
];
const UTXO_FIELDS: &[&str] = &[
    "domain",
    "owner",
    "asset",
    "amount",
    "blinding",
    "dataHash",
    "ringDataHash",
    "ringProgramId",
];
const INPUT_FIELDS: &[&str] = &[
    "utxo",
    "isDummy",
    "statePathElements",
    "statePathIndex",
    "nullifierLowValue",
    "nullifierNextValue",
    "nullifierLowPathElements",
    "nullifierLowPathIndex",
    "treeSlot",
    "nullifier",
    "ownerPkHash",
    "nullifierSecret",
];
const OUTPUT_FIELDS: &[&str] = &["utxo", "isDummy", "hash", "ownerPkHash", "nullifierPk"];
pub(crate) const TREE_SLOT_FIELDS: &[&str] = &["id", "utxoRoot", "nullifierRoot"];

struct Utxo {
    domain: Fr,
    owner: Fr,
    asset: Fr,
    amount: Fr,
    blinding: Fr,
    data_hash: Fr,
    ring_data_hash: Fr,
    ring_program_id: Fr,
}

struct Input {
    utxo: Utxo,
    state_path: Vec<Fr>,
    state_path_index: Fr,
    nullifier_low_value: Fr,
    nullifier_next_value: Fr,
    nullifier_low_path: Vec<Fr>,
    nullifier_low_path_index: Fr,
    tree_slot: Fr,
    nullifier: Fr,
    owner_pk_hash: Fr,
    nullifier_secret: Fr,
}

struct Output {
    utxo: Utxo,
    hash: Fr,
    owner_pk_hash: Fr,
    nullifier_pk: Fr,
}

pub(crate) struct TreeSlot {
    pub id: Fr,
    pub utxo_root: Fr,
    pub nullifier_root: Fr,
}

fn utxo(value: &Value) -> Result<Utxo, Error> {
    let map = object(value, UTXO_FIELDS)?;
    Ok(Utxo {
        domain: element(&map["domain"])?,
        owner: element(&map["owner"])?,
        asset: element(&map["asset"])?,
        amount: element(&map["amount"])?,
        blinding: element(&map["blinding"])?,
        data_hash: element(&map["dataHash"])?,
        ring_data_hash: element(&map["ringDataHash"])?,
        ring_program_id: element(&map["ringProgramId"])?,
    })
}

pub(crate) fn tree_slots(value: &Value) -> Result<Vec<TreeSlot>, Error> {
    value
        .as_array()
        .ok_or(Error::Request)?
        .iter()
        .map(|slot| {
            let map = object(slot, TREE_SLOT_FIELDS)?;
            Ok::<_, Error>(TreeSlot {
                id: element(&map["id"])?,
                utxo_root: element(&map["utxoRoot"])?,
                nullifier_root: element(&map["nullifierRoot"])?,
            })
        })
        .collect()
}

/// The slot each input selects, in range, its roots set (the state root
/// only where the input needs one): the circuit fails these as opaque
/// proving errors.
pub(crate) fn check_tree_slots(
    slots: &[TreeSlot],
    inputs: &[Fr],
    needs_state_root: impl Fn(usize) -> bool,
) -> Result<(), Error> {
    if slots.len() != INPUT_TREES {
        return Err(Error::Request);
    }
    for (i, slot) in inputs.iter().enumerate() {
        let index = small(slot)
            .filter(|s| *s < INPUT_TREES as u64)
            .ok_or(Error::Request)? as usize;
        let selected = &slots[index];
        if is_zero(&selected.nullifier_root)
            || (needs_state_root(i) && is_zero(&selected.utxo_root))
        {
            return Err(Error::Request);
        }
    }
    Ok(())
}

pub(crate) fn assign_tree_slots(assignment: &mut Assignment, prefix: &str, slots: &[TreeSlot]) {
    for (k, slot) in slots.iter().enumerate() {
        assignment.set(format!("{prefix}_{k}_ID"), slot.id);
        assignment.set(format!("{prefix}_{k}_UtxoRoot"), slot.utxo_root);
        assignment.set(format!("{prefix}_{k}_NullifierRoot"), slot.nullifier_root);
    }
}

/// `inputFlags` packs the dummy policy in bit 0 and each input's tree slot
/// in its own 3 bits above it.
fn check_input_flags(flags: &Fr, slots: &[Fr]) -> Result<(), Error> {
    let mut want = small(flags).ok_or(Error::Request)? & 1;
    for (i, slot) in slots.iter().enumerate() {
        let slot = small(slot)
            .filter(|s| *s < 1 << TREE_INDEX_BITS)
            .ok_or(Error::Request)?;
        want |= slot << (1 + TREE_INDEX_BITS as usize * i);
    }
    if small(flags) != Some(want) {
        return Err(Error::Request);
    }
    Ok(())
}

/// The cache selection: which inputs read a cached UTXO instead of a tree.
struct Cache {
    tree_id: Fr,
    read_hash_chain: Fr,
    read_hashes: Vec<Fr>,
    is_cached: Vec<Fr>,
    read_index: Vec<Fr>,
}

impl Cache {
    fn entry(values: &[Fr], i: usize) -> Fr {
        values.get(i).copied().unwrap_or_else(|| Fr::from(0u64))
    }

    fn validate(&self, inputs: usize) -> Result<(), Error> {
        if small(&self.tree_id).is_none_or(|id| id >> 16 != 0) {
            return Err(Error::Request);
        }
        for list in [&self.read_hashes, &self.is_cached, &self.read_index] {
            if !list.is_empty() && list.len() != inputs {
                return Err(Error::Request);
            }
        }
        let reads = (0..inputs).any(|i| !is_zero(&Self::entry(&self.read_hashes, i)));
        if !reads && !is_zero(&self.tree_id) {
            return Err(Error::Request);
        }
        for i in 0..inputs {
            let cached = Self::entry(&self.is_cached, i);
            let index = Self::entry(&self.read_index, i);
            if !is_zero(&cached) && cached != Fr::from(1u64) {
                return Err(Error::Request);
            }
            let index = small(&index)
                .filter(|x| *x < inputs as u64)
                .ok_or(Error::Request)?;
            if !is_zero(&cached) && is_zero(&Self::entry(&self.read_hashes, index as usize)) {
                return Err(Error::Request);
            }
        }
        Ok(())
    }

    fn assign(&self, assignment: &mut Assignment, inputs: usize) {
        assignment.set("CachedInputs_TreeID".into(), self.tree_id);
        assignment.set("CachedInputs_ReadHashChain".into(), self.read_hash_chain);
        for i in 0..inputs {
            assignment.set(
                format!("CachedInputs_ReadHashes_{i}"),
                Self::entry(&self.read_hashes, i),
            );
            assignment.set(
                format!("CachedInputs_IsCached_{i}"),
                Self::entry(&self.is_cached, i),
            );
            assignment.set(
                format!("CachedInputs_ReadIndex_{i}"),
                Self::entry(&self.read_index, i),
            );
        }
    }
}

/// The request's values on the circuit's inputs, checked as the Go prover
/// checked them. `names` are the key's input names: the signer vector's
/// width comes from them.
pub fn assign(
    json: &Value,
    circuit: Circuit,
    key_inputs: u32,
    key_outputs: u32,
    names: &[String],
) -> Result<Assignment, Error> {
    let map = object(json, REQUEST_FIELDS)?;
    let (n_inputs, n_outputs) = (count(&map["nInputs"])?, count(&map["nOutputs"])?);
    let inputs: Vec<Input> = map["inputs"]
        .as_array()
        .ok_or(Error::Request)?
        .iter()
        .map(|input| {
            let map = object(input, INPUT_FIELDS)?;
            element(&map["isDummy"])?;
            Ok::<_, Error>(Input {
                utxo: utxo(&map["utxo"])?,
                state_path: elements(&map["statePathElements"])?,
                state_path_index: element(&map["statePathIndex"])?,
                nullifier_low_value: element(&map["nullifierLowValue"])?,
                nullifier_next_value: element(&map["nullifierNextValue"])?,
                nullifier_low_path: elements(&map["nullifierLowPathElements"])?,
                nullifier_low_path_index: element(&map["nullifierLowPathIndex"])?,
                tree_slot: element(&map["treeSlot"])?,
                nullifier: element(&map["nullifier"])?,
                owner_pk_hash: element(&map["ownerPkHash"])?,
                nullifier_secret: element(&map["nullifierSecret"])?,
            })
        })
        .collect::<Result<_, _>>()?;
    let outputs: Vec<Output> = map["outputs"]
        .as_array()
        .ok_or(Error::Request)?
        .iter()
        .map(|output| {
            let map = object(output, OUTPUT_FIELDS)?;
            element(&map["isDummy"])?;
            Ok::<_, Error>(Output {
                utxo: utxo(&map["utxo"])?,
                hash: element(&map["hash"])?,
                owner_pk_hash: element(&map["ownerPkHash"])?,
                nullifier_pk: element(&map["nullifierPk"])?,
            })
        })
        .collect::<Result<_, _>>()?;
    let slots = tree_slots(&map["treeSlots"])?;
    let output_tree_id = element(&map["outputTreeId"])?;
    let external_data_hash = element(&map["externalDataHash"])?;
    let private_tx_hash = element(&map["privateTxHash"])?;
    // Required and non-zero: the circuit derives every blinding from it.
    let blinding_seed = element(&map["blindingSeed"])?;
    if is_zero(&blinding_seed) {
        return Err(Error::Request);
    }
    let public_assets = elements(&map["publicAssets"])?;
    let public_amounts = elements(&map["publicAmounts"])?;
    if public_assets.len() != PUBLIC_SLOTS || public_amounts.len() != PUBLIC_SLOTS {
        return Err(Error::Request);
    }
    let ring_program_id = element(&map["ringProgramId"])?;
    let signer_pk_hashes = elements(&map["signerPkHashes"])?;
    let input_flags = element(&map["inputFlags"])?;
    let published_owners = elements(&map["publishedOutputOwnerPkHashes"])?;
    let cache = Cache {
        tree_id: element(&map["cacheTreeId"])?,
        read_hash_chain: element(&map["cacheReadHashChain"])?,
        read_hashes: elements(&map["cacheReadHashes"])?,
        is_cached: elements(&map["cacheIsCached"])?,
        read_index: elements(&map["cacheReadIndex"])?,
    };
    let public_input_hash = element(&map["publicInputHash"])?;

    // ValidateShape.
    if inputs.len() != n_inputs as usize || outputs.len() != n_outputs as usize {
        return Err(Error::Request);
    }
    if inputs.iter().any(|i| {
        i.state_path.len() != STATE_TREE_HEIGHT
            || i.nullifier_low_path.len() != NULLIFIER_TREE_HEIGHT
    }) {
        return Err(Error::Request);
    }
    let input_slots: Vec<Fr> = inputs.iter().map(|i| i.tree_slot).collect();
    let authority = circuit == Circuit::TransferRingAuthority;
    check_tree_slots(&slots, &input_slots, |i| {
        authority
            || (small(&inputs[i].utxo.domain) == Some(UTXO_DOMAIN)
                && is_zero(&Cache::entry(&cache.is_cached, i)))
    })?;
    check_input_flags(&input_flags, &input_slots)?;
    cache.validate(inputs.len())?;
    // The key's shape, before the names decide.
    if n_inputs != key_inputs || n_outputs != key_outputs {
        return Err(Error::Shape);
    }
    // CreateWitness: the signer vector is as wide as the key's.
    let signers = names
        .iter()
        .filter(|n| n.starts_with("Public_SignerPkHashes_"))
        .count();
    if signer_pk_hashes.len() != signers || signers == 0 {
        return Err(Error::Request);
    }
    let want_published = if authority { 0 } else { outputs.len() };
    if published_owners.len() != want_published {
        return Err(Error::Request);
    }

    let mut a = Assignment::default();
    if !authority {
        cache.assign(&mut a, inputs.len());
    }
    // Public.
    a.set_all(
        "Public_Nullifiers",
        &inputs.iter().map(|i| i.nullifier).collect::<Vec<_>>(),
    );
    a.set_all(
        "Public_OutputHashes",
        &outputs.iter().map(|o| o.hash).collect::<Vec<_>>(),
    );
    assign_tree_slots(&mut a, "Public_TreeSlots", &slots);
    a.set("Public_OutputTreeID".into(), output_tree_id);
    a.set("Public_PrivateTxHash".into(), private_tx_hash);
    a.set("Public_ExternalDataHash".into(), external_data_hash);
    a.set_all("Public_PublicAssets", &public_assets);
    a.set_all("Public_PublicAmounts", &public_amounts);
    if circuit != Circuit::TransferConfidential {
        a.set("Public_RingProgramID".into(), ring_program_id);
    }
    a.set("Public_InputFlags".into(), input_flags);
    a.set_all("Public_SignerPkHashes", &signer_pk_hashes);
    match circuit {
        Circuit::TransferConfidential => a.set_all("Public_OutputOwnerPkHashes", &published_owners),
        Circuit::TransferRing => {
            a.set_all("Public_PublishedOutputOwnerPkHashes", &published_owners)
        }
        _ => {}
    }
    a.set("Public_PublicInputHash".into(), public_input_hash);
    // Private.
    for (i, input) in inputs.iter().enumerate() {
        let p = format!("Private_Inputs_{i}");
        assign_utxo(&mut a, &format!("{p}_Utxo"), &input.utxo);
        a.set_all(&format!("{p}_StatePathElements"), &input.state_path);
        a.set(format!("{p}_StatePathIndex"), input.state_path_index);
        a.set(format!("{p}_TreeSlot"), input.tree_slot);
        a.set(format!("{p}_NullifierLowValue"), input.nullifier_low_value);
        a.set(
            format!("{p}_NullifierNextValue"),
            input.nullifier_next_value,
        );
        a.set_all(
            &format!("{p}_NullifierLowPathElements"),
            &input.nullifier_low_path,
        );
        a.set(
            format!("{p}_NullifierLowPathIndex"),
            input.nullifier_low_path_index,
        );
        a.set(format!("{p}_NullifierSecret"), input.nullifier_secret);
        a.set(
            format!("Private_InputOwnerPkHashes_{i}"),
            input.owner_pk_hash,
        );
    }
    for (i, output) in outputs.iter().enumerate() {
        assign_utxo(&mut a, &format!("Private_Outputs_{i}"), &output.utxo);
        match circuit {
            Circuit::TransferConfidential => {
                a.set(
                    format!("Private_OutputNullifierPks_{i}"),
                    output.nullifier_pk,
                );
            }
            Circuit::TransferRing => {
                a.set(
                    format!("Private_OutputOwnerPkHashes_{i}"),
                    output.owner_pk_hash,
                );
                a.set(
                    format!("Private_OutputNullifierPks_{i}"),
                    output.nullifier_pk,
                );
            }
            _ => {}
        }
    }
    a.set("Private_BlindingSeed".into(), blinding_seed);
    let _ = request::parse;
    Ok(a)
}

fn assign_utxo(a: &mut Assignment, prefix: &str, utxo: &Utxo) {
    a.set(format!("{prefix}_Domain"), utxo.domain);
    a.set(format!("{prefix}_Owner"), utxo.owner);
    a.set(format!("{prefix}_Asset"), utxo.asset);
    a.set(format!("{prefix}_Amount"), utxo.amount);
    a.set(format!("{prefix}_Blinding"), utxo.blinding);
    a.set(format!("{prefix}_DataHash"), utxo.data_hash);
    a.set(format!("{prefix}_RingDataHash"), utxo.ring_data_hash);
    a.set(format!("{prefix}_RingProgramID"), utxo.ring_program_id);
}
