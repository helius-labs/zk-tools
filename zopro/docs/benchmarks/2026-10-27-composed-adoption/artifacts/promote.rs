#![recursion_limit = "256"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-composed-adoption";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const C: &str = "/tmp/zopro-composed-cpu/candidate";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-composed-adoption";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn archived(root: &str, report: &str, file: &str) {
    let index = read(Path::new(report).join("artifact-index.json"));
    let expected = index[file].as_str().unwrap();
    assert_eq!(hash(Path::new(root).join(file)), expected);
    assert_eq!(
        hash(Path::new(report).join("artifacts").join(file)),
        expected
    );
}
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    let c = Path::new(C);
    let d = Path::new(D);
    let head = Command::new("git")
        .args(["-C", "/home/ubuntu/src/zk-tools", "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(head.status.success());
    assert_eq!(
        String::from_utf8(head.stdout).unwrap().trim(),
        "f096c7ac31fe464419c19ee9e7b441ef53911384"
    );
    let perfroot = "/tmp/zopro-composed-cpu";
    let perfreport = Path::new(P).join("docs/benchmarks/2026-10-27-composed-cpu-sdk");
    let qualroot = "/tmp/zopro-composed-qualification";
    let qualreport = Path::new(P).join("docs/benchmarks/2026-10-27-composed-qualification");
    for f in [
        "analysis.json",
        "source-audit.json",
        "source-pins.json",
        "native/composed/artifacts.json",
        "native/core/tests.json",
        "native/prover/tests.json",
        "record-memory-argument.md",
        "proof-inventory.json",
    ] {
        archived(perfroot, perfreport.to_str().unwrap(), f)
    }
    for f in [
        "analysis.json",
        "closeout.json",
        "proof-inventory.json",
        "portable/field/tests.json",
        "portable/core/tests.json",
        "portable/prover/tests.json",
        "qualification/result.json",
    ] {
        archived(qualroot, qualreport.to_str().unwrap(), f)
    }
    let a = read(Path::new(perfroot).join("analysis.json"));
    let q = read(Path::new(qualroot).join("analysis.json"));
    assert_eq!(a["full_gate_passed"], false);
    assert_eq!(a["calibration_failures"].as_array().unwrap().len(), 1);
    assert!(a["calibration_failures"][0]
        .as_str()
        .unwrap()
        .starts_with("1x2-w16 fresh/accepted interval"));
    for field in ["regressions", "primary_gain_failures", "aggregate_failures"] {
        assert!(a[field].as_array().unwrap().is_empty());
    }
    for (name, case) in a["cases"].as_object().unwrap() {
        for pair in ["composed/accepted", "composed/fresh"] {
            assert!(
                case["pairs"][pair]["median_confidence"]["upper"]
                    .as_f64()
                    .unwrap()
                    <= 1.02
            );
        }
        if case["workers"] == 32 {
            let ci = &case["pairs"]["fresh/accepted"]["median_confidence"];
            assert!(
                ci["lower"].as_f64().unwrap() >= 1. / 1.03 && ci["upper"].as_f64().unwrap() <= 1.03
            );
        }
        if name == "1x2-w32" {
            for pair in ["composed/accepted", "composed/fresh"] {
                assert_eq!(case["pairs"][pair]["wins"], 15);
                assert!(
                    case["pairs"][pair]["median_confidence"]["upper"]
                        .as_f64()
                        .unwrap()
                        < 1.
                );
            }
        }
    }
    assert_eq!(q["qualification_passed"], true);
    assert_eq!(q["next_prior_count"], 9154);
    assert_eq!(q["independent_exports"], 85);
    assert_eq!(q["portable_field_tests"], 11);
    assert_eq!(q["portable_core_tests"], 99);
    assert_eq!(q["portable_prover_tests"], 8);
    for inv in [
        Path::new(perfroot).join("proof-inventory.json"),
        Path::new(qualroot).join("proof-inventory.json"),
    ] {
        for (f, h) in read(inv).as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
        }
    }
    let previous_path = "/tmp/zopro-grouping-module/production-source-manifest.json";
    let old = read(previous_path);
    let originals = old["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 170);
    for (f, h) in originals {
        assert_eq!(hash(p.join(f)), h.as_str().unwrap());
    }
    let sourcepins = read(Path::new(perfroot).join("source-pins.json"));
    for (f, h) in sourcepins.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let receiptpath = Path::new(perfroot).join("native/composed/artifacts.json");
    let receipt = read(&receiptpath);
    for (f, h) in receipt["input_pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let binary = receipt["binary"].as_str().unwrap();
    assert_eq!(
        hash(binary),
        "8241648f298893953bcacac515d3d52a7f2d363cb264e08d232af3037decb265"
    );
    let audit = read(Path::new(perfroot).join("source-audit.json"));
    assert_eq!(
        audit["all170accepted_bytes_recovered_by_reversing_composition"],
        true
    );
    assert_eq!(audit["checked_forecasts_charge_new_vector_headers"], true);
    let changed: Vec<String> = serde_json::from_value(audit["changed"].clone()).unwrap();
    assert_eq!(changed.len(), 4);
    let newfile = audit["new_module"].as_str().unwrap();
    assert!(!p.join(newfile).exists());
    let mut nextfiles = BTreeMap::new();
    let mut diff = Vec::new();
    for (f, h) in originals {
        let candidate = hash(c.join(f));
        if candidate != h.as_str().unwrap() {
            assert!(changed.contains(f));
            diff.push(json!({"file":f,"previous_sha256":h,"new_sha256":candidate}));
        } else {
            assert!(!changed.contains(f));
        }
        nextfiles.insert(f.clone(), candidate);
    }
    assert_eq!(diff.len(), 4);
    nextfiles.insert(newfile.into(), hash(c.join(newfile)));
    assert_eq!(nextfiles.len(), 171);
    // Fresh writes preserve the exact qualified bytes and refresh mtimes.
    // Never restore old source timestamps and rely on stale Cargo output.
    for f in changed.iter().map(String::as_str).chain([newfile]) {
        fs::write(p.join(f), fs::read(c.join(f)).unwrap()).unwrap();
    }
    for (f, h) in &nextfiles {
        assert_eq!(&hash(p.join(f)), h);
        assert_eq!(&hash(c.join(f)), h);
    }
    save(
        r.join("source-promotion.json"),
        &json!({"previous_source_manifest":previous_path,"previous_manifest_sha256":hash(previous_path),"previous_files":170,"new_files":171,"changed_originals":diff,"unchanged_originals":166,"new_private_module":newfile,"all171production_files_exactly_match_measured_candidate":true,"formatting_changes_after_tests":false,"native_arithmetic_rebuilt":false,"new_proofs_or_benchmarks":0}),
    );
    let mut ms = serde_json::Map::new();
    let mut m5 = serde_json::Map::new();
    for (name, case) in a["cases"].as_object().unwrap() {
        let t = case["modes"]["composed"]["ms"].as_f64().unwrap();
        ms.insert(name.clone(), json!(t));
        if case["workers"] == 32 {
            if let Some(cpu) = case["reference"]["cpu_ms"].as_f64() {
                let metal = case["reference"]["metal_ms"].as_f64().unwrap();
                m5.insert(case["shape"].as_str().unwrap().into(),json!({"cpu_reference_ms":cpu,"metal_reference_ms":metal,"accepted_ms":t,"cpu_speedup":cpu/t,"metal_speedup":metal/t,"required_ms_for_CPU10x":cpu/10.,"goal10x":t<=cpu/10.}));
            }
        }
    }
    assert_eq!(m5.len(), 8);
    let now = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .unwrap();
    let now = String::from_utf8(now.stdout).unwrap().trim().to_owned();
    let rustflags = read(Path::new(perfroot).join("native-plan.json"))["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "on-libraries")
        .unwrap()["environment"]["RUSTFLAGS"]
        .clone();
    let manifest = json!({"adopted":true,"adopted_at":now,"review_base_commit":"f096c7ac31fe464419c19ee9e7b441ef53911384","adoption_kind":"explicit post-study engineering decision with documented physical16 calibration uncertainty","adoption_review_passed":true,"qualified_scope_gates_passed":false,"full_performance_gate_passed":false,"resource_qualification_passed":true,"calibration_failures_retained":a["calibration_failures"],"decision":r.join("decision.md"),"decision_sha256":hash(r.join("decision.md")),"previous_accepted_manifest":previous_path,"previous_accepted_manifest_sha256":hash(previous_path),"source_files_sha256":nextfiles,"source_changes":changed,"new_private_module":newfile,"source_snapshot":c.join(".xtask/snapshot.json"),"native_build_source_pins":Path::new(perfroot).join("source-pins.json"),"native_build_source_pins_sha256":hash(Path::new(perfroot).join("source-pins.json")),"native_build_source_snapshot":c.join(".xtask/snapshot.json"),"native_build":Path::new(perfroot).join("native/result.json"),"native_cli_build":receiptpath,"native_binary":binary,"native_binary_sha256":receipt["binary_sha256"],"native_binary_interface":"cpu-sdk IMAGE REQUEST_JSON ROUNDS [PROOF_DIRECTORY]","standard_cli_source":p.join("crates/zopro-bench/examples/cpu-sdk.rs"),"standard_cli_source_sha256":hash(p.join("crates/zopro-bench/examples/cpu-sdk.rs")),"standard_cli_helper":p.join("crates/zopro-bench/examples/support/perf.rs"),"standard_cli_helper_sha256":hash(p.join("crates/zopro-bench/examples/support/perf.rs")),"rustflags":rustflags,"environment":read(Path::new(perfroot).join("perf-plan.json"))["environment"],"basis_images":old["basis_images"],"basis_image_hashes":old["basis_image_hashes"],"large_images":old["large_images"],"performance_protocol":Path::new(perfroot).join("perf-plan.json"),"performance_result":Path::new(perfroot).join("analysis.json"),"performance_result_sha256":hash(Path::new(perfroot).join("analysis.json")),"final_performance_report":perfreport.join("README.md"),"qualification_report":qualreport.join("README.md"),"qualification_result":Path::new(qualroot).join("analysis.json"),"qualification_result_sha256":hash(Path::new(qualroot).join("analysis.json")),"limits":old["limits"],"ms":ms,"m5":m5,"goal_achieved":false,"original8_CPU10x_rows":0,"AWS_comparison_matches_current_source":false,"historical_C7a_source_manifest":previous_path,"native_core_tests_inherited":116,"native_raw_tests_inherited":2,"native_cubic_tests_inherited":6,"portable_field_tests":11,"portable_core_tests":99,"portable_prover_tests":8,"supporting_studies":[{"root":perfroot,"SDK_requests":22260,"independent_exports":465},{"root":"/tmp/zopro-physical16-scheduling","SDK_requests":4536,"independent_exports":9},{"root":qualroot,"SDK_requests":216,"core_requests":4,"independent_exports":85}],"scope":"Exact clean borrowed G1 pending operands, adaptive G2 prefix/norm records and direct width-three Poseidon formulas; conservative vector-header forecasts; original fallbacks and compiler/runtime options retained.","limitations":["Original frozen full performance gate remains false solely for physical16 fresh/accepted calibration. Adoption explicitly accepts that uncertainty; no study verdict is rewritten.","Individual paired intervals assume independent/exchangeable blocks and are not familywise simultaneous guarantees. Variability remains at16workers.","Lifetime RSS is mixed across shapes;24x2 increased about49MiB. No universal peak-RAM saving or minimum-RAM bound is claimed.","Small primary limit cases do not trigger high-limit reclaim;4999 does exercise hard-limit reclaim under95GiB without OOM/swap.","New source is not measured on AWS/Aeglos/ARM/physical phones/TEE. Existing C7a comparison belongs to the previous source.","No giant concurrent-proof, throughput, arbitrary application fairness or universal transient allocation qualification.","All-eight supplied CPU10x goal remains unmet; no target or required shape is removed."]});
    save(r.join("production-source-manifest.json"), &manifest);
    save(
        r.join("review.json"),
        &json!({"source_adopted":true,"adoption_review_passed":true,"full_frozen_performance_gate_passed":false,"resource_qualification_passed":true,"calibration_uncertainty_explicitly_accepted":a["calibration_failures"],"all_candidate_nonregression_bounds_passed":true,"all_nine32worker_control_calibrations_passed":true,"no_observation_excluded_or_repeated":true,"source_identity":read(r.join("source-promotion.json")),"new_independent_exports":0,"prior_proof_count_unchanged":9154,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    for n in [
        "decision.md",
        "promote.rs",
        "promote-v1.rs",
        "preparation-failures.json",
        "source-promotion.json",
        "production-source-manifest.json",
        "review.json",
    ] {
        fs::copy(r.join(n), d.join("artifacts").join(n)).unwrap();
        index.insert(n.to_owned(), hash(d.join("artifacts").join(n)));
    }
    save(d.join("artifact-index.json"), &json!(index));
    let mut md=String::from("# Adopt combined CPU implementation\n\nThe working baseline now uses the exact qualified borrowed-G1/adaptive-G2/Poseidon source. Primary32 measures **8.372 ms versus8.653 ms** previously, with a paired median **3.13%** reduction and15/15wins; the original-eight aggregate improves1.98%. Native, portable, fallback, concurrent-call and declared memory checks pass. No new benchmark or proof is generated by promotion.\n\nThis is an **explicit post-study engineering decision** accepting the remaining physical16 control-calibration uncertainty. The [original full performance gate](../2026-10-27-composed-cpu-sdk/README.md#frozen-qualification) **remains false**. Its only failed check is control-to-control calibration at16workers; all candidate nonregression bounds pass, including16workers against both controls. The candidate's16-worker ratio intervals are[0.938349,0.990891] against accepted and[0.903113,0.974211] against fresh. This does not resolve control equivalence, establish a cause for variability, or turn the frozen study into a passing experiment. No observation is excluded, rerun or relabeled. See the [decision and retained limitations](artifacts/decision.md).\n\n## Source and memory\n\nFour source files change and one private module is added; all171 production files match the measured candidate byte-for-byte. Safe borrowed G1 operands retain owned fallbacks. G2 uses the existing fixed32ordinary-pair adaptive rule and prefix/norm records; width-three Poseidon uses the existing two-product operation plus its third product. Checked forecasts conservatively charge new vector headers. No point table, key copy, duplicate plan, new compiler/runtime option or per-circuit/core timing selector is added. Field arithmetic, key/image formats and fresh blinding remain unchanged.\n\n[Resource qualification](../2026-10-27-composed-qualification/README.md) includes native116 core/raw2/cubic6 receipts, fresh portable11field99core8prover, all nine SDK shapes, raw/preparation-denied and requested small-stack cases, four concurrent callers and both allocator policies. Both4999 proofs independently verify under95GiB/swap0 at166.524/164.686s with94.684GiB peak process RSS. Small primary cap cases fit without triggering soft-limit reclaim;4999 exercises hard-limit reclaim. These are tested capacities, not minimum RAM or matched large-circuit speed comparisons. Lifetime RSS is mixed:24x2 increases about49MiB while54decreases about87MiB.\n\n## Existing verified measurements\n\nThese values are the combined column of the unchanged fifteen-block latency study. Supplied M5 CPU/Metal references are historical, not fresh Apple measurements. No54reference was supplied. No new AWS/Aeglos result is claimed; the prior C7a comparison belongs to the previous source.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for name in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
        "merge-54x1-w32",
    ] {
        let case = &a["cases"][name];
        let t = case["modes"]["composed"]["ms"].as_f64().unwrap();
        if let Some(cpu) = case["reference"]["cpu_ms"].as_f64() {
            let metal = case["reference"]["metal_ms"].as_f64().unwrap();
            writeln!(
                md,
                "|{}|{t:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
                case["shape"].as_str().unwrap(),
                cpu / t,
                metal / t,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(md, "|54→1|{t:.6}|—|—|—|—|—|").unwrap();
        }
    }
    md.push_str("\nNative arithmetic is not rebuilt solely for byte-identical source promotion. Exact qualified SDK: `/tmp/zopro-composed-cpu/native/composed-sdk`, SHA-256 `8241648f298893953bcacac515d3d52a7f2d363cb264e08d232af3037decb265`. The [new manifest](artifacts/production-source-manifest.json) records `adoption_review_passed=true`, `full_performance_gate_passed=false`, `qualified_scope_gates_passed=false` and `resource_qualification_passed=true` separately. The old accepted manifest/artifacts and all original study verdicts remain retained.\n\nSource/library/report/proof pins are checked during promotion; all171 production bytes match the qualified snapshot without formatting cleanup. The global proof-identity ledger stays9,154; supporting studies retain their original counts. There is no new native build, performance rerun or proof export to count. The original all-eight CPU10× goal remains **active and unmet0/8**.\n\n[Source promotion audit](artifacts/source-promotion.json), [review](artifacts/review.json), [manifest](artifacts/production-source-manifest.json), [artifact hashes](artifact-index.json).\n");
    fs::write(d.join("README.md"), md).unwrap();
    println!("Promoted exact171 candidate source files; post-study uncertainty explicitly accepted; originalfullgatefalse; ledger9154 unchanged; no new proofs");
}
