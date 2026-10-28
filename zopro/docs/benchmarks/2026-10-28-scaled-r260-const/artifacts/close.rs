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
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-const";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-scaled-r260-const";
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
    assert_eq!(a["timed_kernel_calls"], 179101696);
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
    }
    let pre = read(format!("{R}/control-preflight.json"));
    assert_eq!(pre["runtime_sections_equivalent"], true);
    assert_eq!(
        hash(format!("{R}/off-probe")),
        pre["off_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(format!("{R}/frozen-probe")),
        pre["frozen_sha256"].as_str().unwrap()
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
        assert_eq!(p["sampled_worker_tids"].as_array().unwrap().len(), 32);
        assert_eq!(p["pids"].as_array().unwrap().len(), 1);
        assert_eq!(p["lost"], 0);
        samples += p["samples"].as_u64().unwrap();
    }
    save(
        format!("{R}/interpretation.json"),
        &json!({"calibration_failures":bad,"nonregression_failures":regressions,"control_executables_byte_identical":false,"control_code_and_constants_byte_identical":true,"all_counters_100_percent_running":running,"profile_samples":samples,"all_profiles_all32_workers":true,"source_originals_exact":168,"source_reverse_recovers_all172":true,"production_changed":false,"proof_ledger":13181,"goal_status":"active"}),
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
        "passes the declared complete native caller screen; full SDK qualification remains required"
    } else {
        "fails the declared complete native caller screen and is not adopted"
    };
    let x = &a["logical32_aggregate"]["on-over-off"];
    let f = &a["logical32_aggregate"]["on-over-frozen"];
    let mut s=format!("# Compile-time modulus constants in scaled G2 batches\n\nThe candidate **{verdict}**. Primary G2/200 at32 logical CPUs has ON/OFF ratio **{:.9}**, marginal interval [{:.9},{:.9}], {}/31 wins; ON/frozen **{:.9}**, interval [{:.9},{:.9}], {}/31 wins. Production and the accepted SDK remain unchanged.\n\n## Arithmetic and storage\n\nRequire explicit const evaluation at three fixed-modulus sites in the validated parent native implementation. The arithmetic remains actual AVX512 IFMA using (x,y) → (x/4,y/8), uniform Montgomery R=2^260=16H and host radix H=2^256. Canonical host raw coordinates import as4a/2a; outputs use modular quarter/half followed by canonicalization. Keep the original two-fused-product complex formulas. Wider temporaries survive several linear operations instead of repeated generic corrections: imported X4p/Y2p, dx8p/dy4p, norm3p, prefix/slope2p, square4p, Xout12p, finishing difference16p and Yout4p. Bounds are inclusive in the native types; strict product inequalities follow from64p<R.\n\nA separate bounded child module calls the existing private uniform CIOS implementation for the proven wider domains. Public get/compact/difference2 contracts and original field runtime stay unchanged. Native codecs, five-product central inverse tree, host/root conversion, packing and scattering are inside the caller. No persistent recoded key, lookup table or second key plan is created. Identity/equal-x batches take a complete scalar fallback before vector mutation. Existing distinct queued-index contract remains; incomplete8lane groups use the original path.\n\nThe typed prefix Vec has640B per8point record, the same active record size as before. A24B optional Vec field is added to experimental scratch. Old and new prefix capacities can coexist after mixed complete/tail calls; this is a real memory cost. Ordinary caller capacity includes the new vector; no unchanged-peak-RSS/minimum-RAM or universal zero-overhead claim is made.\n\nA private accepted172-source copy adds only cfg-only dispatch/scratch/module declarations and diagnostic capacity accounting.168 originals remain byte-exact; reversing edits recovers all172. The complete accepted OFF prefix body remains verbatim. Frozen and fresh OFF code and constants are checked byte-identical before timing; only build ID and20 verified panic source-line fields differ, addressing the prior canonical-Xsum control setup error. This is a distinct wider-arithmetic candidate; rejected Gauss/GCD/GLV/weighted/square/FFT candidate runtime is absent.\n\n## Native correctness\n\nThe full field suite passes33 tests with debug and overflow checking. Inherited unchanged OFF qualification has122 core tests; the changed ON build passes124 core tests:119 accepted tests,3 original point-adapter tests, plus2 ON point tests. New native point coverage checks2560 full-width-generator outputs at sizes8/16/32/64/200 over8 changing rounds, scratch reuse, four whole-call exceptional batches and an empty case. Inherited tests cover original tails and FAST true/false. Optional external-fixture cases that return early are not claimed.\n\nThree separate tests against the actual frozen ON field library compare42,885 wider-domain CIOS scalar tuples with independent BigUint full-radix REDC, check2076 canonical raw codec inputs for both X/Y, and16,462 wider export/alias cases including inclusive bounds. Padding into8lane groups repeats inputs and is not extra distinct coverage. This upgrades the scalar model to actual native instruction validation. Complete complex/linear chains are also exercised by the native point oracles.\n\nInitial generation fails before setup/build because a replacement matches both link and capture step declarations. Anchoring to the link variant loop repairs only generation. The original generator/partial outputs are preserved. Parent-only setup/numeric/preflight failures are documented in its report; they are not new failures here. The narrow20source-line/build-ID metadata allowance is declared before this follow-up, without changing performance gates. Native arithmetic is unchanged apart from compile-time evaluation. No successful tests or captures are repeated for bookkeeping.\n\n## Caller timing\n\n31 fixed rotated/reversed blocks, G2prefix32/200 and unchangedG1borrow200, soloCPU2/physical16CPUs0–15/logical32CPUs0–31 on Ryzen9950X. Three reset checked1024call primes precede8192timedcalls/worker. Ready/start/done/release barriers put checking/allocation outside every worker's clock; wave time includes slowest worker and barrier tail. These synthetic ordinary points at real stream sizes are point-kernel timings, not full proofs or whole MSMs.\n\n| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF | Marginal interval | Wins |\n|---|---|---:|---:|---:|---:|---|---:|\n",x["median"].as_f64().unwrap(),x["low"].as_f64().unwrap(),x["high"].as_f64().unwrap(),x["wins"],f["median"].as_f64().unwrap(),f["low"].as_f64().unwrap(),f["high"].as_f64().unwrap(),f["wins"]);
    for (kind, n) in [
        ("g2-prefix", 32),
        ("g2-prefix", 200),
        ("g1-borrowed-signed", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let v = |m| {
                a["cells"][format!("{kind}/{n}/{t}/m{m}")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.
            };
            let p = &a["pairs"][format!("{kind}/{n}/{t}/2-over-1")];
            writeln!(
                s,
                "| {kind}/{n} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9},{:.9}] | {}/31 |",
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
                "| {kind}/{n}/{t} | {:.9} | [{:.9},{:.9}] | {:.9} | [{:.9},{:.9}] |",
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
    writeln!(s,"\nCalibration **{}**; nonregression **{}**; primary gain **{}**. Calibration failures: {}. Nonregression failures: {}.\n",a["gates"]["calibration"],a["gates"]["nonregression"],a["gates"]["logical32_gain"],json!(bad),json!(regressions)).unwrap();
    s.push_str("All9OFF/frozen intervals must fit[1/1.03,1.03], all18 ON/frozen and ON/OFF upper ratios<=1.02. The primary G2/200 logical32 gain requires upper<1 and>=21/31 wins against both controls; legacy logical32_aggregate contains only this primary caller. Ordered ranks10–22 of31 give nominal97.055% marginal coverage under independent exchangeable blocks, not familywise. No gain floor, exclusions, extension or unchanged reroll. All positive estimates and uncertainty remain within their measured scope. Identical control code and constants exclude different compiled arithmetic as a control gap; host variation, barrier tails and process placement/address layout remain unresolved.\n\n## Counters, profiles and provenance\n\nAll837 latency processes complete before4FIFO grouped perf-stat, then4 separate FIFO199Hz cycles:u profiles(G2/200,G1/200 OFF/ON logical32,262144calls/worker), then12owner/decode/loss steps.845kernel captures/857workflow steps verify complete generator results, capacity consistency and zero global timed allocations.179,101,696timed kernel calls are not proofs. No owned helper/build/hash/archive/other benchmark overlaps captures; host snapshots do not prove exclusive use.\n\n| Counter case | Cycles/call | Instructions/call | IPC |\n|---|---:|---:|---:|\n");
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
    writeln!(s,"\nAll events100% running: **{running}**. Counter observations do not identify a memory level, execution-port cause or removable proof latency. SMT cycles sum logical-thread events, not disjoint physical cores.\n\n| Profile | Samples | Sampled worker TIDs | Lost |\n|---|---:|---:|---:|").unwrap();
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
    writeln!(s,"\nTotal{samples} samples, each profile one PID/all32worker TIDs/zero lost. Actual original and scaled apply disassembly is archived; static counts are not executed work, dynamic spills or stack bounds. Every transitive runtime library/proc macro is frozen and flags/features checked. Executables/raw profiles stay local with hashes;4loss dumps use deterministic gzip and verified byte-exact decompression. Captured logs and measured sources retain bytes, including trailing blank lines; authored report/helper whitespace is checked separately.\n").unwrap();
    if pass {
        s.push_str("The passed native caller screen permits a separate pure full SDK trial. No adoption or full-proof speedup follows from this screen alone.\n\n");
    } else {
        s.push_str("This exact native candidate is closed without a full SDK trial or adoption. No unchanged arithmetic/layout reroll is selected.\n\n");
    }
    s.push_str("## Inherited CPU and Metal comparison\n\nNo new full-proof latency or peak RSS is established. Accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU10x objective remains active and unmet,0/8. These accepted local timings and supplied M5 references are inherited.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    s.push_str("\nHistorical primary peak134.50MiB; merge54 remains161.341835ms /1910.1875MiB, with no supplied M5 reference. No new SDK/Gnark proof, portable/mobile/TEE, concurrent-proof, pressure/giant-circuit or AWS/Aeglos measurement is claimed. Independent proofledger remains13,181.\n\n[Design](artifacts/design.md), [bounded field code](artifacts/scaled.rs), [point kernel](artifacts/g2_scaled.rs), [numeric oracle](artifacts/wide-check.rs), [analysis](artifacts/analysis.json), [native suites](artifacts/native/result.json), [raw native checks](artifacts/numeric-linked/wide-tests/stdout.log), [all captures](artifacts/runs/result.json), [source audit](artifacts/source-audit.json), [control preflight](artifacts/control-preflight.json).\n");
    s.push_str(&fs::read_to_string(format!("{R}/followup-scope.md")).unwrap());
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
    println!("Archived{}verifiedhashes;gate={pass}", all.len());
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
        "Constant-only scaledR260 caller gate passed; SDK pending; pushed verified"
    } else {
        "Constant-only scaledR260 caller gate failed; closed/not adopted; pushed verified"
    });
    v["root"] = json!(R);
    v["report"] = json!(format!("{D}/README.md"));
    v["component"] = a.clone();
    v["control_source_deviation_historical"] = v["control_source_deviation"].clone();
    v["control_source_deviation"] = Value::Null;
    v["control_runtime_audit"] = read(format!("{R}/control-preflight.json"));
    v["last_goal_turn_classification"] = json!("progress");
    v["next_action"] = json!(if pass {
        "Pure full SDK qualification of exact native scaled R260 with accepted/fresh controls and new independent proof exports"
    } else {
        "Use the actual constant-only profiles to select distinct operation-count work removal; no unchanged scaledR260 reroll"
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
    let h=format!("CURRENT COMPLETE/PUSHED — CONSTANT-ONLY COORDINATE-SCALED R260\n\nHEAD/remoteverified{head}; root{R}; report{D}/README.md. Caller gate={pass};notadopted/noSDKproofgain. Primaryratios{};gates{}. All172 production unchanged/SDK14da/historical1x2 8.324853ms/RSS134.50MiB;goalACTIVE0/8;proofledger13181. Noownedjobs/cloud. PreviousgoalturnPROGRESS published990fd7b60model+40c089de9canonicalXsum/c1fb38e8acontrolcaveat;currentPROGRESS nativeboundedfield/fullpointcaller.\n\nExactly3constcontexts replace runtimebuilder (parentprofile8.596%period); parentkernel/numericbytesunchanged. Nativecoordinate(x/4,y/8),R260;hostrawimport4a/2a,exportquarter/half;no persistentrecodedkey. ChildboundedWide APIcallsoriginalprivateCIOS provenwiderdomains;publiccontractsunchanged.168originalsexact/reverse172;cfg-onlyprefixearlydispatch originalOFFbodyverbatim. Frozen/OFFcode/constantsbyteidentical;only20verifiedpaniclines+2/buildid differ;preflightmetadataexception beforecapture/perfgatesunchanged. OptionaltypedprefixVec24Bnewfield,640B/8pointrecord;old/new capacities cancoexist aftermixedcomplete/tails,reportcost. Nativecheckedfield33PASS,inheritedOFF122/newON124PASS,raw3tests42885CIOS+2076bothcodecs+16462exports;new2560fullwidthG2outputs/fourwholeexceptionbatches/empty. Initialgeneration2-matchrepair preserved beforebuild;parentfailureshistorical only.31blocks9contexts3variants837latency+8PMU+12decode=857steps845captures179101696kernelcallsNOTproofs;alloracles/zeroGLOBALtimedalloc/capacityverified. AllprofilesonePID/all32workers/zeroLost;4lossgzroundtrip;rawperf/exeslocalhash. Noownedhelper/build/hash/archiveoverlapcapture. Fullanalysis/counters/profiles/pinsinreport.\n\nNEXT:{}. Preserveallpositiveestimates/uncertainty/failedgates;no unchangedreroll. Acceptedengineeringexceptionfalsefullgates unchanged. No newSDK/Gnark/RSS/stack/minRAM/mobile/TEE/pressure/giant/AWSclaims. CommitANDpush eachcompletedexperiment,noquestions/subagents/largecaches/Casmrewrite. Olderheadershistorical.\n\n",a["logical32_aggregate"],a["gates"],v["next_action"]);
    fs::write(handoff, format!("{h}{old}")).unwrap();
    println!("Verified push{head};state/handoff updated;goalactive0/8");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "archive" => archive(),
        "state" => state(),
        _ => panic!("mode"),
    }
}
