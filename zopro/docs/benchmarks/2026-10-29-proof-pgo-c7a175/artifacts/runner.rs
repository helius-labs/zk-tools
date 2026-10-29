//! Frozen C7a matrix. This executable never builds or selects candidates.
#![allow(dead_code)]
#[path = "support/config.rs"]
mod config;
#[path = "support/results.rs"]
mod results;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const PERF: &str = "/usr/lib/linux-tools/6.8.0-147-generic/perf";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
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
fn run(argv: &[String], p: &Path) {
    let f = fs::File::create(p).unwrap();
    let status = Command::new(&argv[0])
        .args(&argv[1..])
        .stdout(Stdio::from(f.try_clone().unwrap()))
        .stderr(Stdio::from(f))
        .status()
        .unwrap();
    assert!(status.success(), "command failed; see {}", p.display());
}
fn state() -> Value {
    let mut out = BTreeMap::new();
    for p in [
        "/proc/loadavg",
        "/proc/swaps",
        "/sys/fs/cgroup/cpu.stat",
        "/sys/fs/cgroup/cpu.max",
        "/sys/fs/cgroup/memory.max",
        "/sys/fs/cgroup/memory.current",
        "/sys/fs/cgroup/memory.peak",
        "/sys/fs/cgroup/memory.events",
        "/sys/fs/cgroup/memory.swap.max",
        "/sys/fs/cgroup/memory.swap.current",
        "/sys/fs/cgroup/memory.swap.events",
    ] {
        if let Ok(s) = fs::read_to_string(p) {
            out.insert(p, s);
        }
    }
    json!(out)
}
fn job(
    dir: &Path,
    c: &Value,
    v: &str,
    w: usize,
    rounds: usize,
    phase: &str,
    name: &str,
    block: usize,
    base: &Value,
) -> (Value, Value) {
    let out = dir.join("runs").join(name);
    fs::create_dir_all(&out).unwrap();
    let root = out.join("proofs");
    let proof = root.join(Path::new(c["image"].as_str().unwrap()).file_stem().unwrap());
    let mut env = base.as_object().unwrap().clone();
    env.insert("RAYON_NUM_THREADS".into(), json!(w.to_string()));
    let engine = match v {
        "plain" => "previous",
        "PGO" => "current",
        "aeglos" => "aeglos",
        _ => panic!("variant"),
    };
    env.insert("ZOPRO_CR_ENGINE".into(), json!(engine));
    let mut command = vec![];
    if phase == "profile" {
        for f in ["ctl", "ack"] {
            assert!(Command::new("mkfifo")
                .args(["-m", "600"])
                .arg(out.join(f))
                .status()
                .unwrap()
                .success());
        }
        env.insert("ZOPRO_BENCH_PERF_CTL".into(), json!(out.join("ctl")));
        env.insert("ZOPRO_BENCH_PERF_ACK".into(), json!(out.join("ack")));
        command.extend([
            json!(PERF),
            json!("record"),
            json!("--no-buildid-cache"),
            json!("--clockid"),
            json!("mono"),
            json!("-e"),
            json!("cycles:u"),
            json!("-F"),
            json!("199"),
            json!("-m"),
            json!("512"),
            json!("--delay=-1"),
            json!(format!(
                "--control=fifo:{},{}",
                out.join("ctl").display(),
                out.join("ack").display()
            )),
            json!("-o"),
            json!(out.join("perf.data")),
            json!("--"),
        ]);
    }
    command.push(json!("env"));
    for (k, v) in env {
        assert!(k.starts_with("MALLOC_") || k.starts_with("RAYON_") || k.starts_with("ZOPRO_"));
        command.push(json!(format!("{k}={}", v.as_str().unwrap())));
    }
    command.extend([
        json!("taskset"),
        json!("-c"),
        json!(format!("0-{}", w - 1)),
        json!("/usr/local/bin/cr-adapter"),
        c["image"].clone(),
        c["request"].clone(),
        json!(rounds.to_string()),
        json!(proof),
    ]);
    let step = json!({"kind":"command","name":format!("measure-{name}"),"argv":command,"timeout_seconds":900});
    let j = json!({"name":name,"case":c,"variant":v,"adapter_engine":engine,"workers":w,"rounds":rounds,"phase":phase,"block":block,"directory":proof,"proof_root":root,"out":out});
    (step, j)
}
fn verify(j: &Value) -> Value {
    json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":["/usr/local/bin/gnark-verify","/keys",j["proof_root"],"/manifest/zopro-keys.lock"],"timeout_seconds":120})
}
fn audit(
    dir: &Path,
    jobs: &[Value],
    seen: &mut BTreeSet<String>,
    inventory: &mut BTreeMap<String, String>,
    categories: &[config::Category],
) -> Vec<Value> {
    let mut rows = vec![];
    for j in jobs {
        let name = j["name"].as_str().unwrap();
        let p = Path::new(j["directory"].as_str().unwrap()).join("0.proof");
        assert_eq!(fs::metadata(&p).unwrap().len(), 164);
        let h = hash(&p);
        assert!(seen.insert(h.clone()));
        let public = p.with_extension("public");
        assert_eq!(hash(&public), j["case"]["public_sha256"].as_str().unwrap());
        inventory.insert(p.display().to_string(), h.clone());
        inventory.insert(public.display().to_string(), hash(public));
        assert_eq!(
            read(dir.join(format!("workflow/verify-{name}/exit.json")))["success"],
            true
        );
        let sdk = results::sdk_output(
            &fs::read_to_string(dir.join(format!("workflow/measure-{name}/stdout.log"))).unwrap(),
            j["rounds"].as_u64().unwrap() as usize,
        )
        .unwrap();
        assert_eq!(sdk.summary["workers"], j["workers"]);
        let profile = if j["phase"] == "profile" {
            let p = results::perf_samples(
                &fs::read_to_string(dir.join(format!("workflow/symbols-{name}/stdout.log")))
                    .unwrap(),
                categories,
            )
            .unwrap();
            assert_eq!(p.lost_records, 0);
            assert_eq!(p.sampled_pids.len(), 1);
            serde_json::to_value(p).unwrap()
        } else {
            Value::Null
        };
        rows.push(json!({"job":j,"sdk":sdk,"profile":profile,"proof_sha256":h}));
    }
    rows
}
fn main() {
    let phase = std::env::args().nth(1).unwrap();
    if phase == "loss" {
        let path = std::env::args().nth(2).unwrap();
        let output = Command::new(PERF)
            .args(["script", "-D", "-i", &path])
            .output()
            .unwrap();
        assert!(output.status.success());
        let count = String::from_utf8_lossy(&output.stdout)
            .lines()
            .chain(String::from_utf8_lossy(&output.stderr).lines())
            .filter(|l| l.contains("PERF_RECORD_LOST"))
            .count();
        println!(
            "{}",
            json!({"zero_lost":count==0,"lost_records":count,"path":path})
        );
        assert_eq!(count, 0);
        return;
    }
    assert!(phase == "preflight" || phase == "measure");
    let dir = Path::new("/output").join(&phase);
    fs::create_dir(&dir).unwrap();
    let m = read("/cr/manifest.json");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(m["container_pins"].clone()).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h, "{p}");
    }
    pins.insert("/cr/manifest.json".into(), hash("/cr/manifest.json"));
    let mut seen: BTreeSet<String> =
        serde_json::from_value(m["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 19261);
    let cases = m["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 9);
    let categories: Vec<config::Category> =
        serde_json::from_value(m["symbol_categories"].clone()).unwrap();
    let mut env = m["environment"].clone();
    env["ZOPRO_BASIS_KEY_DIR"] = json!("/cr-basis");
    env["ZOPRO_CPU_LAGRANGE_DIR"] = json!("/output/lagrange-cache");
    env["ZOPRO_CPU_DEDUP"] = json!("1");
    env["ZOPRO_CR_AFTER_LATENCY"] = json!(dir.join("after-latency.json"));
    save(dir.join("before.json"), &state());
    for (name, args) in [
        ("lscpu.txt", vec![]),
        ("topology.csv", vec!["-p=CPU,CORE,SOCKET,ONLINE"]),
    ] {
        let out = Command::new("lscpu").args(args).output().unwrap();
        assert!(out.status.success());
        fs::write(dir.join(name), out.stdout).unwrap();
    }
    let variants = ["plain", "PGO", "aeglos"];
    let mut jobs = vec![];
    let mut steps = vec![];
    if phase == "preflight" {
        for c in cases {
            for v in variants {
                let name = format!("{}-w4-{v}", c["shape"].as_str().unwrap());
                let (s, j) = job(&dir, c, v, 4, 1, "preflight", &name, 0, &env);
                steps.push(s);
                steps.push(verify(&j));
                jobs.push(j);
            }
        }
        assert_eq!(jobs.len(), 27);
        assert_eq!(steps.len(), 54);
    } else {
        let local = read("/cr/preflight-complete.json");
        assert_eq!(local["status"], "complete");
        assert_eq!(local["sdk_requests"], 108);
        assert_eq!(local["distinct_gnark_exports"], 27);
        for h in local["new_proof_sha256"].as_array().unwrap() {
            assert!(seen.insert(h.as_str().unwrap().into()));
        }
        assert_eq!(seen.len(), 19288);
        pins.insert(
            "/cr/preflight-complete.json".into(),
            hash("/cr/preflight-complete.json"),
        );
        let quota = fs::read_to_string("/sys/fs/cgroup/cpu.max").unwrap();
        let q: Vec<_> = quota.split_whitespace().collect();
        assert_eq!(
            q[0].parse::<u64>().unwrap(),
            96 * q[1].parse::<u64>().unwrap()
        );
        assert_eq!(
            fs::read_to_string("/sys/fs/cgroup/memory.max")
                .unwrap()
                .trim(),
            (30u64 << 30).to_string()
        );
        assert_eq!(
            fs::read_to_string("/sys/fs/cgroup/memory.swap.max")
                .unwrap()
                .trim(),
            "0"
        );
        let topology = fs::read_to_string(dir.join("topology.csv")).unwrap();
        let mut cores = BTreeSet::new();
        let mut cpus = BTreeSet::new();
        for l in topology.lines().filter(|l| !l.starts_with('#')) {
            let f: Vec<_> = l.split(',').collect();
            assert_eq!(f[3], "Y");
            cpus.insert(f[0].parse::<usize>().unwrap());
            assert!(cores.insert((f[1], f[2])));
        }
        assert_eq!(cpus, (0..96).collect());
        assert_eq!(cores.len(), 96);
        run(
            &[
                PERF.into(),
                "stat".into(),
                "-e".into(),
                "cycles:u".into(),
                "--".into(),
                "/bin/true".into(),
            ],
            &dir.join("perf-availability.log"),
        );
        let mut contexts = vec![];
        for c in cases {
            for w in [32usize, 64, 96] {
                contexts.push((c.clone(), w));
            }
        }
        for b in 0..15 {
            let mut cs = contexts.clone();
            let len = cs.len();
            cs.rotate_left(b % len);
            if b % 2 == 1 {
                cs.reverse();
            }
            for (ci, (c, w)) in cs.iter().enumerate() {
                let mut vs = variants;
                vs.rotate_left((ci + b) % 3);
                if b % 2 == 1 {
                    vs.reverse();
                }
                for v in vs {
                    let name = format!("b{b}-{}-w{w}-{v}", c["shape"].as_str().unwrap());
                    let (s, j) = job(
                        &dir,
                        c,
                        v,
                        *w,
                        if c["shape"] == "1x2" { 101 } else { 17 },
                        "latency",
                        &name,
                        b,
                        &env,
                    );
                    steps.push(s);
                    jobs.push(j);
                }
            }
        }
        assert_eq!(jobs.len(), 1215);
        for j in &jobs {
            steps.push(verify(j));
        }
        let primary = cases.iter().find(|c| c["shape"] == "1x2").unwrap();
        for w in [32usize, 96] {
            for v in variants {
                let name = format!("cycles-1x2-w{w}-{v}");
                let (s, j) = job(&dir, primary, v, w, 251, "profile", &name, 0, &env);
                steps.push(s);
                jobs.push(j);
            }
        }
        for j in jobs.iter().filter(|j| j["phase"] == "profile") {
            steps.push(verify(j));
        }
        for j in jobs.iter().filter(|j| j["phase"] == "profile") {
            let name = j["name"].as_str().unwrap();
            let data = Path::new(j["out"].as_str().unwrap()).join("perf.data");
            steps.push(json!({"kind":"command","name":format!("symbols-{name}"),"argv":[PERF,"script","--show-lost-events","-i",data,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
            steps.push(json!({"kind":"command","name":format!("events-{name}"),"argv":[PERF,"evlist","-v","-i",data],"timeout_seconds":120}));
        }
        assert_eq!(jobs.len(), 1221);
        for j in jobs.iter().filter(|j| j["phase"] == "profile") {
            steps.push(json!({"kind":"command","name":format!("loss-{}",j["name"].as_str().unwrap()),"argv":["/usr/local/bin/aws-runner","loss",Path::new(j["out"].as_str().unwrap()).join("perf.data")],"timeout_seconds":120}));
        }
        assert_eq!(steps.len(), 2460);
    }
    let sdk: usize = jobs
        .iter()
        .map(|j| j["rounds"].as_u64().unwrap() as usize + 3)
        .sum();
    assert_eq!(sdk, if phase == "preflight" { 108 } else { 37164 });
    save(dir.join("jobs.json"), &json!(jobs));
    save(dir.join("pins.json"), &json!(pins));
    save(
        dir.join("plan.json"),
        &json!({"version":1,"source_pins":dir.join("pins.json"),"steps":steps}),
    );
    run(
        &[
            "/usr/local/bin/cr-xtask".into(),
            "workflow".into(),
            dir.join("plan.json").display().to_string(),
            dir.join("workflow").display().to_string(),
        ],
        &dir.join("workflow.log"),
    );
    let before = seen.clone();
    let mut inventory = BTreeMap::new();
    let rows = audit(&dir, &jobs, &mut seen, &mut inventory, &categories);
    assert_eq!(seen.len(), if phase == "preflight" { 19288 } else { 20509 });
    for (p, h) in &pins {
        assert_eq!(hash(p), *h, "closing{p}");
    }
    save(dir.join("rows.json"), &json!(rows));
    save(dir.join("proof-inventory.json"), &json!(inventory));
    save(dir.join("after.json"), &state());
    let new: Vec<_> = seen.difference(&before).cloned().collect();
    save(
        dir.join("complete.json"),
        &json!({"status":"complete","sdk_requests":sdk,"distinct_gnark_exports":jobs.len(),"new_proof_sha256":new,"proof_ledger":seen.len(),"all_fixture_checks_passed":true,"production_changed":false}),
    );
    println!(
        "{phase} complete {sdk}SDK {}Gnark ledger{}",
        jobs.len(),
        seen.len()
    );
}
