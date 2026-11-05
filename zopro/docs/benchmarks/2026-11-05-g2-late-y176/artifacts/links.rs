use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-late-y176";
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn save(n: &str, v: &Value) { fs::write(format!("{R}/{n}"), serde_json::to_vec_pretty(v).unwrap()).unwrap(); }
fn main() {
    assert!(!Path::new(&format!("{R}/link-plan.json")).exists());
    assert_eq!(read(format!("{R}/native/result.json"))["status"], "complete");
    for arm in ["off", "on"] {
        let t = read(format!("{R}/native/core-{arm}/tests.json"));
        assert_eq!(t["unfiltered"], true);
        assert_eq!(t["passed_tests"].as_array().unwrap().len(), 130);
        let stdout = fs::read_to_string(format!("{R}/native/core-{arm}/stdout.log")).unwrap();
        assert!(stdout.contains("130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
        for step in [format!("core-{arm}"), format!("libraries-{arm}"), format!("freeze-{arm}")] {
            assert_eq!(read(format!("{R}/native/{step}/exit.json"))["success"], true);
        }
    }
    let mut pins: BTreeMap<String, String> = serde_json::from_value(read(format!("{R}/build-pins.json"))).unwrap();
    for (p, h) in &pins { assert_eq!(hash(p), *h, "{p}"); }
    let mut steps = Vec::new();
    for arm in ["off", "on"] {
        let info = read(format!("{R}/artifacts-{arm}.json"));
        assert_eq!(info["rustflags"].as_str().unwrap().contains("zopro_g2_late_y"), arm == "on");
        for (p, h) in info["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap()); pins.insert(p.clone(), h.as_str().unwrap().into());
        }
        let dep = Path::new(info["libraries"]["zopro_core"].as_str().unwrap()).parent().unwrap();
        let mut argv = vec!["/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc".into(), "--edition=2021".into(), "-O".into(), "-C".into(), "target-cpu=native".into(), "-C".into(), "lto=fat".into(), "-C".into(), "codegen-units=1".into(), "-L".into(), format!("dependency={}", dep.display())];
        for (name, p) in info["libraries"].as_object().unwrap() { argv.extend(["--extern".into(), format!("{name}={}", p.as_str().unwrap())]); }
        argv.extend([format!("{R}/probe.rs"), "-o".into(), format!("{R}/{arm}-probe")]);
        steps.push(json!({"kind":"command","name":format!("link-{arm}"),"argv":argv,"environment":{},"timeout_seconds":600}));
    }
    for name in ["links.rs", "link-tool", "probe.rs", "perf.rs", "artifacts-off.json", "artifacts-on.json", "native/result.json"] {
        let p = format!("{R}/{name}"); pins.insert(p.clone(), hash(p));
    }
    save("link-pins.json", &json!(pins));
    save("link-plan.json", &json!({"version":1,"source_pins":format!("{R}/link-pins.json"),"steps":steps}));
    println!("Prepared two direct optimized links;130 unfiltered core tests each,6 successful native steps, coherent library/source pins checked");
}
