//! A zopro key packed from a dump (`ZOPRO_DUMP=<dir>`, the 2 -> 2 shape),
//! read back, and used to prove the dump's witness: the proof verifies
//! with the key's own verifying key. A key whose fixed points are not the
//! verifying key's does not pack. Skipped without a dump.
use std::path::Path;

use zopro_core::{
    artifact::{pack, Artifact},
    dump::Dump,
    groth16::assemble,
    verify::Proof,
    Points,
};

fn read(dir: &Path, name: &str) -> Vec<u8> {
    std::fs::read(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

#[test]
fn packs_loads_and_proves() {
    let Ok(dump) = std::env::var("ZOPRO_DUMP") else {
        eprintln!("set ZOPRO_DUMP: skipped");
        return;
    };
    let dir = Path::new(&dump).join("2x2");
    let dump = Dump::read(&dir).expect("dump");
    let bytes = pack(&dump.parts("transfer_confidential_2_2")).expect("pack");
    eprintln!("zopro key: {} bytes", bytes.len());

    let artifact = Artifact::parse(&bytes).expect("parse");
    assert_eq!(artifact.name, "transfer_confidential_2_2");
    assert_eq!(artifact.metadata, &dump.metadata[..]);
    assert_eq!(
        artifact.input_names().len(),
        (artifact.public - 1 + artifact.secret) as usize
    );
    let vectors = artifact.vectors().expect("vectors");
    assert_eq!(
        vectors[0], dump.vectors[0],
        "A decompresses to what was packed"
    );
    assert_eq!(
        vectors[4], dump.vectors[4],
        "B2 decompresses to what was packed"
    );
    let key = artifact.key(artifact.points(), None).expect("key");
    let image = key.solver_image().expect("solver image");
    let [a, b, k, z, b2] = &vectors;
    let cached = artifact
        .key(Points::Kernel([a, b, k, z, b2]), Some(&image))
        .expect("key from the cached vectors and image");
    let verifying_key = artifact.verifying_key().expect("verifying key");
    let fixed = artifact.fixed_points().expect("fixed points");

    let witness = read(&dir, "witness");
    let proof_parts = key.solve(&witness).expect("solve");
    let proof = Proof::from_raw(&assemble(&proof_parts, &fixed)).expect("proof");
    let public = zopro_core::decode_frs(&read(&dir, "public"));
    assert!(verifying_key.verify(&proof, &public));
    let proof_parts = cached.solve(&witness).expect("solve");
    let proof = Proof::from_raw(&assemble(&proof_parts, &fixed)).expect("proof");
    assert!(verifying_key.verify(&proof, &public));

    // A delta that is not the verifying key's would unblind proofs.
    let mut tampered = dump.parts("transfer_confidential_2_2");
    let mut fixed = dump.head.fixed.clone();
    fixed[128..192].copy_from_slice(&dump.head.fixed[..64]);
    tampered.fixed = &fixed;
    assert!(pack(&tampered).is_err());
}
