use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-stages";
const SDK: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/native/candidate-sdk";
const SDKHASH: &str = "81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const BASE:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json";
const P: &str = "/home/ubuntu/src/zopro-perf-work/sdk-control-copies";
const LAST: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const GROUP: &str = "zopro174stage";
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
fn output(argv: &[&str]) -> String {
    let o = Command::new(argv[0]).args(&argv[1..]).output().unwrap();
    assert!(
        o.status.success(),
        "{argv:?}:{}",
        String::from_utf8_lossy(&o.stderr)
    );
    String::from_utf8(o.stdout).unwrap()
}
fn prepare() {
    assert!(!r("plan.json").exists());
    assert_eq!(hash(SDK), SDKHASH);
    let mut pins = BTreeMap::new();
    let m = read(BASE);
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 174);
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{LAST}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 14551);
    for (f, h) in read(format!("{LAST}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 15028);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let bytes = fs::read(SDK).unwrap();
    let u16at = |p: usize| u16::from_le_bytes(bytes[p..p + 2].try_into().unwrap());
    let u64at = |p: usize| u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap());
    assert_eq!(&bytes[..4], b"\x7fELF");
    let phoff = u64at(32) as usize;
    let mut loads = vec![];
    for i in 0..u16at(56) as usize {
        let p = phoff + i * u16at(54) as usize;
        let kind = u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap());
        if kind == 1 {
            loads.push((u64at(p + 16), u64at(p + 8), u64at(p + 32)));
        }
    }
    let mut probes = vec![];
    for (label, addr) in [
        ("sdk", 0x4c6740u64),
        ("core", 0x43ae90),
        ("solver", 0x3f87b0),
        ("product", 0x444410),
        ("canonical", 0x441150),
        ("g2", 0x464480),
    ] {
        let (a, o, _) = loads
            .iter()
            .copied()
            .find(|(a, _, n)| addr >= *a && addr < *a + *n)
            .unwrap();
        let off = addr - a + o;
        for (ret, suffix) in [(false, "enter"), (true, "exit")] {
            let name = format!("{label}_{suffix}");
            probes.push(json!({"name":name,"virtual_address":addr,"file_offset":off,"definition":format!("{}:{GROUP}/{name} {SDK}:0x{off:x}",if ret{"r"}else{"p"})}));
        }
    }
    for (label, addr) in [("join_enter", 0x441a33u64), ("join_exit", 0x441a38)] {
        let (a, o, _) = loads
            .iter()
            .copied()
            .find(|(a, _, n)| addr >= *a && addr < *a + *n)
            .unwrap();
        let off = addr - a + o;
        probes.push(json!({"name":label,"virtual_address":addr,"file_offset":off,"definition":format!("p:{GROUP}/{label} {SDK}:0x{off:x}")}));
    }
    assert_eq!(probes.len(), 14);
    let asm = output(&[
        "objdump",
        "-dC",
        "--no-show-raw-insn",
        "--start-address=0x441150",
        "--stop-address=0x4428de",
        SDK,
    ]);
    assert!(asm
        .lines()
        .any(|l| l.contains("441a33:") && l.contains("call") && l.contains("497460")));
    assert!(asm.lines().any(|l| l.trim_start().starts_with("441a38:")));
    fs::write(r("canonical-disassembly.txt"), asm).unwrap();
    fs::write(
        r("symbols.txt"),
        output(&["nm", "-CS", "--defined-only", SDK]),
    )
    .unwrap();
    save(r("probes.json"), &json!(probes));
    let all = read(format!("{P}/cases.json"));
    let all = all.as_array().unwrap();
    let mut cases = vec![all[0].clone()];
    let mut physical = all[0].clone();
    physical["name"] = json!("1x2-w16");
    physical["workers"] = json!(16);
    physical["cpu_affinity"] = json!("0-15");
    cases.push(physical);
    cases.push(all.iter().find(|c| c["shape"] == "16x2").unwrap().clone());
    cases.push(all[8].clone());
    save(r("cases.json"), &json!(cases));
    for c in &cases {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    for e in fs::read_dir(format!("{P}/lagrange-cache")).unwrap() {
        pin(&mut pins, e.unwrap().path());
    }
    for f in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());
    }
    let mut env = read(format!("{LAST}/run-plan.json"))["steps"][0]["environment"].clone();
    let mut jobs = vec![];
    let mut steps = vec![
        json!({"kind":"command","name":"install","argv":[r("run-helper"),"install"],"timeout_seconds":120}),
    ];
    let mut push = |case: &Value, label: &str, block: usize, pre: bool, steps: &mut Vec<Value>| {
        let name = if pre {
            "preflight".to_string()
        } else {
            format!("b{block}-{}-{label}", case["name"].as_str().unwrap())
        };
        let mut c = case.clone();
        if pre {
            c["workers"] = json!(4);
            c["cpu_affinity"] = json!("0-3");
        }
        let rounds = if pre { 1 } else { 31 };
        let mut e = env.clone();
        e["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
        if label == "on" {
            e["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
            e["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
        }
        let dir = r("proofs").join(&name);
        let data = r("perf").join(format!("{name}.data"));
        let mut argv = vec!["sudo".to_string(), "-n".into(), "env".into()];
        for (k, v) in e.as_object().unwrap() {
            argv.push(format!("{k}={}", v.as_str().unwrap()));
        }
        if label == "on" {
            argv.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "--clockid".into(),
                "mono".into(),
                "-e".into(),
                "cycles/freq=199/u".into(),
                "-e".into(),
                format!("{GROUP}:*/period=1/"),
                "-m".into(),
                "512".into(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    r("ctl").display(),
                    r("ack").display()
                ),
                "-o".into(),
                data.display().to_string(),
                "--".into(),
            ]);
        }
        argv.extend([
            "taskset".into(),
            "-c".into(),
            c["cpu_affinity"].as_str().unwrap().into(),
            SDK.into(),
            c["image"].as_str().unwrap().into(),
            c["request"].as_str().unwrap().into(),
            rounds.to_string(),
            dir.display().to_string(),
        ]);
        steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":900}));
        jobs.push(json!({"name":name,"label":label,"block":block,"preflight":pre,"case":c,"rounds":rounds,"directory":dir,"data":data}));
    };
    fs::create_dir(r("perf")).unwrap();
    push(&cases[0], "on", 0, true, &mut steps);
    let verifier = |j: &Value| {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120})
    };
    // End the closure's borrow before reading the preflight job, then rebuild it
    // through the local push function below for the remaining fixed cases.
    drop(push);
    steps.push(verifier(&jobs[0]));
    decode_steps(&jobs[0], &mut steps);
    steps.push(json!({"kind":"command","name":"check-preflight","argv":[r("run-helper"),"check-preflight"],"timeout_seconds":120}));
    for b in 0..3 {
        let mut order = vec![0, 1, 2, 3];
        order.rotate_left(b % 4);
        if b % 2 != 0 {
            order.reverse();
        }
        for i in order {
            let labels = if (b + i) % 2 == 0 {
                ["off", "on"]
            } else {
                ["on", "off"]
            };
            for label in labels {
                push_job(&cases[i], label, b, &env, &mut jobs, &mut steps);
            }
        }
    }
    assert_eq!(jobs.len(), 25);
    for j in jobs.iter().skip(1) {
        steps.push(verifier(j));
    }
    for j in jobs.iter().skip(1).filter(|j| j["label"] == "on") {
        decode_steps(j, &mut steps);
    }
    steps.push(json!({"kind":"command","name":"cleanup","argv":[r("run-helper"),"cleanup"],"timeout_seconds":120}));
    assert_eq!(steps.len(), 92);
    save(r("jobs.json"), &json!(jobs));
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
    for f in [SDK, BASE, VERIFY, X] {
        pin(&mut pins, f);
    }
    for f in [
        "design.md",
        "run.rs",
        "run-helper",
        "parser-tests.log",
        "probes.json",
        "cases.json",
        "jobs.json",
        "prior-proof-sha256.json",
        "canonical-disassembly.txt",
        "symbols.txt",
        "host-before.txt",
    ] {
        pin(&mut pins, r(f));
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared92fixedsteps/25SDKexports/820requests;14ELF-offsetprobes;immutable174source");
}
fn push_job(
    c: &Value,
    label: &str,
    b: usize,
    env: &Value,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
) {
    let name = format!("b{b}-{}-{label}", c["name"].as_str().unwrap());
    let mut e = env.clone();
    e["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
    let dir = r("proofs").join(&name);
    let data = r("perf").join(format!("{name}.data"));
    if label == "on" {
        e["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
        e["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
    }
    let mut argv = vec!["sudo".to_string(), "-n".into(), "env".into()];
    for (k, v) in e.as_object().unwrap() {
        argv.push(format!("{k}={}", v.as_str().unwrap()));
    }
    if label == "on" {
        argv.extend([
            "perf".into(),
            "record".into(),
            "--no-buildid-cache".into(),
            "--clockid".into(),
            "mono".into(),
            "-e".into(),
            "cycles/freq=199/u".into(),
            "-e".into(),
            format!("{GROUP}:*/period=1/"),
            "-m".into(),
            "512".into(),
            "--delay=-1".into(),
            format!(
                "--control=fifo:{},{}",
                r("ctl").display(),
                r("ack").display()
            ),
            "-o".into(),
            data.display().to_string(),
            "--".into(),
        ]);
    }
    argv.extend([
        "taskset".into(),
        "-c".into(),
        c["cpu_affinity"].as_str().unwrap().into(),
        SDK.into(),
        c["image"].as_str().unwrap().into(),
        c["request"].as_str().unwrap().into(),
        "31".into(),
        dir.display().to_string(),
    ]);
    steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":900}));
    jobs.push(json!({"name":name,"label":label,"block":b,"preflight":false,"case":c,"rounds":31,"directory":dir,"data":data}));
}
fn decode_steps(j: &Value, steps: &mut Vec<Value>) {
    let name = j["name"].as_str().unwrap();
    steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["sudo","-n","perf","script","--ns","--header","--show-lost-events","-i",j["data"],"-F","pid,tid,time,event,period,ip,sym"],"timeout_seconds":180}));
    steps.push(json!({"kind":"command","name":format!("evlist-{name}"),"argv":["sudo","-n","perf","evlist","-v","-i",j["data"]],"timeout_seconds":120}));
    steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[r("run-helper"),"loss",j["data"]],"timeout_seconds":180}));
}
fn defs() -> String {
    output(&["sudo", "-n", "cat", "/sys/kernel/tracing/uprobe_events"])
}
fn write_def(s: &str) {
    assert!(Command::new("sudo")
        .arg("-n")
        .arg(r("run-helper"))
        .args(["write-control", s])
        .status()
        .unwrap()
        .success());
}
fn write_control(s: &str) {
    // tee -a seeks to EOF, which tracefs rejects. Write a predeclared owned
    // command directly, without seeking, truncating or globally enabling it.
    let probes = read(r("probes.json"));
    assert!(probes.as_array().unwrap().iter().any(|p| {
        p["definition"].as_str() == Some(s)
            || s == format!("-:{GROUP}/{}", p["name"].as_str().unwrap())
    }));
    fs::OpenOptions::new()
        .write(true)
        .open("/sys/kernel/tracing/uprobe_events")
        .unwrap()
        .write_all(format!("{s}\n").as_bytes())
        .unwrap();
}
fn install() {
    assert_eq!(hash(SDK), SDKHASH);
    assert!(!defs().contains(&format!(":{GROUP}/")));
    for p in read(r("probes.json")).as_array().unwrap() {
        write_def(p["definition"].as_str().unwrap());
    }
    let got = defs();
    let lines: Vec<_> = got
        .lines()
        .filter(|l| l.contains(&format!(":{GROUP}/")))
        .collect();
    assert_eq!(lines.len(), 14);
    save(
        r("installed.json"),
        &json!({"definitions":lines,"SDK_sha256":SDKHASH,"group":GROUP,"unrelated_definitions":got.lines().filter(|l|!l.contains(&format!(":{GROUP}/"))).collect::<Vec<_>>()}),
    );
    println!("Registered14owned uprobes; no globalenable");
}
fn cleanup() {
    let s = defs();
    let mut removed = vec![];
    for l in s.lines().filter(|l| l.contains(&format!(":{GROUP}/"))) {
        let first = l.split_whitespace().next().unwrap();
        let event = first.split_once(':').unwrap().1;
        assert!(event.starts_with(&format!("{GROUP}/")));
        write_def(&format!("-:{event}"));
        removed.push(event.to_string());
    }
    assert!(!defs().contains(&format!(":{GROUP}/")));
    save(
        r("cleanup.json"),
        &json!({"removed":removed,"owned_group_absent":true}),
    );
    println!("Removed{}ownedprobes", removed.len());
}
fn loss(path: &str) {
    let mut c = Command::new("sudo")
        .args(["-n", "perf", "script", "-D", "-i", path])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = 0;
    let mut lost = vec![];
    for l in BufReader::new(c.stdout.take().unwrap()).lines() {
        let l = l.unwrap();
        lines += 1;
        if l.contains("PERF_RECORD_LOST") {
            lost.push(l);
        }
    }
    let o = c.wait_with_output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    println!(
        "{}",
        json!({"raw_decoded_lines":lines,"lost_records":lost,"zero_lost":lost.is_empty()})
    );
    assert!(lost.is_empty());
}
#[derive(Clone, Debug)]
struct Span {
    kind: String,
    tid: u64,
    start: u64,
    end: u64,
}
#[derive(Clone, Debug)]
struct Cycle {
    time: u64,
    period: u64,
    symbol: String,
}
fn ns(s: &str) -> u64 {
    let (a, b) = s.trim_end_matches(':').split_once('.').unwrap();
    assert!(b.len() <= 9);
    a.parse::<u64>().unwrap() * 1_000_000_000
        + b.parse::<u64>().unwrap() * 10u64.pow(9 - b.len() as u32)
}
fn parse(t: &str) -> (Vec<Span>, Vec<Cycle>, BTreeSet<u64>) {
    let mut stacks: BTreeMap<(u64, String), Vec<u64>> = BTreeMap::new();
    let mut spans = vec![];
    let mut cycles = vec![];
    let mut pids = BTreeSet::new();
    for line in t.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        assert!(!line.contains("LOST"), "lostevent");
        let f: Vec<_> = line.split_whitespace().collect();
        assert!(f.len() >= 6, "badline:{line}");
        let (pid, tid, time, period, symbol_start) = if let Some((pid, tid)) = f[0].split_once('/')
        {
            (
                pid.parse::<u64>().unwrap(),
                tid.parse::<u64>().unwrap(),
                ns(f[1]),
                f[2].parse::<u64>().unwrap(),
                5,
            )
        } else {
            (
                f[0].parse::<u64>().unwrap(),
                f[1].parse::<u64>().unwrap(),
                ns(f[2]),
                f[4].parse::<u64>().unwrap(),
                6,
            )
        };
        let event = f[3].trim_end_matches(':');
        pids.insert(pid);
        if let Some(event) = event.strip_prefix(&format!("{GROUP}:")) {
            assert_eq!(period, 1);
            let (kind, edge) = event.rsplit_once('_').unwrap();
            let key = (tid, kind.to_string());
            if edge == "enter" {
                stacks.entry(key).or_default().push(time);
            } else {
                assert_eq!(edge, "exit");
                let start = stacks.get_mut(&key).unwrap().pop().unwrap();
                assert!(time >= start);
                spans.push(Span {
                    kind: kind.into(),
                    tid,
                    start,
                    end: time,
                });
            }
        } else {
            assert!(event.starts_with("cycles"), "unexpectedevent{event}");
            assert!(period > 0);
            cycles.push(Cycle {
                time,
                period,
                symbol: f[symbol_start..].join(" "),
            });
        }
    }
    assert!(stacks.values().all(Vec::is_empty), "unmatchedentries");
    spans.sort_by_key(|s| s.start);
    (spans, cycles, pids)
}
fn within(a: &Span, b: &Span) -> bool {
    a.start >= b.start && a.end <= b.end
}
fn observe(spans: &[Span], cycles: &[Cycle], rounds: usize, small: bool) -> Value {
    let sdks: Vec<_> = spans.iter().filter(|s| s.kind == "sdk").collect();
    assert_eq!(sdks.len(), rounds);
    for kind in ["solver", "product"] {
        assert_eq!(spans.iter().filter(|s| s.kind == kind).count(), rounds);
    }
    let mut rows = vec![];
    let mut phase_period = BTreeMap::<String, u64>::new();
    let mut phase_symbols = BTreeMap::<String, BTreeMap<String, u64>>::new();
    let mut attributed = 0u64;
    for sdk in sdks {
        let get = |kind: &str| {
            spans
                .iter()
                .filter(|s| s.kind == kind && within(s, sdk))
                .collect::<Vec<_>>()
        };
        let sol = get("solver");
        let prod = get("product");
        assert_eq!(sol.len(), 1);
        assert_eq!(prod.len(), 1);
        let (solver, product) = (sol[0], prod[0]);
        assert!(solver.end <= product.start);
        let cores = get("core");
        let core = *cores
            .iter()
            .filter(|s| within(solver, s) && within(product, s))
            .min_by_key(|s| s.end - s.start)
            .unwrap();
        assert_eq!(core.tid, solver.tid);
        assert_eq!(core.tid, product.tid);
        let joins = get("join");
        let g2 = get("g2");
        let canonical = get("canonical");
        if small {
            assert_eq!(joins.len(), 1);
            assert_eq!(g2.len(), 1);
            assert_eq!(canonical.len(), 1);
        } else {
            assert!(joins.len() <= 1 && g2.len() <= 1 && canonical.len() <= 1);
        }
        if let Some(j) = joins.first() {
            assert!(j.start >= product.end && j.end <= core.end);
            assert_eq!(j.tid, core.tid);
            if let Some(g) = g2.first() {
                assert!(within(g, j));
            }
        }
        let phases = [
            ("pre_solver", core.start, solver.start),
            ("solver", solver.start, solver.end),
            ("gather_public_reclaim", solver.end, product.start),
            ("FFT_product", product.start, product.end),
            ("post_product", product.end, core.end),
        ];
        let mut times = BTreeMap::new();
        let mut covered = 0;
        for (label, start, end) in phases {
            assert!(end >= start);
            times.insert(label, (end - start) as f64 / 1e6);
            covered += end - start;
        }
        assert_eq!(covered, core.end - core.start);
        for c in cycles
            .iter()
            .filter(|c| c.time >= sdk.start && c.time <= sdk.end)
        {
            let label = phases
                .iter()
                .find(|(_, a, b)| c.time >= *a && c.time < *b)
                .map(|(l, _, _)| *l)
                .unwrap_or("SDK_outside_innercore");
            *phase_period.entry(label.into()).or_default() += c.period;
            *phase_symbols
                .entry(label.into())
                .or_default()
                .entry(c.symbol.clone())
                .or_default() += c.period;
            attributed += c.period;
        }
        rows.push(json!({"SDK_ms":(sdk.end-sdk.start)as f64/1e6,"core_ms":(core.end-core.start)as f64/1e6,"SDK_outside_innercore_ms":((sdk.end-sdk.start)-(core.end-core.start))as f64/1e6,"phases_ms":times,"small_join_ms":joins.first().map(|s|(s.end-s.start)as f64/1e6),"G2_inclusive_ms":g2.first().map(|s|(s.end-s.start)as f64/1e6),"core_tid":core.tid,"sdk_tid":sdk.tid,"start_ns":sdk.start,"end_ns":sdk.end}));
    }
    let total: u64 = cycles.iter().map(|c| c.period).sum();
    assert!(total >= attributed);
    phase_period.insert("outside_SDK_spans".into(), total - attributed);
    json!({"requests":rows,"phase_cycle_period":phase_period,"phase_symbols":phase_symbols,"cycle_samples":cycles.len(),"total_cycle_period":total,"spans":spans.len()})
}
fn logs(j: &Value) -> PathBuf {
    if j["preflight"] == true {
        r("preflight-parser-failure/workflow")
    } else {
        r("runs")
    }
}
fn trace(j: &Value) -> Value {
    let name = j["name"].as_str().unwrap();
    let text = fs::read_to_string(logs(j).join(format!("decode-{name}/stdout.log"))).unwrap();
    let (s, c, pids) = parse(&text);
    assert_eq!(pids.len(), 1);
    assert!(!c.is_empty());
    let ev = fs::read_to_string(logs(j).join(format!("evlist-{name}/stdout.log"))).unwrap();
    let lines: Vec<_> = ev.lines().filter(|l| l.contains(": type:")).collect();
    assert_eq!(lines.len(), 16);
    assert_eq!(lines.iter().filter(|l| l.starts_with(GROUP)).count(), 14);
    assert_eq!(
        lines.iter().filter(|l| l.starts_with("dummy:u:")).count(),
        1
    );
    let cyc = lines.iter().find(|l| l.starts_with("cycles")).unwrap();
    assert!(cyc.contains("sample_freq }: 199") && cyc.contains("freq: 1"));
    for l in lines.iter().filter(|l| l.starts_with(GROUP)) {
        assert!(l.contains("sample_freq }: 1,") && !l.contains("freq: 1,"));
    }
    let loss = read(logs(j).join(format!("loss-{name}/stdout.log")));
    assert_eq!(loss["zero_lost"], true);
    let mut out = observe(
        &s,
        &c,
        j["rounds"].as_u64().unwrap() as usize,
        j["case"]["shape"] == "1x2",
    );
    out["pids"] = json!(pids);
    out
}
fn preflight() {
    let j = read(r("jobs.json"))[0].clone();
    let a = trace(&j);
    save(r("preflight-analysis.json"), &a);
    println!(
        "ExactELFpreflight complete:1SDK/solver/product/join/G2,onePID,noLost;{}cycle samples",
        a["cycle_samples"]
    );
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    assert_eq!(read(r("cleanup.json"))["owned_group_absent"], true);
    for (f, h) in read(r("pins-parse-repaired.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    let mut inventory = BTreeMap::new();
    let mut cells: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut traces = BTreeMap::new();
    let mut total = 0;
    let mut samples = 0;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(logs(j).join(format!("{name}/stdout.log"))).unwrap();
        let times: Vec<f64> = text
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| {
                serde_json::from_str::<Value>(l).unwrap()["ms"]
                    .as_f64()
                    .unwrap()
            })
            .collect();
        let summary: Value = serde_json::from_str(
            text.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(times.len() as u64, j["rounds"].as_u64().unwrap());
        assert_eq!(summary["workers"], j["case"]["workers"]);
        assert_eq!(summary["priming"], 3);
        total += times.len() + 3;
        let verified =
            fs::read_to_string(logs(j).join(format!("verify-{name}/stdout.log"))).unwrap();
        assert!(verified.contains("VERIFIED proof_sha256="));
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
        assert!(verified.contains(&format!(
            "image_sha256={}",
            j["case"]["image_sha256"].as_str().unwrap()
        )));
        if j["label"] == "on" {
            let a = trace(j);
            samples += a["cycle_samples"].as_u64().unwrap();
            traces.insert(name.to_string(), a);
        }
        if j["preflight"] != true {
            cells.entry(format!("{}/{}",j["case"]["name"].as_str().unwrap(),j["label"].as_str().unwrap())).or_default().push(json!({"name":name,"block":j["block"],"median_ms":median(&times),"summary":summary}));
        }
    }
    assert_eq!(total, 820);
    assert_eq!(prior.len(), 15053);
    assert_eq!(inventory.len(), 50);
    let mut rows = BTreeMap::new();
    for (k, v) in &cells {
        assert_eq!(v.len(), 3);
        rows.insert(k.clone(),json!({"median_ms":median(&v.iter().map(|x|x["median_ms"].as_f64().unwrap()).collect::<Vec<_>>()),"blocks":v}));
    }
    let a = json!({"status":"completed exact174ELF diagnostic; no optimization or adoption","cells":rows,"traces":traces,"cycle_samples":samples,"SDK_requests":total,"new_independent_exports":25,"independent_proof_count":15053,"production_changed":false,"accepted_manifest":BASE,"acceptedSDK":SDK,"acceptedSDK_sha256":SDKHASH,"owned_probes_removed":true,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"inherited_M5":read(BASE)["m5"],"scope":"local primary16/32,16x2/32,54/32 diagnostics; no new all8/AWS/mobile/pressure measurement"});
    save(r("analysis.json"), &a);
    save(r("proof-inventory.json"), &json!(inventory));
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
    println!("Completed820SDK/25freshGnark/ledger15053;{samples}cycle samples;no productionchange");
    for (k, v) in rows {
        println!("{k} {}ms", v["median_ms"]);
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("install") => install(),
        Some("cleanup") => cleanup(),
        Some("write-control") => write_control(&std::env::args().nth(2).unwrap()),
        Some("loss") => loss(&std::env::args().nth(2).unwrap()),
        Some("check-preflight") => preflight(),
        Some("finish") => finish(),
        _ => panic!("prepare|install|cleanup|loss|check-preflight|finish"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn line(tid: u64, time: u64, e: &str) -> String {
        format!("1 {tid} 0.{time:09}: {GROUP}:{e}: 1 123 symbol\n")
    }
    #[test]
    fn recursive_tid_pairing_and_phase_partition() {
        let events = [
            (1, 0, "sdk_enter"),
            (1, 1, "core_enter"),
            (2, 3, "core_enter"),
            (2, 4, "canonical_enter"),
            (2, 5, "solver_enter"),
            (2, 20, "solver_exit"),
            (2, 30, "product_enter"),
            (2, 40, "product_exit"),
            (2, 45, "join_enter"),
            (3, 50, "g2_enter"),
            (3, 68, "g2_exit"),
            (2, 70, "join_exit"),
            (2, 78, "canonical_exit"),
            (2, 80, "core_exit"),
            (1, 90, "core_exit"),
            (1, 100, "sdk_exit"),
        ];
        let mut t = events
            .into_iter()
            .map(|(tid, n, e)| line(tid, n, e))
            .collect::<String>();
        t.push_str("1 2 0.000000010: cycles/freq=199/u: 50 123 solver\n");
        let (s, c, p) = parse(&t);
        assert_eq!(p.len(), 1);
        let o = observe(&s, &c, 1, true);
        assert_eq!(o["requests"][0]["core_tid"], 2);
        assert_eq!(o["phase_cycle_period"]["solver"], 50);
        assert_eq!(o["requests"][0]["phases_ms"]["FFT_product"], 0.00001);
    }
    #[test]
    #[should_panic]
    fn unmatched_return_rejected() {
        parse(&line(1, 5, "solver_exit"));
    }
    #[test]
    #[should_panic]
    fn lost_records_rejected() {
        parse("LOST 2 events");
    }
    #[test]
    fn actual_perf_pid_tid_and_period_order() {
        let text = "999458/999458 98437.330387946: 1 zopro174stage:sdk_enter: 5c523449a740 sdk\n999458/999463 98437.330488616: 73385 cycles/freq=199/u: 5c52340d2b6a worker\n999458/999458 98437.340387946: 1 zopro174stage:sdk_exit: 5c523449a740 sdk\n";
        let (s, c, p) = parse(text);
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].end - s[0].start, 10_000_000);
        assert_eq!(c[0].period, 73385);
        assert_eq!(p, BTreeSet::from([999458]));
    }
}
