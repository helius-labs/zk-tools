use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn receipt(p: impl AsRef<Path>) {
    let p = p.as_ref();
    let e = read(p.join("exit.json"));
    assert_eq!(e["success"], true, "{}", p.display());
    assert_eq!(e["returncode"], 0);
    for s in ["stdout", "stderr"] {
        assert_eq!(
            hash(p.join(format!("{s}.log"))),
            e[format!("{s}_sha256")].as_str().unwrap()
        );
    }
}
fn remove(s: &mut String, begin: &str, end: &str) {
    let st = s.find(begin).unwrap();
    assert_eq!(s.matches(begin).count(), 1);
    let en = s[st..].find(end).unwrap() + st + end.len();
    s.replace_range(st..en, "");
}
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let cfg = read(r.join("perf-plan-v2.json"));
    assert_eq!(
        hash(r.join("perf-plan.json")),
        hash(r.join("perf-plan-v2.json"))
    );
    assert_eq!(cfg["pins"].as_object().unwrap().len(), 361);
    for (p, h) in cfg["pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let relocation = read(r.join("pin-relocation.json"));
    for name in [
        "build-input-pins.json",
        "input-pins.json",
        "build-input-pins-v2.json",
        "input-pins-v2.json",
    ] {
        let pins = read(r.join(name));
        for (p, h) in pins.as_object().unwrap() {
            let actual = if name == "input-pins.json"
                && p == relocation["original_metadata_path"].as_str().unwrap()
            {
                relocation["preserved_original_metadata"].as_str().unwrap()
            } else {
                p
            };
            assert_eq!(hash(actual), h.as_str().unwrap(), "{name}:{p}");
        }
    }
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(base["source_files_sha256"].as_object().unwrap().len(), 170);
    assert_eq!(
        hash("/tmp/zopro-grouping-module/native/candidate-sdk"),
        "03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d"
    );
    fs::copy(
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        r.join("accepted-manifest.json"),
    )
    .unwrap();
    let mut source_recovery = Vec::new();
    for version in ["candidate", "candidate-v2"] {
        let mut recovered = 0;
        for name in [
            "crates/zopro-core/src/lib.rs",
            "crates/zopro-core/src/alloc.rs",
            "crates/zopro-core/src/alloc/linux.rs",
        ] {
            let mut s = fs::read_to_string(r.join(version).join(name)).unwrap();
            if name.ends_with("/lib.rs") {
                remove(
                    &mut s,
                    "        // BEGIN FUSED_PRODUCT_OVERLAP\n",
                    "        // END FUSED_PRODUCT_OVERLAP\n",
                );
                remove(
                    &mut s,
                    "\n// BEGIN FUSED_PRODUCT_OVERLAP_FLAG\n",
                    "// END FUSED_PRODUCT_OVERLAP_FLAG\n",
                );
            } else if name.ends_with("alloc.rs") {
                remove(
                    &mut s,
                    "\n// BEGIN FUSED_PRODUCT_OVERLAP_REEXPORT\n",
                    "// END FUSED_PRODUCT_OVERLAP_REEXPORT\n",
                );
            } else {
                remove(
                    &mut s,
                    "\n// BEGIN FUSED_PRODUCT_OVERLAP_RECLAIM\n",
                    "// END FUSED_PRODUCT_OVERLAP_RECLAIM\n",
                );
            }
            assert_eq!(
                digest(s.as_bytes()),
                base["source_files_sha256"][name].as_str().unwrap()
            );
            recovered += 1;
        }
        source_recovery.push(json!({"version":version,"recovered_original_whole_files":recovered}));
    }
    for stage in ["native", "native-v2"] {
        assert_eq!(
            read(r.join(stage).join("result.json"))["status"],
            "complete"
        );
        for mode in ["core-off", "core-on"] {
            receipt(r.join(stage).join(mode));
            assert_eq!(
                read(r.join(stage).join(mode).join("tests.json"))["passed_tests"]
                    .as_array()
                    .unwrap()
                    .len(),
                110
            );
        }
        receipt(r.join(stage).join("prover"));
        assert_eq!(
            read(r.join(stage).join("prover/tests.json"))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
    }
    let pre = read(r.join("preflight-v2/result.json"));
    assert_eq!(pre["status"], "complete");
    assert_eq!(pre["steps"].as_array().unwrap().len(), 43);
    for n in pre["steps"].as_array().unwrap() {
        let n = n.as_str().unwrap();
        let d = r.join("preflight-v2").join(n);
        receipt(&d);
        if n.ends_with("-verify") {
            assert!(fs::read_to_string(d.join("stdout.log"))
                .unwrap()
                .contains("verified 1 distinct proofs across 1 pinned keys"));
        }
    }
    let template = read(r.join("all-nine-template.json"));
    let prior: BTreeSet<String> =
        serde_json::from_value(template["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4321);
    let prior2: BTreeSet<String> = serde_json::from_value(
        read(r.join("all-nine-template-v2.json"))["prior_proof_sha256"].clone(),
    )
    .unwrap();
    assert_eq!(prior2.len(), 4330);
    assert_eq!(cfg["prior_proof_sha256"].as_array().unwrap().len(), 4348);
    let fixtures = template["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            (
                Path::new(c["image"].as_str().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                c["public_sha256"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for c in template["cases"].as_array().unwrap() {
        let shape = c["shape"].as_str().unwrap();
        for step in ["prove", "verify"] {
            receipt(r.join("preflight").join(format!("off-w4-{shape}-{step}")));
        }
        assert!(fs::read_to_string(
            r.join("preflight")
                .join(format!("off-w4-{shape}-verify/stdout.log"))
        )
        .unwrap()
        .contains("verified 1 distinct proofs across 1 pinned keys"));
    }
    let failed = read(r.join("preflight/on-w4-1x2-prove/exit.json"));
    assert_eq!(failed["success"], false);
    assert_eq!(failed["returncode"], 101);
    assert!(
        fs::read_to_string(r.join("preflight/on-w4-1x2-prove/stderr.log"))
            .unwrap()
            .contains("left: 30922")
    );
    let perf = read(r.join("perf/result.json"));
    assert_eq!(perf["status"], "complete");
    assert_eq!(perf["distinct_verified_exports"], 219);
    let rows = read(r.join("perf/rows.json"));
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 219);
    let mut sdk = 0;
    let mut samples = 0;
    let mut latency = 0;
    let mut profiles = BTreeSet::new();
    let mut profile_rows = Vec::new();
    for row in rows {
        let phase = row["phase"].as_str().unwrap();
        let name = row["case"].as_str().unwrap();
        let variant = row["variant"].as_str().unwrap();
        let block = row["block"].as_u64().unwrap();
        let d = r
            .join("perf")
            .join(phase)
            .join(format!("b{block}"))
            .join(name)
            .join(variant);
        receipt(d.join("run"));
        receipt(d.join("verify"));
        assert!(fs::read_to_string(d.join("verify/stdout.log"))
            .unwrap()
            .contains("verified 1 distinct proofs across 1 pinned keys"));
        assert_eq!(row["summary"]["priming"], 3);
        assert_eq!(
            row["summary"]["workers"],
            if name == "1x2-w16" { 16 } else { 32 }
        );
        sdk += row["samples_ms"].as_array().unwrap().len() + 3;
        let cmd = read(d.join("run/command.json"));
        let argv = cmd["argv"].as_array().unwrap();
        assert!(!argv.iter().any(|v| v
            .as_str()
            .is_some_and(|s| s.contains("FUSED_PRODUCT_OVERLAP_TRACE"))));
        if phase == "profile" {
            receipt(d.join("symbols"));
            assert_eq!(row["profile"]["lost_records"], 0);
            assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
            samples += row["profile"]["samples"].as_u64().unwrap();
            assert!(profiles.insert((name.to_owned(), variant.to_owned(), block)));
            let case = cfg["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["name"] == name)
                .unwrap();
            assert_eq!(
                row["samples_ms"].as_array().unwrap().len(),
                case["profile_rounds"].as_u64().unwrap() as usize
            );
            assert!(argv.iter().any(|v| v == "--no-buildid-cache"));
            assert!(argv
                .windows(2)
                .any(|p| p[0] == "--clockid" && p[1] == "mono"));
            assert!(argv
                .iter()
                .any(|v| v.as_str().is_some_and(|s| s.starts_with("--control=fifo:"))));
            let n = row["samples_ms"].as_array().unwrap().len() as f64;
            profile_rows.push(json!({"case":name,"variant":variant,"cpu_ms_per_request":(row["summary"]["user_cpu_ms"].as_f64().unwrap()+row["summary"]["system_cpu_ms"].as_f64().unwrap())/n,"mean_wall_ms":row["summary"]["loop_wall_ms"].as_f64().unwrap()/n,"category_percent":row["profile"]["category_percent"],"samples":row["profile"]["samples"],"lost_records":0,"scope":"One warm capture, sample shares are not removable wall cost or a causal model."}));
        } else {
            assert_eq!(phase, "latency");
            latency += 1;
        }
    }
    assert_eq!(sdk, 9120);
    assert_eq!(latency, 210);
    assert_eq!(profiles.len(), 9);
    for name in ["1x2-w32", "12x2-w32", "merge-54x1-w32"] {
        for v in ["accepted", "off", "on"] {
            assert!(profiles.contains(&(name.into(), v.into(), 0)));
        }
    }
    let mut inventory = BTreeMap::new();
    for file in [
        "initial-proof-inventory.json",
        "preflight-proof-inventory-v2.json",
    ] {
        for (p, h) in read(r.join(file)).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            assert!(inventory
                .insert(p.clone(), h.as_str().unwrap().to_owned())
                .is_none());
        }
    }
    for (p, h) in read(r.join("perf/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        let p = r.join("perf").join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        assert!(inventory
            .insert(
                p.to_str().unwrap().to_owned(),
                h.as_str().unwrap().to_owned()
            )
            .is_none());
    }
    assert_eq!(inventory.len(), 492);
    let mut unique = BTreeSet::new();
    for (p, h) in &inventory {
        let p = Path::new(p);
        if p.extension().is_some_and(|e| e == "proof") {
            assert!(unique.insert(h.clone()));
            assert!(!prior.contains(h));
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
            assert!(inventory.contains_key(p.with_extension("public").to_str().unwrap()));
            if !p.starts_with(r.join("preflight-proofs")) {
                assert!(!prior2.contains(h));
            }
        } else {
            assert_eq!(p.extension().unwrap(), "public");
            let stem = p.parent().unwrap().file_name().unwrap().to_str().unwrap();
            assert_eq!(&fixtures[stem], h);
        }
    }
    assert_eq!(unique.len(), 246);
    let old =
        fs::read_to_string(r.join("preflight-v2/accepted-layout-observation/stdout.log")).unwrap();
    let new =
        fs::read_to_string(r.join("preflight-v2/candidate-layout-observation/stdout.log")).unwrap();
    assert_eq!(old, new);
    assert_eq!(old.trim(), "NATIVE_PLAN_BYTES=1728 ALIGN=32");
    assert_eq!(read(r.join("activation-audit-v2.json"))["status"], "passed");
    let pilot = read(r.join("pilot-result.json"));
    assert_eq!(pilot["performance_gate_passed"], false);
    assert_eq!(pilot["failed_checks"].as_array().unwrap().len(), 13);
    assert!(pilot["cases"]
        .as_object()
        .unwrap()
        .values()
        .all(|v| v["off_calibration_factor"].as_f64().unwrap() <= 1.03));
    let out = json!({"status":"closed_verified; fixed speed gate failed; not adopted","adopted":false,"goal_achieved":false,"sdk_requests":sdk+72+36,"distinct_gnark_exports":246,"proof_public_members":492,"v2_requests":sdk+72,"v2_exports":237,"v1_successful_requests":36,"v1_successful_exports":9,"failed_on_completed_sdk_requests":0,"latency_processes":210,"profiles":9,"profile_samples":samples,"lost_records":0,"all_declared_profile_tuples_rounds_clock_FIFO_flags_verified":true,"performance_pins":361,"initial_pins_rehashed_with_single_exact_metadata_relocation":319,"core_tests_per_mode_each_version":110,"raw_tests_each_version":2,"cubic_tests_each_version":6,"field_changes":false,"workspace_forecast_changes":false,"source_recovery":source_recovery,"accepted_170_files_and_sdk_unchanged":true,"source_pins_each_version":172,"prior_before_initial_preflight":4321,"prior_before_v2_preflight":4330,"prior_before_perf":4348,"failed_checks":pilot["failed_checks"],"primary_and_original8_fail_independently_of_other_checks":true,"all_off_calibration_checks_passed":true,"initial_guard_failure":"Euler row count was30922 vsincorrectexpectedpaddeddomain32768; repaired before timings byEuleronlycount=max(a.len,b.len).","summary_cli_failure":"Measurementscomplete first; frozen reader initially calledwithout2requiredarguments. Correctedinvocation and byte-identicalconfigalias; no source/gate/timing rerun.","metadata_relocation":relocation,"no_new_pressure_portable_mobile_large_cloud_or_aeglos_qualification":true});
    fs::write(
        r.join("closeout.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    fs::write(
        r.join("proof-inventory.json"),
        serde_json::to_vec_pretty(&inventory).unwrap(),
    )
    .unwrap();
    fs::write(r.join("profile-analysis.json"),serde_json::to_vec_pretty(&json!({"samples":samples,"profiles":profile_rows,"lost_records":0,"scope":"All9warmtuples; onecaptureeach, not exclusive phase costs or causal attribution."})).unwrap()).unwrap();
    println!("CLOSED9228SDK246Gnarkexports492members361pins9profiles{samples}samples0lost;13speedfails; production unchanged");
}
