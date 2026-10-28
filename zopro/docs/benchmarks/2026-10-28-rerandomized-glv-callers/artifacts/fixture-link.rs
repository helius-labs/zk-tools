use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
fn main() {
    let root = std::env::current_dir().unwrap();
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read("build-pins-v2.json").unwrap()).unwrap();
    let p = root.join("probe.rs").display().to_string();
    let old = pins[&p].clone();
    pins.insert(
        p.clone(),
        format!("{:x}", Sha256::digest(fs::read(&p).unwrap())),
    );
    fs::write(
        "build-pins-v3.json",
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let mut plan: Value = serde_json::from_slice(&fs::read("build-plan-v2.json").unwrap()).unwrap();
    let steps = plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| s["name"] == "build-probe")
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(steps.len(), 1);
    plan["steps"] = json!(steps);
    plan["source_pins"] = json!(root.join("build-pins-v3.json"));
    fs::write(
        "build-plan-v3.json",
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    fs::write("fixture-repair.json",serde_json::to_vec_pretty(&json!({"old_probe_source_sha256":old,"new_probe_source_sha256":pins[&p],"change":"Before any capture, make timed fixtures use inverse=u^-1 for G1 and u for G2, matching windowed_outputs input correlation. Precompute inverse outside timer. Only main fixture/wrapper arguments and independent expected result change; candidate and extracted original function/test bytes unchanged. Link benchmark only; no successful test or library build repeated.","test_executables":"fixture-v2/checked and optimized retain successful test source; capture primes independently verify all corrected fixtures before timing"})).unwrap()).unwrap();
}
