use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-codec176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(f: &str, v: &Value) {
    fs::write(format!("{R}/{f}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("review.json").exists());
    assert_eq!(
        read(format!("{R}/build-runs/result.json"))["status"],
        "complete"
    );
    let pins = read(format!("{R}/build-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let audit = read(format!("{R}/source-audit.json"));
    let mut exact = 0;
    for (p, h) in audit["accepted_source_files"].as_object().unwrap() {
        assert_eq!(hash(format!("{Z}/{p}")), h.as_str().unwrap());
        let path = format!("{R}/source/{p}");
        assert_eq!(hash(&path), audit["source_files"][p]);
        let mut b = fs::read(path).unwrap();
        if format!("{:x}", Sha256::digest(&b)) == h.as_str().unwrap() {
            exact += 1;
        }
        if let Some(s) = audit["suffixes"].get(p) {
            let suffix = s.as_str().unwrap();
            assert!(b.ends_with(suffix.as_bytes()));
            b.truncate(b.len() - suffix.len());
        }
        if p == "crates/zopro-core/src/simd.rs" {
            let text = String::from_utf8(b).unwrap();
            let after = audit["edit_after"].as_str().unwrap();
            assert_eq!(text.matches(after).count(), 1);
            b = text
                .replacen(after, audit["edit_before"].as_str().unwrap(), 1)
                .into_bytes();
        }
        if p == "scripts/ifma8.py" {
            let text = String::from_utf8(b).unwrap();
            let after = audit["generator_after"].as_str().unwrap();
            assert_eq!(text.matches(after).count(), 1);
            b = text
                .replacen(after, audit["generator_before"].as_str().unwrap(), 1)
                .into_bytes();
        }
        assert_eq!(format!("{:x}", Sha256::digest(b)), h.as_str().unwrap());
    }
    assert_eq!(exact, 171);
    let plan = read(format!("{R}/build-plan.json"));
    assert_eq!(plan["steps"].as_array().unwrap().len(), 10);
    let mut receipts = Vec::new();
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let dir = format!("{R}/build-runs/{n}");
        let exit = read(format!("{dir}/exit.json"));
        assert_eq!(exit["returncode"], 0, "{n}");
        assert_eq!(exit["success"], true);
        for key in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{dir}/{key}.log")),
                exit[format!("{key}_sha256")]
            );
        }
        let actual = read(format!("{dir}/command.json"));
        assert_eq!(
            &actual["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap().as_slice()
        );
        if !["focused-test", "codec-tests", "generator-check"].contains(&n) {
            let a: Vec<_> = s["argv"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            for f in ["-O", "target-cpu=native", "codegen-units=1"] {
                assert!(a.contains(&f));
            }
            if !n.starts_with("core-") && n != "field-library" {
                assert!(a.contains(&"lto=fat"));
            }
        }
        receipts.push(json!({"name":n,"exit_sha256":hash(format!("{dir}/exit.json")),"elapsed_ms":exit["elapsed_ms"]}));
    }
    let tests = read(format!("{R}/build-runs/focused-test/tests.json"));
    assert_eq!(tests["passed_tests"].as_array().unwrap().len(), 1);
    let stdout = fs::read_to_string(format!("{R}/build-runs/focused-test/stdout.log")).unwrap();
    assert!(stdout.contains("HALF_ROOT_ORACLE 1025fullwidth/lazy/pattern cases;8200 inverse lanes plus2050 root exports"));
    let summary = stdout
        .lines()
        .find(|s| s.starts_with("test result: ok."))
        .unwrap();
    assert!(summary.contains("1 passed; 0 failed"));
    let codec_tests = read(format!("{R}/build-runs/codec-tests/tests.json"));
    assert_eq!(codec_tests["passed_tests"].as_array().unwrap().len(), 2);
    let field_stdout =
        fs::read_to_string(format!("{R}/build-runs/codec-tests/stdout.log")).unwrap();
    assert!(field_stdout.contains("ROOT_PRODUCT_MIXED_PASS fq=4292 fr=4292"));
    assert!(field_stdout.contains("ROOT_PRODUCT_UNIFORM_PASS fq=4292 fr=4292"));
    let field_summary = field_stdout
        .lines()
        .find(|s| s.starts_with("test result: ok."))
        .unwrap();
    assert!(field_summary.contains("2 passed; 0 failed"));
    let field = audit["new_shared_field_library"].as_str().unwrap();
    let field_hash = hash(field);
    assert_ne!(
        field_hash,
        hash(audit["old_field_library_for_reference"].as_str().unwrap())
    );
    for step in plan["steps"].as_array().unwrap().iter().filter(|s| {
        s["name"].as_str().unwrap().starts_with("core-") || s["name"] == "build-focused-test"
    }) {
        assert!(step["argv"]
            .as_array()
            .unwrap()
            .contains(&json!(format!("zopro_bn254={field}"))));
    }
    let frozen = read(format!("{R}/frozen-reference.json"));
    assert_eq!(
        hash(frozen["binary"].as_str().unwrap()),
        frozen["binary_sha256"]
    );
    let mut ids = BTreeMap::from([("frozen", frozen)]);
    let mut codegen = BTreeMap::new();
    for arm in ["OFF", "ON"] {
        let binary = format!("{R}/caller-{arm}");
        let core = format!("{R}/libs/{arm}/libzopro_core.rlib");
        ids.insert(arm,json!({"binary":binary,"binary_sha256":hash(&binary),"core":core,"core_sha256":hash(&core)}));
        let args = ["-d", "-C", "--no-show-raw-insn", &binary];
        let out = Command::new("objdump").args(args).output().unwrap();
        fs::write(format!("{R}/codegen-{arm}.stdout.log"), &out.stdout).unwrap();
        fs::write(format!("{R}/codegen-{arm}.stderr.log"), &out.stderr).unwrap();
        assert!(out.status.success());
        let text = std::str::from_utf8(&out.stdout).unwrap();
        let mut bodies = BTreeMap::<String, String>::new();
        let mut selected = None;
        for line in text.lines() {
            if line.starts_with('0') && line.ends_with(">:") {
                selected = if (line.contains("FqBatch") && line.contains("inverse_nonzero"))
                    || (line.contains("stream_g1_impl") && line.contains("G1Config"))
                    || (line.contains("g2_borrowed::apply") && line.contains("G2Config"))
                {
                    Some(line.to_owned())
                } else {
                    None
                };
            }
            if let Some(n) = &selected {
                bodies
                    .entry(n.clone())
                    .or_default()
                    .push_str(&format!("{line}\n"));
            }
        }
        let mut functions = Vec::new();
        for (n, b) in &bodies {
            let instructions: Vec<_> = b
                .lines()
                .skip(1)
                .filter_map(|l| l.split_once(':'))
                .map(|(_, a)| a.trim())
                .filter(|s| !s.is_empty())
                .collect();
            let count = |needle: &str| instructions.iter().filter(|s| s.contains(needle)).count();
            functions.push(json!({"symbol":n,"instruction_rows":instructions.len(),"IFMA_sites":count("vpmadd52"),"ZMM_operand_rows":count("%zmm"),"YMM_operand_rows":count("%ymm"),"RSP_operand_rows":count("%rsp"),"permutation_rows":count("vperm"),"unpack_rows":count("vpunpck"),"vector_compare_rows":count("vpcmp"),"vector_blend_rows":count("vpblend"),"body_sha256":format!("{:x}",Sha256::digest(b.as_bytes())),"static_only_not_dynamic_cost":true}));
        }
        let inverse: Vec<_> = functions
            .iter()
            .filter(|v| v["symbol"].as_str().unwrap().contains("inverse_nonzero"))
            .collect();
        assert_eq!(
            inverse.len(),
            1,
            "actual function inventory retained in decode; inspect rather than redump"
        );
        save(&format!("selected-codegen-{arm}.json"), &json!(bodies));
        codegen.insert(arm,json!({"actual_decode_argv":args,"consumer_sha256":hash(&binary),"stdout_sha256":hash(format!("{R}/codegen-{arm}.stdout.log")),"exit_status":0,"functions":functions}));
    }
    assert_ne!(ids["OFF"]["binary_sha256"], ids["ON"]["binary_sha256"]);
    assert_ne!(ids["OFF"]["core_sha256"], ids["ON"]["core_sha256"]);
    save("identities.json", &json!(ids));
    save("codegen.json", &json!(codegen));
    let out = json!({"review_passed":true,"source_pins_checked":pins.as_object().unwrap().len(),"accepted_sources_recovered":176,"accepted_sources_exact":exact,"snapshot_files":179,"new_native_commands":receipts,"new_field_test_populations":2,"field_root_product_configurations_checked":17168,"field_test_summary":field_summary,"reapplied_focused_core_test_populations":1,"actual_core_test_summary":summary,"reapplied_inverse_lane_checks":8200,"reapplied_root_export_checks":2050,"unchanged_field_core_test_suites_rerun":false,"independent_field_qualification":"2 new mixed/uniform Arkworks codec populations; original scalar range22616 cases inherited without rerun","new_shared_field_library":{"path":field,"sha256":field_hash,"explicitly_linked_by_both_cores_and_core_test":true},"frozen_reuse":"frozen-reference.json plus source/adapter/driver recovery in setup and bound pins","identities":ids,"final_consumer_codegen":codegen,"new_timing_observations":0,"new_proofs":0,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"});
    save("review.json", &out);
    println!("Review PASS ten new commands,171exact/176recovered,2new Ark codec populations/17168configs,1reapplied inverse test/8200lanes,coherent sharedfield/2actual final consumers; no timing");
}
