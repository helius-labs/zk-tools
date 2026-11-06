use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-capture176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-callers176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-split-redc-capture176";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash(p: impl AsRef<Path>) -> String {
    digest(&fs::read(p).unwrap())
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else if p.is_file() {
            out.push(p);
        }
    }
    out.sort();
    out
}
fn index(p: &Path) -> Value {
    json!({"local_path":p,"bytes":fs::metadata(p).unwrap().len(),"sha256":hash(p)})
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    json!({"bytes":b.len(),"sha256":digest(&b),"utf8":std::str::from_utf8(&b).unwrap()})
}
fn iv(v: &Value) -> String {
    format!(
        "{:.9} [{:.9}, {:.9}], {}/15",
        v["median"].as_f64().unwrap(),
        v["low"].as_f64().unwrap(),
        v["high"].as_f64().unwrap(),
        v["wins"]
    )
}
fn interval(x: &[f64]) -> Value {
    assert_eq!(x.len(), 15);
    let mut s = x.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[7],"low":s[3],"high":s[11],"wins":x.iter().filter(|&&v|v<1.).count(),"ratios":x})
}
fn main() {
    assert!(!Path::new(D).exists());
    let root = Path::new(R);
    let a = load(root.join("analysis.json"));
    let review = load(root.join("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(a["gate_passed"], false);
    assert_eq!(review["gate_passed"], false);
    assert_eq!(load(root.join("runs/result.json"))["status"], "complete");
    assert_eq!(review["analysis_sha256"], hash(root.join("analysis.json")));
    assert_eq!(a["failed_checks"].as_array().unwrap().len(), 21);
    assert_eq!(review["failed_timing_checks"].as_array().unwrap().len(), 21);
    let mut classes = BTreeMap::new();
    for f in a["failed_checks"].as_array().unwrap() {
        *classes
            .entry(f.as_str().unwrap().rsplit('/').next().unwrap().to_string())
            .or_insert(0usize) += 1;
    }
    assert_eq!(
        classes,
        BTreeMap::from([
            ("calibration".into(), 4),
            ("nonregression".into(), 15),
            ("gain".into(), 2)
        ])
    );
    let mut worker_aggregates = BTreeMap::new();
    for w in [1, 16, 32] {
        let rows = a["rows"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["context"].as_str().unwrap().ends_with(&format!("-w{w}")))
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 8);
        for den in ["OFF", "frozen"] {
            let xs = (0..15)
                .map(|b| {
                    (rows
                        .iter()
                        .map(|r| {
                            r["pairs"][format!("ON/{den}")]["ratios"][b]
                                .as_f64()
                                .unwrap()
                                .ln()
                        })
                        .sum::<f64>()
                        / 8.)
                        .exp()
                })
                .collect::<Vec<_>>();
            worker_aggregates.insert(format!("w{w}/ON/{den}"), interval(&xs));
        }
        if w != 32 {
            for r in rows {
                for den in ["OFF", "frozen"] {
                    assert!(r["pairs"][format!("ON/{den}")]["high"].as_f64().unwrap() < 1.);
                }
            }
        }
    }
    let mut peak = 0f64;
    let mut min_target = 1f64;
    let mut max_unknown = 0f64;
    for row in a["rows"].as_array().unwrap() {
        peak = peak.max(row["max_observed_process_RSS_MiB"]["ON"].as_f64().unwrap());
    }
    for p in a["profiles"].as_object().unwrap().values() {
        assert_eq!(p["qualified"], true);
        assert_eq!(p["zero_lost"], true);
        assert_eq!(p["worker_share"], 1.0);
        min_target = min_target.min(p["target_share"].as_f64().unwrap());
        max_unknown = max_unknown.max(p["unknown_share"].as_f64().unwrap());
    }
    assert_eq!(peak, 18.75);
    assert_eq!(review["total_profile_samples"], 119477);
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    save(
        art.join("worker-topology-descriptive.json"),
        &json!(worker_aggregates),
    );
    let mut report=String::from("# Split REDC planes: complete-caller comparison\n\n**Not adopted: the required 32-worker gain and nonregression gate fails.** The all-context aggregate improves by about 2.1% against fresh OFF, and every 1/16-worker context improves against both controls. Those gains are retained. They do not overcome the declared 32-worker failures or establish a full-proof improvement. Production remains unchanged.\n\n## Decision\n\nThe fixed gate fails **21 of 76 checks**: four control calibrations, 15 nonregression checks and both LOGICAL32 gain checks. All four calibration failures are at 32 workers: G1-200, G1-352, G2-200 and G2-201. No observation, context, interval or threshold is replaced. This is not a globally calibrated campaign.\n\nThe larger G2 32-worker controls at 352 and 353 points do calibrate. ON loses all 15 paired blocks against both controls at each size; ON/OFF medians are 4.64% and 3.69% slower respectively. These losses cannot be dismissed as the unrelated control failures elsewhere. The implementation is closed without a topology-only selection, SDK trial, source adoption or unchanged retry.\n\n| Declared aggregate | ON/control ratio [marginal interval], wins |\n|---|---|\n");
    for (name, v) in a["aggregates"].as_object().unwrap() {
        report.push_str(&format!("| {name} | {} |\n", iv(v)));
    }
    report.push_str("\nThe ALL gain checks pass. Every individual 1/16-worker context also has its ON/OFF and ON/frozen interval below parity; these are real component improvements. The primary LOGICAL32 aggregate includes both curves and all four sizes, as declared for a global field change. No smaller successful subset becomes the adoption scope.\n\nThe following worker aggregates summarize the same complete data descriptively; they add no new measurements or decision gates. Each includes all eight curve/size contexts.\n\n| Worker summary | ON/control ratio [marginal interval], wins |\n|---|---|\n");
    for (name, v) in &worker_aggregates {
        report.push_str(&format!("| {name} | {} |\n", iv(v)));
    }
    report.push_str("\n## Complete timings\n\nTimes are median ns per complete batch wave, not proof latency or concurrent-proof throughput. Paired ratios use the same fixed block and need not equal ratios of displayed medians. All 24 contexts remain.\n\n| Context | Frozen ns/wave | OFF ns/wave | ON ns/wave | ON/OFF ratio [interval], wins | ON/frozen ratio [interval], wins | OFF/frozen ratio [interval], wins |\n|---|---:|---:|---:|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let t = &row["median_ns_per_wave"];
        report.push_str(&format!(
            "| {} | {:.3} | {:.3} | {:.3} | {} | {} | {} |\n",
            row["context"].as_str().unwrap(),
            t["frozen"].as_f64().unwrap(),
            t["OFF"].as_f64().unwrap(),
            t["ON"].as_f64().unwrap(),
            iv(&row["pairs"]["ON/OFF"]),
            iv(&row["pairs"]["ON/frozen"]),
            iv(&row["pairs"]["OFF/frozen"])
        ));
    }
    report.push_str("\nAll failed checks:\n\n");
    for f in a["failed_checks"].as_array().unwrap() {
        report.push_str(&format!("- `{}`\n", f.as_str().unwrap()));
    }
    report.push_str("\n## What changed and what the counters show\n\nThe [qualified kernel](../2026-11-06-split-redc-native176/README.md) keeps product and reduction coefficients in two planes for the first four CIOS rows, merges before the original final row, and retains the final representation/corrections. Its source DAG and partial register-only chains shorten, but the actual [caller code](../2026-11-06-split-redc-callers176/README.md) adds vector additions and stack operands with the same IFMA counts. There is no new heap buffer, lookup table, key/point copy, extra plan or production pool.\n\n| PMU context | ON/OFF retired instructions | ON/OFF cycles |\n|---|---:|---:|\n");
    for (name, v) in review["PMU_ON_OFF_diagnostics"].as_object().unwrap() {
        report.push_str(&format!(
            "| {name} | {:+.3}% | {:+.3}% |\n",
            100. * (v["instructions:u"]["ON_OFF_ratio"].as_f64().unwrap() - 1.),
            100. * (v["cycles:u"]["ON_OFF_ratio"].as_f64().unwrap() - 1.)
        ));
    }
    report.push_str("\nThese fixed post-latency diagnostics support the measured tradeoff: solo cycle improvements coexist with more instructions and G2 32-worker cycle regressions. They do not isolate execution ports, spill latency, cache levels, store forwarding or a cause for the control variability. Generic cache-miss/branch counters and absolute counts remain in the JSON; a large ratio from a low baseline is not a cache-capacity conclusion. The 32-worker G2 cases keep roughly 31–32 CPUs busy and have low timed minor-fault rates. This is utilization of the loaded caller test, not the whole proof pipeline, and does not establish a RAM-limit explanation.\n\n## Protocol and qualification\n\nAll 1,279 workflow commands succeed once: 72 preflights plus checker, 1,080 latency processes in 15 paired blocks, 18 PMU calls, 18 cycle profiles and 90 offline ownership/decode steps. All 1,188 caller processes pass complete generator/output-hash, unchanged-capacity and zero-timed-allocation checks. Campaign and independent Rust reviewer were frozen before clocks; each independently reconstructs all 76 checks from the saved observations. Sorted zero-based ranks 3/7/11 of 15 are marginal intervals, without family-wise coverage. There is no minimum positive-gain floor.\n\nEach process uses three 128-round primes. Latency rounds per worker are 32,768 for 200/201 and 16,384 for 352/353 points. Workers use CPU 2 solo, CPUs 0–15 at 16, or 0–31 at 32, with a nonexclusive host. No owned build/test/second benchmark/hash/archive overlaps latency. One small checkpoint write was made during correctness preflights (progress still at preflight 31), as permitted by the protocol.\n\nAfter all latency, grouped user cycles/instructions/cache-miss/branch-miss counters have at least 90% running coverage. Profiles use fixed 16,000,000-cycle periods with FIFO timed-region control. All recordings precede ownership/hash/decode work; each stream is decoded once. Executable/PID/TID/FORK/MMAP/CPU/period checks pass, and no successful stage is repeated.\n\n");
    report.push_str(&format!("All **18 profiles qualify with 119,477 samples**, zero lost events and all worker samples on assigned workers. Minimum native-kernel/inverse attribution is {:.4}% and maximum unknown attribution {:.4}%. Flat samples have skid and are not exclusive costs or removable proof time.\n\n",100.*min_target,100.*max_unknown));
    report.push_str("## Resources\n\nScratch capacities match across all arms and before/after timing; no timed heap allocations occur. Candidate maximum observed process RSS is **18.75 MiB** in this small caller matrix, including fixture setup and threads. This is not full-proof peak RAM, forced-memory evidence or minimum RAM. CPU-time/wall ratios and timed minor faults are diagnostic only.\n\n| Context | OFF / ON peak RSS MiB | OFF / ON median busy CPUs | OFF / ON minor faults per wave |\n|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let rss = &row["max_observed_process_RSS_MiB"];
        let busy = &row["median_busy_CPUs"];
        let f = &row["median_minor_faults_per_wave"];
        report.push_str(&format!(
            "| {} | {} / {} | {:.4} / {:.4} | {:.9} / {:.9} |\n",
            row["context"].as_str().unwrap(),
            rss["OFF"],
            rss["ON"],
            busy["OFF"].as_f64().unwrap(),
            busy["ON"].as_f64().unwrap(),
            f["OFF"].as_f64().unwrap(),
            f["ON"].as_f64().unwrap()
        ));
    }
    report.push_str("\n## Unchanged full-proof baseline\n\nThese are inherited accepted-176 local 32-worker proof times, not this candidate’s proof results. The accepted physical-16 calibration engineering exception and prior false full gates remain documented; this study does not reopen them. SDK6d83 and canonical proof ledger 30,846 stay current. The inherited 132.75 MiB RSS is not a new peak or minimum RAM.\n\n| Shape | Accepted CPU ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU 10× target ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (name, ms, cpu, metal) in [
        ("1x2", 7.928660, 37., 24.),
        ("2x2", 11.151932, 59., 30.),
        ("4x2", 16.086386, 109., 48.),
        ("8x2", 24.179453, 189., 74.),
        ("12x2", 33.884455, 284., 101.),
        ("16x2", 40.077381, 335., 124.),
        ("24x2", 65.850777, 528., 176.),
        ("merge24", 65.060937, 507., 175.),
    ] {
        report.push_str(&format!(
            "| {name} | {ms:.6} | {cpu:.0} | {:.3}× | {metal:.0} | {:.3}× | {:.1} |\n",
            cpu / ms,
            metal / ms,
            cpu / 10.
        ));
    }
    report.push_str("\nInherited merge54 remains 161.883899 ms without a supplied M5 reference. The original all-eight CPU 10× goal remains active and unmet (0/8). No new SDK/Gnark proof, AWS/Aeglos, portability, huge-circuit, pressure, swap, minimum-RAM or concurrent-proof measurement is claimed.\n\n## Artifacts and continuation\n\nThe [decision](artifacts/decision.md) closes this exact implementation while preserving its positive lower-thread results. The [next step](artifacts/next.md) requires distinct useful-work removal with a full-proof impact argument, not a topology/layout/annotation retry or another generic profile sweep.\n\n[Analysis and all ratios](artifacts/analysis.json), [independent review](artifacts/review.json), [worker summaries](artifacts/worker-topology-descriptive.json), [protocol](artifacts/protocol.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [profile symbols](artifacts/profile-symbols.json), [host metadata](artifacts/host-public.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json) and [checksums](SHA256SUMS). Raw recordings/streams remain locally under `/home/ubuntu/src/zopro-perf-work/split-redc-capture176`. The selected UTF-8 archive excludes executable/library/key/witness/proof blobs; the public host file omits the unrelated process list.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "campaign.rs",
        "plan.json",
        "pins.json",
        "identities.json",
        "protocol.md",
        "decision.md",
        "next.md",
        "profile-symbols.json",
        "publish.rs",
    ] {
        fs::copy(root.join(f), art.join(f)).unwrap();
    }
    fs::copy(
        Path::new(Q).join("qualification.json"),
        art.join("caller-qualification.json"),
    )
    .unwrap();
    let mut host = load(root.join("host.json"));
    host.as_object_mut().unwrap().remove("processes");
    host["original_local_identity"] = index(&root.join("host.json"));
    save(art.join("host-public.json"), &host);
    let raw = walk(&root.join("perf"))
        .into_iter()
        .map(|p| {
            (
                p.strip_prefix(root).unwrap().display().to_string(),
                index(&p),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(raw.len(), 18);
    save(art.join("raw-profile-index.json"), &json!(raw));
    let mut members = BTreeMap::new();
    for f in [
        "campaign.rs",
        "review.rs",
        "publish.rs",
        "migrate.rs",
        "protocol.md",
        "decision.md",
        "next.md",
        "prepare.stdout.log",
        "prepare.stderr.log",
        "capture.stdout.log",
        "capture.stderr.log",
        "finish.stdout.log",
        "finish.stderr.log",
        "review.stdout.log",
        "review.stderr.log",
    ] {
        members.insert(f.to_string(), member(&root.join(f)));
    }
    let mut streams = BTreeMap::new();
    for p in walk(&root.join("runs")) {
        let rel = p.strip_prefix(root).unwrap().display().to_string();
        if p.file_name().unwrap() == "stdout.log"
            && (rel.starts_with("runs/decode-") || rel.starts_with("runs/metadata-"))
        {
            streams.insert(rel, index(&p));
        } else {
            members.insert(rel, member(&p));
        }
    }
    for p in walk(&root.join("stat")) {
        members.insert(
            p.strip_prefix(root).unwrap().display().to_string(),
            member(&p),
        );
    }
    assert_eq!(streams.len(), 36);
    save(art.join("profile-stream-index.json"), &json!(streams));
    let bytes = serde_json::to_vec(&members).unwrap();
    let archive = art.join("helpers-and-receipts.json.gz");
    let mut zip = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(&archive).unwrap())
        .spawn()
        .unwrap();
    zip.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(zip.wait().unwrap().success());
    let unpack = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(&archive)
        .output()
        .unwrap();
    assert!(unpack.status.success());
    assert_eq!(unpack.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&unpack.stdout).unwrap();
    for entry in decoded.as_object().unwrap().values() {
        let text = entry["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(text.len() as u64, entry["bytes"].as_u64().unwrap());
        assert_eq!(digest(text), entry["sha256"]);
    }
    save(
        art.join("publication.json"),
        &json!({"status":"pass","commands":1279,"callers":1188,"timing_checks":76,"failed_timing_checks":21,"failure_classes":classes,"profiles":18,"profile_samples":119477,"UTF8_archive_members":members.len(),"archive_roundtrip_verified":true,"archive_sha256":hash(&archive),"raw_recordings_local":18,"decoded_streams_local":36,"gate_passed":false,"adopted":false,"new_proofs":0,"proof_ledger":30846,"publisher_binary_sha256":hash(root.join("publish"))}),
    );
    let files = walk(Path::new(D));
    let mut sums = String::new();
    for p in &files {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    save(
        root.join("publication.json"),
        &json!({"status":"pass","report":D,"files":files.len()+1,"members":members.len(),"roundtrip_exact":true,"gate_passed":false}),
    );
    println!(
        "published {} files/{}UTF8 members; all positives/failures retained; no adoption",
        files.len() + 1,
        members.len()
    );
}
