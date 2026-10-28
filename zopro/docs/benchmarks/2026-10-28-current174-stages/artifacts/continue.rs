use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-stages";
fn main() {
    let r = Path::new(R);
    let mut pins: Value =
        serde_json::from_slice(&fs::read(r.join("pins-install-repaired.json")).unwrap()).unwrap();
    for f in [
        "run.rs",
        "run-helper",
        "parser-tests.log",
        "continue.rs",
        "preflight-parser-failure/workflow/preflight/stdout.log",
        "preflight-parser-failure/workflow/verify-preflight/stdout.log",
        "preflight-parser-failure/workflow/decode-preflight/stdout.log",
        "preflight-parser-failure/workflow/evlist-preflight/stdout.log",
        "preflight-parser-failure/workflow/loss-preflight/stdout.log",
        "proofs/preflight/0.proof",
        "proofs/preflight/0.public",
    ] {
        let p = r.join(f);
        pins[p.display().to_string()] =
            json!(format!("{:x}", Sha256::digest(fs::read(&p).unwrap())));
    }
    let mut plan: Value =
        serde_json::from_slice(&fs::read(r.join("plan-selector-repaired.json")).unwrap()).unwrap();
    let steps = plan["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 91);
    assert_eq!(steps[5]["name"], "check-preflight");
    plan["steps"] = json!(&steps[5..]);
    plan["source_pins"] = json!(r.join("pins-parse-repaired.json"));
    fs::write(
        r.join("pins-parse-repaired.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        r.join("plan-parse-repaired.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    println!("Continue86steps;one successful verified preflight+3decodes retained, no recapture");
}
