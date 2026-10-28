use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-runtime-layout";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-runtime-layout";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn copy_tree(root: &Path, dir: &Path, out: &Path) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        let t = e.file_type().unwrap();
        if t.is_dir() {
            if ["profiles", "source", "target", "frozen"].contains(&n.as_str()) {
                continue;
            }
            copy_tree(root, &p, out);
        } else if t.is_file()
            && ["rs", "md", "json", "jsonl", "log", "txt"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let dest = out.join(p.strip_prefix(root).unwrap());
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(p, dest).unwrap();
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
    use std::os::unix::fs::MetadataExt;
    let root = Path::new(R);
    let a = read(root.join("analysis.json"));
    assert_eq!(a["captures"], 110);
    assert_eq!(a["latency_processes"], 98);
    assert_eq!(
        read(root.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        128
    );
    assert!(!Path::new(D).exists());
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let id = read(root.join("identity-audit.json"));
    let original = fs::metadata(root.join("probe")).unwrap();
    for alias in ["baseline-aa", "baseline-bb"] {
        let m = fs::metadata(root.join(alias)).unwrap();
        assert_eq!((m.dev(), m.ino()), (original.dev(), original.ino()));
        assert_eq!(
            hash(root.join(alias)),
            id["binary_sha256"].as_str().unwrap()
        );
    }
    let manifest=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (f, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let mut report=String::from("# G2 baseline runtime-layout diagnostic\n\n**The tested prefix-buffer placements do not reproduce or explain the previous 3× control gap.** Natural placement and fixed page offsets 0/2048 have nearly equal latency here. No prover change is adopted. The old packing candidate and its failed calibration remain closed; these diagnostic timings do not replace its observations.\n\nOne newly instrumented baseline executable runs through two equal-length hardlink aliases with identical bytes and inode. Field/core/prover arithmetic comes exclusively from the previous study's frozen accepted-arithmetic libraries. No field, core or production allocator source is rebuilt or changed. The Rust observation wrapper records allocation layout outside clocks and changes only backing for requests matching the small G2 prefix record size/alignment in the forced modes.\n\n## Unprofiled observations\n\nSeven fixed rotated/reversed blocks cover G2 at 200/352 pairs in three placement modes and a natural G1-owned/200 control. Two aliases give 98 fresh processes. Each uses CPU 2, one active caller, three 1,024-call primes with independent checks/reset, then 16,384 timed complete caller calls. Values below are medians of process means across both aliases, in µs per batch.\n\n| Caller | Pairs | Natural | Page offset 0 | Page offset 2048 |\n|---|---:|---:|---:|---:|\n");
    for size in [200, 352] {
        writeln!(
            report,
            "| G2 | {size} | {:.6} | {:.6} | {:.6} |",
            a["cells"][format!("g2/{size}/m0/pooled")]["median_ns_per_batch"]
                .as_f64()
                .unwrap()
                / 1000.,
            a["cells"][format!("g2/{size}/m1/pooled")]["median_ns_per_batch"]
                .as_f64()
                .unwrap()
                / 1000.,
            a["cells"][format!("g2/{size}/m2/pooled")]["median_ns_per_batch"]
                .as_f64()
                .unwrap()
                / 1000.
        )
        .unwrap();
    }
    writeln!(
        report,
        "| G1 owned | 200 | {:.6} | — | — |\n",
        a["cells"]["g1/200/m0/pooled"]["median_ns_per_batch"]
            .as_f64()
            .unwrap()
            / 1000.
    )
    .unwrap();
    report.push_str("| G2 pairs | Forced offset | Alias | Paired forced/natural median | Marginal min–max bounds |\n|---|---:|---|---:|---|\n");
    for size in [200, 352] {
        for mode in [1, 2] {
            for alias in ["a", "b"] {
                let i = &a["layout_pairs"][format!("g2/{size}/m{mode}/{alias}")];
                writeln!(
                    report,
                    "| {size} | {} | {alias} | {:.9} | [{:.9}, {:.9}] |",
                    if mode == 1 { 0 } else { 2048 },
                    i["median"].as_f64().unwrap(),
                    i["low"].as_f64().unwrap(),
                    i["high"].as_f64().unwrap()
                )
                .unwrap();
            }
        }
    }
    report.push_str("\nSmall positive estimates are retained, including about 0.14% lower 200-pair latency at offset 2048. Every placement interval includes parity. This establishes no reproducible placement speedup and supplies no basis for choosing a production padding rule. Bounds are the min/max of seven paired block ratios, nominal 98.4375% marginal coverage under independent exchangeable blocks, with no familywise guarantee. No gain gate, sample exclusion, extension or parameter sweep was used.\n\n| Caller/context | Alias b/a median | Marginal min–max bounds |\n|---|---:|---|\n");
    for (k, v) in a["alias_pairs"].as_object().unwrap() {
        writeln!(
            report,
            "| {k} | {:.9} | [{:.9}, {:.9}] |",
            v["median"].as_f64().unwrap(),
            v["low"].as_f64().unwrap(),
            v["high"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\nG2 alias results stay close here. The G1 negative control retains a wider roughly −4.3% to +4.2% paired range, so this is not a claim of universally stable measurement. The changed harness, longer priming, common executable and controlled placements distinguish this study from the earlier failed control comparison. The original anomaly's cause remains unresolved; it is not assigned to prefix placement, compiler behavior, RAM capacity or the old candidate.\n\n## Allocation evidence and correctness\n\nAll 110 timing/stat/cycle processes report zero timed allocations and pass independent weighted-generator output checks. Digests match across modes and aliases for each size/round scope. Every G2 process records exactly one live prefix-sized, 64-byte-aligned buffer: 16,000 bytes for 200 pairs, 28,160 for 352. Natural timing buffers consistently occupy page offsets 3072/1024 respectively; forced modes record exactly 0 or 2048. Stack/fixture offsets vary across processes and are preserved in the results. The fixed 512-event journal does not overflow. It is disabled before clocks, and live buffer maps are reconstructed afterward.\n\nForced requests use a bounded pointer registry, separate backing, and head/tail canaries. Ownership-based deallocation remains valid across mode changes; resize copies the smaller live extent and preserves old ownership if allocation fails. The diagnostic reserves 4,128 extra backing bytes per intercepted request. This overhead and the fixed observation journal are not a proposed prover policy, new universal memory limit, measured RAM minimum or heap saving. Matching requests during setup can also be intercepted; the live-buffer assertion checks the actual clock-time scope.\n\nFour native probe tests pass: two new allocator/caller tests cover both sizes, offsets, zeroing, guards, resize transitions and real G2/G1 generator oracles; two inherited perf-helper tests check FIFO behavior. Complete field/core suites are inherited through the exact frozen libraries, not rerun. Initial preparation expected only the two new tests; the first checker stopped before aliases or captures when it saw four passing tests. A versioned count-only repair retained the original successful suite and probe binaries. No successful test or capture was repeated.\n\n## Hardware counters and profiles\n\nAfter all 98 unprofiled captures, six separate FIFO-gated perf-stat processes use the group `{cycles:u,instructions:u,cache-misses:u,branch-misses:u}`; six separate 199 Hz cycles:u profiles follow. Each performs 262,144 calls after three 1,024-call primes. All twelve complete before decoding, hash or archive work. No owned build/helper/hash/archive/other benchmark overlaps captures. An unrelated host ceremony job is recorded; no exclusive-host claim is made.\n\nAll four grouped stat counters report 100% running time. Counts below are per batch from one stat process per context; they are descriptive observations, not confidence intervals.\n\n| G2 pairs | Mode | Cycles/batch | Instructions/batch | IPC | Cache misses/batch | Branch misses/batch |\n|---|---|---:|---:|---:|---:|---:|\n");
    for size in [200, 352] {
        for mode in 0..3 {
            let v = &a["stat"][format!("stat-g2-{size}-m{mode}")];
            for e in v["events"].as_object().unwrap().values() {
                assert_eq!(e["running_percent"], 100.);
            }
            writeln!(
                report,
                "| {size} | {} | {:.3} | {:.3} | {:.6} | {:.6} | {:.6} |",
                ["natural", "offset 0", "offset 2048"][mode],
                v["events"]["cycles:u"]["per_batch"].as_f64().unwrap(),
                v["events"]["instructions:u"]["per_batch"].as_f64().unwrap(),
                v["IPC"].as_f64().unwrap(),
                v["events"]["cache-misses:u"]["per_batch"].as_f64().unwrap(),
                v["events"]["branch-misses:u"]["per_batch"]
                    .as_f64()
                    .unwrap()
            )
            .unwrap();
        }
    }
    report.push_str("\nThe instruction and cycle counts are similar across tested placements. Generic cache-miss counts are retained without assigning them to a particular cache level or causal stall. Counters include the enabled loop and small control/accounting edges. Profiling may alter natural heap placement; actual profile layouts are recorded rather than assumed equal to unprofiled layouts.\n\n| Cycle capture | Samples | Sampled PIDs | Lost |\n|---|---:|---:|---:|\n");
    let mut total = 0u64;
    for (name, v) in a["profiles"].as_object().unwrap() {
        total += v["samples"].as_u64().unwrap();
        writeln!(
            report,
            "| {name} | {} | {} | 0 |",
            v["samples"],
            v["pids"].as_array().unwrap().len()
        )
        .unwrap();
    }
    writeln!(report,"\nTotal samples: {total}. Every cycle capture has one sampled PID and zero lost records. Most sampled period falls in G2 prefix application, which includes inline packing/arithmetic; scalar Fq inversion is also visible. These shares are not exclusive opcode costs or removable proof wall time. Raw perf.data files stay local with hashes; decoded observations are archived.\n").unwrap();
    report.push_str("The preceding six-counter availability check left one event uncounted on a short command. The four-counter group used here was verified available before the design froze; no post-result event substitution occurred. This check is separate from the kernel experiment. Source, protocol, helpers, exact compiler, libraries, both alias paths and executable hashes were pinned before capture and pass afterward. No ELF executable was edited; all baseline library/executable inspections are read-only.\n\n## Accepted comparison (inherited)\n\nThe full all-eight 10× CPU objective remains active and unmet, 0/8. These are the accepted 172-source full-proof observations, not this diagnostic's proof measurements; M5 CPU/Metal times are supplied references.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
        let v = &manifest["m5"][shape];
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
    report.push_str("\nAccepted historical 1x2 peak RSS: 134.50 MiB; primary CPU target: 3.7 ms. Merge54: 161.341835 ms / 1910.1875 MiB peak, with no supplied M5 reference. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates.\n\nNo new SDK/Gnark proof was generated; the independently verified ledger remains 12,983. There is no new complete-MSM/proof, all-eight/54, pressure/concurrency, mobile/ARM/TEE, giant-circuit or AWS/Aeglos result. No cloud resource was created.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [identity audit](artifacts/identity-audit.json), [library provenance](artifacts/library-provenance.json), allocator/probe source and full raw logs are archived. The [preceding packing report](../2026-10-28-direct-ref-pack/README.md) retains its original failed controls and binary-restoration record.\n\nNext: leave the unproven page-placement hypothesis unadopted. Return to reducing work or dependencies in the dominant point-addition kernels, using this common-image diagnostic structure for future component controls. Do not rerun the closed direct-input candidate or select a padding offset from these small estimates.\n");
    save(
        root.join("closeout.json"),
        &json!({"diagnostic_completed":true,"old_control_gap_explained":false,"tested_prefix_placements_show_large_effect":false,"all_layout_intervals_include_parity":true,"adopted":false,"production172_unchanged":true,"accepted_SDK_unchanged":true,"captures":110,"latency_processes":98,"stat_processes":6,"cycle_processes":6,"all_counters_100_percent_running":true,"cycle_samples":total,"new_probe_tests":2,"inherited_perf_tests_executed":2,"timed_allocations":0,"alias_same_inode_and_digest_rechecked":true,"final_pins_verified":pins.as_object().unwrap().len(),"new_Gnark_exports":0,"proof_ledger":12983,"next":"Return to point-addition operation/dependency reduction; no page-offset adoption and no rerun of closed direct-input trial"}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    copy_tree(root, root, &Path::new(D).join("artifacts"));
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
    save(
        root.join("archive-summary.json"),
        &json!({"hashes":files.len(),"git_files":files.len()+1,"GNU_strict_passed":true,"proof_ledger":12983,"adopted":false}),
    );
    println!(
        "Archived{}hashes/{}files;110diagnosticcaptures;noadoptionorold-gapexplanation",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let a = read(format!("{R}/analysis.json"));
    let close = read(format!("{R}/closeout.json"));
    let summary = read(format!("{R}/archive-summary.json"));
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    for(k,v)in[("status",json!("G2 baseline runtime-layout diagnostic complete; tested offsets do not reproduce3xcontrolgap;committed/pushed/verified;noadoption")),("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("production_changed",json!(false)),("adopted",json!(false)),("last_goal_turn_classification",json!("PROGRESS")),("SDK_requests",json!(0)),("new_independent_exports",json!(0)),("independent_proof_count",json!(12983)),("artifact_hashes",summary["hashes"].clone()),("git_files_new_report",summary["git_files"].clone()),("component",a.clone()),("previous_completed_experiment",previous),("next_action",close["next"].clone()),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([]))]{s[k]=v;}
    s["control_runtime_audit"] = close.clone();
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let head=format!("CURRENT COMPLETE/PUSHED — G2 BASELINE RUNTIME-LAYOUT DIAGNOSTIC\n\nHEAD/remote verified {commit}. Root {R}; report {D}/README.md. {}strictverifiedhashes/{}Gitfiles. Diagnosticcompleted,NOadoption,oldG2gapUNEXPLAINED. Accepted172/SDK14da unchanged,local1x2historical8.324853ms/RSS134.50MiB,goalACTIVEunmet0/8,ledger12983unchanged,nojobs/cloud. LastgoalturnPROGRESS: new isolatedbaseline placement/instrumentation study completed,notstatus/no-progress.\n\nSinglebaseline executable linkedONLYfromdirect-ref-pack/frozen-libraries/frozen acceptedarithmetic+originalcalleradapters,NOnewcore/field/Cargobuild. baseline-aa/bb equal-lengthhardlinkswithsameinode/SHArecheckedaftercapture. NewObservedallocatorlog512events>=1024Boutsideclock;3modesnatural/forcedprefixpageoffset0/2048forrequests16000B/28160B align64. Registry64blocksownsdeallocationindependentmode;head/tailcanaries,backing+4128B diagnosticnotproduction. Native4probe testsPASSONCE(2newallocator/caller+2inheritedperf). Firstpreparechecked2instead4andfailedBEFOREaliases/captures;repair-preparation.rscreatesdriver-v2 withcountonlyfixandpins,NOtest/probebuildrepeat.\n\n7blocks7contexts2aliases=98latency,16384callsafter3x1024prime/check/reset;G2two sizes3modes+G1owned200natural. THEN6FIFOwarmstat262144calls4groupedcounterseach100%running,THEN6warm199Hzcyclesprofiles262144calls,THEN18decode/owner/losssteps,128totalworkflowsteps. All110generatororacle/digest/zero-timed-allocationchecks;noeventjournaloverflow,exactly1liveG2prefixbuffer. Natural offsets200=3072/352=1024(all14each),forcedactual0/2048,stack/fixturevaries. Capture96958TERMINALexit0. Nohelper/build/hash/archiveoverlap. Allfinalpins {}checkedagain. All172production+14daSDKunchanged.\n\nPooledG2 ns/batch200 natural7836.895/page0 7835.083/page2048 7826.023;352 natural13230.685/page0 13221.800/page2048 13228.230. Every7blockminmaxplacementinterval includesparity;smallpositiveestimatesretained,nochoosepadding. G2alias b/a close, G1negativecontrol pairedrange.956906..1.041932retained soNOUNIVERSALstabilityclaim. Statcycles20041254.765/41175.148/41202.101,inst109427.31each,IPC2.65;35269592.878/69506.322/69552.380,inst184968.83/184950.83/184959.83,IPC2.66;genericmisses recorded,nocachelevelorstallcausalclaim. Sixcycleprofiles {}samplesonePIDzeroLost;G2prefix90-94%periodinclinlinepacking/arith,scalarFqinversevisible;notproofwallcost. TestedplacementsDONOTexplain/reproduceold3xgap;oldclosedpackingcandidate remainsnegative/unqualifiedG2,norescue/reroll. Longerprimes/newdriver differoldschedule,notreplacementdata.\n\nNEXT: operation/dependency reductionin dominantpointkernels; doNOTadoptpagepaddingorunchangedoldcandidate. Sourceauditthisturnfoundno prior README trial precomputingT=2Ax+Bxandusingdx=T-slopeSquared todecoupleYfromXoutput;onlyIDEA,NOTselected/protocoled/built. Mustauditpriorgraph/anchorvariantsandfreezedesignfirstifpursued. Earlierfinishphase3loops/inputanchorrecords/quarterR260/pathpacking/GLVringalreadynegative. NoSDK/Gnark/all8/54/resource/portable/AWSvalidationnew. PreserveFFTengineeringexception/falsefullperformancegates;latestAWSstale17137261af77. CommitANDpush eachcompletedexperiment,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Previousheadershistorical.\n\n",summary["hashes"],summary["git_files"],close["final_pins_verified"],close["cycle_samples"]);
    fs::write(hp, format!("{head}{old}")).unwrap();
    println!("State/handoffupdated;goalactive0/8;noadoption;ledger12983");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
