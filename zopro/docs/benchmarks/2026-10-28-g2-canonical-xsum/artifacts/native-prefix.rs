use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn setup() {
    assert!(!r("source").exists());
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let q = r("source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
    }
    let p = r("source/crates/zopro-core/src/msm/g2_prefix.rs");
    let s = fs::read_to_string(&p).unwrap();
    let old = "        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));";
    let new="        #[cfg(zopro_g2_canonical_xsum)]\n        let xsum=core::array::from_fn(|i|canonical_add::<FAST>(&ax[i],&bx[i]));\n        #[cfg(not(zopro_g2_canonical_xsum))]\n        let xsum=Fq2::lanes_add(&ax,&bx);\n        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&xsum);";
    assert_eq!(s.matches(old).count(), 1);
    let changed = s.replace(old, new);
    assert_eq!(changed.replace(new, old), s);
    fs::write(p, changed).unwrap();
    let msm_suffix="\n#[cfg(zopro_canonical_xsum_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_canonical_xsum_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1};\n";
    let core_suffix="\n#[cfg(zopro_canonical_xsum_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1};\n";
    for (f, suffix) in [
        ("crates/zopro-core/src/msm.rs", msm_suffix),
        ("crates/zopro-core/src/lib.rs", core_suffix),
    ] {
        let p = r("source").join(f);
        let s = fs::read_to_string(&p).unwrap();
        fs::write(p, format!("{s}{suffix}")).unwrap();
    }
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/direct-ref-pack/caller_probe.rs",
        r("source/crates/zopro-core/src/msm/caller_probe.rs"),
    )
    .unwrap();
    save(
        r("source-audit.json"),
        &json!({"original_files":172,"private_originals_exact":169,"runtime_file":"crates/zopro-core/src/msm/g2_prefix.rs","old":old,"new":new,"msm_suffix":msm_suffix,"core_suffix":core_suffix,"reverse_edit_recovers_accepted":true,"field_unchanged":true,"adapter_exact":true}),
    );
    let flags = format!(
        "{} --cfg zopro_canonical_xsum_caller_probe",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        let p = r("source").join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "design.md",
        "make-native.rs",
        "make-native",
        "native-prefix.rs",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "native-flags.txt",
        "source/crates/zopro-core/src/msm/caller_probe.rs",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let mut steps = vec![];
    for label in ["off", "on"] {
        let full = if label == "on" {
            format!("{flags} --cfg zopro_g2_canonical_xsum")
        } else {
            flags.clone()
        };
        let env = json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        let cargo = r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse","msm::g2_prefix::tests::g2_prefix_complete_curve_tails_reuse"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared6nativecore/build/freeze steps; fieldunchanged;122coretestsOFF/ONexpected");
}
