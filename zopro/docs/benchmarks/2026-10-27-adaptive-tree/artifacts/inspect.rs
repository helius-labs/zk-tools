use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: &str) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-adaptive-tree");
    let mut records = Vec::new();
    for (name, binary) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate-sdk",
        ),
        ("fresh", "/tmp/zopro-poseidon-fused-dot/native/fresh-sdk"),
        ("tree", "/tmp/zopro-g2-tree-records/native/tree-sdk"),
        ("adaptive", "/tmp/zopro-adaptive-tree/native/adaptive-sdk"),
    ] {
        let out = Command::new("nm")
            .args(["-S", "-C", binary])
            .output()
            .unwrap();
        assert!(out.status.success());
        let text = String::from_utf8(out.stdout).unwrap();
        let selected: Vec<_> = text
            .lines()
            .filter(|l| l.contains("zopro_core::msm::reduce_layer::<"))
            .collect();
        assert_eq!(selected.len(), 2);
        fs::write(
            r.join(format!("symbols-{name}.txt")),
            selected.join("\n") + "\n",
        )
        .unwrap();
        for line in selected {
            let f: Vec<_> = line.split_whitespace().collect();
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            let curve = if line.contains("G2Config") {
                "G2"
            } else {
                "G1"
            };
            records.push(json!({"variant":name,"binary":binary,"binary_sha256":hash(binary),"curve":curve,"address":start,"symbol_bytes":size,"symbol":f[3]}));
        }
    }
    let adaptive = records
        .iter()
        .find(|r| r["variant"] == "adaptive" && r["curve"] == "G2")
        .unwrap()["symbol_bytes"]
        .as_u64()
        .unwrap();
    let tree = records
        .iter()
        .find(|r| r["variant"] == "tree" && r["curve"] == "G2")
        .unwrap()["symbol_bytes"]
        .as_u64()
        .unwrap();
    let audit = json!({"records":records,"adaptive_G2_vs_unconditional_size_ratio":adaptive as f64/tree as f64,"capture_phase":"after all latency/profiles/analysis","static_only":true,"causal_latency_or_cache_claim":false,"hypothesis":"Runtime guard places both arithmetic branches in one G2 reduce_layer symbol. Compare a distinct implementation with pair-selection/compaction in the wrapper and const-specialized ordinary-pair kernels, preserving the fixed32 rule and all formulas. No inline annotation, threshold sweep, pool change or rerun of this candidate."});
    fs::write(
        r.join("codegen-audit.json"),
        serde_json::to_vec_pretty(&audit).unwrap(),
    )
    .unwrap();
    println!("Static G2symbol bytes adaptive{adaptive},unconditional{tree};no newCPUcaptures");
}
