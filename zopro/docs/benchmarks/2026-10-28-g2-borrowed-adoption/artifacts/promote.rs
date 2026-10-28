#![recursion_limit = "256"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write as _, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-adoption";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const F: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
const OLD:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption";
const BASE: &str = "f3b354828e253e8b332940661876c7fcc4e394df";
const SDK: &str = "81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20";
const CHANGE: [&str; 3] = [
    "crates/zopro-core/src/msm.rs",
    "crates/zopro-core/src/msm_affine.rs",
    "crates/zopro-core/src/msm_affine/g2_phi.rs",
];
const ADD: [&str; 2] = [
    "crates/zopro-core/src/msm/g2_borrowed.rs",
    "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
];
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
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn n(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn archived(root: &Path, report: &Path, f: &str) {
    let index = read(report.join("artifact-index.json"));
    let key = format!("artifacts/{f}");
    let h = index[&key].as_str().unwrap();
    assert_eq!(hash(root.join(f)), h);
    assert_eq!(hash(report.join(key)), h);
}
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    let f = Path::new(F);
    let q = Path::new(Q);
    let d = Path::new(D);
    assert!(!d.exists());
    let report = p.join("docs/benchmarks/2026-10-28-g2-borrowed-production");
    let head = Command::new("git")
        .args(["-C", P, "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(head.status.success());
    assert_eq!(String::from_utf8(head.stdout).unwrap().trim(), BASE);
    for file in [
        "analysis.json",
        "interpretation.md",
        "source-audit.json",
        "pins.json",
        "proof-inventory.json",
        "native/candidate/artifacts.json",
        "native/candidate-core/tests.json",
        "native/candidate-prover/tests.json",
        "runs/result.json",
        "final-protocol.md",
    ] {
        archived(f, &report, file);
    }
    let a = read(f.join("analysis.json"));
    assert_eq!(a["final_artifact_gate_passed"], false);
    assert_eq!(a["failed_checks"], json!(["16x2-w32/candidate/accepted"]));
    assert_eq!(a["new_independent_exports"], 477);
    assert_eq!(a["independent_proof_count"], 15028);
    assert_eq!(a["original8_CPU10x_rows"], 0);
    for v in ["accepted", "control"] {
        let x = &a["pairs"][format!("1x2-w32/candidate/{v}")];
        assert!(n(x, "high") < 1.);
        assert_eq!(x["wins"], 15);
    }
    let exception = &a["pairs"]["16x2-w32/candidate/accepted"];
    assert_eq!(n(exception, "high"), 1.025268463655265);
    assert!(n(exception, "median") < 1.001);
    assert_eq!(read(f.join("native/result.json"))["status"], "complete");
    let audit = read(f.join("source-audit.json"));
    assert_eq!(audit["reverse172_exact"], true);
    assert_eq!(audit["originals_exact"], 169);
    let qual = read(q.join("analysis.json"));
    assert_eq!(qual["qualified_additional_scope"], true);
    archived(
        q,
        &p.join("docs/benchmarks/2026-10-28-g2-borrowed-qualification"),
        "analysis.json",
    );
    for (file, h) in read(f.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(file), h.as_str().unwrap(), "{file}");
    }
    let old = read(OLD);
    let originals = old["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 172);
    let mut sources = BTreeMap::new();
    let mut snapshot = BTreeMap::new();
    let mut changed = vec![];
    for (file, h) in originals {
        assert_eq!(hash(p.join(file)), h.as_str().unwrap());
        let from = f.join("source").join(file);
        let next = hash(&from);
        if next != h.as_str().unwrap() {
            assert!(CHANGE.contains(&file.as_str()));
            changed.push(json!({"file":file,"old_sha256":h,"new_sha256":next}));
        }
        sources.insert(file.clone(), next.clone());
        snapshot.insert(from.display().to_string(), next);
    }
    assert_eq!(changed.len(), 3);
    for file in ADD {
        assert!(!p.join(file).exists());
        let from = f.join("source").join(file);
        let h = hash(&from);
        assert_eq!(h, hash(q.join("source").join(file)));
        sources.insert(file.into(), h.clone());
        snapshot.insert(from.display().to_string(), h);
    }
    assert_eq!(sources.len(), 174);
    let receipt = read(f.join("native/candidate/artifacts.json"));
    assert_eq!(receipt["binary_sha256"], SDK);
    assert_eq!(hash(receipt["binary"].as_str().unwrap()), SDK);
    for (file, h) in receipt["input_pins"].as_object().unwrap() {
        assert_eq!(hash(file), h.as_str().unwrap());
    }
    assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
    let flags = fs::read_to_string(f.join("native-flags.txt")).unwrap();
    assert_eq!(flags, old["rustflags"].as_str().unwrap());
    assert!(!flags.contains("zopro_g2_borrow_queue") && !flags.contains("zopro_g2_queue_probe"));
    let decision = fs::read_to_string(r.join("decision.md")).unwrap();
    assert!(decision.contains(SDK));
    // All evidence/source checks precede the five-file production mutation.
    for file in CHANGE.into_iter().chain(ADD) {
        let to = p.join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(f.join("source").join(file), to).unwrap();
    }
    for (file, h) in &sources {
        assert_eq!(hash(p.join(file)), *h);
        assert_eq!(hash(f.join("source").join(file)), *h);
    }
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let promotion = json!({"previous_source_files":172,"new_source_files":174,"originals_exact":169,"changed_originals":changed,"new_modules":ADD,"all174production_files_exactly_match_measured_candidate":true,"no_formatting_or_rebuild_after_capture":true,"new_proofs_or_benchmarks":0,"proof_ledger":15028,"native_coherent_libraries_rebuilt_before_capture":true,"field_source_unchanged":true});
    save(r.join("source-promotion.json"), &promotion);
    save(
        r.join("source-snapshot.json"),
        &json!({"version":1,"source_root":f.join("source"),"source_pins":snapshot}),
    );
    fs::copy(
        f.join("native/candidate/artifacts.json"),
        r.join("native-sdk-receipt.json"),
    )
    .unwrap();
    let mut ms = BTreeMap::new();
    let mut m5 = BTreeMap::new();
    for c in read(f.join("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        let x = &a["cells"][format!("{case}/candidate")];
        let t = n(x, "median_ms");
        ms.insert(case.to_string(), t);
        if c["workers"] == 32 && !c["reference"].is_null() {
            let cpu = n(&c["reference"], "cpu_ms");
            let metal = n(&c["reference"], "metal_ms");
            m5.insert(c["shape"].as_str().unwrap().to_string(),json!({"accepted_ms":t,"cpu_reference_ms":cpu,"metal_reference_ms":metal,"cpu_speedup":cpu/t,"metal_speedup":metal/t,"required_ms_for_CPU10x":cpu/10.,"goal10x":t<=cpu/10.}));
        }
    }
    let env = read(f.join("run-plan.json"))["steps"][0]["environment"].clone();
    let review = json!({"adoption_review_passed":true,"source_adopted":true,"final_artifact_gate_passed":false,"old_full_performance_gate_passed":false,"final_failed_checks_retained":a["failed_checks"],"old_failed_checks_retained":a["old_failed_checks"],"explicit_engineering_exceptions":["16x2 default32 upper1.025268 despite paired median1.0000168","physical16 behavior unresolved; no speed improvement claimed","forced-high accepted baseline timeout; candidate condition untested"],"all_default32_calibration_checks_passed":true,"new_native_core_tests":122,"new_native_raw_tests":2,"new_native_cubic_tests":6,"portable_tests_inherited":{"field":11,"core":99,"prover":8},"new_adoption_proofs_or_benchmarks":0,"proof_ledger_unchanged":15028,"source_identity":promotion,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0});
    save(r.join("review.json"), &review);
    let manifest = json!({"adopted":true,"adoption_kind":"explicit engineering exception accepting16x2 bound and physical16/resource uncertainty","adoption_review_passed":true,"review_base_commit":BASE,"previous_accepted_manifest":OLD,"previous_accepted_manifest_sha256":hash(OLD),"source_files_sha256":sources,"source_snapshot":format!("{D}/artifacts/source-snapshot.json"),"native_build_source_snapshot":format!("{D}/artifacts/source-snapshot.json"),"rustflags":flags,"native_binary":receipt["binary"],"native_binary_sha256":SDK,"native_binary_interface":"cpu-sdk IMAGE REQUEST_JSON ROUNDS [PROOF_DIRECTORY]","native_build":format!("{F}/native/result.json"),"native_cli_build":format!("{D}/artifacts/native-sdk-receipt.json"),"native_build_source_pins":format!("{F}/build-pins.json"),"performance_result":format!("{F}/analysis.json"),"performance_result_sha256":hash(f.join("analysis.json")),"final_performance_report":report,"performance_protocol":format!("{F}/final-protocol.md"),"qualification_result":format!("{Q}/analysis.json"),"qualification_result_sha256":hash(q.join("analysis.json")),"qualification_scope":"inherited exact-module/excluded-path evidence; native clean integration newly tested","environment":env,"ms":ms,"m5":m5,"source_changes":promotion,"decision":format!("{D}/artifacts/decision.md"),"decision_sha256":hash(r.join("decision.md")),"full_performance_gate_passed":false,"final_artifact_gate_passed":false,"qualified_scope_gates_passed":false,"resource_qualification_passed":true,"resource_qualification_scope":"finite bounded primary scenarios only; actualhigh0, forcedhighbaseline timeout/candidateunexecuted","performance_failures_retained":a["failed_checks"],"historical_performance_failures_retained":a["old_failed_checks"],"native_core_tests":122,"native_raw_tests":2,"native_cubic_tests":6,"portable_field_tests_inherited":11,"portable_core_tests_inherited":99,"portable_prover_tests_inherited":8,"proof_ledger":15028,"AWS_comparison_matches_current_source":false,"limitations":["physical16 uncertain","16x2 failed nonregression bound accepted explicitly","RSS mixed; requested traffic is not RSS","no new giant,phone,ARM,TEE,swap,minRAM,AWS or universalpressure qualification","previous FFT adoption performance exceptions remain historical"],"goal_achieved":false,"goal_status":"active","original8_CPU10x_rows":0});
    save(r.join("production-source-manifest.json"), &manifest);
    let mut out=String::from("# Borrowed G2 pending operands\n\nThe accepted CPU source now uses proof-local borrowed G2 pending records. The exact clean SDK measures **8.366931ms** for1×2 versus8.563320ms accepted: paired **2.137% lower latency**,15/15wins; versus rebuiltcontrol,1.691% lower,15/15wins. Field source, key formats and conservative forecasts remain. No persistent table, extra plan or pool is added.\n\nAdoption explicitly accepts a failed16×2 nonregression upper bound1.025268 (paired median1.0000168), uncertain physical16 behavior, and the accepted-only forced-high timeout. The final and historical performance gates remain **false**. See [decision](artifacts/decision.md), [review](artifacts/review.json), and [complete final-artifact study](../2026-10-28-g2-borrowed-production/README.md). No observation is excluded or repeated.\n\nPrimary RSS median/max is131.25/135.00MiB versus130.25/132.75MiB accepted. Peaks are mixed; earlier1.4MiB lower requested allocation traffic is not an RSS guarantee. Merge54 is164.388184ms, candidate median/max RSS1802.20/1885.21MiB. No supplied54reference. No new AWS/Aeglos or giant/mobile measurement.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | Required ms for CPU10× |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (shape, v) in &m5 {
        writeln!(
            out,
            "| {shape} | {:.6} | {} | {:.3}× | {} | {:.3}× | {} |",
            n(v, "accepted_ms"),
            v["cpu_reference_ms"],
            n(v, "cpu_speedup"),
            v["metal_reference_ms"],
            n(v, "metal_speedup"),
            v["required_ms_for_CPU10x"]
        )
        .unwrap();
    }
    writeln!(out,"\nThe M5 figures are supplied historical references; local Ryzen9950X warm single-proof latency includes solving, randomized proving, SDK verification and serialization. The original all-eight10×CPU goal remains **active and unmet,0/8**.\n\nAll174 production files exactly match the measured source. New clean-build tests pass122core+2raw+6cubic;477fresh exports passed independent Gnark verification and26208SDK requests verified internally. Adoption adds no benchmark or proof; ledger stays15028. Portable/fallback/concurrency/resource evidence is inherited within its recorded scope. [Accepted manifest](artifacts/production-source-manifest.json).\n\nSDK:`{}`; SHA-256`{SDK}`.\n",receipt["binary"].as_str().unwrap()).unwrap();
    fs::write(d.join("README.md"), out).unwrap();
    let mut index = BTreeMap::new();
    for file in [
        "decision.md",
        "promote.rs",
        "review.json",
        "source-promotion.json",
        "source-snapshot.json",
        "native-sdk-receipt.json",
        "production-source-manifest.json",
    ] {
        fs::copy(r.join(file), d.join("artifacts").join(file)).unwrap();
        assert_eq!(hash(r.join(file)), hash(d.join("artifacts").join(file)));
        index.insert(format!("artifacts/{file}"), hash(r.join(file)));
    }
    index.insert("README.md".into(), hash(d.join("README.md")));
    save(d.join("artifact-index.json"), &json!(index));
    index.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Adopted exact174sourcefiles,3changed+2added; finalgateFALSE, reviewedexceptions;0newproofs;goalACTIVE0/8");
}
