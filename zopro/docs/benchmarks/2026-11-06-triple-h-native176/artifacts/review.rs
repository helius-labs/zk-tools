use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let mut checked = 0;
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "pin{p}");
        checked += 1;
    }
    let manifest = read(format!("{R}/source-manifest.json"));
    let additions = BTreeMap::from([
        (
            "crates/zopro-core/src/lib.rs",
            "\n#[cfg(zopro_h_only_probe)]\npub use msm::h_only_probe::ReductionCaller;\n",
        ),
        (
            "crates/zopro-core/src/msm.rs",
            "\n#[cfg(zopro_h_only_probe)]\npub mod h_only_probe;\n",
        ),
        (
            "crates/zopro-core/src/simd.rs",
            "\n#[cfg(zopro_h_only_probe)]\nmod h_checked;\n",
        ),
    ]);
    let mut exact = 0;
    for (p, h) in manifest["accepted"].as_object().unwrap() {
        let text = fs::read(format!("{R}/source/{p}")).unwrap();
        if let Some(extra) = additions.get(p.as_str()) {
            assert!(text.ends_with(extra.as_bytes()));
            assert_eq!(
                format!("{:x}", Sha256::digest(&text[..text.len() - extra.len()])),
                h.as_str().unwrap()
            );
        } else {
            assert_eq!(hash(format!("{R}/source/{p}")), h.as_str().unwrap());
            exact += 1;
        }
        assert_eq!(hash(format!("{Z}/{p}")), h.as_str().unwrap());
    }
    assert_eq!(exact, 173);
    assert_eq!(manifest["candidate"].as_object().unwrap().len(), 178);
    let module = fs::read_to_string(format!(
        "{R}/source/crates/zopro-core/src/msm/h_only_probe.rs"
    ))
    .unwrap();
    let old = fs::read_to_string(
        "/home/ubuntu/src/zopro-perf-work/unweighted-triple-caller174/reduction_probe.rs",
    )
    .unwrap();
    let oldtail = old
        .split_once("struct Work {")
        .unwrap()
        .1
        .split_once("#[cfg(test)]")
        .unwrap()
        .0;
    let tail = module
        .split_once("struct Work {")
        .unwrap()
        .1
        .split_once("#[cfg(test)]")
        .unwrap()
        .0;
    // prepare inserts one separating newline before the new test module.
    assert_eq!(tail, format!("{oldtail}\n"));
    assert_eq!(
        fs::read(format!("{R}/probe.rs")).unwrap(),
        fs::read("/home/ubuntu/src/zopro-perf-work/unweighted-triple-caller174/probe.rs").unwrap()
    );
    assert_eq!(fs::read(format!("{R}/source/crates/zopro-core/src/simd/h_checked.rs")).unwrap(),fs::read("/home/ubuntu/src/zopro-perf-work/deferred-equality176/candidate-fixed/source/crates/zopro-core/src/simd/checked_inverse.rs").unwrap());
    assert!(!module.contains("unsafe"));
    let inverse_at = module
        .find("let Some(mut inverse) = chains.inverse_checked()")
        .unwrap();
    let first_store = module.find("points[k] = LimbPoint").unwrap();
    assert!(inverse_at < first_store);
    assert!(!module[..inverse_at].contains("*n ="));
    assert!(module.contains("for (&start, n) in starts.iter().zip(counts.iter_mut())"));
    let plan = read(format!("{R}/plan.json"));
    let mut receipts = Vec::new();
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        let dir = format!("{R}/runs/{name}");
        let receipt = read(format!("{dir}/command.json"));
        let exit = read(format!("{dir}/exit.json"));
        assert_eq!(exit["success"], true, "{name}");
        assert_eq!(exit["returncode"], 0);
        assert_eq!(
            &receipt["argv"].as_array().unwrap()[4..],
            step["argv"].as_array().unwrap().as_slice()
        );
        assert_eq!(hash(format!("{dir}/stdout.log")), exit["stdout_sha256"]);
        assert_eq!(hash(format!("{dir}/stderr.log")), exit["stderr_sha256"]);
        if name.starts_with("build-") {
            let a: Vec<_> = step["argv"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap())
                .collect();
            assert!(a.contains(&"-O"));
            assert!(a.contains(&"target-cpu=native"));
            assert!(a.contains(&"codegen-units=1"));
            if name != "build-core" {
                assert!(a.contains(&"lto=fat"));
            }
        }
        receipts.push(json!({"name":name,"returncode":0,"elapsed_ms":exit["elapsed_ms"]}));
    }
    assert_eq!(receipts.len(), 6);
    let focused = read(format!("{R}/runs/native-focused-tests/tests.json"));
    assert_eq!(focused["passed_tests"].as_array().unwrap().len(), 5);
    let native = fs::read_to_string(format!("{R}/runs/native-focused-tests/stderr.log")).unwrap();
    for s in [
        "complete layers=48",
        "inverse masks=255+nonzero",
        "rejected-before-write=72; admitted opposite lanes=8",
        "malformed layouts=7",
    ] {
        assert!(native.contains(s), "{s}");
    }
    let mut capacity = None;
    for workers in [1, 4, 16, 32] {
        let prefix = format!("H_NATIVE workers={workers} stack_bytes=524288 per_worker_capacity=");
        let line = native.lines().find(|s| s.starts_with(&prefix)).unwrap();
        let b: Vec<u64> = serde_json::from_str(line.strip_prefix(&prefix).unwrap()).unwrap();
        assert_eq!(b.len(), 4);
        assert!(b.iter().all(|n| *n > 0));
        if let Some(prior) = &capacity {
            assert_eq!(&b, prior);
        } else {
            capacity = Some(b);
        }
    }
    let stdout = fs::read_to_string(format!("{R}/runs/native-focused-tests/stdout.log")).unwrap();
    assert!(stdout.contains("5 passed; 0 failed;"));
    let oracle = read(format!("{R}/runs/independent-oracle/tests.json"));
    assert_eq!(oracle["passed_tests"].as_array().unwrap().len(), 1);
    assert!(
        fs::read_to_string(format!("{R}/runs/independent-oracle/stdout.log"))
            .unwrap()
            .contains("216 boundary triples;both paths/lifetimes")
    );
    let mut binaries = BTreeMap::new();
    for p in [
        "libs/libzopro_core.rlib",
        "core-tests",
        "oracle-tests",
        "caller",
    ] {
        binaries.insert(p, hash(format!("{R}/{p}")));
    }
    let out = json!({"review_passed":true,"pins_checked":checked,"all_176_originals_recovered":true,"original_files_byte_exact":exact,"candidate_source_files":178,"original_binary_and_weighted_routes_byte_exact":true,"old_complete_caller_and_fixture_tail_byte_exact":true,"old_driver_byte_exact":true,"checked_inverse_prior_art_byte_exact":true,"new_unsafe_blocks":0,"focused_native_tests":5,"independent_Arkworks_tests":1,"partial_and_remainder_layers":48,"root_zero_masks":255,"exceptional_rejections_without_mutation":72,"opposite_lanes_admitted":8,"malformed_layouts":7,"test_workers":[1,4,16,32],"test_worker_stack_bytes":524288,"per_worker_retained_capacity_bytes":capacity,"capacity_categories":["common points/counts/output","binary fallback scratch","ternary indices+both 2-vector record buffers","fixed Work"],"capacity_is_not_RSS_or_minimum_RAM":true,"native_test_summary":stdout.lines().find(|s|s.starts_with("test result:")).unwrap(),"binaries_sha256":binaries,"workflow_receipts":receipts,"native_implementation_qualified":true,"performance_measured":false,"new_proofs":0,"production_changed":false,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Native review PASS: {checked}pins/176recovered/6receipts/5native+1Arkworks/1,4,16,32workers512KiB; NO latency or adoption");
}
