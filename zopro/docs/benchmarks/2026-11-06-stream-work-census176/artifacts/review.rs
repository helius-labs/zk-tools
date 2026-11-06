use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/stream-work-census176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("review.json").exists());
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = read(r.join("plan.json"));
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    let mut receipts = vec![];
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        let b = r.join(format!("runs/{name}"));
        let exit = read(b.join("exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let command = read(b.join("command.json"));
        assert_eq!(
            &command["argv"].as_array().unwrap()[4..],
            step["argv"].as_array().unwrap()
        );
        for s in ["stdout", "stderr"] {
            assert_eq!(
                hash(b.join(format!("{s}.log"))),
                exit[format!("{s}_sha256")]
            );
        }
        receipts.push(json!({"name":name,"exit_sha256":hash(b.join("exit.json")),"command_sha256":hash(b.join("command.json"))}));
    }
    assert_eq!(receipts.len(), 2);
    let result = read(r.join("result.json"));
    assert_eq!(result["binary_sha256"], hash(r.join("model")));
    assert_eq!(result["complete"], true);
    assert_eq!(result["performance_claim"], false);
    assert_eq!(result["native_kernel_executed"], false);
    assert_eq!(result["new_proofs"], 0);
    assert_eq!(result["guard_classification_checks"], 8);
    assert_eq!(result["inherited_capacity_rows_checked"], 72);
    let sizes = [
        0usize, 1, 7, 8, 9, 15, 16, 199, 200, 201, 351, 352, 353, 511,
    ];
    let mut seen = BTreeSet::new();
    let mut points = 0;
    let mut derived = BTreeMap::new();
    for v in result["cases"].as_array().unwrap() {
        let kind = v["kind"].as_str().unwrap();
        let n = v["size"].as_u64().unwrap() as usize;
        assert!(sizes.contains(&n));
        let sign = v["sign_mode"].as_u64().unwrap();
        let reuse = v["reuse"].as_u64().unwrap();
        assert!(sign < 3 && reuse < 2);
        assert!(seen.insert((kind.to_string(), n, sign, reuse)));
        assert_eq!(v["point_outputs_checked"], n);
        points += n;
        let g = n / 8;
        let t = n % 8;
        let forward = if kind == "G1" {
            (2, 0, 2, 0, 0, 4)
        } else {
            assert_eq!(kind, "G2");
            (1, 1, 3, 2, 2, 4)
        };
        let reverse = if kind == "G1" {
            (4, 0, 3, 1, 0, 0)
        } else {
            (6, 4, 5, 0, 2, 8)
        };
        let vm = (forward.0 + reverse.0) * g + 5;
        let vf = (forward.1 + reverse.1) * g;
        let sm = (forward.2 + reverse.2) * t + 1;
        let ss = (forward.3 + reverse.3) * t;
        let sf = (forward.4 + reverse.4) * t;
        let packs = (forward.5 + reverse.5) * g;
        let exports = if kind == "G1" { 2 * g } else { 4 * g };
        let expected = json!({"vector_mul":vm,"vector_fused2":vf,"scalar_mul":sm,"scalar_square":ss,"scalar_fused2":sf,"scalar_inverse":1,"coordinate_vector_packs":packs,"coordinate_vector_exports":exports,"vector_IFMA_source_proxy":100*vm+150*vf});
        assert_eq!(v["counts"], expected);
        assert_eq!(
            result["source_operation_counts"][format!("{kind}-{n}")],
            expected
        );
        derived.insert(format!("{kind}-{n}"), expected);
    }
    assert_eq!(seen.len(), 168);
    assert_eq!(points, 26676);
    assert_eq!(result["point_outputs_checked"], points);
    assert_eq!(result["batch_executions"], seen.len());
    assert_eq!(derived.len(), 28);
    let source = Path::new("/home/ubuntu/src/zk-tools/zopro");
    let manifest=read(source.join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    for (p, h) in result["source_bound"].as_object().unwrap() {
        assert_eq!(hash(source.join(p)), h.as_str().unwrap());
        assert_eq!(manifest["source_files_sha256"][p], *h);
    }
    assert_eq!(result["source_bound"].as_object().unwrap().len(), 5);
    let mut bindings = vec![];
    for(file,phase,needles)in[
  ("crates/zopro-core/src/msm.rs","G1 forward",vec!["letnumerator=scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&chains,signs::<SIGNED,C>(&batch[k..k+LANES]));","chains=chains.mul(&denominator);"]),
  ("crates/zopro-core/src/msm.rs","G1 reverse",vec!["letslope=g.numerator.mul(&inverse);","inverse=inverse.mul(&g.denominator);","letx=slope.mul(&slope).sub(&g.xsum);","lety=slope.mul(&g.ax.sub(&x)).sub(&g.ay);"]),
  ("crates/zopro-core/src/msm/g2_borrowed.rs","G2 prefix",vec!["letnorm=Fq2::lanes_norm(&denominator);","s.prefix_records.push([chains,norm]);","chains=chains.mul(&norm);"]),
  ("crates/zopro-core/src/msm/g2_borrowed.rs","G2 reverse",vec!["letfactor=s.prefix_records[j][0].mul(&inverse);","inverse=inverse.mul(&s.prefix_records[j][1]);","letslope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);"]),
  ("crates/zopro-core/src/msm.rs","G2 vector complex products",vec!["dy[0].sum2(&dx[0],&dy[1],&dx[1])","dy[1].difference2(&dx[0],&dy[0],&dx[1])","a[0].difference2(&b[0],&a[1],&b[1])","a[0].sum2(&b[1],&a[1],&b[0])","value[0].sum2(&value[0],&value[1],&value[1])"]),
  ("crates/zopro-core/src/simd.rs","inverse bridge",vec!["letpairs=self.mul(&swapped);","letquads=pairs.mul(&other_pair);","letinverse=(products[0]*products[4]).inverse().unwrap();",".mul(&other_quad).mul(&other_pair).mul(&swapped)"]),
  ("crates/zopro-bn254/src/fq2.rs","x86 scalar fused2",vec!["Fq::sum_of_products(&a.c0,&b.c0,&(-a.c1),&b.c1)","Fq::sum_of_products(&a.c0,&b.c1,&a.c1,&b.c0)"]),
 ]{let text=fs::read_to_string(source.join(file)).unwrap();let compact=text.chars().filter(|c|!c.is_whitespace()).collect::<String>();for needle in &needles{assert!(compact.contains(needle),"{phase}: {needle}");}bindings.push(json!({"file":file,"phase":phase,"source_sha256":hash(source.join(file)),"whitespace_normalized_required_statements":needles}));}
    let prior = read("/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/analysis.json");
    let mut capacities = vec![];
    for row in prior["rows"].as_array().unwrap() {
        let name = row["context"].as_str().unwrap();
        let p = name.split('-').collect::<Vec<_>>();
        let g1 = p[0] == "G1";
        let n = p[1].parse::<usize>().unwrap();
        let bytes = n / 8 * if g1 { 1600 } else { 640 };
        let tail = if n % 8 == 0 {
            0
        } else if g1 {
            640
        } else {
            1280
        };
        for arm in ["frozen", "OFF", "ON"] {
            assert_eq!(
                row["scratch_capacity_bytes_first_worker"][arm],
                json!([bytes, tail])
            );
            capacities.push(
                json!({"context":name,"arm":arm,"record_bytes":bytes,"tail_capacity_bytes":tail}),
            );
        }
    }
    assert_eq!(capacities.len(), 72);
    let out = json!({"review_passed":true,"source_bindings":bindings,"pins_checked":pins.as_object().unwrap().len(),"receipts":receipts,"batch_count_checked":seen.len(),"point_count_checked":points,"guard_classifications":8,"native_kernel_or_exception_qualification_claimed":false,"performance_claimed":false,"derived_counts":derived,"capacity_comparisons":capacities,"result_sha256":hash(r.join("result.json")),"reviewer_source_sha256":hash(r.join("review.rs")),"reviewer_binary_sha256":hash(r.join("review")),"new_proofs":0,"goal_achieved":false});
    fs::write(
        r.join("review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Review PASS:2receipts/{}pins/168cases/26676point checks/72inheritedcapacities/7sourcebindings; not a native or speed qualification",pins.as_object().unwrap().len());
    let _ = PathBuf::new();
}
