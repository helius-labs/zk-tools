use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-prefix-replay176";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut d = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        d.update(&b[..n]);
    }
    format!("{:x}", d.finalize())
}
fn main() {
    assert!(!Path::new(&format!("{R}/preflight-review.json")).exists());
    assert_eq!(load(format!("{R}/preflight-runs/completed.json")).as_array().unwrap().len(), 55);
    let plan = load(format!("{R}/preflight-plan.json"));
    let mut receipts = 0;
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        if !name.starts_with("preflight-") && !name.starts_with("verify-preflight-") {
            continue;
        }
        let prefix = format!("{R}/preflight-runs/{name}");
        let exit = load(format!("{prefix}/exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let cmd = load(format!("{prefix}/command.json"));
        let a = cmd["argv"].as_array().unwrap();
        let expected = step["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - expected.len()..], expected);
        for s in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{prefix}/{s}.log")),
                exit[format!("{s}_sha256")].as_str().unwrap()
            );
        }
        receipts += 1;
    }
    assert_eq!(receipts, 54);
    let ledger_path =
        "/home/ubuntu/src/zopro-perf-work/cios-preadd-sdk176/primary-pilot/proof-sha256-after-diagnostic.json";
    let mut ledger: BTreeSet<String> = serde_json::from_value(load(ledger_path)).unwrap();
    assert_eq!(ledger.len(), 31465);
    let before_hash = hash(ledger_path);
    let jobs = load(format!("{R}/preflight-jobs.json"));
    assert_eq!(jobs.as_array().unwrap().len(), 27);
    let mut results = Vec::new();
    let mut shapes = BTreeMap::<String, usize>::new();
    for job in jobs.as_array().unwrap() {
        let name = job["name"].as_str().unwrap();
        let proof = job["proof"].as_str().unwrap();
        let public = job["public"].as_str().unwrap();
        assert_eq!(fs::metadata(proof).unwrap().len(), 164);
        let ph = hash(proof);
        let public_hash = hash(public);
        assert_eq!(public_hash, job["public_sha256"].as_str().unwrap());
        let output = fs::read_to_string(format!("{R}/preflight-runs/{name}/stdout.log")).unwrap();
        let summaries = output
            .lines()
            .filter_map(|l| l.strip_prefix("SUMMARY "))
            .collect::<Vec<_>>();
        assert_eq!(summaries.len(), 1);
        let summary: Value = serde_json::from_str(summaries[0]).unwrap();
        assert_eq!(summary["rounds"], 1);
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["workers"], 32);
        assert_eq!(
            output.lines().filter(|l| l.starts_with("SAMPLE ")).count(),
            1
        );
        let verify =
            fs::read_to_string(format!("{R}/preflight-runs/verify-{name}/stdout.log")).unwrap();
        let lines = verify
            .lines()
            .filter_map(|l| l.strip_prefix("VERIFIED "))
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        let fields = lines[0]
            .split_whitespace()
            .map(|s| s.split_once('=').unwrap())
            .collect::<BTreeMap<_, _>>();
        assert_eq!(fields["proof_sha256"], ph);
        assert_eq!(fields["public_sha256"], public_hash);
        assert_eq!(
            fields["image_sha256"],
            job["image_sha256"].as_str().unwrap()
        );
        assert_eq!(fields["vk_sha256"].len(), 64);
        assert!(ledger.insert(ph.clone()), "nonfresh export {name}");
        *shapes
            .entry(job["shape"].as_str().unwrap().to_string())
            .or_default() += 1;
        results.push(json!({"name":name,"shape":job["shape"],"arm":job["arm"],"proof_sha256":ph,"public_sha256":public_hash,"image_sha256":fields["image_sha256"],"vk_sha256":fields["vk_sha256"],"verified":true,"fresh":true,"summary":summary}));
    }
    assert_eq!(shapes.len(), 9);
    assert!(shapes.values().all(|n| *n == 3));
    assert_eq!(ledger.len(), 31492);
    assert_eq!(hash(ledger_path), before_hash);
    fs::write(
        format!("{R}/proof-sha256-after-preflight.json"),
        serde_json::to_vec_pretty(&ledger).unwrap(),
    )
    .unwrap();
    let result = json!({"review_passed":true,"independent_Gnark_verified_fresh_exports":27,"completed_SDK_requests_including_primes":108,"new_canonical_proof_count":ledger.len(),"proof_ledger_before":ledger_path,"proof_ledger_before_sha256":before_hash,"proof_ledger_after":format!("{R}/proof-sha256-after-preflight.json"),"shape_arms":shapes,"results":results,"timing_gate_not_started":true,"preflight_times_not_performance_evidence":true,"production_changed":false,"adopted":false});
    fs::write(
        format!("{R}/preflight-review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("PASS27fresh independentGnark exports/9shapes×3arms,108SDKrequests;ledger31492;timing gate not started");
}
