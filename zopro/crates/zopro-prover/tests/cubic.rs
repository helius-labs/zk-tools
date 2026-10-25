//! A small gnark circuit end to end: X³ + X + 5 = Y with X not zero
//! (`tools/gnark`'s test writes the dump in tests/fixtures/cubic). The key
//! packs, loads, proves an assignment by name, refuses a wrong one, keeps
//! its metadata, loads again from its raw form, in place too, and from
//! vectors decompressed in runs, and proves on the CPU once its GPU is let
//! go.
use std::path::Path;

use zopro_bn254::Fr;
use zopro_prover::{
    Assignment, Error, Prover,
    zopro_core::{
        artifact::{Artifact, kernel_points, pack},
        dump::Dump,
    },
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

/// A page without a thread pool decompresses the key's vectors in runs of
/// points across workers and loads the key from the joined results: the
/// same raw form as a load of the key file, and a key that proves.
#[test]
fn loads_vectors_decompressed_in_runs() {
    let key = key();
    let (prover, source) = Prover::load_cacheable(&key).expect("load");
    let whole = source.expect("a key file's raw source").export(&prover);
    let artifact = Artifact::parse(&key).expect("parse");
    let mut i = 0;
    let vectors = artifact.vector_ranges(&key).map(|(at, len)| {
        let (g2, size) = (i == 4, if i == 4 { 64 } else { 32 });
        i += 1;
        key[at..at + len]
            .chunks(3 * size)
            .flat_map(|run| kernel_points(run, g2).expect("decompress"))
            .collect::<Vec<u8>>()
    });
    let mut short = vectors.clone();
    short[1].truncate(short[1].len() - 64);
    assert!(Prover::load_decompressed(&key, short).is_err());
    let (mut prover, source) = Prover::load_decompressed(&key, vectors).expect("load");
    let witness = prover.witness(&assignment(3, 35)).expect("witness");
    assert!(prover.prove(&witness).is_ok());
    assert_eq!(source.export(&prover), whole, "the raw form");
}

#[test]
fn proves_from_its_image_in_place() {
    let key = key();
    let (prover, source) = Prover::load_cacheable(&key).expect("load");
    let mut image = Vec::new();
    Prover::write_image(&key, &mut image).expect("write");
    assert_eq!(image, source.expect("a key file's raw source").export(&prover), "the raw form");
    let image: zopro_prover::zopro_core::Image = std::sync::Arc::new(image);
    let mut again = Prover::load_image(image.clone()).expect("load the image");
    let witness = again.witness(&assignment(3, 35)).expect("witness");
    assert!(again.prove(&witness).is_ok());
    let truncated = (*image).as_ref()[..(*image).as_ref().len() - 1].to_vec();
    assert!(Prover::load_image(std::sync::Arc::new(truncated)).is_err());
}

/// A prover whose GPU is let go proves on the CPU (a GPU that fails, or
/// whose proof does not verify, takes this path in `prove`): from its image
/// with the image's points; from the key file's bytes, which only the GPU
/// holds, with the points given again. Skipped without a GPU.
#[test]
fn proves_on_the_cpu_after_the_gpu() {
    let key = key();
    let mut image = Vec::new();
    Prover::write_image(&key, &mut image).expect("write");
    let mut prover = Prover::load_image(std::sync::Arc::new(image)).expect("load the image");
    if !prover.has_gpu() {
        eprintln!("no GPU: skipped");
        return;
    }
    let witness = prover.witness(&assignment(3, 35)).expect("witness");
    let check = |prover: &mut Prover, on: &str| {
        let proved = prover
            .prove(&witness)
            .unwrap_or_else(|e| panic!("{on}: {e}"));
        assert!(prover.verify(&proved.proof, &[Fr::from(35u64)]), "{on}");
    };
    check(&mut prover, "GPU, image");
    prover
        .drop_gpu()
        .expect("the CPU proves with the image's points");
    assert!(!prover.has_gpu());
    check(&mut prover, "CPU after the GPU, image");

    let mut prover = Prover::load(&key).expect("load");
    assert!(prover.drop_gpu().is_err(), "no points for the CPU");
    assert!(prover.has_gpu(), "the prover keeps its GPU");
    check(&mut prover, "GPU, kept");
    let artifact = Artifact::parse(&key).expect("parse");
    prover
        .drop_gpu_with(artifact.points())
        .expect("the CPU proves with the key file's points");
    assert!(!prover.has_gpu());
    check(&mut prover, "CPU after the GPU, key file");
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
