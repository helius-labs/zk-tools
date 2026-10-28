use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
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
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, &p);
        for root in ["source", "control-source"] {
            let q = r(root).join(f);
            fs::create_dir_all(q.parent().unwrap()).unwrap();
            fs::copy(&p, q).unwrap();
        }
    }
    let guard="all(zopro_g2_horner_buckets,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let mut edits = vec![];
    for f in [
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/msm_affine.rs",
    ] {
        let p = r("source").join(f);
        let mut s = fs::read_to_string(&p).unwrap();
        let suffix = format!("\n#[cfg({guard})]\nmod g2_borrowed;\n");
        s.push_str(&suffix);
        fs::write(p, s).unwrap();
        edits.push(json!({"file":f,"suffix":suffix}));
    }
    let file = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let before = "    let size = batch_size(bits).max(32);";
    let after=format!("    #[cfg({guard})]\n    if let Some(value) = super::g2_borrowed::try_window(bases, short, w, bits) {{\n        return value;\n    }}\n{before}");
    let p = r("source").join(file);
    let mut s = fs::read_to_string(&p).unwrap();
    once(&mut s, before, &after);
    fs::write(p, s).unwrap();
    edits.push(json!({"file":file,"old":before,"new":after}));
    for f in [
        "crates/zopro-core/src/msm/g2_borrowed.rs",
        "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
    ] {
        let from = Path::new(P).join("source").join(f);
        let to = r("source").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        assert_eq!(hash(&from), hash(&to));
        pin(&mut pins, from);
        pin(&mut pins, to);
    }
    let mut exact = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = r("source").join(f);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1
        } else {
            let mut back = fs::read_to_string(&p).unwrap();
            for e in edits.iter().rev().filter(|e| e["file"] == *f) {
                if let Some(s) = e.get("suffix") {
                    back = back.strip_suffix(s.as_str().unwrap()).unwrap().to_string();
                } else {
                    once(
                        &mut back,
                        e["new"].as_str().unwrap(),
                        e["old"].as_str().unwrap(),
                    );
                }
            }
            assert_eq!(back, fs::read_to_string(Path::new(Z).join(f)).unwrap());
        }
        for dir in ["source", "control-source"] {
            pin(&mut pins, r(dir).join(f));
        }
    }
    assert_eq!(exact, 169);
    assert_eq!(
        hash(r("source/crates/zopro-core/src/lib.rs")),
        hash(Path::new(Z).join("crates/zopro-core/src/lib.rs"))
    );
    save(
        r("source-audit.json"),
        &json!({"original_files":172,"originals_exact":169,"reverse172_exact":true,"new_modules":2,"edits":edits,"field_unchanged":true,"no_public_probe_exports":true,"private_experiment_flags_removed":true,"existing_runtime_optouts_unchanged":true,"forecasts_unchanged":true,"portable_exclusion":"supported-native guard false under no_ifma or missing AVX512 features both before/after","production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_g2_queue_probe") && !flags.contains("zopro_g2_borrow_queue"));
    fs::write(r("native-flags.txt"), flags).unwrap();
    let mut steps = vec![];
    for label in ["control", "candidate"] {
        let cargo = r(if label == "control" {
            "control-source/Cargo.toml"
        } else {
            "source/Cargo.toml"
        });
        let env = json!({"RUSTFLAGS":flags,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        if label == "candidate" {
            steps.push(json!({"kind":"command","name":"candidate-core","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["msm::g2_borrowed::tests::signed_borrowed_prefix_tails_and_reuse","msm_affine::g2_borrowed::tests::full_width_phi_msm_matches_generator_sum","msm_affine::g2_borrowed::tests::exceptional_keys_and_scalar_edges_match_generator_sum"],"unfiltered":true,"timeout_seconds":1800}));
            steps.push(json!({"kind":"command","name":"candidate-prover","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        }
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    for f in [
        "design.md",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "native-flags.txt",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        pin(&mut pins, Path::new(Z).join(f));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared8clean-native steps;169exact/reverse172+2testedmodules;no probe exports/experimental flag/productionmutation");
}
fn freeze(label: &str) {
    assert!(["control", "candidate"].contains(&label));
    let flags = std::env::var("RUSTFLAGS").unwrap();
    assert_eq!(flags, fs::read_to_string(r("native-flags.txt")).unwrap());
    let root = r("frozen").join(label);
    fs::create_dir_all(&root).unwrap();
    let mut all = BTreeMap::<String, BTreeSet<String>>::new();
    let mut pins = BTreeMap::new();
    for l in fs::read_to_string(r(format!("native/libraries-{label}/stdout.log")))
        .unwrap()
        .lines()
    {
        let a: Value = serde_json::from_str(l).unwrap();
        if a["reason"] != "compiler-artifact" {
            continue;
        }
        let n = a["target"]["name"].as_str().unwrap();
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
            let to = root.join(format!("lib{n}-{h}.{ext}"));
            if to.exists() {
                assert_eq!(hash(&to), h)
            } else {
                fs::copy(&p, &to).unwrap();
            }
            pin(&mut pins, &to);
            if ext == "rlib" {
                all.entry(n.into())
                    .or_default()
                    .insert(to.display().to_string());
            }
        }
    }
    let mut lines = String::new();
    let mut libs = BTreeMap::new();
    for n in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "libc",
        "memmap2",
        "rayon",
        "serde_json",
    ] {
        assert_eq!(all[n].len(), 1);
        let p = all[n].first().unwrap();
        libs.insert(n, p);
        lines.push_str(
            &serde_json::to_string(
                &json!({"reason":"compiler-artifact","target":{"name":n},"filenames":[p]}),
            )
            .unwrap(),
        );
        lines.push('\n');
    }
    let mut fingerprints = vec![];
    for e in fs::read_dir(r(format!("target-{label}/release/.fingerprint"))).unwrap() {
        let e = e.unwrap();
        for n in ["zopro-core", "zopro-bn254"] {
            let p = e.path().join(format!("lib-{}.json", n.replace('-', "_")));
            if e.file_name().to_string_lossy().starts_with(n) && p.exists() {
                let f = read(&p);
                assert_eq!(
                    f["rustflags"],
                    json!(flags.split_whitespace().collect::<Vec<_>>())
                );
                if n == "zopro-core" {
                    assert!(f["features"].as_str().unwrap().contains("ifma8"));
                }
                fingerprints.push(json!({"path":p,"receipt":f}));
            }
        }
    }
    assert!(fingerprints.len() >= 2);
    fs::write(r(format!("frozen-{label}-cargo.jsonl")), lines).unwrap();
    save(
        r(format!("frozen-{label}.json")),
        &json!({"rustflags":flags,"libraries":libs,"input_pins":pins,"fingerprints":fingerprints,"proc_macro_shared_objects_frozen":true}),
    );
    println!(
        "Frozen{label}{}rlib/sofiles; nativeflags/features checked",
        pins.len()
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("setup") => setup(),
        Some("freeze") => freeze(&a[2]),
        _ => panic!("setup/freeze"),
    }
}
