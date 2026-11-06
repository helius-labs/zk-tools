use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-callers176";
const N: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
const B: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("build-plan.json").exists());
    let review = read(format!("{N}/review.json"));
    assert_eq!(review["review_passed"], true);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{N}/pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h);
    }
    assert_eq!(
        hash(format!("{N}/libs/libzopro_core.rlib")),
        review["binaries_sha256"]["libs/libzopro_core.rlib"]
    );
    let source = read(format!("{N}/source-manifest.json"));
    let mut off_source = BTreeMap::new();
    for (p, h) in source["candidate"].as_object().unwrap() {
        let from = Path::new(N).join("source").join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let to = Path::new(R).join("OFF-source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        if p == "crates/zopro-core/src/msm/h_only_probe.rs" {
            let text = fs::read_to_string(&to).unwrap();
            assert_eq!(text.matches("if SELECTED\n").count(), 1);
            fs::write(&to, text.replace("if SELECTED\n", "if false\n")).unwrap();
        }
        off_source.insert(p.clone(), hash(&to));
        pins.insert(to.display().to_string(), hash(to));
    }
    assert_eq!(off_source.len(), 178);
    fs::create_dir_all(format!("{R}/OFF-libs")).unwrap();
    let receipt = read(format!("{N}/runs/build-core/command.json"));
    let mut core: Vec<String> = receipt["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|s| s.as_str().unwrap().to_owned())
        .collect();
    for a in &mut core {
        *a = a
            .replace(&format!("{N}/source/"), &format!("{R}/OFF-source/"))
            .replace(&format!("{N}/libs"), &format!("{R}/OFF-libs"));
    }
    let mut env = receipt["environment"].clone();
    env["CARGO_PKG_NAME"] = json!("zopro-core");
    env["CARGO_PKG_VERSION"] = json!("0.1.0");
    env["CARGO_MANIFEST_DIR"] = json!(format!("{R}/OFF-source/crates/zopro-core"));
    let driver = read(format!("{N}/runs/build-final-caller/command.json"));
    let old: Vec<String> = driver["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|s| s.as_str().unwrap().to_owned())
        .collect();
    let frozen = read(format!("{B}/frozen-candidate.json"));
    let mut steps = vec![
        json!({"kind":"command","name":"core-OFF","argv":core,"environment":env,"timeout_seconds":900}),
    ];
    for arm in ["ON", "OFF"] {
        let mut argv = old.clone();
        for a in &mut argv {
            if a == &format!("{N}/probe.rs") {
                *a = format!("{R}/driver.rs");
            } else if a == &format!("{N}/caller") {
                *a = format!("{R}/caller-{arm}");
            } else if arm == "OFF" {
                *a = a.replace(&format!("{N}/libs"), &format!("{R}/OFF-libs"));
            }
        }
        for name in ["libc", "sha2"] {
            let path = frozen["libraries"]
                .get(name)
                .and_then(Value::as_str)
                .map(str::to_owned)
                .unwrap_or_else(|| {
                    let matches: Vec<_> = frozen["input_pins"]
                        .as_object()
                        .unwrap()
                        .keys()
                        .filter(|p| p.contains(&format!("/lib{name}-")) && p.ends_with(".rlib"))
                        .cloned()
                        .collect();
                    assert_eq!(matches.len(), 1);
                    matches[0].clone()
                });
            argv.extend(["--extern".into(), format!("{name}={path}")]);
        }
        steps.push(json!({"kind":"command","name":format!("driver-{arm}"),"argv":argv,"environment":driver["environment"],"timeout_seconds":600}));
    }
    for p in [
        "design.md",
        "driver.rs",
        "perf_support.rs",
        "build.rs",
        "build",
    ] {
        let p = format!("{R}/{p}");
        pins.insert(p.clone(), hash(p));
    }
    for p in [
        "review.json",
        "codegen.json",
        "source-manifest.json",
        "libs/libzopro_core.rlib",
        "caller",
        "probe.rs",
    ] {
        let p = format!("{N}/{p}");
        pins.insert(p.clone(), hash(p));
    }
    save(
        format!("{R}/OFF-source-manifest.json"),
        &json!({"source_files":off_source,"only_change":"h_only_probe.rs Work condition SELECTED -> false","original_qualified_core_reused_for_ON_and_frozen":true}),
    );
    save(format!("{R}/build-pins.json"), &json!(pins));
    save(
        format!("{R}/build-plan.json"),
        &json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":steps}),
    );
    println!("Frozen3missing builds; qualifiedONcore+field unchanged; freshOFF only1token changed, same Work size; no timing");
}
