fn finish() {
    assert!(!r("analysis.json").exists());
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = read(r("plan.json"));
    let mut callers = BTreeMap::new();
    let mut receipts = Vec::new();
    for step in plan["steps"].as_array().unwrap() {
        let n = step["name"].as_str().unwrap();
        let base = r(format!("runs/{n}"));
        let exit = read(base.join("exit.json"));
        assert_eq!(exit["returncode"], 0, "{n}");
        assert_eq!(exit["success"], true);
        let cmd = read(base.join("command.json"));
        assert_eq!(
            &cmd["argv"].as_array().unwrap()[4..],
            step["argv"].as_array().unwrap().as_slice()
        );
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(base.join(format!("{stream}.log"))),
                exit[format!("{stream}_sha256")]
            );
        }
        receipts.push(json!({"name":n,"exit_sha256":hash(base.join("exit.json")),"elapsed_ms":exit["elapsed_ms"]}));
        if n != "preflight-check"
            && ["preflight-", "latency-", "stat-", "cycles-"]
                .iter()
                .any(|p| n.starts_with(p))
        {
            let v = result(n);
            assert_eq!(v["complete_projective_oracle"], true);
            assert_eq!(v["SDK_requests"], 0);
            assert_eq!(v["proofs"], 0);
            assert_eq!(v["priming_rounds_per_worker"], 24);
            assert_eq!(v["benchmark_Rayon_broadcast"], true);
            assert_eq!(
                v["allocation_count_scope"],
                "sum of worker TLS deltas, excludes main/coordinator"
            );
            if v["fresh_buffers"] == false {
                assert_eq!(v["timed_allocations"], 0);
            }
            let w = v["workers"].as_u64().unwrap();
            assert_eq!(v["worker_rows"].as_array().unwrap().len(), w as usize);
            let mut tids = BTreeSet::new();
            let mut worker_allocations = 0u64;
            for row in v["worker_rows"].as_array().unwrap() {
                let id = row["id"].as_u64().unwrap();
                assert_eq!(row["cpu"], if w == 1 { 2 } else { id });
                assert!(tids.insert(row["tid"].as_u64().unwrap()));
                assert_eq!(row["output_sha256"], v["output_sha256"]);
                assert_eq!(row["pool_workers"], w);
                worker_allocations += row["timed_worker_allocation_calls"].as_u64().unwrap();
            }
            assert_eq!(v["timed_allocations"], worker_allocations);
            callers.insert(n.to_owned(), v);
        }
    }
    assert_eq!(callers.len(), 1326);
    assert_eq!(receipts.len(), 1402);
    let mut failed = Vec::new();
    let mut rows = Vec::new();
    let mut all_ratios = BTreeMap::new();
    let mut gate_count = 0;
    for c in contexts() {
        let mut times = BTreeMap::new();
        let mut cpu = BTreeMap::new();
        let mut rss = BTreeMap::new();
        let mut caps = BTreeMap::new();
        let mut forecasts = BTreeMap::new();
        let mut allocs = BTreeMap::new();
        let mut faults = BTreeMap::new();
        let mut usage = BTreeMap::new();
        for arm in ["frozen", "OFF", "ON"] {
            let pre = &callers[&format!("preflight-{}-{arm}", c.name)];
            let mut t = Vec::new();
            let mut u = Vec::new();
            let mut f = Vec::new();
            let mut allocation = Vec::new();
            let mut maxrss = 0f64;
            for b in 0..15 {
                let v = &callers[&format!("latency-{}-b{b:02}-{arm}", c.name)];
                assert_eq!(v["case"], c.case);
                assert_eq!(v["workers"], c.workers);
                assert_eq!(v["rounds_per_worker"], c.rounds);
                assert_eq!(v["output_sha256"], pre["output_sha256"]);
                let cap = &v["worker_rows"][0]["scratch_capacity_bytes"];
                for each in v["worker_rows"].as_array().unwrap() {
                    assert_eq!(&each["scratch_capacity_bytes"], cap);
                }
                assert_eq!(*cap, pre["worker_rows"][0]["scratch_capacity_bytes"]);
                for other in ["frozen", "OFF", "ON"] {
                    let ov = &callers[&format!("latency-{}-b{b:02}-{other}", c.name)];
                    assert_eq!(v["output_sha256"], ov["output_sha256"]);
                }
                t.push(v["elapsed_ns"].as_f64().unwrap() / c.rounds as f64);
                u.push(
                    (v["process_usage"]["user_cpu_ms"].as_f64().unwrap()
                        + v["process_usage"]["system_cpu_ms"].as_f64().unwrap())
                        / c.rounds as f64,
                );
                f.push(v["process_usage"]["minor_faults"].as_f64().unwrap() / c.rounds as f64);
                allocation.push(v["allocation_calls_per_worker_round"].as_f64().unwrap());
                maxrss = maxrss.max(
                    v["process_usage"]["whole_process_maxrss_kib"]
                        .as_f64()
                        .unwrap()
                        / 1024.,
                );
            }
            caps.insert(arm, pre["worker_rows"][0]["scratch_capacity_bytes"].clone());
            forecasts.insert(arm, pre["worker_rows"][0]["source_forecast_bytes"].clone());
            cpu.insert(arm, median(&u));
            faults.insert(arm, median(&f));
            allocs.insert(arm, median(&allocation));
            rss.insert(arm, maxrss);
            usage.insert(arm, median(&u) / (median(&t) / 1e6));
            times.insert(arm, t);
        }
        assert_eq!(caps["frozen"], caps["OFF"]);
        let mut pairs = BTreeMap::new();
        for (num, den) in [("OFF", "frozen"), ("ON", "OFF"), ("ON", "frozen")] {
            let values = times[num]
                .iter()
                .zip(&times[den])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let stats = interval(&values);
            gate_count += 1;
            let passed = if num == "OFF" {
                stats["low"].as_f64().unwrap() >= 0.97 && stats["high"].as_f64().unwrap() <= 1.03
            } else {
                stats["high"].as_f64().unwrap() <= 1.01
            };
            if !passed {
                failed.push(format!(
                    "{}/{num}/{den}/{}",
                    c.name,
                    if num == "OFF" {
                        "calibration"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if num == "ON" && c.case == "group35973" && [16, 32].contains(&c.workers) {
                gate_count += 1;
                if stats["high"].as_f64().unwrap() >= 1.0 || stats["wins"].as_u64().unwrap() < 11 {
                    failed.push(format!("{}/{num}/{den}/gain", c.name));
                }
            }
            all_ratios.insert(format!("{}/{num}/{den}", c.name), values);
            pairs.insert(format!("{num}/{den}"), stats);
        }
        rows.push(json!({"context":c.name,"case":c.case,"workers":c.workers,"main":c.main,"weighted":c.case.starts_with("weighted"),"rounds_per_worker":c.rounds,"median_ns_per_wave":times.iter().map(|(k,v)|(*k,median(v))).collect::<BTreeMap<_,_>>(),"CPU_ms_per_wave":cpu,"mean_busy_CPUs_from_medians":usage,"max_observed_process_RSS_MiB":rss,"median_minor_faults_per_wave":faults,"median_alloc_calls_per_worker_round":allocs,"retained_capacity_bytes_first_worker":caps,"source_forecast_bytes_first_worker":forecasts,"pairs":pairs}));
    }
    let mut aggregates = BTreeMap::new();
    for den in ["OFF", "frozen"] {
        let v = (0..15)
            .map(|b| {
                contexts()
                    .iter()
                    .filter(|c| c.main && c.workers == 32)
                    .map(|c| all_ratios[&format!("{}/ON/{den}", c.name)][b].ln())
                    .sum::<f64>()
                    / 4.
            })
            .map(f64::exp)
            .collect::<Vec<_>>();
        let stats = interval(&v);
        gate_count += 1;
        if stats["high"].as_f64().unwrap() >= 1. || stats["wins"].as_u64().unwrap() < 11 {
            failed.push(format!("main32aggregate/ON/{den}/gain"));
        }
        aggregates.insert(den, stats);
    }
    assert_eq!(gate_count, 87);
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    for c in contexts().iter().filter(|c| profiled(c)) {
        for arm in ["frozen", "OFF", "ON"] {
            let stat_name = format!("stat-{}-{arm}", c.name);
            let mut stats = BTreeMap::new();
            let mut counter_qualified = true;
            for line in fs::read_to_string(r(format!("stat/{stat_name}.csv")))
                .unwrap()
                .lines()
                .filter(|s| !s.starts_with('#') && !s.trim().is_empty())
            {
                let f = line.split(';').collect::<Vec<_>>();
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
                let count = f[0].trim().parse::<f64>().ok();
                let coverage = f[4].trim().parse::<f64>().ok();
                let qualified = count.is_some() && coverage.is_some_and(|x| x >= 90.);
                counter_qualified &= qualified;
                stats.insert(event.to_owned(),json!({"count":count,"coverage_percent":coverage,"qualified":qualified,"per_wave":count.map(|n|n/(profr(c)/4)as f64)}));
            }
            counter_qualified &= stats.len() == 4;
            if !counter_qualified {
                failed.push(format!("{stat_name}/counter_qualification"));
            }
            counters.insert(
                stat_name,
                json!({"qualified":counter_qualified,"events":stats}),
            );
            let name = format!("cycles-{}-{arm}", c.name);
            let v = &callers[&name];
            assert_eq!(v["rounds_per_worker"], profr(c));
            let assigned = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| (row["tid"].as_u64().unwrap(), row["cpu"].as_u64().unwrap()))
                .collect::<BTreeMap<_, _>>();
            let metadata =
                fs::read_to_string(r(format!("runs/metadata-{name}/stdout.log"))).unwrap();
            let binary = binary_for(arm);
            let owners = metadata
                .lines()
                .filter(|s| s.contains("PERF_RECORD_MMAP") && s.contains(binary.to_str().unwrap()))
                .map(|s| {
                    s.split_whitespace()
                        .next()
                        .unwrap()
                        .split('/')
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                })
                .collect::<BTreeSet<_>>();
            assert_eq!(owners.len(), 1);
            let pid = *owners.first().unwrap();
            assert_eq!(v["pid"], pid);
            let mut forked = BTreeSet::from([pid]);
            for l in metadata.lines() {
                if let Some((_, tail)) = l.split_once("PERF_RECORD_FORK(") {
                    let f = tail
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if f[0] == pid {
                        forked.insert(f[1]);
                    }
                }
            }
            assert!(assigned.keys().all(|tid| forked.contains(tid)));
            let attributes =
                fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();
            for token in ["cycles:u", "16000000", "CPU", "TIME", "PERIOD"] {
                assert!(attributes.contains(token));
            }
            assert!(!attributes.contains("freq: 1"));
            let loss = read(r(format!("runs/loss-{name}/stdout.log")));
            let zero_lost = loss["zero_lost"] == true;
            let owner = read(r(format!("runs/own-{name}/stdout.log")));
            assert_eq!(owner["sha256_before"], owner["sha256_after"]);
            assert_eq!(hash(r(format!("perf/{name}.data"))), owner["sha256_before"]);
            let mut count = 0;
            let mut worker_samples = 0;
            let mut unknown = 0;
            let mut bins = BTreeMap::<String, u64>::new();
            let mut tid_counts = BTreeMap::<u64, u64>::new();
            for l in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|s| !s.starts_with('#') && !s.trim().is_empty())
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let (p, t) = f[0].split_once('/').unwrap();
                assert_eq!(p.parse::<u64>().unwrap(), pid);
                let tid = t.parse::<u64>().unwrap();
                assert!(forked.contains(&tid));
                let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                if let Some(&expected) = assigned.get(&tid) {
                    assert_eq!(expected, cpu);
                    worker_samples += 1;
                } else {
                    assert!(if c.workers == 1 {
                        cpu == 2
                    } else {
                        cpu < c.workers as u64
                    });
                }
                assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
                let raw = f[5..].join(" ");
                let symbol = rustc_demangle::try_demangle(&raw)
                    .map(|s| format!("{s:#}"))
                    .unwrap_or(raw);
                if symbol.starts_with("[unknown]") {
                    unknown += 1;
                }
                *bins.entry(symbol).or_default() += 1;
                *tid_counts.entry(tid).or_default() += 1;
                count += 1;
            }
            let unknown_share = if count == 0 {
                1.
            } else {
                unknown as f64 / count as f64
            };
            let worker_share = if count == 0 {
                0.
            } else {
                worker_samples as f64 / count as f64
            };
            let qualified =
                count >= 1000 && unknown_share <= 0.05 && zero_lost && worker_share >= 0.95;
            if !qualified {
                failed.push(format!("{name}/profile_qualification"));
            }
            symbols.insert(name.clone(), json!(bins));
            profiles.insert(name,json!({"qualified":qualified,"samples":count,"unknown_samples":unknown,"unknown_share":unknown_share,"worker_samples":worker_samples,"worker_share":worker_share,"other_owned_samples":count-worker_samples,"zero_lost":zero_lost,"PID":pid,"worker_tid_cpu":assigned,"owned_TIDs_from_metadata":forked,"sample_counts_by_tid":tid_counts,"raw_sha256":owner["sha256_before"]}));
        }
    }
    save(r("profile-symbols.json"), &json!(symbols));
    save(
        r("analysis.json"),
        &json!({"gate_passed":failed.is_empty(),"failed_checks":failed,"timing_checks":gate_count,"rows":rows,"main32_aggregates":aggregates,"counters":counters,"profiles":profiles,"caller_receipts":callers,"command_receipts":receipts,"adopted":false,"production_changed":false,"goal_achieved":false,"new_proofs":0,"proof_ledger":30846,"goal_status":"active/unmet0/8"}),
    );
    println!("Closed1402commands/1326threadedcallers/27contexts incl6weighted; gate={} failed_checks={}; no SDK/adoption",failed.is_empty(),failed.len());
}
