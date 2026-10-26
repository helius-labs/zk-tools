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
    let root = Path::new("/tmp/zopro-deferred-dif32");
    assert_eq!(
        read(root.join("build-v3/result.json"))["status"],
        "complete"
    );
    assert_eq!(
        read(root.join("oracle-qualification.json"))["status"],
        "passed"
    );
    let field_tests = read(root.join("build/field-tests/tests.json"));
    assert_eq!(field_tests["passed_tests"].as_array().unwrap().len(), 33);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("build-input-pins-v3.json"))).unwrap();
    for name in [
        "accepted-probe",
        "target/release/probe",
        "analyze.rs",
        "timing.rs",
        "repair.json",
        "build-plan-v3.json",
        "build-v3/result.json",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let fingerprint = root.join("target/release/.fingerprint");
    let mut fresh = Vec::new();
    for e in fs::read_dir(fingerprint).unwrap() {
        let p = e.unwrap().path().join("lib-zopro_bn254.json");
        if p.exists() {
            let j = read(&p);
            if j["profile"].is_number() {
                fresh.push(p);
            }
        }
    }
    let mut fresh_libraries = Vec::new();
    for p in fresh {
        let name = p
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_prefix("zopro-bn254-")
            .unwrap();
        let lib = root.join(format!("target/release/deps/libzopro_bn254-{name}.rlib"));
        if lib.exists() {
            pins.insert(lib.to_str().unwrap().into(), hash(&lib));
            fresh_libraries.push(json!({"path":lib,"sha256":hash(&lib),"fingerprint":read(&p)}));
        }
    }
    assert!(!fresh_libraries.is_empty());
    fs::write(
        root.join("fresh-field-libraries.json"),
        serde_json::to_vec_pretty(&fresh_libraries).unwrap(),
    )
    .unwrap();
    let cases = [(1, 0), (256, 173), (16384, 8191)];
    let variants = [
        ("accepted", root.join("accepted-probe"), "reference"),
        ("fresh", root.join("target/release/probe"), "reference"),
        ("deferred", root.join("target/release/probe"), "deferred"),
    ];
    let mut steps = Vec::new();
    let mut order = Vec::new();
    for block in 0..31 {
        for ci in 0..3 {
            let (q, column) = cases[(ci + block) % 3];
            let mut vi = (0..3).map(|v| (v + ci + block) % 3).collect::<Vec<_>>();
            if block % 2 == 1 {
                vi.reverse();
            }
            for i in vi {
                let (label, binary, mode) = &variants[i];
                let name = format!("b{block:02}-q{q}-{label}");
                steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","2",binary,mode,q.to_string(),column.to_string(),"100000"],"timeout_seconds":60}));
                order.push(json!({"name":name,"q":q,"column":column,"block":block,"variant":label,"mode":mode}));
            }
        }
    }
    assert_eq!(steps.len(), 279);
    fs::write(
        root.join("timing-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("timing-order.json"),
        serde_json::to_vec_pretty(&order).unwrap(),
    )
    .unwrap();
    fs::write(root.join("timing-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":root.join("timing-input-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    println!(
        "Frozen279processes{}pins accepted{} fresh{}",
        pins.len(),
        hash(root.join("accepted-probe")),
        hash(root.join("target/release/probe"))
    );
}
