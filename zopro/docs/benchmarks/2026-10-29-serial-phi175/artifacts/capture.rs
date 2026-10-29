use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Read,BufRead},
    path::{Path, PathBuf},
    process::{Command,Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/serial-phi175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-preparation175";
const ACCEPTED: &str =
    "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
const ACCEPTED_HASH: &str = "20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
const VS: [&str; 3] = ["accept", "offset", "serial"];
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
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (f, h) in &pins {
        assert_eq!(hash(f), *h);
    }
    assert_eq!(hash(ACCEPTED), ACCEPTED_HASH);
    pin(&mut pins, ACCEPTED);
    let receipt = read(r("native/candidate/artifacts.json"));
    assert_eq!(
        receipt["binary_sha256"],
        hash(receipt["binary"].as_str().unwrap())
    );
    for (f, h) in receipt["input_pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
        pin(&mut pins, f);
    }
    assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
    assert_eq!(
        read(r("frozen-candidate.json"))["rustflags"],
        read(MANIFEST)["rustflags"]
    );
    let mut ids = BTreeMap::new();
    for v in VS {
        let binary = if v == "accept" {
            PathBuf::from(ACCEPTED)
        } else {
            r("native/candidate-sdk")
        };
        ids.insert(
            v,
            json!({"binary":binary,"sha256":hash(&binary),"probe":if v=="serial"{"1"}else{"0"}}),
        );
    }
    assert_eq!(ids["offset"]["binary"], ids["serial"]["binary"]);
    save(r("identities.json"), &json!(ids));
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{OLD}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 17811);
    for (f, h) in read(format!("{OLD}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 17830);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let mut cases=read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json").as_array().unwrap().clone();assert_eq!(cases.len(),9);
    let mut physical=cases[0].clone();physical["name"]=json!("1x2-w16");physical["workers"]=json!(16);physical["cpu_affinity"]=json!("0-15");cases.push(physical);let cases=json!(cases);assert_eq!(cases.as_array().unwrap().len(),10);save(r("cases.json"),&cases);
    let base=read(MANIFEST)["environment"].clone();
    let mut jobs = vec![];
    let mut steps = vec![];
    let verifier = |j: &Value| {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120})
    };
    for c in cases.as_array().unwrap() {
        for v in VS {
            let j = push(c, v, 0, true, &base, &ids, &mut jobs, &mut steps);
            steps.push(verifier(&j));
        }
    }
    for b in 0..7 {
        let mut order:Vec<usize>=(0..10).collect();
        order.rotate_left(b % 10);
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
                push(&cases[i], v, b, false, &base, &ids, &mut jobs, &mut steps);
            }
        }
    }
    assert_eq!(jobs.len(),240);
    for f in["ctl","ack"]{assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());}
    for c in cases.as_array().unwrap().iter().filter(|c|c["shape"]=="1x2"){for v in VS{diagnostic(c,v,"cycles",&base,&ids,&mut jobs,&mut steps);}}
    assert_eq!(jobs.len(),246);
    let owner=format!("{}:{}",output(&["id","-u"]).trim(),output(&["id","-g"]).trim());
    for j in jobs.iter().filter(|j|j["diagnostic"]==true){steps.push(json!({"kind":"command","name":format!("own-{}",j["name"].as_str().unwrap()),"argv":["sudo","-n","chown",owner,j["data"]],"timeout_seconds":120}));}

    for j in jobs.iter().filter(|j| j["preflight"] != true) {
        steps.push(verifier(j));
    }
    for j in jobs.iter().filter(|j|j["diagnostic"]==true){let name=j["name"].as_str().unwrap();steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["sudo","-n","perf","script","--header","--show-lost-events","-i",j["data"],"-F","pid,tid,period,ip,sym"],"timeout_seconds":180}));steps.push(json!({"kind":"command","name":format!("evlist-{name}"),"argv":["sudo","-n","perf","evlist","-v","-i",j["data"]],"timeout_seconds":120}));steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[r("capture-helper"),"loss",j["data"]],"timeout_seconds":180}));}
    assert_eq!(steps.len(),516);
    save(r("jobs.json"), &json!(jobs));
    for c in cases.as_array().unwrap() {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    for e in
        fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap()
    {
        pin(&mut pins, e.unwrap().path());
    }
    for f in [VERIFY, X, MANIFEST] {
        pin(&mut pins, f);
    }
    for f in [
        "make-capture.rs","history-audit.md","audit.rs","audit","code-audit.json","archive.rs","archive","state.rs","initial-code-audit/audit.rs","initial-code-audit/failure.json","capture.rs",
        "capture-helper",
        "design.md",
        "identities.json",
        "source-audit.json",
        "jobs.json",
        "cases.json",
        "prior-proof-sha256.json",
        "native/result.json",
        "native/candidate/artifacts.json",
        "frozen-candidate.json",
    ] {
        pin(&mut pins, r(f));
    }
    for e in fs::read_dir(r("codegen")).unwrap(){pin(&mut pins,e.unwrap().path());}
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    fs::write(
        r("host-before.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!("Prepared516steps/30preflight+210latency+6profiles;13824SDK/246freshGnark;17830->18076");
}
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
    env["ZOPRO_CPU_SERIAL_PHI_PROBE"] = ids[v]["probe"].clone();
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
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (f, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 17830);
    let mut inventory = BTreeMap::new();
    let mut cells = BTreeMap::<String, Vec<Value>>::new();
    let mut requests = 0;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let times = text
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| {
                serde_json::from_str::<Value>(l).unwrap()["ms"]
                    .as_f64()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(times.len() as u64, j["rounds"].as_u64().unwrap());
        assert!(times.iter().all(|x| x.is_finite() && *x > 0.));
        let s: Value = serde_json::from_str(
            text.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(s["workers"], j["case"]["workers"]);
        assert_eq!(s["rounds"], j["rounds"]);
        assert_eq!(s["priming"], 3);
        requests += times.len() + 3;
        let verified = fs::read_to_string(r(format!("runs/verify-{name}/stdout.log"))).unwrap();
        assert!(verified.contains("VERIFIED proof_sha256="));
        assert!(verified.contains(&format!(
            "image_sha256={}",
            j["case"]["image_sha256"].as_str().unwrap()
        )));
        for ext in ["proof", "public"] {
            let p = Path::new(j["directory"].as_str().unwrap()).join(format!("0.{ext}"));
            let h = hash(&p);
            if ext == "proof" {
                assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                assert!(prior.insert(h.clone()));
            } else {
                assert_eq!(h, j["case"]["public_sha256"].as_str().unwrap());
            }
            assert!(verified.contains(&format!("{ext}_sha256={h}")));
            inventory.insert(p.display().to_string(), h);
        }
        if j["preflight"] != true && j["diagnostic"] != true {
            let cpu = (s["user_cpu_ms"].as_f64().unwrap() + s["system_cpu_ms"].as_f64().unwrap())
                / times.len() as f64;
            let rss = s["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.;
            cells.entry(format!("{}/{}",j["case"]["name"].as_str().unwrap(),j["variant"].as_str().unwrap())).or_default().push(json!({"name":name,"block":j["block"],"median_ms":median(&times),"CPU_ms":cpu,"RSS_MiB":rss,"summary":s}));
        }
    }
    assert_eq!(requests, 13824);
    assert_eq!(prior.len(), 18076);
    assert_eq!(inventory.len(), 492);
    save(r("proof-inventory.json"), &json!(inventory));
    let mut summaries = BTreeMap::new();
    for (k, rows) in &cells {
        assert_eq!(rows.len(), 7);
        let collect = |f: &str| {
            rows.iter()
                .map(|r| r[f].as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        summaries.insert(k.clone(),json!({"median_ms":median(&collect("median_ms")),"CPU_ms":median(&collect("CPU_ms")),"median_RSS_MiB":median(&collect("RSS_MiB")),"max_RSS_MiB":collect("RSS_MiB").into_iter().fold(0.,f64::max),"blocks":rows}));
    }
    let mut pairs = BTreeMap::new();
    let mut checks = BTreeMap::new();
    let mut calibration = BTreeMap::new();
    let mut failed = vec![];
    let mut refs = BTreeMap::new();
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        for (a, b) in [
            ("offset", "accept"),
            ("serial", "offset"),
            ("serial", "accept"),
        ] {
            let x = &cells[&format!("{case}/{a}")];
            let y = &cells[&format!("{case}/{b}")];
            let mut ratios = vec![];
            let mut savings = vec![];
            for i in 0..7 {
                assert_eq!(x[i]["block"], i);
                assert_eq!(y[i]["block"], i);
                let xx = x[i]["median_ms"].as_f64().unwrap();
                let yy = y[i]["median_ms"].as_f64().unwrap();
                ratios.push(xx / yy);
                savings.push(yy - xx);
            }
            let mut s = ratios.clone();
            s.sort_by(f64::total_cmp);
            let wins = ratios.iter().filter(|&&x| x < 1.).count();
            let key = format!("{case}/{a}/{b}");
            pairs.insert(key.clone(),json!({"median":s[3],"low":s[0],"high":s[6],"wins":wins,"ratios":ratios,"paired_saving_ms":median(&savings),"nominal_coverage":0.984375}));
            if a == "offset" {
                let pass = s[0] >= 1. / 1.03 && s[6] <= 1.03;
                calibration.insert(key.clone(), pass);
                if c["workers"]==32 {
                    checks.insert(key.clone(), pass);
                    if !pass {
                        failed.push(key)
                    }
                }
            } else if case == "1x2-w32" {
                let pass = s[6] < 1. && wins >= 5;
                checks.insert(key.clone(), pass);
                if !pass {
                    failed.push(key)
                }
            } else if c["workers"] == 32 {
                let pass = s[6] <= 1.03;
                checks.insert(key.clone(), pass);
                if !pass {
                    failed.push(key)
                }
            }
        }
        if c["workers"] == 32 && !c["reference"].is_null() {
            let mut vs = BTreeMap::new();
            for v in VS {
                let ms = summaries[&format!("{case}/{v}")]["median_ms"]
                    .as_f64()
                    .unwrap();
                let cpu = c["reference"]["cpu_ms"].as_f64().unwrap();
                let metal = c["reference"]["metal_ms"].as_f64().unwrap();
                vs.insert(v,json!({"ms":ms,"CPU_speedup":cpu/ms,"Metal_speedup":metal/ms,"CPU10x":ms<=cpu/10.}));
            }
            refs.insert(
                case.to_string(),
                json!({"reference":c["reference"],"variants":vs}),
            );
        }
    }
    let mut aggregates=BTreeMap::new();
    for control in["accept","offset"]{let mut ratios=vec![];for b in 0..7{let mut log=0.;let mut n=0;for c in read(r("cases.json")).as_array().unwrap().iter().filter(|c|c["workers"]==32&&!c["reference"].is_null()){let k=format!("{}/serial/{control}",c["name"].as_str().unwrap());log+=pairs[&k]["ratios"][b].as_f64().unwrap().ln();n+=1;}assert_eq!(n,8);ratios.push((log/8.).exp());}let mut sorted=ratios.clone();sorted.sort_by(f64::total_cmp);let wins=ratios.iter().filter(|&&r|r<1.).count();let pass=sorted[6]<1.&&wins>=5;let name=format!("original8/serial/{control}");checks.insert(name.clone(),pass);if !pass{failed.push(name.clone());}aggregates.insert(name,json!({"median":sorted[3],"low":sorted[0],"high":sorted[6],"wins":wins,"ratios":ratios,"pass":pass}));}
    let diagnostics=analyze_diagnostics();
    let a = json!({"diagnostics":diagnostics,"original8_aggregates":aggregates,"status":"completed serial phi_split actual SDK experiment; no adoption","gate_passed":failed.is_empty(),"failed_checks":failed,"checks":checks,"calibration_checks":calibration,"pairs":pairs,"cells":summaries,"new_reference_comparisons":refs,"inherited_accepted_M5":read(MANIFEST)["m5"],"SDK_requests":requests,"new_independent_exports":246,"independent_proof_count":18076,"native_core_tests":{"OFF":124,"ON":124},"native_prover_tests":{"OFF":8,"ON":8},"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"accepted_original8_CPU10x_rows":0,"scope":"all9shapes+primary16,7blocks,6automaticprofiles; no clean/portable/resource adoption qualification"});
    save(r("analysis.json"), &a);
    fs::write(
        r("host-after.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!(
        "Gate{}failed{};13824SDK/246Gnark/ledger18076",
        a["gate_passed"], a["failed_checks"]
    );
    for (k, v) in summaries {
        println!(
            "{k} {:.6}ms CPU{:.3} RSS{:.2}/{:.2}",
            v["median_ms"].as_f64().unwrap(),
            v["CPU_ms"].as_f64().unwrap(),
            v["median_RSS_MiB"].as_f64().unwrap(),
            v["max_RSS_MiB"].as_f64().unwrap()
        );
    }
    for (k, v) in pairs {
        println!(
            "{k} {}[{},{}]wins{}saving{}ms",
            v["median"], v["low"], v["high"], v["wins"], v["paired_saving_ms"]
        );
    }
}
fn diagnostic(c: &Value, v: &str, panel: &str, base: &Value,
    ids: &BTreeMap<&str, Value>, jobs: &mut Vec<Value>, steps: &mut Vec<Value>) {
    let name = format!("{panel}-{}-{v}", c["name"].as_str().unwrap());
    let rounds = if panel == "cycles" { 251 } else { 31 };
    let dir = r("proofs").join(&name);
    let data = r("perf").join(format!("{name}.data"));
    fs::create_dir_all(data.parent().unwrap()).unwrap();
    let mut env = base.clone();
    env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
    env["ZOPRO_CPU_DEDUP"] = json!("1");
    env["ZOPRO_CPU_SERIAL_PHI_PROBE"] = ids[v]["probe"].clone();
    env["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
    env["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
    let mut argv = vec!["sudo".into(), "-n".into(), "env".into()];
    for (k, val) in env.as_object().unwrap() {
        argv.push(format!("{k}={}", val.as_str().unwrap()));
    }
    argv.push("perf".into());
    if panel == "cycles" {
        argv.extend(["record".into(),"--no-buildid-cache".into(),"-e".into(),"cycles:u".into(),"-F".into(),"199".into(),"-m".into(),"512".into(),"-o".into(),data.display().to_string()]);
    } else {
        let events = if panel == "fetch" {
            "{cycles:u,instructions:u,L1-icache-load-misses:u,iTLB-load-misses:u}"
        } else {
            "{cycles:u,instructions:u,stalled-cycles-frontend:u,L1-dcache-load-misses:u}"
        };
        argv.extend(["stat".into(),"--no-big-num".into(),"-x".into(),";".into(),"-e".into(),events.into()]);
    }
    argv.extend(["--delay=-1".into(),format!("--control=fifo:{},{}",r("ctl").display(),r("ack").display()),"--".into(),"taskset".into(),"-c".into(),c["cpu_affinity"].as_str().unwrap().into(),ids[v]["binary"].as_str().unwrap().into(),c["image"].as_str().unwrap().into(),c["request"].as_str().unwrap().into(),rounds.to_string(),dir.display().to_string()]);
    steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":900}));
    jobs.push(json!({"name":name,"case":c,"variant":v,"preflight":false,"diagnostic":true,"panel":panel,"rounds":rounds,"directory":dir,"data":data}));
}
fn loss(data: &str) {
    let mut child = Command::new("sudo").args(["-n","perf","script","-D","-i",data])
        .stdout(Stdio::piped()).spawn().unwrap();
    let mut lines = 0;
    let mut lost = vec![];
    for line in std::io::BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        lines += 1;
        if line.contains("PERF_RECORD_LOST") { lost.push(line); }
    }
    assert!(child.wait().unwrap().success());
    println!("{}",json!({"raw_decoded_lines":lines,"lost_records":lost,"zero_lost":lost.is_empty()}));
}
fn analyze_diagnostics() -> Value {
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    for j in read(r("jobs.json")).as_array().unwrap().iter().filter(|j|j["diagnostic"]==true) {
        let name = j["name"].as_str().unwrap();
        if j["panel"] != "cycles" {
            let text = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
            let mut values = BTreeMap::new();
            for l in text.lines() {
                let f: Vec<_> = l.split(';').map(str::trim).collect();
                if f.len()<5 || f[2].is_empty() { continue; }
                let count: f64 = f[0].parse().expect("PMU event unavailable");
                let runtime: f64 = f[3].parse().unwrap();
                let running: f64 = f[4].parse().unwrap();
                assert!(count.is_finite() && count >= 0. && runtime>0. && running>=99.9);
                assert!(values.insert(f[2].to_string(),json!({"count":count,"per_request":count/31.,"runtime":runtime,"running_percent":running})).is_none());
            }
            assert_eq!(values.len(),4);
            counters.insert(name.to_string(),json!({"case":j["case"]["name"],"variant":j["variant"],"panel":j["panel"],"events":values}));
        } else {
            let lost = read(r(format!("runs/loss-{name}/stdout.log")));
            assert_eq!(lost["zero_lost"],true);
            let ev = fs::read_to_string(r(format!("runs/evlist-{name}/stdout.log"))).unwrap();
            assert!(ev.contains("cycles:u") && ev.contains("199") && ev.contains("freq: 1"));
            let mut pids = BTreeSet::new();
            let mut tids = BTreeSet::new();
            let mut symbols = BTreeMap::<String,(u64,u64)>::new();
            let mut samples = 0u64;
            let mut period = 0u64;
            let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            for line in text.lines().filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#')) {
                let f: Vec<_> = line.split_whitespace().collect();
                assert!(f.len()>=4,"unexpected sample {line}");
                let (pid,tid) = f[0].split_once('/').expect("PID/TID column");
                pids.insert(pid.parse::<u32>().unwrap());
                tids.insert(tid.parse::<u32>().unwrap());
                let weight = f[1].parse::<u64>().unwrap();
                assert!(weight>0);
                let entry = symbols.entry(f[3..].join(" ")).or_default();
                entry.0 += 1; entry.1 += weight;
                samples += 1; period += weight;
            }
            assert_eq!(pids.len(),1);
            assert!(samples>0);
            let mut symbols:Vec<_> = symbols.into_iter().map(|(name,(n,p))|json!({"symbol":name,"samples":n,"period":p,"cycle_share":p as f64/period as f64})).collect();
            symbols.sort_by(|a,b|b["period"].as_u64().unwrap().cmp(&a["period"].as_u64().unwrap()));
            profiles.insert(name.to_string(),json!({"case":j["case"]["name"],"variant":j["variant"],"samples":samples,"period":period,"PIDs":pids,"TIDs":tids,"lost":lost,"symbols":symbols}));
        }
    }
    assert!(counters.is_empty());
    assert_eq!(profiles.len(),6);
    json!({"counters":counters,"counter_capture":false,"profiles":profiles,"profile_one_PID_each_zero_lost":true,"diagnostic_timings_used_for_latency_gate":false})
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("loss")=>loss(&std::env::args().nth(2).unwrap()),
        _ => panic!("prepare|finish"),
    }
}
