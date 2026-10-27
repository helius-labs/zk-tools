use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn main() {
    let root = Path::new("/tmp/zopro-g1-reverse-sdk");
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(root.join("performance/result.json")).unwrap())
            .unwrap()["status"],
        "complete"
    );
    let mut rows = Vec::new();
    for (name, path) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate-sdk",
        ),
        ("fresh", "/tmp/zopro-poseidon-fused-dot/native/fresh-sdk"),
        ("overlap", "/tmp/zopro-g1-reverse-sdk/native/overlap-sdk"),
    ] {
        let bytes = fs::read(path).unwrap();
        let output = Command::new("nm")
            .args(["-C", "-S", "--defined-only", path])
            .output()
            .unwrap();
        assert!(output.status.success());
        let text = String::from_utf8(output.stdout).unwrap();
        let selected = text
            .lines()
            .filter(|s| s.contains("stream_g1_impl"))
            .collect::<Vec<_>>();
        assert!(!selected.is_empty());
        fs::write(
            root.join(format!("sdk-symbols-{name}.txt")),
            selected.join("\n"),
        )
        .unwrap();
        for line in selected {
            let fields: Vec<_> = line.split_whitespace().collect();
            rows.push(json!({"variant":name,"binary":path,"binary_sha256":format!("{:x}",Sha256::digest(&bytes)),"address":u64::from_str_radix(fields[0],16).unwrap(),"bytes":u64::from_str_radix(fields[1],16).unwrap(),"symbol":fields[3..].join(" ")}));
        }
    }
    fs::write(root.join("sdk-codegen-audit.json"),serde_json::to_vec_pretty(&json!({"after_all_captures":true,"static_only":true,"dynamic_instructions_spills_and_cache_not_measured":true,"symbols":rows})).unwrap()).unwrap();
    println!(
        "Recorded{}actualSDKG1symbols after all captures;no rerun or causalclaim.",
        rows.len()
    );
}
