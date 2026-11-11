#![allow(dead_code, unused_imports)]
include!("common.rs");
const P: &str = "/home/ubuntu/src/zopro-perf-work/g1-offset177";
fn cmd(name: &str, argv: Vec<String>, environment: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":600})
}
fn strings(v: Value) -> Vec<String> {
    v.as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().into())
        .collect()
}
fn check_copies() {
    use std::os::unix::fs::MetadataExt;
    let ids = read(format!("{P}/runtime-consumers.json"));
    for a in ["REF", "OFF", "ON"] {
        for c in ids[a]["copies"].as_array().unwrap() {
            let p = c["path"].as_str().unwrap();
            let m = fs::metadata(p).unwrap();
            assert_eq!(hash(p), c["sha256"]);
            assert_eq!(m.dev(), c["device"].as_u64().unwrap());
            assert_eq!(m.ino(), c["inode"].as_u64().unwrap());
            assert_eq!(m.mode() as u64, c["mode"].as_u64().unwrap());
        }
    }
}
fn prepare() {
    assert!(!r("plan.json").exists());
    check_copies();
    assert_eq!(read(SEED).as_array().unwrap().len(), 52945);
    let ids = read(format!("{P}/runtime-consumers.json"));
    let previous = read(format!("{P}/main-jobs.json"));
    let j = previous
        .as_array()
        .unwrap()
        .iter()
        .find(|j| j["case"]["name"] == "1x2-w32")
        .unwrap();
    let case = j["case"].clone();
    let base = j["environment"].clone();
    let mut jobs = Vec::new();
    let mut steps = Vec::new();
    fs::create_dir(r("perf")).unwrap();
    for f in ["ctl", "ack"] {
        assert!(
            Command::new("mkfifo")
                .args(["-m", "600"])
                .arg(r(f))
                .status()
                .unwrap()
                .success()
        );
    }
    steps.push(cmd(
        "pages-before",
        strings(json!(["sudo", "-n", r("pages-before/census"), "capture"])),
        json!({}),
    ));
    for block in 0..3 {
        let mut panels = vec!["plain", "op", "fetch"];
        panels.rotate_left(block);
        if block % 2 == 1 {
            panels.reverse();
        }
        for (pi, panel) in panels.iter().enumerate() {
            let mut copies = vec![0, 1, 2];
            copies.rotate_left((block + pi) % 3);
            if block % 2 == 1 {
                copies.reverse();
            }
            for copy in copies {
                let name = format!("b{block}-{panel}-c{copy}");
                let binary = ids["OFF"]["copies"][copy]["path"].as_str().unwrap();
                let mut env = base.clone();
                let mut argv = Vec::new();
                let data = r(format!("perf/{name}.data"));
                let dir = r(format!("proofs/{name}"));
                if *panel != "plain" {
                    env["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
                    env["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
                    argv = strings(json!([
                        "sudo",
                        "-n",
                        "perf",
                        "record",
                        "--no-buildid-cache",
                        "--sample-cpu",
                        "--timestamp",
                        "-e",
                        if *panel == "op" {
                            "ibs_op/cnt_ctl=1/"
                        } else {
                            "ibs_fetch/rand_en=1/"
                        },
                        "-c",
                        "524288",
                        "-m",
                        "1024",
                        "--raw-samples"
                    ]));
                    if *panel == "op" {
                        argv.extend(strings(json!(["-d", "-W"])));
                    }
                    argv.extend(strings(json!([
                        "--delay=-1",
                        format!(
                            "--control=fifo:{},{}",
                            r("ctl").display(),
                            r("ack").display()
                        ),
                        "-o",
                        data,
                        "--",
                        "env"
                    ])));
                    for (k, v) in env.as_object().unwrap() {
                        argv.push(format!("{k}={}", v.as_str().unwrap()));
                    }
                }
                argv.extend(strings(json!([
                    "/usr/bin/taskset",
                    "-c",
                    "0-31",
                    binary,
                    case["image"],
                    case["request"],
                    "61",
                    dir
                ])));
                steps.push(cmd(
                    &name,
                    argv,
                    if *panel == "plain" {
                        env.clone()
                    } else {
                        json!({})
                    },
                ));
                jobs.push(json!({"name":name,"case":case,"copy":copy,"block":block,"panel":panel,"rounds":61,"binary":binary,"proof_dir":dir,"data":data,"environment":env}));
            }
        }
    }
    assert_eq!(jobs.len(), 27);
    steps.push(cmd(
        "pages-after",
        strings(json!(["sudo", "-n", r("pages-after/census"), "capture"])),
        json!({}),
    ));
    for j in &jobs {
        let name = j["name"].as_str().unwrap();
        let d = j["proof_dir"].as_str().unwrap();
        steps.push(cmd(
            &format!("verify-{name}"),
            strings(json!([
                VERIFY,
                case["image"],
                case["image_sha256"],
                format!("{d}/0.proof"),
                format!("{d}/0.public"),
                case["public_sha256"]
            ])),
            json!({}),
        ));
    }
    steps.push(cmd(
        "disassembly",
        strings(json!([
            "objdump",
            "-d",
            "-M",
            "intel",
            "--no-show-raw-insn",
            ids["OFF"]["copies"][0]["path"]
        ])),
        json!({}),
    ));
    for j in jobs.iter().filter(|j| j["panel"] != "plain") {
        let name = j["name"].as_str().unwrap();
        let data = &j["data"];
        let mode = j["panel"].as_str().unwrap();
        let copy = j["copy"].as_u64().unwrap();
        steps.push(cmd(
            &format!("own-{name}"),
            strings(json!(["sudo", "-n", "chown", "ubuntu:ubuntu", data])),
            json!({}),
        ));
        steps.push(cmd(
            &format!("raw-{name}"),
            strings(json!(["sudo", "-n", "perf", "report", "-D", "-i", data])),
            json!({}),
        ));
        let fields = if mode == "op" {
            "pid,tid,time,ip,weight,data_src,sym,symoff,dso"
        } else {
            "pid,tid,time,ip,sym,symoff,dso"
        };
        steps.push(cmd(
            &format!("script-{name}"),
            strings(json!([
                "sudo",
                "-n",
                "perf",
                "script",
                "--ns",
                "--header",
                "--show-lost-events",
                "--no-demangle",
                "-i",
                data,
                "-F",
                fields
            ])),
            json!({}),
        ));
        steps.push(cmd(
            &format!("metadata-{name}"),
            strings(json!([
                "sudo",
                "-n",
                "perf",
                "script",
                "--ns",
                "--header",
                "--show-mmap-events",
                "--show-task-events",
                "-i",
                data,
                "-F",
                "pid,tid,cpu,time,ip"
            ])),
            json!({}),
        ));
        steps.push(cmd(
            &format!("decode-{name}"),
            strings(json!([
                r(format!("decode-{copy}")),
                mode,
                r(format!("run/raw-{name}/stdout.log")),
                r(format!("run/script-{name}/stdout.log")),
                r("run/disassembly/stdout.log"),
                r(format!("{name}-analysis.json"))
            ])),
            json!({}),
        ));
    }
    save("jobs.json", &json!(jobs));
    let mut pins = BTreeMap::new();
    for f in [
        "protocol.md",
        "setup.rs",
        "common.rs",
        "flow.rs",
        "flow",
        "legacy-decoder.rs",
        "jobs.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    for i in 0..3 {
        for f in [format!("decode-{i}.rs"), format!("decode-{i}")] {
            pins.insert(r(&f).display().to_string(), hash(r(f)));
        }
    }
    for phase in ["before", "after"] {
        for f in ["census.rs", "census"] {
            let p = r(format!("pages-{phase}/{f}"));
            pins.insert(p.display().to_string(), hash(&p));
        }
    }
    for a in ["REF", "OFF", "ON"] {
        for c in ids[a]["copies"].as_array().unwrap() {
            let p = c["path"].as_str().unwrap();
            pins.insert(p.into(), hash(p));
        }
    }
    for p in [
        SEED,
        VERIFY,
        "/usr/bin/perf",
        "/usr/bin/taskset",
        "/usr/bin/objdump",
        case["image"].as_str().unwrap(),
        case["request"].as_str().unwrap(),
        &format!("{P}/runtime-consumers.json"),
    ] {
        pins.insert(p.into(), hash(p));
    }
    assert_eq!(hash(case["image"].as_str().unwrap()), case["image_sha256"]);
    save("pins.json", &json!(pins));
    save(
        "plan.json",
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!(
        "Prepared27SDKprocesses/1728calls/27exports,18IBSprofiles,2isolatedpagecensuses; no capture yet"
    );
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn finish() {
    assert!(!r("result.json").exists());
    assert_eq!(read(r("postrun/result.json"))["status"], "complete");
    check_copies();
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for s in read(r("plan.json"))["steps"].as_array().unwrap() { if s["name"]=="raw-b0-op-c1" {break;} receipt("run",s); }
    for s in read(r("post-plan.json"))["steps"].as_array().unwrap() {receipt("postrun",s);}
    let mut ledger: BTreeSet<String> = serde_json::from_value(read(SEED)).unwrap();
    assert_eq!(ledger.len(), 52945);
    let mut rows = Vec::new();
    for j in read(r("jobs.json")).as_array().unwrap() {
        let (samples, summary) = sdk("run", j);
        let binding = proof("run", j, &mut ledger);
        let mean = samples.iter().map(|n| *n as f64).sum::<f64>() / samples.len() as f64 / 1e6;
        let name = j["name"].as_str().unwrap();
        let panel = j["panel"].as_str().unwrap();
        let diagnostic = if panel == "plain" {
            Value::Null
        } else {
            let a = read(r(format!("{name}-analysis.json")));
            let pid = a["user_pids"].as_array().unwrap();
            assert_eq!(pid.len(), 1);
            let meta = fs::read_to_string(r(format!("postrun/metadata-{name}/stdout.log"))).unwrap();
            let matches = meta
                .lines()
                .filter(|l| {
                    l.contains("PERF_RECORD_MMAP2") && l.ends_with(j["binary"].as_str().unwrap())
                })
                .collect::<Vec<_>>();
            assert_eq!(matches.len(), 1);
            let actual = matches[0]
                .split_whitespace()
                .next()
                .unwrap()
                .split('/')
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            assert_eq!(pid[0], actual);
            let workers = summary["placement_before"]["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|w| {
                    (
                        w["tid"].as_u64().unwrap(),
                        w["allowed"][0].as_u64().unwrap(),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            for l in meta.lines().filter(|l| {
                !l.starts_with('#') && !l.contains("PERF_RECORD") && !l.trim().is_empty()
            }) {
                let p = l.split_whitespace().collect::<Vec<_>>();
                if p.len() < 4 {
                    continue;
                }
                let Some((p1, t)) = p[0].split_once('/') else {
                    continue;
                };
                let p1 = p1.parse::<u64>().unwrap();
                let t = t.parse::<u64>().unwrap();
                let cpu = p[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                assert_eq!(p1, actual);
                if let Some(&c) = workers.get(&t) {
                    assert_eq!(cpu, c);
                } else {
                    assert_eq!(t, actual);
                    assert!(cpu < 32);
                }
            }
            let g2 = &a["groups"]["user/G2feed"];
            let enough = g2["samples"].as_u64().unwrap() >= if panel == "op" { 1000 } else { 300 };
            let resolved =
                panel != "op" || a["unresolved_SDK_symbols"].as_object().unwrap().is_empty();
            json!({"qualified":enough&&resolved&&a["lost_records"]==0,"analysis":a,"perf_data_sha256":hash(j["data"].as_str().unwrap()),"actual_PID_TID_CPU_and_MMAP_checked":true})
        };
        rows.push(json!({"copy":j["copy"],"block":j["block"],"panel":panel,"mean_ms":mean,"summary":summary,"binding":binding,"diagnostic":diagnostic}));
    }
    assert_eq!(ledger.len(), 52972);
    let mut observer = Vec::new();
    for copy in 0..3 {
        for panel in ["op", "fetch"] {
            let ratios = (0..3)
                .map(|b| {
                    let get = |p: &str| {
                        rows.iter()
                            .find(|r| r["copy"] == copy && r["block"] == b && r["panel"] == p)
                            .unwrap()["mean_ms"]
                            .as_f64()
                            .unwrap()
                    };
                    get(panel) / get("plain")
                })
                .collect::<Vec<_>>();
            let m = median(&ratios);
            observer.push(json!({"copy":copy,"panel":panel,"ratios":ratios,"median_ratio":m,"passed":(1./1.05..=1.05).contains(&m)}));
        }
    }
    let frame_map = |phase: &str| {
        let v = read(r(format!("pages-{phase}/raw-private.json")));
        let mut out = BTreeMap::new();
        for r in v["records"].as_array().unwrap() {
            for s in r["segments"].as_array().unwrap() {
                for p in s["pages"].as_array().unwrap() {
                    out.insert(
                        format!(
                            "{}/{}/{}",
                            r["arm"].as_str().unwrap(),
                            r["copy"],
                            p["file_offset"]
                        ),
                        p["pfn"].as_u64().unwrap(),
                    );
                }
            }
        }
        out
    };
    let before = frame_map("before");
    let after = frame_map("after");
    assert_eq!(
        before.keys().collect::<Vec<_>>(),
        after.keys().collect::<Vec<_>>()
    );
    let changes = before
        .iter()
        .filter(|(k, v)| after.get(*k) != Some(*v))
        .count();
    let quality = rows
        .iter()
        .filter(|r| r["panel"] != "plain")
        .all(|r| r["diagnostic"]["qualified"] == true);
    let observers = observer.iter().all(|c| c["passed"] == true);
    save("proof-sha256-after-profile.json", &json!(ledger));
    save(
        "result.json",
        &json!({"rows":rows,"observer_checks":observer,"quality_passed":quality,"observer_checks_passed":observers,"page_frames_compared":before.len(),"page_frames_changed_between_endpoints":changes,"fresh_verified_exports":27,"canonical_count":52972,"diagnostic_only":true}),
    );
    println!(
        "Verified27freshproofs/ledger52972; quality={quality}, observer={observers}; changedpageframes={changes}"
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare|finish"),
    }
}
