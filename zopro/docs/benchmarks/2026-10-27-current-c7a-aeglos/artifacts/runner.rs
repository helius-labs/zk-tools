//! Fixed current/previous/Aeglos comparison. Builds never run here.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
};
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
fn run(args: &[String], path: &Path) {
    let f = fs::File::create(path).unwrap();
    assert!(
        Command::new(&args[0])
            .args(&args[1..])
            .stdout(Stdio::from(f.try_clone().unwrap()))
            .stderr(Stdio::from(f))
            .status()
            .unwrap()
            .success(),
        "command failed; see {}",
        path.display()
    );
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
fn proofs(p: &Path, out: &mut Vec<std::path::PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            proofs(&p, out)
        } else if p.extension().is_some_and(|x| x == "proof") {
            out.push(p)
        }
    }
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
        assert_eq!(hash(p), *h, "container pin {p}");
    }
    pins.insert("/cr/manifest.json".into(), hash("/cr/manifest.json"));
    let mut prior: BTreeSet<String> =
        serde_json::from_value(m["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 9904);
    let mut env = m["environment"].clone();
    env["ZOPRO_CPU_LAGRANGE_DIR"] = json!("/output/lagrange-cache");
    env["ZOPRO_BASIS_KEY_DIR"] = json!("/cr-basis");
    let modes = ["previous", "current", "aeglos"];
    save(dir.join("before.json"), &state());
    for (name, args) in [
        ("lscpu.txt", vec![]),
        ("topology.txt", vec!["-e=CPU,CORE,SOCKET,NODE"]),
    ] {
        let out = Command::new("lscpu").args(args).output().unwrap();
        assert!(out.status.success());
        fs::write(dir.join(name), out.stdout).unwrap();
    }
    if phase == "preflight" {
        let mut steps = Vec::new();
        let mut jobs = Vec::new();
        for c in m["cases"].as_array().unwrap() {
            for mode in modes {
                let name = format!("{}-w4-{mode}", c["shape"].as_str().unwrap());
                let proofroot = dir.join("proofs").join(&name);
                let proofdir =
                    proofroot.join(Path::new(c["image"].as_str().unwrap()).file_stem().unwrap());
                let mut e = env.clone();
                e["RAYON_NUM_THREADS"] = json!("4");
                e["ZOPRO_CR_ENGINE"] = json!(mode);
                steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","0-3","/usr/local/bin/cr-adapter",c["image"],c["request"],"1",proofdir],"environment":e,"timeout_seconds":600}));
                steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/usr/local/bin/gnark-verify","/keys",proofroot,"/manifest/zopro-keys.lock"],"timeout_seconds":120}));
                jobs.push(
                    json!({"name":name,"case":c,"mode":mode,"workers":4,"proof_dir":proofdir}),
                );
            }
        }
        assert_eq!(jobs.len(), 27);
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
        let mut ps = Vec::new();
        proofs(&dir.join("proofs"), &mut ps);
        assert_eq!(ps.len(), 27);
        let mut inv = BTreeMap::new();
        for p in ps {
            assert_eq!(fs::metadata(&p).unwrap().len(), 164);
            let h = hash(&p);
            assert!(prior.insert(h.clone()));
            inv.insert(p.display().to_string(), h);
            let public = p.with_extension("public");
            let stem = p.parent().unwrap().file_name().unwrap();
            let c = m["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|c| Path::new(c["image"].as_str().unwrap()).file_stem().unwrap() == stem)
                .unwrap();
            assert_eq!(hash(&public), c["public_sha256"].as_str().unwrap());
            inv.insert(public.display().to_string(), hash(public));
        }
        for j in &jobs {
            let log = fs::read_to_string(
                dir.join("workflow")
                    .join(j["name"].as_str().unwrap())
                    .join("stdout.log"),
            )
            .unwrap();
            let s: Value = serde_json::from_str(
                log.lines()
                    .find_map(|l| l.strip_prefix("SUMMARY "))
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(s["rounds"], 1);
            assert_eq!(s["priming"], 3);
            assert_eq!(s["workers"], 4);
        }
        assert_eq!(prior.len(), 9931);
        save(dir.join("proof-inventory.json"), &json!(inv));
        save(
            dir.join("complete.json"),
            &json!({"status":"complete","sdk_requests":108,"distinct_gnark_exports":27,"all_fixture_checks_passed":true,"prior_after":prior,"after":state()}),
        );
    } else {
        let receipt = read("/cr/preflight-complete.json");
        assert_eq!(receipt["status"], "complete");
        assert_eq!(receipt["sdk_requests"], 108);
        assert_eq!(receipt["distinct_gnark_exports"], 27);
        assert_eq!(receipt["all_fixture_checks_passed"], true);
        for h in receipt["new_proof_sha256"].as_array().unwrap() {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
        assert_eq!(prior.len(), 9931);
        pins.insert(
            "/cr/preflight-complete.json".into(),
            hash("/cr/preflight-complete.json"),
        );
        let perf = "/usr/lib/linux-tools/6.8.0-147-generic/perf";
        run(
            &[
                perf.into(),
                "stat".into(),
                "-e".into(),
                "cycles:u".into(),
                "--".into(),
                "/bin/true".into(),
            ],
            &dir.join("perf-availability.log"),
        );
        assert_eq!(
            fs::read_to_string("/sys/fs/cgroup/memory.swap.max")
                .unwrap()
                .trim(),
            "0"
        );
        let mut cases = Vec::new();
        for c in m["cases"].as_array().unwrap() {
            for workers in [32, 64, 96] {
                let mut c = c.clone();
                c["name"] = json!(format!("{}-w{workers}", c["shape"].as_str().unwrap()));
                c["workers"] = json!(workers);
                c["cpu_affinity"] = json!(format!("0-{}", workers - 1));
                c["rounds"] = json!(if c["shape"] == "1x2" { 101 } else { 17 });
                c["profile_rounds"] = json!(251);
                cases.push(c);
            }
        }
        assert_eq!(cases.len(), 27);
        env["ZOPRO_CR_AFTER_LATENCY"] = json!(dir.join("after-latency.json"));
        let variants:Vec<_>=modes.iter().map(|engine|json!({"name":engine,"binary":"/usr/local/bin/cr-adapter","environment":{"ZOPRO_CR_ENGINE":engine}})).collect();
        let config = json!({"version":1,"blocks":3,"profile_blocks":1,"profile_cases":["1x2-w32","1x2-w96"],"command":["{binary}","{image}","{request}","{rounds}","{proof_dir}/{stem}"],"variants":variants,"cases":cases,"pins":pins,"prior_proof_sha256":prior,"verifier":["/usr/local/bin/gnark-verify","/keys","{proof_dir}","/manifest/zopro-keys.lock"],"profile_environment":{"ZOPRO_BENCH_PERF_CTL":"{control}","ZOPRO_BENCH_PERF_ACK":"{ack}"},"environment":env,"checks":[],"child_prefix":[],"perf_prefix":[],"perf":perf,"frequency_hz":199,"monotonic_clock":true,"symbol_categories":m["symbol_categories"],"timeout_seconds":600});
        save(dir.join("config.json"), &config);
        run(
            &[
                "/usr/local/bin/cr-xtask".into(),
                "perf".into(),
                dir.join("config.json").display().to_string(),
                dir.join("performance").display().to_string(),
            ],
            &dir.join("workflow.log"),
        );
        let rows = read(dir.join("performance/rows.json"));
        assert_eq!(rows.as_array().unwrap().len(), 249);
        let sdk: usize = rows
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["samples_ms"].as_array().unwrap().len() + 3)
            .sum();
        assert_eq!(sdk, 8652);
        assert!(dir.join("after-latency.json").exists());
        save(dir.join("after-profiles.json"), &state());
        save(
            dir.join("complete.json"),
            &json!({"status":"complete","sdk_requests":sdk,"distinct_gnark_exports":249,"after":state()}),
        );
    }
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    println!("{phase} complete");
}
