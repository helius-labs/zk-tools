use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32");
    let mut plan: Value =
        serde_json::from_slice(&fs::read(root.join("build-plan-initial.json")).unwrap()).unwrap();
    let remaining = plan["steps"].as_array().unwrap()[3..].to_vec();
    plan["steps"] = json!(remaining);
    plan["source_pins"] = json!(root.join("build-input-pins-v2.json"));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(root.join("build-input-pins.json")).unwrap()).unwrap();
    pins.insert(
        root.join("probe.rs").to_str().unwrap().into(),
        hash(root.join("probe.rs")),
    );
    for name in [
        "probe-initial.rs",
        "repair.rs",
        "build-plan-initial.json",
        "build/field-tests/stdout.log",
        "build/field-tests/tests.json",
        "build/field-tests/exit.json",
        "build/generator-check/exit.json",
        "build/component-oracles/stderr.log",
        "build/component-oracles/exit.json",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
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
    fs::write(root.join("repair.json"),serde_json::to_vec_pretty(&json!({"initial_failure":"Five oracle compile diagnostics:BigUint shift result type, reference comparison in two macro expansions, untyped reverse_bits index in two macro expansions. No oracle ran.","repair":"Explicit BigUint/usize types and borrowed bound in test code only; remove unused test import.","field_source_changed":false,"field_tests_repeated":false,"completed_inherited":"33 unfiltered native field tests and generator check pass; exact receipts pinned.","new_workflow":"Run only remaining six oracle tests and two benchmark builds. No timing gate or algorithm change."})).unwrap()).unwrap();
    println!(
        "Reuse33successful field tests and generator; remaining workflow frozen{}pins",
        pins.len()
    );
}
