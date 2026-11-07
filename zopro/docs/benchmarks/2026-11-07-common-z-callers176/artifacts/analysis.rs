fn finish() {
    assert!(!r("analysis.json").exists());
    assert_eq!(read(r("capture/result.json"))["status"], "complete");
    for (p, h) in read(r("capture-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = read(r("capture-plan.json"));
    let mut receipts = Vec::new();
    let mut callers = BTreeMap::new();
    for s in plan["steps"].as_array().unwrap() {
        let name = s["name"].as_str().unwrap();
        receipts.push(receipt(&r(format!("capture/{name}")), s));
        if ["preflight-", "latency-", "stat-", "cycles-"]
            .iter()
            .any(|p| name.starts_with(p))
            && name != "preflight-review"
        {
            let v = result(name);
            validate_caller(&v);
            callers.insert(name.to_string(), v);
        }
    }
    assert_eq!(receipts.len(), 1949);
    assert_eq!(callers.len(), 1928);
    let mut failed = Vec::new();
    let mut rows = Vec::new();
    let mut ratios = BTreeMap::<String, Vec<f64>>::new();
    let mut checks = Vec::new();
    for c in contexts() {
        let mut times = BTreeMap::<String, Vec<f64>>::new();
        let mut metrics = BTreeMap::new();
        let mut caps = BTreeMap::new();
        for arm in ARMS {
            let pre = &callers[&format!("preflight-{}-{arm}", c.name())];
            let cap = &pre["worker_rows"][0]["scratch_capacity_bytes"];
            let capbytes = cap
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap())
                .sum::<u64>();
            caps.insert(arm, capbytes);
            let mut t = Vec::new();
            let mut cpu = Vec::new();
            let mut faults = Vec::new();
            let mut alloc = Vec::new();
            let mut maxrss = 0.;
            for b in 0..15 {
                let v = &callers[&format!("latency-{}-b{b:02}-{arm}", c.name())];
                assert_eq!(v["rounds_per_worker"], c.n);
                assert_eq!(v["case"], c.case);
                assert_eq!(v["workers"], c.w);
                assert_eq!(v["output_sha256"], pre["output_sha256"]);
                for row in v["worker_rows"].as_array().unwrap() {
                    assert_eq!(&row["scratch_capacity_bytes"], cap);
                }
                let time = v["ns_per_wave"].as_f64().unwrap();
                assert!(time > 0.);
                t.push(time);
                let usage = &v["process_usage"];
                cpu.push(
                    (usage["user_cpu_ms"].as_f64().unwrap()
                        + usage["system_cpu_ms"].as_f64().unwrap())
                        / c.n as f64,
                );
                faults.push(usage["minor_faults"].as_f64().unwrap() / c.n as f64);
                alloc.push(v["allocation_calls_per_worker_round"].as_f64().unwrap());
                maxrss = f64::max(
                    maxrss,
                    usage["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.,
                );
                for other in ARMS {
                    assert_eq!(
                        v["output_sha256"],
                        callers[&format!("latency-{}-b{b:02}-{other}", c.name())]["output_sha256"]
                    );
                }
            }
            metrics.insert(arm,json!({"median_us":median(&t)/1000.,"median_cpu_ms_per_wave":median(&cpu),"median_busy_CPU_equivalent":median(&cpu)/(median(&t)/1e6),"minor_faults_per_wave":median(&faults),"worker_allocation_calls_per_round":median(&alloc),"whole_process_maxrss_MiB":maxrss,"worker_capacity_bytes":capbytes}));
            times.insert(arm.into(), t);
        }
        for (numerator, denominator, kind) in [
            ("fresh", "frozen", "calibration"),
            ("OFF", "frozen", "calibration"),
            ("ON", "frozen", "nonregression"),
            ("ON", "fresh", "nonregression"),
            ("ON", "OFF", "nonregression"),
        ] {
            let data = (0..15)
                .map(|b| times[numerator][b] / times[denominator][b])
                .collect::<Vec<_>>();
            let i = interval(&data);
            let pass = if kind == "calibration" {
                i["low"].as_f64().unwrap() >= 1. / 1.03 && i["high"].as_f64().unwrap() <= 1.03
            } else {
                i["high"].as_f64().unwrap() <= 1.03
            };
            let name = format!("{}/{numerator}/{denominator}/{kind}", c.name());
            if !pass {
                failed.push(name.clone());
            }
            checks.push(json!({"name":name,"passed":pass,"statistics":i}));
            ratios.insert(format!("{}/{numerator}/{denominator}", c.name()), data);
        }
        for other in ["frozen", "fresh", "OFF"] {
            let pass = caps["ON"] as f64 <= caps[other] as f64 * 1.05;
            let name = format!("{}/ON/{other}/heap_capacity", c.name());
            if !pass {
                failed.push(name.clone());
            }
            checks.push(json!({"name":name,"passed":pass,"ON_bytes":caps["ON"],"control_bytes":caps[other]}));
        }
        rows.push(
            json!({"context":c.name(),"case":c.case,"workers":c.w,"rounds":c.n,"arms":metrics}),
        );
    }
    let mut aggregate = BTreeMap::new();
    for control in ["frozen", "fresh", "OFF"] {
        let data = (0..15)
            .map(|b| {
                let v = contexts()
                    .into_iter()
                    .filter(|c| c.w == 32)
                    .map(|c| ratios[&format!("{}/ON/{control}", c.name())][b].ln())
                    .collect::<Vec<_>>();
                (v.iter().sum::<f64>() / v.len() as f64).exp()
            })
            .collect::<Vec<_>>();
        let i = interval(&data);
        let pass = i["high"].as_f64().unwrap() < 1. && i["wins"].as_u64().unwrap() >= 11;
        let name = format!("all10cases32/ON/{control}/gain");
        if !pass {
            failed.push(name.clone());
        }
        checks.push(json!({"name":name,"passed":pass,"statistics":i}));
        aggregate.insert(control, i);
    }
    let mut counters = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    for c in contexts()
        .into_iter()
        .filter(|c| c.w == 32 && ["tile511", "groups3x4"].contains(&c.case))
    {
        for arm in ["frozen", "ON"] {
            let name = format!("stat-{}-{arm}", c.name());
            let mut values = BTreeMap::new();
            for line in fs::read_to_string(r(format!("stat/{name}.csv")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#'))
            {
                let f = line.split(';').collect::<Vec<_>>();
                if f.len() < 5 || !f[2].ends_with(":u") {
                    continue;
                }
                let value = f[0].parse::<f64>().unwrap();
                let running = f[4].parse::<f64>().unwrap();
                assert!(running >= 99.99);
                values.insert(f[2].to_string(),json!({"value":value,"running_percent":running,"per_worker_call":value/(counters_rounds(c.case)*c.w)as f64}));
            }
            assert_eq!(values.len(), 4);
            counters.insert(name, json!(values));
            let name = format!("cycles-{}-{arm}", c.name());
            let v = &callers[&name];
            let pid = v["pid"].as_u64().unwrap();
            let assigned = v["worker_rows"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| (v["tid"].as_u64().unwrap(), v["cpu"].as_u64().unwrap()))
                .collect::<BTreeMap<_, _>>();
            let metadata =
                fs::read_to_string(r(format!("capture/metadata-{name}/stdout.log"))).unwrap();
            let mut owned = BTreeSet::from([pid]);
            for l in metadata.lines() {
                if let Some((_, tail)) = l.split_once("PERF_RECORD_FORK(") {
                    let f = tail
                        .split(')')
                        .next()
                        .unwrap()
                        .split(':')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<_>>();
                    if f[0] == pid {
                        owned.insert(f[1]);
                    }
                }
            }
            assert!(assigned.keys().all(|t| owned.contains(t)));
            let events =
                fs::read_to_string(r(format!("capture/events-{name}/stdout.log"))).unwrap();
            for token in ["cycles:u", "4000000", "CPU", "TIME", "PERIOD"] {
                assert!(events.contains(token));
            }
            assert!(!events.contains("freq: 1"));
            let loss = read(r(format!("capture/loss-{name}/stdout.log")));
            let owner = read(r(format!("capture/own-{name}/stdout.log")));
            assert_eq!(owner["sha256_before"], owner["sha256_after"]);
            assert_eq!(hash(r(format!("perf/{name}.data"))), owner["sha256_before"]);
            let mut samples = 0usize;
            let mut workers = 0usize;
            let mut unknown = 0usize;
            let mut bins = BTreeMap::<String, usize>::new();
            for l in fs::read_to_string(r(format!("capture/decode-{name}/stdout.log")))
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = l.split_whitespace().collect::<Vec<_>>();
                let (p, t) = f[0].split_once('/').unwrap();
                assert_eq!(p.parse::<u64>().unwrap(), pid);
                let tid = t.parse::<u64>().unwrap();
                assert!(owned.contains(&tid));
                let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
                if let Some(&wanted) = assigned.get(&tid) {
                    assert_eq!(cpu, wanted);
                    workers += 1;
                } else {
                    assert!(cpu < 32);
                }
                assert_eq!(f[3].parse::<u64>().unwrap(), 4000000);
                let sym = f[5..].join(" ");
                if sym.starts_with("[unknown]") {
                    unknown += 1;
                }
                *bins.entry(sym).or_default() += 1;
                samples += 1;
            }
            let worker_share = workers as f64 / samples.max(1) as f64;
            let unknown_share = unknown as f64 / samples.max(1) as f64;
            let pass = samples >= 1000
                && worker_share >= 0.95
                && unknown_share <= 0.05
                && loss["zero_lost"] == true;
            if !pass {
                failed.push(format!("{name}/profile_qualification"));
            }
            profiles.insert(name.clone(),json!({"passed":pass,"samples":samples,"worker_samples":workers,"worker_share":worker_share,"unknown_samples":unknown,"unknown_share":unknown_share,"zero_lost":loss["zero_lost"],"PID":pid,"raw_sha256":owner["sha256_before"]}));
            symbols.insert(name, json!(bins));
        }
    }
    save(r("observations.json"), &json!(callers));
    save(r("receipts.json"), &json!(receipts));
    save(r("profile-symbols.json"), &json!(symbols));
    save(
        r("analysis.json"),
        &json!({"gate_passed":failed.is_empty(),"failed_checks":failed,"checks":checks,"rows":rows,"all10cases32_aggregate":aggregate,"counters":counters,"profiles":profiles,"production_changed":false,"adopted":false,"goal_achieved":false,"new_SDK_requests":0,"new_proofs":0,"canonical_ledger_count":31387,"caller_processes":1928,"commands":1949,"scope":"complete unweighted reducer caller waves, not SDK proofs or throughput"}),
    );
    println!(
        "Analyzed1949commands/1928callers; gate={} failed={}; no adoption or SDK measurement",
        failed.is_empty(),
        failed.len()
    );
}
fn counters_rounds(case: &str) -> usize {
    if case == "tile511" {
        8192
    } else {
        65536
    }
}
