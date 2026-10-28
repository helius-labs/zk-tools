use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const GUARD:&str="all(zopro_direct_ref_pack, feature=\"ifma8\", target_arch=\"x86_64\", target_feature=\"avx2\", target_feature=\"avx512ifma\", target_feature=\"avx512vl\", target_feature=\"avx512dq\", not(no_ifma))";
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
fn manifest() -> Value {
    read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"))
}
fn setup() {
    assert!(!r("source").exists());
    let m = manifest();
    let mut pins = BTreeMap::new();
    let adapter = fs::read_to_string(
        "/home/ubuntu/src/zopro-perf-work/inverse-finish-callers/caller_probe.rs",
    )
    .unwrap();
    fs::write(r("caller_probe.rs"), &adapter).unwrap();
    let suffixes=[("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_inverse_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_inverse_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1};\n"),("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_inverse_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1};\n")];
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
        for name in ["frozen-source", "source"] {
            let q = r(name).join(f);
            fs::create_dir_all(q.parent().unwrap()).unwrap();
            fs::copy(&p, &q).unwrap();
        }
    }
    for name in ["frozen-source", "source"] {
        for (f, suffix) in suffixes {
            let p = r(name).join(f);
            let s = fs::read_to_string(&p).unwrap();
            fs::write(p, format!("{s}{suffix}")).unwrap();
        }
        fs::write(
            r(name).join("crates/zopro-core/src/msm/caller_probe.rs"),
            &adapter,
        )
        .unwrap();
    }
    let field_suffix="\n#[cfg(all(any(test, zopro_direct_ref_pack), target_arch=\"x86_64\", target_feature=\"avx2\", target_feature=\"avx512ifma\", target_feature=\"avx512vl\", target_feature=\"avx512dq\", not(no_ifma)))]\nmod direct_input;\n";
    let path = r("source/crates/zopro-bn254/src/lib.rs");
    let old = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{old}{field_suffix}")).unwrap();
    fs::copy(
        r("direct_input.rs"),
        r("source/crates/zopro-bn254/src/direct_input.rs"),
    )
    .unwrap();
    let bridge=format!("\n#[cfg({GUARD})]\nimpl FqBatch {{\n    #[inline(always)]\n    pub(crate) fn from_refs(values: [&Fq;8]) -> Self {{Self(zopro_bn254::ifma8::Ifma8::from_refs(values))}}\n}}\n");
    let path = r("source/crates/zopro-core/src/simd.rs");
    let old = fs::read_to_string(&path).unwrap();
    fs::write(&path, format!("{old}{bridge}")).unwrap();
    let path = r("source/crates/zopro-core/src/msm.rs");
    let old = fs::read_to_string(&path).unwrap();
    let start = old.find("fn stream_g1_impl<").unwrap();
    let lo = old[start..].find("    if s.write_groups {").unwrap() + start;
    let hi = old[lo..].find("    s.tail.clear();").unwrap() + lo;
    let original = &old[lo..hi];
    let mut forward = original.to_string();
    let changes=[("let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch_index::<SIGNED>(batch[k + i].0)]);","let a: [&Affine<C>; LANES] = core::array::from_fn(|i| &buckets[batch_index::<SIGNED>(batch[k + i].0)]);"),("let b: [Affine<C>; LANES] = core::array::from_fn(|i| *batch[k + i].1.borrow());","let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1.borrow());"),("FqBatch::new(&a.map(|p| p.x))","FqBatch::from_refs(a.map(|p| &p.x))"),("FqBatch::new(&a.map(|p| p.y))","FqBatch::from_refs(a.map(|p| &p.y))"),("FqBatch::new(&b.map(|p| p.x))","FqBatch::from_refs(b.map(|p| &p.x))"),("FqBatch::new(&b.map(|p| p.y))","FqBatch::from_refs(b.map(|p| &p.y))")];
    for (a, b) in changes {
        assert_eq!(forward.matches(a).count(), 2, "{a}");
        forward = forward.replace(a, b);
    }
    let helper=format!("//! Same forward arithmetic and records; direct referenced coordinate loads.\nuse super::*;\n#[inline(always)]\npub(super) fn forward<C:CurveConfig<Base=Fq>,const FAST:bool,const SIGNED:bool>(buckets:&[Affine<C>],batch:&[(u32,impl std::borrow::Borrow<Affine<C>>+Copy)],s:&mut FqAddScratch,full:usize)->FqBatch{{\nlet mut chains=FqBatch::one();\n{forward}\nchains\n}}\n");
    fs::write(r("g1_direct_input.rs"), &helper).unwrap();
    fs::write(
        r("source/crates/zopro-core/src/msm/direct_input.rs"),
        helper,
    )
    .unwrap();
    let replacement=format!("    #[cfg({GUARD})]\n    {{ chains = direct_input::forward::<C,FAST,SIGNED>(buckets,batch,s,full); }}\n    #[cfg(not({GUARD}))]\n    {{\n{original}    }}\n");
    let module = format!("\n#[cfg({GUARD})]\nmod direct_input;\n");
    let changed = format!("{}{}{}{module}", &old[..lo], replacement, &old[hi..]);
    fs::write(&path, changed).unwrap();
    save(
        r("source-audit.json"),
        &json!({"production_files":172,"frozen_originals_exact":170,"fresh_originals_exact":168,"core_suffixes":suffixes,"field_suffix":field_suffix,"simd_suffix":bridge,"msm_suffix":module,"old_forward":original,"new_forward":replacement,"forward_packing_substitutions":changes,"arithmetic_and_record_formulas_exact":true,"no_inverse_changes":true,"official_ifma_generator_unchanged":true}),
    );
    let flags = format!(
        "{} --cfg zopro_inverse_caller_probe",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    for name in ["frozen-source", "source"] {
        for f in m["source_files_sha256"].as_object().unwrap().keys() {
            let p = r(name).join(f);
            pins.insert(p.display().to_string(), hash(p));
        }
        let p = r(name).join("crates/zopro-core/src/msm/caller_probe.rs");
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "design.md",
        "native.rs",
        "native-helper",
        "direct_input.rs",
        "g1_direct_input.rs",
        "caller_probe.rs",
        "source-audit.json",
        "native-flags.txt",
        "probe.rs",
        "perf.rs",
        "source/crates/zopro-bn254/src/direct_input.rs",
        "source/crates/zopro-core/src/msm/direct_input.rs",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    let mut steps = vec![];
    for label in ["frozen", "fresh", "selected"] {
        let fullflags = if label == "selected" {
            format!("{flags} --cfg zopro_direct_ref_pack")
        } else {
            flags.clone()
        };
        let source = if label == "frozen" {
            "frozen-source"
        } else {
            "source"
        };
        let cargo = r(source).join("Cargo.toml");
        let env = json!({"RUSTFLAGS":fullflags,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16"});
        if label != "frozen" {
            steps.push(json!({"kind":"command","name":format!("{label}-field-tests"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":["direct_input::tests::direct_reference_words_match_split_for_unaligned_permuted_and_aliased_inputs"],"unfiltered":true,"timeout_seconds":1200}));
        }
        let mut args = vec![
            "cargo".into(),
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
            args.push("msm::caller_probe::tests::".into());
        }
        args.extend(["--".into(), "--test-threads=1".into()]);
        steps.push(json!({"kind":"command","name":format!("{label}-core-tests"),"argv":args,"environment":env,"required_tests":["msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse"],"unfiltered":label!="frozen","timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("{label}-build"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("{label}-freeze"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
    }
    steps.push(json!({"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180}));
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    audit();
    println!("Prepared3isolatednativevariants;noinverse/IFMAarithmeticchange");
}
fn audit() {
    let m = manifest();
    let a = read(r("source-audit.json"));
    for name in ["frozen-source", "source"] {
        let mut same = 0;
        for (f, h) in m["source_files_sha256"].as_object().unwrap() {
            assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
            let p = r(name).join(f);
            if hash(&p) == h.as_str().unwrap() {
                same += 1;
                continue;
            }
            let mut text = fs::read_to_string(p).unwrap();
            if name == "source" {
                if f == "crates/zopro-bn254/src/lib.rs" {
                    text = text
                        .strip_suffix(a["field_suffix"].as_str().unwrap())
                        .unwrap()
                        .to_string();
                }
                if f == "crates/zopro-core/src/simd.rs" {
                    text = text
                        .strip_suffix(a["simd_suffix"].as_str().unwrap())
                        .unwrap()
                        .to_string();
                }
                if f == "crates/zopro-core/src/msm.rs" {
                    text = text
                        .strip_suffix(a["msm_suffix"].as_str().unwrap())
                        .unwrap()
                        .to_string();
                    assert_eq!(text.matches(a["new_forward"].as_str().unwrap()).count(), 1);
                    text = text.replace(
                        a["new_forward"].as_str().unwrap(),
                        a["old_forward"].as_str().unwrap(),
                    );
                }
            }
            for v in a["core_suffixes"].as_array().unwrap() {
                if v[0] == *f {
                    text = text
                        .strip_suffix(v[1].as_str().unwrap())
                        .unwrap()
                        .to_string();
                }
            }
            assert_eq!(text, fs::read_to_string(Path::new(Z).join(f)).unwrap());
        }
        assert_eq!(same, if name == "source" { 168 } else { 170 });
    }
    println!(
        "All172productionexact;168fresh/170frozenoriginalsexact;inverseeditsrecoveralloriginals"
    );
}
fn freeze(label: &str) {
    let flags = std::env::var("RUSTFLAGS").unwrap();
    let base = fs::read_to_string(r("native-flags.txt")).unwrap();
    assert_eq!(
        flags,
        if label == "selected" {
            format!("{base} --cfg zopro_direct_ref_pack")
        } else {
            base
        }
    );
    let text = fs::read_to_string(r(format!("{}/{label}-build/stdout.log",if label=="frozen"{"native"}else{"native-rest"}))).unwrap();
    let root = r(format!("frozen-libraries/{label}"));
    fs::create_dir_all(&root).unwrap();
    let mut libs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut pins = BTreeMap::new();
    let mut macros = BTreeMap::new();
    for line in text.lines() {
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
            if p.extension().and_then(|s|s.to_str()) == Some("rlib") && ["zopro_core","zopro_bn254"].contains(&name) {
                let stem=p.file_stem().unwrap().to_str().unwrap();
                let cargo_name=name.replace('_',"-");
                let paths=if let Some(suffix)=stem.strip_prefix(&format!("lib{name}-")) {
                    vec![r(format!("target-{label}/release/.fingerprint/{cargo_name}-{suffix}/lib-{name}.json"))]
                }else{
                    assert_eq!(stem,format!("lib{name}"));
                    let dir=r(format!("target-{label}/release/.fingerprint"));
                    fs::read_dir(dir).unwrap().map(|e|e.unwrap()).filter(|e|e.file_name().to_string_lossy().starts_with(&format!("{cargo_name}-"))).map(|e|e.path().join(format!("lib-{name}.json"))).filter(|p|p.is_file()).collect::<Vec<_>>()
                };
                assert!(!paths.is_empty());let mut fingerprint=json!([]);
                for path in paths {let f=read(&path);assert_eq!(f["rustflags"],json!(flags.split_whitespace().collect::<Vec<_>>()));fingerprint.as_array_mut().unwrap().push(json!({"path":path,"receipt":f}));}
                save(r(format!("native/{label}-{name}-fingerprint.json")),&fingerprint);
            }
            let h = hash(&p);
            let ext = p.extension().unwrap().to_str().unwrap();
            let dest = root.join(format!("lib{name}-{h}.{ext}"));
            if dest.exists() {
                assert_eq!(hash(&dest), h)
            } else {
                fs::copy(&p, &dest).unwrap();
            }
            assert_eq!(hash(&dest), h);
            if ext == "rlib" {
                libs.entry(name.into())
                    .or_default()
                    .insert(dest.display().to_string());
                pins.insert(dest.display().to_string(), h);
            } else {
                macros.insert(dest.display().to_string(), h);
            }
        }
    }
    let mut required = BTreeMap::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "memmap2",
        "serde_json",
        "sha2",
        "libc",
        "rayon",
    ] {
        let found = libs.get(name).unwrap();
        assert_eq!(found.len(), 1, "{name}");
        required.insert(name, found.first().unwrap());
    }
    assert!(!macros.is_empty());
    save(
        r(format!("native/{label}-artifacts.json")),
        &json!({"rustflags":flags,"libraries":required,"all_libraries":libs,"input_pins":pins}),
    );
    save(
        r(format!("native/{label}-proc-macros.json")),
        &json!({"input_pins":macros,"repair_scope":"none;sharedobjects frozen withrlibs immediatelyafterbuild"}),
    );
    println!(
        "Froze{label}{}rlibs/{}proc-macros",
        pins.len(),
        macros.len()
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "setup" => setup(),
        "audit" => audit(),
        "freeze" => freeze(&a[2]),
        _ => panic!("mode"),
    }
}
