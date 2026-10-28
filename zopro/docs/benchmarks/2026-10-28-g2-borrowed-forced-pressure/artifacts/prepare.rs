use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-forced-pressure";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn setup() {
    assert!(!r("resources.rs").exists());
    assert_eq!(
        read(format!("{P}/analysis.json"))["independent_proof_count"],
        14551
    );
    let mut s = fs::read_to_string(format!("{P}/resources.rs")).unwrap();
    once(&mut s, P, R);
    s = s.replace("zopro-borrow-q-", "zopro-borrow-forced-");
    once(
        &mut s,
        "ballast.resize(32 << 20, 0u8)",
        "ballast.resize(112 << 20, 0u8)",
    );
    once(
        &mut s,
        "Duration::from_secs(100)",
        "Duration::from_secs(300)",
    );
    once(&mut s, "MemoryHigh=134217728", "MemoryHigh=100663296");
    once(&mut s, "!= \"134217728\"", "!= \"100663296\"");
    once(
        &mut s,
        "\"SDK_requests\":36,\"core_requests\":0,\"independent_exports_planned\":12",
        "\"SDK_requests\":20,\"core_requests\":0,\"independent_exports_planned\":8",
    );
    fs::write(r("resources.rs"), s).unwrap();
    let cases = read(format!("{P}/resource-cases.json"));
    let cases: Vec<_> = cases
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["kind"] == "warm")
        .cloned()
        .collect();
    assert_eq!(cases.len(), 4);
    save(r("resource-cases.json"), &json!(cases));
    fs::copy(format!("{P}/inputs.json"), r("inputs.json")).unwrap();
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{P}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 14462);
    for (f, h) in read(format!("{P}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        if f.ends_with(".proof") {
            assert_eq!(hash(f), h.as_str().unwrap());
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 14551);
    save(r("prior-proof-sha256.json"), &json!(prior));
    println!("Prepared4forced-pressurecases;96high/192max/112ballast;20SDK/8exports;no SDKchange");
}
fn plan() {
    assert!(!r("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{P}/pins.json"))).unwrap();
    for (f, h) in &pins {
        assert_eq!(hash(f), *h)
    }
    for n in [
        "design.md",
        "prepare.rs",
        "prepare",
        "resources.rs",
        "resources",
        "resource-cases.json",
        "inputs.json",
        "prior-proof-sha256.json",
        "finish.rs",
        "finish",
        "close.rs",
        "close",
    ] {
        let p = r(n);
        pins.insert(p.display().to_string(), hash(p));
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":[{"kind":"command","name":"forced-pressure","argv":[r("resources"),"run"],"timeout_seconds":2400}]}),
    );
    println!("Frozenforcedpressureworkflow; originalqualifiedinputs rechecked");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("setup") => setup(),
        Some("plan") => plan(),
        _ => panic!("setup/plan"),
    }
}
