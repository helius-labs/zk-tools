use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-adoption";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
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
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn seal() {
    let d = Path::new(D);
    let r = Path::new(R);
    let m = read(d.join("artifacts/production-source-manifest.json"));
    let files = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 174);
    for (f, h) in files {
        assert_eq!(hash(Path::new(P).join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    save(
        d.join("artifacts/post-promotion-audit.json"),
        &json!({"production_files_checked":174,"all_match_measured_source":true,"SDK_hash_matches_manifest":true,"no_new_tests_benchmarks_or_exports":true,"proof_ledger":15028,"first_promotion_helper_compile":"macro recursion limit rejected JSON expansion before execution; added recursion_limit256; no source mutation or successful capture repeated","pre_repair_source":"promote-before-recursion-fix.rs","authored_whitespace_check":"required before commit","tone":"factual, no external maintainer criticism"}),
    );
    for f in [
        "close.rs",
        "promote-before-recursion-fix.rs",
        "promotion.log",
    ] {
        fs::copy(r.join(f), d.join("artifacts").join(f)).unwrap();
        assert_eq!(hash(r.join(f)), hash(d.join("artifacts").join(f)));
    }
    let mut all = vec![];
    walk(d, &mut all);
    all.sort();
    let mut index = BTreeMap::new();
    for f in all {
        let rel = f.strip_prefix(d).unwrap().display().to_string();
        if ["SHA256SUMS", "artifact-index.json"].contains(&rel.as_str()) {
            continue;
        }
        index.insert(rel, hash(f));
    }
    save(d.join("artifact-index.json"), &json!(index));
    index.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!(
        "Sealed{}archive hashes;174productionfiles/SDKverified",
        index.len()
    );
}
fn state() {
    let d = Path::new(D);
    let m = read(d.join("artifacts/production-source-manifest.json"));
    let a = read("/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/analysis.json");
    let o = Command::new("git")
        .args(["-C", P, "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(o.status.success());
    let commit = String::from_utf8(o.stdout).unwrap().trim().to_string();
    let remote = fs::read_to_string(Path::new(R).join("remote.log")).unwrap();
    assert_eq!(remote.split_whitespace().next().unwrap(), commit);
    let statuspath = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(statuspath);
    s["status"] =
        json!("clean borrowed G2 source adopted and pushed; performance exceptions retained");
    s["root"] = json!(R);
    s["report"] = json!(D);
    s["commit"] = json!(commit);
    s["pushed"] = json!(true);
    s["remote_verified"] = json!(true);
    s["adopted"] = json!(true);
    s["production_changed"] = json!(true);
    s["accepted_source_files"] = json!(174);
    s["accepted_source_manifest"] = json!(format!("{D}/artifacts/production-source-manifest.json"));
    s["accepted_local_SDK_path"] = m["native_binary"].clone();
    s["accepted_local_SDK_sha256"] = m["native_binary_sha256"].clone();
    s["last_qualified_local_1x2_ms"] = json!(8.366931);
    s["accepted_primary_RSS_MiB"] = json!(135.);
    s["latest_full_sdk_measurement"] = a.clone();
    s["independent_proof_count"] = json!(15028);
    s["new_independent_exports"] = json!(477);
    s["SDK_requests"] = json!(26208);
    s["original8_CPU10x_rows"] = json!(0);
    s["goal_status"] = json!("active");
    s["goal_achieved"] = json!(false);
    s["owned_live_jobs"] = json!([]);
    s["owned_cloud_resources"] = json!([]);
    s["last_goal_turn_classification"]=json!("progress: fixed final artifact477independentexports/all8comparison and exact174source adoption");
    s["g2_borrowed_adoption"] = read(d.join("artifacts/review.json"));
    s["next_action"]=json!("Find a materially different CPU reduction from current critical-path evidence; no unchanged empty-precheck, guard-batch, batch-transfer or arithmetic reroll. Preserve source174/SDK81033 and all8 CPU/Metal target, no new AWS claim.");
    save(statuspath, &s);
    let header=format!("CURRENT COMPLETE/PUSHED — CLEAN BORROWED G2 ADOPTED\n\nCommit/remoteverified{commit}; report{D}; acceptedmanifest{D}/artifacts/production-source-manifest.json. Exact174productionfiles=measuredcleansource,3originalchanges+2privateG2modules,169originalsexact. AcceptedSDK{} SHA{}. Native122core+2raw+6cubicpass; fieldSOURCEunchanged/coherentnativelibrariesrebuilt. Fixed477SDKcaptures+477Gnarkverification complete;26208internallyverifiedrequests/477fresh164Bexports/954members;ledger14551->15028. Session47937terminal/reaped0;finish,archive,publication77462,promotion96956terminal/reaped0. Noownedjobs/cloud.\n\nACTUALCLEANprimary32candidate8.366931 vsaccepted8.563320/freshcontrol8.526204; candidate/acceptedpaired.9786269598351882[.9724897117061744,.9812218078085982],15wins;candidate/control.9830914420157155[.9724010318213789,.987757656436995],15wins. PrimaryRSSmedian/max131.25/135.00MiB;requestedtraffic1.4MiBsavingOLDexperimentalnotnewRSSguarantee. Merge54candidate164.388184ms RSS1802.20/1885.21MiB. All8CPU+Metal tableinnewmanifest/report;CPU10xACTIVEUNMET0/8. NoAWS/Aeglosnewcomparison.\n\nEXPLICITENGINEERINGEXCEPTION, gatesREMAINFALSE: final16x2candidate/acceptedupper1.025268463655265>1.02, pairedmedian1.0000167976106455[.9837754299646532,1.025268463655265],7wins;allotherdefault32checks+allcalibrationpass. Descriptiveprimary16candidate11.031550 vsaccepted10.935407 paired1.00627754[.92677552,1.05957888],7wins;no16gainclaim. OriginalexperimentalTWO16boundsremainFALSE. Forcedhighbaseline300stimeout/candidateUNEXECUTED retained;noquota rerolls. Exactmodulefallback/portable11/99/8,smallstacks,concurrent,boundedcases inheritedscopeONLY. No phone/ARM/TEE/giant/minRAM/swap/universalpressureclaim. PreviousFFTexceptionsunchangedhistorical.\n\nARCHIVE fullproductionstudycommitf3b354828615? actualcommitf3b354828e253e8b332940661876c7fcc4e394df;6160hashesafterpublicationmetadata, captured4blankEOFfindingspreserved/authored0. Genericarchive omittedmanifests/locks/licenses/shaders; archive-fix supplementsALL174/172sourcefiles,reindexstrictverify. SourcepromotionhelperinitialJSONmacrorecursioncompilefailure repairedBEFOREexecution (256attribute), oldsourcepreserved;no test/capture repeated. Promotiononlycopiedexact5filesthen174hashchecked. DoNOT runoldfinal-helperfinish afterpromotion: oldcapturepins includeoldproduction172paths; capturedclosingauditpassedBEFOREintentionalmutation. Follownewacceptedmanifest forfuturebaseline.\n\nNEXT: realCPUreduction guidedbycurrentcode/criticalpath,notanotherunchangedclosedexperiment. Read-onlyauditfoundempty-victimprecheckALREADYnegative2026-10-22-universal-empty-steal-probe(local+C7a):epochsharelower,wakeup/systemCPUhigher,noprimarylatencygain. Guardreuse2026-10-25-batched-steal andbatchtransfer2026-10-27-batch-local-steal alsoCLOSED. Do notblindlyhoistepochguards. PriorfullSDKBorrowedgainretainedbutuseactualcleanartifacttimings. CommitANDpushaftereachexperiment; politefactualreports;Rust/xtask,nosubagentsunlessauthorized,nohugetables/Casm. Status-onlyprecedinggoalturnNO_PROGRESS;thisgoalturnPROGRESS.\n\n",m["native_binary"].as_str().unwrap(),m["native_binary_sha256"].as_str().unwrap());
    let path = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(path).unwrap();
    fs::write(path, format!("{header}{old}")).unwrap();
    println!("Updated current174source acceptance/handoff;goalACTIVE0/8");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("seal") => seal(),
        Some("state") => state(),
        _ => panic!("seal|state"),
    }
}
