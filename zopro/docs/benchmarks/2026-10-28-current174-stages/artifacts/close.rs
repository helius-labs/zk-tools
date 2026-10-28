use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-stages";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-current174-stages";
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
fn root_hash(p: &Path) -> String {
    let o = Command::new("sudo")
        .args(["-n", "sha256sum"])
        .arg(p)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}
fn seal() {
    let d = Path::new(D);
    let mut files = vec![];
    walk(d, &mut files);
    files.sort();
    let mut pins = BTreeMap::new();
    for f in files {
        let rel = f.strip_prefix(d).unwrap().display().to_string();
        if ["artifact-index.json", "SHA256SUMS"].contains(&rel.as_str()) {
            continue;
        }
        pins.insert(rel, hash(f));
    }
    save(d.join("artifact-index.json"), &json!(pins));
    pins.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        pins.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Sealed{}archive hashes", pins.len());
}
fn archive() {
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    assert_eq!(
        read(r.join("analysis.json"))["independent_proof_count"],
        15053
    );
    assert_eq!(read(r.join("cleanup.json"))["owned_group_absent"], true);
    let mut paths = vec![];
    walk(r, &mut paths);
    paths.sort();
    let mut local = BTreeMap::new();
    let mut compressed = BTreeMap::new();
    for from in paths {
        let rel = from.strip_prefix(r).unwrap();
        let ext = from.extension().and_then(|x| x.to_str()).unwrap_or("");
        if ext == "data" || fs::metadata(&from).unwrap().permissions().readonly() {
            local.insert(rel.display().to_string(),json!({"sha256":root_hash(&from),"bytes":fs::metadata(&from).unwrap().len(),"path":from}));
            continue;
        }
        if !["rs", "md", "json", "log", "txt", "proof", "public"].contains(&ext) {
            continue;
        }
        let mut to = d.join("artifacts").join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        if fs::metadata(&from).unwrap().len() > 1024 * 1024 && (ext == "log" || ext == "txt") {
            let o = Command::new("gzip")
                .args(["-n", "-c"])
                .arg(&from)
                .output()
                .unwrap();
            assert!(o.status.success());
            to = PathBuf::from(format!("{}.gz", to.display()));
            fs::write(&to, o.stdout).unwrap();
            let back = Command::new("gzip")
                .args(["-d", "-c"])
                .arg(&to)
                .output()
                .unwrap();
            assert!(back.status.success());
            let h = hash(&from);
            assert_eq!(format!("{:x}", Sha256::digest(&back.stdout)), h);
            compressed.insert(rel.display().to_string(),json!({"original_sha256":h,"original_bytes":back.stdout.len(),"archive":to.strip_prefix(d).unwrap(),"compressed_sha256":hash(&to),"roundtrip_exact":true}));
        } else {
            fs::copy(&from, &to).unwrap();
            assert_eq!(hash(&from), hash(&to));
        }
    }
    save(d.join("artifacts/raw-local-pins.json"), &json!(local));
    save(
        d.join("artifacts/compressed-members.json"),
        &json!(compressed),
    );
    let mut report = fs::read_to_string(r.join("report.md")).unwrap();
    report = report
        .replace("Source174", "Source 174")
        .replace("SDK81033", "SDK 81033")
        .replace("measures31", "measures 31")
        .replace("plus199Hz", "plus 199 Hz")
        .replace("accepted8.366931ms", "accepted 8.366931 ms")
        .replace("all-eight10×", "all-eight 10×")
        .replace("unmet0/8", "unmet, 0/8")
        .replace("verifies25fresh164-byte", "verifies 25 fresh 164-byte")
        .replace("exports,50proof/public", "exports, 50 proof/public")
        .replace("ledger15028→15053", "ledger 15028→15053");
    report.push_str("\nThe raw-boundary follow-up in [interpretation](artifacts/interpretation.md) and [matched remainders](artifacts/remainders.json) records a1.55–1.71ms delay from join entry to the G2 raw routine, followed by only about10–16µs until join return. Caller start, preparation and scheduling are not yet separated. Earlier negative B-first/grouping/runtime experiments remain closed.\n");
    fs::write(d.join("README.md"), format!("{}\n", report.trim_end())).unwrap();
    seal();
    println!(
        "Archived diagnostic;{}local-only blobs,{}lossless compressed logs",
        local.len(),
        compressed.len()
    );
}
fn state() {
    let r = Path::new(R);
    let o = Command::new("git")
        .args(["-C", "/home/ubuntu/src/zk-tools", "rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(o.status.success());
    let commit = String::from_utf8(o.stdout).unwrap().trim().to_string();
    assert_eq!(
        fs::read_to_string(r.join("remote.log"))
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap(),
        commit
    );
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    s["status"] = json!("exact174 stage diagnostic complete and pushed; no new optimization");
    s["root"] = json!(R);
    s["report"] = json!(D);
    s["commit"] = json!(commit);
    s["pushed"] = json!(true);
    s["remote_verified"] = json!(true);
    s["owned_live_jobs"] = json!([]);
    s["owned_probes"] = json!([]);
    s["independent_proof_count"] = json!(15053);
    s["new_independent_exports"] = json!(25);
    s["SDK_requests"] = json!(820);
    s["production_changed"] = json!(false);
    s["goal_status"] = json!("active");
    s["goal_achieved"] = json!(false);
    s["original8_CPU10x_rows"] = json!(0);
    s["last_goal_turn_classification"]=json!("progress: exact174 ELF phase/cycle diagnostic changes next action;25fresh independent proofs");
    s["current174_stages"] = read(r.join("summary.json"));
    s["next_action"]=json!("Separate G2 query-start from caller grouping/admission before raw routine using exact-ELF coarse caller boundaries; do not revive failed B-first/empty-probe/guard-batch/grouping patches. Then test a distinct CPU reduction. Preserve accepted174 SDK81033,8.366931ms, all8 CPU/Metal objective and false old performance gates.");
    save(p, &s);
    let header=format!("CURRENT COMPLETE/PUSHED — EXACT174 ELF STAGE DIAGNOSTIC\n\nCommit/remoteverified{commit};root{R};report{D}. Source174/SDK81033 unchanged,accepted8.366931ms/manifest2026-10-28-g2-borrowed-adoption remains. Diagnostic25freshGnark/820SDK/ledger15028->15053/50members,112658cycles,13recordingsonePID/zeroLost/all14traceperiod1+cyclesfreq199+dummy metadata validated. All14ownedprobeszopro174stage removed;noownedjobs/cloud. Workflow16329terminal/reaped0,finishcomplete. No build/helper/hash/archive/otherbench overlaps captures. Fourparser tests pass; no prover/dependency rebuild/copy/source instrumentation.\n\nNEW WALL EVIDENCE: primary32 solver1.170829ms,gather/public/reclaim.193304,FFT.605934,postproduct5.560433,directjoin5.546757,pre_solver.090180,SDKoutsideinnercore.778088. Matchedperrequest SDKminusjoin blockmedians2.887880/2.946079/2.971778ms: evenhalvingMSMonly insufficient3.7target ifremainderunchanged; conditionaldiagnosticbudgetnotuniversalbound. G2raw starts1.714265/1.545426/1.612232ms AFTERjoinstart, returns.010179/.016170/.009748ms BEFOREjoinend. G2inclusive4ms doesNOTmeanoffcriticaltail! Mustdistinguishcallerstart/preparationfromscheduling. Physical16entrydelay.752080/.754043/.953689ms. Fullspansremainder.json. PrimarypostproducttopCPU G1stream23.57%,borrowedG2apply19.63%,Fqinverse7.29%,G2reduce7.07%,G1feed6.74%,G2feed6.48%,epoch6.09%,G1reduce4.79%;notremovablewallfractions.16x2 solver5.288/FFT10.222/post23.352;54solver20.461/FFT65.120/post72.857ms.\n\nON/OFF pairedmedians1.051619/1.058905/1.025218/1.016425 forprimary32/16,16x2,54. Physical16lastON14.142937 vsOFF9.612047 (1.471376) retained;noassumedpreemption/uprobecause. DiagnosticOFF8.151452notnewacceptedlatency. No newall8/AWS/Aeglos/mobile/ARM/TEE/giant/swap/minRAM/pressureclaim. Inheritedall8CPU+Metal tableandgoalACTIVEUNMET0/8.\n\nNEXT CONCRETE: coarseexactELF caller-boundary diagnostic for existingg2_query_exclusive VA0x457590,scalar_groups0x217790,group_affineG20x22b010 pluscurrentjoin/rawmarkers. ComputeoffsetsfromPT_LOAD/retainasm/declarefixedprotocol/proofs. Needknowwhether1.6mspreraw isstartdelayorcallerprep BEFOREchangingalgorithm. SimpleB-firstalreadynegative2026-10-22-universal-primary-b-first,exactgroupingprefixcutoff2026-10-27-grouping-prefix-cutoffalsofullfailedscreen;bitmaplowerbound,parallelgroups,borrowedgroupkeysnegativeclosed. Do notreviveunchangedempty-victimprecheck/guardbatch/batchtransfer. Real10xrequireCPUandprefixworkreduction,notonlysampledCPUcategory.\n\nSETUPHISTORYpreserved: tee-a lseekSEEK_ENDtracefsEINVAL,atomicpipefixineffective,directrestrictedrootwriterworks. Oneinstallmanualsuccessnotrepeated. perf6.8rejecttracepoint /period=1/ term; defaultperiod1actualvalidated. Onepreflight4SDK/1Gnark+3decodescompletedthenparserPID/TID/fieldorder/dummy-metadatarepair; reusedsameproof/data. Continuation86steps skips5successfulpreflightsteps,logs inpreflight-parser-failure/workflow;actualplanplan-parse-repaired.json/pins-parse-repaired.json;source174allclosingpins pass. Parserunit3old/4new appropriateafterformatfix. ContinuehelpercompilePathBufborrowfixbeforecaptures. Rawperf stayslocalhash;large logs gziproundtrip exact;archivehashes strictverified. CommitANDpush eachexperiment;politefactualRust/xtask,nosubagents/questions/hugetables/Casm.\n\n");
    let p = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(p).unwrap();
    fs::write(p, format!("{header}{old}")).unwrap();
    println!("State/handoffupdated;goalACTIVE0/8");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        Some("state") => state(),
        _ => panic!("archive|seal|state"),
    }
}
