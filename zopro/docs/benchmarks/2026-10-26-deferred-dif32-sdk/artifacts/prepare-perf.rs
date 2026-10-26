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
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32-sdk");
    assert_eq!(
        read(root.join("preflight/result.json"))["status"],
        "complete"
    );
    let validation = read(root.join("preflight/validate-inventory/stdout.log"));
    assert_eq!(validation["status"], "passed");
    assert_eq!(validation["distinct_gnark"], 18);
    let old =
        fs::read_to_string(root.join("preflight/accepted-layout-observation/stdout.log")).unwrap();
    let fresh =
        fs::read_to_string(root.join("preflight/candidate-layout-observation/stdout.log")).unwrap();
    assert_eq!(old, fresh);
    assert_eq!(old.trim(), "NATIVE_PLAN_BYTES=1728 ALIGN=32");
    let mut config = read(root.join("perf-template.json"));
    assert_eq!(config["monotonic_clock"], true);
    let mut prior: BTreeSet<String> =
        serde_json::from_value(config["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4084);
    let inventory = read(root.join("preflight-proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 36);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 4102);
    config["prior_proof_sha256"] = json!(prior);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("input-pins.json"))).unwrap();
    for name in [
        "prepare-perf.rs",
        "analyze.rs",
        "preflight-plan.json",
        "preflight/result.json",
        "preflight/validate-inventory/stdout.log",
        "preflight-proof-inventory.json",
        "preflight/accepted-layout-observation/stdout.log",
        "preflight/candidate-layout-observation/stdout.log",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    config["pins"] = json!(pins);
    fs::write(
        root.join("perf-plan.json"),
        serde_json::to_vec_pretty(&config).unwrap(),
    )
    .unwrap();
    let phase = json!({"status":"native113core tests bothmodes,raw2/cubic6 and all-nine4workerpreflight passed; fixed all-nine latency/profiles ready","candidate_sdk":"/tmp/zopro-deferred-dif32-sdk/native/candidate-sdk","candidate_sdk_sha256":"35cabbdfa0e5ab2e3e20f5ee359fdf787bb76f5a5fea5914f3071d4f865bde9c","preflight_exports":18,"preflight_sdk":72,"prior_before_perf":4102,"perf_pins":pins.len(),"layout_baseline_candidate_equal_bytes":1728,"goal_achieved":false,"adopted":false});
    fs::write(
        root.join("work-state.json"),
        serde_json::to_vec_pretty(&phase).unwrap(),
    )
    .unwrap();
    let h = Path::new("/home/ubuntu/src/zopro-cpu-perf-handoff.md");
    let previous = fs::read_to_string(h).unwrap();
    let headline="# CURRENT — 2026-10-26: DEFERRED DIF32 SDK LATENCY/PROFILES STARTING; COMPONENT PUSHED50cc0b8b\n\nGoal ACTIVE/UNMET all8CPU10x,CPU/Metalall8;no budget. root /tmp/zopro-deferred-dif32-sdk; exactcandidate173sources fieldbytesidenticalto32%fastercomponentpushed50cc0b8b. Nativeexec14751COMPLETE(core113OFF+113ON,raw2cubic6, libraries/sdk). Four newtests actualcolumn12800outputs/1209320canonicalrootlanes/12cachedcolumns; packedFFTvslegacyquad4096/8192/32768/262144cached+uncached;4concurrentcallers512KiB;rootpolicydefaultOFFandtwistedfalse. ActualPlanABI measuredusingFROZENacceptedandnewactualrlibs:1728bytesalign32 BOTH;candidateTwiddles448/Pair320. Newfieldkernelallmathunchanged; onlyDIFcolumns ON perPlanexplicitZOPRO_CPU_DEFERRED_DIF32=1,empty/twistedOFF; cached/uncachedcallbacksreadoriginalQuadroots. SDK SHA35cabbdfa0e5ab2e3e20f5ee359fdf787bb76f5a5fea5914f3071d4f865bde9c.\n\nPreflightexec31281COMPLETE42steps(layoutlinks+observations,all9EulerOFF/ONat4workers);72freshinternallyverifiedSDK18distinctGNARKexports36members,fixturepublic/freshnessvalidationpass. Prior4084->4102beforeperf. Newperf-plan.json frozenafterpreflight;perf-sourcepinsincludesanalyzer andall receipts, mandatorypreflightvalidationcheckagain.7blocks10cases3variantsaccepted03e126/OFF/ON;32workerall9=31measured+3primes,physical16primaryCPUs0-15=101+3;8610latencySDKthen9profilesprimary101/12x251/merge549 for510SDK,219perfexports. +preflight=9192benchmarkSDK237exportsIFallcomplete. DoNOTclaimuntilactualterminal. Source/actual9directlibs+4scheduler+63registrysrc+archives+keys/images/requests/verifierpinned. No ownotherCPUload permittedduringlatency. NoAWS. Exactbenchhandle fromtoolnext; don'tinferlivenessfromstatefile/progress.json. SDKscreennoadoptionyet;unsatisfied fullgoalpreserved. COMMIT/PUSH completedSDKexperimentbeforeanother;componentalreadypushed.\n\nOlderCURRENTbelowhistoricalsuperseded.\n\n";
    fs::write(h, headline.to_owned() + &previous).unwrap();
    println!(
        "Frozen{}perf pins4102prior; nativeand18preflightproofsverified; Planlayoutunchanged1728",
        pins.len()
    );
}
