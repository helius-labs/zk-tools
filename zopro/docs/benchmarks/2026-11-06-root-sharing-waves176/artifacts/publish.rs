use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-waves176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-root-sharing-waves176";
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash(p: &Path) -> String {
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
fn read(p: impl AsRef<Path>) -> Value {
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
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let s = std::str::from_utf8(&b).unwrap();
    json!({"bytes":b.len(),"sha256":digest(&b),"utf8":s})
}
fn index(p: &Path) -> Value {
    json!({"local_path":p,"bytes":fs::metadata(p).unwrap().len(),"sha256":hash(p)})
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
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let a = read(r.join("analysis.json"));
    let review = read(r.join("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(review["analysis_sha256"], hash(&r.join("analysis.json")));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    let art = d.join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut text=String::from("# Ready-root inversion sharing: latency and CPU comparison\n\n**The combined latency gate failed; no broker or production change is selected.** At 16 workers the optimistic shared bulk loses 26–27%; at 32 it wins 2.3–2.6% before request and handoff costs. Summed inversion-stage CPU time falls about 12×/31×. These are distinct results, and neither is a full-proof speedup.\n\n");
    text.push_str(&fs::read_to_string(r.join("decision.md")).unwrap());
    text.push_str("\n\n## Scope and frozen method\n\nThe [arithmetic qualification](../2026-11-06-root-sharing-qualification176/README.md) and its accepted field library are inherited without another native build, mathematical population, decoder or suite run. Driver `40efd381…` executes identical individual-inversion code for control-a and control-b in the SAME executable. They are repeated controls, not differently compiled production references or accepted proof SDK calls. Shared uses one existing benchmark worker on CPU2, with all other workers parked. Solo uses the exact individual fallback. No production worker/service is added.\n\nThe public input matrix has 256 rows of 32 canonical Fq roots (256 KiB), including deterministic full-width hashed roots and boundary rows. It is input data, not a precomputed inverse lookup. Individual workers traverse columns; the coordinator traverses rows. Different address streams, parked peers and omitted request handling make this favorable to the shared calculation; they prevent treating it as a universal inversion-latency bound. No cache-level cause is inferred from the generic counter.\n\nAll threads exist before timing, pinned CPU2 solo, physical cores0–15 at16 and CPUs0–31 at32, each with a512 KiB configured stack. Three256-wave primes precede sum reset. Main/worker start/end barriers, arithmetic, source packing/unpacking and output sums are timed. Fixture generation and individual-inverse reference/checksum validation are outside the clock. Every process requires zero timed allocation. The four wrapping-u64 sums per root are not a cryptographic proof; the wrapper's canonical full-output Arkworks qualification is independently bound.\n\nThe Rust/xtask protocol was frozen before any driver clock:9preflights and1strict checker,135latency processes (15paired blocks×3worker counts×3labels),6grouped counter processes,6cycle profiles and30offline ownership/decode/metadata/event/loss commands. All187commands and156driver outputs succeeded once. Context and label order rotates/reverses. Each latency process runs262144waves. No pilot, extra blocks, favorable topology, changed gate, batch-width/layout/attribute sweep or unchanged reroll is used. All latency completes before diagnostics, with no competing owned compute/build/hash/package workload during capture.\n\nIntervals use sorted zero-based ranks3/7/11 of15 paired ratios; coverage is marginal, not a simultaneous family-wise guarantee. All controlB/A intervals must lie inside[0.97,1.03]. Every shared/control interval has nonregression upper≤1.01. At16and32, each shared/control comparison and the equal-log16+32 aggregate require upper<1 and≥11wins. These are15timing checks; no minimum positive-gain floor. All three calibrations pass. Six latency checks fail and are retained.\n\n## Complete timing and resource results\n\nns/wave is the time to finish one root inversion per worker, not proof latency or proof throughput. CPU time sums the process's active threads; sharing deliberately parks its peers. RSS is the maximum process peak across the15blocks, including this small fixture and thread setup. It is not a full-proof peak, actual stack use or minimum RAM.\n\n|Workers|ControlA ns/wave|ControlB ns/wave|Shared ns/wave|Shared/A [interval], wins|Shared/B [interval], wins|ControlB/A [interval], wins|\n|---|---:|---:|---:|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let t = &row["median_ns_per_wave"];
        text.push_str(&format!(
            "|{}|{:.6}|{:.6}|{:.6}|{}|{}|{}|\n",
            row["workers"],
            t["control-a"].as_f64().unwrap(),
            t["control-b"].as_f64().unwrap(),
            t["shared"].as_f64().unwrap(),
            iv(&row["pairs"]["shared/control-a"]),
            iv(&row["pairs"]["shared/control-b"]),
            iv(&row["pairs"]["control-b/control-a"])
        ));
    }
    text.push_str("\n|Threaded aggregate|Shared/control [interval], wins|\n|---|---|\n");
    for (name, v) in a["threaded_aggregates"].as_object().unwrap() {
        text.push_str(&format!("|{name}|{}|\n", iv(v)));
    }
    text.push_str("\n|Workers|A / B / shared CPU µs per wave|A / B / shared median busy CPUs|A / B / shared peak RSS MiB|A / B / shared minor faults per wave|\n|---|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let values = |key: &str, factor: f64| {
            ["control-a", "control-b", "shared"]
                .map(|label| format!("{:.9}", row[key][label].as_f64().unwrap() * factor))
                .join(" / ")
        };
        text.push_str(&format!(
            "|{}|{}|{}|{}|{}|\n",
            row["workers"],
            values("CPU_ms_per_wave", 1000.),
            values("median_busy_CPUs", 1.),
            values("max_observed_process_RSS_MiB", 1.),
            values("median_minor_faults_per_wave", 1.)
        ));
    }
    text.push_str("\nAll failed checks:\n\n");
    for failure in a["failed_checks"].as_array().unwrap() {
        text.push_str(&format!("- `{}`\n", failure.as_str().unwrap()));
    }
    text.push_str("\n## Counters and automatic profiles\n\nPanels cover16/32workers and all3labels after all latency. Fixed FIFO-controlled cycles:u sampling period is16,000,000. Controls profile2,097,152waves, shared16,777,216; counts were fixed from active-core structure/prior budgets before clocks. Stat uses one quarter. Each of four grouped user events requires≥90%coverage; all six panels have100%. Per-wave normalization uses actual counts. Differing-duration shared output sums validate against individual references inside the pinned driver after the clock; equal-duration control hashes also match directly. There is no diagnostic speed or proof-throughput claim.\n\n|Panel|User cycles/wave|Instructions/wave|Cache misses/wave|Branch misses/wave|\n|---|---:|---:|---:|---:|\n");
    for (name, events) in a["counters"].as_object().unwrap() {
        text.push_str(&format!(
            "|{name}|{:.6}|{:.6}|{:.9}|{:.9}|\n",
            events["cycles:u"]["per_wave"].as_f64().unwrap(),
            events["instructions:u"]["per_wave"].as_f64().unwrap(),
            events["cache-misses:u"]["per_wave"].as_f64().unwrap(),
            events["branch-misses:u"]["per_wave"].as_f64().unwrap()
        ));
    }
    text.push_str("\nAll6profiles qualify:≥1,000samples, unknown≤5%, zeroLOST, exact period, executableMMAP/FORK/PID/TID/CPU ownership and≥95%active-worker samples. All49,708samples are on declared active workers. Controls have all workers active; shared only the coordinator. Other owned main/parked-thread samples remain explicitly counted. No recording is extended or re-decoded for packaging.\n\n|Profile|Samples|Unknown share|Active-worker share|Other owned samples|Qualified|\n|---|---:|---:|---:|---:|:---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        text.push_str(&format!(
            "|{name}|{}|{:.5}%|{:.5}%|{}|{}|\n",
            p["samples"],
            100. * p["unknown_share"].as_f64().unwrap(),
            100. * p["active_worker_share"].as_f64().unwrap(),
            p["other_owned_samples"],
            p["qualified"]
        ));
    }
    text.push_str("\n## Decision and continuation\n\nThe32-worker latency win and much lower inversion-stage CPU work are preserved. They do not establish a profitable service after publication/ownership/waiting costs, a complete-proof arrival model or the required all-context gain. Current leaf scratch accounting assumes no nested fork while a leaf waits; arbitrary job helping would need a new lifetime/admission design. No broker, new pool, wait window or32-only rescue is selected from this result. It does not prove every possible asynchronous sharing method impossible.\n\nThe [next stage](artifacts/next.md) requires a genuinely different whole-path algorithm, checked against primary sources and the existing negative history, before another native implementation. Prepared verification lines/public-input windows, fixed blinding windows and prior final-exponent studies were found already present; they are not new optimizations. No new candidate is currently selected. All eight original CPU10× targets remain active.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-root-codec176/README.md").unwrap();
    let section = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    text.push_str(&section[..section.find("\n## Continuation and artifacts").unwrap()]);
    text.push_str("\n\n## Storage and artifacts\n\nRaw profiles and every original workflow file remain at `/home/ubuntu/src/zopro-perf-work/root-sharing-waves176/perf/` and `runs/`. The six raw recordings and twelve sample/metadata decode stdout streams have local byte/SHA indexes. All other workflow files, including every latency output, command, exit and stat CSV, are in the verified UTF8 archive. Host-public excludes the unrelated process listing; its original local file stays pinned. No executable/library blobs, private key/image/witness/proof bytes, inverse-result table or third-party full text is embedded. No capture, successful suite or decode is repeated for publication.\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [driver](artifacts/driver.rs), [bulk wrapper](artifacts/bulk.rs), [native qualification](artifacts/qualification-review.json), [symbols](artifacts/profile-symbols.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decode index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). No fresh SDK/proof/Gnark/AWS/Aeglos/huge/mobile/pressure/swap/minimum-RAM/throughput result.\n");
    fs::write(d.join("README.md"), text).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "campaign.rs",
        "perf_io.rs",
        "protocol.md",
        "decision.md",
        "next.md",
        "plan.json",
        "pins.json",
        "profile-symbols.json",
        "publish.rs",
    ] {
        fs::copy(r.join(f), art.join(f)).unwrap();
    }
    for f in ["driver.rs", "bulk.rs", "fixture.rs", "perf.rs"] {
        fs::copy(Path::new(Q).join(f), art.join(f)).unwrap();
    }
    fs::copy(
        Path::new(Q).join("review.json"),
        art.join("qualification-review.json"),
    )
    .unwrap();
    let mut host = read(r.join("host.json"));
    host.as_object_mut().unwrap().remove("processes");
    host["original_local_identity"] = index(&r.join("host.json"));
    save(&art.join("host-public.json"), &host);
    let mut raw = BTreeMap::new();
    for p in walk(&r.join("perf")) {
        raw.insert(p.strip_prefix(r).unwrap().display().to_string(), index(&p));
    }
    assert_eq!(raw.len(), 6);
    save(&art.join("raw-profile-index.json"), &json!(raw));
    let mut members = BTreeMap::new();
    for f in [
        "campaign.rs",
        "perf_io.rs",
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
        members.insert(f.to_string(), member(&r.join(f)));
    }
    let mut streams = BTreeMap::new();
    for p in walk(&r.join("runs")) {
        let rel = p.strip_prefix(r).unwrap().display().to_string();
        if p.file_name().unwrap() == "stdout.log"
            && (rel.starts_with("runs/decode-") || rel.starts_with("runs/metadata-"))
        {
            streams.insert(rel, index(&p));
        } else {
            members.insert(rel, member(&p));
        }
    }
    assert_eq!(streams.len(), 12);
    save(&art.join("profile-stream-index.json"), &json!(streams));
    for p in walk(&r.join("stat")) {
        members.insert(p.strip_prefix(r).unwrap().display().to_string(), member(&p));
    }
    let bytes = serde_json::to_vec(&members).unwrap();
    let archive = art.join("helpers-and-receipts.json.gz");
    let mut gzip = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(&archive).unwrap())
        .spawn()
        .unwrap();
    gzip.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(gzip.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(&archive)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for e in decoded.as_object().unwrap().values() {
        let b = e["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), e["bytes"].as_u64().unwrap() as usize);
        assert_eq!(digest(b), e["sha256"]);
    }
    save(
        &art.join("publication.json"),
        &json!({"published":true,"commands":187,"drivers":156,"timing_checks":15,"failed_timing_checks":6,"profiles":6,"profile_samples":49708,"UTF8_archive_members":members.len(),"archive_roundtrip_checked":true,"archive_sha256":hash(&archive),"raw_recordings_local":6,"decoded_stdout_streams_local":12,"gate_passed":false,"broker_implemented":false,"adopted":false,"new_proofs":0,"proof_ledger":30846,"publisher_binary_sha256":hash(&r.join("publish"))}),
    );
    let files = walk(d);
    let sums = files
        .iter()
        .map(|p| format!("{}  {}\n", hash(p), p.strip_prefix(d).unwrap().display()))
        .collect::<String>();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/{}UTF8members/6raw+12decodedlocalindexes/hashroundtripPASS;failedgate/nobroker/adoption",files.len()+1,members.len());
}
