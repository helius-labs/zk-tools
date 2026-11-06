use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const BASE: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const W: &str = "/home/ubuntu/src/zopro-perf-work/word-export176/target-candidate/release/deps";
const T: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
const C: &str = "/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("build-plan.json").exists());
    let b = Path::new(BASE).join("frozen/candidate");
    // Select the immutable accepted library by its complete digest, not crate prefix.
    let expected = "1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713";
    let field = b.join(format!("libzopro_bn254-{expected}.rlib"));
    assert_eq!(hash(&field), expected);
    let flags: Value =
        serde_json::from_slice(&fs::read(Path::new(BASE).join("frozen-candidate.json")).unwrap())
            .unwrap();
    let rustflags = flags["rustflags"]
        .as_str()
        .unwrap()
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();
    let mut pins = BTreeMap::new();
    let mut pin = |p: PathBuf| {
        pins.insert(p.display().to_string(), hash(p));
    };
    for p in [C, X] {
        pin(PathBuf::from(p));
    }
    for f in [
        "prepare.rs",
        "prepare",
        "bulk.rs",
        "fixture.rs",
        "oracle.rs",
        "driver.rs",
        "perf.rs",
        "design.md",
    ] {
        pin(r.join(f));
    }
    for d in [W, T, b.to_str().unwrap()] {
        for e in fs::read_dir(d).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|e| e == "rlib") {
                pin(p);
            }
        }
    }
    pin(Path::new(BASE).join("frozen-candidate.json"));
    let s = Path::new("/home/ubuntu/src/zk-tools/zopro");
    let manifest=s.join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let v: Value = serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
    for (rel, h) in v["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(s.join(rel)), h.as_str().unwrap());
        pin(s.join(rel));
    }
    pin(manifest);
    for p in["/home/ubuntu/src/zopro-perf-work/stream-work-census176/result.json","/home/ubuntu/src/zopro-perf-work/stream-work-census176/review.json","/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/analysis.json","/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-31-weighted-frontier176/README.md","/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-21-ryzen-research/g2-shared-inverse-production/README.md"]{pin(PathBuf::from(p));}
    let mut steps = vec![];
    for oracle in [true, false] {
        let name = if oracle { "oracle" } else { "driver" };
        let mut argv = vec![
            C.into(),
            "--edition=2021".into(),
            "-C".into(),
            "opt-level=3".into(),
            "-C".into(),
            "codegen-units=1".into(),
        ];
        if !oracle {
            argv.extend(["-C".into(), "lto=fat".into()]);
        }
        argv.extend(rustflags.clone());
        argv.push(r.join(format!("{name}.rs")).display().to_string());
        for (crate_name, path) in [
            (
                "serde_json",
                PathBuf::from(format!("{T}/libserde_json-cdebb9338c2784ad.rlib")),
            ),
            (
                "sha2",
                PathBuf::from(format!("{T}/libsha2-16b8f2c03bdf5faa.rlib")),
            ),
            ("zopro_bn254", field.clone()),
        ] {
            argv.extend([
                "--extern".into(),
                format!("{crate_name}={}", path.display()),
            ]);
        }
        if oracle {
            for (crate_name, hash) in [
                ("ark_bn254", "d28868a71f06f5b5"),
                ("ark_ff", "3a8e68dc8045d2bd"),
            ] {
                argv.extend([
                    "--extern".into(),
                    format!("{crate_name}={W}/lib{crate_name}-{hash}.rlib"),
                ]);
            }
        } else {
            argv.extend(["--extern".into(),format!("libc={}/liblibc-1c05b85983496aa8e78e217245b072feeb2e1076038b370fcb7eca849c9e4cab.rlib",b.display())]);
        }
        for d in [W, T, b.to_str().unwrap()] {
            argv.extend(["-L".into(), format!("dependency={d}")]);
        }
        argv.extend(["-o".into(), r.join(name).display().to_string()]);
        steps.push(json!({"kind":"command","name":format!("build-{name}"),"argv":argv,"timeout_seconds":600}));
        if oracle {
            steps.push(json!({"kind":"command","name":"run-oracle","argv":[r.join("oracle")],"timeout_seconds":600}));
        }
    }
    fs::write(
        r.join("build-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(r.join("build-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    println!("Frozen3changedwrapper qualification/linkcommands/{}pins; unchangedacceptedfield/source176 reused; no driver clock",pins.len());
    let _ = field;
}
