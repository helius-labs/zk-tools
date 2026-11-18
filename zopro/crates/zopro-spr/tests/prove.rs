//! Proofs of the dump requests through a zopro key packed from a dump
//! (`ZOPRO_DUMP=<dir>` with the `2x2` and `merge-24x1` shapes, as
//! `tools/spr` writes them from the same requests): the witness vector is
//! the dump's byte for byte, the proof verifies and reports the gnark key's
//! sha256, and a raw proof is left in the dump for gnark's own verifier to
//! referee. Skipped without a dump.
use std::path::Path;

use zopro_spr::{
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

/// A deployed key can be staged independently of the gnark dump.
#[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
#[test]
#[ignore = "needs ZOPRO_TEST_KEY"]
fn typed_transfer_loads_and_reuses_the_cached_image() {
    use zopro_spr::{Error, Fr, ProvingInput, zopro_prover::disk::image_path};

    let source = std::env::var_os("ZOPRO_TEST_KEY").expect("path to a 2→2 zopro key");
    let directory = tempfile::tempdir().unwrap();
    let key = directory.path().join("transfer_confidential_2_2.zopro");
    std::fs::copy(source, &key).unwrap();
    let request = include_str!("fixtures/transfer-2x2.request.json");
    let mut input = ProvingInput::from_json(request).unwrap();

    // SAFETY: the files are confined to the private temp directory and each
    // mapped prover is dropped before we replace or remove its image.
    let mut prover = unsafe { Prover::load_cached(&key) }.unwrap();
    assert_eq!(
        prover.witness_input(&input).unwrap(),
        prover.witness(request).unwrap()
    );
    let result = prover.prove_input(&input).unwrap();
    let proof = proof_json::from_json(&result.proof_json).unwrap();
    let witness = prover.witness_input(&input).unwrap();
    assert!(prover.verify(&proof, &witness[..1]));
    let ProvingInput::Transfer(transfer) = &mut input else {
        panic!("expected transfer");
    };
    transfer.blinding_seed = Fr::from(0);
    assert_eq!(prover.witness_input(&input), Err(Error::Request));
    drop(prover);

    let path = image_path(&key);
    assert!(path.is_file());
    // A hit can be opened without reading the original compressed key.
    std::fs::remove_file(key.clone()).unwrap();
    // SAFETY: the image is still in the private directory and is immutable.
    let cached = unsafe { Prover::load_cached(&key) }.unwrap();
    assert_eq!(cached.witness(request).unwrap(), witness);
}
