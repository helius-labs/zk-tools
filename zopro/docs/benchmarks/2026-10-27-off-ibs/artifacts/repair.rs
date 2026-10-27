use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/off-ibs";
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn save(p: impl AsRef<Path>, v: &Value) { fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap() }
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap(); let mut h = Sha256::new(); let mut b = [0; 65536];
    loop { let n = f.read(&mut b).unwrap(); if n == 0 { break; } h.update(&b[..n]); }
    format!("{:x}", h.finalize())
}
fn replace_once(s: &mut String, from: &str, to: &str) { assert_eq!(s.matches(from).count(), 1, "{from}"); *s = s.replacen(from, to, 1); }
fn patch() {
    let r = Path::new(R);
    let mut s = fs::read_to_string(r.join("control-closeout.rs")).unwrap();
    replace_once(&mut s, "fn finish(){", "fn decoded(name:&str)->std::path::PathBuf { let a=Path::new(R).join(\"runs\").join(name).join(\"stdout.log\"); if a.exists(){a}else{Path::new(R).join(\"decode-runs\").join(name).join(\"stdout.log\")} }\nfn finish(){");
    replace_once(&mut s, "assert_eq!(read(r.join(\"runs/result.json\"))[\"status\"],\"complete\");", "assert_eq!(read(r.join(\"decode-runs/result.json\"))[\"status\"],\"complete\"); assert_eq!(read(r.join(\"runs/completed.json\")).as_array().unwrap().len(),14); assert!(read(r.join(\"runs/failure.json\"))[\"error\"].as_str().unwrap().contains(\"101\")); for(f,h)in read(r.join(\"decode-pins.json\")).as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());}");
    replace_once(&mut s, "r.join(\"runs\").join(format!(\"script-{name}\")).join(\"stdout.log\"),r.join(\"runs\").join(format!(\"raw-{name}\")).join(\"stdout.log\")", "decoded(&format!(\"script-{name}\")),decoded(&format!(\"raw-{name}\"))");
    replace_once(&mut s, "\"limits\":\"Three descriptive paired blocks under IBS; no unprofiled latency qualification or causal wall-time/bandwidth interpretation\"", "\"limits\":\"Three descriptive paired blocks under IBS; no unprofiled latency qualification or causal wall-time/bandwidth interpretation\",\"analysis_repair\":\"Original workflow stopped at first analysis because perf omits IbsOpRip when RipInvalid is set. New decoder allows an omitted RIP only with that flag and joins actual perf IP. Original helpers/pins/failure retained; new offline-only plan continues remaining decode; no capture or Gnark repeated.\"");
    replace_once(&mut s, "[all observations](artifacts/analysis.json).", "[all observations](artifacts/analysis.json). The first decoder stopped because perf omits IbsOpRip when RipInvalid is set. A new offline decoder permits that omission only with the invalid flag, records it separately and still joins actual perf IP. Its targeted regression test passed; original helpers, pins and failure are retained. The separate continuation plan performs only remaining offline decoding, repeating no capture or proof verification.");
    fs::write(r.join("control-closeout.rs"), s).unwrap();
}
fn plan() {
    let r = Path::new(R); assert!(!r.join("decode-plan.json").exists());
    let test = fs::read_to_string(r.join("invalid-rip-test.log")).unwrap(); assert!(test.contains("test result: ok. 1 passed; 0 failed"));
    let old = read(r.join("run-plan.json")); let completed = read(r.join("runs/completed.json")); assert_eq!(completed.as_array().unwrap().len(), 14);
    let mut pins: BTreeMap<String,String> = serde_json::from_value(read(r.join("pins.json"))).unwrap();
    for (p,h) in &pins { assert_eq!(hash(p), *h); }
    for name in ["decoder-v3.rs","decoder-v3","invalid-rip-test.log","control-closeout.rs","controller-closeout","repair.rs","repair","runs/completed.json","runs/failure.json","runs/analyze-b0-control/stderr.log"] { let p=r.join(name); pins.insert(p.display().to_string(),hash(&p)); }
    let mut steps=Vec::new();
    for step in old["steps"].as_array().unwrap().iter().skip(14) {
        let mut step=step.clone(); let name=step["name"].as_str().unwrap().to_owned();
        assert!(name.starts_with("analyze-")||name.starts_with("script-")||name.starts_with("raw-"));
        if name.starts_with("analyze-") {
            step["argv"][0]=json!(r.join("decoder-v3"));
            if name!="analyze-b0-control" {
                for i in [1,2] { step["argv"][i]=json!(step["argv"][i].as_str().unwrap().replace("/runs/","/decode-runs/")); }
            }
        }
        steps.push(step);
    }
    assert_eq!(steps.len(),16);
    save(r.join("decode-pins.json"),&json!(pins));
    save(r.join("decode-plan.json"),&json!({"version":1,"source_pins":r.join("decode-pins.json"),"steps":steps}));
    println!("Offline-only continuation:16steps; no capture or proof verification repeated");
}
fn main() { match std::env::args().nth(1).as_deref() { Some("patch")=>patch(),Some("plan")=>plan(),_=>panic!("patch|plan") } }
