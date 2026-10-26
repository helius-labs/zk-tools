use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, path::Path};
const R: &str = "/tmp/zopro-pairing-clean";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-pairing-clean";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let protocol = read(r.join("screen/protocol.json"));
    for (p, h) in protocol["pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "closing pin {p}");
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 72);
    let mut proofs = BTreeSet::new();
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(proofs.insert(h.as_str().unwrap()));
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
        }
        let rel = Path::new(p).strip_prefix(r).unwrap();
        assert_eq!(hash(d.join("artifacts").join(rel)), h.as_str().unwrap());
    }
    assert_eq!(proofs.len(), 36);
    assert_eq!(
        read(r.join("screen/freshness.json"))["prior_proof_sha256"]
            .as_array()
            .unwrap()
            .len(),
        6850
    );
    assert_eq!(
        read(r.join("screen-analysis.json"))["screen_advances_to_full"],
        false
    );
    assert!(!r.join("performance").exists());
    let audit = json!({"status":"complete short screen; did not advance","production_sources_checked_unchanged":170,"source_input_library_binary_tool_pins_checked":protocol["pins"].as_object().unwrap().len(),"sdk_requests":432,"independent_gnark_exports":36,"proof_public_members":72,"next_prior_count":6850,"proof_members_archived_and_hash_checked":true,"screen_repeated":false,"full_stage_executed":false,"new_cycle_profiles":0,"adopted":false,"candidate_retained":true,"goal_achieved":false,"next_action":"Preserve the scalar-chain candidates. The clean integration did not resolve physical16 control variation or beat the accepted primary32 binary in this screen. Inspect query execution and allocation costs before choosing a distinct optimization; do not reroll the unchanged candidate or repeat failed generic waiting-policy variants."});
    save(r.join("closeout.json"), &audit);
    fs::copy(r.join("close.rs"), d.join("artifacts/close.rs")).unwrap();
    fs::copy(r.join("closeout.json"), d.join("artifacts/closeout.json")).unwrap();
    let mut index = read(d.join("artifact-index.json"));
    for rel in ["close.rs", "closeout.json"] {
        index[rel] = json!(hash(d.join("artifacts").join(rel)));
    }
    for (rel, h) in index.as_object().unwrap() {
        assert_eq!(hash(d.join("artifacts").join(rel)), h.as_str().unwrap());
    }
    save(d.join("artifact-index.json"), &index);
    println!(
        "{} hashes checked;432SDK36Gnark;nextprior6850",
        index.as_object().unwrap().len()
    );
}
