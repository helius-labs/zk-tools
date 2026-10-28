use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-queue";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
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
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let q = e.path();
        if e.file_type().unwrap().is_dir() {
            let n = e.file_name();
            let n = n.to_string_lossy();
            if n != "profiles" && n != "frozen" && !n.starts_with("target-") {
                walk(&q, out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(q)
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["captures"], 845);
    assert_eq!(a["timed_MSM_calls"], 7907);
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    assert_eq!(
        read(format!("{R}/resources/result.json"))["status"],
        "complete"
    );
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
    }
    let locations = read(format!("{R}/control-locations.json"));
    assert_eq!(locations["native_code_or_arithmetic_changed"], false);
    assert_eq!(
        read(format!("{R}/control-sections.json"))["all_text_and_rodata_byte_identical"],
        true
    );
    let mut bad = vec![];
    let mut regressions = vec![];
    for (k, v) in a["pairs"].as_object().unwrap() {
        if k.ends_with("1-over-0") {
            if v["low"].as_f64().unwrap() < 1. / 1.03 || v["high"].as_f64().unwrap() > 1.03 {
                bad.push(k.clone())
            }
        } else if v["high"].as_f64().unwrap() > 1.02 {
            regressions.push(k.clone())
        }
    }
    let mut running = true;
    for v in a["stat"].as_object().unwrap().values() {
        for e in v["events"].as_object().unwrap().values() {
            running &= e["running_percent"] == 100.;
        }
    }
    let mut samples = 0;
    for p in a["profiles"].as_object().unwrap().values() {
        assert_eq!(p["pids"].as_array().unwrap().len(), 1);
        assert_eq!(p["lost"], 0);
        assert!(!p["sampled_worker_tids"].as_array().unwrap().is_empty());
        samples += p["samples"].as_u64().unwrap();
    }
    save(
        format!("{R}/interpretation.json"),
        &json!({"calibration_failures":bad,"nonregression_failures":regressions,"control_code_and_constants_byte_identical":true,"source_line_records":locations["panic_locations"].as_array().unwrap().len(),"all_counters_100_percent_running":running,"profile_samples":samples,"field_unchanged":true,"production_changed":false,"goal_status":"active","proof_ledger":13181}),
    );
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    let mut compressed = vec![];
    for p in files {
        let rel = p.strip_prefix(R).unwrap();
        let b = fs::read(&p).unwrap();
        if b.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let q = Path::new(D).join("artifacts").join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        if rel.to_string_lossy().contains("runs/loss-") && rel.ends_with("stdout.log") {
            let out = Command::new("gzip")
                .args(["-n", "-c"])
                .arg(&p)
                .output()
                .unwrap();
            assert!(out.status.success());
            let z = q.with_extension("log.gz");
            fs::write(&z, out.stdout).unwrap();
            let back = Command::new("gzip").arg("-dc").arg(&z).output().unwrap();
            assert!(back.status.success());
            assert_eq!(back.stdout, b);
            compressed.push(json!({"path":z.strip_prefix(D).unwrap(),"uncompressed_sha256":hash(p),"gzip_sha256":hash(z)}));
        } else {
            fs::write(q, b).unwrap();
        }
    }
    assert_eq!(compressed.len(), 4);
    save(
        format!("{D}/artifacts/local-executable-pins.json"),
        &json!(local),
    );
    save(
        format!("{D}/artifacts/loss-compression.json"),
        &json!(compressed),
    );
    let pass = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let verdict = if pass {
        "passes the declared complete-MSM screen; full SDK qualification remains required"
    } else {
        "fails the declared complete-MSM screen and is not adopted"
    };
    let x = &a["logical32_aggregate"]["on-over-off"];
    let f = &a["logical32_aggregate"]["on-over-frozen"];
    let mut s=format!("# Borrowed pending operands in G2 phi MSM\n\nThe candidate **{verdict}**. At30,089points/32workers, ON/OFF paired ratio is **{:.9}**, marginal interval [{:.9},{:.9}], {}/31wins; ON/frozen is **{:.9}**, interval [{:.9},{:.9}], {}/31wins. Production and accepted SDK remain unchanged.\n\n## Change and scope\n\nOrdinary G2 phi windows store borrowed16byte (encoded bucket index,point reference) entries instead of owned136byte point entries. Signs are retained in the index until bucket insertion/projective fallback or Y packing. The new window omits the two scalar denominator/prefix vectors that packed dispatch does not use. The signed borrowed prefix uses the accepted field operations, formula and Fq2AddScratch prefix/norm/tail storage. No field arithmetic, scalar decomposition, width12/parts2, key representation, persistent table/cache or execution plan changes. Phi still transforms temporary high-component bucket sums, not the input key.\n\nA cfg-only dispatch selects this window after the unchanged special top-window branch. Unsupported packed/direct/canonical/vector/prefix options retain the old window. Existing identity/equal/opposite handling and collision/projective fallback remain. References borrow immutable key operands for the window lifetime.168 original private files remain exact; reverse edits recover all172 accepted files. An independent diagnostic frozen copy has170 exact originals and only two identical full-MSM export suffixes. Field source remains byte-exact, and its accepted qualification is inherited.\n\nThis differs from earlier borrowed sparse-group views and the accepted G1 borrowed queue: it replaces G2's internal pending batch and collision queue. Earlier packing/record negatives remain valid, and no large gain is inferred from source byte counts.\n\n## Complete-MSM measurements\n\nRyzen9950X,31fixed rotated/reversed blocks,3point sizes×soloCPU2/physical16CPUs0–15/logical32CPUs0–31×frozen/OFF/ON. Each process builds public synthetic generator-derived bases and full-width scalars, shares a fixed block/size seed across variants, makes3checked primes, then7measured complete G2 phi MSMs. Timing includes queue fill/flush, scalar-input reset copy, field arithmetic, inverse, merging, weighted reduction, allocation and deallocation. The input reset is necessary because raw_exclusive consumes its words and is charged equally. Every full output is checked against a known generator-weighted scalar sum after timing. These are complete-MSM component measurements, not real proof or SDK latencies.\n\n| Points | Workers | Frozen ms/MSM | OFF ms/MSM | ON ms/MSM | ON/OFF | Marginal interval | Wins |\n|---|---|---:|---:|---:|---:|---|---:|\n",x["median"].as_f64().unwrap(),x["low"].as_f64().unwrap(),x["high"].as_f64().unwrap(),x["wins"],f["median"].as_f64().unwrap(),f["low"].as_f64().unwrap(),f["high"].as_f64().unwrap(),f["wins"]);
    for n in [4096, 16384, 30089] {
        for t in ["solo", "physical16", "logical32"] {
            let v = |m| {
                a["cells"][format!("g2-msm/{n}/{t}/m{m}")]["ns_per_MSM"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1e6
            };
            let p = &a["pairs"][format!("g2-msm/{n}/{t}/2-over-1")];
            writeln!(
                s,
                "| {n} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9},{:.9}] | {}/31 |",
                v(0),
                v(1),
                v(2),
                p["median"].as_f64().unwrap(),
                p["low"].as_f64().unwrap(),
                p["high"].as_f64().unwrap(),
                p["wins"]
            )
            .unwrap();
        }
    }
    s.push_str("\n| Points/workers | OFF/frozen | Interval | ON/frozen | Interval |\n|---|---:|---|---:|---|\n");
    for n in [4096, 16384, 30089] {
        for t in ["solo", "physical16", "logical32"] {
            let p = &a["pairs"][format!("g2-msm/{n}/{t}/1-over-0")];
            let q = &a["pairs"][format!("g2-msm/{n}/{t}/2-over-0")];
            writeln!(
                s,
                "| {n}/{t} | {:.9} | [{:.9},{:.9}] | {:.9} | [{:.9},{:.9}] |",
                p["median"].as_f64().unwrap(),
                p["low"].as_f64().unwrap(),
                p["high"].as_f64().unwrap(),
                q["median"].as_f64().unwrap(),
                q["low"].as_f64().unwrap(),
                q["high"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    writeln!(s,"\nCalibration **{}**, nonregression **{}**, primary gain **{}**. Calibration failures:{}; nonregression failures:{}.\n",a["gates"]["calibration"],a["gates"]["nonregression"],a["gates"]["logical32_gain"],json!(bad),json!(regressions)).unwrap();
    s.push_str("All9OFF/frozen intervals must fit[1/1.03,1.03]; all18ON/frozen+ON/OFF upper ratios<=1.02. Primary30,089/logical32 requires upper<1 and>=21/31wins vsboth controls; legacy logical32_aggregate contains only this primary case. Ordered ranks10–22of31 provide nominal97.055%marginal coverage under independent exchangeable blocks, not familywise. No gain floor, excluded blocks, extensions or unchanged reroll. All positive estimates and uncertainty are retained.\n\n## Separate requested-memory observations\n\nOrdinary timing and PMU executables use the original uninstrumented allocator; no allocation counters run in those intervals.27separate resource processes enable requested-allocation accounting in a different executable, after3primes, around1MSM. Their elapsed times do not enter timing gates. Requested peak extra is the maximum tracked live requested bytes above the warmed fixture/pool baseline; it excludes allocator internals and is not physical memory, minimumRAM or a universal bound. Whole-processRSS includes fixtures, pool and primes. Counting code snapshots raw counters before constructing its JSON report.\n\n| Points/workers | OFF peak extra MiB | ON peak extra MiB | OFF requests | ON requests | OFF/ON RSS MiB |\n|---|---:|---:|---:|---:|---:|\n");
    for n in [4096, 16384, 30089] {
        for t in ["solo", "physical16", "logical32"] {
            let off = &a["capacities_per_worker"][format!("resource-g2-msm-{n}-{t}-m1")];
            let on = &a["capacities_per_worker"][format!("resource-g2-msm-{n}-{t}-m2")];
            writeln!(
                s,
                "| {n}/{t} | {:.4} | {:.4} | {} | {} | {:.3}/{:.3} |",
                off["heap"]["peak_extra"].as_f64().unwrap() / 1048576.,
                on["heap"]["peak_extra"].as_f64().unwrap() / 1048576.,
                off["heap"]["allocation_requests"],
                on["heap"]["allocation_requests"],
                off["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.,
                on["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.
            )
            .unwrap();
        }
    }
    s.push_str("\nThe resource dictionary retains the historical JSON field name capacities_per_worker, but entries here are whole-process resource probes, not per-worker capacities. All raw requested bytes, live-before/after and frozen observations remain available. Fewer queue payload bytes do not by themselves prove lower process peak or faster proofs. No key-sized cache is added.\n\n## Correctness, controls and profiles\n\nNative OFF passes119core tests; ON passes122including3new tests. Signed borrowed prefix coverage includes lengths0/1/7/8/9/15/16/32/200/352/511, sign patterns, tails and3changing rounds. Full-width phi MSM checks1024/4096/8192points against generator sums; adversarial1024/2048point cases include zero, identity, equal/opposite bases and scalar edges. Original tests cover unaffected paths; optional missing-fixture scenarios are not claimed. Three harness tests pervariant pass (one MSM/reset/affinity test and two inherited FIFO tests). One separately compiled exact heap-accounting test passes, without repeating the harness suite.\n\nAll runtime rlibs/proc macros are frozen from independent target roots and actual core/field flags/features checked. Before timing, complete.text/.rodata and all other runtime section bytes match frozen/OFF except individually decoded panic source-line fields+2 in g2_phi and build ID. No other control difference is accepted. Static IFMA/disassembly counts are recorded, not dynamic work or spill/stack bounds. Initial prefix generation expected two index sites but found three; its repair masks all three including scatter before builds, preserving initial generator and receipts. Any other failures/repairs remain recorded. No successful tests or captures are repeated for bookkeeping.\n\nAll837latency processes finish before4FIFOgrouped perf-stat, then4FIFO199Hzcycles profiles(30,089points,OFF/ON,16/32workers,256MSMs after3primes), then12owner/decode/loss commands. After these,27resource processes run separately. There are7907timed complete MSM calls in ordinary/PMU captures plus2535primes; resource probes add108diagnosticMSMs. These counts exclude native/harness test internals and are not proofs. No owned build/helper/hash/archive/otherbenchmark overlaps captures. Host snapshots do not establish exclusive use.\n\n| Counter case | Cycles/MSM | Instructions/MSM | IPC |\n|---|---:|---:|---:|\n");
    for (k, v) in a["stat"].as_object().unwrap() {
        writeln!(
            s,
            "| {k} | {} | {} | {} |",
            v["events"]["cycles:u"]["per_MSM"], v["events"]["instructions:u"]["per_MSM"], v["IPC"]
        )
        .unwrap();
    }
    writeln!(s,"\nAll event groups100% running: **{running}**. Counters do not isolate RAM bandwidth, an execution port or removable proof latency. SMT cycles sum logical-thread events.\n\n| Profile | Samples | Sampled pool workers | Lost |\n|---|---:|---:|---:|").unwrap();
    for (k, v) in a["profiles"].as_object().unwrap() {
        writeln!(
            s,
            "| {k} | {} | {} | {} |",
            v["samples"],
            v["sampled_worker_tids"].as_array().unwrap().len(),
            v["lost"]
        )
        .unwrap();
    }
    writeln!(s,"\nTotal{samples} samples; one PID/profile and zeroLost. Actual sampled worker coverage is reported, without assuming every configured worker receives a sample. Full symbol maps are retained. Rawperf/executables stay local with hashes;4loss dumps use deterministic gzip and byte-exact round-trip verification. Captured logs/measured source bytes, including trailing blank lines, are preserved; authored helper/report whitespace checked separately.\n").unwrap();
    if pass {
        s.push_str("The complete-MSM screen permits a separate pure full SDK trial with fresh proofs and resource/compatibility qualification. This is not production adoption or proof-time evidence.\n\n");
    } else {
        s.push_str("The exact borrowed-queue implementation is closed without a full SDK trial or adoption. No unchanged queue/layout reroll is selected.\n\n");
    }
    s.push_str("## Inherited CPU and Metal comparison\n\nNo new full-proof latency/RSS is established. Accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The all-eight CPU10x objective remains active and unmet,0/8. These accepted local figures and supplied M5 references are inherited.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
    let shapes = [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ];
    let accepted = [
        8.324853, 11.460675, 15.854365, 24.084626, 33.693397, 39.981636, 66.010670, 65.086624,
    ];
    let cpu = [37., 59., 109., 189., 284., 335., 528., 507.];
    let metal = [24., 30., 48., 74., 101., 124., 176., 175.];
    for i in 0..8 {
        writeln!(
            s,
            "| {} | {:.6} | {:.1} | {:.1} | {:.3}× | {:.3}× |",
            shapes[i],
            accepted[i],
            cpu[i],
            metal[i],
            cpu[i] / accepted[i],
            metal[i] / accepted[i]
        )
        .unwrap();
    }
    s.push_str("\nHistorical primary134.50MiB; merge54 remains161.341835ms/1910.1875MiB without a supplied M5 reference. No new SDK/Gnark proof, mobile/TEE, minimumRAM, memory-pressure, giant-circuit or AWS/Aeglos result. Proofledger remains13,181.\n\nThe memory-band paper reviewed during selection was already tested in an earlier experiment; its BLS12-381 results are not borrowed-queue or Zopro speed evidence ([Noël/Plantard2026](https://link.springer.com/article/10.1007/s13389-026-00393-z), [prior negative bands](../2026-10-25-g1-sequential-bands-component/README.md)).\n\n[Design](artifacts/design.md), [new window](artifacts/g2_borrowed_window.rs), [new signed prefix](artifacts/g2_borrowed_prefix.rs), [analysis](artifacts/analysis.json), [native suites](artifacts/native/result.json), [harness checks](artifacts/linked/result.json), [all timing/PMU](artifacts/runs/result.json), [resource probes](artifacts/resources/result.json), [source audit](artifacts/source-audit.json), [control audit](artifacts/control-locations.json).\n");
    s.push_str(&fs::read_to_string(format!("{R}/conclusion.md")).unwrap());
    fs::write(format!("{D}/README.md"), s).unwrap();
    let mut all = vec![];
    walk(Path::new(D), &mut all);
    all.sort();
    let mut sums = String::new();
    for p in &all {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        )
        .unwrap();
    }
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    let check = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    fs::write(format!("{R}/archive-verification.log"), &check.stdout).unwrap();
    assert!(check.status.success());
    save(
        format!("{R}/archive-summary.json"),
        &json!({"artifact_hashes":all.len(),"Git_files":all.len()+1,"component_gate_passed":pass,"production172_unchanged":true,"profile_samples":samples,"run_pins_verified":pins.as_object().unwrap().len(),"proof_ledger":13181,"goal_status":"active","CPU10x_rows":0}),
    );
    println!("Archived{}strictverifiedhashes;MSMgate={pass}", all.len());
}
fn state() {
    let h = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir("/home/ubuntu/src/zk-tools")
        .output()
        .unwrap();
    assert!(h.status.success());
    let head = String::from_utf8(h.stdout).unwrap().trim().to_string();
    let remote = Command::new("git")
        .args([
            "-c",
            "url.https://github.com/.insteadOf=git@github.com:",
            "-c",
            "credential.helper=!gh auth git-credential",
            "ls-remote",
            "origin",
            "refs/heads/sergei/zopro-cpu-perf",
        ])
        .current_dir("/home/ubuntu/src/zk-tools")
        .output()
        .unwrap();
    assert!(remote.status.success());
    assert_eq!(
        String::from_utf8(remote.stdout)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap(),
        head
    );
    let a = read(format!("{R}/analysis.json"));
    let pass = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut v = read(p);
    v["previous_completed_experiment"] = v["commit"].clone();
    v["commit"] = json!(head);
    v["status"] = json!(if pass {
        "BorrowedG2queue completeMSMgate passed; SDK pending; pushed verified"
    } else {
        "BorrowedG2queue completeMSMgate failed; closed/notadopted; pushed verified"
    });
    v["root"] = json!(R);
    v["report"] = json!(format!("{D}/README.md"));
    v["component"] = a.clone();
    v["control_runtime_audit"] = read(format!("{R}/control-locations.json"));
    v["last_goal_turn_classification"] = json!("progress");
    v["next_action"] = json!(if pass {
        "Pure fullSDK qualification of exact borrowedG2queue with accepted/fresh controls and new independently verified proof exports"
    } else {
        "Diagnose byte-identical control gap via unchanged binaries/mode labels/equal-length aliases; retain positive physical16/memory observations and failed gate; no optimization reroll/adoption"
    });
    v["pushed"] = json!(true);
    v["remote_verified"] = json!(true);
    v["goal_status"] = json!("active");
    v["goal_achieved"] = json!(false);
    v["production_changed"] = json!(false);
    v["adopted"] = json!(false);
    v["owned_live_jobs"] = json!([]);
    v["owned_cloud_resources"] = json!([]);
    let summary = read(format!("{R}/archive-summary.json"));
    v["artifact_hashes"] = summary["artifact_hashes"].clone();
    v["git_files_new_report"] = summary["Git_files"].clone();
    save(p, &v);
    let handoff = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(handoff).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — BORROWED G2 PENDING QUEUE\n\nHEAD/remoteverified{head};root{R};report{D}/README.md. CompleteMSMgate={pass};notadopted/noSDKproofgain. Primary30089logical32ratios{};gates{}. Production172/SDK14da unchanged/historical1x2 8.324853ms/RSS134.50MiB;goalACTIVE0/8;proofledger13181;noownedjobs/cloud. PreviousgoalturnPROGRESS scaledR260native62e60fb86/const240c0f251 publishednegative;currentPROGRESS nativeborrowedG2queue/fullMSM/memory/profiles.\n\nG2phi ordinary window borrowed16Bpending entries vsowned136B, omits2unusedscalarbuffers. Samefield/phi/width12parts2/key/bucketrepresentation/prefixrecords;no persistenttables. Materializesignsoninsert/projective/Ypack. Topwindow/unsupportedconfigoldverbatim.168originalsexact/reverse172;frozen170exact+2diagwrappers. Fieldunchangedqualificationinherited. CoreOFF119/ON122PASS(3newtests signedprefix/tails/reuse+fullwidthG2MSM+exceptionkeys);3harness each+oneheapaccountingexactPASS. Frozen/OFF.text/.rodata+allruntimebytesexact exceptdecodedg2_phiRustpanicline+2/buildid.837latency/4stat/4cycles/12decode857steps845captures7907timedMSMs+2535primes;27separateresourcecaptures108MSMs;fulloutputsverified. Timingallocatoruninstrumented;heapobserverseparatecfgexes. Rawprofiles/exeslocalhash,4lossgzroundtrip,pinsverified. Noownedhelper/build/hash/archiveoverlap/noexclusivehostclaim. Actualsampleworkercoverage reported. Readanalysis/interpretation/allresourcecells;sourcebytesnotphysicalRAM/minRAM/provinglatency. Initialprefixgeneration2vs3indexsite repair beforebuildpreserved;allotherfailuresreceiptsretained.\n\nNEXT:{}. No unchangedreroll; preserveallpositives/uncertainty/failedgates. No newSDK/Gnark/RSSqualification/mobile/TEE/pressure/giant/AWSclaim. AcceptedFFTengineeringexceptionfalsefullgatesunchanged. CommitANDpushcompletedexperiments,noquestions/subagents/hugecaches/Casmrewrite. Olderheadershistorical.\n\n",a["logical32_aggregate"],a["gates"],v["next_action"]);
    fs::write(handoff, format!("{header}{old}")).unwrap();
    println!("Verified push{head};state/handoff updated;goalactive0/8");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "archive" => archive(),
        "state" => state(),
        _ => panic!("mode"),
    }
}
