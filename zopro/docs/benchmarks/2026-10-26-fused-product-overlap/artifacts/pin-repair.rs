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
    let old: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-input-pins.json"))).unwrap();
    let mut pins = BTreeMap::new();
    for (p, h) in old {
        assert_eq!(hash(&p), h);
        let p2 = p.replace("/candidate/", "/candidate-v2/");
        pins.insert(p2.clone(), hash(p2));
    }
    for n in [
        "repair.rs",
        "repair.json",
        "pin-repair.rs",
        "prepare-v2.json",
        "implement-v2.rs",
        "overlap-block-v2.rs.txt",
        "source-audit-v2.json",
        "native/result.json",
        "native/core-off/tests.json",
        "native/core-on/tests.json",
        "native/candidate/artifacts.json",
        "preflight/on-w4-1x2-prove/exit.json",
        "preflight/on-w4-1x2-prove/stderr.log",
    ] {
        let p = r.join(n);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    assert_eq!(
        pins.keys().filter(|p| p.contains("/candidate-v2/")).count(),
        172
    );
    fs::write(
        r.join("build-input-pins-v2.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let p = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(p).unwrap();
    let head="# CURRENT — 2026-10-26: FUSED PRODUCT OVERLAP V2 BUILDING; PREFLIGHT CAUGHT ROW-COUNT GUARD ERROR\n\nGoal active/unmet all8CPU10x, no budget. Newexperimentroot/tmp/zopro-fused-product-overlap, lastpushedde784292. Initialnativeexec33233COMPLETE110coreOFF+110ON/raw2/cubic6+SDK. Initialpreflightexec83787TERMINALFAILED newONprimary assertionwronglyexpectedpaddeddomain32768 butEulerreturn30922actualrows. NineOFFshapes36SDK9Gnarkexports18members PASS before failure; ONfirstprimepanickedbeforeSDKcompletion/export; ZEROlatencytimingsofar. Alloriginalsource/binary/receipts retained.\n\nRepair sourcecandidate-v2: restrictnewbranchqueries.euler, product_count=max(a.len,b.len) beforemovingvectors; existingfusedeligibility/queryslice/assertionuseactualcount. Exactsameproduct algorithm, ordering,H-firstschedule,buckets,allgates.3fileadditionslib/alloc/alloclinux;167acceptedfiles inclfield/forecastunchanged. Newnative-plan-v2 outputnative-v2 target-native-v2 rebuildandfull110coreOFF+ON/raw/cubictestsrequiredforchangedsource. Oldnative/candidateSDK7948f75...notnewv2. Alloldbuild180pinshashescheckedbeforev2pins. Currentnewnativehandlefromnexttool; do notinferlivenessfromstateJSON. V2preflightmustincludeold9proofsinfreshnessprior4321->4330, then18new->4348beforeperf. Intendedcompletecounts9192+36=9228SDK,237+9=246Gnarkexports492membersplus2fixedfixtures; partialfailedproofnotcounted. Fulltimingstillfixed7blocks3variantsall9w32+physical16,8610latencySDKthen9profiles510SDK. TracesONLYpreflightanddisabledfortiming/profiles. NoAWS/adoption. COMMIT/PUSH wholecompletedexperimentincludingv1failurebeforeanother.\n\nOlderheadershistoricalsuperseded.\n\n";
    fs::write(p, head.to_owned() + &old).unwrap();
    println!(
        "V2{}pins; no timingbefore sourceguardrepair; fullnative rebuilding",
        pins.len()
    );
    let _ = json!({});
}
