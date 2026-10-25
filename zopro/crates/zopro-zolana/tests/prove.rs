//! Proofs of the dump requests through a zopro key packed from a dump
//! (`ZOPRO_DUMP=<dir>` with the `2x2` and `merge-24x1` shapes, as
//! `tools/zolana` writes them from the same requests): the witness vector is
//! the dump's byte for byte, the proof verifies and reports the gnark key's
//! sha256, and a raw proof is left in the dump for gnark's own verifier to
//! referee. Skipped without a dump.
use std::path::Path;

use zopro_zolana::{
    Metadata, Prover, proof_json,
    zopro_core::{artifact::pack, dump::Dump},
    zopro_prover::witness,
};

fn check(shape: &str, key_name: &str, request: &str) {
    let Ok(dump) = std::env::var("ZOPRO_DUMP") else {
        eprintln!("set ZOPRO_DUMP: skipped");
        return;
    };
    let dir = Path::new(&dump).join(shape);
    if !dir.exists() {
        eprintln!("no {shape} dump: skipped");
        return;
    }
    let dump = Dump::read(&dir).expect("dump");
    let bytes = pack(&dump.parts(key_name)).expect("pack");
    let mut prover = Prover::load(&bytes).expect("load");
    assert_eq!(prover.name(), key_name);
    let metadata = Metadata::parse(&dump.metadata).expect("Zolana metadata");
    assert_eq!(prover.key_sha256(), metadata.key_sha256);
    let values = prover.witness(request).expect("witness");
    assert_eq!(
        witness::encode(&values),
        std::fs::read(dir.join("witness")).expect("the dump's witness"),
        "the witness vector"
    );
    let result = prover.prove(request).expect("prove");
    assert_eq!(result.proof.len(), 164);
    assert_eq!(result.public_inputs.len(), 44);
    // The gnark key's sha256, which the SDKs check against their pins.
    assert_eq!(
        result.proof_json["provingKeySha256"].as_str(),
        Some(metadata.key_sha256.as_str())
    );
    let proof = proof_json::from_json(&result.proof_json).expect("proof json");
    assert!(prover.verify(&proof, &values[..1]));
    // For gnark's verifier: the raw proof beside the dump.
    std::fs::write(dir.join("proof"), proof.to_raw()).expect("write the proof");
    eprintln!("{shape}: proved in {} ms", result.prove_ms);
}

#[test]
fn transfer_2x2() {
    check(
        "2x2",
        "transfer_confidential_2_2",
        include_str!("fixtures/transfer-2x2.request.json"),
    );
}

#[test]
fn merge_24x1() {
    check(
        "merge-24x1",
        "merge_24_1",
        include_str!("fixtures/merge-24x1.request.json"),
    );
}
