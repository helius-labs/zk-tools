use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-norm-numerator176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn load(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &str) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut d = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        d.update(&b[..n]);
    }
    format!("{:x}", d.finalize())
}
fn main() {
    let mut pins = 0;
    for (p, h) in load(&format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins += 1;
    }
    let manifest=load(&format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(&format!("{Z}/{p}")), h.as_str().unwrap());
    }
    assert_eq!(
        manifest["source_files_sha256"].as_object().unwrap().len(),
        176
    );
    let plan = load(&format!("{R}/plan.json"));
    assert_eq!(load(&format!("{R}/runs/result.json"))["status"], "complete");
    assert_eq!(
        load(&format!("{R}/runs/completed.json"))
            .as_array()
            .unwrap()
            .len(),
        2
    );
    for s in plan["steps"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        let base = format!("{R}/runs/{name}");
        let exit = load(&format!("{base}/exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let c = load(&format!("{base}/command.json"));
        let a = c["argv"].as_array().unwrap();
        let b = s["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - b.len()..], b);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(&format!("{base}/{stream}.log")),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
    }
    let a = load(&format!("{R}/analysis.json"));
    assert_eq!(a["mathematical_qualification_passed"], true);
    assert_eq!(a["native_qualified"], false);
    assert_eq!(a["performance_qualified"], false);
    let mut raw = 0;
    assert_eq!(a["raw_scaled_fused"].as_array().unwrap().len(), 2);
    for row in a["raw_scaled_fused"].as_array().unwrap() {
        assert_eq!(row["inclusive_corner_outputs"], 2 * 7u64.pow(4));
        assert_eq!(row["random_outputs"], 2 * 2048);
        assert_eq!(row["radix_boundary_outputs"], 14 * 3 * 2);
        assert_eq!(row["changing_chain_outputs"], 1024);
        let count = 2 * 7u64.pow(4) + 2 * 2048 + 14 * 3 * 2 + 1024;
        assert_eq!(row["outputs"], count);
        raw += count;
    }
    let sizes = [
        0u64, 1, 7, 8, 9, 15, 16, 31, 32, 63, 64, 199, 200, 201, 351, 352, 353, 511,
    ];
    let mut seen = BTreeSet::new();
    let mut outputs = 0;
    for row in a["point_model"]["rows"].as_array().unwrap() {
        let n = row["points"].as_u64().unwrap();
        let s = row["signs"].as_u64().unwrap();
        let reuse = row["reuse"].as_u64().unwrap();
        assert!(sizes.contains(&n) && s < 3 && reuse < 3);
        assert!(seen.insert((n, s, reuse)));
        assert_eq!(row["full_vector_groups"], n / 8);
        assert_eq!(row["scalar_tail"], n % 8);
        assert_eq!(row["outputs_checked"], n);
        assert_eq!(
            row["unchanged_bucket_slots_checked"],
            (n + 1).next_power_of_two() - n
        );
        outputs += n;
    }
    assert_eq!(seen.len(), sizes.len() * 3 * 3);
    assert_eq!(outputs, sizes.iter().sum::<u64>() * 9);
    assert_eq!(a["point_model"]["new_point_outputs"], outputs);
    assert_eq!(a["point_model"]["guard_checks"], 6);
    let field = fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/ifma.rs")).unwrap();
    let point = fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap();
    let borrowed =
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm/g2_borrowed.rs")).unwrap();
    let anchors = [
        field.contains("if FUSED && MIXED {\n                Self::reduce_fused(result)"),
        field.contains("row!(4);"),
        field.contains("let negative = unsafe"),
        point.contains("dy[0].sum2(&dx[0], &dy[1], &dx[1])"),
        point.contains("dy[1].difference2(&dx[0], &dy[0], &dx[1])"),
        borrowed.contains("let factor = s.prefix_records[j][0].mul(&inverse);"),
        borrowed.contains("inverse = inverse.mul(&s.prefix_records[j][1]);"),
    ];
    assert!(anchors.iter().all(|a| *a));
    let head = load(&format!("{R}/upstream-check.json"));
    assert_eq!(head["observed_head"], head["measured_reference"]);
    let out = format!("{R}/review.json");
    assert!(!Path::new(&out).exists());
    let review = json!({"review_passed":true,"input_pins_checked":pins,"accepted_source_files_checked":176,"model_receipts_checked":2,"source_anchors_checked":anchors.len(),"new_raw_outputs_reconciled":raw,"new_point_outputs_reconciled":outputs,"point_cases_reconciled":seen.len(),"guard_checks":6,"native_selected_next":true,"native_qualified":false,"performance_claimed":false,"production_changed":false,"analysis_sha256":hash(&format!("{R}/analysis.json")),"model_binary_sha256":hash(&format!("{R}/model")),"review_scope":"Offline source, receipt and coverage reconciliation; no model/native/proof replay and no second independent derivation or REDC implementation."});
    fs::write(out, serde_json::to_vec_pretty(&review).unwrap()).unwrap();
    println!("PASS20012raw+21717point outputs/162cases/source bindings; native stage next");
}
