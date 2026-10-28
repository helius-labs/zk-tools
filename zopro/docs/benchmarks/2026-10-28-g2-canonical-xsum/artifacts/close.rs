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
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-canonical-xsum";
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
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
    }
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
    let hashes = [
        hash(format!("{R}/frozen-probe")),
        hash(format!("{R}/off-probe")),
        hash(format!("{R}/on-probe")),
    ];
    let mut all_running = true;
    for v in a["stat"].as_object().unwrap().values() {
        for e in v["events"].as_object().unwrap().values() {
            all_running &= e["running_percent"] == 100.;
        }
    }
    let mut samples = 0;
    for p in a["profiles"].as_object().unwrap().values() {
        assert_eq!(p["sampled_worker_tids"].as_array().unwrap().len(), 32);
        assert_eq!(p["pids"].as_array().unwrap().len(), 1);
        assert_eq!(p["lost"], 0);
        samples += p["samples"].as_u64().unwrap();
    }
    let interp = json!({"calibration_failures":bad,"nonregression_failures":regressions,"control_executables_byte_identical":hashes[0]==hashes[1],"executable_sha256":{"frozen":hashes[0],"off":hashes[1],"on":hashes[2]},"all_counters_100_percent_running":all_running,"profile_samples":samples,"all_profiles_all32_workers":true,"goal_status":"active","proof_ledger":13181});
    save(format!("{R}/interpretation.json"), &interp);
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
    let passed = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let verdict = if passed {
        "passes the declared native caller screen; full SDK qualification remains required"
    } else {
        "fails the declared native caller screen and is not adopted"
    };
    let x = &a["logical32_aggregate"]["on-over-off"];
    let f = &a["logical32_aggregate"]["on-over-frozen"];
    let mut s=format!("# Canonical X-coordinate sum in G2 prefix addition\n\nThe candidate **{verdict}**. Primary G2/200 at 32 logical CPUs has ON/OFF ratio **{:.9}**, marginal interval [{:.9}, {:.9}], {}/31 wins; ON/frozen is **{:.9}**, interval [{:.9}, {:.9}], {}/31 wins. Production and accepted SDK remain unchanged.\n\n## Change\n\nThe prefix reverse loop packs canonical Ax and Bx but uses generic Fq2::lanes_add for their sum. Under FAST=true, route these two base-field sums through existing canonical_add instead. Inputs below p yield a sum below 2p, removing the generic final 2p corrections. FAST=false uses the existing generic fallback. This is separate from the coordinate-scaled R260 model: no representation, wide field primitive, key/bucket layout, table, cache, heap allocation or public option changes.\n\nOne runtime expression in msm/g2_prefix.rs changes under private cfg zopro_g2_canonical_xsum; OFF retains its original expression. Original diagnostic adapters are appended in msm/lib. Reversing those edits recovers all 172 accepted sources; 169 originals remain byte-exact. Field, G1, FFT, solver and all other arithmetic are unchanged. Earlier rejected reduced-Gauss and other candidate code is absent.\n\n## Complete caller results\n\n31 fixed rotated/reversed blocks on Ryzen 9950X, independent accepted frozen/fresh OFF/ON executables. Solo uses CPU2, physical16 uses CPUs0–15, logical32 uses CPUs0–31. Each process runs three reset/checked 1024-call primes, then 8192 calls per worker. Ready/start/done/release barriers keep every worker's checking and allocations outside every worker's clock. The wave measurement includes slowest-worker and barrier tails. These synthetic ordinary points at real stream sizes are kernel timings, not full proofs or complete MSMs.\n\n| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF | Marginal interval | Wins |\n|---|---|---:|---:|---:|---:|---|---:|\n",x["median"].as_f64().unwrap(),x["low"].as_f64().unwrap(),x["high"].as_f64().unwrap(),x["wins"],f["median"].as_f64().unwrap(),f["low"].as_f64().unwrap(),f["high"].as_f64().unwrap(),f["wins"]);
    for (kind, n) in [
        ("g2-prefix", 32),
        ("g2-prefix", 200),
        ("g1-borrowed-signed", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let val = |mode| {
                a["cells"][format!("{kind}/{n}/{t}/m{mode}")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.
            };
            let v = &a["pairs"][format!("{kind}/{n}/{t}/2-over-1")];
            writeln!(
                s,
                "| {kind}/{n} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9}, {:.9}] | {}/31 |",
                val(0),
                val(1),
                val(2),
                v["median"].as_f64().unwrap(),
                v["low"].as_f64().unwrap(),
                v["high"].as_f64().unwrap(),
                v["wins"]
            )
            .unwrap();
        }
    }
    s.push_str("\n| Caller/pairs/layout | OFF/frozen | Interval | ON/frozen | Interval |\n|---|---:|---|---:|---|\n");
    for (kind, n) in [
        ("g2-prefix", 32),
        ("g2-prefix", 200),
        ("g1-borrowed-signed", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let p = &a["pairs"][format!("{kind}/{n}/{t}/1-over-0")];
            let q = &a["pairs"][format!("{kind}/{n}/{t}/2-over-0")];
            writeln!(
                s,
                "| {kind}/{n}/{t} | {:.9} | [{:.9}, {:.9}] | {:.9} | [{:.9}, {:.9}] |",
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
    writeln!(s,"\nCalibration **{}**, nonregression **{}**, primary gain **{}**. Calibration failures: {}. Nonregression failures: {}.\n",a["gates"]["calibration"],a["gates"]["nonregression"],a["gates"]["logical32_gain"],interp["calibration_failures"],interp["nonregression_failures"]).unwrap();
    s.push_str("All nine OFF/frozen intervals must fit [1/1.03,1.03]; every ON/frozen and ON/OFF upper ratio must be <=1.02. The primary G2/200 logical32 gain requires upper<1 and at least21/31 wins against both controls. The legacy logical32_aggregate field contains only this primary caller. Ordered ranks10–22 of31 have nominal97.055% marginal coverage under independent exchangeable blocks, not familywise coverage. No minimum gain floor, excluded blocks, extension or unchanged reroll is used. All positive estimates remain reported within scope.\n\n## Validation and profiling\n\nFull OFF and ON native core suites each pass 122 tests: 119 accepted tests and three original point-adapter tests. Existing prefix tests cover FAST true/false, tails, scratch reuse and generator sums. Each frozen/OFF/ON harness passes three tests including complete point oracles and concurrency/barrier/affinity checks. Some optional external-fixture tests return when fixtures are absent; those scenarios are not claimed. No fresh field test is claimed; unchanged canonical_add retains its prior qualification. Every native dependency and proc-macro shared object is frozen separately; actual field/core flags and features are checked.\n\nAll 837 latency processes finish before four FIFO grouped perf-stat captures, then four separate FIFO199Hz cycles:u profiles, then 12 decode/owner/loss steps. PMU captures use logical32, G2/200 and G1/200 OFF/ON, 262144 calls per worker. No owned build/helper/hash/archive/other benchmark overlaps captures. Host snapshots do not prove exclusive-host use.\n\n");
    writeln!(s,"All {} kernel captures verify generator outputs, capacity consistency and zero global timed allocations. There are {} timed kernel calls, not proofs. Per-worker capacity and per-thread CPU/throughput data remain in analysis.json. No measured peak RSS, stack bound or minimum RAM is established.\n",a["captures"],a["timed_kernel_calls"]).unwrap();
    s.push_str("| Counter case | Cycles/call | Instructions/call | IPC |\n|---|---:|---:|---:|\n");
    for (k, v) in a["stat"].as_object().unwrap() {
        writeln!(
            s,
            "| {k} | {} | {} | {} |",
            v["events"]["cycles:u"]["per_kernel_call"],
            v["events"]["instructions:u"]["per_kernel_call"],
            v["IPC"]
        )
        .unwrap();
    }
    writeln!(s,"\nAll counter events100% running: **{all_running}**. Counter observations do not isolate a memory level, execution port or removable proof latency. SMT event cycles are summed logical-thread cycles.\n").unwrap();
    s.push_str("| Profile | Samples | Sampled worker TIDs | Lost |\n|---|---:|---:|---:|\n");
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
    writeln!(s,"\nTotal {samples} samples, one PID/profile and all32 worker TIDs with zero lost records. Static disassembly counts are archived, not executed work or a causal decomposition. Frozen/OFF executable bytes identical: **{}**; all executable hashes are in interpretation.json. Process placement/address layout and host variation are not isolated. Raw profiles and executables stay local with hashes. Four loss dumps use deterministic gzip with verified byte-exact decompression. Captured logs, including trailing blank lines, retain original bytes; authored source/report whitespace is checked separately.\n",interp["control_executables_byte_identical"]).unwrap();
    if !passed {
        s.push_str("This exact candidate is closed without an SDK trial or production adoption. The broader coordinate-scaled R260 native prototype remains a separate next candidate; this result cannot establish its performance.\n\n");
    } else {
        s.push_str("A separate pure SDK trial is permitted. This screen alone does not justify production adoption or a full-proof speed claim.\n\n");
    }
    s.push_str("## Inherited CPU and Metal comparison\n\nNo new full-proof timing or peak RSS is established. Accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU10x objective is active and unmet, 0/8. These accepted local timings and supplied M5 references are inherited.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    s.push_str("\nHistorical primary peak134.50MiB; merge54 remains161.341835ms /1910.1875MiB with no supplied M5 reference. No new SDK/Gnark, portable/mobile/TEE, concurrent-proof, memory-pressure, giant-circuit or AWS/Aeglos measurement is claimed. Independent proof ledger remains13,181.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native suites](artifacts/native/result.json), [harness tests](artifacts/linked/result.json), [all captures](artifacts/runs/result.json), [interpretation](artifacts/interpretation.json), [code generation](artifacts/codegen.json).\n");
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
        &json!({"artifact_hashes":all.len(),"Git_files":all.len()+1,"component_gate_passed":passed,"production172_unchanged":true,"profile_samples":samples,"run_pins_verified":pins.as_object().unwrap().len(),"proof_ledger":13181,"goal_status":"active","CPU10x_rows":0}),
    );
    println!("Archived{}verifiedhashes;gate={passed}", all.len());
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
    let passed = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut v = read(p);
    v["previous_completed_experiment"] = v["commit"].clone();
    v["commit"] = json!(head);
    v["status"] = json!(if passed {
        "CanonicalG2Xsum caller passed; SDK pending; pushed verified"
    } else {
        "CanonicalG2Xsum caller gate failed; closed/not adopted; pushed verified"
    });
    v["root"] = json!(R);
    v["report"] = json!(format!("{D}/README.md"));
    v["component"] = a.clone();
    v["last_goal_turn_classification"] = json!("progress");
    v["next_action"] = json!(if passed {
        "Pure full SDK trial of canonical G2 Xsum with accepted/fresh controls and new independent proof exports"
    } else {
        "Native bounded coordinate-scaled R260 prototype after passed exact model/work ledger; no unchanged canonicalXsum reroll"
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
    let primary = &a["logical32_aggregate"];
    let header=format!("CURRENT COMPLETE/PUSHED — CANONICAL G2 PREFIX XSUM CALLERS\n\nHEAD/remote verified {head}; root {R}; report {D}/README.md. Caller gate={passed};notadopted/noSDKproofgain. Primaryratios {primary}. Fullanalysis gates{}; original172/SDK14da unchanged;historical1x2 8.324853ms/RSS134.50MiB;goalACTIVE0/8;proofledger13181. Noownedjobs/cloud. CurrentgoalturnPROGRESS published990fd7b60scaledR260exactmodelthencompletedthisdistinctnativecaller.\n\nRuntimeonlyprefixreverse xsum genericFq2lanes_add->existingcanonical_addFAST underprivatecfgzopro_g2_canonical_xsum;FASTfalsegenericfallback.169originalsexact/reverseall172. NoR260/Gauss/GCD/GLV/weighted/fieldchange/keycopy/extraheap. NativeOFF/ON122tests each;3harness eachfrozen/OFF/ON.31blocks9contexts3variants837latency+8PMU+12decode857steps845captures.179101696timedkernelcallsNOTproofs; alloracles/zeroGLOBALtimedalloc/capacitychecked. Allprofilesall32workersonePIDzeroLost;rawperf/exeslocalhash;4lossgzroundtrip. Readanalysis/interpretation/archive-summary for all ratios/gates/pins/counters. Nohelper/build/hash/archiveoverlapcapture;noexclusivehostclaim. Noownedsuccessfulcommandrerunforbookkeeping.\n\nNEXT:{}. Preserveallpositiveestimates/failedgates, no unchangedreroll. Broader scaledR260 passedexactscreen990fd7b60;nativewideprototype stillunmeasured. KeepacceptedFFTengineeringexceptionfalsegates,commitANDpushcompletedexperiments,noquestions/subagents/hugecaches/Casmrewrite. Olderheadershistorical.\n\n",a["gates"],v["next_action"]);
    fs::write(handoff, format!("{header}{old}")).unwrap();
    println!("Verified push {head};state/handoff updated;goalactive0/8");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "archive" => archive(),
        "state" => state(),
        _ => panic!("mode"),
    }
}
