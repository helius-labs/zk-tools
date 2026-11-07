use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/common-z-callers176";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const F: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn copy(a: impl AsRef<Path>, b: impl AsRef<Path>) {
    fs::create_dir_all(b.as_ref().parent().unwrap()).unwrap();
    assert!(!b.as_ref().exists());
    fs::copy(a, b).unwrap();
}
fn edit(p: &str, from: &str, to: &str, edits: &mut Vec<Value>) {
    let path = r("source").join(p);
    let s = fs::read_to_string(&path).unwrap();
    assert_eq!(s.matches(from).count(), 1, "{p}: {from}");
    fs::write(&path, s.replacen(from, to, 1)).unwrap();
    edits.push(json!({"path":p,"before":from,"after":to}));
}
fn compile_env(mut argv: Vec<String>) -> Vec<String> {
    let mut out = vec![
        "/usr/bin/env".into(),
        "CARGO_PKG_NAME=zopro-core".into(),
        "CARGO_PKG_VERSION=0.1.0".into(),
        format!("CARGO_MANIFEST_DIR={R}/source/crates/zopro-core"),
    ];
    out.append(&mut argv);
    out
}
fn main() {
    assert!(!r("build-plan.json").exists());
    let manifest=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let originals = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 176);
    let mut baseline = BTreeMap::new();
    let mut pins = BTreeMap::new();
    for (p, h) in originals {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
        let old = Path::new(OLD).join("FROZEN-source").join(p);
        let bytes = fs::read(&old).unwrap();
        let original = fs::read(Path::new(Z).join(p)).unwrap();
        if [
            "crates/zopro-core/src/msm.rs",
            "crates/zopro-core/src/lib.rs",
        ]
        .contains(&p.as_str())
        {
            assert!(bytes.starts_with(&original));
        } else {
            assert_eq!(bytes, original);
        }
        copy(&old, r("source").join(p));
        baseline.insert(p.clone(), hash(&old));
        pins.insert(old.display().to_string(), hash(&old));
    }
    for p in [
        "crates/zopro-core/src/msm/g1_tree_records_probe.rs",
        "crates/zopro-core/src/msm/g1_weighted_caller.rs",
    ] {
        let old = Path::new(OLD).join("FROZEN-source").join(p);
        copy(&old, r("source").join(p));
        baseline.insert(p.into(), hash(&old));
        pins.insert(old.display().to_string(), hash(&old));
    }
    let msm = "crates/zopro-core/src/msm.rs";
    let mut edits = Vec::new();
    let cfg="#[cfg(all(zopro_common_z_finish,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
    edit(msm,"    // The first point of each pair the affine formula adds, in bucket order.\n",&format!("    #[cfg(zopro_common_z_finish)]\n    common_z: bool,\n    // The first point of each pair the affine formula adds, in bucket order.\n"),&mut edits);
    edit(msm,"            vector_inverse: crate::simd::cpu_option(\"ZOPRO_CPU_TREE_VECTOR_INVERSE\"),",&format!("            #[cfg(zopro_common_z_finish)]\n            common_z: C::Base::common_z_enabled(),\n            vector_inverse: crate::simd::cpu_option(\"ZOPRO_CPU_TREE_VECTOR_INVERSE\"),"),&mut edits);
    let prefix = "pub(super) trait BatchInverse: Field {\n";
    edit(msm,prefix,&format!("{prefix}    #[cfg(zopro_common_z_finish)]\n    fn common_z_enabled()->bool {{false}}\n    #[cfg(zopro_common_z_finish)]\n    fn finish_common_z<C:CurveConfig<Base=Self>>(_p:&mut[LimbPoint<Self>],_s:&[usize],_n:&mut[usize])->bool {{false}}\n"),&mut edits);
    let prefix = "impl BatchInverse for Fq {\n";
    edit(msm,prefix,&format!("{prefix}    {cfg}\n    fn common_z_enabled()->bool {{crate::simd::cpu_option(\"ZOPRO_CPU_COMMON_Z_FINISH\")}}\n    {cfg}\n    fn finish_common_z<C:CurveConfig<Base=Self>>(p:&mut[LimbPoint<Self>],s:&[usize],n:&mut[usize])->bool {{common_z_finish::finish::<C>(p,s,n)}}\n"),&mut edits);
    let before="                while local_counts.iter().any(|&count| count > 1) {\n                    reduce_layer(points, local_starts, local_counts, scratch);\n";
    edit(msm,before,"                while local_counts.iter().any(|&count| count > 1) {\n                    #[cfg(zopro_common_z_finish)]\n                    if scratch.common_z && C::Base::finish_common_z::<C>(points,local_starts,local_counts) {break;}\n                    reduce_layer(points, local_starts, local_counts, scratch);\n",&mut edits);
    let before="            while count[0] > 1 {\n                reduce_layer(&mut points, &[0], &mut count, &mut scratch);\n";
    edit(msm,before,"            while count[0] > 1 {\n                #[cfg(zopro_common_z_finish)]\n                if scratch.common_z && C::Base::finish_common_z::<C>(&mut points,&[0],&mut count) {break;}\n                reduce_layer(&mut points, &[0], &mut count, &mut scratch);\n",&mut edits);
    let source = fs::read_to_string(r("source").join(msm)).unwrap();
    let suffix = format!("\n{cfg}\nmod common_z_finish;\n");
    fs::write(r("source").join(msm), format!("{source}{suffix}")).unwrap();
    edits.push(json!({"path":msm,"before":"","append":suffix}));
    let probe = "crates/zopro-core/src/msm/g1_tree_records_probe.rs";
    let before =
        "        while self.counts.iter().any(|&n| n > 1) {\n            reduce_layer::<G1Config>(";
    edit(probe,before,"        while self.counts.iter().any(|&n| n > 1) {\n            #[cfg(zopro_common_z_finish)]\n            if self.scratch.common_z && <Fq as BatchInverse>::finish_common_z::<G1Config>(&mut self.points,starts,&mut self.counts) {break;}\n            reduce_layer::<G1Config>(",&mut edits);
    copy(
        r("finish.rs"),
        r("source/crates/zopro-core/src/msm/common_z_finish.rs"),
    );
    copy(
        r("segmented.rs"),
        r("source/crates/zopro-core/src/msm/common_z_segmented.rs"),
    );
    let mut hashes = BTreeMap::new();
    for (p, h) in &baseline {
        let path = r("source").join(p);
        let mut bytes = fs::read(&path).unwrap();
        for e in edits.iter().rev().filter(|e| e["path"] == *p) {
            let mut s = String::from_utf8(bytes).unwrap();
            if let Some(append) = e["append"].as_str() {
                assert!(s.ends_with(append));
                s.truncate(s.len() - append.len());
            } else {
                let from = e["after"].as_str().unwrap();
                assert_eq!(s.matches(from).count(), 1);
                s = s.replacen(from, e["before"].as_str().unwrap(), 1);
            }
            bytes = s.into_bytes();
        }
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), *h);
        hashes.insert(p.clone(), hash(&path));
        pins.insert(path.display().to_string(), hash(path));
    }
    for p in [
        "crates/zopro-core/src/msm/common_z_finish.rs",
        "crates/zopro-core/src/msm/common_z_segmented.rs",
    ] {
        hashes.insert(p.into(), hash(r("source").join(p)));
        pins.insert(
            r("source").join(p).display().to_string(),
            hash(r("source").join(p)),
        );
    }
    save(
        r("source-audit.json"),
        &json!({"accepted_source_files":176,"original_diagnostic_files":178,"candidate_source_files":180,"baseline_source_hashes":baseline,"candidate_source_hashes":hashes,"all178_recover":true,"edits":edits}),
    );
    // Reuse the qualified TLS harness, changing only cases/labels.
    let old_driver = fs::read_to_string(format!("{OLD}/driver-tls.rs")).unwrap();
    let driver=old_driver.replacen("        \"tile257\" =>", "        \"small16\" => (vec![16],false,false),\n        \"groups3x4\" => (vec![3;4],false,false),\n        \"small-mixed\" => (vec![3,4,0,1,2],false,false),\n        \"tile257\" =>",1).replace("[\"frozen\", \"OFF\", \"ON\"]","[\"frozen\", \"fresh\", \"OFF\", \"ON\"]");
    assert_ne!(driver, old_driver);
    fs::write(r("driver.rs"), driver).unwrap();
    copy(format!("{OLD}/perf_support.rs"), r("perf_support.rs"));
    let old = read(format!("{OLD}/build-runs/core-frozen/command.json"));
    let old_args = old["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|a| a.as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let mut steps = Vec::new();
    let mut all_args = Vec::new();
    for arm in ["fresh", "candidate"] {
        let out = r(&format!("libs/{arm}"));
        fs::create_dir_all(&out).unwrap();
        let mut args = old_args
            .iter()
            .map(|a| {
                a.replace(&format!("{OLD}/FROZEN-source/"), &format!("{R}/source/"))
                    .replace(&format!("{OLD}/libs/frozen"), &out.display().to_string())
            })
            .collect::<Vec<_>>();
        if arm == "candidate" {
            args.extend(["--cfg".into(), "zopro_common_z_finish".into()]);
        }
        let mut env = old["environment"].clone();
        for k in ["CARGO_PKG_NAME", "CARGO_PKG_VERSION", "CARGO_MANIFEST_DIR"] {
            env.as_object_mut().unwrap().remove(k);
        }
        env["ZOPRO_CPU_COMMON_Z_FINISH"] = json!("1");
        steps.push(json!({"kind":"command","name":format!("core-{arm}"),"argv":compile_env(args.clone()),"environment":env,"timeout_seconds":900}));
        all_args.push(args);
    }
    let old_test = read(
        "/home/ubuntu/src/zopro-perf-work/g1-tree-records176/runs/build-tests-native/command.json",
    );
    let mut tests = old_test["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|a| {
            a.as_str()
                .unwrap()
                .replace(
                    "/home/ubuntu/src/zopro-perf-work/g1-tree-records176/source/",
                    &format!("{R}/source/"),
                )
                .replace(
                    "/home/ubuntu/src/zopro-perf-work/g1-tree-records176/libs/native",
                    &format!("{R}/libs/candidate"),
                )
                .replace(
                    "/home/ubuntu/src/zopro-perf-work/g1-tree-records176/core-tests-native",
                    &format!("{R}/core-tests"),
                )
        })
        .collect::<Vec<_>>();
    let pos = tests
        .iter()
        .position(|s| s == "zopro_g1_tree_records")
        .unwrap();
    assert_eq!(tests[pos - 1], "--cfg");
    tests.drain(pos - 1..=pos);
    tests.extend(["--cfg".into(), "zopro_common_z_finish".into()]);
    steps.push(json!({"kind":"command","name":"build-changed-core-tests","argv":compile_env(tests),"environment":old["environment"],"timeout_seconds":900}));
    steps.push(json!({"kind":"command","name":"changed-core-tests","argv":[r("core-tests"),PathBuf::from("--test-threads=1"),PathBuf::from("--nocapture")],"environment":{"ZOPRO_CPU_COMMON_Z_FINISH":"1"},"timeout_seconds":900,"unfiltered":true,"required_tests":["msm::common_z_finish::tests::segmented_frontier_preserves_outputs_and_transactional_fallback","msm::common_z_finish::tests::production_grouping_and_indexed_paths_preserve_complete_results"]}));
    let consumers = read(format!("{OLD}/consumer-tls-plan.json"));
    let base = &consumers["steps"][0]["argv"].as_array().unwrap();
    for arm in ["frozen", "fresh", "candidate"] {
        let out = if arm == "frozen" {
            format!("{OLD}/libs/frozen")
        } else {
            format!("{R}/libs/{arm}")
        };
        let args = base
            .iter()
            .map(|v| {
                v.as_str()
                    .unwrap()
                    .replace(&format!("{OLD}/libs/frozen"), &out)
                    .replace(&format!("{OLD}/driver-tls.rs"), &format!("{R}/driver.rs"))
                    .replace(
                        &format!("{OLD}/caller-frozen-tls"),
                        &format!("{R}/caller-{arm}"),
                    )
            })
            .collect::<Vec<_>>();
        steps.push(json!({"kind":"command","name":format!("consumer-{arm}"),"argv":args,"timeout_seconds":900}));
    }
    for args in &all_args {
        for a in args {
            if let Some((_, p)) = a.split_once('=') {
                if p.ends_with(".rlib") && Path::new(p).exists() {
                    pins.insert(p.into(), hash(p));
                }
            }
        }
    }
    for p in [
        format!("{OLD}/libs/frozen/libzopro_core.rlib"),
        format!("{OLD}/driver-tls.rs"),
        format!("{OLD}/perf_support.rs"),
        format!("{OLD}/consumer-tls-plan.json"),
        format!("{OLD}/build-runs/core-frozen/command.json"),
        "/home/ubuntu/src/zopro-perf-work/common-z-small-native176/kernel.rs".into(),
        "/home/ubuntu/src/zopro-perf-work/common-z-small-native176/qualification.json".into(),
        "/home/ubuntu/src/zopro-perf-work/g1-tree-records176/runs/build-tests-native/command.json"
            .into(),
    ] {
        pins.insert(p.clone(), hash(p));
    }
    for p in [
        "design.md",
        "finish.rs",
        "segmented.rs",
        "setup.rs",
        "setup",
        "driver.rs",
        "perf_support.rs",
        "source-audit.json",
    ] {
        let path = r(p);
        pins.insert(path.display().to_string(), hash(&path));
    }
    for name in ["serde_json", "sha2", "libc", "rayon"] {
        let p = fs::read_dir(F)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(&format!("lib{name}-"))
                    && p.extension().unwrap_or_default() == "rlib"
            })
            .unwrap();
        pins.insert(p.display().to_string(), hash(&p));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared180source files; all178 diagnostic originals recover;176 accepted originals match;{} build/test steps",steps.len());
}
