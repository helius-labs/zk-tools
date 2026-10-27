use serde_json::json;
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
const R: &str = "/tmp/zopro-tree-kernel-split";
fn hash(p: &str) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    let mut records = Vec::new();
    let mut split_refs = false;
    for (name, binary) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate-sdk",
        ),
        ("fresh", "/tmp/zopro-poseidon-fused-dot/native/fresh-sdk"),
        ("parent", "/tmp/zopro-adaptive-tree/native/adaptive-sdk"),
        ("split", "/tmp/zopro-tree-kernel-split/native/split-sdk"),
    ] {
        let out = Command::new("nm")
            .args(["-S", "-C", binary])
            .output()
            .unwrap();
        assert!(out.status.success());
        let text = String::from_utf8(out.stdout).unwrap();
        let selected: Vec<_> = text
            .lines()
            .filter(|l| {
                l.contains("zopro_core::msm::reduce_layer::<")
                    || l.contains("zopro_core::msm::pairs::apply::<")
            })
            .collect();
        fs::write(
            r.join(format!("symbols-{name}.txt")),
            selected.join("\n") + "\n",
        )
        .unwrap();
        let mut local = Vec::new();
        for line in selected {
            let f: Vec<_> = line.split_whitespace().collect();
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            let symbol = f[3..].join(" ");
            let curve = if symbol.contains("G2Config") {
                "G2"
            } else {
                "G1"
            };
            local.push(json!({"variant":name,"binary":binary,"binary_sha256":hash(binary),"curve":curve,"address":start,"symbol_bytes":size,"symbol":symbol,"helper":symbol.contains("pairs::apply")}));
        }
        if name == "split" {
            let root = local
                .iter()
                .find(|s| s["curve"] == "G2" && s["helper"] == false)
                .unwrap();
            let helpers: Vec<_> = local
                .iter()
                .filter(|s| s["curve"] == "G2" && s["helper"] == true)
                .collect();
            let start = root["address"].as_u64().unwrap();
            let size = root["symbol_bytes"].as_u64().unwrap();
            let out = Command::new("objdump")
                .args([
                    "-d",
                    "-C",
                    "--no-show-raw-insn",
                    &format!("--start-address=0x{start:x}"),
                    &format!("--stop-address=0x{:x}", start + size),
                    binary,
                ])
                .output()
                .unwrap();
            assert!(out.status.success());
            let body = String::from_utf8(out.stdout).unwrap();
            fs::write(r.join("split-G2-caller.asm.txt"), &body).unwrap();
            split_refs = helpers.len() == 2
                && helpers
                    .iter()
                    .all(|h| body.contains(&format!("<{}", h["symbol"].as_str().unwrap())));
        }
        records.extend(local);
    }
    let audit = json!({"records":records,"split_structure_observed":split_refs,"both_outlined_G2_helpers_referenced_from_caller":split_refs,"phase":"after native validation before any SDK capture","static_only":true,"cache_or_latency_causality_claim":false,"new_inline_annotations":false});
    fs::write(
        r.join("codegen-audit.json"),
        serde_json::to_vec_pretty(&audit).unwrap(),
    )
    .unwrap();
    println!("SpecializedG2helpersreferencedfromcaller={split_refs};staticcodeauditonly");
}
