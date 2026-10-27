use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/tmp/zopro-pmu-c7a-20261027";
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
fn proofs(dir: &Path, out: &mut Vec<PathBuf>) {
    if !dir.exists() {
        return;
    }
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            proofs(&p, out)
        } else if p.extension().is_some_and(|v| v == "proof") {
            out.push(p)
        }
    }
}
fn main() {
    let r = Path::new(R);
    let m = read(r.join("context/manifest.json"));
    let mut seen: BTreeSet<String> =
        serde_json::from_value(m["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 12143);
    let local = r.join("local-output/preflight");
    let cloud = r.join("remote-results/output/measure");
    let lc = read(local.join("complete.json"));
    assert_eq!(lc["status"], "complete");
    assert_eq!(lc["sdk_requests"], 8);
    assert_eq!(lc["distinct_gnark_exports"], 2);
    let mut inv = BTreeMap::new();
    let mut local_exports = 0;
    let mut cloud_exports = 0;
    for (kind, root) in [("local", &local), ("cloud", &cloud)] {
        let mut files = Vec::new();
        proofs(root, &mut files);
        for p in files {
            assert_eq!(fs::metadata(&p).unwrap().len(), 164);
            let h = hash(&p);
            assert!(seen.insert(h));
            assert_eq!(
                hash(p.with_extension("public")),
                m["case"]["public_sha256"].as_str().unwrap()
            );
            for f in [p.clone(), p.with_extension("public")] {
                inv.insert(f.display().to_string(), hash(f));
            }
            if kind == "local" {
                local_exports += 1
            } else {
                cloud_exports += 1
            }
        }
    }
    assert_eq!(local_exports, 2);
    let mut quality = Vec::new();
    let complete = if cloud.join("complete.json").exists() {
        let v = read(cloud.join("complete.json"));
        assert_eq!(v["status"], "complete");
        assert_eq!(v["sdk_requests"], 13112);
        assert_eq!(v["distinct_gnark_exports"], 28);
        assert_eq!(cloud_exports, 28);
        true
    } else {
        quality.push("Cloud diagnostic did not complete; retained partial exports and receipts without recapture".to_owned());
        false
    };
    let mut post_verified = 0;
    if cloud_exports > 0 {
        let log = fs::File::create(r.join("post-transport-gnark.log")).unwrap();
        let status = Command::new("/tmp/zopro-gnark-verify/verify")
            .args([
                "/tmp/zopro-keys",
                cloud.to_str().unwrap(),
                "/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock",
            ])
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        assert!(status.success());
        let out = fs::read_to_string(r.join("post-transport-gnark.log")).unwrap();
        assert!(out.contains(&format!(
            "verified {cloud_exports} distinct proofs across 1 pinned keys"
        )));
        post_verified = cloud_exports;
        save(
            r.join("post-transport-gnark-exit.json"),
            &json!({"success":true,"code":status.code(),"distinct_verified":post_verified,"repeat_verification_not_new_exports":true}),
        );
    }
    let rows = if cloud.join("rows.json").exists() {
        read(cloud.join("rows.json"))
    } else {
        json!([])
    };
    let mut sdk_lower = 8u64;
    let mut cycle_samples = 0;
    if complete {
        assert_eq!(rows.as_array().unwrap().len(), 28);
        for row in rows.as_array().unwrap() {
            let j = &row["job"];
            assert_eq!(row["sdk"]["summary"]["workers"], j["workers"]);
            assert_eq!(row["sdk"]["summary"]["priming"], 3);
            let rounds = j["rounds"].as_u64().unwrap();
            assert_eq!(
                row["sdk"]["samples_ms"].as_array().unwrap().len(),
                rounds as usize
            );
            sdk_lower += rounds + 3;
            if j["phase"] == "counter" {
                assert_eq!(rounds, 501);
                let c = row["counters"].as_object().unwrap();
                assert_eq!(c.len(), 4);
                for v in c.values() {
                    assert!(v["count"].as_f64().unwrap() >= 0.);
                    assert!(v["runtime_ns"].as_f64().unwrap() > 0.);
                    assert!(v["running_percent"].as_f64().unwrap() >= 99.9);
                }
            } else {
                assert_eq!(rounds, 251);
                assert_eq!(row["profile"]["lost_records"], 0);
                assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
                cycle_samples += row["profile"]["samples"].as_u64().unwrap();
            }
        }
        assert_eq!(sdk_lower, 13120);
        assert_eq!(seen.len(), 12173);
    } else if cloud.join("jobs.json").exists() {
        for j in read(cloud.join("jobs.json")).as_array().unwrap() {
            let name = j["name"].as_str().unwrap();
            let out = cloud.join(format!("workflow/measure-{name}/stdout.log"));
            let exit = cloud.join(format!("workflow/measure-{name}/exit.json"));
            if out.exists() && exit.exists() && read(exit)["success"] == true {
                if let Some(s) = fs::read_to_string(out)
                    .unwrap()
                    .lines()
                    .find_map(|l| l.strip_prefix("SUMMARY "))
                {
                    let s: Value = serde_json::from_str(s).unwrap();
                    sdk_lower += s["rounds"].as_u64().unwrap() + s["priming"].as_u64().unwrap();
                }
            }
        }
    }
    for (f, h) in read("/tmp/zopro-clean-fft-adoption/production-source-manifest.json")
        ["source_files_sha256"]
        .as_object()
        .unwrap()
    {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(f)),
            h.as_str().unwrap()
        );
    }
    for f in ["current-field", "current-core", "current-prover"] {
        let n = read(r.join(format!("native/{f}/tests.json")))["passed_tests"]
            .as_array()
            .unwrap()
            .len();
        assert_eq!(
            n,
            match f {
                "current-field" => 33,
                "current-core" => 119,
                _ => 8,
            }
        );
    }
    let mut identity = read("/tmp/zopro-clean-fft-adoption/production-source-manifest.json");
    identity["native_binary"] = read(r.join("native/current/artifacts.json"))["binary"].clone();
    identity["native_binary_sha256"] =
        read(r.join("native/current/artifacts.json"))["binary_sha256"].clone();
    identity["rustflags"] = m["current_rustflags"].clone();
    identity["native_cli_build"] = json!(r.join("native/current/artifacts.json"));
    identity["source_changed"] = json!(false);
    identity["scope"] =
        json!("Exact accepted172 source freshlybuiltznver4forC7adiagnostic; local14daSDKunchanged");
    save(r.join("aws-accepted-manifest.json"), &identity);
    save(r.join("proof-inventory.json"), &json!(inv));
    save(
        r.join("analysis.json"),
        &json!({"status":if complete{"complete current-source C7a PMU comparison"}else{"partial current-source C7a PMU comparison"},"complete":complete,"acceptance_measurement":false,"source_changed":false,"SDK_requests":if complete{json!(13120)}else{Value::Null},"completed_SDK_calls_lower_bound":sdk_lower,"independent_exports":local_exports+cloud_exports,"local_exports":local_exports,"cloud_exports":cloud_exports,"post_transport_distinct_verified":post_verified,"proof_public_members":(local_exports+cloud_exports)*2,"prior_count":12143,"next_prior_count":seen.len(),"cycle_samples":cycle_samples,"rows":rows,"quality_failures":quality,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    println!(
        "Analyzed complete={complete};verifiedexports{};ledger{};source unchanged",
        local_exports + cloud_exports,
        seen.len()
    );
}
