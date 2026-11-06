use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-callers176";
const N: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.),
        "{a}!={b}"
    );
}
fn check_stats(v: &[f64], actual: &Value) {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    assert_eq!(s.len(), 15);
    for (k, i) in [("low", 3), ("median", 7), ("high", 11)] {
        close(s[i], actual[k].as_f64().unwrap());
    }
    assert_eq!(
        v.iter().filter(|&&x| x < 1.).count(),
        actual["wins"].as_u64().unwrap() as usize
    );
    for (a, b) in v.iter().zip(actual["ratios"].as_array().unwrap()) {
        close(*a, b.as_f64().unwrap());
    }
}
fn main() {
    assert!(!Path::new(R).join("review.json").exists());
    let analysis = read(format!("{R}/analysis.json"));
    let plan = read(format!("{R}/plan.json"));
    let decl = read(format!("{R}/declaration.json"));
    assert_eq!(decl["steps"], 1107);
    assert_eq!(decl["blocks"], 15);
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let native = read(format!("{N}/review.json"));
    assert_eq!(native["review_passed"], true);
    assert_eq!(
        hash(format!("{N}/libs/libzopro_core.rlib")),
        native["binaries_sha256"]["libs/libzopro_core.rlib"]
    );
    let old_manifest = read(format!("{N}/source-manifest.json"));
    let off = read(format!("{R}/OFF-source-manifest.json"));
    let mut exact = 0;
    for (p, h) in old_manifest["candidate"].as_object().unwrap() {
        let a = fs::read(format!("{N}/source/{p}")).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&a)), h.as_str().unwrap());
        let b = fs::read(format!("{R}/OFF-source/{p}")).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&b)), off["source_files"][p]);
        if p == "crates/zopro-core/src/msm/h_only_probe.rs" {
            let a = String::from_utf8(a).unwrap();
            assert_eq!(a.matches("if SELECTED\n").count(), 1);
            assert_eq!(a.replace("if SELECTED\n", "if false\n").as_bytes(), b);
        } else {
            assert_eq!(a, b);
            exact += 1;
        }
    }
    assert_eq!(exact, 177);
    let identities = read(format!("{R}/identities.json"));
    for arm in ["frozen", "OFF", "ON"] {
        let id = &identities[arm];
        assert_eq!(hash(id["binary"].as_str().unwrap()), id["binary_sha256"]);
        assert_eq!(hash(id["core"].as_str().unwrap()), id["core_sha256"]);
    }
    assert_eq!(
        identities["frozen"]["binary_sha256"],
        identities["ON"]["binary_sha256"]
    );
    assert_eq!(
        identities["ON"]["core_sha256"],
        native["binaries_sha256"]["libs/libzopro_core.rlib"]
    );
    let build = read(format!("{R}/build-plan.json"));
    for s in build["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let base = format!("{R}/build-runs/{n}");
        assert_eq!(read(format!("{base}/exit.json"))["returncode"], 0);
        let actual = read(format!("{base}/command.json"));
        assert_eq!(
            &actual["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap().as_slice()
        );
        let a: Vec<_> = s["argv"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap())
            .collect();
        for flag in ["-O", "target-cpu=native", "codegen-units=1"] {
            assert!(a.contains(&flag));
        }
        if n.starts_with("driver-") {
            assert!(a.contains(&"lto=fat"));
        }
    }
    let mut counts = BTreeMap::<String, usize>::new();
    let mut callers = BTreeMap::new();
    let mut measured = 0u64;
    let mut primed = 0u64;
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let dir = format!("{R}/runs/{n}");
        let cmd = read(format!("{dir}/command.json"));
        let exit = read(format!("{dir}/exit.json"));
        assert_eq!(
            &cmd["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap().as_slice()
        );
        assert_eq!(exit["returncode"], 0);
        assert_eq!(exit["success"], true);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{dir}/{stream}.log")),
                exit[format!("{stream}_sha256")]
            );
        }
        let category = if n == "preflight-check" {
            "check"
        } else {
            n.split('-').next().unwrap()
        };
        *counts.entry(category.into()).or_default() += 1;
        if ["preflight", "latency", "stat", "cycles", "legacy"].contains(&category) {
            let text = fs::read_to_string(format!("{dir}/stdout.log")).unwrap();
            let lines: Vec<_> = text
                .lines()
                .filter_map(|s| s.strip_prefix("RESULT "))
                .collect();
            assert_eq!(lines.len(), 1);
            let v: Value = serde_json::from_str(lines[0]).unwrap();
            if category != "legacy" {
                assert_eq!(v["complete_projective_oracle"], true);
                assert_eq!(v["SDK_requests"], 0);
                assert_eq!(v["proofs"], 0);
                let w = v["workers"].as_u64().unwrap();
                let rounds = v["rounds_per_worker"].as_u64().unwrap();
                measured += w * rounds;
                primed += w * 24;
                if v["fresh_buffers"] == false {
                    assert_eq!(v["timed_allocations"], 0);
                }
                let mut tids = BTreeSet::new();
                for row in v["worker_rows"].as_array().unwrap() {
                    assert!(tids.insert(row["tid"].as_u64().unwrap()));
                    assert_eq!(
                        row["cpu"],
                        if w == 1 {
                            2
                        } else {
                            row["id"].as_u64().unwrap()
                        }
                    );
                    assert_eq!(row["output_sha256"], v["output_sha256"]);
                }
                assert_eq!(tids.len(), w as usize);
                assert_eq!(v, analysis["caller_receipts"][n]);
            }
            callers.insert(n.to_owned(), v);
        }
    }
    for (k, n) in [
        ("preflight", 63),
        ("legacy", 14),
        ("check", 1),
        ("latency", 945),
        ("stat", 12),
        ("cycles", 12),
        ("own", 12),
        ("decode", 12),
        ("metadata", 12),
        ("events", 12),
        ("loss", 12),
    ] {
        assert_eq!(counts[k], n);
    }
    let mut failed = Vec::new();
    let mut ratios = BTreeMap::new();
    let mut checks = 0;
    for row in analysis["rows"].as_array().unwrap() {
        let context = row["context"].as_str().unwrap();
        let mut vectors = BTreeMap::new();
        for arm in ["frozen", "OFF", "ON"] {
            let mut times = Vec::new();
            let mut cpus = Vec::new();
            let mut faults = Vec::new();
            let mut allocs = Vec::new();
            let mut maxrss = 0f64;
            let pre = &callers[&format!("preflight-{context}-{arm}")];
            for b in 0..15 {
                let v = &callers[&format!("latency-{context}-b{b:02}-{arm}")];
                assert_eq!(v["case"], row["case"]);
                assert_eq!(v["workers"], row["workers"]);
                assert_eq!(v["rounds_per_worker"], row["rounds_per_worker"]);
                assert_eq!(v["output_sha256"], pre["output_sha256"]);
                let rounds = v["rounds_per_worker"].as_f64().unwrap();
                let ns = v["elapsed_ns"].as_f64().unwrap();
                close(v["ns_per_wave"].as_f64().unwrap(), ns / rounds);
                times.push(ns / rounds);
                cpus.push(
                    (v["process_usage"]["user_cpu_ms"].as_f64().unwrap()
                        + v["process_usage"]["system_cpu_ms"].as_f64().unwrap())
                        / rounds,
                );
                faults.push(v["process_usage"]["minor_faults"].as_f64().unwrap() / rounds);
                allocs.push(v["allocation_calls_per_worker_round"].as_f64().unwrap());
                maxrss = maxrss.max(
                    v["process_usage"]["whole_process_maxrss_kib"]
                        .as_f64()
                        .unwrap()
                        / 1024.,
                );
                for each in v["worker_rows"].as_array().unwrap() {
                    assert_eq!(
                        each["scratch_capacity_bytes"],
                        row["retained_capacity_bytes_first_worker"][arm]
                    );
                }
                assert_eq!(
                    v["output_sha256"],
                    callers[&format!("latency-{context}-b{b:02}-frozen")]["output_sha256"]
                );
            }
            times.sort_by(f64::total_cmp);
            cpus.sort_by(f64::total_cmp);
            faults.sort_by(f64::total_cmp);
            allocs.sort_by(f64::total_cmp);
            close(times[7], row["median_ns_per_wave"][arm].as_f64().unwrap());
            close(cpus[7], row["CPU_ms_per_wave"][arm].as_f64().unwrap());
            close(
                faults[7],
                row["median_minor_faults_per_wave"][arm].as_f64().unwrap(),
            );
            close(
                allocs[7],
                row["median_alloc_calls_per_worker_round"][arm]
                    .as_f64()
                    .unwrap(),
            );
            close(
                maxrss,
                row["max_observed_process_RSS_MiB"][arm].as_f64().unwrap(),
            );
            vectors.insert(
                arm,
                (0..15)
                    .map(|b| {
                        let v = &callers[&format!("latency-{context}-b{b:02}-{arm}")];
                        v["elapsed_ns"].as_f64().unwrap() / v["rounds_per_worker"].as_f64().unwrap()
                    })
                    .collect::<Vec<_>>(),
            );
        }
        for (num, den) in [("OFF", "frozen"), ("ON", "OFF"), ("ON", "frozen")] {
            let v = vectors[num]
                .iter()
                .zip(&vectors[den])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let s = &row["pairs"][format!("{num}/{den}")];
            check_stats(&v, s);
            checks += 1;
            let pass = if num == "OFF" {
                s["low"].as_f64().unwrap() >= 0.97 && s["high"].as_f64().unwrap() <= 1.03
            } else {
                s["high"].as_f64().unwrap() <= 1.01
            };
            if !pass {
                failed.push(format!(
                    "{context}/{num}/{den}/{}",
                    if num == "OFF" {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if num == "ON" && row["main"] == true {
                checks += 1;
                if s["high"].as_f64().unwrap() >= 1. || s["wins"].as_u64().unwrap() < 11 {
                    failed.push(format!("{context}/{num}/{den}/gain"));
                }
            }
            ratios.insert(format!("{context}/{num}/{den}"), v);
        }
    }
    assert_eq!(analysis["rows"].as_array().unwrap().len(), 21);
    for den in ["OFF", "frozen"] {
        let v = (0..15)
            .map(|b| {
                ["tile384", "tile511", "tile2048", "group35973"]
                    .iter()
                    .map(|c| ratios[&format!("{c}-w32/ON/{den}")][b].ln())
                    .sum::<f64>()
                    / 4.
            })
            .map(f64::exp)
            .collect::<Vec<_>>();
        let s = &analysis["main32_aggregates"][den];
        check_stats(&v, s);
        checks += 1;
        if s["high"].as_f64().unwrap() >= 1. || s["wins"].as_u64().unwrap() < 11 {
            failed.push(format!("main32aggregate/ON/{den}/gain"));
        }
    }
    assert_eq!(checks, 89);
    let mut profile_samples = 0u64;
    let syms = read(format!("{R}/profile-symbols.json"));
    for (name, profile) in analysis["profiles"].as_object().unwrap() {
        let pid = profile["PID"].as_u64().unwrap();
        let mut count = 0;
        let mut unknown = 0;
        let mut tids = BTreeMap::<u64, u64>::new();
        for l in fs::read_to_string(format!("{R}/runs/decode-{name}/stdout.log"))
            .unwrap()
            .lines()
            .filter(|s| !s.starts_with('#') && !s.trim().is_empty())
        {
            let f: Vec<_> = l.split_whitespace().collect();
            let (p, t) = f[0].split_once('/').unwrap();
            assert_eq!(p.parse::<u64>().unwrap(), pid);
            let tid = t.parse::<u64>().unwrap();
            let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
            if tid != pid {
                assert_eq!(profile["worker_tid_cpu"][tid.to_string()], cpu);
            }
            assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
            if f[5..].join(" ").starts_with("[unknown]") {
                unknown += 1;
            }
            *tids.entry(tid).or_default() += 1;
            count += 1;
        }
        assert_eq!(profile["samples"], count);
        assert_eq!(profile["unknown_samples"], unknown);
        assert_eq!(
            syms[name]
                .as_object()
                .unwrap()
                .values()
                .map(|v| v.as_u64().unwrap())
                .sum::<u64>(),
            count
        );
        assert_eq!(json!(tids), profile["sample_counts_by_tid"]);
        let share = if count == 0 {
            1.
        } else {
            unknown as f64 / count as f64
        };
        close(share, profile["unknown_share"].as_f64().unwrap());
        let loss = read(format!("{R}/runs/loss-{name}/stdout.log"));
        let qualified = count >= 1000 && share <= 0.05 && loss["zero_lost"] == true;
        assert_eq!(profile["qualified"], qualified);
        if !qualified {
            failed.push(format!("{name}/profile_qualification"));
        }
        assert_eq!(hash(format!("{R}/perf/{name}.data")), profile["raw_sha256"]);
        profile_samples += count;
    }
    for (name, panel) in analysis["counters"].as_object().unwrap() {
        let mut count = 0;
        let mut qualified = true;
        for line in fs::read_to_string(format!("{R}/stat/{name}.csv"))
            .unwrap()
            .lines()
            .filter(|s| !s.starts_with('#') && !s.trim().is_empty())
        {
            let f: Vec<_> = line.split(';').collect();
            if f.len() < 5 {
                continue;
            }
            let event = f[2].trim();
            if ![
                "cycles:u",
                "instructions:u",
                "cache-misses:u",
                "branch-misses:u",
            ]
            .contains(&event)
            {
                continue;
            }
            let a = f[0].trim().parse::<f64>().ok();
            let coverage = f[4].trim().parse::<f64>().ok();
            let q = a.is_some() && coverage.is_some_and(|x| x >= 90.);
            qualified &= q;
            let v = &panel["events"][event];
            assert_eq!(v["qualified"], q);
            if let Some(a) = a {
                close(a, v["count"].as_f64().unwrap());
            } else {
                assert!(v["count"].is_null());
            }
            if let Some(a) = coverage {
                close(a, v["coverage_percent"].as_f64().unwrap());
            }
            count += 1;
        }
        qualified &= count == 4;
        assert_eq!(panel["qualified"], qualified);
        if !qualified {
            failed.push(format!("{name}/counter_qualification"));
        }
    }
    let mut actual: Vec<String> = analysis["failed_checks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    actual.sort();
    failed.sort();
    assert_eq!(actual, failed);
    assert_eq!(analysis["gate_passed"], failed.is_empty());
    let ledger = read(
        "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176/proof-sha256-after.json",
    );
    assert_eq!(ledger.as_array().unwrap().len(), 30846);
    let out = json!({"review_passed":true,"pins_checked":pins.as_object().unwrap().len(),"new_build_commands":3,"capture_commands":1107,"threaded_caller_processes":1032,"legacy_preflight_processes":14,"new_native_tests":0,"inherited_native_tests":5,"inherited_Arkworks_tests":1,"unchanged_qualified_core_reused":true,"fresh_OFF_source_files_exact":177,"fresh_OFF_only_change":"Work SELECTED condition -> false","timing_checks_independently_reproduced":checks,"gate_passed":failed.is_empty(),"failed_checks":failed,"measured_complete_reduction_rounds":measured,"priming_complete_reduction_rounds":primed,"profiles":12,"profile_samples":profile_samples,"new_proofs":0,"ledger":30846,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Independent reviewPASS1107commands/1032threaded+14legacy/{profile_samples}samples; gate={} failed={}",failed.is_empty(),failed.len());
}
