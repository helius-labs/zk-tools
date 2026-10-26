use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    path::Path,
    process::Command,
};
const R: &str = "/tmp/zopro-lazy-linear";
const REPORT: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-lazy-linear";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    assert!(n > 0);
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.
    }
}
fn gm(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn build() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("native/field/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        37
    );
    let mut found = BTreeSet::new();
    for line in fs::read_to_string(r.join("native/libraries/stdout.log"))
        .unwrap()
        .lines()
    {
        let v: Value = serde_json::from_str(line).unwrap();
        if v["reason"] == "compiler-artifact" && v["target"]["name"] == "zopro_bn254" {
            for f in v["filenames"].as_array().unwrap() {
                let f = f.as_str().unwrap();
                if f.ends_with(".rlib") {
                    found.insert(f.to_string());
                }
            }
        }
    }
    assert_eq!(found.len(), 1);
    let candidate = found.first().unwrap();
    let old = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("source-pins.json"))).unwrap();
    add(candidate, &mut pins);
    let mut argv = vec![
        "rustc".to_string(),
        "--edition=2021".into(),
        "-O".into(),
        "-C".into(),
        "target-cpu=native".into(),
        "-C".into(),
        "lto=fat".into(),
        "-C".into(),
        "codegen-units=1".into(),
    ];
    for dep in [
        Path::new(candidate).parent().unwrap(),
        Path::new(old["libraries"]["zopro_bn254"].as_str().unwrap())
            .parent()
            .unwrap(),
    ] {
        argv.extend(["-L".into(), format!("dependency={}", dep.display())]);
    }
    for (name, p) in [
        (
            "frozen_bn254",
            old["libraries"]["zopro_bn254"].as_str().unwrap(),
        ),
        ("zopro_bn254", candidate.as_str()),
        (
            "serde_json",
            old["libraries"]["serde_json"].as_str().unwrap(),
        ),
        ("libc", old["libraries"]["libc"].as_str().unwrap()),
    ] {
        add(p, &mut pins);
        argv.extend(["--extern".into(), format!("{name}={p}")]);
    }
    argv.extend([
        r.join("probe.rs").display().to_string(),
        "-o".into(),
        r.join("probe").display().to_string(),
    ]);
    for file in [
        "probe.rs",
        "support/perf.rs",
        "pipeline.rs",
        "pipeline",
        "native-plan.json",
        "native/result.json",
        "native/libraries/stdout.log",
        "source-audit.json",
    ] {
        add(r.join(file), &mut pins);
    }
    save(r.join("link-pins.json"), &json!(pins));
    save(
        r.join("link-plan.json"),
        &json!({"version":1,"source_pins":r.join("link-pins.json"),"steps":[{"kind":"command","name":"consumer","argv":argv,"timeout_seconds":600}]}),
    );
    save(
        r.join("libraries.json"),
        &json!({"frozen":old["libraries"]["zopro_bn254"],"candidate":candidate,"frozen_sha256":hash(old["libraries"]["zopro_bn254"].as_str().unwrap()),"candidate_sha256":hash(candidate)}),
    );
    println!("Prepared linked exact-library consumer");
}
fn jobs() {
    let r = Path::new(R);
    assert_eq!(read(r.join("link/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("link-pins.json"))).unwrap();
    for f in ["probe", "libraries.json", "link/result.json"] {
        add(r.join(f), &mut pins);
    }
    add("/usr/bin/perf", &mut pins);
    let mut steps = Vec::new();
    let mut jobs = Vec::new();
    let mut cases = Vec::new();
    for field in ["fq", "fr"] {
        for radix in ["256", "260"] {
            for pattern in 0..4 {
                cases.push((field, radix, pattern));
            }
        }
    }
    for block in 0..7 {
        let mut ordered = cases.clone();
        let len = ordered.len();
        ordered.rotate_left(block * 3 % len);
        if block % 2 == 1 {
            ordered.reverse();
        }
        for (field, radix, pattern) in ordered {
            let mut modes = vec!["frozen", "fresh", "lazy"];
            modes.rotate_left((block + pattern as usize) % 3);
            if block % 2 == 1 {
                modes.reverse();
            }
            for mode in modes {
                let name = format!("b{block}-{field}-{radix}-p{pattern}-{mode}");
                let argv = json!([
                    "taskset",
                    "-c",
                    "2",
                    r.join("probe"),
                    field,
                    radix,
                    mode,
                    pattern.to_string(),
                    "1000000",
                    block.to_string()
                ]);
                steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":120}));
                jobs.push(json!({"phase":"latency","name":name,"field":field,"radix":radix,"pattern":pattern,"mode":mode,"block":block,"iterations":1000000}));
            }
        }
    }
    for mode in ["frozen", "fresh", "lazy"] {
        let profile = r.join("profiles").join(mode);
        fs::create_dir_all(&profile).unwrap();
        for f in ["ctl", "ack"] {
            let status = Command::new("mkfifo")
                .arg(profile.join(f))
                .status()
                .unwrap();
            assert!(status.success());
        }
        let data = profile.join("perf.data");
        let control = format!(
            "--control=fifo:{},{}",
            profile.join("ctl").display(),
            profile.join("ack").display()
        );
        steps.push(json!({"kind":"command","name":format!("profile-{mode}"),"argv":["sudo","-n","/usr/bin/perf","record","--no-buildid-cache","--clockid","mono","--delay=-1",control,"-e","cycles:u","-F","999","-o",data,"--","sudo","-n","-u","ubuntu","env",format!("ZOPRO_BENCH_PERF_CTL={}",profile.join("ctl").display()),format!("ZOPRO_BENCH_PERF_ACK={}",profile.join("ack").display()),"taskset","-c","2",r.join("probe"),"fq","256",mode,"3","20000000","99"],"timeout_seconds":180}));
        jobs.push(json!({"phase":"profile","name":format!("profile-{mode}"),"field":"fq","radix":"256","pattern":3,"mode":mode,"iterations":20000000}));
    }
    // Decode only after every latency and profile capture.
    for mode in ["frozen", "fresh", "lazy"] {
        let data = r.join("profiles").join(mode).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("samples-{mode}"),"argv":["sudo","-n","/usr/bin/perf","script","--show-lost-events","-i",data,"-F","comm,pid,tid,cpu,time,event,ip,sym,period"],"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("report-{mode}"),"argv":["sudo","-n","/usr/bin/perf","report","--stdio","--header","--percent-limit","0","-i",data],"timeout_seconds":180}));
    }
    save(r.join("measurement-pins.json"), &json!(pins));
    save(r.join("jobs.json"), &json!(jobs));
    save(
        r.join("measurement-plan.json"),
        &json!({"version":1,"source_pins":r.join("measurement-pins.json"),"steps":steps}),
    );
    println!("Frozen336latency processes then3profiles; zero SDK/Gnark calls");
}
fn analyze() {
    let r = Path::new(R);
    assert_eq!(read(r.join("decode/result.json"))["status"], "complete");
    for (p, h) in read(r.join("measurement-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut rows = Vec::new();
    for job in read(r.join("jobs.json")).as_array().unwrap() {
        let log = fs::read_to_string(
            r.join("measure")
                .join(job["name"].as_str().unwrap())
                .join("stdout.log"),
        )
        .unwrap();
        let result: Value =
            serde_json::from_str(log.lines().find_map(|s| s.strip_prefix("RESULT ")).unwrap())
                .unwrap();
        assert_eq!(result["frozen_reference_equal"], true);
        assert_eq!(result["iterations"], job["iterations"]);
        rows.push(json!({"job":job,"result":result}));
    }
    assert_eq!(rows.len(), 339);
    let mut cases = BTreeMap::new();
    let mut calibration = Vec::new();
    let mut failures = Vec::new();
    let mut agg_frozen = vec![Vec::new(); 7];
    let mut agg_fresh = vec![Vec::new(); 7];
    for field in ["fq", "fr"] {
        for radix in ["256", "260"] {
            for pattern in 0..4 {
                let name = format!("{field}-{radix}-p{pattern}");
                let get = |block: usize, mode: &str| -> &Value {
                    &rows
                        .iter()
                        .find(|r| {
                            r["job"]["phase"] == "latency"
                                && r["job"]["field"] == field
                                && r["job"]["radix"] == radix
                                && r["job"]["pattern"] == pattern
                                && r["job"]["block"] == block
                                && r["job"]["mode"] == mode
                        })
                        .unwrap()["result"]
                };
                let mut timings = BTreeMap::new();
                for mode in ["frozen", "fresh", "lazy"] {
                    timings.insert(
                        mode,
                        median(
                            (0..7)
                                .map(|b| get(b, mode)["ns_per_lane_iteration"].as_f64().unwrap())
                                .collect(),
                        ),
                    );
                }
                let mut pairs = BTreeMap::new();
                for (a, b) in [("fresh", "frozen"), ("lazy", "fresh"), ("lazy", "frozen")] {
                    let ratios: Vec<_> = (0..7)
                        .map(|block| {
                            get(block, a)["ns_per_lane_iteration"].as_f64().unwrap()
                                / get(block, b)["ns_per_lane_iteration"].as_f64().unwrap()
                        })
                        .collect();
                    let ratio = gm(&ratios);
                    let wins = ratios.iter().filter(|x| **x < 1.).count();
                    if a == "fresh" {
                        for (block, x) in ratios.iter().enumerate() {
                            if !(1. / 1.03..=1.03).contains(x) {
                                calibration.push(format!("{name}/b{block}:fresh/frozen={x}"));
                            }
                        }
                        let m = median(ratios.clone());
                        if !(1. / 1.02..=1.02).contains(&m) {
                            calibration.push(format!("{name}:medianfresh/frozen={m}"));
                        }
                    } else if field == "fq" && radix == "256" {
                        if ratio > 1.03 {
                            failures.push(format!("{name}:{a}/{b}={ratio}>1.03"));
                        }
                        if pattern == 3 && (ratio >= 1. || wins < 4) {
                            failures.push(format!("{name}:{a}/{b}={ratio},wins{wins}/7"));
                        }
                        for (block, v) in ratios.iter().enumerate() {
                            if b == "fresh" {
                                agg_fresh[block].push(*v);
                            } else {
                                agg_frozen[block].push(*v);
                            }
                        }
                    }
                    pairs.insert(
                        format!("{a}/{b}"),
                        json!({"geometric_ratio":ratio,"wins":wins,"block_ratios":ratios}),
                    );
                }
                for block in 0..7 {
                    assert_eq!(
                        get(block, "frozen")["checksum"],
                        get(block, "fresh")["checksum"]
                    );
                    assert_eq!(
                        get(block, "frozen")["checksum"],
                        get(block, "lazy")["checksum"]
                    );
                }
                cases.insert(name, json!({"timings":timings,"pairs":pairs}));
            }
        }
    }
    let mut aggregates = BTreeMap::new();
    for (name, values) in [("lazy/fresh", agg_fresh), ("lazy/frozen", agg_frozen)] {
        let by_block: Vec<_> = values
            .iter()
            .map(|v| {
                assert_eq!(v.len(), 4);
                gm(v)
            })
            .collect();
        let ratio = gm(&by_block);
        let wins = by_block.iter().filter(|x| **x < 1.).count();
        if ratio >= 1. || wins < 4 {
            failures.push(format!("FqR256aggregate {name}={ratio},wins{wins}/7"));
        }
        aggregates.insert(
            name,
            json!({"ratio":ratio,"wins":wins,"block_ratios":by_block}),
        );
    }
    let mut profiles = BTreeMap::new();
    for mode in ["frozen", "fresh", "lazy"] {
        let text = fs::read_to_string(r.join(format!("decode/samples-{mode}/stdout.log"))).unwrap();
        let report =
            fs::read_to_string(r.join(format!("decode/report-{mode}/stdout.log"))).unwrap();
        assert!(!text.contains("PERF_RECORD_LOST"));
        assert!(report.contains("Total Lost Samples: 0"));
        let mut pids = BTreeSet::new();
        let mut count = 0;
        for l in text.lines().filter(|s| s.contains("cycles:u:")) {
            let p: Vec<_> = l.split_whitespace().collect();
            pids.insert(p[1].split('/').next().unwrap().parse::<u32>().unwrap());
            count += 1;
        }
        assert_eq!(pids.len(), 1);
        assert!(count > 0);
        let out = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(r.join(format!("profiles/{mode}/perf.data")))
            .output()
            .unwrap();
        assert!(out.status.success());
        profiles.insert(mode,json!({"samples":count,"lost":0,"pids":pids,"sha256":String::from_utf8(out.stdout).unwrap().split_whitespace().next().unwrap()}));
    }
    let passed = calibration.is_empty() && failures.is_empty();
    save(
        r.join("analysis.json"),
        &json!({"component_passed":passed,"calibration_failures":calibration,"failures":failures,"cases":cases,"aggregates":aggregates,"profiles":profiles,"rows":rows,"sdk_proofs":0,"gnark_exports":0,"next_prior_count":6850,"adopted":false,"goal_achieved":false}),
    );
    println!(
        "componentpassed={passed}; {}calibration failures; {}speed failures",
        calibration.len(),
        failures.len()
    );
}
fn report() {
    let r = Path::new(R);
    let a = read(r.join("analysis.json"));
    let mut md=format!("# Lazy linear SIMD field operations\n\nThe arithmetic component {} its predeclared gate. Production remains unchanged, and this is not a whole-proof speed measurement.\n\nThe earlier R260 bounded-subtraction probe produced canonical outputs and failed its historical3% gain floor despite a small measured gain. That result remains unchanged. This new Rust primitive returns normalized lazy values below2p: it omits the old final canonical correction, adds a matching addition operation, and selects quotient-times-modulus coefficients through four fixed choices per digit. All old field-operation bodies remain identical.\n\nFor X=a+b or X=2p+a-b, a downward leading-digit quotient gives0<=X-qp<2p. One signed carry pass therefore suffices. The quotient is0..3. Five small constant vectors replace five low64-bit multiplications; at most320bytes per eight-lane table instance, independent of key/input size. No point table, input cache, heap buffer, key conversion, scheduler or production dispatch changes. [Frozen design and bounds](artifacts/design.md).\n\n## Component measurements\n\nRyzen9950X, one thread on CPU2, seven rotated/reversed fresh-process blocks. The consumer explicitly links both the exact accepted native field library and candidate library. Modes are accepted, fresh ordinary and lazy. Each process uses three512-iteration primes and1,000,000timed iterations, with matched changing input vectors. Inputs/packing and frozen-reference setup/checks are outside timing; the entire arithmetic kernel and output materialization are inside. Streams4 divides per-lane figures by four. The curve expression uses existing multiply plus add/sub operations; it is not a complete elliptic-curve addition or proof. No owned build/profile/hash/archive overlaps latency.\n\n|Field/radix/pattern|Accepted ns/lane iteration|Fresh ns|Lazy ns|Lazy/fresh paired|Wins|\n|---|---:|---:|---:|---:|---:|\n",if a["component_passed"]==true{"passes"}else{"fails"});
    for (name, cell) in a["cases"].as_object().unwrap() {
        let t = &cell["timings"];
        let p = &cell["pairs"]["lazy/fresh"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.9}|{}/7|",
            t["frozen"].as_f64().unwrap(),
            t["fresh"].as_f64().unwrap(),
            t["lazy"].as_f64().unwrap(),
            p["geometric_ratio"].as_f64().unwrap(),
            p["wins"]
        )
        .unwrap();
    }
    md.push_str("\np0=dependent add,p1=dependent subtract,p2=four independent alternating add/sub streams,p3=dependent affine-expression arithmetic. Displayed times are medians of process times; ratios are geometric means of paired block ratios. Reference outputs and all128 final64-bit words per process agree.\n\nThe intended Fq/R256 curve expression and four-pattern aggregate must improve against both controls with4/7winning blocks. Every Fq/R256 pattern must remain within3% of each control. Fresh/accepted calibration must stay within factor1.03eachblock and1.02pairedmedian across all16cases. Other field/radix timing is descriptive; its correctness is mandatory. No minimum gain floor, exclusions or unchanged retry.\n");
    for key in ["calibration_failures", "failures"] {
        writeln!(md, "\n{key}:\n").unwrap();
        if a[key].as_array().unwrap().is_empty() {
            md.push_str("- None.\n");
        } else {
            for f in a[key].as_array().unwrap() {
                writeln!(md, "- {}", f.as_str().unwrap()).unwrap();
            }
        }
    }
    md.push_str("\n## Correctness and profiles\n\nAll37 native field tests and generator consistency pass. New tests check exact unnormalized quotient/remainder against independent Arkworks big integers, inclusive2p/limb/modulus boundaries, random loose values and eight-step add/sub/multiply/fused-product chains against Arkworks in both primes, both Montgomery radices and four/eight lanes. The independent reduced-radix model checks1,600,784 operation cases, quotient bounds, signed carries, exact remainders and closure below2p. The range argument and finite tests are not a formal machine-code proof. The first generator run stopped before writing output because a test helper name matched its insertion marker; only test helper names changed before compilation.\n\nAll336latency processes finish before three999Hz cycles:u profiles of the intended expression,20million iterations plus three primes each. Warm FIFO gating excludes reference setup and primes. Binaries and root-owned raw captures remain local with checked hashes; --no-buildid-cache avoids executable metadata changes. Profiles are descriptive cycle samples, not exclusive opcode costs or removable proof wall time.\n\n|Mode|Samples|Lost|PIDs|\n|---|---:|---:|---:|\n");
    for mode in ["frozen", "fresh", "lazy"] {
        let p = &a["profiles"][mode];
        writeln!(
            md,
            "|{mode}|{}|{}|{}|",
            p["samples"],
            p["lost"],
            p["pids"].as_array().unwrap().len()
        )
        .unwrap();
    }
    md.push_str("\nZero new SDK or Gnark proofs; prior6,850unchanged. All170 production files and exact source/library/consumer/tool pins are checked unchanged. A component pass only motivates separately declared actual-caller/full-SDK, consumer, portable and resource qualification. No AWS/Aeglos,phone,TEE,giant,concurrent,pressure,minimum-RAM or full-proof gain is claimed. [Latest complete SDK CPU/Metal comparison](../2026-10-26-pairing-x-chain-sdk/README.md) retains all eight supplied references; primitive nanoseconds do not replace proof timings. The all-eight CPU10×goal remains active and unmet.\n\n[Analysis](artifacts/analysis.json),[source audit](artifacts/source-audit.json),[native tests](artifacts/native/field/stdout.log),[model](artifacts/native/model-tests/stdout.log),[consumer](artifacts/probe.rs),[jobs](artifacts/jobs.json),[artifact hashes](artifact-index.json).\n");
    let d = Path::new(REPORT);
    fs::create_dir_all(d).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    let mut hashes = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut hashes);
    save(d.join("artifact-index.json"), &json!(hashes));
    println!("Archived{}artifacts", hashes.len());
}
fn archive(root: &Path, p: &Path, d: &Path, out: &mut BTreeMap<String, String>) {
    let mut entries: Vec<_> = fs::read_dir(p)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for p in entries {
        let rel = p.strip_prefix(root).unwrap();
        if p.is_dir() {
            if rel.starts_with("target") {
                continue;
            }
            archive(root, &p, d, out);
        } else {
            let name = p.file_name().unwrap().to_string_lossy();
            if name == "perf.data"
                || !["rs", "json", "jsonl", "toml", "lock", "md", "txt", "log"]
                    .iter()
                    .any(|e| p.extension().is_some_and(|x| x == *e))
            {
                continue;
            }
            let q = d.join(rel);
            fs::create_dir_all(q.parent().unwrap()).unwrap();
            fs::copy(&p, &q).unwrap();
            let h = hash(&q);
            assert_eq!(h, hash(&p));
            out.insert(rel.display().to_string(), h);
        }
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("build") => build(),
        Some("jobs") => jobs(),
        Some("analyze") => analyze(),
        Some("report") => report(),
        _ => panic!("mode"),
    }
}
