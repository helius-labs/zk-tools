use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-current-c7a-20261027";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-current-c7a-aeglos";
const M: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
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
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        let rel = p.strip_prefix(root).unwrap();
        let s = rel.display().to_string();
        if name.contains(".secret")
            || name == "docker-auth"
            || name == "local-verification-proofs"
            || name.starts_with("target")
        {
            continue;
        }
        if e.file_type().unwrap().is_dir() {
            archive(root, &p, out, index)
        } else if e.file_type().unwrap().is_file() {
            if s.starts_with("current/")
                && !s.ends_with(".xtask/source-pins.json")
                && !s.ends_with(".xtask/snapshot.json")
            {
                continue;
            }
            if name != "Dockerfile"
                && ![
                    "rs", "md", "json", "jsonl", "txt", "log", "proof", "public", "sh",
                ]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            {
                continue;
            }
            let to = out.join(rel);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(&p, &to).unwrap();
            assert_eq!(hash(&p), hash(&to));
            index.insert(s, hash(&to));
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["SDK_requests"], 8760);
    assert_eq!(a["independent_exports"], 276);
    assert_eq!(a["post_transport_distinct_verified"], 276);
    assert_eq!(a["next_prior_count"], 10180);
    assert_eq!(a["cycle_samples"], 178577);
    assert_eq!(read(r.join("cleanup-complete.json"))["status"], "complete");
    assert_eq!(
        read(r.join("cleanup-complete.json"))
            ["all_live_TemporaryProverBenchmark_instances_in_region"],
        0
    );
    let m = read(M);
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(f)),
            h.as_str().unwrap()
        );
        assert_eq!(hash(r.join("current").join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    for (p, h) in read(r.join("source-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for (p, h) in read(r.join("proof-inventory.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let image = read(r.join("context/manifest.json"));
    for (mode, receipt) in [
        ("current", r.join("native/current/artifacts.json")),
        (
            "previous",
            Path::new("/tmp/zopro-ready-c7a/native/accepted/artifacts.json").to_path_buf(),
        ),
    ] {
        let receipt = read(receipt);
        for (p, h) in receipt["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
        assert_eq!(
            hash(receipt["binary"].as_str().unwrap()),
            image["container_pins"][format!("/usr/local/bin/cr-{mode}")]
                .as_str()
                .unwrap()
        );
    }
    for case in image["cases"].as_array().unwrap() {
        let filename = Path::new(case["image"].as_str().unwrap())
            .file_name()
            .unwrap();
        assert_eq!(
            hash(r.join("context/basis").join(filename)),
            image["container_pins"][case["image"].as_str().unwrap()]
                .as_str()
                .unwrap()
        );
    }
    let rows = read(r.join("remote-results/output/measure/performance/rows.json"));
    let profiles: Vec<_> = rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["phase"] == "profile")
        .collect();
    assert_eq!(profiles.len(), 6);
    let mut raw = BTreeMap::new();
    for row in &profiles {
        assert_eq!(row["profile"]["lost_records"], 0);
        assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
        let p = r
            .join("remote-results/output/measure/performance/profile/b0")
            .join(row["case"].as_str().unwrap())
            .join(row["variant"].as_str().unwrap())
            .join("perf.data");
        raw.insert(p.display().to_string(), hash(&p));
    }
    save(r.join("raw-local-only-sha256.json"), &json!(raw));
    let pool = "/tmp/zopro-aeglos-windowed-ablation/aeglos/cpu/pool.cpp";
    let poolhash = hash(pool);
    assert_eq!(
        poolhash,
        "f7071560101b476762d7c9e4810fde10f414895fc5047846053e9c9f5d519c23"
    );
    save(
        r.join("aeglos-pool-source-identity.json"),
        &json!({"local_source":pool,"sha256":poolhash,"aeglos_commit":"0290613e8eeca17a4ce88f6f2d726e8e06a5385d","retained_build_receipt":"/tmp/zopro-aeglos-windowed-ablation/build-manifest.json","retained_build_receipt_sha256":hash("/tmp/zopro-aeglos-windowed-ablation/build-manifest.json"),"binary_sha256":image["container_pins"]["/usr/local/bin/aeglos-ablation"],"flat_Pool_work_contains_dispatch_and_waiting_not_exclusive_idle":true}),
    );
    let after = read(r.join("remote-results/output/measure/after-profiles.json"));
    let cloud_manifest = json!({"source_manifest":M,"source_manifest_sha256":hash(M),"source_files_sha256":m["source_files_sha256"],"source_files":171,"native_target":"znver4","native_binary":r.join("native/current-sdk"),"native_binary_sha256":image["container_pins"]["/usr/local/bin/cr-current"],"native_cli_build":r.join("native/current/artifacts.json"),"rustflags":image["current_rustflags"],"runtime_environment":image["environment"],"cases":a["cases"],"best_tested":a["best_tested"],"source_changed":false,"no_new_source_adoption":true,"goal_achieved":false});
    save(r.join("aws-accepted-manifest.json"), &cloud_manifest);
    let mut md = String::from(
        r#"# Current CPU provers on c7a.24xlarge

Current adopted Zopro takes **11.966 ms for 1×2 at 32 workers**; pinned Aeglos takes **7.990 ms at 96 workers**. Aeglos remains faster for the primary proof at its best tested count. For 54→1, Zopro takes **126.887 ms at 64 workers**, versus Aeglos's **172.322 ms at 96**. Best-worker choices are descriptive.

At the same 32 workers, current Zopro improves on previous Zopro by **3.31% paired geometrically, 3/3 winning blocks**, with a median paired saving of **389.993 µs**. Primary improvements do not extend to 96 workers: current/previous is **1.02603**, with 0/3 wins. The small-proof scaling issue remains. This comparison adopts no new source; the previously adopted 171-file implementation stays in place. The original all-eight supplied M5 CPU 10× goal remains active, achieved 0/8.

## Method

One dedicated c7a.24xlarge, AMD EPYC 9R14, 96 physical cores without SMT. All engines use compact first-N CPU masks at 32/64/96 workers, inside the same 30 GiB container with swap disabled. Three fixed rotated/reversed fresh-process blocks cover all nine shapes. Primary processes run 101 timed requests after three primes; other shapes run 17 after three. All 243 latency processes finish before six separate 199 Hz warm user-cycle profiles, primary32/96 for each engine, 251 requests after three primes.

The current adopted source is freshly built for `target-cpu=znver4`, with all other retained flags. Previous is the exact prior AWS-tested Zopro executable; Aeglos is the pinned `0290613` Sorted/prepared-key executable from earlier comparisons, not a claim about latest upstream. Existing setup and native-budget policies are preserved. Aeglos reports prepared-key cache hits for timed calls. SDK adapters preserve each engine's timer and normalize output afterward. Witness solving, fresh proof randomness, proving, internal verification and serialization are timed. Loading/setup/primes are excluded from latency and included in lifetime RSS.

Every SDK call verifies internally; final exports also pass independent Gnark checks. No build, helper, hash, archive or second benchmark overlaps timed capture. Independent verifiers run between completed processes. Three blocks provide point estimates and wins; no 95% confidence or universal guarantee is claimed. Nothing is excluded, extended or successfully recaptured. Same-worker ratios and cross-worker best-tested comparisons remain separate.

## Same-worker latency

Times are medians of three process medians in milliseconds. Ratios are geometric means of within-block pairs; differences between displayed medians need not agree. No minimum gain floor is applied.

|Shape|Workers|Previous Zopro ms|Current Zopro ms|Aeglos ms|Current/previous|Wins|Current/Aeglos|Wins|
|---|---:|---:|---:|---:|---:|---:|---:|---:|
"#,
    );
    let shapes = [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
        "merge-54x1",
    ];
    for shape in shapes {
        for workers in [32, 64, 96] {
            let c = &a["cases"][format!("{shape}-w{workers}")];
            writeln!(
                md,
                "|{shape}|{workers}|{:.6}|{:.6}|{:.6}|{:.6}|{}/3|{:.6}|{}/3|",
                num(&c["modes"]["previous"]["ms"]),
                num(&c["modes"]["current"]["ms"]),
                num(&c["modes"]["aeglos"]["ms"]),
                num(&c["pairs"]["current/previous"]["geometric_ratio"]),
                c["pairs"]["current/previous"]["wins"],
                num(&c["pairs"]["current/aeglos"]["geometric_ratio"]),
                c["pairs"]["current/aeglos"]["wins"]
            )
            .unwrap();
        }
    }
    md.push_str("\nAt 32 workers, primary current/Aeglos is **1.009442**, with 1/3 wins: this run does not establish a same-worker primary win either. At 64 and 96 workers, current/Aeglos is1.423340/1.770117, with 0/3 wins in each. The prospective primary win condition against both controls is unmet. Current/previous original-eight geometric aggregates are0.982506/0.996596/0.993811 at32/64/96 workers, with3/3,3/3and2/3 block wins. Positive small aggregate gains are retained alongside individual regressions, including16×2 at64workers and primary96. This is a comparison, not a new implementation-adoption gate.\n\n## Best tested configurations and resources\n\nEach engine selects the lowest displayed median across the three tested counts. Whole-process RSS is the maximum across blocks for that selected configuration, including loading/setup/primes. CPU is summed warm user+system time per request. Neither is minimum RAM or concurrent throughput.\n\n|Shape|Previous ms/workers|Current ms/workers|Aeglos ms/workers|Current CPU ms|Aeglos CPU ms|Current RSS MiB|Aeglos RSS MiB|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for shape in shapes {
        let b = &a["best_tested"][shape];
        writeln!(
            md,
            "|{shape}|{:.3}/{}|{:.3}/{}|{:.3}/{}|{:.3}|{:.3}|{:.2}|{:.2}|",
            num(&b["previous"]["ms"]),
            b["previous"]["workers"],
            num(&b["current"]["ms"]),
            b["current"]["workers"],
            num(&b["aeglos"]["ms"]),
            b["aeglos"]["workers"],
            num(&b["current"]["cpu_ms_per_request"]),
            num(&b["aeglos"]["cpu_ms_per_request"]),
            num(&b["current"]["maxrss_mib"]),
            num(&b["aeglos"]["maxrss_mib"])
        )
        .unwrap();
    }
    md.push_str("\nAt their best tested counts, Zopro has lower latency than Aeglos on8×2and larger cases here; Aeglos has lower latency on1×2,2×2and4×2. Primary RSS is120.30MiB Zopro versus474.48MiB Aeglos;54→1 is1794.83MiB versus13063.67MiB. These ratios compare whole-process peaks, including setup. They do not imply that adding tables would fix Zopro's scaling, that RAM capacity is the only speed difference, or that either engine has a universal memory bound. Full per-core CPU/RSS values are retained below and in analysis.json.\n\n|Shape/workers|CPU previous/current/Aeglos ms per request|RSS previous/current/Aeglos MiB|\n|---|---:|---:|\n");
    for shape in shapes {
        for workers in [32, 64, 96] {
            let c = &a["cases"][format!("{shape}-w{workers}")];
            writeln!(
                md,
                "|{shape}/{workers}|{:.3}/{:.3}/{:.3}|{:.2}/{:.2}/{:.2}|",
                num(&c["modes"]["previous"]["cpu_ms_per_request"]),
                num(&c["modes"]["current"]["cpu_ms_per_request"]),
                num(&c["modes"]["aeglos"]["cpu_ms_per_request"]),
                num(&c["modes"]["previous"]["maxrss_mib"]),
                num(&c["modes"]["current"]["maxrss_mib"]),
                num(&c["modes"]["aeglos"]["maxrss_mib"])
            )
            .unwrap();
        }
    }
    md.push_str("\n## Warm profiles and interpretation\n\nThe six separate profiles contain **178,577 samples**, zero lost records and one sampled SDK process per capture. Sampled shares are not removable wall-time fractions or exclusive critical-path costs. The sampled process and observer differ from each unprofiled latency run. Average occupied CPUs is warm summed CPU time divided by warm loop wall time; it is not a fraction of useful arithmetic.\n\n|Primary workers|Engine|Samples|CPU ms/request|Warm loop ms/request|Average occupied CPUs|Epoch %|Scheduler %|\n|---|---|---:|---:|---:|---:|---:|---:|\n");
    for workers in [32, 96] {
        let name = format!("1x2-w{workers}");
        for engine in ["previous", "current", "aeglos"] {
            let p = &a["profile_comparison"][&name][engine];
            let epoch = p["category_percent"]["epoch"]
                .as_f64()
                .map(|x| format!("{x:.2}"))
                .unwrap_or_else(|| "—".into());
            let scheduler = p["category_percent"]["scheduler"]
                .as_f64()
                .map(|x| format!("{x:.2}"))
                .unwrap_or_else(|| "—".into());
            writeln!(
                md,
                "|{workers}|{engine}|{}|{:.3}|{:.3}|{:.2}|{epoch}|{scheduler}|",
                p["samples"],
                num(&p["cpu_ms_per_request"]),
                num(&p["wall_ms_per_request"]),
                num(&p["mean_occupied_logical_cpus"])
            )
            .unwrap();
        }
    }
    md.push_str("\nThe generic categories match Zopro symbols; Aeglos appears as100%other in that matcher, which is a categorization limitation rather than missing samples or absent arithmetic. Its retained flat symbols show Pool::work at17.48%/26.51% for32/96workers, PairBatch<Lane1>::finish at12.17%/10.17%, load at10.85%/9.02% and Lane1::invert_all at10.37%/9.50%. Pool::work includes dispatch and waiting and is not directly comparable to the sum of Zopro's epoch/scheduler categories.\n\nCurrent Zopro at96workers has49.55%epoch and12.52%scheduler weight, versus8.03%and5.26%at32. Its average occupied CPUs rise21.48→53.03 while latency worsens; Aeglos rises27.36→67.25 while latency improves. This confirms a scaling difference with current source and motivates examining task readiness, long indivisible work and coordination. It does not prove that removing a sampled fraction would recover that wall time, or establish a cache, DRAM, fault, fence or syscall cause. Prior failed empty-victim, epoch-collection, guard-reuse, retained-deque and idle-policy experiments remain negative evidence; they are not revived unchanged by these profiles.\n\nThe memory cgroup records no high/max/OOM/kill/swap event. Host swap is absent and container swap.max is0. Aggregate container peak is13,793,710,080bytes across the entire campaign, not a per-engine RSS measurement. Thus this run did not force a memory-capacity failure or reclaim threshold. Memory-access costs remain possible and were not isolated. The96-CPU quota records7throttle periods/47,753µs summed throttling over the container lifetime; no per-engine attribution is made and zero throttling is not claimed. The profiles and CPU occupancy do not support reducing the explanation to RAM capacity alone.\n\n## Supplied M5 references\n\nReferences are supplied historical M5 timings, not fresh Apple measurements. Ratios use current Zopro's best tested worker count. No54→1 reference was supplied.\n\n|Shape|Current C7a ms/workers|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|Required CPU10× ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for shape in shapes {
        let b = &a["best_tested"][shape]["current"];
        let c = &a["cases"][b["case"].as_str().unwrap()];
        if let Some(cpu) = c["reference"]["cpu_ms"].as_f64() {
            let metal = num(&c["reference"]["metal_ms"]);
            let t = num(&b["ms"]);
            writeln!(
                md,
                "|{shape}|{t:.6}/{}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|",
                b["workers"],
                cpu / t,
                metal / t,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(
                md,
                "|{shape}|{:.6}/{}|—|—|—|—|—|",
                num(&b["ms"]),
                b["workers"]
            )
            .unwrap();
        }
    }
    md.push_str("\n**0/8 rows meet the supplied CPU10× objective.** The primary target is3.7ms, so a3.31%improvement at32workers is useful progress and remains far from completion.\n\n## Validation, identities and lifecycle\n\nThe current C7a build passes33unfiltered field tests,116core tests, two raw and six cubic prover tests, plus generated-field consistency. All171snapshot/production source bytes match the adopted manifest. Previous Zopro and Aeglos reuse exact inherited qualified artifacts. Current SDK SHA256 is6b5bb9d6f540db07a3de1eb15ad180af8b9499166414d5f9a67e99f310a1789e; previous019ca258641c0cde9a9915028ceaa6c686388dce736b96d154ece118fca8c7e7; pinned Aeglos driver eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b. The pinned pool source hash matches its retained build receipt; its source structure does not alone establish a performance cause.\n\nLocal exact-container preflight adds108SDKcalls/27exports. Cloud latency adds7,128calls/243exports; profiles1,524calls/six exports. Total **8,760SDKcalls,276distinct independent Gnark exports and552proof/public members**, global ledger **9,904→10,180**. Every export is164bytes, matches its exact fixture public hash, and is fresh against all prior/new identities. All276 are independently reverified locally after retrieval; repeat verification is counted once. The6,425,870-byte archive matches remote SHA2561cce916c833901654ce1e6aad9f50a83f3511e54367a1d22b8c8a6ef3d572cd2. Raw perf data stays local with hashes.\n\nA local mount permission repair occurs before any SDK call. A packaging helper then creates an incorrect destination-account tag; publication fails before provisioning. Retagging the same immutable image fixes publication without rebuilding or repeating proofs. An offline analyzer ownership error is fixed before post-transport verification. Original failures and repairs are retained; none changes prover code, flags, successful tests or measured captures.\n\nExactly one owned c7a.24xlarge is launched, with a90-minute self-termination timer and delete-on-termination volume. After retrieval it is terminated; the owned volume is absent, the unique ECRtag and private transfer object/bucket are removed, and private presigned/Docker credential files are deleted. Readback finds zero live TemporaryProverBenchmark instances in the region. No unowned or production resource is modified. This is a container CPU benchmark, not a Nitro-enclave, phone, ARM, forester250/4999, minimum-RAM, sustained-throughput or concurrent-server result.\n\nThe next optimization needs to address the observed C7a small-proof parallelism/coordination gap while preserving low-memory and concurrent-call behavior. Review prior scheduling/queue failures and real task dependencies before choosing a distinct experiment; further isolated arithmetic gains must be judged in complete proofs. No source promotion or unchanged acceptance rerun follows from this comparison.\n\n[Protocol](artifacts/design.md), [all cases and paired results](artifacts/analysis.json), [raw rows](artifacts/remote-results/output/measure/performance/rows.json), [native/source identity](artifacts/aws-accepted-manifest.json), [proof inventory](artifacts/proof-inventory.json), [post-transport verification](artifacts/post-transport-gnark.log), [cleanup readback](artifacts/cleanup-complete.json), [raw profile hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).\n");
    for (before, after) in [
        ("is1.423340/1.770117", "is 1.423340/1.770117"),
        (
            "are0.982506/0.996596/0.993811 at32/64/96",
            "are 0.982506/0.996596/0.993811 at 32/64/96",
        ),
        ("with3/3,3/3and2/3", "with 3/3, 3/3 and 2/3"),
        ("including16×2 at64workers", "including 16×2 at 64 workers"),
        ("on8×2and", "on 8×2 and"),
        ("on1×2,2×2and4×2", "on 1×2, 2×2 and 4×2"),
        (
            "is120.30MiB Zopro versus474.48MiB Aeglos;54→1 is1794.83MiB versus13063.67MiB",
            "is 120.30 MiB Zopro versus 474.48 MiB Aeglos; 54→1 is 1794.83 MiB versus 13063.67 MiB",
        ),
        ("as100%other", "as 100% other"),
        (
            "at17.48%/26.51% for32/96workers",
            "at 17.48%/26.51% for 32/96 workers",
        ),
        (
            "at12.17%/10.17%, load at10.85%/9.02% and Lane1::invert_all at10.37%/9.50%",
            "at 12.17%/10.17%, load at 10.85%/9.02% and Lane1::invert_all at 10.37%/9.50%",
        ),
        (
            "at96workers has49.55%epoch and12.52%scheduler",
            "at 96 workers has 49.55% epoch and 12.52% scheduler",
        ),
        ("versus8.03%and5.26%at32", "versus 8.03% and 5.26% at 32"),
        ("rise21.48→53.03", "rise 21.48→53.03"),
        ("rises27.36→67.25", "rises 27.36→67.25"),
        ("swap.max is0", "swap.max is 0"),
        ("is13,793,710,080bytes", "is 13,793,710,080 bytes"),
        (
            "The96-CPU quota records7throttle periods/47,753µs",
            "The 96-CPU quota records 7 throttle periods/47,753 µs",
        ),
        (
            "target is3.7ms, so a3.31%improvement at32workers",
            "target is 3.7 ms, so a 3.31% improvement at 32 workers",
        ),
        (
            "passes33unfiltered field tests,116core",
            "passes 33 unfiltered field tests, 116 core",
        ),
        ("All171snapshot/production", "All 171 snapshot/production"),
        ("is6b5bb9", "is 6b5bb9"),
        ("previous019ca", "previous 019ca"),
        ("adds108SDKcalls/27exports", "adds 108 SDK calls/27 exports"),
        ("adds7,128calls/243exports", "adds 7,128 calls/243 exports"),
        ("profiles1,524calls/six", "profiles 1,524 calls/six"),
        ("8,760SDKcalls,276distinct", "8,760 SDK calls, 276 distinct"),
        ("and552proof/public", "and 552 proof/public"),
        ("export is164bytes", "export is 164 bytes"),
        ("All276", "All 276"),
        ("The6,425,870-byte", "The 6,425,870-byte"),
        ("remote SHA2561cce", "remote SHA256 1cce"),
        ("a90-minute", "a 90-minute"),
        ("ECRtag", "ECR tag"),
        ("forester250/4999", "forester 250/4999"),
    ] {
        md = md.replace(before, after);
    }
    let close = json!({"status":"complete current C7a comparison","source_files_unchanged":171,"SDK_requests":8760,"distinct_independent_exports":276,"proof_public_members":552,"post_transport_verified":276,"next_prior_count":10180,"profiles":6,"cycle_samples":178577,"cleanup_complete":true,"instance_terminated":true,"adopted_new_source":false,"goal_achieved":false,"original8_CPU10x_rows":0});
    save(r.join("closeout.json"), &close);
    let _ = after;
    fs::create_dir_all(d).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!("Closed current C7a comparison: {} archive hashes;8760SDK276Gnark;ledger10180;cleanupcomplete;goal0/8",index.len());
}
