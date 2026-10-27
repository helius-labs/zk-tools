#![recursion_limit = "256"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-clean-fft-adoption";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const C: &str = "/tmp/zopro-clean-fft-leaves/candidate";
const PERF: &str = "/tmp/zopro-clean-fft-leaves";
const QUAL: &str = "/tmp/zopro-clean-fft-qualification";
const OLD: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
const BASE: &str = "924ee491b9a50575fb49aa022ff7a8b908f279b5";
const SDK: &str = "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a";
const CHANGED: &str = "crates/zopro-core/src/fft/pair.rs";
const ADDED: &str = "crates/zopro-core/src/fft/pair/leaf.rs";
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
fn archived(root: &Path, report: &Path, file: &str) {
    let index = read(report.join("artifact-index.json"));
    let want = index[file].as_str().unwrap();
    assert_eq!(hash(root.join(file)), want);
    assert_eq!(hash(report.join("artifacts").join(file)), want);
}
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    let c = Path::new(C);
    let perf = Path::new(PERF);
    let qual = Path::new(QUAL);
    let perfreport = p.join("docs/benchmarks/2026-10-27-clean-fft-leaves");
    let qualreport = p.join("docs/benchmarks/2026-10-27-clean-fft-qualification");
    let d = p.join("docs/benchmarks/2026-10-27-clean-fft-adoption");
    let head = Command::new("git")
        .args(["-C", P, "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(head.status.success());
    assert_eq!(String::from_utf8(head.stdout).unwrap().trim(), BASE);
    for f in [
        "analysis.json",
        "source-audit.json",
        "source-pins.json",
        "integration-review.md",
        "native/candidate/artifacts.json",
        "native/candidate-core/tests.json",
        "native/candidate-prover/tests.json",
        "proof-inventory.json",
    ] {
        archived(perf, &perfreport, f);
    }
    for f in [
        "analysis.json",
        "closeout.json",
        "proof-inventory.json",
        "resource-result.json",
        "engineering-review-plan.md",
        "qualification/result.json",
        "portable/field/tests.json",
        "portable/core/tests.json",
        "portable/prover/tests.json",
    ] {
        archived(qual, &qualreport, f);
    }
    let a = read(perf.join("analysis.json"));
    let q = read(qual.join("analysis.json"));
    assert_eq!(a["full_gate_passed"], false);
    assert!(a["calibration_failures"].as_array().unwrap().is_empty());
    assert!(a["aggregate_failures"].as_array().unwrap().is_empty());
    assert_eq!(a["primary_gain_failures"].as_array().unwrap().len(), 2);
    assert_eq!(
        a["regressions"],
        json!(["8x2-w32 candidate/fresh upper=1.0241365008166312"])
    );
    for pair in ["candidate/accepted", "candidate/fresh"] {
        assert!(a["original8"][pair]["upper"].as_f64().unwrap() < 1.);
        assert!(
            a["cases"]["1x2-w32"]["pairs"][pair]["median_confidence"]["upper"]
                .as_f64()
                .unwrap()
                <= 1.02
        );
    }
    for (name, case) in a["cases"].as_object().unwrap() {
        for pair in ["candidate/accepted", "candidate/fresh"] {
            let upper = case["pairs"][pair]["median_confidence"]["upper"]
                .as_f64()
                .unwrap();
            if name == "8x2-w32" && pair == "candidate/fresh" {
                assert!(upper > 1.02 && upper < 1.025)
            } else {
                assert!(upper <= 1.02)
            }
        }
    }
    assert_eq!(q["qualification_passed"], true);
    assert_eq!(q["independent_exports"], 85);
    assert_eq!(q["next_prior_count"], 11660);
    assert_eq!(q["portable_field_tests"], 11);
    assert_eq!(q["portable_core_tests"], 99);
    assert_eq!(q["portable_prover_tests"], 8);
    for root in [perf, qual] {
        for (f, h) in read(root.join("proof-inventory.json")).as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
        }
    }
    let old = read(OLD);
    let originals = old["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 171);
    for (f, h) in originals {
        assert_eq!(hash(p.join(f)), h.as_str().unwrap());
    }
    let audit = read(perf.join("source-audit.json"));
    assert_eq!(audit["inverse_recovers_original"], true);
    assert_eq!(audit["candidate_exact_originals"], 170);
    assert_eq!(audit["no_new_unsafe_flag_buffer_or_key_table"], true);
    for (f, h) in read(perf.join("source-pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let snapshot = read(c.join(".xtask/snapshot.json"));
    let snapshotpins = snapshot["source_pins"].as_object().unwrap();
    assert_eq!(snapshotpins.len(), 172);
    let mut nextfiles = BTreeMap::new();
    let mut differences = Vec::new();
    for (f, h) in snapshotpins {
        assert_eq!(hash(f), h.as_str().unwrap());
        let rel = Path::new(f).strip_prefix(c).unwrap().to_str().unwrap();
        if let Some(previous) = originals.get(rel) {
            if previous != h {
                assert_eq!(rel, CHANGED);
                differences.push(json!({"file":rel,"previous_sha256":previous,"new_sha256":h}));
            }
        } else {
            assert_eq!(rel, ADDED);
            assert!(!p.join(rel).exists());
        }
        nextfiles.insert(rel.to_owned(), h.as_str().unwrap().to_owned());
    }
    assert_eq!(differences.len(), 1);
    assert_eq!(nextfiles.len(), 172);
    assert!(originals.keys().all(|k| nextfiles.contains_key(k)));
    let receiptpath = perf.join("native/candidate/artifacts.json");
    let receipt = read(&receiptpath);
    assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
    for (f, h) in receipt["input_pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let binary = receipt["binary"].as_str().unwrap();
    assert_eq!(hash(binary), SDK);
    assert_eq!(receipt["binary_sha256"], SDK);
    let plan = read(perf.join("native-plan.json"));
    let flags = &plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == "candidate-libraries")
        .unwrap()["environment"]["RUSTFLAGS"];
    assert_eq!(flags, &old["rustflags"]);
    let decision = fs::read_to_string(r.join("decision.md")).unwrap();
    assert!(decision.contains(SDK));
    // All checks above precede mutation. Copy only the measured source bytes.
    for f in [CHANGED, ADDED] {
        fs::create_dir_all(p.join(f).parent().unwrap()).unwrap();
        fs::write(p.join(f), fs::read(c.join(f)).unwrap()).unwrap();
    }
    for (f, h) in &nextfiles {
        assert_eq!(&hash(p.join(f)), h);
        assert_eq!(&hash(c.join(f)), h);
    }
    save(
        r.join("source-promotion.json"),
        &json!({"previous_source_manifest":OLD,"previous_manifest_sha256":hash(OLD),"previous_files":171,"new_files":172,"changed_originals":differences,"unchanged_originals":170,"new_private_module":ADDED,"all172production_files_exactly_match_measured_candidate":true,"formatting_changes_after_tests":false,"native_arithmetic_rebuilt":false,"new_proofs_or_benchmarks":0}),
    );
    let mut ms = serde_json::Map::new();
    let mut m5 = serde_json::Map::new();
    for (name, case) in a["cases"].as_object().unwrap() {
        let t = case["modes"]["candidate"]["ms"].as_f64().unwrap();
        ms.insert(name.clone(), json!(t));
        if case["workers"] == 32 {
            if let Some(cpu) = case["reference"]["cpu_ms"].as_f64() {
                let metal = case["reference"]["metal_ms"].as_f64().unwrap();
                m5.insert(case["shape"].as_str().unwrap().into(),json!({"accepted_ms":t,"cpu_reference_ms":cpu,"cpu_speedup":cpu/t,"metal_reference_ms":metal,"metal_speedup":metal/t,"required_ms_for_CPU10x":cpu/10.,"goal10x":t<=cpu/10.}));
            }
        }
    }
    assert_eq!(m5.len(), 8);
    assert!(m5.values().all(|v| v["goal10x"] == false));
    let now = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .unwrap();
    assert!(now.status.success());
    let now = String::from_utf8(now.stdout).unwrap().trim().to_owned();
    let mut manifest = old.clone();
    let m = manifest.as_object_mut().unwrap();
    for (k,v) in json!({"adopted":true,"adopted_at":now,"review_base_commit":BASE,"adoption_kind":"explicit engineering decision accepting primary and 8x2 performance uncertainty","adoption_review_passed":true,"resource_qualification_passed":true,"qualified_scope_gates_passed":false,"full_performance_gate_passed":false,"calibration_failures_retained":a["calibration_failures"],"nonregression_bound_failures_retained":a["regressions"],"primary_gain_failures_retained":a["primary_gain_failures"],"all_candidate_nonregression_bounds_passed":false,"decision":r.join("decision.md"),"decision_sha256":hash(r.join("decision.md")),"previous_accepted_manifest":OLD,"previous_accepted_manifest_sha256":hash(OLD),"source_files_sha256":nextfiles,"source_changes":[CHANGED],"new_private_module":ADDED,"source_snapshot":c.join(".xtask/snapshot.json"),"native_build_source_pins":perf.join("source-pins.json"),"native_build_source_pins_sha256":hash(perf.join("source-pins.json")),"native_build_source_snapshot":c.join(".xtask/snapshot.json"),"native_build":perf.join("native/result.json"),"native_cli_build":receiptpath,"native_binary":binary,"native_binary_sha256":SDK,"rustflags":flags,"environment":read(perf.join("perf-plan.json"))["environment"],"performance_protocol":perf.join("perf-plan.json"),"performance_result":perf.join("analysis.json"),"performance_result_sha256":hash(perf.join("analysis.json")),"final_performance_report":perfreport.join("README.md"),"qualification_report":qualreport.join("README.md"),"qualification_result":qual.join("analysis.json"),"qualification_result_sha256":hash(qual.join("analysis.json")),"ms":ms,"m5":m5,"goal_achieved":false,"original8_CPU10x_rows":0,"AWS_comparison_matches_current_source":false,"historical_C7a_source_manifest":OLD,"native_core_tests_inherited":119,"native_raw_tests_inherited":2,"native_cubic_tests_inherited":6,"supporting_studies":[{"root":PERF,"SDK_requests":43410,"independent_exports":465},{"root":QUAL,"SDK_requests":216,"core_requests":4,"independent_exports":85}],"scope":"Bounded serial butterflies within existing expanded FFT leaves; original outer recursion, storage, roots, field kernels and fallback paths retained.","limitations":["Parent full gate remains false: primary gain uncertain; 8x2 candidate/fresh nonregression upper 1.024137 exceeds 1.02.","Individual intervals assume independent/exchangeable blocks and are not simultaneous familywise guarantees.","RSS is mixed; unchanged allocation geometry does not establish universal allocator peaks or minimum RAM.","Small limit cases did not trigger reclaim; 4999 exercised 95GiB hard-limit reclaim without OOM/kill/swap.","No current-source AWS/Aeglos, physical phone, ARM or TEE measurement.","No giant concurrent proof, throughput, arbitrary fairness or 193GiB-key qualification.","All eight supplied CPU10x targets remain unmet."]}).as_object().unwrap() {m.insert(k.clone(),v.clone());}
    save(r.join("production-source-manifest.json"), &manifest);
    save(
        r.join("review.json"),
        &json!({"source_adopted":true,"adoption_review_passed":true,"resource_qualification_passed":true,"full_performance_gate_passed":false,"qualified_scope_gates_passed":false,"all_candidate_nonregression_bounds_passed":false,"primary_gain_uncertainty_explicitly_accepted":a["primary_gain_failures"],"nonregression_bound_uncertainty_explicitly_accepted":a["regressions"],"calibration_failures":a["calibration_failures"],"aggregate_intervals_below_parity":true,"no_observation_excluded_or_repeated":true,"source_identity":read(r.join("source-promotion.json")),"new_independent_exports":0,"prior_proof_count_unchanged":11660,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    for n in [
        "decision.md",
        "promote.rs",
        "source-promotion.json",
        "production-source-manifest.json",
        "review.json",
    ] {
        fs::copy(r.join(n), d.join("artifacts").join(n)).unwrap();
        index.insert(n.to_owned(), hash(d.join("artifacts").join(n)));
    }
    for (source, name) in [
        (receiptpath.as_path(), "native-sdk-receipt.json"),
        (
            c.join(".xtask/snapshot.json").as_path(),
            "source-snapshot.json",
        ),
    ] {
        fs::copy(source, d.join("artifacts").join(name)).unwrap();
        index.insert(name.to_owned(), hash(d.join("artifacts").join(name)));
    }
    save(d.join("artifact-index.json"), &json!(index));
    let mut md=String::from("# Bound parallel work inside expanded FFT leaves\n\nThe accepted CPU source now uses the exact tested bounded FFT leaves. The original-eight paired aggregate improves **0.856% against accepted and 0.419% against fresh**, with intervals below parity. Primary latency is **8.324853 ms**; a reliable primary improvement is unproven. Merge 54 measures **161.341835 ms**, saving a paired 2.142 / 2.572 ms against accepted / fresh.\n\nThis separate engineering decision accepts primary uncertainty and the 8×2 candidate/fresh nonregression interval [0.983854, 1.024137], whose upper bound exceeds the frozen 1.02 requirement. All calibration checks and all other nonregression bounds pass. The [original performance gate](../2026-10-27-clean-fft-leaves/README.md) remains **false**. No observation is excluded, rerun or relabeled. See the [decision](artifacts/decision.md).\n\nOne source file changes and one private module is added. All 172 production source files exactly match the measured candidate. Existing leaf storage, field kernels and outer transform geometry remain; no new table, buffer, solver plan, pool or runtime option is added. Native arithmetic is retained without rebuilding or changing tested bytes.\n\n[Resource qualification](../2026-10-27-clean-fft-qualification/README.md) passes portable, fallback, small-stack, concurrent and memory cases. Forester 4999 completes under 95 GiB with swap disabled at 160.741 / 149.120 seconds and 94.692 GiB peak RSS, with reclaim and no OOM/kill. These are capacity observations, not minimum RAM or matched speed comparisons. RSS across shapes remains mixed.\n\nThe following values reuse the candidate column of the completed fifteen-block local study. M5 CPU and Metal figures are supplied historical references; no 54 reference was supplied.\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required ms for CPU 10×|\n|---|---:|---:|---:|---:|---:|---:|\n");
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
        let t = case["modes"]["candidate"]["ms"].as_f64().unwrap();
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
    write!(md,"\nExact SDK: `{binary}`, SHA-256 `{SDK}`. The [manifest](artifacts/production-source-manifest.json) records adoption/resource review as passed and full/qualified-scope performance gates as false. The latest C7a comparison belongs to the previous 171-file source; no new AWS or Aeglos result is claimed.\n\nPromotion adds no benchmark, build or proof export. The proof ledger stays **11,660**. Physical phones, ARM, TEE and universal/minimum memory remain unqualified. The original all-eight CPU 10× goal remains **active and unmet, 0/8**.\n\n[Source audit](artifacts/source-promotion.json), [review](artifacts/review.json), [artifact hashes](artifact-index.json).\n").unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    println!("Promoted exact 172-file source; original full gate remains false; proof ledger 11660 unchanged");
}
