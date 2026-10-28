use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss";
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
fn label(mode: usize) -> &'static str {
    ["frozen", "off", "on"][mode]
}
fn count(t: &str) -> usize {
    match t {
        "solo" => 1,
        "physical16" => 16,
        "logical32" => 32,
        _ => panic!("topology"),
    }
}
fn contexts() -> Vec<(&'static str, usize)> {
    vec![
        ("g2-prefix", 32),
        ("g2-prefix", 200),
        ("g1-borrowed-signed", 200),
    ]
}
fn link() {
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    assert!(!r("link-plan.json").exists());
    for name in ["off", "on"] {
        let t = fs::read_to_string(r(format!("native/core-{name}/stdout.log"))).unwrap();
        assert!(t.contains("test result: ok. 123 passed; 0 failed"));
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for p in [
        "make-native.rs",
        "make-native",
        "native-prefix.rs",
        "source-audit.json",
        "oracle.rs",
    ] {
        pin(&mut pins, r(p));
    }
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let frozen =
        read("/home/ubuntu/src/zopro-perf-work/direct-ref-pack/native/frozen-artifacts.json");
    let macros =
        read("/home/ubuntu/src/zopro-perf-work/direct-ref-pack/native/frozen-proc-macros.json");
    for a in [&frozen, &macros] {
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
    }
    save(r("artifacts-frozen.json"), &frozen);
    pin(&mut pins, r("artifacts-frozen.json"));
    for p in [
        "probe.rs",
        "perf.rs",
        "collect.rs",
        "driver.rs",
        "driver",
        "make-native.rs",
        "make-native",
        "native/result.json",
    ] {
        pin(&mut pins, r(p));
    }
    let mut steps = vec![];
    for name in ["frozen", "off", "on"] {
        let a = read(r(format!("artifacts-{name}.json")));
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
        pin(&mut pins, r(format!("artifacts-{name}.json")));
        let flags = a["rustflags"].as_str().unwrap();
        assert_eq!(flags.contains("zopro_g2_reduced_gauss"), name == "on");
        assert!(!flags.contains("zopro_g1_weighted_anchor"));
        let dir = if name == "frozen" {
            "/home/ubuntu/src/zopro-perf-work/direct-ref-pack/frozen-libraries/frozen".to_owned()
        } else {
            r(format!("frozen/{name}")).display().to_string()
        };
        for test in [true, false] {
            let out = format!("{name}-probe{}", if test { "-tests" } else { "" });
            let mut argv = vec!["rustc".to_owned()];
            argv.extend(flags.split_whitespace().map(str::to_owned));
            argv.extend([
                "--edition=2021".into(),
                "-O".into(),
                r("probe.rs").display().to_string(),
                "-o".into(),
                r(&out).display().to_string(),
                "-L".into(),
                format!("dependency={dir}"),
            ]);
            for n in [
                "zopro_core",
                "zopro_bn254",
                "rayon",
                "libc",
                "serde_json",
                "sha2",
            ] {
                argv.extend([
                    "--extern".into(),
                    format!("{n}={}", a["libraries"][n].as_str().unwrap()),
                ]);
            }
            if test {
                argv.push("--test".into());
            }
            save(r(format!("link-{out}.json")), &json!(argv));
            steps.push(json!({"kind":"command","name":format!("link-{out}"),"argv":argv,"timeout_seconds":300}));
            if test {
                steps.push(json!({"kind":"command","name":format!("harness-{name}"),"argv":[r(&out),PathBuf::from("--test-threads=1")],"environment":env(),"required_tests":["tests::independent_workers_preserve_oracles_affinity_barriers_and_reuse"],"unfiltered":true,"timeout_seconds":300}));
            }
        }
    }
    for n in ["zopro_bn254", "zopro_core"] {
        assert_ne!(
            hash(
                read(r("artifacts-off.json"))["libraries"][n]
                    .as_str()
                    .unwrap()
            ),
            hash(
                read(r("artifacts-on.json"))["libraries"][n]
                    .as_str()
                    .unwrap()
            )
        );
    }
    save(r("link-pins.json"), &json!(pins));
    save(
        r("link-plan.json"),
        &json!({"version":1,"source_pins":r("link-pins.json"),"steps":steps}),
    );
    println!(
        "Prepared9frozen/OFF/ONlink/harnessteststeps;inheritedfieldqualification/core123x2PASS"
    );
}
fn prepare() {
    assert_eq!(read(r("linked/result.json"))["status"], "complete");
    assert!(!r("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("link-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for n in ["frozen", "off", "on"] {
        let log = fs::read_to_string(r(format!("linked/harness-{n}/stdout.log"))).unwrap();
        assert!(log.contains("test result: ok. 3 passed; 0 failed"));
        for suffix in ["", "-tests"] {
            pin(&mut pins, r(format!("{n}-probe{suffix}")));
        }
        pin(&mut pins, r(format!("linked/harness-{n}/stdout.log")));
    }
    for p in ["link-plan.json", "link-pins.json", "linked/result.json"] {
        pin(&mut pins, r(p));
    }
    pin(&mut pins, X);
    let mut code = vec![];
    for variant in ["off", "on"] {
        let binary = r(format!("{variant}-probe"));
        let out = Command::new("nm")
            .arg("--defined-only")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let symbols = String::from_utf8(out.stdout).unwrap();
        let mut count = 0;
        for (i, line) in symbols
            .lines()
            .filter(|s| s.contains("g2_prefix"))
            .enumerate()
        {
            let symbol = line.split_whitespace().last().unwrap();
            let name = format!("{:#}", rustc_demangle::demangle(symbol));
            let out = Command::new("objdump")
                .args(["-d", "--no-show-raw-insn"])
                .arg(format!("--disassemble={symbol}"))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            let ifma = text.matches("vpmadd52").count();
            assert!(ifma > 0);
            count += 1;
            let p = r(format!("codegen-{variant}-{i}.asm.txt"));
            fs::write(&p, &text).unwrap();
            pin(&mut pins, &p);
            code.push(json!({"variant":variant,"symbol":name,"ifma_sites":ifma,"instructions":text.lines().filter(|l|l.contains(':')&&l.split_once(':').unwrap().0.trim().chars().all(|c|c.is_ascii_hexdigit())).count(),"path":p}));
        }
        assert!(count > 0);
    }
    save(
        r("codegen.json"),
        &json!({"read_only_objdump":true,"actual_G2_prefix_symbols":code,"static_counts_not_executed_work":true}),
    );
    pin(&mut pins, r("codegen.json"));
    let mut topology = BTreeMap::new();
    for cpu in 0..32 {
        let path = format!("/sys/devices/system/cpu/cpu{cpu}/topology");
        topology.insert(cpu,json!({"core_id":fs::read_to_string(format!("{path}/core_id")).unwrap().trim(),"siblings":fs::read_to_string(format!("{path}/thread_siblings_list")).unwrap().trim()}));
    }
    assert_eq!(topology[&2]["siblings"], "2,18");
    let ids: BTreeSet<_> = (0..16)
        .map(|c| topology[&c]["core_id"].as_str().unwrap())
        .collect();
    assert_eq!(ids.len(), 16);
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
            r(format!("{}-probe", label(mode))).display().to_string(),
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
        jobs.push(json!({"name":name,"phase":phase,"block":block,"kind":kind,"size":size,"mode":mode,"topology":topology,"worker_count":count(topology),"rounds":rounds}));
    };
    for b in 0..31 {
        let mut c = vec![];
        for (kind, n) in contexts() {
            for t in ["solo", "physical16", "logical32"] {
                c.push((kind, n, t));
            }
        }
        c.rotate_left(b % 9);
        if b % 2 != 0 {
            c.reverse();
        }
        for (i, (kind, n, t)) in c.into_iter().enumerate() {
            let mut modes = vec![0, 1, 2];
            modes.rotate_left((b + i) % 3);
            if b % 2 != 0 {
                modes.reverse();
            }
            for m in modes {
                add(
                    format!("b{b}-{kind}-{n}-{t}-m{m}"),
                    "latency",
                    b,
                    kind,
                    n,
                    m,
                    t,
                    8192,
                );
            }
        }
    }
    for phase in ["stat", "cycles"] {
        for (kind, n) in [("g1-borrowed-signed", 200), ("g2-prefix", 200)] {
            for m in [1, 2] {
                let name = format!("{phase}-{kind}-{n}-m{m}");
                add(name.clone(), phase, 0, kind, n, m, "logical32", 262144);
                if phase == "cycles" {
                    profiles.push(name);
                }
            }
        }
    }
    assert_eq!(jobs.len(), 845);
    for name in profiles {
        let p = r("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 857);
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!(
        "Prepared837latency/4stat/4cycles/12decode;9contexts31blocks;frozenOFFONindependentlibs"
    );
}
include!("collect.rs");
fn bounds(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    json!({"median":x[15],"low":x[9],"high":x[21],"wins":v.iter().filter(|v|**v<1.).count(),"ratios":v,"nominal_marginal_coverage":0.9705506265163422})
}
fn analyze() {
    let (results, stat, profiles, caps, total) = collect();
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let (mut calib, mut nonreg) = (true, true);
    for (kind, n) in contexts() {
        for t in ["solo", "physical16", "logical32"] {
            for m in 0..3 {
                let mut measures = BTreeMap::new();
                for metric in [
                    "wall_ns_per_wave",
                    "throughput_ns_per_call",
                    "mean_worker_ns_per_batch",
                    "thread_cpu_ns_per_call",
                ] {
                    let v: Vec<_> = (0..31)
                        .map(|b| {
                            results[&format!("b{b}-{kind}-{n}-{t}-m{m}")][metric]
                                .as_f64()
                                .unwrap()
                        })
                        .collect();
                    let mut x = v.clone();
                    x.sort_by(f64::total_cmp);
                    measures.insert(metric, json!({"median":x[15],"process_values":v}));
                }
                cells.insert(format!("{kind}/{n}/{t}/m{m}"), json!(measures));
            }
            for (a, b) in [(1, 0), (2, 0), (2, 1)] {
                let v: Vec<_> = (0..31)
                    .map(|block| {
                        results[&format!("b{block}-{kind}-{n}-{t}-m{a}")]["wall_ns_per_wave"]
                            .as_f64()
                            .unwrap()
                            / results[&format!("b{block}-{kind}-{n}-{t}-m{b}")]["wall_ns_per_wave"]
                                .as_f64()
                                .unwrap()
                    })
                    .collect();
                let p = bounds(&v);
                if a == 1 {
                    calib &= p["low"].as_f64().unwrap() >= 1. / 1.03
                        && p["high"].as_f64().unwrap() <= 1.03;
                } else {
                    nonreg &= p["high"].as_f64().unwrap() <= 1.02;
                }
                pairs.insert(format!("{kind}/{n}/{t}/{a}-over-{b}"), p);
            }
        }
    }
    let mut aggregates = BTreeMap::new();
    let mut gain = true;
    for base in [0, 1] {
        let v: Vec<_> = (0..31)
            .map(|b| {
                let mut sum = 0.;
                for (kind, n) in [("g2-prefix", 200)] {
                    sum += (results[&format!("b{b}-{kind}-{n}-logical32-m2")]["wall_ns_per_wave"]
                        .as_f64()
                        .unwrap()
                        / results[&format!("b{b}-{kind}-{n}-logical32-m{base}")]
                            ["wall_ns_per_wave"]
                            .as_f64()
                            .unwrap())
                    .ln();
                }
                sum.exp()
            })
            .collect();
        let p = bounds(&v);
        gain &= p["high"].as_f64().unwrap() < 1. && p["wins"].as_u64().unwrap() >= 21;
        aggregates.insert(format!("on-over-{}", label(base)), p);
    }
    let pins = read(r("pins-v2.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    save(
        r("analysis.json"),
        &json!({"scope":"native actual-caller screen, independent frozen/freshOFF/ON, synthetic ordinary points at real batch sizes; no proof-speed claim","cells":cells,"pairs":pairs,"logical32_aggregate":aggregates,"gates":{"calibration":calib,"nonregression":nonreg,"logical32_gain":gain,"component_gate_passed":calib&&nonreg&&gain},"stat":stat,"profiles":profiles,"results":results,"capacities_per_worker":caps,"timed_kernel_calls":total,"captures":845,"latency_processes":837,"native_field_tests_each":0,"native_field_configurations":0,"native_core_tests_each":123,"native_core_configurations":2,"harness_tests_each":3,"harness_configurations":3,"final_pins_verified":pins.as_object().unwrap().len(),"new_SDK_or_Gnark_proofs":0,"independent_proof_count":13181,"production_changed":false,"adopted":false,"goal_active_original8_achieved":0}),
    );
    for (k, v) in aggregates {
        println!("{k}:{v}");
    }
    println!(
        "calibration={calib};nonreg={nonreg};gain={gain};componentpass={}",
        calib && nonreg && gain
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "link" => link(),
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!("mode"),
    }
}
