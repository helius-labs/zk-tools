use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-cancel176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("plan.json").exists());
    let old = "/home/ubuntu/src/zopro-perf-work/triple-affine-screen/model.rs";
    let bytes = fs::read(old).unwrap();
    fs::write(format!("{R}/original.rs"), &bytes).unwrap();
    let mut pins = BTreeMap::new();
    pins.insert(old.to_owned(), hash(old));
    let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    for (p, h) in read(&manifest)["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_owned());
    }
    pins.insert(manifest.clone(), hash(&manifest));
    let deps = "/home/ubuntu/src/zopro-perf-work/word-export176/target-candidate/release/deps";
    let tool = "/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
    let mut libraries = BTreeMap::new();
    for (name, file) in [
        ("ark_bn254", "libark_bn254-d28868a71f06f5b5.rlib"),
        ("ark_ec", "libark_ec-59ebd5b49f9b3011.rlib"),
        ("ark_ff", "libark_ff-3a8e68dc8045d2bd.rlib"),
    ] {
        libraries.insert(name, format!("{deps}/{file}"));
    }
    libraries.insert(
        "serde_json",
        format!("{tool}/libserde_json-cdebb9338c2784ad.rlib"),
    );
    for e in fs::read_dir(deps).unwrap() {
        let p = e.unwrap().path();
        if p.extension().is_some_and(|e| e == "rlib" || e == "so") {
            pins.insert(p.display().to_string(), hash(p));
        }
    }
    for p in libraries.values() {
        pins.insert(p.clone(), hash(p));
    }
    let base = || {
        let mut a = vec![
            "/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc".to_owned(),
            "--edition=2021".into(),
            "-O".into(),
            "-C".into(),
            "target-cpu=native".into(),
            "-L".into(),
            format!("dependency={deps}"),
            "-L".into(),
            format!("dependency={tool}"),
        ];
        for (n, p) in &libraries {
            a.extend(["--extern".into(), format!("{n}={p}")]);
        }
        a.push(format!("{R}/study.rs"));
        a
    };
    let mut test = base();
    test.extend(["--test".into(), "-o".into(), format!("{R}/algebra-tests")]);
    let mut study = base();
    study.extend(["-o".into(), format!("{R}/study")]);
    let steps = json!([{"kind":"command","name":"build-tests","argv":test,"timeout_seconds":300},{"kind":"command","name":"algebra-tests","argv":[format!("{R}/algebra-tests"),"cancel::cancel_tests::","--test-threads=1","--nocapture"],"timeout_seconds":120,"unfiltered":false,"required_tests":["cancel::cancel_tests::exhaustive_toy_h_only_and_opposite_domain","cancel::cancel_tests::changed_formula_field_ledgers"]},{"kind":"command","name":"build-study","argv":study,"timeout_seconds":300},{"kind":"command","name":"curve-study","argv":[format!("{R}/study")],"timeout_seconds":600}]);
    for name in [
        "derivation.md",
        "design.md",
        "prepare.rs",
        "prepare",
        "original.rs",
        "cancel.rs",
        "study.rs",
    ] {
        let p = format!("{R}/{name}");
        pins.insert(p.clone(), hash(p));
    }
    fs::write(
        format!("{R}/pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        format!("{R}/plan.json"),
        serde_json::to_vec_pretty(
            &json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":steps}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("Frozen4commands/new2tests/old3filtered/11264BNattempts+496batch triples; original field/prover unchanged");
}
