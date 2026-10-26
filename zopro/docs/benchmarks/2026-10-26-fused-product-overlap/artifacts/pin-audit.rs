use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
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
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let mut reports = Vec::new();
    for name in [
        "build-input-pins.json",
        "input-pins.json",
        "build-input-pins-v2.json",
        "input-pins-v2.json",
        "perf-plan-v2.json",
    ] {
        let mut v = read(r.join(name));
        if name.starts_with("perf-") {
            v = v["pins"].clone();
        }
        let pins: BTreeMap<String, String> = serde_json::from_value(v).unwrap();
        let mut mismatches = Vec::new();
        for (p, h) in &pins {
            let got = hash(p);
            if &got != h {
                mismatches.push(json!({"path":p,"expected":h,"actual":got}));
            }
        }
        reports.push(json!({"config":name,"pins":pins.len(),"mismatches":mismatches}));
    }
    fs::write(
        r.join("pin-diff-audit.json"),
        serde_json::to_vec_pretty(&reports).unwrap(),
    )
    .unwrap();
    println!("{}", serde_json::to_string_pretty(&reports).unwrap());
}
