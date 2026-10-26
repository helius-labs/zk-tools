use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn write(p: impl AsRef<Path>, v: &impl serde::Serialize) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
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
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    pins.insert(p.to_str().unwrap().into(), hash(p));
}
fn tree(p: &Path, pins: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            tree(&p, pins)
        } else {
            assert!(p.is_file());
            add(&p, pins);
        }
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-instruction-census");
    let plan = read(root.join("extraction-plan.json"));
    let result = read(root.join("extraction/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["steps"].as_array().unwrap().len(), 18);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(root.join("extraction-pins.json")).unwrap()).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        let base = root.join("extraction").join(name);
        let receipt = read(base.join("exit.json"));
        assert_eq!(receipt["returncode"], 0);
        assert_eq!(receipt["success"], true);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(base.join(format!("{stream}.log"))),
                receipt[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
    }
    tree(&root.join("extraction"), &mut pins);
    for name in [
        "extraction-pins.json",
        "extraction-plan.json",
        "extraction.log",
        "prepare-analysis.rs",
        "source-provenance.json",
    ] {
        add(root.join(name), &mut pins);
    }
    let mut config = read(root.join("analysis-template.json"));
    config["pins"] = json!(pins);
    assert!(!root.join("analysis-config.json").exists());
    write(root.join("analysis-config.json"), &config);
    add(root.join("analysis-config.json"), &mut pins);
    write(root.join("analysis-pins.json"), &pins);
    write(
        root.join("analysis-plan.json"),
        &json!({"version":1,"source_pins":root.join("analysis-pins.json"),"environment":{},"steps":[{"kind":"command","name":"instruction-reader","argv":[root.join("target/release/xtask"),"instructions",root.join("analysis-config.json"),root.join("analysis")],"timeout_seconds":300}]}),
    );
    let mut bad = config.clone();
    bad["pins"][config["binaries"][0]["path"].as_str().unwrap().to_owned()] = json!("0".repeat(64));
    write(root.join("invalid-hash-config.json"), &bad);
    println!(
        "Verified 18 extraction receipts; froze {} pins for all nine captures",
        pins.len()
    );
}
