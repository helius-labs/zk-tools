use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175";
const VS: [&str; 3] = ["frozen", "off", "on"];
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn env() -> Value {
    json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","ZOPRO_CPU_PACKED":"1"})
}
fn contexts() -> Vec<Value> {
    let mut v = vec![];
    for k in ["g1-owned", "g1-borrowed-signed", "g2-borrowed-signed"] {
        for n in [200, 352] {
            for t in ["solo", "physical16", "logical32"] {
                v.push(json!({"kind":k,"size":n,"topology":t,"name":format!("{k}-{n}-{t}")}));
            }
        }
    }
    v
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (f, h) in &pins {
        assert_eq!(hash(f), *h);
    }
    let mut ids = BTreeMap::new();
    for v in VS {
        let p = read(r(format!("probe-{v}.json")));
        assert_eq!(hash(p["binary"].as_str().unwrap()), p["sha256"]);
        pins.insert(
            p["binary"].as_str().unwrap().into(),
            p["sha256"].as_str().unwrap().into(),
        );
        for (f, h) in p["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            pins.insert(f.into(), h.as_str().unwrap().into());
        }
        pins.insert(
            r(format!("probe-{v}.json")).display().to_string(),
            hash(r(format!("probe-{v}.json"))),
        );
        ids.insert(v, p);
    }
    for (v, field, core) in [("off", 33, 128), ("on", 35, 128)] {
        let base_dir = "native";
        for (s, n) in [("field", field), ("core", core)] {
            let info = read(r(format!("{base_dir}/{s}-{v}/tests.json")));
            assert_eq!(info["passed_tests"].as_array().unwrap().len(), n);
            assert_eq!(info["unfiltered"], true);
            for f in [
                "tests.json",
                "command.json",
                "exit.json",
                "stdout.log",
                "stderr.log",
            ] {
                let p = r(format!("{base_dir}/{s}-{v}/{f}"));
                pins.insert(p.display().to_string(), hash(p));
            }
        }
    }
    let mut code = vec![];
    fs::create_dir(r("codegen")).unwrap();
    for v in VS {
        let binary = PathBuf::from(ids[v]["binary"].as_str().unwrap());
        let out = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let symbols = String::from_utf8(out.stdout).unwrap();
        let mut hits = 0;
        for line in symbols.lines().filter(|l| {
            l.contains("g2_borrowed") && l.contains("apply") || l.contains("stream_g1_impl")
        }) {
            let sym = line.split_whitespace().last().unwrap();
            let out = Command::new("objdump")
                .args(["-d", "--no-show-raw-insn"])
                .arg(format!("--disassemble={sym}"))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            if !text.contains("vpmadd52") {
                continue;
            }
            let p = r(format!("codegen/{v}-{hits}.asm"));
            fs::write(&p, &text).unwrap();
            pins.insert(p.display().to_string(), hash(&p));
            code.push(json!({"variant":v,"nm":line,"assembly":p,"sha256":hash(&p),"static_IFMA_sites":text.matches("vpmadd52").count()}));
            hits += 1;
        }
        assert!(hits >= 2, "missingG1/G2 {v}");
    }
    save(r("codegen.json"), &json!(code));
    let cs = contexts();
    assert_eq!(cs.len(), 18);
    let mut steps = vec![];
    let mut jobs = vec![];
    let base = env();
    let mut push = |c: &Value, v: &str, b: usize, phase: &str| {
        let mode = VS.iter().position(|&x| x == v).unwrap();
        let name = if phase == "latency" {
            format!("b{b}-{}-{v}", c["name"].as_str().unwrap())
        } else {
            format!("{phase}-{}-{v}", c["name"].as_str().unwrap())
        };
        let rounds = if phase == "preflight" {
            32
        } else if phase == "profile" {
            262144
        } else {
            8192
        };
        let binary = PathBuf::from(ids[v]["binary"].as_str().unwrap());
        let args = vec![
            binary.display().to_string(),
            c["kind"].as_str().unwrap().into(),
            c["size"].as_u64().unwrap().to_string(),
            mode.to_string(),
            c["topology"].as_str().unwrap().into(),
            rounds.to_string(),
        ];
        let mut e = base.clone();
        let argv = if phase == "profile" {
            e["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
            e["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
            let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, val) in e.as_object().unwrap() {
                a.push(format!("{k}={}", val.as_str().unwrap()));
            }
            a.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "-e".into(),
                "cycles:u".into(),
                "-c".into(),
                "16000000".into(),
                "--timestamp".into(),
                "--sample-cpu".into(),
                "-m".into(),
                "512".into(),
                "-o".into(),
                r(format!("perf/{name}.data")).display().to_string(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    r("ctl").display(),
                    r("ack").display()
                ),
                "--".into(),
            ]);
            a.extend(args);
            a
        } else {
            args
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":base,"timeout_seconds":900}));
        jobs.push(json!({"name":name,"kind":c["kind"],"size":c["size"],"topology":c["topology"],"context":c["name"],"variant":v,"mode":mode,"phase":phase,"block":b,"rounds":rounds}));
    };
    for c in &cs {
        for v in VS {
            push(c, v, 0, "preflight");
        }
    }
    for b in 0..31 {
        let mut order: Vec<usize> = (0..18).collect();
        order.rotate_left(b % 18);
        if b % 2 != 0 {
            order.reverse();
        }
        for i in order {
            let mut vs = VS;
            vs.rotate_left((b + i) % 3);
            if (b + i) % 2 != 0 {
                vs.reverse();
            }
            for v in vs {
                push(&cs[i], v, b, "latency");
            }
        }
    }
    for c in cs
        .iter()
        .filter(|c| c["size"] == 200 && c["topology"] == "logical32" && c["kind"] != "g1-owned")
    {
        for v in ["off", "on"] {
            push(c, v, 0, "profile");
        }
    }
    drop(push);
    assert_eq!(jobs.len(), 1732);
    fs::create_dir(r("perf")).unwrap();
    for f in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());
    }
    let uid = String::from_utf8(Command::new("id").arg("-u").output().unwrap().stdout).unwrap();
    let gid = String::from_utf8(Command::new("id").arg("-g").output().unwrap().stdout).unwrap();
    for j in jobs.iter().filter(|j| j["phase"] == "profile") {
        let name = j["name"].as_str().unwrap();
        steps.push(json!({"kind":"command","name":format!("own-{name}"),"argv":["sudo","-n","chown",format!("{}:{}",uid.trim(),gid.trim()),r(format!("perf/{name}.data"))],"timeout_seconds":30}));
    }
    for j in jobs.iter().filter(|j| j["phase"] == "profile") {
        let name = j["name"].as_str().unwrap();
        let data = r(format!("perf/{name}.data"));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",data,"-F","pid,tid,period,ip,sym"],"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("events-{name}"),"argv":["perf","evlist","-v","-i",data],"timeout_seconds":30}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[r("capture-helper"),"loss",data],"timeout_seconds":180}));
    }
    assert_eq!(steps.len(), 1748);
    save(r("jobs.json"), &json!(jobs));
    save(r("contexts.json"), &json!(cs));
    for f in [
        "capture.rs",
        "capture-helper",
        "native/result.json",
        "codegen.json",
        "make-capture.rs",
        "capture-design.md",
        "report.rs",
        "report-helper",
        "native-attempt0.log",
        "jobs.json",
        "contexts.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared1748steps/54preflight1674latency4profiles; sourceflags/tests/libs/codegen closed; ledger25231unchanged");
}
fn loss(data: &str) {
    let mut c = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lost = vec![];
    let mut lines = 0;
    for line in BufReader::new(c.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        lines += 1;
        if line.contains("PERF_RECORD_LOST") {
            lost.push(line);
        }
    }
    assert!(c.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":lines,"lost":lost,"zero_lost":lost.is_empty()})
    );
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn rank(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut s = v.to_vec();
    assert!(s.iter().all(|v| v.is_finite() && *v > 0.));
    s.sort_by(f64::total_cmp);
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":v.iter().filter(|&&x|x<1.).count(),"ratios":v})
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (f, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut rows = BTreeMap::<String, Vec<Value>>::new();
    let mut signatures = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut calls = 0u64;
    let mut primes = 0u64;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let results = text
            .lines()
            .filter_map(|l| l.strip_prefix("RESULT "))
            .collect::<Vec<_>>();
        assert_eq!(results.len(), 1);
        let out: Value = serde_json::from_str(results[0]).unwrap();
        for key in ["kind", "size", "mode", "topology"] {
            assert_eq!(out[key], j[key]);
        }
        assert_eq!(out["iterations_per_worker"], j["rounds"]);
        assert_eq!(out["internally_verified"], true);
        assert_eq!(out["timed_allocations"], 0);
        assert_eq!(out["prime_iterations_per_worker"], 1024);
        let workers = if j["topology"] == "solo" {
            1
        } else if j["topology"] == "physical16" {
            16
        } else {
            32
        };
        assert_eq!(out["worker_count"], workers);
        calls += out["total_kernel_calls"].as_u64().unwrap();
        primes += workers * 3 * 1024;
        let signature = json!([
            out["output_sha256"],
            out["capacity_bytes_per_worker"],
            out["fixture_reference_bytes_per_worker"]
        ]);
        let key = format!(
            "{}/{}/{}/{}",
            j["kind"], j["size"], j["rounds"], j["topology"]
        );
        if let Some(old) = signatures.insert(key, signature.clone()) {
            assert_eq!(old, signature);
        }
        if j["phase"] == "latency" {
            rows.entry(format!(
                "{}/{}",
                j["context"].as_str().unwrap(),
                j["variant"].as_str().unwrap()
            ))
            .or_default()
            .push(json!({"block":j["block"],"name":name,"result":out}));
        } else if j["phase"] == "profile" {
            let loss = read(r(format!("runs/loss-{name}/stdout.log")));
            assert_eq!(loss["zero_lost"], true);
            let ev = fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();
            assert!(
                ev.contains("cycles:u")
                    && ev.contains("16000000")
                    && ev.contains("TIME")
                    && ev.contains("CPU")
            );
            let mut pids = BTreeSet::new();
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut samples = 0;
            for l in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let (pid, _) = f[0].split_once('/').unwrap();
                pids.insert(pid.to_string());
                *symbols.entry(f[3..].join(" ")).or_default() += f[1].parse::<u64>().unwrap();
                samples += 1;
            }
            assert_eq!(pids.len(), 1);
            assert!(samples >= 1000);
            let unknown: u64 = symbols
                .iter()
                .filter(|(name, _)| name.contains("unknown"))
                .map(|(_, count)| count)
                .sum();
            let total: u64 = symbols.values().sum();
            assert!(unknown as f64 / total as f64 <= 0.05);
            profiles.insert(name.to_string(),json!({"samples":samples,"PIDs":pids,"zero_lost":true,"symbol_periods":symbols,"raw_sha256":hash(r(format!("perf/{name}.data")))}));
        }
    }
    let mut cells = BTreeMap::new();
    for (k, v) in &rows {
        assert_eq!(v.len(), 31);
        for (b, r) in v.iter().enumerate() {
            assert_eq!(r["block"], b);
        }
        cells.insert(k.clone(),json!({"median_wave_ns":median(&v.iter().map(|r|r["result"]["wall_ns_per_wave"].as_f64().unwrap()).collect::<Vec<_>>()),"median_thread_CPU_ns":median(&v.iter().map(|r|r["result"]["thread_cpu_ns_per_call"].as_f64().unwrap()).collect::<Vec<_>>()),"blocks":v}));
    }
    let mut pairs = BTreeMap::new();
    let mut checks = BTreeMap::new();
    let cs = contexts();
    for c in &cs {
        let ctx = c["name"].as_str().unwrap();
        for (a, b) in [("off", "frozen"), ("on", "off"), ("on", "frozen")] {
            let ratios = (0..31)
                .map(|i| {
                    rows[&format!("{ctx}/{a}")][i]["result"]["wall_ns_per_wave"]
                        .as_f64()
                        .unwrap()
                        / rows[&format!("{ctx}/{b}")][i]["result"]["wall_ns_per_wave"]
                            .as_f64()
                            .unwrap()
                })
                .collect::<Vec<_>>();
            let v = rank(&ratios);
            let pass = if a == "off" {
                v["low"].as_f64().unwrap() >= 1. / 1.03 && v["high"].as_f64().unwrap() <= 1.03
            } else {
                v["high"].as_f64().unwrap() <= 1.03
            };
            checks.insert(format!("{ctx}/{a}/{b}"), pass);
            pairs.insert(format!("{ctx}/{a}/{b}"), v);
        }
    }
    let mut aggregates = BTreeMap::new();
    for group in ["all18", "logical32_6"] {
        let selected = cs
            .iter()
            .filter(|c| group == "all18" || c["topology"] == "logical32")
            .collect::<Vec<_>>();
        assert_eq!(selected.len(), if group == "all18" { 18 } else { 6 });
        for control in ["off", "frozen"] {
            let ratios = (0..31)
                .map(|b| {
                    (selected
                        .iter()
                        .map(|c| {
                            pairs[&format!("{}/on/{control}", c["name"].as_str().unwrap())]
                                ["ratios"][b]
                                .as_f64()
                                .unwrap()
                                .ln()
                        })
                        .sum::<f64>()
                        / selected.len() as f64)
                        .exp()
                })
                .collect::<Vec<_>>();
            let v = rank(&ratios);
            checks.insert(
                format!("{group}/on/{control}/gain"),
                v["high"].as_f64().unwrap() < 1. && v["wins"].as_u64().unwrap() >= 21,
            );
            aggregates.insert(format!("{group}/on/{control}"), v);
        }
    }
    let failed = checks
        .iter()
        .filter(|(_, p)| !**p)
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();
    let a = json!({"complete":true,"component_gate":failed.is_empty(),"failed_checks":failed,"checks":checks,"pairs":pairs,"aggregates":aggregates,"cells":cells,"profiles":profiles,"processes_checked":1732,"preflights":54,"latencies":1674,"profile_processes":4,"workflow_steps":1748,"measured_kernel_calls_including_profiles_and_preflights":calls,"priming_kernel_calls":primes,"total_kernel_calls":calls+primes,"timed_allocations_all_zero":true,"output_and_capacity_signatures_equal":true,"native_field_tests":{"off":33,"on":35},"native_core_tests":{"off":128,"on":128},"SDK_requests":0,"new_Gnark_exports":0,"proof_ledger":25231,"adopted":false,"production_changed":false,"goal_achieved":false,"scope":"actualG1owned/G1borrowed/G2borrowedkernels onsyntheticvalidgeometry; noSDKspeedclaim","inherited_M5":read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")["m5"]});
    save(r("analysis.json"), &a);
    println!(
        "Gate{} failed{}; total{}kernelcalls; no proofs/ledger25231",
        a["component_gate"], a["failed_checks"], a["total_kernel_calls"]
    );
    for (k, v) in cells {
        println!("{k} {:.6}us", v["median_wave_ns"].as_f64().unwrap() / 1000.);
    }
    for (k, v) in aggregates {
        println!(
            "{k} {} [{},{}]wins{}",
            v["median"], v["low"], v["high"], v["wins"]
        );
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("loss") => loss(&std::env::args().nth(2).unwrap()),
        _ => panic!(),
    }
}
