use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/radix-groups175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}

include!("code-audit-v2.rs");
fn main() {
    let pins = read(r("build-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let completed = read(r("native/completed.json"));
    assert_eq!(completed.as_array().unwrap().len(), 10);
    let failure = read(r("native/failure.json"));
    assert_eq!(failure["completed"], completed);
    assert!(failure["error"].as_str().unwrap().contains("code-audit"));
    for label in ["control", "candidate"] {
        let tests = read(r(format!("native/{label}-core/tests.json")));
        assert_eq!(tests["passed_tests"].as_array().unwrap().len(), 128);
        assert_eq!(tests["unfiltered"], true);
        assert!(tests["passed_tests"].as_array().unwrap().contains(&json!(
            "msm_affine::grouping::radix::tests::compiled_policy_dispatch_matches_cfg"
        )));
        for (p, h) in read(r(format!("frozen-{label}.json")))["input_pins"]
            .as_object()
            .unwrap()
        {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
    }
    inspect();
}
