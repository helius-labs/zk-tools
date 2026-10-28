use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/rerandomized-glv-callers";
const BASE: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn extract<'a>(source: &'a str, start: &str) -> &'a str {
    let at = source.find(start).unwrap();
    let body = at + source[at..].find('{').unwrap();
    let mut n = 0;
    for (i, c) in source[body..].char_indices() {
        if c == '{' {
            n += 1;
        } else if c == '}' {
            n -= 1;
            if n == 0 {
                return &source[at..body + i + 1];
            }
        }
    }
    panic!("extract")
}
fn main() {
    let root = PathBuf::from(R);
    assert!(!root.join("build-plan.json").exists());
    let manifest=read(format!("{BASE}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let mut pins: BTreeMap<String, String> = BTreeMap::new();
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let path = Path::new(BASE).join(p);
        assert_eq!(hash(&path), h.as_str().unwrap());
        pins.insert(path.display().to_string(), h.as_str().unwrap().to_string());
    }
    let g = fs::read_to_string(format!("{BASE}/crates/zopro-core/src/groth16.rs")).unwrap();
    let rerandom = fs::read_to_string(format!(
        "{BASE}/crates/zopro-core/src/groth16_rerandomize.rs"
    ))
    .unwrap();
    let phi =
        fs::read_to_string(format!("{BASE}/crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
    let mut orig = String::from(
        "use super::*;\npub(super) struct FixedBase<C:CurveConfig>(pub(super) Vec<Affine<C>>);\n",
    );
    let implementation = extract(&g, "impl<C: CurveConfig> FixedBase<C>");
    orig.push_str("impl<C:CurveConfig> FixedBase<C> { const POINTS:usize=64*15;\n");
    for item in ["pub(crate) fn new(", "pub(crate) fn mul("] {
        orig.push_str(extract(implementation, item));
        orig.push('\n');
    }
    orig.push_str("}\n");
    let mut audit = vec![
        json!({"source":"groth16.rs","item":"FixedBase impl","sha256":format!("{:x}",Sha256::digest(implementation.as_bytes()))}),
    ];
    for (source, name) in [
        (&g, "blinding_digits"),
        (&rerandom, "odd_multiples"),
        (&rerandom, "add_digit"),
        (&rerandom, "pair_g1"),
        (&rerandom, "joint_g2"),
    ] {
        let function = extract(source, &format!("fn {name}"));
        audit.push(json!({"item":name,"verbatim_body_sha256":format!("{:x}",Sha256::digest(function.as_bytes()))}));
        orig.push_str("pub(super) ");
        orig.push_str(function);
        orig.push('\n');
    }
    fs::write(root.join("original.rs"), orig).unwrap();
    let a = phi.find("const C1:").unwrap();
    let b = phi.find("use zopro_bn254::{Fq, G2Affine").unwrap();
    let splitter = &phi[a..b];
    fs::write(root.join("split.rs"), splitter).unwrap();
    audit.push(json!({"item":"accepted bounded split helpers","sha256":format!("{:x}",Sha256::digest(splitter.as_bytes()))}));
    save(root.join("extraction-audit.json"), &json!(audit));
    fs::copy(
        format!("{BASE}/crates/zopro-bench/examples/support/perf.rs"),
        root.join("perf.rs"),
    )
    .unwrap();
    let deps = "/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";
    let native = "/home/ubuntu/src/zopro-perf-work/mixed-sdk/target-control/release/deps";
    let receipt =
        read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json");
    let mut libs = BTreeMap::new();
    for (name, file) in [
        ("ark_bn254", "libark_bn254-d9ceb28748a3bfd8.rlib"),
        ("ark_ff", "libark_ff-9216c709978bd3d2.rlib"),
    ] {
        libs.insert(name.to_string(), format!("{deps}/{file}"));
    }
    for name in ["zopro_bn254", "rayon", "libc", "serde_json"] {
        libs.insert(
            name.to_string(),
            receipt["libraries"][name].as_str().unwrap().to_string(),
        );
    }
    // Select the unique Arkworks EC dependency from its already-built directory.
    let ec: Vec<_> = fs::read_dir(deps)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("libark_ec-")
                && p.extension().is_some_and(|e| e == "rlib")
        })
        .collect();
    assert_eq!(ec.len(), 1);
    libs.insert("ark_ec".into(), ec[0].display().to_string());
    for dir in [deps, native] {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_file() && p.extension().is_some_and(|e| e == "rlib" || e == "so") {
                pins.insert(p.display().to_string(), hash(p));
            }
        }
    }
    for file in [
        "design.md",
        "setup.rs",
        "setup",
        "candidate.rs",
        "probe.rs",
        "original.rs",
        "split.rs",
        "perf.rs",
        "extraction-audit.json",
    ] {
        let p = root.join(file);
        pins.insert(p.display().to_string(), hash(p));
    }
    let mut common: Vec<String> = manifest["rustflags"]
        .as_str()
        .unwrap()
        .split_whitespace()
        .map(str::to_string)
        .collect();
    common.extend([
        "--edition=2021".into(),
        "-O".into(),
        root.join("probe.rs").display().to_string(),
        "-L".into(),
        format!("dependency={deps}"),
        "-L".into(),
        format!("dependency={native}"),
        "-C".into(),
        "lto=fat".into(),
        "-C".into(),
        "codegen-units=1".into(),
    ]);
    for (name, path) in &libs {
        common.extend(["--extern".into(), format!("{name}={path}")]);
    }
    let mut steps = vec![];
    for name in ["checked", "optimized", "probe"] {
        let mut argv = vec!["rustc".to_string()];
        argv.extend(common.clone());
        if name != "probe" {
            argv.push("--test".into());
        }
        if name == "checked" {
            argv.extend([
                "-C".into(),
                "overflow-checks=yes".into(),
                "-C".into(),
                "debug-assertions=yes".into(),
            ]);
        }
        argv.extend(["-o".into(), root.join(name).display().to_string()]);
        steps.push(json!({"kind":"command","name":format!("build-{name}"),"argv":argv,"environment":{},"timeout_seconds":600}));
        if name != "probe" {
            steps.push(json!({"kind":"command","name":format!("test-{name}"),"argv":[root.join(name),"--test-threads=1","--nocapture"],"environment":{},"timeout_seconds":600}));
        }
    }
    let compiler = Command::new("rustc").arg("-vV").output().unwrap();
    assert!(compiler.status.success());
    fs::write(root.join("compiler.txt"), compiler.stdout).unwrap();
    pins.insert(
        root.join("compiler.txt").display().to_string(),
        hash(root.join("compiler.txt")),
    );
    save(root.join("build-pins.json"), &json!(pins));
    save(
        root.join("build-plan.json"),
        &json!({"version":1,"source_pins":root.join("build-pins.json"),"steps":steps}),
    );
    println!(
        "Prepared exact extracted caller build; {} libraries, {} steps",
        libs.len(),
        steps.len()
    );
}
