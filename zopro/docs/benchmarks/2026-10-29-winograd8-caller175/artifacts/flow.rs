use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/winograd8-caller175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-winograd8-caller175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const FIELD:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-production/frozen/candidate/libzopro_bn254-5778e0f87c7d66ba5e17a5f97bf5992c699ca4d2f3df952e07aca8ff03a40370.rlib";
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
fn output(command: &mut Command) -> String {
    let o = command.output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["full_performance_gate_passed"], false);
    let source = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(source.len(), 175);
    for (p, h) in source {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
        pin(&mut pins, r("source").join(p));
    }
    let original =
        fs::read_to_string(Path::new(Z).join("crates/zopro-core/src/fft/pair.rs")).unwrap();
    let within = fs::read_to_string(r("original-within.rs")).unwrap();
    assert!(original.contains(within.trim_end()));
    let log = fs::read_to_string(r("tests.log")).unwrap();
    assert!(log.contains("3 passed; 0 failed"));
    for name in [
        "basis_matrices_match_independent_dft_all_primitive_roots",
        "changing_recurrence_checks_all_fixtures_against_matrix_power",
        "full_width_boundaries_and_independent_sequences_match",
    ] {
        assert!(log.contains(&format!("test tests::{name} ... ok")));
    }
    let receipt =
        read("/home/ubuntu/src/zopro-perf-work/balanced-kh-production/frozen-candidate.json");
    assert_eq!(receipt["libraries"]["zopro_bn254"], FIELD);
    assert_eq!(receipt["rustflags"], m["rustflags"]);
    assert_eq!(
        hash(FIELD),
        "5778e0f87c7d66ba5e17a5f97bf5992c699ca4d2f3df952e07aca8ff03a40370"
    );
    pin(&mut pins, FIELD);
    pin(
        &mut pins,
        "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/frozen-candidate.json",
    );
    fs::write(
        r("host-before.txt"),
        output(Command::new("ps").args(["-eo", "pid,ppid,comm,pcpu,pmem,etimes", "--sort=-pcpu"])),
    )
    .unwrap();
    fs::write(r("compiler.txt"), output(Command::new("rustc").arg("-vV"))).unwrap();
    fs::write(r("cpu.txt"), output(&mut Command::new("lscpu"))).unwrap();
    fs::create_dir(r("perf")).unwrap();
    let mut jobs = vec![];
    let mut steps = vec![];
    for block in 0..31 {
        let mut powers = [1u64, 3, 5, 7];
        powers.rotate_left(block % 4);
        if block % 2 != 0 {
            powers.reverse();
        }
        for power in powers {
            let mut labels = ["A", "B", "candidate"];
            labels.rotate_left(block % 3);
            if block % 2 != 0 {
                labels.reverse();
            }
            for mode in labels {
                let name = format!("b{block}-p{power}-{mode}");
                let seed = 99173u64 + block as u64 * 8191 + power * 131;
                jobs.push(json!({"name":name,"block":block,"power":power,"mode":mode,"rounds":8191,"seed":seed}));
                steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","2",r("driver"),mode,power.to_string(),"8191",seed.to_string()],"timeout_seconds":120}));
            }
        }
    }
    assert_eq!(jobs.len(), 372);
    let mut profiles = vec![];
    for power in [1u64, 3] {
        for mode in ["A", "candidate"] {
            let name = format!("cycles-p{power}-{mode}");
            let data = r(format!("perf/{name}.data"));
            profiles
                .push(json!({"name":name,"power":power,"mode":mode,"data":data,"rounds":131071}));
            steps.push(json!({"kind":"command","name":name,"argv":["sudo","-n","perf","record","--no-buildid-cache","-e","cycles:u","-F","199","-m","512","-o",data,"--","taskset","-c","2",r("driver"),mode,power.to_string(),"131071","88771"],"timeout_seconds":180}));
        }
    }
    for j in &profiles {
        let name = j["name"].as_str().unwrap();
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["sudo","-n","perf","script","--header","--show-lost-events","-i",j["data"],"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["/home/ubuntu/src/zopro-perf-work/g2-norm-heap175/capture-helper","loss",j["data"]],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 384);
    save(r("jobs.json"), &json!(jobs));
    save(r("profiles.json"), &json!(profiles));
    for f in [
        "design.md",
        "algebra.md",
        "research.json",
        "build-commands.json",
        "driver.rs",
        "driver",
        "tests",
        "tests.log",
        "tests.stderr.log",
        "test-build.log",
        "build.log",
        "original-within.rs",
        "accepted-manifest.json",
        "flow.rs",
        "flow-helper",
        "pmu-feasibility.log",
        "compiler.txt",
        "cpu.txt",
        "jobs.json",
        "profiles.json",
    ] {
        pin(&mut pins, r(f));
    }
    pin(&mut pins, X);
    pin(
        &mut pins,
        "/home/ubuntu/src/zopro-perf-work/g2-norm-heap175/capture-helper",
    );
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared384steps/372latencies/4profiles; all175source exact;3new native tests;0SDK/ledger17239");
}
fn median(x: &[f64]) -> f64 {
    assert!(!x.is_empty());
    let mut s = x.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let jobs = read(r("jobs.json"));
    let mut cells = BTreeMap::<String, Vec<f64>>::new();
    let mut checksums = BTreeMap::<(u64, u64), Value>::new();
    let mut calls = 0;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let v: Value = serde_json::from_str(
            fs::read_to_string(r(format!("runs/{name}/stdout.log")))
                .unwrap()
                .trim(),
        )
        .unwrap();
        for k in ["mode", "power", "rounds", "seed"] {
            assert_eq!(v[k], j[k]);
        }
        assert_eq!(v["checked_outputs"], 1024);
        assert_eq!(v["priming_calls"], 192);
        let ns = v["ns_per_call"].as_f64().unwrap();
        assert!(ns.is_finite() && ns > 0.);
        calls += v["rounds"].as_u64().unwrap() + 192;
        let key = (j["power"].as_u64().unwrap(), j["block"].as_u64().unwrap());
        if let Some(old) = checksums.insert(key, v["checksum"].clone()) {
            assert_eq!(old, v["checksum"]);
        }
        cells
            .entry(format!("p{}/{}", j["power"], j["mode"].as_str().unwrap()))
            .or_default()
            .push(ns);
    }
    let mut summaries = BTreeMap::new();
    for (k, v) in &cells {
        assert_eq!(v.len(), 31);
        summaries.insert(k.clone(), json!({"ns":median(v),"blocks":v}));
    }
    let mut pairs = BTreeMap::new();
    let mut failed = vec![];
    for power in [1, 3, 5, 7] {
        for (a, b) in [("B", "A"), ("candidate", "A"), ("candidate", "B")] {
            let key = format!("p{power}/{a}/{b}");
            let x = &cells[&format!("p{power}/{a}")];
            let y = &cells[&format!("p{power}/{b}")];
            let ratios: Vec<_> = x.iter().zip(y).map(|(a, b)| a / b).collect();
            let savings: Vec<_> = x.iter().zip(y).map(|(a, b)| b - a).collect();
            let mut sorted = ratios.clone();
            sorted.sort_by(f64::total_cmp);
            let wins = ratios.iter().filter(|x| **x < 1.).count();
            let pass = if a == "B" {
                sorted[9] >= 1. / 1.03 && sorted[21] <= 1.03
            } else {
                sorted[21] < 1. && wins >= 21
            };
            if !pass {
                failed.push(key.clone());
            }
            pairs.insert(key,json!({"median":sorted[15],"low":sorted[9],"high":sorted[21],"wins":wins,"pass":pass,"ratios":ratios,"saving_ns":median(&savings)}));
        }
    }
    let mut profiles = vec![];
    for j in read(r("profiles.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
        let mut symbols = BTreeMap::<String, (u64, u64)>::new();
        let mut pids = BTreeSet::new();
        let mut samples = 0;
        let mut total = 0u64;
        for l in text
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
        {
            let f: Vec<_> = l.split_whitespace().collect();
            assert!(f.len() >= 4);
            let (pid, _) = f[0].split_once('/').unwrap();
            pids.insert(pid.parse::<u64>().unwrap());
            let period = f[1].parse::<u64>().unwrap();
            let s = symbols.entry(f[3..].join(" ")).or_default();
            s.0 += 1;
            s.1 += period;
            samples += 1;
            total += period;
        }
        let loss: Value = serde_json::from_str(
            fs::read_to_string(r(format!("runs/loss-{name}/stdout.log")))
                .unwrap()
                .trim(),
        )
        .unwrap();
        assert!(loss["lost_records"].as_array().unwrap().is_empty());
        assert!(pids.len() <= 1);
        let mut syms: Vec<_> = symbols
            .into_iter()
            .map(|(name, (n, p))| json!({"symbol":name,"samples":n,"period":p}))
            .collect();
        syms.sort_by(|a, b| {
            b["period"]
                .as_u64()
                .unwrap()
                .cmp(&a["period"].as_u64().unwrap())
        });
        profiles.push(json!({"name":name,"samples":samples,"period":total,"PIDs":pids,"lost":loss,"symbols":syms,"scope":"whole process including setup/primes/checks; sparse descriptive samples"}));
        let v: Value = serde_json::from_str(
            fs::read_to_string(r(format!("runs/{name}/stdout.log")))
                .unwrap()
                .trim(),
        )
        .unwrap();
        assert_eq!(v["checked_outputs"], 1024);
        assert_eq!(v["rounds"], 131071);
        calls += 131071 + 192;
    }
    let a = json!({"status":"complete","gate_passed":failed.is_empty(),"failed_checks":failed,"pairs":pairs,"cells":summaries,"profiles":profiles,"latency_processes":372,"profile_processes":4,"complete_leaf_calls_including_primes":calls,"new_native_tests":3,"field_tests_inherited":33,"production_source_unchanged":175,"SDK_requests":0,"new_proofs":0,"proof_ledger":17239,"adopted":false,"goal_achieved":false,"candidate_all8_qualified":false});
    save(r("analysis.json"), &a);
    println!(
        "Component gate{} failed{};{calls}complete leaf calls;0SDK/proofs;ledger17239",
        a["gate_passed"], a["failed_checks"]
    );
    for (k, v) in summaries {
        println!("{k}{}ns", v["ns"]);
    }
    for (k, v) in pairs {
        println!(
            "{k}{}[{},{}]wins{}save{}ns",
            v["median"], v["low"], v["high"], v["wins"], v["saving_ns"]
        );
    }
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path());
        }
    }
}
fn report() {
    let a = read(r("analysis.json"));
    let mut s = String::from("# Eight-point factored NTT caller\n\n");
    writeln!(s,"The declared caller gate is **{}**. This independently derived finite-field DFT8 replaces two eight-lane vector products with one, adding linear arithmetic and lane permutations. It transforms two independent eight-point inputs and returns the same bit-reversed outputs. No real FFT or SDK implementation is changed or adopted.\n",a["gate_passed"]).unwrap();
    s.push_str("The research lead is [Ingonyama's Winograd NTT note](https://www.ingonyama.com/oldblogs/pushing-the-limits-on-ntt-computation). All six pages were reviewed. Its hardware counts are not CPU speed predictions. The upstream GPL implementation was not ported; this kernel follows the independently derived [DFT8 identity](artifacts/algebra.md). The current control composes the original DIF butterfly with the verbatim accepted dif_within function. Both link the exact accepted native Fr R260 library; all 175 production hashes remain exact. Earlier fixed/serial leaves, pass grain and Euler compaction/fusion studies remain separate results.\n\n## Complete leaf arithmetic\n\nRyzen 9950X, CPU 2, one thread. Four primitive eighth roots, 31 fixed rotated/reversed process blocks per root, three labels. A and B use identical original code. Each process runs 192 priming calls, resets outside the timer, then performs 8,191 complete transformations on 64 changing preallocated fixtures. All loads, shuffles, arithmetic, stores and blackboxing are timed; construction and independent output checks are outside. All 1,024 final field outputs are checked. There is no owned build/helper/hash/archive/second workload during capture and no exclusive-host claim.\n\n| Root power | A ns | B ns | Factored ns | Factored/A ratio [bounds] | Wins | Factored/B ratio [bounds] | Wins |\n|---|---:|---:|---:|---|---:|---|---:|\n");
    for p in [1, 3, 5, 7] {
        let c = |m: &str| a["cells"][format!("p{p}/{m}")]["ns"].as_f64().unwrap();
        let x = &a["pairs"][format!("p{p}/candidate/A")];
        let y = &a["pairs"][format!("p{p}/candidate/B")];
        writeln!(s,"| {p} | {:.3} | {:.3} | {:.3} | {:.9} [{:.9}, {:.9}] | {} | {:.9} [{:.9}, {:.9}] | {} |",c("A"),c("B"),c("candidate"),x["median"].as_f64().unwrap(),x["low"].as_f64().unwrap(),x["high"].as_f64().unwrap(),x["wins"],y["median"].as_f64().unwrap(),y["low"].as_f64().unwrap(),y["high"].as_f64().unwrap(),y["wins"]).unwrap();
    }
    writeln!(s,"\nFailed checks: `{}`. Each root must improve against both controls with upper ratio below one and at least 21/31 wins. All B/A intervals must fit [1/1.03,1.03]. Bounds use sorted ranks 10–22 of 31, nominal 97.055% per comparison under independent exchangeable blocks, without familywise coverage. No minimum gain floor, exclusions, sample extensions, threshold changes or successful-capture repeats. Failure closes this vector factorization; passing only permits a separate real-FFT caller assessment including twiddles, packing and dataflow before SDK qualification.\n",a["failed_checks"]).unwrap();
    s.push_str("## Correctness and scope\n\nThree new native tests pass: all 16 basis inputs for each primitive root (both independent sequences); 4,096 boundary/full-width fixtures against scalar direct DFT; and changing recurrence at seven lengths through 8,191 calls, with every final fixture checked against independently exponentiated DFT matrices. Basis equality proves this linear identity over the field assuming the inherited field arithmetic. It does not prove an unimplemented large-FFT decomposition. Existing 33-test field qualification is inherited through exact source/artifact identity, not rerun.\n\nThe kernel adds no unsafe code, assembly, heap allocation, key table or input-sized buffer. Both methods share a three-vector constant payload, approximately 960 bytes per root; compiler frames and caller buffers are additional. These are not process RSS, minimum RAM or pressure bounds. Root constants are computed before priming. Extra additions and shuffles can erase multiplication savings.\n\nAll 372 latency processes finish before four whole-process 199 Hz cycles profiles. Profiles include setup, primes and output checks, are sparse, and do not isolate kernel costs or removable proof wall time.\n\n| Profile | Samples | PIDs | Lost records |\n|---|---:|---:|---:|\n");
    for p in a["profiles"].as_array().unwrap() {
        writeln!(
            s,
            "| {} | {} | {} | {} |",
            p["name"].as_str().unwrap(),
            p["samples"],
            p["PIDs"].as_array().unwrap().len(),
            p["lost"]["lost_records"].as_array().unwrap().len()
        )
        .unwrap();
    }
    writeln!(s,"\nAll **{} complete leaf calls including primes** finish with matching checked outputs. There are **zero new SDK requests or proofs**; the independent-proof ledger stays **17,239**. No new full-proof, AWS/Aeglos, portable/phone/TEE, giant, active-pressure, concurrency, RSS or minimum-RAM result follows. The original all-eight CPU 10× objective remains active and unmet, 0/8.\n",a["complete_leaf_calls_including_primes"]).unwrap();
    s.push_str("## Inherited CPU and Metal comparison\n\nAccepted source 175 measurements and supplied historical M5 references follow. These are not new measurements or candidate proof speeds. The accepted physical-16 baseline-calibration exception remains recorded in its adoption report.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU 10× target ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    let m = read(r("accepted-manifest.json"));
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
        let p = &m["m5"][shape];
        writeln!(
            s,
            "| {shape} | {:.6} | {} | {:.3}× | {} | {:.3}× | {:.1} |",
            p["accepted_ms"].as_f64().unwrap(),
            p["cpu_reference_ms"],
            p["cpu_speedup"].as_f64().unwrap(),
            p["metal_reference_ms"],
            p["metal_speedup"].as_f64().unwrap(),
            p["required_ms_for_CPU10x"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nAccepted 1×2 remains 8.022165 ms with 134 MiB peak RSS; its CPU target is 3.7 ms. Merge54 remains 162.250464 ms without a supplied M5 reference. [Design](artifacts/design.md), [algebra](artifacts/algebra.md), [analysis](artifacts/analysis.json), [native tests](artifacts/tests.log), [driver](artifacts/driver.rs), [commands](artifacts/run-plan.json), [capture completion](artifacts/runs/result.json), [research provenance](artifacts/research.json).\n");
    fs::write(Path::new(D).join("README.md"), s).unwrap();
}
fn seal() {
    let d = Path::new(D);
    let mut paths = vec![];
    walk(d, &mut paths);
    let mut pins = BTreeMap::new();
    for p in paths {
        let n = p.strip_prefix(d).unwrap().display().to_string();
        if ["artifact-index.json", "SHA256SUMS"].contains(&n.as_str()) {
            continue;
        }
        pins.insert(n, hash(p));
    }
    save(d.join("artifact-index.json"), &json!(pins));
    pins.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        pins.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Sealed{}hashes", pins.len());
}
fn archive() {
    let d = Path::new(D);
    assert!(!d.exists());
    assert_eq!(read(r("analysis.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut paths = vec![];
    walk(Path::new(R), &mut paths);
    let mut binaries = BTreeMap::new();
    let mut data = BTreeMap::new();
    for p in paths {
        let name = p.file_name().unwrap().to_str().unwrap();
        if name.starts_with("git-")
            || name.ends_with(".note")
            || [
                "archive.log",
                "archive.stderr.log",
                "strict-hashes.log",
                "commit.log",
                "push.log",
                "remote.log",
            ]
            .contains(&name)
        {
            continue;
        }
        if p.extension().is_some_and(|s| s == "data") {
            assert!(p.strip_prefix(R).unwrap().starts_with("perf"));
            assert!(Command::new("sudo")
                .args(["-n", "chown", "ubuntu:ubuntu"])
                .arg(&p)
                .status()
                .unwrap()
                .success());
            data.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let mut f = fs::File::open(&p).unwrap();
        let mut b = [0; 8];
        let n = f.read(&mut b).unwrap();
        if b[..n].starts_with(b"\x7fELF") || b[..n].starts_with(b"!<arch>") {
            binaries.insert(p.display().to_string(), hash(p));
            continue;
        }
        let to = d.join("artifacts").join(p.strip_prefix(R).unwrap());
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&p, &to).unwrap();
        assert_eq!(hash(p), hash(to));
    }
    assert_eq!(data.len(), 4);
    save(d.join("artifacts/local-perf-data.json"), &json!(data));
    save(d.join("artifacts/local-binaries.json"), &json!(binaries));
    report();
    seal();
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("prepare/finish/archive/seal"),
    }
}
