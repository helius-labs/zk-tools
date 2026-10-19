//! A small gnark circuit end to end: X³ + X + 5 = Y with X not zero
//! (`tools/gnark`'s test writes the dump in tests/fixtures/cubic). The key
//! packs, loads, proves an assignment by name, refuses a wrong one, keeps
//! its metadata, and loads again from its raw form.
use std::path::Path;

use ark_bn254::Fr;
use zopro_prover::{
    Assignment, Error, Prover,
    zopro_core::{artifact::pack, dump::Dump},
};

fn key() -> Vec<u8> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cubic");
    pack(&Dump::read(&dir).expect("the cubic dump").parts("cubic")).expect("pack")
}

fn assignment(x: u64, y: u64) -> Assignment {
    let mut assignment = Assignment::default();
    assignment.set("X".into(), Fr::from(x));
    assignment.set("Y".into(), Fr::from(y));
    assignment
}

#[test]
fn proves_by_name() {
    let mut prover = Prover::load(&key()).expect("load");
    assert_eq!(prover.name(), "cubic");
    assert_eq!(prover.metadata(), b"cubic");
    assert_eq!(prover.names(), ["Y", "X"]);
    assert_eq!(prover.public_inputs(), 1);

    let witness = prover.witness(&assignment(3, 35)).expect("witness");
    let proved = prover.prove(&witness).expect("prove");
    assert_eq!(proved.public, [Fr::from(35u64)]);
    assert!(prover.verify(&proved.proof, &[Fr::from(35u64)]));
    assert!(!prover.verify(&proved.proof, &[Fr::from(36u64)]));
    assert_eq!(proved.proof.to_gnark().len(), 164);
    assert_eq!(proved.public_witness().len(), 44);

    let wrong = prover.witness(&assignment(3, 36)).expect("witness");
    assert!(matches!(prover.prove(&wrong), Err(Error::Prove(_))));
    let mut missing = Assignment::default();
    missing.set("X".into(), Fr::from(3u64));
    assert!(matches!(prover.witness(&missing), Err(Error::Witness(_))));
}

#[test]
fn loads_from_its_raw_form() {
    let (prover, source) = Prover::load_cacheable(&key()).expect("load");
    let raw = source.expect("a key file's raw source").export(&prover);
    let (mut again, source) = Prover::load_cacheable(&raw).expect("load the raw form");
    assert!(source.is_none());
    let witness = again.witness(&assignment(3, 35)).expect("witness");
    assert!(again.prove(&witness).is_ok());
    assert!(Prover::load_cacheable(&raw[..raw.len() - 1]).is_err());
}

#[test]
fn parses_values() {
    use zopro_prover::witness::parse_value;
    assert_eq!(parse_value("35"), Some(Fr::from(35u64)));
    assert_eq!(parse_value("0x23"), Some(Fr::from(35u64)));
    assert_eq!(parse_value("-1"), None);
    assert_eq!(parse_value(""), None);
    // The modulus itself is not a field element; one below it is.
    let r = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    assert_eq!(parse_value(r), None);
    let below = "21888242871839275222246405745257275088548364400416034343698204186575808495616";
    assert_eq!(parse_value(below), Some(-Fr::from(1u64)));
}
