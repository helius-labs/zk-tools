use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-half-roots176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const B: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const H: &str = "/home/ubuntu/src/zopro-perf-work/high-borrow176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: &str, v: &Value) {
    fs::write(format!("{R}/{p}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("source").exists());
    let manifest_path=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let m = read(&manifest_path);
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 176);
    let cfg="#[cfg(all(feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
    let before="        let products = quads.get();\n        let inverse = (products[0] * products[4]).inverse().unwrap();";
    let after="        #[cfg(zopro_inverse_half_roots)]\n        let inverse = {\n            let (low, high) = quads.0.halves();\n            (low.get()[0] * high.get()[0]).inverse().unwrap()\n        };\n        #[cfg(not(zopro_inverse_half_roots))]\n        let inverse = {\n            let products = quads.get();\n            (products[0] * products[4]).inverse().unwrap()\n        };";
    let suffixes=BTreeMap::from([("crates/zopro-core/src/msm.rs".to_owned(),format!("\n{cfg}\npub mod half_roots_caller;\n")),("crates/zopro-core/src/lib.rs".to_owned(),format!("\n{cfg}\n#[doc(hidden)]pub use msm::half_roots_caller::{{G1Caller,G2Caller}};\n")),("crates/zopro-core/src/simd.rs".to_owned(),format!("\n{cfg}\n#[cfg(test)]mod half_roots_native {{ include!(\"simd/half_roots_tests.rs\"); }}\n"))]);
    let mut pins = BTreeMap::new();
    let mut hashes = BTreeMap::new();
    let mut exact = 0;
    for (p, h) in originals {
        let from = Path::new(Z).join(p);
        let original = fs::read(&from).unwrap();
        assert_eq!(hash(&from), h.as_str().unwrap());
        let mut bytes = original.clone();
        if p == "crates/zopro-core/src/simd.rs" {
            let s = String::from_utf8(bytes).unwrap();
            assert_eq!(s.matches(before).count(), 1);
            bytes = s.replacen(before, after, 1).into_bytes();
        }
        if let Some(s) = suffixes.get(p) {
            bytes.extend_from_slice(s.as_bytes());
        }
        if bytes == original {
            exact += 1;
        }
        let dest = Path::new(R).join("source").join(p);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::write(&dest, &bytes).unwrap();
        hashes.insert(p.clone(), hash(&dest));
        pins.insert(dest.display().to_string(), hash(&dest));
        let mut recovered = bytes;
        if let Some(s) = suffixes.get(p) {
            assert!(recovered.ends_with(s.as_bytes()));
            recovered.truncate(recovered.len() - s.len());
        }
        if p == "crates/zopro-core/src/simd.rs" {
            let s = String::from_utf8(recovered).unwrap();
            assert_eq!(s.matches(after).count(), 1);
            recovered = s.replacen(after, before, 1).into_bytes();
        }
        assert_eq!(recovered, original);
    }
    assert_eq!(exact, 173);
    let g1 = fs::read_to_string(format!("{H}/g1-caller.rs")).unwrap();
    let g2 = fs::read_to_string(format!("{H}/g2-caller.rs")).unwrap();
    let child=format!("use super::*;use zopro_bn254::{{G1Affine,G1Config,G1Projective,G2Affine,G2Config,G2Projective}};\n{g1}\n{g2}\n");
    let p = Path::new(R).join("source/crates/zopro-core/src/msm/half_roots_caller.rs");
    fs::write(&p, &child).unwrap();
    hashes.insert(
        "crates/zopro-core/src/msm/half_roots_caller.rs".into(),
        hash(&p),
    );
    pins.insert(p.display().to_string(), hash(&p));
    let p = Path::new(R).join("source/crates/zopro-core/src/simd/half_roots_tests.rs");
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::copy(format!("{R}/tests.rs"), &p).unwrap();
    hashes.insert(
        "crates/zopro-core/src/simd/half_roots_tests.rs".into(),
        hash(&p),
    );
    pins.insert(p.display().to_string(), hash(&p));
    assert_eq!(hashes.len(), 178);
    let old_driver = fs::read_to_string(format!("{H}/driver.rs")).unwrap();
    assert_eq!(old_driver.matches("zopro_high_borrow").count(), 1);
    fs::write(
        format!("{R}/driver.rs"),
        old_driver.replacen("zopro_high_borrow", "zopro_inverse_half_roots", 1),
    )
    .unwrap();
    fs::copy(format!("{H}/control/src/perf.rs"), format!("{R}/perf.rs")).unwrap();
    let reference = read("/home/ubuntu/src/zopro-perf-work/word-export176/caller-reference.json");
    assert_eq!(reference["passed"], true);
    let frozen = reference["identity"].clone();
    assert_eq!(
        hash(frozen["binary"].as_str().unwrap()),
        frozen["binary_sha256"]
    );
    assert_eq!(
        frozen["binary_sha256"],
        "389307b91908fe4822d9b4a788ff507944257f064bd0f11cde317ebd2cf6782b"
    );
    let frozen_child = fs::read_to_string(format!(
        "{H}/control/source/crates/zopro-core/src/msm/high_borrow_caller.rs"
    ))
    .unwrap();
    assert!(frozen_child.contains(&g1));
    assert!(frozen_child.contains(&g2));
    let mut frozen_exact = 0;
    for (p, h) in originals {
        let bytes = fs::read(format!("{H}/control/source/{p}")).unwrap();
        let orig = fs::read(format!("{Z}/{p}")).unwrap();
        assert!(bytes.starts_with(&orig));
        if bytes == orig {
            frozen_exact += 1;
        }
        assert_eq!(format!("{:x}", Sha256::digest(orig)), h.as_str().unwrap());
        pins.insert(
            format!("{H}/control/source/{p}"),
            format!("{:x}", Sha256::digest(bytes)),
        );
    }
    assert_eq!(frozen_exact, 174);
    assert_eq!(
        old_driver,
        fs::read_to_string(format!("{H}/control/src/main.rs")).unwrap()
    );
    for f in [
        "-C opt-level=3",
        "-C lto -C",
        "-C codegen-units=1",
        "-C target-cpu=native",
    ] {
        assert!(frozen["actual_rustc"].as_str().unwrap().contains(f));
    }
    pins.insert(
        frozen["binary"].as_str().unwrap().into(),
        frozen["binary_sha256"].as_str().unwrap().into(),
    );
    save("frozen-reference.json", &frozen);
    let libinfo = read(format!("{B}/frozen-candidate.json"));
    for (p, h) in libinfo["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().into());
    }
    let oldplan = read("/home/ubuntu/src/zopro-perf-work/g1-tree-callers176/build-plan.json");
    let oldcore = oldplan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "core-frozen")
        .unwrap();
    let original_args: Vec<String> = oldcore["argv"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().into())
        .collect();
    let mut args = Vec::new();
    let mut i = 0;
    while i < original_args.len() {
        if original_args[i] == "--cfg"
            && original_args
                .get(i + 1)
                .is_some_and(|s| s == "zopro_g1_tree_records_probe")
        {
            i += 2;
            continue;
        }
        args.push(
            original_args[i]
                .replace(
                    "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176/FROZEN-source",
                    &format!("{R}/source"),
                )
                .replace(
                    "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176/libs/frozen",
                    &format!("{R}/libs/ARM"),
                ),
        );
        i += 1;
    }
    let mut env = oldcore["environment"].clone();
    env["CARGO_MANIFEST_DIR"] = json!(format!("{R}/source/crates/zopro-core"));
    assert_eq!(env["CARGO_PKG_NAME"], "zopro-core");
    let mut steps = Vec::new();
    for arm in ["OFF", "ON"] {
        fs::create_dir_all(format!("{R}/libs/{arm}")).unwrap();
        let mut a = args
            .iter()
            .map(|s| s.replace("libs/ARM", &format!("libs/{arm}")))
            .collect::<Vec<_>>();
        if arm == "ON" {
            a.extend(["--cfg".into(), "zopro_inverse_half_roots".into()]);
        }
        steps.push(json!({"kind":"command","name":format!("core-{arm}"),"argv":a,"environment":env,"timeout_seconds":600}));
    }
    let mut testargs = args
        .iter()
        .map(|s| s.replace("libs/ARM", "libs/ON"))
        .collect::<Vec<_>>();
    let kind = testargs.iter().position(|s| s == "--crate-type").unwrap();
    testargs.drain(kind..kind + 2);
    let out = testargs.iter().position(|s| s == "-o").unwrap() + 1;
    testargs[out] = format!("{R}/core-tests");
    testargs.extend([
        "--cfg".into(),
        "zopro_inverse_half_roots".into(),
        "--test".into(),
        "-C".into(),
        "lto=fat".into(),
    ]);
    steps.push(json!({"kind":"command","name":"build-focused-test","argv":testargs,"environment":env,"timeout_seconds":600}));
    steps.push(json!({"kind":"command","name":"focused-test","argv":[format!("{R}/core-tests"),"--exact","simd::half_roots_native::half_roots_preserve_lazy_products_and_all_inverse_lanes","--nocapture","--test-threads=1"],"environment":env,"required_tests":["simd::half_roots_native::half_roots_preserve_lazy_products_and_all_inverse_lanes"],"timeout_seconds":120}));
    let support = format!("{B}/frozen/candidate");
    let field = libinfo["libraries"]["zopro_bn254"].as_str().unwrap();
    assert!(field.contains("1200e635"));
    let sha2 = libinfo["input_pins"]
        .as_object()
        .unwrap()
        .keys()
        .filter(|p| p.contains("/libsha2-") && p.ends_with(".rlib"))
        .collect::<Vec<_>>();
    assert_eq!(sha2.len(), 1);
    for arm in ["OFF", "ON"] {
        let mut a = vec![
            args[0].clone(),
            "--edition=2021".into(),
            format!("{R}/driver.rs"),
            "-O".into(),
            "-C".into(),
            "target-cpu=native".into(),
            "-C".into(),
            "lto=fat".into(),
            "-C".into(),
            "codegen-units=1".into(),
            "-C".into(),
            "debuginfo=1".into(),
            "-L".into(),
            format!("dependency={support}"),
            "-L".into(),
            format!("dependency={R}/libs/{arm}"),
        ];
        for name in ["libc", "rayon", "serde_json"] {
            a.extend([
                "--extern".into(),
                format!("{name}={}", libinfo["libraries"][name].as_str().unwrap()),
            ]);
        }
        a.extend([
            "--extern".into(),
            format!("sha2={}", sha2[0]),
            "--extern".into(),
            format!("zopro_core={R}/libs/{arm}/libzopro_core.rlib"),
            "-o".into(),
            format!("{R}/caller-{arm}"),
        ]);
        if arm == "ON" {
            a.extend(["--cfg".into(), "zopro_inverse_half_roots".into()]);
        }
        steps.push(json!({"kind":"command","name":format!("driver-{arm}"),"argv":a,"environment":env,"timeout_seconds":600}));
    }
    save(
        "source-audit.json",
        &json!({"source_files":hashes,"originals_exact":exact,"accepted_source_files":m["source_files_sha256"],"edit_before":before,"edit_after":after,"suffixes":suffixes,"all176_originals_recovered":true,"frozen_174_exact_two_append_only":true,"frozen_originals_pins_checked":176,"G1_G2_adapter_bodies_exact":true,"driver_cfg_only_edit":true,"field_library_reused":field,"field_and_get_implementations_unchanged":true,"old_kernel_test_suites_not_rerun":true,"new_native_test_population":1,"performance_measured":false}),
    );
    for f in [
        "setup.rs",
        "design.md",
        "tests.rs",
        "driver.rs",
        "perf.rs",
        "source-audit.json",
        "frozen-reference.json",
    ] {
        pins.insert(format!("{R}/{f}"), hash(format!("{R}/{f}")));
    }
    for f in [
        "g1-caller.rs",
        "g2-caller.rs",
        "driver.rs",
        "control/src/main.rs",
        "control/src/perf.rs",
        "control/source/crates/zopro-core/src/msm/high_borrow_caller.rs",
    ] {
        pins.insert(format!("{H}/{f}"), hash(format!("{H}/{f}")));
    }
    pins.insert(manifest_path.clone(), hash(manifest_path));
    save("build-pins.json", &json!(pins));
    save(
        "build-plan.json",
        &json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":steps}),
    );
    println!("Prepared178sources/173exact/all176recovered; unchangedfield;6newnativecommands; frozen caller reuse audited; no timing");
}
