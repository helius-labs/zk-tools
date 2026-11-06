use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-callers176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-codec176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-root-callers176";
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
    assert_eq!(review["failed_timing_checks"].as_array().unwrap().len(), 24);
    assert_eq!(
        filehash(&root.join("analysis.json")),
        review["analysis_sha256"]
    );
    assert_eq!(load(root.join("runs/result.json"))["status"], "complete");
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Lazy inverse-root codec: complete-caller comparison\n\n**The candidate did not pass its performance gate and is not adopted.** This follows the [native codec qualification](../2026-11-06-inverse-root-codec176/README.md), which passed correctness and removed a redundant vector canonical comparison. The complete path did not establish gain across both curves and controls. No full-proof speedup is claimed.\n\n");
    report.push_str(&fs::read_to_string(root.join("decision.md")).unwrap());
    report.push_str("\n\n## Measurement boundary and frozen protocol\n\nG1/G2 ×200/201/352/353 points ×1/16/32 workers gives24 contexts, including full batches and scalar tails. Frozen is accepted source176 with the audited matching caller adapter, not the accepted proof SDK. Fresh OFF/ON share one qualified changed field library and matching cores; ON alone selects the lazy root product. Existing native tests, builds and codegen were inherited, not repeated. Binaries are immutable and pinned. No architecture table, heap buffer, key/plan copy or production pool is added.\n\nEach worker fixture uses public generator points with seed19+worker; these are not arbitrary254-bit curve-coordinate fixtures. Independent generator output checks, hashes and capacities are checked after the clock and across every arm. Native field qualification separately covered full-width/lazy field inputs against Arkworks. Three128-round priming passes and reset precede timing. The benchmark Rayon pool pins CPU2 solo, physical cores0–15 for16 workers and CPUs0–31 for32, with512KiB stacks. Fixtures are dispatched through par_iter_mut inside pool.install; a fixture is not permanently bound to a particular TID. The clock includes install/scheduling. All timed calls allocate zero times.\n\nThe Rust/xtask protocol was frozen before any clock observation. All1279 commands succeeded once:72 preflights and1checker,1080 latency calls,18 grouped counter calls,18 fixed-period profiles and90 offline steps. All1188 callers and78 timing checks were reconstructed independently. No pilot, reroll, extra block, favorable subset or profile extension was used. No other owned compute/build/package/hash workload overlapped capture.\n\nFifteen paired blocks rotate/reverse contexts and arms. Latency rounds are32768 at200/201 and16384 at352/353 per worker. ns/wave is elapsed time divided by rounds, not aggregate calls or proof latency. Intervals use sorted zero-based ranks3/7/11 of15; their coverage is marginal, not a simultaneous family-wise guarantee. Calibration requires every OFF/frozen interval inside[0.97,1.03]. Every ON/OFF and ON/frozen interval must have upper≤1.01. Separate G1/G2 equal-log32-worker aggregates across all four sizes, and an equal-log aggregate of all24 contexts, require upper<1 and≥11wins against both controls. No minimum positive-gain floor is imposed. All checks and estimates are retained.\n\n|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF [interval], wins|ON/frozen [interval], wins|OFF/frozen [interval], wins|\n|---|---:|---:|---:|---|---|---|\n");
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
    report.push_str("\n|Aggregate|Paired ON/control ratio [interval], wins|\n|---|---|\n");
    for (name, v) in a["aggregates"].as_object().unwrap() {
        report.push_str(&format!("|{name}|{}|\n", iv(v)));
    }
    report.push_str("\nAll failed checks (5calibration,13nonregression,6gain):\n\n");
    for s in a["failed_checks"].as_array().unwrap() {
        report.push_str(&format!("- `{}`\n", s.as_str().unwrap()));
    }
    report.push_str("\n## Resources\n\nCapacities are actual retained caller vectors per worker, identical in all arms and after every timed call. RSS is the maximum observed process peak across15 latency blocks, including setup and threads. It is not a full-proof peak or minimum RAM. CPU/fault observations cover the timed region; busy CPUs are median process CPU-time/wall-time ratios. No page-fault bottleneck is established.\n\n|Context|OFF/ON capacity B per worker|OFF/ON peak RSS MiB|OFF/ON median busy CPUs|OFF/ON minor faults per wave|\n|---|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let rss = &row["max_observed_process_RSS_MiB"];
        let busy = &row["median_busy_CPUs"];
        let faults = &row["median_minor_faults_per_wave"];
        report.push_str(&format!(
            "|{}|{} / {}|{} / {}|{:.4} / {:.4}|{:.9} / {:.9}|\n",
            row["context"].as_str().unwrap(),
            row["scratch_capacity_bytes_first_worker"]["OFF"],
            row["scratch_capacity_bytes_first_worker"]["ON"],
            rss["OFF"],
            rss["ON"],
            busy["OFF"].as_f64().unwrap(),
            busy["ON"].as_f64().unwrap(),
            faults["OFF"].as_f64().unwrap(),
            faults["ON"].as_f64().unwrap()
        ));
    }
    report.push_str("\n## CPU counters and profiles\n\nAll latency finished before diagnostics. Fixed panels: for each curve200w1,200w32,352w32 and all three arms. Solo profiles run2097152rounds;32-worker profiles65536; counters use one quarter of profile rounds. Duration was fixed from prior controls before observing the candidate. FIFO control brackets the measured region. Each of four grouped user events requires≥90% coverage. Profiles use period16000000cycles:u, at least1000samples,≤5%unknown, zero loss and≥95%worker samples. Executable MMAP/FORK metadata checks PID/TID ownership and each assigned worker CPU. All18 panels and18 profiles qualify;119253samples, all on declared workers.\n\nComplete-caller instruction savings against freshOFF are only0.0066–0.0271%, despite24fewer static inverse rows. Counters do not show a consistent cycle gain. Cache-miss ratios fluctuate and are not a causal cache-level attribution. Flat inverse sample shares below are not a removable-wall-time bound.\n\n|Context|ON/OFF cycles|ON/OFF instructions|ON/OFF cache misses|ON/OFF branch misses|\n|---|---:|---:|---:|---:|\n");
    for (name, events) in review["PMU_ON_OFF_diagnostics"].as_object().unwrap() {
        report.push_str(&format!(
            "|{name}|{:.9}|{:.9}|{:.9}|{:.9}|\n",
            events["cycles:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            events["instructions:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            events["cache-misses:u"]["ON_OFF_ratio"].as_f64().unwrap(),
            events["branch-misses:u"]["ON_OFF_ratio"].as_f64().unwrap()
        ));
    }
    report.push_str("\n|Profile|Samples|Unknown share|Worker share|Flat inverse share|Qualified|\n|---|---:|---:|---:|---:|:---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        let rp = &review["profiles"][name];
        report.push_str(&format!(
            "|{name}|{}|{:.5}%|{:.5}%|{:.5}%|{}|\n",
            p["samples"],
            p["unknown_share"].as_f64().unwrap() * 100.,
            p["worker_share"].as_f64().unwrap() * 100.,
            rp["inverse_nonzero_flat_share"].as_f64().unwrap() * 100.,
            p["qualified"]
        ));
    }
    report.push_str("\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let prior=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-root-codec176/README.md").unwrap();
    let section = &prior[prior
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&section[..section.find("\n## Continuation and artifacts").unwrap()]);
    report.push_str("\n\n## Artifacts and continuation\n\nFull raw profiles and all original workflow files remain under `/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/perf/` and `runs/`. Raw recordings and36 normal/metadata decode stdout streams are indexed by bytes andSHA256. Other workflow files, including every latency output, command, exit and counter CSV, are in the checked UTF8 archive. No capture, decode, native suite or qualified build was repeated for packaging. Host-public omits the unrelated process listing; the immutable original host file remains local and pinned. No private key/image/witness/proof bytes or binary/library blobs are embedded.\n\nThe [next stage](artifacts/next.md) is a work-count/algebra hypothesis on the dominant production streaming updates, using existing production evidence. This failed candidate does not select a full-SDK trial. There is no new AWS/Aeglos/Gnark/proof/huge/phone/pressure/swap/minimum-RAM result. All eight CPU10× requirements remain active and unmet.\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [caller identities](artifacts/identities.json), [source qualification](artifacts/qualification-source-audit.json), [symbols](artifacts/profile-symbols.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS).\n");
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
    fs::copy(
        Path::new(Q).join("identities.json"),
        art.join("identities.json"),
    )
    .unwrap();
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
        &json!({"published":true,"new_capture":false,"commands":1279,"callers":1188,"timing_checks":78,"failed_timing_checks":24,"profiles":18,"profile_samples":119253,"native_qualification_inherited_not_rerun":true,"UTF8_archive_members":members.len(),"archive_roundtrip_verified":true,"archive_sha256":filehash(&archive),"raw_recordings_local":raw.len(),"decoded_stdout_streams_local":streams.len(),"gate_passed":false,"adopted":false,"new_proofs":0,"ledger":30846,"publisher_binary_sha256":filehash(&root.join("publish"))}),
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
