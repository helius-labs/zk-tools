use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/stream-work-census176";
const C: &str = "/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc";
const W: &str = "/home/ubuntu/src/zopro-perf-work/word-export176/target-candidate/release/deps";
const T: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("plan.json").exists());
    let mut pins = BTreeMap::new();
    let mut pin = |p: PathBuf| {
        pins.insert(p.display().to_string(), hash(&p));
    };
    for p in [C, X] {
        pin(PathBuf::from(p));
    }
    for f in ["model.rs", "design.md", "prepare.rs", "prepare"] {
        pin(r.join(f));
    }
    for d in [W, T] {
        for e in fs::read_dir(d).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|e| e == "rlib") {
                pin(p);
            }
        }
    }
    let s = Path::new("/home/ubuntu/src/zk-tools/zopro");
    for f in["crates/zopro-core/src/msm.rs","crates/zopro-core/src/msm/g2_borrowed.rs","crates/zopro-core/src/simd.rs","crates/zopro-bn254/src/ifma8.rs","crates/zopro-bn254/src/fq2.rs","docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"]{pin(s.join(f));}
    for p in [
        "/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/analysis.json",
        "/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/review.json",
        "/home/ubuntu/src/zopro-perf-work/primary-instruction-census176/analysis.json",
        "/home/ubuntu/src/zopro-perf-work/primary-instruction-census176/report.md",
        "/home/ubuntu/src/zopro-perf-work/instruction-census176/history-audit.md",
    ] {
        pin(PathBuf::from(p));
    }
    let mut argv = vec![
        C.to_string(),
        "--edition=2021".into(),
        "-O".into(),
        r.join("model.rs").display().to_string(),
    ];
    for (name, path) in [
        (
            "serde_json",
            format!("{T}/libserde_json-cdebb9338c2784ad.rlib"),
        ),
        ("sha2", format!("{T}/libsha2-16b8f2c03bdf5faa.rlib")),
        (
            "ark_bn254",
            format!("{W}/libark_bn254-d28868a71f06f5b5.rlib"),
        ),
        ("ark_ec", format!("{W}/libark_ec-59ebd5b49f9b3011.rlib")),
        ("ark_ff", format!("{W}/libark_ff-3a8e68dc8045d2bd.rlib")),
    ] {
        assert!(Path::new(&path).exists());
        argv.extend(["--extern".into(), format!("{name}={path}")]);
    }
    argv.extend([
        "-L".into(),
        format!("dependency={W}"),
        "-L".into(),
        format!("dependency={T}"),
        "-o".into(),
        r.join("model").display().to_string(),
    ]);
    let plan = json!({"version":1,"source_pins":r.join("pins.json"),"environment":{},"steps":[{"kind":"command","name":"build-model","argv":argv,"timeout_seconds":600},{"kind":"command","name":"run-model","argv":[r.join("model")],"timeout_seconds":600}]});
    fs::write(
        r.join("pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        r.join("plan.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    println!(
        "Frozen2normal-Rustmodelcommands/{}pins; no native prover/profile/clock/SDK/proof",
        pins.len()
    );
    let _ = Value::Null;
}
