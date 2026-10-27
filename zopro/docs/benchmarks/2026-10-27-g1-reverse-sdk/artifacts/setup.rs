use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-g1-reverse-sdk";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    let m = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let component = Path::new("/tmp/zopro-g1-reverse-overlap");
    assert_eq!(
        read(component.join("analysis.json"))["component_advances"],
        true
    );
    assert_eq!(
        read(component.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        111
    );
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let prod = Path::new("/home/ubuntu/src/zk-tools/zopro").join(p);
        assert_eq!(hash(&prod), h.as_str().unwrap());
        let dest = r.join("candidate").join(p);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(prod, dest).unwrap();
    }
    let edits = read(component.join("edits.json"));
    let old = edits["old_stream"].as_str().unwrap();
    let new = edits["new_stream"].as_str().unwrap();
    let p = r.join("candidate/crates/zopro-core/src/msm.rs");
    let original = fs::read_to_string(&p).unwrap();
    assert_eq!(original.matches(old).count(), 1);
    let changed = original.replacen(old, new, 1);
    assert_eq!(changed.replace(new, old), original);
    fs::write(p, changed).unwrap();
    let mut changed = Vec::new();
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        if hash(r.join("candidate").join(p)) != h.as_str().unwrap() {
            changed.push(p);
        }
    }
    assert_eq!(changed, vec!["crates/zopro-core/src/msm.rs"]);
    for rel in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        assert_eq!(
            hash(r.join("candidate").join(rel)),
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel))
        );
    }
    save(
        r.join("edits.json"),
        &json!({"old_stream":old,"new_stream":new}),
    );
    save(
        r.join("source-audit.json"),
        &json!({"originals":170,"unchanged_candidate_originals":169,"accepted_bytes_recovered":true,"exact_component_stream_body":true,"diagnostic_original_and_probe_removed":true,"new_modules":false,"field_sources_unchanged":true,"component_native_tests_reused":111,"production_changed":false}),
    );
    let mut plan: Value = serde_json::from_str(
        &serde_json::to_string(&read("/tmp/zopro-tree-kernel-split/native-plan.json"))
            .unwrap()
            .replace("/tmp/zopro-tree-kernel-split", R),
    )
    .unwrap();
    let steps = plan["steps"].as_array_mut().unwrap();
    steps.retain(|s| ["compiler", "on-libraries", "split"].contains(&s["name"].as_str().unwrap()));
    for step in steps {
        if step["name"] == "split" {
            step["name"] = json!("overlap");
        }
    }
    save(r.join("native-plan.json"), &plan);
    let mut pins = BTreeMap::new();
    for (p, _) in m["source_files_sha256"].as_object().unwrap() {
        let f = r.join("candidate").join(p);
        pins.insert(f.display().to_string(), hash(f));
    }
    for name in [
        "setup.rs",
        "design.md",
        "edits.json",
        "source-audit.json",
        "native-plan.json",
        "candidate/crates/zopro-bench/examples/cpu-sdk.rs",
        "candidate/crates/zopro-bench/examples/support/perf.rs",
    ] {
        let f = r.join(name);
        pins.insert(f.display().to_string(), hash(f));
    }
    for name in [
        "analysis.json",
        "native/core/tests.json",
        "source-audit.json",
        "edits.json",
        "closeout.json",
    ] {
        let f = component.join(name);
        pins.insert(f.display().to_string(), hash(f));
    }
    pins.insert(
        "/tmp/zopro-xtask-fast-screen-target/release/xtask".to_string(),
        hash("/tmp/zopro-xtask-fast-screen-target/release/xtask"),
    );
    save(r.join("source-pins.json"), &json!(pins));
    println!("Frozen standardSDKonefilechange with exacttestedstreambody;169originalsunchanged;no diagnosticcfg.");
}
