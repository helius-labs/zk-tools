use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk";
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
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    assert!(!s.is_empty());
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn number(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn samples(name: &str) -> (Vec<f64>, Value) {
    let t = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
    let v = t
        .lines()
        .filter_map(|l| l.strip_prefix("SAMPLE "))
        .map(|l| number(&serde_json::from_str::<Value>(l).unwrap(), "ms"))
        .collect();
    let s =
        serde_json::from_str(t.lines().find_map(|l| l.strip_prefix("SUMMARY ")).unwrap()).unwrap();
    (v, s)
}
fn main() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins = read(r("pins.json"));
    for (f, h) in pins.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap(), "{f}")
    }
    let jobs = read(r("jobs.json"));
    let jobs = jobs.as_array().unwrap();
    assert_eq!(jobs.len(), 750);
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 13712);
    let mut inventory = BTreeMap::new();
    let mut cells: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut counters: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut resources: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    let mut requests = 0;
    let mut total_samples = 0;
    for j in jobs {
        let name = j["name"].as_str().unwrap();
        let phase = j["phase"].as_str().unwrap();
        let (times, s) = samples(name);
        let count = j["rounds"].as_u64().unwrap() as usize;
        assert_eq!(times.len(), count);
        assert!(times.iter().all(|&x| x > 0. && x.is_finite()));
        assert_eq!(s["rounds"], j["rounds"]);
        assert_eq!(s["workers"], j["workers"]);
        assert_eq!(s["priming"], 3);
        requests += count + 3;
        if phase == "resource" {
            assert_eq!(s["heap_instrumented"], true)
        } else {
            assert!(s.get("heap").is_none());
            assert!(s.get("heap_instrumented").is_none());
        }
        let dir = Path::new(j["directory"].as_str().unwrap());
        let verified = fs::read_to_string(r(format!("runs/verify-{name}/stdout.log"))).unwrap();
        assert!(verified.contains("VERIFIED proof_sha256="));
        assert!(verified.contains(&format!(
            "image_sha256={}",
            j["case"]["image_sha256"].as_str().unwrap()
        )));
        for ext in ["proof", "public"] {
            let p = dir.join(format!("0.{ext}"));
            let h = hash(&p);
            if ext == "proof" {
                assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                assert!(prior.insert(h.clone()));
                assert!(verified.contains(&format!("proof_sha256={h}")))
            } else {
                assert_eq!(h, j["case"]["public_sha256"].as_str().unwrap());
                assert!(verified.contains(&format!("public_sha256={h}")))
            }
            inventory.insert(p.display().to_string(), h);
        }
        let key = format!(
            "{}/{}",
            j["case"]["name"].as_str().unwrap(),
            j["variant"].as_str().unwrap()
        );
        let row = json!({"name":name,"block":j["block"],"median_ms":median(&times),"cpu_ms_per_request":(number(&s,"user_cpu_ms")+number(&s,"system_cpu_ms"))/count as f64,"RSS_MiB":number(&s,"whole_process_maxrss_kib")/1024.,"summary":s});
        match phase {
            "latency" => cells.entry(key).or_default().push(row),
            "resource" => {
                assert_eq!(count, 1);
                let mut row = row;
                let h = &s["heap"];
                for f in [
                    "live_before",
                    "live_after",
                    "peak_live",
                    "peak_extra",
                    "allocation_requests",
                    "requested_bytes",
                ] {
                    assert!(h[f].as_u64().is_some());
                }
                assert!(h["peak_live"].as_u64().unwrap() >= h["live_before"].as_u64().unwrap());
                row["heap"] = h.clone();
                resources.entry(key).or_default().push(row)
            }
            "counter" => {
                let t = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
                let mut events = BTreeMap::new();
                for l in t.lines() {
                    let f: Vec<_> = l.trim().split(';').collect();
                    if f.len() < 5 {
                        continue;
                    }
                    let event = f[2].strip_suffix(":u").unwrap_or(f[2]);
                    if [
                        "cycles",
                        "instructions",
                        "L1-icache-load-misses",
                        "iTLB-load-misses",
                        "stalled-cycles-frontend",
                        "L1-dcache-load-misses",
                    ]
                    .contains(&event)
                    {
                        let n: f64 = f[0].parse().unwrap();
                        let pct: f64 = f[4].trim_end_matches('%').parse().unwrap();
                        assert!(pct >= 99.9, "{name} {event} {pct}");
                        assert!(events.insert(event,json!({"count":n,"per_request":n/count as f64,"running_percent":pct})).is_none());
                    }
                }
                assert_eq!(events.len(), 4);
                let mut row = row;
                row["events"] = json!(events);
                counters
                    .entry(format!("{key}/{}", j["panel"].as_str().unwrap()))
                    .or_default()
                    .push(row)
            }
            "profile" => {
                let t = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
                let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
                assert!(!loss.contains("PERF_RECORD_LOST"), "{name}");
                let (mut pids, mut tids) = (BTreeSet::new(), BTreeSet::new());
                let (mut samples, mut period, mut borrowed) = (0u64, 0u64, 0u64);
                let mut symbols = BTreeMap::<String, u64>::new();
                for l in t.lines().filter(|l| !l.trim_start().starts_with('#')) {
                    let f: Vec<_> = l.split_whitespace().collect();
                    if f.len() < 4 {
                        continue;
                    }
                    let parsed = if let Some((p, tid)) = f[0].split_once('/') {
                        p.parse::<u64>()
                            .ok()
                            .zip(tid.parse::<u64>().ok())
                            .zip(f[1].parse::<u64>().ok())
                            .map(|((p, tid), n)| (p, tid, n, 3))
                    } else if f.len() >= 5 {
                        f[0].parse::<u64>()
                            .ok()
                            .zip(f[1].parse::<u64>().ok())
                            .zip(f[2].parse::<u64>().ok())
                            .map(|((p, tid), n)| (p, tid, n, 4))
                    } else {
                        None
                    };
                    if let Some((pid, tid, n, start)) = parsed {
                        pids.insert(pid);
                        tids.insert(tid);
                        let symbol =
                            format!("{:#}", rustc_demangle::demangle(&f[start..].join(" ")));
                        if symbol.contains("g2_borrowed") {
                            borrowed += n
                        }
                        *symbols.entry(symbol).or_default() += n;
                        samples += 1;
                        period += n;
                    }
                }
                assert_eq!(pids.len(), 1, "{name}");
                assert!(samples > 0 && period > 0);
                assert!(
                    tids.len() >= j["workers"].as_u64().unwrap() as usize,
                    "sampled coverage {name}:{}",
                    tids.len()
                );
                if j["variant"] == "on" {
                    assert!(borrowed > 0, "no selected borrowed path samples {name}")
                } else {
                    assert_eq!(borrowed, 0)
                }
                total_samples += samples;
                let file = r("profiles").join(name).join("perf.data");
                raw.insert(file.display().to_string(), hash(&file));
                profiles.insert(name.to_string(),json!({"samples":samples,"lost":0,"pids":pids,"sampled_TIDs":tids,"sampled_TID_count":tids.len(),"intended_workers":j["workers"],"period":period,"borrowed_symbol_period":borrowed,"borrowed_period_fraction":borrowed as f64/period as f64,"symbols":symbols,"summary":s}));
            }
            "preflight" => {}
            _ => panic!("phase"),
        }
    }
    assert_eq!(requests, 39060);
    assert_eq!(prior.len(), 14462);
    assert_eq!(inventory.len(), 1500);
    assert_eq!(profiles.len(), 6);
    save(r("proof-inventory.json"), &json!(inventory));
    let mut summaries = BTreeMap::new();
    for (k, rows) in &cells {
        assert_eq!(rows.len(), 15);
        let collect = |f: &str| rows.iter().map(|v| number(v, f)).collect::<Vec<_>>();
        let rss = collect("RSS_MiB");
        summaries.insert(k,json!({"median_ms":median(&collect("median_ms")),"cpu_ms":median(&collect("cpu_ms_per_request")),"median_RSS_MiB":median(&rss),"max_RSS_MiB":rss.iter().copied().fold(0.,f64::max),"blocks":rows}));
    }
    let mut pairs = BTreeMap::new();
    let mut failed = vec![];
    let mut checks = BTreeMap::new();
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        for (a, b) in [
            ("frozen", "accepted"),
            ("off", "frozen"),
            ("off", "accepted"),
            ("on", "accepted"),
            ("on", "frozen"),
            ("on", "off"),
        ] {
            let x = &cells[&format!("{case}/{a}")];
            let y = &cells[&format!("{case}/{b}")];
            let mut ratios = vec![];
            for i in 0..15 {
                assert_eq!(x[i]["block"], i);
                assert_eq!(y[i]["block"], i);
                ratios.push(number(&x[i], "median_ms") / number(&y[i], "median_ms"));
            }
            let mut s = ratios.clone();
            s.sort_by(f64::total_cmp);
            let wins = s.iter().filter(|&&r| r < 1.).count();
            let key = format!("{case}/{a}/{b}");
            let v = json!({"median":s[7],"low":s[3],"high":s[11],"wins":wins,"all_ratios":ratios,"nominal_coverage":0.96484375});
            pairs.insert(key.clone(), v);
            let pass = if (a, b) == ("frozen", "accepted") || (a, b) == ("off", "frozen") {
                Some(s[3] >= 1. / 1.03 && s[11] <= 1.03)
            } else if a == "on" {
                Some(if case == "1x2-w32" {
                    s[11] < 1. && wins >= 11
                } else {
                    s[11] <= 1.02
                })
            } else {
                None
            };
            if let Some(pass) = pass {
                checks.insert(key.clone(), pass);
                if !pass {
                    failed.push(key)
                }
            }
        }
    }
    let mut counter_summary = BTreeMap::new();
    for (k, v) in counters {
        assert_eq!(v.len(), 3);
        let mut m = BTreeMap::new();
        for event in v[0]["events"].as_object().unwrap().keys() {
            m.insert(
                event,
                median(
                    &v.iter()
                        .map(|x| number(&x["events"][event], "per_request"))
                        .collect::<Vec<_>>(),
                ),
            );
        }
        counter_summary.insert(k, json!({"median_events_per_request":m,"observations":v}));
    }
    let mut resource_summary = BTreeMap::new();
    for (k, v) in resources {
        assert_eq!(v.len(), 3);
        let mut m = BTreeMap::new();
        for f in [
            "live_before",
            "live_after",
            "peak_live",
            "peak_extra",
            "allocation_requests",
            "requested_bytes",
        ] {
            m.insert(
                f,
                median(&v.iter().map(|x| number(&x["heap"], f)).collect::<Vec<_>>()),
            );
        }
        resource_summary.insert(k, json!({"median_heap":m,"observations":v}));
    }
    let mut refs = BTreeMap::new();
    let mut achieved = 0;
    for c in read(r("cases.json")).as_array().unwrap().iter().take(9) {
        if c["reference"].is_null() {
            continue;
        }
        let case = c["name"].as_str().unwrap();
        let shape = c["shape"].as_str().unwrap();
        let cpu = number(&c["reference"], "cpu_ms");
        let metal = number(&c["reference"], "metal_ms");
        let mut out = BTreeMap::new();
        for v in ["accepted", "frozen", "off", "on"] {
            let ms = number(&summaries[&format!("{case}/{v}")], "median_ms");
            out.insert(
                v,
                json!({"ms":ms,"cpu_speedup":cpu/ms,"metal_speedup":metal/ms,"CPU10x":ms<=cpu/10.}),
            );
            if v == "on" && ms <= cpu / 10. {
                achieved += 1
            }
        }
        refs.insert(shape.to_owned(),json!({"cpu_reference_ms":cpu,"metal_reference_ms":metal,"required_ms":cpu/10.,"variants":out}));
    }
    let gate = failed.is_empty();
    let analysis = json!({"status":if gate{"passed full SDK screen; broader qualification required"}else{"failed full SDK screen; not adopted"},"gate_passed":gate,"failed_checks":failed,"checks":checks,"cells":summaries,"pairs":pairs,"counters":counter_summary,"resources":resource_summary,"profiles":profiles,"raw_profile_pins":raw,"profile_samples":total_samples,"SDK_requests":requests,"new_independent_exports":750,"independent_proof_count":14462,"source_originals_exact":168,"reverse172_exact":true,"native_core_tests_inherited":{"off":119,"on":122},"new_native_tests":0,"native_libraries_rebuilt":false,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":achieved,"m5":refs,"scope":"all8+54 native verified SDK, primary16/32, separate heap observations/counters/profiles; no AWS/mobile/pressure/250/4999 qualification"});
    save(r("analysis.json"), &analysis);
    println!("SDK gate{gate};{}failedchecks;39060verifiedrequests/750Gnarkexports/ledger14462;CPU10x{achieved}/8",failed.len());
    for (k, v) in &summaries {
        println!(
            "{k} {:.6}ms medianRSS{:.2}MiB",
            number(v, "median_ms"),
            number(v, "median_RSS_MiB")
        );
    }
    for (k, v) in &pairs {
        if k.starts_with("1x2-") || k.starts_with("merge-54") {
            println!(
                "{k} {} [{},{}] wins{}",
                v["median"], v["low"], v["high"], v["wins"]
            );
        }
    }
}
