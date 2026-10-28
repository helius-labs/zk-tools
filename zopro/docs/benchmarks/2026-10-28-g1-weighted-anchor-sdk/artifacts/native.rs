use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor-sdk";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor";
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
    assert!(!r("build-plan.json").exists());
    assert_eq!(
        read(format!("{P}/analysis.json"))["gates"]["component_gate_passed"],
        true
    );
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{P}/pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        let q = r("source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(p, q).unwrap();
    }
    let audit = read(format!("{P}/source-audit.json"));
    let parent = fs::read_to_string(format!("{P}/source/crates/zopro-core/src/msm.rs")).unwrap();
    let pure = parent
        .strip_suffix(audit["msm_suffix"].as_str().unwrap())
        .unwrap();
    assert!(!pure.contains("caller_probe"));
    let mut back = pure.to_string();
    for e in audit["msm_edits"].as_array().unwrap().iter().rev() {
        assert_eq!(back.matches(e["new"].as_str().unwrap()).count(), 1);
        back = back.replace(e["new"].as_str().unwrap(), e["old"].as_str().unwrap());
    }
    assert_eq!(
        back,
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap()
    );
    fs::write(r("source/crates/zopro-core/src/msm.rs"), pure).unwrap();
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("caller_probe"));
    assert!(!flags.contains("symmetric_square"));
    fs::write(r("native-flags.txt"), flags).unwrap();
    save(
        r("source-audit.json"),
        &json!({"production_files":172,"private_originals_exact":171,"runtime_changed_only":"crates/zopro-core/src/msm.rs","msm_edits":audit["msm_edits"],"parent_runtime_bytes_exact":true,"diagnostic_adapters_removed":true,"field_unchanged":true,"reverse_edits_recover_accepted":true}),
    );
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        let p = r("source").join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "design.md",
        "native.rs",
        "native-helper",
        "make-sdk.rs",
        "make-sdk",
        "capture.rs",
        "capture",
        "finish.rs",
        "finish",
        "source-audit.json",
        "native-flags.txt",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let mut steps = vec![];
    for label in ["off", "on"] {
        let full = if label == "on" {
            format!("{flags} --cfg zopro_g1_weighted_anchor")
        } else {
            flags.into()
        };
        let env = json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        let cargo = r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["solver::tests::mapped_prefixes_validate_roundtrip_and_keep_the_owner_alive","solver::tests::refined_components_match_sequential_execution","solver::tests::solves_and_rejects_invalid_witnesses"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("raw-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","raw::tests::"],"environment":env,"required_tests":["raw::tests::rejects_overflowing_and_truncated_lengths","raw::tests::sparse_images_cross_four_gib_without_truncation"],"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("cubic-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--test","cubic"],"environment":env,"required_tests":["parses_values","proves_by_name","writes_dump_directly_without_intermediate_point_copies","loads_from_its_raw_form","proves_from_its_image_in_place","reads_legacy_key_and_raw_image"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared12pureNativeSDKsteps;172originals171exact,onlyparentruntimeMSMchange;NOdiagnosticadapters");
}
fn freeze(label: &str) {
    assert!(["off", "on"].contains(&label));
    let flags = std::env::var("RUSTFLAGS").unwrap();
    let base = fs::read_to_string(r("native-flags.txt")).unwrap();
    assert_eq!(
        flags,
        if label == "on" {
            format!("{base} --cfg zopro_g1_weighted_anchor")
        } else {
            base
        }
    );
    let log = fs::read_to_string(r(format!("native/libraries-{label}/stdout.log"))).unwrap();
    let root = r(format!("frozen/{label}"));
    fs::create_dir_all(&root).unwrap();
    let mut libs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut pins = BTreeMap::new();
    for line in log.lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        if v["reason"] != "compiler-artifact" {
            continue;
        }
        let name = v["target"]["name"].as_str().unwrap();
        let files = v["filenames"].as_array().unwrap();
        let metas: Vec<_> = files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rmeta") && p.contains("/deps/"))
            .collect();
        for file in files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rlib") || p.ends_with(".so"))
        {
            let p = if file.ends_with(".rlib") && !file.contains("/deps/") && metas.len() == 1 {
                Path::new(metas[0]).with_extension("rlib")
            } else {
                PathBuf::from(file)
            };
            let h = hash(&p);
            let ext = p.extension().unwrap().to_str().unwrap();
            let dest = root.join(format!("lib{name}-{h}.{ext}"));
            if dest.exists() {
                assert_eq!(hash(&dest), h)
            } else {
                fs::copy(&p, &dest).unwrap();
            }
            assert_eq!(hash(&dest), h);
            pins.insert(dest.display().to_string(), h);
            if ext == "rlib" {
                libs.entry(name.into())
                    .or_default()
                    .insert(dest.display().to_string());
            }
        }
    }
    let mut selected = BTreeMap::new();
    let mut lines = String::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "memmap2",
        "serde_json",
        "libc",
        "rayon",
    ] {
        let found = libs.get(name).unwrap();
        assert_eq!(found.len(), 1, "{name}");
        let p = found.first().unwrap();
        selected.insert(name, p.clone());
        lines.push_str(
            &serde_json::to_string(
                &json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[p]}),
            )
            .unwrap(),
        );
        lines.push('\n');
    }
    let mut fingerprints = vec![];
    for e in fs::read_dir(r(format!("target-{label}/release/.fingerprint"))).unwrap() {
        let e = e.unwrap();
        for name in ["zopro-core", "zopro-bn254"] {
            if e.file_name()
                .to_string_lossy()
                .starts_with(&format!("{name}-"))
            {
                let p = e
                    .path()
                    .join(format!("lib-{}.json", name.replace('-', "_")));
                if p.exists() {
                    let f = read(&p);
                    assert_eq!(
                        f["rustflags"],
                        json!(flags.split_whitespace().collect::<Vec<_>>())
                    );
                    if name == "zopro-core" {
                        assert!(f["features"].as_str().unwrap().contains("ifma8"));
                    }
                    fingerprints.push(json!({"path":p,"receipt":f}));
                }
            }
        }
    }
    for name in ["zopro-core", "zopro-bn254"] {
        assert!(fingerprints
            .iter()
            .any(|f| f["path"].as_str().unwrap().contains(name)));
    }
    fs::write(r(format!("frozen-{label}-cargo.jsonl")), lines).unwrap();
    save(
        r(format!("frozen-{label}.json")),
        &json!({"label":label,"rustflags":flags,"libraries":selected,"all_libraries":libs,"input_pins":pins,"fingerprints":fingerprints,"proc_macro_shared_objects_frozen":true,"cargo_artifacts_sha256":format!("{:x}",Sha256::digest(log.as_bytes()))}),
    );
    println!(
        "Froze{label}{}rlib/sofiles;8requiredSDKlibs;core/fieldflagschecked",
        pins.len()
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "setup" => setup(),
        "freeze" => freeze(&a[2]),
        _ => panic!("mode"),
    }
}
