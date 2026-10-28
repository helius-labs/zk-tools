use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/unweighted-triple-caller174";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-unweighted-triple-caller174";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
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
    let mut buf = [0; 65536];
    loop {
        let n = f.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(pins: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn cases() -> [(&'static str, usize); 7] {
    [
        ("tile257", 1024),
        ("tile384", 1024),
        ("tile511", 1024),
        ("tile2048", 256),
        ("group35973", 16),
        ("groups31x256", 64),
        ("tails", 1024),
    ]
}
fn env() -> BTreeMap<String, String> {
    [
        ("RAYON_NUM_THREADS", "1"),
        ("ZOPRO_CPU_TREE_VECTOR_INVERSE", "1"),
        ("ZOPRO_CPU_G2_PREFIX_RECORDS", "1"),
        ("ZOPRO_CPU_DIRECT", "1"),
        ("ZOPRO_CPU_CANONICAL_BATCH", "1"),
        ("ZOPRO_CPU_VECTOR_INVERSE", "1"),
        ("ZOPRO_CPU_G1_SCRATCH_STORE", "2"),
        ("MALLOC_ARENA_MAX", "32"),
        ("MALLOC_MMAP_THRESHOLD_", "1073741824"),
        ("MALLOC_TRIM_THRESHOLD_", "-1"),
    ]
    .map(|(k, v)| (k.into(), v.into()))
    .into()
}
fn command(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":300})
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for (p, h) in read(r("frozen-candidate.json"))["input_pins"]
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    let executable = r("native/probe-sdk");
    assert!(executable.is_file());
    for p in [
        "native/result.json",
        "native/probe/artifacts.json",
        "native/probe-sdk",
        "frozen-candidate.json",
        "frozen-candidate-cargo.jsonl",
        "native/core/stdout.log",
        "native/core/stderr.log",
        "native/oracle/stdout.log",
        "native/oracle/stderr.log",
        "oracle-tests",
        "oracle-command.json",
        "oracle-build.stdout",
        "oracle-build.stderr",
        "capture.rs",
        "capture-helper",
        "host-before.txt",
        "rustc.txt",
        "design.md",
        "build-plan.json",
    ] {
        pin(&mut pins, r(p));
    }
    let oracle = read(r("oracle-command.json"));
    let args = oracle["argv"].as_array().unwrap();
    for pair in args.windows(2) {
        if pair[0] == "--extern" {
            let (_, p) = pair[1].as_str().unwrap().split_once('=').unwrap();
            pin(&mut pins, p);
        }
    }
    pin(&mut pins, X);
    let mut steps = Vec::new();
    let exe = executable.display().to_string();
    for (case, _) in cases() {
        for variant in ["binary", "ternary"] {
            steps.push(command(
                format!("preflight-{case}-{variant}"),
                vec![
                    "taskset".into(),
                    "-c".into(),
                    "2".into(),
                    exe.clone(),
                    case.into(),
                    variant.into(),
                    "3".into(),
                ],
            ));
        }
    }
    for block in 0..31 {
        let mut jobs: Vec<_> = cases()
            .into_iter()
            .flat_map(|(c, n)| ["binary", "ternary"].map(move |v| (c, n, v)))
            .collect();
        let len = jobs.len();
        jobs.rotate_left(block * 5 % len);
        if block % 2 == 1 {
            jobs.reverse();
        }
        for (case, n, variant) in jobs {
            steps.push(command(
                format!("lat-b{block:02}-{case}-{variant}"),
                vec![
                    "taskset".into(),
                    "-c".into(),
                    "2".into(),
                    exe.clone(),
                    case.into(),
                    variant.into(),
                    n.to_string(),
                ],
            ));
        }
    }
    let environment = env();
    for (case, n) in [("tile384", 262144), ("tile2048", 65536)] {
        for variant in ["binary", "ternary"] {
            let key = format!("{case}-{variant}");
            let path = r("profiles").join(&key);
            fs::create_dir_all(&path).unwrap();
            let mut argv: Vec<String> = vec!["sudo".into(), "-n".into(), "env".into()];
            argv.extend(environment.iter().map(|(k, v)| format!("{k}={v}")));
            argv.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "-e".into(),
                "cycles:u".into(),
                "-F".into(),
                "199".into(),
                "-o".into(),
                path.join("perf.data").display().to_string(),
                "--".into(),
                "taskset".into(),
                "-c".into(),
                "2".into(),
                exe.clone(),
                case.into(),
                variant.into(),
                n.to_string(),
            ]);
            steps.push(command(format!("profile-{key}"), argv));
        }
    }
    for case in ["tile384", "tile2048"] {
        for variant in ["binary", "ternary"] {
            let key = format!("{case}-{variant}");
            let data = r(format!("profiles/{key}/perf.data")).display().to_string();
            steps.push(command(
                format!("owner-{key}"),
                vec![
                    "sudo".into(),
                    "-n".into(),
                    "chown".into(),
                    "ubuntu:ubuntu".into(),
                    data.clone(),
                ],
            ));
            steps.push(command(
                format!("decode-{key}"),
                vec![
                    "perf".into(),
                    "script".into(),
                    "--header".into(),
                    "-i".into(),
                    data.clone(),
                    "-F".into(),
                    "pid,tid,period,ip,sym".into(),
                ],
            ));
            steps.push(command(
                format!("loss-{key}"),
                vec![
                    "perf".into(),
                    "script".into(),
                    "-D".into(),
                    "-i".into(),
                    data,
                ],
            ));
        }
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"environment":environment,"steps":steps}),
    );
    save(
        r("declaration.json"),
        &json!({"cases":cases(),"blocks":31,"latency_processes":434,"preflights":14,"profiles":4,
        "same_executable":true,"main_cases":["tile384","tile511","tile2048","group35973"],"component_gate":"all main upper ratio<1 and>=21wins; no gain floor",
        "source_files":175,"production_changed":false,"new_proof_exports":0,"independent_proof_count":15553,"goal_achieved":false}),
    );
    println!(
        "Prepared434latency+14preflight+4profiles;{}pins",
        pins.len()
    );
}
fn result(name: &str) -> Value {
    let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let lines: Vec<_> = text
        .lines()
        .filter_map(|s| s.strip_prefix("RESULT "))
        .collect();
    assert_eq!(lines.len(), 1);
    let v: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(v["verified_projective"], true);
    assert_eq!(v["SDK_requests"], 0);
    v
}
fn median(mut a: Vec<f64>) -> f64 {
    a.sort_by(f64::total_cmp);
    a[a.len() / 2]
}
fn stats(a: Vec<f64>) -> Value {
    assert_eq!(a.len(), 31);
    let mut b = a.clone();
    b.sort_by(f64::total_cmp);
    json!({"median":b[15],"low":b[9],"high":b[21],"wins":b.iter().filter(|&&v|v<1.).count(),"ratios":a})
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let mut rows = Vec::new();
    let mut main_pass = true;
    for (case, n) in cases() {
        let reference = result(&format!("preflight-{case}-binary"))["coordinates"].clone();
        assert_eq!(
            reference,
            result(&format!("preflight-{case}-ternary"))["coordinates"]
        );
        let mut times = BTreeMap::new();
        let mut memory = BTreeMap::new();
        for variant in ["binary", "ternary"] {
            let mut vals = Vec::new();
            let mut mem = None;
            for b in 0..31 {
                let v = result(&format!("lat-b{b:02}-{case}-{variant}"));
                assert_eq!(v["coordinates"], reference);
                assert_eq!(v["rounds"], n);
                if v["fresh_buffers"] == false {
                    assert_eq!(v["timed_allocation_calls"], 0);
                }
                let m = json!({"common":v["scratch_common_bytes"],"binary":v["scratch_binary_bytes"],"ternary":v["scratch_ternary_bytes"],"fixed":v["scratch_fixed_bytes"],"total":v["scratch_total_bytes"],"allocation_calls_per_round":v["allocation_calls_per_round"]});
                if let Some(old) = &mem {
                    assert_eq!(&m, old);
                } else {
                    mem = Some(m);
                }
                vals.push(v["ns_per_round"].as_f64().unwrap());
            }
            times.insert(variant, vals);
            memory.insert(variant, mem.unwrap());
        }
        let ratio = stats(
            times["ternary"]
                .iter()
                .zip(&times["binary"])
                .map(|(a, b)| a / b)
                .collect(),
        );
        let passed = ratio["high"].as_f64().unwrap() < 1. && ratio["wins"].as_u64().unwrap() >= 21;
        let main = ["tile384", "tile511", "tile2048", "group35973"].contains(&case);
        if main {
            main_pass &= passed;
        }
        rows.push(json!({"case":case,"rounds":n,"binary_ns":median(times["binary"].clone()),"ternary_ns":median(times["ternary"].clone()),
            "ratio":ratio,"gain_gate":passed,"main_case":main,"scratch_capacity_bytes":memory,"all_outputs_equal":true}));
    }
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    for case in ["tile384", "tile2048"] {
        for variant in ["binary", "ternary"] {
            let key = format!("{case}-{variant}");
            let output = result(&format!("profile-{key}"));
            let decode = fs::read_to_string(r(format!("runs/decode-{key}/stdout.log"))).unwrap();
            let lost = fs::read_to_string(r(format!("runs/loss-{key}/stdout.log"))).unwrap();
            assert!(!lost.contains("PERF_RECORD_LOST"));
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut pids = BTreeSet::new();
            let mut samples = 0;
            let mut period = 0;
            for line in decode.lines().filter(|s| !s.trim_start().starts_with('#')) {
                let w: Vec<_> = line.split_whitespace().collect();
                if w.len() < 4 {
                    continue;
                }
                let row = if let Some((p, _)) = w[0].split_once('/') {
                    p.parse::<u64>()
                        .ok()
                        .zip(w[1].parse::<u64>().ok())
                        .map(|(p, n)| (p, n, 3))
                } else if w.len() >= 5 {
                    w[0].parse::<u64>()
                        .ok()
                        .zip(w[2].parse::<u64>().ok())
                        .map(|(p, n)| (p, n, 4))
                } else {
                    None
                };
                if let Some((p, n, start)) = row {
                    pids.insert(p);
                    samples += 1;
                    period += n;
                    *symbols.entry(w[start..].join(" ")).or_default() += n;
                }
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            pin(&mut raw, r(format!("profiles/{key}/perf.data")));
            profiles.insert(key,json!({"samples":samples,"period":period,"pids":pids,"lost":0,"symbols":symbols,"output":output,"whole_process":true}));
        }
    }
    save(
        r("analysis.json"),
        &json!({"status":"complete","rows":rows,"main_component_gate":main_pass,"profiles":profiles,
        "raw_profile_sha256":raw,"all_closing_pins_match":true,"latency_processes":434,"preflight_processes":14,
        "profile_processes":4,"SDK_requests":0,"new_proof_exports":0,"independent_proof_count":15553,"production_changed":false,
        "adopted":false,"goal_achieved":false,"scope":"complete single-thread unweighted G1 caller; modeled inputs; no SDK or memory-pressure qualification"}),
    );
    for row in rows {
        println!(
            "{} binary{:.3}us ternary{:.3}us ratio{:.6} [{:.6},{:.6}] wins{} gate{}",
            row["case"],
            row["binary_ns"].as_f64().unwrap() / 1000.,
            row["ternary_ns"].as_f64().unwrap() / 1000.,
            row["ratio"]["median"].as_f64().unwrap(),
            row["ratio"]["low"].as_f64().unwrap(),
            row["ratio"]["high"].as_f64().unwrap(),
            row["ratio"]["wins"],
            row["gain_gate"]
        );
    }
    println!("Main component gate {main_pass}; no proof-time claim");
}
fn copy_tree(root: &Path, dir: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if e.file_type().unwrap().is_dir() {
            if name.starts_with("target-") || ["frozen", "profiles"].contains(&name.as_str()) {
                continue;
            }
            copy_tree(root, &p, dest, index);
        } else {
            let rel = p.strip_prefix(root).unwrap();
            let source = rel.starts_with("source");
            if !source
                && !["rs", "json", "md", "log", "txt", "stdout", "stderr"]
                    .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            {
                continue;
            }
            if [
                "archive.log",
                "push.log",
                "remote.log",
                "commit.log",
                "strict-hashes.log",
                "staged-whitespace.log",
            ]
            .contains(&name.as_str())
            {
                continue;
            }
            let to = dest.join(rel);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(&p, &to).unwrap();
            index.insert(format!("artifacts/{}", rel.display()), hash(to));
        }
    }
}
fn archive() {
    assert!(r("analysis.json").exists());
    let dest = Path::new(D);
    fs::create_dir_all(dest.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    copy_tree(
        Path::new(R),
        Path::new(R),
        &dest.join("artifacts"),
        &mut index,
    );
    let source_count = index
        .keys()
        .filter(|s| s.starts_with("artifacts/source/"))
        .count();
    assert_eq!(source_count, 175);
    save(dest.join("artifact-index.json"), &json!(index));
    fs::write(
        dest.join("SHA256SUMS"),
        index
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!(
        "Archived{}hashedfiles including175source; no binaries/perfdata/targets",
        index.len()
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("analyze") => analyze(),
        Some("archive") => archive(),
        _ => panic!("prepare/analyze/archive"),
    }
}
