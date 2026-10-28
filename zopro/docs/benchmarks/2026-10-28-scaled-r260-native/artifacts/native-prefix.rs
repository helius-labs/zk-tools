use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native";
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
    let cfg="all(zopro_g2_scaled_r260,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let mut edits = vec![];
    let mut edit = |f: &str, old: String, new: String| {
        let p = r("source").join(f);
        let s = fs::read_to_string(&p).unwrap();
        assert_eq!(s.matches(&old).count(), 1, "{f}");
        fs::write(p, s.replace(&old, &new)).unwrap();
        edits.push(json!({"file":f,"old":old,"new":new}));
    };
    edit("crates/zopro-core/src/msm/g2_prefix.rs","    let full=batch.len()/LANES*LANES;".into(),format!("    #[cfg({cfg})]\n    if FAST && batch.len()%LANES==0 {{return g2_scaled::apply(buckets,batch,&mut s.scaled_prefix_records);}}\n    let full=batch.len()/LANES*LANES;"));
    edit("crates/zopro-core/src/msm.rs","    tail: Vec<[Fq2; 5]>,".into(),format!("    #[cfg({cfg})]\n    scaled_prefix_records:Vec<g2_scaled::Record>,\n    tail: Vec<[Fq2; 5]>,"));
    let msm_suffix=format!("\n#[cfg({cfg})]\nmod g2_scaled;\n#[cfg(zopro_scaled_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_scaled_caller_probe)]\npub use caller_probe::{{Caller,CallerG1,CallerG2,BorrowedG1}};\n");
    let lib_suffix="\n#[cfg(zopro_scaled_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1};\n".to_owned();
    let field_suffix =
        "\n#[cfg(zopro_g2_scaled_r260)]\n#[path=\"scaled.rs\"]\npub mod scaled;\n".to_owned();
    let mut suffixes = vec![];
    for (f, suffix) in [
        ("crates/zopro-core/src/msm.rs", msm_suffix),
        ("crates/zopro-core/src/lib.rs", lib_suffix),
        ("crates/zopro-bn254/src/ifma8.rs", field_suffix),
    ] {
        let p = r("source").join(f);
        let s = fs::read_to_string(&p).unwrap();
        fs::write(p, format!("{s}{suffix}")).unwrap();
        suffixes.push(json!({"file":f,"suffix":suffix}));
    }
    for (from, to) in [
        ("scaled.rs", "crates/zopro-bn254/src/scaled.rs"),
        ("g2_scaled.rs", "crates/zopro-core/src/msm/g2_scaled.rs"),
    ] {
        fs::copy(r(from), r("source").join(to)).unwrap();
    }
    let original =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/direct-ref-pack/caller_probe.rs")
            .unwrap();
    let old = "            + self.0.scratch.fq2.tail.capacity() * std::mem::size_of::<[Fq2; 5]>()";
    let new=format!("{old}\n            + {{#[cfg({cfg})] {{self.0.scratch.fq2.scaled_prefix_records.capacity()*std::mem::size_of::<g2_scaled::Record>()}} #[cfg(not({cfg}))] {{0}}}}");
    assert_eq!(original.matches(old).count(), 1);
    fs::write(
        r("source/crates/zopro-core/src/msm/caller_probe.rs"),
        original.replace(old, &new),
    )
    .unwrap();
    let mut exact = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = r("source").join(f);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
            continue;
        }
        let mut s = fs::read_to_string(&p).unwrap();
        for suffix in suffixes.iter().rev().filter(|v| v["file"] == *f) {
            s = s
                .strip_suffix(suffix["suffix"].as_str().unwrap())
                .unwrap()
                .to_string();
        }
        for e in edits.iter().rev().filter(|v| v["file"] == *f) {
            let new = e["new"].as_str().unwrap();
            assert_eq!(s.matches(new).count(), 1);
            s = s.replace(new, e["old"].as_str().unwrap());
        }
        assert_eq!(s, fs::read_to_string(Path::new(Z).join(f)).unwrap());
    }
    assert_eq!(exact, 168);
    save(
        r("source-audit.json"),
        &json!({"production_files":172,"private_originals_exact":exact,"reverse_all172_exact":true,"edits":edits,"suffixes":suffixes,"adapter_capacity_edit":{"old":old,"new":new},"old_prefix_body_verbatim_after_cfg_dispatch":true,"old_field_runtime_verbatim":true,"new_native_field_api":"bounded child module only","scratch_optional_vec_bytes":24,"record_bytes":640,"possible_old_and_new_capacity_coexistence":true}),
    );
    let flags = format!(
        "{} --cfg zopro_scaled_caller_probe",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    for f in m["source_files_sha256"].as_object().unwrap().keys() {
        let p = r("source").join(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    for f in [
        "design.md",
        "native-prefix.rs",
        "native.rs",
        "native-helper",
        "make-native.rs",
        "make-native",
        "scaled.rs",
        "g2_scaled.rs",
        "wide-check.rs",
        "source-audit.json",
        "native-flags.txt",
        "source/crates/zopro-bn254/src/scaled.rs",
        "source/crates/zopro-core/src/msm/g2_scaled.rs",
        "source/crates/zopro-core/src/msm/caller_probe.rs",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let mut steps = vec![];
    let checked = json!({"RUSTFLAGS":format!("{flags} --cfg zopro_g2_scaled_r260 -C debug-assertions=yes -C overflow-checks=yes"),"CARGO_TARGET_DIR":r("target-checked"),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4"});
    steps.push(json!({"kind":"command","name":"field-checked","argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":checked,"required_tests":["inverse::tests::fq","inverse::tests::fr","inverse::tests::one_word_finish"],"unfiltered":true,"timeout_seconds":1200}));
    for label in ["off", "on"] {
        let full = if label == "on" {
            format!("{flags} --cfg zopro_g2_scaled_r260")
        } else {
            flags.clone()
        };
        let env = json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        let mut required = vec![
            "msm::caller_probe::tests::owned_g1_generator_oracle_tails_and_reuse",
            "msm::caller_probe::tests::borrowed_signed_g1_generator_oracle_tails_and_reuse",
            "msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse",
            "msm::g2_prefix::tests::g2_prefix_complete_curve_tails_reuse",
        ];
        if label == "on" {
            required.extend([
                "msm::g2_scaled::tests::full_width_generator_outputs_and_reuse",
                "msm::g2_scaled::tests::whole_call_exception_fallback_before_vector_mutation",
            ]);
        }
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared7native steps;checkedfield33/core122OFF124ON;168exact/reverse172;originalOFFbodypreserved");
}
