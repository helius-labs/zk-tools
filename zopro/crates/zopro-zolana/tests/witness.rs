//! The transfer and merge assignments against the Go prover's flattened
//! witness of the same request (gnark's names, decimal values), as
//! `tools/zolana`'s `TestZoproDump` writes it (`witness.json`).
use std::collections::BTreeMap;

use serde_json::Value;
use zopro_zolana::{
    Circuit, request,
    zopro_prover::{Assignment, witness},
};

const REQUEST: &str = include_str!("fixtures/transfer-2x2.request.json");
const WITNESS: &str = include_str!("fixtures/transfer-2x2.witness.json");
const MERGE_REQUEST: &str = include_str!("fixtures/merge-24x1.request.json");
const MERGE_WITNESS: &str = include_str!("fixtures/merge-24x1.witness.json");

/// The golden witness, and its names: the key's.
fn golden(witness: &str) -> (BTreeMap<String, String>, Vec<String>) {
    let golden: BTreeMap<String, String> = serde_json::from_str(witness).unwrap();
    let names = golden.keys().cloned().collect();
    (golden, names)
}

fn assert_matches(assignment: &Assignment, witness: &str) {
    let (golden, _) = golden(witness);
    assert_eq!(assignment.len(), golden.len(), "{:?}", assignment.names());
    for (name, value) in &golden {
        let got = assignment
            .get(name)
            .unwrap_or_else(|| panic!("{name} not assigned"));
        assert_eq!(&witness::decimal(got), value, "{name}");
    }
}

#[test]
fn transfer_2x2_matches_the_go_witness() {
    let json = request::parse(REQUEST).unwrap();
    // The key's names, as the golden lists them (the signer width comes
    // from them).
    let (_, names) = golden(WITNESS);
    let assignment =
        zopro_zolana::test_support::transfer(&json, Circuit::TransferConfidential, 2, 2, &names)
            .unwrap();
    assert_matches(&assignment, WITNESS);
}

#[test]
fn merge_24x1_matches_the_go_witness() {
    let json = request::parse(MERGE_REQUEST).unwrap();
    let assignment = zopro_zolana::test_support::merge(&json, Circuit::Merge, 24).unwrap();
    assert_matches(&assignment, MERGE_WITNESS);
}

#[test]
fn rejects_what_the_go_prover_rejected() {
    let json = request::parse(REQUEST).unwrap();
    let (_, names) = golden(WITNESS);
    let ok = |json: &Value| {
        zopro_zolana::test_support::transfer(json, Circuit::TransferConfidential, 2, 2, &names)
            .is_ok()
    };
    assert!(ok(&json));
    let mut zero_seed = json.clone();
    zero_seed["blindingSeed"] = "0x0".into();
    assert!(!ok(&zero_seed));
    let mut extra = json.clone();
    extra["extra"] = "1".into();
    assert!(!ok(&extra));
    let mut missing = json.clone();
    missing.as_object_mut().unwrap().remove("cacheReadIndex");
    assert!(!ok(&missing));
    let mut flags = json.clone();
    flags["inputFlags"] = "0x3".into();
    assert!(!ok(&flags));
    let mut big = json.clone();
    big["externalDataHash"] =
        "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001".into();
    assert!(!ok(&big));
    let mut signers = json.clone();
    signers["signerPkHashes"].as_array_mut().unwrap().pop();
    assert!(!ok(&signers));
}

#[test]
fn rejects_what_the_go_prover_rejected_of_a_merge() {
    let json = request::parse(MERGE_REQUEST).unwrap();
    let ok = |json: &Value, circuit| zopro_zolana::test_support::merge(json, circuit, 24).is_ok();
    assert!(ok(&json, Circuit::Merge));
    // The default merge requires the ciphertext's salt.
    let mut no_salt = json.clone();
    no_salt
        .as_object_mut()
        .unwrap()
        .remove("outputCiphertextSalt");
    assert!(!ok(&no_salt, Circuit::Merge));
    // The mint's two chunks, no asset.
    let mut asset = json.clone();
    asset["asset"] = "0x7".into();
    assert!(!ok(&asset, Circuit::Merge));
    asset.as_object_mut().unwrap().remove("mintChunks");
    assert!(!ok(&asset, Circuit::Merge));
    for width in [1, 3] {
        let mut chunks = json.clone();
        chunks["mintChunks"] = vec!["0x1"; width].into();
        assert!(!ok(&chunks, Circuit::Merge));
    }
    let mut zero_secret = json.clone();
    zero_secret["userNullifierSecret"] = "0x0".into();
    assert!(!ok(&zero_secret, Circuit::Merge));
    // 23 inputs is no merge's shape.
    let mut short = json.clone();
    short["inputs"].as_array_mut().unwrap().pop();
    assert!(!ok(&short, Circuit::Merge));
    // A ring merge has no ciphertext: no salt, its ring values instead.
    let mut ring = no_salt.clone();
    ring["circuitType"] = "merge-ring".into();
    let assignment = zopro_zolana::test_support::merge(&ring, Circuit::MergeRing, 24).unwrap();
    assert!(assignment.get("OutputCiphertextSalt").is_none());
    assert!(assignment.get("UserSigningPkHash").is_none());
    assert!(assignment.get("RingProgramID").is_some());
    let mut ring_salt = json.clone();
    ring_salt["circuitType"] = "merge-ring".into();
    assert!(!ok(&ring_salt, Circuit::MergeRing));
}

#[test]
fn parses_as_zolanas_strict_parse() {
    assert!(request::parse(REQUEST).is_ok());
    // A repeated key, data after the body, a body that is not an object.
    let repeated = REQUEST.replacen(r#""nInputs":2"#, r#""nInputs":2,"nInputs":2"#, 1);
    assert_ne!(repeated, REQUEST);
    assert_eq!(request::parse(&repeated), Err(request::Error::Request));
    assert!(request::parse(&format!("{REQUEST}{{}}")).is_err());
    for body in ["{", "null", "[]", ""] {
        assert!(request::parse(body).is_err(), "{body}");
    }
    // No more than 32 levels.
    let nested = |levels: usize| format!("{{\"a\":{}1{}}}", "[".repeat(levels), "]".repeat(levels));
    assert!(request::parse(&nested(31)).is_ok());
    assert!(request::parse(&nested(32)).is_err());
    // Hex as zolana's FromHex reads it: `0x` first, then big.Int's sign.
    let decimal = |s| request::hex_element(s).map(|v| witness::decimal(&v));
    assert_eq!(decimal("-0").unwrap(), "0");
    assert_eq!(decimal("0x-0").unwrap(), "0");
    assert_eq!(decimal("0x+ff").unwrap(), "255");
    assert_eq!(decimal("+ff").unwrap(), "255");
    for bad in [
        "-1", "0x-1", "-0x0", "+0x5", "0x0x5", "0X5", "5_0", "0x", "-", "",
    ] {
        assert!(request::hex_element(bad).is_err(), "{bad}");
    }
    assert_eq!(
        decimal("00000000000000000000000000000000000000000000000000000000000000000000ff").unwrap(),
        "255"
    );
}
