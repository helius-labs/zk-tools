use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const R: &str = "/tmp/zopro-composed-cpu";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    let h = hash(p);
    if let Some(old) = pins.insert(p.display().to_string(), h.clone()) {
        assert_eq!(old, h);
    }
}
fn prior() -> BTreeSet<String> {
    let mut seen: BTreeSet<String> = serde_json::from_value(
        read("/tmp/zopro-g2-tree-sdk/context/manifest.json")["prior_proof_sha256"].clone(),
    )
    .unwrap();
    assert_eq!(seen.len(), 5958);
    for inventory in [
        "/tmp/zopro-g2-tree-sdk/proof-inventory.json",
        "/tmp/zopro-g1-quarter-current/proof-inventory.json",
        "/tmp/zopro-pairing-x-chain-sdk/proof-inventory.json",
        "/tmp/zopro-control-rescheduling/all-proof-inventory.json",
        "/tmp/zopro-pairing-clean/proof-inventory.json",
        "/tmp/zopro-borrowed-pending/proof-inventory.json",
        "/tmp/zopro-borrowed-g1/proof-inventory.json",
        "/tmp/zopro-poseidon-frontier/proof-inventory.json",
        "/tmp/zopro-poseidon-fused-dot/proof-inventory.json",
        "/tmp/zopro-poseidon-direct-dot/proof-inventory.json",
        "/tmp/zopro-sdk-paired-control/proof-inventory.json",
        "/tmp/zopro-arithmetic-leaves-corrected/combined-proof-inventory.json",
        "/tmp/zopro-fused-top/proof-inventory.json",
        "/tmp/zopro-g2-tree-records/proof-inventory.json",
        "/tmp/zopro-adaptive-tree/proof-inventory.json",
        "/tmp/zopro-tree-kernel-split/proof-inventory.json",
        "/tmp/zopro-g1-reverse-sdk/proof-inventory.json",
    ] {
        for (p, h) in read(inventory).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            if p.ends_with(".proof") {
                assert_eq!(fs::metadata(p).unwrap().len(), 164);
                assert!(seen.insert(h.as_str().unwrap().to_string()));
            }
        }
    }
    assert_eq!(seen.len(), 8595);
    seen
}
fn prepare() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        116
    );
    let mut p = read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json");
    assert_eq!(
        read(r.join("source-audit.json"))
            ["all170accepted_bytes_recovered_by_reversing_composition"],
        true
    );
    p["profile_cases"] = json!(["1x2-w32", "4x2-w32"]);

    let mut pins: BTreeMap<String, String> = serde_json::from_value(p["pins"].clone()).unwrap();
    for (f, h) in read(r.join("source-pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
        add(f, &mut pins);
    }
    let mut variants = Vec::new();
    for (name, path) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
        ),
        (
            "fresh",
            "/tmp/zopro-poseidon-fused-dot/native/fresh/artifacts.json",
        ),
        (
            "composed",
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
        ),
    ] {
        let a = read(path);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (f, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            add(f, &mut pins);
        }
        add(path, &mut pins);
        add(a["binary"].as_str().unwrap(), &mut pins);
        variants.push(json!({"name":name,"binary":a["binary"]}));
    }
    for name in [
        "prepare.rs",
        "prepare",
        "analyze.rs",
        "analyze",
        "stats.rs",
        "adapt.rs",
        "close.rs",
        "close",
        "native-plan.json",
        "native/result.json",
        "source-audit.json",
        "design.md",
    ] {
        add(r.join(name), &mut pins);
    }
    add("/usr/bin/perf", &mut pins);
    add(
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
        &mut pins,
    );
    let seen = prior();
    p["variants"] = json!(variants);
    p["blocks"] = json!(15);
    p["pins"] = json!(pins);
    p["prior_proof_sha256"] = json!(seen);
    save(r.join("perf-before-preflight.json"), &p);
    let mut steps = Vec::new();
    let mut jobs = Vec::new();
    let binary = r.join("native/composed-sdk");
    for case in p["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let name = format!("{}-w4-composed", case["shape"].as_str().unwrap());
        let stem = Path::new(case["image"].as_str().unwrap())
            .file_stem()
            .unwrap();
        let dir = r.join("preflight-proofs").join(&name).join(stem);
        let mut env = p["environment"].clone();
        env["RAYON_NUM_THREADS"] = json!("4");
        steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","0-3",binary,case["image"],case["request"],"1",dir],"environment":env,"timeout_seconds":600}));
        steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",dir.parent().unwrap(),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
        jobs.push(json!({"name":name,"directory":dir,"public_sha256":case["public_sha256"]}));
    }
    assert_eq!(jobs.len(), 9);
    for (f, h) in &pins {
        assert_eq!(&hash(f), h);
    }
    save(r.join("preflight-jobs.json"), &json!(jobs));
    save(r.join("preflight-pins.json"), &json!(pins));
    save(
        r.join("preflight-plan.json"),
        &json!({"version":1,"source_pins":r.join("preflight-pins.json"),"steps":steps}),
    );
    println!("Preparedall9 preflight36SDK9exports; native116checked; prior8595");
}
fn qualify() {
    let r = Path::new(R);
    assert_eq!(read(r.join("preflight/result.json"))["status"], "complete");
    let mut p = read(r.join("perf-before-preflight.json"));
    let mut seen = prior();
    let mut pins: BTreeMap<String, String> = serde_json::from_value(p["pins"].clone()).unwrap();
    let mut inventory = BTreeMap::new();
    for job in read(r.join("preflight-jobs.json")).as_array().unwrap() {
        let dir = Path::new(job["directory"].as_str().unwrap());
        let proof = dir.join("0.proof");
        let public = dir.join("0.public");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        assert_eq!(hash(&public), job["public_sha256"].as_str().unwrap());
        assert!(seen.insert(hash(&proof)));
        for f in [proof, public] {
            inventory.insert(f.display().to_string(), hash(&f));
            add(&f, &mut pins);
        }
        let name = job["name"].as_str().unwrap();
        let log = fs::read_to_string(r.join("preflight").join(name).join("stdout.log")).unwrap();
        let summary: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(summary["rounds"], 1);
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["workers"], 4);
        let verify = fs::read_to_string(
            r.join("preflight")
                .join(format!("verify-{name}"))
                .join("stdout.log"),
        )
        .unwrap();
        assert!(verify.contains("verified 1 distinct proofs across 1 pinned keys"));
    }
    assert_eq!(inventory.len(), 18);
    assert_eq!(seen.len(), 8604);
    save(r.join("preflight-proof-inventory.json"), &json!(inventory));
    save(
        r.join("preflight-analysis.json"),
        &json!({"passed":true,"SDK_requests":36,"independent_exports":9,"next_prior_count":8604}),
    );
    for name in [
        "preflight-plan.json",
        "preflight/result.json",
        "preflight-analysis.json",
        "preflight-proof-inventory.json",
    ] {
        add(r.join(name), &mut pins);
    }
    for (f, h) in &pins {
        assert_eq!(&hash(f), h);
    }
    p["pins"] = json!(pins);
    p["prior_proof_sha256"] = json!(seen);
    save(r.join("perf-plan.json"), &p);
    println!("All9preflightpassed;36SDK9Gnark;readyfixed15block3variantfullmatrix");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("qualify") => qualify(),
        _ => panic!("mode"),
    }
}
