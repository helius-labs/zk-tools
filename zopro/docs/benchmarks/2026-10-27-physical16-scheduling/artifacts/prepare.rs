use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const R: &str = "/tmp/zopro-physical16-scheduling";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    let old = read("/tmp/zopro-composed-cpu/perf-plan.json");
    let mut pins: BTreeMap<String, String> = serde_json::from_value(old["pins"].clone()).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h);
    }
    let mut seen: BTreeSet<String> =
        serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 8604);
    let mut local = BTreeSet::new();
    let mut duplicates = 0;
    for (p, h) in read("/tmp/zopro-composed-cpu/proof-inventory.json")
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
            assert!(local.insert(h.as_str().unwrap().to_owned()));
            if !seen.insert(h.as_str().unwrap().to_owned()) {
                duplicates += 1;
            }
        }
    }
    assert_eq!(local.len(), 465);
    assert_eq!(duplicates, 9);
    assert_eq!(seen.len(), 9060);
    let case = old["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "1x2-w16")
        .unwrap()
        .clone();
    let inputs = json!({"variants":old["variants"],"environment":old["environment"],"case":case,"prior_proof_sha256":seen,"prior_count":9060,"expected_final_count":9069});
    save(r.join("inputs.json"), &inputs);
    for n in [
        "capture.rs",
        "capture",
        "prepare.rs",
        "prepare",
        "analyze.rs",
        "analyze",
        "design.md",
        "inputs.json",
        "analyzer-tests.log",
    ] {
        let p = r.join(n);
        pins.insert(p.display().to_string(), hash(&p));
    }
    for p in [
        "/usr/bin/perf",
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
    ] {
        pins.insert(p.into(), hash(p));
    }
    save(r.join("pins.json"), &json!(pins));
    fs::create_dir(r.join("captures")).unwrap();
    let mut steps = Vec::new();
    let mut runs = Vec::new();
    for b in 0..3 {
        let mut names = vec!["accepted", "fresh", "composed"];
        names.rotate_left(b);
        if b % 2 == 1 {
            names.reverse();
        }
        for name in names {
            let label = format!("b{b}-{name}");
            let d = r.join("captures").join(&label);
            steps.push(json!({"kind":"command","name":format!("record-{label}"),"argv":[r.join("capture"),name,d],"environment":old["environment"],"timeout_seconds":600}));
            steps.push(json!({"kind":"command","name":format!("verify-{label}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",d.join("proofs"),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
            runs.push(json!({"name":label,"block":b,"variant":name,"directory":d}));
        }
    }
    save(r.join("runs.json"), &json!(runs));
    pins.insert(
        r.join("runs.json").display().to_string(),
        hash(r.join("runs.json")),
    );
    save(r.join("pins.json"), &json!(pins));
    save(
        r.join("workflow-plan.json"),
        &json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps}),
    );
    println!("Prepared nine exact-artifact physical16 diagnostics; 4536 SDK calls / 9 exports planned; prior9060");
}
