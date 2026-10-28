use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/gcd-vector-callers";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-gcd-vector-callers";
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
            if n.starts_with("target-")
                || ["source", "target", "profiles", "frozen"].contains(&n.as_str())
            {
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
    assert_eq!(a["captures"], 1124);
    assert_eq!(
        read(root.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        1136
    );
    let gate = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let audit = read(root.join("source-audit.json"));
    let mut inverse =
        fs::read_to_string(root.join("source/crates/zopro-bn254/src/inverse.rs")).unwrap();
    let suffix = audit["inverse_suffix"].as_str().unwrap();
    assert!(inverse.ends_with(suffix));
    inverse.truncate(inverse.len() - suffix.len());
    inverse = inverse.replace(
        audit["new"].as_str().unwrap(),
        audit["old"].as_str().unwrap(),
    );
    assert_eq!(
        inverse,
        fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/inverse.rs")).unwrap()
    );
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
        if ![
            "crates/zopro-bn254/src/inverse.rs",
            "crates/zopro-core/src/msm.rs",
            "crates/zopro-core/src/lib.rs",
        ]
        .contains(&f.as_str())
        {
            assert_eq!(hash(root.join("source").join(f)), h.as_str().unwrap());
        }
    }
    for (f, key) in [
        ("crates/zopro-core/src/msm.rs", "msm_suffix"),
        ("crates/zopro-core/src/lib.rs", "core_suffix"),
    ] {
        let s = fs::read_to_string(root.join("source").join(f)).unwrap();
        assert_eq!(
            s.strip_suffix(audit[key].as_str().unwrap()).unwrap(),
            fs::read_to_string(Path::new(Z).join(f)).unwrap()
        );
    }
    let paired =
        fs::read_to_string(root.join("source/crates/zopro-bn254/src/inverse_vector_rows.rs"))
            .unwrap();
    let prototype =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/gcd-vector-rows/paired.rs").unwrap();
    assert_eq!(
        paired.split_once("#[cfg(test)]").unwrap().0,
        prototype.split_once("#[cfg(test)]").unwrap().0
    );
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let status = if gate {
        "passes the concurrent actual-caller gate; full-SDK measurement is pending"
    } else {
        "fails the concurrent actual-caller gate and is not adopted"
    };
    let off = &a["logical32_aggregate"]["on-over-off"];
    let frozen = &a["logical32_aggregate"]["on-over-frozen"];
    let mut report=format!("# Paired SIMD GCD rows in concurrent point callers\n\nThe candidate **{status}**. Its 32-logical-CPU real-stream aggregate ON/OFF is **{:.9}**, bounds [{:.9},{:.9}], {}/31 wins. ON/frozen is **{:.9}**, bounds [{:.9},{:.9}], {}/31 wins. Calibration {}, nonregression {}, gain {}. Production and the accepted SDK remain unchanged.\n\nThis ports the exact runtime helper from the [complete-inverse prototype](../2026-10-28-gcd-vector-rows/README.md), which improved 32-thread Fq inversion by about 1.46% versus a copied ordinary routine and 0.76% versus the accepted field-library control. Four VPMULUDQ products compute both full-integer 31-bit GCD rows after shared radix-32 conversion. Masked corrections handle signed coefficients; scalar carries reconstruct the existing signed five-word values, then the original absolute-value, row-sign and 31-bit shift follow. Matrices, modular 31/62-bit coefficient updates, Euclid finish, scale and all surrounding point/IFMA arithmetic remain unchanged.\n\nThe new helper is selected only under a private cfg plus x86-64 AVX512F/AVX2 and not(no_ifma). The original two integer updates remain verbatim in the fallback. Test reference paths alone differ from the prototype; the runtime prefix is byte-exact. Bounded intrinsic loads/stores are the same as the prototype. There is no added heap buffer, point/key table, cached plan, assembly backend or public runtime option. Active-word dispatch and all rejected weighted/square/finish code are absent.\n\n## Complete caller measurements\n\n31 fixed rotated/reversed blocks cover G1 borrowed-signed batches 200/352, G2 prefix/200, and G1 owned/200 as a supplement. Each runs solo [2], physical16 [0–15], logical32 [0–31], with independent frozen accepted-library, fresh OFF and paired ON executables. All share the same harness source. Three checked/reset 1,024-call primes precede 8,192 complete calls per worker. Ready/start/done/release barriers prevent any worker's checking/allocation from overlapping any clock. Main ns/wave includes the slowest worker and barrier tail; per-worker elapsed/CPU and throughput remain in the raw results. These are synthetic ordinary points at real stream sizes, not witness/MSM/proof latency.\n\n| Caller | Pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF paired median | Marginal bounds | Wins |\n|---|---:|---|---:|---:|---:|---:|---|---:|\n",off["median"].as_f64().unwrap(),off["low"].as_f64().unwrap(),off["high"].as_f64().unwrap(),off["wins"],frozen["median"].as_f64().unwrap(),frozen["low"].as_f64().unwrap(),frozen["high"].as_f64().unwrap(),frozen["wins"],a["gates"]["calibration"],a["gates"]["nonregression"],a["gates"]["logical32_gain"]);
    for (kind, n) in [
        ("g1-owned", 200),
        ("g1-borrowed-signed", 200),
        ("g1-borrowed-signed", 352),
        ("g2-prefix", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let p = &a["pairs"][format!("{kind}/{n}/{t}/2-over-1")];
            writeln!(
                report,
                "| {kind} | {n} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9},{:.9}] | {}/31 |",
                a["cells"][format!("{kind}/{n}/{t}/m0")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                a["cells"][format!("{kind}/{n}/{t}/m1")]["wall_ns_per_wave"]["median"]
                    .as_f64()
                    .unwrap()
                    / 1000.,
                a["cells"][format!("{kind}/{n}/{t}/m2")]["wall_ns_per_wave"]["median"]
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
    report.push_str("\n| Caller/pairs/layout | OFF/frozen paired median | Marginal bounds | ON/frozen paired median | Marginal bounds |\n|---|---:|---|---:|---|\n");
    for (kind, n) in [
        ("g1-owned", 200),
        ("g1-borrowed-signed", 200),
        ("g1-borrowed-signed", 352),
        ("g2-prefix", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let f = &a["pairs"][format!("{kind}/{n}/{t}/1-over-0")];
            let p = &a["pairs"][format!("{kind}/{n}/{t}/2-over-0")];
            writeln!(
                report,
                "| {kind}/{n}/{t} | {:.9} | [{:.9},{:.9}] | {:.9} | [{:.9},{:.9}] |",
                f["median"].as_f64().unwrap(),
                f["low"].as_f64().unwrap(),
                f["high"].as_f64().unwrap(),
                p["median"].as_f64().unwrap(),
                p["low"].as_f64().unwrap(),
                p["high"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    report.push_str("\nThe native real-stream aggregate is 0.590% slower than fresh OFF and 0.857% slower than frozen, with both marginal intervals above parity. Every one of the twelve ON/OFF paired medians is above one. The owned/200 physical16 estimate is about 2.52% slower than OFF and fails the nonregression bound; other failed bounds are listed in result-interpretation.json. A small roughly 0.277% lower borrowed/352 solo estimate against frozen remains included, but it crosses parity and fresh OFF is already faster than frozen there.\n\nThis reversal also appears without SMT: borrowed/200 solo ON/OFF is about 0.619% slower, and G2/200 solo about 0.730% slower, with bounds above parity. SMT alone therefore cannot explain the integration result. The standalone prototype and native caller differ in surrounding computation, input distributions and compiled context; exact helper source does not establish equivalent whole-routine execution cost. The exclusive cause is unresolved. The positive inverse-only result is retained within its measured scope, while this native integration is closed without an SDK trial.\n");
    report.push_str("\nThe primary aggregate is the per-block geometric mean of borrowed G1/200, borrowed G1/352 and G2/200 at logical32. It requires upper bounds below parity and at least 21/31 wins against both controls. All twelve contexts require ON upper bounds at most 1.02; OFF/frozen calibration bounds must fit [1/1.03,1.03]. Ordered ranks 10–22 of 31 ratios give nominal 97.055063% marginal coverage under independent exchangeable blocks, without familywise guarantees. Owned G1 remains a nonregression supplement. All small positive estimates, uncertainty and failed checks remain visible; there is no positive-gain floor, exclusion, extension, successful-capture retry or unchanged reroll. Ratios of table medians need not equal paired medians.\n\n## Native validation and provenance\n\nThree full native field suites pass 35 tests each: checked active ON with overflow/debug checks, ordinary OFF, and active ON. They reuse the exact prototype's two pair/carry oracles (65,616 pairs / 131,232 rows) with only parent-reference paths adapted, plus the accepted 33 field tests. The rows include every bit length 1–256, arbitrary bounded rows, zero, cancellation, ±2^31 and full-word cases. Existing full inverse/Fermat/raw-power/one-word tests remain. Repeated cases across builds are not additional distinct inputs.\n\nBoth complete native core suites pass 122 tests, including the three original owned/borrowed G1/G2 generator adapters. Each frozen/OFF/ON harness passes three tests, including concurrency/affinity/barrier/output and inherited perf-helper checks. Some optional external-fixture core cases return without fixtures; those external scenarios are not claimed. There is no raw/cubic/SDK qualification at this stage.\n\nThe initial source-generator attempt stopped before native setup because it expected spaces around a retained long-string assignment. The preserved versioned fix changes that source-format match only. No native test/build/capture had run before it, and no successful command was repeated. A later link-plan preparer stopped before probe linking on inherited make-harness filenames; its preserved fix pins the actual make-native helpers. No native build/test or capture was repeated. The native/link/capture workflows themselves pass on their first attempts. An inherited console label says field34x3, while the enforced assertions and all three unfiltered field-suite logs correctly record 35; the original output is retained. The runtime helper matches the passed prototype exactly before the test module. Reversing the private inverse edit and removing diagnostic reexports recovers all 172 originals; 169 originals remain byte-exact. All transitive rlibs/proc-macro objects are frozen immediately in separate target roots. Native flags/features and distinct field/core hashes are checked, and read-only disassembly records actual inverse code inside the point-caller executables.\n\nAll **1,124** latency/stat/profile processes pass independent full generator output, digest and capacity checks with zero global timed allocations. They execute **216,432,640 timed kernel calls**, not SDK/Gnark proofs. Per-worker G1/200 scratch is 40,000 bytes, G1/352 70,400 and G2/200 16,000. Borrowed fixture reference vectors are created outside clocks. The helper's fixed local arrays are not a measured stack bound or proof-RSS/RAM-minimum claim.\n\n## Counters and profiles\n\nAll 1,116 latency captures finish before four FIFO-gated grouped perf-stat processes, followed by four separate 199 Hz cycles:u profiles. Each uses logical32 and 262,144 calls per worker after the same primes. All eight finish before decode/hash/compression/archive. No owned build/helper/hash/archive/other benchmark overlaps capture. Host/compiler/topology snapshots are archived without an exclusive-host claim.\n\n| Context/mode | Cycles/kernel call | Instructions/kernel call | IPC | Branch misses/call | Generic cache misses/call |\n|---|---:|---:|---:|---:|---:|\n");
    let mut all_running = true;
    for (k, v) in a["stat"].as_object().unwrap() {
        for e in v["events"].as_object().unwrap().values() {
            all_running &= e["running_percent"].as_f64().unwrap() == 100.;
        }
        writeln!(
            report,
            "| {k} | {:.3} | {:.3} | {:.6} | {:.6} | {:.6} |",
            v["events"]["cycles:u"]["per_kernel_call"].as_f64().unwrap(),
            v["events"]["instructions:u"]["per_kernel_call"]
                .as_f64()
                .unwrap(),
            v["IPC"].as_f64().unwrap(),
            v["events"]["branch-misses:u"]["per_kernel_call"]
                .as_f64()
                .unwrap(),
            v["events"]["cache-misses:u"]["per_kernel_call"]
                .as_f64()
                .unwrap()
        )
        .unwrap();
    }
    writeln!(report,"\nAll groups at 100% running: {all_running}. Counts sum logical-thread events and divide by all kernel calls; SMT cycles are not disjoint physical-core cycles. Each counter row is a single descriptive observation, not a confidence bound or exclusive cost attribution. IPC, sampled period and generic cache misses do not establish removable proof time, an execution-port cause, cache level or DRAM bandwidth.\n").unwrap();
    report.push_str(
        "| Profile | Samples | Sampled worker TIDs | PIDs | Lost |\n|---|---:|---:|---:|---:|\n",
    );
    let mut samples = 0u64;
    for (k, v) in a["profiles"].as_object().unwrap() {
        samples += v["samples"].as_u64().unwrap();
        writeln!(
            report,
            "| {k} | {} | {} | {} | {} |",
            v["samples"],
            v["sampled_worker_tids"].as_array().unwrap().len(),
            v["pids"].as_array().unwrap().len(),
            v["lost"]
        )
        .unwrap();
    }
    writeln!(report,"\nTotal samples: {samples}. Decodes and full symbol weights are archived. Raw perf.data stays local with hashes; loss dumps are archived losslessly using deterministic gzip.\n").unwrap();
    report.push_str("## Inherited full-proof comparison\n\nNo proof timing changes here. The all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source timings and supplied M5 references are inherited. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    report.push_str("\nHistorical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof is generated; ledger remains 13,082. No new all-eight/54, portable/mobile/TEE, whole-proof concurrency/pressure, giant-circuit or AWS/Aeglos result is claimed. No cloud resource is created.\n\n");
    if gate {
        report.push_str("Next: build pure libraries without caller adapters and compare accepted/fresh OFF/ON full-SDK latency at 32/16 workers. Only that result can justify all-eight/54 and portable/resource/concurrency qualification before adoption.\n");
    } else {
        report.push_str("The exact native integration is closed at this caller gate. Retain the complete-inverse gain and every point-caller positive estimate/failure; no unchanged arithmetic, annotation/layout or scheduling reroll follows.\n");
    }
    report.push_str("\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [runtime provenance](artifacts/runtime-provenance.json), [source audit](artifacts/source-audit.json), [actual code](artifacts/codegen.json), [native validation](artifacts/native/result.json) and raw logs are included.\n");
    save(
        root.join("closeout.json"),
        &json!({"component_gate_passed":gate,"adopted":false,"production172_unchanged":true,"prototype_runtime_exact":true,"native_field_tests_each":35,"native_field_configurations":3,"native_core_tests_each":122,"native_core_configurations":2,"captures":1124,"timed_kernel_calls":216432640,"profile_samples":samples,"timed_allocations":0,"final_pins_verified":pins.as_object().unwrap().len(),"all_counters_100_percent_running":all_running,"new_Gnark_proofs":0,"proof_ledger":13082,"next":if gate{"PureSDK accepted/OFF/ON then all8/54portable/resources if passed"}else{"Exactnativevectorintegrationclosed; no unchangedreroll"}}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    let mut compressed = BTreeMap::new();
    copy(root, root, &Path::new(D).join("artifacts"), &mut compressed);
    for f in [
        "crates/zopro-bn254/src/inverse.rs",
        "crates/zopro-bn254/src/inverse_vector_rows.rs",
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/msm/caller_probe.rs",
    ] {
        let q = Path::new(D).join("artifacts/private-source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(root.join("source").join(f), q).unwrap();
    }
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
        "Archived{}hashes/{}files;nativevectorcaller gate={gate};notadopted",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let a = read(format!("{R}/analysis.json"));
    let gate = a["gates"]["component_gate_passed"].as_bool().unwrap();
    let summary = read(format!("{R}/archive-summary.json"));
    let next = if gate {
        "PureSDK accepted/OFF/ON beforeall8/54portable/resourcesqualification"
    } else {
        "Nativepairedvectorintegrationclosed; preserveprototypegainandcallerfailures, choose distinct work removal without unchangedreroll"
    };
    for (k, v) in [
        (
            "status",
            json!(format!(
                "Paired SIMD GCD actualcaller gate={gate};committed/pushed/verified;notadopted"
            )),
        ),
        ("root", json!(R)),
        ("report", json!(format!("{D}/README.md"))),
        ("commit", json!(commit)),
        ("pushed", json!(true)),
        ("remote_verified", json!(true)),
        ("production_changed", json!(false)),
        ("adopted", json!(false)),
        ("last_goal_turn_classification", json!("PROGRESS")),
        ("SDK_requests", json!(0)),
        ("new_independent_exports", json!(0)),
        ("independent_proof_count", json!(13082)),
        ("artifact_hashes", summary["hashes"].clone()),
        ("git_files_new_report", summary["git_files"].clone()),
        ("component", a),
        ("vector_rows_callers", read(format!("{R}/closeout.json"))),
        ("previous_completed_experiment", previous),
        ("next_action", json!(next)),
        ("owned_live_jobs", json!([])),
        ("owned_cloud_resources", json!([])),
    ] {
        s[k] = v;
    }
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — PAIRED SIMD GCD ACTUAL CALLERS\n\nHEAD/remoteverified{commit};root{R};report{D}/README.md.{}strictverifiedhashes/{}Gitfiles,4lossgzroundtripSHAverified. Nativeactualcallergate={gate},NOTadopted/proofgain. Accepted172/SDK14da unchanged,local1x2historical8.324853ms/RSS134.50MiB,goalACTIVEunmet0/8,proofledger13082unchanged,noownedjobs/cloud. LastgoalturnPROGRESS:positiveprototypec1d8bf19d carriedthroughguardednativeintegration+actualconcurrentpointcallers.\n\nExactpairedruntimeprefixfromgcd-vector-rows;onlycfgtestreferencepathsadapted toparentinverse,and2test-onlyreferencehelpersappended. Newinverse_vector_rows.rs underx86AVX512F+AVX2+not(no_ifma),privatecfgzopro_gcd_vector_rows enablesdispatch;oldtwointegerupdatesVERBATIMfallback.4VPMULUDQforboth31bitGCDintegerrows,sharedradix32conversion/signcorrections,interleavedscalarcarry/oldabs/sign/31shift;matrixloop/modular31-62updates/Euclidfinish/scaleUNCHANGED. Noactivewords/finish/square/weightedruntime. NewunsafeONLYboundedRustnativeintrinsicsfrompassedprototype,noproductionenabled/heap/keytable/publicoption/Casmbackend. 169originalsexact+inverse/msm/libdiag changesreverseall172. Diagnosticcfgzopro_vector_rows_caller_probe exportsoriginalCalleradapters.\n\nNative32773TERMINAL;35fullfieldcheckedON/OFF/ON,122coreOFF/ON;3harnesstestsfrozen/OFF/ONallPASS.131232roworaclescasesreusedexactprototypeinputsNOTnewdistinctcount. Initialmake-native generatorv1 failedBEFOREnative setup becauseRustfmtlongstringassignmenthasnospaces;preservedv1source/bin+preparation-repair.json. FixedsourceformatmatchONLY,no successfulnativebuild/tests/capturesrepeated. SeparateOFF/ONtargetsallrlib/sofreezeimmediate;flags/features/hasheschecked,independentfrozenacceptedlibsfromdirect-ref-pack. Runtimeprefixexact+actualinversecodegenread-onlyverifiedbeforecapture.\n\n31blocks12contexts3variants1116latency8192calls/workerafter3x1024primes;G1borrow200/352,G2200,owned200supplement xsolo/physical16/logical32. ALLpostchecks/allocsoutsideALLworkerclocksbarriers;mainwaveandperworkerCPUrecorded. Then4FIFOgroupstatlogical32borrow352/G2200OFF/ON262144calls/worker;then4FIFO199Hzcycles same;then12owner/decode/loss1136steps1124captures. Allgenerator/digest/capacity/zeroGLOBALtimedallocchecks;216432640timedkernelcallsNOTproofs. Readanalysis.jsonforALLmedians/pairedCIs/gateflags/counters/profiles;NOgenericcache/IPCcausalclaim. No helper/build/hash/archiveoverlapcaptures. ProfileonePID/zeroLost;counterrunningpercentretained.\n\nNEXT:{next}. IfpassedSDKNEWpurecopyaccepted172+exactruntimeinverse/helperONLY,removeDIAGNOSTICmsm/libadapterexports/testreferencewrappersasappropriate(no runtimechange). Compareaccepted/freshOFF/ON32/16fullproof andindependentGnark;parentcallerbytesalonearenotproofgain. IfnegativeCLOSEexactintegration,nothreshold/annotation/layout/schedule reroll. latestfullSDKremainsweightednegativea0d72e62b;latestAWSstale17137261af77. PreserveFFTengineeringexceptionfalsegates,goalactive0/8,ledger13082. No newSDK/Gnark/all8/54/portable/pressure/stack/mobile/TEE/giant/AWSresults. CommitANDpushcompletedexperimentsinclnegative,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Olderheadershistorical.\n\n",summary["hashes"],summary["git_files"]);
    fs::write(hp, format!("{header}{old}")).unwrap();
    println!("State/handoffupdated;nativecallergate={gate};goalactive0/8");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
