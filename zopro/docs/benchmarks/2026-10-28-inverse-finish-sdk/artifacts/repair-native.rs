use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/home/ubuntu/src/zopro-perf-work/inverse-finish-sdk");
    let parent = Path::new("/home/ubuntu/src/zopro-perf-work/inverse-finish-callers");
    let mut plan: Value =
        serde_json::from_slice(&fs::read("build-plan-first.json").unwrap()).unwrap();
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read("build-pins-first.json").unwrap()).unwrap();
    for (label, variant) in [("off", "fresh"), ("on", "selected")] {
        let receipt: Value = serde_json::from_slice(
            &fs::read(parent.join(format!("native/{variant}-artifacts.json"))).unwrap(),
        )
        .unwrap();
        let mut map = receipt["libraries"].as_object().unwrap().clone();
        let memmap = receipt["all_libraries"]["memmap2"].as_array().unwrap();
        assert_eq!(memmap.len(), 1);
        map.insert("memmap2".into(), memmap[0].clone());
        let mut text = String::new();
        for (name, path) in map {
            assert_eq!(
                hash(path.as_str().unwrap()),
                receipt["input_pins"][path.as_str().unwrap()]
                    .as_str()
                    .unwrap()
            );
            text.push_str(&serde_json::to_string(&json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[path]})).unwrap());
            text.push('\n');
        }
        let dest = root.join(format!("frozen-{label}-cargo-v2.jsonl"));
        fs::write(&dest, text).unwrap();
        pins.insert(dest.display().to_string(), hash(dest));
    }
    let steps = plan["steps"].as_array().unwrap()[2..].to_vec();
    let steps: Vec<_> = steps
        .into_iter()
        .map(|mut s| {
            if s["kind"] == "link-sdk" {
                let label = s["name"].as_str().unwrap().to_string();
                s["artifacts"] = json!(root.join(format!("frozen-{label}-cargo-v2.jsonl")));
            }
            s
        })
        .collect();
    assert_eq!(steps.len(), 4);
    plan["steps"] = json!(steps);
    for name in [
        "experiment.rs",
        "experiment",
        "repair-native.rs",
        "repair-native",
    ] {
        let p = root.join(name);
        pins.insert(p.display().to_string(), hash(p));
    }
    plan["source_pins"] = json!(root.join("build-pins-final.json"));
    fs::write(
        "build-pins-final.json",
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        "build-plan-final.json",
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    println!("PreparedonlyunrunOFFlinkandONsuites/link;OFFpassedtestsreused");
}
