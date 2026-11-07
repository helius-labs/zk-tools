use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/g2-field-volume176";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-operation-census176";
const REPO: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn n(v: &Value, key: &str) -> u64 {
    v[key].as_u64().unwrap()
}
fn units(vm_lo: u64, vm_hi: u64, vf: u64, sm: u64, ss: u64, sf: u64, inverses: u64) -> Value {
    json!({"VM_lower":vm_lo,"VM_upper":vm_hi,"VF":vf,"SM":sm,"SS":ss,"SF":sf,"I":inverses,
        "vector_proxy_lower":100*vm_lo+150*vf,"vector_proxy_upper":100*vm_hi+150*vf})
}
fn main() {
    assert!(!Path::new(&format!("{ROOT}/ledger.json")).exists());
    for (p, h) in read(format!("{ROOT}/input-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let manifest=read(format!("{REPO}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let source_files = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(source_files.len(), 176);
    for (p, h) in source_files {
        assert_eq!(hash(format!("{REPO}/{p}")), h.as_str().unwrap(), "{p}");
    }
    let review = read(format!("{OLD}/review.json"));
    for k in [
        "passed",
        "production176_exact",
        "all177_parent_source_bytes_recovered",
        "histograms_and_all_counter_conservation_checked",
    ] {
        assert_eq!(review[k], true);
    }
    let a = read(format!("{OLD}/analysis.json"));
    let msm = fs::read_to_string(format!("{REPO}/crates/zopro-core/src/msm.rs")).unwrap();
    assert!(msm.contains("F::<C>::TREE_PREFIX && m >= 32"));
    assert!(msm.contains("Fq::sum_of_products(&value.c0, &value.c0, &value.c1, &value.c1)"));
    let simd = fs::read_to_string(format!("{REPO}/crates/zopro-core/src/simd.rs")).unwrap();
    let inv = simd
        .split("pub(crate) fn inverse_nonzero(&self) -> Self {")
        .nth(1)
        .unwrap()
        .split("#[inline(always)]")
        .next()
        .unwrap();
    assert_eq!(inv.matches(".mul(").count(), 5);
    assert!(inv.contains("(products[0] * products[4]).inverse().unwrap()"));
    let mut cases = BTreeMap::new();
    for shape in ["1x2", "2x2"] {
        let c = &a["cases"][format!("{shape}-OFF-w32")];
        let solo = &a["cases"][format!("{shape}-OFF-w1")];
        for k in [
            "totals",
            "global",
            "normalized_projective_points",
            "fixture_sha256",
            "result_sha256",
        ] {
            assert_eq!(c[k], solo[k], "{shape}/{k}");
        }
        assert_eq!(c["candidate_used"], false);
        assert_eq!(c["independent_complete_MSM_hash_matches"], true);
        let mut blocks = 0;
        let mut tails = 0;
        let mut batches = 0;
        let mut pairs = 0;
        for row in c["rows"].as_array().unwrap() {
            for (j, h) in row["batch_histograms"]
                .as_array()
                .unwrap()
                .iter()
                .enumerate()
            {
                let mut rb = 0;
                let mut rt = 0;
                let mut rn = 0;
                let mut rp = 0;
                for (len, count) in h.as_object().unwrap() {
                    let m: u64 = len.parse().unwrap();
                    let count = count.as_u64().unwrap();
                    assert!(m > 0 && m <= n(row, "batch_size"));
                    rb += (m / 8) * count;
                    rt += (m % 8) * count;
                    rn += count;
                    rp += m * count;
                }
                let component = &row["components"][j];
                assert_eq!(rb * 8, n(component, "vector_pairs"));
                assert_eq!(rt, n(component, "scalar_tail_pairs"));
                assert_eq!(rn, n(component, "batches"));
                assert_eq!(rp, n(component, "affine_updates"));
                blocks += rb;
                tails += rt;
                batches += rn;
                pairs += rp;
            }
        }
        let t = &c["totals"];
        let g = &c["global"];
        assert_eq!(blocks * 8, n(t, "vector_pairs"));
        assert_eq!(tails, n(t, "scalar_tail_pairs"));
        assert_eq!(batches, n(t, "batches"));
        assert_eq!(pairs, n(t, "affine_updates"));
        assert_eq!(n(g, "reduction_vector_pairs") % 8, 0);
        let tree_blocks = n(g, "reduction_vector_pairs") / 8;
        let tree_tails = n(g, "reduction_scalar_tail_pairs");
        let layers = n(g, "reduction_layers");
        assert_eq!(
            n(g, "reduction_generic_pairs"),
            8 * tree_blocks + tree_tails
        );
        assert_eq!(n(g, "zero_generic_layers"), 0);
        let stream = units(
            7 * blocks + 5 * batches,
            7 * blocks + 5 * batches,
            5 * blocks,
            8 * tails + batches,
            2 * tails,
            4 * tails,
            batches,
        );
        let extra = tree_blocks.min(3 * layers);
        let tree = units(
            7 * tree_blocks + 5 * layers,
            7 * tree_blocks + 5 * layers + extra,
            5 * tree_blocks,
            7 * tree_tails + layers,
            0,
            5 * tree_tails,
            layers,
        );
        let sp = n(&stream, "vector_proxy_lower") as f64;
        let share_lo = sp / (sp + n(&tree, "vector_proxy_upper") as f64);
        let share_hi = sp / (sp + n(&tree, "vector_proxy_lower") as f64);
        cases.insert(shape,json!({"fixture_sha256":c["fixture_sha256"],"result_sha256":c["result_sha256"],
            "input_rows":c["input_rows"],"ordinary_windows":c["ordinary_windows"],
            "stream_blocks":blocks,"stream_tail_pairs":tails,"stream_batches":batches,
            "tree_blocks":tree_blocks,"tree_tail_pairs":tree_tails,"tree_layers":layers,
            "stream":stream,"tree":tree,"stream_share_of_counted_vector_proxy":[share_lo,share_hi],
            "excluded_projective_add_calls":t["projective_adds"],"excluded_normalized_points":c["normalized_projective_points"],
            "excluded_identity_or_equal_slots":g["reduction_identity_or_equal_pairs"],
            "tree_prefix_modes_not_observed":true,"worker_aggregate_equality_checked":true}));
    }
    let result = json!({"source_files_verified":176,"cases":cases,"inherited_census_correctness_not_replayed":true,
        "new_native_calls":0,"new_proofs":0,"new_timings":0,"adopted":false,"runtime_candidate_selected":false,
        "scope":"streaming ordinary affine and generic reduction pairs plus their eight-lane inverse bridges only; incomplete whole-query ledger",
        "proxy_is_not_latency":true,"scalar_and_inverse_costs_not_priced":true});
    fs::write(
        format!("{ROOT}/ledger.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    for (shape, v) in result["cases"].as_object().unwrap() {
        println!(
            "{shape}: stream VM={} VF={}, tree VM={}..{} VF={}, stream vector-proxy share={}",
            v["stream"]["VM_lower"],
            v["stream"]["VF"],
            v["tree"]["VM_lower"],
            v["tree"]["VM_upper"],
            v["tree"]["VF"],
            v["stream_share_of_counted_vector_proxy"]
        );
    }
}
