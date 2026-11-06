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
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176";
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
struct C {
    name: String,
    case: &'static str,
    workers: usize,
    rounds: usize,
    main: bool,
}
fn contexts() -> Vec<C> {
    let mut out = Vec::new();
    for (case, rounds, main) in [
        ("tile257", 1024, false),
        ("tile384", 1024, true),
        ("tile511", 1024, true),
        ("tile2048", 256, true),
        ("group35973", 16, true),
        ("groups31x256", 64, false),
        ("tails", 1024, false),
        ("weighted128", 1024, false),
        ("weighted256", 1024, false),
    ] {
        for workers in [1, 16, 32] {
            out.push(C {
                name: format!("{case}-w{workers}"),
                case,
                workers,
                rounds,
                main,
            });
        }
    }
    out
}
fn profiled(c: &C) -> bool {
    (["tile384", "group35973"].contains(&c.case) && [1, 32].contains(&c.workers))
        || (c.case == "weighted256" && c.workers == 32)
}
fn profr(c: &C) -> usize {
    match (c.case, c.workers) {
        ("tile384", 1) => 524288,
        ("tile384", 32) => 32768,
        ("group35973", 1) => 16384,
        ("group35973", 32) => 128,
        ("weighted256", 32) => 32768,
        _ => panic!("profilecontext"),
    }
}
fn binary_for(arm: &str) -> PathBuf {
    r(format!("caller-{arm}-tls"))
}
fn argv(c: &C, arm: &str, rounds: usize) -> Vec<String> {
    vec![
        "taskset".into(),
        "-c".into(),
        if c.workers == 1 {
            "2"
        } else if c.workers == 16 {
            "0-15"
        } else {
            "0-31"
        }
        .into(),
        binary_for(arm).display().to_string(),
        c.case.into(),
        c.workers.to_string(),
        rounds.to_string(),
        arm.into(),
    ]
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600})
}
fn result(name: &str) -> Value {
    let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let mut lines = text.lines().filter_map(|l| l.strip_prefix("RESULT "));
    let value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert!(lines.next().is_none());
    value
}
fn prepare() {
    assert!(!r("plan.json").exists());
    let review = read(r("consumer-review.json"));
    assert_eq!(review["passed"], true);
    assert_eq!(read(r("build-review.json"))["passed"], true);
    let ids = read(r("identities-tls.json"));
    for arm in ["frozen", "OFF", "ON"] {
        assert_eq!(
            hash(ids[arm]["binary"].as_str().unwrap()),
            ids[arm]["binary_sha256"]
        );
        assert_eq!(
            hash(ids[arm]["core"].as_str().unwrap()),
            ids[arm]["core_sha256"]
        );
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("consumer-tls-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for arm in ["frozen", "OFF", "ON"] {
        let p = binary_for(arm);
        pins.insert(p.display().to_string(), hash(p));
    }
    let mut topology = Vec::new();
    for cpu in 0..32 {
        let dir = format!("/sys/devices/system/cpu/cpu{cpu}/topology");
        let core = fs::read_to_string(format!("{dir}/core_id"))
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let socket = fs::read_to_string(format!("{dir}/physical_package_id"))
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        assert_eq!(core, cpu % 16);
        assert_eq!(socket, 0);
        topology.push(json!({"cpu":cpu,"core":core,"socket":socket}));
    }
    save(
        r("host-before.json"),
        &json!({"topology":topology,"proc_status":fs::read_to_string("/proc/self/status").unwrap(),"proc_cpuinfo":fs::read_to_string("/proc/cpuinfo").unwrap()}),
    );
    let environment = json!({"RAYON_NUM_THREADS":"1","ZOPRO_CPU_TREE_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1"});
    let cs = contexts();
    let mut steps = Vec::new();
    for c in &cs {
        for arm in ["frozen", "OFF", "ON"] {
            steps.push(step(format!("preflight-{}-{arm}", c.name), argv(c, arm, 3)));
        }
    }
    steps.push(step(
        "preflight-check".into(),
        vec![r("campaign").display().to_string(), "preflight".into()],
    ));
    for b in 0..15 {
        let mut order: Vec<_> = (0..cs.len()).collect();
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
                    format!("latency-{}-b{b:02}-{arm}", c.name),
                    argv(c, arm, c.rounds),
                ));
            }
        }
    }
    for dir in ["stat", "perf"] {
        fs::create_dir(r(dir)).unwrap();
    }
    for p in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(p)).status().unwrap().success());
    }
    for profile in [false, true] {
        for c in cs.iter().filter(|c| profiled(c)) {
            for arm in ["frozen", "OFF", "ON"] {
                let name = format!(
                    "{}-{}-{arm}",
                    if profile { "cycles" } else { "stat" },
                    c.name
                );
                let mut env = environment.clone();
                env["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
                env["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
                let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
                for (k, v) in env.as_object().unwrap() {
                    a.push(format!("{k}={}", v.as_str().unwrap()));
                }
                a.push("perf".into());
                if profile {
                    a.extend([
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
                    a.extend([
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
                a.extend([
                    "--delay=-1".into(),
                    format!("--control=fifo:{R}/ctl,{R}/ack"),
                    "--".into(),
                ]);
                a.extend(argv(c, arm, if profile { profr(c) } else { profr(c) / 4 }));
                let mut s = step(name, a);
                s["environment"] = env;
                steps.push(s);
            }
        }
    }
    for c in cs.iter().filter(|c| profiled(c)) {
        for arm in ["frozen", "OFF", "ON"] {
            let name = format!("cycles-{}-{arm}", c.name);
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
                let mut a = vec![
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
                    a.extend([
                        "--show-mmap-events".into(),
                        "--show-task-events".into(),
                        "--show-lost-events".into(),
                    ]);
                }
                steps.push(step(
                    format!("{}-{name}", if meta { "metadata" } else { "decode" }),
                    a,
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
    assert_eq!(steps.len(), 1402);
    for f in [
        "design.md",
        "design-tls-addendum.md",
        "profile-thread-addendum.md",
        "driver.rs",
        "driver-tls.rs",
        "weighted.rs",
        "wrapper-oracle.rs",
        "perf_support.rs",
        "build.rs",
        "build-plan.json",
        "build-pins.json",
        "build-review.json",
        "consumer-tls-plan.json",
        "consumer-tls-pins.json",
        "consumer-review.json",
        "identities-tls.json",
        "tls-source-edits.json",
        "prepare-tls.rs",
        "validate-build.rs",
        "validate-consumer.rs",
        "prior-fixture-bindings.json",
        "source-audit.json",
        "preparation.md",
        "host-before.json",
        "capture-prepare.rs",
        "finish.rs",
        "make-campaign.rs",
        "campaign.rs",
        "campaign",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    pins.insert(X.into(), hash(X));
    save(
        r("declaration.json"),
        &json!({"blocks":15,"contexts":27,"unweighted_contexts":21,"weighted_contexts":6,"arms":["frozen","OFF","ON"],"steps":1402,"preflight_callers":81,"preflight_checks":1,"latency_processes":1215,"PMU_processes":15,"profile_processes":15,"offline_steps":75,"timing_checks":87,"calibration_interval":[0.97,1.03],"nonregression_upper":1.01,"large_gain_contexts":["group35973-w16","group35973-w32"],"gain_upper_strict":1.,"gain_wins_min":11,"interval_zero_based":[3,7,11],"allocation_counter":"worker TLS, not shared atomic","weighted_clock_scope":"prebuilt bases; public weighted_affine inside intended pool; projective output normalization/check outsideclock","profile_worker_sample_share_min":0.95,"profile_other_owned_threads_allowed":true,"profile_period":16000000,"profile_min_samples":1000,"profile_unknown_share_max":0.05,"production_changed":false,"new_proofs":0,"goal_status":"active/unmet0/8"}),
    );
    pins.insert(
        r("declaration.json").display().to_string(),
        hash(r("declaration.json")),
    );
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"environment":environment,"steps":steps}),
    );
    println!("Frozen1402commands/27contexts/15blocks/81preflights+check/1215latencies/15PMU+15profiles/75offline; zero previous newcandidate timing");
}
fn preflight() {
    let prior = read(r("prior-fixture-bindings.json"));
    for c in contexts() {
        let weighted = c.case.starts_with("weighted");
        let mut expected = None;
        let mut baseline = None;
        let mut forecast = None;
        for arm in ["frozen", "OFF", "ON"] {
            let v = result(&format!("preflight-{}-{arm}", c.name));
            assert_eq!(v["complete_projective_oracle"], true);
            assert_eq!(v["benchmark_Rayon_broadcast"], true);
            assert_eq!(v["rounds_per_worker"], 3);
            assert_eq!(v["workers"], c.workers);
            assert_eq!(v["weighted"], weighted);
            assert_eq!(v["priming_rounds_per_worker"], 24);
            assert_eq!(
                v["allocation_count_scope"],
                "sum of worker TLS deltas, excludes main/coordinator"
            );
            let mut total = 0;
            let mut tids = BTreeSet::new();
            let cap = &v["worker_rows"][0]["scratch_capacity_bytes"];
            let f = &v["worker_rows"][0]["source_forecast_bytes"];
            for w in v["worker_rows"].as_array().unwrap() {
                assert_eq!(w["pool_workers"], c.workers);
                assert!(tids.insert(w["tid"].as_u64().unwrap()));
                assert_eq!(
                    w["cpu"],
                    if c.workers == 1 {
                        2
                    } else {
                        w["id"].as_u64().unwrap()
                    }
                );
                assert_eq!(&w["scratch_capacity_bytes"], cap);
                assert_eq!(&w["source_forecast_bytes"], f);
                assert_eq!(w["output_sha256"], v["output_sha256"]);
                total += w["timed_worker_allocation_calls"].as_u64().unwrap();
            }
            assert_eq!(total, v["timed_allocations"].as_u64().unwrap());
            assert_eq!(tids.len(), c.workers);
            if v["fresh_buffers"] == false {
                assert_eq!(total, 0);
            }
            if let Some(e) = &expected {
                assert_eq!(&v["output_sha256"], e);
            } else {
                expected = Some(v["output_sha256"].clone());
            }
            if weighted {
                assert!(cap.is_null());
                assert!(f.as_u64().unwrap() > 0);
                if let Some(old) = &forecast {
                    assert_eq!(f, old);
                } else {
                    forecast = Some(f.clone());
                }
            } else {
                assert_eq!(
                    v["output_sha256"],
                    prior["unweighted_output_sha256"][c.case]
                );
                assert_eq!(cap.as_array().unwrap().len(), 5);
                if arm == "ON" {
                    assert_eq!(cap[2], 0);
                    assert!(cap[3].as_u64().unwrap() > 0);
                } else {
                    assert_eq!(cap[3], 0);
                    if let Some(old) = &baseline {
                        assert_eq!(cap, old);
                    } else {
                        baseline = Some(cap.clone());
                    }
                }
            }
        }
    }
    println!("PASS81preflights/all full projective outputs/prior unweighted hashes/workerCPU+pool/TLSallocation sums/capacity+weightedforecast invariants");
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
        json!({"sha256_before":before,"sha256_after":hash(data)})
    );
}
fn loss(data: &str) {
    let mut p = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = 0;
    let mut lost = Vec::new();
    for s in BufReader::new(p.stdout.take().unwrap()).lines() {
        let s = s.unwrap();
        lines += 1;
        if s.contains("PERF_RECORD_LOST") {
            lost.push(s);
        }
    }
    assert!(p.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":lines,"zero_lost":lost.is_empty(),"lost":lost})
    );
}
fn median(x: &[f64]) -> f64 {
    let mut x = x.to_vec();
    x.sort_by(f64::total_cmp);
    x[x.len() / 2]
}
fn interval(x: &[f64]) -> Value {
    assert_eq!(x.len(), 15);
    let mut s = x.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"low":s[3],"median":s[7],"high":s[11],"wins":x.iter().filter(|&&r|r<1.).count(),"ratios":x})
}
include!("finish.rs");
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("preflight") => preflight(),
        Some("finish") => finish(),
        Some("own") => own(&a[2]),
        Some("loss") => loss(&a[2]),
        _ => panic!("prepare/preflight/finish/own/loss"),
    }
}
