use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk";
const C: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
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
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn setup() {
    assert!(!r("build-plan.json").exists());
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let mut pins = BTreeMap::new();
    assert_eq!(read(format!("{P}/analysis.json"))["gate_passed"], false);
    for f in m["source_files_sha256"]
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .chain([
            "crates/zopro-core/src/msm/g2_borrowed.rs",
            "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
        ])
    {
        let from = Path::new(C).join("source").join(f);
        let to = r("source").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        pin(&mut pins, &from);
        pin(&mut pins, to);
    }
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    for f in ["warm.rs", "concurrent.rs"] {
        let p = Path::new(Z)
            .join("docs/benchmarks/2026-10-27-clean-fft-qualification/artifacts")
            .join(f);
        fs::copy(&p, r(f)).unwrap();
        pin(&mut pins, p);
        pin(&mut pins, r(f));
    }
    let mut resource = fs::read_to_string(format!(
        "{Z}/docs/benchmarks/2026-10-27-clean-fft-qualification/artifacts/resources.rs"
    ))
    .unwrap();
    resource = resource
        .replace("/tmp/zopro-clean-fft-qualification", R)
        .replace("zopro-cleafft-q-", "zopro-borrow-q-");
    once(
        &mut resource,
        "\"SDK_requests\":36,\"core_requests\":4,\"independent_exports_planned\":16",
        "\"SDK_requests\":36,\"core_requests\":0,\"independent_exports_planned\":12",
    );
    fs::write(r("resources.rs"), resource).unwrap();
    let mut steps = vec![];
    let mut env = read(format!(
        "{Z}/docs/benchmarks/2026-10-27-clean-fft-qualification/artifacts/portable-plan.json"
    ))["environment"]
        .clone();
    env["CARGO_TARGET_DIR"] = json!(r("target-portable"));
    env["RUSTFLAGS"] = json!(format!(
        "{} --cfg zopro_g2_borrow_queue",
        env["RUSTFLAGS"].as_str().unwrap()
    ));
    env["ZOPRO_BASIS_KEY_DIR"] =
        json!("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/images");
    env["ZOPRO_CPU_LAGRANGE_DIR"] =
        json!("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache");
    save(r("portable-environment.json"), &env);
    for (name, package, extra) in [
        ("portable-field", "zopro-bn254", vec!["--lib"]),
        ("portable-core", "zopro-core", vec!["--lib"]),
        (
            "portable-prover",
            "zopro-prover",
            vec!["--lib", "--test", "cubic"],
        ),
    ] {
        let mut argv: Vec<String> = vec!["cargo", "test", "--manifest-path"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        argv.extend([
            r("source/Cargo.toml").display().to_string(),
            "--locked".into(),
            "--offline".into(),
            "--release".into(),
            "-p".into(),
            package.into(),
        ]);
        argv.extend(extra.into_iter().map(str::to_owned));
        argv.extend(["--".into(), "--test-threads=1".into()]);
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":env,"unfiltered":true,"timeout_seconds":1800}));
    }
    steps.push(json!({"kind":"command","name":"portable-libraries","argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":"freeze-portable","argv":[r("prepare"),"freeze"],"environment":env,"timeout_seconds":180}));
    steps.push(json!({"kind":"link-sdk","name":"portable","artifacts":r("portable-cargo.jsonl"),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"rustc_flags":["-O","-C","target-cpu=x86-64","-C","lto=fat","-C","codegen-units=1"],"timeout_seconds":600}));
    for variant in ["accepted", "on"] {
        let p = format!("{P}/libraries-{variant}.jsonl");
        pin(&mut pins, &p);
        for (driver, source) in [("warm", "warm.rs"), ("concurrent", "concurrent.rs")] {
            steps.push(json!({"kind":"link-sdk","name":format!("{variant}-{driver}"),"artifacts":p,"driver":r(source),"timeout_seconds":600}));
        }
    }
    for f in [
        "design.md",
        "prepare.rs",
        "prepare",
        "resources.rs",
        "portable-environment.json",
    ] {
        pin(&mut pins, r(f));
    }
    for f in ["analysis.json", "pins.json", "source-audit.json"] {
        pin(&mut pins, format!("{P}/{f}"));
    }
    for variant in ["accepted", "on"] {
        let receipt = if variant == "accepted" {
            read(format!("{P}/accepted-artifacts.json"))
        } else {
            read(format!("{P}/parent-artifacts-on.json"))
        };
        for (f, h) in receipt["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            pin(&mut pins, f);
        }
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared10 qualification build/test/linksteps; exact174private source files; native tests inherited");
}
fn freeze() {
    let env = read(r("portable-environment.json"));
    let flags = env["RUSTFLAGS"].as_str().unwrap();
    assert_eq!(std::env::var("RUSTFLAGS").unwrap(), flags);
    let mut all = BTreeMap::<String, BTreeSet<String>>::new();
    let mut pins = BTreeMap::new();
    let root = r("frozen-portable");
    fs::create_dir_all(&root).unwrap();
    for l in fs::read_to_string(r("build/portable-libraries/stdout.log"))
        .unwrap()
        .lines()
    {
        let a: Value = serde_json::from_str(l).unwrap();
        if a["reason"] != "compiler-artifact" {
            continue;
        }
        let name = a["target"]["name"].as_str().unwrap();
        let files = a["filenames"].as_array().unwrap();
        let metas: Vec<_> = files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rmeta") && p.contains("/deps/"))
            .collect();
        for f in files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rlib") || p.ends_with(".so"))
        {
            let p = if f.ends_with(".rlib") && !f.contains("/deps/") && metas.len() == 1 {
                Path::new(metas[0]).with_extension("rlib")
            } else {
                PathBuf::from(f)
            };
            let h = hash(&p);
            let ext = p.extension().unwrap().to_str().unwrap();
            let to = root.join(format!("lib{name}-{h}.{ext}"));
            if to.exists() {
                assert_eq!(hash(&to), h)
            } else {
                fs::copy(&p, &to).unwrap();
            }
            pin(&mut pins, &to);
            if ext == "rlib" {
                all.entry(name.into())
                    .or_default()
                    .insert(to.display().to_string());
            }
        }
    }
    let mut lines = String::new();
    let mut selected = BTreeMap::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "libc",
        "memmap2",
        "rayon",
        "serde_json",
    ] {
        assert_eq!(all[name].len(), 1);
        let p = all[name].first().unwrap();
        selected.insert(name, p);
        lines.push_str(
            &serde_json::to_string(
                &json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[p]}),
            )
            .unwrap(),
        );
        lines.push('\n');
    }
    let mut fingerprints = vec![];
    for e in fs::read_dir(r("target-portable/release/.fingerprint")).unwrap() {
        let e = e.unwrap();
        for n in ["zopro-core", "zopro-bn254"] {
            let p = e.path().join(format!("lib-{}.json", n.replace('-', "_")));
            if e.file_name().to_string_lossy().starts_with(n) && p.exists() {
                let f = read(&p);
                assert_eq!(
                    f["rustflags"],
                    json!(flags.split_whitespace().collect::<Vec<_>>())
                );
                fingerprints.push(json!({"path":p,"receipt":f}));
            }
        }
    }
    assert!(fingerprints.len() >= 2);
    fs::write(r("portable-cargo.jsonl"), lines).unwrap();
    save(
        r("portable-artifacts.json"),
        &json!({"rustflags":flags,"input_pins":pins,"libraries":selected,"fingerprints":fingerprints,"proc_macros_frozen":true}),
    );
    println!(
        "Frozen{}portable libraries/shared objects; flags checked",
        pins.len()
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("setup") => setup(),
        Some("freeze") => freeze(),
        _ => panic!("setup/freeze"),
    }
}
