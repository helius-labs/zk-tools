use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor";
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
        fs::copy(&p, q).unwrap();
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
    }
    let mut edits = vec![];
    let path = r("source/crates/zopro-core/src/msm.rs");
    let mut s = fs::read_to_string(&path).unwrap();
    let constructor = "                write_groups: store_mode == 2,\n";
    let latched="                weighted_anchor: cfg!(all(zopro_g1_weighted_anchor,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))),\n";
    let signature="fn stream_g1_impl<C: CurveConfig<Base = Fq>, const FAST: bool, const SIGNED: bool>(\n    buckets: &mut [Affine<C>],\n    batch: &[(u32, impl std::borrow::Borrow<Affine<C>> + Copy)],\n    s: &mut FqAddScratch,\n) {\n";
    let wrapped=format!("#[inline(always)]\n{signature}    if s.weighted_anchor {{ stream_g1_anchor_kernel::<C,FAST,SIGNED,true>(buckets,batch,s) }}\n    else {{ stream_g1_anchor_kernel::<C,FAST,SIGNED,false>(buckets,batch,s) }}\n}}\nfn stream_g1_anchor_kernel<C: CurveConfig<Base=Fq>,const FAST:bool,const SIGNED:bool,const WEIGHTED:bool>(\n    buckets:&mut[Affine<C>],batch:&[(u32,impl std::borrow::Borrow<Affine<C>>+Copy)],s:&mut FqAddScratch,\n) {{\n");
    for(old,new,count)in[
        (constructor.to_string(),format!("{constructor}{latched}"),1),
        ("struct FqAddScratch {\n".to_string(),"struct FqAddScratch {\n    weighted_anchor: bool,\n".to_string(),1),
        (signature.to_string(),wrapped,1),
        ("                core::ptr::addr_of_mut!((*slot).ax).write(ax);".to_string(),"                if !WEIGHTED { core::ptr::addr_of_mut!((*slot).ax).write(ax); }".to_string(),1),
        ("                core::ptr::addr_of_mut!((*slot).xsum).write(canonical_add::<FAST>(&ax, &bx));".to_string(),"                let xsum=canonical_add::<FAST>(&ax,&bx);\n                core::ptr::addr_of_mut!((*slot).xsum).write(xsum);\n                if WEIGHTED { core::ptr::addr_of_mut!((*slot).ax).write(ax.add(&xsum)); }".to_string(),1),
        ("            s.groups.push(FqAddGroup {\n                ax,\n                ay,\n                xsum: canonical_add::<FAST>(&ax, &bx),".to_string(),"            let xsum=canonical_add::<FAST>(&ax,&bx);\n            let anchor=if WEIGHTED {ax.add(&xsum)}else{ax};\n            s.groups.push(FqAddGroup {\n                ax:anchor,\n                ay,\n                xsum,".to_string(),1),
        ("        let x = slope.mul(&slope).sub(&g.xsum);\n        let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);".to_string(),"        let square=slope.mul(&slope);\n        let x=square.sub(&g.xsum);\n        let delta=if WEIGHTED {g.ax.sub(&square)}else{g.ax.sub(&x)};\n        let y=slope.mul(&delta).sub(&g.ay);".to_string(),1),
    ]{assert_eq!(s.matches(&old).count(),count,"{old}");s=s.replace(&old,&new);edits.push(json!({"old":old,"new":new}));}
    let suffix="\n#[cfg(zopro_anchor_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_anchor_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1};\n";
    fs::write(&path, format!("{s}{suffix}")).unwrap();
    let mut back = s;
    for e in edits.iter().rev() {
        back = back.replace(e["new"].as_str().unwrap(), e["old"].as_str().unwrap());
    }
    assert_eq!(
        back,
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap()
    );
    let core_suffix="\n#[cfg(zopro_anchor_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1};\n";
    let p = r("source/crates/zopro-core/src/lib.rs");
    let s = fs::read_to_string(&p).unwrap();
    fs::write(p, format!("{s}{core_suffix}")).unwrap();
    let adapter =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/direct-ref-pack/caller_probe.rs")
            .unwrap();
    fs::write(
        r("caller_probe.rs"),
        format!("{adapter}\ninclude!(\"anchor_tests.rs\");\n"),
    )
    .unwrap();
    fs::copy(
        r("caller_probe.rs"),
        r("source/crates/zopro-core/src/msm/caller_probe.rs"),
    )
    .unwrap();
    fs::copy(
        r("anchor_tests.rs"),
        r("source/crates/zopro-core/src/msm/anchor_tests.rs"),
    )
    .unwrap();
    save(
        r("source-audit.json"),
        &json!({"production_files":172,"private_originals_exact":170,"msm_edits":edits,"msm_suffix":suffix,"core_suffix":core_suffix,"field_and_generated_arithmetic_unchanged":true,"group_record_layout_unchanged":true,"original_msm_recovered_exact":true,"runtime_change":"precompute T=2Ax+Bx in Ax slot, Y from T-square, one private latched bool/dispatch"}),
    );
    let flags = format!(
        "{} --cfg zopro_anchor_caller_probe --cfg zopro_g1_weighted_anchor",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        let p = r("source").join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "design.md",
        "native.rs",
        "native-helper",
        "anchor_tests.rs",
        "caller_probe.rs",
        "probe.rs",
        "perf.rs",
        "source-audit.json",
        "native-flags.txt",
        "source/crates/zopro-core/src/msm/caller_probe.rs",
        "source/crates/zopro-core/src/msm/anchor_tests.rs",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let env = json!({"RUSTFLAGS":flags,"CARGO_TARGET_DIR":r("target"),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
    let steps = vec![
        json!({"kind":"command","name":"core-tests","argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["msm::caller_probe::weighted_anchor_tests::vector_y_anchor_matches_scalar_field_graph","msm::caller_probe::weighted_anchor_tests::alternating_anchor_modes_preserve_owned_borrowed_tails_and_scratch","msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse","msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse"],"unfiltered":true,"timeout_seconds":1200}),
        json!({"kind":"command","name":"libraries","argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}),
        json!({"kind":"command","name":"freeze","argv":[r("native-helper"),PathBuf::from("freeze")],"environment":env,"timeout_seconds":180}),
    ];
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared accepted172copy,170originalsexact;defaultweightedONfullcoretests,onecommon-image librarybuild");
}
fn freeze() {
    let expected = fs::read_to_string(r("native-flags.txt")).unwrap();
    assert_eq!(std::env::var("RUSTFLAGS").unwrap(), expected);
    let log = fs::read_to_string(r("native/libraries/stdout.log")).unwrap();
    let root = r("frozen");
    fs::create_dir_all(&root).unwrap();
    let mut libraries: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
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
            let path = if file.ends_with(".rlib") && !file.contains("/deps/") && metas.len() == 1 {
                Path::new(metas[0]).with_extension("rlib")
            } else {
                PathBuf::from(file)
            };
            let h = hash(&path);
            let ext = path.extension().unwrap().to_str().unwrap();
            let q = root.join(format!("lib{name}-{h}.{ext}"));
            if q.exists() {
                assert_eq!(hash(&q), h)
            } else {
                fs::copy(&path, &q).unwrap();
            }
            assert_eq!(hash(&q), h);
            pins.insert(q.display().to_string(), h);
            if ext == "rlib" {
                libraries
                    .entry(name.into())
                    .or_default()
                    .insert(q.display().to_string());
            }
        }
    }
    let mut selected = BTreeMap::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "libc",
        "rayon",
        "memmap2",
        "serde_json",
        "sha2",
    ] {
        let x = libraries.get(name).unwrap();
        assert_eq!(x.len(), 1, "{name}");
        selected.insert(name, x.first().unwrap());
    }
    let mut fingerprints = vec![];
    for e in fs::read_dir(r("target/release/.fingerprint")).unwrap() {
        let e = e.unwrap();
        let name = e.file_name().to_string_lossy().to_string();
        for crate_name in ["zopro-core", "zopro-bn254"] {
            if name.starts_with(&format!("{crate_name}-")) {
                let p = e
                    .path()
                    .join(format!("lib-{}.json", crate_name.replace('-', "_")));
                if p.is_file() {
                    let f = read(&p);
                    assert_eq!(
                        f["rustflags"],
                        json!(expected.split_whitespace().collect::<Vec<_>>())
                    );
                    fingerprints.push(json!({"path":p,"receipt":f}));
                }
            }
        }
    }
    assert_eq!(fingerprints.len(), 2);
    save(
        r("artifacts.json"),
        &json!({"rustflags":expected,"libraries":selected,"all_libraries":libraries,"input_pins":pins,"fingerprints":fingerprints,"freeze_scope":"allrlib/so immediatelyafterisolatedbuild;unhashedcoreallowed,hashedmetacompanionforprimarywhereavailable"}),
    );
    println!(
        "Froze{}rlib/sofiles; verifiedfield/coreflags;oneimageordinary/weighted",
        pins.len()
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "setup" => setup(),
        "freeze" => freeze(),
        _ => panic!("mode"),
    }
}
