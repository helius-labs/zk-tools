//! A request proved with its zopro key loaded as the mobile SDK loads it,
//! from the key's image read in place: on the GPU, then on the CPU once the
//! GPU is let go, both proofs verified. The key is the key set's 2 → 2
//! transfer key (`ZOPRO_KEYS_DIR`, as tests/blinding.rs takes it). Skipped
//! without it, and without a GPU.
use std::path::Path;

use zopro_zolana::{Prover, proof_json, zopro_prover};

#[test]
fn transfer_2x2_proves_on_the_cpu_after_the_gpu() {
    let Ok(keys) = std::env::var("ZOPRO_KEYS_DIR") else {
        eprintln!("set ZOPRO_KEYS_DIR: skipped");
        return;
    };
    let path = Path::new(&keys).join("transfer_confidential_2_2.zopro");
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let mut image = Vec::new();
    zopro_prover::Prover::write_image(&bytes, &mut image).expect("the key's image");
    let loaded = zopro_prover::Prover::load_image(std::sync::Arc::new(image)).expect("load");
    let mut prover = Prover::new(loaded).expect("a Zolana key");
    if !prover.prover().has_gpu() {
        eprintln!("no GPU: skipped");
        return;
    }
    let request = include_str!("fixtures/transfer-2x2.request.json");
    let values = prover.witness(request).expect("witness");

    let gpu = prover.prove(request).expect("prove on the GPU");
    prover
        .prover_mut()
        .drop_gpu()
        .expect("the CPU proves with the image's points");
    assert!(!prover.prover().has_gpu());
    let cpu = prover.prove(request).expect("prove on the CPU");
    let public = &values[..prover.prover().public_inputs()];
    for result in [&gpu, &cpu] {
        let proof = proof_json::from_json(&result.proof_json).expect("proof json");
        assert!(prover.verify(&proof, public));
    }
    eprintln!(
        "proved on the GPU in {} ms, on the CPU in {} ms",
        gpu.prove_ms, cpu.prove_ms
    );
}
