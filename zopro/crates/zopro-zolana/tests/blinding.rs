//! One request proved twice with its zopro key: the two proofs both verify
//! and differ in each of Ar, Bs and Krs, every proof blinded afresh. The
//! key is the key set's 2 → 2 transfer key (`ZOPRO_KEYS_DIR=<dir>` holding
//! `transfer_confidential_2_2.zopro`, as `tools/bench.sh` downloads it and
//! `tools/zolana/zopro-keys.lock` pins it). Skipped without it.
use std::path::Path;

use zopro_zolana::{Prover, proof_json};

#[test]
fn transfer_2x2_proofs_differ() {
    let Ok(keys) = std::env::var("ZOPRO_KEYS_DIR") else {
        eprintln!("set ZOPRO_KEYS_DIR: skipped");
        return;
    };
    let path = Path::new(&keys).join("transfer_confidential_2_2.zopro");
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut prover = Prover::load(&bytes).expect("load");
    let request = include_str!("fixtures/transfer-2x2.request.json");
    let values = prover.witness(request).expect("witness");

    let first = prover.prove(request).expect("prove");
    let second = prover.prove(request).expect("prove");
    assert_eq!(first.public_inputs, second.public_inputs);
    for proof in ["ar", "bs", "krs"] {
        assert_ne!(
            first.proof_json[proof], second.proof_json[proof],
            "{proof} is blinded afresh"
        );
    }
    for result in [&first, &second] {
        let proof = proof_json::from_json(&result.proof_json).expect("proof json");
        assert!(prover.verify(&proof, &values[..1]));
    }
    eprintln!(
        "proved twice in {} and {} ms, GPU {}",
        first.prove_ms,
        second.prove_ms,
        prover.prover().has_gpu()
    );
}
