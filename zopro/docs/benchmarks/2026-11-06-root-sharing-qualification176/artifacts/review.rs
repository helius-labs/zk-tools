use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const FIELD:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn receipt(base: &Path, step: &Value, expected: i64) -> Value {
    let name = step["name"].as_str().unwrap();
    let root = base.join(name);
    let exit = load(root.join("exit.json"));
    assert_eq!(exit["returncode"], expected);
    assert_eq!(exit["success"], expected == 0);
    let command = load(root.join("command.json"));
    assert_eq!(
        &command["argv"].as_array().unwrap()[4..],
        step["argv"].as_array().unwrap()
    );
    for stream in ["stdout", "stderr"] {
        assert_eq!(
            hash(root.join(format!("{stream}.log"))),
            exit[format!("{stream}_sha256")]
        );
    }
    json!({"name":name,"returncode":expected,"command_sha256":hash(root.join("command.json")),"exit_sha256":hash(root.join("exit.json"))})
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("review.json").exists());
    let mut pin_counts = BTreeMap::new();
    for f in ["build-pins.json", "driver-repair-pins.json"] {
        let p = load(r.join(f));
        for (file, h) in p.as_object().unwrap() {
            assert_eq!(hash(file), h.as_str().unwrap());
        }
        pin_counts.insert(f, p.as_object().unwrap().len());
    }
    let initial = load(r.join("build-plan.json"));
    assert_eq!(initial["steps"].as_array().unwrap().len(), 3);
    let mut receipts = vec![];
    for (i, step) in initial["steps"].as_array().unwrap().iter().enumerate() {
        receipts.push(receipt(
            &r.join("build-runs"),
            step,
            if i == 2 { 1 } else { 0 },
        ));
    }
    assert!(!r.join("build-runs/result.json").exists());
    assert_eq!(
        load(r.join("build-runs/completed.json"))
            .as_array()
            .map(|a| a.len())
            .unwrap_or(2),
        2
    );
    assert!(
        fs::read_to_string(r.join("build-runs/build-driver/stderr.log"))
            .unwrap()
            .contains("Can't find section .llvmbc")
    );
    let repair = load(r.join("driver-repair-plan.json"));
    assert_eq!(repair["steps"].as_array().unwrap().len(), 1);
    assert_eq!(
        load(r.join("driver-repair-runs/result.json"))["status"],
        "complete"
    );
    receipts.push(receipt(
        &r.join("driver-repair-runs"),
        &repair["steps"][0],
        0,
    ));
    let a = initial["steps"][2]["argv"].as_array().unwrap();
    let b = repair["steps"][0]["argv"].as_array().unwrap();
    assert_eq!(a.len(), b.len());
    let changes = a
        .iter()
        .zip(b)
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(i, (a, b))| json!({"index":i,"old":a,"new":b}))
        .collect::<Vec<_>>();
    assert_eq!(changes.len(), 2);
    assert!(changes
        .iter()
        .all(|c| ["serde_json=", "sha2="].iter().any(|prefix| c["old"]
            .as_str()
            .unwrap()
            .starts_with(prefix)
            && c["new"].as_str().unwrap().starts_with(prefix))));
    let strings = b.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>();
    for flag in [
        "opt-level=3",
        "codegen-units=1",
        "lto=fat",
        "target-cpu=native",
    ] {
        assert!(strings.contains(&flag));
    }
    assert!(strings.contains(&format!("zopro_bn254={FIELD}").as_str()));
    assert_eq!(
        hash(FIELD),
        "1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713"
    );
    let oracle = load(r.join("oracle.json"));
    assert_eq!(oracle["passed"], true);
    for (key, n) in [
        ("bulk_root_output_configurations", 12544),
        ("bulk_zero_rejections", 49),
        ("general_batch_cases", 776),
        ("general_batch_root_outputs_checked", 37248),
        ("fixture_rows", 256),
        ("fixture_input_bytes", 262144),
    ] {
        assert_eq!(oracle[key], n);
    }
    let counts = oracle["general_batch_counts"].as_array().unwrap();
    assert_eq!(counts.len(), 97);
    for (b, c) in counts.iter().enumerate() {
        assert_eq!(c["roots"], b);
        assert_eq!(c["scalar_multiplies"], if b == 0 { 0 } else { 3 * (b - 1) });
        assert_eq!(c["scalar_inversions"], usize::from(b > 0));
    }
    let expected_outputs = (0..=96usize).sum::<usize>() * 8;
    assert_eq!(expected_outputs, 37248);
    for c in oracle["conceptual_state_only"].as_array().unwrap() {
        let w = c["workers"].as_u64().unwrap() as usize;
        assert_eq!(c["slot_bytes"], 64 * w);
        assert_eq!(c["mask_bytes"], 64 * w.div_ceil(64));
        assert_eq!(c["inline_state_bytes"], 128);
        assert_eq!(
            c["total_requested_bytes_without_allocator_rounding"],
            128 + 64 * w + 64 * w.div_ceil(64)
        );
        assert_eq!(
            c["leader_arrays_payload_bytes_if_three_Fq_arrays_and_usize_ids"],
            104 * w
        );
    }
    assert_eq!(
        oracle["bulk_source_counts"],
        json!({"1":{"vector_mul":0,"scalar_mul":0,"scalar_inverse":1},"16":{"vector_mul":8,"scalar_mul":1,"scalar_inverse":1},"32":{"vector_mul":14,"scalar_mul":1,"scalar_inverse":1}})
    );
    let bulk = fs::read_to_string(r.join("bulk.rs")).unwrap();
    let compact = bulk
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let simd = fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/simd.rs")
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    for pair in [
        ("letpairs=a.mul(&swapped);", "letpairs=self.mul(&swapped);"),
        (
            "letquads=pairs.mul(&other_pair);",
            "letquads=pairs.mul(&other_pair);",
        ),
        (
            "letinverse=(products[0]*products[4]).inverse()?;",
            "letinverse=(products[0]*products[4]).inverse().unwrap();",
        ),
        (
            ".mul(&other_quad).mul(&other_pair).mul(&swapped)",
            ".mul(&other_quad).mul(&other_pair).mul(&swapped)",
        ),
    ] {
        assert!(compact.contains(pair.0));
        assert!(simd.contains(pair.1));
    }
    let workspace = fs::read_to_string(
        "/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/msm_affine/workspace.rs",
    )
    .unwrap();
    assert!(workspace.contains("Leaf kernels here do not fork while their scratch is live"));
    assert!(workspace.contains("mul(leaf, rayon::current_num_threads().max(1))"));
    let driver = fs::read_to_string(r.join("driver.rs")).unwrap();
    for text in [
        "stack_size(512 * 1024)",
        "ready.wait()",
        "start.wait()",
        "finish.wait()",
        "assert_eq!(allocations, 0",
        "shared::<16>",
        "shared::<32>",
        "Control::from_env()",
        "zero_request_publication_or_queue_cost",
    ] {
        assert!(driver.contains(text), "{text}");
    }
    assert!(!r.join("capture-plan.json").exists());
    let initial_prepare_stderr =
        fs::read_to_string(r.join("repair-initial/repair-driver.stderr.log")).unwrap();
    assert!(initial_prepare_stderr.contains("No such file or directory"));
    let repair_source = fs::read_to_string(r.join("repair-driver.rs")).unwrap();
    assert!(repair_source.contains("build-runs/completed.json"));
    let old_repair_source = fs::read_to_string(r.join("repair-initial/repair-driver.rs")).unwrap();
    assert!(old_repair_source.contains("build-runs/result.json"));
    let mut identities = BTreeMap::new();
    for f in [
        "oracle",
        "driver",
        "bulk.rs",
        "fixture.rs",
        "driver.rs",
        "oracle.rs",
        "perf.rs",
        "build-plan.json",
        "driver-repair-plan.json",
    ] {
        identities.insert(f, json!({"path":r.join(f),"sha256":hash(r.join(f))}));
    }
    identities.insert("accepted_field", json!({"path":FIELD,"sha256":hash(FIELD)}));
    let out = json!({"review_passed":true,"pins_checked":pin_counts,"command_attempts":4,"successful_commands":3,"receipts":receipts,"driver_dependency_only_changes":changes,"oracle_first_success_reused":true,"driver_link_repaired":true,"initial_repair_helper_missing_result_file_retained":true,"driver_source_unchanged_during_repair":true,"driver_optimized_native_fatLTO":true,"identities":identities,"fixture_sha256":oracle["fixture_hash"],"bulk_outputs_checked":12544,"zero_rejections":49,"general_batch_cases":776,"general_root_outputs_checked":37248,"broker_implemented":false,"concurrency_qualified":false,"driver_linked_not_executed":true,"timing_started":false,"new_proofs":0,"proof_ledger":30846,"adopted":false,"reviewer_source_sha256":hash(r.join("review.rs")),"reviewer_binary_sha256":hash(r.join("review"))});
    fs::write(
        r.join("review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("ReviewPASS:4commandattempts/3successes/unchangedfield+source176/oraclesreused/nativefatLTOdriverlinked; no clock/broker/concurrency claim");
    let _ = PathBuf::new();
}
