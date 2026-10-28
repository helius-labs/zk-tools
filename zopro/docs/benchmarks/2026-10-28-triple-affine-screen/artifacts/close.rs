use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/triple-affine-screen";
const DEST: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-triple-affine-screen";

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn write(path: impl AsRef<Path>, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}
fn hash(path: impl AsRef<Path>) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut digest = Sha256::new();
    let mut bytes = [0; 65536];
    loop {
        let n = file.read(&mut bytes).unwrap();
        if n == 0 { break; }
        digest.update(&bytes[..n]);
    }
    format!("{:x}", digest.finalize())
}
fn main() {
    let root = Path::new(ROOT);
    let dest = Path::new(DEST);
    assert!(!dest.join("artifact-index.json").exists());
    let pins = read(root.join("pins.json"));
    for (path, expected) in pins.as_object().unwrap() {
        assert_eq!(hash(path), expected.as_str().unwrap(), "{path}");
    }
    let analysis = read(root.join("analysis.json"));
    assert_eq!(analysis["total_triple_attempts"], 11264);
    assert_eq!(analysis["new_tests"], 3);
    assert_eq!(analysis["production_changed"], false);
    assert_eq!(analysis["proof_ledger_unchanged"], 12524);
    assert_eq!(analysis["fields"].as_array().unwrap().len(), 2);
    for field in analysis["fields"].as_array().unwrap() {
        assert_eq!(field["ordinary_results_checked"], 5254);
        assert_eq!(field["need_fallback"]["infinity"], 224);
        assert_eq!(field["need_fallback"]["d_zero"], 110);
        assert_eq!(field["need_fallback"]["H_zero"], 44);
    }
    assert!(fs::read_to_string(root.join("tests.log")).unwrap()
        .contains("test result: ok. 3 passed; 0 failed"));
    write(root.join("closeout.json"), &json!({
        "status": "exact model complete; no native or prover measurement",
        "all_original_pins_match": true,
        "pinned_paths": pins.as_object().unwrap().len(),
        "accepted_production_files_unchanged": 172,
        "total_BN254_triple_attempts": 11264,
        "ordinary_results_checked": 10508,
        "fallback_requests_checked": 756,
        "additional_batch_triples": 496,
        "new_tests_passed": 3,
        "failed_build_sources_and_logs_retained": true,
        "native_timing": false,
        "SDK_requests": 0,
        "proof_ledger": 12524,
        "adopted": false,
        "goal_achieved": false,
        "excluded_local_files": ["executables", "coz-author.pdf"],
        "next_action": "small actual caller; compare against two binary layers including fallback and data movement"
    }));
    fs::create_dir_all(dest.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if !path.is_file() { continue; }
        let name = path.file_name().unwrap().to_str().unwrap();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !["rs", "md", "json", "log"].contains(&ext) || name == "close.log" { continue; }
        let relative = format!("artifacts/{name}");
        fs::copy(&path, dest.join(&relative)).unwrap();
        index.insert(relative, hash(&path));
    }
    index.insert("README.md".into(), hash(dest.join("README.md")));
    let sums = index.iter().map(|(path, sum)| format!("{sum}  {path}\n")).collect::<String>();
    fs::write(dest.join("SHA256SUMS"), sums).unwrap();
    write(dest.join("artifact-index.json"), &json!({"sha256": index}));
    println!("Closed exact study: {} pinned paths match, {} archived hashes; no tests repeated", pins.as_object().unwrap().len(), index.len());
}
