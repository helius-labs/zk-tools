#![recursion_limit = "256"]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-qualification175";
const PILOT: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
const ACCEPTED: &str =
    "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
const ACCEPTED_HASH: &str = "20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
const VS: [&str; 3] = ["accepted", "OFF", "ON"];
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
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn output(a: &[&str]) -> String {
    let o = Command::new(a[0]).args(&a[1..]).output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
include!("qualification.rs");
fn push(
    c: &Value,
    v: &str,
    b: usize,
    pre: bool,
    base: &Value,
    ids: &BTreeMap<&str, Value>,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
) -> Value {
    let name = if pre {
        format!("pre-{}-{v}", c["name"].as_str().unwrap())
    } else {
        format!("b{b}-{}-{v}", c["name"].as_str().unwrap())
    };
    let rounds = if pre {
        1
    } else if c["shape"] == "1x2" {
        151
    } else {
        31
    };
    let mut env = base.clone();
    env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
    env["ZOPRO_CPU_DEDUP"] = json!("1");

    let dir = r("proofs").join(&name);
    steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",c["cpu_affinity"],ids[v]["binary"],c["image"],c["request"],rounds.to_string(),dir],"environment":env,"timeout_seconds":900}));
    let j = json!({"name":name,"case":c,"variant":v,"block":b,"preflight":pre,"rounds":rounds,"directory":dir});
    jobs.push(j.clone());
    j
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn diagnostic(
    c: &Value,
    v: &str,
    panel: &str,
    base: &Value,
    ids: &BTreeMap<&str, Value>,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
) {
    let name = format!("{panel}-{}-{v}", c["name"].as_str().unwrap());
    let rounds = if panel == "cycles" { 251 } else { 31 };
    let dir = r("proofs").join(&name);
    let data = r("perf").join(format!("{name}.data"));
    fs::create_dir_all(data.parent().unwrap()).unwrap();
    let mut env = base.clone();
    env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
    env["ZOPRO_CPU_DEDUP"] = json!("1");
    env["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
    env["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
    let mut argv = vec!["sudo".into(), "-n".into(), "env".into()];
    for (k, val) in env.as_object().unwrap() {
        argv.push(format!("{k}={}", val.as_str().unwrap()));
    }
    argv.push("perf".into());
    if panel == "cycles" {
        argv.extend([
            "record".into(),
            "--no-buildid-cache".into(),
            "-e".into(),
            "cycles:u".into(),
            "-F".into(),
            "199".into(),
            "-m".into(),
            "512".into(),
            "-o".into(),
            data.display().to_string(),
        ]);
    } else {
        assert_eq!(panel, "arith");
        let events = "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}";
        argv.extend([
            "stat".into(),
            "--no-big-num".into(),
            "-x".into(),
            ";".into(),
            "-e".into(),
            events.into(),
        ]);
    }
    argv.extend([
        "--delay=-1".into(),
        format!(
            "--control=fifo:{},{}",
            r("ctl").display(),
            r("ack").display()
        ),
        "--".into(),
        "taskset".into(),
        "-c".into(),
        c["cpu_affinity"].as_str().unwrap().into(),
        ids[v]["binary"].as_str().unwrap().into(),
        c["image"].as_str().unwrap().into(),
        c["request"].as_str().unwrap().into(),
        rounds.to_string(),
        dir.display().to_string(),
    ]);
    steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":900}));
    jobs.push(json!({"name":name,"case":c,"variant":v,"preflight":false,"diagnostic":true,"panel":panel,"rounds":rounds,"directory":dir,"data":data}));
}
fn loss(data: &str) {
    let mut child = Command::new("sudo")
        .args(["-n", "perf", "script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = 0;
    let mut lost = vec![];
    for line in std::io::BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        lines += 1;
        if line.contains("PERF_RECORD_LOST") {
            lost.push(line);
        }
    }
    assert!(child.wait().unwrap().success());
    println!(
        "{}",
        json!({"raw_decoded_lines":lines,"lost_records":lost,"zero_lost":lost.is_empty()})
    );
}
fn analyze_diagnostics() -> Value {
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    for j in read(r("jobs.json"))
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["diagnostic"] == true)
    {
        let name = j["name"].as_str().unwrap();
        if j["panel"] != "cycles" {
            let text = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
            let mut values = BTreeMap::new();
            for l in text.lines() {
                let f: Vec<_> = l.split(';').map(str::trim).collect();
                if f.len() < 5 || f[2].is_empty() {
                    continue;
                }
                let count: f64 = f[0].parse().expect("PMU event unavailable");
                let runtime: f64 = f[3].parse().unwrap();
                let running: f64 = f[4].parse().unwrap();
                assert!(count.is_finite() && count >= 0. && runtime > 0. && running >= 99.9);
                assert!(values.insert(f[2].to_string(),json!({"count":count,"per_request":count/31.,"runtime":runtime,"running_percent":running})).is_none());
            }
            assert_eq!(values.len(), 4);
            counters.insert(name.to_string(),json!({"case":j["case"]["name"],"variant":j["variant"],"panel":j["panel"],"events":values}));
        } else {
            let lost = read(r(format!("runs/loss-{name}/stdout.log")));
            assert_eq!(lost["zero_lost"], true);
            let ev = fs::read_to_string(r(format!("runs/evlist-{name}/stdout.log"))).unwrap();
            assert!(ev.contains("cycles:u") && ev.contains("199") && ev.contains("freq: 1"));
            let mut pids = BTreeSet::new();
            let mut tids = BTreeSet::new();
            let mut symbols = BTreeMap::<String, (u64, u64)>::new();
            let mut samples = 0u64;
            let mut period = 0u64;
            let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            for line in text
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            {
                let f: Vec<_> = line.split_whitespace().collect();
                assert!(f.len() >= 4, "unexpected sample {line}");
                let (pid, tid) = f[0].split_once('/').expect("PID/TID column");
                pids.insert(pid.parse::<u32>().unwrap());
                tids.insert(tid.parse::<u32>().unwrap());
                let weight = f[1].parse::<u64>().unwrap();
                assert!(weight > 0);
                let entry = symbols.entry(f[3..].join(" ")).or_default();
                entry.0 += 1;
                entry.1 += weight;
                samples += 1;
                period += weight;
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            let mut symbols:Vec<_> = symbols.into_iter().map(|(name,(n,p))|json!({"symbol":name,"samples":n,"period":p,"cycle_share":p as f64/period as f64})).collect();
            symbols.sort_by(|a, b| {
                b["period"]
                    .as_u64()
                    .unwrap()
                    .cmp(&a["period"].as_u64().unwrap())
            });
            profiles.insert(name.to_string(),json!({"case":j["case"]["name"],"variant":j["variant"],"samples":samples,"period":period,"PIDs":pids,"TIDs":tids,"lost":lost,"symbols":symbols}));
        }
    }
    assert_eq!(counters.len(), 3);
    assert_eq!(profiles.len(), 3);
    json!({"counters":counters,"profiles":profiles,"all_events_running_at_least_99_9_percent":true,"profile_one_PID_each_zero_lost":true,"diagnostic_timings_used_for_latency_gate":false})
}

fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("finish")=>finish(),Some("loss")=>loss(&std::env::args().nth(2).unwrap()),_=>panic!("prepare|finish|loss")}}
