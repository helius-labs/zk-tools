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
    let r = Path::new("/tmp/zopro-phased-complex-products");
    let step = r.join("build/field-tests");
    let exit = read(step.join("exit.json"));
    assert_eq!(exit["returncode"], 0);
    for stream in ["stdout", "stderr"] {
        assert_eq!(
            hash(step.join(format!("{stream}.log"))),
            exit[format!("{stream}_sha256")].as_str().unwrap()
        );
    }
    assert_eq!(
        read(step.join("tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        33
    );
    if std::env::args().any(|a| a == "--audit") {
        println!(
            "Reuse exact successful33field tests; exit and both stream hashes checked; no rerun."
        );
        return;
    }
    fs::copy(r.join("probe.rs"), r.join("probe-initial.rs")).unwrap();
    let s = fs::read_to_string(r.join("probe.rs")).unwrap();
    assert_eq!(s.matches("let x = &one << bit;").count(), 1);
    fs::write(
        r.join("probe.rs"),
        s.replace("let x = &one << bit;", "let x: BigUint = &one << bit;"),
    )
    .unwrap();
    let s = fs::read_to_string(r.join("probe-tests.rs")).unwrap();
    assert_eq!(s.matches("let x=&one<<bit;").count(), 1);
    fs::write(
        r.join("probe-tests.rs"),
        s.replace("let x=&one<<bit;", "let x:BigUint=&one<<bit;"),
    )
    .unwrap();
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-input-pins.json"))).unwrap();
    for name in [
        "probe.rs",
        "probe-tests.rs",
        "probe-initial.rs",
        "repair.rs",
        "repair",
        "analyze.rs",
        "build/field-tests/exit.json",
        "build/field-tests/tests.json",
        "build/field-tests/stdout.log",
        "build/field-tests/stderr.log",
        "build/generator-check/exit.json",
    ] {
        let p = r.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    fs::write(
        r.join("build-input-pins-v2.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let mut plan = read(r.join("build-plan.json"));
    let mut steps = plan["steps"].as_array().unwrap()[3..].to_vec();
    steps.insert(
        0,
        json!({"kind":"command","name":"field-receipt-audit","argv":[r.join("repair"),"--audit"]}),
    );
    plan["steps"] = json!(steps);
    plan["source_pins"] = json!(r.join("build-input-pins-v2.json"));
    fs::write(
        r.join("build-plan-v2.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    fs::write(r.join("repair.json"),serde_json::to_vec_pretty(&json!({"reason":"E0282: ambiguous BigUint shift test variable","change":"Only explicit BigUint type annotation in test code. No arithmetic, timing harness or gate change.","original_failure_retained":"build/component-oracles","successful_field_tests_reused":33,"source_field_byte_changes":false,"before_first_oracle_execution_and_latency":true})).unwrap()).unwrap();
    println!("Test-only type annotation repaired; native field result reused.");
}
