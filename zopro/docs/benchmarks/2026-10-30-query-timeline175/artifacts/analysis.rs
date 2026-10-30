fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let jobs = read(r("jobs.json"));
    let mut ledger: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(ledger.len(), 24943);
    let mut rows = vec![];
    let mut inventory = vec![];
    let mut vk = None;
    let mut all_requests = 0u64;
    for j in jobs.as_array().unwrap() {
        let n = j["name"].as_str().unwrap();
        let log = fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap();
        let samples: Vec<Value> = log
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        let timeline: Vec<Value> = log
            .lines()
            .filter_map(|l| l.strip_prefix("TIMELINE "))
            .map(|s| serde_json::from_str(s).unwrap())
            .collect();
        let usage: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        let count = j["rounds"].as_u64().unwrap() as usize;
        assert_eq!(samples.len(), count);
        assert_eq!(usage["rounds"], count);
        assert_eq!(usage["priming"], 3);
        assert_eq!(usage["workers"], j["case"]["workers"]);
        all_requests += count as u64 + 3;
        for (i, s) in samples.iter().enumerate() {
            assert_eq!(s["round"], i);
            assert!(s["ms"].as_f64().unwrap() > 0.);
        }
        if j["variant"] == "traced" {
            assert_eq!(timeline.len(), count);
            for (i, t) in timeline.iter().enumerate() {
                assert_eq!(t["round"], i);
                let x: Vec<u64> = serde_json::from_value(t["offsets_ns"].clone()).unwrap();
                assert_eq!(x.len(), 13);
                assert!(x[..7].windows(2).all(|p| p[0] <= p[1]));
                assert!(x[7..]
                    .chunks_exact(2)
                    .all(|q| x[4] <= q[0] && q[0] <= q[1] && q[1] <= x[5]));
                assert!((x[6] as f64) <= samples[i]["ms"].as_f64().unwrap() * 1e6 + 1.);
            }
        } else {
            assert!(timeline.is_empty());
        }
        let p = Path::new(j["directory"].as_str().unwrap());
        let proof = p.join("0.proof");
        let public = p.join("0.public");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        let ph = hash(proof);
        assert!(ledger.insert(ph.clone()));
        let public_h = hash(public);
        assert_eq!(public_h, j["case"]["public_sha256"]);
        let receipt = fs::read_to_string(r(format!("runs/verify-{n}/stdout.log"))).unwrap();
        let verified: Vec<_> = receipt
            .lines()
            .filter_map(|l| l.strip_prefix("VERIFIED "))
            .collect();
        assert_eq!(verified.len(), 1);
        let f: BTreeMap<_, _> = verified[0]
            .split_whitespace()
            .map(|s| s.split_once('=').unwrap())
            .collect();
        assert_eq!(f["proof_sha256"], ph);
        assert_eq!(f["public_sha256"], public_h);
        assert_eq!(f["image_sha256"], j["case"]["image_sha256"]);
        assert_eq!(f["vk_sha256"].len(), 64);
        if let Some(previous) = vk.replace(f["vk_sha256"].to_string()) {
            assert_eq!(previous, f["vk_sha256"]);
        }
        inventory.push(json!({"name":n,"proof_sha256":ph,"public_sha256":public_h,"image_sha256":j["case"]["image_sha256"],"verified":true}));
        rows.push(json!({"name":n,"case":j["case"]["name"],"variant":j["variant"],"panel":j["panel"],"block":j["block"],"median_ms":median(&samples.iter().map(|s|s["ms"].as_f64().unwrap()).collect::<Vec<_>>()),"CPU_ms":(usage["user_cpu_ms"].as_f64().unwrap()+usage["system_cpu_ms"].as_f64().unwrap())/count as f64,"RSS_MiB":usage["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"samples":samples,"timeline":timeline,"usage":usage}));
    }
    assert_eq!(all_requests, 43002);
    assert_eq!(ledger.len(), 25231);
    assert_eq!(inventory.len(), 288);
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let mut stages = BTreeMap::new();
    let mut qualified = BTreeMap::new();
    let mut failed = vec![];
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap().to_string();
        let mut times = BTreeMap::new();
        for v in ["accepted", "OFF", "traced"] {
            let mut selected: Vec<_> = rows
                .iter()
                .filter(|row| {
                    row["case"] == case && row["variant"] == v && row["panel"] == "latency"
                })
                .collect();
            selected.sort_by_key(|row| row["block"].as_u64().unwrap());
            assert_eq!(selected.len(), 31);
            for (b, row) in selected.iter().enumerate() {
                assert_eq!(row["block"], b);
            }
            let ms: Vec<_> = selected
                .iter()
                .map(|row| row["median_ms"].as_f64().unwrap())
                .collect();
            cells.insert(format!("{case}/{v}"),json!({"median_ms":median(&ms),"CPU_ms":median(&selected.iter().map(|row|row["CPU_ms"].as_f64().unwrap()).collect::<Vec<_>>()),"max_RSS_MiB":selected.iter().map(|row|row["RSS_MiB"].as_f64().unwrap()).fold(0.,f64::max),"process_medians":ms}));
            times.insert(v, ms);
        }
        let mut ok = true;
        for (n, d) in [("OFF", "accepted"), ("traced", "OFF")] {
            let ratios: Vec<_> = times[n].iter().zip(&times[d]).map(|(n, d)| n / d).collect();
            let s = stats(&ratios);
            let pass =
                s["low"].as_f64().unwrap() >= 1. / 1.01 && s["high"].as_f64().unwrap() <= 1.01;
            let key = format!("{case}/{n}/{d}");
            if !pass {
                ok = false;
                failed.push(key.clone());
            }
            pairs.insert(key, json!({"statistics":s,"within_factor1_01":pass}));
        }
        qualified.insert(case.clone(), ok);
        let records: Vec<_> = rows
            .iter()
            .filter(|row| {
                row["case"] == case && row["variant"] == "traced" && row["panel"] == "latency"
            })
            .flat_map(|row| row["timeline"].as_array().unwrap())
            .map(|t| serde_json::from_value::<[u64; 13]>(t["offsets_ns"].clone()).unwrap())
            .collect();
        assert_eq!(records.len(), 31 * 151);
        let mut metrics = BTreeMap::<&str, Vec<f64>>::new();
        let mut last = [0usize; 3];
        let mut ties = 0;
        for x in &records {
            for (name, value) in [
                ("solver", x[0]),
                ("public_correction_and_release", x[1] - x[0]),
                ("canonical_conversion", x[2] - x[1]),
                ("scalar_extraction", x[3] - x[2]),
                ("product", x[4] - x[3]),
                ("prefix_to_product_end", x[4]),
                ("query_join", x[5] - x[4]),
                ("query_A_inclusive", x[8] - x[7]),
                ("query_KH_inclusive", x[10] - x[9]),
                ("query_B2_inclusive", x[12] - x[11]),
                ("postjoin_output", x[6] - x[5]),
                ("canonical_span", x[6]),
                ("join_after_last_query", x[5] - x[8].max(x[10]).max(x[12])),
            ] {
                metrics.entry(name).or_default().push(value as f64 / 1e6);
            }
            let ends = [x[8], x[10], x[12]];
            let max = *ends.iter().max().unwrap();
            let winners: Vec<_> = ends
                .iter()
                .enumerate()
                .filter(|(_, v)| **v == max)
                .map(|(i, _)| i)
                .collect();
            if winners.len() == 1 {
                last[winners[0]] += 1
            } else {
                ties += 1;
            }
            let mut sorted = ends;
            sorted.sort();
            metrics
                .entry("last_minus_second_query_end")
                .or_default()
                .push((sorted[2] - sorted[1]) as f64 / 1e6);
        }
        stages.insert(case,json!({"interpret_as_unperturbed":ok,"requests":records.len(),"median_intervals_ms":metrics.iter().map(|(k,v)|(*k,median(v))).collect::<BTreeMap<_,_>>(),"last_query":{"A":last[0],"KH":last[1],"B2":last[2],"ties":ties},"scope":"per-request wall intervals, nested/parallel spans overlap; not scheduler-ready timestamps, exclusive CPU costs or removable proof wall time"}));
    }
    save(r("rows.json"), &json!(rows));
    save(r("proof-inventory.json"), &json!(inventory));
    save(r("proof-sha256-after.json"), &json!(ledger));
    save(
        r("analysis.json"),
        &json!({"complete":true,"SDK_requests":all_requests,"new_independent_exports":288,"prior_proof_count":24943,"independent_proof_count":25231,"cells":cells,"pairs":pairs,"stages":stages,"topology_qualified":qualified,"overhead_failed_checks":failed,"source_files":176,"originals_exact":174,"reverse175_exact":true,"native_core_tests":126,"native_prover_tests":8,"new_PMUs":0,"new_sampling_profiles":0,"profiling":"stage timestamps; existing flat profiles retained as motivation","adopted":false,"production_changed":false,"goal_achieved":false}),
    );
    println!("Analyzed fixed31block timelines; distinctproofledger25231; overhead failures{}; production unchanged",failed.len());
}
