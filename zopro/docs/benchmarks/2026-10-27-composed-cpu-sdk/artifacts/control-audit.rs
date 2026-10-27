use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-composed-cpu");
    assert_eq!(
        read(r.join("performance/result.json"))["status"],
        "complete"
    );
    let accepted = read("/tmp/zopro-grouping-module/native-plan.json");
    let fresh = read("/tmp/zopro-poseidon-fused-dot/native-plan.json");
    let composed = read(r.join("native-plan.json"));
    let flags = |p: &Value, name: &str| -> String {
        let step = p["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == name)
            .unwrap();
        step["environment"]["RUSTFLAGS"]
            .as_str()
            .or_else(|| p["environment"]["RUSTFLAGS"].as_str())
            .unwrap()
            .to_string()
    };
    let a = flags(&accepted, "libraries");
    let f = flags(&fresh, "fresh-libraries");
    let c = flags(&composed, "on-libraries");
    assert_eq!(a, f);
    assert_eq!(a, c);
    let rows = read(r.join("performance/rows.json"));
    let rows:Vec<_>=rows.as_array().unwrap().iter().filter(|x|x["phase"]=="latency"&&x["case"]=="1x2-w16").map(|x|{
        let rounds=x["summary"]["rounds"].as_f64().unwrap();json!({"block":x["block"],"variant":x["variant"],"median_ms":x["median_ms"],"cpu_ms_per_request":(x["summary"]["user_cpu_ms"].as_f64().unwrap()+x["summary"]["system_cpu_ms"].as_f64().unwrap())/rounds,"involuntary_switches_per_request":x["summary"]["involuntary_switches"].as_f64().unwrap()/rounds,"voluntary_switches_per_request":x["summary"]["voluntary_switches"].as_f64().unwrap()/rounds})
    }).collect();
    assert_eq!(rows.len(), 45);
    let mut artifacts = Vec::new();
    for (name, p) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
        ),
        (
            "fresh",
            "/tmp/zopro-poseidon-fused-dot/native/fresh/artifacts.json",
        ),
        (
            "composed",
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
        ),
    ] {
        let v = read(p);
        let binary = v["binary"].as_str().unwrap();
        assert_eq!(hash(binary), v["binary_sha256"]);
        artifacts
            .push(json!({"variant":name,"receipt":p,"binary":binary,"sha256":v["binary_sha256"]}));
    }
    let out = json!({"after_all_captures":true,"native_RUSTFLAGS_equal":true,"RUSTFLAGS":a,"control_recipes":["/tmp/zopro-grouping-module/native-plan.json","/tmp/zopro-poseidon-fused-dot/native-plan.json"],"common_runtime_environment":read(r.join("performance/protocol.json"))["environment"],"artifacts":artifacts,"physical16_rows":rows,"calibration_failure_retained":read(r.join("analysis.json"))["calibration_failures"],"physical16_CPU_profiles_in_this_study":0,"source_of_variation_established":false,"counters_do_not_identify_external_preemption_or_self_yield":true,"no_new_timings_or_repeated_capture":true});
    fs::write(
        r.join("physical16-control-audit.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Postcapturecontrolflagsmatch;45physical16observationsretained;causeunresolved;no timingrepeat.");
}
