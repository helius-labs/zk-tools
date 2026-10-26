//! The merge requests (`merge`, `merge-ring`) onto the spp_merge circuits'
//! inputs, as the Go prover's merge package did.
use zopro_bn254::Fr;
use serde_json::Value;

use crate::{
    Assignment, Circuit,
    request::{Error, element, elements, is_zero, object},
    transfer::{assign_tree_slots, check_tree_slots, tree_slots},
};

const STATE_TREE_HEIGHT: usize = 32;
const NULLIFIER_TREE_HEIGHT: usize = 40;
const SUPPORTED_INPUTS: &[usize] = &[8, 24, 54];

const REQUEST_FIELDS: &[&str] = &[
    "circuitType",
    "inputs",
    "output",
    "treeSlots",
    "outputTreeId",
    "asset",
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

struct Input {
    domain: Fr,
    amount: Fr,
    blinding: Fr,
    ring_data_hash: Fr,
    state_path: Vec<Fr>,
    state_path_index: Fr,
    nullifier_low_value: Fr,
    nullifier_next_value: Fr,
    nullifier_low_path: Vec<Fr>,
    nullifier_low_path_index: Fr,
    tree_slot: Fr,
    nullifier: Fr,
}

pub fn assign(json: &Value, circuit: Circuit, key_inputs: u32) -> Result<Assignment, Error> {
    let map = object(json, REQUEST_FIELDS)?;
    let inputs: Vec<Input> = map["inputs"]
        .as_array()
        .ok_or(Error::Request)?
        .iter()
        .map(|input| {
            let map = object(input, INPUT_FIELDS)?;
            Ok::<_, Error>(Input {
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
    let asset = element(&map["asset"])?;
    let owner_pk_hash = element(&map["ownerPkHash"])?;
    let user_nullifier_pk = element(&map["userNullifierPk"])?;
    // Required and non-zero: it seeds the private tx and output blindings.
    let user_nullifier_secret = element(&map["userNullifierSecret"])?;
    if is_zero(&user_nullifier_secret) {
        return Err(Error::Request);
    }
    let external_data_hash = element(&map["externalDataHash"])?;
    let private_tx_hash = element(&map["privateTxHash"])?;
    let public_input_hash = element(&map["publicInputHash"])?;
    let allow_dummy_inputs = element(&map["allowDummyInputs"])?;
    let top_ring_data_hash = element(&map["outputRingDataHash"])?;
    let ring_program_id = element(&map["ringProgramId"])?;

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
    check_tree_slots(&slots, &input_slots, |_| true)?;
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
    a.set("Asset".into(), asset);
    a.set("OwnerPkHash".into(), owner_pk_hash);
    a.set("UserNullifierPk".into(), user_nullifier_pk);
    a.set("UserNullifierSecret".into(), user_nullifier_secret);
    // CommonPublicInputs, embedded: its fields keep the circuit's path.
    a.set("OutputHash".into(), output_hash);
    a.set("PrivateTxHash".into(), private_tx_hash);
    a.set("ExternalDataHash".into(), external_data_hash);
    a.set("AllowDummyInputs".into(), allow_dummy_inputs);
    assign_tree_slots(&mut a, "TreeSlots", &slots);
    a.set("OutputTreeID".into(), output_tree_id);
    match circuit {
        Circuit::MergeRing => {
            a.set("OutputRingDataHash".into(), top_ring_data_hash);
            a.set("RingProgramID".into(), ring_program_id);
        }
        _ => a.set("UserSigningPkHash".into(), owner_pk_hash),
    }
    a.set("PublicInputHash".into(), public_input_hash);
    Ok(a)
}
