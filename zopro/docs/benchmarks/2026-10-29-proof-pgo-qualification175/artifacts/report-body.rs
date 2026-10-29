const PILOT: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
fn n(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn archive() {
    let a = read(r("analysis.json"));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["failed_checks"], json!(["1x2-w16/OFF/accepted"]));
    assert_eq!(a["SDK_requests_new"], 20046);
    assert_eq!(a["new_independent_exports"], 414);
    assert_eq!(a["independent_proof_count"], 19261);
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read(M);
    let source = Path::new(PILOT).join("source");
    let mut files = BTreeMap::new();
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(source.join(p)), h.as_str().unwrap());
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        files.insert(p, encoded(&fs::read(source.join(p)).unwrap()));
    }
    assert_eq!(files.len(), 175);
    let d = Path::new(D).join("artifacts");
    assert!(!d.exists());
    fs::create_dir_all(&d).unwrap();
    compressed(&d, "source-snapshot", &json!(files));
    let result = read(r("runs/result.json"));
    assert_eq!(result["status"], "complete");
    let steps = result["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 837);
    let mut receipts = vec![];
    for step in steps {
        let step = step.as_str().unwrap();
        let mut files = BTreeMap::new();
        for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
            let b = fs::read(r(format!("runs/{step}/{f}"))).unwrap();
            if f == "exit.json" {
                let e: Value = serde_json::from_slice(&b).unwrap();
                assert_eq!(e["returncode"], 0);
                assert_eq!(e["success"], true);
            }
            if f == "command.json" {
                let c: Value = serde_json::from_slice(&b).unwrap();
                // Workflow command receipts are already limited to task environment keys.
                if let Some(env) = c.get("environment").and_then(Value::as_object) {
                    for key in env.keys() {
                        assert!(
                            key.starts_with("MALLOC_")
                                || key.starts_with("RAYON_")
                                || key.starts_with("ZOPRO_")
                        );
                    }
                }
            }
            files.insert(f, encoded(&b));
        }
        receipts.push(json!({"workflow":"runs","name":step,"files":files}));
    }
    compressed(&d, "workflow-receipts", &json!(receipts));
    let inv = read(r("proof-inventory.json"));
    let mut proof = BTreeMap::new();
    for (p, h) in inv.as_object().unwrap() {
        let b = fs::read(p).unwrap();
        assert_eq!(digest(&b), h.as_str().unwrap());
        proof.insert(p, encoded(&b));
    }
    assert_eq!(proof.len(), 828);
    compressed(&d, "proof-public-members", &json!(proof));
    let mut profiles = vec![];
    let mut raw = BTreeMap::new();
    for (name, p) in a["diagnostics_physical16_new"]["profiles"]
        .as_object()
        .unwrap()
    {
        assert_eq!(p["lost"]["zero_lost"], true);
        assert_eq!(p["PIDs"].as_array().unwrap().len(), 1);
        let symbols: Vec<Value> = p["symbols"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| {
                let symbol = s["symbol"].as_str().unwrap();
                let pretty = rustc_demangle::try_demangle(symbol)
                    .map(|s| format!("{s:#}"))
                    .unwrap_or_else(|_| symbol.into());
                json!({"raw":symbol,"demangled":pretty,"samples":s["samples"],"period":s["period"]})
            })
            .collect();
        profiles.push(json!({"name":name,"samples":p["samples"],"PIDs":p["PIDs"],"zero_lost":true,"symbols":symbols}));
        let path = r(format!("perf/{name}.data"));
        assert!(Command::new("sudo")
            .args(["-n", "chown", "ubuntu:ubuntu"])
            .arg(&path)
            .status()
            .unwrap()
            .success());
        raw.insert(name, encoded(&fs::read(path).unwrap()));
    }
    assert_eq!(profiles.len(), 3);
    compressed(&d, "demangled-profile-symbols", &json!(profiles));
    compressed(&d, "physical16-perf-data", &json!(raw));
    for f in [
        "result.json",
        "source-pins.json",
        "protocol.json",
        "completed.json",
    ] {
        fs::copy(r(format!("runs/{f}")), d.join(format!("runs-{f}"))).unwrap();
    }
    for f in [
        "analysis.json",
        "finish.log",
        "design.md",
        "bootstrap.rs",
        "qualification.rs",
        "capture.rs",
        "capture-tests.log",
        "topology.csv",
        "pmu-feasibility.log",
        "pins.json",
        "run-plan.json",
        "jobs.json",
        "cases.json",
        "all32-cases.json",
        "identities.json",
        "prior-proof-sha256.json",
        "proof-inventory.json",
        "prepare.log",
        "host-before.txt",
        "host-after.txt",
        "report.rs",
        "report-body.rs",
    ] {
        if f.ends_with(".log") {
            save(
                d.join(format!("{f}.json")),
                &encoded(&fs::read(r(f)).unwrap()),
            );
        } else {
            fs::copy(r(f), d.join(f)).unwrap();
        }
    }
    let parent_dir = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-proof-pgo175/artifacts",
    );
    let mut parent_evidence = BTreeMap::new();
    for f in [
        "candidate-manifest.json",
        "analysis.json",
        "training-audit.json",
        "code-audit.json",
        "closeout.json",
        "identities.json",
    ] {
        let p = parent_dir.join(f);
        parent_evidence.insert(f, json!({"path":p,"sha256":hash(&p)}));
    }
    save(
        d.join("parent-evidence.json"),
        &json!({"commit":"e91a83283926697b7de787a12de4b14db5250a02","files":parent_evidence,"primary32_inherited_not_repeated":true,"native_tests_inherited_each":{"field":33,"core":124,"prover":8},"new_native_tests":0,"new_capture_analysis_tests":2}),
    );
    let mut candidate = read(parent_dir.join("candidate-manifest.json"));
    candidate["full_performance_gate_passed"] = json!(false);
    candidate["candidate_performance_failures"] = a["failed_checks"].clone();
    candidate["proof_ledger"] = json!(19261);
    candidate["qualification_status"] = json!({"primary_latency_pilot":"passed_inherited","new7_aggregate_gain":"passed","all_context_nonregression":"passed","calibration":"failed_physical16","compatibility":"not_run","resources":"not_run","final_artifact":"not_run","adoption":"not_adopted"});
    candidate.as_object_mut().unwrap().remove("native_tests_fresh_each");
    candidate["native_tests_inherited_each"] = json!({"field":33,"core":124,"prover":8});
    candidate["primary32_inherited_not_repeated"] = json!(true);
    candidate["qualification_note"]=json!("Positive aggregate and inherited primary gains retained; sole failed check is physical16 plain/accepted calibration. All30cells retained. No native rebuild, retraining, successful capture rerun, waiver or subset promotion.");
    let mut ms = BTreeMap::new();
    for (name, c) in a["cells"].as_object().unwrap() {
        if let Some(name) = name.strip_suffix("/ON") {
            ms.insert(name, n(c, "median_ms"));
        }
    }
    candidate["ms"] = json!(ms);
    candidate["m5"] = a["reference_comparisons"].clone();
    save(d.join("candidate-manifest.json"), &candidate);
    save(
        d.join("accepted-comparison.json"),
        &json!({"manifest":M,"manifest_sha256":hash(M),"native_binary_sha256":manifest["native_binary_sha256"],"ms":manifest["ms"],"m5":manifest["m5"],"inherited_not_remeasured":true,"accepted_full_performance_gate_passed":false,"accepted_physical16_engineering_exception_retained_not_transferred":true,"accepted_final_artifact_gate_passed":true}),
    );
    let close = json!({"full_performance_gate_passed":false,"failed_checks":a["failed_checks"],"new7_gain_gate_passed_both":true,"all_context_nonregression_passed":true,"steps":837,"original_streams_byte_exact":3348,"SDK_requests_new":20046,"Gnark_new":414,"proof_public_members":828,"proof_ledger":19261,"source_files_exact":175,"primary32_inherited_not_repeated":true,"native_tests_inherited_each":{"field":33,"core":124,"prover":8},"capture_analysis_tests_new":2,"closing_pins_verified":pins.as_object().unwrap().len(),"adopted":false,"production_changed":false,"goal_active_unmet":true,"all8_CPU10x_rows":0,"successful_measurement_or_test_reruns":0});
    save(d.join("closeout.json"), &close);
    save(r("closeout.json"), &close);
    write_readme(&a, &manifest, &d);
    seal();
}
fn write_readme(a: &Value, m: &Value, d: &Path) {
    let mut b=String::from("# Current-source PGO: broader latency qualification\n\nPGO improves the newly measured seven-reference-shape aggregate by 0.771% versus accepted and 1.162% versus fresh plain, winning all 15 aggregate blocks against both. The inherited 1×2 pilot improves by 0.828% versus accepted (7.924730 versus 7.988771 ms). All candidate nonregression checks pass. **Full qualification fails only the plain/accepted physical16 calibration check; the candidate is not adopted and production is unchanged.** Positive results remain recorded. This is a small compiler-guided gain, not a 2× improvement.\n\n## What failed\n\nCalibration compares two builds without PGO: the accepted SDK and the freshly rebuilt plain SDK. For 1×2 on sixteen physical cores their paired ratio interval is [0.970947, 1.049961], outside the fixed [1/1.03, 1.03] range at its upper end. The median paired ratio is 0.995654, with individual ratios from 0.905438 to 1.310104. All observations are retained. The failure does not establish that PGO is slower: its physical16 medians are 9.601386 ms versus 9.719106 accepted and 9.825013 plain. The PGO/accepted interval [0.951877, 1.018386] passes the 2% nonregression bound but does not establish a strict physical16 gain. No cause is assigned to the variability. The prior production engineering exception is not transferred, and no capture is repeated or excluded.\n\n## Fixed experiment and provenance\n\nThis continues the [published primary pilot](../2026-10-29-proof-pgo175/README.md), commit `e91a83283926697b7de787a12de4b14db5250a02`. All 175 prover source files, original SDK paths, libraries, compiler flags, proof-only profile and request fixtures remain unchanged. Accepted SDK SHA starts `20fde19f`, plain `2e3634f0`, PGO `144621f1`. No new native build or training occurs. The parent nine-shape native preflights, primary32 latency blocks and logical32 profiles are inherited; they are not independent replication. Parent field/core/prover test results (33/124/8 for each compiler build) are inherited. Two new analysis-helper tests pass, checking paired ranks, ties and inherited block identities.\n\nRyzen 9950X, sixteen physical / 32 logical CPUs, 128 GiB RAM. Thirty-two-worker cases use CPUs 0–31; the physical16 case uses CPUs 0–15, verified to cover sixteen distinct cores. The host is shared, not exclusive. All helpers and inputs were prepared before capture, with no owned build, hash, archive or other CPU helper overlapping capture.\n\nNine new contexts comprise the eight other logical32 shapes, including 54→1, and primary physical16. Three physical16 preflights precede fifteen rotated/reversed blocks of all nine contexts and three variants. Primary16 processes measure 151 requests after three primes; other shapes measure 31 after three primes. All 405 latency processes finish before their 405 independent Gnark checks. Three FIFO-controlled four-event stats and three separate 199 Hz user-cycle profiles follow, with priming excluded; all six finish before their independent verification and decoding. Loading is excluded; witness solving, fresh randomized proving, internal verification and serialization are included.\n\nAll 837 workflow steps finish successfully. The 20,046 SDK requests include primes and diagnostics; 414 newly exported 164-byte proofs pass independent Gnark, image/public binding and hash-freshness checks. Proof ledger 18,847→19,261; 828 proof/public members are retained. No successful test, measurement or proof capture is repeated.\n\n## Latency, CPU service and peak RSS\n\nMedians are displayed for context and need not reproduce within-block paired ratios. RSS columns are maximum observed process RSS in MiB, not minimum RAM requirements. `Inherited` applies only to the three primary32 rows.\n\n|Context|Accepted ms|Plain ms|PGO ms|Accepted CPU ms|Plain CPU ms|PGO CPU ms|Accepted max RSS|Plain max RSS|PGO max RSS|Phase|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|\n");
    for case in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
        "merge-54x1-w32",
        "1x2-w16",
    ] {
        let cs: Vec<_> = ["accepted", "OFF", "ON"]
            .map(|v| &a["cells"][format!("{case}/{v}")])
            .into_iter()
            .collect();
        writeln!(
            b,
            "|{case}|{:.6}|{:.6}|{:.6}|{:.3}|{:.3}|{:.3}|{:.2}|{:.2}|{:.2}|{}|",
            n(cs[0], "median_ms"),
            n(cs[1], "median_ms"),
            n(cs[2], "median_ms"),
            n(cs[0], "CPU_ms"),
            n(cs[1], "CPU_ms"),
            n(cs[2], "CPU_ms"),
            n(cs[0], "max_RSS_MiB"),
            n(cs[1], "max_RSS_MiB"),
            n(cs[2], "max_RSS_MiB"),
            if case == "1x2-w32" {
                "Inherited"
            } else {
                "New"
            }
        )
        .unwrap();
    }
    b.push_str("\n## Paired estimates and declared checks\n\nFifteen-block ratios use median rank 8 and bounds at ranks 4 and 12. Nominal individual coverage is 96.484375% conditional on independent, exchangeable blocks; it is not a simultaneous or familywise guarantee. Shared-host and temporal variation remain limitations. No positive-gain floor is used. Plain/accepted bounds must lie within [1/1.03,1.03] in all ten contexts. PGO upper bounds must be ≤1.03 against both controls, or ≤1.02 for physical16. Inherited primary32 must retain strict gain and at least 10/15 wins; new gain evidence requires the equal-log aggregate of all seven remaining reference shapes to have upper bound <1 and at least 10/15 wins against both controls. Any failed condition fails qualification; no threshold waiver or subset promotion is applied.\n\n|Comparison|Paired ratio|Bounds|Wins / 15|Paired saving µs|Check|\n|---|---:|---|---:|---:|---|\n");
    for (name, p) in a["pairs"].as_object().unwrap() {
        writeln!(
            b,
            "|{name}|{:.9}|[{:.9}, {:.9}]|{}|{:.3}|{}|",
            n(p, "median"),
            n(p, "low"),
            n(p, "high"),
            p["wins"],
            n(p, "paired_saving_ms") * 1000.,
            a["checks"][name]
        )
        .unwrap();
    }
    b.push_str("\n|Aggregate|Ratio|Bounds|Wins / 15|Role|\n|---|---:|---|---:|---|\n");
    for (name, p) in a["aggregate"].as_object().unwrap() {
        writeln!(
            b,
            "|{name}|{:.9}|[{:.9}, {:.9}]|{}|{}|",
            n(p, "median"),
            n(p, "low"),
            n(p, "high"),
            p["wins"],
            if p["used_as_gain_gate"] == true {
                "New gain gate"
            } else {
                "Mixed-phase descriptive only"
            }
        )
        .unwrap();
    }
    b.push_str("\nThe original-eight aggregate combines inherited primary32 and seven new contexts and is descriptive only. It is not contemporaneous eight-shape replication. Individual cases vary: the displayed PGO median for 8×2 is 24.287429 versus 24.070460 ms accepted, and 54→1 is 161.603688 versus 161.523532 ms. Positive aggregate evidence does not mean every shape improves.\n\n## Supplied M5 references\n\nThese are supplied M5 CPU/Metal timings, not new same-machine M5 measurements. Primary32 is inherited. Every original reference shape remains above its CPU-10× target; 0/8 meet the objective. No M5 reference was supplied for 54→1.\n\n|Shape|M5 CPU ms|M5 Metal ms|PGO ms|CPU speedup|Metal speedup|CPU-10× target ms|Phase|\n|---|---:|---:|---:|---:|---:|---:|---|\n");
    for case in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
    ] {
        let c = &a["reference_comparisons"][case];
        let v = &c["variants"]["ON"];
        writeln!(
            b,
            "|{case}|{:.0}|{:.0}|{:.6}|{:.3}×|{:.3}×|{:.1}|{}|",
            n(&c["reference"], "cpu_ms"),
            n(&c["reference"], "metal_ms"),
            n(v, "ms"),
            n(v, "CPU_speedup"),
            n(v, "Metal_speedup"),
            n(&c["reference"], "cpu_ms") / 10.,
            if c["inherited_primary"] == true {
                "Inherited"
            } else {
                "New"
            }
        )
        .unwrap();
    }
    b.push_str("\nAccepted production remains the prior 8.022165 ms / 134 MiB primary result and its retained full-performance-gate failure; this report does not silently reclassify it as fully passing. Its exact timings and reference comparisons are in [accepted comparison](artifacts/accepted-comparison.json).\n\n## Profiles and limits\n\nAll four physical16 events run at 100%; all three profiles contain one target PID and zero decoded lost events. Sample counts accepted/plain/PGO are 5,120/5,226/5,328. Instructions/request are 1,338.263 million PGO versus 1,381.862 million plain; cycles are 582.008 versus 585.530 million. These are separate diagnostic captures, not paired latency results or proof of an exclusive cause. Logical32 profiles are inherited from the pilot. Raw physical16 perf data, decoded receipts and demangled symbols are archived. Inclusive symbol residence is not an exclusive wall-time fraction.\n\nThere is no new AWS, Aeglos, portable, mobile, TEE, 250/4999, memory-pressure, concurrency or minimum-RAM qualification. Compatibility/resource/final-artifact work has not been performed for adoption.\n\n## Paper 2026/2667\n\n[Faster complete formulas and their application to curves with efficient endomorphisms](https://eprint.iacr.org/2026/2667), Aranha and Barreto, offers a five-product complete-addition block, complete tripling, a six-point τ-NAF scalar method and imaginary Toom–Cook extension arithmetic. Our hot bulk MSM uses batched affine additions, so the complete projective formulas are not a direct replacement. Individual scalar callers and Fq6 multiplication are plausible separate trials, without per-key point-table expansion. No implementation, timing or full-proof gain from the paper is claimed here. This review did not change the frozen PGO experiment.\n\n## Artifacts\n\n[Analysis](artifacts/analysis.json), [fixed protocol](artifacts/design.md), [candidate identity and failed qualification](artifacts/candidate-manifest.json), [receipts](artifacts/workflow-receipts.json.gz), [proof/public bytes](artifacts/proof-public-members.json.gz), [source snapshot](artifacts/source-snapshot.json.gz), [raw physical16 profiles](artifacts/physical16-perf-data.json.gz), [demangled symbols](artifacts/demangled-profile-symbols.json.gz), [parent provenance](artifacts/parent-evidence.json), [closeout](artifacts/closeout.json). Archives encode every member with SHA-256 and UTF-8 or hex and are verified by decompression round-trip. Artifact index and SHA256SUMS cover the report.\n");
    let _ = m;
    fs::write(d.parent().unwrap().join("README.md"), b).unwrap();
}
