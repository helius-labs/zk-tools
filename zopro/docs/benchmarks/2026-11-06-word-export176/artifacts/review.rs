use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/word-export176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn median(x: &[f64]) -> f64 {
    let mut y = x.to_vec();
    y.sort_by(f64::total_cmp);
    y[y.len() / 2]
}
fn close(a: f64, b: f64) {
    assert!(
        (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.),
        "{a}!={b}"
    );
}
fn stat(v: &[f64], expected: &Value) {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    for (k, i) in [("low", 3), ("median", 7), ("high", 11)] {
        close(s[i], expected[k].as_f64().unwrap());
    }
    assert_eq!(
        v.iter().filter(|&&x| x < 1.).count(),
        expected["wins"].as_u64().unwrap() as usize
    );
    for (a, b) in v.iter().zip(expected["ratios"].as_array().unwrap()) {
        close(*a, b.as_f64().unwrap());
    }
}
fn main() {
    assert!(!Path::new(R).join("review.json").exists());
    let analysis = read(format!("{R}/analysis.json"));
    let plan = read(format!("{R}/plan.json"));
    let pins = read(format!("{R}/pins.json"));
    let mut pins_checked = 0;
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins_checked += 1;
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let audit = read(format!("{R}/caller-source-audit.json"));
    let mut originals = 0;
    let mut exact = 0;
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
        let mut b = fs::read(Path::new(R).join("caller-source").join(p)).unwrap();
        if format!("{:x}", Sha256::digest(&b)) == h.as_str().unwrap() {
            exact += 1;
        }
        if let Some(s) = audit["core_suffixes"].get(p) {
            let s = s.as_str().unwrap();
            assert!(b.ends_with(s.as_bytes()));
            b.truncate(b.len() - s.len());
        }
        if p == "crates/zopro-core/src/simd.rs" {
            let mut s = String::from_utf8(b).unwrap();
            let old = audit["simd_edit"][0].as_str().unwrap();
            let new = audit["simd_edit"][1].as_str().unwrap();
            assert_eq!(s.matches(new).count(), 1);
            s = s.replacen(new, old, 1);
            b = s.into_bytes();
        }
        if p == "crates/zopro-bn254/src/ifma8.rs" {
            let suffix = audit["field_include"].as_str().unwrap();
            assert!(b.ends_with(suffix.as_bytes()));
            b.truncate(b.len() - suffix.len());
        }
        assert_eq!(format!("{:x}", Sha256::digest(b)), h.as_str().unwrap());
        originals += 1;
    }
    assert_eq!((originals, exact), (176, 172));
    assert_eq!(
        fs::read(format!("{R}/word_export.rs")).unwrap(),
        fs::read(format!(
            "{R}/caller-source/crates/zopro-bn254/src/word_export.rs"
        ))
        .unwrap()
    );
    let ids = read(format!("{R}/caller-identities.json"));
    for arm in ["frozen", "OFF", "ON"] {
        assert_eq!(
            hash(ids[arm]["binary"].as_str().unwrap()),
            ids[arm]["binary_sha256"]
        );
    }
    let field = read(format!("{R}/field-info.json"));
    assert_eq!(hash(field["library"].as_str().unwrap()), field["sha256"]);
    assert_eq!(field["direct_lanes"], 19216);
    assert_eq!(field["chain_lanes"], 131072);
    for arm in ["OFF", "ON"] {
        let c = read(format!("{R}/caller-native-repair/core-{arm}/command.json"));
        let a = c["argv"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(a.iter().any(|x| *x == "zopro_word_export"), arm == "ON");
        let actualfield = format!("zopro_bn254={}", field["library"].as_str().unwrap());
        assert!(a.iter().any(|x| *x == actualfield));
        let link = &ids[arm]["actual_link_argv"];
        let a = link
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect::<Vec<_>>();
        assert!(a.contains(&"-O"));
        for x in ["target-cpu=native", "lto=fat", "codegen-units=1"] {
            assert!(a.windows(2).any(|p| p[0] == "-C" && p[1] == x));
        }
        assert_eq!(a.contains(&"zopro_word_export"), arm == "ON");
        assert!(a.iter().any(|x| *x == actualfield));
        assert_eq!(
            hash(ids[arm]["core_library"].as_str().unwrap()),
            ids[arm]["core_sha256"]
        );
    }
    let mut command_count = 0;
    for (planfile, dir) in [
        ("native-plan.json", "native"),
        ("codegen-plan.json", "codegen"),
        ("caller-native-repair-plan.json", "caller-native-repair"),
        ("plan.json", "runs"),
    ] {
        let p = read(format!("{R}/{planfile}"));
        for step in p["steps"].as_array().unwrap() {
            let name = step["name"].as_str().unwrap();
            let base = format!("{R}/{dir}/{name}");
            let receipt = read(format!("{base}/exit.json"));
            assert_eq!(receipt["returncode"], 0);
            assert_eq!(receipt["success"], true);
            for f in ["stdout", "stderr"] {
                assert_eq!(
                    hash(format!("{base}/{f}.log")),
                    receipt[format!("{f}_sha256")]
                );
            }
            let actual = read(format!("{base}/command.json"));
            assert_eq!(
                &actual["argv"].as_array().unwrap()[4..],
                step["argv"].as_array().unwrap()
            );
            command_count += 1;
        }
    }
    assert_eq!(command_count, 675);
    assert!(
        fs::read_to_string(format!("{R}/caller-native-repair/core-tests/stdout.log"))
            .unwrap()
            .contains("127 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out")
    );
    let mut callers = BTreeMap::new();
    let mut measured = 0u64;
    let mut priming = 0u64;
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        if n == "preflight-check"
            || !["preflight-", "latency-", "stat-", "cycles-"]
                .iter()
                .any(|p| n.starts_with(p))
        {
            continue;
        }
        let text = fs::read_to_string(format!("{R}/runs/{n}/stdout.log")).unwrap();
        let rows = text
            .lines()
            .filter_map(|l| l.strip_prefix("CALLER "))
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 1);
        let v: Value = serde_json::from_str(rows[0]).unwrap();
        assert_eq!(v["complete_generator_oracle"], true);
        assert_eq!(v["timed_allocations"], 0);
        assert_eq!(v["priming_passes"], 3);
        assert_eq!(v["priming_rounds"], 128);
        let w = v["workers"].as_u64().unwrap();
        let allowed = v["worker_rows"].as_array().unwrap();
        assert_eq!(allowed.len(), w as usize);
        let mut tids = BTreeSet::new();
        for (i, row) in allowed.iter().enumerate() {
            assert_eq!(row["worker"], i);
            assert_eq!(row["cpu"], if w == 1 { 2 } else { i });
            assert!(tids.insert(row["tid"].as_u64().unwrap()));
        }
        measured += w * v["rounds_per_worker"].as_u64().unwrap();
        priming += w * 384;
        assert_eq!(analysis["caller_receipts"][n], v);
        callers.insert(n.to_owned(), v);
    }
    assert_eq!(callers.len(), 600);
    let mut failures = BTreeSet::new();
    let mut ratios_all = BTreeMap::<String, Vec<f64>>::new();
    for row in analysis["rows"].as_array().unwrap() {
        let ctx = row["context"].as_str().unwrap();
        let parts = ctx.split('-').collect::<Vec<_>>();
        let w = parts[2].strip_prefix('w').unwrap().parse::<u64>().unwrap();
        let size = parts[1].parse::<u64>().unwrap();
        let mut times = BTreeMap::new();
        for arm in ["frozen", "OFF", "ON"] {
            let mut t = Vec::new();
            let mut cpu = Vec::new();
            let mut rss = 0f64;
            for b in 0..15 {
                let v = &callers[&format!("latency-{ctx}-b{b:02}-{arm}")];
                assert_eq!(v["kind"], parts[0]);
                assert_eq!(v["size"], size);
                assert_eq!(v["workers"], w);
                assert_eq!(v["rounds_per_worker"], 4096);
                for other in ["frozen", "OFF", "ON"] {
                    let ov = &callers[&format!("latency-{ctx}-b{b:02}-{other}")];
                    assert_eq!(v["output_sha256"], ov["output_sha256"]);
                    assert_eq!(
                        v["scratch_capacity_bytes_by_worker"],
                        ov["scratch_capacity_bytes_by_worker"]
                    );
                }
                t.push(v["elapsed_ns"].as_f64().unwrap() / 4096.);
                cpu.push(
                    (v["usage"]["user_cpu_ms"].as_f64().unwrap()
                        + v["usage"]["system_cpu_ms"].as_f64().unwrap())
                        / 4096.,
                );
                rss = rss.max(v["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
            }
            close(median(&t), row["median_ns_per_wave"][arm].as_f64().unwrap());
            close(median(&cpu), row["CPU_ms_per_wave"][arm].as_f64().unwrap());
            close(
                rss,
                row["counter_RSS_MiB_diagnostic"][arm].as_f64().unwrap(),
            );
            times.insert(arm, t);
        }
        for (num, den) in [("OFF", "frozen"), ("ON", "frozen"), ("ON", "OFF")] {
            let ratios = times[num]
                .iter()
                .zip(&times[den])
                .map(|(x, y)| x / y)
                .collect::<Vec<_>>();
            let key = format!("{num}/{den}");
            let s = &row["pairs"][&key];
            stat(&ratios, s);
            let high = s["high"].as_f64().unwrap();
            let low = s["low"].as_f64().unwrap();
            if if num == "OFF" {
                low < 1. / 1.03 || high > 1.03
            } else {
                high > 1.03
            } {
                failures.insert(format!(
                    "{ctx}/{num}/{den}/{}",
                    if num == "OFF" {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            ratios_all.insert(format!("{ctx}/{num}/{den}"), ratios);
        }
        for arm in ["frozen", "OFF", "ON"] {
            let v = &callers[&format!("preflight-{ctx}-{arm}")];
            assert_eq!(v["rounds_per_worker"], 64);
            assert_eq!(v["kind"], parts[0]);
            assert_eq!(v["size"], size);
            assert_eq!(v["workers"], w);
            for other in ["frozen", "OFF", "ON"] {
                let ov = &callers[&format!("preflight-{ctx}-{other}")];
                assert_eq!(v["output_sha256"], ov["output_sha256"]);
                assert_eq!(
                    v["scratch_capacity_bytes_by_worker"],
                    ov["scratch_capacity_bytes_by_worker"]
                );
            }
        }
    }
    for curve in ["G1", "G2"] {
        for den in ["frozen", "OFF"] {
            let v = (0..15)
                .map(|i| {
                    ((ratios_all[&format!("{curve}-200-w32/ON/{den}")][i].ln()
                        + ratios_all[&format!("{curve}-352-w32/ON/{den}")][i].ln())
                        / 2.)
                        .exp()
                })
                .collect::<Vec<_>>();
            let s = &analysis["curve32_aggregates"][format!("{curve}/{den}")];
            stat(&v, s);
            if s["high"].as_f64().unwrap() >= 1. || s["wins"].as_u64().unwrap() < 10 {
                failures.insert(format!("{curve}-32aggregate/ON/{den}/gain"));
            }
        }
    }
    let mut samples = 0u64;
    for (name, p) in analysis["profiles"].as_object().unwrap() {
        let v = &callers[name];
        let pid = v["pid"].as_u64().unwrap();
        let tids = v["worker_rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| (v["tid"].as_u64().unwrap(), v["cpu"].as_u64().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut count = 0u64;
        let mut unknown = 0;
        for line in fs::read_to_string(format!("{R}/runs/decode-{name}/stdout.log"))
            .unwrap()
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        {
            let f = line.split_whitespace().collect::<Vec<_>>();
            let (a, b) = f[0].split_once('/').unwrap();
            assert_eq!(a.parse::<u64>().unwrap(), pid);
            let tid = b.parse::<u64>().unwrap();
            let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
            if tid == pid {
                assert!(cpu < 32)
            } else {
                assert_eq!(tids[&tid], cpu)
            }
            assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
            if f[5..].join(" ").starts_with("[unknown]") {
                unknown += 1;
            }
            count += 1;
        }
        assert_eq!(count, p["samples"]);
        close(
            unknown as f64 / count as f64,
            p["unknown_share"].as_f64().unwrap(),
        );
        assert_eq!(hash(format!("{R}/perf/{name}.data")), p["raw_sha256"]);
        assert_eq!(
            read(format!("{R}/runs/loss-{name}/stdout.log"))["zero_lost"],
            true
        );
        let qualified = count >= 1000 && (unknown as f64 / count as f64) <= 0.05;
        assert_eq!(qualified, p["qualified"].as_bool().unwrap());
        if !qualified {
            failures.insert(format!("{name}/profile"));
        }
        samples += count;
    }
    assert_eq!(analysis["profiles"].as_object().unwrap().len(), 12);
    let recorded = analysis["failed_checks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(failures, recorded);
    assert_eq!(analysis["gate_passed"], failures.is_empty());
    let ledger = read(
        "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176/proof-sha256-after.json",
    );
    assert_eq!(ledger.as_array().unwrap().len(), 30846);
    let review = json!({"review_passed":true,"pins_checked":pins_checked,"successful_workflow_commands":command_count,"caller_processes":callers.len(),"measured_kernel_batches":measured,"priming_kernel_batches":priming,"profiles":12,"profile_samples":samples,"source_originals_recovered":176,"source_originals_exact":172,"native_core_tests":127,"reapplied_core_test_populations":2,"field_direct_lanes":19216,"field_chain_lanes":131072,"gate_passed":failures.is_empty(),"failed_checks":failures,"new_proofs":0,"ledger":30846,"adopted":false,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&review).unwrap(),
    )
    .unwrap();
    println!("Independent review PASS {command_count}commands/600callers/{samples}samples; gate={} and all failures retained",failures.is_empty());
}
