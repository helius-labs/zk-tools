//! The merge requests (`merge`, `merge-ring`) onto the spp_merge circuits'
//! inputs, as the Go prover's merge package does. The default merge also
//! publishes the output's amount and mint encrypted to the owner, which the
//! circuit computes from the request's mint chunks and ciphertext salt; the
//! ring merge has no ciphertext.
use serde_json::Value;
use zopro_bn254::Fr;

use crate::{
    Assignment, Circuit,
    request::{Error, element, elements, is_zero, object},
    transfer::{TreeSlot, assign_tree_slots, check_tree_slots, tree_slots},
};

const STATE_TREE_HEIGHT: usize = 32;
const NULLIFIER_TREE_HEIGHT: usize = 40;
const SUPPORTED_INPUTS: &[usize] = &[8, 24, 52];
/// The mint's hash_bytes_32 chunks: bytes 0..31 and byte 31.
const MINT_CHUNKS: usize = 2;

/// A ring merge's fields; a default merge has `outputCiphertextSalt` too.
const RING_REQUEST_FIELDS: &[&str] = &[
    "circuitType",
    "inputs",
    "output",
    "treeSlots",
    "outputTreeId",
    "mintChunks",
    "ownerPkHash",
    "userNullifierPk",
    "userNullifierSecret",
    "externalDataHash",
    "privateTxHash",
    "publicInputHash",
    "allowDummyInputs",
    "outputRingDataHash",
    "ringProgramId",
];
const SALT_FIELD: &str = "outputCiphertextSalt";
const INPUT_FIELDS: &[&str] = &[
    "domain",
    "amount",
    "blinding",
    "ringDataHash",
    "statePathElements",
    "statePathIndex",
    "nullifierLowValue",
    "nullifierNextValue",
    "nullifierLowPathElements",
    "nullifierLowPathIndex",
    "treeSlot",
    "nullifier",
];
const OUTPUT_FIELDS: &[&str] = &["ringDataHash", "hash"];

pub struct MergeInput {
    pub domain: Fr,
    pub amount: Fr,
    pub blinding: Fr,
    pub ring_data_hash: Fr,
    pub state_path: Vec<Fr>,
    pub state_path_index: Fr,
    pub nullifier_low_value: Fr,
    pub nullifier_next_value: Fr,
    pub nullifier_low_path: Vec<Fr>,
    pub nullifier_low_path_index: Fr,
    pub tree_slot: Fr,
    pub nullifier: Fr,
}

/// The assembled merge fields. The confidential circuit has a ciphertext
/// salt, and the ring circuit supplies its own ring data and program.
pub struct MergeInputs {
    pub inputs: Vec<MergeInput>,
    pub output_ring_data_hash: Fr,
    pub output_hash: Fr,
    pub tree_slots: Vec<TreeSlot>,
    pub output_tree_id: Fr,
    pub mint_chunks: Vec<Fr>,
    pub output_ciphertext_salt: Option<Fr>,
    pub owner_pk_hash: Fr,
    pub user_nullifier_pk: Fr,
    pub user_nullifier_secret: Fr,
    pub external_data_hash: Fr,
    pub private_tx_hash: Fr,
    pub public_input_hash: Fr,
    pub allow_dummy_inputs: Fr,
    pub published_output_ring_data_hash: Fr,
    pub ring_program_id: Fr,
}

impl MergeInputs {
    pub(crate) fn circuit(&self) -> Circuit {
        if self.output_ciphertext_salt.is_some() {
            Circuit::Merge
        } else {
            Circuit::MergeRing
        }
    }
}
pub(crate) fn decode(json: &Value, circuit: Circuit) -> Result<MergeInputs, Error> {
    // The default merge requires the ciphertext's salt; a ring merge, which
    // has no ciphertext, omits it.
    let ring = circuit == Circuit::MergeRing;
    let fields: Vec<&str> = RING_REQUEST_FIELDS
        .iter()
        .copied()
        .chain((!ring).then_some(SALT_FIELD))
        .collect();
    let map = object(json, &fields)?;
    let inputs: Vec<MergeInput> = map["inputs"]
        .as_array()
        .ok_or(Error::Request)?
        .iter()
        .map(|input| {
            let map = object(input, INPUT_FIELDS)?;
            Ok::<_, Error>(MergeInput {
                domain: element(&map["domain"])?,
                amount: element(&map["amount"])?,
                blinding: element(&map["blinding"])?,
                ring_data_hash: element(&map["ringDataHash"])?,
                state_path: elements(&map["statePathElements"])?,
                state_path_index: element(&map["statePathIndex"])?,
                nullifier_low_value: element(&map["nullifierLowValue"])?,
                nullifier_next_value: element(&map["nullifierNextValue"])?,
                nullifier_low_path: elements(&map["nullifierLowPathElements"])?,
                nullifier_low_path_index: element(&map["nullifierLowPathIndex"])?,
                tree_slot: element(&map["treeSlot"])?,
                nullifier: element(&map["nullifier"])?,
            })
        })
        .collect::<Result<_, _>>()?;
    let output = object(&map["output"], OUTPUT_FIELDS)?;
    let (output_ring_data_hash, output_hash) =
        (element(&output["ringDataHash"])?, element(&output["hash"])?);
    let slots = tree_slots(&map["treeSlots"])?;
    let output_tree_id = element(&map["outputTreeId"])?;
    let mint_chunks = elements(&map["mintChunks"])?;
    let salt = if ring {
        None
    } else {
        Some(element(&map[SALT_FIELD])?)
    };
    let owner_pk_hash = element(&map["ownerPkHash"])?;
    let user_nullifier_pk = element(&map["userNullifierPk"])?;
    // Required and non-zero: it seeds the private tx and output blindings.
    let user_nullifier_secret = element(&map["userNullifierSecret"])?;
    let external_data_hash = element(&map["externalDataHash"])?;
    let private_tx_hash = element(&map["privateTxHash"])?;
    let public_input_hash = element(&map["publicInputHash"])?;
    let allow_dummy_inputs = element(&map["allowDummyInputs"])?;
    let top_ring_data_hash = element(&map["outputRingDataHash"])?;
    let ring_program_id = element(&map["ringProgramId"])?;

    Ok(MergeInputs {
        inputs,
        output_ring_data_hash,
        output_hash,
        tree_slots: slots,
        output_tree_id,
        mint_chunks,
        output_ciphertext_salt: salt,
        owner_pk_hash,
        user_nullifier_pk,
        user_nullifier_secret,
        external_data_hash,
        private_tx_hash,
        public_input_hash,
        allow_dummy_inputs,
        published_output_ring_data_hash: top_ring_data_hash,
        ring_program_id,
    })
}

pub(crate) fn assign(input: &MergeInputs, key_inputs: u32) -> Result<Assignment, Error> {
    let MergeInputs {
        ref inputs,
        output_ring_data_hash,
        output_hash,
        tree_slots: ref slots,
        output_tree_id,
        ref mint_chunks,
        output_ciphertext_salt: salt,
        owner_pk_hash,
        user_nullifier_pk,
        user_nullifier_secret,
        external_data_hash,
        private_tx_hash,
        public_input_hash,
        allow_dummy_inputs,
        published_output_ring_data_hash: top_ring_data_hash,
        ring_program_id,
    } = *input;
    if mint_chunks.len() != MINT_CHUNKS || is_zero(&user_nullifier_secret) {
        return Err(Error::Request);
    }
    // ValidateShape.
    if !SUPPORTED_INPUTS.contains(&inputs.len()) {
        return Err(Error::Request);
    }
    if inputs.iter().any(|i| {
        i.state_path.len() != STATE_TREE_HEIGHT
            || i.nullifier_low_path.len() != NULLIFIER_TREE_HEIGHT
    }) {
        return Err(Error::Request);
    }
    let input_slots: Vec<Fr> = inputs.iter().map(|i| i.tree_slot).collect();
    check_tree_slots(slots, &input_slots, |_| true)?;
    if inputs.len() != key_inputs as usize {
        return Err(Error::Shape);
    }

    let mut a = Assignment::default();
    for (i, input) in inputs.iter().enumerate() {
        let p = format!("Inputs_{i}");
        a.set(format!("{p}_Domain"), input.domain);
        a.set(format!("{p}_Amount"), input.amount);
        a.set(format!("{p}_Blinding"), input.blinding);
        a.set(format!("{p}_RingDataHash"), input.ring_data_hash);
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
        a.set(format!("Nullifiers_{i}"), input.nullifier);
    }
    a.set("Output_RingDataHash".into(), output_ring_data_hash);
    a.set_all("MintChunks", mint_chunks);
    a.set("OwnerPkHash".into(), owner_pk_hash);
    a.set("UserNullifierPk".into(), user_nullifier_pk);
    a.set("UserNullifierSecret".into(), user_nullifier_secret);
    // CommonPublicInputs, embedded: its fields keep the circuit's path.
    a.set("OutputHash".into(), output_hash);
    a.set("PrivateTxHash".into(), private_tx_hash);
    a.set("ExternalDataHash".into(), external_data_hash);
    a.set("AllowDummyInputs".into(), allow_dummy_inputs);
    assign_tree_slots(&mut a, "TreeSlots", slots);
    a.set("OutputTreeID".into(), output_tree_id);
    match salt {
        None => {
            a.set("OutputRingDataHash".into(), top_ring_data_hash);
            a.set("RingProgramID".into(), ring_program_id);
        }
        Some(salt) => {
            a.set("UserSigningPkHash".into(), owner_pk_hash);
            a.set("OutputCiphertextSalt".into(), salt);
        }
    }
    a.set("PublicInputHash".into(), public_input_hash);
    Ok(a)
}
