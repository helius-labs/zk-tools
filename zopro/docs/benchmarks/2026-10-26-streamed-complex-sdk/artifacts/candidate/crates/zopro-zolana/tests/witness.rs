//! The transfer assignment against the Go prover's flattened witness of the
//! same request (gnark's names, decimal values).
use std::collections::BTreeMap;

use zopro_zolana::{Circuit, request, zopro_prover::witness};

const REQUEST: &str = include_str!("fixtures/transfer-2x2.request.json");
const WITNESS: &str = include_str!("fixtures/transfer-2x2.witness.json");

#[test]
fn transfer_2x2_matches_the_go_witness() {
    let golden: BTreeMap<String, String> = serde_json::from_str(WITNESS).unwrap();
    let json = request::parse(REQUEST).unwrap();
    // The key's names, as the golden lists them (the signer width comes
    // from them).
    let names: Vec<String> = golden.keys().cloned().collect();
    let assignment =
        zopro_zolana::test_support::transfer(&json, Circuit::TransferConfidential, 2, 2, &names)
            .unwrap();
    assert_eq!(assignment.len(), golden.len(), "{:?}", assignment.names());
    for (name, value) in &golden {
        let got = assignment
            .get(name)
            .unwrap_or_else(|| panic!("{name} not assigned"));
        assert_eq!(&witness::decimal(got), value, "{name}");
    }
}

#[test]
fn rejects_what_the_go_prover_rejected() {
    let json = request::parse(REQUEST).unwrap();
    let names: Vec<String> = serde_json::from_str::<BTreeMap<String, String>>(WITNESS)
        .unwrap()
        .keys()
        .cloned()
        .collect();
    let ok = |json: &serde_json::Value| {
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
    assert_eq!(
        request::hex_element("-0")
            .map(|v| witness::decimal(&v))
            .unwrap(),
        "0"
    );
    assert!(request::hex_element("-1").is_err());
    assert_eq!(
        request::hex_element(
            "00000000000000000000000000000000000000000000000000000000000000000000ff"
        )
        .map(|v| witness::decimal(&v))
        .unwrap(),
        "255"
    );
}
