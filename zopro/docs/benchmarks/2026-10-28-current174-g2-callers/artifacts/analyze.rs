use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-g2-callers";
const PARENT: &str = "/home/ubuntu/src/zopro-perf-work/current174-stages";
const SDK: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/native/candidate-sdk";
const HASH: &str = "81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json";
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
fn output(a: &[&str]) -> String {
    let o = Command::new(a[0]).args(&a[1..]).output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
#[allow(dead_code, unused_imports, unused_mut)]
mod parser {
    include!("/home/ubuntu/src/zopro-perf-work/current174-stages/run.rs");
    pub fn callers(text: &str, rounds: usize) -> Value {
        let (spans, cycles, pids) = parse(text);
        assert_eq!(pids.len(), 1);
        for kind in ["sdk", "join", "query", "g2"] {
            assert_eq!(
                spans.iter().filter(|s| s.kind == kind).count(),
                rounds,
                "{kind}"
            );
        }
        let mut rows = vec![];
        let mut threaded = vec![];
        for line in text
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        {
            let f: Vec<_> = line.split_whitespace().collect();
            if f[3].starts_with("cycles") {
                let tid = f[0].split_once('/').unwrap().1.parse::<u64>().unwrap();
                threaded.push((tid, ns(f[1]), f[2].parse::<u64>().unwrap()));
            }
        }
        for sdk in spans.iter().filter(|s| s.kind == "sdk") {
            let select = |kind: &str, outer: &Span, same: bool| {
                spans
                    .iter()
                    .filter(|s| s.kind == kind && within(s, outer) && (!same || s.tid == outer.tid))
                    .collect::<Vec<_>>()
            };
            let join = select("join", sdk, false);
            let query = select("query", sdk, false);
            let raw = select("g2", sdk, false);
            assert_eq!(join.len(), 1);
            assert_eq!(query.len(), 1);
            assert_eq!(raw.len(), 1);
            let (join, query, raw) = (join[0], query[0], raw[0]);
            assert!(within(query, join) && within(raw, query));
            assert_eq!(query.tid, raw.tid);
            let groups = select("grouping", query, true)
                .into_iter()
                .filter(|s| s.end <= raw.start)
                .collect::<Vec<_>>();
            assert_eq!(groups.len(), 1);
            let groups = groups[0];
            assert!(groups.end <= raw.start);
            let points = select("points", query, true)
                .into_iter()
                .filter(|s| s.start >= groups.end && s.end <= raw.start)
                .collect::<Vec<_>>();
            assert!(points.len() <= 1);
            if let Some(p) = points.first() {
                assert!(p.start >= groups.end && p.end <= raw.start);
            }
            let admission = select("admission", groups, true);
            assert!(admission.len() <= 1);
            let heads = admission
                .first()
                .map(|s| select("headroom", s, true))
                .unwrap_or_default();
            assert!(heads.len() <= 1);
            let length = |s: &Span| s.end - s.start;
            let before = raw.start - query.start;
            let gd = length(groups);
            let pd = points.first().map(|s| length(s)).unwrap_or(0);
            assert!(before >= gd + pd);
            let cpu = |s: &Span| {
                let samples = threaded
                    .iter()
                    .filter(|(tid, t, _)| *tid == s.tid && *t >= s.start && *t <= s.end)
                    .collect::<Vec<_>>();
                json!({"samples":samples.len(),"period":samples.iter().map(|(_,_,p)|*p).sum::<u64>()})
            };
            rows.push(json!({"SDK_ms":length(sdk)as f64/1e6,"join_ms":length(join)as f64/1e6,"join_to_query_ms":(query.start-join.start)as f64/1e6,"query_before_raw_ms":before as f64/1e6,"grouping_ms":gd as f64/1e6,"points_ms":pd as f64/1e6,"points_present":!points.is_empty(),"caller_other_ms":(before-gd-pd)as f64/1e6,"admission_ms":admission.first().map(|s|length(s)as f64/1e6),"headroom_ms":heads.first().map(|s|length(s)as f64/1e6),"raw_ms":length(raw)as f64/1e6,"query_after_raw_ms":(query.end-raw.end)as f64/1e6,"join_after_query_ms":(join.end-query.end)as f64/1e6,"query_tid":query.tid,"query_cycles":cpu(query),"grouping_cycles":cpu(groups),"start_ns":sdk.start,"end_ns":sdk.end}));
        }
        json!({"requests":rows,"cycle_samples":cycles.len(),"cycle_period":cycles.iter().map(|c|c.period).sum::<u64>(),"pids":pids,"spans":spans.len()})
    }
    #[cfg(test)]
    mod caller_tests {
        use super::*;
        #[test]
        fn partitions_preparation_and_dispatch() {
            let ev = [
                (1, 0, "sdk_enter"),
                (2, 20, "join_enter"),
                (3, 30, "query_enter"),
                (3, 40, "grouping_enter"),
                (3, 45, "admission_enter"),
                (3, 48, "headroom_enter"),
                (3, 55, "headroom_exit"),
                (3, 60, "admission_exit"),
                (3, 80, "grouping_exit"),
                (3, 85, "points_enter"),
                (3, 90, "points_exit"),
                (3, 100, "g2_enter"),
                (3, 175, "g2_exit"),
                (3, 180, "query_exit"),
                (2, 190, "join_exit"),
                (1, 200, "sdk_exit"),
            ];
            let mut t = String::new();
            for (tid, n, e) in ev {
                t.push_str(&format!("1/{tid} 0.{n:09}: 1 zopro174stage:{e}: 123 sym\n"));
            }
            let a = callers(&t, 1);
            assert_eq!(a["requests"][0]["caller_other_ms"], 0.000025);
            assert_eq!(a["requests"][0]["join_to_query_ms"], 0.00001);
            assert_eq!(a["requests"][0]["admission_ms"], 0.000015);
        }
        #[test]
        #[should_panic]
        fn incomplete_raw_is_rejected() {
            callers("1/1 0.000000001: 1 zopro174stage:sdk_enter: 123 sym\n1/1 0.000000002: 1 zopro174stage:sdk_exit: 123 sym\n",1);
        }
        #[test]
        fn point_groups_inside_raw_are_not_caller_preparation() {
            let ev = [
                (1, 0, "sdk_enter"),
                (2, 10, "join_enter"),
                (3, 20, "query_enter"),
                (3, 25, "grouping_enter"),
                (3, 35, "grouping_exit"),
                (3, 40, "g2_enter"),
                (3, 45, "points_enter"),
                (3, 50, "points_exit"),
                (3, 55, "points_enter"),
                (3, 60, "points_exit"),
                (3, 70, "g2_exit"),
                (3, 75, "query_exit"),
                (2, 80, "join_exit"),
                (1, 90, "sdk_exit"),
            ];
            let text = ev
                .into_iter()
                .map(|(tid, t, e)| format!("1/{tid} 0.{t:09}: 1 zopro174stage:{e}: 123 symbol\n"))
                .collect::<String>();
            let a = callers(&text, 1);
            assert_eq!(a["requests"][0]["points_present"], false);
            assert_eq!(a["requests"][0]["points_ms"], 0.0);
            assert_eq!(a["requests"][0]["grouping_ms"], 0.00001);
        }
    }
}
fn prepare() {
    assert!(!r("plan.json").exists());
    assert_eq!(hash(SDK), HASH);
    let mut pins = BTreeMap::new();
    for (f, h) in read(MANIFEST)["source_files_sha256"].as_object().unwrap() {
        let p = Path::new("/home/ubuntu/src/zk-tools/zopro").join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{PARENT}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 15028);
    for (f, h) in read(format!("{PARENT}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(f), h.as_str().unwrap());
        if f.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 15053);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let bytes = fs::read(SDK).unwrap();
    let u16at = |p: usize| u16::from_le_bytes(bytes[p..p + 2].try_into().unwrap()) as usize;
    let u64at = |p: usize| u64::from_le_bytes(bytes[p..p + 8].try_into().unwrap());
    let ph = u64at(32) as usize;
    let mut loads = vec![];
    for i in 0..u16at(56) {
        let p = ph + i * u16at(54);
        if u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap()) == 1 {
            loads.push((u64at(p + 16), u64at(p + 8), u64at(p + 32)));
        }
    }
    let mut probes = vec![];
    for (label, addr) in [
        ("sdk", 0x4c6740u64),
        ("g2", 0x464480),
        ("query", 0x457590),
        ("grouping", 0x217790),
        ("points", 0x22b010),
        ("admission", 0x21b050),
        ("headroom", 0x21bd90),
    ] {
        let (a, o, _) = loads
            .iter()
            .copied()
            .find(|(a, _, n)| addr >= *a && addr < *a + *n)
            .unwrap();
        let off = addr - a + o;
        for (ret, edge) in [(false, "enter"), (true, "exit")] {
            let name = format!("{label}_{edge}");
            probes.push(json!({"name":name,"virtual_address":addr,"file_offset":off,"definition":format!("{}:{GROUP}/{name} {SDK}:0x{off:x}",if ret{"r"}else{"p"})}));
        }
    }
    for (name, addr) in [("join_enter", 0x441a33u64), ("join_exit", 0x441a38)] {
        let (a, o, _) = loads
            .iter()
            .copied()
            .find(|(a, _, n)| addr >= *a && addr < *a + *n)
            .unwrap();
        let off = addr - a + o;
        probes.push(json!({"name":name,"virtual_address":addr,"file_offset":off,"definition":format!("p:{GROUP}/{name} {SDK}:0x{off:x}")}));
    }
    assert_eq!(probes.len(), 16);
    save(r("probes.json"), &json!(probes));
    let asm = output(&[
        "objdump",
        "-dC",
        "--no-show-raw-insn",
        "--start-address=0x457590",
        "--stop-address=0x457bac",
        SDK,
    ]);
    assert!(asm.contains("217790") && asm.contains("22b010") && asm.contains("464480"));
    fs::write(r("query-disassembly.txt"), asm).unwrap();
    for f in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());
    }
    fs::create_dir(r("perf")).unwrap();
    let cases = read(format!("{PARENT}/cases.json"));
    let env = read("/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/run-plan.json")
        ["steps"][0]["environment"]
        .clone();
    let mut jobs = vec![];
    let mut steps = vec![
        json!({"kind":"command","name":"install","argv":[r("run-helper"),"install"],"timeout_seconds":120}),
    ];
    for b in 0..3 {
        let indices = if b % 2 == 0 { [0, 1] } else { [1, 0] };
        for i in indices {
            let c = cases[i].clone();
            let name = format!("b{b}-{}", c["name"].as_str().unwrap());
            let mut e = env.clone();
            e["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
            e["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
            e["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
            let data = r("perf").join(format!("{name}.data"));
            let dir = r("proofs").join(&name);
            let mut argv = vec!["sudo".to_string(), "-n".into(), "env".into()];
            for (k, v) in e.as_object().unwrap() {
                argv.push(format!("{k}={}", v.as_str().unwrap()));
            }
            argv.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "--clockid".into(),
                "mono".into(),
                "-e".into(),
                "cycles/freq=199/u".into(),
                "-e".into(),
                format!("{GROUP}:*"),
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
                "taskset".into(),
                "-c".into(),
                c["cpu_affinity"].as_str().unwrap().into(),
                SDK.into(),
                c["image"].as_str().unwrap().into(),
                c["request"].as_str().unwrap().into(),
                "31".into(),
                dir.display().to_string(),
            ]);
            steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600}));
            jobs.push(json!({"name":name,"case":c,"block":b,"data":data,"directory":dir}));
        }
    }
    for j in &jobs {
        let name = j["name"].as_str().unwrap();
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["sudo","-n","perf","script","--ns","--header","--show-lost-events","-i",j["data"],"-F","pid,tid,time,event,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("evlist-{name}"),"argv":["sudo","-n","perf","evlist","-v","-i",j["data"]],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[r("run-helper"),"loss",j["data"]],"timeout_seconds":120}));
    }
    steps.push(json!({"kind":"command","name":"cleanup","argv":[r("run-helper"),"cleanup"],"timeout_seconds":120}));
    assert_eq!(steps.len(), 32);
    save(r("jobs.json"), &json!(jobs));
    for c in cases.as_array().unwrap().iter().take(2) {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    for f in [SDK, MANIFEST, VERIFY, X] {
        pin(&mut pins, f);
    }
    pin(&mut pins, format!("{PARENT}/run.rs"));
    for f in [
        "run.rs",
        "run-helper",
        "design.md",
        "tests.log",
        "prior-proof-sha256.json",
        "jobs.json",
        "probes.json",
        "query-disassembly.txt",
    ] {
        pin(&mut pins, r(f));
    }
    for ent in
        fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap()
    {
        pin(&mut pins, ent.unwrap().path());
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
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
    println!("Prepared32steps;6callerprofiles/204SDK/ledger15053->15059, no productionchange");
}
fn defs() -> String {
    output(&["sudo", "-n", "cat", "/sys/kernel/tracing/uprobe_events"])
}
fn write_control(s: &str) {
    assert!(read(r("probes.json"))
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p["definition"].as_str() == Some(s)
            || s == format!("-:{GROUP}/{}", p["name"].as_str().unwrap())));
    fs::OpenOptions::new()
        .write(true)
        .open("/sys/kernel/tracing/uprobe_events")
        .unwrap()
        .write_all(format!("{s}\n").as_bytes())
        .unwrap();
}
fn send(s: &str) {
    assert!(Command::new("sudo")
        .arg("-n")
        .arg(r("run-helper"))
        .args(["write-control", s])
        .status()
        .unwrap()
        .success());
}
fn install() {
    assert_eq!(hash(SDK), HASH);
    assert!(!defs().contains(&format!(":{GROUP}/")));
    for p in read(r("probes.json")).as_array().unwrap() {
        send(p["definition"].as_str().unwrap());
    }
    let s = defs();
    assert_eq!(
        s.lines()
            .filter(|l| l.contains(&format!(":{GROUP}/")))
            .count(),
        16
    );
    fs::write(r("installed.txt"), s).unwrap();
    println!("Registered16ownedprobes");
}
fn cleanup() {
    let s = defs();
    let mut names = vec![];
    for l in s.lines().filter(|l| l.contains(&format!(":{GROUP}/"))) {
        let n = l
            .split_whitespace()
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1;
        send(&format!("-:{n}"));
        names.push(n);
    }
    assert!(!defs().contains(&format!(":{GROUP}/")));
    save(
        r("cleanup.json"),
        &json!({"owned_group_absent":true,"removed":names}),
    );
    println!("Removed{}ownedprobes", names.len());
}
fn loss(p: &str) {
    let mut c = Command::new("sudo")
        .args(["-n", "perf", "script", "-D", "-i", p])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lost = vec![];
    let mut lines = 0;
    for l in BufReader::new(c.stdout.take().unwrap()).lines() {
        let l = l.unwrap();
        lines += 1;
        if l.contains("PERF_RECORD_LOST") {
            lost.push(l);
        }
    }
    let o = c.wait_with_output().unwrap();
    assert!(o.status.success());
    println!(
        "{}",
        json!({"zero_lost":lost.is_empty(),"lost":lost,"decoded_lines":lines})
    );
    assert!(lost.is_empty());
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    assert_eq!(read(r("cleanup.json"))["owned_group_absent"], true);
    for (f, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 15053);
    let mut inventory = BTreeMap::new();
    let mut traces = BTreeMap::new();
    let mut samples = 0;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
        let mut a = parser::callers(&text, 31);
        assert!(a["cycle_samples"].as_u64().unwrap() > 0);
        samples += a["cycle_samples"].as_u64().unwrap();
        let ev = fs::read_to_string(r(format!("runs/evlist-{name}/stdout.log"))).unwrap();
        let lines = ev
            .lines()
            .filter(|l| l.contains(": type:"))
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 18);
        assert_eq!(lines.iter().filter(|l| l.starts_with(GROUP)).count(), 16);
        for l in lines {
            if l.starts_with(GROUP) {
                assert!(l.contains("sample_freq }: 1,") && !l.contains("freq: 1,"));
            }
            if l.starts_with("cycles") {
                assert!(l.contains("sample_freq }: 199") && l.contains("freq: 1"));
            }
        }
        assert_eq!(
            read(r(format!("runs/loss-{name}/stdout.log")))["zero_lost"],
            true
        );
        let log = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let times = log
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| {
                serde_json::from_str::<Value>(l).unwrap()["ms"]
                    .as_f64()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(times.len(), 31);
        let summary: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(summary["workers"], j["case"]["workers"]);
        assert_eq!(summary["priming"], 3);
        a["SDK_median_ms"] = json!(median(&times));
        a["SDK_summary"] = summary;
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
        let rows = a["requests"].as_array().unwrap();
        let mut med = BTreeMap::new();
        for k in [
            "SDK_ms",
            "join_ms",
            "join_to_query_ms",
            "query_before_raw_ms",
            "grouping_ms",
            "points_ms",
            "caller_other_ms",
            "admission_ms",
            "headroom_ms",
            "raw_ms",
            "query_after_raw_ms",
            "join_after_query_ms",
        ] {
            let values = rows
                .iter()
                .filter_map(|v| v[k].as_f64())
                .collect::<Vec<_>>();
            if !values.is_empty() {
                med.insert(k, median(&values));
            }
        }
        a["medians"] = json!(med);
        println!("{name}:{}", a["medians"]);
        traces.insert(name.to_string(), a);
    }
    assert_eq!(prior.len(), 15059);
    assert_eq!(inventory.len(), 12);
    save(r("proof-inventory.json"), &json!(inventory));
    save(
        r("analysis.json"),
        &json!({"status":"complete caller diagnostic; no optimization","traces":traces,"cycle_samples":samples,"SDK_requests":204,"new_independent_exports":6,"independent_proof_count":15059,"production_changed":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"inherited_M5":read(MANIFEST)["m5"],"acceptedSDK":SDK,"acceptedSDK_sha256":HASH,"owned_probes_removed":true}),
    );
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
    println!("Complete204SDK/6freshGnark/ledger15059/{samples}cycles");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("install") => install(),
        Some("cleanup") => cleanup(),
        Some("write-control") => write_control(&std::env::args().nth(2).unwrap()),
        Some("loss") => loss(&std::env::args().nth(2).unwrap()),
        Some("finish") => finish(),
        _ => panic!("prepare|install|cleanup|write-control|loss|finish"),
    }
}
