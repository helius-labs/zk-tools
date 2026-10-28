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
const R: &str = "/home/ubuntu/src/zopro-perf-work/b2-group-only174";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-b2-group-only174";
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
        let q = e.path();
        if e.file_type().unwrap().is_dir() {
            if !["target-candidate", "frozen"].contains(&e.file_name().to_string_lossy().as_ref()) {
                walk(&q, out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(q)
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
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["independent_proof_count"], 15211);
    let mut text=String::from("# B2-only grouping bypass\n\nThe isolated bypass is **not adopted**. Primary32 is **1.285% slower than OFF**, with0/7wins and paired bounds[1.002332,1.025829]. It is1.934% slower than accepted. Primary OFF/accepted calibration passes, and both gain checks fail. The positive physical16 OFF comparison remains reported with wide uncertainty.\n\nThe private source changes only the grouping attempt inside g2_query_exclusive. GlobalDEDUP=1, workspace forecasts, G1/fusedKH, key/solver/field source and accepted native flags remain. This isolates the caller attempt from the [global grouping-switch negative](../2026-10-28-dedup-off174/README.md). No table, plan, unsafe code or production runtime option is added.173of174 original files remain exact; inverse patch recovers all174.\n\nBoth modes pass122unfiltered core and8prover tests. Coherent libraries are frozen with actual field/core flags/features and transitive proc macros. OFF/ON share one candidate executable path/inode; the accepted SDK is retained independently. The7fixed interleaved blocks cover primary16/32,16x2/32 and54/32,151primary/31other requests after3primes. Loading is excluded; solving, fresh proving, verification and serialization are included. No owned build/helper/hash/archive/verifier/profile/otherbench overlaps timing; no exclusive-host claim.\n\n| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\n|---|---|---:|---:|---:|---:|\n");
    for (k, v) in a["cells"].as_object().unwrap() {
        let (c, m) = k.split_once('/').unwrap();
        writeln!(
            text,
            "| {c} | {m} | {:.6} | {:.3} | {:.2} | {:.2} |",
            v["median_ms"].as_f64().unwrap(),
            v["CPU_ms"].as_f64().unwrap(),
            v["median_RSS_MiB"].as_f64().unwrap(),
            v["max_RSS_MiB"].as_f64().unwrap()
        )
        .unwrap();
    }
    text.push_str("\nHere accept is the retained SDK, offset is private OFF and bypass is private ON. RSS includes key load/primes and is not minimum RAM. Primary32 peak drops from136.00MiB OFF to88.25MiB ON, while CPU rises188.197→192.407ms/request. The lower peak does not override the latency regression.\n\n| Pair | Median ratio | Lower | Upper | Wins | Paired saving ms | Required check |\n|---|---:|---:|---:|---:|---:|---|\n");
    for (k, v) in a["pairs"].as_object().unwrap() {
        let check = a["checks"]
            .get(k)
            .map(|v| if v == true { "pass" } else { "fail" })
            .unwrap_or("descriptive");
        writeln!(
            text,
            "| {k} | {:.9} | {:.9} | {:.9} | {}/7 | {:.6} | {check} |",
            v["median"].as_f64().unwrap(),
            v["low"].as_f64().unwrap(),
            v["high"].as_f64().unwrap(),
            v["wins"],
            v["paired_saving_ms"].as_f64().unwrap()
        )
        .unwrap();
    }
    writeln!(text, "\nFailed checks: `{}`.\n", a["failed_checks"]).unwrap();
    text.push_str("Bounds are the ordered extrema of7, nominal98.4375% under independent exchangeable blocks without familywise guarantees. The prospective primary condition requires upper<1 and at least5/7wins against both controls plus OFF/accepted calibration withinfactor1.03. Larger ON comparisons require upper<=1.03, with calibration separately reported. Physical16 is descriptive. No gain floor, excluded observation, extension, successful-capture repeat or changed threshold. Further all-eight/clean/portable/resource qualification is not performed after this failed gate.\n\nThe earlier traced1.4ms grouping interval was elapsed time, not a removable CPU or wall-time budget. This negative holds the original forecast fixed and therefore rejects a simple explanation based solely on the previous global-switch forecast difference. The result does not isolate a cache, allocator, scheduling or preemption cause. See [interpretation](artifacts/interpretation.md).\n\nAll12preflights independently verify before84timing processes;84final exports independently verify afterward. Totals:7944complete internally verified SDK requests,96fresh164-byte Gnark exports,192proof/public members, ledger15115→15211. Original source/library/executable/input/helper pins match after capture. Source174/SDK81033 and accepted8.366931ms remain unchanged; the private selector is not promoted.\n\nNew reference comparisons cover only1x2 and16x2. M5figures are supplied historical references, not matched-machine measurements.\n\n| Cell | Variant | ms | M5 CPU speedup | M5 Metal speedup |\n|---|---|---:|---:|---:|\n");
    for (k, v) in a["new_reference_comparisons"].as_object().unwrap() {
        for (m, x) in v["variants"].as_object().unwrap() {
            writeln!(
                text,
                "| {k} | {m} | {:.6} | {:.3}× | {:.3}× |",
                x["ms"].as_f64().unwrap(),
                x["CPU_speedup"].as_f64().unwrap(),
                x["Metal_speedup"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    text.push_str("\nThe following complete accepted-source all-eight table is inherited. The original10×CPU goal remains active/unmet, accepted0/8. No54M5reference was supplied. No new AWS/Aeglos, mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["inherited_accepted_M5"].as_object().unwrap() {
        writeln!(
            text,
            "| {k} | {:.6} | {} | {:.3}× | {} | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"],
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"],
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    text.push_str("\n[Protocol](artifacts/design.md), [all observations](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native build](artifacts/native/result.json), [proof inventory](artifacts/proof-inventory.json), [closing source pins](artifacts/pins.json).\n");
    let mut paths = vec![];
    walk(r, &mut paths);
    paths.sort();
    for from in paths {
        let rel = from.strip_prefix(r).unwrap();
        let ext = from.extension().and_then(|x| x.to_str()).unwrap_or("");
        if !rel.starts_with("source")
            && !["rs", "md", "json", "jsonl", "log", "txt", "proof", "public"].contains(&ext)
        {
            continue;
        }
        let to = d.join("artifacts").join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        assert_eq!(hash(&from), hash(&to));
    }
    fs::write(d.join("README.md"), format!("{}\n", text.trim_end())).unwrap();
    let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json");
    let mut sourcepins = BTreeMap::new();
    for f in manifest["source_files_sha256"].as_object().unwrap().keys() {
        let from = r.join("source").join(f);
        let to = d.join("artifacts/source").join(f);
        assert_eq!(hash(&from), hash(&to));
        sourcepins.insert(f.clone(), hash(from));
    }
    assert_eq!(sourcepins.len(), 174);
    save(
        d.join("artifacts/complete-source-pins.json"),
        &json!(sourcepins),
    );
    seal();
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
    let a = read(r.join("analysis.json"));
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    s["status"] = json!("B2-only grouping bypass rejected/pushed; no speedgain despite lowerRSS");
    s["root"] = json!(R);
    s["report"] = json!(D);
    s["commit"] = json!(commit);
    s["pushed"] = json!(true);
    s["remote_verified"] = json!(true);
    s["owned_live_jobs"] = json!([]);
    s["owned_probes"] = json!([]);
    s["independent_proof_count"] = json!(15211);
    s["new_independent_exports"] = json!(96);
    s["SDK_requests"] = json!(7944);
    s["b2_group_only174"] = a;
    s["adopted"] = json!(false);
    s["production_changed"] = json!(false);
    s["goal_status"] = json!("active");
    s["goal_achieved"] = json!(false);
    s["original8_CPU10x_rows"] = json!(0);
    s["last_goal_turn_classification"]=json!("progress: isolated B2same-SDK OFF/ON negative withcalibration/96freshproofs rejects removable-span hypothesis");
    s["next_action"]=json!("ClosebothglobalandB2-onlygroupingbypass; auditcurrentwhole-pool homogeneousG1/G2phase possibility andoldhistory/Aeglos scheduling, thenprospectiveCPU/latency/profile testifdistinct. No arbitrarydelay/heapwarmup/newtable/memorycap; originalall8goal retained");
    save(p, &s);
    let h=format!("CURRENT COMPLETE/PUSHED — B2-ONLY BYPASS REJECTED\n\nCommit/remoteverified{commit};root{R};report{D}. Accepted174source/SDK81033/8.366931msunchanged. PrivateonefileguardonlyB2groups,173originalsexact/reverse174;GLOBALDEDUP1/workspaceforecasts/G1/fusedKH/key/solver/fieldSOURCE/acceptednativeflagsunchanged. Bothmodes122core+raw2/cubic6pass(244core16prover). Coherentfresh8libraries/procmacrosfrozen/core+fieldflagsfeaturesaudited;candidateSDK57472a9f3001cef0dc4ae913a4f789415e0c80308afbfb9c948184725d5ca8f3 sameOFF/ONpath/inode. No productionadoption.\n\nRESULT: primary32accepted8.105792/OFF8.123635/ON8.228011ms. ON/OFFpaired1.012848435[1.002331608,1.025829172],0/7wins,NEGATIVE104.376us;ON/accepted1.019335438[1.009802219,1.022932276],0wins,NEGATIVE156.495us. OFF/acceptedcalPASS1.003475706[.996229366,1.016833260]. GateFALSEtwofailedprimarygainchecks;otherprospectivechecksPASS. LargerONmedians16x240.485223vsOFF40.324146;54164.634203vsOFF162.279503. Physical16ON/OFF.981179737[.871928758,1.236685801],5winspositiveestimateuncertain;ON/accepted1.019932718,2wins. Preservepositivebutno16winclaim. No repeats/exclusions/extensions/relabeledgates.\n\nMEM/CPU: primary32ONmedian/maxRSS85.25/88.25MiBvsOFF130.25/136.00/accepted132.25/136.25,CPU192.407vs188.197ms;forecastEXACTunchangedso earlierglobalforecastconfoundnotsoleexplanation. LowerRSSnotminRAMoruniversalqual. Traced1.4msgroupingspanNOTremovabletime: globalbypassANDisolatedbypassbothSLOWER. Stopchasinggroupingbypass. Allocationhistory/residency/interleaving/cachepossible,NOTisolatedcause. DoNOTimitategroupingwithsyntheticwarmupallocorpretendcacheproven.\n\nVALIDATION: native66415terminal/reaped0;capture20631terminal/reaped0,all192steps:12preflightsindependentGnarkBEFORE84latencies,84Gnarkafter.7944SDK/96fresh164Bexports/192proofpublic;ledger15115->15211. Allclosing174source/SDK/library/helper/inputpins pass;all174privatesourcefilesarchivedincludingmanifests/locks/licenses/shaders,excludehugeRLIB/binariesbutretainlocalhashreceipts. No helper/build/hash/archive/profile/verifier/secondbench overlaps captures. No ownedjobs/cloud/probes. No newfield-suite/portable/concurrent/pressure/phone/ARM/TEE/giant/swap/minRAM/AWS/Aeglosqual. Inheritedall8CPU+Metal tableandfullgoalACTIVEUNMET0/8 retained;no newaccepted8.105792benchmarkclaim.\n\nNEXT: auditoldhomogeneouswhole-poolG1/G2phases/actualAeglosscheduling beforeselectingDISTINCTCPUwork-reduction experiment. ConsiderallG1queries thenB2 usingSAMEfullRayonpool (notfixedworkerpartitions,notB-firstpriorityswap,notproductoverlap,notarbitrarydelay); couldreduceinstructionsetmix/livebuffersbutmayloseparallelism/C7a96cores. Needsprospective matchedCPU/latency/groupedPMU/phaseprofiling,sourcefallbacktests andcalibration;no cachecause/speedpredictionclaim. AlternativelargefusedKHcurrentlyeligiblek+h<65536;olderlinearwitness/rankcensus rulesoutlargesimplerqueryfactorization. Avoidunchangedbitmap/prefix/parallelgroups/borrowed-map/globalbypass/B2-onlybypass retries. UserwantscleverCPUwork,noCasmrewrite/gianttables/syntheticRAMcaps. CommitANDpusheachexperiment,politefacts,noquestions/subagents.\n\n");
    let p = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(p).unwrap();
    fs::write(p, format!("{h}{old}")).unwrap();
    println!("Updatedstate/handoff;accepted174unchanged;goalACTIVE0/8");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        Some("state") => state(),
        _ => panic!("archive|seal|state"),
    }
}
