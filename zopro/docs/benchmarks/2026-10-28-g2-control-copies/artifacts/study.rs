use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-control-copies";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-control-copies";
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
fn env() -> Value {
    json!({"MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","RAYON_NUM_THREADS":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_MSM_SEED":"17","ZOPRO_BENCH_PERF_CTL":r("shared-ctl"),"ZOPRO_BENCH_PERF_ACK":r("shared-ack")})
}
fn setup() {
    assert!(!r("run-plan.json").exists());
    let mut pins = BTreeMap::new();
    for (p, h) in read(format!("{P}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.to_owned(), h.as_str().unwrap().to_owned());
    }
    let cases = [
        ("original-f", "frozen-probe", false),
        ("original-o", "off-probe", false),
        ("copy-001-f", "frozen-probe", true),
        ("copy-002-f", "frozen-probe", true),
        ("copy-001-o", "off-probe", true),
        ("copy-002-o", "off-probe", true),
    ];
    let mut ids = BTreeMap::new();
    for (name, source, copy) in cases {
        let p = format!("{P}/{source}");
        let meta = fs::metadata(&p).unwrap();
        if copy {
            let bytes = fs::read(&p).unwrap();
            fs::write(r(name), bytes).unwrap();
            fs::set_permissions(r(name), meta.permissions()).unwrap();
        } else {
            fs::hard_link(&p, r(name)).unwrap();
        }
        let out = fs::metadata(r(name)).unwrap();
        assert_eq!(hash(&p), hash(r(name)));
        assert_eq!(copy, out.ino() != meta.ino());
        assert_eq!(name.len(), 10);
        ids.insert(name,json!({"source":p,"new_file":copy,"path":r(name),"inode":out.ino(),"device":out.dev(),"sha256":hash(r(name)),"bytes":out.len(),"path_bytes":r(name).as_os_str().len()}));
        pins.insert(r(name).display().to_string(), hash(r(name)));
    }
    assert_ne!(ids["copy-001-f"]["inode"], ids["copy-002-f"]["inode"]);
    assert_ne!(ids["copy-001-o"]["inode"], ids["copy-002-o"]["inode"]);
    save(r("identities.json"), &json!(ids));
    for f in ["shared-ctl", "shared-ack"] {
        assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());
    }
    for f in [
        "design.md",
        "study.rs",
        "study",
        "identities.json",
        "capability-fetch.stdout.log",
        "capability-fetch.stderr.log",
        "capability-stall.stdout.log",
        "capability-stall.stderr.log",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    for f in ["analysis.json", "identities.json", "design.md"] {
        let p = format!("/home/ubuntu/src/zopro-perf-work/g2-control-identity/{f}");
        pins.insert(p.clone(), hash(p));
    }
    for (name, args) in [
        (
            "host-before",
            vec!["ps", "-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"],
        ),
        ("compiler", vec!["rustc", "-vV"]),
    ] {
        let o = Command::new(args[0]).args(&args[1..]).output().unwrap();
        assert!(o.status.success());
        fs::write(r(format!("{name}.txt")), o.stdout).unwrap();
        pins.insert(
            r(format!("{name}.txt")).display().to_string(),
            hash(r(format!("{name}.txt"))),
        );
    }
    let panels = [
        (
            "fetch",
            "{cycles:u,instructions:u,L1-icache-load-misses:u,iTLB-load-misses:u}",
        ),
        (
            "stall",
            "{cycles:u,instructions:u,stalled-cycles-frontend:u,L1-dcache-load-misses:u}",
        ),
    ];
    let mut jobs = vec![];
    let mut steps = vec![];
    for b in 0..3 {
        let mut order: Vec<_> = (0..6).collect();
        order.rotate_left(b);
        if b % 2 != 0 {
            order.reverse();
        }
        for (i, c) in order.into_iter().enumerate() {
            let mut panel_order = vec![0, 1];
            if (b + i) % 2 != 0 {
                panel_order.reverse();
            }
            for pi in panel_order {
                let name = cases[c].0;
                let (panel, event) = panels[pi];
                let job = format!("b{b}-{name}-{panel}");
                let e = env();
                let mut argv = vec!["sudo".to_string(), "-n".into(), "env".into()];
                for (k, v) in e.as_object().unwrap() {
                    argv.push(format!("{k}={}", v.as_str().unwrap()));
                }
                argv.extend([
                    "perf".into(),
                    "stat".into(),
                    "--no-big-num".into(),
                    "-x".into(),
                    ";".into(),
                    "-e".into(),
                    event.into(),
                    "--delay=-1".into(),
                    format!(
                        "--control=fifo:{},{}",
                        r("shared-ctl").display(),
                        r("shared-ack").display()
                    ),
                    "--".into(),
                    r(name).display().to_string(),
                    "g2-msm".into(),
                    "30089".into(),
                    "0".into(),
                    "solo".into(),
                    "7".into(),
                ]);
                steps.push(json!({"kind":"command","name":job,"argv":argv,"environment":e,"timeout_seconds":300}));
                jobs.push(json!({"name":job,"block":b,"case":name,"panel":panel}));
            }
        }
    }
    assert_eq!(steps.len(), 36);
    save(r("jobs.json"), &json!(jobs));
    pins.insert(r("jobs.json").display().to_string(), hash(r("jobs.json")));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared36fresh-backing/frontendcounterprocesses;allpathslength10/sharedFIFO/mode0");
}
fn median(v: &[f64]) -> f64 {
    let mut x = v.to_vec();
    x.sort_by(f64::total_cmp);
    x[x.len() / 2]
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut rows = BTreeMap::new();
    let mut digest = None;
    let mut running = true;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let line = text
            .lines()
            .find_map(|s| s.strip_prefix("RESULT "))
            .unwrap();
        let v: Value = serde_json::from_str(line).unwrap();
        for (k, n) in [
            ("size", 30089),
            ("mode", 0),
            ("input_seed", 17),
            ("rounds", 7),
            ("prime_calls", 3),
            ("worker_count", 1),
        ] {
            assert_eq!(v[k], n);
        }
        assert_eq!(v["cpus"], json!([2]));
        assert_eq!(v["internally_verified"], true);
        assert_eq!(v["heap_instrumented"], false);
        if let Some(old) = digest.replace(v["output_sha256"].clone()) {
            assert_eq!(old, v["output_sha256"]);
        }
        let stderr = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
        let mut counters = BTreeMap::new();
        for l in stderr.lines() {
            let x: Vec<_> = l.trim().split(';').collect();
            if x.len() < 5 {
                continue;
            }
            let event = x[2].strip_suffix(":u").unwrap_or(x[2]);
            if ![
                "cycles",
                "instructions",
                "L1-icache-load-misses",
                "iTLB-load-misses",
                "stalled-cycles-frontend",
                "L1-dcache-load-misses",
            ]
            .contains(&event)
            {
                continue;
            }
            let value: f64 = x[0].trim().parse().unwrap();
            let pct: f64 = x[4].trim().parse().unwrap();
            assert!(pct > 0.);
            running &= pct == 100.;
            counters.insert(
                event,
                json!({"count":value,"per_MSM":value/7.,"running_percent":pct}),
            );
        }
        assert_eq!(counters.len(), 4);
        rows.insert(name.to_owned(),json!({"case":j["case"],"panel":j["panel"],"block":j["block"],"result":v,"counters":counters}));
    }
    let mut cells = BTreeMap::new();
    for c in [
        "original-f",
        "original-o",
        "copy-001-f",
        "copy-002-f",
        "copy-001-o",
        "copy-002-o",
    ] {
        for panel in ["fetch", "stall"] {
            let rs: Vec<_> = (0..3)
                .map(|b| &rows[&format!("b{b}-{c}-{panel}")])
                .collect();
            let mut stats = BTreeMap::new();
            for m in ["ns_per_MSM", "CPU_ms_per_MSM"] {
                let v: Vec<_> = rs
                    .iter()
                    .map(|v| v["result"][m].as_f64().unwrap())
                    .collect();
                stats.insert(m, json!({"median":median(&v),"observations":v}));
            }
            for m in rs[0]["counters"].as_object().unwrap().keys() {
                let v: Vec<_> = rs
                    .iter()
                    .map(|v| v["counters"][m]["per_MSM"].as_f64().unwrap())
                    .collect();
                stats.insert(m, json!({"median":median(&v),"observations":v}));
            }
            cells.insert(format!("{c}/{panel}"), json!(stats));
        }
    }
    save(
        r("analysis.json"),
        &json!({"scope":"fresh byte-exact copies/sharedFIFO/frontenddiagnostic; no optimizationgate","cells":cells,"rows":rows,"processes":36,"counted_MSM_calls":252,"primes":108,"all_counters_100_percent_running":running,"pins_verified":pins.as_object().unwrap().len(),"production_changed":false,"proof_ledger":13181,"SDK_requests":0,"new_Gnark_exports":0,"goal_status":"active","CPU10x_rows":0}),
    );
    let o = Command::new("ps")
        .args(["-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"])
        .output()
        .unwrap();
    assert!(o.status.success());
    fs::write(r("host-after.txt"), o.stdout).unwrap();
    println!("36copies/frontendprocesseschecked;countergroups100%={running};pinsverified");
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(r("analysis.json"));
    assert_eq!(a["processes"], 36);
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    for p in files {
        let b = fs::read(&p).unwrap();
        if b.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let q = Path::new(D)
            .join("artifacts")
            .join(p.strip_prefix(R).unwrap());
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::write(q, b).unwrap();
    }
    save(
        format!("{D}/artifacts/local-executable-pins.json"),
        &json!(local),
    );
    let mut s=String::from("# Fresh control backing and frontend counters\n\nThis diagnostic compares original control backing with two new byte-exact files from each source. All six executable names have equal lengths in one directory; mode0,seed17,CPU2,arguments and shared FIFO environment values are identical. Original aliases retain parent inodes; fresh files have new inodes and exact source hashes. Original sources and executables are unchanged.\n\nThree fixed rotated/reversed blocks cover six files and two grouped panels:36processes. Each has3checkedprimes+7counted completeMSMs at30,089points. All outputs agree with the generator oracle and each other. Capability-only true commands demonstrate both panels can count; their logs are retained. Cache event names omit:u in perf output despite explicit userland requests, so labels are normalized.\n\n| Image/panel | ms/MSM | CPU ms/MSM | Cycles/MSM | Instructions/MSM | L1I misses/MSM | iTLB misses/MSM | Frontend stalled cycles/MSM | L1D misses/MSM |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["cells"].as_object().unwrap() {
        let val = |m: &str| {
            v[m]["median"]
                .as_f64()
                .map(|x| format!("{x:.3}"))
                .unwrap_or_else(|| "—".into())
        };
        writeln!(
            s,
            "| {k} | {:.6} | {} | {} | {} | {} | {} | {} | {} |",
            v["ns_per_MSM"]["median"].as_f64().unwrap() / 1e6,
            val("CPU_ms_per_MSM"),
            val("cycles"),
            val("instructions"),
            val("L1-icache-load-misses"),
            val("iTLB-load-misses"),
            val("stalled-cycles-frontend"),
            val("L1-dcache-load-misses")
        )
        .unwrap();
    }
    writeln!(s,"\nAll groups100%running: **{}**. All3observations are retained. These descriptive panels do not supply confidence/adoption gates or isolate a unique microarchitectural cause. Different inodes do not prove independent physical placement. Generic cache/stall events are not DRAM bandwidth, exclusive instruction costs or removable proof latency.\n",a["all_counters_100_percent_running"]).unwrap();
    s.push_str("There are252countedMSMs+108primes, no new native builds, SDK/Gnark/proofs or optimization acceptance measurement. No ASLR/globalkernel/cache settings change. All pinned sources/images/compiler/helper inputs are rechecked after capture. No owned build/helper/hash/archive/otherbenchmark overlaps counters; host snapshots do not prove exclusive use. Goal remainsACTIVE0/8, proofledger13,181, all172production/SDK14da unchanged.\n\nThe inherited accepted all-eight CPU/Metal figures remain in the [borrowed-queue report](../2026-10-28-g2-borrowed-queue/README.md):primary8.324853ms versus37ms M5CPU/24ms M5Metal,4.445×/2.883×. Target3.7ms remains unmet. Primaryhistoricalpeak134.50MiB; merge54 161.341835ms/1910.1875MiB without a suppliedM5reference. No new full-proof speed, mobile/TEE/minRAM/pressure/giant/AWS/Aeglos claim. The parent borrowed-queue gate remains failed and its positivephysical16/memory observations retained.\n\n[Design](artifacts/design.md), [file identities](artifacts/identities.json), [all observations](artifacts/analysis.json), [workflow](artifacts/runs/result.json), [prior identity diagnostic](../2026-10-28-g2-control-identity/README.md). Captured logs retain bytes including trailing blank lines; authored source/report whitespace checked separately.\n");
    if r("interpretation.md").exists() {
        s.push_str(&fs::read_to_string(r("interpretation.md")).unwrap());
    }
    fs::write(format!("{D}/README.md"), s).unwrap();
    let mut all = vec![];
    walk(Path::new(D), &mut all);
    all.sort();
    let mut sums = String::new();
    for p in &all {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        )
        .unwrap();
    }
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    let check = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    assert!(check.status.success());
    fs::write(r("archive-verification.log"), check.stdout).unwrap();
    save(
        r("archive-summary.json"),
        &json!({"artifact_hashes":all.len(),"Git_files":all.len()+1,"goal_status":"active","proof_ledger":13181}),
    );
    println!("Archived{}strictverifiedhashes", all.len());
}
fn state() {
    let o = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir("/home/ubuntu/src/zk-tools")
        .output()
        .unwrap();
    assert!(o.status.success());
    let head = String::from_utf8(o.stdout).unwrap().trim().to_string();
    let remote = Command::new("git")
        .args([
            "-c",
            "url.https://github.com/.insteadOf=git@github.com:",
            "-c",
            "credential.helper=!gh auth git-credential",
            "ls-remote",
            "origin",
            "refs/heads/sergei/zopro-cpu-perf",
        ])
        .current_dir("/home/ubuntu/src/zk-tools")
        .output()
        .unwrap();
    assert!(remote.status.success());
    assert_eq!(
        String::from_utf8(remote.stdout)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap(),
        head
    );
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut v = read(p);
    v["previous_completed_experiment"] = v["commit"].clone();
    v["commit"] = json!(head);
    v["root"] = json!(R);
    v["report"] = json!(format!("{D}/README.md"));
    v["status"] =
        json!("Fresh byte-exact backing/frontend diagnostic complete and pushed; noadoption");
    v["control_copies_diagnostic"] = read(r("analysis.json"));
    v["next_action"]=json!("Read fresh-copy/frontend evidence to resolve baseline comparability; retain all failed gates and positive borrowedG2 estimates; no unchanged optimization reroll");
    v["last_goal_turn_classification"] = json!("progress");
    v["goal_status"] = json!("active");
    v["goal_achieved"] = json!(false);
    v["pushed"] = json!(true);
    v["remote_verified"] = json!(true);
    v["owned_live_jobs"] = json!([]);
    v["owned_cloud_resources"] = json!([]);
    let x = read(r("archive-summary.json"));
    v["artifact_hashes"] = x["artifact_hashes"].clone();
    v["git_files_new_report"] = x["Git_files"].clone();
    save(p, &v);
    let handoff = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(handoff).unwrap();
    let header=format!("CURRENT COMPLETE/PUSHED — FRESH CONTROL BACKING/FRONTEND\n\nHEAD/remoteverified{head};root{R};report{D}/README.md.36counterprocesses3blocks6images2panels,252MSMs+108primes;no candidate/nativebuild/SDKproof. Originalaliases+2freshbyteexactfiles/source;allnames10bytes/mode0/sharedFIFO/seed17/CPU2. Fetchcycles/instructions/L1I/iTLB,stallcycles/instructions/frontend/L1D capabilitiescounted first;allactualpercentagesverified. Readanalysis/interpretationforresults;noadoption/nooldgatechange. Production172/SDK14da/historical1x2 8.324853ms/RSS134.50MiB unchanged;goalACTIVE0/8;proofledger13181;noownedjobs/cloud. ThisturnPROGRESS borrowedG2fullMSMmemorya30e720d+identity47a634d8+thisdistinctcopydiagnostic. Parentcomponentfailedgate/positivephysical16and1.35MiBrequestedpeaksaving retained.\n\nNEXT:{}. No unchangedoptimizationreroll. Allsource/image/helperpinschecked,nohelper/build/hash/archiveoverlapcapture,noASLR/globalsettingschanged,noexclusivehostclaim. No actualprooftiminggain/mobile/pressure/AWSclaim. CommitANDpushcompletedexperiments,noquestions/subagents/hugecaches/Casmrewrite. Olderheadershistorical.\n\n",v["next_action"]);
    fs::write(handoff, format!("{header}{old}")).unwrap();
    println!("Verifiedpush{head};state/handoff updated;goalactive0/8");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "setup" => setup(),
        "finish" => finish(),
        "archive" => archive(),
        "state" => state(),
        _ => panic!("mode"),
    }
}
