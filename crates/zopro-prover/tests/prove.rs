//! Proofs of the dump requests (the web test data the Go dump test proves)
//! through a zopro key packed from a dump
//! (`ZOPRO_DUMP=<dir>` with the `2x2` and `merge-24x1` shapes, as the Go dump
//! test writes them from the same requests): the witness vector is the
//! dump's byte for byte, the proof verifies, and a raw proof is left in the
//! dump for gnark's own verifier to referee. Skipped without a dump.
use std::path::Path;

use zopro_core::{
    artifact::{Parts, pack},
    verify::Proof,
};
use zopro_prover::{Prover, witness};

fn read(dir: &Path, name: &str) -> Vec<u8> {
    std::fs::read(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn lines(dir: &Path, name: &str) -> Vec<String> {
    String::from_utf8(read(dir, name))
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

/// A zopro key file from a dump directory.
fn packed(dir: &Path, name: &str) -> Vec<u8> {
    let head = read(dir, "head");
    let word = |i: usize| u32::from_le_bytes(head[16 + 4 * i..20 + 4 * i].try_into().unwrap());
    let vk_len = u32::from_le_bytes(head[604..608].try_into().unwrap()) as usize;
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&head[60..92]);
    let files = ["a", "b", "k", "z", "b2", "program", "coefficients"].map(|n| read(dir, n));
    let names = [lines(dir, "public-names"), lines(dir, "secret-names")];
    let [a, b, k, z, b2, program, coefficients] = &files;
    pack(&Parts {
        name,
        inputs: word(0),
        outputs: word(1),
        flags: word(2),
        public: word(3),
        secret: word(4),
        constraints: word(5),
        digest,
        fixed: &head[92..540],
        domain: &head[540..604],
        verifying_key: &head[608..608 + vk_len],
        public_names: &names[0],
        secret_names: &names[1],
        a,
        b,
        k,
        z,
        b2,
        program,
        coefficients,
    })
    .expect("pack")
}

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
    let bytes = packed(&dir, key_name);
    let mut prover = Prover::load(&bytes).expect("load");
    assert_eq!(prover.name(), key_name);
    let values = prover.witness(request).expect("witness");
    assert_eq!(
        witness::encode(&values),
        read(&dir, "witness"),
        "the witness vector"
    );
    let result = prover.prove(request).expect("prove");
    assert_eq!(result.proof.len(), 164);
    assert_eq!(result.public_inputs.len(), 44);
    let proof = Proof::from_json(&result.proof_json).expect("proof json");
    // The gnark key's sha256, which the SDKs check against their pins.
    assert_eq!(
        result.proof_json["provingKeySha256"].as_str(),
        Some(zopro_core::verify::hex_of(&read(&dir, "head")[60..92]).as_str())
    );
    assert!(prover.verify(&proof, &values[..1]));
    // For gnark's verifier: the raw proof beside the dump.
    let mut raw = Vec::with_capacity(256);
    for p in [proof.ar, proof.krs] {
        raw.extend_from_slice(&ark_ff::BigInteger::to_bytes_be(
            &ark_ff::PrimeField::into_bigint(p.x),
        ));
        raw.extend_from_slice(&ark_ff::BigInteger::to_bytes_be(
            &ark_ff::PrimeField::into_bigint(p.y),
        ));
    }
    for c in [proof.bs.x.c1, proof.bs.x.c0, proof.bs.y.c1, proof.bs.y.c0] {
        raw.extend_from_slice(&ark_ff::BigInteger::to_bytes_be(
            &ark_ff::PrimeField::into_bigint(c),
        ));
    }
    std::fs::write(dir.join("proof"), &raw).expect("write the proof");
    eprintln!("{shape}: proved in {} ms", result.prove_ms);
}

#[test]
fn transfer_2x2() {
    check(
        "2x2",
        "transfer_confidential_2_2",
        include_str!("fixtures/transfer-2x2.dump.request.json"),
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
