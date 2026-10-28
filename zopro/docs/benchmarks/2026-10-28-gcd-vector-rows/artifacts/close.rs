use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/gcd-vector-rows";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-gcd-vector-rows";
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
    assert_eq!(a["captures"], 566);
    assert_eq!(a["gates"]["prototype_gate_passed"], true);
    assert_eq!(
        read(root.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        578
    );
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
    }
    let audit = read(root.join("source-audit.json"));
    let original = fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/inverse.rs")).unwrap();
    let ordinary = fs::read_to_string(root.join("ordinary.rs")).unwrap();
    assert_eq!(
        ordinary
            .strip_suffix(audit["reference_test_suffix"].as_str().unwrap())
            .unwrap(),
        original
    );
    let selected = fs::read_to_string(root.join("selected.rs")).unwrap();
    assert_eq!(
        selected.replace(
            audit["new"].as_str().unwrap(),
            audit["old"].as_str().unwrap()
        ),
        original
    );
    assert_eq!(
        hash(root.join("portable.rs")),
        hash(format!("{Z}/crates/zopro-bn254/src/portable.rs"))
    );
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let mut report=String::from("# Paired SIMD integer rows in complete GCD inversions\n\n**The standalone prototype passes its complete-inversion screen.** At 32 logical CPUs, Fq selected/fresh paired median is 0.985412381, bounds [0.984074626,0.986847015], 31/31 wins; selected/frozen-library is 0.992434843, bounds [0.990014093,0.993630494], also 31/31. All six calibrations/nonregression contexts pass. This permits actual point-caller integration; it is not a prover speedup or adoption.\n\nThe candidate computes both full four-word, 31-bit GCD integer rows using four AVX-512 VPMULUDQ products. It shares conversion of A/B into eight radix-32 digits between rows. Multiplication by a coefficient's unsigned low32 bits and a masked subtraction of `digit << 32` for negative coefficients recovers the signed result modulo 2^64, including ±2^31. The existing row L1 bound keeps each signed column inside i64. Scalar radix-32 carries reconstruct both signed five-word values, followed by the original absolute-value/row-sign normalization and 31-bit shift.\n\nWith B=2^31, column bounds are [−(2^32−1)B,(2^32−1)B]. Adding an incoming carry in [−B,B−1] stays in i64, and the next radix-32 carry remains in that same interval. Only integer rows use this bound: modular coefficient updates can have 62-bit coefficients and remain unchanged. Matrices, balanced halving loop, Euclid finish, coefficient updates and final Montgomery correction retain accepted code. No active-word thresholds, lookup table, heap allocation, key preparation, assembly backend or public option is added. Rust intrinsics use bounded unaligned 32-byte loads and 64-byte stores in the standalone helper; no production unsafe code is enabled here.\n\nEarlier packed30/31 and SSE2 trials changed the small matrix-coefficient loop. This changes the subsequent full-integer products and is separate from those trials and the rejected active-word update. The [Rust multiply intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_mul_epu32.html) uses unsigned low32 inputs and 64-bit results; the [conversion intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_cvtepu32_epi64.html) zero-extends the packed digits. These semantics are not performance predictions.\n\n## Complete inverse measurements\n\nOne native executable contains a frozen accepted-library control, a copied ordinary routine, and the paired routine. The external prototype copies accepted inverse.rs/portable.rs exactly, with only the declared pair-update replacement and test-only reference wrappers. Production's 172 files and core are unchanged. All field multiplication and other operations link the exact immutable accepted SDK field library, SHA f1285944f3fc44982915821769ac1c646c3cb8c2f3c67003dc95afcda0ce8a04. This is library/source-level primitive calibration, not the accepted proof executable or an exact-SDK machine-code equivalence claim.\n\n31 fixed rotated/reversed blocks cover Fq/Fr at solo [2], physical16 [0–15], logical32 [0–31] and three modes: 558 fresh processes. Each worker runs three checked one-pass primes over 4,096 deterministic nonzero field inputs, then 16 complete passes (65,536 inversions). Independent Arkworks references are prepared outside clocks. Worker output buffers are private; input/reference buffers are shared read-only. Start/done/release barriers prevent checking or allocation from overlapping any timed worker. Main ns/inverse-wave includes the slowest worker and barrier tail; per-worker clocks/CPU and aggregate throughput are retained. Input reads, inverse calls and output writes are timed. No eight-lane batch-inverse substitution is made.\n\n| Field | Layout | Frozen ns/wave | Fresh ns/wave | Paired ns/wave | Paired/fresh median | Marginal bounds | Wins |\n|---|---|---:|---:|---:|---:|---|---:|\n");
    for field in ["fq", "fr"] {
        for t in ["solo", "physical16", "logical32"] {
            let p = &a["pairs"][format!("{field}/{t}/2-over-1")];
            writeln!(
                report,
                "| {field} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9}, {:.9}] | {}/31 |",
                a["cells"][format!("{field}/{t}/m0")]["wall_ns_per_inverse_wave"]["median"]
                    .as_f64()
                    .unwrap(),
                a["cells"][format!("{field}/{t}/m1")]["wall_ns_per_inverse_wave"]["median"]
                    .as_f64()
                    .unwrap(),
                a["cells"][format!("{field}/{t}/m2")]["wall_ns_per_inverse_wave"]["median"]
                    .as_f64()
                    .unwrap(),
                p["median"].as_f64().unwrap(),
                p["low"].as_f64().unwrap(),
                p["high"].as_f64().unwrap(),
                p["wins"]
            )
            .unwrap();
        }
    }
    report.push_str("\n| Field/layout | Fresh/frozen paired median | Marginal bounds | Selected/frozen paired median | Marginal bounds |\n|---|---:|---|---:|---|\n");
    for field in ["fq", "fr"] {
        for t in ["solo", "physical16", "logical32"] {
            let f = &a["pairs"][format!("{field}/{t}/1-over-0")];
            let p = &a["pairs"][format!("{field}/{t}/2-over-0")];
            writeln!(
                report,
                "| {field}/{t} | {:.9} | [{:.9}, {:.9}] | {:.9} | [{:.9}, {:.9}] |",
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
    report.push_str("\nBounds are ranks 10–22 of 31 paired process ratios, nominal 97.055063% marginal coverage under independent exchangeable blocks, without familywise guarantees. Fresh/frozen bounds must fit [1/1.03,1.03] in all contexts. Fq/logical32 must have selected upper bounds below parity and at least 21/31 wins against both controls; every selected upper bound must be at most 1.02. All checks pass. No gain floor, exclusions, schedule extensions or successful-capture repeats are used. Fresh/frozen differences remain visible, including a roughly 0.66% slower fresh Fq/logical32 control; selected beats both. The small positive solo/physical gains are retained without equating them to proof gains.\n\n## Correctness and captured code\n\nChecked-overflow/debug and optimized suites both pass 10 tests: three existing full-inverse/one-word tests for each copied algorithm, two new pair/carry tests and two inherited perf-helper tests. The new pair test compares 65,616 input pairs (131,232 rows) against the original four-word routine and an independent unsigned five-word magnitude oracle. It covers every bit width 1–256, arbitrary bounded rows with divisible inputs, zero, cancellation, full-word and pure ±2^31 cases. Zero is explicitly excluded from negative-sign normalization. Repeated builds are not additional distinct cases. Existing 10,000-random-input-per-field inversion tests retain their Fermat/product checks and raw-word/one-word boundaries.\n\nRead-only disassembly confirms four VPMULUDQ sites in each selected Fq/Fr inverse and zero in the copied ordinary inverses; all six repeated-call functions are also recorded. Static counts do not establish performance causes. Native tests and every capture succeed on their first attempts. There is no Cargo/core/field-library rebuild, source/ISA/gate change or timing retry. All available rlibs/proc-macro objects in the two link dependency directories are hashed after the standalone build and before capture, including unused files; this supplements the direct build-input guards and does not claim every available library was used.\n\nAll **566** latency/stat/profile processes check complete outputs against Arkworks and `x * inverse = 1`, with matching input/output digests and zero global timed allocations. They perform **731,512,832 timed inverse calls**, not SDK/Gnark proofs. The helper's two eight-i64 digit buffers and two five-u64 reconstructed rows are fixed local storage; no measured stack footprint, proof RSS or minimum RAM follows from those source arrays.\n\n## Counters and profiles\n\nAll 558 unprofiled captures finish before four separate FIFO-gated grouped perf-stat captures, then four separate 199 Hz cycles:u profiles. These use logical32 and 128 passes/worker (524,288 inversions), after three one-pass primes. All eight finish before decoding, hashing, compression or archive. No owned helper/build/hash/archive/other benchmark overlaps capture. Host/compiler/topology snapshots are retained; no exclusive-host claim is made.\n\n| Field/mode | Cycles/inverse | Instructions/inverse | IPC | Branch misses/inverse | Generic cache misses/inverse |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["stat"].as_object().unwrap() {
        for e in v["events"].as_object().unwrap().values() {
            assert_eq!(e["running_percent"], 100.);
        }
        writeln!(
            report,
            "| {k} | {:.3} | {:.3} | {:.6} | {:.6} | {:.6} |",
            v["events"]["cycles:u"]["per_inverse"].as_f64().unwrap(),
            v["events"]["instructions:u"]["per_inverse"]
                .as_f64()
                .unwrap(),
            v["IPC"].as_f64().unwrap(),
            v["events"]["branch-misses:u"]["per_inverse"]
                .as_f64()
                .unwrap(),
            v["events"]["cache-misses:u"]["per_inverse"]
                .as_f64()
                .unwrap()
        )
        .unwrap();
    }
    report.push_str("\nAll four events run at 100% in every group. Counts sum logical-thread events and divide by all inverse calls; SMT cycles are not disjoint physical-core cycles. Single stat observations show about 2.49% fewer Fq instructions and 2.48% fewer Fr instructions, with cycles lower about 1.15%/1.99%. IPC is lower despite lower cycles, so IPC alone is not the outcome. These observations do not identify an exclusive instruction, execution-port, cache or DRAM cause.\n\n| Profile | Samples | Sampled worker TIDs | PIDs | Lost |\n|---|---:|---:|---:|---:|\n");
    let mut samples = 0u64;
    for (k, v) in a["profiles"].as_object().unwrap() {
        samples += v["samples"].as_u64().unwrap();
        assert_eq!(v["sampled_worker_tids"].as_array().unwrap().len(), 32);
        writeln!(
            report,
            "| {k} | {} | 32 | {} | 0 |",
            v["samples"],
            v["pids"].as_array().unwrap().len()
        )
        .unwrap();
    }
    writeln!(report,"\nTotal samples: {samples}. Each profile has one PID, all 32 worker TIDs sampled and zero lost records. Full symbols and decodes are archived; period shares are not removable proof wall time. Raw perf.data remains local with hashes, and loss dumps are archived losslessly with deterministic gzip.\n").unwrap();
    report.push_str("## Inherited proof comparisons and next step\n\nNo proof measurement changes here. The all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source observations and supplied M5 references are inherited. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    report.push_str("\nHistorical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof is generated; ledger remains 13,082. No new actual point caller, all-eight/54, native prover, portable/mobile/TEE, pressure, giant-circuit or AWS/Aeglos result is claimed. No cloud resource is created.\n\nNext: port the exact paired helper behind native guards into a private accepted-source snapshot, independently validate it, then compare complete concurrent point callers. SIMD root work can interact with the surrounding IFMA arithmetic differently from this inverse-only load. Only a passing caller study permits pure full-SDK/all-shape/resource qualification before adoption. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [code](artifacts/codegen.json), [field-library provenance](artifacts/field-library.json) and raw logs are included.\n");
    save(
        root.join("closeout.json"),
        &json!({"prototype_gate_passed":true,"actual_point_caller_gain_proven":false,"prover_gain_proven":false,"adopted":false,"production172_unchanged":true,"checked_tests":10,"optimized_tests":10,"new_distinct_pair_cases":65616,"new_distinct_row_cases":131232,"captures":566,"timed_inverse_calls":731512832,"profile_samples":samples,"all_counters_100_percent_running":true,"timed_allocations":0,"final_pins_verified":pins.as_object().unwrap().len(),"new_Gnark_proofs":0,"proof_ledger":13082,"next":"Private guarded native integration and actualconcurrentpointcallers; fullSDK onlyifpassed"}),
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
        "Archived{}hashes/{}files;vectorrowsprototypePASSnotadopted",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let summary = read(format!("{R}/archive-summary.json"));
    for(k,v)in[("status",json!("Paired SIMD GCD rows passed complete-inverse prototype gate; actualpointcaller/nativeintegration pending; committed/pushed/verified; no adoption")),("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("production_changed",json!(false)),("adopted",json!(false)),("last_goal_turn_classification",json!("PROGRESS")),("SDK_requests",json!(0)),("new_independent_exports",json!(0)),("independent_proof_count",json!(13082)),("artifact_hashes",summary["hashes"].clone()),("git_files_new_report",summary["git_files"].clone()),("component",read(format!("{R}/analysis.json"))),("vector_rows_prototype",read(format!("{R}/closeout.json"))),("previous_completed_experiment",previous),("next_action",json!("Port exact SIMD integerpair helper undernativeguards,validateandtestconcurrentpointcallers beforeSDK")),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([]))]{s[k]=v;}
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — PAIRED SIMD GCD INTEGER ROWS PROTOTYPE\n\nHEAD/remote verified{commit};root{R};report{D}/README.md.{}strictverifiedhashes/{}Gitfiles,4lossgzroundtripSHAverified. LastgoalturnPROGRESS:distinctSIMDfullintegerrowprototypepassedcompleteinverseconcurrentgate. NOactualpointcaller/provergain/adoptionyet. Accepted172/SDK14da unchanged,local1x2historical8.324853ms/RSS134.50MiB,goalACTIVEunmet0/8,ledger13082unchanged,noownedjobs/cloud.\n\nStandaloneRustprobecopiesEXACTacceptedportable.rs/inverse.rs ordinary+selected;ordinaryhascfgtestreferencewrappersONLY;selectedreplaces2fullintegerupdateswithpaired::update;inverseauditrecoversentireoriginal. No Cargo/core/field-librarybuild. LinkexactacceptedSDKfieldrlibmixed-sdk/target-control/release/deps/libzopro_bn254-505b170cdc507df1.rlibSHAf1285944f3fc44982915821769ac1c646c3cb8c2f3c67003dc95afcda0ce8a04;privateportablecopyexistingnativeflagsinclbalancedGCD. No rejectedactivewords/finish/square/weightedsourceinherited.\n\nNewpaired.rs uses4VPMULUDQforBOTH31bitintegerrowsaftershared4u64->8radix32cvts;unsignedlow32coeffproductsminusmasked(digit<<32)fornegativecoeffs;col|C|<2^63;incomingcarry[-2^31,2^31-1]preservesi64bounds. Bothrowsinterleavedscalar32carryingthenEXACTold5wordabs/rowsign/31shift. Matrices/modular31-62updates/Euclidfinish/finalscaleunchanged. Two128B? actualsource2x8i64=128Bdigitbuffers+2x5u64=80Breconstructedrows,fixedlocalsnotactualstackbound. NewunsafeONLYbounded32Bload/64BstoreRustintrinsics;nofullproductionnativeactivationyet. PriorSSE2/packed30/31smallmatrixtrialsdifferent;notreopened.\n\nNative91643TERMINALexit0;10checkedoverflow/debugand10optimizedtestsPASSONCE(ordinary3inverse/finish+selected3+paired2new+perf2). New65,616pairs131,232rows testEVERYbits1..256,32768arbitraryboundedrowsinputsdivisible2^31,±2^31/zero/cancel/fullwordcorners,old4word+independentunsigned5wordoracle. Zeromagnitudeexplicitlynotnegative. Existing10kfullinverseperfieldFermat/powers/finishrunbothalgorithms. NativetestsonlycopiedalgorithmsNOTnew33field/core suites. Codegenread-only10symbols6calculate+2ordinary+2selected;4VPMULUDQsitesperselectedinverse(8total),0ordinary.\n\nCapture12551TERMINALexit0;31blocksFq/Fr xsolo/physical16/logical32 x3modes558latency. Mode0frozenfieldAPI,1copiedordinary,2pairedsameimage.4096nonzerodenseMontgomeryinputsperfield,Arkworksreferencesoutsideclock;3primepasses,16timedpasses=65536inverses/worker. Independentoutputs/sharedreadonlyinputs,ready/start/done/releasebarriersALLpostchecksallocsoutsideALLclocks. Then4FIFOgroupstatlogical32Fq/Fr1/2,128passes/worker;then4FIFO199Hzcycles samecases/count;then12owner/decode/loss578steps566captures. ALLArkworks/productidentity/digestsandzeroGLOBALtimedallocs.731512832timedinversecallsNOTproofs.484finalpinsverifiedinclallavailable(usedANDunused)rlibs/soin2depdirsAFTERstandalonebuildBEFOREcapture. Noownedhelper/build/hash/archiveoverlap. Allfirstattempt/no repairs/retries.\n\nGateALLPASS. Fqlogical32selected/fresh.985412381CI[.984074626,.986847015]31/31;selected/frozen.992434843[.990014093,.993630494]31/31. Fresh/frozen1.006606CI[1.003225,1.008825]retained,nosourcemachinecodeequivalenceclaim. Fqsolo.993322selected/fresh,physical16.995702;Frlogical32.978746557[.976717319,.981533385]31/31,Frphysical16.989261,solo.986556. All6nonreg/calibPASS. Fq32ns816.880frozen/821.826fresh/809.700paired;Fr823.159/826.387/809.122. StatFqinst8115.86->7914.01(-2.49%),cycles4038.815->3992.312(-1.15%);Frinst8131.70->7930.01(-2.48%),cycles4070.814->3989.949(-1.99%);IPCdropsdespiteimprovement,notcausalport/cacheproof.4profiles10004samplesall32workerTIDsonePIDzeroLost;notproverwalltime.\n\nNEXTNEWROOTgcd-vector-callers:copyaccepted172ONLY;nativeinverse::pairedmoduleunderprivatecfgzopro_gcd_vector_rows ANDx86AVX512F+AVX2+not(no_ifma),fallbackoldverbatim. PortEXACTruntimepaired.rshelper;test-onlyreferencesadaptcrate::ordinary toparentinverse refs;appendoriginalprototypeordinarytestreferencewrapperscfgtest. Noactivewordsorfieldfinishchanges. Need35nativefieldtestsOFF/ON+checkedON(33accepted+2pairtests),122coreOFF/ONoriginalCalleradapters;freezealllibs/soimmediateindependenttargets. Reusegcd-active-wordsnative/driver/harnessbutREPLACEactivewordsmethod/flags/counts,neverinheritrejectedruntime. ActualpointcallerG1borrow200/352,owned200,G2200 xsolo/16/32independentfrozen/freshOFF/ONfixedprospectivegate. OnlypositiveactualcallerpermitsPUREfullSDK/all8/54portable/resourcequalification;primitive1.46%doesNOTpredictproofgain,expectsmallifanysinceonlyinversionrootportionchanged. No profile/fieldcfg/scheduling/layoutsweeps. No newSDK/Gnark/AWS/portable/pressure/mobile/TEE/giantmeasurements. KeepFFTengineeringexceptionfalsegatesandgoalactive0/8. CommitANDpushclosedexperimentsinclnegative,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Olderheadershistorical.\n\n",summary["hashes"],summary["git_files"]);
    fs::write(hp, format!("{header}{old}")).unwrap();
    println!("State/handoffupdated;goalactive0/8;vectorprototypepassawaitingpointcallers");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
