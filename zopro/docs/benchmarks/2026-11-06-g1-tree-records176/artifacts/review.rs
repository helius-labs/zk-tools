use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-records176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("review.json").exists());
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let audit = read(format!("{R}/source-audit.json"));
    let mut exact = 0;
    for (p, h) in audit["accepted_source_hashes"].as_object().unwrap() {
        let prod = format!("{Z}/{p}");
        assert_eq!(hash(&prod), h.as_str().unwrap());
        let candidate = format!("{R}/source/{p}");
        let bytes = fs::read(&candidate).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            audit["candidate_source_hashes"][p]
        );
        if p == "crates/zopro-core/src/msm.rs" {
            let mut text = String::from_utf8(bytes).unwrap();
            let suffix = audit["msm_appendix"].as_str().unwrap();
            assert!(text.ends_with(suffix));
            text.truncate(text.len() - suffix.len());
            for pair in audit["msm_edits"].as_array().unwrap().iter().rev() {
                let old = pair[0].as_str().unwrap();
                let new = pair[1].as_str().unwrap();
                assert_eq!(text.matches(new).count(), 1);
                text = text.replace(new, old);
            }
            assert_eq!(text.as_bytes(), fs::read(&prod).unwrap());
        } else if p == "crates/zopro-core/src/lib.rs" {
            let suffix = audit["lib_appendix"].as_str().unwrap().as_bytes();
            assert!(bytes.ends_with(suffix));
            assert_eq!(
                &bytes[..bytes.len() - suffix.len()],
                fs::read(prod).unwrap()
            );
        } else {
            assert_eq!(bytes, fs::read(prod).unwrap());
            exact += 1;
        }
    }
    assert_eq!(exact, 174);
    assert_eq!(
        audit["candidate_source_hashes"].as_object().unwrap().len(),
        177
    );
    let msm = fs::read_to_string(format!("{R}/source/crates/zopro-core/src/msm.rs")).unwrap();
    let original = fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap();
    for (begin, end) in [
        ("fn weighted_sum<", "// In Fq2 ="),
        ("fn invert_lanes(", "/// Reuses the lane-form"),
        ("impl BatchInverse for Fq2 {", "#[cfg(test)]\nmod tests"),
        (
            "pub(crate) fn workspace_batch<",
            "// Full batches need no scalar-tail",
        ),
    ] {
        let a = &msm[msm.find(begin).unwrap()..];
        let b = &original[original.find(begin).unwrap()..];
        assert_eq!(
            &a[..a.find(end).unwrap()],
            &b[..b.find(end).unwrap()],
            "body{begin}"
        );
    }
    let old=fs::read_to_string(format!("{Z}/docs/benchmarks/2026-10-25-prefix-records-component/source/crates/zopro-core/src/msm/prefix_records.rs")).unwrap();
    assert!(old.contains("pub(super) fn g1<"));
    assert!(!old.contains("reduce_layer"));
    assert!(!old.contains("TREE_PREFIX"));
    let oldtree=fs::read_to_string(format!("{Z}/docs/benchmarks/2026-10-27-g2-tree-records/artifacts/candidate/crates/zopro-core/src/msm.rs")).unwrap();
    let fq = oldtree
        .split_once("impl BatchInverse for Fq {")
        .unwrap()
        .1
        .split_once("impl BatchInverse for Fq2")
        .unwrap()
        .0;
    assert!(!fq.contains("TREE_PREFIX"));
    let libs = read(format!("{R}/libraries.json"));
    let mut outputs = BTreeMap::new();
    let plan = read(format!("{R}/plan.json"));
    let mut receipts = Vec::new();
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let base = format!("{R}/runs/{n}");
        let command = read(format!("{base}/command.json"));
        let exit = read(format!("{base}/exit.json"));
        assert_eq!(
            &command["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap().as_slice()
        );
        assert_eq!(exit["returncode"], 0, "{n}");
        assert_eq!(exit["success"], true);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{base}/{stream}.log")),
                exit[format!("{stream}_sha256")]
            );
        }
        if n.starts_with("build-") {
            let a: Vec<_> = s["argv"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap())
                .collect();
            for flag in ["-O", "codegen-units=1"] {
                assert!(a.contains(&flag));
            }
            assert!(a.contains(&if n.ends_with("native") {
                "target-cpu=native"
            } else {
                "target-cpu=x86-64"
            }));
            if !n.starts_with("build-core-") {
                assert!(a.contains(&"lto=fat"));
            } else if n.ends_with("portable") {
                assert!(a.contains(&"no_ifma"));
            }
        }
        receipts.push(json!({"name":n,"returncode":0,"elapsed_ms":exit["elapsed_ms"]}));
    }
    assert_eq!(receipts.len(), 10);
    let mut summaries = BTreeMap::new();
    let mut capacities = BTreeMap::new();
    for mode in ["native", "portable"] {
        let tests = read(format!("{R}/runs/tests-{mode}/tests.json"));
        assert_eq!(
            tests["passed_tests"].as_array().unwrap().len(),
            if mode == "native" { 129 } else { 4 }
        );
        assert_eq!(tests["unfiltered"], mode == "native");
        let stderr = fs::read_to_string(format!("{R}/runs/tests-{mode}/stderr.log")).unwrap();
        assert!(stderr.contains(&format!(
            "exact_pairs_tails_cases=780 prefix_enabled={} lanes={}",
            mode == "native",
            if mode == "native" { 8 } else { 4 }
        )));
        assert!(stderr.contains("weighted_identity_cases=120 existing_forecasts_unchanged"));
        let mut last = None;
        for workers in [1, 4, 16, 32] {
            assert!(stderr.contains(&format!("G1_TREE rayon_workers={workers} stack_bytes=524288 public_group+indexed+weighted_pass")));
            let prefix =
                format!("G1_TREE std_workers={workers} stack_bytes=524288 scratch_capacity=");
            let line = stderr.lines().find(|l| l.starts_with(&prefix)).unwrap();
            let v: Vec<u64> = serde_json::from_str(line.strip_prefix(&prefix).unwrap()).unwrap();
            assert_eq!(v.len(), 4);
            if mode == "native" {
                assert_eq!(v[0], 0);
                assert!(v[1] > 0);
            } else {
                assert_eq!(v[1], 0);
                assert!(v[0] > 0);
            }
            if let Some(last) = &last {
                assert_eq!(&v, last);
            } else {
                last = Some(v);
            }
        }
        capacities.insert(mode, last.unwrap());
        let stdout = fs::read_to_string(format!("{R}/runs/tests-{mode}/stdout.log")).unwrap();
        let summary = stdout
            .lines()
            .find(|l| l.starts_with("test result:"))
            .unwrap();
        summaries.insert(mode, summary.to_owned());
        let oracle = read(format!("{R}/runs/oracle-{mode}/tests.json"));
        assert_eq!(oracle["passed_tests"].as_array().unwrap().len(), 2);
        let stdout = fs::read_to_string(format!("{R}/runs/oracle-{mode}/stdout.log")).unwrap();
        assert!(stdout.contains("grouped cases=36 full254bit/boundary/mixed/fresh+reuse"));
        assert!(stdout.contains("weighted cases=22 full254bit/multiple+partial chunks"));
        for p in [
            format!("libs/{mode}/libzopro_core.rlib"),
            format!("core-tests-{mode}"),
            format!("oracle-tests-{mode}"),
        ] {
            outputs.insert(p.clone(), hash(format!("{R}/{p}")));
        }
        let field = libs[mode]["field_and_support_reused"]["zopro_bn254"]
            .as_str()
            .unwrap();
        if mode == "native" {
            assert!(
                field.contains("1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713")
            );
        } else {
            assert!(
                field.contains("d5a6c7707f2fc9576fe0f26568272dc180c174f615fc91693d4da7d8d3bdce23")
            );
        }
    }
    let portable_filtered = summaries["portable"].split(';').find(|s| s.contains("filtered out")).unwrap().split_whitespace().next().unwrap().parse::<usize>().unwrap();
    let result = json!({"review_passed":true,"pins_checked":pins.as_object().unwrap().len(),"source_count":177,"originals_recovered":176,"originals_exact":174,"G2_implementation_and_default_policy_preserved":true,"weighted_identity_inverse_and_forecast_bodies_exact":true,"field_libraries_reused":true,"native_tests":129,"native_original_tests_newly_executed":125,"new_native_tests":4,"portable_focused_tests":4,"portable_original_tests_filtered":portable_filtered,"Arkworks_tests_each_mode":2,"ordinary_partial_tail_layer_cases_each_mode":780,"weighted_identity_cases_each_mode":120,"std_and_Rayon_workers":[1,4,16,32],"worker_stack_bytes":524288,"std_worker_capacity_bytes":capacities,"capacity_categories":["legacyGroups","prefixRecords","pairIndices","fixedScratch"],"capacities_not_RSS_minRAM_or_totalStack":true,"native_legacy_Group_capacity_zero":true,"portable_prefix_capacity_zero":true,"summaries":summaries,"outputs_sha256":outputs,"receipts":receipts,"performance_measured":false,"new_proofs":0,"production_changed":false,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("ReviewPASS177sources/176recovered/10firstsuccess/native129+portable4+Ark2each; oldG1Groupunallocated; no performance/adoption");
}
