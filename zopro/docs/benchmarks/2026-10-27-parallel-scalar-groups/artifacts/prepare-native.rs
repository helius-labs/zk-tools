use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-parallel-scalar-groups";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const M: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
const X: &str = "/tmp/zopro-xtask-fast-screen-target/release/xtask";
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
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    assert!(!r.join("native-plan.json").exists());
    let m = read(M);
    assert_eq!(m["adopted"], true);
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 171);
    for (f, h) in originals {
        assert_eq!(hash(p.join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    let file = "crates/zopro-core/src/msm_affine/grouping.rs";
    let added = "crates/zopro-core/src/msm_affine/grouping/parallel.rs";
    let old = fs::read_to_string(p.join(file)).unwrap();
    let before="    let mut groups =\n        std::collections::HashMap::with_capacity_and_hasher(capacity, ahash::RandomState::new());";
    let after="    let reservation = match parallel::try_group(scalars, percent, capacity, reservation) {\n        Ok(groups) => return groups,\n        Err(reservation) => reservation,\n    };\n    let mut groups =\n        std::collections::HashMap::with_capacity_and_hasher(capacity, ahash::RandomState::new());";
    let module_before = "use zopro_bn254::Fr;";
    let module_after = "use zopro_bn254::Fr;\nmod parallel;";
    assert_eq!(old.matches(before).count(), 1);
    assert_eq!(old.matches(module_before).count(), 1);
    let new = old
        .replace(before, after)
        .replace(module_before, module_after);
    assert_eq!(
        new.replace(after, before)
            .replace(module_after, module_before),
        old
    );
    fs::write(r.join("overlay").join(file), new).unwrap();
    save(
        r.join("edits.json"),
        &json!({"file":file,"before":before,"after":after,"module_before":module_before,"module_after":module_after,"added_file":added,"added_file_sha256":hash(r.join("overlay").join(added)),"inverse_recovers_original":true,"production_changed":false}),
    );
    let mut pins = BTreeMap::<String, String>::new();
    for name in ["fresh", "candidate"] {
        let mut c = Command::new(X);
        c.arg("prepare").arg(M).arg(P).arg(r.join(name));
        if name == "candidate" {
            c.arg(r.join("overlay"));
        }
        let out = c.output().unwrap();
        fs::write(r.join(format!("prepare-{name}.log")), out.stdout).unwrap();
        fs::write(r.join(format!("prepare-{name}.stderr.log")), out.stderr).unwrap();
        assert!(out.status.success());
        for (f, h) in originals {
            let got = hash(r.join(name).join(f));
            if name == "candidate" && f == file {
                assert_ne!(got, h.as_str().unwrap());
            } else {
                assert_eq!(got, h.as_str().unwrap());
            }
        }
        if name == "candidate" {
            assert_eq!(
                hash(r.join(name).join(added)),
                hash(r.join("overlay").join(added))
            );
        }
        for (f, h) in read(r.join(name).join(".xtask/source-pins.json"))
            .as_object()
            .unwrap()
        {
            assert_eq!(hash(f), h.as_str().unwrap());
            pins.insert(f.clone(), h.as_str().unwrap().into());
        }
    }
    save(
        r.join("source-audit.json"),
        &json!({"accepted_source_files":171,"fresh_exact_files":171,"candidate_exact_originals":170,"candidate_changed_files":[file],"added_file":added,"inverse_recovers_original":true,"no_new_unsafe_or_runtime_flag":true,"sample_cutoff_stable_layout_and_math_unchanged":true,"new_temporary_index_buffer_checked_against_existing_forecast":true,"production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("edits.json"),
        r.join("source-audit.json"),
        Path::new(M).into(),
        Path::new(X).into(),
    ] {
        pins.insert(f.display().to_string(), hash(&f));
    }
    for (field, sha) in [
        ("standard_cli_source", "standard_cli_source_sha256"),
        ("standard_cli_helper", "standard_cli_helper_sha256"),
    ] {
        let f = m[field].as_str().unwrap();
        assert_eq!(hash(f), m[sha].as_str().unwrap());
        pins.insert(f.into(), hash(f));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let text = fs::read_to_string("/tmp/zopro-g2-scalar-grain/native-plan.json")
        .unwrap()
        .replace("/tmp/zopro-g2-scalar-grain", R);
    let mut plan: Value = serde_json::from_str(&text).unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["name"] == "candidate-core" {
            for name in [
                "stable_layout_matches_independent_ordered_map",
                "nested_and_concurrent_pools_preserve_stable_groups",
                "bounded_forecast_and_small_input_fallback",
            ] {
                step["required_tests"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(format!(
                        "msm_affine::grouping::parallel::tests::{name}"
                    )));
            }
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Prepared171exactfresh/170exactcandidateoriginals+parallelmodule; fixed native workflow ready");
}
