use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/tmp/zopro-control-rescheduling";
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
    let mut pins = read(r.join("capture-pins.json"));
    for f in [
        "analyze-pretty.rs",
        "runner-pretty.rs",
        "prepare-reader.rs",
        "retained-trace-reader-repair.md",
        "preflight-inventory.json",
        "proof-inventory.json",
    ] {
        let p = r.join(f);
        pins[p.display().to_string()] = json!(hash(&p));
    }
    let stage = std::env::args().nth(1).unwrap();
    if stage == "build" {
        let old = read(r.join("workflow-build.json"));
        let mut steps = Vec::new();
        for s in old["steps"].as_array().unwrap() {
            if !s["name"].as_str().unwrap().starts_with("analyzer") && s["name"] != "runner-build" {
                continue;
            }
            let mut s = s.clone();
            s["name"] = json!(format!("pretty-{}", s["name"].as_str().unwrap()));
            for a in s["argv"].as_array_mut().unwrap() {
                if let Some(v) = a.as_str() {
                    for (from, to) in [
                        ("analyze.rs", "analyze-pretty.rs"),
                        ("analyzer-tests", "analyzer-pretty-tests"),
                        ("analyze", "analyze-pretty"),
                        ("runner.rs", "runner-pretty.rs"),
                        ("runner", "runner-pretty"),
                    ] {
                        if v == format!("{R}/{from}") {
                            *a = json!(format!("{R}/{to}"));
                            break;
                        }
                    }
                }
            }
            if s["name"] == "pretty-analyzer-test-run" {
                s["required_tests"].as_array_mut().unwrap().push(json!(
                    "tests::perf_pretty_switch_has_numeric_ids_and_preemption"
                ));
            }
            steps.push(s);
        }
        save(r.join("reader-build-pins.json"), &pins);
        save(
            r.join("workflow-reader-build.json"),
            &json!({"version":1,"source_pins":r.join("reader-build-pins.json"),"steps":steps}),
        );
    } else {
        assert_eq!(stage, "run");
        for f in ["analyze-pretty", "runner-pretty"] {
            let p = r.join(f);
            pins[p.display().to_string()] = json!(hash(&p));
        }
        save(r.join("reader-run-pins.json"), &pins);
        save(
            r.join("workflow-reader-run.json"),
            &json!({"version":1,"source_pins":r.join("reader-run-pins.json"),"steps":[{"kind":"command","name":"retained-decode","argv":[r.join("runner-pretty"),"decode"],"timeout_seconds":1800}]}),
        );
    }
}
