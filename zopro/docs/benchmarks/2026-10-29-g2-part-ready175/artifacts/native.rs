use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-part-ready175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
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
fn setup() {
    assert!(!r("build-plan.json").exists());
    let m = read(MANIFEST);
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(f);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let to = r("source").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        pin(&mut pins, from);
        pin(&mut pins, to);
    }
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 175);
    let file = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let before="    hb_raw(bases, words, 12, 2)";
    let after="    #[cfg(zopro_phi_part_ready)]\n    if std::env::var(\"ZOPRO_CPU_PHI_PART_READY\").as_deref() == Ok(\"1\") {\n        if let Some(result) = part_ready::raw(bases, words) { return result; }\n    }\n    hb_raw(bases, words, 12, 2)";
    let p = r("source").join(file);
    let text = fs::read_to_string(&p).unwrap();
    assert_eq!(text.matches(before).count(), 1);
    let next = text.replace(before, after);
    let next = format!("#[cfg(zopro_phi_part_ready)]\nmod part_ready;\n{}", next);
    let child = r("source/crates/zopro-core/src/msm_affine/g2_phi/part_ready.rs");
    fs::create_dir_all(child.parent().unwrap()).unwrap();
    fs::copy(r("part_ready.rs"), &child).unwrap();
    pin(&mut pins, &child);
    assert_eq!(next.strip_prefix("#[cfg(zopro_phi_part_ready)]\nmod part_ready;\n").unwrap().replace(after, before), text);
    fs::write(&p, next).unwrap();
    pin(&mut pins, &p);
    let mut exact = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        if hash(r("source").join(f)) == h.as_str().unwrap() {
            exact += 1
        } else {
            assert_eq!(f, file);
        }
    }
    assert_eq!(exact, 174);
    save(
        r("source-audit.json"),
        &json!({"original_files":175,"originals_exact":174,"changed_file":file,"before":before,"after":after,"reverse175_exact":true,"global_DEDUP_one":true,"workspace_forecast_source_exact":true,"field_source_exact":true,"G1_fusedKH_key_solver_sources_exact":true,"public_API_export_ISA_changes":false,"new_table_plan_unsafe":false,"candidate_files":176,"new_child":"crates/zopro-core/src/msm_affine/g2_phi/part_ready.rs","field_qualification":"inherited33 exact17files","production_changed":false}),
    );
    let flags = format!("{} --cfg zopro_phi_part_ready", m["rustflags"].as_str().unwrap());
    fs::write(r("native-flags.txt"), &flags).unwrap();
    let cargo = r("source/Cargo.toml");
    let mut env = json!({"RUSTFLAGS":flags,"CARGO_TARGET_DIR":r("target-candidate"),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_DEDUP":"1","ZOPRO_CPU_ZERO_AWARE_GROUPS":"1","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
    let mut steps = vec![];
    for mode in ["0", "1"] {
        env["ZOPRO_CPU_PHI_PART_READY"] = json!(mode);
        let label = if mode == "0" { "off" } else { "on" };
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["msm_affine::exclusive_g2_tests::exclusive_queries_match_generator_oracle","msm_affine::g2_borrowed::tests::exceptional_keys_and_scalar_edges_match_generator_sum","msm_affine::g2_phi::part_ready::tests::slot_preserves_part_order_for_both_arrivals","msm_affine::g2_phi::part_ready::tests::panic_releases_pending_and_consumed_inputs_without_waiting","msm_affine::g2_phi::part_ready::tests::memory_bound_and_independent_generator_oracle_across_threads"],"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("prover-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
    }
    env["ZOPRO_CPU_PHI_PART_READY"] = json!("0");
    steps.push(json!({"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180}));
    steps.push(json!({"kind":"link-sdk","name":"candidate","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    assert_eq!(steps.len(), 7);
    for f in [
        "bootstrap.rs","history-audit.md","native.rs",
        "native-helper",
        "design.md",
        "part_ready.rs","source-audit.json",
        "native-flags.txt",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [
        MANIFEST,
        "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native.rs",
    ] {
        pin(&mut pins, f);
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared7native steps,174exact/reverse175, onlyscalar-loop/globalDEDUP1/forecastsunchanged");
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
