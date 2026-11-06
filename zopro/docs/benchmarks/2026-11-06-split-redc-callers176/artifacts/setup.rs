use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-callers176";
const N: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-native176";
const O: &str = "/home/ubuntu/src/zopro-perf-work/g2-validated-batch176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(f: &str, v: &Value) {
    fs::write(format!("{R}/{f}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn replace(v: &Value) -> Value {
    match v {
        Value::String(s) => json!(s
            .replace(O, R)
            .replace("zopro_validated_g2", "zopro_split_redc_planes")),
        Value::Array(a) => json!(a.iter().map(replace).collect::<Vec<_>>()),
        Value::Object(o) => json!(o
            .iter()
            .map(|(k, v)| (k.clone(), replace(v)))
            .collect::<BTreeMap<_, _>>()),
        _ => v.clone(),
    }
}
fn prepare() {
    assert!(!Path::new(R).join("source").exists());
    assert_eq!(read(format!("{N}/qualification.json"))["status"], "pass");
    let native = read(format!("{N}/source-audit.json"));
    let cfg="#[cfg(all(feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
    let suffix = BTreeMap::from([
        (
            "crates/zopro-core/src/msm.rs",
            format!("\n{cfg}\npub mod validated_caller;\n"),
        ),
        (
            "crates/zopro-core/src/lib.rs",
            format!(
                "\n{cfg}\n#[doc(hidden)]pub use msm::validated_caller::{{G1Caller,G2Caller}};\n"
            ),
        ),
    ]);
    let mut pins = BTreeMap::new();
    let mut files = BTreeMap::new();
    let mut exact_native = 0;
    for (name, h) in native["hashes"].as_object().unwrap() {
        let from = Path::new(N).join("source").join(name);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let old = fs::read(&from).unwrap();
        let mut bytes = old.clone();
        if let Some(s) = suffix.get(name.as_str()) {
            bytes.extend(s.as_bytes());
        } else {
            exact_native += 1;
        }
        let dest = Path::new(R).join("source").join(name);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::write(&dest, &bytes).unwrap();
        if let Some(s) = suffix.get(name.as_str()) {
            bytes.truncate(bytes.len() - s.len());
        }
        assert_eq!(bytes, old);
        pins.insert(dest.display().to_string(), hash(&dest));
        files.insert(name.clone(), hash(&dest));
    }
    assert_eq!(exact_native, 175);
    let child = format!("{R}/source/crates/zopro-core/src/msm/validated_caller.rs");
    fs::copy(
        format!("{O}/source/crates/zopro-core/src/msm/validated_caller.rs"),
        &child,
    )
    .unwrap();
    let text = fs::read_to_string(&child).unwrap();
    assert!(text.contains("super::g2_borrowed::apply"));
    assert!(!text.contains("g2_validated"));
    pins.insert(child.clone(), hash(&child));
    files.insert(
        "crates/zopro-core/src/msm/validated_caller.rs".into(),
        hash(&child),
    );
    let driver = fs::read_to_string(format!("{O}/driver.rs")).unwrap();
    assert_eq!(driver.matches("zopro_validated_g2").count(), 1);
    fs::write(
        format!("{R}/driver.rs"),
        driver.replace("zopro_validated_g2", "zopro_split_redc_planes"),
    )
    .unwrap();
    let mut fields = BTreeMap::new();
    for role in ["OFF", "ON"] {
        let f = read(format!("{N}/field-{role}.json"));
        assert_eq!(f["sha256"], hash(f["frozen"].as_str().unwrap()));
        pins.insert(
            f["frozen"].as_str().unwrap().into(),
            f["sha256"].as_str().unwrap().into(),
        );
        fields.insert(role, f);
        fs::create_dir_all(format!("{R}/libs/{role}")).unwrap();
    }
    let template = replace(&read(format!("{O}/build-plan.json")));
    let mut steps = vec![];
    for role in ["OFF", "ON"] {
        for base in ["core", "build-core-test", "core-test", "driver"] {
            let mut s = template["steps"]
                .as_array()
                .unwrap()
                .iter()
                .find(|s| s["name"] == format!("{base}-{role}"))
                .unwrap()
                .clone();
            if base == "core-test" && role == "ON" {
                s["required_tests"] = json!([
                    "msm::validated_caller::validation_tests::complete_signed_tails_and_reuse"
                ]);
                s["unfiltered"] = json!(true);
            }
            for arg in s["argv"].as_array_mut().unwrap() {
                let a = arg.as_str().unwrap();
                if a.starts_with("zopro_bn254=") {
                    *arg = json!(format!(
                        "zopro_bn254={}",
                        fields[role]["frozen"].as_str().unwrap()
                    ));
                }
            }
            steps.push(s);
            if base == "core" || base == "driver" {
                steps.push(json!({"kind":"command","name":format!("freeze-{base}-{role}"),"argv":[format!("{R}/setup"),"freeze",base,role],"timeout_seconds":120}));
            }
        }
    }
    for s in &steps {
        for arg in s["argv"].as_array().unwrap() {
            let a = arg.as_str().unwrap();
            let path = a
                .split_once('=')
                .filter(|(k, _)| {
                    matches!(
                        *k,
                        "zopro_bn254"
                            | "zopro_core"
                            | "libc"
                            | "rayon"
                            | "serde_json"
                            | "sha2"
                            | "ark_ref"
                            | "ark_ec"
                            | "ark_ff"
                            | "ark_std"
                    )
                })
                .map(|(_, p)| p)
                .unwrap_or(a);
            if Path::new(path).is_file() {
                pins.insert(path.into(), hash(path));
            }
        }
    }
    for f in ["setup.rs", "setup", "design.md", "driver.rs", "perf.rs"] {
        let p = format!("{R}/{f}");
        pins.insert(p.clone(), hash(p));
    }
    for f in [
        "qualification.json",
        "source-audit.json",
        "field-OFF.json",
        "field-ON.json",
    ] {
        let p = format!("{N}/{f}");
        pins.insert(p.clone(), hash(p));
    }
    save(
        "source-audit.json",
        &json!({"source_files":178,"native_source_originals_exact":175,"all_native177_recoverable":true,"production176_recovery":"inherit native three inverse edits, then strip two core diagnostic suffixes; all other core runtime source byte-exact","hashes":files,"suffixes":suffix,"adapter_sha256":hash(&child),"driver_only_cfg_edit":true,"fields":fields,"no_rejected_core_kernel_inherited":true}),
    );
    save("build-pins.json", &json!(pins));
    save(
        "build-plan.json",
        &json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"environment":{},"steps":steps}),
    );
    println!(
        "prepared178sources/175nativeexact/12commands/{}pins; no field work or timings repeated",
        pins.len()
    );
}
fn freeze(kind: &str, role: &str) {
    assert!(["OFF", "ON"].contains(&role));
    let src = match kind {
        "core" => format!("{R}/libs/{role}/libzopro_core.rlib"),
        "driver" => format!("{R}/caller-{role}"),
        _ => panic!("freeze core|driver"),
    };
    let h = hash(&src);
    let dir = format!("{R}/frozen/{role}");
    fs::create_dir_all(&dir).unwrap();
    let dest = format!(
        "{dir}/{}-{h}{}",
        if kind == "core" {
            "libzopro_core"
        } else {
            "caller"
        },
        if kind == "core" { ".rlib" } else { "" }
    );
    assert!(!Path::new(&dest).exists());
    fs::copy(&src, &dest).unwrap();
    assert_eq!(hash(&dest), h);
    save(
        &format!("{kind}-{role}.json"),
        &json!({"role":role,"kind":kind,"source":src,"frozen":dest,"sha256":h}),
    );
    println!("frozen {kind}/{role} {h}");
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("freeze") => freeze(&a[2], &a[3]),
        _ => panic!("prepare|freeze"),
    }
}
