#![recursion_limit = "256"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};

const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-adoption176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default-compat175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const PREVIOUS: &str =
    "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
const D: &str = "docs/benchmarks/2026-10-30-g2-vector-sign-adoption176";
const SDK_HASH: &str = "6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661";
const CHANGES: [&str; 4] = [
    "crates/zopro-bn254/src/lib.rs",
    "crates/zopro-bn254/src/signed_delta.rs",
    "crates/zopro-core/src/simd.rs",
    "crates/zopro-core/src/msm/g2_borrowed.rs",
];

fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let mode = std::env::args().nth(1).expect("review/adopt");
    assert!(["review", "adopt"].contains(&mode.as_str()));
    let r = Path::new(R);
    let p = Path::new(P);
    let q = Path::new(Q);
    let z = Path::new(Z);
    let previous_path = z.join(PREVIOUS);
    let previous = read(&previous_path);
    let performance = read(p.join("analysis.json"));
    let qualification = read(q.join("analysis.json"));
    assert_eq!(performance["final_artifact_gate_passed"], true);
    assert_eq!(performance["original_full_performance_gate_passed"], false);
    assert!(performance["failed_checks"].as_array().unwrap().is_empty());
    assert_eq!(qualification["qualified_additional_scope"], true);
    assert_eq!(qualification["independent_proof_count"], 27353);
    assert_eq!(qualification["SDK_requests"], 248);
    assert_eq!(qualification["new_independent_exports"], 89);
    assert_eq!(read(q.join("qualification-review.json"))["passed"], true);
    assert_eq!(read(p.join("final-source-review.json"))["passed"], true);
    let original = read("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-sdk175/analysis.json");
    assert_eq!(original["gate_passed"], false);
    assert_eq!(original["failed_checks"], json!(["1x2-w16/OFF/accepted"]));
    assert_eq!(hash(p.join("native/candidate-sdk")), SDK_HASH);
    for (f, h) in previous["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(z.join(f)),
            h.as_str().unwrap(),
            "production changed: {f}"
        );
    }
    assert!(!z.join(CHANGES[1]).exists());
    let parent_pins = read(p.join("build-pins.json"));
    let prefix = format!("{P}/source/");
    let mut source = BTreeMap::new();
    let mut inputs = BTreeMap::new();
    for (f, h) in parent_pins.as_object().unwrap() {
        if let Some(rel) = f.strip_prefix(&prefix) {
            assert_eq!(hash(f), h.as_str().unwrap());
            assert_eq!(hash(q.join("source").join(rel)), h.as_str().unwrap());
            source.insert(rel.to_string(), h.as_str().unwrap().to_string());
            inputs.insert(f.clone(), h.as_str().unwrap().to_string());
        }
    }
    assert_eq!(source.len(), 176);
    let changed = source
        .keys()
        .filter(|f| {
            previous["source_files_sha256"]
                .get(*f)
                .and_then(Value::as_str)
                != Some(source[*f].as_str())
        })
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(changed, CHANGES.into_iter().map(str::to_string).collect());
    for (file, expected) in [
        (p.join("native/candidate-field/tests.json"), 35),
        (p.join("native/candidate-core/tests.json"), 125),
        (p.join("native/candidate-prover/tests.json"), 8),
        (p.join("native/portable-field/tests.json"), 11),
        (p.join("native/portable-core/tests.json"), 101),
        (p.join("native/portable-prover/tests.json"), 8),
    ] {
        let tests = read(&file);
        assert_eq!(tests["unfiltered"], true);
        assert_eq!(tests["passed_tests"].as_array().unwrap().len(), expected);
        inputs.insert(file.display().to_string(), hash(file));
    }
    let report = z.join("docs/benchmarks/2026-10-30-g2-vector-sign-default175");
    let compat_report = z.join("docs/benchmarks/2026-10-30-g2-vector-sign-default-compat175");
    for base in [&report, &compat_report] {
        let review = read(base.join("artifacts/post-publication-review.json"));
        assert_eq!(review["passed"], true);
        for name in [
            "README.md",
            "SHA256SUMS",
            "artifacts/analysis.json",
            "artifacts/review.json",
            "artifacts/post-publication-review.json",
        ] {
            let file = base.join(name);
            inputs.insert(file.display().to_string(), hash(file));
        }
    }
    for file in [
        previous_path.clone(),
        p.join("analysis.json"),
        q.join("analysis.json"),
        p.join("final-source-review.json"),
        q.join("qualification-review.json"),
        q.join("proof-sha256-after.json"),
        p.join("native/candidate-sdk"),
        p.join("native-flags.txt"),
        r.join("adopt.rs"),
        r.join("adopt"),
    ] {
        inputs.insert(file.display().to_string(), hash(file));
    }
    let review = json!({
        "passed": true, "original_full_performance_gate_passed": false,
        "final_artifact_gate_passed": true, "functional_resource_gate_passed": true,
        "source_files": 176, "originals_exact": 172, "changes": CHANGES,
        "new_native_field_core_prover_tests": [35,125,8],
        "new_portable_field_core_prover_tests": [11,101,8],
        "source_exact_to_actual_tested_and_measured_default": true,
        "native_binary_sha256": SDK_HASH, "proof_ledger": 27353,
        "original8_CPU10x_rows": 0, "goal_achieved": false,
        "decision": "adopt under an explicit engineering exception to the original physical16 control calibration",
        "reason": "Default primary gain passes the prospective final-artifact gate, all candidate nonregression and memory checks pass, and the exact default artifact passes functional/resource qualification. The original failed calibration remains false and recorded."
    });
    if mode == "review" {
        assert!(!r.join("pre-adoption-review.json").exists());
        save(r.join("pre-adoption-review.json"), &review);
        save(r.join("input-pins.json"), &json!(inputs));
        println!("Adoption review PASS: exact 176-file candidate, six suites, final performance, functional/resource checks; historical gate remains false; production unchanged");
        return;
    }
    assert_eq!(read(r.join("pre-adoption-review.json")), review);
    for (file, h) in read(r.join("input-pins.json")).as_object().unwrap() {
        assert_eq!(hash(file), h.as_str().unwrap());
    }
    let d = z.join(D);
    assert!(!d.exists());
    for rel in CHANGES {
        fs::copy(p.join("source").join(rel), z.join(rel)).unwrap();
    }
    for (rel, h) in &source {
        assert_eq!(hash(z.join(rel)), *h);
    }
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut ms = BTreeMap::new();
    let mut rows = BTreeMap::new();
    let mut m5 = BTreeMap::new();
    let cases = read(p.join("cases.json"));
    let mut attained = 0;
    for case in cases.as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let cell = &performance["cells"][format!("{name}/ON")];
        let time = cell["median_ms"].as_f64().unwrap();
        ms.insert(name.to_string(), time);
        rows.insert(name.to_string(), cell.clone());
        if case["workers"] == 32 && !case["reference"].is_null() {
            let cpu = case["reference"]["cpu_ms"].as_f64().unwrap();
            let metal = case["reference"]["metal_ms"].as_f64().unwrap();
            if time <= cpu / 10. {
                attained += 1;
            }
            m5.insert(case["shape"].as_str().unwrap().to_string(), json!({"accepted_ms":time,"cpu_reference_ms":cpu,"metal_reference_ms":metal,"cpu_speedup":cpu/time,"metal_speedup":metal/time,"required_ms_for_CPU10x":cpu/10.,"goal10x":time<=cpu/10.}));
        }
    }
    assert_eq!(attained, 0);
    let decision = json!({
        "adopted":true,"review_passed":true,"production_changed":true,
        "adoption_kind":"engineering exception to original physical16 fresh-control calibration",
        "original_full_performance_gate_passed":false,"failed_checks_retained":original["failed_checks"],
        "original_calibration":original["pairs"]["1x2-w16/OFF/accepted"],
        "final_artifact_gate_passed":true,"final_artifact_failed_checks":performance["failed_checks"],
        "primary32_ON_accepted":performance["pairs"]["1x2-w32/ON/accepted"],
        "primary32_ON_private":performance["pairs"]["1x2-w32/ON/OFF"],
        "original8_aggregates":performance["original8_aggregates"],
        "native_binary":p.join("native/candidate-sdk"),"native_binary_sha256":SDK_HASH,
        "source_files":176,"originals_exact":172,"source_changes":CHANGES,
        "native_field_core_prover_tests":[35,125,8],"portable_field_core_prover_tests":[11,101,8],
        "functional_resource_qualification_passed":true,"qualification_SDK_requests":55788+248,
        "qualification_independent_exports":972+89,"proof_ledger":27353,
        "requests_tests_or_exports_during_promotion":0,"new_heap_table_key_copy_pool_or_plan":false,
        "goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,
        "reason":review["reason"],"limitations":["Original private full gate remains false for physical16 control calibration; previous adoption exceptions remain historical","No demonstrated improvement over private implementation; its primary interval includes parity","Memory observations mixed across shapes; cgroup charges are not standalone RAM; zero high events do not establish active throttling","No new AWS/Aeglos, phone/ARM/TEE, huge250/4999, swap, minimum-RAM or concurrent-throughput result","Separate CLI build unverified; standard SDK engine qualified"]
    });
    save(d.join("artifacts/decision.json"), &decision);
    let manifest = json!({
        "adopted":true,"adoption_review_passed":true,"adoption_kind":decision["adoption_kind"],
        "decision":d.join("artifacts/decision.json"),"decision_sha256":hash(d.join("artifacts/decision.json")),
        "previous_accepted_manifest":previous_path,"previous_accepted_manifest_sha256":hash(z.join(PREVIOUS)),
        "previous_accepted_limitations_retained":previous["limitations"],
        "rustflags":previous["rustflags"],"environment":previous["environment"],
        "native_binary":p.join("native/candidate-sdk"),"native_binary_sha256":SDK_HASH,
        "native_binary_interface":"cpu-sdk IMAGE REQUEST_JSON ROUNDS [PROOF_DIRECTORY]",
        "native_cli_build":"unverified","native_build":p.join("native/result.json"),
        "native_build_source_pins":d.join("artifacts/production-input-pins.json"),
        "native_original_build_source_pins":p.join("build-pins.json"),
        "native_build_source_snapshot":p.join("source"),"source_snapshot":p.join("source"),
        "source_files_sha256":source,"source_changes":CHANGES,
        "native_field_tests":35,"native_core_tests":125,"native_cubic_tests":8,
        "portable_field_tests":11,"portable_core_tests":101,"portable_prover_tests":8,
        "full_performance_gate_passed":false,"final_artifact_gate_passed":true,
        "performance_failures_retained":original["failed_checks"],
        "performance_result":p.join("analysis.json"),"performance_result_sha256":hash(p.join("analysis.json")),
        "performance_protocol":p.join("capture-design.md"),"final_performance_report":report.join("README.md"),
        "qualification_result":q.join("analysis.json"),"qualification_result_sha256":hash(q.join("analysis.json")),
        "qualified_scope_gates_passed":true,"resource_qualification_passed":true,
        "resource_qualification_result":q.join("resource-result.json"),"resource_qualification_result_sha256":hash(q.join("resource-result.json")),
        "qualification_scope":"Actual default native/portable field/core/prover, 45 functional plus four concurrent SDK processes, eight 32-worker configured-memory cases; exact measured and qualified source",
        "proof_ledger":27353,"ms":ms,"m5":m5,"limitations":decision["limitations"],
        "AWS_comparison_matches_current_source":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0
    });
    let mut production_pins = inputs;
    for (rel, h) in &source {
        production_pins.insert(z.join(rel).display().to_string(), h.clone());
    }
    save(
        d.join("artifacts/production-input-pins.json"),
        &json!(production_pins),
    );
    save(
        d.join("artifacts/production-source-manifest.json"),
        &manifest,
    );
    save(d.join("artifacts/local-rows.json"), &json!(rows));
    for (from, to) in [
        (
            r.join("pre-adoption-review.json"),
            "pre-adoption-review.json",
        ),
        (r.join("input-pins.json"), "pre-adoption-input-pins.json"),
        (r.join("adopt.rs"), "adopt.rs"),
        (q.join("proof-sha256-after.json"), "proof-sha256-after.json"),
    ] {
        fs::copy(from, d.join("artifacts").join(to)).unwrap();
    }
    fs::copy(
        report.join("accepted-rows.json"),
        d.join("historical-accepted-rows.json"),
    )
    .unwrap();
    let mut md = String::from("# Default G2 vector-sign adoption\n\nAdopt the measured signed canonical G2 coordinate difference under the existing native guards. Primary at 32 workers measures **7.928660 ms versus 8.041215 ms**, a paired **1.54% improvement** with 30/31 wins. Peak RSS is **132.75 versus 135 MiB**. The all-eight equal-log paired improvement is 0.377%; memory observations are mixed across shapes.\n\nThis is an explicit engineering exception to the original private-candidate 16-core fresh-control calibration failure. That full gate remains **false**. The distinct, prospective final-artifact gate passes every check, and the exact default artifact passes the functional/resource qualification. The interval against the private implementation includes parity; no improvement over that implementation is established. Earlier adoption exceptions remain in the previous manifest.\n\nThe production source is byte-exact to the tested 176-file snapshot: three existing files change and one handwritten field module is added. Generated arithmetic and generator scripts stay unchanged. The experiment-only flag is removed. Arithmetic, strict normalized output below 2p, negative-zero correction, native guards and fallback behavior are preserved. No heap allocation, per-key table, persistent point copy, second execution plan or pool is added. Accepted native flags and runtime settings remain unchanged.\n\nFresh actual-artifact suites pass: native field/core/prover 35/125/8; baseline-x86 with IFMA disabled 11/101/8. The same default artifact passes 45 functional processes, four concurrent processes and eight configured-memory cases. Across performance and compatibility qualification, 56,036 SDK requests verify internally and 1,061 fresh exports independently pass Gnark. The verified ledger is 27,353. Promotion adds no test, proof, profile or timing capture; all 176 production source hashes match the qualified artifact.\n\nMemory cases use 32 workers, swap disabled, a 192 MiB maximum and a 128 MiB warm high limit derived from the accepted 135 MiB peak RSS. All eight exit normally with zero high/max/OOM events. Candidate cgroup peaks are 61.62–62.96 MiB cold and 104.49–104.59 MiB warm, including charged private image/controller/SDK/ballast. Shared mappings can be charged elsewhere. These are not standalone or minimum-RAM estimates, and zero high events do not establish active throttling. No new AWS/Aeglos, phone/ARM/TEE, huge250/4999, swap or concurrent-throughput result is claimed. Separate CLI build is unverified; the standard SDK engine is qualified.\n\n## Supplied M5 comparisons\n\n|Shape|Adopted ms|Peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|---:|\n");
    for case in cases
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let cell = &performance["cells"][format!("{}/ON", case["name"].as_str().unwrap())];
        let time = cell["median_ms"].as_f64().unwrap();
        let rss = cell["max_RSS_MiB"].as_f64().unwrap();
        if let Some(cpu) = case["reference"]["cpu_ms"].as_f64() {
            let metal = case["reference"]["metal_ms"].as_f64().unwrap();
            writeln!(
                md,
                "|{}|{time:.6}|{rss:.2}|{cpu:.0} / {:.3}×|{metal:.0} / {:.3}×|{:.1}|",
                case["shape"].as_str().unwrap(),
                cpu / time,
                metal / time,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(
                md,
                "|{}|{time:.6}|{rss:.2}|—|—|—|",
                case["shape"].as_str().unwrap()
            )
            .unwrap();
        }
    }
    md.push_str("\nThe original all-eight CPU 10× objective remains **active and unmet, 0/8**. Primary requires 3.7 ms. No M5 reference was supplied for 54→1. Historical AWS rows describe the earlier source and are not measurements of this adoption.\n\n[Performance](../2026-10-30-g2-vector-sign-default175/README.md), [functional/resources](../2026-10-30-g2-vector-sign-default-compat175/README.md), [decision](artifacts/decision.json), [production manifest](artifacts/production-source-manifest.json), [local rows](artifacts/local-rows.json), [historical accepted rows](historical-accepted-rows.json).\n");
    fs::write(d.join("README.md"), md).unwrap();
    println!("Adopted exact default vector-sign source176; final-artifact and functional/resource checks pass; original full gate remains false; goal active/unmet0/8");
}
