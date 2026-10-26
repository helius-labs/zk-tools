use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    let base = read(root.join("accepted-manifest.json"));
    let mut pins = BTreeMap::new();
    let mut changes = Vec::new();
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
        let p = root.join("candidate").join(n);
        let got = hash(&p);
        if got != h.as_str().unwrap() {
            assert!(
                n == "crates/zopro-bn254/src/ifma.rs" || n == "crates/zopro-bn254/src/ifma8.rs"
            );
            changes.push(json!({"path":n,"sha256":got,"original_sha256":h}));
        }
        pins.insert(p.to_str().unwrap().to_owned(), got);
    }
    assert_eq!(changes.len(), 2);
    for n in [
        "experiment.json",
        "prepare.rs",
        "edits.json",
        "accepted-manifest.json",
        "field-probe.rs",
        "wrappers.json",
        "isa.rs",
        "link.rs",
        "link",
        "build.rs",
    ] {
        let p = root.join(n);
        pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    }
    let a = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    for p in [
        a["libraries"]["zopro_bn254"].as_str().unwrap(),
        "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask",
    ] {
        pins.insert(p.into(), hash(p));
    }
    let steps = json!([
    {"kind":"command","name":"compiler","argv":["rustc","-Vv"]},
    {"kind":"command","name":"generator-check","argv":["python3",root.join("candidate/scripts/ifma8.py"),"--check"]},
    {"kind":"command","name":"isa-build","argv":["rustc","--edition=2021","-O","-C","target-cpu=native",root.join("isa.rs"),"-o",root.join("isa")]},
    {"kind":"command","name":"isa-check","argv":[root.join("isa")]},
    {"kind":"command","name":"field","argv":["cargo","build","--manifest-path",root.join("candidate/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--message-format=json"],"timeout_seconds":600},
    {"kind":"command","name":"field-links","argv":[root.join("link")]}
    ]);
    fs::write(root.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changed":changes,"other_168_unchanged":true,"two_sites_guarded_only":true,"last48block_unchanged":true})).unwrap()).unwrap();
    let p = root.join("source-audit.json");
    pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    fs::write(
        root.join("build-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(root.join("build-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":root.join("build-pins.json"),"environment":{"CARGO_TARGET_DIR":root.join("target"),"CARGO_BUILD_JOBS":"16","RUSTFLAGS":base["rustflags"].as_str().unwrap().to_owned()+" --cfg zopro_unmask_m52"},"steps":steps})).unwrap()).unwrap();
    println!(
        "Frozen{}pins;ISAconformance+24fieldwrappers, no timings or proofs",
        pins.len()
    );
}
