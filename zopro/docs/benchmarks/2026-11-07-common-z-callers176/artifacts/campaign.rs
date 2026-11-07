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
const R: &str = "/home/ubuntu/src/zopro-perf-work/common-z-callers176";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-callers176";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
const ARMS: [&str; 4] = ["frozen", "fresh", "OFF", "ON"];
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
#[derive(Clone)]
struct C {
    case: &'static str,
    w: usize,
    n: usize,
}
impl C {
    fn name(&self) -> String {
        format!("{}-w{}", self.case, self.w)
    }
}
fn contexts() -> Vec<C> {
    let mut out = Vec::new();
    for (case, n) in [
        ("tile257", 8192),
        ("tile384", 8192),
        ("tile511", 8192),
        ("tile2048", 2048),
        ("group35973", 128),
        ("groups31x256", 512),
        ("tails", 8192),
        ("small16", 65536),
        ("groups3x4", 65536),
        ("small-mixed", 65536),
    ] {
        for w in [1, 16, 32] {
            out.push(C { case, w, n });
        }
    }
    out
}
fn binary(arm: &str) -> PathBuf {
    r(format!(
        "caller-{}",
        if ["ON", "OFF"].contains(&arm) {
            "candidate"
        } else {
            arm
        }
    ))
}
fn env(arm: &str) -> Value {
    let mut env = read(format!("{OLD}/build-runs/core-frozen/command.json"))["environment"].clone();
    for k in ["CARGO_PKG_NAME", "CARGO_PKG_VERSION", "CARGO_MANIFEST_DIR"] {
        env.as_object_mut().unwrap().remove(k);
    }
    env["ZOPRO_CPU_COMMON_Z_FINISH"] = json!(if arm == "ON" { "1" } else { "0" });
    env
}
fn argv(c: &C, arm: &str, n: usize) -> Vec<String> {
    vec![
        "taskset".into(),
        "-c".into(),
        match c.w {
            1 => "2",
            16 => "0-15",
            32 => "0-31",
            _ => unreachable!(),
        }
        .into(),
        binary(arm).display().to_string(),
        c.case.into(),
        c.w.to_string(),
        n.to_string(),
        arm.into(),
    ]
}
fn step(name: String, argv: Vec<String>, environment: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":900})
}
fn result(name: &str) -> Value {
    let s = fs::read_to_string(r(format!("capture/{name}/stdout.log"))).unwrap();
    let mut values = s.lines().filter_map(|l| l.strip_prefix("RESULT "));
    let v = serde_json::from_str(values.next().unwrap()).unwrap();
    assert!(values.next().is_none());
    v
}
fn receipt(base: &Path, step: &Value) -> Value {
    let e = read(base.join("exit.json"));
    assert_eq!(e["success"], true, "{}", base.display());
    assert_eq!(e["returncode"], 0);
    let cmd = read(base.join("command.json"));
    assert_eq!(
        &cmd["argv"].as_array().unwrap()[4..],
        step["argv"].as_array().unwrap().as_slice()
    );
    for stream in ["stdout", "stderr"] {
        assert_eq!(
            hash(base.join(format!("{stream}.log"))),
            e[format!("{stream}_sha256")].as_str().unwrap()
        );
    }
    json!({"name":step["name"],"command":cmd,"exit":e})
}
fn prepare() {
    assert!(!r("capture-plan.json").exists());
    assert_eq!(
        read(r("continuation-runs/result.json"))["status"],
        "complete"
    );
    let mut pins = read(r("continuation-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut build_receipts = Vec::new();
    for (plan, out) in [
        ("build-plan.json", "build-runs"),
        ("continuation-plan.json", "continuation-runs"),
    ] {
        for (i, s) in read(r(plan))["steps"]
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
        {
            if plan == "build-plan.json" && i >= 2 {
                continue;
            }
            build_receipts.push(receipt(
                &r(format!("{out}/{}", s["name"].as_str().unwrap())),
                s,
            ));
        }
    }
    let tests = fs::read_to_string(r("continuation-runs/changed-core-tests/stdout.log")).unwrap();
    assert!(tests.contains("test result: ok."));
    assert!(
        tests.contains("segmented_frontier_preserves_outputs_and_transactional_fallback ... ok")
    );
    assert!(
        tests.contains("production_grouping_and_indexed_paths_preserve_complete_results ... ok")
    );
    let mut ids = BTreeMap::new();
    for arm in ["frozen", "fresh", "candidate"] {
        let b = r(format!("caller-{arm}"));
        let core = if arm == "frozen" {
            PathBuf::from(format!("{OLD}/libs/frozen/libzopro_core.rlib"))
        } else {
            r(format!("libs/{arm}/libzopro_core.rlib"))
        };
        pins[b.display().to_string()] = json!(hash(&b));
        pins[core.display().to_string()] = json!(hash(&core));
        ids.insert(
            arm,
            json!({"binary":b,"binary_sha256":hash(&b),"core":core,"core_sha256":hash(&core)}),
        );
        let cmd = read(r(format!("continuation-runs/consumer-{arm}/command.json")));
        let a = cmd["argv"].as_array().unwrap();
        for flag in ["-O", "target-cpu=native", "lto=fat", "codegen-units=1"] {
            assert!(a.contains(&json!(flag)));
        }
    }
    save(r("identities.json"), &json!(ids));
    save(
        r("build-review.json"),
        &json!({"passed":true,"successful_commands":build_receipts,"failed_initial_test_compile_preserved":true,"runtime_libraries_rebuilt_for_test_repair":false,"changed_core_test_stdout_sha256":hash(r("continuation-runs/changed-core-tests/stdout.log"))}),
    );
    let topology=(0..32).map(|cpu|json!({"cpu":cpu,"siblings":fs::read_to_string(format!("/sys/devices/system/cpu/cpu{cpu}/topology/thread_siblings_list")).unwrap()})).collect::<Vec<_>>();
    save(
        r("host-before.json"),
        &json!({"topology":topology,"proc_status":fs::read_to_string("/proc/self/status").unwrap(),"cpuinfo":fs::read_to_string("/proc/cpuinfo").unwrap()}),
    );
    let cs = contexts();
    let mut steps = Vec::new();
    for c in &cs {
        for arm in ARMS {
            steps.push(step(
                format!("preflight-{}-{arm}", c.name()),
                argv(c, arm, 8),
                env(arm),
            ));
        }
    }
    steps.push(step(
        "preflight-review".into(),
        vec![
            r("campaign").display().to_string(),
            "preflight-review".into(),
        ],
        json!({}),
    ));
    for block in 0..15 {
        let mut order = (0..cs.len()).collect::<Vec<_>>();
        order.rotate_left(block % cs.len());
        if block % 2 == 1 {
            order.reverse();
        }
        for i in order {
            let c = &cs[i];
            let mut arms = ARMS;
            arms.rotate_left((block + i) % 4);
            if block % 2 == 1 {
                arms.reverse();
            }
            for arm in arms {
                steps.push(step(
                    format!("latency-{}-b{block:02}-{arm}", c.name()),
                    argv(c, arm, c.n),
                    env(arm),
                ));
            }
        }
    }
    for dir in ["perf", "stat"] {
        fs::create_dir(r(dir)).unwrap();
    }
    for p in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(p)).status().unwrap().success());
    }
    for profile in [false, true] {
        for c in cs
            .iter()
            .filter(|c| c.w == 32 && ["tile511", "groups3x4"].contains(&c.case))
        {
            for arm in ["frozen", "ON"] {
                let name = format!(
                    "{}-{}-{arm}",
                    if profile { "cycles" } else { "stat" },
                    c.name()
                );
                let mut e = env(arm);
                e["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
                e["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
                let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
                for (k, v) in e.as_object().unwrap() {
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
                        "4000000".into(),
                        "-m".into(),
                        "512".into(),
                        "-o".into(),
                        r(format!("perf/{name}.data")).display().to_string(),
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
                        r(format!("stat/{name}.csv")).display().to_string(),
                    ]);
                }
                a.extend([
                    "--delay=-1".into(),
                    format!("--control=fifo:{R}/ctl,{R}/ack"),
                    "--".into(),
                ]);
                let n = if c.case == "tile511" { 32768 } else { 262144 };
                a.extend(argv(c, arm, if profile { n } else { n / 4 }));
                steps.push(step(name, a, e));
            }
        }
    }
    for c in cs
        .iter()
        .filter(|c| c.w == 32 && ["tile511", "groups3x4"].contains(&c.case))
    {
        for arm in ["frozen", "ON"] {
            let name = format!("cycles-{}-{arm}", c.name());
            let data = r(format!("perf/{name}.data")).display().to_string();
            steps.push(step(
                format!("own-{name}"),
                vec![
                    "sudo".into(),
                    "-n".into(),
                    r("campaign").display().to_string(),
                    "own".into(),
                    data.clone(),
                ],
                json!({}),
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
                    json!({}),
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
                json!({}),
            ));
            steps.push(step(
                format!("loss-{name}"),
                vec![r("campaign").display().to_string(), "loss".into(), data],
                json!({}),
            ));
        }
    }
    assert_eq!(steps.len(), 1949);
    for p in [
        "campaign.rs",
        "campaign",
        "analysis.rs",
        "design.md",
        "driver.rs",
        "perf_support.rs",
        "source-audit-after-test-repair.json",
        "build-review.json",
        "identities.json",
        "host-before.json",
        "continuation-plan.json",
        "continuation-pins.json",
    ] {
        let p = r(p);
        pins[p.display().to_string()] = json!(hash(&p));
    }
    pins[X] = json!(hash(X));
    let prior = format!("{OLD}/prior-fixture-bindings.json");
    pins[&prior] = json!(hash(&prior));
    save(r("capture-pins.json"), &pins);
    save(
        r("capture-plan.json"),
        &json!({"version":1,"source_pins":r("capture-pins.json"),"steps":steps}),
    );
    save(
        r("declaration.json"),
        &json!({"contexts":30,"arms":ARMS,"blocks":15,"preflights":120,"latency_processes":1800,"PMU_processes":4,"profile_processes":4,"offline_commands":20,"preflight_review_commands":1,"commands":1949,"rank_indices":[3,7,11],"calibration_factor":1.03,"nonregression_upper":1.03,"heap_capacity_factor":1.05,"gain_upper_strict":1.,"gain_wins_min":11,"gain_contexts":"equal-log all10cases32workers against all3controls","gain_floor":0,"profile_period":4000000,"profile_min_samples":1000,"profile_unknown_share_max":0.05,"profile_worker_share_min":0.95,"profile_zero_LOST_required":true,"no_reroll_or_extension":true,"new_proofs":0}),
    );
    println!("Prepared1949commands:120preflights+review,1800latencies,8PMU/profile,20offline; no prior candidate timing");
}
fn validate_caller(v: &Value) {
    assert_eq!(v["complete_projective_oracle"], true);
    assert_eq!(v["SDK_requests"], 0);
    assert_eq!(v["proofs"], 0);
    assert_eq!(v["priming_rounds_per_worker"], 24);
    let w = v["workers"].as_u64().unwrap();
    let rows = v["worker_rows"].as_array().unwrap();
    assert_eq!(rows.len(), w as usize);
    let mut tids = BTreeSet::new();
    let mut alloc = 0;
    for row in rows {
        let id = row["id"].as_u64().unwrap();
        assert_eq!(row["cpu"], if w == 1 { 2 } else { id });
        assert_eq!(row["pool_workers"], w);
        assert!(tids.insert(row["tid"].as_u64().unwrap()));
        assert_eq!(row["output_sha256"], v["output_sha256"]);
        alloc += row["timed_worker_allocation_calls"].as_u64().unwrap();
        assert_eq!(
            row["scratch_capacity_bytes"],
            rows[0]["scratch_capacity_bytes"]
        );
    }
    assert_eq!(v["timed_allocations"], alloc);
    if v["fresh_buffers"] == false {
        assert_eq!(alloc, 0);
    }
    assert_eq!(
        v["allocation_count_scope"],
        "sum of worker TLS deltas, excludes main/coordinator"
    );
}
fn preflight_review() {
    let old = read(format!("{OLD}/prior-fixture-bindings.json"));
    let mut n = 0;
    for c in contexts() {
        let reference = result(&format!("preflight-{}-frozen", c.name()));
        for arm in ARMS {
            let v = result(&format!("preflight-{}-{arm}", c.name()));
            validate_caller(&v);
            assert_eq!(v["output_sha256"], reference["output_sha256"]);
            if let Some(h) = old["unweighted_output_sha256"].get(c.case) {
                assert_eq!(&v["output_sha256"], h);
            }
            n += 1;
        }
    }
    println!("PASS {n} complete preflights; inherited seven fixture hashes; workerCPU/TID/output/TLS/capacity checks");
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
    let after = hash(data);
    assert_eq!(before, after);
    println!("{}", json!({"sha256_before":before,"sha256_after":after}));
}
fn loss(data: &str) {
    let mut p = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut n = 0;
    let mut lost = Vec::new();
    for l in BufReader::new(p.stdout.take().unwrap()).lines() {
        let l = l.unwrap();
        n += 1;
        if l.contains("PERF_RECORD_LOST") {
            lost.push(l);
        }
    }
    assert!(p.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":n,"zero_lost":lost.is_empty(),"lost":lost})
    );
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn interval(v: &[f64]) -> Value {
    assert_eq!(v.len(), 15);
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    json!({"low":x[3],"median":x[7],"high":x[11],"wins":v.iter().filter(|&&r|r<1.).count(),"ratios":v})
}
include!("analysis.rs");
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("preflight-review") => preflight_review(),
        Some("finish") => finish(),
        Some("own") => own(&a[2]),
        Some("loss") => loss(&a[2]),
        _ => panic!("prepare | preflight-review | finish | own | loss"),
    }
}
