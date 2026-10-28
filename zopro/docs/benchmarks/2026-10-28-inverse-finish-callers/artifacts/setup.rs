use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-finish-callers";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const P: &str = "/home/ubuntu/src/zopro-perf-work/inverse-direct-finish";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("frozen-source").exists());
    let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let suffixes=[("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_inverse_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_inverse_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1};\n"),("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_inverse_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1};\n")];
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
        for name in ["frozen-source", "source"] {
            let dest = r.join(name).join(f);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
        }
    }
    for name in ["frozen-source", "source"] {
        for (f, suffix) in suffixes {
            let p = r.join(name).join(f);
            let text = fs::read_to_string(&p).unwrap();
            fs::write(&p, format!("{text}{suffix}")).unwrap();
        }
        fs::copy(
            r.join("caller_probe.rs"),
            r.join(name)
                .join("crates/zopro-core/src/msm/caller_probe.rs"),
        )
        .unwrap();
    }
    for f in [
        "crates/zopro-bn254/src/inverse.rs",
        "crates/zopro-bn254/src/inverse/direct_finish.rs",
    ] {
        let dest = r.join("source").join(f);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(Path::new(P).join("source").join(f), &dest).unwrap();
        assert_eq!(hash(&dest), hash(Path::new(P).join("source").join(f)));
    }
    let parent: Value =
        serde_json::from_slice(&fs::read(format!("{P}/source-audit.json")).unwrap()).unwrap();
    let audit = json!({"production_files":172,"frozen_original_exact":170,"fresh_original_exact":169,"core_suffixes":suffixes,"parent_inverse_edits":parent["inverse_edits"],"inverse_source_sha256":hash(r.join("source/crates/zopro-bn254/src/inverse.rs")),"reducer_source_sha256":hash(r.join("source/crates/zopro-bn254/src/inverse/direct_finish.rs")),"mathematical_core_bodies_unchanged":true,"production_changed":false});
    save(r.join("source-audit.json"), &audit);
    let flags = format!(
        "{} --cfg zopro_inverse_caller_probe",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r.join("native-flags.txt"), &flags).unwrap();
    for name in ["frozen-source", "source"] {
        for f in m["source_files_sha256"].as_object().unwrap().keys() {
            let p = r.join(name).join(f);
            pins.insert(p.display().to_string(), hash(p));
        }
        let p = r
            .join(name)
            .join("crates/zopro-core/src/msm/caller_probe.rs");
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "source/crates/zopro-bn254/src/inverse/direct_finish.rs",
        "caller_probe.rs",
        "setup.rs",
        "setup",
        "freeze.rs",
        "freeze",
        "design.md",
        "source-audit.json",
        "native-flags.txt",
        "freeze-test.log",
    ] {
        let p = r.join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    save(r.join("build-pins.json"), &json!(pins));
    let env = json!({"ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","RAYON_NUM_THREADS":"4","CARGO_BUILD_JOBS":"16","CARGO_TARGET_DIR":r.join("target")});
    let required = [
        "msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse",
        "msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse",
        "msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse",
    ];
    let mut steps = vec![];
    for label in ["frozen", "fresh", "selected"] {
        let source = if label == "frozen" {
            "frozen-source"
        } else {
            "source"
        };
        let cargo = r.join(source).join("Cargo.toml");
        let flags = if label == "selected" {
            format!("{flags} --cfg zopro_direct_finish_reduction")
        } else {
            flags.clone()
        };
        let mut argv = vec![
            "cargo".to_string(),
            "test".into(),
            "--manifest-path".into(),
            cargo.display().to_string(),
            "--locked".into(),
            "--offline".into(),
            "--release".into(),
            "-p".into(),
            "zopro-core".into(),
            "--lib".into(),
        ];
        if label == "frozen" {
            argv.push("msm::caller_probe::tests::".into());
        }
        argv.extend(["--".into(), "--test-threads=1".into()]);
        steps.push(json!({"kind":"command","name":format!("{label}-core-tests"),"argv":argv,"environment":{"RUSTFLAGS":flags},"required_tests":required,"unfiltered":label!="frozen","timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("{label}-build"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":{"RUSTFLAGS":flags},"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r.join("freeze"),label,r.join(format!("native/{label}-build/stdout.log"))],"environment":{"RUSTFLAGS":flags},"timeout_seconds":120}));
    }
    save(
        r.join("build-plan.json"),
        &json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":env,"steps":steps}),
    );
    println!(
        "Prepared3nativevariants;fieldarithmeticexactparent;frozenlibrarycaptureaftereachbuild"
    );
}
