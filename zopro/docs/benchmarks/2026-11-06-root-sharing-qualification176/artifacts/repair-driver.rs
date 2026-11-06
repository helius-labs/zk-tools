use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const B: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("driver-repair-plan.json").exists());
    let original = read(r.join("build-plan.json"));
    let mut pins = read(r.join("build-pins.json"))
        .as_object()
        .unwrap()
        .iter()
        .map(|(p, h)| (p.clone(), h.as_str().unwrap().to_owned()))
        .collect::<BTreeMap<_, _>>();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for name in ["build-oracle", "run-oracle"] {
        assert_eq!(
            read(r.join(format!("build-runs/{name}/exit.json")))["returncode"],
            0
        );
    }
    assert_eq!(read(r.join("oracle.json"))["passed"], true);
    let failed_exit = read(r.join("build-runs/build-driver/exit.json"));
    assert_eq!(failed_exit["returncode"], 1);
    assert!(
        fs::read_to_string(r.join("build-runs/build-driver/stderr.log"))
            .unwrap()
            .contains("Can't find section .llvmbc")
    );
    let mut step = original["steps"][2].clone();
    assert_eq!(step["name"], "build-driver");
    step["name"] = json!("build-driver-native-deps");
    let replacements=[("serde_json=",format!("serde_json={B}/libserde_json-792a1ccfa9d7a509acedb3e992a9c8158d940f10c0b04abe034fe51f74e5811f.rlib")),("sha2=",format!("sha2={B}/libsha2-1c93ccb1b17fec6ca7a76b00c9b7b3dcd3eab11efd897da6dc9ea59c955d7b30.rlib"))];
    let mut changes = vec![];
    for arg in step["argv"].as_array_mut().unwrap() {
        for (prefix, replacement) in &replacements {
            if arg.as_str().unwrap().starts_with(prefix) {
                changes.push(json!({"old":arg.clone(),"new":replacement}));
                *arg = json!(replacement);
                break;
            }
        }
    }
    assert_eq!(changes.len(), 2);
    for f in [
        "repair-driver.rs",
        "repair-driver",
        "build-plan.json",
        "build-runs/completed.json",
        "oracle.json",
        "build-runs/build-driver/exit.json",
        "build-runs/build-driver/stderr.log",
        "build-runs/build-oracle/exit.json",
        "build-runs/run-oracle/exit.json",
        "oracle",
    ] {
        let p = r.join(f);
        pins.insert(p.display().to_string(), hash(&p));
    }
    let repair = json!({"reason":"fat LTO requires bitcode; nonnative helper dependencies omitted it","source_unchanged":true,"accepted_field_unchanged":true,"original_successful_oracle_not_rebuilt_or_rerun":true,"changes":changes,"optimization_flags_unchanged":true});
    fs::write(
        r.join("driver-repair.json"),
        serde_json::to_vec_pretty(&repair).unwrap(),
    )
    .unwrap();
    fs::write(
        r.join("driver-repair-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(r.join("driver-repair-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("driver-repair-pins.json"),"environment":{},"steps":[step]})).unwrap()).unwrap();
    println!("Frozen failed driver link repair only:2helper-library references; sources/nativefield/optimizationflags exact; successful2oracle stages reused");
    let _ = PathBuf::new();
}
