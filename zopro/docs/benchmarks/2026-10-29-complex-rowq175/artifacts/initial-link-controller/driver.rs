use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/complex-rowq175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
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
fn verify(pins: &BTreeMap<String, String>) {
    for (p, h) in pins {
        assert_eq!(hash(p), *h, "{p}");
    }
}
fn label(m: usize) -> &'static str {
    ["frozen", "off", "on"][m]
}
fn contexts() -> Vec<(&'static str, usize, &'static str)> {
    let mut v = vec![];
    for (k, n) in [
        ("g2-borrowed-signed", 200),
        ("g2-borrowed-signed", 352),
        ("g1-borrowed-signed", 200),
    ] {
        for t in ["solo", "physical16", "logical32"] {
            v.push((k, n, t));
        }
    }
    v
}
fn env() -> Value {
    json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","ZOPRO_CPU_PACKED":"1"})
}
fn link() {
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    assert!(!r("link-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    verify(&pins);
    let mut steps = vec![];
    let mut tests = BTreeMap::new();
    let frozen = read(r("inherited-frozen-tests.json"));
    assert_eq!(frozen["passed_tests"].as_array().unwrap().len(), 128);
    tests.insert("frozen", 128usize);
    for name in ["off", "on"] {
        let core = read(r(format!("native/core-{name}/tests.json")));
        assert_eq!(core["unfiltered"], true);
        assert_eq!(core["passed_tests"].as_array().unwrap().len(), 128);
        tests.insert(name, 128usize);
        pin(&mut pins, r(format!("native/core-{name}/tests.json")));
        let a = read(r(format!("artifacts-{name}.json")));
        assert_eq!(
            a["rustflags"]
                .as_str()
                .unwrap()
                .contains("--cfg zopro_complex_rowq"),
            name == "on"
        );
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
        pin(&mut pins, r(format!("artifacts-{name}.json")));
        let mut argv = vec!["rustc".to_owned()];
        argv.extend(
            a["rustflags"]
                .as_str()
                .unwrap()
                .split_whitespace()
                .map(str::to_owned),
        );
        let dependency = Path::new(a["libraries"]["zopro_core"].as_str().unwrap())
            .parent()
            .unwrap();
        argv.extend([
            "--edition=2021".into(),
            "-O".into(),
            r("probe.rs").display().to_string(),
            "-o".into(),
            r(format!("{name}-probe")).display().to_string(),
            "-L".into(),
            format!("dependency={}", dependency.display()),
        ]);
        for lib in [
            "zopro_core",
            "zopro_bn254",
            "rayon",
            "libc",
            "serde_json",
            "sha2",
        ] {
            argv.extend([
                "--extern".into(),
                format!("{lib}={}", a["libraries"][lib].as_str().unwrap()),
            ]);
        }
        steps.push(json!({"kind":"command","name":format!("link-{name}"),"argv":argv,"timeout_seconds":300}));
    }
    let field = read(r("native/field-on/tests.json"));
    assert_eq!(field["passed_tests"].as_array().unwrap().len(), 2);
    pin(&mut pins, r("native/field-on/tests.json"));
    let log = fs::read_to_string(r("native/field-on/stdout.log")).unwrap();
    assert!(log.contains("2 passed; 0 failed; 33 filtered out"));
    pin(&mut pins, r("native/field-on/stdout.log"));
    pin(&mut pins, r("native/field-on/stderr.log"));
    let qual = json!({"new_field_tests":2,"new_Arkworks_and_exact_raw_output_lanes":690176,"new_complex_model":read(r("model.json")),"unchanged33field_tests_inherited":true,"frozen128core_tests_inherited_byte_exact":true,"fresh_core_tests_each_mode":128,"full_SDK_qualification":false});
    save(r("field-qualification.json"), &qual);
    save(r("native-tests.json"), &json!(tests));
    for f in [
        "field-qualification.json",
        "native-tests.json",
        "driver.rs",
        "driver",
        "make-driver.rs",
        "native/result.json",
    ] {
        pin(&mut pins, r(f));
    }
    save(r("link-pins.json"), &json!(pins));
    save(
        r("link-plan.json"),
        &json!({"version":1,"source_pins":r("link-pins.json"),"steps":steps}),
    );
    println!("Prepared2freshprobe links; frozenbinary/tests reusedexact; newfield690176 +128coreeachmode");
}
fn prepare() {
    assert_eq!(read(r("linked/result.json"))["status"], "complete");
    assert!(!r("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("link-pins.json"))).unwrap();
    verify(&pins);
    for f in ["link-plan.json", "link-pins.json", "linked/result.json"] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, X);
    let mut code = vec![];
    for n in ["frozen", "off", "on"] {
        let binary = r(format!("{n}-probe"));
        pin(&mut pins, &binary);
        let o = Command::new("nm")
            .args(["--defined-only"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let symbols = String::from_utf8(o.stdout).unwrap();
        let mut found = 0;
        for (i, line) in symbols
            .lines()
            .filter(|l| l.contains("g2_borrowed") && l.contains("apply"))
            .enumerate()
        {
            let symbol = line.split_whitespace().last().unwrap();
            let o = Command::new("objdump")
                .args(["-d", "--no-show-raw-insn"])
                .arg(format!("--disassemble={symbol}"))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(o.status.success());
            let text = String::from_utf8(o.stdout).unwrap();
            let count = text.matches("vpmadd52").count();
            if count == 0 {
                continue;
            }
            let p = r(format!("codegen-{n}-{i}.asm.txt"));
            fs::write(&p, &text).unwrap();
            pin(&mut pins, &p);
            code.push(json!({"variant":n,"symbol":symbol,"ifma_sites":count,"bytes":text.len(),"path":p,"sha256":hash(&p)}));
            found += 1;
        }
        assert!(found > 0, "no actual IFMA borrowedG2 symbol in {n}");
    }
    save(
        r("codegen.json"),
        &json!({"actual_borrowed_G2":code,"static_sites_not_executed_work":true}),
    );
    pin(&mut pins, r("codegen.json"));
    let mut topology = BTreeMap::new();
    for cpu in 0..32 {
        let p = format!("/sys/devices/system/cpu/cpu{cpu}/topology");
        topology.insert(cpu,json!({"core_id":fs::read_to_string(format!("{p}/core_id")).unwrap().trim(),"siblings":fs::read_to_string(format!("{p}/thread_siblings_list")).unwrap().trim()}));
    }
    assert_eq!(topology[&2]["siblings"], "2,18");
    assert_eq!(
        (0..16)
            .map(|n| topology[&n]["core_id"].as_str().unwrap())
            .collect::<BTreeSet<_>>()
            .len(),
        16
    );
    save(r("topology.json"), &json!(topology));
    pin(&mut pins, r("topology.json"));
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
        fs::write(r(format!("{name}.txt")), o.stdout).unwrap();
        pin(&mut pins, r(format!("{name}.txt")));
    }
    let o = Command::new("rustup")
        .args(["which", "rustc"])
        .output()
        .unwrap();
    assert!(o.status.success());
    pin(&mut pins, String::from_utf8(o.stdout).unwrap().trim());
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   k: &str,
                   n: usize,
                   t: &str,
                   m: usize,
                   rounds: usize| {
        let normal = vec![
            r(format!("{}-probe", label(m))).display().to_string(),
            k.into(),
            n.to_string(),
            m.to_string(),
            t.into(),
            rounds.to_string(),
        ];
        let mut environment = env();
        let argv = if phase == "profile" {
            let d = r("profiles").join(&name);
            fs::create_dir_all(&d).unwrap();
            assert!(Command::new("mkfifo")
                .arg(d.join("ctl"))
                .arg(d.join("ack"))
                .status()
                .unwrap()
                .success());
            environment["ZOPRO_BENCH_PERF_CTL"] = json!(d.join("ctl"));
            environment["ZOPRO_BENCH_PERF_ACK"] = json!(d.join("ack"));
            let mut a: Vec<String> = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in environment.as_object().unwrap() {
                a.push(format!("{k}={}", v.as_str().unwrap()));
            }
            a.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "-e".into(),
                "cycles:u".into(),
                "-F".into(),
                "199".into(),
                "-m".into(),
                "512".into(),
                "-o".into(),
                d.join("perf.data").display().to_string(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    d.join("ctl").display(),
                    d.join("ack").display()
                ),
                "--".into(),
            ]);
            a.extend(normal);
            a
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":300}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"kind":k,"size":n,"topology":t,"mode":m,"rounds":rounds}));
    };
    for (k, n, t) in contexts() {
        for m in 0..3 {
            add(
                format!("pre-{k}-{n}-{t}-m{m}"),
                "preflight",
                0,
                k,
                n,
                t,
                m,
                32,
            );
        }
    }
    for b in 0..31 {
        let mut c = contexts();
        c.rotate_left(b % 9);
        if b % 2 != 0 {
            c.reverse();
        }
        for (i, (k, n, t)) in c.into_iter().enumerate() {
            let mut modes = vec![0, 1, 2];
            modes.rotate_left((b + i) % 3);
            if b % 2 != 0 {
                modes.reverse();
            }
            for m in modes {
                add(
                    format!("b{b}-{k}-{n}-{t}-m{m}"),
                    "latency",
                    b,
                    k,
                    n,
                    t,
                    m,
                    8192,
                );
            }
        }
    }
    for t in ["solo", "logical32"] {
        for m in [1, 2] {
            let name = format!("profile-g2-200-{t}-m{m}");
            add(
                name.clone(),
                "profile",
                0,
                "g2-borrowed-signed",
                200,
                t,
                m,
                262144,
            );
            profiles.push(name);
        }
    }
    drop(add);
    assert_eq!(jobs.len(), 868);
    for name in profiles {
        let p = r("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 880);
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared 27 preflights,837 latency processes,4 warm profiles,12 postcapture steps; {} pins",pins.len());
}
fn bounds(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    json!({"median":x[15],"low":x[9],"high":x[21],"wins":v.iter().filter(|v|**v<1.).count(),"ratios":v})
}
fn collect() -> (BTreeMap<String, Value>, Value) {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let jobs = read(r("jobs.json"));
    let mut rows = BTreeMap::new();
    let mut seen = BTreeMap::new();
    let mut profiles = vec![];
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let out = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let parsed: Vec<_> = out
            .lines()
            .filter_map(|l| l.strip_prefix("RESULT "))
            .collect();
        assert_eq!(parsed.len(), 1);
        let v: Value = serde_json::from_str(parsed[0]).unwrap();
        assert_eq!(v["internally_verified"], true);
        assert_eq!(v["timed_allocations"], 0);
        for key in ["kind", "size", "topology", "mode"] {
            assert_eq!(v[key], j[key]);
        }
        assert_eq!(v["iterations_per_worker"], j["rounds"]);
        assert_eq!(v["SDK_requests"], 0);
        let key = format!(
            "{}/{}/{}/{}",
            j["kind"], j["size"], j["rounds"], j["topology"]
        );
        let signature = json!([
            v["output_sha256"],
            v["capacity_bytes_per_worker"],
            v["fixture_reference_bytes_per_worker"]
        ]);
        if let Some(old) = seen.insert(key, signature.clone()) {
            assert_eq!(old, signature);
        }
        if j["phase"] == "profile" {
            let p = r("profiles").join(name).join("perf.data");
            let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
            assert!(!loss.contains("PERF_RECORD_LOST"));
            let mut symbols: BTreeMap<String, (u64, u64)> = BTreeMap::new();
            let mut pids = BTreeSet::new();
            let mut samples = 0u64;
            for l in text
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
            {
                let c: Vec<_> = l.split_whitespace().collect();
                assert!(c.len() >= 4);
                let (pid, tid) = c[0].split_once('/').unwrap();
                let pid: u32 = pid.parse().unwrap();
                let _tid: u32 = tid.parse().unwrap();
                let period: u64 = c[1].parse().unwrap();
                pids.insert(pid);
                samples += 1;
                let key = c[3..].join(" ");
                let entry = symbols.entry(key).or_default();
                entry.0 += 1;
                entry.1 += period;
            }
            assert!(samples > 0);
            assert_eq!(pids.len(), 1);
            let mut hot: Vec<_> = symbols
                .into_iter()
                .map(|(s, (n, p))| json!({"symbol":s,"samples":n,"period":p}))
                .collect();
            hot.sort_by_key(|v| std::cmp::Reverse(v["period"].as_u64().unwrap()));
            profiles.push(json!({"name":name,"path":p,"sha256":hash(&p),"bytes":fs::metadata(&p).unwrap().len(),"samples":samples,"lost":0,"sampled_PIDs":pids,"top_symbols":hot.into_iter().take(25).collect::<Vec<_>>(),"FIFO_gated_after_priming_before_oracle":true}));
        }
        rows.insert(name.into(), v);
    }
    assert_eq!(rows.len(), 868);
    (rows, json!(profiles))
}
fn analyze() {
    let (rows, profiles) = collect();
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let (mut calibration, mut nonregression, mut individual_gain) = (true, true, true);
    let mut capacities = BTreeMap::new();
    for (k, n, t) in contexts() {
        for m in 0..3 {
            let mut metrics = BTreeMap::new();
            for metric in [
                "wall_ns_per_wave",
                "throughput_ns_per_call",
                "mean_worker_ns_per_batch",
                "thread_cpu_ns_per_call",
            ] {
                let v: Vec<_> = (0..31)
                    .map(|b| {
                        rows[&format!("b{b}-{k}-{n}-{t}-m{m}")][metric]
                            .as_f64()
                            .unwrap()
                    })
                    .collect();
                let mut x = v.clone();
                x.sort_by(f64::total_cmp);
                metrics.insert(metric, json!({"median":x[15],"values":v}));
            }
            cells.insert(format!("{k}/{n}/{t}/m{m}"), json!(metrics));
            let v = &rows[&format!("b0-{k}-{n}-{t}-m{m}")];
            capacities.insert(format!("{k}/{n}/{t}/m{m}"),json!({"scratch_bytes_per_worker":v["capacity_bytes_per_worker"],"fixture_reference_bytes_per_worker":v["fixture_reference_bytes_per_worker"]}));
        }
        for (a, b) in [(1, 0), (2, 0), (2, 1)] {
            let v: Vec<_> = (0..31)
                .map(|block| {
                    rows[&format!("b{block}-{k}-{n}-{t}-m{a}")]["wall_ns_per_wave"]
                        .as_f64()
                        .unwrap()
                        / rows[&format!("b{block}-{k}-{n}-{t}-m{b}")]["wall_ns_per_wave"]
                            .as_f64()
                            .unwrap()
                })
                .collect();
            let p = bounds(&v);
            let high = p["high"].as_f64().unwrap();
            if a == 1 {
                calibration &= p["low"].as_f64().unwrap() >= 1. / 1.03 && high <= 1.03;
            } else {
                nonregression &= high <= 1.03;
                if k == "g2-borrowed-signed" && t == "logical32" {
                    individual_gain &= high < 1. && p["wins"].as_u64().unwrap() >= 21;
                }
            }
            pairs.insert(format!("{k}/{n}/{t}/{a}-over-{b}"), p);
        }
    }
    let mut aggregate = BTreeMap::new();
    let mut aggregate_gain = true;
    for base in [0, 1] {
        let v: Vec<_> = (0..31)
            .map(|b| {
                let sum: f64 = [200, 352]
                    .into_iter()
                    .map(|n| {
                        (rows[&format!("b{b}-g2-borrowed-signed-{n}-logical32-m2")]
                            ["wall_ns_per_wave"]
                            .as_f64()
                            .unwrap()
                            / rows[&format!("b{b}-g2-borrowed-signed-{n}-logical32-m{base}")]
                                ["wall_ns_per_wave"]
                                .as_f64()
                                .unwrap())
                        .ln()
                    })
                    .sum();
                (sum / 2.).exp()
            })
            .collect();
        let p = bounds(&v);
        aggregate_gain &= p["high"].as_f64().unwrap() < 1. && p["wins"].as_u64().unwrap() >= 21;
        aggregate.insert(format!("on-over-{}", label(base)), p);
    }
    let pins: BTreeMap<String, String> = serde_json::from_value(read(r("pins.json"))).unwrap();
    verify(&pins);
    let passed = calibration && nonregression && individual_gain && aggregate_gain;
    let v = json!({"scope":"current175 native actual borrowed caller; not SDK latency","gates":{"calibration":calibration,"nonregression":nonregression,"individual_logical32_G2_gain":individual_gain,"aggregate_logical32_G2_gain":aggregate_gain,"caller_passed":passed},"cells":cells,"pairs":pairs,"logical32_G2_aggregate":aggregate,"capacities":capacities,"profiles":profiles,"captures":868,"preflight_processes":27,"latency_processes":837,"profile_processes":4,"workflow_steps":880,"native_core_tests":read(r("native-tests.json")),"new_field_qualification":read(r("field-qualification.json")),"timed_allocations_all_zero":true,"all_output_capacity_signatures_equal":true,"final_pins_verified":pins.len(),"SDK_requests":0,"independent_proof_ledger":18076,"adopted":false,"production_changed":false,"goal_active_unmet":true,"goal_original8_achieved":0});
    save(r("analysis.json"), &v);
    save(r("observations.json"), &json!(rows));
    println!("{}", v["gates"]);
    for (k, v) in aggregate {
        println!(
            "{k} median={} bounds=[{},{}] wins={}",
            v["median"], v["low"], v["high"], v["wins"]
        );
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("link") => link(),
        Some("prepare") => prepare(),
        Some("analyze") => analyze(),
        _ => panic!("link|prepare|analyze"),
    }
}
