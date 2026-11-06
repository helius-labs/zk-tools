use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-validated-capture176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/g2-validated-batch176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g2-validated-capture176";
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
    let mut v = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else if p.is_file() {
            v.push(p)
        }
    }
    v.sort();
    v
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
fn main() {
    assert!(!Path::new(D).exists());
    let root = Path::new(R);
    let a = load(root.join("analysis.json"));
    let review = load(root.join("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(a["gate_passed"], false);
    assert_eq!(a["gate_passed"], review["gate_passed"]);
    assert_eq!(load(root.join("runs/result.json"))["status"], "complete");
    assert_eq!(hash(root.join("analysis.json")), review["analysis_sha256"]);
    let nf = a["failed_checks"].as_array().unwrap().len();
    assert_eq!(nf, review["failed_timing_checks"].as_array().unwrap().len());
    let mut failclasses = BTreeMap::<String, usize>::new();
    for f in a["failed_checks"].as_array().unwrap() {
        *failclasses
            .entry(f.as_str().unwrap().rsplit('/').next().unwrap().into())
            .or_default() += 1;
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=format!("# Once-validated G2 batches: complete-caller comparison\n\n**Rejected: the candidate does not establish a complete-caller gain.** Its four-size 32-worker G2 aggregate is {:.3}% slower than fresh OFF and {:.3}% faster than the frozen accepted-source caller; both intervals include parity. Solo G2 regresses at every size. No SDK trial, proof-speed improvement or production change follows.\n\n## Decision and limitations\n\nThe preregistered gate fails **{nf}/{} timing checks**: {} calibration, {} nonregression and {} gain checks. The G1-352-w32 and G2-200-w32 control-calibration failures remain in the result, so this is not a globally calibrated campaign. Stable solo G2 controls do calibrate, and ON loses all 15 paired blocks at every solo size against both controls. Small favorable estimates elsewhere remain published. No subset, extension, reroll or threshold change replaces the result.\n\n",100.*(a["aggregates"]["G2/OFF"]["median"].as_f64().unwrap()-1.),100.*(1.-a["aggregates"]["G2/frozen"]["median"].as_f64().unwrap()),a["timing_checks"],failclasses["calibration"],failclasses["nonregression"],failclasses["gain"]);
    report.push_str("The [qualified implementation](../2026-11-06-g2-validated-batch176/README.md) validates every masked destination once before writes, then uses fixed-size groups and justified unchecked accesses through the same field arithmetic. Validation stays **inside every timed kernel call**. Original signs, scalar tails, inversion, scratch capacities and input representation remain. No descriptor, point/key copy, table, plan or pool is added. Fewer compiled checks are not themselves a measured benefit.\n\n## Timings\n\nThe table reports medians of 15 process observations, in ns per complete batch wave with all workers active. Ratios pair observations within each fixed block and need not equal ratios of displayed medians. These are component timings, not full-query/proof latency or concurrent-proof throughput.\n\n|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF ratio [interval], wins|ON/frozen ratio [interval], wins|OFF/frozen ratio [interval], wins|\n|---|---:|---:|---:|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let t = &row["median_ns_per_wave"];
        report.push_str(&format!(
            "|{}|{:.3}|{:.3}|{:.3}|{}|{}|{}|\n",
            row["context"].as_str().unwrap(),
            t["frozen"].as_f64().unwrap(),
            t["OFF"].as_f64().unwrap(),
            t["ON"].as_f64().unwrap(),
            iv(&row["pairs"]["ON/OFF"]),
            iv(&row["pairs"]["ON/frozen"]),
            iv(&row["pairs"]["OFF/frozen"])
        ));
    }
    report.push_str("\n|Aggregate|ON/control ratio [interval], wins|\n|---|---|\n");
    for (name, v) in a["aggregates"].as_object().unwrap() {
        report.push_str(&format!("|{name}|{}|\n", iv(v)));
    }
    report.push_str("\nAll failed checks:\n\n");
    for f in a["failed_checks"].as_array().unwrap() {
        report.push_str(&format!("- `{}`\n", f.as_str().unwrap()));
    }
    report.push_str("\n## Measurement protocol\n\nThe [protocol](artifacts/protocol.md), campaign and independent analyzer were frozen before the first observation. All 24 contexts are G1/G2 × 200/201/352/353 points × 1/16/32 workers, using three immutable executables. Frozen389307 is the audited accepted-source caller, distinct from the production SDK; its original field-input provenance remains in the qualification note. Fresh OFF/ON explicitly link the original field1200 and their matching new cores. Existing native tests, models and builds are inherited, not repeated.\n\nEach worker owns a public-generator fixture with seed19+worker. These are not arbitrary full-width curve coordinates; prior native field qualification covers full-width/lazy values. Three 128-round primes and reset precede timing. Every caller checks independent generator output, cross-arm hashes, equal retained capacities and zero timed allocations. Construction, reset and final checks are outside the timer. Timing includes dispatch and the complete kernel.\n\nThe diagnostic Rayon pool uses 512 KiB stacks and pins CPU2 solo, physical cores0–15 for16 workers and CPUs0–31 for32. Fixtures are dispatched through par_iter_mut and are not permanently bound to a worker TID. Fixed allocator environment is retained. Host topology, kernel, tools and governor observations are saved; frequency is not locked and exclusive host use is not established. No owned build, native test, second benchmark, verifier, packaging or large hash overlaps capture. Small checkpoint/log updates are bookkeeping.\n\nFifteen fixed blocks rotate/reverse contexts and arms. Rounds per worker are 32768 at200/201 and16384 at352/353. Rank intervals use sorted zero-based ranks3/7/11 of15 paired ratios; their coverage is marginal, not simultaneous family-wise. All24 OFF/frozen intervals must fit[0.97,1.03]; all48 ON/control upper nonregression bounds must be≤1.01. G2 logical32 across all four sizes and the all24-context aggregate require upper<1 and≥11/15 wins against each control. There is no positive-gain floor.\n\nAll 1279 commands succeed once:72 preflights plus1 checker,1080 latency,18 PMU,18 cycles and90 offline commands. All1188 callers pass output/capacity/oracle/allocation guards. Independent review reconstructs argv, environments, exits/output hashes, all paired ratios,76 timing checks and their failures. No native/capture/profile/decode/analyzer stage is repeated for publication.\n\n## CPU evidence\n\nThe final G2 function has3.62% fewer static instruction rows and35 fewer bounds-panic call sites, plus an upfront assertion path; IFMA count remains1450. Grouped PMU panels show about1.25–1.30% fewer executed G2 instructions, but higher cycle counts. The combined validation/iteration/access transformation therefore fails despite reducing instructions. The scan cost and other generated-code changes were not separately ablated; no individual causal cost is assigned. Generic cache-miss or fault counters do not establish a RAM-capacity bottleneck.\n\n|Context|ON/OFF cycles|ON/OFF instructions|ON/OFF cache misses|ON/OFF branch misses|\n|---|---:|---:|---:|---:|\n");
    for (name, e) in review["PMU_ON_OFF_diagnostics"].as_object().unwrap() {
        report.push_str(&format!(
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
            e["cycles:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            e["instructions:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            e["cache-misses:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            e["branch-misses:u"]["ON_OFF_ratio"].as_f64().unwrap()
        ));
    }
    report.push_str(&format!("\nDiagnostics follow all latency. Fixed panels cover each curve at200w1,200w32,352w32, all three arms. Profiles run2097152 rounds solo and65536 at32 workers; PMU uses one quarter. FIFO enable/disable brackets the timed interval. Counters are grouped user cycles,instructions,cache misses,branch misses, all at≥90% coverage. Profiles use cycles:u period16000000 with512 mmap pages.\n\nAll **18 profiles qualify with {} samples**, zero loss and all samples on assigned workers. At least99.82% resolve to the measured native kernel or vector/scalar field inverse, with≤0.103% unknown. MMAP/FORK metadata verifies executable/PID/TID ownership and CPU assignments. Exact owned raw paths are chowned after recordings with hashes unchanged, before once-only decoding. Flat samples are not exclusive instruction cost, spill latency, memory-level attribution or removable proof time. Inadequate recordings would be retained rather than extended.\n\n|Profile|Samples|Unknown share|Worker share|Native kernel/inverse share|\n|---|---:|---:|---:|---:|\n",review["total_profile_samples"]));
    for (name, p) in a["profiles"].as_object().unwrap() {
        report.push_str(&format!(
            "|{name}|{}|{:.5}%|{:.5}%|{:.5}%|\n",
            p["samples"],
            p["unknown_share"].as_f64().unwrap() * 100.,
            p["worker_share"].as_f64().unwrap() * 100.,
            p["target_share"].as_f64().unwrap() * 100.
        ));
    }
    report.push_str("\n## Resources\n\nAll scratch capacities match across arms and before/after timing. G2 per-worker capacities are16000 bytes at200 points and28160 at352, plus1280 bytes for201/353 tails. Candidate maximum observed process RSS across this small caller matrix is18.75 MiB, including setup and threads. It is not full-proof peak RAM or minimum RAM. CPU-time/wall ratios and timed minor faults remain diagnostics.\n\n|Context|OFF/ON process peak RSS MiB|OFF/ON median busy CPUs|OFF/ON minor faults per wave|\n|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let rss = &row["max_observed_process_RSS_MiB"];
        let b = &row["median_busy_CPUs"];
        let f = &row["median_minor_faults_per_wave"];
        report.push_str(&format!(
            "|{}|{} / {}|{:.4} / {:.4}|{:.9} / {:.9}|\n",
            row["context"].as_str().unwrap(),
            rss["OFF"],
            rss["ON"],
            b["OFF"].as_f64().unwrap(),
            b["ON"].as_f64().unwrap(),
            f["OFF"].as_f64().unwrap(),
            f["ON"].as_f64().unwrap()
        ));
    }
    report.push_str("\n## Accepted proof baseline, unchanged\n\nInherited local32-worker full-proof timings and supplied M5 references follow. No new proof/SDK/AWS/Aeglos comparison is added. SDK6d83/source176 and ledger30846 remain current. Inherited132.75 MiB RSS is neither a new peak nor minimum RAM.\n\n|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU10× limit ms|\n|---|---:|---:|---:|---:|\n");
    for (name, local, cpu, metal, limit) in [
        ("1×2", 7.928660, 37., 24., 3.7),
        ("2×2", 11.151932, 59., 30., 5.9),
        ("4×2", 16.086386, 109., 48., 10.9),
        ("8×2", 24.179453, 189., 74., 18.9),
        ("12×2", 33.884455, 284., 101., 28.4),
        ("16×2", 40.077381, 335., 124., 33.5),
        ("24×2", 65.850777, 528., 176., 52.8),
        ("merge24", 65.060937, 507., 175., 50.7),
    ] {
        report.push_str(&format!("|{name}|{local:.6}|{cpu}|{metal}|{limit}|\n"));
    }
    report.push_str("\nAccepted merge54 remains161.883899 ms without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet (0/8). This stage adds no portable/device, huge-circuit, forced-pressure, swap, minimum-RAM or concurrent-proof qualification.\n\n## Artifacts and continuation\n\nThe [decision](artifacts/decision.md) closes this implementation. The [next step](artifacts/next.md) calls for whole-path dependency/resource accounting before another candidate, using saved evidence without an unchanged retry.\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [symbols](artifacts/profile-symbols.json), [host](artifacts/host-public.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json) and [checksums](SHA256SUMS) preserve all arms and failures. Full recordings/decoded streams remain local under `/home/ubuntu/src/zopro-perf-work/g2-validated-capture176/`. The selected UTF-8 archive excludes executable/library/key/witness/proof blobs; host-public omits the unrelated process list while the original remains locally pinned.\n");
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
        art.join("native-qualification.json"),
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
        members.insert(f.to_owned(), member(&root.join(f)));
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
        assert_eq!(text.len(), entry["bytes"].as_u64().unwrap() as usize);
        assert_eq!(digest(text), entry["sha256"]);
    }
    save(
        art.join("publication.json"),
        &json!({"status":"pass","commands":review["commands_checked"],"callers":review["callers_checked"],"timing_checks":a["timing_checks"],"failed_timing_checks":nf,"failure_classes":failclasses,"profiles":18,"profile_samples":review["total_profile_samples"],"UTF8_archive_members":members.len(),"archive_roundtrip_verified":true,"archive_sha256":hash(archive),"raw_recordings_local":raw.len(),"decoded_stdout_streams_local":streams.len(),"gate_passed":a["gate_passed"],"adopted":false,"new_proofs":0,"ledger":30846,"publisher_binary_sha256":hash(root.join("publish"))}),
    );
    let mut sums = String::new();
    let files = walk(Path::new(D));
    for p in &files {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/{}UTF8members/18raw+36decoded local indexes/archive roundtrip PASS; gate remains failed, no adoption.",files.len()+1,members.len());
}
