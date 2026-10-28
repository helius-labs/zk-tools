use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
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
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_string());
        for dir in ["source", "source-frozen"] {
            let q = r(dir).join(f);
            fs::create_dir_all(q.parent().unwrap()).unwrap();
            fs::copy(&p, q).unwrap();
        }
    }
    let cfg="all(zopro_g2_borrow_queue,zopro_g2_horner_buckets,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let diag="all(zopro_g2_queue_probe,zopro_g2_horner_buckets,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
    let affine_diag=format!("\n#[cfg({diag})]\npub(crate)fn queue_probe_g2(bases:&[zopro_bn254::G2Affine],words:&mut[[u64;4]])->zopro_bn254::G2Projective{{g2_phi::raw_exclusive(bases,words)}}\n");
    let core_diag=format!("\n#[cfg({diag})]\npub fn queue_probe_g2(bases:&[zopro_bn254::G2Affine],words:&mut[[u64;4]])->zopro_bn254::G2Projective{{msm_affine::queue_probe_g2(bases,words)}}\n");
    let mut suffixes = vec![];
    for dir in ["source", "source-frozen"] {
        for (f, suffix) in [
            ("crates/zopro-core/src/msm_affine.rs", &affine_diag),
            ("crates/zopro-core/src/lib.rs", &core_diag),
        ] {
            let p = r(dir).join(f);
            let old = fs::read_to_string(&p).unwrap();
            fs::write(p, format!("{old}{suffix}")).unwrap();
            if dir == "source" {
                suffixes.push(json!({"file":f,"suffix":suffix}));
            }
        }
    }
    for (f, suffix) in [
        (
            "crates/zopro-core/src/msm.rs",
            format!("\n#[cfg({cfg})]\nmod g2_borrowed;\n"),
        ),
        (
            "crates/zopro-core/src/msm_affine.rs",
            format!("\n#[cfg({cfg})]\nmod g2_borrowed;\n"),
        ),
    ] {
        let p = r("source").join(f);
        let s = fs::read_to_string(&p).unwrap();
        fs::write(p, format!("{s}{suffix}")).unwrap();
        suffixes.push(json!({"file":f,"suffix":suffix}));
    }
    let f = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let old = "    let size = batch_size(bits).max(32);";
    let new=format!("    #[cfg({cfg})]\n    if let Some(value)=super::g2_borrowed::try_window(bases,short,w,bits){{return value;}}\n{old}");
    let p = r("source").join(f);
    let s = fs::read_to_string(&p).unwrap();
    assert_eq!(s.matches(old).count(), 1);
    fs::write(p, s.replace(old, &new)).unwrap();
    for (from, to) in [
        (
            "g2_borrowed_prefix.rs",
            "crates/zopro-core/src/msm/g2_borrowed.rs",
        ),
        (
            "g2_borrowed_window.rs",
            "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
        ),
    ] {
        fs::copy(r(from), r("source").join(to)).unwrap();
    }
    let mut exact = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = r("source").join(f);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
            continue;
        }
        let mut s = fs::read_to_string(&p).unwrap();
        if f == "crates/zopro-core/src/msm_affine/g2_phi.rs" {
            s = s.replace(&new, old);
        }
        for x in suffixes.iter().rev().filter(|v| v["file"] == *f) {
            s = s
                .strip_suffix(x["suffix"].as_str().unwrap())
                .unwrap()
                .to_owned();
        }
        assert_eq!(s, fs::read_to_string(Path::new(Z).join(f)).unwrap());
    }
    assert_eq!(exact, 168);
    let frozen_exact = m["source_files_sha256"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(f, h)| hash(r("source-frozen").join(f)) == h.as_str().unwrap())
        .count();
    assert_eq!(frozen_exact, 170);
    save(
        r("source-audit.json"),
        &json!({"original_files":172,"private_originals_exact":exact,"frozen_originals_exact":frozen_exact,"reverse_all172_exact":true,"field_unchanged":true,"runtime_dispatch":{"file":f,"old":old,"new":new},"suffixes":suffixes,"frozen_suffixes":{"msm_affine":affine_diag,"lib":core_diag},"owned_entry_bytes":136,"borrowed_entry_bytes":16,"removed_unused_scalar_buffers":2,"new_key_tables":0}),
    );
    let flags = format!(
        "{} --cfg zopro_g2_queue_probe",
        m["rustflags"].as_str().unwrap()
    );
    fs::write(r("native-flags.txt"), &flags).unwrap();
    for dir in ["source", "source-frozen"] {
        for f in m["source_files_sha256"].as_object().unwrap().keys() {
            let p = r(dir).join(f);
            pins.insert(p.display().to_string(), hash(p));
        }
    }
    for f in [
        "design.md",
        "make-prefix.rs",
        "make-prefix",
        "native-prefix.rs",
        "native.rs",
        "native-helper",
        "make-native.rs",
        "make-native",
        "g2_borrowed_prefix.rs",
        "g2_borrowed_window.rs",
        "source-audit.json",
        "native-flags.txt",
        "source/crates/zopro-core/src/msm/g2_borrowed.rs",
        "source/crates/zopro-core/src/msm_affine/g2_borrowed.rs",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let mut steps = vec![];
    for label in ["frozen", "off", "on"] {
        let full = if label == "on" {
            format!("{flags} --cfg zopro_g2_borrow_queue")
        } else {
            flags.clone()
        };
        let env = json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16","RAYON_NUM_THREADS":"4","ZOPRO_CPU_PACKED":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
        let cargo = r(if label == "frozen" {
            "source-frozen/Cargo.toml"
        } else {
            "source/Cargo.toml"
        });
        if label != "frozen" {
            let mut required=vec!["msm_affine::g2_phi::tests::decomposition_and_endomorphism_match_independent_field_oracles"];
            if label == "on" {
                required.extend(["msm::g2_borrowed::tests::signed_borrowed_prefix_tails_and_reuse","msm_affine::g2_borrowed::tests::full_width_phi_msm_matches_generator_sum","msm_affine::g2_borrowed::tests::exceptional_keys_and_scalar_edges_match_generator_sum"]);
            }
            steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1200}));
        }
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared8nativebuildtestfreeze steps;coreOFF119/ON122;newfrozenMSMdiagnostic;fieldunchanged");
}
