use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};

fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let z = Path::new("/home/ubuntu/src/zk-tools/zopro");
    let d = z.join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176");
    let p = Path::new("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175");
    let q = Path::new("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default-compat175");
    let m = read(d.join("artifacts/production-source-manifest.json"));
    let decision = read(d.join("artifacts/decision.json"));
    assert_eq!(m["adopted"], true);
    assert_eq!(m["full_performance_gate_passed"], false);
    assert_eq!(m["final_artifact_gate_passed"], true);
    assert_eq!(m["resource_qualification_passed"], true);
    assert_eq!(m["goal_achieved"], false);
    assert_eq!(m["goal_status"], "active");
    assert_eq!(m["original8_CPU10x_rows"], 0);
    assert_eq!(
        hash(d.join("artifacts/decision.json")),
        m["decision_sha256"]
    );
    for (f, h) in read(d.join("artifacts/production-input-pins.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let sources = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(sources.len(), 176);
    for (rel, h) in sources {
        assert_eq!(hash(z.join(rel)), h.as_str().unwrap());
        assert_eq!(
            fs::read(z.join(rel)).unwrap(),
            fs::read(p.join("source").join(rel)).unwrap()
        );
    }
    let old = read(m["previous_accepted_manifest"].as_str().unwrap());
    assert_eq!(
        hash(m["previous_accepted_manifest"].as_str().unwrap()),
        m["previous_accepted_manifest_sha256"]
    );
    let mut unchanged = 0;
    for (rel, h) in old["source_files_sha256"].as_object().unwrap() {
        if sources[rel] == *h {
            unchanged += 1;
        }
        if rel.starts_with("scripts/") || rel.ends_with("ifma8.rs") {
            assert_eq!(&sources[rel], h);
        }
    }
    assert_eq!(unchanged, 172);
    assert_eq!(m["rustflags"], old["rustflags"]);
    assert_eq!(
        m["rustflags"],
        fs::read_to_string(p.join("native-flags.txt")).unwrap()
    );
    assert_eq!(
        m["rustflags"],
        read(p.join("frozen-candidate.json"))["rustflags"]
    );
    assert_eq!(m["environment"], old["environment"]);
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"]
    );
    let a = read(p.join("analysis.json"));
    assert_eq!(a["final_artifact_gate_passed"], true);
    assert_eq!(a["original_full_performance_gate_passed"], false);
    assert!(a["failed_checks"].as_array().unwrap().is_empty());
    assert_eq!(
        decision["primary32_ON_accepted"],
        a["pairs"]["1x2-w32/ON/accepted"]
    );
    assert_eq!(
        decision["primary32_ON_private"],
        a["pairs"]["1x2-w32/ON/OFF"]
    );
    let mut references = 0;
    let mut achieved = 0;
    for c in read(p.join("cases.json")).as_array().unwrap() {
        let n = c["name"].as_str().unwrap();
        let ms = a["cells"][format!("{n}/ON")]["median_ms"].as_f64().unwrap();
        assert_eq!(m["ms"][n], ms);
        if c["workers"] == 32 && !c["reference"].is_null() {
            let shape = c["shape"].as_str().unwrap();
            let cpu = c["reference"]["cpu_ms"].as_f64().unwrap();
            let metal = c["reference"]["metal_ms"].as_f64().unwrap();
            assert_eq!(m["m5"][shape]["accepted_ms"], ms);
            assert_eq!(m["m5"][shape]["cpu_speedup"], cpu / ms);
            assert_eq!(m["m5"][shape]["metal_speedup"], metal / ms);
            assert_eq!(m["m5"][shape]["required_ms_for_CPU10x"], cpu / 10.);
            if ms <= cpu / 10. {
                achieved += 1;
            }
            references += 1;
        }
    }
    assert_eq!((references, achieved), (8, 0));
    let qa = read(q.join("analysis.json"));
    assert_eq!(qa["qualified_additional_scope"], true);
    assert_eq!(qa["independent_proof_count"], 27353);
    let ledger: BTreeSet<String> =
        serde_json::from_value(read(d.join("artifacts/proof-sha256-after.json"))).unwrap();
    assert_eq!(ledger.len(), 27353);
    assert_eq!(
        read(d.join("artifacts/proof-sha256-after.json")),
        read(q.join("proof-sha256-after.json"))
    );
    assert_eq!(decision["qualification_SDK_requests"], 56036);
    assert_eq!(decision["qualification_independent_exports"], 1061);
    assert_eq!(decision["requests_tests_or_exports_during_promotion"], 0);
    let mut links = 0;
    let md = fs::read_to_string(d.join("README.md")).unwrap();
    let mut tail = md.as_str();
    while let Some(i) = tail.find("](") {
        tail = &tail[i + 2..];
        let j = tail.find(')').unwrap();
        let target = tail[..j].split('#').next().unwrap();
        if !target.starts_with("http") {
            assert!(d.join(target).exists(), "{target}");
            links += 1;
        }
        tail = &tail[j + 1..];
    }
    let result = json!({"passed":true,"source_files_exact_to_tested_measured_qualified_artifact":176,"old_originals_exact":172,"generated_arithmetic_and_scripts_unchanged":true,"all_production_input_pins_verified":true,"accepted_native_flags_and_settings_unchanged":true,"all_ten_local_rows_exact":true,"all_eight_CPU_and_Metal_comparisons_recomputed":true,"proof_ledger_exact":27353,"promotion_requests_tests_or_verifier_reruns":0,"readme_links_checked":links,"original_full_performance_gate":false,"final_artifact_and_resource_gates":true,"adopted":true,"goal_achieved":false,"original8_CPU10x_rows":0});
    fs::write(
        d.join("artifacts/post-adoption-review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Post-adoption review PASS:176 exact files, all pins/flags/settings/10rows/eightCPU+Metal/ledger27353; original gate false, final+resources true; goal0/8");
}
