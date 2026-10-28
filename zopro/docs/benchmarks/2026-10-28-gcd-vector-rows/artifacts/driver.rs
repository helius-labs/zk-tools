use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/gcd-vector-rows";
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
fn pin(m: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    m.insert(p.display().to_string(), hash(p));
}
fn workers(t: &str) -> usize {
    match t {
        "solo" => 1,
        "physical16" => 16,
        "logical32" => 32,
        _ => panic!("topology"),
    }
}
fn prepare() {
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    assert!(!r("run-plan.json").exists());
    for mode in ["checked", "optimized"] {
        let t = fs::read_to_string(r(format!("native/test-{mode}/stdout.log"))).unwrap();
        assert!(t.contains("test result: ok. 10 passed; 0 failed"));
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for p in [
        "driver.rs",
        "driver",
        "build-plan.json",
        "build-pins.json",
        "native/result.json",
        "probe",
        "checked",
        "optimized",
        "native/test-checked/stdout.log",
        "native/test-optimized/stdout.log",
    ] {
        pin(&mut pins, r(p));
    }
    pin(&mut pins, X);
    let mut libraries = BTreeMap::new();
    for dir in [
        "/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps",
        "/home/ubuntu/src/zopro-perf-work/mixed-sdk/target-control/release/deps",
    ] {
        for e in fs::read_dir(dir).unwrap() {
            let e = e.unwrap();
            let p = e.path();
            if e.file_type().unwrap().is_file()
                && ["rlib", "so"].contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            {
                pin(&mut libraries, &p);
                pin(&mut pins, p);
            }
        }
    }
    save(
        r("available-library-pins.json"),
        &json!({"scope":"allavailable rlibs/so inbothRustlinkdependencydirectories afterstandalonebuild andbeforecapture; includesunusedfiles; noCargo rebuilt","files":libraries}),
    );
    pin(&mut pins, r("available-library-pins.json"));
    for (name, argv) in [
        (
            "host",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
        ("compiler", vec!["rustc", "-vV"]),
        ("machine", vec!["lscpu"]),
    ] {
        let o = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
        assert!(o.status.success());
        let p = r(format!("{name}.txt"));
        fs::write(&p, o.stdout).unwrap();
        pin(&mut pins, p);
    }
    let o = Command::new("rustup")
        .args(["which", "rustc"])
        .output()
        .unwrap();
    assert!(o.status.success());
    pin(&mut pins, String::from_utf8(o.stdout).unwrap().trim());
    let mut topology = BTreeMap::new();
    for c in 0..32 {
        let base = format!("/sys/devices/system/cpu/cpu{c}/topology");
        topology.insert(c,json!({"core_id":fs::read_to_string(format!("{base}/core_id")).unwrap().trim(),"siblings":fs::read_to_string(format!("{base}/thread_siblings_list")).unwrap().trim()}));
    }
    assert_eq!(topology[&2]["siblings"], "2,18");
    assert_eq!(
        (0..16)
            .map(|c| topology[&c]["core_id"].as_str().unwrap())
            .collect::<BTreeSet<_>>()
            .len(),
        16
    );
    save(r("topology.json"), &json!(topology));
    pin(&mut pins, r("topology.json"));
    let nm = Command::new("nm")
        .arg("--defined-only")
        .arg(r("probe"))
        .output()
        .unwrap();
    assert!(nm.status.success());
    fs::write(r("symbols.txt"), &nm.stdout).unwrap();
    pin(&mut pins, r("symbols.txt"));
    let mut code = vec![];
    let mut muls = 0usize;
    for (i, l) in String::from_utf8(nm.stdout)
        .unwrap()
        .lines()
        .filter(|l| l.contains("calculate") || l.contains("selected") || l.contains("ordinary"))
        .enumerate()
    {
        let symbol = l.split_whitespace().last().unwrap();
        let name = format!("{:#}", rustc_demangle::demangle(symbol));
        let o = Command::new("objdump")
            .args(["-d", "--no-show-raw-insn"])
            .arg(format!("--disassemble={symbol}"))
            .arg(r("probe"))
            .output()
            .unwrap();
        assert!(o.status.success());
        let text = String::from_utf8(o.stdout).unwrap();
        let n = text.matches("vpmuludq").count();
        muls += n;
        let p = r(format!("codegen-{i}.asm.txt"));
        fs::write(&p, text).unwrap();
        pin(&mut pins, &p);
        code.push(json!({"symbol":symbol,"demangled":name,"path":p,"vpmuludq_sites":n}));
    }
    assert!(muls >= 4);
    save(
        r("codegen.json"),
        &json!({"read_only_objdump":true,"vpmuludq_sites_inspected":muls,"symbols":code}),
    );
    pin(&mut pins, r("codegen.json"));
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   field: &str,
                   mode: usize,
                   t: &str,
                   passes: usize| {
        let normal = vec![
            r("probe").display().to_string(),
            field.into(),
            mode.to_string(),
            t.into(),
            passes.to_string(),
        ];
        let mut environment = json!({});
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
        jobs.push(json!({"name":name,"phase":phase,"block":block,"field":field,"mode":mode,"topology":t,"passes":passes,"worker_count":workers(t)}));
    };
    for b in 0..31 {
        let mut contexts = vec![];
        for field in ["fq", "fr"] {
            for t in ["solo", "physical16", "logical32"] {
                contexts.push((field, t));
            }
        }
        contexts.rotate_left(b % 6);
        if b % 2 != 0 {
            contexts.reverse();
        }
        for (i, (field, t)) in contexts.into_iter().enumerate() {
            let mut modes = vec![0, 1, 2];
            modes.rotate_left((b + i) % 3);
            if b % 2 != 0 {
                modes.reverse();
            }
            for mode in modes {
                add(
                    format!("b{b}-{field}-{t}-m{mode}"),
                    "latency",
                    b,
                    field,
                    mode,
                    t,
                    16,
                );
            }
        }
    }
    for phase in ["stat", "cycles"] {
        for field in ["fq", "fr"] {
            for mode in [1, 2] {
                let name = format!("{phase}-{field}-m{mode}");
                add(name.clone(), phase, 0, field, mode, "logical32", 128);
                if phase == "cycles" {
                    profiles.push(name);
                }
            }
        }
    }
    assert_eq!(jobs.len(), 566);
    for name in profiles {
        let p = r("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 578);
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared558completeinverselatency/4stat/4cycles/12decode;566captures578steps;{muls}VPMULUDQsitesinspected");
}
fn median(x: &[f64]) -> f64 {
    let mut x = x.to_vec();
    x.sort_by(f64::total_cmp);
    x[x.len() / 2]
}
fn bounds(x: &[f64]) -> Value {
    assert_eq!(x.len(), 31);
    let mut s = x.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":x.iter().filter(|v|**v<1.).count(),"ratios":x,"nominal_marginal_coverage":0.9705506265163422})
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let jobs = read(r("jobs.json"));
    let mut results = BTreeMap::new();
    let mut input_hashes = BTreeMap::new();
    let mut output_hashes = BTreeMap::new();
    let mut stats = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut total = 0u64;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let log = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = log
            .lines()
            .filter_map(|s| s.strip_prefix("RESULT "))
            .collect();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        for key in ["field", "mode", "topology", "passes", "worker_count"] {
            assert_eq!(v[key], j[key]);
        }
        assert_eq!(v["input_count"], 4096);
        assert_eq!(v["prime_passes"], 3);
        assert_eq!(v["timed_allocations"], 0);
        assert_eq!(v["Arkworks_verified"], true);
        assert_eq!(v["product_identity_verified"], true);
        let count = v["total_inversions"].as_u64().unwrap();
        assert_eq!(
            count,
            j["passes"].as_u64().unwrap() * 4096 * j["worker_count"].as_u64().unwrap()
        );
        total += count;
        let key = j["field"].as_str().unwrap();
        for (map, member) in [
            (&mut input_hashes, "input_sha256"),
            (&mut output_hashes, "output_sha256"),
        ] {
            let h = v[member].as_str().unwrap();
            if let Some(old) = map.insert(key.to_owned(), h.to_owned()) {
                assert_eq!(old, h);
            }
        }
        let mut tids = BTreeSet::new();
        let mut cpus = BTreeSet::new();
        for w in v["workers"].as_array().unwrap() {
            assert!(tids.insert(w["tid"].as_u64().unwrap()));
            assert!(cpus.insert(w["cpu"].as_u64().unwrap()));
            assert_eq!(w["output_sha256"], v["output_sha256"]);
        }
        assert_eq!(tids.len(), j["worker_count"].as_u64().unwrap() as usize);
        assert_eq!(
            cpus.into_iter().collect::<Vec<_>>(),
            v["cpus"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap())
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
                let n: f64 = f[0].trim().parse().expect("mustcount");
                let pct: f64 = f[4].trim().parse().unwrap();
                assert!(pct > 0.);
                events.insert(
                    f[2].to_owned(),
                    json!({"count":n,"per_inverse":n/count as f64,"running_percent":pct}),
                );
            }
            assert_eq!(events.len(), 4);
            let ipc = events["instructions:u"]["count"].as_f64().unwrap()
                / events["cycles:u"]["count"].as_f64().unwrap();
            stats.insert(name.to_owned(), json!({"events":events,"IPC":ipc}));
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
                let w: u64 = f[1].parse().unwrap();
                pids.insert(p);
                observed.insert(t);
                samples += 1;
                period += w;
                let sym = format!("{:#}", rustc_demangle::demangle(&f[3..].join(" ")));
                *symbols.entry(sym).or_default() += w;
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            let sampled: BTreeSet<_> = observed.intersection(&tids).copied().collect();
            profiles.insert(name.to_owned(),json!({"samples":samples,"period":period,"pids":pids,"sampled_tids":observed,"sampled_worker_tids":sampled,"symbols":symbols,"lost":0,"raw_perf_sha256":hash(r("profiles").join(name).join("perf.data"))}));
        }
        results.insert(name.to_owned(), v);
    }
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let (mut calibration, mut nonreg, mut gain) = (true, true, true);
    for field in ["fq", "fr"] {
        for t in ["solo", "physical16", "logical32"] {
            for mode in 0..3 {
                let mut measures = BTreeMap::new();
                for metric in [
                    "wall_ns_per_inverse_wave",
                    "throughput_ns_per_inverse",
                    "thread_cpu_ns_per_inverse",
                ] {
                    let v: Vec<_> = (0..31)
                        .map(|b| {
                            results[&format!("b{b}-{field}-{t}-m{mode}")][metric]
                                .as_f64()
                                .unwrap()
                        })
                        .collect();
                    measures.insert(metric, json!({"median":median(&v),"process_values":v}));
                }
                cells.insert(format!("{field}/{t}/m{mode}"), json!(measures));
            }
            for (top, base) in [(1, 0), (2, 0), (2, 1)] {
                let v: Vec<_> = (0..31)
                    .map(|b| {
                        results[&format!("b{b}-{field}-{t}-m{top}")]["wall_ns_per_inverse_wave"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{b}-{field}-{t}-m{base}")]
                                ["wall_ns_per_inverse_wave"]
                                .as_f64()
                                .unwrap()
                    })
                    .collect();
                let p = bounds(&v);
                if top == 1 {
                    calibration &= p["low"].as_f64().unwrap() >= 1. / 1.03
                        && p["high"].as_f64().unwrap() <= 1.03;
                } else {
                    nonreg &= p["high"].as_f64().unwrap() <= 1.02;
                    if field == "fq" && t == "logical32" {
                        gain &=
                            p["high"].as_f64().unwrap() < 1. && p["wins"].as_u64().unwrap() >= 21;
                    }
                }
                pairs.insert(format!("{field}/{t}/{top}-over-{base}"), p);
            }
        }
    }
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    save(
        r("analysis.json"),
        &json!({"scope":"complete scalar inversions in externalprototype; no nativeproverintegration/corebuild/actualpointcaller/proofgain","cells":cells,"pairs":pairs,"gates":{"calibration":calibration,"all_contexts_nonregression":nonreg,"Fq_logical32_gain":gain,"prototype_gate_passed":calibration&&nonreg&&gain},"stat":stats,"profiles":profiles,"results":results,"timed_inverse_calls":total,"captures":566,"latency_processes":558,"checked_tests":10,"optimized_tests":10,"new_distinct_pair_cases":65616,"new_distinct_row_cases":131232,"production172_unchanged":true,"adopted":false,"new_SDK_or_Gnark_proofs":0,"independent_proof_count":13082,"goal_active_original8_achieved":0,"final_pins_verified":pins.as_object().unwrap().len()}),
    );
    for (k, v) in pairs {
        println!(
            "{k}:{} [{},{}],{}/31",
            v["median"], v["low"], v["high"], v["wins"]
        );
    }
    println!(
        "calibration={calibration};nonreg={nonreg};gain={gain};prototypepass={}",
        calibration && nonreg && gain
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!("mode"),
    }
}
