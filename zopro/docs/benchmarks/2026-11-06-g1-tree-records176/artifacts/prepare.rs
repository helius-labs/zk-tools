use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-records176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const B: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const N: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn replace(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    assert!(!Path::new(R).join("plan.json").exists());
    let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let original = read(&manifest)["source_files_sha256"].clone();
    assert_eq!(original.as_object().unwrap().len(), 176);
    let mut pins = BTreeMap::new();
    for (p, h) in original.as_object().unwrap() {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let to = Path::new(R).join("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        pins.insert(from.display().to_string(), hash(from));
    }
    let changes=[
        ("let use_prefix = F::<C>::TREE_PREFIX && m >= 32;","let use_prefix = F::<C>::TREE_PREFIX && m >= F::<C>::TREE_PREFIX_MIN;"),
        ("s.prefix_records.reserve(full / LANES);","F::<C>::reserve_tree_prefix(&mut s.prefix_records, full / LANES);"),
        ("    const TREE_PREFIX: bool = false;","    const TREE_PREFIX: bool = false;\n    const TREE_PREFIX_MIN: usize = 32;\n    fn reserve_tree_prefix(records: &mut Vec<[FqBatch; 2]>, n: usize) { records.reserve(n); }"),
        ("impl BatchInverse for Fq {","impl BatchInverse for Fq {\n    const TREE_PREFIX: bool = cfg!(all(zopro_g1_tree_records, feature=\"ifma8\", target_arch=\"x86_64\", target_feature=\"avx512ifma\", target_feature=\"avx512vl\", target_feature=\"avx512dq\", not(no_ifma)));\n    const TREE_PREFIX_MIN: usize = LANES;\n    fn reserve_tree_prefix(records: &mut Vec<[FqBatch; 2]>, n: usize) { records.reserve_exact(n); }")
    ];
    let path = format!("{R}/source/crates/zopro-core/src/msm.rs");
    let mut msm = fs::read_to_string(&path).unwrap();
    for (a, b) in &changes {
        replace(&mut msm, a, b);
    }
    let suffix = "\n#[cfg(zopro_g1_tree_records_probe)]\npub mod g1_tree_records_probe;\n";
    msm.push_str(suffix);
    fs::write(&path, &msm).unwrap();
    let p = format!("{R}/source/crates/zopro-core/src/lib.rs");
    let mut lib = fs::read_to_string(&p).unwrap();
    let libsuffix =
        "\n#[cfg(zopro_g1_tree_records_probe)]\npub use msm::g1_tree_records_probe::TreeCaller;\n";
    lib.push_str(libsuffix);
    fs::write(p, lib).unwrap();
    let p = format!("{R}/source/crates/zopro-core/src/msm/g1_tree_records_probe.rs");
    fs::create_dir_all(Path::new(&p).parent().unwrap()).unwrap();
    fs::copy(format!("{R}/probe.rs"), &p).unwrap();
    let mut recovery = msm.strip_suffix(suffix).unwrap().to_owned();
    for (a, b) in changes.iter().rev() {
        replace(&mut recovery, b, a);
    }
    assert_eq!(
        hash(format!("{Z}/crates/zopro-core/src/msm.rs")),
        format!("{:x}", Sha256::digest(recovery.as_bytes()))
    );
    let mut actual = BTreeMap::new();
    for p in original.as_object().unwrap().keys().chain(std::iter::once(
        &"crates/zopro-core/src/msm/g1_tree_records_probe.rs".to_owned(),
    )) {
        let path = Path::new(R).join("source").join(p);
        let h = hash(&path);
        actual.insert(p.clone(), h.clone());
        pins.insert(path.display().to_string(), h);
    }
    assert_eq!(actual.len(), 177);
    let previous = read(format!("{N}/runs/build-core/command.json"));
    let old: Vec<String> = previous["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|s| s.as_str().unwrap().to_owned())
        .collect();
    let ark = "/home/ubuntu/src/zopro-perf-work/word-export176/target-candidate/release/deps";
    let mut library_info = BTreeMap::new();
    let mut steps = Vec::new();
    for mode in ["native", "portable"] {
        let kind = if mode == "native" {
            "candidate"
        } else {
            "portable"
        };
        let frozenfile = format!("{B}/frozen-{kind}.json");
        let frozen = read(&frozenfile);
        for (p, h) in frozen["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins.insert(p.clone(), h.as_str().unwrap().to_owned());
        }
        pins.insert(frozenfile.clone(), hash(&frozenfile));
        let support = format!("{B}/frozen/{kind}");
        let out = format!("{R}/libs/{mode}");
        fs::create_dir_all(&out).unwrap();
        let mut selected = BTreeMap::new();
        for name in [
            "ahash",
            "getrandom",
            "libc",
            "log",
            "miniz_oxide",
            "rayon",
            "sha2",
            "wasm_bindgen",
            "wasm_bindgen_rayon",
            "zopro_bn254",
        ] {
            let paths: Vec<_> = frozen["input_pins"]
                .as_object()
                .unwrap()
                .keys()
                .filter(|p| p.contains(&format!("/lib{name}-")) && p.ends_with(".rlib"))
                .cloned()
                .collect();
            let path = if paths.len() == 1 {
                paths[0].clone()
            } else {
                assert_eq!(name, "getrandom");
                let log = format!("{B}/native/libraries-{kind}/stdout.log");
                let mut chosen = None;
                for line in fs::read_to_string(&log).unwrap().lines() {
                    let v: Value = serde_json::from_str(line).unwrap();
                    if v["reason"] == "compiler-artifact"
                        && v["package_id"]
                            .as_str()
                            .is_some_and(|s| s.ends_with("#getrandom@0.2.17"))
                    {
                        let files = v["filenames"].as_array().unwrap();
                        for f in files
                            .iter()
                            .filter_map(Value::as_str)
                            .filter(|f| f.ends_with(".rlib"))
                        {
                            let h = hash(f);
                            let p = format!("{support}/libgetrandom-{h}.rlib");
                            assert!(paths.contains(&p));
                            chosen = Some(p);
                        }
                    }
                }
                pins.insert(log.clone(), hash(log));
                chosen.unwrap()
            };
            selected.insert(name.to_owned(), path);
        }
        let mut core = old.clone();
        let p = core.iter().position(|s| s == "zopro_h_only_probe").unwrap();
        assert_eq!(core[p - 1], "--cfg");
        core.drain(p - 1..=p);
        for a in &mut core {
            if a == &format!("{N}/source/crates/zopro-core/src/lib.rs") {
                *a = format!("{R}/source/crates/zopro-core/src/lib.rs");
            } else if a == &format!("dependency={B}/frozen/candidate") {
                *a = format!("dependency={support}");
            } else if a == &format!("dependency={N}/libs") {
                *a = format!("dependency={out}");
            } else if a == &format!("{N}/libs/libzopro_core.rlib") {
                *a = format!("{out}/libzopro_core.rlib");
            } else if mode == "portable" && a == "target-cpu=native" {
                *a = "target-cpu=x86-64".into();
            } else if let Some((name, _)) = a.split_once('=') {
                if selected.contains_key(name) {
                    *a = format!("{name}={}", selected[name]);
                }
            }
        }
        core.extend([
            "--cfg".into(),
            "zopro_g1_tree_records_probe".into(),
            "--cfg".into(),
            "zopro_g1_tree_records".into(),
        ]);
        if mode == "portable" {
            core.extend(["--cfg".into(), "no_ifma".into()]);
        }
        let mut env = previous["environment"].clone();
        env["CARGO_PKG_NAME"] = json!("zopro-core");
        env["CARGO_PKG_VERSION"] = json!("0.1.0");
        env["CARGO_MANIFEST_DIR"] = json!(format!("{R}/source/crates/zopro-core"));
        steps.push(json!({"kind":"command","name":format!("build-core-{mode}"),"argv":core,"environment":env,"timeout_seconds":900}));
        let mut test = core.clone();
        let p = test.iter().position(|s| s == "--crate-type").unwrap();
        test.drain(p..p + 2);
        let p = test.iter().position(|s| s == "-o").unwrap();
        test[p + 1] = format!("{R}/core-tests-{mode}");
        test.extend(["--test".into(), "-C".into(), "lto=fat".into()]);
        steps.push(json!({"kind":"command","name":format!("build-tests-{mode}"),"argv":test,"environment":env,"timeout_seconds":900}));
        let required = [
            "full_vectors_partial_tails_and_exact_retained_capacity",
            "weighted_copied_even_identity_and_original_forecasts",
            "grouped_and_weighted_public_paths_in_small_stack_rayon_pools",
            "fixed_stack_workers_reuse_complete_scratch_through_exceptions",
        ]
        .map(|s| format!("msm::g1_tree_records_probe::tests::{s}"));
        let mut run = vec![format!("{R}/core-tests-{mode}")];
        if mode == "portable" {
            run.push("msm::g1_tree_records_probe::tests::".into());
        }
        run.extend(["--test-threads=1".into(), "--nocapture".into()]);
        steps.push(json!({"kind":"command","name":format!("tests-{mode}"),"argv":run,"environment":env,"timeout_seconds":900,"unfiltered":mode=="native","required_tests":required}));
        let mut oracle = vec![
            old[0].clone(),
            "--edition=2021".into(),
            "-O".into(),
            "-C".into(),
            format!(
                "target-cpu={}",
                if mode == "native" { "native" } else { "x86-64" }
            ),
            "-C".into(),
            "lto=fat".into(),
            "-C".into(),
            "codegen-units=1".into(),
            "-C".into(),
            "debuginfo=1".into(),
            "--test".into(),
            "-L".into(),
            format!("dependency={support}"),
            "-L".into(),
            format!("dependency={out}"),
            "-L".into(),
            format!("dependency={ark}"),
            "--extern".into(),
            format!("zopro_core={out}/libzopro_core.rlib"),
        ];
        for (name, file) in [
            ("ark_bn254", "libark_bn254-d28868a71f06f5b5.rlib"),
            ("ark_ec", "libark_ec-59ebd5b49f9b3011.rlib"),
            ("ark_ff", "libark_ff-3a8e68dc8045d2bd.rlib"),
        ] {
            let p = format!("{ark}/{file}");
            oracle.extend(["--extern".into(), format!("{name}={p}")]);
            pins.insert(p.clone(), hash(p));
        }
        oracle.extend([
            format!("{R}/oracle.rs"),
            "-o".into(),
            format!("{R}/oracle-tests-{mode}"),
        ]);
        steps.push(json!({"kind":"command","name":format!("build-oracle-{mode}"),"argv":oracle,"environment":env,"timeout_seconds":600}));
        steps.push(json!({"kind":"command","name":format!("oracle-{mode}"),"argv":[format!("{R}/oracle-tests-{mode}"),"--test-threads=1","--nocapture"],"environment":env,"timeout_seconds":600,"unfiltered":true,"required_tests":["tests::complete_grouped_and_indexed_outputs_match_arkworks","tests::complete_weighted_outputs_match_arkworks"]}));
        library_info.insert(mode,json!({"field_and_support_reused":selected,"frozen_manifest":frozenfile,"targetcpu":if mode=="native"{"native"}else{"x86-64/no_ifma"}}));
    }
    for p in [
        "design.md",
        "probe.rs",
        "oracle.rs",
        "prepare.rs",
        "prepare",
    ] {
        let p = format!("{R}/{p}");
        pins.insert(p.clone(), hash(p));
    }
    for p in[manifest,format!("{N}/runs/build-core/command.json"),format!("{Z}/docs/benchmarks/2026-10-25-prefix-records-component/source/crates/zopro-core/src/msm/prefix_records.rs"),format!("{Z}/docs/benchmarks/2026-10-27-g2-tree-records/artifacts/candidate/crates/zopro-core/src/msm.rs")]{pins.insert(p.clone(),hash(p));}
    save(
        format!("{R}/source-audit.json"),
        &json!({"accepted_count":176,"candidate_count":177,"accepted_source_hashes":original,"candidate_source_hashes":actual,"unchanged_originals":174,"changed_originals":["crates/zopro-core/src/msm.rs","crates/zopro-core/src/lib.rs"],"inverse_edits_recover_all_originals":true,"msm_edits":changes,"msm_appendix":suffix,"lib_appendix":libsuffix,"no_new_field_math_or_unsafe":true,"G2_default_threshold":32,"G2_default_reservation":"reserve","G1_full_native_vector_boundary":"LANES","G1_reservation":"reserve_exact","new_tests":4,"new_Arkworks_tests_each_mode":2,"production_changed":false}),
    );
    save(format!("{R}/libraries.json"), &json!(library_info));
    pins.insert(
        format!("{R}/libraries.json"),
        hash(format!("{R}/libraries.json")),
    );
    pins.insert(
        format!("{R}/source-audit.json"),
        hash(format!("{R}/source-audit.json")),
    );
    assert_eq!(steps.len(), 10);
    save(format!("{R}/pins.json"), &json!(pins));
    save(
        format!("{R}/plan.json"),
        &json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":steps}),
    );
    println!("Frozen177-source/native+portable field-support reused/10commands/native129tests(125old+4new),portable4focused,2Ark each; no timing");
}
