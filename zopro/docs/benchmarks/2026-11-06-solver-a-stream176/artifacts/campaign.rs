use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/solver-a-stream176";
const CASES: &str = "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176/cases.json";
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, pins: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, pins)
        } else {
            pins.insert(p.display().to_string(), hash(p));
        }
    }
}
fn args(shape: &str, w: usize, mode: &str, n: usize, order: usize) -> Vec<String> {
    vec![
        "taskset".into(),
        "-c".into(),
        if w == 1 {
            "2".into()
        } else {
            format!("0-{}", w - 1)
        },
        r("caller-bin").display().to_string(),
        CASES.into(),
        shape.into(),
        w.to_string(),
        mode.into(),
        n.to_string(),
        order.to_string(),
    ]
}
fn step(name: impl Into<String>, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name.into(),"argv":argv,"timeout_seconds":600})
}
fn caller(out: &str, name: &str) -> Value {
    let text = fs::read_to_string(r(format!("{out}/{name}/stdout.log"))).unwrap();
    let lines = text
        .lines()
        .filter_map(|l| l.strip_prefix("CALLER "))
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);
    serde_json::from_str(lines[0]).unwrap()
}
fn pins() -> BTreeMap<String, String> {
    let mut pins = BTreeMap::new();
    for d in ["source", "driver", "native"] {
        walk(&r(d), &mut pins);
    }
    for f in [
        "setup.rs",
        "design.md",
        "stream.rs",
        "ready.rs",
        "trial.rs",
        "driver.rs",
        "campaign.rs",
        "campaign",
        "source-audit.json",
        "build-pins.json",
        "build-plan.json",
        "environment.json",
        "caller-bin",
        "identities.json",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    pins.insert(CASES.into(), hash(CASES));
    for c in read(CASES)
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| ["1x2", "2x2"].contains(&c["shape"].as_str().unwrap()))
    {
        for f in ["image", "request"] {
            let p = c[f].as_str().unwrap();
            pins.insert(p.into(), hash(p));
        }
    }
    let oracle =
        "/home/ubuntu/src/zopro-perf-work/coalesce-query176/consumers/oracle-1x2-A/stdout.log";
    pins.insert(oracle.into(), hash(oracle));
    pins
}
fn prepare_preflight() {
    assert!(!r("preflight-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let tests = fs::read_to_string(r("native/core-tests/stdout.log")).unwrap();
    assert!(tests
        .contains("test result: ok. 130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    let build = fs::read_to_string(r("native/driver-build/stderr.log")).unwrap();
    let line = build
        .lines()
        .find(|l| l.contains("Running") && l.contains("--crate-name zopro_solver_a_stream"))
        .unwrap();
    for s in [
        "-C opt-level=3",
        "-C lto=fat",
        "-C codegen-units=1",
        "-C target-cpu=native",
        "--cfg zopro_early_a",
    ] {
        assert!(line.contains(s), "{s}");
    }
    let bin = r("target/release/zopro-solver-a-stream");
    fs::copy(&bin, r("caller-bin")).unwrap();
    save(
        r("identities.json"),
        &json!({"binary_sha256":hash(&bin),"actual_driver_rustc":line,"original_solver_and_msm_exact":true,"component_not_SDK":true,"new_proofs":0}),
    );
    let mut steps = vec![];
    for w in [1, 4, 16, 32] {
        let mut s = step(format!("primary-w{w}"), args("1x2", w, "preflight", 1, 0));
        s["environment"] = json!({"ZOPRO_CPU_UNINIT_SOLVER":"1"});
        steps.push(s);
    }
    steps.push(step("unsupported-2x2", args("2x2", 4, "decline", 1, 0)));
    save(r("preflight-pins.json"), &json!(pins()));
    save(
        r("preflight-plan.json"),
        &json!({"version":1,"source_pins":r("preflight-pins.json"),"environment":read(r("environment.json")),"steps":steps}),
    );
    println!("Prepared5functional callers; no timing protocol/capture yet");
}
fn prepare_timing() {
    assert!(!r("capture-plan.json").exists());
    assert_eq!(read(r("preflight/result.json"))["status"], "complete");
    for w in [1, 4, 16, 32] {
        let v = caller("preflight", &format!("primary-w{w}"));
        assert_eq!(v["all_results_match_independent_A_MSM"], true);
    }
    assert_eq!(
        caller("preflight", "unsupported-2x2")["preparation_declined"],
        true
    );
    fs::write(r("timing-protocol.md"),"# Solver→A necessary component screen\n\nFrozen before any candidate timing:15 fixed process blocks at16/32 workers,31 interleaved OFF/ON pairs per process, alternating pair order by block and iteration. Rotate worker-context order each block. Same executable/key, original byte-exact solver and A path for OFF. Six primes and two checked postcalls per process. No frozen accepted solver→A binary exists; this is not an accepted full-SDK calibration. Fresh proof-local solver/query buffers on every call; input witness construction and readiness preparation outside clock. Includes admission system calls, copies, repeated canonicalization, grouping cancellation, actor scheduling and whole solver→A completion. Excludes quotient FFT, K/H/G2 and proof assembly/verification.\n\nUse median31 raw calls per arm in each block. Pair block ON/OFF ratios; sorted ranks4–12/15 give nominal96.484375% marginal median intervals conditional on exchangeable blocks, not simultaneous/familywise coverage. Advance only if primary32 upper bound<1 with>=10/15 wins, and16 upper bound<=1.02. No minimum gain floor. Keep every block; no exclusions/extensions/rerolls. Passing is necessary, never sufficient for adoption; full SDK/all-eight/merge54/resources required later.\n\nAll30latency processes finish before four4096-call warm PMU and four4096-call16M-user-cycle profiles, one per16/32×OFF/ON. Profiles/counters are explanatory, not proof critical-path fractions or independent latency estimates. No owned compiler/helper/hash/archive/second workload overlaps recording.\n").unwrap();
    let mut steps = vec![];
    for b in 0..15 {
        let ws = if b % 2 == 0 { [16, 32] } else { [32, 16] };
        for w in ws {
            steps.push(step(
                format!("latency-b{b:02}-w{w}"),
                args("1x2", w, "mixed", 31, b % 2),
            ));
        }
    }
    for d in ["stat", "perf"] {
        fs::create_dir(r(d)).unwrap();
    }
    for f in ["ctl", "ack"] {
        assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());
    }
    for profile in [false, true] {
        for w in [16, 32] {
            for mode in ["off", "on"] {
                let name = format!("{}-w{w}-{mode}", if profile { "cycles" } else { "stat" });
                let mut argv = vec![
                    "sudo".into(),
                    "-n".into(),
                    "env".into(),
                    format!("ZOPRO_BENCH_PERF_CTL={R}/ctl"),
                    format!("ZOPRO_BENCH_PERF_ACK={R}/ack"),
                    "perf".into(),
                ];
                if profile {
                    argv.extend([
                        "record".into(),
                        "--no-buildid-cache".into(),
                        "--sample-cpu".into(),
                        "--timestamp".into(),
                        "-P".into(),
                        "-e".into(),
                        "cycles:u".into(),
                        "-c".into(),
                        "16000000".into(),
                        "-m".into(),
                        "512".into(),
                        "-o".into(),
                        format!("{R}/perf/{name}.data"),
                    ]);
                } else {
                    argv.extend([
                        "stat".into(),
                        "--no-big-num".into(),
                        "-x".into(),
                        ";".into(),
                        "-e".into(),
                        "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                        "-o".into(),
                        format!("{R}/stat/{name}.csv"),
                    ]);
                }
                argv.extend([
                    "--delay=-1".into(),
                    format!("--control=fifo:{R}/ctl,{R}/ack"),
                    "--".into(),
                ]);
                argv.extend(args("1x2", w, mode, 4096, 0));
                steps.push(step(name, argv));
            }
        }
    }
    for w in [16, 32] {
        for mode in ["off", "on"] {
            let name = format!("cycles-w{w}-{mode}");
            let data = format!("{R}/perf/{name}.data");
            steps.push(step(
                format!("own-{name}"),
                vec![
                    "sudo".into(),
                    "-n".into(),
                    format!("{R}/campaign"),
                    "own".into(),
                    data.clone(),
                ],
            ));
            for metadata in [false, true] {
                let mut argv = vec![
                    "perf".into(),
                    "script".into(),
                    "--header".into(),
                    "--ns".into(),
                    "-i".into(),
                    data.clone(),
                    "-F".into(),
                    "pid,tid,cpu,time,period,ip,sym".into(),
                ];
                if metadata {
                    argv.extend([
                        "--show-mmap-events".into(),
                        "--show-task-events".into(),
                        "--show-lost-events".into(),
                    ]);
                }
                steps.push(step(
                    format!("{}-{name}", if metadata { "metadata" } else { "decode" }),
                    argv,
                ));
            }
            steps.push(step(
                format!("events-{name}"),
                vec![
                    "perf".into(),
                    "evlist".into(),
                    "-v".into(),
                    "-i".into(),
                    data.clone(),
                ],
            ));
            steps.push(step(
                format!("loss-{name}"),
                vec![format!("{R}/campaign"), "loss".into(), data],
            ));
        }
    }
    let mut pins = pins();
    walk(&r("preflight"), &mut pins);
    for f in [
        "preflight-plan.json",
        "preflight-pins.json",
        "timing-protocol.md",
    ] {
        let p = r(f);
        pins.insert(p.display().to_string(), hash(p));
    }
    save(r("capture-pins.json"), &json!(pins));
    save(
        r("capture-plan.json"),
        &json!({"version":1,"source_pins":r("capture-pins.json"),"environment":read(r("environment.json")),"steps":steps}),
    );
    println!("Frozen15blocks×16/32×31pairedcalls; all latency before8 warm PMU/profile callers; no measurements yet");
}
fn own(data: &str) {
    assert!(Path::new(data).starts_with(r("perf")) && data.ends_with(".data"));
    let before = hash(data);
    let m = fs::metadata(r("campaign")).unwrap();
    assert!(Command::new("chown")
        .arg(format!("{}:{}", m.uid(), m.gid()))
        .arg(data)
        .status()
        .unwrap()
        .success());
    assert_eq!(before, hash(data));
    println!(
        "{}",
        json!({"data":data,"sha256_before":before,"sha256_after":hash(data)})
    );
}
fn loss(data: &str) {
    let mut c = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lost = vec![];
    let mut lines = 0;
    for l in BufReader::new(c.stdout.take().unwrap()).lines() {
        let l = l.unwrap();
        lines += 1;
        if l.contains("PERF_RECORD_LOST") {
            lost.push(l);
        }
    }
    assert!(c.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":lines,"lost":lost,"zero_lost":lost.is_empty()})
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("preflight") => prepare_preflight(),
        Some("timing") => prepare_timing(),
        Some("own") => own(&std::env::args().nth(2).unwrap()),
        Some("loss") => loss(&std::env::args().nth(2).unwrap()),
        _ => panic!("preflight/timing/own/loss"),
    }
}
