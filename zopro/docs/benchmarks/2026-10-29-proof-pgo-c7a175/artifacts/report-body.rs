fn number(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn best<'a>(a: &'a Value, shape: &str, mode: &str) -> (usize, &'a Value) {
    [32usize, 64, 96]
        .into_iter()
        .map(|w| (w, &a["cells"][format!("{shape}-w{w}/{mode}")]))
        .min_by(|a, b| number(a.1, "ms").total_cmp(&number(b.1, "ms")))
        .unwrap()
}
fn archive() {
    let a = read(r("analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(a["performance_gate_passed"], false);
    assert_eq!(a["proof_ledger"], 20509);
    assert_eq!(a["SDK_requests"], 37272);
    assert_eq!(a["new_independent_exports"], 1248);
    assert_eq!(a["post_transport_verified"], 1221);
    let termination = read(r("termination-observation.json"));
    assert_eq!(
        termination["Reservations"][0]["Instances"][0]["InstanceId"],
        "i-019acb8ff0bdc76cc"
    );
    assert_eq!(
        termination["Reservations"][0]["Instances"][0]["State"]["Name"],
        "terminated"
    );
    assert!(read(r("volume-observation.json"))["Volumes"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(read(r("ecr-cleanup.json"))["failures"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(read(r("remaining-temporary-instances.json"))
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        read(r("bucket-cleanup.json"))["status"],
        "owned object removed; bucket deleted"
    );
    assert!(!r("upload-parameters.secret.json").exists());
    assert!(!r("transport-response.secret").exists());
    let d = Path::new(D).join("artifacts");
    assert!(!d.exists());
    fs::create_dir_all(&d).unwrap();
    let manifest = read(M);
    let mut source = BTreeMap::new();
    let mut closing = BTreeMap::new();
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let from = r("source").join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
        source.insert(p, encoded(&fs::read(&from).unwrap()));
        closing.insert(from.display().to_string(), hash(&from));
    }
    assert_eq!(source.len(), 175);
    compressed(&d, "source-snapshot", &json!(source));
    for (p, h) in read(r("build-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        closing.insert(p.clone(), h.as_str().unwrap().to_owned());
    }
    for f in [
        "native/control/artifacts.json",
        "native-use/candidate/artifacts.json",
    ] {
        let v = read(r(f));
        let p = v["binary"].as_str().unwrap();
        assert_eq!(hash(p), v["binary_sha256"].as_str().unwrap());
        closing.insert(p.into(), hash(p));
        for (p, h) in v["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            closing.insert(p.clone(), hash(p));
        }
    }
    let mut receipts = vec![];
    let mut streams = 0;
    for (flow, n, label) in [
        (r("native"), 6, "native"),
        (r("native-use"), 6, "native-use"),
        (r("local-output/preflight/workflow"), 54, "preflight"),
        (r("remote-results/output/measure/workflow"), 2460, "measure"),
    ] {
        let v = read(flow.join("result.json"));
        assert_eq!(v["status"], "complete");
        assert_eq!(v["steps"].as_array().unwrap().len(), n);
        for step in v["steps"].as_array().unwrap() {
            let name = step.as_str().unwrap();
            let mut files = BTreeMap::new();
            for file in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
                let p = flow.join(name).join(file);
                let b = fs::read(&p).unwrap();
                if file == "exit.json" {
                    let e: Value = serde_json::from_slice(&b).unwrap();
                    assert_eq!(e["success"], true);
                    assert_eq!(e["returncode"], 0);
                }
                files.insert(file, encoded(&b));
                streams += 1;
            }
            receipts.push(json!({"workflow":label,"name":name,"files":files}));
        }
        for file in [
            "result.json",
            "source-pins.json",
            "protocol.json",
            "completed.json",
        ] {
            fs::copy(flow.join(file), d.join(format!("{label}-{file}"))).unwrap();
        }
    }
    assert_eq!(receipts.len(), 2526);
    assert_eq!(streams, 10104);
    compressed(&d, "workflow-receipts", &json!(receipts));
    let inv = read(r("proof-inventory.json"));
    assert_eq!(inv.as_object().unwrap().len(), 2496);
    let mut proof = BTreeMap::new();
    for (p, h) in inv.as_object().unwrap() {
        let b = fs::read(p).unwrap();
        assert_eq!(digest(&b), h.as_str().unwrap());
        proof.insert(p, encoded(&b));
        closing.insert(p.clone(), hash(p));
    }
    compressed(&d, "proof-public-members", &json!(proof));
    let mut drivers = BTreeMap::new();
    let mut raw = BTreeMap::new();
    for base in [
        r("local-output/preflight"),
        r("remote-results/output/measure"),
    ] {
        let jobs = read(base.join("jobs.json"));
        for j in jobs.as_array().unwrap() {
            let p = j["directory"].as_str().unwrap();
            let actual = if base.starts_with(r("local-output")) {
                r("local-output").join(p.strip_prefix("/output/").unwrap())
            } else {
                r("remote-results").join(p.strip_prefix('/').unwrap())
            };
            for f in ["driver.stdout", "driver.stderr"] {
                let p = actual.join(f);
                drivers.insert(p.display().to_string(), encoded(&fs::read(&p).unwrap()));
            }
            if j["phase"] == "profile" {
                let p = r("remote-results")
                    .join(j["out"].as_str().unwrap().strip_prefix('/').unwrap())
                    .join("perf.data");
                raw.insert(
                    j["name"].as_str().unwrap().to_owned(),
                    encoded(&fs::read(&p).unwrap()),
                );
            }
        }
    }
    assert_eq!(drivers.len(), 2496);
    assert_eq!(raw.len(), 6);
    compressed(&d, "raw-driver-streams", &json!(drivers));
    compressed(&d, "raw-perf-data", &json!(raw));
    for f in [
        "analysis.json",
        "analysis.log",
        "analyze.rs",
        "design.md",
        "native.rs",
        "build-setup.rs",
        "build-setup-body.rs",
        "source-audit.json",
        "build-plan.json",
        "use-plan.json",
        "build-pins.json",
        "control-flags.txt",
        "candidate-flags.txt",
        "frozen-control.json",
        "frozen-candidate.json",
        "frozen-control-cargo.jsonl",
        "frozen-candidate-cargo.jsonl",
        "native/control/artifacts.json",
        "native-use/candidate/artifacts.json",
        "runner.rs",
        "support/config.rs",
        "support/results.rs",
        "prepare-image.rs",
        "prepare-image-repair.rs",
        "initial-image-preparation/Dockerfile",
        "initial-image-preparation/manifest.json",
        "initial-image-preparation/repair.json",
        "context/Dockerfile",
        "context/manifest.json",
        "context/preflight-complete.json",
        "base-image-id.txt",
        "aeglos-source-identity.json",
        "image-manifest.json",
        "run-id.txt",
        "instance-id.txt",
        "cloud.rs",
        "cloud-current.rs",
        "cloud-network-resolution.json",
        "launch-template.json",
        "launch-template-current.json",
        "user-data-template.sh",
        "cloud-current.rs",
        "transfer.rs",
        "sigv4.rs",
        "identity.json",
        "launch.json",
        "launch-result.json",
        "boot-command.json",
        "boot-result.json",
        "pull-command.json",
        "pull-result.json",
        "measure-command.json",
        "measure-result.json",
        "package-command.json",
        "package-result.json",
        "upload-command.json",
        "upload-result.json",
        "upload-parameters-redacted.json",
        "retrieval-complete.json",
        "transport-check.json",
        "bucket.json",
        "bucket-cleanup.json",
        "termination-result.json",
        "termination-observation.json",
        "volume-observation.json",
        "ecr-cleanup.json",
        "remaining-temporary-instances.json",
        "verification-copy-inventory.json",
        "post-transport-gnark-exit.json",
        "initial-post-transport/repair.json",
        "post-transport-command.json",
        "initial-post-transport/command.json",
        "proof-inventory.json",
        "proof-sha256-after.json",
        "remote-results/hardware.txt",
        "remote-results/container-inspect.json",
        "remote-results/post-host-state.txt",
        "remote-results/output/measure/complete.json",
        "remote-results/output/measure/before.json",
        "remote-results/output/measure/after.json",
        "remote-results/output/measure/after-latency.json",
        "local-output/preflight/complete.json",
        "local-output/preflight/rows.json",
        "local-output/preflight/jobs.json",
        "remote-results/output/measure/jobs.json",
        "report.rs",
        "report-body.rs",
    ] {
        let to = d.join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(r(f), to).unwrap();
        closing.insert(r(f).display().to_string(), hash(r(f)));
    }
    for f in [
        "prepare.log",
        "native.log",
        "native-use.log",
        "prepare-image.log",
        "prepare-image-repair.log",
        "initial-image-preparation/prepare-image.log",
        "initial-image-preparation/docker-preflight-build.log",
        "initial-post-transport/no-proofs.log",
        "post-transport-gnark.log",
        "verification-prepare.log",
    ] {
        let to = d.join(format!("{f}.json"));
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        save(to, &encoded(&fs::read(r(f)).unwrap()));
        closing.insert(r(f).display().to_string(), hash(r(f)));
    }
    // Keep data and symbol parsing from the immutable cloud rows; no new capture.
    compressed(
        &d,
        "cloud-rows",
        &read(r("remote-results/output/measure/rows.json")),
    );
    let local_parent="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-proof-pgo-qualification175/artifacts/analysis.json";
    let pgo_parent="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-proof-pgo175/artifacts/candidate-manifest.json";
    save(
        d.join("inherited-provenance.json"),
        &json!({"local_qualification_commit":"b7fea4abf49acfb8f171e5edd42b4d2a067f98ff","local_analysis":local_parent,"local_analysis_sha256":hash(local_parent),"PGO_parent_manifest":pgo_parent,"PGO_parent_manifest_sha256":hash(pgo_parent),"profile_path":"/home/ubuntu/src/zopro-perf-work/proof-pgo175/merged.profdata","profile_sha256":"0f9dbf4b871db206b18bacdd343e6fbb4ea92bbeae1942404fdc7d83839a8d08","profile_retrained":false,"local_failed_gate_retained":true,"source_files_exact":175,"new_native_tests_each":{"field":33,"core":124,"prover":8}}),
    );
    save(
        d.join("accepted-comparison.json"),
        &json!({"manifest":M,"sha256":hash(M),"native_binary_sha256":manifest["native_binary_sha256"],"ms":manifest["ms"],"m5":manifest["m5"],"inherited_local_not_remeasured":true,"full_performance_gate_passed":false,"physical16_engineering_exception_retained":true,"final_artifact_gate_passed":true,"production_changed":false}),
    );
    for (p, h) in &closing {
        assert_eq!(hash(p), *h);
    }
    save(d.join("closing-pins.json"), &json!(closing));
    let close = json!({"complete":true,"SDK_requests":37272,"local_SDK":108,"cloud_SDK":37164,"new_Gnark_exports":1248,"post_transport_reverified_existing":1221,"proof_ledger":20509,"proof_public_members":2496,"native_steps":12,"local_preflight_steps":54,"cloud_steps":2460,"receipts":2526,"original_streams_byte_exact":10104,"raw_driver_streams":2496,"raw_profiles":6,"source_files_exact":175,"closing_pins_verified":closing.len(),"performance_gate_passed":false,"primary32_gain_passed":false,"adopted":false,"production_changed":false,"all8_CPU10x_rows":0,"goal_active_unmet":true,"owned_instances":0,"owned_volumes":0,"owned_ECR_tags":0,"owned_transfer_buckets":0,"local_URL_secret_files":0,"initial_Docker_failure_before_SDK_retained":true,"initial_post_transport_zero_proofs_retained":true,"unplanned_successful_SDK_capture_native_test_or_proof_verification_reruns":0});
    save(d.join("closeout.json"), &close);
    save(r("closeout.json"), &close);
    write_readme(&a, &d);
    seal();
    println!("Archived full C7a175 comparison;PGOtransfergateFALSE;37272SDK1248Gnark/ledger20509;all owned resources removed");
}
fn write_readme(a: &Value, d: &Path) {
    let m = read(r("context/manifest.json"));
    let mut b=String::from("# Current Zopro, transferred proof-PGO profile and pinned Aeglos on C7a\n\nThe transferred PGO profile does not pass C7a qualification. Across all eight reference circuits it is approximately flat at 32 workers and slower by 5.704% at 64 and 4.997% at 96; both larger-worker aggregates lose all 15 blocks. The primary32 paired ratio is 1.001256 [0.998103, 1.008667], with 5/15 wins: no established gain. Seventeen of 27 PGO/plain nonregression checks fail. The candidate remains unadopted and production is unchanged.\n\nAt their respective fastest tested worker counts, plain Zopro records 11.915591 ms for 1×2 versus pinned Aeglos Sorted's 7.959184 ms. For 54→1, plain Zopro records 122.396711 ms versus 173.236337 ms, with maximum RSS 1,877.03 versus 13,115.29 MiB. Plain Zopro has a lower displayed best-count median on six of nine circuits; Aeglos on 1×2, 2×2 and 4×2. Best-count selection is descriptive; same-worker paired results are retained below. Neither Zopro build meets any of the eight supplied CPU-10× targets.\n\n## Builds and fixed scope\n\nBoth Zopro builds use the exact 175 accepted source files, Rust 1.98.1 / LLVM 22.1.8 and `target-cpu=znver4`. All other retained baseline flags and the standard SDK driver are shared. Plain SDK SHA is `53cfecd8f2353d3842d3eeb02c1e106d4d23a13a3fb28b328027ec93eaeb6f5a`; PGO SDK SHA is `0a7c564ae62e13a833569184147dd7a92110276c29aeea666a22ee7d9c33b41a`. These are fresh Zen4 builds of current accepted source, not the byte-identical local Zen5 accepted executable. Each build freshly passes 33 field, 124 core and 8 prover tests. No new arithmetic-case count is claimed for reuse of the existing tests.\n\nPGO consumes the same proof-only profile `0f9dbf4b…` from the [local pilot](../2026-10-29-proof-pgo175/README.md), trained across all nine shapes at 32 workers on Ryzen 9950X. There is no C7a retraining, profile choice or new training capture. The consumer contains no LLVM profiling-runtime symbols; all compiler warnings are retained. The [broader local qualification](../2026-10-29-proof-pgo-qualification175/README.md) retains its aggregate gain and failed physical16 calibration check. This is an explicit architecture-transfer experiment; the local result is not universal compiler evidence.\n\nAeglos reuses pinned commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, its Sorted engine, existing preparation policy, qualified native archive and pool-aware SDK driver. It is not asserted to be the latest upstream. The adapter checks prepared-key cache hits and preserves the driver's verified-request timer. Current Zopro images and established underlying keys/fixture bindings are pinned. No per-key lookup expansion, source change, new memory cap or prover architecture change occurs.\n\nOne dedicated c7a.24xlarge provides 96 physical cores without SMT. Cases use 32/64/96 workers pinned to CPUs 0…N−1. The network-disabled container has CPU quota 96, memory 30 GiB and no swap. This is a benchmark container limit, not a Zopro architectural cap or minimum RAM requirement. Fifteen fixed blocks rotate/reverse both context and engine order across all nine circuits, including 54→1. Primary processes measure 101 requests after three primes; other circuits 17 after three. Loading/preparation/primes are excluded from latency but included in process lifetime RSS. Timers include fresh assignment/witness solving, fresh randomness, proving, internal verification and serialization.\n\nAll 1,215 latency processes finish before their independent Gnark verifications. Six separate 199 Hz user-cycle profiles follow for primary32/96 across all three engines, 251 requests after three primes with FIFO enable/ack. All six finish before their verification, decoding and raw-loss audits. No owned build, hash, archive or other CPU helper overlaps captures. All helpers are prepared before capture; post-capture analysis and report helpers do not run during measurement. No successful capture or test is repeated, extended or excluded.\n\n## Complete latency and resources\n\nEach cell is the median of 15 process medians. CPU columns are median per-process mean user+system CPU milliseconds per request. RSS is the maximum lifetime process RSS across the 15 processes, including load and preparation; it is not a cgroup peak, incremental workspace or minimum RAM.\n\n|Circuit|Workers|Plain ms|PGO ms|Aeglos ms|Plain CPU ms|PGO CPU ms|Aeglos CPU ms|Plain max RSS MiB|PGO max RSS MiB|Aeglos max RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for c in m["cases"].as_array().unwrap() {
        let shape = c["shape"].as_str().unwrap();
        for w in [32, 64, 96] {
            let cs: Vec<_> = ["plain", "PGO", "aeglos"]
                .map(|v| &a["cells"][format!("{shape}-w{w}/{v}")])
                .into_iter()
                .collect();
            writeln!(
                b,
                "|{shape}|{w}|{:.6}|{:.6}|{:.6}|{:.3}|{:.3}|{:.3}|{:.2}|{:.2}|{:.2}|",
                number(cs[0], "ms"),
                number(cs[1], "ms"),
                number(cs[2], "ms"),
                number(cs[0], "CPU_ms"),
                number(cs[1], "CPU_ms"),
                number(cs[2], "CPU_ms"),
                number(cs[0], "max_RSS_MiB"),
                number(cs[1], "max_RSS_MiB"),
                number(cs[2], "max_RSS_MiB")
            )
            .unwrap();
        }
    }
    b.push_str("\n## Paired results and fixed checks\n\nRatios use all 15 paired blocks, median rank 8 and bounds at ranks 4 and 12. Nominal individual coverage is 96.484375% under independent, exchangeable blocks; these are not simultaneous/familywise guarantees. Paired ratios and savings need not equal ratios or differences of displayed marginal medians. In particular, primary32's displayed PGO median is slightly lower, while its paired estimate is slightly slower and uncertain.\n\nPGO qualification requires primary32 upper ratio <1 and at least 10/15 wins, upper ratio ≤1.03 in all 27 PGO/plain contexts, and strict gain with at least 10 wins for each worker count's equal-log aggregate of all eight reference shapes. There is no positive-gain floor, post-capture threshold waiver or subset promotion. Same-worker Aeglos win checks use strict upper ratio <1 and at least 10/15 wins; best-count selection is kept separate. All fixed PGO gain conditions fail, and 17 nonregression conditions fail. Positive individual estimates remain in the table.\n\n|All-eight PGO/plain aggregate|Ratio|Bounds|Wins / 15|\n|---|---:|---|---:|\n");
    for (name, p) in a["aggregates"].as_object().unwrap() {
        writeln!(
            b,
            "|{name}|{:.9}|[{:.9}, {:.9}]|{}|",
            number(p, "median"),
            number(p, "low"),
            number(p, "high"),
            p["wins"]
        )
        .unwrap();
    }
    b.push_str("\n|Same-worker comparison|Ratio|Bounds|Wins / 15|Paired saving µs|\n|---|---:|---|---:|---:|\n");
    for (name, p) in a["pairs"].as_object().unwrap() {
        writeln!(
            b,
            "|{name}|{:.9}|[{:.9}, {:.9}]|{}|{:.3}|",
            number(p, "median"),
            number(p, "low"),
            number(p, "high"),
            p["wins"],
            number(p, "paired_saving_ms") * 1000.
        )
        .unwrap();
    }
    b.push_str("\n## Best tested worker counts\n\nThese selections describe this complete matrix and are not a generally optimal worker policy or a new paired comparison across different worker counts. All same-worker observations remain above.\n\n|Circuit|Plain ms (workers)|PGO ms (workers)|Aeglos ms (workers)|Plain max RSS MiB|Aeglos max RSS MiB|\n|---|---:|---:|---:|---:|---:|\n");
    for c in m["cases"].as_array().unwrap() {
        let shape = c["shape"].as_str().unwrap();
        let (p, pc) = best(a, shape, "plain");
        let (g, gc) = best(a, shape, "PGO");
        let (x, xc) = best(a, shape, "aeglos");
        writeln!(
            b,
            "|{shape}|{:.6} ({p})|{:.6} ({g})|{:.6} ({x})|{:.2}|{:.2}|",
            number(pc, "ms"),
            number(gc, "ms"),
            number(xc, "ms"),
            number(pc, "max_RSS_MiB"),
            number(xc, "max_RSS_MiB")
        )
        .unwrap();
    }
    b.push_str("\n## Supplied M5 CPU and Metal references\n\nThese are supplied historical M5 figures, not new Apple measurements. Every original reference shape is retained. No M5 reference was supplied for 54→1. Both Zopro builds remain at 0/8 CPU-10× targets, so the original goal remains active and unmet. Speedups below use each engine's descriptive fastest tested count. Full PGO/Aeglos CPU+Metal ratios are also retained in analysis.json.\n\n|Shape|M5 CPU ms|M5 Metal ms|Plain ms|Plain CPU speedup|Plain Metal speedup|PGO ms|PGO CPU speedup|PGO Metal speedup|CPU-10× target ms|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for (shape, c) in a["reference_comparisons_best_tested_descriptive"]
        .as_object()
        .unwrap()
    {
        let p = &c["variants"]["plain"];
        let g = &c["variants"]["PGO"];
        writeln!(
            b,
            "|{shape}|{:.0}|{:.0}|{:.6}|{:.3}×|{:.3}×|{:.6}|{:.3}×|{:.3}×|{:.1}|",
            number(c, "reference_CPU_ms"),
            number(c, "reference_Metal_ms"),
            number(p, "ms"),
            number(p, "CPU_speedup"),
            number(p, "Metal_speedup"),
            number(g, "ms"),
            number(g, "CPU_speedup"),
            number(g, "Metal_speedup"),
            number(c, "CPU10x_target_ms")
        )
        .unwrap();
    }
    b.push_str("\n## Profiles, scaling and limits\n\nAll six profiles contain one SDK PID and zero decoded lost records, confirmed by separate raw perf record audits. The following cycle-residence percentages are flat symbol classifications, not exclusive work, idle-time measurements or removable wall-time fractions. PGO can change inlining and symbol attribution, so epoch and scheduler categories should be considered together; their split alone does not establish a mechanism. Aeglos matches `other` in the inherited generic classifier; its complete raw symbols remain available.\n\n|Profile|Samples|G1 %|G2 %|Epoch %|Scheduler %|Epoch + scheduler %|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        if name.ends_with("-aeglos") {
            writeln!(
                b,
                "|{name}|{}|unclassified|unclassified|unclassified|unclassified|unclassified|",
                p["samples"]
            )
            .unwrap();
            continue;
        }
        let c = &p["category_percent"];
        let get = |k: &str| c[k].as_f64().unwrap_or(0.);
        writeln!(
            b,
            "|{name}|{}|{:.2}|{:.2}|{:.2}|{:.2}|{:.2}|",
            p["samples"],
            get("G1_arithmetic"),
            get("G2_arithmetic"),
            get("epoch"),
            get("scheduler"),
            get("epoch") + get("scheduler")
        )
        .unwrap();
    }
    b.push_str("\nPlain Zopro's combined epoch/scheduler residence rises from 14.09% at 32 workers to 61.48% at 96; PGO shows 14.63% and 63.83%. Plain primary CPU service rises from 256.128 to 704.247 ms/request while latency worsens from 11.915591 to 13.777708 ms. Aeglos's corresponding latency falls from 11.867991 to 7.959184 ms. This points further investigation toward work availability, critical paths and coordination at larger pools. It does not isolate a clock, memory-capacity, cache, syscall or fence cause, or predict a gain from removing a symbol's samples. Previous scheduling trials retain their outcomes; these profiles do not justify repeating unchanged rejected variants.\n\nWhole-container workflow CPU counters record 39 throttled periods and 521,975 throttled microseconds. This scope includes loading, preparation, priming, independent verification, latency and profiling; it is not a per-variant or timed-only attribution. All memory/swap event deltas are zero, including OOM. Nonzero throttling is retained and no timing is repeated to remove it. Cgroup events and process RSS have different scopes. These measurements do not establish minimum RAM, concurrent throughput, phones, TEE overhead, portable builds or 250/4999-circuit performance.\n\n## Verification and cleanup\n\nExact-container local preflight completes 108 SDK requests and 27 independent exports, all nine circuits across three engines at four workers. Cloud completes 37,164 SDK requests and 1,221 independent exports. Combined: **37,272 requests and 1,248 distinct Gnark-verified exports**, 2,496 proof/public members, ledger 19,261→20,509. Every retained proof is 164 bytes, fixture-bound and disjoint from all prior/new identities. All 1,221 cloud exports are independently reverified locally after SHA-256/size-checked retrieval; that declared repeat verification adds no new proofs. The first local verifier invocation found no proofs in the nested layout and verified zero; hash-checked flat copies then pass once. The original failure is retained.\n\nThe initial Docker packaging attempt failed before any SDK request or VM launch because `FROM sha256:localID` was treated as a registry reference. A continuation uses the existing local base tag after exact image-ID verification; native builds/tests are not rerun. Original helper source, failing logs, Dockerfile and repair evidence remain archived. The old benchmark subnet/profile no longer existed; a separate launch-template resolution uses the existing current qualification subnet, no-ingress security group and instance profile, preserving the original helper metadata. No network or IAM resource is created or modified.\n\nExactly one owned c7a.24xlarge (`i-019acb8ff0bdc76cc`) uses a 90-minute termination timer and encrypted delete-on-termination storage. The instance is confirmed terminated, its tagged volume absent, the unique ECR tag deleted, and the private transfer object/bucket and local URL files removed. No regional TemporaryProverBenchmark instance remains. No production resource is changed.\n\n## Artifacts\n\n[Analysis](artifacts/analysis.json), [fixed protocol](artifacts/design.md), [source/build audit](artifacts/source-audit.json), [all native/preflight/cloud receipts](artifacts/workflow-receipts.json.gz), [original driver streams](artifacts/raw-driver-streams.json.gz), [all proof/public bytes](artifacts/proof-public-members.json.gz), [cloud samples and profile symbols](artifacts/cloud-rows.json.gz), [raw perf data](artifacts/raw-perf-data.json.gz), [source snapshot](artifacts/source-snapshot.json.gz), [post-transport verification](artifacts/post-transport-gnark.log.json), [closing pins](artifacts/closing-pins.json), [closeout](artifacts/closeout.json). Each encoded archive member includes its SHA-256; decompression round-trips are checked. Artifact index and SHA256SUMS cover the report.\n");
    fs::write(d.parent().unwrap().join("README.md"), b).unwrap();
}
fn review() {
    let d = Path::new(D).join("artifacts");
    write_readme(&read(r("analysis.json")), &d);
    let mut close = read(r("closeout.json"));
    close
        .as_object_mut()
        .unwrap()
        .remove("successful_SDK_capture_native_test_or_proof_verification_reruns");
    close["unplanned_successful_SDK_capture_native_test_or_proof_verification_reruns"] = json!(0);
    save(r("closeout.json"), &close);
    save(d.join("closeout.json"), &close);
    let mut pins = read(d.join("closing-pins.json"));
    for file in ["report.rs", "report-body.rs"] {
        fs::copy(r(file), d.join(file)).unwrap();
        pins[r(file).display().to_string()] = json!(hash(r(file)));
    }
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    save(d.join("closing-pins.json"), &pins);
    seal();
}
