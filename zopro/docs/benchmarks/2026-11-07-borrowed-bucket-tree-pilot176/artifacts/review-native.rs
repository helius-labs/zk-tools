use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176/native";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn replace(s: &mut String, from: &str, to: &str) {
    assert_eq!(s.matches(from).count(), 1, "unique inverse edit: {from}");
    *s = s.replacen(from, to, 1);
}
fn main() {
    assert!(!Path::new(&format!("{R}/native-review.json")).exists());
    let accepted=load(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let parents = accepted["source_files_sha256"].as_object().unwrap();
    assert_eq!(parents.len(), 176);
    let parent = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let child = "crates/zopro-core/src/msm_affine/g2_phi/borrowed_tree.rs";
    let tests = "crates/zopro-core/src/msm_affine/g2_phi/borrowed_tree/tests.rs";
    let mut sources = vec![];
    let mut pin_checks = 0;
    for (folder, pins) in [
        ("source", "core-pins.json"),
        ("source-fixed", "core-fixed-pins.json"),
    ] {
        let pins = load(format!("{R}/{pins}"));
        assert_eq!(pins.as_object().unwrap().len(), 178);
        for (p, h) in pins.as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin_checks += 1;
        }
        let mut exact = 0;
        let mut fields = 0;
        for (p, h) in parents {
            assert_eq!(hash(format!("{Z}/{p}")), h.as_str().unwrap());
            if p != parent {
                assert_eq!(hash(format!("{R}/{folder}/{p}")), h.as_str().unwrap());
                exact += 1;
            }
            if p.starts_with("crates/zopro-bn254/") {
                fields += 1;
            }
        }
        assert_eq!(exact, 175);
        let mut source = fs::read_to_string(format!("{R}/{folder}/{parent}")).unwrap();
        replace(&mut source, "mod borrowed_tree;\n", "");
        replace(&mut source,"fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize, tree: bool) -> Vec<G2Affine> {","fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize) -> Vec<G2Affine> {");
        replace(
            &mut source,
            "    if tree && bits == 12 { return borrowed_tree::window(bases, short, w); }\n",
            "",
        );
        replace(&mut source,"    let _tree = if bits == 12 && parts == 2 { borrowed_tree::admit(bases.len()) } else { None };\n    let tree = _tree.is_some();\n","");
        replace(
            &mut source,
            "hb_window(&bases[start..end], &short[start..end], w, bits, tree)",
            "hb_window(&bases[start..end], &short[start..end], w, bits)",
        );
        assert_eq!(
            format!("{:x}", Sha256::digest(source.as_bytes())),
            parents[parent].as_str().unwrap()
        );
        for p in [child, tests] {
            assert!(Path::new(&format!("{R}/{folder}/{p}")).is_file());
            assert!(!parents.contains_key(p));
        }
        sources.push(json!({"snapshot":folder,"files":178,"parents_exact":175,"parent_recovered":1,"children":2,"field_files_exact":fields}));
    }
    let implementation = fs::read_to_string(format!("{R}/source-fixed/{child}")).unwrap();
    assert!(!implementation.contains("unsafe"));
    assert!(!implementation.contains("#[inline"));
    assert!(!implementation.contains("stack_size"));
    assert_eq!(implementation.matches("par_chunks_mut").count(), 1);
    let leaf = &implementation
        [implementation.find("fn stage").unwrap()..implementation.find("struct Route").unwrap()];
    assert!(!leaf.contains(".par_") && !leaf.contains("rayon::") && !leaf.contains("join("));
    assert!(leaf.contains(") -> [usize; BAND]"));
    let original = fs::read_to_string(format!("{R}/source/{child}")).unwrap();
    assert!(original.contains(") -> [G2Affine; BAND]"));
    assert!(implementation.contains("point: &output[b]"));
    let mut receipts = 0;
    let old_tests=load("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-core/tests.json");
    let old_names = old_tests["passed_tests"].as_array().unwrap();
    assert_eq!(old_names.len(), 125);
    let plan = load(format!("{R}/core-fixed-plan.json"));
    assert_eq!(
        load(format!("{R}/core-fixed-runs/result.json"))["status"],
        "complete"
    );
    let mut modes = vec![];
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        let prefix = format!("{R}/core-fixed-runs/{name}");
        let exit = load(format!("{prefix}/exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let cmd = load(format!("{prefix}/command.json"));
        let got = cmd["argv"].as_array().unwrap();
        let expected = step["argv"].as_array().unwrap();
        assert_eq!(&got[got.len() - expected.len()..], expected);
        assert_eq!(
            cmd["environment"]["RUSTFLAGS"],
            step["environment"]["RUSTFLAGS"]
        );
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{prefix}/{stream}.log")),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        let t = load(format!("{prefix}/tests.json"));
        assert_eq!(t["unfiltered"], true);
        let names = t["passed_tests"].as_array().unwrap();
        assert_eq!(names.len(), 130);
        for n in old_names {
            assert!(names.contains(n));
        }
        for n in step["required_tests"].as_array().unwrap() {
            assert!(names.contains(n));
        }
        let out = fs::read_to_string(format!("{prefix}/stdout.log")).unwrap();
        assert!(out.contains("130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
        modes.push(json!({"name":name,"core_passed":130,"unfiltered":true,"new_tests":5,"argv_and_streams_match":true}));
        receipts += 1;
    }
    let failed = load(format!("{R}/core-runs/core-off/exit.json"));
    assert_eq!(failed["success"], false);
    assert_eq!(failed["returncode"], 101);
    let stderr = fs::read_to_string(format!("{R}/core-runs/core-off/stderr.log")).unwrap();
    assert!(stderr.contains("has overflowed its stack"));
    assert!(!Path::new(&format!("{R}/core-runs/core-on/exit.json")).exists());
    let passed = fs::read_to_string(format!("{R}/descriptor-reuse.stdout.log")).unwrap();
    assert_eq!(
        passed.trim(),
        "20 descriptor allocation/lifetime reuse cases pass"
    );
    let value = json!({"review_passed":true,"source_pin_checks":pin_checks,"snapshots":sources,"original_parent_recovered_both":true,"field_qualification_inherited_not_replayed":true,"core_receipts":receipts,"modes":modes,"initial_failed_suite_preserved":true,"stack_limit_bytes":512*1024,"no_stack_increase":true,"new_unsafe_operations":0,"record_layout_inherited":true,"new_native_tests":5,"new_proofs":0,"new_timings":0,"production_changed":false,"adopted":false,"performance_qualified":false,"limits":"Source/receipt/core-test review, not formal lifetime/stack/admission proof, independent field/curve oracle, observed RAM or performance evidence. Corrected runtime stage necessitates new OFF/ON suites; successful mathematical models and inherited field suites were not repeated."});
    fs::write(
        format!("{R}/native-review.json"),
        serde_json::to_vec_pretty(&value).unwrap(),
    )
    .unwrap();
    println!("native review PASS;356source pins;130OFF/130ON;175exact+1recovered+2children each");
}
