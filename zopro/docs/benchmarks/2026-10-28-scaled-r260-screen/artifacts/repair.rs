use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    assert!(!Path::new("repair.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read("build-pins.json").unwrap()).unwrap();
    let path = std::env::current_dir()
        .unwrap()
        .join("model.rs")
        .display()
        .to_string();
    assert_eq!(pins[&path], hash("initial-type-failure/model.rs"));
    let old = pins[&path].clone();
    for (p, h) in &pins {
        if p != &path {
            assert_eq!(hash(p), *h);
        }
    }
    pins.insert(path.clone(), hash(&path));
    fs::write("build-pins.json", serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    let plan: Value = serde_json::from_slice(&fs::read("build-plan.json").unwrap()).unwrap();
    fs::write(
        "build-plan-v2.json",
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    fs::rename("native", "native-initial-type-failure").unwrap();
    fs::write("repair.json",serde_json::to_vec_pretty(&json!({"failure":"native-initial-type-failure/build-model-tests/stderr.log","change":"Annotate BigUint result and usize shift width in power-boundary fixture; no arithmetic algorithm change","initial_model_sha256":old,"repaired_model_sha256":pins[&path],"initial_sources_and_pins":"initial-type-failure","tests_or_study_executed_before_repair":0,"successful_commands_repeated":false,"initial_native_directory_relocated":true})).unwrap()).unwrap();
}
