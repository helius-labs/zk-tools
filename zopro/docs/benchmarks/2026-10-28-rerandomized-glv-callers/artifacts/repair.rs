use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
fn main() {
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read("build-pins.json").unwrap()).unwrap();
    let p = format!(
        "{}/candidate.rs",
        std::env::current_dir().unwrap().display()
    );
    let old = pins[&p].clone();
    pins.insert(
        p.clone(),
        format!("{:x}", Sha256::digest(fs::read(&p).unwrap())),
    );
    fs::write(
        "build-pins-v2.json",
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let mut plan: Value = serde_json::from_slice(&fs::read("build-plan.json").unwrap()).unwrap();
    plan["source_pins"] = json!(format!(
        "{}/build-pins-v2.json",
        std::env::current_dir().unwrap().display()
    ));
    fs::write(
        "build-plan-v2.json",
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    fs::write("repair.json",serde_json::to_vec_pretty(&json!({"initial_candidate":old,"repaired_candidate":pins[&p],"change":"Explicit usize index and u64 zero in signed-component closure; initial checked build failed before any test or timing; no successful command repeated","initial_workflow":"native/failure.json"})).unwrap()).unwrap();
}
