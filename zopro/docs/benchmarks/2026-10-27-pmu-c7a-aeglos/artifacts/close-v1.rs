use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-pmu-c7a-20261027";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-pmu-c7a-aeglos";
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
fn med(v: impl IntoIterator<Item = f64>) -> f64 {
    let mut v: Vec<_> = v.into_iter().collect();
    assert!(!v.is_empty());
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn n(v: &Value) -> f64 {
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
                .contains(&p.extension().and_then(|v| v.to_str()).unwrap_or(""))
            {
                continue;
            }
            let to = out.join(rel);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(&p, &to).unwrap();
            assert_eq!(hash(&p), hash(&to));
            index.insert(s, hash(to));
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(read(r.join("cleanup-complete.json"))["status"], "complete");
    assert_eq!(
        read(r.join("cleanup-complete.json"))
            ["all_live_TemporaryProverBenchmark_instances_in_region"],
        0
    );
    let m = read("/tmp/zopro-clean-fft-adoption/production-source-manifest.json");
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(f)),
            h.as_str().unwrap()
        );
    }
    for (f, h) in read(r.join("proof-inventory.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut md=format!("# Current-source C7a primary hardware counters\n\nDiagnostic complete: **{}**. This compares the exact accepted 172-file Zopro source, freshly built for Zen4, with pinned Aeglos `0290613` Sorted at 32 and 96 workers on one temporary c7a.24xlarge. No source optimization or adoption occurs. These are instrumented warm SDK observations, not a new unprofiled latency qualification.\n\nThe protocol uses three fixed rotated/reversed blocks and separate four-event branch/cache groups: 24 counter processes, followed by four separate 199 Hz user-cycle profiles. Each counter process runs 501 requests after three primes; each profile runs 251 after three. Loading and primes precede FIFO enable/ack, and disable/ack precedes proof export. Counters include userland SDK logging, arithmetic, polling and coordination; SDK rusage includes kernel time too. Different panels are different processes.\n\n## Counter observations\n\nCells are medians of three processes. IPC is the median of per-process ratios; counts are per timed request. No statistical latency guarantee or isolated cause follows from these three diagnostic blocks. Generic cache events are not DRAM bytes, bandwidth or miss latency.\n\n|Workers|Engine|Panel|Cycles/request million|Instructions/request million|IPC|CPU ms/request|Instrumented SDK ms|\n|---|---|---|---:|---:|---:|---:|---:|\n",a["complete"]);
    let rows = a["rows"].as_array().unwrap();
    for workers in [32, 96] {
        for engine in ["current", "aeglos"] {
            for panel in ["branch", "cache"] {
                let rs: Vec<_> = rows
                    .iter()
                    .filter(|v| {
                        v["job"]["phase"] == "counter"
                            && v["job"]["workers"] == workers
                            && v["job"]["engine"] == engine
                            && v["job"]["panel"] == panel
                    })
                    .collect();
                if rs.is_empty() {
                    continue;
                }
                writeln!(
                    md,
                    "|{workers}|{engine}|{panel}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|",
                    med(rs
                        .iter()
                        .map(|v| n(&v["counters"]["cycles:u"]["count"]) / 501.))
                        / 1e6,
                    med(rs
                        .iter()
                        .map(|v| n(&v["counters"]["instructions:u"]["count"]) / 501.))
                        / 1e6,
                    med(rs
                        .iter()
                        .map(|v| n(&v["counters"]["instructions:u"]["count"])
                            / n(&v["counters"]["cycles:u"]["count"]))),
                    med(rs.iter().map(|v| (n(&v["sdk"]["summary"]["user_cpu_ms"])
                        + n(&v["sdk"]["summary"]["system_cpu_ms"]))
                        / 501.)),
                    med(rs.iter().map(|v| n(&v["sdk"]["median_ms"])))
                )
                .unwrap();
            }
        }
    }
    md.push_str("\n## Separate warm profiles\n\nAll counter captures finish before these profiles. Flat category shares are not removable wall-time fractions or exclusive critical-path costs. Generic categories match Zopro symbols; Aeglos appears as other, a matcher limitation. Its raw flat symbols remain archived.\n\n|Workers|Engine|Samples|CPU ms/request|Profile SDK ms|Epoch %|Scheduler %|\n|---|---|---:|---:|---:|---:|---:|\n");
    for v in rows.iter().filter(|v| v["job"]["phase"] == "profile") {
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.3}|{:.2}|{:.2}|",
            v["job"]["workers"],
            v["job"]["engine"].as_str().unwrap(),
            v["profile"]["samples"],
            (n(&v["sdk"]["summary"]["user_cpu_ms"]) + n(&v["sdk"]["summary"]["system_cpu_ms"]))
                / 251.,
            n(&v["sdk"]["median_ms"]),
            n(&v["profile"]["category_percent"]["epoch"]),
            n(&v["profile"]["category_percent"]["scheduler"])
        )
        .unwrap();
    }
    write!(md,"\n## Identity, verification and lifecycle\n\nThe current build passes generator consistency, 33 field and 119 core tests plus two raw and six cubic prover tests. Local preflight uses the exact final SDK binaries for eight calls and two independently verified primary exports at four workers. Aeglos retains the qualified archive/driver and original source hash, including its existing shifted-base policy; no latest-release claim is made.\n\nCompleted diagnostic: {}. SDK calls: {} (completed-call lower bound {}). Distinct independent exports: {}, including {} cloud exports independently reverified after transport; repeat verification is counted once. Proof/public members: {}. The ledger advances **12,143→{}**. All retained proofs are 164 bytes, fixture-bound and fresh against prior/new identities. Actual counters, hardware, memory events and throttle observations remain in the archive.\n\nExactly one owned instance uses a 90-minute self-termination timer and encrypted delete-on-termination volume. The measurement container is network-disabled, CPU quota 96, memory 30 GiB and swap disabled. Owned instance/volume, temporary ECR tag, private transfer object/bucket and credential files are removed before closure. No production resource or global perf setting is changed. Zero remaining regional TemporaryProverBenchmark instances is confirmed.\n\nThe [accepted all-eight CPU/Metal comparison](../2026-10-27-clean-fft-adoption/README.md) remains the canonical local comparison against supplied historical M5 figures. The [older C7a comparison](../2026-10-27-current-c7a-aeglos/README.md) used previous 171-file source and remains separate. No 54 reference was supplied, and no all-nine unprofiled cloud rerun occurs here. The original all-eight CPU 10× goal remains active and unmet, 0/8. Worker count can change geometry, frequency and coordination; counter changes do not isolate a clock, RAM-capacity, fence or syscall cause.\n\n[Protocol](artifacts/design.md), [analysis and raw rows](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [post-transport verification](artifacts/post-transport-gnark.log), [source identity](artifacts/aws-accepted-manifest.json), [cleanup](artifacts/cleanup-complete.json), [artifact hashes](artifact-index.json).\n",a["complete"],a["SDK_requests"],a["completed_SDK_calls_lower_bound"],a["independent_exports"],a["post_transport_distinct_verified"],a["proof_public_members"],a["next_prior_count"]).unwrap();
    let mut raw = BTreeMap::new();
    let base = r.join("remote-results/output/measure/runs");
    if base.exists() {
        for e in fs::read_dir(base).unwrap() {
            let p = e.unwrap().path().join("perf.data");
            if p.exists() {
                raw.insert(p.display().to_string(), hash(&p));
            }
        }
    }
    save(r.join("raw-local-only-sha256.json"), &json!(raw));
    save(
        r.join("closeout.json"),
        &json!({"status":a["status"],"complete":a["complete"],"SDK_requests":a["SDK_requests"],"independent_exports":a["independent_exports"],"next_prior_count":a["next_prior_count"],"source_changed":false,"acceptance_measurement":false,"goal_achieved":false,"original8_CPU10x_rows":0,"owned_live_jobs":[],"cleanup_complete":true}),
    );
    fs::create_dir_all(d.join("artifacts")).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    println!(
        "Closed currentC7aPMU: {}artifacthashes;ledger{};source unchanged",
        index.len(),
        a["next_prior_count"]
    );
}
