use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn main() {
    let r = env::current_dir().unwrap();
    let a = env::args().nth(1).unwrap();
    let out = r.join("remote/output");
    let p = r.join("partial");
    let verifier = r.join("bundle/bin/verify");
    assert_eq!(read("retrieved.json")["verified_remote_digest"], true);
    assert_eq!(
        read(out.join("independent-preflight.json"))["checked_exports"],
        54
    );
    let failed = out.join("latency-run/latency-merge-54x1-w96-ON-plain-b00");
    let exit = read(failed.join("exit.json"));
    assert_eq!(exit["success"], false);
    assert_eq!(hash(failed.join("stderr.log")), exit["stderr_sha256"]);
    assert!(fs::read_to_string(failed.join("stderr.log"))
        .unwrap()
        .contains("overflowed its stack"));
    let mut steps = vec![];
    let mut rows = vec![];
    let mut pins = BTreeMap::new();
    pins.insert(verifier.clone(), hash(&verifier));
    let mut seed: BTreeSet<String> = serde_json::from_value(read("local-ledger.json")).unwrap();
    assert_eq!(seed.len(), 64457);
    assert_eq!(
        hash("local-ledger.json"),
        "b6af87ea95fa5c3b03732e8d3c069ca7c147f4b1ab8c114ccfbd8a63c7a19e99"
    );
    for j in read(out.join("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let phase = j["phase"].as_str().unwrap();
        let logs = out.join(format!("{phase}-run")).join(name);
        if !logs.join("exit.json").exists() || read(logs.join("exit.json"))["success"] != true {
            continue;
        }
        let e = read(logs.join("exit.json"));
        assert_eq!(e["returncode"], 0);
        for s in ["stdout", "stderr"] {
            assert_eq!(
                hash(logs.join(format!("{s}.log"))),
                e[format!("{s}_sha256")]
            );
        }
        let raw = fs::read_to_string(logs.join("stdout.log")).unwrap();
        let sums: Vec<Value> = raw
            .lines()
            .filter_map(|v| v.strip_prefix("SUMMARY "))
            .map(|v| serde_json::from_str(v).unwrap())
            .collect();
        assert_eq!(sums.len(), 1);
        let s = &sums[0];
        assert_eq!(s["placement_before"], s["placement_after"]);
        assert_eq!(s["workers"], j["workers"]);
        assert_eq!(s["priming"], 3);
        let samples: Vec<Value> = raw
            .lines()
            .filter_map(|v| v.strip_prefix("SAMPLE "))
            .map(|v| serde_json::from_str(v).unwrap())
            .collect();
        assert_eq!(samples.len(), j["rounds"].as_u64().unwrap() as usize);
        assert_eq!(s["rounds"], j["rounds"]);
        let mut total = 0.;
        for (i, x) in samples.iter().enumerate() {
            assert_eq!(x["round"], i);
            let ms = x["ms"].as_f64().unwrap();
            assert!(ms.is_finite() && ms > 0.);
            total += ms;
        }
        if j["arm"] != "REF" {
            assert_eq!(
                s["restricted_requests"],
                if j["arm"] == "ON" && j["workers"] == 96 {
                    samples.len()
                } else {
                    0
                }
            );
            assert_eq!(s["restriction_bypasses"], 0);
        }
        let proof = out.join("proofs").join(name).join("0.proof");
        let public = proof.with_file_name("0.public");
        let h = hash(&proof);
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        assert!(seed.insert(h.clone()));
        let ph = hash(&public);
        assert_eq!(ph, j["case"]["public_sha256"]);
        let image = r.join("bundle").join(j["case"]["image"].as_str().unwrap());
        let ih = j["case"]["image_sha256"].as_str().unwrap().to_owned();
        pins.insert(image.clone(), ih.clone());
        pins.insert(proof.clone(), h.clone());
        pins.insert(public.clone(), ph.clone());
        steps.push(json!({"kind":"command","name":name,"argv":[verifier,image,PathBuf::from(&ih),proof,public,PathBuf::from(&ph)],"timeout_seconds":300}));
        if a == "finish" {
            let v = p.join("run").join(name);
            let ve = read(v.join("exit.json"));
            assert_eq!(ve["success"], true);
            assert_eq!(hash(v.join("stdout.log")), ve["stdout_sha256"]);
            let verdict = fs::read_to_string(v.join("stdout.log")).unwrap();
            assert!(verdict.contains(&format!("VERIFIED proof_sha256={h} ")));
            assert!(verdict.contains(&format!(" image_sha256={ih}")));
        }
        let n = samples.len() as f64;
        rows.push(json!({"name":name,"shape":j["case"]["shape"],"phase":phase,"workers":j["workers"],"arm":j["arm"],"timed":samples.len(),"ms":total/n,"cpu_ms":(s["user_cpu_ms"].as_f64().unwrap()+s["system_cpu_ms"].as_f64().unwrap())/n,"rss_mib":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"proof_sha256":h}));
    }
    assert_eq!(steps.len(), 107);
    assert_eq!(seed.len(), 64564);
    if a == "prepare" {
        fs::create_dir(&p).unwrap();
        save(p.join("pins.json"), json!(pins));
        save(
            p.join("plan.json"),
            json!({"version":1,"source_pins":p.join("pins.json"),"steps":steps}),
        );
    } else {
        assert_eq!(a, "finish");
        assert_eq!(read(p.join("run/result.json"))["status"], "complete");
        save(p.join("rows.json"), json!(rows));
        save(p.join("ledger.json"), json!(seed));
        println!(
            "107 exports verified; total 64564 hash {}",
            hash(p.join("ledger.json"))
        );
    }
}
