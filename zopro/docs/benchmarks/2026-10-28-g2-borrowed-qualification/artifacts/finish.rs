use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
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
fn files(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            files(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn main() {
    assert_eq!(read(r("functional/result.json"))["status"], "complete");
    assert_eq!(read(r("resource-result.json"))["passed"], true);
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}")
    }
    let mut counts = BTreeMap::new();
    for (n, expected) in [
        ("portable-field", 11),
        ("portable-core", 99),
        ("portable-prover", 8),
    ] {
        let t = fs::read_to_string(r(format!("build/{n}/stdout.log"))).unwrap();
        let mut count = 0;
        for l in t
            .lines()
            .filter_map(|l| l.strip_prefix("test result: ok. "))
        {
            assert!(l.contains("0 failed") && l.contains("0 filtered out"));
            count += l
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
        assert_eq!(count, expected);
        counts.insert(n, count);
    }
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(seen.len(), 14462);
    let mut inventory = BTreeMap::new();
    let jobs = read(r("jobs.json"));
    let mut exports = 0;
    let mut requests = 0;
    let mut cases = vec![];
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let kind = j["kind"].as_str().unwrap();
        let dir = Path::new(j["directory"].as_str().unwrap());
        let mut paths = vec![];
        files(dir, &mut paths);
        let mut proofs: Vec<_> = paths
            .into_iter()
            .filter(|p| p.extension().is_some_and(|x| x == "proof"))
            .collect();
        proofs.sort();
        assert_eq!(proofs.len(), j["exports"].as_u64().unwrap() as usize);
        if kind == "functional" {
            let t = fs::read_to_string(r(format!("functional/{name}/stdout.log"))).unwrap();
            let line = t.lines().find_map(|l| l.strip_prefix("SUMMARY ")).unwrap();
            let s: Value = serde_json::from_str(line).unwrap();
            assert_eq!(s["rounds"], 1);
            assert_eq!(s["priming"], 3);
            assert_eq!(s["workers"], j["configuration"]["workers"]);
        }
        if kind == "concurrent" {
            let t = fs::read_to_string(r(format!("functional/{name}/stdout.log"))).unwrap();
            let c: Value =
                serde_json::from_str(t.lines().find_map(|l| l.strip_prefix("COMPLETE ")).unwrap())
                    .unwrap();
            assert_eq!(c["workers"], 32);
            assert_eq!(c["callers"], 4);
            assert_eq!(c["rounds"], 2);
            assert_eq!(c["proofs"], 8);
            assert_eq!(c["idle_tids"].as_array().unwrap().len(), 33);
            let loaded: Value =
                serde_json::from_str(t.lines().find_map(|l| l.strip_prefix("THREADS ")).unwrap())
                    .unwrap();
            assert_eq!(loaded["tids"].as_array().unwrap().len(), 37);
        }
        if kind == "resource" {
            let a = read(r(format!("runs/{name}/result.json")));
            assert_eq!(a["passed"], true);
            assert_eq!(a["normal_exit"], true);
            assert_eq!(a["memory_events"]["oom"], 0);
            assert_eq!(a["memory_events"]["oom_kill"], 0);
            assert_eq!(a["memory_events"]["max"], 0);
            assert_eq!(a["metrics"]["memory.max"], "201326592");
            assert_eq!(a["metrics"]["memory.swap.max"], "0");
            if j["configuration"] == "warm" {
                assert_eq!(a["lowered"], true);
                assert_eq!(a["metrics"]["memory.high"], "134217728");
                let child = read(r(format!("runs/{name}/child-receipt.json")));
                assert_eq!(child["ballast_bytes"], 33554432);
                assert_eq!(child["success"], true);
            }
            cases.push(a);
        }
        for p in proofs {
            let public = p.with_extension("public");
            let ph = hash(&p);
            let pu = hash(&public);
            assert_eq!(fs::metadata(&p).unwrap().len(), 164);
            assert!(seen.insert(ph.clone()));
            assert_eq!(pu, j["public_sha256"].as_str().unwrap());
            let log = match kind {
                "resource" => r(format!("runs/{name}/verifier.log")),
                "concurrent" => r(format!(
                    "functional/verify-{name}-{}/stdout.log",
                    p.file_stem().unwrap().to_str().unwrap()
                )),
                _ => r(format!("functional/verify-{name}/stdout.log")),
            };
            let t = fs::read_to_string(log).unwrap();
            assert!(t.contains(&format!("VERIFIED proof_sha256={ph}")));
            assert!(t.contains(&format!("public_sha256={pu}")));
            let ih = j
                .get("verification_image_sha256")
                .unwrap_or(&j["image_sha256"])
                .as_str()
                .unwrap();
            assert!(t.contains(&format!("image_sha256={ih}")));
            inventory.insert(p.display().to_string(), ph);
            inventory.insert(public.display().to_string(), pu);
            exports += 1;
        }
        requests += j["SDK_requests"].as_u64().unwrap();
    }
    assert_eq!(requests, 248);
    assert_eq!(exports, 89);
    assert_eq!(inventory.len(), 178);
    assert_eq!(seen.len(), 14551);
    assert_eq!(cases.len(), 8);
    save(r("proof-inventory.json"), &json!(inventory));
    let a = json!({"status":"additional qualification passed; adoption assessment required","qualified_additional_scope":true,"production_changed":false,"adopted":false,"original_performance_screen_passed":false,"original_failed_checks":["1x2-w16/off/frozen","1x2-w16/on/frozen"],"primary32_gain_retained":true,"new_portable_tests":counts,"native_tests_inherited":{"off":119,"on":122},"new_native_tests":0,"functional_processes":45,"concurrent_processes":4,"resource_processes":8,"SDK_requests":requests,"new_independent_exports":exports,"independent_proof_count":14551,"resources":cases,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"scope":"portable all8+54; native1/4worker small-stack raw/basis/denied; native opt-outs; concurrent4callers/32workers primary/54; cold/warm192MiB cgroup primary no swap. No new latency/AWS/mobile/250/4999/minimumRAM result."});
    save(r("analysis.json"), &a);
    println!("Qualification passed;248SDK/89Gnark/ledger14551; portable11field99core8prover;8pressurecases; performancegate stillFALSE; sourceunchanged");
}
