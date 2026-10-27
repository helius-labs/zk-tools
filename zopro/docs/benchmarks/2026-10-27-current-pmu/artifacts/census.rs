use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    io::Read,
    path::Path,
    process::Command,
};
const R: &str = "/tmp/zopro-current-pmu";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const M: &str = "/tmp/zopro-clean-fft-adoption/production-source-manifest.json";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-current-pmu";
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
fn pin(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn prepare() {
    let r = Path::new(R);
    assert!(!r.join("plan.json").exists());
    let m = read(M);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 172);
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(P).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(p, &mut pins);
    }
    let binary = m["native_binary"].as_str().unwrap();
    assert_eq!(
        hash(binary),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    pin(binary, &mut pins);
    pin(M, &mut pins);
    let receipt = read(m["native_cli_build"].as_str().unwrap());
    assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
    for (f, h) in receipt["input_pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
        pin(f, &mut pins);
    }
    pin(m["native_cli_build"].as_str().unwrap(), &mut pins);
    let old = read("/tmp/zopro-g1-raw-scratch/performance/protocol.json");
    let mut prior: BTreeSet<String> =
        serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 11669);
    let mut duplicates = 0;
    for (f, h) in read("/tmp/zopro-g1-raw-scratch/proof-inventory.json")
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            if !prior.insert(h.as_str().unwrap().into()) {
                duplicates += 1;
            }
        }
    }
    assert_eq!(duplicates, 9);
    assert_eq!(prior.len(), 12125);
    let selected = ["1x2-w16", "1x2-w32", "merge-54x1-w32"];
    let cases: Vec<Value> = selected
        .iter()
        .map(|n| {
            old["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["name"] == *n)
                .unwrap()
                .clone()
        })
        .collect();
    let mut steps = Vec::new();
    let mut jobs = Vec::new();
    fs::create_dir(r.join("runs")).unwrap();
    for block in 0..3usize {
        let mut order = vec![0, 1, 2];
        order.rotate_left(block);
        if block % 2 == 1 {
            order.reverse();
        }
        for index in order {
            let c = &cases[index];
            let panels = if block % 2 == 0 {
                ["branch", "cache"]
            } else {
                ["cache", "branch"]
            };
            for panel in panels {
                let name = format!("b{block}-{}-{panel}", c["name"].as_str().unwrap());
                let out = r.join("runs").join(&name);
                fs::create_dir(&out).unwrap();
                for fifo in ["ctl", "ack"] {
                    assert!(Command::new("mkfifo")
                        .args(["-m", "600"])
                        .arg(out.join(fifo))
                        .status()
                        .unwrap()
                        .success());
                }
                let counters = out.join("counters.txt");
                fs::write(&counters, b"").unwrap();
                let stem = Path::new(c["image"].as_str().unwrap()).file_stem().unwrap();
                let proofdir = out.join("proofs").join(stem);
                let events = if panel == "branch" {
                    "{cycles:u,instructions:u,branches:u,branch-misses:u}"
                } else {
                    "{cycles:u,instructions:u,cache-references:u,cache-misses:u}"
                };
                let rounds = if c["shape"] == "1x2" { 501 } else { 31 };
                let mut env = m["environment"].as_object().unwrap().clone();
                env.insert(
                    "RAYON_NUM_THREADS".into(),
                    json!(c["workers"].as_u64().unwrap().to_string()),
                );
                env.insert("ZOPRO_BENCH_PERF_CTL".into(), json!(out.join("ctl")));
                env.insert("ZOPRO_BENCH_PERF_ACK".into(), json!(out.join("ack")));
                let mut argv = vec![
                    json!("sudo"),
                    json!("-n"),
                    json!("perf"),
                    json!("stat"),
                    json!("--no-big-num"),
                    json!("-x"),
                    json!(";"),
                    json!("--delay=-1"),
                    json!(format!(
                        "--control=fifo:{},{}",
                        out.join("ctl").display(),
                        out.join("ack").display()
                    )),
                    json!("-e"),
                    json!(events),
                    json!("-o"),
                    json!(counters),
                    json!("--"),
                    json!("env"),
                ];
                for (k, v) in &env {
                    argv.push(json!(format!("{k}={}", v.as_str().unwrap())));
                }
                argv.extend([
                    json!("taskset"),
                    json!("-c"),
                    c["cpu_affinity"].clone(),
                    json!(binary),
                    c["image"].clone(),
                    c["request"].clone(),
                    json!(rounds.to_string()),
                    json!(proofdir),
                ]);
                assert!(argv.iter().all(|a| a.is_string()));
                steps.push(json!({"kind":"command","name":format!("measure-{name}"),"argv":argv,"timeout_seconds":600}));
                steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",out.join("proofs"),Path::new(P).join("tools/zolana/zopro-keys.lock")],"timeout_seconds":120}));
                pin(c["image"].as_str().unwrap(), &mut pins);
                pin(c["request"].as_str().unwrap(), &mut pins);
                jobs.push(json!({"name":name,"block":block,"panel":panel,"case":c,"rounds":rounds,"directory":proofdir,"counters":counters}));
            }
        }
    }
    assert_eq!(jobs.len(), 18);
    save(r.join("jobs.json"), &json!(jobs));
    save(
        r.join("inputs.json"),
        &json!({"manifest":m,"prior_proof_sha256":prior,"prior_count":12125,"planned_count":12143,"SDK_requests":6252}),
    );
    for (f, h) in old["pins"].as_object().unwrap() {
        if f.starts_with("/tmp/zopro-keys/") {
            assert_eq!(hash(f), h.as_str().unwrap());
            pin(f, &mut pins);
        }
    }
    for f in [
        r.join("design.md"),
        r.join("census.rs"),
        r.join("census"),
        r.join("jobs.json"),
        r.join("inputs.json"),
        Path::new("/usr/bin/perf").into(),
        Path::new("/tmp/zopro-gnark-verify/verify").into(),
        Path::new(P).join("tools/zolana/zopro-keys.lock"),
    ] {
        pin(f, &mut pins);
    }
    save(r.join("pins.json"), &json!(pins));
    save(
        r.join("plan.json"),
        &json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps}),
    );
    println!("Prepared18counterprocesses6252SDK18Gnark;12125->12143planned");
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    assert_eq!(v.len(), 3);
    v[1]
}
fn finish() {
    let r = Path::new(R);
    assert_eq!(read(r.join("workflow/result.json"))["status"], "complete");
    let input = read(r.join("inputs.json"));
    let mut seen: BTreeSet<String> =
        serde_json::from_value(input["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 12125);
    for (f, h) in read(r.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut inventory = BTreeMap::new();
    let mut rows = Vec::new();
    let mut calls = 0;
    for job in read(r.join("jobs.json")).as_array().unwrap() {
        let name = job["name"].as_str().unwrap();
        let path = Path::new(job["directory"].as_str().unwrap()).join("0.proof");
        assert_eq!(fs::metadata(&path).unwrap().len(), 164);
        assert!(seen.insert(hash(&path)));
        assert_eq!(
            hash(path.with_extension("public")),
            job["case"]["public_sha256"].as_str().unwrap()
        );
        for f in [path.clone(), path.with_extension("public")] {
            inventory.insert(f.display().to_string(), hash(f));
        }
        let log =
            fs::read_to_string(r.join(format!("workflow/measure-{name}/stdout.log"))).unwrap();
        let s: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(s["rounds"], job["rounds"]);
        assert_eq!(s["priming"], 3);
        assert_eq!(s["workers"], job["case"]["workers"]);
        calls += s["rounds"].as_u64().unwrap() + 3;
        assert_eq!(
            read(r.join(format!("workflow/verify-{name}/exit.json")))["success"],
            true
        );
        let mut counters = serde_json::Map::new();
        for line in fs::read_to_string(job["counters"].as_str().unwrap())
            .unwrap()
            .lines()
        {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let cols: Vec<_> = line.split(';').collect();
            assert!(cols.len() >= 5);
            let count: f64 = cols[0].trim().parse().unwrap();
            let runtime_ns: f64 = cols[3].trim().parse().unwrap();
            let pct: f64 = cols[4].trim().parse().unwrap();
            assert!(count >= 0. && runtime_ns > 0. && pct >= 99.9);
            counters.insert(
                cols[2].to_owned(),
                json!({"count":count,"runtime_ns":runtime_ns,"running_percent":pct}),
            );
        }
        assert_eq!(counters.len(), 4);
        let cycles = counters["cycles:u"]["count"].as_f64().unwrap();
        let instructions = counters["instructions:u"]["count"].as_f64().unwrap();
        assert!(cycles > 0. && instructions > 0.);
        let rounds = job["rounds"].as_f64().unwrap();
        rows.push(json!({"job":job,"summary":s,"counters":counters,"IPC":instructions/cycles,"cycles_per_request":cycles/rounds,"instructions_per_request":instructions/rounds,"proof_sha256":hash(path)}));
    }
    assert_eq!(calls, 6252);
    assert_eq!(inventory.len(), 36);
    assert_eq!(seen.len(), 12143);
    save(r.join("proof-inventory.json"), &json!(inventory));
    save(
        r.join("analysis.json"),
        &json!({"status":"complete accepted-source hardware-counter census","source_changed":false,"SDK_requests":6252,"independent_exports":18,"proof_public_members":36,"prior_count":12125,"next_prior_count":12143,"rows":rows,"acceptance_measurement":false,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    let d = Path::new(D);
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut md=String::from("# Accepted-source warm hardware counters\n\nThe unchanged accepted SDK was measured in three fixed blocks at primary16, primary32 and merge54/32, using separate branch and cache panels. These are diagnostic counters around warm requests, not acceptance timings or a candidate speedup. The existing 172-file source and SDK14da8231 remain unchanged.\n\nEach panel counts four events together, with actual running percentages validated. A six-event capability-only group was unschedulable and is retained as not-counted evidence, not interpreted. The existing SDK FIFO/ack protocol enables counters after loading and three primes, then disables them before export. No build/helper/hash/archive/second owned CPU overlaps requests. Required independent verifiers run serially between captures.\n\n## Counter medians\n\nEach cell is the median of three process observations. Cycles/instructions are per timed request, IPC is the median of per-process ratios. Different panels are separate processes. All event scopes are userland and include useful arithmetic, polling and coordination; rusage CPU also includes kernel time. Generic cache events do not measure DRAM bytes, bandwidth or load latency.\n\n|Case|Panel|Cycles/request million|Instructions/request million|IPC|CPU ms/request|Request ms|\n|---|---|---:|---:|---:|---:|---:|\n");
    for case in ["1x2-w16", "1x2-w32", "merge-54x1-w32"] {
        for panel in ["branch", "cache"] {
            let rs: Vec<_> = rows
                .iter()
                .filter(|r| r["job"]["case"]["name"] == case && r["job"]["panel"] == panel)
                .collect();
            assert_eq!(rs.len(), 3);
            let med =
                |f: &dyn Fn(&Value) -> f64| median(&rs.iter().map(|r| f(r)).collect::<Vec<_>>());
            writeln!(
                md,
                "|{case}|{panel}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|",
                med(&|v| v["cycles_per_request"].as_f64().unwrap()) / 1e6,
                med(&|v| v["instructions_per_request"].as_f64().unwrap()) / 1e6,
                med(&|v| v["IPC"].as_f64().unwrap()),
                med(&|v| (v["summary"]["user_cpu_ms"].as_f64().unwrap()
                    + v["summary"]["system_cpu_ms"].as_f64().unwrap())
                    / v["summary"]["rounds"].as_f64().unwrap()),
                med(&|v| v["summary"]["loop_wall_ms"].as_f64().unwrap()
                    / v["summary"]["rounds"].as_f64().unwrap())
            )
            .unwrap();
        }
    }
    md.push_str("\nThese counters do not isolate SMT, memory latency or scheduling causes. Worker count can change algorithm geometry, coordination and frequency. Three diagnostic blocks do not establish broad confidence bounds. Raw branch/cache counts, runtime percentages, SDK samples and commands remain in analysis and the archive.\n\n## Validation and scope\n\nAll 6,252 SDK calls internally verify; all 18 exported proofs independently pass Gnark, are 164 bytes, have exact fixture bindings and are fresh against 12,125 prior/new identities. Ledger **12,125→12,143**, 36 proof/public members. All frozen source/library/input/helper pins are rechecked. No native rebuild, new native/portable suite or new performance qualification is claimed. No AWS/global perf setting/production resource changes occur.\n\nThe [accepted CPU/Metal comparison](../2026-10-27-clean-fft-adoption/README.md) retains all eight supplied historical M5 references; none is remeasured here. No 54 reference was supplied. The original CPU 10× objective remains active and unmet, 0/8. No hardware counter ratio is treated as a speedup or removable wall time.\n\n[Design](artifacts/design.md), [all rows](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [workflow](artifacts/workflow/result.json), [artifact hashes](artifact-index.json).\n\nThe [perf-stat manual](https://man7.org/linux/man-pages/man1/perf-stat.1.html) documents FIFO acknowledgement, inherited counting, grouping and output fields. Existing [IBS observations](../2026-10-23-local-ibs-memory/README.md) and previous negative experiments remain separate evidence.\n");
    fs::write(d.join("README.md"), md).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    println!(
        "Closed18PMUprocesses6252SDK18Gnark;ledger12143;{}artifacthashes;no source change",
        index.len()
    );
}
fn archive(root: &Path, dir: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            archive(root, &p, dest, index)
        } else if e.file_type().unwrap().is_file()
            && ["rs", "md", "json", "log", "txt", "proof", "public"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let rel = p.strip_prefix(root).unwrap();
            let out = dest.join(rel);
            fs::create_dir_all(out.parent().unwrap()).unwrap();
            fs::copy(&p, &out).unwrap();
            assert_eq!(hash(&p), hash(&out));
            index.insert(rel.display().to_string(), hash(out));
        }
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare|finish"),
    }
}
