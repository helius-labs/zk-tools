use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
fn main() {
    let root = Path::new("/tmp/zopro-fused-product-overlap");
    assert_eq!(
        read(root.join("preflight-v2/result.json"))["status"],
        "complete"
    );
    let validation = read(root.join("preflight-v2/validate-inventory/stdout.log"));
    assert_eq!(validation["status"], "passed");
    assert_eq!(validation["distinct_gnark"], 18);
    let old = fs::read_to_string(root.join("preflight-v2/accepted-layout-observation/stdout.log"))
        .unwrap();
    let fresh =
        fs::read_to_string(root.join("preflight-v2/candidate-layout-observation/stdout.log"))
            .unwrap();
    assert_eq!(old, fresh);
    assert_eq!(old.trim(), "NATIVE_PLAN_BYTES=1728 ALIGN=32");
    assert_eq!(
        read(root.join("activation-audit-v2.json"))["status"],
        "passed"
    );
    let mut config = read(root.join("perf-template-v2.json"));
    assert_eq!(config["monotonic_clock"], true);
    let mut prior: BTreeSet<String> =
        serde_json::from_value(config["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4330);
    let inventory = read(root.join("preflight-proof-inventory-v2.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 36);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 4348);
    config["prior_proof_sha256"] = json!(prior);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("input-pins-v2.json"))).unwrap();
    for name in [
        "prepare-perf-v2.rs",
        "activation-audit-v2.json",
        "preflight-v2/validate-activation/stdout.log",
        "analyze.rs",
        "preflight-plan-v2.json",
        "preflight-v2/result.json",
        "preflight-v2/validate-inventory/stdout.log",
        "preflight-proof-inventory-v2.json",
        "preflight-v2/accepted-layout-observation/stdout.log",
        "preflight-v2/candidate-layout-observation/stdout.log",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    config["pins"] = json!(pins);
    fs::write(
        root.join("perf-plan-v2.json"),
        serde_json::to_vec_pretty(&config).unwrap(),
    )
    .unwrap();
    println!("Frozen{}perf pins4348prior;18preflightproofsverified; activation and Planlayout audits passed",pins.len());
}
