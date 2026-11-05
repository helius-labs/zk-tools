use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/finalize-drain-bundle176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const ACCEPTED: &str =
    "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
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
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut bytes = [0; 65536];
    loop {
        let n = file.read(&mut bytes).unwrap();
        if n == 0 {
            break;
        }
        h.update(&bytes[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(pins: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    pins.insert(p.as_ref().display().to_string(), hash(p));
}
fn step(name: impl Into<String>, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name.into(),"argv":argv,"timeout_seconds":900})
}
fn verify_step(j: &Value) -> Value {
    let c = &j["case"];
    let dir = Path::new(j["directory"].as_str().unwrap());
    step(
        format!("verify-{}", j["name"].as_str().unwrap()),
        vec![
            VERIFY.into(),
            c["image"].as_str().unwrap().into(),
            c["image_sha256"].as_str().unwrap().into(),
            dir.join("0.proof").display().to_string(),
            dir.join("0.public").display().to_string(),
            c["public_sha256"].as_str().unwrap().into(),
        ],
    )
}
fn add_job(
    c: &Value,
    v: &str,
    block: usize,
    panel: &str,
    base: &Value,
    ids: &Value,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
) {
    let name = if panel == "latency" {
        format!("b{block}-{}-{v}", c["name"].as_str().unwrap())
    } else {
        format!("{panel}-{}-{v}", c["name"].as_str().unwrap())
    };
    let rounds = match panel {
        "preflight" => 1,
        "latency" => {
            if c["shape"] == "1x2" {
                if c["workers"]==16 {301} else {151}
            } else {
                31
            }
        }
        "stat" => 31,
        "cycles" => if c["shape"]=="1x2"{251}else{101},
        _ => panic!("panel"),
    };
    let dir = r(format!("proofs/{name}"));
    let mut env = base.clone();
    env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
    env["ZOPRO_CPU_DEDUP"] = json!("1");env["ZOPRO_CPU_FINALIZE_DRAIN_BUNDLE"]=json!(if v=="ON"{"1"}else{"0"});
    let mut argv = Vec::new();
    let mut data = None;
    if panel == "stat" || panel == "cycles" {
        env["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
        env["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
        argv.extend(["sudo".into(), "-n".into(), "env".into()]);
        for (k, value) in env.as_object().unwrap() {
            argv.push(format!("{k}={}", value.as_str().unwrap()));
        }
        argv.push("perf".into());
        if panel == "stat" {
            let file = r(format!("stat/{name}.csv"));
            argv.extend([
                "stat".into(),
                "--no-big-num".into(),
                "-x".into(),
                ";".into(),
                "-e".into(),
                "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                "-o".into(),
                file.display().to_string(),
            ]);
            data = Some(file);
        } else {
            let file = r(format!("perf/{name}.data"));
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
                file.display().to_string(),
            ]);
            data = Some(file);
        }
        argv.extend([
            "--delay=-1".into(),
            format!(
                "--control=fifo:{},{}",
                r("ctl").display(),
                r("ack").display()
            ),
            "--".into(),
        ]);
    }
    argv.extend([
        "taskset".into(),
        "-c".into(),
        c["cpu_affinity"].as_str().unwrap().into(),
        ids[v]["binary"].as_str().unwrap().into(),
        c["image"].as_str().unwrap().into(),
        c["request"].as_str().unwrap().into(),
        rounds.to_string(),
        dir.display().to_string(),
    ]);
    let mut command = step(&name, argv);
    if panel == "latency" || panel == "preflight" {
        command["environment"] = env;
    }
    steps.push(command);
    jobs.push(json!({"name":name,"case":c,"variant":v,"block":block,"panel":panel,"rounds":rounds,"directory":dir,"data":data,"binary":ids[v]["binary"]}));
}
include!("capture-prepare.rs");
fn loss(data: &str) {
    let mut child = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut records = Vec::new();
    let mut lines = 0;
    for line in BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        lines += 1;
        if line.contains("PERF_RECORD_LOST") {
            records.push(line);
        }
    }
    assert!(child.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":lines,"zero_lost":records.is_empty(),"records":records})
    );
}
include!("sampling.rs");
include!("counter.rs");
include!("metadata.rs");
include!("analysis.rs");
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("preflight-routes") => preflight_routes(),
        Some("loss") => loss(&args[2]),
        _ => panic!("mode"),
    }
}

