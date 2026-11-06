use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-waves176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
const LABELS: [&str; 3] = ["control-a", "control-b", "shared"];
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
include!("perf_io.rs");
fn profr(label: &str) -> usize {
    if label == "shared" {
        16777216
    } else {
        2097152
    }
}
fn args(w: usize, label: &str, n: usize) -> Vec<String> {
    vec![
        "taskset".into(),
        "-c".into(),
        match w {
            1 => "2",
            16 => "0-15",
            32 => "0-31",
            _ => panic!("workers"),
        }
        .into(),
        format!("{Q}/driver"),
        w.to_string(),
        n.to_string(),
        label.into(),
    ]
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600})
}
fn prepare() {
    assert!(!r("plan.json").exists());
    let q = Path::new(Q);
    let review = read(q.join("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(
        hash(q.join("driver")),
        review["identities"]["driver"]["sha256"]
    );
    assert_eq!(
        read(q.join("driver-repair-runs/result.json"))["status"],
        "complete"
    );
    let mut pins = read(q.join("driver-repair-pins.json"))
        .as_object()
        .unwrap()
        .iter()
        .map(|(p, h)| (p.clone(), h.as_str().unwrap().to_owned()))
        .collect::<BTreeMap<_, _>>();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for f in [
        "review.json",
        "review.rs",
        "review",
        "oracle.json",
        "driver",
        "driver.rs",
        "bulk.rs",
        "fixture.rs",
        "perf.rs",
        "build-plan.json",
        "driver-repair-plan.json",
        "driver-repair.json",
        "driver-repair-runs/result.json",
    ] {
        pins.insert(q.join(f).display().to_string(), hash(q.join(f)));
    }
    let mut host = BTreeMap::new();
    for (name, argv) in [
        ("time_utc", vec!["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"]),
        ("cpu", vec!["lscpu", "-J"]),
        ("topology", vec!["lscpu", "-e=CPU,CORE,SOCKET,ONLINE"]),
        ("kernel", vec!["uname", "-a"]),
        ("perf", vec!["perf", "--version"]),
        ("sudo_noninteractive", vec!["sudo", "-n", "true"]),
        (
            "processes",
            vec!["ps", "-eo", "pid,ppid,pcpu,comm,args", "--sort=-pcpu"],
        ),
    ] {
        let out = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(out.status.success());
        host.insert(name,json!({"argv":argv,"stdout":String::from_utf8(out.stdout).unwrap(),"stderr":String::from_utf8(out.stderr).unwrap()}));
    }
    for (name, path) in [
        (
            "governor",
            "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor",
        ),
        ("boost", "/sys/devices/system/cpu/cpufreq/boost"),
        (
            "perf_event_paranoid",
            "/proc/sys/kernel/perf_event_paranoid",
        ),
    ] {
        host.insert(
            name,
            json!({"path":path,"value":fs::read_to_string(path).ok()}),
        );
    }
    save(r("host.json"), &json!(host));
    for f in [
        "campaign.rs",
        "campaign",
        "perf_io.rs",
        "protocol.md",
        "host.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    pins.insert(X.into(), hash(X));
    let env = json!({"MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1"});
    let mut steps = vec![];
    for w in [1, 16, 32] {
        for label in LABELS {
            steps.push(step(format!("preflight-w{w}-{label}"), args(w, label, 64)));
        }
    }
    steps.push(step(
        "preflight-check".into(),
        vec![r("campaign").display().to_string(), "preflight".into()],
    ));
    for b in 0..15 {
        let mut order = [1usize, 16, 32];
        order.rotate_left(b % 3);
        if b % 2 == 1 {
            order.reverse();
        }
        for (i, w) in order.into_iter().enumerate() {
            let mut labels = LABELS;
            labels.rotate_left((b + i) % 3);
            if b % 2 == 1 {
                labels.reverse();
            }
            for label in labels {
                steps.push(step(
                    format!("latency-w{w}-b{b:02}-{label}"),
                    args(w, label, 262144),
                ));
            }
        }
    }
    for dir in ["stat", "perf"] {
        fs::create_dir(r(dir)).unwrap();
    }
    for fifo in ["ctl", "ack"] {
        assert!(Command::new("mkfifo")
            .arg(r(fifo))
            .status()
            .unwrap()
            .success());
    }
    for profile in [false, true] {
        for w in [16, 32] {
            for label in LABELS {
                let name = format!("{}-w{w}-{label}", if profile { "cycles" } else { "stat" });
                let mut penv = env.clone();
                penv["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
                penv["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
                let mut argv = vec!["sudo".into(), "-n".into(), "env".into()];
                for (k, v) in penv.as_object().unwrap() {
                    argv.push(format!("{k}={}", v.as_str().unwrap()));
                }
                argv.push("perf".into());
                if profile {
                    argv.extend([
                        "record".into(),
                        "--no-buildid-cache".into(),
                        "--sample-cpu".into(),
                        "--timestamp".into(),
                        "-P".into(),
                        "-e".into(),
                        "cycles:u".into(),
                        "-c".into(),
                        "16000000".into(),
                        "-m".into(),
                        "512".into(),
                        "-o".into(),
                        format!("{R}/perf/{name}.data"),
                    ]);
                } else {
                    argv.extend([
                        "stat".into(),
                        "--no-big-num".into(),
                        "-x".into(),
                        ";".into(),
                        "-e".into(),
                        "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                        "-o".into(),
                        format!("{R}/stat/{name}.csv"),
                    ]);
                }
                argv.extend([
                    "--delay=-1".into(),
                    format!("--control=fifo:{R}/ctl,{R}/ack"),
                    "--".into(),
                ]);
                argv.extend(args(
                    w,
                    label,
                    if profile {
                        profr(label)
                    } else {
                        profr(label) / 4
                    },
                ));
                let mut s = step(name, argv);
                s["environment"] = penv;
                steps.push(s);
            }
        }
    }
    for w in [16, 32] {
        for label in LABELS {
            let name = format!("cycles-w{w}-{label}");
            let data = format!("{R}/perf/{name}.data");
            steps.push(step(
                format!("own-{name}"),
                vec![
                    "sudo".into(),
                    "-n".into(),
                    r("campaign").display().to_string(),
                    "own".into(),
                    data.clone(),
                ],
            ));
            for meta in [false, true] {
                let mut argv = vec![
                    "perf".into(),
                    "script".into(),
                    "--header".into(),
                    "--ns".into(),
                    "-i".into(),
                    data.clone(),
                    "-F".into(),
                    "pid,tid,cpu,time,period,ip,sym".into(),
                ];
                if meta {
                    argv.extend([
                        "--show-mmap-events".into(),
                        "--show-task-events".into(),
                        "--show-lost-events".into(),
                    ]);
                }
                steps.push(step(
                    format!("{}-{name}", if meta { "metadata" } else { "decode" }),
                    argv,
                ));
            }
            steps.push(step(
                format!("events-{name}"),
                vec![
                    "perf".into(),
                    "evlist".into(),
                    "-v".into(),
                    "-i".into(),
                    data.clone(),
                ],
            ));
            steps.push(step(
                format!("loss-{name}"),
                vec![r("campaign").display().to_string(), "loss".into(), data],
            ));
        }
    }
    assert_eq!(steps.len(), 187);
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"environment":env,"steps":steps}),
    );
    println!(
        "Frozen187commands/156driverprocesses/{}pins; no clock observed",
        pins.len()
    );
}
fn wave(name: &str) -> Value {
    let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let rows = text
        .lines()
        .filter_map(|l| l.strip_prefix("ROOT_WAVE "))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 1);
    serde_json::from_str(rows[0]).unwrap()
}
fn validate(v: &Value, w: usize, label: &str, n: usize) {
    assert_eq!(v["label"], label);
    assert_eq!(v["workers"], w);
    assert_eq!(v["roots_per_wave"], w);
    assert_eq!(v["rounds"], n);
    assert_eq!(v["timed_allocations"], 0);
    assert_eq!(v["priming_passes"], 3);
    assert_eq!(v["priming_rounds"], 256);
    assert_eq!(v["fixture_bytes"], 262144);
    assert_eq!(
        v["fixture_sha256"],
        read(format!("{Q}/oracle.json"))["fixture_hash"]
    );
    assert_eq!(v["zero_request_publication_or_queue_cost"], true);
    assert_eq!(v["worker_start_end_barriers_inside_clock"], true);
    assert_eq!(v["proofs"], 0);
    assert_eq!(v["new_production_threads"], 0);
    assert_eq!(v["coordinator"], if w == 1 { 0 } else { 2 });
    let sharing = label == "shared";
    assert_eq!(v["active_workers"], if sharing { 1 } else { w });
    assert_eq!(v["parked_workers"], if sharing { w - 1 } else { 0 });
    let rows = v["worker_rows"].as_array().unwrap();
    assert_eq!(rows.len(), w);
    let mut tids = BTreeSet::new();
    for (i, row) in rows.iter().enumerate() {
        assert_eq!(row["worker"], i);
        assert_eq!(row["cpu"], if w == 1 { 2 } else { i });
        assert_eq!(row["active"], !sharing || i == if w == 1 { 0 } else { 2 });
        assert!(tids.insert(row["tid"].as_u64().unwrap()));
    }
}
fn preflight() {
    for w in [1, 16, 32] {
        let mut expected = None;
        for label in LABELS {
            let name = format!("preflight-w{w}-{label}");
            assert_eq!(read(r(format!("runs/{name}/exit.json")))["returncode"], 0);
            let v = wave(&name);
            validate(&v, w, label, 64);
            if let Some(ref e) = expected {
                assert_eq!(&v["output_sha256"], e);
            } else {
                expected = Some(v["output_sha256"].clone());
            }
        }
    }
    println!("Checked9preflights/zero allocations/equal output hashes/fixture and active TID-CPU mapping");
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn interval(v: &[f64]) -> Value {
    assert_eq!(v.len(), 15);
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[7],"low":s[3],"high":s[11],"wins":v.iter().filter(|&&r|r<1.).count(),"ratios":v})
}
fn finish() {
    assert!(!r("analysis.json").exists());
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = read(r("plan.json"));
    let mut calls = BTreeMap::new();
    let mut receipts = vec![];
    for s in plan["steps"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        let b = r(format!("runs/{name}"));
        let e = read(b.join("exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let cmd = read(b.join("command.json"));
        assert_eq!(
            &cmd["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap()
        );
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(b.join(format!("{stream}.log"))),
                e[format!("{stream}_sha256")]
            );
        }
        receipts.push(json!({"name":name,"exit_sha256":hash(b.join("exit.json"))}));
        if name != "preflight-check"
            && ["preflight-", "latency-", "stat-", "cycles-"]
                .iter()
                .any(|p| name.starts_with(p))
        {
            let v = wave(name);
            let argv = s["argv"].as_array().unwrap();
            let p = argv
                .iter()
                .position(|a| a == &json!(format!("{Q}/driver")))
                .unwrap();
            validate(
                &v,
                argv[p + 1].as_str().unwrap().parse().unwrap(),
                argv[p + 3].as_str().unwrap(),
                argv[p + 2].as_str().unwrap().parse().unwrap(),
            );
            calls.insert(name.to_string(), v);
        }
    }
    assert_eq!(calls.len(), 156);
    assert_eq!(receipts.len(), 187);
    let mut rows = vec![];
    let mut failed = vec![];
    let mut vectors = BTreeMap::new();
    let mut checks = 0;
    for w in [1, 16, 32] {
        let mut times = BTreeMap::new();
        let mut cpu = BTreeMap::new();
        let mut rss = BTreeMap::new();
        let mut busy = BTreeMap::new();
        let mut faults = BTreeMap::new();
        for label in LABELS {
            let mut t = vec![];
            let mut c = vec![];
            let mut b = vec![];
            let mut f = vec![];
            let mut peak = 0f64;
            for block in 0..15 {
                let v = &calls[&format!("latency-w{w}-b{block:02}-{label}")];
                assert_eq!(v["rounds"], 262144);
                for other in LABELS {
                    assert_eq!(
                        v["output_sha256"],
                        calls[&format!("latency-w{w}-b{block:02}-{other}")]["output_sha256"]
                    );
                }
                let ns = v["elapsed_ns"].as_f64().unwrap();
                let ms = v["usage"]["user_cpu_ms"].as_f64().unwrap()
                    + v["usage"]["system_cpu_ms"].as_f64().unwrap();
                t.push(ns / 262144.);
                c.push(ms / 262144.);
                b.push(ms * 1e6 / ns);
                f.push(v["usage"]["minor_faults"].as_f64().unwrap() / 262144.);
                peak = peak.max(v["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
            }
            times.insert(label, t);
            cpu.insert(label, median(&c));
            busy.insert(label, median(&b));
            faults.insert(label, median(&f));
            rss.insert(label, peak);
        }
        let mut pairs = BTreeMap::new();
        for (num, den) in [
            ("control-b", "control-a"),
            ("shared", "control-a"),
            ("shared", "control-b"),
        ] {
            let ratios = times[num]
                .iter()
                .zip(&times[den])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let stats = interval(&ratios);
            let calib = num == "control-b";
            let pass = if calib {
                stats["low"].as_f64().unwrap() >= 0.97 && stats["high"].as_f64().unwrap() <= 1.03
            } else {
                stats["high"].as_f64().unwrap() <= 1.01
            };
            checks += 1;
            if !pass {
                failed.push(format!(
                    "w{w}/{num}/{den}/{}",
                    if calib {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if w > 1 && num == "shared" {
                checks += 1;
                if stats["high"].as_f64().unwrap() >= 1. || stats["wins"].as_u64().unwrap() < 11 {
                    failed.push(format!("w{w}/{num}/{den}/gain"));
                }
            }
            vectors.insert(format!("w{w}/{num}/{den}"), ratios);
            pairs.insert(format!("{num}/{den}"), stats);
        }
        rows.push(json!({"workers":w,"median_ns_per_wave":times.iter().map(|(k,v)|(*k,median(v))).collect::<BTreeMap<_,_>>(),"CPU_ms_per_wave":cpu,"median_busy_CPUs":busy,"median_minor_faults_per_wave":faults,"max_observed_process_RSS_MiB":rss,"pairs":pairs}));
    }
    let mut aggregates = BTreeMap::new();
    for den in ["control-a", "control-b"] {
        let a = (0..15)
            .map(|b| {
                ((vectors[&format!("w16/shared/{den}")][b].ln()
                    + vectors[&format!("w32/shared/{den}")][b].ln())
                    / 2.)
                    .exp()
            })
            .collect::<Vec<_>>();
        let stats = interval(&a);
        checks += 1;
        if stats["high"].as_f64().unwrap() >= 1. || stats["wins"].as_u64().unwrap() < 11 {
            failed.push(format!("threaded/shared/{den}/gain"));
        }
        aggregates.insert(den, stats);
    }
    assert_eq!(checks, 15);
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut all_symbols = BTreeMap::new();
    let mut samples_total = 0;
    for w in [16, 32] {
        for label in LABELS {
            let stat = format!("stat-w{w}-{label}");
            let n = profr(label) / 4;
            assert_eq!(calls[&stat]["rounds"], n);
            if label == "control-b" {
                assert_eq!(
                    calls[&stat]["output_sha256"],
                    calls[&format!("stat-w{w}-control-a")]["output_sha256"]
                );
            }
            let mut events = BTreeMap::new();
            for line in fs::read_to_string(r(format!("stat/{stat}.csv")))
                .unwrap()
                .lines()
            {
                let f = line.split(';').collect::<Vec<_>>();
                if f.len() < 5
                    || ![
                        "cycles:u",
                        "instructions:u",
                        "cache-misses:u",
                        "branch-misses:u",
                    ]
                    .contains(&f[2].trim())
                {
                    continue;
                }
                let count = f[0].trim().parse::<f64>().ok();
                let coverage = f[4].trim().parse::<f64>().ok();
                let pass = count.is_some() && coverage.is_some_and(|v| v >= 90.);
                if !pass {
                    failed.push(format!("{stat}/{}/counter", f[2].trim()));
                }
                events.insert(f[2].trim().to_string(),json!({"count":count,"coverage_percent":coverage,"qualified":pass,"per_wave":count.map(|v|v/n as f64)}));
            }
            assert_eq!(events.len(), 4);
            counters.insert(stat, json!(events));
            let name = format!("cycles-w{w}-{label}");
            let v = &calls[&name];
            assert_eq!(v["rounds"], profr(label));
            if label == "control-b" {
                assert_eq!(
                    v["output_sha256"],
                    calls[&format!("cycles-w{w}-control-a")]["output_sha256"]
                );
            }
            let pid = v["pid"].as_u64().unwrap();
            let assigned = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| {
                    (
                        r["tid"].as_u64().unwrap(),
                        (r["cpu"].as_u64().unwrap(), r["active"].as_bool().unwrap()),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            let meta = fs::read_to_string(r(format!("runs/metadata-{name}/stdout.log"))).unwrap();
            let owners = meta
                .lines()
                .filter(|s| s.contains("PERF_RECORD_MMAP") && s.contains(&format!("{Q}/driver")))
                .map(|s| {
                    s.split_whitespace()
                        .next()
                        .unwrap()
                        .split('/')
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<BTreeSet<_>>();
            assert_eq!(owners, BTreeSet::from([pid]));
            let mut owned = BTreeSet::from([pid]);
            for line in meta.lines() {
                if let Some((_, s)) = line.split_once("PERF_RECORD_FORK(") {
                    let p = s
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|v| v.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if p[0] == pid {
                        owned.insert(p[1]);
                    }
                }
            }
            assert!(assigned.keys().all(|tid| owned.contains(tid)));
            let attrs = fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();
            for word in ["cycles:u", "16000000", "CPU", "TIME", "PERIOD"] {
                assert!(attrs.contains(word));
            }
            assert!(!attrs.contains("freq: 1"));
            let lost = read(r(format!("runs/loss-{name}/stdout.log")))["zero_lost"] != true;
            let data = r(format!("perf/{name}.data"));
            let owner = read(r(format!("runs/own-{name}/stdout.log")));
            assert_eq!(owner["sha256_before"], owner["sha256_after"]);
            assert_eq!(hash(&data), owner["sha256_before"]);
            let mut count = 0u64;
            let mut unknown = 0u64;
            let mut active = 0u64;
            let mut bins = BTreeMap::<String, u64>::new();
            let mut tids = BTreeMap::<u64, u64>::new();
            for line in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|s| !s.starts_with('#') && !s.trim().is_empty())
            {
                let f = line.split_whitespace().collect::<Vec<_>>();
                let (p, t) = f[0].split_once('/').unwrap();
                assert_eq!(p.parse::<u64>().unwrap(), pid);
                let tid = t.parse::<u64>().unwrap();
                assert!(owned.contains(&tid));
                let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                assert!(cpu < w as u64);
                if let Some(&(expected, is_active)) = assigned.get(&tid) {
                    assert_eq!(expected, cpu);
                    if is_active {
                        active += 1;
                    }
                }
                assert_eq!(f[3], "16000000");
                let raw = f[5..].join(" ");
                let sym = rustc_demangle::try_demangle(&raw)
                    .map(|s| format!("{s:#}"))
                    .unwrap_or(raw);
                if sym.starts_with("[unknown]") {
                    unknown += 1;
                }
                *bins.entry(sym).or_default() += 1;
                *tids.entry(tid).or_default() += 1;
                count += 1;
            }
            let unknown_share = if count == 0 {
                1.
            } else {
                unknown as f64 / count as f64
            };
            let active_share = if count == 0 {
                0.
            } else {
                active as f64 / count as f64
            };
            let qualified = count >= 1000 && unknown_share <= 0.05 && active_share >= 0.95 && !lost;
            if !qualified {
                failed.push(format!("{name}/profile_qualification"));
            }
            all_symbols.insert(name.clone(), json!(bins));
            profiles.insert(name,json!({"qualified":qualified,"samples":count,"unknown_samples":unknown,"unknown_share":unknown_share,"active_worker_samples":active,"active_worker_share":active_share,"other_owned_samples":count-active,"zero_lost":!lost,"PID":pid,"worker_tid_cpu_active":assigned,"owned_TIDs_from_metadata":owned,"samples_by_tid":tids,"raw_sha256":hash(data)}));
            samples_total += count;
        }
    }
    save(r("profile-symbols.json"), &json!(all_symbols));
    save(
        r("analysis.json"),
        &json!({"gate_passed":failed.is_empty(),"failed_checks":failed,"timing_checks":checks,"rows":rows,"threaded_aggregates":aggregates,"counters":counters,"profiles":profiles,"profile_samples_total":samples_total,"caller_receipts":calls,"command_receipts":receipts,"adopted":false,"production_changed":false,"broker_implemented":false,"new_proofs":0,"proof_ledger":30846,"goal_achieved":false}),
    );
    println!("Complete187commands/156ROOT_WAVEresults/15timingchecks/6profiles/{samples_total}samples;gate={};{}failed checks",failed.is_empty(),failed.len());
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("preflight") => preflight(),
        Some("finish") => finish(),
        Some("own") => own(&std::env::args().nth(2).unwrap()),
        Some("loss") => loss(&std::env::args().nth(2).unwrap()),
        _ => panic!("prepare/preflight/finish/own/loss"),
    }
}
