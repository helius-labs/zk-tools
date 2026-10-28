fn archive() {
    assert!(!Path::new(D).exists());
    let root = Path::new(R);
    let a = read(root.join("analysis.json"));
    assert_eq!(a["captures"], 1124);
    assert_eq!(a["gates"]["component_gate_passed"], false);
    assert_eq!(
        read(root.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        1136
    );
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
    assert_eq!(inverse.matches(audit["new"].as_str().unwrap()).count(), 1);
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
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    let mut report=String::from("# Active-word GCD integer updates\n\n**Rejected at the concurrent actual-caller gate; no SDK trial or adoption.** The three real streaming contexts at 32 logical CPUs have ON/fresh-OFF paired aggregate 1.000239921, bounds [0.994370045,1.009596838], 14/31 wins. ON/frozen is 1.002958819, bounds [0.997131296,1.007339453], 12/31 wins. Calibration and nonregression pass, but the declared aggregate gain does not.\n\nThe candidate shortens only the full GCD integer updates after the existing bit-length test: two words through 128 bits, three through 192, otherwise four. A single width dispatch selects both matrix rows. Known-zero high words no longer participate in signed products/carries. The binary-GCD matrices, row-sign normalization, 31/62-bit modular coefficient updates, exact one-word Euclid finish and final Montgomery scale remain unchanged. This adds no heap buffer, lookup table, cached key/plan, assembly, unsafe operation or public runtime option.\n\nFor N-word A/B and row L1 norm at most 2^31, the signed combination's absolute magnitude is below 2^(64N+31). After the existing exact division by 2^31, it fits N words. Signed intermediates fit i128; two's-complement negation covers N+1 words and output upper words are zero. Modular coefficients remain four words. This differs from the earlier rejected signed62 safegcd algorithm, which changed the entire inversion method and also shrank integers. No previous inverse/weighted/square candidate source is inherited.\n\n## Complete caller measurements\n\nThe new concurrency harness is derived from the preceding [diagnostic](../2026-10-28-concurrent-callers/README.md), with the weighted setter removed. Three executables link independent immutable libraries: frozen accepted arithmetic from direct-ref-pack, fresh OFF, and active-word ON. Both fresh variants share identical private source; only the active-word compile cfg differs. No mutable Cargo-root library is reused.\n\n31 rotated/reversed blocks cover G1 owned/200, G1 borrowed-signed/200 and /352, and G2 prefix/200, on one thread [2], 16 physical cores [0–15] and 32 logical CPUs [0–31]. Every worker has independent fixtures/scratch, three checked/reset 1,024-call primes, then 8,192 timed complete calls. Main elapsed spans start to done barriers, including the slowest worker; per-worker elapsed, thread CPU, throughput and usage are preserved. Output checks and all allocations happen outside every worker's clock. These synthetic ordinary points use real stream batch sizes but are not witness/MSM/proof timing.\n\n| Caller | Pairs | Layout | Frozen µs/wave | Fresh OFF µs/wave | ON µs/wave | ON/OFF paired median | Marginal bounds | Wins |\n|---|---:|---|---:|---:|---:|---:|---|---:|\n");
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
                "| {kind} | {n} | {t} | {:.6} | {:.6} | {:.6} | {:.9} | [{:.9}, {:.9}] | {}/31 |",
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
    report.push_str("\n| Caller/pairs/layout | Fresh/frozen paired median | Marginal bounds | ON/frozen paired median | Marginal bounds |\n|---|---:|---|---:|---|\n");
    for (kind, n) in [
        ("g1-owned", 200),
        ("g1-borrowed-signed", 200),
        ("g1-borrowed-signed", 352),
        ("g2-prefix", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            let f = &a["pairs"][format!("{kind}/{n}/{t}/1-over-0")];
            let on = &a["pairs"][format!("{kind}/{n}/{t}/2-over-0")];
            writeln!(
                report,
                "| {kind}/{n}/{t} | {:.9} | [{:.9}, {:.9}] | {:.9} | [{:.9}, {:.9}] |",
                f["median"].as_f64().unwrap(),
                f["low"].as_f64().unwrap(),
                f["high"].as_f64().unwrap(),
                on["median"].as_f64().unwrap(),
                on["low"].as_f64().unwrap(),
                on["high"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    report.push_str("\nThe primary geometric aggregate includes borrowed G1/200, borrowed G1/352 and G2/200 at logical32. Owned G1 is a nonregression supplement. Aggregate upper bounds must be below parity with at least 21/31 wins against both controls; every context's ON upper bound must be at most 1.02, and fresh/frozen calibration must fit [1/1.03,1.03]. Bounds are ranks 10–22 of 31 paired ratios, nominal 97.055063% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, extension or successful-capture retry is used. All context calibrations and nonregressions pass; both primary gain comparisons fail.\n\nSmall improvements are retained: owned G1 solo ON/OFF is about 0.836% lower with bounds below parity; G2 solo is about 0.511% lower with bounds below parity. Borrowed solo estimates improve about 0.27–0.29% with bounds crossing parity. G2 logical32 has a positive roughly 0.259% estimate but also crosses parity. These are valid scoped observations, not zero gains or evidence of a qualified parallel prover gain. Several fresh/frozen controls differ despite unchanged baseline arithmetic; all controls and raw observations remain included.\n\n## Counters and profiles\n\nAll 1,116 unprofiled latency processes finish first, followed by four FIFO-gated grouped perf-stat processes, then four separate 199 Hz cycles:u profiles. Each uses logical32 and 262,144 calls per worker after the same primes. All eight finish before decode/hash/compression/archive work. No owned build/helper/hash/archive/other benchmark overlaps capture. Host/compiler/topology snapshots and pins are archived; no exclusive-host claim is made.\n\n| Context/mode | Cycles/kernel call | Instructions/kernel call | IPC | Branch misses/call | Generic cache misses/call |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["stat"].as_object().unwrap() {
        for e in v["events"].as_object().unwrap().values() {
            assert_eq!(e["running_percent"], 100.);
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
    report.push_str("\nAll four grouped events run at 100%. Counts sum logical-thread events and divide by all kernel calls; SMT cycles are not disjoint physical-core cycles. These are single counter processes, descriptive observations without confidence bounds. Retired instructions fall about 0.427% for G1 and 0.331% for G2, while cycles rise about 0.830% and 0.791% in these stat observations. Branch misses also rise. Fewer executed instructions alone do not establish throughput or latency improvement; these counters do not prove an exclusive branch, port or cache cause. Generic cache misses are neither cache-level attribution nor DRAM bandwidth.\n\n| Profile | Samples | Sampled worker TIDs | PIDs | Lost |\n|---|---:|---:|---:|---:|\n");
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
    writeln!(report,"\nTotal samples: {samples}. Every profile has one PID, all 32 active workers sampled and zero lost records. Full symbol weights and decodes are retained; sampled inverse/stream periods are not removable wall time. Raw perf.data remains local with hashes, and loss-record dumps are archived losslessly with deterministic gzip.\n").unwrap();
    report.push_str("## Correctness, provenance and memory\n\nAll 34 unfiltered native field tests pass in three configurations: ordinary OFF, active ON, and active ON with overflow/debug checks. The one new test performs 48,655 row comparisons at every bit length 65–254, including 128/192 boundaries, full-word corners and pure ±2^31 rows. It compares the original four-word method with a separate unsigned five-word magnitude oracle and the shortened method. The same cases rerun across builds are not additional distinct inputs. Existing 10,000 random full inverse tests per field, raw powers, canonical boundaries and one-word finishing checks retain their independent Fermat identity.\n\nBoth full native core suites pass 122 tests: 119 accepted cases plus three unchanged diagnostic caller oracles. Each of the three linked harness configurations also passes three tests, including the concurrency/affinity/barrier/output test and two inherited perf-helper tests. All tests/builds/captures succeed on their first attempts; no successful command is repeated. The skipped-word thresholds are exact machine-word boundaries, not tuned parameters.\n\nThe 172-file private snapshot has 169 exact originals, only inverse.rs arithmetic dispatch/module inclusion, and diagnostic-only core/MSM reexports. Reverse edits recover every original byte. G1/G2 arithmetic, coefficients and Euclid finish are unchanged. All transitive rlibs and proc-macro objects are frozen immediately after each separate target build; exact flags/features and independent frozen/fresh/candidate hashes are checked. No rejected weighted-anchor, direct-finish or symmetric-square kernel is included.\n\nAll **1,124** latency/stat/profile processes pass full independent generator output/digest/capacity checks and zero timed global allocations. They execute **216,432,640 timed kernel calls**, not SDK proofs. Per-worker scratch is unchanged: G1/200 40,000 bytes, G1/352 70,400, G2/200 16,000; borrowed reference vectors 3,200/5,632 bytes are created outside clocks. The helper has fixed small local arrays; no measured stack bound, proof peak RSS, RAM minimum or pressure benefit is established.\n\n## Research audit and inherited proof comparison\n\nThe reviewed [SQIsign paper, sections 3.1–3.7](https://arxiv.org/html/2608.13948v2), reorganizes SIMD dependency graphs and uses a Montgomery-friendly prime. Its scheme, parameters and reported speedups are not BN254 predictions. The [Cd monograph's introductory geometry and operation tables](https://arxiv.org/pdf/2608.01675v5) give an untwisted family with rational four-torsion. That family cannot directly represent the odd-order BN254 G1 group by a ground-field isomorphism; no curve-model port was selected. Only the stated sections were reviewed, not the entire monograph. The implemented active-word change follows the existing source's zero high words and does not copy either implementation. Earlier GLV/ring, SRE, memory-adaptive buckets, shared-complex products and inverse trials were audited and remain separate closed evidence.\n\nThe all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source timings and supplied M5 references are inherited, not remeasured here. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
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
    report.push_str("\nHistorical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof was generated; ledger remains 13,082. No new all-eight/54, portable/mobile/TEE, pressure, giant-circuit or AWS/Aeglos result is claimed. No cloud resource was created.\n\nThe exact active-word candidate is closed without SDK integration. Retain its solo gains and failed parallel aggregate; do not rerun the same thresholds, arithmetic or annotation/layout variants to seek a pass. The next mechanism needs a distinct reduction in arithmetic or bookkeeping cost that survives complete concurrent caller tests. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native validation](artifacts/native/result.json) and raw logs are included.\n");
    save(
        root.join("closeout.json"),
        &json!({"component_gate_passed":false,"calibration_passed":true,"nonregression_passed":true,"logical32_aggregate_gain_passed":false,"adopted":false,"production172_unchanged":true,"SDK_trial_permitted":false,"native_field_tests_each":34,"native_field_configurations":3,"new_distinct_integer_row_cases":48655,"native_core_tests_each":122,"native_core_configurations":2,"harness_tests_each":3,"harness_configurations":3,"captures":1124,"timed_kernel_calls":216432640,"profile_samples":samples,"all_counters_100_percent_running":true,"timed_allocations":0,"final_pins_verified":pins.as_object().unwrap().len(),"new_Gnark_proofs":0,"proof_ledger":13082}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    let mut compressed = BTreeMap::new();
    copy(root, root, &Path::new(D).join("artifacts"), &mut compressed);
    for f in [
        "crates/zopro-bn254/src/inverse.rs",
        "crates/zopro-bn254/src/inverse_active_words.rs",
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
        "Archived{}hashes/{}Gitfiles;4rawlossgzverified;activewordsclosedwithoutSDK",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let summary = read(format!("{R}/archive-summary.json"));
    for(k,v)in[("status",json!("Active-word GCD candidate failed concurrent caller gain gate; closed without SDK; committed/pushed/verified; no adoption")),("root",json!(R)),("report",json!(format!("{D}/README.md"))),("commit",json!(commit)),("pushed",json!(true)),("remote_verified",json!(true)),("production_changed",json!(false)),("adopted",json!(false)),("last_goal_turn_classification",json!("PROGRESS")),("SDK_requests",json!(0)),("new_independent_exports",json!(0)),("independent_proof_count",json!(13082)),("artifact_hashes",summary["hashes"].clone()),("git_files_new_report",summary["git_files"].clone()),("component",read(format!("{R}/analysis.json"))),("active_words_result",read(format!("{R}/closeout.json"))),("previous_completed_experiment",previous),("next_action",json!("Active-word candidate closed; find distinct arithmetic/work removal surviving concurrent callers, no threshold/annotation/layout reroll")),("owned_live_jobs",json!([])),("owned_cloud_resources",json!([]))]{s[k]=v;}
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — ACTIVE-WORD GCD CALLER SCREEN\n\nHEAD/remote verified{commit};root{R};report{D}/README.md.{}strictverifiedhashes/{}Gitfiles;4rawlossgzroundtripSHAverified. LastgoalturnPROGRESS:implemented/validateddistinctworkremoval,completedconcurrentactualcallerdiagnostic,closednegativewithoutunnecessarySDKtrial. Accepted172/SDK14da unchanged,local1x2historical8.324853ms/RSS134.50MiB,goalACTIVEunmet0/8,proofledger13082unchanged,noownedjobs/cloud.\n\nCandidatecopiesaccepted172;onlyinverseintegerupdatesprocess2wordsbits<=128,3<=192,4otherwise. MatchONCEperpairusesexistingbits,rownorm<=2^31,abslinear<2^(64N+31),outputupperwordszero. Matrices/31-62coeffupdates/Euclidfinish/scaleEXACTaccepted. No heap/table/keyplan/newunsafe/publicoption. Privatecfgzopro_gcd_active_words. No rejectedweighted/finish/square inherited. Diagnosticcfgzopro_active_words_caller_probe addoriginalCalleradapters.169originalsexact+inverseedit/msm/libdiag;reverseall172exact.\n\nNative24159TERMINALexit0:34fullfieldcheckedON/OFF/ONeach;new48,655rowcases atEVERYbits65..254+wordcorners/pure±2^31,againstold4wordandunsigned5wordmagnitudeoracle. Existing10kfullinverseFermatperfield+rawpowers+finish preserved.122coreOFF/ON,3harnesstestsper3variantsallPASSonce. Independentfrozenexactdirect-ref-pack/native/frozen-artifacts+frozen-proc-macros libs;freshOFF/ONownseparatetargetroots,allrlibs/sofreezeimmediate;field/coreflags/featuresandhasheschecked;NOmutablerootreuse. Harnessborrow200/352,owned200,G2prefix200 allsolo/physical16/logical32. Separatefrozen/OFF/ONexecutablesSAMEharnessremovesweightedsetter.\n\nCapture40251TERMINALexit0;31blocks12contexts3variants1116latency8192calls/workerafter3x1024primes;then4groupstat262144calls/workerlogical32G1borrow352/G2200OFF/ON;then4FIFO199Hzcycles samecases/count;then12owner/decode/loss1136steps1124captures. Allfullgenerator/digest/capacitycheckszeroGLOBALtimedallocs;216432640timedkernelcallsNOTproofs.559finalpinsverified,allnative/link/capturefirstattempt,NOrepairs/retries. No ownedhelper/build/hash/archiveoverlap.\n\nGATEScalibrationPASS/nonregPASS/gainFAIL. Primary3realstreamlogical32aggregateON/OFF1.000239921CI[.994370045,1.009596838],14/31;ON/frozen1.002958819[.997131296,1.007339453],12/31. Allboundsnominal97.055%marginalrank10..22,nofamilywise. SmallpositivesRETained:ownedG1soloON/OFF.991638173CI[.990045,.998619],24/31;G2solo.994894794[.992188,.996978],26/31;G2logical32.997410947CI[.992866,1.001239],19/31uncertain. Noqualifiedparallelgain,NOnewSDK. StatG1inst85029.988->84667.327(-.427%),cycles48676.667->49080.562(+.830%);G2inst109428.421->109066.333(-.331%),cycles60055.376->60530.352(+.791%);branchmissesrise,genericmissesvary,NOSINGLEcausalclaim/DRAMattribution.4profiles87440samplesall32workerTIDs/onePID/zeroLost;notproofwallcost.\n\nPaperreviewSQIsign2608.13948v2sections3.1-3.7specialprime/algorithmSIMDscheduling,nospeedtransfer;Cd2608.01675v5introgeometryuntwistedrational4torsionnotground-fieldisomorphictoodd-orderBN254G1,nocurvemodelportselected. Onlystatedsectionsreadnot636pagemonograph. PriorGLVbucket/ring,SRE,NoelPlantard,sharedcomplex/halfMontgomery/square/zero-runs/finishalreadyclosed.\n\nNEXT:exactactive-wordcandidateCLOSED;noSDK/noidthreshold/annotation/layout/schedulingreroll. FindDISTINCTlargerworkremoval;concurrentcallersremainmandatoryevidenceNOTinstruction-count-onlycriterion. PotentialideaSIMDinteger-rowfullupdateswouldneedpriorpacked-GCDauditandabound/workmodel;NOTselectedorimplemented. Allproductionall8CPU/Metal/FFTengineeringexceptionfalsegatesunchanged;latestAWSstale17137261af77. No newSDK/Gnark/all8/54portable/resource/mobile/TEE/giantproofs. CommitANDpushcompletedexperimentsincludingnegative,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Olderheadershistorical.\n\n",summary["hashes"],summary["git_files"]);
    fs::write(hp, format!("{header}{old}")).unwrap();
    println!("Updatedstate/handoff;goalactive0/8;ledger13082;activewordsclosed");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
