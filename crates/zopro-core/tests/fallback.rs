//! A GPU key that lets its GPU go proves on the CPU. The cubic circuit's
//! witness (zopro-prover's fixture) proved on the GPU, then on the CPU after
//! `drop_gpu`, every proof verified. With its points in place in an image
//! of the key, the CPU proves with the image's; with points decoded from
//! the key file's bytes, which only the GPU holds, `drop_gpu` fails and the
//! key keeps its GPU, and `drop_gpu_with` takes the points again. The GPU
//! takes half of G2, so the CPU's share comes from both kinds of store.
//! With `--features metal` or `--features wgpu`, skipped without a GPU.
#![cfg(gpu_any)]
use std::path::Path;

use zopro_core::{
    artifact::{pack, Artifact},
    decode_frs,
    dump::Dump,
    groth16::assemble,
    verify::Proof,
    Image, Key, Points,
};

#[test]
fn proves_on_the_cpu_after_the_gpu() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../zopro-prover/tests/fixtures/cubic");
    let read = |name: &str| std::fs::read(dir.join(name)).expect(name);
    let key_file = pack(&Dump::read(&dir).expect("the cubic dump").parts("cubic")).expect("pack");
    let artifact = Artifact::parse(&key_file).expect("parse");
    let (verifying_key, fixed) = (
        artifact.verifying_key().expect("verifying key"),
        artifact.fixed_points().expect("fixed points"),
    );
    let (witness, public) = (read("witness"), decode_frs(&read("public")));
    let check = |key: &Key, on: &str| {
        let parts = key.solve(&witness).unwrap_or_else(|e| panic!("{on}: {e}"));
        let proof = Proof::from_raw(&assemble(&parts, &fixed)).expect("proof");
        assert!(
            verifying_key.verify(&proof, &public),
            "{on}: the proof verifies"
        );
    };
    // SAFETY: this binary's only test, the only reader of the variable.
    unsafe { std::env::set_var("ZOPRO_G2_GPU_SHARE", "0.5") };

    // The points in place in an image of the key, as a host maps it.
    let mut image = Vec::new();
    let ranges = artifact.vectors().expect("vectors").map(|vector| {
        image.extend_from_slice(&vector);
        (image.len() - vector.len(), vector.len())
    });
    let image: Image = std::sync::Arc::new(image);
    let mut key = artifact
        .key(Points::Mapped(&image, ranges), None)
        .expect("key");
    if !key.has_gpu() {
        eprintln!("no GPU: skipped");
        return;
    }
    check(&key, "GPU, image");
    key.drop_gpu()
        .expect("the CPU proves with the image's points");
    assert!(!key.has_gpu());
    check(&key, "CPU after the GPU, image");

    // The points decoded from the key file: the GPU holds the only copy.
    let mut key = artifact.key(artifact.points(), None).expect("key");
    assert!(key.has_gpu());
    assert!(key.drop_gpu().is_err(), "no points for the CPU");
    assert!(key.has_gpu(), "the key keeps its GPU");
    check(&key, "GPU, kept");
    let z = &artifact.z[..artifact.z.len() - 32];
    let other = Points::Compressed([artifact.a, artifact.b, artifact.k, z, artifact.b2]);
    assert!(key.drop_gpu_with(other).is_err(), "another key's points");
    assert!(key.has_gpu(), "the key keeps its GPU");
    key.drop_gpu_with(artifact.points())
        .expect("the CPU proves with the key file's points");
    assert!(!key.has_gpu());
    check(&key, "CPU after the GPU, key file");
}
