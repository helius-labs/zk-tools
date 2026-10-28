use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    io::Read,
    path::Path,
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-direct-ref-pack";
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
fn identities() -> Value {
    let r = Path::new(R);
    let mut receipts = BTreeMap::new();
    for label in ["frozen", "fresh", "selected"] {
        let mut a = read(r.join(format!("native/{label}-artifacts.json")));
        let flags = a["rustflags"].as_str().unwrap();
        let base = fs::read_to_string(r.join("native-flags.txt")).unwrap();
        let expected = if label == "selected" {
            format!("{} --cfg zopro_direct_ref_pack", base.trim())
        } else {
            base.trim().to_string()
        };
        assert_eq!(flags, expected);
        assert_eq!(
            flags
                .split_whitespace()
                .any(|s| s == "zopro_direct_ref_pack"),
            label == "selected"
        );
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert!(p.starts_with(&format!("{R}/frozen-libraries/{label}/")));
            assert_eq!(hash(p), h.as_str().unwrap());
        }
        let macros = read(r.join(format!("native/{label}-proc-macros.json")));
        for (p, h) in macros["input_pins"].as_object().unwrap() {
            assert!(p.starts_with(&format!("{R}/frozen-libraries/{label}/")));
            assert_eq!(hash(p), h.as_str().unwrap());
            a["input_pins"][p] = h.clone();
        }
        receipts.insert(label, a);
    }
    let off = &receipts["fresh"]["libraries"]["zopro_bn254"];
    let on = &receipts["selected"]["libraries"]["zopro_bn254"];
    assert_ne!(off, on);
    assert_ne!(hash(off.as_str().unwrap()), hash(on.as_str().unwrap()));
    json!(receipts)
}
fn link() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native-rest/result.json"))["status"], "complete");
    assert!(!r.join("probe-artifacts.json").exists());
    let ids = identities();
    let flags = fs::read_to_string(r.join("native-flags.txt")).unwrap();
    let mut probes = BTreeMap::new();
    for label in ["frozen", "fresh", "selected"] {
        let a = &ids[label];
        let binary = r.join(format!("{label}-probe"));
        let mut argv: Vec<_> = flags.split_whitespace().map(str::to_owned).collect();
        argv.extend([
            "--edition=2021".into(),
            "-O".into(),
            r.join("probe.rs").display().to_string(),
            "-o".into(),
            binary.display().to_string(),
            "-L".into(),
            format!("dependency={R}/frozen-libraries/{label}"),
        ]);
        for name in [
            "zopro_core",
            "zopro_bn254",
            "libc",
            "rayon",
            "sha2",
            "serde_json",
        ] {
            argv.extend([
                "--extern".into(),
                format!("{name}={}", a["libraries"][name].as_str().unwrap()),
            ]);
        }
        save(
            r.join(format!("link-{label}-command.json")),
            &json!({"argv":argv,"libraries":a["libraries"],"flags_of_library":a["rustflags"]}),
        );
        let out = Command::new("rustc").args(&argv).output().unwrap();
        fs::write(r.join(format!("link-{label}-stdout.log")), out.stdout).unwrap();
        fs::write(r.join(format!("link-{label}-stderr.log")), out.stderr).unwrap();
        assert!(out.status.success(), "link {label}");
        probes.insert(label,json!({"binary":binary,"sha256":hash(&binary),"source_sha256":hash(r.join("probe.rs")),"libraries_receipt_sha256":hash(r.join(format!("native/{label}-artifacts.json")))}));
    }
    for (a, b) in [("fresh", "selected"), ("frozen", "selected")] {
        assert_ne!(probes[a]["sha256"], probes[b]["sha256"]);
    }
    save(r.join("probe-artifacts.json"), &json!(probes));
    save(
        r.join("control-identity-audit.json"),
        &json!({"immutable_libraries_checked":true,"expected_flagsets_checked":true,"OFF_ON_field_hashes_distinct":true,"OFF_ON_probe_hashes_distinct":true,"receipts":ids,"probes":probes,"timing_started":false}),
    );
    println!("Linked3probesexclusivelyfromfrozenlibraries;OFF_ONidentitiesdistinct");
}
fn cases() -> Vec<(String, usize)> {
    ["g1-owned", "g1-borrowed-signed", "g2-prefix"]
        .into_iter()
        .flat_map(|k| [200, 352].map(|n| (k.to_string(), n)))
        .collect()
}
fn coverage() -> f64 {
    let (mut c, mut s) = (1u64, 1u64);
    for k in 1..=9 {
        c = c * (32 - k) / k;
        s += c;
    }
    1. - 2. * s as f64 / (1u64 << 31) as f64
}
fn prepare() {
    let r = Path::new(R);
    assert!(!r.join("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-pins-v3.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let audit=Command::new(r.join("native-helper")).arg("audit").output().unwrap();
    assert!(audit.status.success());fs::write(r.join("pre-capture-source-audit.log"),audit.stdout).unwrap();
    for label in["fresh","selected"] {
        let file=r.join(format!("native-rest/{label}-field-tests/stdout.log"));
        let text=fs::read_to_string(&file).unwrap();
        assert!(text.contains("test result: ok. 34 passed; 0 failed"));
        assert!(text.contains("test direct_input::tests::direct_reference_words_match_split_for_unaligned_permuted_and_aliased_inputs ... ok"));
        pin(&mut pins,file);
    }
    for label in ["frozen", "fresh", "selected"] {
        let n = if label == "frozen" { 1 } else { 122 };
        let file = r.join(format!("{}/{label}-core-tests/stdout.log", if label=="frozen"{"native"}else{"native-rest"}));
        let t = fs::read_to_string(&file).unwrap();
        assert!(t.contains(&format!("test result: ok. {n} passed; 0 failed")));
        for name in [
            "owned_g1_generator_oracle_tails_and_reuse",
            "borrowed_signed_g1_generator_oracle_tails_and_reuse",
            "owned_g2_prefix_generator_oracle_tails_and_reuse",
        ] {
            if label != "frozen" || name == "owned_g2_prefix_generator_oracle_tails_and_reuse" { assert!(t.contains(&format!("test msm::caller_probe::tests::{name} ... ok"))); }
        }
        pin(&mut pins, file);
        pin(&mut pins, r.join(format!("native/{label}-artifacts.json")));
        pin(
            &mut pins,
            r.join(format!("native/{label}-proc-macros.json")),
        );
    }
    let before=fs::read_to_string(r.join("native-first/frozen-core-tests/stdout.log")).unwrap();
    for name in["owned_g1_generator_oracle_tails_and_reuse","borrowed_signed_g1_generator_oracle_tails_and_reuse"]{assert!(before.contains(&format!("test msm::caller_probe::tests::{name} ... ok")));}
    let ids = identities();
    for (_, a) in ids.as_object().unwrap() {
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
    }
    let probes = read(r.join("probe-artifacts.json"));
    for (_, a) in probes.as_object().unwrap() {
        let p = a["binary"].as_str().unwrap();
        assert_eq!(hash(p), a["sha256"].as_str().unwrap());
        pin(&mut pins, p);
    }
    assert_ne!(probes["fresh"]["sha256"], probes["selected"]["sha256"]);
    for f in [
        "experiment.rs",
        "experiment",
        "probe.rs",
        "perf.rs",
        "probe-artifacts.json",
        "control-identity-audit.json",
        "compiler.txt",
        "machine.txt",
        "host-workload.txt",
        "pre-capture-source-audit.log",
        "make-experiment.rs",
        "native-rest/result.json",
        "build-plan-v3.json",
        "build-pins-v3.json",
    ] {
        pin(&mut pins, r.join(f));
    }
    pin(&mut pins, X);
    let base = json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2"});
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   kind: &str,
                   n: usize,
                   label: &str,
                   rounds: usize| {
        let normal = vec![
            "taskset".to_string(),
            "-c".into(),
            "2".into(),
            r.join(format!("{label}-probe")).display().to_string(),
            kind.into(),
            n.to_string(),
            rounds.to_string(),
        ];
        let mut env = base.clone();
        let argv = if phase == "profile" {
            let p = r.join("profiles").join(&name);
            fs::create_dir_all(&p).unwrap();
            assert!(Command::new("mkfifo")
                .arg(p.join("ctl"))
                .arg(p.join("ack"))
                .status()
                .unwrap()
                .success());
            env["ZOPRO_BENCH_PERF_CTL"] = json!(p.join("ctl"));
            env["ZOPRO_BENCH_PERF_ACK"] = json!(p.join("ack"));
            let mut cmd = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in env.as_object().unwrap() {
                cmd.push(format!("{k}={}", v.as_str().unwrap()));
            }
            cmd.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "-e".into(),
                "cycles:u".into(),
                "-F".into(),
                "199".into(),
                "-m".into(),
                "512".into(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    p.join("ctl").display(),
                    p.join("ack").display()
                ),
                "-o".into(),
                p.join("perf.data").display().to_string(),
                "--".into(),
            ]);
            cmd.extend(normal);
            cmd
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":240}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"kind":kind,"size":n,"variant":label,"iterations":rounds}));
    };
    for b in 0..31 {
        let mut cs = cases();
        cs.rotate_left(b % 6);
        if b % 2 != 0 {
            cs.reverse();
        }
        for (i, (kind, n)) in cs.iter().enumerate() {
            let mut labels = vec!["frozen", "fresh", "selected"];
            labels.rotate_left((b + i) % 3);
            if (b + i) % 2 != 0 {
                labels.reverse();
            }
            for label in labels {
                add(
                    format!("b{b}-{kind}-{n}-{label}"),
                    "latency",
                    b,
                    kind,
                    *n,
                    label,
                    1024,
                );
            }
        }
    }
    for (kind, n) in [("g1-borrowed-signed", 352), ("g1-owned", 200)] {
        for label in ["fresh", "selected"] {
            let name = format!("profile-{kind}-{label}");
            add(name.clone(), "profile", 0, kind, n, label, 262144);
            profiles.push(name);
        }
    }
    assert_eq!(jobs.len(), 562);
    for name in profiles {
        let p = r.join("profiles").join(&name).join("perf.data");
        for (kind, argv) in [
            (
                "owner",
                vec![
                    "sudo".into(),
                    "-n".into(),
                    "chown".into(),
                    "ubuntu:ubuntu".into(),
                    p.display().to_string(),
                ],
            ),
            (
                "decode",
                vec![
                    "perf".into(),
                    "script".into(),
                    "--header".into(),
                    "--show-lost-events".into(),
                    "-i".into(),
                    p.display().to_string(),
                    "-F".into(),
                    "pid,tid,period,ip,sym".into(),
                ],
            ),
            (
                "loss",
                vec![
                    "perf".into(),
                    "script".into(),
                    "-D".into(),
                    "-i".into(),
                    p.display().to_string(),
                ],
            ),
        ] {
            steps.push(json!({"kind":"command","name":format!("{kind}-{name}"),"argv":argv,"timeout_seconds":120}));
        }
    }
    save(r.join("jobs.json"), &json!(jobs));
    pin(&mut pins, r.join("jobs.json"));
    save(r.join("pins.json"), &json!(pins));
    save(
        r.join("run-plan.json"),
        &json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps}),
    );
    save(
        r.join("interval-coverage.json"),
        &json!({"nominal_coverage":coverage(),"n":31,"lower_rank":10,"upper_rank":22,"familywise":false}),
    );
    println!("Prepared558latency+4profiles+12decode;{}pins", pins.len());
}
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn interval(v: &[f64]) -> Value {
    assert_eq!(v.len(), 31);
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":v.iter().filter(|&&r|r<1.).count(),"ratios":v,"nominal_coverage":coverage()})
}
fn result(name: &str) -> Value {
    let t = fs::read_to_string(format!("{R}/runs/{name}/stdout.log")).unwrap();
    serde_json::from_str(t.lines().find_map(|s| s.strip_prefix("RESULT ")).unwrap()).unwrap()
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if e.file_type().unwrap().is_dir() {
            if [
                "source",
                "frozen-source",
                "target",
                "profiles",
                "frozen-libraries",
            ]
            .contains(&name.as_str())
            {
                continue;
            }
            archive(root, &p, out, index)
        } else if ["rs", "md", "json", "log", "txt"]
            .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            && ![
                "close.log",
                "close-error.log",
                "archive-verification.log",
                "push.log",
            ]
            .contains(&name.as_str())
        {
            if p.to_string_lossy().contains("/loss-") && name == "stdout.log" {
                continue;
            }
            let rel = p.strip_prefix(root).unwrap();
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            index.insert(format!("artifacts/{}", rel.display()), hash(dest));
        }
    }
}
fn finish() {
    let r = Path::new(R);
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        574
    );
    for (p, h) in read(r.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let _ = identities();
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let mut failed = vec![];
    let mut aggregates: BTreeMap<&str, Vec<f64>> =
        [("frozen", vec![0.; 31]), ("fresh", vec![0.; 31])].into();
    let mut table = String::new();
    let mut pair_table = String::new();
    let mut outputs = BTreeMap::<String, String>::new();
    for (kind, n) in cases() {
        let mut times = BTreeMap::new();
        let mut display = BTreeMap::new();
        let mut capacities = vec![];
        let mut reference = vec![];
        for label in ["frozen", "fresh", "selected"] {
            let (mut ns, mut cpu, mut rss) = (vec![], vec![], vec![]);
            for b in 0..31 {
                let a = result(&format!("b{b}-{kind}-{n}-{label}"));
                assert_eq!(a["iterations"], 1024);
                assert_eq!(a["size"], n);
                assert_eq!(a["timed_allocations"], 0);
                assert_eq!(a["internally_verified"], true);
                let h = a["output_sha256"].as_str().unwrap();
                if let Some(old) = outputs.insert(format!("{kind}-{n}"), h.into()) {
                    assert_eq!(old, h);
                }
                ns.push(a["ns_per_round"].as_f64().unwrap());
                cpu.push(
                    (a["usage"]["user_cpu_ms"].as_f64().unwrap()
                        + a["usage"]["system_cpu_ms"].as_f64().unwrap())
                        * 1e6
                        / 1024.,
                );
                rss.push(a["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
                capacities.push(a["capacity_bytes"].as_u64().unwrap());
                reference.push(a["fixture_reference_bytes"].as_u64().unwrap());
            }
            let v = json!({"kind":kind,"size":n,"variant":label,"median_process_mean_ns_per_batch":median(&ns),"process_means":ns,"CPU_ns_per_batch":median(&cpu),"median_process_peak_RSS_MiB":median(&rss),"max_process_peak_RSS_MiB":rss.iter().copied().fold(0.,f64::max),"timed_allocations":0,"arithmetic_scratch_bytes":capacities.last().unwrap(),"fixture_reference_bytes":reference.last().unwrap()});
            display.insert(label, median(&ns) / 1000.);
            times.insert(label, ns);
            cells.insert(format!("{kind}/{n}/{label}"), v);
        }
        assert!(capacities.windows(2).all(|w| w[0] == w[1]));
        assert!(reference.windows(2).all(|w| w[0] == w[1]));
        writeln!(
            table,
            "| {kind} | {n} | {:.3} | {:.3} | {:.3} |",
            display["frozen"], display["fresh"], display["selected"]
        )
        .unwrap();
        for (a, b) in [
            ("fresh", "frozen"),
            ("selected", "frozen"),
            ("selected", "fresh"),
        ] {
            let ratios: Vec<_> = times[a].iter().zip(&times[b]).map(|(a, b)| a / b).collect();
            if a == "selected" && kind.starts_with("g1-") {
                for (i, x) in ratios.iter().enumerate() {
                    aggregates.get_mut(b).unwrap()[i] += x.ln() / 4.;
                }
            }
            let v = interval(&ratios);
            let pass = if a == "fresh" {
                v["low"].as_f64().unwrap() >= 1. / 1.03 && v["high"].as_f64().unwrap() <= 1.03
            } else {
                v["high"].as_f64().unwrap() <= 1.02
            };
            let key = format!("{kind}/{n}/{a}/{b}");
            if !pass {
                failed.push(key.clone());
            }
            writeln!(
                pair_table,
                "| {kind} | {n} | {a}/{b} | {:.9} | [{:.9}, {:.9}] | {}/31 | {} |",
                v["median"].as_f64().unwrap(),
                v["low"].as_f64().unwrap(),
                v["high"].as_f64().unwrap(),
                v["wins"],
                if pass { "pass" } else { "fail" }
            )
            .unwrap();
            pairs.insert(key, v);
        }
    }
    let mut agg = BTreeMap::new();
    for (label, xs) in aggregates {
        let ratios: Vec<_> = xs.iter().map(|x| x.exp()).collect();
        let v = interval(&ratios);
        if v["high"].as_f64().unwrap() >= 1. || v["wins"].as_u64().unwrap() < 21 {
            failed.push(format!("aggregate/selected/{label}"));
        }
        agg.insert(label, v);
    }
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    let mut profile_table = String::new();
    for kind in ["g1-borrowed-signed", "g1-owned"] {
        for label in ["fresh", "selected"] {
            let name = format!("profile-{kind}-{label}");
            let a = result(&name);
            assert_eq!(a["iterations"], 262144);
            assert_eq!(a["timed_allocations"], 0);
            assert_eq!(a["internally_verified"], true);
            assert!(
                !fs::read_to_string(r.join(format!("runs/loss-{name}/stdout.log")))
                    .unwrap()
                    .contains("PERF_RECORD_LOST")
            );
            let text =
                fs::read_to_string(r.join(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let mut pids = BTreeSet::new();
            let (mut n, mut period) = (0u64, 0u64);
            let mut symbols = BTreeMap::<String, u64>::new();
            for line in text.lines().filter(|s| !s.trim_start().starts_with('#')) {
                let f: Vec<_> = line.split_whitespace().collect();
                if f.len() < 4 {
                    continue;
                }
                let fields = if let Some((p, _)) = f[0].split_once('/') {
                    p.parse::<u64>()
                        .ok()
                        .zip(f[1].parse::<u64>().ok())
                        .map(|(p, t)| (p, t, 3))
                } else if f.len() >= 5 {
                    f[0].parse::<u64>()
                        .ok()
                        .zip(f[2].parse::<u64>().ok())
                        .map(|(p, t)| (p, t, 4))
                } else {
                    None
                };
                if let Some((p, t, start)) = fields {
                    pids.insert(p);
                    n += 1;
                    period += t;
                    *symbols
                        .entry(format!(
                            "{:#}",
                            rustc_demangle::demangle(&f[start..].join(" "))
                        ))
                        .or_default() += t;
                }
            }
            assert!(n > 0);
            assert_eq!(pids.len(), 1);
            writeln!(profile_table, "| {kind} | {label} | {n} | 0 |").unwrap();
            raw.insert(
                r.join("profiles")
                    .join(&name)
                    .join("perf.data")
                    .display()
                    .to_string(),
                hash(r.join("profiles").join(&name).join("perf.data")),
            );
            profiles.insert(name,json!({"samples":n,"lost":0,"pids":pids,"period":period,"demangled_symbols":symbols}));
        }
    }
    let gate = failed.is_empty();
    let status = if gate {
        "passed actual addition-caller screen; full SDK qualification required"
    } else {
        "rejected actual addition-caller screen"
    };
    let pins = read(r.join("pins.json"));
    let analysis = json!({"status":status,"gate_passed":gate,"failed_checks":failed,"cells":cells,"pairs":pairs,"aggregates":agg,"profiles":profiles,"raw_profile_pins":raw,"SDK_requests":0,"new_independent_exports":0,"independent_proof_count":12983,"adapter_tests_frozen":3,"core_tests_fresh":122,"core_tests_selected":122,"latency_processes":558,"profile_processes":4,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"capture_pins":pins.as_object().unwrap().len()});
    save(r.join("analysis.json"), &analysis);
    save(r.join("tables.json"),&json!({"latency":table,"pairs":pair_table,"profiles":profile_table}));
    println!("Direct referenced input screen {status}; failed checks {:?}",failed);
    for(k,v)in cells {println!("{k} {}",v["median_process_mean_ns_per_batch"]);}
    for(k,v)in agg {println!("aggregate/{k} {v}");}
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("link") => link(),
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("link/prepare/finish"),
    }
}
