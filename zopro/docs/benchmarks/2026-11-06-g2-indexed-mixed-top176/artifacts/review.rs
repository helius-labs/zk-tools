use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-indexed-mixed-top176";
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
fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.));
}
fn center(v: &mut [f64]) -> f64 {
    *v.select_nth_unstable_by(v.len() / 2, f64::total_cmp).1
}
fn collect(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            collect(&p, out)
        } else {
            out.push(p);
        }
    }
}
fn main() {
    assert!(!r("review.json").exists());
    let a = load(r("analysis.json"));
    let jobs = load(r("jobs.json"));
    let sourceaudit = load(r("source-audit.json"));
    let manifest=load(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut files = vec![];
    collect(&r("source"), &mut files);
    assert_eq!(files.len(), 178);
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let original = fs::read(Path::new(Z).join(p)).unwrap();
        assert_eq!(digest(Path::new(Z).join(p)), h.as_str().unwrap());
        let copy = fs::read(r("source").join(p)).unwrap();
        assert!(copy.starts_with(&original));
        if copy == original {
            exact += 1;
        }
    }
    assert_eq!(exact, 171);
    for (name, c) in sourceaudit["clones"].as_object().unwrap() {
        let mut restored = c["candidate"].as_str().unwrap().to_owned();
        for change in c["edits"].as_array().unwrap().iter().rev() {
            restored =
                restored.replacen(change[1].as_str().unwrap(), change[0].as_str().unwrap(), 1);
        }
        assert_eq!(restored, c["original"]);
        let (old, new) = match name.as_str() {
            "query" => (
                "crates/zopro-core/src/msm_affine.rs",
                "crates/zopro-core/src/msm_affine.rs",
            ),
            "borrowed_feed" | "borrowed_window" => (
                "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
                "crates/zopro-core/src/msm_affine/g2_borrowed/indexed_mixed.rs",
            ),
            _ => (
                "crates/zopro-core/src/msm_affine/g2_phi.rs",
                "crates/zopro-core/src/msm_affine/g2_phi/indexed_mixed.rs",
            ),
        };
        assert!(fs::read_to_string(Path::new(Z).join(old))
            .unwrap()
            .contains(&restored));
        assert!(fs::read_to_string(r("source").join(new))
            .unwrap()
            .contains(c["candidate"].as_str().unwrap()));
    }
    let mut pins = 0;
    for f in ["algebra-pins.json", "build-pins.json", "capture-pins.json"] {
        for (p, h) in load(r(f)).as_object().unwrap() {
            assert_eq!(digest(p), h.as_str().unwrap());
            pins += 1;
        }
    }
    let mut commands = 0;
    for (plan, out) in [
        ("algebra-plan.json", "algebra-check"),
        ("build-plan.json", "native"),
        ("capture-plan.json", "runs"),
    ] {
        for s in load(r(plan))["steps"].as_array().unwrap() {
            let base = r(out).join(s["name"].as_str().unwrap());
            let e = load(base.join("exit.json"));
            assert_eq!(e["returncode"], 0);
            assert_eq!(e["success"], true);
            for f in ["stdout", "stderr"] {
                assert_eq!(
                    digest(base.join(format!("{f}.log"))),
                    e[format!("{f}_sha256")]
                );
            }
            let cmd = load(base.join("command.json"));
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                s["argv"].as_array().unwrap()
            );
            let mut expected_env = s.get("environment").cloned().unwrap_or(json!({}));
            // process::run records only config::managed variables. RUSTC is
            // passed to the process but omitted from this receipt projection.
            // The actual compiler argv is independently checked at freeze.
            expected_env.as_object_mut().unwrap().remove("RUSTC");
            assert_eq!(cmd["environment"], expected_env);
            commands += 1;
        }
    }
    let native = fs::read_to_string(r("native/core-tests/stdout.log")).unwrap();
    assert!(native.contains("128 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    let new = native
        .lines()
        .filter(|l| l.starts_with("test ") && l.contains("indexed_mixed_tests::"))
        .collect::<Vec<_>>();
    assert_eq!(new.len(), 3);
    assert!(new.iter().all(|s| s.ends_with(" ... ok")));
    let mut callers = BTreeMap::new();
    let mut allcalls = 0;
    let mut reusedcalls = 0;
    for j in jobs.as_array().unwrap() {
        let base = j["reused_from"]
            .as_str()
            .map(PathBuf::from)
            .unwrap_or_else(|| r("runs").join(j["name"].as_str().unwrap()));
        let exit = load(base.join("exit.json"));
        assert_eq!(exit["success"], true);
        for f in ["stdout", "stderr"] {
            assert_eq!(
                digest(base.join(format!("{f}.log"))),
                exit[format!("{f}_sha256")]
            );
        }
        let text = fs::read_to_string(base.join("stdout.log")).unwrap();
        let lines = text
            .lines()
            .filter_map(|l| l.strip_prefix("CALLER "))
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        for k in [
            "every_output_matches_independent_full_MSM",
            "all_timed_outputs_checked",
            "point_inputs_unchanged",
            "normalization_inside_timer",
        ] {
            assert_eq!(v[k], true);
        }
        assert_eq!(v["query"], j["actual_query"]);
        assert_eq!(v["rounds"], j["rounds"]);
        assert_eq!(v["workers"], j["case"]["workers"]);
        assert_eq!(
            v["metadata"]["route"],
            if j["variant"] == "ON" && j["case"]["query"] == "B" {
                328
            } else {
                0
            }
        );
        let n = j["rounds"].as_u64().unwrap() + 4;
        allcalls += n;
        if j.get("reused_from").is_some() {
            reusedcalls += n;
            let cmd = load(base.join("command.json"));
            assert_eq!(cmd["environment"], j["command"]["environment"]);
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                j["command"]["argv"].as_array().unwrap()
            );
        }
        callers.insert(j["name"].as_str().unwrap().to_owned(), v);
    }
    let mut failures = vec![];
    let mut ratios = BTreeMap::<(String, String), Vec<f64>>::new();
    for row in a["rows"].as_array().unwrap() {
        let case = row["case"].as_str().unwrap();
        let mut times = BTreeMap::<String, Vec<f64>>::new();
        for arm in ["FROZEN", "OFF", "ON"] {
            let mut v = vec![];
            let mut rss = 0f64;
            let mut cpu = vec![];
            for b in 0..15 {
                let c = &callers[&format!("latency-b{b:02}-{case}-{arm}")];
                let mut samples = c["samples_ns"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_u64().unwrap() as f64)
                    .collect::<Vec<_>>();
                assert_eq!(samples.len(), 17);
                v.push(center(&mut samples));
                rss = rss.max(c["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
                cpu.push(
                    (c["usage"]["user_cpu_ms"].as_f64().unwrap()
                        + c["usage"]["system_cpu_ms"].as_f64().unwrap())
                        / 17.,
                );
            }
            let mut v2 = v.clone();
            near(
                center(&mut v2) / 1e6,
                row["median_ms"][arm].as_f64().unwrap(),
            );
            near(rss, row["max_process_RSS_MiB"][arm].as_f64().unwrap());
            near(
                center(&mut cpu),
                row["median_CPU_ms_per_query"][arm].as_f64().unwrap(),
            );
            times.insert(arm.into(), v);
        }
        for (num, den) in [("OFF", "FROZEN"), ("ON", "OFF"), ("ON", "FROZEN")] {
            let name = format!("{num}/{den}");
            let v = times[num]
                .iter()
                .zip(&times[den])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let wins = v.iter().filter(|&&r| r < 1.).count();
            let mut sorted = v.clone();
            sorted.sort_by(f64::total_cmp);
            let s = &row["comparisons"][&name];
            near(sorted[7], s["median"].as_f64().unwrap());
            near(sorted[3], s["low"].as_f64().unwrap());
            near(sorted[11], s["high"].as_f64().unwrap());
            assert_eq!(wins, s["wins"].as_u64().unwrap() as usize);
            let pass = if num == "OFF" {
                sorted[3] >= 1. / 1.03 && sorted[11] <= 1.03
            } else {
                sorted[11] <= 1.03
            };
            if !pass {
                failures.push(format!(
                    "{case}/{num}/{den}/{}",
                    if num == "OFF" {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if num == "ON" && case == "1x2-B-w32" && (sorted[11] >= 1. || wins < 10) {
                failures.push(format!("{case}/{num}/{den}/primary-gain"));
            }
            ratios.insert((case.into(), name), v);
        }
    }
    for den in ["OFF", "FROZEN"] {
        let name = format!("ON/{den}");
        let v = (0..15)
            .map(|b| {
                (ratios[&("1x2-B-w32".into(), name.clone())][b]
                    * ratios[&("2x2-B-w32".into(), name.clone())][b])
                    .sqrt()
            })
            .collect::<Vec<_>>();
        let wins = v.iter().filter(|&&r| r < 1.).count();
        let mut sorted = v;
        sorted.sort_by(f64::total_cmp);
        let s = &a["aggregates"][&name];
        near(sorted[7], s["median"].as_f64().unwrap());
        near(sorted[3], s["low"].as_f64().unwrap());
        near(sorted[11], s["high"].as_f64().unwrap());
        assert_eq!(wins, s["wins"].as_u64().unwrap() as usize);
        if sorted[11] >= 1. || wins < 10 {
            failures.push(format!("B32-two-shape/ON/{den}/gain"));
        }
    }
    let mut samples_all = 0;
    for j in jobs
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["panel"] == "stat" || j["panel"] == "cycles")
    {
        let name = j["name"].as_str().unwrap();
        if j["panel"] == "stat" {
            let mut seen = BTreeSet::new();
            for line in fs::read_to_string(j["data"].as_str().unwrap())
                .unwrap()
                .lines()
            {
                let f = line.split(';').collect::<Vec<_>>();
                if f.len() > 4 && a["counters"][name].get(f[2].trim()).is_some() {
                    let key = f[2].trim();
                    let count = f[0].trim().parse::<u64>().unwrap();
                    assert_eq!(count, a["counters"][name][key]["count"]);
                    near(
                        count as f64 / 200.,
                        a["counters"][name][key]["per_query"].as_f64().unwrap(),
                    );
                    assert!(f[4].trim().parse::<f64>().unwrap() >= 99.9);
                    seen.insert(key.to_owned());
                }
            }
            assert_eq!(seen.len(), 4);
            continue;
        }
        let v = &callers[name];
        let pid = v["pid"].as_u64().unwrap();
        let threads = v["worker_rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| (r["tid"].as_u64().unwrap(), r["cpu"].as_u64().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut count = 0;
        let mut unknown = 0;
        for line in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
            .unwrap()
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        {
            let f = line.split_whitespace().collect::<Vec<_>>();
            let ids = f[0]
                .split('/')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(ids[0], pid);
            let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
            assert!(cpu < 32);
            if ids[1] != pid {
                assert_eq!(threads[&ids[1]], cpu);
            }
            assert_eq!(f[3], "16000000");
            if f[5] == "[unknown]" {
                unknown += 1;
            }
            count += 1;
        }
        assert_eq!(count, a["profiles"][name]["samples"].as_u64().unwrap());
        assert_eq!(unknown, a["profiles"][name]["unknown"].as_u64().unwrap());
        assert!(count >= 1000 && unknown as f64 / count as f64 <= 0.05);
        assert_eq!(
            digest(j["data"].as_str().unwrap()),
            a["profiles"][name]["raw_sha256"]
        );
        let mut child = Command::new("perf")
            .args(["script", "-D", "-i", j["data"].as_str().unwrap()])
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
        samples_all += count;
    }
    assert_eq!(commands, 377);
    assert_eq!(pins, a["pin_occurrences_checked"]);
    assert_eq!(a["failed_checks"], json!(failures));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(allcalls, 14112);
    assert_eq!(reusedcalls, 35);
    assert_eq!(samples_all, 135242);
    let result = json!({"passed":true,"original_production176_unchanged":true,"snapshot_files":178,"originals_exact":171,"all_originals_prefix_recovered":true,"six_clones_inverse_and_actual_bodies_checked":true,"native_tests":128,"new_native_tests":3,"prior_algebra_tests":4,"pin_occurrences_checked":pins,"successful_workflow_commands":commands,"new_capture_commands":372,"reused_preflight_receipts":7,"reused_environment_and_argv_exact":true,"all_actual_command_argv_checked":true,"caller_rows":348,"complete_query_calls":allcalls,"new_query_calls":allcalls-reusedcalls,"all_timing_CPU_RSS_gates_reconstructed":true,"PMU_counts_reparsed":true,"profile_samples_reparsed":samples_all,"raw_loss_checks":6,"zero_lost":true,"component_gate_passed":false,"failed_checks":failures,"adopted":false,"new_proofs":0,"canonical_proof_ledger":30846,"goal_achieved":false});
    fs::write(
        r("review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("{result}");
}
