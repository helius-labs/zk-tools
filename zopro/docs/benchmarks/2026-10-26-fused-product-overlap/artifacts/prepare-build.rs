use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let old = read("/tmp/zopro-deferred-dif32-sdk/native-plan.json");
    let text = serde_json::to_string(&old)
        .unwrap()
        .replace(
            "/tmp/zopro-deferred-dif32-sdk",
            "/tmp/zopro-fused-product-overlap",
        )
        .replace("zopro_deferred_dif32", "zopro_fused_product_overlap")
        .replace(
            "ZOPRO_CPU_DEFERRED_DIF32",
            "ZOPRO_CPU_FUSED_PRODUCT_OVERLAP",
        );
    let mut plan: Value = serde_json::from_str(&text).unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["name"] == "core-off" || step["name"] == "core-on" {
            step["required_tests"] = json!([
                "alloc::linux::overlap_guard_observes_nested_reclamation",
                "msm_affine::fused_kh::tests::combined_queries_generator_oracle"
            ]);
            step["argv"]
                .as_array_mut()
                .unwrap()
                .retain(|v| v != "--nocapture");
        }
    }
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let mut pins = BTreeMap::<String, String>::new();
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
        let p = r.join("candidate").join(n);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    for name in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        let p = r.join("candidate").join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    assert_eq!(pins.len(), 172);
    let field = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let lib = field["libraries"]["zopro_bn254"].as_str().unwrap();
    assert_eq!(hash(lib), field["input_pins"][lib].as_str().unwrap());
    pins.insert(lib.into(), hash(lib));
    for name in [
        "experiment.json",
        "prepare.json",
        "overlap-block.rs.txt",
        "implement.rs",
        "source-audit.json",
        "prepare-build.rs",
    ] {
        let p = r.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let p = "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask";
    pins.insert(p.into(), hash(p));
    fs::write(
        r.join("build-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    plan["source_pins"] = json!(r.join("build-input-pins.json"));
    fs::write(
        r.join("native-plan.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    let state = json!({"status":"frozen native build pending","root":r,"accepted_unchanged":true,"source_pins":172,"build_pins":pins.len(),"parent_commit":"de7842923b2de7e9768e45ddad96ed50e269557e","goal_active":true,"goal_achieved":false,"live_job":"Inspect exact workflow handle; state file is not evidence of liveness."});
    fs::write(
        r.join("work-state.json"),
        serde_json::to_vec_pretty(&state).unwrap(),
    )
    .unwrap();
    let p = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(p).unwrap();
    let head="# CURRENT — 2026-10-26: FUSED K/H PRODUCT OVERLAP BUILDING; LAST PUSH de784292\n\nGoal ACTIVE/UNMET all8CPU10x/CPU+Metalall8, no budget. Previousphasedcomplexnegativepushedde784292,allownedjobsreaped. New DECLARED /tmp/zopro-fused-product-overlap/experiment.json BEFOREcode/timing; candidate172sourcepins fromaccepted170+unchangedSDKdriver/support.3corefileADDITIONS(lib.rs,alloc.rs,alloc/linux.rs),167acceptedfilesincludingALLfieldandworkspaceforecastunchanged. New optin cfgzopro_fused_product_overlap +processOnceLockZOPRO_CPU_FUSED_PRODUCT_OVERLAP=1(defaultabsent/0OFF). Onlycurrenteligibleinternalomit_brealKH; launchH-first product->fusedKH branch whileAandG2readyqueriesstartparallel. SafescopedRayonjoins, samearithmetic/buckets/widths/entry/threads/vectorinputs,no table/keycopy/heapbuffer/Keylayoutchanges. Runtimeenvparsedonceperprocessnotperproof; tracepreflightonly. ReclamationglobalGuardcounterdisablesoverlap atbranchstartusingexistingadvisorydecision. WorkspaceforecastalreadySUMSFFTfields+allMSMparent+Nleafscratch,sourceunchanged; no optimisticmaxsubstitution. Public/generic/large/mismatchsequential. Prior4queryoverlap marginalandwarmpressurefailurestillvalid; this3querycombinedKH/Euler/currentG2prefixpathnewexplicitfixedtrial,noparametersweep.\n\nNative workflow fullcoreOFF+ON,newguardnestedtest+existingcombinedknown-generatororacles,raw/cubic,libraries+SDKlink pending. Newfieldtestsnotclaimed:fieldsourceexactaccepted, acceptedreferencefieldartifactSHAaudited,candidateRLIBseparatelypinned. Fixedfuturepreflightall9OFF/ON4workers1+3=>72SDK18Gnark;7blocksall9w32x31+primaryphysical16x101+3primes3variants=>8610latencySDK,THEN9warmmonotonicprofiles(primary101/12x2-51/merge54-9)510SDK. Expectedcomplete9192SDK237exports474members(2fixedfixturesseparate). Samefixedgatesprimaryandorig8improveboth4/7wins,eachcase<=2%paired/5%block,OFFcalfactor<=1.03,no floor/exclusion/reroll. Ifpositive MUSTqualify public/generic/portable/admission-denied/oneworker/concurrent/coldpressure+warmlimittransitionsdefault/retainedbeforeadoption. NoAWS/newproductionchange. UserCOMMIT/PUSH EACHCOMPLETED experiment beforeanother. Inspectactualnativehandle, no fictivelivenessfromJSON.\n\nOldCURRENTsectionsbelowhistorical.\n\n";
    fs::write(p, head.to_owned() + &old).unwrap();
    println!(
        "Frozen172sources{}buildpins; coreOFF/ON+raw/cubic+SDK, no field arithmetic change",
        pins.len()
    );
}
