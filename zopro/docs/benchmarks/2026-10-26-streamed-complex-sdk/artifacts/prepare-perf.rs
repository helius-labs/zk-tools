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
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    assert_eq!(
        read(root.join("preflight/result.json"))["status"],
        "complete"
    );
    let verified = read(root.join("preflight/validate-inventory/stdout.log"));
    assert_eq!(verified["status"], "passed");
    assert_eq!(verified["distinct_gnark"], 18);
    let expected = "NATIVE_PLAN_BYTES=1728 ALIGN=32";
    for mode in ["accepted", "off", "on"] {
        assert_eq!(
            fs::read_to_string(
                root.join(format!("preflight/{mode}-layout-observation/stdout.log"))
            )
            .unwrap()
            .trim(),
            expected
        );
    }
    let mut config = read(root.join("perf-template.json"));
    let mut prior: BTreeSet<String> =
        serde_json::from_value(config["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4567);
    let inventory = read(root.join("preflight-proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 36);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("input-pins.json"))).unwrap();
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        assert!(pins
            .insert(p.to_owned(), h.as_str().unwrap().into())
            .is_none());
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 4585);
    config["prior_proof_sha256"] = json!(prior);
    for n in [
        "prepare-perf.rs",
        "analyze",
        "preflight-plan.json",
        "preflight/result.json",
        "preflight/validate-inventory/stdout.log",
        "preflight-proof-inventory.json",
        "preflight/accepted-layout-observation/stdout.log",
        "preflight/off-layout-observation/stdout.log",
        "preflight/on-layout-observation/stdout.log",
        "accepted-symbols.txt",
        "off-symbols.txt",
        "on-symbols.txt",
    ] {
        let p = root.join(n);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    config["pins"] = json!(pins);
    assert_eq!(config["monotonic_clock"], true);
    fs::write(
        root.join("perf-plan.json"),
        serde_json::to_vec_pretty(&config).unwrap(),
    )
    .unwrap();
    println!(
        "Frozen{}performancepins4585priorproofs;18preflightexports andall3Planlayouts verified",
        pins.len()
    );
}
