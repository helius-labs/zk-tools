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
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-callers176";
const N: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
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
    let mut all = Vec::new();
    for (case, rounds, main) in [
        ("tile257", 1024, false),
        ("tile384", 1024, true),
        ("tile511", 1024, true),
        ("tile2048", 256, true),
        ("group35973", 16, true),
        ("groups31x256", 64, false),
        ("tails", 1024, false),
    ] {
        for workers in [1, 16, 32] {
            all.push(C {
                name: format!("{case}-w{workers}"),
                case,
                workers,
                rounds,
                main,
            });
        }
    }
    all
}
fn profiled(c: &C) -> bool {
    ["tile384", "group35973"].contains(&c.case) && [1, 32].contains(&c.workers)
}
fn profr(c: &C) -> usize {
    if c.case == "tile384" {
        1048576
    } else {
        16384
    }
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
        r(if arm == "OFF" {
            "caller-OFF"
        } else {
            "caller-ON"
        })
        .display()
        .to_string(),
        c.case.into(),
        if arm == "ON" { "ternary" } else { "binary" }.into(),
        c.workers.to_string(),
        rounds.to_string(),
    ]
}
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600})
}
fn result(n: &str) -> Value {
    let text = fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap();
    let mut lines = text.lines().filter_map(|s| s.strip_prefix("RESULT "));
    let v = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert!(lines.next().is_none());
    v
}
fn digest_coordinates(v: &Value) -> String {
    let mut h = Sha256::new();
    for p in v.as_array().unwrap() {
        for c in p.as_array().unwrap() {
            for w in c.as_array().unwrap() {
                h.update(w.as_u64().unwrap().to_le_bytes());
            }
        }
    }
    format!("{:x}", h.finalize())
}
fn prepare() {
    assert!(!r("plan.json").exists());
    let build = read(r("build-plan.json"));
    for s in build["steps"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        let base = r(format!("build-runs/{name}"));
        assert_eq!(read(base.join("exit.json"))["returncode"], 0);
        let cmd = read(base.join("command.json"));
        assert_eq!(
            &cmd["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap().as_slice()
        );
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let original = read(format!("{N}/review.json"));
    assert_eq!(
        hash(format!("{N}/libs/libzopro_core.rlib")),
        original["binaries_sha256"]["libs/libzopro_core.rlib"]
    );
    let mut identities = BTreeMap::new();
    for arm in ["frozen", "OFF", "ON"] {
        let binary = r(if arm == "OFF" {
            "caller-OFF"
        } else {
            "caller-ON"
        });
        let core = if arm == "OFF" {
            r("OFF-libs/libzopro_core.rlib")
        } else {
            Path::new(N).join("libs/libzopro_core.rlib")
        };
        identities.insert(arm,json!({"binary":binary,"binary_sha256":hash(&binary),"core":core,"core_sha256":hash(&core),"route":if arm=="ON"{"ternary"}else{"binary"}}));
        pins.insert(binary.display().to_string(), hash(binary));
        pins.insert(core.display().to_string(), hash(core));
    }
    assert_ne!(
        identities["OFF"]["binary_sha256"],
        identities["ON"]["binary_sha256"]
    );
    assert_eq!(
        identities["frozen"]["binary_sha256"],
        identities["ON"]["binary_sha256"]
    );
    save(r("identities.json"), &json!(identities));
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
        &json!({"topology":topology,"proc_status":fs::read_to_string("/proc/self/status").unwrap(),"proc_cpuinfo":fs::read_to_string("/proc/cpuinfo").unwrap(),"uname":Command::new("uname").arg("-a").output().map(|o|String::from_utf8(o.stdout).unwrap()).unwrap()}),
    );
    let environment = json!({"RAYON_NUM_THREADS":"1","ZOPRO_CPU_TREE_VECTOR_INVERSE":"1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1"});
    let cs = contexts();
    let mut steps = Vec::new();
    for c in &cs {
        for arm in ["frozen", "OFF", "ON"] {
            steps.push(step(format!("preflight-{}-{arm}", c.name), argv(c, arm, 3)));
        }
    }
    for c in cs.iter().filter(|c| c.workers == 1) {
        for route in ["binary", "ternary"] {
            steps.push(step(
                format!("legacy-{}-{route}", c.case),
                vec![
                    "taskset".into(),
                    "-c".into(),
                    "2".into(),
                    format!("{N}/caller"),
                    c.case.into(),
                    route.into(),
                    "3".into(),
                ],
            ));
        }
    }
    steps.push(step(
        "preflight-check".into(),
        vec![r("campaign").display().to_string(), "preflight".into()],
    ));
    for b in 0..15 {
        let mut order: Vec<usize> = (0..cs.len()).collect();
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
    for d in ["stat", "perf"] {
        fs::create_dir(r(d)).unwrap();
    }
    for f in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());
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
    assert_eq!(steps.len(), 1107);
    for f in [
        "design.md",
        "driver.rs",
        "perf_support.rs",
        "build.rs",
        "build-plan.json",
        "build-pins.json",
        "OFF-source-manifest.json",
        "identities.json",
        "host-before.json",
        "campaign.rs",
        "finish.rs",
        "campaign",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    pins.insert(X.into(), hash(X));
    save(
        r("declaration.json"),
        &json!({"blocks":15,"contexts":21,"main_contexts":12,"arms":["frozen","OFF","ON"],"steps":1107,"new_preflights":63,"legacy_preflights":14,"latency_processes":945,"PMU_processes":12,"profile_processes":12,"offline_steps":60,"calibration_interval":[0.97,1.03],"nonregression_upper":1.01,"gain_upper_strict":1.0,"gain_wins_min":11,"interval_order_zero_based":[3,7,11],"main_cases":["tile384","tile511","tile2048","group35973"],"whole_goal":"all-eight CPU10x active/unmet0/8","new_proofs":0,"production_changed":false}),
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
    println!("Frozen1107commands/15blocks/21contexts/3arms/77preflights/945latencies/12PMU/12profiles/60offline; NO capture yet");
}
fn preflight() {
    for c in contexts() {
        let mut expected = None;
        let mut binary = None;
        for arm in ["frozen", "OFF", "ON"] {
            let v = result(&format!("preflight-{}-{arm}", c.name));
            assert_eq!(v["complete_projective_oracle"], true);
            assert_eq!(v["rounds_per_worker"], 3);
            assert_eq!(v["workers"], c.workers);
            assert_eq!(v["priming_rounds_per_worker"], 24);
            if v["fresh_buffers"] == false {
                assert_eq!(v["timed_allocations"], 0);
            }
            if let Some(e) = &expected {
                assert_eq!(&v["output_sha256"], e);
            } else {
                expected = Some(v["output_sha256"].clone());
            }
            let cap = &v["worker_rows"][0]["scratch_capacity_bytes"];
            for w in v["worker_rows"].as_array().unwrap() {
                assert_eq!(&w["scratch_capacity_bytes"], cap);
                let id = w["id"].as_u64().unwrap();
                assert_eq!(w["cpu"], if c.workers == 1 { 2 } else { id });
            }
            if arm == "frozen" {
                binary = Some(cap.clone());
            }
            if arm == "OFF" {
                assert_eq!(Some(cap), binary.as_ref());
            }
            if c.workers == 1 && arm != "OFF" {
                let legacy = result(&format!(
                    "legacy-{}-{}",
                    c.case,
                    if arm == "ON" { "ternary" } else { "binary" }
                ));
                assert_eq!(
                    digest_coordinates(&legacy["coordinates"]),
                    v["output_sha256"]
                );
                assert_eq!(legacy["scratch_common_bytes"], cap[0]);
                assert_eq!(legacy["scratch_binary_bytes"], cap[1]);
                assert_eq!(legacy["scratch_ternary_bytes"], cap[2]);
                assert_eq!(legacy["scratch_fixed_bytes"], cap[3]);
                assert_eq!(
                    legacy["allocation_calls_per_round"],
                    v["allocation_calls_per_worker_round"]
                );
            }
        }
    }
    println!("PASS63threaded+14legacy outputs/capacities/allocation identities, workerCPU, full projective oracle; no gain claim");
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
