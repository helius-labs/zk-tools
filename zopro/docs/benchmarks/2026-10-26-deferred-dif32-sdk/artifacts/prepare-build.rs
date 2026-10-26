use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
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
    let mut protocol = read(root.join("experiment.json"));
    protocol["constraints"]=json!("No new retained/per-proof heap buffer, lookup table, preparation, transformed key, root cache, worker or scheduler. Arithmetic adds no heap allocation. Standard environment parsing once per Plan may allocate an ephemeral flag string at construction. Record actual native type sizes and linked frames; preserve resource accounting. Component field bytes must match.");
    fs::write(
        root.join("experiment.json"),
        serde_json::to_vec_pretty(&protocol).unwrap(),
    )
    .unwrap();
    let mut plan = read("/tmp/zopro-serial-fft-leaves/native-plan-v2.json");
    let text = serde_json::to_string(&plan)
        .unwrap()
        .replace(
            "/tmp/zopro-serial-fft-leaves/candidate-v2",
            "/tmp/zopro-deferred-dif32-sdk/candidate",
        )
        .replace(
            "/tmp/zopro-serial-fft-leaves/target-native-v2",
            "/tmp/zopro-deferred-dif32-sdk/target-native",
        )
        .replace(
            "/tmp/zopro-serial-fft-leaves/native-v2",
            "/tmp/zopro-deferred-dif32-sdk/native",
        )
        .replace("zopro_serial_fft_leaves", "zopro_deferred_dif32")
        .replace("ZOPRO_CPU_SERIAL_FFT_LEAVES", "ZOPRO_CPU_DEFERRED_DIF32");
    plan = serde_json::from_str(&text).unwrap();
    let names=["fft::pair::deferred::tests::plan_samples_explicit_mode_and_twisted_roots_keep_fallback","fft::pair::deferred::tests::actual_packed_columns_match_quad_reference_with_canonical_roots","fft::pair::deferred::tests::packed_full_transforms_match_legacy_quads_with_cached_and_uncached_roots","fft::pair::deferred::tests::concurrent_shared_root_transforms_use_small_worker_stacks"];
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["name"] == "core-off" || step["name"] == "core-on" {
            step["required_tests"] = json!(names);
            step["argv"]
                .as_array_mut()
                .unwrap()
                .push(json!("--nocapture"));
        }
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("candidate/.xtask/source-pins.json"))).unwrap();
    for name in [
        "experiment.json",
        "deferred.rs",
        "implement.rs",
        "prepare-build.rs",
        "source-audit.json",
        "prepare.json",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let component = Path::new("/tmp/zopro-deferred-dif32");
    for name in [
        "closeout.json",
        "profile-analysis.json",
        "component-result.json",
        "source-audit.json",
        "oracle-qualification.json",
        "build/field-tests/tests.json",
        "build/field-tests/exit.json",
        "build/generator-check/exit.json",
        "build-v3/result.json",
        "push-receipt.json",
    ] {
        let p = component.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let xtask = Path::new("/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask");
    pins.insert(xtask.to_str().unwrap().into(), hash(xtask));
    plan["source_pins"] = json!(root.join("build-input-pins.json"));
    fs::write(
        root.join("build-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("native-plan.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    let state = json!({"status":"newSDKintegration declared and source frozen; native tests/build pending","parent_pushed_commit":"50cc0b8bffba2b51a68249d8691c94df891a87a4","experiment":root.join("experiment.json"),"source_pins":173,"build_pins":pins.len(),"goal_active":true,"goal_achieved":false,"owned_live_jobs":"Inspect exact native workflow exec handle; do not infer liveness from this file.","accepted_unchanged":true});
    fs::write(
        root.join("work-state.json"),
        serde_json::to_vec_pretty(&state).unwrap(),
    )
    .unwrap();
    let h = Path::new("/home/ubuntu/src/zopro-cpu-perf-handoff.md");
    let old = fs::read_to_string(h).unwrap();
    let heading="# CURRENT — 2026-10-26: DEFERRED DIF32 SDK INTEGRATION BUILDING; COMPONENT PUSHED50cc0b8b\n\nGoal ACTIVE/UNMET10x all8CPU,CPU/Metalall8,nobudget. CompletedcomponentPUSHED50cc0b8b (32.15%fastertile,NOTproof), allreceipts below. NewauthorizedSDKexperimentroot /tmp/zopro-deferred-dif32-sdk, experiment.json frozenBEFORESDKcode/test/timing. Candidatefromaccepted170+EXACTcomponent2fieldfiles; newnativeTwiddlesboolfalse(defaultabsent/0OFF), PlanconstructionreadsZOPRO_CPU_DEFERRED_DIF32=1ONonce toordinarycanonicalroots only. Empty/twistedTwiddlesfalse:twistedmultiplicationcanbelazy>=p. Nativecolumn_kernel onlyDIF!FORWARD dispatchnewfieldcallbacktoEXACTexistingcached/uncached31roots,nocopyrootarray;DIT/OFF/portable/genericbodyretained. Newcorefilesremoveadditionsrecoversoriginalbyteforbyte,173sourcepins inclnew4testmodule+2unchangedSDKdriverfiles. Component33field+6arithmeticoraclesreusedafterexactfieldSHA/flagsaudit; no newfieldformula. StandardenvflagparseatPlanconstructionmayallocateephemeralString, no newperproof/retainedheapbuffer. NativeworkflowcoreOFF+ONfull,raw/cubic,libraries+SDKlink;4newtestsactualcanonicalrawroots andcached/uncachedcolumnsquadreference (assertcachedpathruns),fullpackedFFTvslegacyquad4096/8192/32768/262144,mode/empty/twistedpolicy,nativelayout and4concurrentcallers512KiBstacks. Pending, inspectexactexec.\n\nFrozenfutureSDKprotocol7blocks9shapes32workers31rounds+primary16physicalCPUs0-15 101rounds,3primes,3variantsaccepted03e126/OFF/ON=>8610latencySDK. AlllatencyTHEN9profiles(primary101/12x251/merge549)510SDK. All9OFF/ON4workerpreflight18GNARKexports72SDKbeforelatency. Sameoldgatesprimaryandoriginal8positivebothcontrols4/7wins,allcases2%paired/5%block,OFFcalfactor1.03,nogainfloor/reroll/exclusion. Mustadduncached/prepdeniedbeforepromotion. FullproofCPU/Metalall8contextgoalstillactive,noproductionadoptionyet. Commit/PUSH AFTERcompletedintegration beforeanother. No newAWS. Work-state.json/currentstatuspendingnative;latestproduction170/SDK03e126unchanged. OlderCURRENTheadersbelowHISTORICALsupersededbythisbyte0section.\n\n";
    fs::write(h, heading.to_owned() + &old).unwrap();
    println!(
        "FrozenSDK173sources{}totalpins; four focused integration tests plus full native suite",
        pins.len()
    );
}
