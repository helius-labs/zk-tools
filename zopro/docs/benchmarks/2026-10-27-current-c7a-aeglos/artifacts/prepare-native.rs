use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-current-c7a-20261027";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const M: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
const X: &str = "/tmp/zopro-xtask-fast-screen-target/release/xtask";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let r = Path::new(R);
    let m = read(M);
    assert_eq!(m["adopted"], true);
    let src = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(src.len(), 171);
    for (f, h) in src {
        assert_eq!(hash(Path::new(P).join(f)), h.as_str().unwrap());
    }
    let out = Command::new(X)
        .args(["prepare", M, P])
        .arg(r.join("current"))
        .output()
        .unwrap();
    fs::write(r.join("prepare.log"), &out.stdout).unwrap();
    fs::write(r.join("prepare.stderr.log"), &out.stderr).unwrap();
    assert!(out.status.success());
    for (f, h) in src {
        assert_eq!(hash(r.join("current").join(f)), h.as_str().unwrap());
    }
    let flags = m["rustflags"]
        .as_str()
        .unwrap()
        .replace("target-cpu=native", "target-cpu=znver4");
    assert_ne!(flags, m["rustflags"].as_str().unwrap());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("current/.xtask/source-pins.json"))).unwrap();
    for p in [
        M,
        X,
        m["standard_cli_source"].as_str().unwrap(),
        m["standard_cli_helper"].as_str().unwrap(),
    ] {
        pins.insert(p.into(), hash(p));
    }
    for name in ["design.md", "prepare-native.rs", "prepare-native"] {
        let p = r.join(name);
        pins.insert(p.display().to_string(), hash(&p));
    }
    save(
        r.join("source-audit.json"),
        &json!({"accepted_manifest":M,"exact_source_files":171,"source_changed":false,"production_changed":false,"source_target_cpu":"native","build_target_cpu":"znver4","all_other_rustflags_unchanged":true,"rejected_candidates_absent":true}),
    );
    pins.insert(
        r.join("source-audit.json").display().to_string(),
        hash(r.join("source-audit.json")),
    );
    save(r.join("source-pins.json"), &json!(pins));
    let old = read("/tmp/zopro-grouping-prefix-cutoff/native-plan-v2.json");
    let mut steps = vec![
        json!({"kind":"command","name":"compiler","argv":["rustc","-Vv"]}),
        json!({"kind":"command","name":"generator-check","argv":["python3",r.join("current/scripts/ifma8.py"),"--check"],"timeout_seconds":120}),
    ];
    let environment = json!({"CARGO_TARGET_DIR":r.join("target-znver4"),"RUSTFLAGS":flags});
    steps.push(json!({"kind":"command","name":"current-field","argv":["cargo","test","--manifest-path",r.join("current/Cargo.toml"),"--offline","--locked","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":environment,"unfiltered":true,"timeout_seconds":1200}));
    for (name, label) in [
        ("fresh-core", "current-core"),
        ("fresh-prover", "current-prover"),
        ("fresh-libraries", "current-libraries"),
    ] {
        let mut s = old["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == name)
            .unwrap()
            .clone();
        s["name"] = json!(label);
        for a in s["argv"].as_array_mut().unwrap() {
            let t = a.as_str().unwrap();
            *a = json!(t.replace(
                "/tmp/zopro-grouping-prefix-cutoff/fresh",
                &r.join("current").display().to_string()
            ));
        }
        s["environment"] = environment.clone();
        steps.push(s);
    }
    steps.push(json!({"kind":"link-sdk","name":"current","artifacts":r.join("native/current-libraries/stdout.log"),"driver":m["standard_cli_source"],"additional_sources":[m["standard_cli_helper"]],"rustc_flags":["-O","-C","target-cpu=znver4","-C","lto=fat","-C","codegen-units=1"],"timeout_seconds":1200}));
    save(
        r.join("native-plan.json"),
        &json!({"version":1,"source_pins":r.join("source-pins.json"),"environment":old["environment"],"steps":steps}),
    );
    println!("Prepared exact adopted171-source znver4 build; no rejected candidate; field/core/raw/cubic+SDK plan ready; no cloud resources");
}
