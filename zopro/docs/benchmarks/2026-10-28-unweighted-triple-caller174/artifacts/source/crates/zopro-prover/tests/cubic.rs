//! A small gnark circuit end to end: X³ + X + 5 = Y with X not zero
//! (`tools/gnark`'s test writes the dump in tests/fixtures/cubic). The key
//! packs, loads, proves an assignment by name, refuses a wrong one, keeps
//! its metadata, and loads again from its raw form, in place too.
use std::path::Path;

use zopro_bn254::Fr;
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

#[test]
fn reads_legacy_key_and_raw_image() {
    let current = key();
    let (_, head_end) = zopro_prover::zopro_core::artifact::Artifact::head(&current).unwrap();
    let mut legacy = current[..564].to_vec();
    legacy[16..20].copy_from_slice(&2u32.to_le_bytes());
    let mut at = 564;
    for _ in 0..6 {
        let len = u64::from_le_bytes(current[at..at + 8].try_into().unwrap()) as usize;
        at += 8;
        legacy.extend_from_slice(&u32::try_from(len).unwrap().to_le_bytes());
        legacy.extend_from_slice(&current[at..at + len]);
        at += len;
    }
    assert_eq!(at, head_end);
    legacy.extend_from_slice(&current[at..]);
    let mut prover = Prover::load(&legacy).unwrap();
    let witness = prover.witness(&assignment(3, 35)).unwrap();
    assert!(prover.prove(&witness).is_ok());

    let mut raw4 = Vec::new();
    Prover::write_image(&legacy, &mut raw4).unwrap();
    let head_len = u64::from_le_bytes(raw4[16..24].try_into().unwrap()) as usize;
    let (artifact, _) = zopro_prover::zopro_core::artifact::Artifact::head(&raw4[24..24 + head_len]).unwrap();
    let vectors_at = (24 + head_len).next_multiple_of(64);
    let vectors_len: usize = artifact.points.iter().enumerate()
        .map(|(i, &n)| n as usize * if i == 4 { 128 } else { 64 }).sum();
    let tail = vectors_at + vectors_len;
    let solver_len = u64::from_le_bytes(raw4[tail..tail + 8].try_into().unwrap());
    let (_, legacy_head_len) = zopro_prover::zopro_core::artifact::Artifact::head(&legacy).unwrap();
    let mut raw3 = b"zopro-raw3\0\0\0\0\0\0".to_vec();
    raw3.extend_from_slice(&(legacy_head_len as u32).to_le_bytes());
    raw3.extend_from_slice(&legacy[..legacy_head_len]);
    raw3.resize(raw3.len().next_multiple_of(64), 0);
    raw3.extend_from_slice(&raw4[vectors_at..tail]);
    raw3.extend_from_slice(&u32::try_from(solver_len).unwrap().to_le_bytes());
    raw3.extend_from_slice(&raw4[tail + 8..]);
    let (mut copied, source) = Prover::load_cacheable(&raw3).unwrap();
    assert!(source.is_none());
    assert!(copied.prove(&witness).is_ok());
    let mut mapped = Prover::load_image(std::sync::Arc::new(raw3)).unwrap();
    assert!(mapped.prove(&witness).is_ok());
}

#[test]
fn writes_dump_directly_without_intermediate_point_copies() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cubic");
    let dump = Dump::read(&dir).unwrap();
    let mut direct = Vec::new();
    Prover::write_dump_image(&dump.parts("cubic"), &mut direct).unwrap();
    let mut packed = Vec::new();
    Prover::write_image(&key(), &mut packed).unwrap();
    assert_eq!(direct, packed);
    let head_len = u64::from_le_bytes(direct[16..24].try_into().unwrap()) as usize;
    let (header, _) = zopro_prover::zopro_core::artifact::Artifact::head(&direct[24..24 + head_len]).unwrap();
    assert!(header.program.is_empty() && header.coefficients.is_empty());
    let vectors_len: usize = header.points.iter().enumerate()
        .map(|(i, &n)| n as usize * if i == 4 { 128 } else { 64 }).sum();
    let mut prover = Prover::load_image(std::sync::Arc::new(direct)).unwrap();
    let witness = prover.witness(&assignment(3, 35)).unwrap();
    assert!(prover.prove(&witness).is_ok());

    let mut damaged = packed;
    let solver_at = (24 + head_len).next_multiple_of(64) + vectors_len + 8;
    damaged[solver_at] ^= 0xff;
    assert!(Prover::load_image(std::sync::Arc::new(damaged)).is_err());
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
