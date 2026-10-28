use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/concurrent-callers";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
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
fn pin(pins: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn env() -> Value {
    json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"})
}
fn setup() {
    assert!(!r("build-plan.json").exists());
    let a = read(format!("{P}/artifacts.json"));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{P}/pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for (p, h) in a["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    for f in ["design.md", "probe.rs", "perf.rs", "driver.rs", "driver"] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, X);
    let flags = a["rustflags"].as_str().unwrap();
    assert!(flags.contains("zopro_g1_weighted_anchor"));
    assert!(flags.contains("zopro_anchor_caller_probe"));
    fs::write(r("native-flags.txt"), flags).unwrap();
    pin(&mut pins, r("native-flags.txt"));
    let mut steps = vec![];
    for tests in [true, false] {
        let name = if tests { "probe-tests" } else { "probe" };
        let mut argv = vec!["rustc".to_owned()];
        argv.extend(flags.split_whitespace().map(str::to_owned));
        argv.extend([
            "--edition=2021".into(),
            "-O".into(),
            r("probe.rs").display().to_string(),
            "-o".into(),
            r(name).display().to_string(),
            "-L".into(),
            format!("dependency={P}/frozen"),
        ]);
        for n in [
            "zopro_core",
            "zopro_bn254",
            "libc",
            "rayon",
            "serde_json",
            "sha2",
        ] {
            argv.extend([
                "--extern".into(),
                format!("{n}={}", a["libraries"][n].as_str().unwrap()),
            ]);
        }
        if tests {
            argv.push("--test".into());
        }
        save(r(format!("link-{name}.json")), &json!(argv));
        steps.push(json!({"kind":"command","name":format!("link-{name}"),"argv":argv,"timeout_seconds":300}));
        if tests {
            steps.push(json!({"kind":"command","name":"harness-tests","argv":[r(name),PathBuf::from("--test-threads=1")],"environment":env(),"required_tests":["tests::independent_workers_preserve_oracles_affinity_barriers_and_reuse"],"unfiltered":true,"timeout_seconds":300}));
        }
    }
    save(
        r("library-provenance.json"),
        &json!({"parent_receipt":a,"native_arithmetic_rebuilt":false,"SDK_candidate_closed":true,"scope":"same frozen core/field/diagnostic adapter libraries as parent; only new concurrency harness linked"}),
    );
    pin(&mut pins, r("library-provenance.json"));
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared3harnesslink/teststeps;arithmeticunchanged;parentfullSDKcandidateclosed");
}
fn workers(t: &str) -> usize {
    match t {
        "solo" => 1,
        "smt" | "separate" => 2,
        "physical16" => 16,
        "logical32" => 32,
        _ => panic!("topology"),
    }
}
fn prepare() {
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let log = fs::read_to_string(r("native/harness-tests/stdout.log")).unwrap();
    assert!(log.contains("test result: ok. 3 passed; 0 failed"));
    assert!(!r("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for f in [
        "probe",
        "probe-tests",
        "native/result.json",
        "native/harness-tests/stdout.log",
        "build-plan.json",
        "build-pins.json",
        "link-probe.json",
        "link-probe-tests.json",
    ] {
        pin(&mut pins, r(f));
    }
    let mut topology = BTreeMap::new();
    for cpu in 0..32 {
        let base = format!("/sys/devices/system/cpu/cpu{cpu}/topology");
        let core = fs::read_to_string(format!("{base}/core_id")).unwrap();
        let siblings = fs::read_to_string(format!("{base}/thread_siblings_list")).unwrap();
        topology.insert(
            cpu,
            json!({"core_id":core.trim(),"siblings":siblings.trim()}),
        );
    }
    assert_eq!(topology[&2]["siblings"], "2,18");
    assert_eq!(topology[&3]["siblings"], "3,19");
    let physical: BTreeSet<_> = (0..16)
        .map(|cpu| topology[&cpu]["core_id"].as_str().unwrap())
        .collect();
    assert_eq!(physical.len(), 16);
    save(r("topology.json"), &json!(topology));
    pin(&mut pins, r("topology.json"));
    for (name, argv) in [
        (
            "host",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
        ("machine", vec!["lscpu"]),
        ("compiler", vec!["rustc", "-vV"]),
    ] {
        let o = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(o.status.success());
        let p = r(format!("{name}.txt"));
        fs::write(&p, o.stdout).unwrap();
        pin(&mut pins, p);
    }
    let c = Command::new("rustup")
        .args(["which", "rustc"])
        .output()
        .unwrap();
    assert!(c.status.success());
    pin(&mut pins, String::from_utf8(c.stdout).unwrap().trim());
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   kind: &str,
                   size: usize,
                   mode: usize,
                   topology: &str,
                   rounds: usize| {
        let normal = vec![
            r("probe").display().to_string(),
            kind.into(),
            size.to_string(),
            mode.to_string(),
            topology.into(),
            rounds.to_string(),
        ];
        let mut environment = env();
        let argv = if phase != "latency" {
            let dir = r("profiles").join(&name);
            fs::create_dir_all(&dir).unwrap();
            assert!(Command::new("mkfifo")
                .arg(dir.join("ctl"))
                .arg(dir.join("ack"))
                .status()
                .unwrap()
                .success());
            environment["ZOPRO_BENCH_PERF_CTL"] = json!(dir.join("ctl"));
            environment["ZOPRO_BENCH_PERF_ACK"] = json!(dir.join("ack"));
            let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in environment.as_object().unwrap() {
                a.push(format!("{k}={}", v.as_str().unwrap()));
            }
            a.push("perf".into());
            if phase == "stat" {
                a.extend([
                    "stat".into(),
                    "--no-big-num".into(),
                    "-x".into(),
                    ";".into(),
                    "-e".into(),
                    "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                ]);
            } else {
                a.extend([
                    "record".into(),
                    "--no-buildid-cache".into(),
                    "-e".into(),
                    "cycles:u".into(),
                    "-F".into(),
                    "199".into(),
                    "-m".into(),
                    "512".into(),
                    "-o".into(),
                    dir.join("perf.data").display().to_string(),
                ]);
            }
            a.extend([
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    dir.join("ctl").display(),
                    dir.join("ack").display()
                ),
                "--".into(),
            ]);
            a.extend(normal);
            a
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":300}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"kind":kind,"size":size,"mode":mode,"topology":topology,"worker_count":workers(topology),"rounds":rounds}));
    };
    for b in 0..15 {
        let mut contexts = vec![];
        for (kind, size) in [
            ("g1-owned", 200),
            ("g1-borrowed-signed", 352),
            ("g2-prefix", 352),
        ] {
            for topology in ["solo", "smt", "separate", "physical16", "logical32"] {
                contexts.push((kind, size, topology));
            }
        }
        contexts.rotate_left(b % 15);
        if b % 2 != 0 {
            contexts.reverse();
        }
        for (i, (kind, size, topology)) in contexts.into_iter().enumerate() {
            let mut modes = vec![0, 1, 2];
            modes.rotate_left((b + i) % 3);
            if b % 2 != 0 {
                modes.reverse();
            }
            for m in modes {
                add(
                    format!("b{b}-{kind}-{topology}-m{m}"),
                    "latency",
                    b,
                    kind,
                    size,
                    m,
                    topology,
                    8192,
                );
            }
        }
    }
    for topology in ["solo", "smt", "physical16", "logical32"] {
        for mode in [0, 2] {
            add(
                format!("stat-{topology}-m{mode}"),
                "stat",
                0,
                "g1-borrowed-signed",
                352,
                mode,
                topology,
                262144,
            );
        }
    }
    for topology in ["physical16", "logical32"] {
        for mode in [0, 2] {
            let name = format!("cycles-{topology}-m{mode}");
            add(
                name.clone(),
                "cycles",
                0,
                "g1-borrowed-signed",
                352,
                mode,
                topology,
                262144,
            );
            profiles.push(name);
        }
    }
    assert_eq!(jobs.len(), 687);
    for name in profiles {
        let p = r("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 699);
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared675latency+8stat+4cycles+12decode/owner/loss;687kernels/699steps;onlyconcurrencyharnessnew");
}
fn median(v: &[f64]) -> f64 {
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    x[x.len() / 2]
}
fn bounds(v: &[f64]) -> Value {
    assert_eq!(v.len(), 15);
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    json!({"median":x[7],"low":x[3],"high":x[11],"wins":v.iter().filter(|v|**v<1.).count(),"ratios":v,"nominal_marginal_coverage":0.96484375})
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let jobs = read(r("jobs.json"));
    let mut results = BTreeMap::new();
    let mut digests = BTreeMap::new();
    let mut caps = BTreeMap::new();
    let mut stat = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut total_calls = 0u64;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|l| l.strip_prefix("RESULT "))
            .collect();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        for f in ["kind", "size", "mode", "topology", "worker_count"] {
            assert_eq!(v[f], j[f]);
        }
        assert_eq!(v["iterations_per_worker"], j["rounds"]);
        assert_eq!(v["prime_calls"], 3);
        assert_eq!(v["prime_iterations_per_worker"], 1024);
        assert_eq!(v["timed_allocations"], 0);
        assert_eq!(v["internally_verified"], true);
        let count = v["total_kernel_calls"].as_u64().unwrap();
        assert_eq!(
            count,
            j["rounds"].as_u64().unwrap() * j["worker_count"].as_u64().unwrap()
        );
        total_calls += count;
        let key = format!("{}-{}", j["kind"], j["rounds"]);
        let h = v["output_sha256"].as_str().unwrap();
        if let Some(old) = digests.insert(key, h.to_owned()) {
            assert_eq!(old, h);
        }
        let key = j["kind"].as_str().unwrap();
        let c = json!([
            v["capacity_bytes_per_worker"],
            v["fixture_reference_bytes_per_worker"]
        ]);
        if let Some(old) = caps.insert(key.to_owned(), c.clone()) {
            assert_eq!(old, c);
        }
        let mut cpus = BTreeSet::new();
        let mut tids = BTreeSet::new();
        for w in v["workers"].as_array().unwrap() {
            assert!(cpus.insert(w["cpu"].as_u64().unwrap()));
            assert!(tids.insert(w["tid"].as_u64().unwrap()));
            assert_eq!(w["digest"], v["output_sha256"]);
        }
        assert_eq!(cpus.len(), j["worker_count"].as_u64().unwrap() as usize);
        assert_eq!(
            cpus.into_iter().collect::<Vec<_>>(),
            v["cpus"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .collect::<Vec<_>>()
        );
        if j["phase"] == "stat" {
            let text = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
            let mut events = BTreeMap::new();
            for l in text.lines() {
                let f: Vec<_> = l.trim().split(';').collect();
                if f.len() < 5 || !f[2].ends_with(":u") {
                    continue;
                }
                let c: f64 = f[0].trim().parse().expect("counter must count");
                let pct: f64 = f[4].trim().parse().unwrap();
                assert!(pct > 0.);
                events.insert(
                    f[2].to_owned(),
                    json!({"count":c,"per_kernel_call":c/count as f64,"running_percent":pct}),
                );
            }
            assert_eq!(events.len(), 4);
            let ipc = events["instructions:u"]["count"].as_f64().unwrap()
                / events["cycles:u"]["count"].as_f64().unwrap();
            stat.insert(name.to_owned(),json!({"events":events,"IPC":ipc,"total_kernel_calls":count,"workers":j["worker_count"]}));
        }
        if j["phase"] == "cycles" {
            let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
            assert!(!loss.contains("PERF_RECORD_LOST"));
            let (mut samples, mut period) = (0u64, 0u64);
            let mut pids = BTreeSet::new();
            let mut observed = BTreeSet::new();
            let mut symbols: BTreeMap<String, u64> = BTreeMap::new();
            for l in text.lines().filter(|l| !l.trim_start().starts_with('#')) {
                let f: Vec<_> = l.split_whitespace().collect();
                if f.len() < 4 {
                    continue;
                }
                let (p, t) = f[0].split_once('/').unwrap();
                let p: u64 = p.parse().unwrap();
                let t: u64 = t.parse().unwrap();
                let weight: u64 = f[1].parse().unwrap();
                pids.insert(p);
                observed.insert(t);
                samples += 1;
                period += weight;
                let s = format!("{:#}", rustc_demangle::demangle(&f[3..].join(" ")));
                *symbols.entry(s).or_default() += weight;
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            let sampled_worker_tids: BTreeSet<_> = observed.intersection(&tids).copied().collect();
            profiles.insert(name.to_owned(),json!({"samples":samples,"period":period,"pids":pids,"sampled_tids":observed,"sampled_worker_tids":sampled_worker_tids,"lost":0,"symbols":symbols,"raw_perf_sha256":hash(r("profiles").join(name).join("perf.data"))}));
        }
        results.insert(name.to_owned(), v);
    }
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let mut calibration = BTreeMap::new();
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-prefix"] {
        for topology in ["solo", "smt", "separate", "physical16", "logical32"] {
            for mode in 0..3 {
                let mut measures = BTreeMap::new();
                for metric in [
                    "wall_ns_per_wave",
                    "throughput_ns_per_call",
                    "mean_worker_ns_per_batch",
                    "thread_cpu_ns_per_call",
                ] {
                    let v: Vec<_> = (0..15)
                        .map(|b| {
                            results[&format!("b{b}-{kind}-{topology}-m{mode}")][metric]
                                .as_f64()
                                .unwrap()
                        })
                        .collect();
                    measures.insert(metric, json!({"median":median(&v),"process_values":v}));
                }
                cells.insert(format!("{kind}/{topology}/m{mode}"), json!(measures));
            }
            for (top, base) in [(1, 0), (2, 0), (2, 1)] {
                let v: Vec<_> = (0..15)
                    .map(|b| {
                        results[&format!("b{b}-{kind}-{topology}-m{top}")]["wall_ns_per_wave"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{b}-{kind}-{topology}-m{base}")]
                                ["wall_ns_per_wave"]
                                .as_f64()
                                .unwrap()
                    })
                    .collect();
                let b = bounds(&v);
                if top == 1 {
                    calibration.insert(format!("{kind}/{topology}"),json!({"pass":b["low"].as_f64().unwrap()>=1./1.03&&b["high"].as_f64().unwrap()<=1.03,"bounds":b}));
                }
                pairs.insert(format!("{kind}/{topology}/{top}-over-{base}"), b);
            }
        }
    }
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    save(
        r("analysis.json"),
        &json!({"scope":"concurrent synthetic point callers; descriptive topology interaction, not a reopened SDK candidate or speed-adoption gate","cells":cells,"pairs":pairs,"ordinary_label_calibration":calibration,"stat":stat,"profiles":profiles,"results":results,"capacities_per_worker":caps,"timed_kernel_calls":total_calls,"captures":687,"latency_processes":675,"native_new_harness_tests":1,"inherited_perf_helper_tests":2,"native_arithmetic_rebuilt":false,"SDK_candidate_stays_closed":true,"final_pins_verified":pins.as_object().unwrap().len(),"new_SDK_or_Gnark_proofs":0,"independent_proof_count":13082,"production_changed":false,"adopted":false,"goal_active_original8_achieved":0}),
    );
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-prefix"] {
        for topology in ["solo", "smt", "separate", "physical16", "logical32"] {
            let b = &pairs[&format!("{kind}/{topology}/2-over-0")];
            println!(
                "{kind}/{topology}:ON/OFF{} bounds[{},{}],{}/15wins",
                b["median"], b["low"], b["high"], b["wins"]
            );
        }
    }
    println!("All687generatorcheckedzeroallocationcaptures;{total_calls}timedkernelcalls;noSDKoradoption");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "setup" => setup(),
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!("mode"),
    }
}
