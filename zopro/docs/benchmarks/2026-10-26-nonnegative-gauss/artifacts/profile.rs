use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-nonnegative-gauss");
    assert_eq!(read(r.join("timing/result.json"))["status"], "complete");
    assert_eq!(read(r.join("component-result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("timing-input-pins.json"))).unwrap();
    for name in [
        "profile.rs",
        "analyze.rs",
        "analyze",
        "component-result.json",
        "timing/result.json",
        "timing-plan.json",
        "timing-order.json",
    ] {
        let p = r.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    fs::write(
        r.join("profile-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let variants = [
        ("accepted", r.join("accepted-probe"), "old"),
        ("fresh", r.join("target/release/probe"), "old"),
        ("streamed", r.join("target/release/probe"), "streamed"),
        ("gauss", r.join("target/release/probe"), "gauss"),
    ];
    let mut steps = Vec::new();
    for (label, binary, mode) in variants {
        let dir = r.join("profiles-data").join(label);
        fs::create_dir_all(&dir).unwrap();
        let data = dir.join("perf.data");
        steps.push(json!({"kind":"command","name":format!("capture-{label}"),"argv":["sudo","-n","perf","record","--no-buildid-cache","-e","cycles:u","-F","999","-o",data,"--","taskset","-c","2",binary,mode,"affine-expression","20000000"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{label}"),"argv":["sudo","-n","perf","script","--header","-i",data,"-F","comm,pid,tid,time,event,period,ip,sym,dso"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{label}"),"argv":["sudo","-n","perf","script","-D","-i",data],"timeout_seconds":120}));
    }
    fs::write(r.join("profile-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("profile-input-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    println!("Frozen4after-latencyprofiles{}pins; no reroll", pins.len());
}
