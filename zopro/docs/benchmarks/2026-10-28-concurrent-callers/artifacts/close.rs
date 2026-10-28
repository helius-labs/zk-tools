use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/concurrent-callers";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-concurrent-callers";
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
fn copy(root: &Path, dir: &Path, out: &Path, compressed: &mut BTreeMap<String, Value>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        let t = e.file_type().unwrap();
        if t.is_dir() {
            if ["source", "target", "profiles", "frozen"].contains(&n.as_str()) {
                continue;
            }
            copy(root, &p, out, compressed);
        } else if t.is_file()
            && ["rs", "md", "json", "jsonl", "log", "txt"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let dest = out.join(p.strip_prefix(root).unwrap());
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            if n == "stdout.log" && p.to_string_lossy().contains("/loss-") {
                let dest = dest.with_extension("log.gz");
                let output = fs::File::create(&dest).unwrap();
                assert!(Command::new("gzip")
                    .args(["-n", "-c"])
                    .arg(&p)
                    .stdout(output)
                    .status()
                    .unwrap()
                    .success());
                compressed.insert(p.display().to_string(),json!({"original_sha256":hash(&p),"original_bytes":fs::metadata(&p).unwrap().len(),"archive_relative":dest.strip_prefix(out).unwrap(),"archive_sha256":hash(&dest)}));
            } else {
                fs::copy(p, dest).unwrap();
            }
        }
    }
}
fn inventory(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            inventory(root, &p, out)
        } else {
            let rel = p.strip_prefix(root).unwrap().display().to_string();
            if rel != "SHA256SUMS" {
                out.insert(rel, hash(p));
            }
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let root = Path::new(R);
    let a = read(root.join("analysis.json"));
    assert_eq!(a["captures"], 687);
    assert_eq!(a["timed_kernel_calls"], 110518272);
    assert_eq!(
        read(root.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        699
    );
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let mut report=String::from("# Concurrent point-caller diagnostic\n\n**The weighted-anchor benefit reverses under the tested SMT loads.** Borrowed signed G1 is about 1.8–2.3% faster alone or on two separate cores, but 1.5% slower on an SMT sibling pair and 1.35% slower across all 32 logical CPUs. This is evidence that a single-caller arithmetic win can fail under concurrent execution. It is not a new proof speedup, a complete explanation of the earlier SDK result, or an explanation of C7a's scaling. The weighted SDK candidate remains closed and unadopted.\n\nThe diagnostic links the exact frozen core/field/adapter libraries from the [parent caller study](../2026-10-28-g1-weighted-anchor/README.md). No Cargo/core/field build or arithmetic change occurs. The new Rust harness constructs private fixtures and scratch per OS thread and pins workers to explicit CPUs. Mode 0 and mode 1 both run ordinary arithmetic; mode 2 selects the same weighted formula tested previously. G2 is unchanged in all modes.\n\n## Unprofiled measurements\n\nThe verified CPU layouts are solo [2], SMT siblings [2,18], separate cores [2,3], 16 physical cores [0–15], and all 32 logical CPUs [0–31]. Sysfs core/sibling records and exact worker affinity checks are archived. Fifteen rotated/reversed blocks × fifteen caller/layout contexts × three modes give 675 fresh processes. Each worker primes three checked/reset 1,024-call batches, then runs 8,192 complete kernel calls.\n\nThe main clock spans release of the start barrier through completion of the slowest worker at the done barrier. The primary metric is ns per parallel wave, not average per-worker latency. Per-worker loop clocks, thread CPU clocks, group throughput and process usage are retained. Setup, thread creation, output checks and serialization are outside all clocks; barriers prevent any worker's post-check allocation from overlapping another worker's loop.\n\n| Caller | CPU layout | Ordinary 0 µs/wave | Weighted µs/wave | Weighted/ordinary 0 paired median | Marginal bounds | Wins |\n|---|---|---:|---:|---:|---|---:|\n");
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-prefix"] {
        for topology in ["solo", "smt", "separate", "physical16", "logical32"] {
            let p = &a["pairs"][format!("{kind}/{topology}/2-over-0")];
            writeln!(
                report,
                "| {kind} | {topology} | {:.6} | {:.6} | {:.9} | [{:.9}, {:.9}] | {}/15 |",
                a["cells"][format!("{kind}/{topology}/m0")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                a["cells"][format!("{kind}/{topology}/m2")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                p["median"].as_f64().unwrap(),
                p["low"].as_f64().unwrap(),
                p["high"].as_f64().unwrap(),
                p["wins"]
            )
            .unwrap();
        }
    }
    report.push_str("\n| Caller/layout | Weighted/ordinary 1 paired median | Marginal bounds | Wins |\n|---|---:|---|---:|\n");
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-prefix"] {
        for topology in ["solo", "smt", "separate", "physical16", "logical32"] {
            let p = &a["pairs"][format!("{kind}/{topology}/2-over-1")];
            writeln!(
                report,
                "| {kind}/{topology} | {:.9} | [{:.9}, {:.9}] | {}/15 |",
                p["median"].as_f64().unwrap(),
                p["low"].as_f64().unwrap(),
                p["high"].as_f64().unwrap(),
                p["wins"]
            )
            .unwrap();
        }
    }
    report.push_str("\nBounds are ordered ranks 4–12 of 15 within-context paired ratios, nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. Tables show medians of process means, so ratios of table medians can differ from paired medians. All fifteen ordinary-label calibration bounds fit the predeclared factor 1.03, but this does not mean zero measurement bias: unchanged G2 solo mode 2/ordinary 1 has a roughly 0.59% lower estimate with bounds below parity. That observation remains included and is not a G2 speedup. There are no exclusions, sample extensions, successful-capture repeats, topology selections or adoption gates.\n\nBorrowed G1 uses 352 pairs; owned G1 uses 200. Borrowing, signedness and geometry differ together and are not individually isolated. The borrowed benefit reverses on the sibling pair against both controls; both corresponding intervals lie above parity. It remains positive on separate cores and at physical16. Owned G1 at logical32 is about 2.5% slower with either control, while the solo improvement remains uncertain. Small positive estimates elsewhere remain reported.\n\nSMT still improves aggregate throughput in these loops; a slower per-thread loop is not evidence that disabling SMT would improve the prover. Worker count also changes frequency, cache working sets, barrier/tail behavior and contention. This diagnostic does not identify an exclusive execution port, front-end bottleneck, cache level, DRAM bandwidth limit or complete SDK critical path. C7a has no SMT in the earlier measurements, so no AWS explanation follows from this local interaction.\n\n## Counters and profiles\n\nAll 675 latency captures finish before eight FIFO-gated grouped perf-stat captures, followed by four separate 199 Hz cycles:u profiles. Counter/profile calls use 262,144 iterations per worker after the same primes. All twelve finish before decoding, hashing, compression or archive work. No owned helper/build/hash/archive/other benchmark overlaps capture. Host snapshots are retained; no exclusive-host claim is made.\n\n| Borrowed G1 layout/mode | Cycles/kernel call | Instructions/kernel call | IPC | Generic cache misses/call |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["stat"].as_object().unwrap() {
        for e in v["events"].as_object().unwrap().values() {
            assert_eq!(e["running_percent"], 100.);
        }
        writeln!(
            report,
            "| {k} | {:.3} | {:.3} | {:.6} | {:.6} |",
            v["events"]["cycles:u"]["per_kernel_call"].as_f64().unwrap(),
            v["events"]["instructions:u"]["per_kernel_call"]
                .as_f64()
                .unwrap(),
            v["IPC"].as_f64().unwrap(),
            v["events"]["cache-misses:u"]["per_kernel_call"]
                .as_f64()
                .unwrap()
        )
        .unwrap();
    }
    report.push_str("\nAll four counters run at 100% in every group. Counts are summed across active threads and divided by total kernel calls. Cycles on two SMT threads are logical-thread cycle counts, not disjoint physical-core cycles. These are one counter process per context, descriptive observations without confidence bounds. Weighted retires about 2.74% more instructions in every layout. Its measured cycles fall about 2.4% alone/physical16 but rise about 1.7% on the sibling pair and 1.0% at logical32. IPC alone is insufficient: it rises in both improving and regressing cases. This fits a latency/throughput tradeoff but does not prove a specific hardware cause. Generic cache-miss observations vary substantially at high concurrency, are not assigned to a cache level, and are not DRAM byte counts.\n\n| Profile | Samples | Sampled worker TIDs | Sampled total TIDs | PIDs | Lost |\n|---|---:|---:|---:|---:|---:|\n");
    let mut samples = 0u64;
    for (k, v) in a["profiles"].as_object().unwrap() {
        samples += v["samples"].as_u64().unwrap();
        writeln!(
            report,
            "| {k} | {} | {} | {} | {} | 0 |",
            v["samples"],
            v["sampled_worker_tids"].as_array().unwrap().len(),
            v["sampled_tids"].as_array().unwrap().len(),
            v["pids"].as_array().unwrap().len()
        )
        .unwrap();
    }
    writeln!(report,"\nTotal samples: {samples}. Every profile records all expected active worker TIDs plus the main control thread, one PID and zero lost records. Full symbol weights and decoded logs are archived; raw perf.data remains local with hashes. Loss-record dumps are archived losslessly with deterministic gzip. Sampled period is not a removable proof wall fraction.\n").unwrap();
    report.push_str("## Functional and memory checks\n\nOne new unfiltered harness test exercises three callers × three modes × five layouts using short loops, independent full generator oracles, exact CPU affinity, barrier coordination, scratch reuse and zero timed allocations. Two inherited perf-helper tests also pass. The exact parent native 124-test core qualification is inherited through frozen libraries; no successful native core/field build or test is repeated.\n\nAll 687 latency/stat/profile processes pass full generator output checks and matching per-round output digests, with zero global timed allocations. They complete **110,518,272 timed kernel calls**, which are not SDK/Gnark proofs. Per-worker scratch capacities remain 40,000 bytes for owned G1, 70,400 for borrowed G1 and 28,160 for G2. The borrowed fixture reference vector is 5,632 bytes per worker, outside clocks. Multiplied scratch for 32 workers, harness barriers, fixtures and stacks are diagnostic allocations; these observations are not a production memory policy, proof RSS or RAM minimum. All production source and accepted executable hashes remain unchanged.\n\n## Proof context and inherited comparisons\n\nThe preceding [full-SDK trial](../2026-10-28-g1-weighted-anchor-sdk/README.md) remains closed: accepted/OFF/ON primary32 medians 8.365439/8.361170/8.378203 ms, paired ON/OFF 1.003326 with bounds [0.996369,1.008874]. This diagnostic neither replaces those measurements nor rescues the candidate. It supplies a concrete reason to include concurrent callers when judging future arithmetic changes. It does not prove that SMT alone caused the SDK outcome.\n\nThe all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source timings and supplied M5 references are inherited, not remeasured here. The accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let v = &m["m5"][shape];
        writeln!(
            report,
            "| {shape} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\nHistorical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof was generated; independent ledger remains 13,082. No new all-eight/54, portable/mobile/TEE, pressure, giant-circuit or AWS/Aeglos measurement is claimed. No cloud resource was created.\n\nNext: judge future arithmetic candidates by actual work/instruction reduction and concurrent behavior as well as isolated latency. Audit previous arithmetic/reduction experiments before choosing a distinct operation-removal mechanism. Keep this candidate closed; no thread-count default, padding, code-layout or unchanged arithmetic reroll follows from the diagnostic. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [CPU topology](artifacts/topology.json), [library provenance](artifacts/library-provenance.json) and raw logs are included.\n");
    save(
        root.join("closeout.json"),
        &json!({"diagnostic_completed":true,"borrowed_G1_benefit_reverses_on_tested_SMT_loads":true,"exclusive_hardware_cause_proven":false,"fullSDK_cause_proven":false,"SDK_candidate_stays_closed":true,"arithmetic_rebuilt":false,"adopted":false,"production172_unchanged":true,"captures":687,"timed_kernel_calls":110518272,"native_harness_tests":1,"inherited_perf_helper_tests_executed":2,"all_counters_100_percent_running":true,"profiles":4,"profile_samples":samples,"timed_allocations":0,"final_pins_verified":pins.as_object().unwrap().len(),"new_Gnark_proofs":0,"proof_ledger":13082}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    let mut compressed = BTreeMap::new();
    copy(root, root, &Path::new(D).join("artifacts"), &mut compressed);
    save(
        Path::new(D).join("artifacts/compressed-loss-logs.json"),
        &json!(compressed),
    );
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    let mut files = BTreeMap::new();
    inventory(Path::new(D), Path::new(D), &mut files);
    fs::write(
        Path::new(D).join("SHA256SUMS"),
        files
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    let o = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    fs::write(root.join("archive-verification.log"), o.stdout).unwrap();
    assert!(o.status.success());
    for v in compressed.values() {
        let o = Command::new("gzip")
            .args(["-d", "-c"])
            .arg(
                Path::new(D)
                    .join("artifacts")
                    .join(v["archive_relative"].as_str().unwrap()),
            )
            .output()
            .unwrap();
        assert!(o.status.success());
        assert_eq!(
            format!("{:x}", Sha256::digest(&o.stdout)),
            v["original_sha256"].as_str().unwrap()
        );
    }
    save(
        root.join("archive-summary.json"),
        &json!({"hashes":files.len(),"git_files":files.len()+1,"GNU_strict_passed":true,"loss_dumps_gzip_roundtrip_hashes_verified":4}),
    );
    println!(
        "Archived{}hashes/{}Gitfiles;4gziprawlossroundtrips;SMTinteractiondiagnosticnotadoption",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let summary = read(format!("{R}/archive-summary.json"));
    for(k,v)in[("status",json!("Concurrent-caller diagnostic complete; weighted G1 benefit reverses on tested SMT loads; prior SDK candidate stays closed; committed/pushed/verified")),("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("production_changed",json!(false)),("adopted",json!(false)),("last_goal_turn_classification",json!("PROGRESS")),("SDK_requests",json!(0)),("new_independent_exports",json!(0)),("independent_proof_count",json!(13082)),("artifact_hashes",summary["hashes"].clone()),("git_files_new_report",summary["git_files"].clone()),("component",read(format!("{R}/analysis.json"))),("concurrent_caller_diagnostic",read(format!("{R}/closeout.json"))),("previous_completed_experiment",previous),("next_action",json!("Audit arithmetic work reduction and earlier negative mechanisms; include concurrent actual-callers in future candidate screens, no weighted-anchor reroll")),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([]))]{s[k]=v;}
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — CONCURRENT POINT-CALLER DIAGNOSTIC\n\nHEAD/remote verified{commit};root{R};report{D}/README.md.{}strictverifiedhashes/{}Gitfiles;4rawlosslogsdeterministicgz roundtripSHAverified. LastgoalturnPROGRESS:newcontrolledconcurrencydiagnosticexplainswhyisolatedwinsareinsufficient,NOTproofspeed/adoptionorAWSexplanation. Accepted172/SDK14daunchanged,local1x2historical8.324853ms/RSS134.50MiB,goalACTIVEunmet0/8,proofledger13082unchanged,noownedjobs/cloud.\n\nSameEXACTfrozeng1-weighted-anchor/artifacts.jsoncommon-imagelibraries,NOcore/field/Cargobuild. NewRustharnessindependentfixtures/scratchperOSthread,exactaffinitysingletonvalidated,start/done/releasebarrierskeepALLpostchecks/allocationsoutsideALLworkerclocks. Fixedsolo[2],SMT[2,18],separate[2,3],physical16[0..15],logical32[0..31],sysfstopologyrecorded. ownedG1n200/borrowedsignedG1n352/G2prefix352;mode0/1bothordinary,2weightedG1,G2same. Native3testspass(1newharnessall3x3x5contexts+2perfhelper);parent124coretests inheritedunchanged.\n\nCapture51930TERMINALexit0;15blocks15contexts3modes675latency8192calls/workerafter3x1024primes;THEN8FIFOgroupstat262144calls/worker;THEN4FIFO199Hzcyclesphysical16/logical32m0/2;THEN12owner/decode/loss=699steps687captures. ALLgenerator/digest/capacitycheckszerotimedglobalallocs;110518272timedkernelcallsNOTproofs. Mainclockstarttodonebarrierprimarywallwave;allworkerloop/CPUtimesrecorded. Noownedhelper/build/hash/archiveoverlap;473finalpinsreverified. No nativefailures/repairs/retries.\n\nBorrowedG1ON/ordinary0 solo.981962735CI[.976852,.986184]15/15;SMT1.015249082[1.014502,1.018818]0/15;separate.976878898[.976255,.978921]15/15;physical16.983873907[.979999,.992784]12/15;logical321.013454423[1.004860,1.020333]1/15. BothordinarycomparisonsreversalonSMT. Ownedlogical321.025236396CI[1.020480,1.037088],0/15;solopositiveuncertain. All15ordinarycalibfactor3%PASSbutunchangedG2solo2/1 .99410168 CI[.991487,.998952]retainedNOTG2gain/nobiasclaim. G2othernearparity. StatG1borrowinst85033.99->87367.99(+2.74%)alltopos;cycles solo37848.91->36931.53,physical1637876.86->36986.74,SMT49205.18->50056.90,logical3248900.15->49396.07. IPCimprovesinbothwinsandlosses. GenericmissesvaryathighconcurrencyNOTcachelevel/DRAMcause. 4profiles51582samplesonePIDzeroLost,all16/32workertidssampled+main.\n\nMeasuredtopologyinteractionNOTspecificport/SMTsolecauseofSDKfailure. Counts/frequency/cache/barriers/tailsallvary,SMTstillhigheraggregatethroughput,NOrecommenddisableSMT. C7aphysical96/noSMT,nocloudinference. PreviousweightedSDKtriala0d72e62b remainsCLOSEDfailed8.378203ONvs8.361170OFFvs8.365439accepted;donotrescue/repeatunchanged. No newSDK/Gnark/all8/54/portable/resource/mobile/TEE/giantproofs;latestAWSstale17137261af77.\n\nNEXT:auditactualarithmeticoperationremovalagainstpastnegativevariants,includeconcurrentcallersinfuturecomponentjudgment. No concrete newalgorithmselectedyet;avoidnewlayout/worker/scheduler/annotation/square/inverseunchangedsweeps. Parent124directgraphmodulecanbereusedasDIAGNOSTICbutproductionnewcandidateMUSTcopyaccepted172notrejectedweightedsource. KeepFFTengineeringexceptionfalsefullperformancegates. CommitANDpush everycompletedexperimentincludingnegative,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Alloldheadershistorical.\n\n",summary["hashes"],summary["git_files"]);
    fs::write(hp, format!("{header}{old}")).unwrap();
    println!("State/handoffupdated;goalactive0/8;ledger13082;candidateclosed");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
