fn rank(ratios: &[f64]) -> Value {
    assert_eq!(ratios.len(), 15);
    assert!(ratios.iter().all(|r| r.is_finite() && *r > 0.));
    let mut s = ratios.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[7],"low":s[3],"high":s[11],"wins":ratios.iter().filter(|&&r|r<1.).count(),"ratios":ratios,"nominal_individual_coverage_if_independent_exchangeable":0.96484375})
}
fn inherited_primary(a: &Value) -> BTreeMap<String, Vec<Value>> {
    assert_eq!(a["gate_passed"], true);
    let mut cells = BTreeMap::new();
    for v in VS {
        let rows = a["cells"][format!("1x2-w32/{v}")]["blocks"]
            .as_array()
            .unwrap();
        assert_eq!(rows.len(), 15);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row["block"], i);
            assert_eq!(row["summary"]["workers"], 32);
            assert_eq!(row["summary"]["rounds"], 151);
            assert_eq!(row["summary"]["priming"], 3);
        }
        cells.insert(format!("1x2-w32/{v}"), rows.clone());
    }
    cells
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    let p = Path::new(PILOT);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(p.join("pins.json"))).unwrap();
    for (file, h) in &pins {
        assert_eq!(hash(file), *h);
    }
    let parent = read(p.join("analysis.json"));
    assert_eq!(parent["gate_passed"], true);
    assert_eq!(parent["independent_proof_count"], 18847);
    assert_eq!(read(p.join("runs/result.json"))["status"], "complete");
    assert_eq!(inherited_primary(&parent).len(), 3);
    let candidate_manifest="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-proof-pgo175/artifacts/candidate-manifest.json";
    let candidate = read(candidate_manifest);
    assert_eq!(candidate["adopted"], false);
    assert_eq!(candidate["primary_latency_pilot_gate_passed"], true);
    assert!(candidate["full_performance_gate_passed"].is_null());
    pin(&mut pins, candidate_manifest);
    let ids = read(p.join("identities.json"));
    for v in VS {
        assert_eq!(
            hash(ids[v]["binary"].as_str().unwrap()),
            ids[v]["sha256"].as_str().unwrap()
        );
        pin(&mut pins, ids[v]["binary"].as_str().unwrap());
    }
    assert_eq!(
        ids["ON"]["sha256"],
        "144621f19b1c3479196a106e6bdb111a542fc2ce1d0311a312a1c1ca46115dfc"
    );
    save(r("identities.json"), &ids);
    let topology = fs::read_to_string(r("topology.csv")).unwrap();
    let mut physical = BTreeSet::new();
    let mut cpus = BTreeSet::new();
    for line in topology.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<_> = line.split(',').collect();
        assert_eq!(f.len(), 4);
        let cpu = f[0].parse::<usize>().unwrap();
        if cpu < 32 {
            assert_eq!(f[3], "Y");
            cpus.insert(cpu);
            if cpu < 16 {
                assert!(physical.insert((f[1].to_string(), f[2].to_string())));
            }
        }
    }
    assert_eq!(cpus.len(), 32);
    assert_eq!(physical.len(), 16);
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(p.join("comparison-prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 18769);
    for (file, h) in read(p.join("proof-inventory.json")).as_object().unwrap() {
        assert_eq!(hash(file), h.as_str().unwrap());
        pin(&mut pins, file);
        if file.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 18847);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let all = read(p.join("preflight-cases.json"));
    assert_eq!(all.as_array().unwrap().len(), 9);
    let mut cases: Vec<Value> = all.as_array().unwrap()[1..].to_vec();
    let mut physical16 = all[0].clone();
    physical16["name"] = json!("1x2-w16");
    physical16["workers"] = json!(16);
    physical16["cpu_affinity"] = json!("0-15");
    cases.push(physical16.clone());
    assert_eq!(cases.len(), 9);
    save(r("cases.json"), &json!(cases));
    save(r("all32-cases.json"), &all);
    let base = read(MANIFEST)["environment"].clone();
    let id_map: BTreeMap<&str, Value> = VS.into_iter().map(|v| (v, ids[v].clone())).collect();
    let mut jobs = vec![];
    let mut steps = vec![];
    let verifier = |j: &Value| {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120})
    };
    for v in VS {
        let j = push(
            &physical16,
            v,
            0,
            true,
            &base,
            &id_map,
            &mut jobs,
            &mut steps,
        );
        steps.push(verifier(&j));
    }
    for b in 0..15 {
        let mut cs = cases.clone();
        let count = cs.len();
        cs.rotate_left(b % count);
        if b % 2 != 0 {
            cs.reverse();
        }
        for c in &cs {
            let mut vs = VS;
            vs.rotate_left(b % 3);
            if b % 2 != 0 {
                vs.reverse();
            }
            for v in vs {
                push(c, v, b, false, &base, &id_map, &mut jobs, &mut steps);
            }
        }
    }
    assert_eq!(jobs.len(), 408);
    for j in jobs.iter().filter(|j| j["preflight"] != true) {
        steps.push(verifier(j));
    }
    assert_eq!(steps.len(), 816);
    assert!(jobs.iter().all(|j| j["case"]["name"] != "1x2-w32"));
    for name in ["ctl", "ack"] {
        assert!(Command::new("mkfifo")
            .arg(r(name))
            .status()
            .unwrap()
            .success());
    }
    for panel in ["arith", "cycles"] {
        for v in VS {
            diagnostic(&physical16, v, panel, &base, &id_map, &mut jobs, &mut steps);
        }
    }
    for j in jobs.iter().filter(|j| j["diagnostic"] == true) {
        steps.push(verifier(j));
    }
    for j in jobs.iter().filter(|j| j["panel"] == "cycles") {
        let name = j["name"].as_str().unwrap();
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["sudo","-n","perf","script","--header","--show-lost-events","-i",j["data"],"-F","pid,tid,period,ip,sym"],"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("evlist-{name}"),"argv":["sudo","-n","perf","evlist","-v","-i",j["data"]],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[r("capture-helper"),"loss",j["data"]],"timeout_seconds":180}));
    }
    assert_eq!(jobs.len(), 414);
    assert_eq!(steps.len(), 837);
    assert_eq!(
        jobs.iter()
            .map(|j| j["rounds"].as_u64().unwrap() + 3)
            .sum::<u64>(),
        20046
    );
    save(r("jobs.json"), &json!(jobs));
    for c in cases.iter() {
        for key in ["image", "request"] {
            pin(&mut pins, c[key].as_str().unwrap());
        }
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
    }
    for f in [
        "analysis.json",
        "identities.json",
        "proof-inventory.json",
        "comparison-prior-proof-sha256.json",
        "code-audit.json",
        "runs/result.json",
        "native/result.json",
        "native-use/result.json",
        "training/result.json",
        "closeout.json",
    ] {
        pin(&mut pins, p.join(f));
    }
    for f in [
        "design.md",
        "bootstrap.rs",
        "qualification.rs",
        "capture.rs",
        "capture-helper",
        "capture-tests",
        "capture-tests.log",
        "topology.csv",
        "pmu-feasibility.log",
        "prior-proof-sha256.json",
        "jobs.json",
        "cases.json",
        "all32-cases.json",
        "identities.json",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [VERIFY, X, MANIFEST] {
        pin(&mut pins, f);
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    fs::write(
        r("host-before.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!("Prepared837steps20046SDK414Gnark;primary32inherited,8other32shapes+physical16new;ledger18847->19261onlyverified");
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let pilot = read(format!("{PILOT}/analysis.json"));
    let mut cells = inherited_primary(&pilot);
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 18847);
    let mut inventory = BTreeMap::new();
    let mut requests = 0usize;
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let text = fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();
        let times: Vec<f64> = text
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| read_sample(l))
            .collect();
        assert_eq!(times.len() as u64, j["rounds"].as_u64().unwrap());
        assert!(times.iter().all(|t| t.is_finite() && *t > 0.));
        let s: Value = serde_json::from_str(
            text.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(s["workers"], j["case"]["workers"]);
        assert_eq!(s["rounds"], j["rounds"]);
        assert_eq!(s["priming"], 3);
        requests += times.len() + 3;
        let verified = fs::read_to_string(r(format!("runs/verify-{name}/stdout.log"))).unwrap();
        assert!(verified.contains("VERIFIED proof_sha256="));
        let c = &j["case"];
        assert!(verified.contains(&format!(
            "image_sha256={}",
            c.get("verification_image_sha256")
                .unwrap_or(&c["image_sha256"])
                .as_str()
                .unwrap()
        )));
        for ext in ["proof", "public"] {
            let p = Path::new(j["directory"].as_str().unwrap()).join(format!("0.{ext}"));
            let h = hash(&p);
            if ext == "proof" {
                assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                assert!(prior.insert(h.clone()));
            } else {
                assert_eq!(h, c["public_sha256"].as_str().unwrap());
            }
            assert!(verified.contains(&format!("{ext}_sha256={h}")));
            inventory.insert(p.display().to_string(), h);
        }
        if j["preflight"] != true && j["diagnostic"] != true {
            let cpu = (s["user_cpu_ms"].as_f64().unwrap() + s["system_cpu_ms"].as_f64().unwrap())
                / times.len() as f64;
            let rss = s["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.;
            cells.entry(format!("{}/{}",c["name"].as_str().unwrap(),j["variant"].as_str().unwrap())).or_default().push(json!({"name":name,"block":j["block"],"median_ms":median(&times),"CPU_ms":cpu,"RSS_MiB":rss,"summary":s,"inherited_primary":false}));
        }
    }
    assert_eq!(requests, 20046);
    assert_eq!(prior.len(), 19261);
    assert_eq!(inventory.len(), 828);
    save(r("proof-inventory.json"), &json!(inventory));
    let mut summaries = BTreeMap::new();
    for (k, rows) in &cells {
        assert_eq!(rows.len(), 15);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row["block"], i);
        }
        let collect = |f: &str| {
            rows.iter()
                .map(|r| r[f].as_f64().unwrap())
                .collect::<Vec<_>>()
        };
        summaries.insert(k.clone(),json!({"median_ms":median(&collect("median_ms")),"CPU_ms":median(&collect("CPU_ms")),"median_RSS_MiB":median(&collect("RSS_MiB")),"max_RSS_MiB":collect("RSS_MiB").into_iter().fold(0.,f64::max),"blocks":rows,"inherited_primary":k.starts_with("1x2-w32/")}));
    }
    assert_eq!(summaries.len(), 30);
    let mut all = read(r("all32-cases.json")).as_array().unwrap().clone();
    all.extend(
        read(r("cases.json"))
            .as_array()
            .unwrap()
            .iter()
            .filter(|c| c["workers"] == 16)
            .cloned(),
    );
    assert_eq!(all.len(), 10);
    let mut pairs = BTreeMap::new();
    let mut checks = BTreeMap::new();
    let mut calibration = BTreeMap::new();
    let mut nonreg = BTreeMap::new();
    let mut failed = vec![];
    let mut refs = BTreeMap::new();
    for c in &all {
        let case = c["name"].as_str().unwrap();
        for (a, b) in [("OFF", "accepted"), ("ON", "OFF"), ("ON", "accepted")] {
            let x = &cells[&format!("{case}/{a}")];
            let y = &cells[&format!("{case}/{b}")];
            let ratios: Vec<_> = (0..15)
                .map(|i| x[i]["median_ms"].as_f64().unwrap() / y[i]["median_ms"].as_f64().unwrap())
                .collect();
            let savings: Vec<_> = (0..15)
                .map(|i| y[i]["median_ms"].as_f64().unwrap() - x[i]["median_ms"].as_f64().unwrap())
                .collect();
            let mut r = rank(&ratios);
            r["paired_saving_ms"] = json!(median(&savings));
            r["inherited_primary"] = json!(case == "1x2-w32");
            let key = format!("{case}/{a}/{b}");
            let pass = if a == "OFF" {
                let p =
                    r["low"].as_f64().unwrap() >= 1. / 1.03 && r["high"].as_f64().unwrap() <= 1.03;
                calibration.insert(key.clone(), p);
                p
            } else if case == "1x2-w32" {
                r["high"].as_f64().unwrap() < 1. && r["wins"].as_u64().unwrap() >= 10
            } else {
                let p = r["high"].as_f64().unwrap() <= if c["workers"] == 16 { 1.02 } else { 1.03 };
                nonreg.insert(key.clone(), p);
                p
            };
            checks.insert(key.clone(), pass);
            if !pass {
                failed.push(key.clone());
            }
            pairs.insert(key, r);
        }
        if c["workers"] == 32 && !c["reference"].is_null() {
            let mut variants = BTreeMap::new();
            for v in VS {
                let ms = summaries[&format!("{case}/{v}")]["median_ms"]
                    .as_f64()
                    .unwrap();
                let cpu = c["reference"]["cpu_ms"].as_f64().unwrap();
                let metal = c["reference"]["metal_ms"].as_f64().unwrap();
                variants.insert(v,json!({"ms":ms,"CPU_speedup":cpu/ms,"Metal_speedup":metal/ms,"CPU10x":ms<=cpu/10.}));
            }
            refs.insert(case,json!({"reference":c["reference"],"variants":variants,"inherited_primary":case=="1x2-w32"}));
        }
    }
    let seven: Vec<&str> = all
        .iter()
        .filter(|c| c["workers"] == 32 && !c["reference"].is_null() && c["shape"] != "1x2")
        .map(|c| c["name"].as_str().unwrap())
        .collect();
    assert_eq!(seven.len(), 7);
    let mut aggregate = BTreeMap::new();
    for b in ["OFF", "accepted"] {
        for include_primary in [false, true] {
            let mut shapes = seven.clone();
            if include_primary {
                shapes.push("1x2-w32");
            }
            let values: Vec<f64> = (0..15)
                .map(|i| {
                    (shapes
                        .iter()
                        .map(|c| {
                            pairs[&format!("{c}/ON/{b}")]["ratios"][i]
                                .as_f64()
                                .unwrap()
                                .ln()
                        })
                        .sum::<f64>()
                        / shapes.len() as f64)
                        .exp()
                })
                .collect();
            let mut r = rank(&values);
            let key = format!(
                "{}/ON/{b}",
                if include_primary {
                    "original8_mixed_phases_descriptive"
                } else {
                    "new7"
                }
            );
            r["used_as_gain_gate"] = json!(!include_primary);
            if !include_primary {
                let pass = r["high"].as_f64().unwrap() < 1. && r["wins"].as_u64().unwrap() >= 10;
                checks.insert(key.clone(), pass);
                if !pass {
                    failed.push(key.clone());
                }
            }
            aggregate.insert(key, r);
        }
    }
    let diagnostics = analyze_diagnostics();
    let n10 = refs
        .values()
        .filter(|r| r["variants"]["ON"]["CPU10x"] == true)
        .count();
    assert_eq!(refs.len(), 8);
    let a = json!({"status":"completed frozenPGO175 broader latency qualification;not adopted","gate_passed":failed.is_empty(),"failed_checks":failed,"checks":checks,"calibration_checks":calibration,"nonregression_checks":nonreg,"pairs":pairs,"cells":summaries,"aggregate":aggregate,"diagnostics_physical16_new":diagnostics,"diagnostics_logical32_inherited":pilot["diagnostics"],"reference_comparisons":refs,"SDK_requests_new":requests,"new_independent_exports":414,"independent_proof_count":19261,"primary32_inherited_not_repeated":true,"primary32_independent_replication":false,"all8_CPU10x_rows":n10,"goal_achieved":n10==8,"goal_status":"active","native_tests_inherited_each":{"field":33,"core":124,"prover":8},"source_files_exact":175,"profile_source_flags_binary_changed":false,"adopted":false,"production_changed":false,"compatibility_resource_final_artifact_pending":true,"parent_pilot":format!("{PILOT}/analysis.json"),"parent_pilot_sha256":hash(format!("{PILOT}/analysis.json")),"scope":"new8other32shapes+physical16,15blocks;primary32pilotinherited;new7aggregategain;all10calnonreg;no waivers or further qualification"});
    save(r("analysis.json"), &a);
    fs::write(
        r("host-after.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!(
        "FULLGATE={}failures={};20046SDK414Gnarkledger19261;CPU10xrows={n10}",
        a["gate_passed"], a["failed_checks"]
    );
    for (k, c) in summaries {
        println!(
            "{k} {:.6}ms CPU{:.3} RSS{:.2}/{:.2} inherited={}",
            c["median_ms"].as_f64().unwrap(),
            c["CPU_ms"].as_f64().unwrap(),
            c["median_RSS_MiB"].as_f64().unwrap(),
            c["max_RSS_MiB"].as_f64().unwrap(),
            c["inherited_primary"]
        );
    }
    for (k, a) in aggregate {
        println!(
            "{k} {}[{},{}]wins{}",
            a["median"], a["low"], a["high"], a["wins"]
        );
    }
}
fn read_sample(s: &str) -> f64 {
    serde_json::from_str::<Value>(s).unwrap()["ms"]
        .as_f64()
        .unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paired_ranks_preserve_ties_and_all_observations() {
        let ratios = vec![0.9; 15];
        let r = rank(&ratios);
        assert_eq!(r["median"], 0.9);
        assert_eq!(r["low"], 0.9);
        assert_eq!(r["high"], 0.9);
        assert_eq!(r["wins"], 15);
        assert_eq!(r["ratios"].as_array().unwrap().len(), 15);
        let r = rank(&(1..=15).rev().map(|n| n as f64 / 10.).collect::<Vec<_>>());
        assert_eq!(r["median"], 0.8);
        assert_eq!(r["low"], 0.4);
        assert_eq!(r["high"], 1.2);
        assert_eq!(r["wins"], 9);
    }
    #[test]
    fn inherited_context_requires_exact_workers_rounds_and_block_ids() {
        let a = read(format!("{PILOT}/analysis.json"));
        assert_eq!(inherited_primary(&a).len(), 3);
        for field in ["block", "workers", "rounds"] {
            let mut bad = a.clone();
            let row = &mut bad["cells"]["1x2-w32/ON"]["blocks"][0];
            if field == "block" {
                row["block"] = json!(1);
            } else {
                row["summary"][field] = json!(17);
            }
            assert!(std::panic::catch_unwind(|| inherited_primary(&bad)).is_err());
        }
    }
}
