use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    assert_eq!(read(root.join("build/result.json"))["status"], "complete");
    let original = fs::read_to_string(root.join("probe.rs")).unwrap();
    let at = original.find("#[cfg(test)]\nmod tests").unwrap();
    assert_eq!(original.matches("#[cfg(test)]\nmod tests").count(), 1);
    for (src, dst) in [
        ("probe.rs", "probe-initial.rs"),
        ("accepted-probe", "accepted-probe-initial"),
        ("target/release/probe", "fresh-probe-initial"),
    ] {
        assert!(!root.join(dst).exists());
        fs::copy(root.join(src), root.join(dst)).unwrap();
    }
    fs::write(
        root.join("probe.rs"),
        original[..at].to_owned() + "include!(\"probe-tests.rs\");\n",
    )
    .unwrap();
    let oldpins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("build-input-pins.json"))).unwrap();
    for (p, h) in &oldpins {
        let actual = if p == root.join("probe.rs").to_str().unwrap() {
            root.join("probe-initial.rs")
        } else {
            p.into()
        };
        assert_eq!(&hash(actual), h, "original pin{p}");
    }
    let mut pins = oldpins;
    let p = root.join("probe.rs");
    pins.insert(p.to_str().unwrap().into(), hash(&p));
    for name in [
        "probe-initial.rs",
        "accepted-probe-initial",
        "fresh-probe-initial",
        "repair-harness.rs",
        "build-plan.json",
        "build-input-pins.json",
        "build/result.json",
        "build/field-tests/stdout.log",
        "build/field-tests/exit.json",
        "build/component-oracles/stdout.log",
        "build/component-oracles/exit.json",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let mut plan = read(root.join("build-plan.json"));
    plan["steps"] = json!(plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| matches!(
            s["name"].as_str().unwrap(),
            "component-oracles" | "fresh-bench-build" | "accepted-bench-link"
        ))
        .cloned()
        .collect::<Vec<_>>());
    plan["source_pins"] = json!(root.join("build-input-pins-v2.json"));
    fs::write(root.join("harness-repair.json"),serde_json::to_vec_pretty(&json!({"issue":"Copied probe contained inline previous Comba coefficient tests; newly prepared row-model test file was initially unused.","initial_tests_scope":"New streamed method passed previous algebra and exact REDC tests, but not the new row-by-row coefficient/sum model or direct oldshared rawdigits comparison.","repair":"Replace only cfg(test) inline section with relative include of newly pinned probe-tests.rs.","runtime_source_prefix_identical":true,"field_source_unchanged":true,"timing_processes_before_fix":0,"reused":"33 unchanged field tests and generator-check exact source; retained receipts.","repeated":"All four new independent oracle tests plus both exact probe builds; no successful measurement repeated.","original185pins_verified_with_single_exact_original_probe_relocation":true})).unwrap()).unwrap();
    let p = root.join("harness-repair.json");
    pins.insert(p.to_str().unwrap().into(), hash(&p));
    fs::write(
        root.join("build-input-pins-v2.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("build-plan-v2.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    println!(
        "Harness test-only repair; {}pins; no timing before fix",
        pins.len()
    );
}
