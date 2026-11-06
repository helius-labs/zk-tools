use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-callers176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-triple-h-callers176";
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
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let gate = a["gate_passed"].as_bool().unwrap();
    let mut report=format!("# H-only complete unweighted G1 caller comparison\n\nThe cancelled-slope native G1 reducer was measured against both frozen and fresh binary controls in all seven original caller cases with 1, 16 and 32 workers. **The predeclared component gate {}.** This is a complete-reduction experiment, not a proof measurement or production adoption. Accepted local 1×2 remains **7.928660 ms**; the CPU 10× target remains unmet for all eight shapes.\n\n",if gate{"passed"}else{"did not pass"});
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Implementation and controls\n\nThe [native qualification](../2026-11-06-triple-h-native176/README.md) passed five focused native tests and independent Arkworks checks, including exceptional fallback, partial lanes, scratch reuse and 512 KiB stacks on 1/4/16/32 workers. Those successful tests were inherited here, not rerun. The accepted field and qualified native core were reused unchanged. This formula uses H rather than dH, retains two SIMD vectors per record, shares the inverse with pair remainders, rereads coordinates and defers compaction until reverse stores are complete. Weighted trees remain unchanged.\n\nThe frozen binary route and new route use the same threaded executable linked to that unchanged native core, with different CLI selections. This frozen control contains the byte-exact accepted binary reducer; it is not the accepted proof SDK. Fresh OFF is a separately built core differing only by replacing the Work loop's SELECTED condition with false. All other 177 snapshot files, struct layout, fixtures and binary arithmetic are exact. A new threaded driver was needed for the declared contention contexts. The original qualified solo caller was retained and used in 14 diagnostic preflights.\n\nAll new core/final-consumer commands explicitly use O3/native/oneCGU; final consumers use fatLTO. The three missing builds completed once. There was no field rebuild, lookup table, key/image copy, additional production pool, C/ASM change or layout/attribute/threshold sweep. All 176 production source files remain exact.\n\n## Protocol and complete results\n\nThe full 1,107-command Rust/xtask protocol was frozen before capture: 63 threaded preflights, 14 original solo preflights and one check; 945 latency processes across 21 contexts, three arms and 15 paired blocks; 12 PMU panels, 12 profiles and 60 offline steps. Every output passed the complete projective fixture oracle. Preflights also matched the original caller's coordinate hashes, capacities and allocation counts. The threaded driver primes 24 complete reductions per worker, then measures only reductions between barriers; output checks/hashing occur after release.\n\nTile257/384/511/2048 use fresh buffers; group35973/groups31x256/tails reuse buffers. Workers duplicate complete public fixtures with private scratch. Solo is pinned to CPU2, 16 workers to the 16 physical cores and 32 workers to all SMT threads. These are contention measurements, not proof-level parallel scaling or concurrent-proof throughput.\n\nRatios below are paired new/control medians with conservative median intervals (sorted ranks 4–12 of 15) and wins. Below one is faster. Fresh OFF/frozen calibration requires the entire interval in [0.97,1.03] for every context. New/control nonregression requires upper ≤1.01 for every context; both-control gain requires upper <1 and at least 11 wins for every main context, plus the four-main 32-worker geometric aggregate. No minimum gain, extra blocks, favorable subset or extension was used. All 89 timing checks and profile/counter eligibility are preserved.\n\n| Case | Workers | Main | Frozen ns/wave | OFF ns/wave | New ns/wave | New/OFF [interval], wins | New/frozen [interval], wins | OFF/frozen calibration |\n|---|---:|:---:|---:|---:|---:|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let p = &row["pairs"];
        report.push_str(&format!(
            "|{}|{}|{}|{:.2}|{:.2}|{:.2}|{}|{}|{}|\n",
            row["case"].as_str().unwrap(),
            row["workers"],
            if row["main"] == true { "yes" } else { "no" },
            row["median_ns_per_wave"]["frozen"].as_f64().unwrap(),
            row["median_ns_per_wave"]["OFF"].as_f64().unwrap(),
            row["median_ns_per_wave"]["ON"].as_f64().unwrap(),
            interval(&p["ON/OFF"]),
            interval(&p["ON/frozen"]),
            interval(&p["OFF/frozen"])
        ));
    }
    report.push_str("\nFour-main geometric aggregate at 32 workers:\n\n");
    for den in ["OFF", "frozen"] {
        report.push_str(&format!(
            "- New/{den}: {}.\n",
            interval(&a["main32_aggregates"][den])
        ));
    }
    report.push_str("\nFailed checks (all retained):\n\n");
    if a["failed_checks"].as_array().unwrap().is_empty() {
        report.push_str("None.\n");
    } else {
        for f in a["failed_checks"].as_array().unwrap() {
            report.push_str(&format!("- `{}`\n", f.as_str().unwrap()));
        }
    }
    report.push_str("\n## Memory, CPU use and faults\n\nCapacity totals below include the common points/counts/output, retained binary fallback scratch, ternary indices and both prefix/denominator buffers, plus fixed structs. Each worker reported stable capacities across blocks; reused cases had zero measured allocation calls. Values are for one complete reduction worker. The RSS columns are the maximum observed whole-process counter across the latency blocks and include setup/fixtures/workers; they are not minimum RAM.\n\n| Case | Workers | OFF capacity B/worker | New capacity B/worker | OFF/new allocations per worker-round | OFF/new RSS MiB | OFF/new busy CPUs | OFF/new minor faults/wave |\n|---|---:|---:|---:|---|---|---|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let cap = |arm: &str| {
            row["retained_capacity_bytes_first_worker"][arm]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap())
                .sum::<u64>()
        };
        let pair = |key: &str| {
            format!(
                "{:.3} / {:.3}",
                row[key]["OFF"].as_f64().unwrap(),
                row[key]["ON"].as_f64().unwrap()
            )
        };
        report.push_str(&format!(
            "|{}|{}|{}|{}|{}|{}|{}|{}|\n",
            row["case"].as_str().unwrap(),
            row["workers"],
            cap("OFF"),
            cap("ON"),
            pair("median_alloc_calls_per_worker_round"),
            pair("max_observed_process_RSS_MiB"),
            pair("mean_busy_CPUs_from_medians"),
            pair("median_minor_faults_per_wave")
        ));
    }
    report.push_str("\nBusy CPUs are process CPU time divided by elapsed wall time, using the reported medians. Minor faults count page mappings that did not require disk reads; this experiment does not establish swap behavior, device limits or a memory-pressure cause. Optional ternary and binary fallback capacities may coexist. Future runtime routing needs existing resource admission; no universal RAM cap is inferred.\n\n## Counters and profiles\n\nAll latency finished before PMU/profiling. Counter panels cover cycles, instructions, cache misses and branch misses with declared ≥90% coverage. Fixed-period user-cycle profiles use 16,000,000 cycles per sample, measured-region FIFO control, raw ownership/hash checks, saved metadata/normal decodes, event attributes, worker TID→CPU attribution and lost-event checks. A profile qualifies only with at least 1,000 samples, unknown ≤5% and no lost events. Profiles explain results and cannot rescue a failed latency gate.\n\n| Profile | Samples | Unknown | Qualified |\n|---|---:|---:|:---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        report.push_str(&format!(
            "|{}|{}|{:.4}%|{}|\n",
            name,
            p["samples"],
            p["unknown_share"].as_f64().unwrap() * 100.,
            p["qualified"]
        ));
    }
    report.push_str("\n## Scope and continuation\n\nOnly a fully eligible complete-caller result selects a separate verified SDK trial covering all eight shapes, merge54, physical16, portable execution, concurrency, resources and Gnark verification. Component gains do not establish proof-level gains. Previous [old dH caller](../2026-10-28-unweighted-triple-caller174/README.md) and [mixed SDK](../2026-10-28-mixed-sdk/README.md) negative results remain unchanged. No new SDK/Gnark proof, current AWS/Aeglos comparison, phone/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-proof throughput result is added.\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("## Accepted CPU and Metal comparisons, unchanged")
        .unwrap()..];
    report.push_str(&section[..section.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Artifact storage\n\nThe 552 MiB of raw recordings and several GiB of decoded streams remain intact at `/home/ubuntu/src/zopro-perf-work/triple-h-callers176/perf/` and `runs/`. They are indexed by exact byte size and SHA256, and were independently reviewed before publication. These large files are not embedded in Git. The receipt archive retains every command/exit and all timing/preflight/counter output; its only omitted workflow files are the 24 large normal/metadata decode stdout streams, all listed in [the decoded-stream index](artifacts/profile-stream-index.json). No capture or decode was repeated for packaging.\n");
    report.push_str("\n\n[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [decision](artifacts/decision.md), [design](artifacts/design.md), [declaration](artifacts/declaration.json), [identities](artifacts/identities.json), [driver](artifacts/driver.rs), [perf control](artifacts/perf_support.rs), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [profile symbols](artifacts/profile-symbols.json), [next stage](artifacts/next.md), [one OFF source delta](artifacts/OFF-source-delta.json.gz), [exact helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw profile index](artifacts/raw-profile-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). ON source is bound by the [prior native source manifest/deltas](../2026-11-06-triple-h-native176/README.md); full snapshots/binaries stay local. No private input, key/image/witness/proof bytes, external paper text or executable/library blobs are included.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "review.json",
        "review.rs",
        "decision.md",
        "next.md",
        "design.md",
        "declaration.json",
        "identities.json",
        "host-before.json",
        "driver.rs",
        "perf_support.rs",
        "build.rs",
        "build-plan.json",
        "build-pins.json",
        "OFF-source-manifest.json",
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
    let mut delta = BTreeMap::new();
    let rel = "crates/zopro-core/src/msm/h_only_probe.rs";
    delta.insert(
        rel.to_owned(),
        member(&Path::new(R).join("OFF-source").join(rel)),
    );
    archive(&art.join("OFF-source-delta.json.gz"), &delta);
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
    for dir in ["build-runs", "runs", "stat"] {
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
    assert_eq!(streams.len(), 24);
    fs::write(
        art.join("profile-stream-index.json"),
        serde_json::to_vec_pretty(&streams).unwrap(),
    )
    .unwrap();
    archive(&art.join("helpers-and-receipts.json.gz"), &helpers);
    let mut raw = BTreeMap::new();
    for (name, profile) in a["profiles"].as_object().unwrap() {
        let p = Path::new(R).join("perf").join(format!("{name}.data"));
        let h = hash_file(&p);
        assert_eq!(h, profile["raw_sha256"]);
        raw.insert(name.clone(),json!({"local_path":p,"raw_bytes":fs::metadata(&p).unwrap().len(),"raw_sha256":h,"storage":"local original recording, not Git-tracked"}));
    }
    fs::write(
        art.join("raw-profile-index.json"),
        serde_json::to_vec_pretty(&raw).unwrap(),
    )
    .unwrap();
    let publication = json!({"review_passed":true,"gate_passed":gate,"source_delta_UTF8_members":delta.len(),"helper_receipt_UTF8_members":helpers.len(),"raw_profiles_indexed_locally":raw.len(),"large_decoded_streams_indexed_locally":streams.len(),"raw_recordings_and_large_streams_embedded_in_Git":false,"all_embedded_archive_hashes_and_lossless_roundtrips_verified":true,"capture_commands":1107,"threaded_caller_processes":1032,"legacy_preflights":14,"timing_checks":89,"new_proofs":0,"adopted":false,"goal_status":"active/unmet0/8"});
    fs::write(
        art.join("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
    )
    .unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(&fs::read(&p).unwrap()),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/{}exactUTF8receipts/1OFFdelta/12raw+24decodedlocalindexes; allhash+embedded-roundtripPASS; gate={gate}; noadoption",walk(Path::new(D)).len(),helpers.len());
}
