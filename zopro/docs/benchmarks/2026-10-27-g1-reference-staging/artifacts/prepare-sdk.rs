use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/tmp/zopro-g1-reference-staging";
const M: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
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
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    let h = hash(p);
    if let Some(old) = pins.insert(p.display().to_string(), h.clone()) {
        assert_eq!(old, h)
    }
}
fn prior() -> BTreeSet<String> {
    let old = read("/tmp/zopro-composed-cpu/perf-plan.json");
    let mut seen: BTreeSet<String> =
        serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 8604);
    let mut duplicates = 0;
    for inv in [
        "/tmp/zopro-composed-cpu/proof-inventory.json",
        "/tmp/zopro-physical16-scheduling/proof-inventory.json",
        "/tmp/zopro-composed-qualification/proof-inventory.json",
    ] {
        for (p, h) in read(inv).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            if p.ends_with(".proof") {
                assert_eq!(fs::metadata(p).unwrap().len(), 164);
                if !seen.insert(h.as_str().unwrap().into()) {
                    duplicates += 1
                }
            }
        }
    }
    assert_eq!(duplicates, 9);
    assert_eq!(seen.len(), 9154);
    seen
}
fn prepare() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native-v2/result.json"))["status"], "complete");
    for p in [
        r.join("native/fresh-core/tests.json"),
        r.join("native-v2/candidate-core/tests.json"),
    ] {
        assert_eq!(read(p)["passed_tests"].as_array().unwrap().len(), 116)
    }
    assert_eq!(
        read(r.join("codegen.json"))["all_stream_instruction_shapes_equal"],
        false
    );
    let m = read(M);
    let mut p = read("/tmp/zopro-composed-cpu/perf-plan.json");
    p["blocks"] = json!(9);
    p["profile_blocks"] = json!(1);
    p["profile_cases"] = json!(["1x2-w32", "4x2-w32"]);
    p["environment"] = m["environment"].clone();
    p["checks"] = json!([]);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("source-pins-v2.json"))).unwrap();
    for (f, h) in &pins {
        assert_eq!(&hash(f), h)
    }
    let mut variants = Vec::new();
    for (name, receipt) in [
        (
            "accepted",
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
        ),
        (
            "fresh",
            "/tmp/zopro-g1-reference-staging/native-v2/fresh/artifacts.json",
        ),
        (
            "candidate",
            "/tmp/zopro-g1-reference-staging/native-v2/candidate/artifacts.json",
        ),
    ] {
        let a = read(receipt);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (f, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            add(f, &mut pins)
        }
        add(receipt, &mut pins);
        add(a["binary"].as_str().unwrap(), &mut pins);
        variants.push(json!({"name":name,"binary":a["binary"]}));
    }
    for c in p["cases"].as_array().unwrap() {
        add(c["image"].as_str().unwrap(), &mut pins);
        add(c["request"].as_str().unwrap(), &mut pins)
    }
    for (f, h) in p["pins"].as_object().unwrap() {
        if f.starts_with("/tmp/zopro-keys/") {
            assert_eq!(hash(f), h.as_str().unwrap());
            add(f, &mut pins)
        }
    }
    for f in [
        "prepare-sdk.rs",
        "prepare-sdk",
        "analyze.rs",
        "analyze",
        "stats.rs",
        "sdk-design.md",
        "codegen.json",
        "inspect.rs",
        "inspect",
        "inspect-tests.log",
        "native-v2/result.json",
        "native-v2/candidate-core/tests.json",
        "native-v2/candidate-prover/tests.json",
    ] {
        add(r.join(f), &mut pins)
    }
    for f in [
        "/usr/bin/perf",
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
        "/tmp/zopro-gnark-verify/verify",
        "/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock",
    ] {
        add(f, &mut pins)
    }
    let seen = prior();
    p["variants"] = json!(variants);
    p["pins"] = json!(pins);
    p["prior_proof_sha256"] = json!(seen);
    save(r.join("perf-before-preflight.json"), &p);
    let mut steps = Vec::new();
    let mut jobs = Vec::new();
    let binary = read(r.join("native-v2/candidate/artifacts.json"))["binary"].clone();
    for case in p["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let name = format!("{}-w4", case["shape"].as_str().unwrap());
        let dir = r.join("preflight-proofs").join(&name).join(
            Path::new(case["image"].as_str().unwrap())
                .file_stem()
                .unwrap(),
        );
        let mut env = p["environment"].clone();
        env["RAYON_NUM_THREADS"] = json!("4");
        steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","0-3",binary,case["image"],case["request"],"1",dir],"environment":env,"timeout_seconds":600}));
        steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",dir.parent().unwrap(),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
        jobs.push(json!({"name":name,"directory":dir,"public_sha256":case["public_sha256"]}));
    }
    assert_eq!(jobs.len(), 9);
    save(r.join("preflight-jobs.json"), &json!(jobs));
    save(r.join("preflight-pins.json"), &json!(pins));
    save(
        r.join("preflight-plan.json"),
        &json!({"version":1,"source_pins":r.join("preflight-pins.json"),"steps":steps}),
    );
    println!("Preparedall9 preflight36SDK9exports; fixed9blockmatrix+6profiles planned;prior9154");
}
fn qualify() {
    let r = Path::new(R);
    assert_eq!(read(r.join("preflight/result.json"))["status"], "complete");
    let mut p = read(r.join("perf-before-preflight.json"));
    let mut seen = prior();
    let mut pins: BTreeMap<String, String> = serde_json::from_value(p["pins"].clone()).unwrap();
    let mut inv = BTreeMap::new();
    for j in read(r.join("preflight-jobs.json")).as_array().unwrap() {
        let dir = Path::new(j["directory"].as_str().unwrap());
        let proof = dir.join("0.proof");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        assert_eq!(
            hash(proof.with_extension("public")),
            j["public_sha256"].as_str().unwrap()
        );
        assert!(seen.insert(hash(&proof)));
        for f in [proof.clone(), proof.with_extension("public")] {
            inv.insert(f.display().to_string(), hash(&f));
            add(&f, &mut pins)
        }
        let summary: Value = serde_json::from_str(
            fs::read_to_string(
                r.join("preflight")
                    .join(j["name"].as_str().unwrap())
                    .join("stdout.log"),
            )
            .unwrap()
            .lines()
            .find_map(|s| s.strip_prefix("SUMMARY "))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(summary["rounds"], 1);
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["workers"], 4);
        let verify = fs::read_to_string(
            r.join("preflight")
                .join(format!("verify-{}", j["name"].as_str().unwrap()))
                .join("stdout.log"),
        )
        .unwrap();
        assert!(verify.contains("verified 1 distinct proofs across 1 pinned keys"));
    }
    assert_eq!(seen.len(), 9163);
    assert_eq!(inv.len(), 18);
    save(r.join("preflight-proof-inventory.json"), &json!(inv));
    save(
        r.join("preflight-analysis.json"),
        &json!({"passed":true,"SDK_requests":36,"independent_exports":9,"next_prior_count":9163}),
    );
    for n in [
        "preflight-plan.json",
        "preflight/result.json",
        "preflight-proof-inventory.json",
        "preflight-analysis.json",
    ] {
        add(r.join(n), &mut pins)
    }
    for (f, h) in &pins {
        assert_eq!(&hash(f), h)
    }
    p["pins"] = json!(pins);
    p["prior_proof_sha256"] = json!(seen);
    save(r.join("perf-plan.json"), &p);
    println!("Ninecandidate4workerpreflightsverified;prior9163;readyfixed9blocks+6profiles");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("qualify") => qualify(),
        _ => panic!("prepare|qualify"),
    }
}
