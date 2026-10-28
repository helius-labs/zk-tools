use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-forced-pressure";
const V: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(seen.len(), 14551);
    let mut rows = vec![];
    let mut inv = BTreeMap::new();
    let mut requests = 0;
    let mut exports = 0;
    let mut passed = true;
    for c in read(r("resource-cases.json")).as_array().unwrap() {
        let name = c["name"].as_str().unwrap();
        let dir = r("runs").join(name);
        if !dir.exists() {
            passed = false;
            rows.push(json!({"name":name,"executed":false,"passed":false,"reason":"controller stopped after a prior failure; no restart/larger retry"}));
            continue;
        }
        let result = dir.join("result.json");
        let a = if result.exists() {
            read(&result)
        } else {
            json!({"passed":false})
        };
        let high = a["memory_events"]["high"].as_u64().unwrap_or(0);
        let ok = a["passed"] == true
            && high > 0
            && a["metrics"]["memory.high"] == "100663296"
            && a["metrics"]["memory.max"] == "201326592"
            && a["metrics"]["memory.swap.max"] == "0"
            && a["memory_events"]["oom"] == 0
            && a["memory_events"]["oom_kill"] == 0
            && a["memory_events"]["max"] == 0;
        passed &= ok;
        let proofdir = dir.join("proofs/transfer_confidential_1_2");
        let mut count = 0;
        for i in 0..2 {
            let p = proofdir.join(format!("{i}.proof"));
            if !p.exists() {
                continue;
            }
            let public = p.with_extension("public");
            assert_eq!(fs::metadata(&p).unwrap().len(), 164);
            let h = hash(&p);
            let ph = hash(&public);
            assert_eq!(
                ph,
                "3f0d846c3905d11595d77dd988df829ddc51325eda2b93e4db8f0e17a5928e5b"
            );
            let log = dir.join("verifier.log");
            let t = fs::read_to_string(&log).unwrap_or_default();
            let ih = c["image_sha256"].as_str().unwrap();
            if !t.contains(&format!("VERIFIED proof_sha256={h}")) {
                fs::create_dir_all(r("extra-verification")).unwrap();
                let argv = vec![
                    c["image"].as_str().unwrap().to_string(),
                    ih.into(),
                    p.display().to_string(),
                    public.display().to_string(),
                    ph.clone(),
                ];
                let o = Command::new(V).args(&argv).output().unwrap();
                fs::write(
                    r(format!("extra-verification/{name}-{i}.stdout.log")),
                    &o.stdout,
                )
                .unwrap();
                fs::write(
                    r(format!("extra-verification/{name}-{i}.stderr.log")),
                    &o.stderr,
                )
                .unwrap();
                save(
                    r(format!("extra-verification/{name}-{i}.json")),
                    &json!({"argv":argv,"exit":o.status.code()}),
                );
                assert!(o.status.success());
                assert!(String::from_utf8(o.stdout)
                    .unwrap()
                    .contains(&format!("VERIFIED proof_sha256={h}")));
            } else {
                assert!(t.contains(&format!("public_sha256={ph}")));
                assert!(t.contains(&format!("image_sha256={ih}")));
            }
            assert!(seen.insert(h.clone()));
            inv.insert(p.display().to_string(), h);
            inv.insert(public.display().to_string(), ph);
            count += 1;
            exports += 1;
        }
        let sdklog = fs::read_to_string(dir.join("sdk.log")).unwrap_or_default();
        let complete = sdklog.lines().find_map(|l| l.strip_prefix("COMPLETE "));
        let known = if let Some(l) = complete {
            let s: Value = serde_json::from_str(l).unwrap();
            assert_eq!(s["SDK_requests"], 5);
            assert_eq!(s["before_tids"], s["after_tids"]);
            assert_eq!(s["workers"], 32);
            5
        } else if dir.join("ready").exists() {
            3 + count
        } else {
            0
        };
        requests += known;
        rows.push(json!({"name":name,"executed":true,"passed":ok,"high_events":high,"known_verified_SDK_requests":known,"independent_exports":count,"result":a}));
    }
    if passed {
        assert_eq!(exports, 8);
        assert_eq!(requests, 20);
        assert_eq!(seen.len(), 14559);
    }
    assert_eq!(seen.len(), 14551 + exports);
    save(r("proof-inventory.json"), &json!(inv));
    let units = Command::new("systemctl")
        .args([
            "--user",
            "list-units",
            "--all",
            "--no-legend",
            "zopro-borrow-forced-*",
        ])
        .output()
        .unwrap();
    assert!(units.status.success());
    fs::write(r("cleanup-units.txt"), &units.stdout).unwrap();
    assert!(
        String::from_utf8(units.stdout).unwrap().trim().is_empty(),
        "owned service remains"
    );
    let a = json!({"status":if passed{"forced high-limit pressure passed"}else{"forced high-limit pressure failed; no larger retry"},"qualified_pressure":passed,"rows":rows,"known_verified_SDK_requests":requests,"new_independent_exports":exports,"independent_proof_count":seen.len(),"all_owned_services_cleaned":true,"source_changed":false,"production_changed":false,"adopted":false,"original_performance_gate_passed":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"limits":{"high":100663296,"max":201326592,"ballast":117440512,"swap":0},"scope":"four primary1x2 warm cases; actual high events required; not latency, minimumRAM, swap, mobile, AWS or250/4999 qualification"});
    save(r("analysis.json"), &a);
    println!(
        "Forcedpressure{passed};{requests}knownSDK/{exports}freshGnark;ledger{};ownedservicesclean",
        seen.len()
    );
    for row in rows {
        println!(
            "{} pass{} high{}",
            row["name"], row["passed"], row["high_events"]
        );
    }
}
