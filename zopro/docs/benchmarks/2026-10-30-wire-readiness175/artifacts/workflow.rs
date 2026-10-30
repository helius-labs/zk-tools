use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/wire-readiness175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const P: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n])
    }
    format!("{:x}", h.finalize())
}
fn pin(m: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    m.insert(p.as_ref().display().to_string(), hash(&p));
}
fn setup() {
    assert!(!r("plan.json").exists());
    let mp = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&mp);
    let frozen = read(Path::new(P).join("frozen-candidate.json"));
    assert_eq!(m["rustflags"], frozen["rustflags"]);
    assert_eq!(frozen["libraries"].as_object().unwrap().len(), 8);
    let mut pins = BTreeMap::new();
    for (p, h) in frozen["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().to_string());
    }
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    pin(&mut pins, &mp);
    let mut steps = vec![];
    let artifacts = Path::new(P).join("frozen-candidate-cargo.jsonl");
    steps.push(json!({"kind":"link-sdk","name":"parser-tests","artifacts":artifacts,"driver":r("census.rs"),"additional_sources":[r("model.rs")],"rustc_flags":["--test","-O","-C","target-cpu=native"],"timeout_seconds":600}));
    steps.push(json!({"kind":"command","name":"parser-tests-run","argv":[r("runs/parser-tests-sdk"),"--test-threads=1"],"required_tests":["model::tests::stored_slot_and_hint_readiness","model::tests::reject_undefined_read_and_duplicate_write","model::tests::reject_truncated_unsorted_and_trailing"],"unfiltered":true,"timeout_seconds":120}));
    steps.push(json!({"kind":"link-sdk","name":"census","artifacts":artifacts,"driver":r("census.rs"),"additional_sources":[r("model.rs")],"timeout_seconds":600}));
    let old = read("/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175/cases.json");
    let env = read("/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175/plan.json")["steps"][0]
        ["environment"]
        .clone();
    let mut cases = vec![];
    for shape in ["1x2", "2x2"] {
        let c = old
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["shape"] == shape && c["workers"] == 32)
            .unwrap()
            .clone();
        pin(&mut pins, c["image"].as_str().unwrap());
        assert_eq!(hash(c["image"].as_str().unwrap()), c["image_sha256"]);
        let file = r(format!("{shape}.solver"));
        let output = r(format!("{shape}.json"));
        steps.push(json!({"kind":"command","name":format!("export-{shape}"),"argv":[r("runs/census-sdk"),"export",c["image"],file],"environment":env,"timeout_seconds":600}));
        steps.push(json!({"kind":"command","name":format!("analyze-{shape}"),"argv":[r("runs/census-sdk"),"analyze",file,output],"timeout_seconds":600}));
        cases.push(c);
    }
    assert_eq!(steps.len(), 7);
    save(r("cases.json"), &json!(cases));
    for p in [
        "census.rs",
        "model.rs",
        "workflow.rs",
        "workflow-helper",
        "design.md",
        "cases.json",
    ] {
        pin(&mut pins, r(p));
    }
    for p in [
        artifacts,
        Path::new(P).join("frozen-candidate.json"),
        Path::new("/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask").to_path_buf(),
    ] {
        pin(&mut pins, p);
    }
    let summary = Path::new(Z).join("docs/benchmarks/2026-10-29-current-cpu-summary");
    fs::create_dir_all(r("inherited-summary")).unwrap();
    for f in ["README.md", "data.json", "source-hashes.json"] {
        fs::copy(summary.join(f), r("inherited-summary").join(f)).unwrap();
        pin(&mut pins, r("inherited-summary").join(f));
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared7steps,3native structural tests,2immutable program exports/censuses; no prover/field builds/proofs/timings");
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let tests = read(r("runs/parser-tests-run/tests.json"));
    assert_eq!(tests["unfiltered"], true);
    assert_eq!(tests["passed_tests"].as_array().unwrap().len(), 3);
    let mut cases = BTreeMap::new();
    let mut files = BTreeMap::new();
    for shape in ["1x2", "2x2"] {
        let v = read(r(format!("{shape}.json")));
        assert_eq!(v["complete"], true);
        assert_eq!(v["online_and_producer_index_counts_agree"], true);
        for (q, c) in v["queries"].as_object().unwrap() {
            let p = c["cuts"][9]["key_order_prefix_points"].as_u64().unwrap();
            let initial = c["input_ready_points"].as_u64().unwrap();
            cases.insert(format!("{shape}/{q}"),json!({"source_order_prefix_blocked_at90percent":p<=initial,"initial_points":initial,"points":c["points"],"cuts":c["cuts"]}));
        }
        files.insert(
            shape,
            json!({"solver_sha256":hash(r(format!("{shape}.solver"))),"census":v}),
        );
    }
    save(
        r("analysis.json"),
        &json!({"complete":true,"native_structural_tests":3,"cases":cases,"parsed_programs":files,"new_solver_executions":0,"SDK_requests":0,"new_independent_exports":0,"new_timing_samples":0,"distinct_proof_ledger":25231,"production_changed":false,"adopted":false,"goal_achieved":false}),
    );
    println!("Finished circuit-only readiness census; proof ledger25231 and production unchanged");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("setup") => setup(),
        Some("finish") => finish(),
        _ => panic!("setup/finish"),
    }
}
