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
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-callers176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-codec176";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn contexts() -> Vec<(String, String, usize, usize)> {
    let mut out = vec![];
    for (kind, n) in [
        ("G1", 200),
        ("G1", 201),
        ("G1", 352),
        ("G1", 353),
        ("G2", 200),
        ("G2", 201),
        ("G2", 352),
        ("G2", 353),
    ] {
        for w in [1, 16, 32] {
            out.push((format!("{kind}-{n}-w{w}"), kind.into(), n, w));
        }
    }
    out
}
fn binary(arm: &str) -> PathBuf {
    match arm {
        "frozen" => PathBuf::from("/home/ubuntu/src/zopro-perf-work/word-export176/caller-frozen"),
        "OFF" | "ON" => Path::new(Q).join(format!("caller-{arm}")),
        _ => panic!("arm"),
    }
}
fn rounds(c: &(String, String, usize, usize)) -> usize {
    if c.2 <= 201 {
        32768
    } else {
        16384
    }
}
fn selected(c: &&(String, String, usize, usize)) -> bool {
    (c.2 == 200 && c.3 == 1) || ([200, 352].contains(&c.2) && c.3 == 32)
}
fn profile_rounds(c: &(String, String, usize, usize)) -> usize {
    if c.3 == 1 {
        2097152
    } else {
        65536
    }
}
fn args(c: &(String, String, usize, usize), arm: &str, rounds: usize) -> Vec<String> {
    vec![
        "taskset".into(),
        "-c".into(),
        if c.3 == 1 {
            "2".into()
        } else if c.3 == 16 {
            "0-15".into()
        } else {
            "0-31".into()
        },
        binary(arm).display().to_string(),
        c.1.clone(),
        c.2.to_string(),
        c.3.to_string(),
        rounds.to_string(),
        if arm == "ON" {
            "ON".into()
        } else {
            "OFF".into()
        },
    ]
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600})
}
fn prepare() {
    assert!(!r("plan.json").exists());
    let q = Path::new(Q);
    assert_eq!(read(q.join("build-runs/result.json"))["status"], "complete");
    let review = read(q.join("review.json"));
    assert_eq!(review["accepted_sources_recovered"], 176);
    let ids = read(q.join("identities.json"));
    for arm in ["frozen", "OFF", "ON"] {
        assert_eq!(
            hash(ids[arm]["binary"].as_str().unwrap()),
            ids[arm]["binary_sha256"]
        );
    }
    assert_ne!(ids["OFF"]["binary_sha256"], ids["ON"]["binary_sha256"]);
    let mut pins = read(q.join("build-pins.json"))
        .as_object()
        .unwrap()
        .iter()
        .map(|(p, h)| (p.clone(), h.as_str().unwrap().to_owned()))
        .collect::<BTreeMap<_, _>>();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for f in [
        "build-plan.json",
        "build-runs/result.json",
        "identities.json",
        "review.json",
        "review.rs",
        "review",
        "codegen.json",
        "source-audit.json",
        "frozen-reference.json",
        "driver.rs",
        "perf.rs",
        "field/libzopro_bn254.rlib",
        "libs/OFF/libzopro_core.rlib",
        "libs/ON/libzopro_core.rlib",
    ] {
        pins.insert(q.join(f).display().to_string(), hash(q.join(f)));
    }
    for arm in ["frozen", "OFF", "ON"] {
        pins.insert(binary(arm).display().to_string(), hash(binary(arm)));
    }
    let mut host = BTreeMap::new();
    for (name, argv) in [
        ("time_utc", vec!["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"]),
        ("kernel", vec!["uname", "-a"]),
        ("cpu", vec!["lscpu", "-J"]),
        ("topology", vec!["lscpu", "-e=CPU,CORE,SOCKET,ONLINE"]),
        ("perf", vec!["perf", "--version"]),
        ("sudo_noninteractive", vec!["sudo", "-n", "true"]),
        (
            "processes",
            vec!["ps", "-eo", "pid,ppid,pcpu,comm,args", "--sort=-pcpu"],
        ),
    ] {
        let output = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(output.status.success(), "{name}");
        host.insert(name,json!({"argv":argv,"stdout":String::from_utf8(output.stdout).unwrap(),"stderr":String::from_utf8(output.stderr).unwrap()}));
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
    for f in ["campaign.rs", "campaign", "protocol.md", "host.json"] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    pins.insert(X.into(), hash(X));
    let env = json!({"MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1"});
    let cs = contexts();
    let mut steps = vec![];
    for c in &cs {
        for arm in ["frozen", "OFF", "ON"] {
            steps.push(step(format!("preflight-{}-{arm}", c.0), args(c, arm, 64)));
        }
    }
    steps.push(step(
        "preflight-check".into(),
        vec![r("campaign").display().to_string(), "preflight".into()],
    ));
    for b in 0..15 {
        let mut order = (0..cs.len()).collect::<Vec<_>>();
        order.rotate_left(b % cs.len());
        if b % 2 == 1 {
            order.reverse();
        }
        for i in order {
            let c = &cs[i];
            let mut arms = ["frozen", "OFF", "ON"];
            arms.rotate_left((b + i) % 3);
            if b % 2 == 1 {
                arms.reverse();
            }
            for arm in arms {
                steps.push(step(
                    format!("latency-{}-b{b:02}-{arm}", c.0),
                    args(c, arm, rounds(c)),
                ));
            }
        }
    }
    for dir in ["stat", "perf"] {
        fs::create_dir(r(dir)).unwrap();
    }
    for name in ["ctl", "ack"] {
        assert!(Command::new("mkfifo")
            .arg(r(name))
            .status()
            .unwrap()
            .success());
    }
    let selected = cs.iter().filter(selected).collect::<Vec<_>>();
    assert_eq!(selected.len(), 6);
    for profile in [false, true] {
        for c in &selected {
            for arm in ["frozen", "OFF", "ON"] {
                let name = format!("{}-{}-{arm}", if profile { "cycles" } else { "stat" }, c.0);
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
                    c,
                    arm,
                    if profile {
                        profile_rounds(c)
                    } else {
                        profile_rounds(c) / 4
                    },
                ));
                let mut s = step(name, argv);
                s["environment"] = penv;
                steps.push(s);
            }
        }
    }
    for c in selected {
        for arm in ["frozen", "OFF", "ON"] {
            let name = format!("cycles-{}-{arm}", c.0);
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
    assert_eq!(steps.len(), 1279);
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"environment":env,"steps":steps}),
    );
    println!("Frozen1279 commands/1188caller processes/{}pins;72preflights+check/1080latency/18PMU/18profiles/90offline; no capture yet",pins.len());
}
fn preflight() {
    let mut count = 0;
    for c in contexts() {
        let mut expected = None;
        let mut caps = None;
        for arm in ["frozen", "OFF", "ON"] {
            let n = format!("preflight-{}-{arm}", c.0);
            let v = caller(&n);
            assert_eq!(read(r(format!("runs/{n}/exit.json")))["returncode"], 0);
            assert_eq!(v["rounds_per_worker"], 64);
            assert_eq!(v["complete_generator_oracle"], true);
            assert_eq!(v["timed_allocations"], 0);
            assert_eq!(v["kind"], c.1);
            assert_eq!(v["size"], c.2);
            assert_eq!(v["workers"], c.3);
            if let Some(e) = &expected {
                assert_eq!(&v["output_sha256"], e);
            } else {
                expected = Some(v["output_sha256"].clone());
            }
            if let Some(e) = &caps {
                assert_eq!(&v["scratch_capacity_bytes_by_worker"], e);
            } else {
                caps = Some(v["scratch_capacity_bytes_by_worker"].clone());
            }
            count += 1;
        }
    }
    assert_eq!(count, 72);
    println!("Checked72preflights/complete independent generator hashes/capacities/zero timed allocations");
}
fn own(data: &str) {
    assert!(Path::new(data).starts_with(r("perf")));
    assert!(data.ends_with(".data"));
    let before = hash(data);
    let md = fs::metadata(r("campaign")).unwrap();
    assert!(Command::new("chown")
        .arg(format!("{}:{}", md.uid(), md.gid()))
        .arg(data)
        .status()
        .unwrap()
        .success());
    assert_eq!(hash(data), before);
    println!(
        "{}",
        json!({"sha256_before":before,"sha256_after":hash(data),"data":data})
    );
}
fn loss(data: &str) {
    let mut child = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = 0;
    let mut lost = vec![];
    for line in BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        lines += 1;
        if line.contains("PERF_RECORD_LOST") {
            lost.push(line);
        }
    }
    assert!(child.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":lines,"zero_lost":lost.is_empty(),"lost":lost})
    );
}
fn caller(n: &str) -> Value {
    let text = fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap();
    let mut rows = text.lines().filter_map(|l| l.strip_prefix("CALLER "));
    let value = serde_json::from_str(rows.next().unwrap()).unwrap();
    assert!(rows.next().is_none());
    value
}
fn median(a: &[f64]) -> f64 {
    let mut a = a.to_vec();
    a.sort_by(f64::total_cmp);
    a[a.len() / 2]
}
fn interval(a: &[f64]) -> Value {
    assert_eq!(a.len(), 15);
    let mut x = a.to_vec();
    x.sort_by(f64::total_cmp);
    json!({"median":x[7],"low":x[3],"high":x[11],"wins":a.iter().filter(|&&r|r<1.).count(),"ratios":a})
}
fn finish() {
    assert!(!r("analysis.json").exists());
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = read(r("plan.json"));
    let mut callers = BTreeMap::new();
    let mut receipts = vec![];
    for step in plan["steps"].as_array().unwrap() {
        let n = step["name"].as_str().unwrap();
        let base = r(format!("runs/{n}"));
        let e = read(base.join("exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let command = read(base.join("command.json"));
        let actual = command["argv"].as_array().unwrap();
        assert_eq!(&actual[4..], step["argv"].as_array().unwrap());
        for f in ["stdout", "stderr"] {
            assert_eq!(
                hash(base.join(format!("{f}.log"))),
                e[format!("{f}_sha256")]
            );
        }
        receipts.push(json!({"name":n,"exit_sha256":hash(base.join("exit.json"))}));
        if n != "preflight-check"
            && ["preflight-", "latency-", "stat-", "cycles-"]
                .iter()
                .any(|p| n.starts_with(p))
        {
            let v = caller(n);
            assert_eq!(v["complete_generator_oracle"], true);
            assert_eq!(v["timed_allocations"], 0);
            assert_eq!(v["priming_passes"], 3);
            assert_eq!(v["priming_rounds"], 128);
            let nworkers = v["workers"].as_u64().unwrap() as usize;
            assert_eq!(v["worker_rows"].as_array().unwrap().len(), nworkers);
            let mut tids = BTreeSet::new();
            for (i, row) in v["worker_rows"].as_array().unwrap().iter().enumerate() {
                assert_eq!(row["worker"], i);
                assert_eq!(row["cpu"], if nworkers == 1 { 2 } else { i });
                assert!(tids.insert(row["tid"].as_u64().unwrap()));
            }
            let cap = &v["scratch_capacity_bytes_by_worker"];
            assert_eq!(cap.as_array().unwrap().len(), nworkers);
            assert!(cap.as_array().unwrap().iter().all(|v| v == &cap[0]));
            let argv = step["argv"].as_array().unwrap();
            let arm = if n.ends_with("-frozen") {
                "frozen"
            } else if n.ends_with("-ON") {
                "ON"
            } else {
                "OFF"
            };
            let bin = json!(binary(arm));
            let pos = argv.iter().position(|arg| arg == &bin).unwrap();
            assert_eq!(v["kind"], argv[pos + 1]);
            assert_eq!(
                v["size"].as_u64().unwrap(),
                argv[pos + 2].as_str().unwrap().parse::<u64>().unwrap()
            );
            assert_eq!(
                v["workers"].as_u64().unwrap(),
                argv[pos + 3].as_str().unwrap().parse::<u64>().unwrap()
            );
            assert_eq!(
                v["rounds_per_worker"].as_u64().unwrap(),
                argv[pos + 4].as_str().unwrap().parse::<u64>().unwrap()
            );
            assert_eq!(v["arm"], argv[pos + 5]);
            callers.insert(n.to_owned(), v);
        }
    }
    assert_eq!(callers.len(), 1188);
    assert_eq!(receipts.len(), 1279);
    for (name, v) in &callers {
        let (prefix, _) = name.rsplit_once('-').unwrap();
        for arm in ["frozen", "OFF", "ON"] {
            let other = &callers[&format!("{prefix}-{arm}")];
            assert_eq!(v["output_sha256"], other["output_sha256"]);
            assert_eq!(
                v["scratch_capacity_bytes_by_worker"],
                other["scratch_capacity_bytes_by_worker"]
            );
        }
    }
    let mut rows = vec![];
    let mut failed = vec![];
    let mut time_vectors = BTreeMap::new();
    for c in contexts() {
        let mut times = BTreeMap::new();
        let mut cpu = BTreeMap::new();
        let mut rss = BTreeMap::new();
        let mut capacities = BTreeMap::new();
        let mut busy = BTreeMap::new();
        let mut faults = BTreeMap::new();
        let mut switches = BTreeMap::new();
        for arm in ["frozen", "OFF", "ON"] {
            let mut t = vec![];
            let mut u = vec![];
            let mut hwm = 0f64;
            let mut busy_values = vec![];
            let mut fault_values = vec![];
            let mut switch_values = vec![];
            for b in 0..15 {
                let n = format!("latency-{}-b{b:02}-{arm}", c.0);
                let v = &callers[&n];
                assert_eq!(v["kind"], c.1);
                assert_eq!(v["size"], c.2);
                assert_eq!(v["workers"], c.3);
                assert_eq!(v["rounds_per_worker"], rounds(&c));
                for other in ["frozen", "OFF", "ON"] {
                    let ov = &callers[&format!("latency-{}-b{b:02}-{other}", c.0)];
                    assert_eq!(v["output_sha256"], ov["output_sha256"]);
                    assert_eq!(
                        v["scratch_capacity_bytes_by_worker"],
                        ov["scratch_capacity_bytes_by_worker"]
                    );
                }
                let cap = v["scratch_capacity_bytes_by_worker"][0].clone();
                for each in v["scratch_capacity_bytes_by_worker"].as_array().unwrap() {
                    assert_eq!(*each, cap);
                }
                capacities.insert(arm, cap);
                let count = rounds(&c) as f64;
                let cpu_ms = v["usage"]["user_cpu_ms"].as_f64().unwrap()
                    + v["usage"]["system_cpu_ms"].as_f64().unwrap();
                t.push(v["elapsed_ns"].as_f64().unwrap() / count);
                u.push(cpu_ms / count);
                busy_values.push(cpu_ms * 1e6 / v["elapsed_ns"].as_f64().unwrap());
                fault_values.push(v["usage"]["minor_faults"].as_f64().unwrap() / count);
                switch_values.push(
                    (v["usage"]["voluntary_switches"].as_f64().unwrap()
                        + v["usage"]["involuntary_switches"].as_f64().unwrap())
                        / count,
                );
                hwm = hwm.max(v["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
            }
            times.insert(arm, t);
            cpu.insert(arm, median(&u));
            rss.insert(arm, hwm);
            busy.insert(arm, median(&busy_values));
            faults.insert(arm, median(&fault_values));
            switches.insert(arm, median(&switch_values));
        }
        let mut pairs = BTreeMap::new();
        for (num, den) in [("OFF", "frozen"), ("ON", "frozen"), ("ON", "OFF")] {
            let ratios = times[num]
                .iter()
                .zip(&times[den])
                .map(|(x, y)| x / y)
                .collect::<Vec<_>>();
            let stats = interval(&ratios);
            let pass = if num == "OFF" {
                stats["low"].as_f64().unwrap() >= 0.97 && stats["high"].as_f64().unwrap() <= 1.03
            } else {
                stats["high"].as_f64().unwrap() <= 1.01
            };
            if !pass {
                failed.push(format!(
                    "{}/{num}/{den}/{}",
                    c.0,
                    if num == "OFF" {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            pairs.insert(format!("{num}/{den}"), stats);
            time_vectors.insert(format!("{}/{num}/{den}", c.0), ratios);
        }
        rows.push(json!({"context":c.0,"rounds_per_worker":rounds(&c),"median_ns_per_wave":times.iter().map(|(k,v)|(*k,median(v))).collect::<BTreeMap<_,_>>(),"CPU_ms_per_wave":cpu,"max_observed_process_RSS_MiB":rss,"median_busy_CPUs":busy,"median_minor_faults_per_wave":faults,"median_context_switches_per_wave":switches,"scratch_capacity_bytes_first_worker":capacities,"pairs":pairs}));
    }
    let mut aggregates = BTreeMap::new();
    for curve in ["G1", "G2", "ALL"] {
        for den in ["frozen", "OFF"] {
            let values = (0..15)
                .map(|b| {
                    let keys = contexts()
                        .into_iter()
                        .filter(|c| curve == "ALL" || (c.1 == curve && c.3 == 32))
                        .collect::<Vec<_>>();
                    (keys
                        .iter()
                        .map(|c| time_vectors[&format!("{}/ON/{den}", c.0)][b].ln())
                        .sum::<f64>()
                        / keys.len() as f64)
                        .exp()
                })
                .collect::<Vec<_>>();
            let stats = interval(&values);
            if stats["high"].as_f64().unwrap() >= 1. || stats["wins"].as_u64().unwrap() < 11 {
                failed.push(format!("{curve}-aggregate/ON/{den}/gain"));
            }
            aggregates.insert(format!("{curve}/{den}"), stats);
        }
    }

    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut symbols_all = BTreeMap::new();
    for c in contexts().iter().filter(selected) {
        for arm in ["frozen", "OFF", "ON"] {
            let stat = format!("stat-{}-{arm}", c.0);
            assert_eq!(callers[&stat]["rounds_per_worker"], profile_rounds(c) / 4);
            let mut values = BTreeMap::new();
            for line in fs::read_to_string(r(format!("stat/{stat}.csv")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let fields = line.split(';').collect::<Vec<_>>();
                let event = fields[2].trim();
                if ![
                    "cycles:u",
                    "instructions:u",
                    "cache-misses:u",
                    "branch-misses:u",
                ]
                .contains(&event)
                {
                    continue;
                }
                let count = fields[0].trim().parse::<f64>().ok();
                let coverage = fields[4].trim().parse::<f64>().ok();
                let qualified = count.is_some() && coverage.is_some_and(|v| v >= 90.);
                if !qualified {
                    failed.push(format!("{stat}/{event}/counter"));
                }
                values.insert(
                    event.to_owned(),
                    json!({"count":count,"coverage_percent":coverage,"qualified":qualified,"per_wave":count.map(|n|n/(profile_rounds(c)/4)as f64)}),
                );
            }
            assert_eq!(values.len(), 4);
            counters.insert(stat, json!(values));
            let name = format!("cycles-{}-{arm}", c.0);
            let v = &callers[&name];
            assert_eq!(v["rounds_per_worker"], profile_rounds(c));
            let pid = v["pid"].as_u64().unwrap();
            let tids = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x["tid"].as_u64().unwrap())
                .chain([pid])
                .collect::<BTreeSet<_>>();
            let binary = binary(arm);
            let metadata =
                fs::read_to_string(r(format!("runs/metadata-{name}/stdout.log"))).unwrap();
            let owners = metadata
                .lines()
                .filter(|l| l.contains("PERF_RECORD_MMAP") && l.contains(binary.to_str().unwrap()))
                .map(|l| {
                    l.split_whitespace()
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
            let mut forked = BTreeSet::from([pid]);
            for l in metadata.lines() {
                if let Some((_, tail)) = l.split_once("PERF_RECORD_FORK(") {
                    let f = tail
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if f[0] == pid {
                        forked.insert(f[1]);
                    }
                }
            }
            assert!(tids.is_subset(&forked));
            let attrs = fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();
            for needle in ["cycles:u", "16000000", "CPU", "TIME", "PERIOD"] {
                assert!(attrs.contains(needle));
            }
            assert!(!attrs.contains("freq: 1"));
            let zero_lost = read(r(format!("runs/loss-{name}/stdout.log")))["zero_lost"] == true;
            let data = r(format!("perf/{name}.data"));
            let owner = read(r(format!("runs/own-{name}/stdout.log")));
            assert_eq!(owner["sha256_before"], owner["sha256_after"]);
            assert_eq!(hash(&data), owner["sha256_before"]);
            let assigned = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| (v["tid"].as_u64().unwrap(), v["cpu"].as_u64().unwrap()))
                .collect::<BTreeMap<_, _>>();
            let mut count = 0u64;
            let mut unknown = 0;
            let mut worker_samples = 0u64;
            let mut tid_counts = BTreeMap::<u64, u64>::new();
            let mut bins = BTreeMap::<String, u64>::new();
            for l in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let (p, t) = f[0].split_once('/').unwrap();
                assert_eq!(p.parse::<u64>().unwrap(), pid);
                assert!(forked.contains(&t.parse::<u64>().unwrap()));
                let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                assert!(if c.3 == 1 { cpu == 2 } else { cpu < c.3 as u64 });
                let tid = t.parse::<u64>().unwrap();
                if let Some(expected) = assigned.get(&tid) {
                    assert_eq!(*expected, cpu);
                    worker_samples += 1;
                }
                assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
                let raw = f[5..].join(" ");
                let sym = rustc_demangle::try_demangle(&raw)
                    .map(|v| format!("{v:#}"))
                    .unwrap_or(raw);
                if sym.starts_with("[unknown]") {
                    unknown += 1;
                }
                *bins.entry(sym).or_default() += 1;
                *tid_counts.entry(tid).or_default() += 1;
                count += 1;
            }
            let unknown_share = if count == 0 {
                1.
            } else {
                unknown as f64 / count as f64
            };
            let worker_share = if count == 0 {
                0.
            } else {
                worker_samples as f64 / count as f64
            };
            let qualified =
                count >= 1000 && unknown_share <= 0.05 && worker_share >= 0.95 && zero_lost;
            if !qualified {
                failed.push(format!("{name}/profile"));
            }
            symbols_all.insert(name.clone(), json!(bins));
            profiles.insert(name,json!({"samples":count,"unknown_share":unknown_share,"worker_share":worker_share,"worker_samples":worker_samples,"other_owned_samples":count-worker_samples,"qualified":qualified,"zero_lost":zero_lost,"PID":pid,"worker_tid_cpu":assigned,"owned_TIDs_from_metadata":forked,"samples_by_tid":tid_counts,"raw_sha256":hash(data)}));
        }
    }
    save(r("profile-symbols.json"), &json!(symbols_all));
    save(
        r("analysis.json"),
        &json!({"gate_passed":failed.is_empty(),"failed_checks":failed,"adopted":false,"production_changed":false,"goal_achieved":false,"new_proofs":0,"proof_ledger":30846,"timing_checks":78,"rows":rows,"aggregates":aggregates,"counters":counters,"profiles":profiles,"caller_receipts":callers,"command_receipts":receipts}),
    );
    println!(
        "Closed1279 commands/1188checkedcallers;gate={};{}failed checks",
        failed.is_empty(),
        failed.len()
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("preflight") => preflight(),
        Some("own") => own(&std::env::args().nth(2).unwrap()),
        Some("loss") => loss(&std::env::args().nth(2).unwrap()),
        _ => panic!("prepare/finish/own/loss"),
    }
}
