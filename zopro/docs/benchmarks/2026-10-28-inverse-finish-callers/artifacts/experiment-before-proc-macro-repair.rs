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
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-finish-callers";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-inverse-finish-callers";
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
        let a = read(r.join(format!("native/{label}-artifacts.json")));
        let flags = a["rustflags"].as_str().unwrap();
        let base = fs::read_to_string(r.join("native-flags.txt")).unwrap();
        let expected = if label == "selected" {
            format!("{} --cfg zopro_direct_finish_reduction", base.trim())
        } else {
            base.trim().to_string()
        };
        assert_eq!(flags, expected);
        assert_eq!(
            flags
                .split_whitespace()
                .any(|s| s == "zopro_direct_finish_reduction"),
            label == "selected"
        );
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert!(p.starts_with(&format!("{R}/frozen-libraries/{label}/")));
            assert_eq!(hash(p), h.as_str().unwrap());
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
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
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
        serde_json::from_value(read(r.join("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let audit = read(r.join("source-audit.json"));
    for name in ["frozen-source", "source"] {
        let mut same = 0;
        for (f, h) in m["source_files_sha256"].as_object().unwrap() {
            let p = r.join(name).join(f);
            let text = if f.ends_with(".rs") {
                Some(fs::read_to_string(&p).unwrap())
            } else {
                None
            };
            if let Some(entry) = audit["core_suffixes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v[0] == *f)
            {
                assert_eq!(
                    text.unwrap()
                        .strip_suffix(entry[1].as_str().unwrap())
                        .unwrap(),
                    fs::read_to_string(Path::new(Z).join(f)).unwrap()
                );
            } else if name == "source" && f == "crates/zopro-bn254/src/inverse.rs" {
                let edits = &audit["parent_inverse_edits"];
                let back = text.unwrap();
                let back = back
                    .strip_suffix(edits[1].as_str().unwrap())
                    .unwrap()
                    .replace(edits[0].as_str().unwrap(), "");
                assert_eq!(back, fs::read_to_string(Path::new(Z).join(f)).unwrap());
            } else {
                assert_eq!(hash(p), h.as_str().unwrap());
                same += 1;
            }
        }
        assert_eq!(same, if name == "source" { 169 } else { 170 });
        assert_eq!(
            hash(
                r.join(name)
                    .join("crates/zopro-core/src/msm/caller_probe.rs")
            ),
            hash(r.join("caller_probe.rs"))
        );
    }
    for label in ["frozen", "fresh", "selected"] {
        let n = if label == "frozen" { 3 } else { 122 };
        let file = r.join(format!("native/{label}-core-tests/stdout.log"));
        let t = fs::read_to_string(&file).unwrap();
        assert!(t.contains(&format!("test result: ok. {n} passed; 0 failed")));
        for name in [
            "owned_g1_generator_oracle_tails_and_reuse",
            "borrowed_signed_g1_generator_oracle_tails_and_reuse",
            "owned_g2_prefix_generator_oracle_tails_and_reuse",
        ] {
            assert!(t.contains(&format!("test msm::caller_probe::tests::{name} ... ok")));
        }
        pin(&mut pins, file);
        pin(&mut pins, r.join(format!("native/{label}-artifacts.json")));
    }
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
        "native/result.json",
        "build-plan.json",
        "build-pins.json",
        "freeze-test.log",
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
    for (kind, n) in [("g1-borrowed-signed", 352), ("g2-prefix", 200)] {
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
            if a == "selected" {
                for (i, x) in ratios.iter().enumerate() {
                    aggregates.get_mut(b).unwrap()[i] += x.ln() / 6.;
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
    for kind in ["g1-borrowed-signed", "g2-prefix"] {
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
    let analysis = json!({"status":status,"gate_passed":gate,"failed_checks":failed,"cells":cells,"pairs":pairs,"aggregates":agg,"profiles":profiles,"raw_profile_pins":raw,"SDK_requests":0,"new_independent_exports":0,"independent_proof_count":12785,"adapter_tests_frozen":3,"core_tests_fresh":122,"core_tests_selected":122,"latency_processes":558,"profile_processes":4,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"capture_pins":pins.as_object().unwrap().len()});
    save(r.join("analysis.json"), &analysis);
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let mut refs = String::new();
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let v = &m["m5"][shape];
        writeln!(
            refs,
            "| {shape} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    let report = format!(
        r#"# Direct inverse finish in addition callers

The candidate **{status}**. This measures complete G1/G2 batch-addition callers, not full proving latency. Production and the accepted SDK are unchanged.

The [parent arithmetic study](../2026-10-28-inverse-direct-finish/README.md) checks a signed 320-bit reduction that removes the inverse finish's scale-restoring field multiplication. Its primitive timing lacked an OFF control because Cargo's top-level library was overwritten; that schedule remains unqualified and was not rerun. This study tests a different boundary with audited libraries: G1 owned unsigned, G1 borrowed signed, and G2 prefix/norm additions, each at 200 and 352 entries. These sizes come from the real primary batch census, but points and destination patterns are synthetic ordinary generator fixtures. This does not measure collision queues, routing, a complete MSM, or a witness distribution.

The field arithmetic matches the parent source exactly. The frozen baseline retains accepted field source; fresh and selected share the candidate source, with the selection cfg present only for selected. No stream arithmetic body changes. Two original core files add diagnostic module/reexports; 170 original frozen files and 169 original fresh files remain exact. The borrowed adapter prepares a reference vector outside the timer and calls the original borrowed signed stream. This vector is diagnostic input storage, not a proposed new production buffer. Arithmetic scratch capacities are checked equal across variants.

Libraries are frozen immediately after each build into create-new content-addressed paths before a later build can overwrite Cargo outputs. All probe links search only frozen dependencies. Flag sets, field-library hashes, executable hashes and complete copies are checked before capture. The [identity audit](artifacts/control-identity-audit.json) verifies distinct OFF/ON fields and probes. A regression test overwrites a simulated mutable Cargo library and confirms the earlier frozen OFF bytes survive.

Three adapter tests pass against frozen libraries. Fresh and selected each pass the complete 122-test native core suite, including the adapters and existing complete-curve tests, with serial test execution. Tests cover independent generator sums, owned and borrowed signs, empty/partial batches, 199/200/351/352 boundaries, all bucket outputs and repeated scratch reuse. The parent's 34-test OFF/ON field qualification and 68,672 independent signed-residue cases are inherited, not rerun or recounted as new cases.

All 558 latency processes complete in a fixed 31-block rotated/reversed schedule on CPU2. Each initializes its fixture and runs three checked eight-round primes, resets outside the timer, then performs 1024 complete batch additions. Every final bucket matches the independent generator-scalar oracle, output digests agree, and timed allocation counts are zero. Entries below are medians of 31 process means, in microseconds per batch.

| Caller | Entries | Frozen µs | Fresh OFF µs | Selected ON µs |
|---|---:|---:|---:|---:|
{table}
| Caller | Entries | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---:|---|---:|---:|---:|---|
{pair_table}
The equal-weight six-case paired geometric aggregates are [recorded in analysis](artifacts/analysis.json): selected/frozen `{:.9}` and selected/fresh `{:.9}`. Aggregate upper bounds must be below one with at least 21/31 wins against both controls. Each individual upper ratio must be at most 1.02, and every fresh/frozen calibration interval must fit [1/1.03,1.03]. Ordered 10th–22nd ratios have nominal {:.6}% coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusions, extension or unchanged reroll is used. Equal case weights do not model the whole prover's work distribution.

Four separate warm 199 Hz cycles:u profiles follow all latency. Each performs 262,144 calls after three eight-call primes; capture excludes initialization, primes and oracle verification, and includes the loop and accounting. All captures precede decode, hash and archive work. No owned helper, build, hash, archive or other benchmark overlaps capture. Every profile has one sampled PID and zero lost records. Symbol shares are not removable proof wall-time fractions.

| Caller | Variant | Samples | Lost |
|---|---|---:|---:|
{profile_table}
CPU usage, RSS, capacities, complete process means and symbols are retained. Component RSS includes fixtures, setup and thread initialization; it is not full-prover RAM or minimum memory. No new SDK/Gnark proof was produced; ledger remains 12,785. There is no new all-eight/54 proof timing, portable/mobile/TEE, pressure/concurrency, large-circuit or AWS/Aeglos result. A passing caller screen requires separate full-SDK and broader qualification before any adoption.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These accepted proof timings and supplied references are inherited.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
{refs}
Accepted primary historical peak RSS is 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB historical peak with no supplied M5 reference. No current-source AWS/Aeglos comparison is added.
"#,
        agg["frozen"]["median"].as_f64().unwrap(),
        agg["fresh"]["median"].as_f64().unwrap(),
        100. * coverage()
    );
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    fs::write(format!("{D}/README.md"), report).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &Path::new(D).join("artifacts"), &mut index);
    for f in [
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm/caller_probe.rs",
        "crates/zopro-bn254/src/inverse.rs",
        "crates/zopro-bn254/src/inverse/direct_finish.rs",
    ] {
        let p = Path::new(D).join("artifacts/private-source").join(f);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(r.join("source").join(f), &p).unwrap();
        index.insert(format!("artifacts/private-source/{f}"), hash(p));
    }
    index.insert("README.md".into(), hash(format!("{D}/README.md")));
    fs::write(
        format!("{D}/SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Closed {status};{}hashes;ledger12785", index.len());
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("link") => link(),
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("link/prepare/finish"),
    }
}
