use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-grouping-prefix-cutoff";
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
    let mut b = [0u8; 65536];
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
    let tests = "crates/zopro-core/src/msm_affine/grouping/tests.rs";
    let old = fs::read_to_string(p.join(file)).unwrap();
    let before = "    for (scalar, class) in scalars.iter().zip(&mut classes) {";
    let after="    // Distinct nonzero counts are monotone; stop once acceptance is impossible.\n    let max_groups = scalars.len() / 100 * percent\n        + scalars.len() % 100 * percent / 100;\n    for (scalar, class) in scalars.iter().zip(&mut classes) {";
    let old_insert="        let index = *groups.entry(*scalar).or_insert_with(|| {\n            values.push(*scalar);\n            counts.push(0);\n            (values.len() - 1) as u32\n        });";
    let new_insert="        let index = match groups.entry(*scalar) {\n            std::collections::hash_map::Entry::Occupied(entry) => *entry.get(),\n            std::collections::hash_map::Entry::Vacant(entry) => {\n                if values.len() == max_groups {\n                    return None;\n                }\n                let index = values.len() as u32;\n                values.push(*scalar);\n                counts.push(0);\n                entry.insert(index);\n                index\n            }\n        };";
    assert_eq!(old.matches(before).count(), 1);
    assert_eq!(old.matches(old_insert).count(), 1);
    let new = old.replace(before, after).replace(old_insert, new_insert);
    assert_eq!(
        new.replace(after, before).replace(new_insert, old_insert),
        old
    );
    let test_old = fs::read_to_string(p.join(tests)).unwrap();
    let addition = fs::read_to_string(r.join("oracle-test.rs")).unwrap();
    for (f, text) in [(file, new), (tests, format!("{test_old}{addition}"))] {
        let out = r.join("overlay").join(f);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::write(out, text).unwrap();
    }
    save(
        r.join("edits.json"),
        &json!({"file":file,"before":before,"after":after,"old_insert":old_insert,"new_insert":new_insert,"tests":tests,"tests_addition_sha256":hash(r.join("oracle-test.rs")),"production_changed":false}),
    );
    let mut pins = BTreeMap::<String, String>::new();
    for name in ["fresh", "candidate"] {
        let mut c = Command::new(X);
        c.arg("prepare").arg(M).arg(P).arg(r.join(name));
        if name == "candidate" {
            c.arg(r.join("overlay"));
        }
        let out = c.output().unwrap();
        fs::write(r.join(format!("prepare-{name}.log")), &out.stdout).unwrap();
        fs::write(r.join(format!("prepare-{name}.stderr.log")), &out.stderr).unwrap();
        assert!(out.status.success());
        for (f, h) in originals {
            let got = hash(r.join(name).join(f));
            if name == "candidate" && (f == file || f == tests) {
                assert_ne!(got, h.as_str().unwrap());
            } else {
                assert_eq!(got, h.as_str().unwrap());
            }
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
        &json!({"accepted_source_files":originals.len(),"fresh_exact_files":171,"candidate_exact_files":169,"candidate_changed_files":[file,tests],"inverse_recovers_original":true,"math_field_scheduler_buffers_capacity_forecasts_unchanged":true,"no_new_unsafe_or_runtime_flag":true,"production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("prepare-native"),
        r.join("oracle-test.rs"),
        r.join("edits.json"),
        r.join("source-audit.json"),
        Path::new(M).to_path_buf(),
        Path::new(X).to_path_buf(),
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
    let oldplan = read("/tmp/zopro-g1-reference-staging/native-plan.json");
    let mut steps = oldplan["steps"].as_array().unwrap().clone();
    for s in &mut steps {
        if s["kind"] == "link-sdk" {
            s["driver"] = m["standard_cli_source"].clone();
            s["additional_sources"] = json!([m["standard_cli_helper"]]);
        }
        if let Some(v) = s["argv"].as_array_mut() {
            for a in v {
                if let Some(t) = a.as_str() {
                    *a = json!(t.replace("/tmp/zopro-g1-reference-staging", R));
                }
            }
        }
        if let Some(v) = s["environment"]["CARGO_TARGET_DIR"].as_str() {
            s["environment"]["CARGO_TARGET_DIR"] =
                json!(v.replace("/tmp/zopro-g1-reference-staging", R));
        }
        if s["kind"] == "link-sdk" {
            let v = s["artifacts"]
                .as_str()
                .unwrap()
                .replace("/tmp/zopro-g1-reference-staging", R);
            s["artifacts"] = json!(v);
        }
        if s["name"] == "candidate-core" {
            s["required_tests"].as_array_mut().unwrap().push(json!(
                "msm_affine::grouping::tests::prefix_cutoff_matches_independent_ordered_map"
            ));
        }
    }
    save(
        r.join("native-plan.json"),
        &json!({"version":1,"source_pins":r.join("source-pins.json"),"environment":oldplan["environment"],"steps":steps}),
    );
    println!("Prepared exact171 fresh /169 unchanged candidate + grouping loop + independent oracle; native workflow ready, no SDK requests yet");
}
