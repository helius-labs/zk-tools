//! A zopro key packed from a dump (`ZOPRO_DUMP=<dir>`, the 2 -> 2 shape),
//! read back, and used to prove the dump's witness: the proof verifies
//! with the key's own verifying key. Skipped without a dump.
use std::path::Path;

use zopro_core::{
    artifact::{pack, Artifact, Parts},
    groth16::assemble,
    verify::Proof,
};

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

/// The dump's head: the key image head the Go bridge writes.
fn parts<'a>(head: &'a [u8], files: &'a [Vec<u8>; 7], names: &'a [Vec<String>; 2]) -> Parts<'a> {
    assert_eq!(&head[..16], b"zolana-key-head1");
    let word = |i: usize| u32::from_le_bytes(head[16 + 4 * i..20 + 4 * i].try_into().unwrap());
    let vk_len = u32::from_le_bytes(head[604..608].try_into().unwrap()) as usize;
    let mut digest = [0u8; 32];
    digest.copy_from_slice(&head[60..92]);
    let [a, b, k, z, b2, program, coefficients] = files;
    Parts {
        name: "",
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
    }
}

#[test]
fn packs_loads_and_proves() {
    let Ok(dump) = std::env::var("ZOPRO_DUMP") else {
        eprintln!("set ZOPRO_DUMP: skipped");
        return;
    };
    let dir = Path::new(&dump).join("2x2");
    let head = read(&dir, "head");
    let files = ["a", "b", "k", "z", "b2", "program", "coefficients"].map(|n| read(&dir, n));
    let names = [lines(&dir, "public-names"), lines(&dir, "secret-names")];
    let mut parts = parts(&head, &files, &names);
    parts.name = "transfer_confidential_2_2";
    let bytes = pack(&parts).expect("pack");
    eprintln!("zopro key: {} bytes", bytes.len());

    let artifact = Artifact::parse(&bytes).expect("parse");
    assert_eq!(artifact.name, "transfer_confidential_2_2");
    assert_eq!((artifact.inputs, artifact.outputs), (2, 2));
    assert_eq!(
        artifact.input_names().len(),
        (artifact.public - 1 + artifact.secret) as usize
    );
    assert_eq!(artifact.input_names()[0], "Public_PublicInputHash");
    let vectors = artifact.vectors().expect("vectors");
    assert_eq!(vectors[0], files[0], "A decompresses to what was packed");
    assert_eq!(vectors[4], files[4], "B2 decompresses to what was packed");
    let key = artifact.key(&vectors, None).expect("key");
    let image = key.solver_image().expect("solver image");
    drop(
        artifact
            .key(&vectors, Some(&image))
            .expect("key from the cached image"),
    );
    let verifying_key = artifact.verifying_key().expect("verifying key");
    let fixed = artifact.fixed_points().expect("fixed points");

    let witness = read(&dir, "witness");
    let proof_parts = key.solve(&witness).expect("solve");
    let proof = Proof::from_raw(&assemble(&proof_parts, &fixed)).expect("proof");
    let public = zopro_core::decode_frs(&read(&dir, "public"));
    assert!(verifying_key.verify(&proof, &public));
}
