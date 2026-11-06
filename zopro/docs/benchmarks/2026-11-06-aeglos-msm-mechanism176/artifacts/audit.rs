use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/aeglos-msm-mechanism176";
const REPO: &str = "/home/ubuntu/src/zk-tools/zopro";
const MANIFEST: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json";
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn wc(c: usize) -> usize { (128+c)/c }
fn sorted(n: usize, threads: usize) -> Value {
    let c = (8..=20).min_by_key(|&c| 2*n*wc(c) + 16*(1usize<<(c-1))).unwrap();
    let windows = wc(c);
    let buckets = 1usize << (c-1);
    let blocks = buckets.min(4usize << (usize::BITS - threads.leading_zeros()).max(1));
    let ranges = (buckets/16).min(2*threads).div_ceil(8)*8;
    json!({"n":n,"degree":2,"threads":threads,"bits":c,"windows":windows,
        "buckets":buckets,"blocks":blocks,"buckets_per_block":buckets/blocks,
        "ranges":ranges,"sum_tasks_if_IFMA8":ranges/8,
        "base_payload_bytes":2*n*windows*128,"unshifted_two_base_payload_bytes":2*n*128,
        "codes_payload_bytes_if_unshared":2*n*windows*4,
        "items_max_payload_bytes_if_unshared":2*n*windows*8,
        "sums_payload_bytes":buckets*128,
        "all_blocks_arena_upper_payload_bytes":2*n*windows*128,
        "scope":"source formula applied to illustrative published raw query sizes; not observed Aeglos plan, live memory, allocation capacity or performance"})
}
fn main() {
    let arg=std::env::args().nth(1).unwrap();
    if arg=="prepare" {
        let mut pins=serde_json::Map::new();
        for name in ["audit.rs","design.md","reading.md","source-pins.sha256","msm_impl.hpp","msm.hpp","msm.cpp","pool.hpp","noel-algorithm4-full.png"] {
            let p=format!("{ROOT}/{name}"); pins.insert(p.clone(),json!(hash(p)));
        }
        pins.insert(MANIFEST.into(),json!(hash(MANIFEST)));
        let manifest=read(MANIFEST);
        for (name,expected) in manifest["source_files_sha256"].as_object().unwrap() {
            let p=format!("{REPO}/{name}"); assert_eq!(hash(&p),expected.as_str().unwrap()); pins.insert(p,expected.clone());
        }
        for report in ["2026-10-22-aeglos-windowed-ablation","2026-10-22-aeglos-zopro-local-profiles","2026-10-22-universal-flat-window-reduction-sdk","2026-10-24-joint-msm-phases","2026-10-26-g2-index-gather","2026-10-29-shared-wire-sdk175","2026-10-30-partial-windows175","2026-10-31-weighted-frontier176","2026-11-06-g2-two-chain-stream176"] {
            let p=format!("{REPO}/docs/benchmarks/{report}/README.md"); pins.insert(p.clone(),json!(hash(p)));
        }
        fs::write(format!("{ROOT}/pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
        let plan=json!({"version":1,"source_pins":format!("{ROOT}/pins.json"),"steps":[{"name":"source-mechanism-audit","kind":"command","argv":[format!("{ROOT}/audit"),"analyze"],"timeout_seconds":120}]});
        fs::write(format!("{ROOT}/plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
        println!("Frozen {} source/reference/helper pins; no runtime candidate",pins.len()); return;
    }
    assert_eq!(arg,"analyze");
    let pins=read(format!("{ROOT}/pins.json"));
    for (p,h) in pins.as_object().unwrap() { assert_eq!(hash(p),h.as_str().unwrap()); }
    for line in fs::read_to_string(format!("{ROOT}/source-pins.sha256")).unwrap().lines() {
        let (expected,p)=line.split_once("  ").unwrap(); assert_eq!(hash(p),expected);
    }
    let manifest=read(MANIFEST); assert_eq!(manifest["source_files_sha256"].as_object().unwrap().len(),176);
    let mut claims=vec![];
    for (file,marker,claim) in [
        ("msm.hpp","static constexpr uint64_t SHIFTED_BYTES = uint64_t(8) << 30;","eight-GiB per-MSM shifted ceiling is further limited by context budget at constructor"),
        ("msm.cpp","std::min(MsmPlan::SHIFTED_BYTES, ctx->budget.limit() / 8)","default constructor uses one eighth context budget; explicit nonzero limit overrides"),
        ("msm.hpp","return size_t(windows) * 2 * n * words();","prepared bases retain every shift and endomorphism image"),
        ("msm_impl.hpp","own->source = other;","eligible Sorted jobs share item descriptors; eligibility compares scalar/index/stride/n/mont/plan dimensions and infinity mask"),
        ("msm_impl.hpp","owner->wires = std::max(owner->wires, own->wires);","Sorted wire recoding sharing covers common prefix through highest required wire, not just queried union"),
        ("msm_impl.hpp","arena.resize((hi - lo) * PW);","Sorted worker-local intermediate arena scales with block item count and persists in thread-local vector capacity"),
        ("msm_impl.hpp","ranges = std::max<size_t>(1, std::min<size_t>(work[0].buckets / 64, 16) / Point::W) * Point::W;","Windowed weighted ranges and windows remain distinct"),
        ("msm_impl.hpp","chunks = std::max<size_t>(1, (4 * threads + windows - 1) / windows);","Windowed targets at least four times thread-count window-chunk tasks"),
        ("msm_impl.hpp","std::stable_sort(jobs.begin(), jobs.end()","all MSM jobs share passes; stable ordering prioritizes degree two"),
        ("pool.hpp","never nested.","Pool interface promises nonnested passes; implementation was not fetched in this audit")
    ] {
        let s=fs::read_to_string(format!("{ROOT}/{file}")).unwrap();
        let positions=s.match_indices(marker).collect::<Vec<_>>(); assert_eq!(positions.len(),1,"{file}: {marker}");
        let line=s[..positions[0].0].bytes().filter(|&b|b==b'\n').count()+1;
        claims.push(json!({"file":file,"line":line,"marker_sha256":format!("{:x}",Sha256::digest(marker.as_bytes())),"claim":claim}));
    }
    let phi=fs::read_to_string(format!("{REPO}/crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
    for expected in ["hb_raw(bases, words, 12, 2)","let windows = 127usize.div_ceil(bits);","let mut high = hb_top_component","let low = hb_top_component"] { assert!(phi.contains(expected)); }
    let analysis=json!({"scope":"pinned read-only structural audit; no benchmark, new proof or runtime implementation",
        "reference_commit":"0290613e8eeca17a4ce88f6f2d726e8e06a5385d","pins_verified":pins.as_object().unwrap().len(),"accepted_source_files_verified":176,
        "claims":claims,"illustrative_sorted_plans":[sorted(30089,32),sorted(30089,96),sorted(52274,32),sorted(52274,96)],
        "zopro_small_G2_geometry":{"bits":12,"windows":11,"ordinary_windows":10,"point_parts":2,"ordinary_bucket_sets":20,"ordinary_buckets_each":2048,"top_magnitude_buckets_each":64,"top_sign_bins_each_component_each_part":128,"top_component_reductions":4,"weighted_calls":11,"weighted_bucket_slots":20544,"endomorphism":"applied to bucket sums after high component, not stored once per key point/shift"},
        "decision":"No distinct low-memory runtime candidate selected from this audit. Earlier shared-phase, recoding/classification, indexed-gather and prefix-chain outcomes remain closed. Expanded shifted-base construction is outside selected architecture.",
        "new_runtime_candidate":false,"new_performance_measurements":0,"new_proofs":0,"new_AWS_resources":0,"goal_achieved":false,"goal_status":"active/unmet0/8"});
    fs::write(format!("{ROOT}/analysis.json"),serde_json::to_vec_pretty(&analysis).unwrap()).unwrap();
    println!("Audit PASS: {} pins, 176 original sources, {} unique source anchors, 4 illustrative plan geometries; no performance claim",pins.as_object().unwrap().len(),claims.len());
}
