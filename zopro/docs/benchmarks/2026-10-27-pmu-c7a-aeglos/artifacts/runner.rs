//! Fixed primary counter/profile census. Never builds or selects code.
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
fn run(argv: &[String], path: &Path) {
    let f = fs::File::create(path).unwrap();
    assert!(
        Command::new(&argv[0])
            .args(&argv[1..])
            .stdout(Stdio::from(f.try_clone().unwrap()))
            .stderr(Stdio::from(f))
            .status()
            .unwrap()
            .success(),
        "failed command; see {}",
        path.display()
    );
}
fn state() -> Value {
    let mut v = BTreeMap::new();
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
            v.insert(p, s);
        }
    }
    json!(v)
}
fn counters(path: &Path) -> Value {
    let mut v = serde_json::Map::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let c: Vec<_> = line.split(';').collect();
        assert!(c.len() >= 5);
        let count: f64 = c[0].trim().parse().unwrap();
        let runtime: f64 = c[3].trim().parse().unwrap();
        let pct: f64 = c[4].trim().parse().unwrap();
        assert!(
            count >= 0. && runtime > 0. && pct >= 99.9,
            "invalid counter {line}"
        );
        v.insert(
            c[2].into(),
            json!({"count":count,"runtime_ns":runtime,"running_percent":pct}),
        );
    }
    assert_eq!(v.len(), 4);
    assert!(v["cycles:u"]["count"].as_f64().unwrap() > 0.);
    assert!(v["instructions:u"]["count"].as_f64().unwrap() > 0.);
    json!(v)
}
fn events(panel: &str) -> &'static str {
    match panel {
        "branch" => "{cycles:u,instructions:u,branches:u,branch-misses:u}",
        "cache" => "{cycles:u,instructions:u,cache-references:u,cache-misses:u}",
        _ => panic!("panel"),
    }
}
fn proof_path(root: &Path, case: &Value) -> PathBuf {
    root.join("proofs").join(
        Path::new(case["image"].as_str().unwrap())
            .file_stem()
            .unwrap(),
    )
}
fn job(
    root: &Path,
    case: &Value,
    engine: &str,
    workers: usize,
    rounds: usize,
    phase: &str,
    panel: &str,
    name: &str,
    base_env: &Value,
) -> (Vec<Value>, Value) {
    let out = root.join("runs").join(name);
    fs::create_dir_all(&out).unwrap();
    let proofdir = proof_path(&out, case);
    let mut env = base_env.as_object().unwrap().clone();
    env.insert("RAYON_NUM_THREADS".into(), json!(workers.to_string()));
    env.insert("ZOPRO_CR_ENGINE".into(), json!(engine));
    if phase != "preflight" {
        for fifo in ["ctl", "ack"] {
            assert!(Command::new("mkfifo")
                .args(["-m", "600"])
                .arg(out.join(fifo))
                .status()
                .unwrap()
                .success());
        }
        env.insert("ZOPRO_BENCH_PERF_CTL".into(), json!(out.join("ctl")));
        env.insert("ZOPRO_BENCH_PERF_ACK".into(), json!(out.join("ack")));
    }
    let mut child = vec![json!("env")];
    for (k, v) in &env {
        child.push(json!(format!("{k}={}", v.as_str().unwrap())));
    }
    child.extend([
        json!("taskset"),
        json!("-c"),
        json!(format!("0-{}", workers - 1)),
        json!("/usr/local/bin/cr-adapter"),
        case["image"].clone(),
        case["request"].clone(),
        json!(rounds.to_string()),
        json!(proofdir),
    ]);
    let mut argv = if phase == "counter" {
        fs::write(out.join("counters.txt"), b"").unwrap();
        vec![
            json!(PERF),
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
            json!(events(panel)),
            json!("-o"),
            json!(out.join("counters.txt")),
            json!("--"),
        ]
    } else if phase == "profile" {
        vec![
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
        ]
    } else {
        Vec::new()
    };
    argv.extend(child);
    assert!(argv.iter().all(|a| a.is_string()));
    let steps = vec![
        json!({"kind":"command","name":format!("measure-{name}"),"argv":argv,"timeout_seconds":600}),
        json!({"kind":"command","name":format!("verify-{name}"),"argv":["/usr/local/bin/gnark-verify","/keys",out.join("proofs"),"/manifest/zopro-keys.lock"],"timeout_seconds":120}),
    ];
    (
        steps,
        json!({"name":name,"case":case,"engine":engine,"workers":workers,"rounds":rounds,"phase":phase,"panel":panel,"directory":proofdir,"out":out}),
    )
}
fn audit_jobs(
    dir: &Path,
    jobs: &[Value],
    seen: &mut BTreeSet<String>,
    inventory: &mut BTreeMap<String, String>,
    categories: &[config::Category],
) -> Vec<Value> {
    let mut rows = Vec::new();
    for j in jobs {
        let name = j["name"].as_str().unwrap();
        let p = Path::new(j["directory"].as_str().unwrap()).join("0.proof");
        assert_eq!(fs::metadata(&p).unwrap().len(), 164);
        assert_eq!(
            hash(p.with_extension("public")),
            j["case"]["public_sha256"].as_str().unwrap()
        );
        assert!(seen.insert(hash(&p)), "proof repeats prior/newidentity");
        for f in [p.clone(), p.with_extension("public")] {
            inventory.insert(f.display().to_string(), hash(f));
        }
        let log =
            fs::read_to_string(dir.join(format!("workflow/measure-{name}/stdout.log"))).unwrap();
        let sdk = results::sdk_output(&log, j["rounds"].as_u64().unwrap() as usize).unwrap();
        assert_eq!(sdk.summary["workers"], j["workers"]);
        assert_eq!(
            read(dir.join(format!("workflow/verify-{name}/exit.json")))["success"],
            true
        );
        let counter = if j["phase"] == "counter" {
            counters(&Path::new(j["out"].as_str().unwrap()).join("counters.txt"))
        } else {
            Value::Null
        };
        let profile = if j["phase"] == "profile" {
            let s = fs::read_to_string(dir.join(format!("workflow/symbols-{name}/stdout.log")))
                .unwrap();
            let p = results::perf_samples(&s, categories).unwrap();
            assert_eq!(p.lost_records, 0);
            assert_eq!(p.sampled_pids.len(), 1);
            serde_json::to_value(p).unwrap()
        } else {
            Value::Null
        };
        rows.push(
            json!({"job":j,"sdk":sdk,"counters":counter,"profile":profile,"proof_sha256":hash(p)}),
        );
    }
    rows
}
fn main() {
    let phase = std::env::args().nth(1).unwrap();
    assert!(matches!(phase.as_str(), "preflight" | "measure"));
    let dir = Path::new("/output").join(&phase);
    fs::create_dir(&dir).unwrap();
    let m = read("/cr/manifest.json");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(m["container_pins"].clone()).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h, "containerpin{p}");
    }
    pins.insert("/cr/manifest.json".into(), hash("/cr/manifest.json"));
    let mut seen: BTreeSet<String> =
        serde_json::from_value(m["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 12143);
    let mut env = m["environment"].clone();
    env["ZOPRO_CPU_LAGRANGE_DIR"] = json!("/output/lagrange-cache");
    env["ZOPRO_BASIS_KEY_DIR"] = json!("/cr-basis");
    env["ZOPRO_CR_AFTER_LATENCY"] = json!(dir.join("before-first-instrumented.json"));
    let categories: Vec<config::Category> =
        serde_json::from_value(m["symbol_categories"].clone()).unwrap();
    let case = &m["case"];
    save(dir.join("before.json"), &state());
    for (name, args) in [
        ("lscpu.txt", vec![]),
        ("topology.txt", vec!["-e=CPU,CORE,SOCKET,NODE"]),
    ] {
        let out = Command::new("lscpu").args(args).output().unwrap();
        assert!(out.status.success());
        fs::write(dir.join(name), out.stdout).unwrap();
    }
    let mut jobs = Vec::new();
    let mut steps = Vec::new();
    if phase == "preflight" {
        for engine in ["current", "aeglos"] {
            let name = format!("preflight-{engine}");
            let (s, j) = job(&dir, case, engine, 4, 1, "preflight", "none", &name, &env);
            steps.extend(s);
            jobs.push(j);
        }
    } else {
        assert_eq!(std::thread::available_parallelism().unwrap().get(), 96);
        assert_eq!(
            fs::read_to_string("/sys/fs/cgroup/memory.swap.max")
                .unwrap()
                .trim(),
            "0"
        );
        let topo = Command::new("lscpu")
            .args(["-p=CPU,CORE,SOCKET"])
            .output()
            .unwrap();
        assert!(topo.status.success());
        let csv = String::from_utf8(topo.stdout).unwrap();
        let mut cores = BTreeSet::new();
        let mut logical = 0;
        for line in csv.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
            let f: Vec<_> = line.split(',').collect();
            assert_eq!(f.len(), 3);
            logical += 1;
            cores.insert((f[1].to_owned(), f[2].to_owned()));
        }
        assert_eq!((logical, cores.len()), (96, 96));
        fs::write(dir.join("topology.csv"), csv).unwrap();
        let receipt = read("/cr/preflight-complete.json");
        assert_eq!(receipt["status"], "complete");
        assert_eq!(receipt["sdk_requests"], 8);
        assert_eq!(receipt["distinct_gnark_exports"], 2);
        for h in receipt["new_proof_sha256"].as_array().unwrap() {
            assert!(seen.insert(h.as_str().unwrap().into()));
        }
        assert_eq!(seen.len(), 12145);
        pins.insert(
            "/cr/preflight-complete.json".into(),
            hash("/cr/preflight-complete.json"),
        );
        for panel in ["branch", "cache"] {
            let p = dir.join(format!("capability-{panel}.txt"));
            run(
                &[
                    PERF.into(),
                    "stat".into(),
                    "--no-big-num".into(),
                    "-x".into(),
                    ";".into(),
                    "-e".into(),
                    events(panel).into(),
                    "-o".into(),
                    p.display().to_string(),
                    "--".into(),
                    "sha256sum".into(),
                    "/usr/local/bin/cr-current".into(),
                ],
                &dir.join(format!("capability-{panel}.log")),
            );
            let valid = counters(&p);
            save(dir.join(format!("capability-{panel}.json")), &valid);
        }
        let cells: Vec<_> = [32usize, 96]
            .into_iter()
            .flat_map(|w| {
                ["current", "aeglos"].into_iter().flat_map(move |engine| {
                    ["branch", "cache"]
                        .into_iter()
                        .map(move |panel| (w, engine, panel))
                })
            })
            .collect();
        for b in 0..3 {
            let mut order = cells.clone();
            let size = order.len();
            order.rotate_left((b * 3) % size);
            if b % 2 == 1 {
                order.reverse();
            }
            for (w, engine, panel) in order {
                let name = format!("b{b}-{engine}-w{w}-{panel}");
                let (s, j) = job(&dir, case, engine, w, 501, "counter", panel, &name, &env);
                steps.extend(s);
                jobs.push(j);
            }
        }
        // Profiles begin only after all counter requests and their verifiers finish.
        for (w, engine) in [
            (32, "current"),
            (32, "aeglos"),
            (96, "current"),
            (96, "aeglos"),
        ] {
            let name = format!("profile-{engine}-w{w}");
            let (s, j) = job(&dir, case, engine, w, 251, "profile", "cycles", &name, &env);
            steps.extend(s);
            jobs.push(j);
        }
        // Decode only after every SDK/profile process is terminal.
        for j in jobs.iter().filter(|j| j["phase"] == "profile") {
            let name = j["name"].as_str().unwrap();
            steps.push(json!({"kind":"command","name":format!("symbols-{name}"),"argv":[PERF,"script","--show-lost-events","--no-demangle","-i",Path::new(j["out"].as_str().unwrap()).join("perf.data"),"-F","pid,tid,period,ip,sym"],"timeout_seconds":600}));
        }
        assert_eq!(jobs.len(), 28);
    }
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
    let mut inventory = BTreeMap::new();
    let rows = audit_jobs(&dir, &jobs, &mut seen, &mut inventory, &categories);
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let exports = if phase == "preflight" { 2 } else { 28 };
    let sdk = if phase == "preflight" { 8 } else { 13112 };
    assert_eq!(inventory.len(), exports * 2);
    assert_eq!(seen.len(), if phase == "preflight" { 12145 } else { 12173 });
    save(dir.join("rows.json"), &json!(rows));
    save(dir.join("proof-inventory.json"), &json!(inventory));
    save(
        dir.join("complete.json"),
        &json!({"status":"complete","sdk_requests":sdk,"distinct_gnark_exports":exports,"proof_public_members":exports*2,"all_fixture_checks_passed":true,"prior_after":seen,"after":state(),"counter_processes":if phase=="measure"{24}else{0},"profiles":if phase=="measure"{4}else{0},"acceptance_measurement":false,"source_changed":false}),
    );
    println!("Complete {phase}: {sdk}SDK/{exports}distinctGnark;source unchanged");
}
