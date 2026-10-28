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
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-direct-finish";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-inverse-direct-finish";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const DEP: &str = "/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";
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
fn link() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert!(!r.join("probe-artifacts.json").exists());
    let flags = fs::read_to_string(r.join("native-flags.txt")).unwrap();
    let accepted =
        read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json");
    let mut receipt = BTreeMap::new();
    for label in ["accepted", "fresh", "candidate"] {
        let lib = if label == "accepted" {
            let p = accepted["libraries"]["zopro_bn254"].as_str().unwrap();
            assert_eq!(hash(p), accepted["input_pins"][p].as_str().unwrap());
            p.to_string()
        } else {
            let text =
                fs::read_to_string(r.join(format!("native/{label}-library/stdout.log"))).unwrap();
            let libs: Vec<_> = text
                .lines()
                .filter_map(|s| serde_json::from_str::<Value>(s).ok())
                .filter(|v| {
                    v["reason"] == "compiler-artifact" && v["target"]["name"] == "zopro_bn254"
                })
                .flat_map(|v| {
                    v["filenames"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|p| {
                            p.as_str()
                                .filter(|p| p.ends_with(".rlib"))
                                .map(str::to_owned)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
            assert_eq!(libs.len(), 1);
            libs[0].clone()
        };
        let binary = r.join(format!("{label}-probe"));
        let mut args: Vec<String> = flags.split_whitespace().map(str::to_owned).collect();
        args.extend([
            "--edition=2021".into(),
            "-O".into(),
            r.join("probe.rs").display().to_string(),
            "-o".into(),
            binary.display().to_string(),
            "-L".into(),
            format!("dependency={DEP}"),
            "-L".into(),
            format!("dependency={}", Path::new(&lib).parent().unwrap().display()),
            "--extern".into(),
            format!("zopro_bn254={lib}"),
            "--extern".into(),
            format!("serde_json={DEP}/libserde_json-9c41682dbe4e057f.rlib"),
            "--extern".into(),
            format!("sha2={DEP}/libsha2-b36128d04ec91f93.rlib"),
            "--extern".into(),
            format!("libc={}", accepted["libraries"]["libc"].as_str().unwrap()),
        ]);
        save(
            r.join(format!("link-{label}-command.json")),
            &json!({"argv":args,"library":lib}),
        );
        let output = Command::new("rustc").args(&args).output().unwrap();
        fs::write(r.join(format!("link-{label}-stdout.log")), &output.stdout).unwrap();
        fs::write(r.join(format!("link-{label}-stderr.log")), &output.stderr).unwrap();
        assert!(output.status.success(), "link {label}");
        receipt.insert(label,json!({"binary":binary,"sha256":hash(&binary),"library":lib,"library_sha256":hash(&lib),"probe_sha256":hash(r.join("probe.rs")),"perf_sha256":hash(r.join("perf.rs"))}));
    }
    save(r.join("probe-artifacts.json"), &json!(receipt));
    println!("Linked3exactfieldlibraryprobes");
}
fn coverage() -> f64 {
    let (mut c, mut tail) = (1u64, 1u64);
    for k in 1..=9 {
        c = c * (32 - k) / k;
        tail += c;
    }
    1. - 2. * tail as f64 / (1u64 << 31) as f64
}
fn prepare() {
    let r = Path::new(R);
    assert!(!r.join("run-plan.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for label in ["fresh", "candidate"] {
        let t = fs::read_to_string(r.join(format!("native/{label}-tests/stdout.log"))).unwrap();
        assert!(t.contains("test result: ok. 34 passed; 0 failed"));
        assert!(t.contains("test inverse::direct_finish_tests::direct_signed_finish_matches_independent_fullwidth_fields ... ok"));
        pin(
            &mut pins,
            r.join(format!("native/{label}-tests/stdout.log")),
        );
    }
    let receipts = read(r.join("probe-artifacts.json"));
    for (_, a) in receipts.as_object().unwrap() {
        for (pkey, hkey) in [("binary", "sha256"), ("library", "library_sha256")] {
            let p = a[pkey].as_str().unwrap();
            assert_eq!(hash(p), a[hkey].as_str().unwrap());
            pin(&mut pins, p);
        }
    }
    for f in [
        "study.rs",
        "study",
        "fixture.rs",
        "fixture",
        "fixture.json",
        "fixture.log",
        "probe.rs",
        "retained-batch.rs",
        "perf.rs",
        "bounds.md",
        "probe-artifacts.json",
        "native/result.json",
        "build-plan.json",
        "build-pins.json",
    ] {
        pin(&mut pins, r.join(f));
    }
    pin(&mut pins, "/home/ubuntu/src/zopro-perf-work/mixed-sdk/target-control/release/deps/liblibc-5a6fe4d81836844f.rlib");
    pin(&mut pins, X);
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut profiles = vec![];
    let mut add = |name: String,
                   phase: &str,
                   block: usize,
                   field: &str,
                   kind: &str,
                   label: &str,
                   rounds: usize| {
        let normal = vec![
            "taskset".to_string(),
            "-c".into(),
            "2".into(),
            r.join(format!("{label}-probe")).display().to_string(),
            r.join("fixture.json").display().to_string(),
            field.into(),
            kind.into(),
            rounds.to_string(),
        ];
        let mut env = json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32"});
        let argv = if phase == "profile" {
            let dir = r.join("profiles").join(&name);
            fs::create_dir_all(&dir).unwrap();
            assert!(Command::new("mkfifo")
                .arg(dir.join("ctl"))
                .arg(dir.join("ack"))
                .status()
                .unwrap()
                .success());
            env["ZOPRO_BENCH_PERF_CTL"] = json!(dir.join("ctl"));
            env["ZOPRO_BENCH_PERF_ACK"] = json!(dir.join("ack"));
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
                    dir.join("ctl").display(),
                    dir.join("ack").display()
                ),
                "-o".into(),
                dir.join("perf.data").display().to_string(),
                "--".into(),
            ]);
            cmd.extend(normal);
            cmd
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":240}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"field":field,"kind":kind,"variant":label,"rounds":rounds}));
    };
    for block in 0..31 {
        let mut contexts = vec![
            ("fq", "scalar"),
            ("fq", "batch8"),
            ("fr", "scalar"),
            ("fr", "batch8"),
        ];
        contexts.rotate_left(block % 4);
        if block % 2 != 0 {
            contexts.reverse();
        }
        for (i, (field, kind)) in contexts.into_iter().enumerate() {
            let mut labels = vec!["accepted", "fresh", "candidate"];
            labels.rotate_left((block + i) % 3);
            if (block + i) % 2 != 0 {
                labels.reverse();
            }
            for label in labels {
                add(
                    format!("b{block}-{field}-{kind}-{label}"),
                    "latency",
                    block,
                    field,
                    kind,
                    label,
                    16,
                );
            }
        }
    }
    for kind in ["scalar", "batch8"] {
        for label in ["fresh", "candidate"] {
            let name = format!("profile-{kind}-{label}");
            add(
                name.clone(),
                "profile",
                0,
                "fq",
                kind,
                label,
                if kind == "scalar" { 1024 } else { 8192 },
            );
            profiles.push(name);
        }
    }
    assert_eq!(jobs.len(), 376);
    for name in profiles {
        let p = r.join("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));
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
    println!("Prepared372latency+4profiles+12decode;{}pins", pins.len());
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
    json!({"median":s[15],"low":s[9],"high":s[21],"wins":v.iter().filter(|&&x|x<1.).count(),"ratios":v,"nominal_coverage":coverage()})
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
            if ["source", "target", "profiles"].contains(&name.as_str()) {
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
        388
    );
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let mut failed = vec![];
    let mut table = String::new();
    let mut pair_table = String::new();
    let mut digests = BTreeMap::<String, String>::new();
    for field in ["fq", "fr"] {
        for kind in ["scalar", "batch8"] {
            let mut times = BTreeMap::new();
            let mut display = BTreeMap::new();
            for label in ["accepted", "fresh", "candidate"] {
                let (mut ns, mut cpu, mut rss) = (vec![], vec![], vec![]);
                for block in 0..31 {
                    let v = result(&format!("b{block}-{field}-{kind}-{label}"));
                    assert_eq!(v["inputs"], 4096);
                    assert_eq!(v["rounds"], 16);
                    assert_eq!(v["elements"], 65536);
                    assert_eq!(v["timed_allocations"], 0);
                    assert_eq!(v["verified_against_Arkworks"], true);
                    let h = v["output_sha256"].as_str().unwrap();
                    if let Some(old) = digests.insert(field.into(), h.into()) {
                        assert_eq!(old, h);
                    }
                    ns.push(v["mean_ns_per_element"].as_f64().unwrap());
                    cpu.push(
                        (v["usage"]["user_cpu_ms"].as_f64().unwrap()
                            + v["usage"]["system_cpu_ms"].as_f64().unwrap())
                            * 1e6
                            / 65536.,
                    );
                    rss.push(v["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
                }
                let row = json!({"field":field,"kind":kind,"variant":label,"median_process_mean_ns_per_element":median(&ns),"process_means":ns,"CPU_ns_per_element":median(&cpu),"median_process_peak_RSS_MiB":median(&rss),"max_process_peak_RSS_MiB":rss.iter().copied().fold(0.,f64::max),"timed_allocations":0});
                display.insert(label, median(&ns));
                times.insert(label, ns);
                cells.insert(format!("{field}/{kind}/{label}"), row);
            }
            writeln!(
                table,
                "| {field} | {kind} | {:.3} | {:.3} | {:.3} |",
                display["accepted"], display["fresh"], display["candidate"]
            )
            .unwrap();
            for (a, b) in [
                ("fresh", "accepted"),
                ("candidate", "accepted"),
                ("candidate", "fresh"),
            ] {
                let ratios: Vec<_> = times[a].iter().zip(&times[b]).map(|(a, b)| a / b).collect();
                let v = interval(&ratios);
                let pass = if a == "fresh" {
                    v["low"].as_f64().unwrap() >= 1. / 1.03 && v["high"].as_f64().unwrap() <= 1.03
                } else if field == "fq" {
                    v["high"].as_f64().unwrap() < 1. && v["wins"].as_u64().unwrap() >= 21
                } else {
                    v["high"].as_f64().unwrap() <= 1.02
                };
                let key = format!("{field}/{kind}/{a}/{b}");
                if !pass {
                    failed.push(key.clone())
                }
                writeln!(
                    pair_table,
                    "| {field} | {kind} | {a}/{b} | {:.9} | [{:.9}, {:.9}] | {}/31 | {} |",
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
    }
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    let mut profile_table = String::new();
    for kind in ["scalar", "batch8"] {
        for label in ["fresh", "candidate"] {
            let name = format!("profile-{kind}-{label}");
            let v = result(&name);
            assert_eq!(v["rounds"], if kind == "scalar" { 1024 } else { 8192 });
            assert_eq!(v["timed_allocations"], 0);
            assert_eq!(v["verified_against_Arkworks"], true);
            assert!(
                !fs::read_to_string(r.join(format!("runs/loss-{name}/stdout.log")))
                    .unwrap()
                    .contains("PERF_RECORD_LOST")
            );
            let t = fs::read_to_string(r.join(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let mut pids = BTreeSet::new();
            let (mut n, mut period) = (0u64, 0u64);
            let mut symbols = BTreeMap::<String, u64>::new();
            for line in t.lines().filter(|s| !s.trim_start().starts_with('#')) {
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
        "passed primitive screen; actual caller and SDK qualification required"
    } else {
        "rejected primitive screen"
    };
    save(
        r.join("analysis.json"),
        &json!({"status":status,"gate_passed":gate,"failed_checks":failed,"cells":cells,"pairs":pairs,"profiles":profiles,"raw_profile_pins":raw,"SDK_requests":0,"new_independent_exports":0,"independent_proof_count":12785,"field_tests_per_build":34,"new_field_tests":1,"independent_signed_residue_cases":68672,"latency_processes":372,"profile_processes":4,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"pins":pins.as_object().unwrap().len()}),
    );
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
        r#"# Direct reduction of the signed Euclidean finish

The candidate **{status}**. These are complete inversion primitive measurements, not proving times. Production and the accepted SDK are unchanged.

The current binary-GCD inverse ends with ordinary one-word Euclid. Its final signed linear form is reduced by one radix64 Montgomery cancellation and a full field multiplication to restore that scale. The candidate reduces that signed 320-bit form directly using a two-word constant reciprocal, up to three negative corrections and one existing canonical subtraction. It preserves the Euclidean loop, 31/62-bit matrices, coefficients and field representations. No variable division is used by the new reducer, while the unchanged Euclidean loop still uses division. There is no heap buffer, lookup table, key copy, new plan or assembly change. [Bounds and derivation](artifacts/bounds.md) state the exact supported modulus/input assumptions and fallback.

The [previous coprime-finish candidate](../2026-10-25-coprime-finish-sdk/README.md) changed termination and failed SDK qualification; that change is absent. Prior signed62 divsteps, fused coefficient updates and first-limb reuse are also separate experiments. The existing inverse follows [Pornin's binary GCD](https://eprint.iacr.org/2020/972); the author's [practical inversion article](https://purplesyringa.moe/blog/faster-practical-modular-inversion/) was inspected for background. Its timing figures are not used as BN254 predictions. The direct finish reduction and its bounds are local derivations.

Both fresh OFF and selected ON pass all 34 native field tests. The initial probe link failed because its libc argument was missing and the product-tree helper belongs to the core wrapper rather than the field vector. Preserved logs show a harness compile failure before any probe ran. Linking libc and copying the exact retained helper repaired the harness; no field test or measurement was repeated. One new test checks 68,672 distinct signed full-width residues across Fq/Fr against Arkworks, with zero/near-modulus values and extreme opposite-sign coefficients. The same test runs in both builds; repeated execution is not counted as new distinct inputs. Existing exponentiation and full inversion tests pass. The probe copies the accepted core eight-lane product-tree body exactly into a generic field wrapper; every output is independently checked against Arkworks. The original finish is recovered byte-for-byte by removing the dispatch/include; 171 other original private files and all 172 production files match accepted hashes. Core and prover sources are unchanged.

The fixed screen uses 31 rotated/reversed blocks, CPU2, and accepted-library/fresh-OFF/selected-ON probes. Each process runs three same-size primes then 16 complete passes over 4096 independent nonzero inputs. Loading, canonical input decoding and reference setup are outside timing; the scalar inverse or existing eight-lane product-tree inverse, batch packing, extraction and output writes are inside. Every process checks all outputs against an independent Arkworks inverse fixture and reports zero timed allocations. Entries are medians of 31 process means in ns per output element.

| Field | Caller | Accepted ns/element | Fresh ns/element | Candidate ns/element |
|---|---|---:|---:|---:|
{table}
| Field | Caller | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---|---|---:|---:|---:|---|
{pair_table}
The 10th–22nd ordered ratios have nominal {:.6}% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. Fresh/accepted calibration must fit [1/1.03,1.03] in all four contexts. Both Fq callers require candidate upper ratio below one and at least 21 wins against both controls. Fr upper ratios must be at most 1.02. There is no gain floor, exclusion, adaptive extension or unchanged reroll.

All 372 latency processes precede four separate FIFO-gated 199 Hz cycles:u profiles. Scalar profiles have 1024 passes and batch profiles 8192 passes, each after three same-size primes. All profiles precede decode, hash and archive work. No owned helper, build or other CPU benchmark overlaps capture. Profiles bracket the measured loop/accounting, exclude setup/primes/oracles, and have one sampled PID and zero lost records. Their symbol shares are not removable full-proof wall-time fractions.

| Caller | Variant | Samples | Lost |
|---|---|---:|---:|
{profile_table}
Detailed means, CPU usage, process RSS, allocation counts, symbols and pins are in [analysis](artifacts/analysis.json). Component RSS includes decoded fixtures and output buffers; it is not prover RAM or minimum memory. No new SDK/Gnark proof was generated; ledger remains 12,785. No new actual MSM caller, all-eight/54, portable/mobile/TEE, pressure/concurrency, large-circuit or AWS/Aeglos result is claimed. Passing only permits a separate caller/full-SDK qualification, not adoption.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These local proof timings and supplied references are inherited.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
{refs}
Accepted primary historical peak RSS is 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB historical peak with no supplied M5 reference. Current-source AWS/Aeglos has not been remeasured here.
"#,
        100. * coverage()
    );
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    fs::write(format!("{D}/README.md"), report).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &Path::new(D).join("artifacts"), &mut index);
    for f in [
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
