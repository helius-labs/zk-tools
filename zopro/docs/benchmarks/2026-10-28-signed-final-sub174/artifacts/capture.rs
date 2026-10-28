use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/signed-final-sub174";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-signed-final-sub174";
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
fn cases() -> Vec<(&'static str, usize, usize)> {
    ["g1-owned", "g1-borrowed-signed", "g2-prefix"]
        .into_iter()
        .flat_map(|c| [(c, 200, 4096), (c, 352, 2048)])
        .collect()
}
fn executable(v: &str) -> PathBuf {
    r(format!(
        "native/{}-sdk",
        if v == "OFF" {
            "control"
        } else {
            assert_eq!(v, "ON");
            "candidate"
        }
    ))
}
fn environment() -> BTreeMap<String, String> {
    [
        ("RAYON_NUM_THREADS", "1"),
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
fn step(name: String, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":300})
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    assert_eq!(read(r("native-completion.json"))["unique_successful_steps"],10);
    for (p,h) in read(r("repair-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p),h.as_str().unwrap(),"{p}");
        pins.insert(p.clone(),h.as_str().unwrap().to_string());
    }
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for label in ["control", "candidate"] {
        let frozen = read(r(format!("frozen-{label}.json")));
        for (p, h) in frozen["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
        for p in [
            format!("frozen-{label}.json"),
            format!("frozen-{label}-cargo.jsonl"),
            format!("native/{label}-sdk"),
            format!("native/{label}/artifacts.json"),
            format!("native/zopro-bn254-{label}/stdout.log"),
            format!("native/zopro-bn254-{label}/stderr.log"),
            format!("native/zopro-core-{label}/stdout.log"),
            format!("native/zopro-core-{label}/stderr.log"),
        ] {
            pin(&mut pins, r(p));
        }
    }
    for p in [
        "capture.rs",
        "capture-helper",
        "host-before.txt",
        "rustc.txt",
        "native/result.json",
        "build-plan.json",
        "design.md",
        "bounds.md",
        "history-audit.md",
        "native-completion.json",
        "build-plan-repair.json",
        "harness-repair.json",
        "link-control-command.json",
        "link-candidate-command.json",
    ] {
        pin(&mut pins, r(p));
    }
    pin(&mut pins, X);
    let mut steps = Vec::new();
    for (c, size, _) in cases() {
        for v in ["OFF", "ON"] {
            steps.push(step(
                format!("preflight-{c}-{size}-{v}"),
                vec![
                    "taskset".into(),
                    "-c".into(),
                    "2".into(),
                    executable(v).display().to_string(),
                    c.into(),
                    size.to_string(),
                    "17".into(),
                ],
            ));
        }
    }
    for b in 0..31 {
        let mut jobs: Vec<_> = cases()
            .into_iter()
            .flat_map(|(c, s, n)| ["OFF", "ON"].map(move |v| (c, s, n, v)))
            .collect();
        let len = jobs.len();
        jobs.rotate_left(b * 5 % len);
        if b % 2 == 1 {
            jobs.reverse();
        }
        for (c, size, n, v) in jobs {
            steps.push(step(
                format!("lat-b{b:02}-{c}-{size}-{v}"),
                vec![
                    "taskset".into(),
                    "-c".into(),
                    "2".into(),
                    executable(v).display().to_string(),
                    c.into(),
                    size.to_string(),
                    n.to_string(),
                ],
            ));
        }
    }
    let environment = environment();
    for size in [200, 352] {
        for v in ["OFF", "ON"] {
            let key = format!("g1-borrowed-signed-{size}-{v}");
            let p = r(format!("profiles/{key}"));
            fs::create_dir_all(&p).unwrap();
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
                p.join("perf.data").display().to_string(),
                "--".into(),
                "taskset".into(),
                "-c".into(),
                "2".into(),
                executable(v).display().to_string(),
                "g1-borrowed-signed".into(),
                size.to_string(),
                "1048576".into(),
            ]);
            steps.push(step(format!("profile-{key}"), argv));
        }
    }
    for size in [200, 352] {
        for v in ["OFF", "ON"] {
            let key = format!("g1-borrowed-signed-{size}-{v}");
            let file = r(format!("profiles/{key}/perf.data")).display().to_string();
            steps.push(step(
                format!("owner-{key}"),
                vec![
                    "sudo".into(),
                    "-n".into(),
                    "chown".into(),
                    "ubuntu:ubuntu".into(),
                    file.clone(),
                ],
            ));
            steps.push(step(
                format!("decode-{key}"),
                vec![
                    "perf".into(),
                    "script".into(),
                    "--header".into(),
                    "-i".into(),
                    file.clone(),
                    "-F".into(),
                    "pid,tid,period,ip,sym".into(),
                ],
            ));
            steps.push(step(
                format!("loss-{key}"),
                vec![
                    "perf".into(),
                    "script".into(),
                    "-D".into(),
                    "-i".into(),
                    file,
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
        &json!({"cases":cases(),"blocks":31,"latency_processes":372,"preflights":12,"profiles":4,"variant_libraries_frozen":true,"source_files":176,
        "component_gate":"ALL4G1 upper<1 and>=21wins;G2 paired medians within3percent;zero timed allocation/identical capacity/outputs","SDK_requests":0,"proof_ledger":15553,"goal_achieved":false}),
    );
    println!(
        "Prepared372latency+12preflight+4profiles;{}pins",
        pins.len()
    );
}
fn result(name: &str) -> Value {
    let t = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let lines: Vec<_> = t
        .lines()
        .filter_map(|s| s.strip_prefix("RESULT "))
        .collect();
    assert_eq!(lines.len(), 1);
    let v: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(v["internally_verified"], true);
    assert_eq!(v["timed_allocations"], 0);
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
fn static_code() -> Value {
    let mut all = BTreeMap::new();
    for v in ["OFF", "ON"] {
        let output = Command::new("objdump")
            .args(["-Cd", "--no-show-raw-insn"])
            .arg(executable(v))
            .output()
            .unwrap();
        assert!(output.status.success());
        let path = r(format!("assembly-{v}.local"));
        fs::write(&path, &output.stdout).unwrap();
        let text = String::from_utf8(output.stdout).unwrap();
        let mut current = String::new();
        let mut lines = Vec::new();
        let mut selected = String::new();
        let mut bodies = Vec::new();
        let mut emit = |name: &str, lines: &Vec<String>| {
            if name.contains("msm::stream_g1_impl") {
                selected.push_str(name);
                selected.push('\n');
                for line in lines {
                    selected.push_str(line);
                    selected.push('\n');
                }
                bodies.push(json!({"symbol":name,"instruction_sites":lines.len(),"IFMA_sites":lines.iter().filter(|s|s.contains("vpmadd52")).count(),"RSP_operand_lines":lines.iter().filter(|s|s.contains("%rsp")).count()}));
            }
        };
        for line in text.lines() {
            if line.ends_with(">:") {
                emit(&current, &lines);
                current = line.into();
                lines.clear();
            } else if line.contains(':') && !line.trim().is_empty() {
                lines.push(line.into());
            }
        }
        emit(&current, &lines);
        assert!(!bodies.is_empty());
        fs::write(r(format!("hot-{v}.s")), selected).unwrap();
        all.insert(v,json!({"whole_assembly_sha256":hash(&path),"functions":bodies,"static_only_not_executed_counts":true}));
    }
    json!(all)
}
fn analyze() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let mut rows = Vec::new();
    let mut component = true;
    for (c, size, n) in cases() {
        let mut times = BTreeMap::new();
        let mut capacity = None;
        let mut output = None;
        for v in ["OFF", "ON"] {
            let pre = result(&format!("preflight-{c}-{size}-{v}"));
            assert_eq!(pre["iterations"], 17);
            let mut values = Vec::new();
            for b in 0..31 {
                let row = result(&format!("lat-b{b:02}-{c}-{size}-{v}"));
                assert_eq!(row["iterations"], n);
                if let Some(old) = &output {
                    assert_eq!(old, &row["output_sha256"]);
                } else {
                    output = Some(row["output_sha256"].clone());
                }
                let m = json!({"scratch":row["capacity_bytes"],"borrowed_references":row["fixture_reference_bytes"]});
                if let Some(old) = &capacity {
                    assert_eq!(old, &m);
                } else {
                    capacity = Some(m);
                }
                values.push(row["ns_per_round"].as_f64().unwrap());
            }
            times.insert(v, values);
        }
        let ratio = stats(
            times["ON"]
                .iter()
                .zip(&times["OFF"])
                .map(|(a, b)| a / b)
                .collect(),
        );
        let gain = ratio["high"].as_f64().unwrap() < 1. && ratio["wins"].as_u64().unwrap() >= 21;
        let negative = c == "g2-prefix";
        let pass = if negative {
            (ratio["median"].as_f64().unwrap() - 1.).abs() <= 0.03
        } else {
            gain
        };
        component &= pass;
        rows.push(json!({"case":format!("{c}-{size}"),"kind":c,"size":size,"rounds":n,"OFF_ns":median(times["OFF"].clone()),"ON_ns":median(times["ON"].clone()),"ratio":ratio,"gain_gate":gain,"required_gate":pass,"negative_control":negative,"capacity":capacity,"output_sha256":output}));
    }
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    let mut dumps = BTreeMap::new();
    for size in [200, 352] {
        for v in ["OFF", "ON"] {
            let key = format!("g1-borrowed-signed-{size}-{v}");
            let output = result(&format!("profile-{key}"));
            assert_eq!(output["iterations"], 1048576);
            let decode = fs::read_to_string(r(format!("runs/decode-{key}/stdout.log"))).unwrap();
            let loss = r(format!("runs/loss-{key}/stdout.log"));
            assert!(!fs::read_to_string(&loss)
                .unwrap()
                .contains("PERF_RECORD_LOST"));
            pin(&mut dumps, &loss);
            pin(&mut raw, r(format!("profiles/{key}/perf.data")));
            let mut symbols = BTreeMap::<String, u64>::new();
            let mut pids = BTreeSet::new();
            let mut samples = 0;
            let mut period = 0;
            for line in decode.lines().filter(|s| !s.trim_start().starts_with('#')) {
                let w: Vec<_> = line.split_whitespace().collect();
                if w.len() < 4 {
                    continue;
                }
                let parsed = if let Some((p, _)) = w[0].split_once('/') {
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
                if let Some((p, n, start)) = parsed {
                    pids.insert(p);
                    samples += 1;
                    period += n;
                    *symbols.entry(w[start..].join(" ")).or_default() += n;
                }
            }
            assert!(samples > 0);
            assert_eq!(pids.len(), 1);
            profiles.insert(key,json!({"samples":samples,"period":period,"pids":pids,"lost":0,"symbols":symbols,"output":output,"whole_process":true}));
        }
    }
    let code = static_code();
    save(
        r("analysis.json"),
        &json!({"status":"complete component","rows":rows,"component_gate":component,"profiles":profiles,"static_code":code,"raw_perf_sha256":raw,"raw_loss_dump_sha256":dumps,
        "all_closing_pins_match":true,"latency_processes":372,"preflights":12,"profiles_count":4,"new_field_tests":2,"caller_tests":3,"source_files":176,"production_changed":false,"adopted":false,
        "SDK_requests":0,"new_proof_exports":0,"independent_proof_count":15553,"goal_achieved":false}),
    );
    for row in rows {
        println!(
            "{} OFF{:.3}us ON{:.3}us ratio{:.8}[{:.8},{:.8}] wins{} required{}",
            row["case"],
            row["OFF_ns"].as_f64().unwrap() / 1000.,
            row["ON_ns"].as_f64().unwrap() / 1000.,
            row["ratio"]["median"].as_f64().unwrap(),
            row["ratio"]["low"].as_f64().unwrap(),
            row["ratio"]["high"].as_f64().unwrap(),
            row["ratio"]["wins"],
            row["required_gate"]
        );
    }
    println!("Main component gate{component};no proof-latency claim");
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
            let s = rel.to_string_lossy();
            if s.starts_with("runs/loss-") && name == "stdout.log" {
                continue;
            }
            if !rel.starts_with("source")
                && !["rs", "json", "md", "log", "txt", "s"]
                    .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            {
                continue;
            }
            if [
                "archive.log",
                "commit.log",
                "push.log",
                "remote.log",
                "publication.log",
                "strict-hashes.log",
                "staged-whitespace.log",
                "staged-whitespace-final.log",
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
    let dest = Path::new(D);
    fs::create_dir_all(dest.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    copy_tree(
        Path::new(R),
        Path::new(R),
        &dest.join("artifacts"),
        &mut index,
    );
    assert_eq!(
        index
            .keys()
            .filter(|s| s.starts_with("artifacts/source/"))
            .count(),
        176
    );
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
        "Archived{}hashes/176source; raw perf/dumps local hashed",
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
