#![recursion_limit = "256"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption";
const F: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-production";
const C: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-compatibility175";
const OLD:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json";
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
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn main() {
    assert_eq!(std::env::args().nth(1).as_deref(), Some("promote"));
    let r = Path::new(R);
    let d = Path::new(D);
    let z = Path::new(Z);
    let f = Path::new(F);
    assert!(!d.exists());
    let base = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(base.status.success());
    let base = String::from_utf8(base.stdout).unwrap().trim().to_string();
    assert_eq!(base, "b1e242ba6fe6d57e700dd4516bc01221593d773c");
    let a = read(r.join("analysis.json"));
    assert_eq!(a, read(f.join("artifacts/analysis.json")));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["failed_checks"], json!(["1x2-w16/OFF/accepted"]));
    assert_eq!(a["final_artifact_candidate_gate_passed"], true);
    assert_eq!(a["previous_full_performance_gate_passed"], false);
    assert_eq!(a["independent_proof_count"], 17005);
    assert_eq!(a["SDK_requests"], 28000);
    assert_eq!(a["new_independent_exports"], 505);
    assert_eq!(
        a["checks"]
            .as_object()
            .unwrap()
            .values()
            .filter(|v| **v == true)
            .count(),
        29
    );
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    for (p, h) in read(r.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let previous = read(OLD);
    let source = previous["source_files_sha256"].as_object().unwrap();
    assert_eq!(source.len(), 174);
    for (p, h) in source {
        assert_eq!(
            hash(z.join(p)),
            h.as_str().unwrap(),
            "existing production changed: {p}"
        );
    }
    let audit = read(r.join("source-audit.json"));
    assert_eq!(audit["reverse174_exact"], true);
    assert_eq!(audit["originals_exact"], 173);
    assert_eq!(
        audit["clean_child_exact_to_formatted_measured_without_trace"],
        true
    );
    let parent = audit["parent"].as_str().unwrap();
    let child = audit["child"].as_str().unwrap();
    assert!(!z.join(child).exists());
    let flags = fs::read_to_string(r.join("native-flags.txt")).unwrap();
    assert_eq!(flags, previous["rustflags"].as_str().unwrap());
    assert!(!flags.contains("zopro_balanced_kh"));
    let ids = read(r.join("identities.json"));
    let bin = ids["ON"]["binary"].as_str().unwrap();
    let bin_hash = hash(bin);
    assert_eq!(bin_hash, ids["ON"]["sha256"].as_str().unwrap());
    assert_eq!(
        bin_hash,
        "20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f"
    );
    for (label, count) in [("candidate", 124), ("portable", 101)] {
        assert_eq!(
            read(r.join(format!("native/{label}-core/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            count
        );
        assert_eq!(
            read(r.join(format!("native/{label}-prover/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
    }
    let compat = read(Path::new(C).join("artifacts/analysis.json"));
    assert_eq!(compat["qualified_additional_scope"], true);
    assert_eq!(compat["new_independent_exports"], 89);
    let mut sources = BTreeMap::new();
    let mut source_paths: Vec<_> = source.keys().cloned().collect();
    source_paths.push(child.into());
    for p in source_paths {
        let compiled = r.join("source").join(&p);
        let archived = f.join("artifacts/source").join(&p);
        let h = hash(&compiled);
        assert_eq!(h, hash(archived));
        sources.insert(p, h);
    }
    assert_eq!(sources.len(), 175);
    let mut m5 = BTreeMap::new();
    for (k, v) in a["new_reference_comparisons"].as_object().unwrap() {
        let shape = k.strip_suffix("-w32").unwrap();
        let on = &v["variants"]["ON"];
        let cpu = v["reference"]["cpu_ms"].as_f64().unwrap();
        let ms = on["ms"].as_f64().unwrap();
        m5.insert(shape,json!({"accepted_ms":ms,"cpu_reference_ms":cpu,"cpu_speedup":on["CPU_speedup"],"metal_reference_ms":v["reference"]["metal_ms"],"metal_speedup":on["Metal_speedup"],"required_ms_for_CPU10x":cpu/10.,"goal10x":ms<=cpu/10.}));
    }
    assert_eq!(m5.len(), 8);
    assert_eq!(m5.values().filter(|v| v["goal10x"] == true).count(), 0);
    let mut ms = BTreeMap::new();
    for c in read(r.join("cases.json")).as_array().unwrap() {
        let k = c["name"].as_str().unwrap();
        ms.insert(
            k.to_string(),
            a["cells"][format!("{k}/ON")]["median_ms"].clone(),
        );
    }
    let decision = json!({"decision":"adopt clean balanced K/H under an explicit engineering exception","adopted":true,"production_changed":true,"review_passed":true,"review_base_commit":base,"performance_report":format!("{F}/README.md"),"performance_commit":"b1e242ba6fe6d57e700dd4516bc01221593d773c","full_performance_gate_passed":false,"failed_checks_retained":a["failed_checks"],"final_artifact_candidate_gate_passed":true,"previous_full_performance_gate_passed":false,"previous_failed_checks_retained":["1x2-w16/OFF/accepted"],"reason":"Primary32 paired gain survives the private pilot, full qualification and actual clean artifact; final artifact wins15/15against both controls, all candidate nonregression and all32worker calibration checks pass. Physical16 baseline uncertainty is explicitly accepted, not relabeled as passed.","primary32_ON_accepted":a["pairs"]["1x2-w32/ON/accepted"],"primary32_ON_OFF":a["pairs"]["1x2-w32/ON/OFF"],"physical16_ON_accepted":a["pairs"]["1x2-w16/ON/accepted"],"physical16_OFF_accepted":a["pairs"]["1x2-w16/OFF/accepted"],"merge54_ON_accepted":a["pairs"]["merge-54x1-w32/ON/accepted"],"native_binary":bin,"native_binary_sha256":bin_hash,"source_files":175,"source_originals_exact":173,"reverse174_exact":true,"default_guard_change_tested":true,"native_core_prover_tests":[124,8],"portable_core_prover_tests":[101,8],"field_tests_inherited_native_portable":[33,11],"portable_real_SDK_preflights":13,"SDK_requests_in_qualification":28000,"independent_exports_in_qualification":505,"proof_ledger":17005,"new_requests_or_exports_during_promotion":0,"resource_compatibility_scope":"Inherited source-equivalent private candidate;8configured192MiB/noSwap cases with high0/max0/OOM0. No new clean-binary pressure/concurrency capture. Earlier forced-high baseline timeout and unexecuted candidate remain historical limitations.","no_new_heap_key_table_pool_plan_or_policy":true,"extra_span_metadata_bytes_64bit":320,"extra_compiler_frames_unmeasured":true,"no_minimum_RAM_claim":true,"historical_limitations_retained":previous["limitations"],"limitations":["physical16 gain versus accepted not established; interval crosses parity","merge54 paired+0.912% estimate retained; interval crosses parity and upper1.013465 fits declared1.03 nonregression","RSS observations mixed; no general reduction","no fresh clean-binary active-throttle, giant250/4999,phone,ARM,TEE,swap,minRAM orAWS/Aeglos qualification"],"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0});
    fs::create_dir_all(d.join("artifacts")).unwrap();
    save(d.join("artifacts/decision.json"), &decision);
    fs::copy(r.join("adopt.rs"), d.join("artifacts/adopt.rs")).unwrap();
    let qualification = f.join("artifacts/analysis.json");
    let resource = Path::new(C).join("artifacts/analysis.json");
    let decision_path = d.join("artifacts/decision.json");
    let manifest = json!({"adopted":true,"adoption_kind":"engineering exception to physical16 identical-baseline calibration","adoption_review_passed":true,"decision":decision_path,"decision_sha256":hash(&decision_path),"review_base_commit":base,"previous_accepted_manifest":OLD,"previous_accepted_manifest_sha256":hash(OLD),"previous_accepted_limitations_retained":previous["limitations"],"historical_performance_results_retained":[{"manifest":OLD,"full_performance_gate_passed":previous["full_performance_gate_passed"],"performance_failures":previous["performance_failures_retained"],"historical_failures":previous["historical_performance_failures_retained"]},{"report":"/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-balanced-kh-qualification175/README.md","full_performance_gate_passed":false,"failed_checks":["1x2-w16/OFF/accepted"]}],"source_files_sha256":sources,"source_changes":[parent,child],"source_snapshot":f.join("artifacts/source"),"native_build_source_snapshot":f.join("artifacts/source"),"native_build_source_pins":d.join("artifacts/production-input-pins.json"),"native_original_build_source_pins":f.join("artifacts/build-pins.json"),"native_build":f.join("artifacts/native/result.json"),"native_binary":bin,"native_binary_sha256":bin_hash,"native_binary_interface":"cpu-sdk IMAGE REQUEST_JSON ROUNDS [PROOF_DIRECTORY]","native_cli_build":null,"native_cli_build_limitation":"separate CLI not rebuilt; actual standard SDK libraries/binary qualified","rustflags":flags,"environment":previous["environment"],"native_core_tests":124,"native_raw_tests":2,"native_cubic_tests":6,"native_field_tests_inherited":33,"portable_core_tests_new":101,"portable_prover_tests_new":8,"portable_field_tests_inherited":11,"portable_SDK_preflights_new":13,"portable_core_tests_inherited":0,"portable_prover_tests_inherited":0,"performance_protocol":f.join("artifacts/final-protocol.md"),"performance_result":qualification,"performance_result_sha256":hash(&qualification),"final_performance_report":format!("{F}/README.md"),"final_artifact_gate_passed":true,"full_performance_gate_passed":false,"qualified_scope_gates_passed":false,"performance_failures_retained":a["failed_checks"],"resource_qualification_passed":true,"resource_qualification_result":resource,"resource_qualification_result_sha256":hash(&resource),"resource_qualification_scope":"inherited source-equivalent private175;configured primary cold/warm192MiB/noSwap high0. No fresh clean-binary active-throttle/concurrent/giant/phone proof. Historical forced-high baseline timeout/candidateunexecuted retained.","qualification_result":qualification,"qualification_result_sha256":hash(&qualification),"qualification_scope":"actual native124/8 portable101/8 plus13portable realproofs and all9native shapes/primary16 latency/profile qualification; inherited private-candidate opt-out/small-stack/concurrent/configured-memory coverage","ms":ms,"m5":m5,"AWS_comparison_matches_current_source":false,"proof_ledger":17005,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"limitations":decision["limitations"]});
    save(
        d.join("artifacts/production-source-manifest.json"),
        &manifest,
    );
    let mut text=String::from("# Balanced K/H production adoption\n\nAdopt the tested variable-width K/H windows under the existing native entry guards. Primary at 32 workers measures **8.022165 ms versus 8.127910 ms**, paired **1.131% faster** with 15/15 wins; process peak RSS is **134.00 versus 135.00 MiB**. This is an incremental gain, with no general RAM reduction claimed.\n\nAdoption is an **explicit engineering exception**: the full performance gate remains **false** solely for physical16 OFF/accepted calibration. All candidate gain/nonregression checks and all32-worker calibration checks pass. The separately declared final-artifact gate is true. Physical16 improvement versus accepted is uncertain; its interval crosses parity. The previous private-candidate and earlier production exceptions remain recorded.\n\nOnly the borrowed K/H parent and one new private module change. Twenty 12/13-bit signed windows retain the complete bucket algorithm and conservative memory forecast. The private experiment flag and trace are absent. Of 174 original files, 173 are byte-exact; reversing the parent edits and removing the child recovers all174. The final175file source matches the actual tested artifact, with unchanged accepted native flags. No retained key table, new heap buffer, pool or execution plan is added; stack span metadata is320bytes on64bit, with compiler frames additional.\n\n## Validation and limits\n\nFresh actual-artifact native124core/8prover and portable101core/8prover tests pass, including both balanced scalar/point oracles. Field33native/11portable evidence is inherited from identical source. Thirteen fresh portable real-proof cases cover all9basis shapes and primary/54raw/denied. All **28,000 SDK requests** and **505 fresh Gnark exports** pass; all source/library/proof pins close. The independent ledger is **17,005**. Promotion adds no new proof/test capture; all175 copied source hashes must match the qualified snapshot.\n\nPrivate-candidate small-stack/opt-out/concurrent/configured-memory checks are inherited by source equivalence; they are not new clean-binary captures. All eight192MiB cases had zero high/max/OOM events, so active throttling was not established. The historical forced-high baseline timeout and unexecuted candidate remain limitations. There is no fresh giant250/4999,phone,ARM,TEE,swap,minRAM orAWS/Aeglos qualification. Separate CLI build is unverified; the actual standard SDK engine is qualified.\n\n54→1 measures162.250464ms versus161.689371ms accepted; its paired+0.912% estimate and interval crossing parity are retained, with upper1.013465 inside the declared1.03 bound. RSS observations are mixed across shapes.\n\n## Supplied M5 comparisons\n\nThese are actual final-artifact32-worker measurements and supplied historical M5 references.\n\n| Shape | Adopted ms | M5 CPU ms | CPU/adopted | M5 Metal ms | Metal/adopted | CPU10× target ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (shape, v) in &m5 {
        writeln!(
            text,
            "| {shape} | {:.6} | {:.0} | {:.3}× | {:.0} | {:.3}× | {:.1} |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap(),
            v["required_ms_for_CPU10x"].as_f64().unwrap()
        )
        .unwrap();
    }
    text.push_str("\nThe original all-eight CPU10× goal remains **active and unmet,0/8**. Primary requires3.7ms. No M5 reference was supplied for54→1. [Full experiment](../2026-10-29-balanced-kh-production/README.md), [decision](artifacts/decision.json), [production source manifest](artifacts/production-source-manifest.json), [inherited compatibility](../2026-10-29-balanced-kh-compatibility175/README.md).\n");
    for (a, b) in [
        ("all174", "all 174"),
        ("final175file", "final 175-file"),
        (
            "metadata is320bytes on64bit",
            "metadata is 320 bytes on 64-bit",
        ),
        ("native124core/8prover", "native 124 core/8 prover"),
        ("portable101core/8prover", "portable 101 core/8 prover"),
        ("Field33native/11portable", "Field 33 native/11 portable"),
        ("all9basis", "all nine basis"),
        ("primary/54raw/denied", "primary/54 raw/denied"),
        ("all175", "all 175"),
        ("eight192MiB", "eight 192 MiB"),
        (
            "giant250/4999,phone,ARM,TEE,swap,minRAM orAWS/Aeglos",
            "giant 250/4999, phone, ARM, TEE, swap, minimum-RAM or AWS/Aeglos",
        ),
        (
            "measures162.250464ms versus161.689371ms",
            "measures 162.250464 ms versus 161.689371 ms",
        ),
        ("paired+0.912%", "paired +0.912%"),
        ("upper1.013465", "upper 1.013465"),
        ("declared1.03", "declared 1.03"),
        ("final-artifact32-worker", "final-artifact 32-worker"),
        ("CPU10×", "CPU 10×"),
        ("unmet,0/8", "unmet, 0/8"),
        ("requires3.7ms", "requires 3.7 ms"),
        ("for54→1", "for 54→1"),
        ("physical16", "physical 16"),
        ("all32-worker", "all 32-worker"),
    ] {
        text = text.replace(a, b);
    }
    fs::write(d.join("README.md"), text).unwrap();
    // Apply exactly the two reviewed source files only after all evidence is checked.
    fs::create_dir_all(z.join(child).parent().unwrap()).unwrap();
    fs::copy(r.join("source").join(child), z.join(child)).unwrap();
    fs::copy(r.join("source").join(parent), z.join(parent)).unwrap();
    for (p, h) in &sources {
        assert_eq!(hash(z.join(p)), *h, "promoted source mismatch: {p}");
    }
    let mut production_pins = BTreeMap::new();
    for (file, h) in &sources {
        production_pins.insert(z.join(file).display().to_string(), h.clone());
        production_pins.insert(
            f.join("artifacts/source").join(file).display().to_string(),
            h.clone(),
        );
    }
    production_pins.insert(bin.to_string(), bin_hash.clone());
    for (file, h) in read(r.join("frozen-candidate.json"))["input_pins"]
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(file), h.as_str().unwrap());
        production_pins.insert(file.clone(), h.as_str().unwrap().to_string());
    }
    for file in [
        r.join("native/candidate/artifacts.json"),
        r.join("frozen-candidate.json"),
        qualification.clone(),
        resource.clone(),
        decision_path.clone(),
    ] {
        production_pins.insert(file.display().to_string(), hash(&file));
    }
    for (file, h) in &production_pins {
        assert_eq!(hash(file), *h);
    }
    save(
        d.join("artifacts/production-input-pins.json"),
        &json!(production_pins),
    );
    save(
        d.join("artifacts/promotion-validation.json"),
        &json!({"all_source_hashes_match":true,"source_files_checked":175,"native_binary_sha256":bin_hash,"production_source_changed":true,"new_tests":0,"new_SDK_requests":0,"new_independent_exports":0,"proof_ledger":17005,"full_performance_gate_passed":false,"final_artifact_gate_passed":true,"goal_achieved":false}),
    );
    let mut files = Vec::new();
    walk(d, &mut files);
    let mut pins = BTreeMap::new();
    for p in files {
        pins.insert(p.strip_prefix(d).unwrap().display().to_string(), hash(p));
    }
    save(d.join("artifact-index.json"), &json!(pins));
    pins.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        pins.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Promoted175exact source files;two source changes;20fde SDK;full gate false retained;ledger17005 unchanged;goal active0/8");
}
