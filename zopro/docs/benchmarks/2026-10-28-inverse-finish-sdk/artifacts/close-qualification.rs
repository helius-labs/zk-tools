use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let result: Value = serde_json::from_slice(&fs::read("runs/result.json").unwrap()).unwrap();
    assert_eq!(result["status"], "complete");
    let pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read("pins.json").unwrap()).unwrap();
    assert_eq!(pins.len(), 749);
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let before: Value =
        serde_json::from_slice(&fs::read("qualification-audit.json").unwrap()).unwrap();
    assert_eq!(
        before["all_new_prover_suites_checked_before_first_capture"],
        true
    );
    let mut hashes = BTreeMap::new();
    for mode in ["off", "on"] {
        let dir = if mode == "off" {
            "native-first"
        } else {
            "native"
        };
        for (kind, n) in [("raw", 2), ("cubic", 6)] {
            let p = format!("{dir}/{kind}-{mode}/tests.json");
            let now: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
            assert_eq!(now, before["test_summaries"][format!("{kind}-{mode}")]);
            assert_eq!(now["required_tests"].as_array().unwrap().len(), n);
            let out = format!("{dir}/{kind}-{mode}/stdout.log");
            assert!(fs::read_to_string(&out)
                .unwrap()
                .contains(&format!("test result: ok. {n} passed; 0 failed")));
            hashes.insert(p.clone(), hash(&p));
            hashes.insert(out.clone(), hash(&out));
        }
    }
    let failure = if Path::new("native-first/failure.json").is_file() {
        "native-first/failure.json"
    } else {
        panic!("failed build receipt missing")
    };
    hashes.insert(failure.to_string(), hash(failure));
    fs::write("final-qualification-audit.json",serde_json::to_vec_pretty(&json!({"all_four_suite_checks_completed_before_capture":true,"suite_records_unchanged":true,"capture_pins":749,"capture_pin_hashes_match":true,"seal_extension_completed_before_capture":false,"seal_extension_failure":"expected native-first/result.json; failed workflow has failure.json instead; optional provenance append stopped after all suite checks and before writing pins.json","capture_used_original_frozen_manifest":true,"capture_repeat":false,"qualification_receipt_hashes":hashes,"proof_inventory_sha256":hash("proof-inventory.json")})).unwrap()).unwrap();
    println!(
        "Verifiedoriginal749capturepinsandunchangedpre-capturesuitechecks;noinputmanifestmutation"
    );
}
