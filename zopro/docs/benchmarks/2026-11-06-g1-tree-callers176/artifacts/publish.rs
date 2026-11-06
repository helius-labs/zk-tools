use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tree-callers176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash_file(p: &Path) -> String {
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buffer[..n]);
    }
    format!("{:x}", h.finalize())
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = std::str::from_utf8(&b).expect("only explicit UTF8 helper/delta/receipt members");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
}
fn archive(path: &Path, members: &BTreeMap<String, Value>) {
    let bytes = serde_json::to_vec(members).unwrap();
    let mut p = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(path).unwrap())
        .spawn()
        .unwrap();
    p.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(p.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), v["sha256"]);
    }
}
fn interval(v: &Value) -> String {
    format!(
        "{:.6} [{:.6}, {:.6}], {}/15",
        v["median"].as_f64().unwrap(),
        v["low"].as_f64().unwrap(),
        v["high"].as_f64().unwrap(),
        v["wins"]
    )
}
fn main() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    let review = read(format!("{R}/review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(a["gate_passed"], review["gate_passed"]);
    assert_eq!(a["gate_passed"], false);
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report = String::from("# G1 binary tree-prefix complete-caller comparison\n\nThe native binary tree-prefix record route was compared with frozen and fresh controls in 27 unweighted/weighted contention contexts. **The predeclared gate did not pass: ten failed checks.** Large reused groups improved and used less scratch, but several small 32-worker contexts did not establish calibration or nonregression. No production change or full-proof speedup is claimed.\n\n");
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Implementation and measurement boundary\n\nThe [native/portable qualification](../2026-11-06-g1-tree-records176/README.md) is inherited without rerunning its successful kernel suites. The generic binary addition formulas and checked batch inverse are unchanged. Native G1 stores prefix/norm values in two SIMD vectors for every full group, rereads coordinates during the reverse pass and keeps the old five-vector Group allocation empty. Exact reservation avoids the allocator's minimum record capacity. G2 retains its existing threshold/reservation; portable G1 retains its original route. This is a scratch lifetime change, not a key lookup table. It is distinct from the earlier streaming-prefix and ternary experiments.\n\nA separate private caller snapshot adds a weighted adapter with prebuilt bases and stored projective output; normalization and oracle checks happen outside the clock. All 177 qualified sources are recovered exactly after removing two adapter appendices; all 176 accepted production sources remain exact. OFF and ON share that candidate snapshot and differ by the experimental cfg. Frozen uses accepted production bodies plus the same private adapters. Three dependent core libraries were needed for the new adapter; unchanged field/support libraries were reused. The original unweighted TreeCaller layout and body are exact. All final consumers use O3/native/fatLTO/one CGU.\n\nPreparation completed 14 commands: three core/wrapper-oracle/driver sequences and two harness commands. Each arm passed 88 new weighted wrapper Arkworks cases including full 254-bit values, exceptions, multiple/partial chunks and repeated calls. The harness ran 16 contexts at 1/4/16/32 workers with 512 KiB stacks and zero timing observations. Its three tests comprise one new broadcast test and two existing FIFO tests. Before capture, source review found that a shared atomic allocation counter would introduce artificial contention. Original preparation artifacts were kept intact; five changed TLS consumer/harness commands passed. All performance captures use those final TLS binaries. No unchanged core or original kernel suite was rebuilt/rerun for this correction. The initial preparation reader's expected harness count was corrected from one to three; retained originals and the actual outputs are archived.\n\nThe benchmark-only Rayon pool uses broadcast callbacks with private fixtures pinned to workers. Public weighted_affine executes inside that pool. A coordinator and main thread control ready/start/finish/release barriers, and checks/serialization happen after release. This avoids routing foreign worker calls through a single global Rayon worker. No production pool is added. The allocation counter sums per-worker TLS deltas, excluding main/coordinator allocations.\n\n## Frozen protocol and all timing results\n\nThe Rust/xtask plan was frozen before any new preflight timing. All 1,402 commands completed successfully once: 81 preflight callers and one checker; 1,215 latency callers (27 contexts × three arms × 15 paired blocks); 15 counter panels; 15 fixed-period profiles; and 75 offline ownership/decode/metadata/event/loss steps. Independent review reconstructed 1,326 caller outputs, 87 timing checks and 155,095 profile samples. No pilot, added blocks, favorable subset or unchanged reroll was used.\n\nEvery worker primes 24 calls. Tile257/384/511/2048 use fresh buffers; group35973/groups31x256/tails reuse buffers. Weighted128/256 call the actual public function with its allocations inside the clock, but base construction and normalization outside. Solo pins CPU2, 16 workers use physical cores 0–15 and 32 workers use all SMT threads 0–31. Workers duplicate complete public fixtures; ns/wave is elapsed time divided by rounds per worker, not aggregate calls or proof latency. This is contention evidence, not proof-level scaling or proof throughput.\n\nRatios are paired medians with intervals using sorted zero-based ranks 3/7/11 of 15 and wins. The intervals have marginal coverage, not simultaneous family-wise coverage. OFF/frozen calibration requires the whole interval in [0.97,1.03] in all 27 contexts. ON/OFF and ON/frozen nonregression require upper ≤1.01 in every context, including weighted. Large-group gains at both 16 and 32 workers and the four-main 32-worker geometric aggregate require upper <1 and at least 11 wins against both controls. There is no minimum gain floor.\n\n|Case|Workers|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF [interval], wins|ON/frozen [interval], wins|OFF/frozen calibration|\n|---|---:|---:|---:|---:|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let p = &row["pairs"];
        report.push_str(&format!(
            "|{}|{}|{:.2}|{:.2}|{:.2}|{}|{}|{}|\n",
            row["case"].as_str().unwrap(),
            row["workers"],
            row["median_ns_per_wave"]["frozen"].as_f64().unwrap(),
            row["median_ns_per_wave"]["OFF"].as_f64().unwrap(),
            row["median_ns_per_wave"]["ON"].as_f64().unwrap(),
            interval(&p["ON/OFF"]),
            interval(&p["ON/frozen"]),
            interval(&p["OFF/frozen"])
        ));
    }
    report.push_str(
        "\nFour-main aggregate (tile384, tile511, tile2048 and group35973) at 32 workers:\n\n",
    );
    for arm in ["OFF", "frozen"] {
        report.push_str(&format!(
            "- ON/{arm}: {}.\n",
            interval(&a["main32_aggregates"][arm])
        ));
    }
    report.push_str("\nAll failed checks:\n\n");
    for f in a["failed_checks"].as_array().unwrap() {
        report.push_str(&format!("- `{}`\n", f.as_str().unwrap()));
    }
    report.push_str("\n## Resources\n\nUnweighted capacity totals sum common points/counts/output, pair indices, legacy records, prefix records and fixed structs for one worker. All block/worker capacities match the preflights. Weighted production scratch is dropped inside the public call, so actual retained capacity is **unknown**, not zero; its unchanged typed source forecast is listed separately, not as measured memory. Reused cases perform zero measured worker allocations. Maximum RSS covers whole-process setup and workers across the 15 latency blocks and is not minimum RAM. Full three-arm capacities, CPU times, allocation counts, forecasts, RSS and faults are in the analysis.\n\n|Case|Workers|OFF/ON capacity B per worker|OFF/ON source forecast B|OFF/ON worker allocations per round|OFF/ON max RSS MiB|OFF/ON busy CPUs|OFF/ON minor faults per wave|\n|---|---:|---|---|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let cap = |arm: &str| {
            row["retained_capacity_bytes_first_worker"][arm]
                .as_array()
                .map(|v| {
                    v.iter()
                        .map(|v| v.as_u64().unwrap())
                        .sum::<u64>()
                        .to_string()
                })
                .unwrap_or("unknown".into())
        };
        let forecast = |arm: &str| {
            row["source_forecast_bytes_first_worker"][arm]
                .as_u64()
                .map(|n| n.to_string())
                .unwrap_or("—".into())
        };
        let pair = |key: &str| {
            format!(
                "{:.3} / {:.3}",
                row[key]["OFF"].as_f64().unwrap(),
                row[key]["ON"].as_f64().unwrap()
            )
        };
        report.push_str(&format!(
            "|{}|{}|{} / {}|{} / {}|{}|{}|{}|{}|\n",
            row["case"].as_str().unwrap(),
            row["workers"],
            cap("OFF"),
            cap("ON"),
            forecast("OFF"),
            forecast("ON"),
            pair("median_alloc_calls_per_worker_round"),
            pair("max_observed_process_RSS_MiB"),
            pair("mean_busy_CPUs_from_medians"),
            pair("median_minor_faults_per_wave")
        ));
    }
    report.push_str("\nBusy CPUs are the ratio of median process CPU time to median wall time. Minor faults are mappings resolved without disk reads. Neither establishes swap behavior or cache-level causality. Weighted forecasts are unchanged and cannot be interpreted as actual allocation bytes. No low-memory-device bound or universal cap is inferred.\n\n## CPU counters and profiles\n\nAll latency capture finished before counter/profile capture. Four grouped user events require ≥90% coverage. Profiles use FIFO-controlled measured regions and a fixed 16,000,000 user-cycle period. Qualification requires ≥1,000 samples, unknown ≤5%, no lost events and ≥95% samples on the declared workers. Every PID/TID is checked against binary MMAP ownership and FORK metadata; worker CPU assignments are exact, with other owned main/coordinator samples separately counted. Raw ownership changes preserve hashes. All 15 panels and all 15 profiles qualified; no capture was extended.\n\n|Panel|Cycles/wave|Instructions/wave|Cache misses/wave|Branch misses/wave|Qualified|\n|---|---:|---:|---:|---:|:---:|\n");
    for (name, p) in a["counters"].as_object().unwrap() {
        let e = &p["events"];
        report.push_str(&format!(
            "|{}|{:.1}|{:.1}|{:.1}|{:.1}|{}|\n",
            name,
            e["cycles:u"]["per_wave"].as_f64().unwrap(),
            e["instructions:u"]["per_wave"].as_f64().unwrap(),
            e["cache-misses:u"]["per_wave"].as_f64().unwrap(),
            e["branch-misses:u"]["per_wave"].as_f64().unwrap(),
            p["qualified"]
        ));
    }
    report.push_str("\n|Profile|Samples|Unknown share|Worker share|Other owned samples|Qualified|\n|---|---:|---:|---:|---:|:---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        report.push_str(&format!(
            "|{}|{}|{:.4}%|{:.4}%|{}|{}|\n",
            name,
            p["samples"],
            100. * p["unknown_share"].as_f64().unwrap(),
            100. * p["worker_share"].as_f64().unwrap(),
            p["other_owned_samples"],
            p["qualified"]
        ));
    }
    report.push_str("\nAt large-group/32 workers the reduction kernel receives 25,865 OFF and 14,006 ON fixed-period samples; outer Work::reduce receives 6,191 and 5,903. Absolute sampled periods suggest that most of the saving occurs inside reduce_layer, while outer copying/reduction work persists. Profiles are statistical observations, not exact instruction or memory-service attribution. Static final-consumer IFMA sites (OFF 600, ON 1,200) include both generic branches and are not executed-work counts; PMU instructions are the dynamic evidence.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&section[..section.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Storage and continuation\n\nRaw profiles (about 8.9 MiB) and all original workflow files (about 84 MiB) remain at `/home/ubuntu/src/zopro-perf-work/g1-tree-callers176/perf/` and `runs/`, indexed by byte size and SHA256. Git includes all latency outputs, full analysis, commands/exits, source deltas, symbols and counter CSVs. Only the 30 normal/metadata decode stdout streams are omitted from the receipt archive and indexed locally. No capture or decode was repeated for packaging.\n\nThe [next stage](artifacts/next.md) starts with actual production call coverage before selecting a genuinely different work/lifetime mechanism. The current failed gate does not select production adoption or a full-SDK candidate. No new proof, AWS/Aeglos, phone/ARM/TEE, pressure/swap, minimum-RAM or proof-throughput result is added. All eight CPU 10× requirements remain active and unmet.\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [decision](artifacts/decision.md), [design](artifacts/design.md), [TLS correction](artifacts/design-tls-addendum.md), [profile ownership declaration](artifacts/profile-thread-addendum.md), [declaration](artifacts/declaration.json), [final identities](artifacts/identities-tls.json), [source audit](artifacts/source-audit.json), [driver](artifacts/driver-tls.rs), [weighted adapter](artifacts/weighted.rs), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [symbols](artifacts/profile-symbols.json), [source deltas](artifacts/source-deltas.json.gz), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Source deltas reconstruct all 178 files of each snapshot using qualified/accepted base manifests; base paths/hashes and inherited qualification are bound in the archived source audits. Full snapshots/binaries remain local. No private key, witness, image or proof bytes, external paper text or executable/library blobs are embedded.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "decision.md",
        "next.md",
        "design.md",
        "design-tls-addendum.md",
        "profile-thread-addendum.md",
        "declaration.json",
        "identities-tls.json",
        "source-audit.json",
        "host-before.json",
        "driver-tls.rs",
        "weighted.rs",
        "perf_support.rs",
        "build-review.json",
        "consumer-review.json",
        "campaign.rs",
        "finish.rs",
        "plan.json",
        "pins.json",
        "profile-symbols.json",
        "publish.rs",
        "preparation.md",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let audit = read(format!("{R}/source-audit.json"));
    let q = read("/home/ubuntu/src/zopro-perf-work/g1-tree-records176/source-audit.json");
    let mut delta = BTreeMap::new();
    let mut total = 0;
    for (dir, key, base, hashes) in [
        (
            "caller-source",
            "candidate_source_hashes",
            "/home/ubuntu/src/zopro-perf-work/g1-tree-records176/source",
            "candidate_source_hashes",
        ),
        (
            "FROZEN-source",
            "frozen_source_hashes",
            "/home/ubuntu/src/zk-tools/zopro",
            "accepted_source_hashes",
        ),
    ] {
        for (rel, h) in audit[key].as_object().unwrap() {
            let path = Path::new(R).join(dir).join(rel);
            assert_eq!(hash_file(&path), h.as_str().unwrap());
            total += 1;
            if let Some(old) = q[hashes].get(rel) {
                assert_eq!(hash_file(&Path::new(base).join(rel)), old.as_str().unwrap());
                if old == h {
                    continue;
                }
            }
            delta.insert(format!("{dir}/{rel}"), member(&path));
        }
    }
    assert_eq!(total, 356);
    archive(&art.join("source-deltas.json.gz"), &delta);
    let mut helpers = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    let mut streams = BTreeMap::new();
    for dir in [
        "build-runs",
        "consumer-tls-runs",
        "initial-build-review",
        "runs",
        "stat",
    ] {
        for p in walk(&Path::new(R).join(dir)) {
            let parent = p.parent().unwrap().file_name().unwrap().to_string_lossy();
            if dir == "runs"
                && p.file_name().unwrap() == "stdout.log"
                && (parent.starts_with("decode-") || parent.starts_with("metadata-"))
            {
                streams.insert(p.strip_prefix(R).unwrap().display().to_string(),json!({"local_path":p,"bytes":fs::metadata(&p).unwrap().len(),"sha256":hash_file(&p),"storage":"local original stream, not Git-tracked"}));
                continue;
            }
            helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    assert_eq!(streams.len(), 30);
    fs::write(
        art.join("profile-stream-index.json"),
        serde_json::to_vec_pretty(&streams).unwrap(),
    )
    .unwrap();
    archive(&art.join("helpers-and-receipts.json.gz"), &helpers);
    let mut raw = BTreeMap::new();
    for (name, p) in a["profiles"].as_object().unwrap() {
        let path = Path::new(R).join("perf").join(format!("{name}.data"));
        let h = hash_file(&path);
        assert_eq!(h, p["raw_sha256"]);
        raw.insert(name.clone(),json!({"local_path":path,"raw_bytes":fs::metadata(&path).unwrap().len(),"sha256":h,"storage":"local original recording, not Git-tracked"}));
    }
    fs::write(
        art.join("raw-profile-index.json"),
        serde_json::to_vec_pretty(&raw).unwrap(),
    )
    .unwrap();
    let publication = json!({"review_passed":true,"gate_passed":false,"source_delta_UTF8_members":delta.len(),"source_snapshot_files_bound":total,"helper_receipt_UTF8_members":helpers.len(),"raw_profiles_indexed_locally":raw.len(),"decoded_streams_indexed_locally":streams.len(),"all_embedded_archive_hashes_and_lossless_roundtrips_verified":true,"preparation_commands":19,"capture_commands":1402,"threaded_callers":1326,"timing_checks":87,"new_proofs":0,"adopted":false,"goal_status":"active/unmet0/8"});
    fs::write(
        art.join("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
    )
    .unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash_file(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published {} files/{} UTF8 receipts/{} source deltas/15 raw+30 decoded indexes; hash+roundtrip PASS; gate=false/no adoption",walk(Path::new(D)).len(),helpers.len(),delta.len());
}
