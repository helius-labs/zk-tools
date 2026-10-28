fn collect() -> (
    BTreeMap<String, Value>,
    BTreeMap<String, Value>,
    BTreeMap<String, Value>,
    BTreeMap<String, Value>,
    u64,
) {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    assert_eq!(read(r("resources/result.json"))["status"], "complete");
    let jobs = read(r("jobs.json"));
    let mut results = BTreeMap::new();
    let mut digests = BTreeMap::new();
    let mut caps = BTreeMap::new();
    let mut stat = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    let mut total_calls = 0u64;
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|l| l.strip_prefix("RESULT "))
            .collect();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        for f in ["kind", "size", "mode", "topology", "worker_count"] {
            assert_eq!(v[f], j[f]);
        }
        assert_eq!(v["rounds"], j["rounds"]);
        assert_eq!(v["prime_calls"], 3);
        assert_eq!(v["heap_instrumented"], false);
        assert_eq!(v["heap"], Value::Null);
        assert_eq!(v["includes_scalar_reset"], true);
        assert_eq!(v["internally_verified"], true);
        assert_eq!(
            v["input_seed"].as_u64().unwrap(),
            j["block"].as_u64().unwrap() + 17
        );
        let count = v["total_MSM_calls"].as_u64().unwrap();
        assert_eq!(count, j["rounds"].as_u64().unwrap());
        total_calls += count;
        let key = format!("{}-{}-{}", j["kind"], j["size"], v["input_seed"]);
        let h = v["output_sha256"].as_str().unwrap();
        if let Some(old) = digests.insert(key, h.to_owned()) {
            assert_eq!(old, h);
        }
        let mut cpus = BTreeSet::new();
        let mut tids = BTreeSet::new();
        for w in v["workers"].as_array().unwrap() {
            assert!(cpus.insert(w["cpu"].as_u64().unwrap()));
            assert!(tids.insert(w["tid"].as_u64().unwrap()));
        }
        assert_eq!(cpus.len(), j["worker_count"].as_u64().unwrap() as usize);
        assert_eq!(
            cpus.into_iter().collect::<Vec<_>>(),
            v["cpus"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_u64().unwrap())
                .collect::<Vec<_>>()
        );
        if j["phase"] == "stat" {
            let text = fs::read_to_string(r(format!("runs/{name}/stderr.log"))).unwrap();
            let mut events = BTreeMap::new();
            for l in text.lines() {
                let f: Vec<_> = l.trim().split(';').collect();
                if f.len() < 5 || !f[2].ends_with(":u") {
                    continue;
                }
                let c: f64 = f[0].trim().parse().expect("counter must count");
                let pct: f64 = f[4].trim().parse().unwrap();
                assert!(pct > 0.);
                events.insert(
                    f[2].to_owned(),
                    json!({"count":c,"per_MSM":c/count as f64,"running_percent":pct}),
                );
            }
            assert_eq!(events.len(), 4);
            let ipc = events["instructions:u"]["count"].as_f64().unwrap()
                / events["cycles:u"]["count"].as_f64().unwrap();
            stat.insert(name.to_owned(),json!({"events":events,"IPC":ipc,"total_MSM_calls":count,"workers":j["worker_count"]}));
        }
        if j["phase"] == "cycles" {
            let text = fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let loss = fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();
            assert!(!loss.contains("PERF_RECORD_LOST"));
            let (mut samples, mut period) = (0u64, 0u64);
            let mut pids = BTreeSet::new();
            let mut observed = BTreeSet::new();
            let mut symbols: BTreeMap<String, u64> = BTreeMap::new();
            for l in text.lines().filter(|l| !l.trim_start().starts_with('#')) {
                let f: Vec<_> = l.split_whitespace().collect();
                if f.len() < 4 {
                    continue;
                }
                let (p, t) = f[0].split_once('/').unwrap();
                let p: u64 = p.parse().unwrap();
                let t: u64 = t.parse().unwrap();
                let weight: u64 = f[1].parse().unwrap();
                pids.insert(p);
                observed.insert(t);
                samples += 1;
                period += weight;
                let s = format!("{:#}", rustc_demangle::demangle(&f[3..].join(" ")));
                *symbols.entry(s).or_default() += weight;
            }
            assert_eq!(pids.len(), 1);
            assert!(samples > 0);
            let sampled_worker_tids: BTreeSet<_> = observed.intersection(&tids).copied().collect();
            assert!(!sampled_worker_tids.is_empty());
            assert!(sampled_worker_tids.len() <= j["worker_count"].as_u64().unwrap() as usize);
            profiles.insert(name.to_owned(),json!({"samples":samples,"period":period,"pids":pids,"sampled_tids":observed,"sampled_worker_tids":sampled_worker_tids,"lost":0,"symbols":symbols,"raw_perf_sha256":hash(r("profiles").join(name).join("perf.data"))}));
        }
        results.insert(name.to_owned(), v);
    }
    for (kind, n) in contexts() {
        for t in ["solo", "physical16", "logical32"] {
            for mode in 0..3 {
                let name = format!("resource-{kind}-{n}-{t}-m{mode}");
                let text = fs::read_to_string(r(format!("resources/{name}/stdout.log"))).unwrap();
                let line = text
                    .lines()
                    .find_map(|l| l.strip_prefix("RESULT "))
                    .unwrap();
                let v: Value = serde_json::from_str(line).unwrap();
                assert_eq!(v["heap_instrumented"], true);
                assert_eq!(v["internally_verified"], true);
                assert_eq!(v["size"], n);
                assert_eq!(v["mode"], mode);
                assert_eq!(v["topology"], t);
                assert_eq!(v["rounds"], 1);
                assert!(v["heap"]["allocation_requests"].as_u64().unwrap() > 0);
                assert!(
                    v["heap"]["peak_live"].as_u64().unwrap()
                        >= v["heap"]["live_before"].as_u64().unwrap()
                );
                caps.insert(name, v);
            }
        }
    }
    (results, stat, profiles, caps, total_calls)
}
