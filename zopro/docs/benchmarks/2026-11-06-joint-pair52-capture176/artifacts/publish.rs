use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/joint-pair52-capture176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/joint-pair52-callers176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-joint-pair52-capture176";
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn filehash(p: &Path) -> String {
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
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p));
        } else if p.is_file() {
            v.push(p);
        }
    }
    v.sort();
    v
}
fn index(p: &Path) -> Value {
    json!({"local_path":p,"bytes":fs::metadata(p).unwrap().len(),"sha256":filehash(p)})
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = std::str::from_utf8(&b).expect("explicit UTF8 helpers/receipts only");
    json!({"bytes":b.len(),"sha256":digest(&b),"utf8":text})
}
fn save(p: &Path, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
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
    assert_eq!(review["failed_timing_checks"].as_array().unwrap().len(), 39);
    assert_eq!(
        filehash(&root.join("analysis.json")),
        review["analysis_sha256"]
    );
    assert_eq!(load(root.join("runs/result.json"))["status"], "complete");
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Joint complex arithmetic: complete-caller comparison\n\n**Rejected: the G2 caller is slower across all measured contexts.** The four-size 32-worker G2 aggregate takes **62.79% longer than fresh OFF** and63.37% longer than the frozen accepted-source caller. Both aggregates have0/15 winning blocks. This follows the [correctness/codegen qualification](../2026-11-06-joint-pair52-callers176/README.md); fewer static IFMAs did not yield faster execution. No proof speedup or production change is claimed.\n\n");
    report.push_str("## Results and limitations\n\nThe preregistered gate fails **39/76 checks**:4 calibration,31 nonregression and4 gain checks. All12 G2 OFF/frozen calibration intervals pass, and every G2 ON/OFF and ON/frozen context regresses. Four unchanged G1 32-worker calibration intervals fail; those failures remain in the result, so the campaign is not globally calibrated. These G1 failures do not explain the much larger G2 losses also observed against fresh OFF and in single-worker runs. No favorable subset, reroll or threshold change replaces the result.\n\nEach row reports median elapsed ns per complete batch wave, with all workers active. It is neither aggregate throughput nor full-proof latency. Paired ratios need not equal ratios of the displayed medians.\n\n|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF ratio [interval], wins|ON/frozen ratio [interval], wins|OFF/frozen ratio [interval], wins|\n|---|---:|---:|---:|---|---|---|\n");
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
    report.push_str("\n## Measurement boundary\n\nThe [protocol](artifacts/protocol.md), Rust campaign and independent analyzer were frozen before the first observation. Twenty-four contexts cover G1/G2,200/201/352/353 points and1/16/32 workers, with frozen/fresh OFF/ON executables. The frozen control is an accepted-source diagnostic caller, distinct from the production SDK. Fresh executables explicitly link their qualified coherent field/core libraries; qualification tests and builds were inherited without repetition.\n\nEvery worker uses a distinct public-generator fixture, seed19+worker. These are not arbitrary full-width coordinate fixtures; native field qualification separately covers full-width/lazy values. Three128-round priming/check passes and reset precede the clock. All1188 callers pass independent generator oracles, cross-arm output hashes, equal scratch capacities and zero timed allocations. The diagnostic Rayon pool uses512 KiB stacks, CPU2 solo, physical cores0–15 for16 workers and CPUs0–31 for32. Timing includes pool dispatch; final checks, digest and JSON are outside the clock. No production pool, lookup table, key copy or retained plan is added.\n\nFifteen fixed paired blocks rotate/reverse all contexts and arms; rounds per worker are32768 at200/201 and16384 at352/353. All1279 commands succeed first attempt:72 preflights,1 checker,1080 latency,18 PMU,18 cycles profiles and90 offline commands. No competing owned compute overlaps capture. Frequency is not locked.\n\nIntervals use sorted zero-based ranks3/7/11 of15 paired ratios and are marginal, not simultaneous family-wise guarantees. All24 calibration intervals must lie within[0.97,1.03]; all48 ON/control nonregression intervals require upper≤1.01. G2 logical32 across all four sizes and the all24-context aggregate each require upper<1 and≥11/15 wins against both controls, without a positive-gain floor. Unchanged G1 requires nonregression. All76 checks and their raw ratios are retained and independently reconstructed.\n\n## CPU evidence\n\nComplete G2 static IFMA sites fall15.17%, but instruction rows rise8.41% and vector stack-addressed rows rise23.22%. Grouped counters show dynamic instructions rising18.37–19.07% and cycles rising80.12–129.25% in the G2 panels. Instructions per cycle therefore decline substantially. Larger live state, memory operations and serial carry/reduction dependencies are plausible contributors; flat profiles and these counters do not isolate their causal costs. Solo cache misses are very low in both builds, so a large cache-miss ratio alone would misdescribe this loss. G1 counter instructions are essentially unchanged.\n\n|Context|ON/OFF cycles|ON/OFF instructions|ON/OFF cache misses|ON/OFF branch misses|\n|---|---:|---:|---:|---:|\n");
    for (name, e) in review["PMU_ON_OFF_diagnostics"].as_object().unwrap() {
        report.push_str(&format!(
            "|{name}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
            e["cycles:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            e["instructions:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            e["cache-misses:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            e["branch-misses:u"]["ON_OFF_ratio"].as_f64().unwrap()
        ));
    }
    report.push_str("\nDiagnostics follow all latency runs. FIFO control brackets counters and cycles:u profiles at fixed period16000000,512 mmap pages; solo profiles use2097152 rounds and32-worker profiles65536, with counters at one quarter. All grouped counter events have≥90% coverage. All18 profiles qualify with **143325 samples**, zero loss,100% on assigned workers,≥99.81% in native kernel/inverse functions and≤0.104% unknown. MMAP/FORK metadata validates executable, PID/TID and CPU ownership. Flat sampled periods are not exclusive cycles or removable proof time. Recordings are not extended. Exact owned raw paths are chowned before decoding with hashes unchanged; each sample/metadata/attribute/loss decode runs once.\n\n|Profile|Samples|Unknown share|Worker share|Target kernel/inverse share|\n|---|---:|---:|---:|---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        report.push_str(&format!(
            "|{name}|{}|{:.5}%|{:.5}%|{:.5}%|\n",
            p["samples"],
            p["unknown_share"].as_f64().unwrap() * 100.,
            p["worker_share"].as_f64().unwrap() * 100.,
            p["target_share"].as_f64().unwrap() * 100.
        ));
    }
    report.push_str("\n## Resources\n\nScratch capacities are identical across arms after every timed call. G2 capacity per worker is16000 bytes at200 points and28160 at352, plus1280 for the201/353 scalar tails. Maximum observed candidate process RSS is18.5 MiB across this small caller matrix, including setup and threads. It is not a proof memory peak or minimum RAM. G2 busy CPU counts are near1 solo and31 at32 workers. Timed minor faults remain very small; this experiment does not establish a RAM-capacity or page-fault bottleneck.\n\n|Context|OFF/ON process peak RSS MiB|OFF/ON median busy CPUs|OFF/ON minor faults per wave|\n|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let rss = &row["max_observed_process_RSS_MiB"];
        let busy = &row["median_busy_CPUs"];
        let f = &row["median_minor_faults_per_wave"];
        report.push_str(&format!(
            "|{}|{} / {}|{:.4} / {:.4}|{:.9} / {:.9}|\n",
            row["context"].as_str().unwrap(),
            rss["OFF"],
            rss["ON"],
            busy["OFF"].as_f64().unwrap(),
            busy["ON"].as_f64().unwrap(),
            f["OFF"].as_f64().unwrap(),
            f["ON"].as_f64().unwrap()
        ));
    }
    report.push_str("\n## Accepted proof baseline, unchanged\n\nThese are inherited local32-worker full-proof timings, not new observations. SDK6d83/source176 and ledger30846 remain unchanged. Inherited132.75 MiB RSS is neither a new peak nor minimum RAM. No new SDK, proof, Gnark, AWS, Aeglos, large-circuit, phone, pressure or swap run is selected by this rejected candidate.\n\n|Shape|M5 CPU ms|M5 Metal ms|Accepted local CPU ms|CPU10× limit ms|\n|---|---:|---:|---:|---:|\n");
    for (name, cpu, metal, local, limit) in [
        ("1×2", 37., 24., 7.928660, 3.7),
        ("2×2", 59., 30., 11.151932, 5.9),
        ("4×2", 109., 48., 16.086386, 10.9),
        ("8×2", 189., 74., 24.179453, 18.9),
        ("12×2", 284., 101., 33.884455, 28.4),
        ("16×2", 335., 124., 40.077381, 33.5),
        ("24×2", 528., 176., 65.850777, 52.8),
        ("merge24", 507., 175., 65.060937, 50.7),
    ] {
        report.push_str(&format!("|{name}|{cpu}|{metal}|{local:.6}|{limit}|\n"));
    }
    report.push_str("\nAccepted merge54 remains161.883899 ms, without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet0/8.\n\n## Decision and artifacts\n\nThe [decision](artifacts/decision.md) closes this implementation. The [next step](artifacts/next.md) uses saved instruction-address samples and decoded consumers to study the IPC regression before selecting a genuinely different whole-path hypothesis. No successful capture, decode, native suite, build or model is repeated for analysis or publication.\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [source audit](artifacts/qualification-source-audit.json), [symbols](artifacts/profile-symbols.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json) and [publication](artifacts/publication.json) preserve all arms and failures. Full raw/decoded streams remain local under `/home/ubuntu/src/zopro-perf-work/joint-pair52-capture176/`. The archive contains selected UTF-8 sources and receipts, excluding executable/library/key/witness/proof blobs. Host-public omits the unrelated process list; original host data remains pinned locally.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "campaign.rs",
        "plan.json",
        "pins.json",
        "protocol.md",
        "decision.md",
        "next.md",
        "profile-symbols.json",
        "publish.rs",
    ] {
        fs::copy(root.join(f), art.join(f)).unwrap();
    }
    fs::copy(root.join("identities.json"), art.join("identities.json")).unwrap();
    fs::copy(
        Path::new(Q).join("source-audit.json"),
        art.join("qualification-source-audit.json"),
    )
    .unwrap();
    let mut host = load(root.join("host.json"));
    host.as_object_mut().unwrap().remove("processes");
    host["original_local_identity"] = index(&root.join("host.json"));
    save(&art.join("host-public.json"), &host);
    let mut raw = BTreeMap::new();
    for p in walk(&root.join("perf")) {
        raw.insert(
            p.strip_prefix(root).unwrap().display().to_string(),
            index(&p),
        );
    }
    assert_eq!(raw.len(), 18);
    save(&art.join("raw-profile-index.json"), &json!(raw));
    let mut members = BTreeMap::new();
    for f in [
        "campaign.rs",
        "review.rs",
        "publish.rs",
        "protocol.md",
        "decision.md",
        "next.md",
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
    save(&art.join("profile-stream-index.json"), &json!(streams));
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
        &art.join("publication.json"),
        &json!({"published":true,"new_capture":false,"commands":1279,"callers":1188,"timing_checks":76,"failed_timing_checks":39,"profiles":18,"profile_samples":143325,"native_qualification_inherited_not_rerun":true,"UTF8_archive_members":members.len(),"archive_roundtrip_verified":true,"archive_sha256":filehash(&archive),"raw_recordings_local":raw.len(),"decoded_stdout_streams_local":streams.len(),"gate_passed":false,"adopted":false,"new_proofs":0,"ledger":30846,"publisher_binary_sha256":filehash(&root.join("publish"))}),
    );
    let mut sums = String::new();
    let files = walk(Path::new(D));
    for p in &files {
        sums.push_str(&format!(
            "{}  {}\n",
            filehash(p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/{}UTF8members/18raw+36decoded local indexes/archive roundtrip PASS; no adoption",files.len()+1,members.len());
}
