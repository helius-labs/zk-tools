use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-half-roots176";
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
        assert_eq!(format!("{:x}", Sha256::digest(b)), h.as_str().unwrap());
    }
    assert_eq!(exact, 173);
    let plan = read(format!("{R}/build-plan.json"));
    assert_eq!(plan["steps"].as_array().unwrap().len(), 6);
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
        if n != "focused-test" {
            let a: Vec<_> = s["argv"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            for f in ["-O", "target-cpu=native", "codegen-units=1"] {
                assert!(a.contains(&f));
            }
            if !n.starts_with("core-") {
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
            functions.push(json!({"symbol":n,"instruction_rows":instructions.len(),"IFMA_sites":count("vpmadd52"),"ZMM_operand_rows":count("%zmm"),"YMM_operand_rows":count("%ymm"),"RSP_operand_rows":count("%rsp"),"permutation_rows":count("vperm"),"unpack_rows":count("vpunpck"),"body_sha256":format!("{:x}",Sha256::digest(b.as_bytes())),"static_only_not_dynamic_cost":true}));
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
    let out = json!({"review_passed":true,"source_pins_checked":pins.as_object().unwrap().len(),"accepted_sources_recovered":176,"accepted_sources_exact":exact,"snapshot_files":178,"new_native_commands":receipts,"focused_tests_passed":1,"actual_test_summary":summary,"new_inverse_lane_checks":8200,"new_root_export_checks":2050,"field_and_original_kernel_test_suites_rerun":false,"independent_field_qualification":"inherited unchanged accepted field; new scalar cross-representation/product-root oracle only","frozen_reuse":"frozen-reference.json plus source/adapter/driver recovery in setup and bound pins","identities":ids,"final_consumer_codegen":codegen,"new_timing_observations":0,"new_proofs":0,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"});
    save("review.json", &out);
    println!("Review PASS six new native commands,173exact/176recovered,1new focused inverse test/8200lanes,2final native consumers decoded once; no timing");
}
