use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    let z = Path::new(Z);
    let parent = Path::new("/home/ubuntu/src/zopro-perf-work/g2-balanced175");
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    let m = read(z.join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    assert_eq!(m["native_field_tests_inherited"], 33);
    let a = read(r.join("source-audit.json"));
    let mut exact = 0;
    let mut fields = 0;
    let mut pins = BTreeMap::new();
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let original = fs::read(z.join(p)).unwrap();
        assert_eq!(hash(z.join(p)), h.as_str().unwrap());
        let path = r.join("source").join(p);
        let bytes = fs::read(&path).unwrap();
        let restored = if let Some(e) = a["edits"].get(p) {
            let mut s = String::from_utf8(bytes.clone()).unwrap();
            if let Some(suffix) = e["suffix"].as_str() {
                s = s.strip_suffix(suffix).unwrap().to_string();
            }
            if let Some(before) = e["before"].as_str() {
                let after = e["after"].as_str().unwrap();
                assert_eq!(s.matches(after).count(), 1);
                s = s.replacen(after, before, 1);
            }
            assert_eq!(bytes, fs::read(parent.join("source").join(p)).unwrap());
            s.into_bytes()
        } else {
            bytes.clone()
        };
        assert_eq!(restored, original, "reverse{p}");
        if bytes == original {
            exact += 1;
        }
        if p.starts_with("crates/zopro-bn254/") {
            assert_eq!(bytes, original);
            fields += 1;
        }
        pins.insert(p.clone(), hash(path));
    }
    assert_eq!(exact, 173);
    assert_eq!(fields, 17);
    assert_eq!(a["edits"].as_object().unwrap().len(), 2);
    let child = a["child"].as_str().unwrap();
    assert_eq!(
        fs::read(r.join("source").join(child)).unwrap(),
        fs::read(parent.join("source").join(child)).unwrap()
    );
    pins.insert(child.into(), hash(r.join("source").join(child)));
    assert_eq!(pins.len(), 176);
    assert_eq!(
        fs::read(r.join("source/crates/zopro-core/src/lib.rs")).unwrap(),
        fs::read(z.join("crates/zopro-core/src/lib.rs")).unwrap()
    );
    assert_eq!(
        fs::read(r.join("source/crates/zopro-core/src/msm_affine/workspace.rs")).unwrap(),
        fs::read(z.join("crates/zopro-core/src/msm_affine/workspace.rs")).unwrap()
    );
    let mut code = BTreeMap::new();
    for label in ["control", "candidate"] {
        for (suite, n) in [("core", 127), ("prover", 8)] {
            let t = read(r.join(format!("native/{label}-{suite}/tests.json")));
            assert_eq!(t["unfiltered"], true);
            assert_eq!(t["passed_tests"].as_array().unwrap().len(), n);
        }
        let p = r.join(format!("native/{label}-sdk"));
        let receipt = read(r.join(format!("native/{label}/artifacts.json")));
        assert_eq!(hash(&p), receipt["binary_sha256"]);
        assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
        for (p, h) in receipt["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
        code.insert(label, json!({"binary":p,"sha256":hash(&p)}));
    }
    for (suite, n) in [("core", 101), ("prover", 8)] {
        let t = read(r.join(format!("native/portable-{suite}/tests.json")));
        assert_eq!(t["unfiltered"], true);
        assert_eq!(t["passed_tests"].as_array().unwrap().len(), n);
    }
    let flags = fs::read_to_string(r.join("native-flags.txt")).unwrap();
    assert_eq!(flags, m["rustflags"]);
    assert_eq!(
        fs::read_to_string(r.join("candidate-flags.txt")).unwrap(),
        format!("{flags} --cfg zopro_g2_balanced")
    );
    assert!(!flags.contains("zopro_g2_queue_probe"));
    fs::write(r.join("final-source-review.json"),serde_json::to_vec_pretty(&json!({"reverse175_exact":true,"source_files":176,"originals_exact":173,"field_files_exact":17,"field_tests_inherited":33,"balanced_child_and_two_parent_edits_exact_to_qualified_caller":true,"private_query_probe_export_absent":true,"heap_forecast_unchanged":true,"native_tests_each":{"core":127,"prover":8},"portable_tests":{"core":101,"prover":8},"private_hashes":pins,"code":code})).unwrap()).unwrap();
    println!("Source176/173exact/reverse175;field17exact;qualifiedgeometryexact;probeexportremoved;native127/8,portable101/8;SDKreceiptsmatch");
}
