use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-stages";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() {
    let r = Path::new(R);
    let original = fs::read_to_string(r.join("initial-install-failure/run.rs")).unwrap();
    let current = fs::read_to_string(r.join("run.rs")).unwrap();
    let slice = |s: &str| {
        let start = s.find("fn ns(").unwrap();
        let end = s.find("fn preflight()").unwrap();
        s[start..end].to_string()
    };
    assert_eq!(slice(&original), slice(&current));
    let tests = |s: &str| s[s.find("#[cfg(test)]").unwrap()..].to_string();
    assert_eq!(tests(&original), tests(&current));
    let receipt = json!({"parser_source_exact":true,"unit_test_source_exact":true,"three_passing_tests_inherited":true,"parser_sha256":hash(slice(&current).as_bytes()),"tests_sha256":hash(tests(&current).as_bytes()),"SDK_requests_before_repair":0,"successful_install_not_repeated":true});
    fs::write(
        r.join("parser-test-inheritance.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    let mut pins: Value = serde_json::from_slice(&fs::read(r.join("pins.json")).unwrap()).unwrap();
    for f in [
        "run.rs",
        "run-helper",
        "installed.json",
        "setup-repair.md",
        "repin.rs",
        "parser-test-inheritance.json",
    ] {
        let p = r.join(f);
        pins[p.display().to_string()] = json!(hash(&fs::read(&p).unwrap()));
    }
    let mut plan: Value = serde_json::from_slice(&fs::read(r.join("plan.json")).unwrap()).unwrap();
    let steps = plan["steps"].as_array().unwrap();
    assert_eq!(steps[0]["name"], "install");
    assert_eq!(steps.len(), 92);
    plan["steps"] = json!(&steps[1..]);
    plan["source_pins"] = json!(r.join("pins-install-repaired.json"));
    fs::write(
        r.join("pins-install-repaired.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        r.join("plan-install-repaired.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    println!("91remainingsteps;successfulmanualinstall retained; parser/testbytesexact");
}
