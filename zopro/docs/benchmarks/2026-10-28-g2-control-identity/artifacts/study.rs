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
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-control-identity";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-control-identity";
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
    json!({"MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","RAYON_NUM_THREADS":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_MSM_SEED":"17"})
}
fn setup() {
    assert!(!r("run-plan.json").exists());
    let parent = read(format!("{P}/pins.json"));
    let mut pins = BTreeMap::new();
    for (p, h) in parent.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.to_owned(), h.as_str().unwrap().to_owned());
    }
    fs::hard_link(format!("{P}/frozen-probe"), r("control-a")).unwrap();
    fs::hard_link(format!("{P}/off-probe"), r("control-b")).unwrap();
    assert_eq!(
        r("control-a").as_os_str().len(),
        r("control-b").as_os_str().len()
    );
    let cases = [
        ("frozen0", format!("{P}/frozen-probe"), 0),
        ("frozen1", format!("{P}/frozen-probe"), 1),
        ("off0", format!("{P}/off-probe"), 0),
        ("off1", format!("{P}/off-probe"), 1),
        ("alias_frozen0", r("control-a").display().to_string(), 0),
        ("alias_off0", r("control-b").display().to_string(), 0),
    ];
    let mut identities = BTreeMap::new();
    for (label, path, mode) in &cases {
        let meta = fs::metadata(path).unwrap();
        identities.insert(*label,json!({"path":path,"mode":mode,"inode":meta.ino(),"device":meta.dev(),"bytes":meta.len(),"sha256":hash(path),"path_bytes":path.len()}));
        pins.insert(path.to_owned(), hash(path));
    }
    assert_eq!(
        identities["frozen0"]["inode"],
        identities["alias_frozen0"]["inode"]
    );
    assert_eq!(
        identities["off0"]["inode"],
        identities["alias_off0"]["inode"]
    );
    save(r("identities.json"), &json!(identities));
    for f in ["design.md", "study.rs", "study", "identities.json"] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    for f in [
        "probe.rs",
        "perf.rs",
        "analysis.json",
        "source-audit.json",
        "control-sections.json",
        "control-locations.json",
        "pins.json",
    ] {
        let p = format!("{P}/{f}");
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
    let mut jobs = vec![];
    let mut steps = vec![];
    for b in 0..7 {
        let mut order: Vec<_> = (0..6).collect();
        order.rotate_left(b % 6);
        if b % 2 != 0 {
            order.reverse();
        }
        for i in order {
            let (label, path, mode) = &cases[i];
            let name = format!("b{b}-{label}");
            let dir = r("fifos").join(&name);
            fs::create_dir_all(&dir).unwrap();
            for f in ["ctl", "ack"] {
                assert!(Command::new("mkfifo")
                    .arg(dir.join(f))
                    .status()
                    .unwrap()
                    .success());
            }
            let mut e = env();
            e["ZOPRO_BENCH_PERF_CTL"] = json!(dir.join("ctl"));
            e["ZOPRO_BENCH_PERF_ACK"] = json!(dir.join("ack"));
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
                "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    dir.join("ctl").display(),
                    dir.join("ack").display()
                ),
                "--".into(),
                path.to_owned(),
                "g2-msm".into(),
                "30089".into(),
                mode.to_string(),
                "solo".into(),
                "7".into(),
            ]);
            steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":e,"timeout_seconds":300}));
            jobs.push(json!({"name":name,"block":b,"case":label,"path":path,"mode":mode}));
        }
    }
    assert_eq!(steps.len(), 42);
    save(r("jobs.json"), &json!(jobs));
    pins.insert(r("jobs.json").display().to_string(), hash(r("jobs.json")));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared42newcontrol-identitycounterprocesses;noONcandidate/rerun/adoptiongate");
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
    let jobs = read(r("jobs.json"));
    let mut rows = BTreeMap::new();
    let mut digest = None;
    let mut all_running = true;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|s| s.strip_prefix("RESULT "))
            .collect();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(v["size"], 30089);
        assert_eq!(v["mode"], j["mode"]);
        assert_eq!(v["input_seed"], 17);
        assert_eq!(v["rounds"], 7);
        assert_eq!(v["prime_calls"], 3);
        assert_eq!(v["internally_verified"], true);
        assert_eq!(v["heap_instrumented"], false);
        assert_eq!(v["worker_count"], 1);
        assert_eq!(v["cpus"], json!([2]));
        if let Some(old) = digest.replace(v["output_sha256"].clone()) {
            assert_eq!(old, v["output_sha256"]);
        }
        let stderr = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
        let mut counters = BTreeMap::new();
        for line in stderr.lines() {
            let x: Vec<_> = line.trim().split(';').collect();
            if x.len() < 5 || !x[2].ends_with(":u") {
                continue;
            }
            let value: f64 = x[0].trim().parse().unwrap();
            let pct: f64 = x[4].trim().parse().unwrap();
            assert!(pct > 0.);
            all_running &= pct == 100.;
            counters.insert(
                x[2],
                json!({"count":value,"per_MSM":value/7.,"running_percent":pct}),
            );
        }
        assert_eq!(counters.len(), 4);
        rows.insert(
            name.to_owned(),
            json!({"case":j["case"],"block":j["block"],"result":v,"counters":counters}),
        );
    }
    let mut cells = BTreeMap::new();
    for c in [
        "frozen0",
        "frozen1",
        "off0",
        "off1",
        "alias_frozen0",
        "alias_off0",
    ] {
        let rs: Vec<_> = (0..7).map(|b| &rows[&format!("b{b}-{c}")]).collect();
        let mut stats = BTreeMap::new();
        for m in ["ns_per_MSM", "CPU_ms_per_MSM"] {
            let v: Vec<_> = rs
                .iter()
                .map(|v| v["result"][m].as_f64().unwrap())
                .collect();
            stats.insert(m, json!({"median":median(&v),"observations":v}));
        }
        for m in [
            "cycles:u",
            "instructions:u",
            "cache-misses:u",
            "branch-misses:u",
        ] {
            let v: Vec<_> = rs
                .iter()
                .map(|v| v["counters"][m]["per_MSM"].as_f64().unwrap())
                .collect();
            stats.insert(m, json!({"median":median(&v),"observations":v}));
        }
        cells.insert(c, json!(stats));
    }
    let mut pairs = BTreeMap::new();
    for (a, b) in [
        ("frozen1", "frozen0"),
        ("off1", "off0"),
        ("off0", "frozen0"),
        ("off1", "frozen1"),
        ("alias_frozen0", "frozen0"),
        ("alias_off0", "off0"),
        ("alias_off0", "alias_frozen0"),
    ] {
        let v: Vec<_> = (0..7)
            .map(|i| {
                rows[&format!("b{i}-{a}")]["result"]["ns_per_MSM"]
                    .as_f64()
                    .unwrap()
                    / rows[&format!("b{i}-{b}")]["result"]["ns_per_MSM"]
                        .as_f64()
                        .unwrap()
            })
            .collect();
        pairs.insert(
            format!("{a}/{b}"),
            json!({"median":median(&v),"ratios":v,"wins":v.iter().filter(|v|**v<1.).count()}),
        );
    }
    let a = json!({"scope":"same-code controlidentity diagnostic; no optimization acceptance inference","cells":cells,"pairs":pairs,"rows":rows,"processes":42,"counted_MSM_calls":294,"primes":126,"all_counters_100_percent_running":all_running,"pins_verified":pins.as_object().unwrap().len(),"production_changed":false,"SDK_requests":0,"new_Gnark_exports":0,"proof_ledger":13181,"goal_status":"active","CPU10x_rows":0});
    save(r("analysis.json"), &a);
    let o = Command::new("ps")
        .args(["-eo", "pid,comm,pcpu,pmem,etimes", "--sort=-pcpu"])
        .output()
        .unwrap();
    assert!(o.status.success());
    fs::write(r("host-after.txt"), o.stdout).unwrap();
    println!(
        "Controlidentitydiagnostic complete; counters100%={all_running};{}pinsverified",
        pins.as_object().unwrap().len()
    );
    println!("{}", a["pairs"]);
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            if e.file_name() != "fifos" {
                walk(&e.path(), out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(r("analysis.json"));
    assert_eq!(a["processes"], 42);
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
    let mut s=String::from("# Same-code executable identity diagnostic\n\nThis compares unchanged frozen/OFF G2 MSM executables while varying mode label and pathname. It does not rerun the borrowed-queue optimization gate or qualify a candidate. Production and accepted SDK remain unchanged; the original component gate remains failed.\n\nSeven fixed rotated/reversed blocks cover six cases,42FIFOgrouped counter processes. Each uses30,089points,onepoolworker pinnedCPU2,seed17,3checkedprimes and7measuredMSMs. Every output matches the generator oracle and all outputs agree across cases. No candidate ON is included.\n\nOriginal frozen modes0/1 and original OFF modes0/1 isolate output label changes with the same path/inode. Equal-length control-a/control-b aliases are hardlinks to those executables, preserving each original inode and byte identity while changing its path. Alias comparison still has different backing inodes and diagnostic line/build-ID data. Paths/inodes/content hashes are recorded. These are controlled interventions, not an exclusive cause of any observed difference.\n\n| Case | ms/MSM | CPU ms/MSM | Cycles/MSM | Instructions/MSM |\n|---|---:|---:|---:|---:|\n");
    for (c, v) in a["cells"].as_object().unwrap() {
        writeln!(
            s,
            "| {c} | {:.6} | {:.6} | {:.3} | {:.3} |",
            v["ns_per_MSM"]["median"].as_f64().unwrap() / 1e6,
            v["CPU_ms_per_MSM"]["median"].as_f64().unwrap(),
            v["cycles:u"]["median"].as_f64().unwrap(),
            v["instructions:u"]["median"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\n| Comparison | Paired median | Wins |\n|---|---:|---:|\n");
    for (k, v) in a["pairs"].as_object().unwrap() {
        writeln!(
            s,
            "| {k} | {:.9} | {}/7 |",
            v["median"].as_f64().unwrap(),
            v["wins"]
        )
        .unwrap();
    }
    writeln!(s,"\nAll event groups100%running: **{}**. All7ratios and observations are retained; no confidence or adoption claim, exclusion, extension or unchanged reroll. Event windows include their common control boundary; genericcache events are not DRAM bytes/bandwidth. CPU/cycle differences alone do not isolate clocks, cachecoloring, aliasing, allocator layout or scheduling.\n",a["all_counters_100_percent_running"]).unwrap();
    s.push_str("There are294countedMSMs+126primes, not proofs. No native rebuild/test/SDK/Gnark calls occur. Both source-verified parent binaries and754parent pins are rechecked, along with new compiler/helper/alias identities. No build/helper/hash/archive/otherowned benchmark overlaps counters; host snapshots do not prove exclusive use. No ASLR/globalkernel settings change. No new pressure/minimumRAM/mobile/TEE/giant/AWS/Aeglos claim. Proofledger13,181; all172production/SDK14da unchanged.\n\n## Inherited CPU and Metal comparison\n\nThe originalall8CPU10xobjective remains active and unmet,0/8. AcceptedFFTengineeringexception/falsefullgates remain. These are inherited accepted local timings/suppliedM5references, not measurements of this diagnostic.\n\n| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |\n|---|---:|---:|---:|---:|---:|\n");
    let shapes = [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ];
    let accepted = [
        8.324853, 11.460675, 15.854365, 24.084626, 33.693397, 39.981636, 66.010670, 65.086624,
    ];
    let cpu = [37., 59., 109., 189., 284., 335., 528., 507.];
    let metal = [24., 30., 48., 74., 101., 124., 176., 175.];
    for i in 0..8 {
        writeln!(
            s,
            "| {} | {:.6} | {:.1} | {:.1} | {:.3}× | {:.3}× |",
            shapes[i],
            accepted[i],
            cpu[i],
            metal[i],
            cpu[i] / accepted[i],
            metal[i] / accepted[i]
        )
        .unwrap();
    }
    s.push_str("\nHistorical primary134.50MiB; merge54 161.341835ms/1910.1875MiB without suppliedM5reference. No proving-time improvement is established.\n\n[Design](artifacts/design.md), [identities](artifacts/identities.json), [all results](artifacts/analysis.json), [workflow](artifacts/runs/result.json), [parent borrowed-queue study](../2026-10-28-g2-borrowed-queue/README.md). Captured logs retain bytes, including trailing blank lines; authored sources/report whitespace checked separately.\n");
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
    v["status"] = json!(
        "Same-code controlidentity diagnostic completed/pushed; no adoption; read interpretation"
    );
    v["control_identity_diagnostic"] = read(r("analysis.json"));
    v["next_action"]=json!("Use controlled identity results to resolve baseline comparability before selecting a new proof diagnostic; preserve borrowedG2 failed gate and positivephysical16/memory estimates");
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
    let header=format!("CURRENT COMPLETE/PUSHED — SAME-CODE CONTROL IDENTITY\n\nHEAD/remoteverified{head};root{R};report{D}/README.md.42counterprocesses7blocks6conditions,294MSMs+126primes; noONcandidate/nativebuild/newSDKproof. Hardlinked equal-lengthaliases preserveparentfrozen/off inode/hash; originalmode0/1isolate outputlabel. Readanalysis/interpretation for actual results/ratios; noadoption/perfgatechange. Production172/SDK14da/historical1x2 8.324853ms/RSS134.50MiB unchanged;goalACTIVE0/8;proofledger13181;noownedjobs/cloud. CurrentturnPROGRESS borrowedG2completeMSM+memory+publishedreport and this distinctcontroldiagnostic. Parent componentfailedgate retained,physical16ON/frozen6.81%faster/ON/OFF37.73%faster,logical32accepteduncertain,1.35MiBrequestedpeaksaved. Fullruntimecode/constants identicalcontrols butparentOff/Frozenstable45–48%gap unresolved beforethis diagnostic.\n\nNEXT:{}. No unchangedoptimizationreroll, preserveallpositiveestimates/uncertainty/failedgates. Allparent754pins rechecked,newidentitiespinned,nohelper/build/hash/archiveoverlapcapture. No ASLR/globalsettingchange/noexclusivehostclaim. No actualprooftiminggain/mobile/pressure/AWSclaim. CommitANDpushcompletedexperiments,noquestions/subagents/hugecaches/Casmrewrite. Olderheadershistorical.\n\n",v["next_action"]);
    fs::write(handoff, format!("{header}{old}")).unwrap();
    println!("Verified push{head};state/handoff updated;goalactive0/8");
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
