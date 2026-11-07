use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/five-point-rational176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn load(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &str) -> String {
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
fn main() {
    let mut pins = 0;
    for (p, h) in load(&format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins += 1;
    }
    let manifest=load(&format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut sources = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(&format!("{Z}/{p}")), h.as_str().unwrap());
        sources += 1;
    }
    assert_eq!(sources, 176);
    let plan = load(&format!("{R}/plan.json"));
    assert_eq!(load(&format!("{R}/runs/result.json"))["status"], "complete");
    let complete = load(&format!("{R}/runs/completed.json"));
    assert_eq!(complete.as_array().unwrap().len(), 2);
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let base = format!("{R}/runs/{n}");
        let cmd = load(&format!("{base}/command.json"));
        let a = cmd["argv"].as_array().unwrap();
        let b = s["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - b.len()..], b);
        let e = load(&format!("{base}/exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(&format!("{base}/{stream}.log")),
                e[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
    }
    let a = load(&format!("{R}/analysis.json"));
    assert_eq!(a["model_passed"], true);
    assert_eq!(a["native_candidate_selected"], false);
    let mut checked = 0;
    for r in a["full_width_results"].as_array().unwrap() {
        assert_eq!(r["new_full_width_five_point_sums"], 256);
        assert_eq!(r["explicit_guard_checks"], 5);
        assert_eq!(r["M"], 104);
        assert_eq!(r["S"], 5);
        assert_eq!(r["I"], 1);
        assert_eq!(r["preprocessing_M"], 29);
        assert_eq!(r["unexpected_declines"], json!({}));
        checked += 256;
    }
    let small = &a["small_field"];
    let declined = small["declines"]
        .as_object()
        .unwrap()
        .values()
        .map(|v| v.as_u64().unwrap())
        .sum::<u64>();
    assert_eq!(small["accepted"].as_u64().unwrap() + declined, 2048);
    assert_eq!(small["independent_integer_group_oracle"], true);
    assert_eq!(small["accepted"], 1121);
    // Sum stage costs independently of the candidate's closed formula.
    let stages = [
        ("complements", 21),
        ("Delta", 3),
        ("weighted inputs", 5),
        ("V", 10),
        ("synthetic quotients", 15),
        ("weighted coefficients", 20),
        ("v", 1),
        ("N", 11),
        ("D", 2),
        ("X", 2),
        ("H", 2),
        ("Nh", 7),
        ("combined denominator", 1),
        ("normalized output", 4),
    ];
    let m: u64 = stages.iter().map(|s| s.1).sum();
    assert_eq!(m, 104);
    let squares = 5;
    let old = load("/home/ubuntu/src/zopro-perf-work/stream-work-census176/result.json");
    assert_eq!(old["complete"], true);
    let source_counts = &old["source_operation_counts"];
    assert_eq!(source_counts["G1-8"]["vector_mul"], 11);
    assert_eq!(source_counts["G2-8"]["vector_mul"], 12);
    assert_eq!(source_counts["G2-8"]["vector_fused2"], 5);
    assert_eq!(source_counts["G1-0"]["vector_mul"], 5);
    assert_eq!(source_counts["G2-0"]["vector_mul"], 5);
    assert_eq!(
        load("/home/ubuntu/src/zopro-perf-work/stream-work-census176/review.json")["review_passed"],
        true
    );
    let mut count_rows = 0;
    for row in a["rows"].as_array().unwrap() {
        let g = row["eight_lane_five_input_groups"].as_u64().unwrap();
        let inverse_vm = 3 * (g - 1) + 5;
        let g1_vm = (m + squares) * g + inverse_vm;
        let g2_vm = 2 * squares * g + inverse_vm + 2 * g;
        let g2_vf = 2 * m * g + g;
        assert_eq!(row["G1_candidate_VM"], g1_vm);
        assert_eq!(row["G2_candidate_VM"], g2_vm);
        assert_eq!(row["G2_candidate_VF"], g2_vf);
        let b1 = 100 * (4 * g * (11 - 5) + 3 * 5);
        let b2 = 100 * (4 * g * (12 - 5) + 3 * 5) + 150 * (4 * g * 5);
        assert_eq!(row["G1_binary_proxy"], b1);
        assert_eq!(row["G2_binary_proxy"], b2);
        assert_eq!(row["G1_candidate_proxy"], 100 * g1_vm);
        assert_eq!(row["G2_candidate_proxy"], 100 * g2_vm + 150 * g2_vf);
        assert!(100 * g1_vm > b1);
        assert!(100 * g2_vm + 150 * g2_vf > b2 + 400 * g);
        assert_eq!(row["candidate_scalar_inverse_barriers"], 1);
        assert_eq!(row["binary_inverse_barriers"], 3);
        count_rows += 1;
    }
    assert_eq!(count_rows, 9);
    assert_eq!(a["variable_vector_work_gate_passed"], false);
    let out = format!("{R}/review.json");
    assert!(!Path::new(&out).exists());
    let result = json!({"review_passed":true,"input_pins_checked":pins,"accepted_source_files_checked":sources,"command_receipts_checked":2,"full_width_outputs_reconciled":checked,"small_field_accepted_outputs_reconciled":1121,"small_field_declines_reconciled":declined,"arithmetic_stage_costs":stages,"M":m,"S":squares,"I":1,"work_rows_checked":count_rows,"gate_passed":false,"native_selected":false,"analysis_sha256":hash(&format!("{R}/analysis.json")),"model_binary_sha256":hash(&format!("{R}/model")),"review_scope":"Offline independent count/receipt/source reconciliation. Does not replay model/native/proof tests and is not a second derivation checker or cryptographic verifier."});
    fs::write(out, serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!("PASS176production hashes,2receipts,9workrows;fixed fivepoint recipe closed");
}
