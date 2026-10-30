use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/query-timeline175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn main() {
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let tests = read(r("native/control-core/tests.json"));
    assert_eq!(tests["unfiltered"], true);
    let passed = tests["passed_tests"].as_array().unwrap();
    assert_eq!(passed.len(), 126);
    for name in [
        "query_timeline::tests::stage_order_and_query_containment",
        "query_timeline::tests::bounded_journal_parallel_spans_and_disabled_reset",
    ] {
        assert!(passed.iter().any(|v| v == name));
    }
    let prover = read(r("native/control-prover/tests.json"));
    assert_eq!(prover["unfiltered"], true);
    assert_eq!(prover["passed_tests"].as_array().unwrap().len(), 8);
    let frozen = read(r("frozen-control.json"));
    for (p, h) in frozen["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    assert_eq!(frozen["libraries"].as_object().unwrap().len(), 8);
    let original = Path::new(Z).join("crates/zopro-bench/examples/cpu-sdk.rs");
    let original_perf = Path::new(Z).join("crates/zopro-bench/examples/support/perf.rs");
    let mut driver = fs::read_to_string(&original).unwrap();
    let mut edits = vec![];
    for(before,after)in[("    for _ in 0..3 {","    let timeline_enabled=match std::env::var(\"ZOPRO_QUERY_TIMELINE\").as_deref(){Ok(\"1\")=>true,Ok(\"0\")=>false,_=>return Err(\"expected ZOPRO_QUERY_TIMELINE=0 or1\".into())};\n    zopro_prover::zopro_core::query_timeline::prepare();\n    for _ in 0..3 {"),("    let before = perf::Usage::now()?;","    zopro_prover::zopro_core::query_timeline::begin(timeline_enabled);\n    let before = perf::Usage::now()?;"),("    if let Some(directory) = args.get(4) {","    let timeline=zopro_prover::zopro_core::query_timeline::take();\n    if timeline_enabled {assert_eq!(timeline.len(),rounds);} else {assert!(timeline.is_empty());}\n    for(round,offsets_ns)in timeline.into_iter().enumerate(){println!(\"TIMELINE {}\",serde_json::json!({\"round\":round,\"offsets_ns\":offsets_ns}));}\n    if let Some(directory) = args.get(4) {")]{assert_eq!(driver.matches(before).count(),1);driver=driver.replace(before,after);edits.push(json!({"before":before,"after":after}));}
    let mut reversed = driver.clone();
    for e in edits.iter().rev() {
        let after = e["after"].as_str().unwrap();
        assert_eq!(reversed.matches(after).count(), 1);
        reversed = reversed.replace(after, e["before"].as_str().unwrap());
    }
    assert_eq!(reversed, fs::read_to_string(&original).unwrap());
    fs::create_dir_all(r("sdk/support")).unwrap();
    fs::write(r("sdk/cpu-sdk.rs"), driver).unwrap();
    fs::copy(&original_perf, r("sdk/support/perf.rs")).unwrap();
    save(
        r("sdk/source-audit.json"),
        &json!({"edits":edits,"original_driver_reversed_exact":true,"proof_timing_loop_unchanged":true,"performance_helper_exact":true,"same_OFF_and_traced_executable":true}),
    );
    let mut pins = BTreeMap::new();
    for p in [
        original,
        original_perf,
        r("sdk/cpu-sdk.rs"),
        r("sdk/support/perf.rs"),
        r("sdk/source-audit.json"),
        r("frozen-control-cargo.jsonl"),
        r("frozen-control.json"),
        r("prepare-sdk.rs"),
        r("prepare-sdk"),
    ] {
        pins.insert(p.display().to_string(), hash(&p));
    }
    for (p, h) in frozen["input_pins"].as_object().unwrap() {
        pins.insert(p.clone(), h.as_str().unwrap().to_string());
    }
    save(r("sdk/pins.json"), &json!(pins));
    save(
        r("sdk/plan.json"),
        &json!({"version":1,"source_pins":r("sdk/pins.json"),"steps":[{"kind":"link-sdk","name":"timeline","driver":r("sdk/cpu-sdk.rs"),"additional_sources":[r("sdk/support/perf.rs")],"artifacts":r("frozen-control-cargo.jsonl"),"timeout_seconds":600}]}),
    );
    println!("Prepared identical OFF/traced SDK link with postclock bounded journal export; original proof timing loop reversed exact");
}
