use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/gcd-vector-rows";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    assert!(!Path::new("build-plan.json").exists());
    let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), h.as_str().unwrap().to_owned());
    }
    let original = fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/inverse.rs")).unwrap();
    let old="        a = integer_update(&old_a, &b, &mut f0, &mut g0);\n        b = integer_update(&old_a, &b, &mut f1, &mut g1);";
    let new="        let mut rows=[f0,g0,f1,g1];\n        let outputs=crate::paired::update(&old_a,&b,&mut rows);\n        a=outputs.0;b=outputs.1;\n        (f0,g0,f1,g1)=(rows[0],rows[1],rows[2],rows[3]);";
    assert_eq!(original.matches(old).count(), 1);
    let selected = original.replace(old, new);
    assert_eq!(selected.replace(new, old), original);
    fs::write("selected.rs", selected).unwrap();
    let reference = r#"
#[cfg(test)]
pub(crate) fn reference_pair(a:[u64;4],b:[u64;4],mut rows:[i64;4])->(([u64;4],[u64;4]),[i64;4]){
    let(mut f0,mut g0,mut f1,mut g1)=(rows[0],rows[1],rows[2],rows[3]);let aa=integer_update(&a,&b,&mut f0,&mut g0);let bb=integer_update(&a,&b,&mut f1,&mut g1);rows=[f0,g0,f1,g1];((aa,bb),rows)
}
#[cfg(test)]
pub(crate) fn reference_matrix(a:[u64;4],b:[u64;4])->[i64;4]{let bits=bit_len(&a).max(bit_len(&b));let(_,_,f0,g0,f1,g1)=zero_run_matrix(approximate(&a,bits),approximate(&b,bits));[f0,g0,f1,g1]}
"#;
    fs::write("ordinary.rs", format!("{original}{reference}")).unwrap();
    fs::copy(
        format!("{Z}/crates/zopro-bn254/src/portable.rs"),
        "portable.rs",
    )
    .unwrap();
    fs::write("source-audit.json",serde_json::to_vec_pretty(&json!({"production172_unchanged":true,"ordinary_original_exact_before_test_suffix":true,"reference_test_suffix":reference,"old":old,"new":new,"selected_inverse_recovered_exact":true,"portable_original_exact":true,"only_algorithm_change":"two full31bitintegerrows via4VPMULUDQ/sharedinputconversion,scalar32digitcarrying"})).unwrap()).unwrap();
    let receipt: Value = serde_json::from_slice(
        &fs::read(
            "/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json",
        )
        .unwrap(),
    )
    .unwrap();
    let field = receipt["libraries"]["zopro_bn254"].as_str().unwrap();
    let field_hash = hash(field);
    assert_eq!(receipt["input_pins"][field], field_hash);
    pins.insert(field.to_owned(), field_hash);
    fs::write("field-library.json",serde_json::to_vec_pretty(&json!({"path":field,"sha256":hash(field),"receipt":"/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json","accepted_SDK_sha256":"14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"})).unwrap()).unwrap();
    let flags = m["rustflags"].as_str().unwrap();
    fs::write("native-flags.txt", flags).unwrap();
    let deps = "/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";
    let mut base = vec!["rustc".to_owned()];
    base.extend(flags.split_whitespace().map(str::to_owned));
    base.extend([
        "--edition=2021".into(),
        "-O".into(),
        format!("{R}/probe.rs"),
        "-L".into(),
        format!("dependency={deps}"),
        "-L".into(),
        format!(
            "dependency={}",
            Path::new(field).parent().unwrap().display()
        ),
    ]);
    for (n, file) in [
        ("zopro_bn254", field.to_owned()),
        (
            "ark_ref",
            format!("{deps}/libark_bn254-d9ceb28748a3bfd8.rlib"),
        ),
        ("ark_ff", format!("{deps}/libark_ff-9216c709978bd3d2.rlib")),
        (
            "serde_json",
            format!("{deps}/libserde_json-9c41682dbe4e057f.rlib"),
        ),
        ("sha2", format!("{deps}/libsha2-b36128d04ec91f93.rlib")),
        (
            "libc",
            receipt["libraries"]["libc"].as_str().unwrap().to_owned(),
        ),
    ] {
        pins.insert(file.clone(), hash(&file));
        base.extend(["--extern".into(), format!("{n}={file}")]);
    }
    let mut steps = vec![];
    for mode in ["checked", "optimized", "probe"] {
        let mut argv = base.clone();
        argv.extend(["-o".into(), format!("{R}/{mode}")]);
        if mode != "probe" {
            argv.push("--test".into());
        }
        if mode == "checked" {
            argv.extend([
                "-C".into(),
                "overflow-checks=on".into(),
                "-C".into(),
                "debug-assertions=on".into(),
            ]);
        }
        fs::write(
            format!("link-{mode}.json"),
            serde_json::to_vec_pretty(&json!(argv)).unwrap(),
        )
        .unwrap();
        steps.push(json!({"kind":"command","name":format!("link-{mode}"),"argv":argv,"timeout_seconds":300}));
        if mode != "probe" {
            steps.push(json!({"kind":"command","name":format!("test-{mode}"),"argv":[format!("{R}/{mode}"),"--test-threads=1"],"required_tests":["paired::tests::paired_rows_match_original_and_independent_unsigned_magnitudes","paired::tests::signed_radix32_carry_bounds_cover_extremes","ordinary::tests::fq","ordinary::tests::fr","selected::tests::fq","selected::tests::fr"],"unfiltered":true,"timeout_seconds":300}));
        }
    }
    for f in [
        "design.md",
        "paired.rs",
        "probe.rs",
        "perf.rs",
        "setup.rs",
        "setup",
        "ordinary.rs",
        "selected.rs",
        "portable.rs",
        "source-audit.json",
        "field-library.json",
        "native-flags.txt",
    ] {
        pins.insert(format!("{R}/{f}"), hash(f));
    }
    fs::write(
        "build-pins.json",
        serde_json::to_vec_pretty(&json!(pins)).unwrap(),
    )
    .unwrap();
    fs::write(
        "build-plan.json",
        serde_json::to_vec_pretty(
            &json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":steps}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("Prepared5standalonechecked/optimizedtest+linksteps;production172unchanged;frozenacceptedfieldlib");
}
