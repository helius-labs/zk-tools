use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/solver-a-stream176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, out)
        } else {
            out.push(p);
        }
    }
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.));
}
fn middle(v: &mut [f64]) -> f64 {
    let m = v.len() / 2;
    *v.select_nth_unstable_by(m, f64::total_cmp).1
}
fn output(out: &str, n: &str) -> Value {
    let s = fs::read_to_string(r(format!("{out}/{n}/stdout.log"))).unwrap();
    let lines = s
        .lines()
        .filter_map(|l| l.strip_prefix("CALLER "))
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);
    serde_json::from_str(lines[0]).unwrap()
}
fn main() {
    assert!(!r("review.json").exists());
    let a = load(r("analysis.json"));
    let manifest=load(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut unchanged = 0;
    let mut total = vec![];
    walk(&r("source"), &mut total);
    assert_eq!(total.len(), 179);
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let original = fs::read(Path::new(Z).join(p)).unwrap();
        assert_eq!(digest(Path::new(Z).join(p)), h.as_str().unwrap());
        let copy = fs::read(r("source").join(p)).unwrap();
        assert!(copy.starts_with(&original), "{p}");
        if copy == original {
            unchanged += 1;
        }
    }
    assert_eq!(unchanged, 171);
    let audit = load(r("source-audit.json"));
    let solver =
        fs::read_to_string(r("source/crates/zopro-core/src/solver/early_feed.rs")).unwrap();
    let mut clone = solver
        .split_once("fn solve_with_a_feed<")
        .unwrap()
        .1
        .trim_end()
        .strip_suffix('}')
        .unwrap()
        .trim_end()
        .to_string();
    clone.insert_str(0, "fn solve_with_a_feed<");
    for pair in audit["solver_clone_edits"].as_array().unwrap().iter().rev() {
        clone = clone.replacen(pair[1].as_str().unwrap(), pair[0].as_str().unwrap(), 1);
    }
    let original =
        fs::read_to_string(Path::new(Z).join("crates/zopro-core/src/solver.rs")).unwrap();
    assert!(original.contains(&clone));
    assert_eq!(clone, fs::read_to_string(r("original-solver.rs")).unwrap());
    let mut pin_count = 0;
    for f in [
        "build-pins.json",
        "preflight-pins.json",
        "capture-pins.json",
        "perf-fixed-pins.json",
    ] {
        for (p, h) in load(r(f)).as_object().unwrap() {
            assert_eq!(digest(p), h.as_str().unwrap());
            pin_count += 1;
        }
    }
    let mut commands = 0;
    for (plan, out) in [
        ("build-plan.json", "native"),
        ("preflight-plan.json", "preflight"),
        ("capture-plan.json", "capture"),
        ("perf-fixed-plan.json", "perf-fixed"),
    ] {
        for s in load(r(plan))["steps"].as_array().unwrap() {
            let n = s["name"].as_str().unwrap();
            if out == "capture" && !n.starts_with("latency-") {
                continue;
            }
            let base = r(format!("{out}/{n}"));
            let e = load(base.join("exit.json"));
            assert_eq!(e["returncode"], 0);
            assert_eq!(e["success"], true);
            for f in ["stdout", "stderr"] {
                assert_eq!(
                    digest(base.join(format!("{f}.log"))),
                    e[format!("{f}_sha256")]
                );
            }
            commands += 1;
        }
    }
    let text = fs::read_to_string(r("native/core-tests/stdout.log")).unwrap();
    let newtests = text
        .lines()
        .filter(|l| {
            l.starts_with("test ")
                && (l.contains("early_stream::tests::") || l.contains("early_feed::tests::"))
        })
        .collect::<Vec<_>>();
    assert_eq!(newtests.len(), 5);
    assert!(newtests.iter().all(|l| l.ends_with(" ... ok")));
    assert!(text.contains("130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    for w in [1, 4, 16, 32] {
        let v = output("preflight", &format!("primary-w{w}"));
        for f in [
            "all_results_match_independent_A_MSM",
            "bad_witness_rejected",
            "admission_fallback_checked",
            "runtime_fallback_checked",
        ] {
            assert_eq!(v[f], true);
        }
        let m = v["outputs"].as_array().unwrap();
        assert_eq!(m.len(), 4);
        for i in [1, 3] {
            assert_eq!(m[i]["streamed"], true);
            assert_eq!(m[i]["published_chunks"], 21);
            assert_eq!(m[i]["early_chunks"], 19);
        }
    }
    assert_eq!(
        output("preflight", "unsupported-2x2")["preparation_declined"],
        true
    );
    let mut failures = vec![];
    for w in [16, 32] {
        let row = a["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["workers"] == w)
            .unwrap();
        let mut arm_times = [vec![], vec![]];
        let mut quotients = vec![];
        for b in 0..15 {
            let v = output("capture", &format!("latency-b{b:02}-w{w}"));
            assert_eq!(v["all_results_match_independent_A_MSM"], true);
            let mut arm = [vec![], vec![]];
            let samples = v["samples"].as_array().unwrap();
            assert_eq!(samples.len(), 62);
            for (i, s) in samples.iter().enumerate() {
                assert_eq!(s["on"], ((i / 2 + b % 2 + i % 2) % 2) == 1);
                let enabled = s["on"].as_bool().unwrap();
                arm[usize::from(enabled)].push(s["ns"].as_u64().unwrap() as f64);
                let meta = &v["metadata"][i];
                assert_eq!(meta["streamed"], enabled);
                assert_eq!(meta["admission_fallback"], false);
                assert_eq!(meta["grouped_fallback"], false);
                if enabled {
                    assert_eq!(meta["published_chunks"], 21);
                    assert_eq!(meta["early_values"], 18958);
                }
            }
            let off = middle(&mut arm[0]);
            let on = middle(&mut arm[1]);
            close(off, row["block_OFF_ns"][b].as_f64().unwrap());
            close(on, row["block_ON_ns"][b].as_f64().unwrap());
            arm_times[0].push(off);
            arm_times[1].push(on);
            quotients.push(on / off);
        }
        close(
            middle(&mut arm_times[0]) / 1e6,
            row["OFF_median_ms"].as_f64().unwrap(),
        );
        close(
            middle(&mut arm_times[1]) / 1e6,
            row["ON_median_ms"].as_f64().unwrap(),
        );
        let wins = quotients.iter().filter(|&&v| v < 1.).count();
        let q = row["paired_ON_OFF"].as_object().unwrap();
        assert_eq!(wins, q["wins"].as_u64().unwrap() as usize);
        let mut q2 = quotients.clone();
        close(middle(&mut q2), q["median"].as_f64().unwrap());
        quotients.sort_by(f64::total_cmp);
        close(quotients[3], q["low"].as_f64().unwrap());
        close(quotients[11], q["high"].as_f64().unwrap());
        let passed = if w == 32 {
            quotients[11] < 1. && wins >= 10
        } else {
            quotients[11] <= 1.02
        };
        assert_eq!(row["passed"], passed);
        if !passed {
            failures.push(format!(
                "w{w}/{}",
                if w == 32 { "gain" } else { "nonregression" }
            ));
        }
    }
    let mut count_all = 0u64;
    for w in [16, 32] {
        for mode in ["off", "on"] {
            let stat = format!("stat-w{w}-{mode}");
            let mut events = BTreeSet::new();
            for l in fs::read_to_string(r(format!("stat/{stat}.csv")))
                .unwrap()
                .lines()
            {
                let f = l.split(';').collect::<Vec<_>>();
                if f.len() > 4 && a["counters"][&stat].get(f[2].trim()).is_some() {
                    let e = f[2].trim();
                    let count = f[0].trim().parse::<u64>().unwrap();
                    assert_eq!(a["counters"][&stat][e]["count"], count);
                    close(
                        count as f64 / 4096.,
                        a["counters"][&stat][e]["per_call"].as_f64().unwrap(),
                    );
                    assert!(f[4].trim().parse::<f64>().unwrap() >= 99.9);
                    events.insert(e.to_string());
                }
            }
            assert_eq!(events.len(), 4);
            let name = format!("cycles-w{w}-{mode}");
            let v = output("perf-fixed", &name);
            let pid = v["pid"].as_u64().unwrap();
            let tids = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|r| r["tid"].as_u64().unwrap())
                .chain([pid])
                .collect::<BTreeSet<_>>();
            let mut count = 0;
            let mut unknown = 0;
            for l in fs::read_to_string(r(format!("perf-fixed/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let p = f[0]
                    .split('/')
                    .map(|v| v.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(p[0], pid);
                assert!(tids.contains(&p[1]));
                assert!(f[1].trim_matches(['[', ']']).parse::<usize>().unwrap() < w);
                assert_eq!(f[3], "16000000");
                if f[5] == "[unknown]" {
                    unknown += 1;
                }
                count += 1;
            }
            assert_eq!(a["profiles"][&name]["samples"], count);
            assert_eq!(a["profiles"][&name]["unknown"], unknown);
            assert!(count >= 400 && unknown as f64 / count as f64 <= 0.05);
            let data = r(format!("perf/{name}.data"));
            assert_eq!(digest(&data), a["profiles"][&name]["raw_sha256"]);
            let mut child = Command::new("perf")
                .args(["script", "-D", "-i"])
                .arg(&data)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let mut lost = 0;
            for l in BufReader::new(child.stdout.take().unwrap()).lines() {
                if l.unwrap().contains("PERF_RECORD_LOST") {
                    lost += 1;
                }
            }
            assert!(child.wait().unwrap().success());
            assert_eq!(lost, 0);
            count_all += count;
        }
    }
    assert_eq!(count_all, a["total_profile_samples"].as_u64().unwrap());
    assert_eq!(a["failed_checks"], json!(failures));
    assert_eq!(a["necessary_component_gate_passed"], false);
    assert_eq!(commands, 66);
    assert_eq!(commands, a["commands_checked"]);
    assert_eq!(pin_count, a["pin_occurrences_checked"]);
    assert_eq!(a["total_solver_A_calls"], 34956);
    let result = json!({"passed":true,"production176_unchanged":true,"snapshot_files":179,"originals_exact":171,"all_originals_restored_by_prefix":true,"clone_recovery_exact":true,"native_tests":130,"new_tests":5,"pin_occurrences":pin_count,"successful_commands":commands,"preserved_failed_profiler_attempts":1,"latency_processes":30,"fixed_blocks":15,"timing_and_gates_reconstructed":true,"PMU_independently_reparsed":true,"profile_samples_reparsed":count_all,"raw_profile_loss_checks":4,"zero_lost":true,"total_solver_A_calls":34956,"new_proofs":0,"canonical_proof_ledger":30846,"component_gate_passed":false,"failed_checks":failures,"adopted":false,"goal_achieved":false});
    fs::write(
        r("review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("{result}");
}
