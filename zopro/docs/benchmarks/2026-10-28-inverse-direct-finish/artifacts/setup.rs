use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-direct-finish";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("source").exists());
    let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let q = r.join("source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_owned());
    }
    let path = r.join("source/crates/zopro-bn254/src/inverse.rs");
    let original = fs::read_to_string(&path).unwrap();
    let anchor = "    let (f0, g0) = (f1, g1);\n";
    assert_eq!(original.matches(anchor).count(), 1);
    let added="    #[cfg(zopro_direct_finish_reduction)]\n    if (C::P[3] as u128)*3 > 1u128<<63 && C::P[3] < 1u64<<63 {\n        return Fp::from_montgomery(direct_finish_reduce::<C>(&u,&v,f0,g0));\n    }\n";
    let suffix="\n#[cfg(any(test,zopro_direct_finish_reduction))]\ninclude!(\"inverse/direct_finish.rs\");\n";
    let changed = format!(
        "{}{suffix}",
        original.replace(anchor, &format!("{anchor}{added}"))
    );
    assert_eq!(
        changed.strip_suffix(suffix).unwrap().replace(added, ""),
        original
    );
    fs::write(&path, changed).unwrap();
    fs::write(r.join("original-inverse.rs"), original).unwrap();
    let dest = r.join("source/crates/zopro-bn254/src/inverse/direct_finish.rs");
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    fs::copy(r.join("direct_finish.rs"), &dest).unwrap();
    save(
        r.join("source-audit.json"),
        &json!({"production_files_exact":172,"private_originals_exact":171,"one_changed_file":"crates/zopro-bn254/src/inverse.rs","inverse_edits":[added,suffix],"original_finish_body_recovered_exact":true,"new_field_module":"crates/zopro-bn254/src/inverse/direct_finish.rs","no_core_prover_changes":true}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    fs::write(r.join("native-flags.txt"), flags).unwrap();
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        let p = r.join("source").join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "direct_finish.rs",
        "source/crates/zopro-bn254/src/inverse/direct_finish.rs",
        "source-audit.json",
        "original-inverse.rs",
        "native-flags.txt",
        "design.md",
        "setup.rs",
        "setup",
    ] {
        let p = r.join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    pins.insert(X.into(), hash(X));
    save(r.join("build-pins.json"), &json!(pins));
    let cargo = r.join("source/Cargo.toml");
    let mut steps = vec![];
    for (label, extra) in [
        ("fresh", ""),
        ("candidate", " --cfg zopro_direct_finish_reduction"),
    ] {
        let env = json!({"RUSTFLAGS":format!("{flags}{extra}")});
        steps.push(json!({"kind":"command","name":format!("{label}-tests"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib"],"environment":env,"required_tests":["inverse::direct_finish_tests::direct_signed_finish_matches_independent_fullwidth_fields"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("{label}-library"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--message-format=json"],"environment":env,"timeout_seconds":600}));
    }
    save(
        r.join("build-plan.json"),
        &json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":{"CARGO_BUILD_JOBS":"16","CARGO_TARGET_DIR":r.join("target")},"steps":steps}),
    );
    println!("Prepared direct signed Euclidean reduction;171 original files exact");
}
