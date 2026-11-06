use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-production-coverage176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const P: &str = "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176";
const C: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: &str, v: &Value) {
    fs::write(format!("{R}/{p}"), serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn manifest() -> Value {
    read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"))
}
fn inputs() -> BTreeMap<String, String> {
    let mut pins = BTreeMap::new();
    for (p, h) in manifest()["source_files_sha256"].as_object().unwrap() {
        pins.insert(format!("{Z}/{p}"), h.as_str().unwrap().into());
    }
    for p in [
        format!("{R}/audit.rs"),
        format!("{R}/design.md"),
        format!("{P}/analysis.json"),
        format!("{P}/profile-symbols.json"),
        format!("{P}/review.json"),
        format!("{P}/proof-sha256-after.json"),
        format!("{C}/analysis.json"),
        format!("{C}/review.json"),
        format!("{C}/source-audit.json"),
        format!("{C}/caller-source/crates/zopro-core/src/msm.rs"),
        format!("{C}/identities-tls.json"),
        format!("{Z}/docs/benchmarks/2026-10-22-universal-window-local-weighted-sdk/README.md"),
        format!("{Z}/docs/benchmarks/2026-10-22-universal-weighted-task-grain/README.md"),
        format!("{Z}/docs/benchmarks/2026-10-31-weighted-frontier176/README.md"),
        "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk".into(),
    ] {
        pins.insert(p.clone(), hash(&p));
    }
    pins
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("prepare") {
        assert!(!Path::new(R).join("pins.json").exists());
        save("pins.json", &json!(inputs()));
        save(
            "plan.json",
            &json!({"version":1,"source_pins":format!("{R}/pins.json"),"environment":{},"steps":[{"kind":"command","name":"accepted-coverage-audit","argv":[format!("{R}/audit")],"timeout_seconds":120}]}),
        );
        println!("Prepared one read-only coverage step; no native/profile/proof commands");
        return;
    }
    assert!(!Path::new(R).join("analysis.json").exists());
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    assert_eq!(
        manifest()["source_files_sha256"].as_object().unwrap().len(),
        176
    );
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk"),
        "6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661"
    );
    let inherited = read(format!("{P}/review.json"));
    assert_eq!(inherited["passed"], true);
    let proof = read(format!("{P}/analysis.json"));
    let syms = read(format!("{P}/profile-symbols.json"));
    let component = read(format!("{C}/analysis.json"));
    let cr = read(format!("{C}/review.json"));
    assert_eq!(cr["review_passed"], true);
    assert_eq!(component["gate_passed"], false);
    assert_eq!(
        read(format!("{P}/proof-sha256-after.json"))
            .as_array()
            .unwrap()
            .len(),
        30846
    );
    let old = fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap();
    let new =
        fs::read_to_string(format!("{C}/caller-source/crates/zopro-core/src/msm.rs")).unwrap();
    let stream = |s: &str| {
        let start = s.find("fn stream_g1_impl<").unwrap();
        let end = s[start..].find("struct Fq2AddGroup").unwrap() + start;
        s[start..end].to_owned()
    };
    assert_eq!(stream(&old), stream(&new));
    let stream_sha = format!("{:x}", Sha256::digest(stream(&old).as_bytes()));
    let source_anchors=[
        ("crates/zopro-core/src/lib.rs","fn solve_canonical_wires","native canonical solver and conditional A / fused K+H / G2 query fork"),
        ("crates/zopro-core/src/msm_affine.rs","pub(super) fn msm_grouped<","equal-scalar base sums call group_affine before msm_raw"),
        ("crates/zopro-core/src/msm_affine.rs","pub(super) fn g2_query_exclusive(","exclusive B2 scalars choose separate G2 decomposition/grouping"),
        ("crates/zopro-core/src/msm_affine.rs","fn top_window<","narrow top digits collect order and call indexed tree plus weighted reduction"),
        ("crates/zopro-core/src/msm_affine.rs","fn window<C:","ordinary bucket filling and final weighted reduction are different functions"),
        ("crates/zopro-core/src/msm.rs","fn group_affine_impl<","equal-scalar groups copy host-limb points into worker scratch then reduce layers"),
        ("crates/zopro-core/src/msm.rs","pub(super) fn reduce_indexed_buckets<","G1 indexed leaf tiles use 2048 points, then grouped partial sums"),
        ("crates/zopro-core/src/msm.rs","pub(super) fn weighted_affine<","256-bucket parallel chunks allocate 2*n-1 host-limb tree slots and call weighted_sum"),
        ("crates/zopro-core/src/msm.rs","fn weighted_sum<","each weighted level copies even points into corrections before reduce_layer"),
        ("crates/zopro-core/src/msm.rs","fn stream_g1_impl<","separate streamed addition kernel; byte-exact between accepted and candidate"),
        ("crates/zopro-core/src/msm_affine/fused_kh.rs","pub(crate)fn fused(","fused K/H selects borrowed pending path on supported native inputs"),
        ("crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs","pub(super) fn fused<","balanced K/H has20 windows then256-bucket weighted groups; no narrow-top special branch"),
    ];
    let mut anchors = Vec::new();
    for (file, token, meaning) in source_anchors {
        let text = fs::read_to_string(format!("{Z}/{file}")).unwrap();
        assert_eq!(text.matches(token).count(), 1, "{token}");
        let at = text.find(token).unwrap();
        anchors.push(json!({"path":file,"line":text[..at].bytes().filter(|&b|b==b'\n').count()+1,"token":token,"meaning":meaning,"file_sha256":hash(format!("{Z}/{file}"))}));
    }
    let mut selected = BTreeMap::new();
    for workers in [16, 32] {
        let name = format!("cycles-1x2-w{workers}-accepted");
        let bins = syms[&name].as_array().unwrap();
        let meta = &proof["profiles"][&name];
        let total = bins
            .iter()
            .map(|s| s["samples"].as_u64().unwrap())
            .sum::<u64>();
        assert_eq!(meta["samples"], total);
        assert_eq!(meta["qualified"], true);
        assert_eq!(meta["period16M"], true);
        assert_eq!(meta["zero_lost"], true);
        assert_eq!(meta["absolute_period_coverage_not_validated"], true);
        let mut cats = BTreeMap::<&str, u64>::new();
        for b in bins {
            let n = b["samples"].as_u64().unwrap();
            let fraction = n as f64 / total as f64;
            assert!((fraction - b["share"].as_f64().unwrap()).abs() < 1e-12);
            let s = b["symbol"].as_str().unwrap();
            let cat = if s.starts_with("zopro_core::msm::stream_g1_impl::<") {
                "G1 streamed add"
            } else if s == "zopro_core::msm::reduce_layer::<zopro_bn254::curve::G1Config>" {
                "G1 tree reduce_layer"
            } else if s == "zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>" {
                "G2 tree reduce_layer"
            } else if s.starts_with("zopro_core::msm::g2_borrowed::apply::<") {
                "G2 streamed add"
            } else if s == "zopro_bn254::inverse::inverse::<zopro_bn254::FqConfig>" {
                "shared scalar Fq inverse"
            } else if s.contains("crossbeam_epoch::default::with_handle") {
                "Crossbeam with_handle"
            } else if s == "[unknown]" {
                "unknown"
            } else {
                "other"
            };
            *cats.entry(cat).or_default() += n;
        }
        let unknown = *cats.get("unknown").unwrap_or(&0);
        assert!(
            (unknown as f64 / total as f64 - meta["unknown_share"].as_f64().unwrap()).abs() < 1e-12
        );
        let categories: BTreeMap<_, _> = cats
            .into_iter()
            .map(|(k, n)| (k, json!({"samples":n,"share":n as f64/total as f64})))
            .collect();
        selected.insert(name,json!({"workers":workers,"samples":total,"metadata":meta,"categories":categories,"all_symbol_bins":bins}));
    }
    let out = json!({"complete":true,"mode":"read-only source and existing profile audit","accepted_source_files":176,"accepted_stream_g1_body_exact_in_candidate":true,"stream_g1_body_sha256":stream_sha,"source_anchors":anchors,"profiles":selected,"profile_ownership_validation":"inherited from saved coarse-phase review; no capture/decoder rerun","absolute_sampled_period_coverage_validated":false,"observer_diagnostic_gate_passed":proof["diagnostic_qualified"],"observer_failed_checks":proof["failed"],"component_gate_passed":false,"component_failed_checks":component["failed_checks"],"no_claim_of_wall_critical_path_or_removable_fraction":true,"new_timing_observations":0,"new_proofs":0,"ledger":30846,"production_changed":false,"adopted":false,"goal_status":"active/unmet0/8"});
    save("analysis.json", &out);
    println!("Coverage audit PASS:176 accepted sources; exact unchanged G1 stream; 20837 existing samples conserved; no new timing/proof/adoption");
}
