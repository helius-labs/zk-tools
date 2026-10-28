use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-g2-callers";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-current174-g2-callers";
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
    let mut paths = vec![];
    walk(d, &mut paths);
    paths.sort();
    let mut pins = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if ["SHA256SUMS", "artifact-index.json"].contains(&rel.as_str()) {
            continue;
        }
        pins.insert(rel, hash(p));
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
    println!("Sealed{}hashes", pins.len());
}
fn archive() {
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let a = read(r.join("analysis.json"));
    assert_eq!(a["independent_proof_count"], 15059);
    let mut report=String::from("# G2 preparation before raw MSM\n\nThe G2 caller starts within about four microseconds of the shared join. Its grouping attempt accounts for most of the previously observed delay before raw MSM: **1.32–1.62 ms at32workers** across three process medians. No grouped-point reduction runs before raw MSM in these requests; the caller takes the retained ungrouped path. This localizes the interval to optional grouping preparation rather than late task dispatch. It does not establish a specific hash/allocator/syscall cause or a speedup from removing it.\n\nThe same adopted174-source SDK81033 runs without rebuilding. Three fixed alternating blocks use primary16/32,3primes+31requests each. Coarse probes record every entry/return; cycles sample at199Hz inside acknowledged warm-loop FIFO gates. Timings are instrumented diagnostics, not a replacement for accepted8.366931ms.16-worker variation remains visible. No source, dependency, global kernel policy or clock setting changes; all16owned probes are removed.\n\n| Process | SDK ms | Join→G2 caller ms | Caller→raw ms | Grouping ms | Admission ms (inside grouping) | Headroom ms (inside admission) | Raw ms |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for (name, t) in a["traces"].as_object().unwrap() {
        let m = &t["medians"];
        writeln!(
            report,
            "| {name} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |",
            m["SDK_ms"].as_f64().unwrap(),
            m["join_to_query_ms"].as_f64().unwrap(),
            m["query_before_raw_ms"].as_f64().unwrap(),
            m["grouping_ms"].as_f64().unwrap(),
            m["admission_ms"].as_f64().unwrap(),
            m["headroom_ms"].as_f64().unwrap(),
            m["raw_ms"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\nMemory admission accounts for about0.276–0.284ms at32workers; headroom about0.117–0.144ms. These nested values must not be added to grouping. Shared mutex waiting, user execution and kernel work are not separated by elapsed spans. Thread-matched cycle samples inside grouping are sparse (zero or one per process), so they do not provide a calibrated CPU-time estimate or exclusive cause. Grouping returns without a grouped point array; this tracer does not expose its return reason and therefore does not prove a full hash-map build or cardinality rejection occurred.\n\nAll204SDK calls verify internally;6fresh164-byte exports independently pass Gnark, fixture public hashes and uniqueness beyond15053prior identities. Ledger15059. Six recordings contain9210cyclesamples, onePID each, no lost records, and validated16traceperiod1+cyclesfreq199+metadata-dummy attributes. Existing4parser tests plus2caller tests passed before capture; an added seventh test qualifies a post-capture scope correction. Raw MSM itself invokes the same point-grouping symbol for bucket reduction, so caller preparation now includes only completed calls before raw entry on the caller TID. The original analyzer failure/source remain; recordings, verification and successful capture helpers were not repeated or changed. All original pins pass.\n\nNext: compare the existing ZOPRO_CPU_DEDUP=1 and0 settings on this exact SDK in a fixed unprofiled protocol. This changes optional grouping across all participating queries and also its workspace forecast; it cannot isolate only B2 or justify a global default change from a primary result. Retain existing B-first, grouping-prefix, bitmap, borrowed-map and parallel-grouping negative results. No new optimization is adopted here.\n\nInherited all-eight results from the accepted source follow, compared with supplied historical M5 references. This study does not remeasure other shapes or AWS/Aeglos. No new mobile/ARM/TEE, giant, minimum-RAM, swap or pressure qualification. The original10×CPU goal remains active and unmet0/8.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    for (shape, v) in a["inherited_M5"].as_object().unwrap() {
        writeln!(
            report,
            "| {shape} | {:.6} | {} | {:.3}× | {} | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"],
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"],
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\n[Protocol](artifacts/design.md), [every request and result](artifacts/analysis.json), [captured observer](artifacts/run.rs), [scope-corrected analyzer](artifacts/analyze.rs), [source pins](artifacts/pins.json), [probes](artifacts/probes.json), [cleanup](artifacts/cleanup.json). Raw perf data stay local with hashes.\n");
    let mut paths = vec![];
    walk(r, &mut paths);
    paths.sort();
    let mut local = BTreeMap::new();
    for from in paths {
        let rel = from.strip_prefix(r).unwrap();
        let ext = from.extension().and_then(|x| x.to_str()).unwrap_or("");
        if ext == "data" {
            let o = Command::new("sudo")
                .args(["-n", "sha256sum"])
                .arg(&from)
                .output()
                .unwrap();
            assert!(o.status.success());
            local.insert(rel.display().to_string(),json!({"path":from,"sha256":String::from_utf8(o.stdout).unwrap().split_whitespace().next().unwrap(),"bytes":fs::metadata(&from).unwrap().len()}));
            continue;
        }
        if !["rs", "md", "json", "txt", "log", "proof", "public"].contains(&ext) {
            continue;
        }
        let to = d.join("artifacts").join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        assert_eq!(hash(&from), hash(&to));
    }
    fs::write(d.join("README.md"), report).unwrap();
    save(d.join("artifacts/raw-local-pins.json"), &json!(local));
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/current174-stages/run.rs",
        d.join("artifacts/parent-parser.rs"),
    )
    .unwrap();
    seal();
}
fn state() {
    let r = Path::new(R);
    let o = Command::new("git")
        .args(["-C", "/home/ubuntu/src/zk-tools", "rev-parse", "HEAD"])
        .output()
        .unwrap();
    let commit = String::from_utf8(o.stdout).unwrap().trim().to_string();
    assert_eq!(
        fs::read_to_string(r.join("remote.log"))
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap(),
        commit
    );
    let path = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(path);
    s["status"] =
        json!("G2 caller diagnostic complete/pushed; optional grouping accounts for delay");
    s["root"] = json!(R);
    s["report"] = json!(D);
    s["commit"] = json!(commit);
    s["remote_verified"] = json!(true);
    s["pushed"] = json!(true);
    s["owned_live_jobs"] = json!([]);
    s["owned_probes"] = json!([]);
    s["new_independent_exports"] = json!(6);
    s["independent_proof_count"] = json!(15059);
    s["SDK_requests"] = json!(204);
    s["current174_g2_callers"] = read(r.join("analysis.json"));
    s["next_action"]=json!("Fixed same-inode unprofiled DEDUP1/0 comparison at primary16/32 plus16x2/54; no global adoption from primary; retain original10x scope and closed grouping patches");
    s["goal_status"] = json!("active");
    s["goal_achieved"] = json!(false);
    s["original8_CPU10x_rows"] = json!(0);
    save(path, &s);
    let h=format!("CURRENT COMPLETE/PUSHED — G2 CALLER BOUNDARIES\n\nCommit/remoteverified{commit};root{R};report{D}. Sameaccepted174/SDK81033/8.366931msunchanged.204SDK/6freshGnark/12members;ledger15053->15059;6recordings9210cycles/onePID/noLost/16traceperiod1+cyclesfreq199+dummy validated. All16ownedprobesremoved;noownedjobs/cloud. Workflow70697terminal/reaped0. Capturedobserver6tests,scope-correctedanalyzer7tests; no nativeprover/dependencybuild.\n\nNEWANSWER: G2callerstarts.003607/.003827/.003957msafterjoin(primary32);grouping1.319249/1.430398/1.622912ms;callerbeforeRaw1.323536/1.434436/1.627229ms. Admission.281822/.275971/.283585ms subsets,headroom.116670/.117141/.143831 subsets. GroupedpointpreparationZERO(actualcallerNonegroupedpath); groupreturnREASONnotinstrumented,doNOTclaimfullhashmapbuild/rejection. G2entryNOTdelayedbyjobdispatch. Threadmatchedgroupingcycles0or1sample/process,sparse—notCPUseconds orproofhashcost. Physical16grouping.773158/1.087631/.801471ms;instrumentedSDK15.030602/15.118388/11.265920ms retained;noexternalpreemptionclaim.\n\nANALYSISREPAIR(no recapture): group_affineG2alsoinvokedinsideRawbucketreduction, sometimesonqueryTID. Initialrun-helperfinishpanickedpoints<=1afterfirstrow; allcaptures+Gnark+cleanupalreadycomplete. Capturedrun.rs/run-helper/pins unchanged. Newanalyze.rs copiesobserverandrestrictsgrouping/points tocompletedBEFORErawentry, addsregressiontestinsideRawpointsignored. Analyze-onlybuild+7testsaftercapture,analyze finishpassesoriginalpins/all6proofs. No successfulrecord/proofverificationrepeated;raw/logshistorypreserved.\n\nNEXT actualtest: existingZOPRO_CPU_DEDUP=1vs0 sameSDKinode/unprofiled fixedbaseline/config; thisbypassesoptionalgroupingonALLqueries andchangesworkspaceforecast. Needprimary16/32,16x2/54 safetycontextbeforethinkingcandidate; sourceunchanged/configonly. No defaultglobaladoptionfromprimaryresult. All8originalCPU10xACTIVEUNMET0/8; inheritedCPU/Metal table/report. No newAWS/Aeglos/mobile/ARM/TEE/giant/minRAM/swap/pressureclaim. KeepoldB-first/bitmaplowerbound/exactprefixcutoff/parallelgroups/borrowed-mapnegativeCLOSED; newmeasurementdoesnotrevivethempatches. CommitANDpusheachexperiment,politefactualRust/xtask,noquestions/subagents/hugetables/Casm.\n\n");
    let p = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(p).unwrap();
    fs::write(p, format!("{h}{old}")).unwrap();
    println!("Updatedcallerstate;goalACTIVE0/8");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        Some("state") => state(),
        _ => panic!("archive|seal|state"),
    }
}
