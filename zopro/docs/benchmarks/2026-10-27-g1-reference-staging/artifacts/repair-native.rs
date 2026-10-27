use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/tmp/zopro-g1-reference-staging";
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
    let mut plan = read(r.join("native-plan.json"));
    assert_eq!(
        read(r.join("native/failure.json"))["completed"],
        json!(["fresh-core", "fresh-prover", "fresh-libraries"])
    );
    assert_eq!(
        read(r.join("native/fresh-core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        116
    );
    assert_eq!(
        read(r.join("native/fresh-prover/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("source-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h)
    }
    let manifest = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    let driver = manifest["standard_cli_source"].as_str().unwrap();
    let helper = manifest["standard_cli_helper"].as_str().unwrap();
    assert_eq!(
        hash(driver),
        manifest["standard_cli_source_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(helper),
        manifest["standard_cli_helper_sha256"].as_str().unwrap()
    );
    let mut steps = Vec::new();
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        if ["fresh-core", "fresh-prover", "fresh-libraries"].contains(&name) {
            continue;
        }
        let mut s = step.clone();
        if s["kind"] == "link-sdk" {
            s["driver"] = json!(driver);
            s["additional_sources"] = json!([helper]);
            if name == "candidate" {
                s["artifacts"] = json!(r.join("native-v2/candidate-libraries/stdout.log"));
            }
        }
        steps.push(s);
    }
    for p in [
        Path::new(driver).to_owned(),
        Path::new(helper).to_owned(),
        r.join("repair-native.rs"),
        r.join("repair-native"),
        r.join("native/fresh-core/tests.json"),
        r.join("native/fresh-prover/tests.json"),
        r.join("native/fresh-libraries/stdout.log"),
        r.join("native-plan.json"),
        r.join("native/failure.json"),
    ] {
        pins.insert(p.display().to_string(), hash(&p));
    }
    save(r.join("source-pins-v2.json"), &json!(pins));
    plan["source_pins"] = json!(r.join("source-pins-v2.json"));
    plan["steps"] = json!(steps);
    save(r.join("native-plan-v2.json"), &plan);
    save(
        r.join("native-repair.json"),
        &json!({"failure":"SDK driver/support are separate manifest fields, not part of171library/source files copied by xtask prepare","failed_stage":"fresh SDK link before invoking rustc","fresh_core116_and_prover8_results_reused_without_rerun":true,"fresh_libraries_reused_unchanged":true,"new_workflow_for_changed_plan":true,"no_source_or_compiler_flag_change":true,"no_request_captures_started":true,"old_plan_and_failure_retained":true,"driver":driver,"helper":helper}),
    );
    println!("Linked consumers now use exact accepted standalone driver/support; fresh116+8 and libraries retained; remaining work in native-v2");
}
