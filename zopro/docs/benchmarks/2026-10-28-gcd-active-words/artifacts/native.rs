use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/gcd-active-words";
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
    let p = r("source/crates/zopro-bn254/src/inverse.rs");
    let mut s = fs::read_to_string(&p).unwrap();
    let old="        a = integer_update(&old_a, &b, &mut f0, &mut g0);\n        b = integer_update(&old_a, &b, &mut f1, &mut g1);";
    let new="        #[cfg(not(zopro_gcd_active_words))]\n        {a = integer_update(&old_a, &b, &mut f0, &mut g0);\n        b = integer_update(&old_a, &b, &mut f1, &mut g1);}\n        #[cfg(zopro_gcd_active_words)]\n        match bits {\n            65..=128 => {a=active_words::integer_update::<2>(&old_a,&b,&mut f0,&mut g0);b=active_words::integer_update::<2>(&old_a,&b,&mut f1,&mut g1);}\n            129..=192 => {a=active_words::integer_update::<3>(&old_a,&b,&mut f0,&mut g0);b=active_words::integer_update::<3>(&old_a,&b,&mut f1,&mut g1);}\n            _ => {a=active_words::integer_update::<4>(&old_a,&b,&mut f0,&mut g0);b=active_words::integer_update::<4>(&old_a,&b,&mut f1,&mut g1);}\n        }";
    assert_eq!(s.matches(old).count(), 1);
    s = s.replace(old, new);
    let suffix="\n#[cfg(any(test,zopro_gcd_active_words))]\n#[path=\"inverse_active_words.rs\"]\nmod active_words;\n";
    fs::write(&p, format!("{s}{suffix}")).unwrap();
    assert_eq!(
        s.replace(new, old),
        fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/inverse.rs")).unwrap()
    );
    fs::copy(
        r("active_words.rs"),
        r("source/crates/zopro-bn254/src/inverse_active_words.rs"),
    )
    .unwrap();
    let msm_suffix="\n#[cfg(zopro_active_words_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_active_words_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1};\n";
    let p = r("source/crates/zopro-core/src/msm.rs");
    let s = fs::read_to_string(&p).unwrap();
    fs::write(p, format!("{s}{msm_suffix}")).unwrap();
    let core_suffix="\n#[cfg(zopro_active_words_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1};\n";
    let p = r("source/crates/zopro-core/src/lib.rs");
    let s = fs::read_to_string(&p).unwrap();
    fs::write(p, format!("{s}{core_suffix}")).unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/direct-ref-pack/caller_probe.rs",
        r("source/crates/zopro-core/src/msm/caller_probe.rs"),
    )
    .unwrap();
    save(
        r("source-audit.json"),
        &json!({"original_files":172,"private_originals_exact":169,"runtime_change_only":"crates/zopro-bn254/src/inverse.rs","old":old,"new":new,"inverse_suffix":suffix,"msm_suffix":msm_suffix,"core_suffix":core_suffix,"coefficient_updates_and_finish_unchanged":true,"original_inverse_recovered_exact":true,"caller_adapter_original_exact":true}),
    );
    let flags = format!(
        "{} --cfg zopro_active_words_caller_probe",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        let p = r("source").join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "design.md",
        "active_words.rs",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "native-flags.txt",
        "source/crates/zopro-bn254/src/inverse_active_words.rs",
        "source/crates/zopro-core/src/msm/caller_probe.rs",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let mut steps = vec![];
    for label in ["checked", "off", "on"] {
        let full = match label {
            "checked" => format!(
                "{flags} --cfg zopro_gcd_active_words -C overflow-checks=on -C debug-assertions=on"
            ),
            "on" => format!("{flags} --cfg zopro_gcd_active_words"),
            _ => flags.clone(),
        };
        let env = json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        let cargo = r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("field-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":["inverse::active_words::tests::shortened_integer_updates_match_unsigned_oracle_and_original","inverse::tests::fq","inverse::tests::fr","inverse::tests::one_word_finish"],"unfiltered":true,"timeout_seconds":1200}));
        if label == "checked" {
            continue;
        }
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared9nativefield/core/build/freeze steps;onlyGCDactiveintegerwords+unchangedcalleradapters");
}
fn freeze(label: &str) {
    assert!(["off", "on"].contains(&label));
    let flags = std::env::var("RUSTFLAGS").unwrap();
    let base = fs::read_to_string(r("native-flags.txt")).unwrap();
    assert_eq!(
        flags,
        if label == "on" {
            format!("{base} --cfg zopro_gcd_active_words")
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
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "memmap2",
        "serde_json",
        "libc",
        "rayon",
        "sha2",
    ] {
        let x = libs.get(name).unwrap();
        assert_eq!(x.len(), 1);
        selected.insert(name, x.first().unwrap());
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
            .any(|v| v["path"].as_str().unwrap().contains(name)));
    }
    save(
        r(format!("artifacts-{label}.json")),
        &json!({"label":label,"rustflags":flags,"libraries":selected,"all_libraries":libs,"input_pins":pins,"fingerprints":fingerprints,"proc_macro_shared_objects_frozen":true}),
    );
    println!(
        "Froze{label}{}rlib/sofiles;core/fieldflagschecked",
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
