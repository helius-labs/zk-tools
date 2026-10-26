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
fn check(dir: &Path) {
    let e = read(dir.join("exit.json"));
    assert_eq!(e["success"], true);
    assert_eq!(e["returncode"], 0);
    for s in ["stdout", "stderr"] {
        assert_eq!(
            hash(dir.join(format!("{s}.log"))),
            e[format!("{s}_sha256")].as_str().unwrap()
        );
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32-sdk");
    let cfg = read(root.join("perf-plan.json"));
    let perf = read(root.join("perf/result.json"));
    assert_eq!(perf["status"], "complete");
    assert_eq!(perf["distinct_verified_exports"], 219);
    assert_eq!(cfg["pins"].as_object().unwrap().len(), 336);
    for (p, h) in cfg["pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let pre = read(root.join("preflight/result.json"));
    assert_eq!(pre["status"], "complete");
    assert_eq!(pre["steps"].as_array().unwrap().len(), 42);
    for name in pre["steps"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let dir = root.join("preflight").join(name);
        check(&dir);
        if name.ends_with("-verify") {
            assert!(fs::read_to_string(dir.join("stdout.log"))
                .unwrap()
                .contains("verified 1 distinct proofs across 1 pinned keys"));
        }
    }
    let rows = read(root.join("perf/rows.json"));
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 219);
    let mut sdk = 0;
    let mut sample_count = 0;
    let mut latency = 0;
    let mut profiles = BTreeSet::new();
    for r in rows {
        let phase = r["phase"].as_str().unwrap();
        let name = r["case"].as_str().unwrap();
        let variant = r["variant"].as_str().unwrap();
        let block = r["block"].as_u64().unwrap();
        let dir = root
            .join("perf")
            .join(phase)
            .join(format!("b{block}"))
            .join(name)
            .join(variant);
        check(&dir.join("run"));
        check(&dir.join("verify"));
        assert!(fs::read_to_string(dir.join("verify/stdout.log"))
            .unwrap()
            .contains("verified 1 distinct proofs across 1 pinned keys"));
        assert_eq!(r["summary"]["priming"], 3);
        assert_eq!(
            r["summary"]["workers"],
            if name == "1x2-w16" { 16 } else { 32 }
        );
        sdk += r["samples_ms"].as_array().unwrap().len() + 3;
        if phase == "profile" {
            check(&dir.join("symbols"));
            assert_eq!(r["profile"]["lost_records"], 0);
            assert_eq!(r["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
            sample_count += r["profile"]["samples"].as_u64().unwrap();
            assert!(profiles.insert((name.to_owned(), variant.to_owned(), block)));
            let case = cfg["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["name"] == name)
                .unwrap();
            assert_eq!(
                r["samples_ms"].as_array().unwrap().len(),
                case["profile_rounds"].as_u64().unwrap() as usize
            );
        } else {
            assert_eq!(phase, "latency");
            latency += 1;
        }
    }
    assert_eq!(sdk, 9120);
    assert_eq!(latency, 210);
    assert_eq!(profiles.len(), 9);
    for case in ["1x2-w32", "12x2-w32", "merge-54x1-w32"] {
        for variant in ["accepted", "off", "on"] {
            assert!(profiles.contains(&(case.into(), variant.into(), 0)));
        }
    }
    let template = read(root.join("all-nine-template.json"));
    let prior: BTreeSet<String> =
        serde_json::from_value(template["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4084);
    let fixtures: BTreeMap<String, String> = template["cases"]
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
                    .into(),
                c["public_sha256"].as_str().unwrap().into(),
            )
        })
        .collect();
    let mut inventory = BTreeMap::<String, String>::new();
    for (p, h) in read(root.join("preflight-proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        assert!(inventory
            .insert(p.clone(), h.as_str().unwrap().into())
            .is_none());
    }
    for (p, h) in read(root.join("perf/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        let p = root.join("perf").join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        assert!(inventory
            .insert(p.to_str().unwrap().into(), h.as_str().unwrap().into())
            .is_none());
    }
    assert_eq!(inventory.len(), 474);
    let mut unique = BTreeSet::new();
    for (p, h) in &inventory {
        let p = Path::new(p);
        if p.extension().is_some_and(|e| e == "proof") {
            assert!(unique.insert(h.clone()));
            assert!(!prior.contains(h));
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
            assert!(inventory.contains_key(p.with_extension("public").to_str().unwrap()));
        } else {
            assert_eq!(p.extension().unwrap(), "public");
            let stem = p.parent().unwrap().file_name().unwrap().to_str().unwrap();
            assert_eq!(&fixtures[stem], h);
        }
    }
    assert_eq!(unique.len(), 237);
    for name in ["core-off", "core-on"] {
        let v = read(root.join("native").join(name).join("tests.json"));
        assert_eq!(v["passed_tests"].as_array().unwrap().len(), 113);
    }
    assert_eq!(
        read(root.join("native/prover/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    let a =
        fs::read_to_string(root.join("preflight/accepted-layout-observation/stdout.log")).unwrap();
    let b =
        fs::read_to_string(root.join("preflight/candidate-layout-observation/stdout.log")).unwrap();
    assert_eq!(a, b);
    assert_eq!(a.trim(), "NATIVE_PLAN_BYTES=1728 ALIGN=32");
    let audit = read(root.join("source-audit.json"));
    assert_eq!(audit["component_field_identity"], true);
    assert_eq!(audit["original_core_files_recovered"], true);
    let pilot = read(root.join("pilot-result.json"));
    assert_eq!(pilot["performance_gate_passed"], false);
    assert_eq!(pilot["failed_checks"].as_array().unwrap().len(), 6);
    let out = json!({"status":"SDK experiment complete; speed screen failed; not adopted","adopted":false,"goal_achieved":false,"sdk_requests":sdk+72,"distinct_gnark_exports":237,"proof_public_members":474,"profiles":9,"profile_samples":sample_count,"lost_records":0,"all_declared_profile_tuples_and_rounds_verified":true,"native_core_off":113,"native_core_on":113,"native_raw_tests":2,"native_cubic_tests":6,"field_qualification":"Byte-identical to pushed component:33fieldtests+6independent integer/Fourier tests reused, not rerun here.","source_pins":173,"performance_pins":336,"prior_before_preflight":4084,"prior_before_perf":4102,"actual_plan_size_accepted_and_candidate":1728,"actual_plan_alignment_both":32,"failed_checks":pilot["failed_checks"],"original8_aggregate":pilot["original8_aggregate"],"decision":"Do not adopt or rerun unchanged. Component's32%gain does not establish fullSDKimprovement; original8aggregate is flat/slower and fixed block/control checks fail. NativePlanheader unchanged and no new arithmetic heapbuffer, but no new memory-pressure/portable/concurrentSDK/huge-key qualification or minimumRAM claim.","limitations":"One profile pertuple; sampledshares not wallcosts. HistoricalsuppliedM5CPU/Metalrefs, no newApple/AWS/Aeglosresult. All8CPU10xgoalunmet."});
    fs::write(
        root.join("closeout.json"),
        format!("{}\n", serde_json::to_string_pretty(&out).unwrap()),
    )
    .unwrap();
    fs::write(
        root.join("proof-inventory.json"),
        serde_json::to_vec_pretty(&inventory).unwrap(),
    )
    .unwrap();
    println!("CLOSED VERIFIED9192SDK237Gnarkexports474members9profiles{sample_count}samples0lost;6speedchecksfailed");
}
