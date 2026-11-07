use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-prefix-replay176";
const X: &str = "/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const ARMS: [&str; 3] = ["accepted", "OFF", "ON"];
fn p(s: &str) -> String {
    format!("{R}/primary-pilot/{s}")
}
fn load(s: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(s).unwrap()).unwrap()
}
fn save(s: impl AsRef<Path>, v: &Value) {
    assert!(
        !s.as_ref().exists(),
        "refuse overwrite {}",
        s.as_ref().display()
    );
    fs::write(s, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(s: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(s).unwrap();
    let mut d = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        d.update(&b[..n]);
    }
    format!("{:x}", d.finalize())
}
fn step(name: &str, argv: Value, env: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":900})
}
fn prepare() {
    fs::create_dir_all(p("proofs")).unwrap();
    fs::create_dir_all(p("stat")).unwrap();
    fs::create_dir_all(p("perf")).unwrap();
    let identities = load(format!("{R}/sdk-identities.json"));
    let previous = load(format!("{R}/preflight-plan.json"));
    let case = &previous["steps"][1];
    let image = case["argv"][4].as_str().unwrap();
    let request = case["argv"][5].as_str().unwrap();
    let env = case["environment"].clone();
    let prejobs = load(format!("{R}/preflight-jobs.json"));
    let binding = prejobs
        .as_array()
        .unwrap()
        .iter()
        .find(|j| j["shape"] == "1x2")
        .unwrap()
        .clone();
    let orders = [
        [0, 1, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
        [0, 2, 1],
        [1, 0, 2],
    ];
    let protocol = json!({"primary":"1x2","workers":32,"cpus":"0-31","blocks":15,"rounds":151,"primes":3,"orders":orders,"unit":"process arithmetic mean of 151 full SDK requests","paired_interval":"sorted block ratios indices3..11 (ranks4..12), nominal96.484375% marginal median coverage conditional on independent exchangeable blocks","gain_gate":"ON upperratio<1 and at least10/15wins vs BOTHcontrols","calibration_gate":"OFF/accepted lower>=1/1.03 and upper<=1.03","no_exclusions_extensions_rerolls_gain_floor":true,"latency_requests":6930,"latency_exports":45,"diagnostic_requests":864,"diagnostic_exports":6,"profile_event":"cycles:u","sample_period":16000000,"mmap_pages":512,"diagnostic_rounds":{"stat":31,"cycles":251},"positive_pilot_selects_all8_plus_merge54_only":true,"baseline_ledger":format!("{R}/proof-sha256-after-preflight.json"),"baseline_ledger_count":31492,"binding":binding,"environment":env,"identities":identities});
    save(p("protocol.json"), &protocol);
    let mut lat = vec![];
    let mut latverify = vec![];
    let mut diag = vec![];
    let mut diagverify = vec![];
    let mut offline = vec![];
    let mut jobs = vec![];
    let verify_job = |name: &str| {
        step(
            &format!("verify-{name}"),
            json!([
                VERIFY,
                image,
                binding["image_sha256"],
                format!("{}/0.proof", p(&format!("proofs/{name}"))),
                format!("{}/0.public", p(&format!("proofs/{name}"))),
                binding["public_sha256"]
            ]),
            json!({}),
        )
    };
    for block in 0..15 {
        for idx in orders[block % 6] {
            let arm = ARMS[idx];
            let name = format!("lat-b{block:02}-{arm}");
            lat.push(step(
                &name,
                json!([
                    "taskset",
                    "-c",
                    "0-31",
                    identities[arm]["binary"],
                    image,
                    request,
                    "151",
                    p(&format!("proofs/{name}"))
                ]),
                env.clone(),
            ));
            latverify.push(verify_job(&name));
            jobs.push(json!({"name":name,"arm":arm,"block":block,"stage":"latency","rounds":151}));
        }
    }
    let old = "/home/ubuntu/src/zopro-perf-work/window-batch-proof-capture176";
    let oldplan = load(format!("{old}/plan.json"));
    for (kind, rounds) in [("stat", 31), ("cycles", 251)] {
        for arm in ARMS {
            let template = oldplan["steps"]
                .as_array()
                .unwrap()
                .iter()
                .find(|s| s["name"] == format!("{kind}-1x2-w32-accepted"))
                .unwrap();
            let name = format!("{kind}-{arm}");
            let mut s = template.clone();
            s["name"] = name.clone().into();
            let a = s["argv"].as_array_mut().unwrap();
            for v in a.iter_mut() {
                let oldval = v.as_str().unwrap();
                let mut value = oldval.replace(old, &p(""));
                value = value
                    .replace("stat-1x2-w32-accepted", &name)
                    .replace("cycles-1x2-w32-accepted", &name);
                if oldval == identities["accepted"]["binary"].as_str().unwrap() {
                    value = identities[arm]["binary"].as_str().unwrap().to_string();
                }
                *v = value.into();
            }
            diag.push(s);
            diagverify.push(verify_job(&name));
            jobs.push(
                json!({"name":name,"arm":arm,"stage":"diagnostic","kind":kind,"rounds":rounds}),
            );
            if kind == "cycles" {
                let data = p(&format!("perf/{name}.data"));
                offline.push(step(
                    &format!("own-{name}"),
                    json!(["sudo", "-n", "chown", "ubuntu:ubuntu", data]),
                    json!({}),
                ));
                offline.push(step(
                    &format!("decode-{name}"),
                    json!([
                        "perf",
                        "script",
                        "--header",
                        "--ns",
                        "-i",
                        data,
                        "-F",
                        "pid,tid,cpu,time,period,ip,sym"
                    ]),
                    json!({}),
                ));
                offline.push(step(
                    &format!("events-{name}"),
                    json!(["perf", "evlist", "-v", "-i", data]),
                    json!({}),
                ));
                offline.push(step(
                    &format!("metadata-{name}"),
                    json!([
                        "perf",
                        "script",
                        "--header",
                        "--show-mmap-events",
                        "--show-task-events",
                        "--ns",
                        "-i",
                        data,
                        "-F",
                        "pid,tid,cpu,time,period,ip,sym"
                    ]),
                    json!({}),
                ));
                offline.push(step(
                    &format!("loss-{name}"),
                    json!([format!("{old}/campaign"), "loss", data]),
                    json!({}),
                ));
            }
        }
    }
    lat.extend(latverify);
    diag.extend(diagverify);
    diag.extend(offline);
    save(p("jobs.json"), &json!(jobs));
    let mut pins = load(format!("{R}/preflight-review-pins.json"));
    for f in [
        format!("{R}/pilot.rs"),
        format!("{R}/pilot"),
        p("protocol.json"),
        p("jobs.json"),
        format!("{R}/sdk-identities.json"),
        format!("{R}/preflight-review.json"),
        format!("{R}/proof-sha256-after-preflight.json"),
        format!("{old}/campaign"),
        format!("{old}/campaign.rs"),
        X.into(),
    ] {
        pins[&f] = hash(&f).into();
    }
    save(p("pins.json"), &pins);
    for (name, steps) in [
        ("latency", lat),
        ("diagnostic", diag),
        (
            "review-latency",
            vec![step(
                "review-latency",
                json!([format!("{R}/pilot"), "latency"]),
                json!({}),
            )],
        ),
        (
            "review-diagnostic",
            vec![step(
                "review-diagnostic",
                json!([format!("{R}/pilot"), "diagnostic"]),
                json!({}),
            )],
        ),
    ] {
        save(
            p(&format!("{name}-plan.json")),
            &json!({"version":1,"source_pins":p("pins.json"),"environment":{},"steps":steps}),
        );
    }
    // Exercise rank endpoints and exact identity ratio before any capture.
    let v = vec![1.; 15];
    let t = paired(&v, &v);
    assert_eq!(t["lower"], 1.);
    assert_eq!(t["upper"], 1.);
    assert_eq!(t["wins"], 0);
    println!("prepared45latency+45verification;6diagnostic+6verification+15offline;no recording launched");
}
fn receipts(stage: &str) -> usize {
    let plan = load(p(&format!("{stage}-plan.json")));
    let result = load(p(&format!("{stage}-runs/result.json")));
    assert_eq!(result["status"], "complete");
    let steps = plan["steps"].as_array().unwrap();
    let complete = load(p(&format!("{stage}-runs/completed.json")));
    assert_eq!(complete.as_array().unwrap().len(), steps.len());
    for s in steps {
        let base = p(&format!("{stage}-runs/{}", s["name"].as_str().unwrap()));
        let exit = load(format!("{base}/exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let c = load(format!("{base}/command.json"));
        let a = c["argv"].as_array().unwrap();
        let b = s["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - b.len()..], b);
        if let Some(env) = s["environment"].as_object() {
            for (key, value) in env {
                assert_eq!(&c["environment"][key], value);
            }
        }
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{base}/{stream}.log")),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
    }
    steps.len()
}
fn observations(stage: &str, prior: &str, count: usize) -> Vec<Value> {
    let jobs = load(p("jobs.json"));
    let protocol = load(p("protocol.json"));
    let binding = &protocol["binding"];
    let mut ledger: BTreeSet<String> = serde_json::from_value(load(prior)).unwrap();
    assert_eq!(ledger.len(), count);
    let mut rows = vec![];
    let preflight = load(format!("{R}/preflight-review.json"));
    let vk = preflight["results"]
        .as_array()
        .map(|v| v.iter().find(|v| v["shape"] == "1x2").unwrap()["vk_sha256"].clone());
    for j in jobs
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["stage"] == stage)
    {
        let name = j["name"].as_str().unwrap();
        let out = fs::read_to_string(p(&format!("{stage}-runs/{name}/stdout.log"))).unwrap();
        let samples: Vec<Value> = out
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let rounds = j["rounds"].as_u64().unwrap() as usize;
        assert_eq!(samples.len(), rounds);
        let mut times = vec![];
        for (i, s) in samples.iter().enumerate() {
            assert_eq!(s["round"], i);
            let ms = s["ms"].as_f64().unwrap();
            assert!(ms.is_finite() && ms > 0.);
            times.push(ms);
        }
        let sums: Vec<Value> = out
            .lines()
            .filter_map(|l| l.strip_prefix("SUMMARY "))
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(sums.len(), 1);
        let summary = &sums[0];
        assert_eq!(summary["rounds"], rounds);
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["workers"], 32);
        let mean = times.iter().sum::<f64>() / rounds as f64;
        let proof = p(&format!("proofs/{name}/0.proof"));
        let public = p(&format!("proofs/{name}/0.public"));
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        let ph = hash(proof);
        let pu = hash(public);
        assert_eq!(pu, binding["public_sha256"].as_str().unwrap());
        let vo = fs::read_to_string(p(&format!("{stage}-runs/verify-{name}/stdout.log"))).unwrap();
        let lines: Vec<_> = vo
            .lines()
            .filter_map(|l| l.strip_prefix("VERIFIED "))
            .collect();
        assert_eq!(lines.len(), 1);
        let fields: BTreeMap<_, _> = lines[0]
            .split_whitespace()
            .map(|s| s.split_once('=').unwrap())
            .collect();
        assert_eq!(fields["proof_sha256"], ph);
        assert_eq!(fields["public_sha256"], pu);
        assert_eq!(
            fields["image_sha256"],
            binding["image_sha256"].as_str().unwrap()
        );
        assert_eq!(fields["vk_sha256"].len(), 64);
        if let Some(ref v) = vk {
            assert_eq!(fields["vk_sha256"], v.as_str().unwrap());
        }
        assert!(ledger.insert(ph.clone()), "nonfresh {name}");
        rows.push(json!({"job":j,"mean_ms":mean,"samples_ms":times,"summary":summary,"proof_sha256":ph,"public_sha256":pu,"vk_sha256":fields["vk_sha256"],"image_sha256":fields["image_sha256"],"independently_verified":true,"fresh":true}));
    }
    let expected = if stage == "latency" { 45 } else { 6 };
    assert_eq!(rows.len(), expected);
    assert_eq!(ledger.len(), count + expected);
    save(
        p(&format!("proof-sha256-after-{stage}.json")),
        &json!(ledger),
    );
    rows
}
fn median(a: &[f64]) -> f64 {
    let mut s = a.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn paired(a: &[f64], b: &[f64]) -> Value {
    assert_eq!(a.len(), 15);
    assert_eq!(b.len(), 15);
    let mut r: Vec<_> = a.iter().zip(b).map(|(a, b)| a / b).collect();
    let wins = r.iter().filter(|x| **x < 1.).count();
    r.sort_by(f64::total_cmp);
    json!({"median":r[7],"lower":r[3],"upper":r[11],"wins":wins,"sorted_ratios":r})
}
fn latency() {
    let receipt_count = receipts("latency");
    let rows = observations(
        "latency",
        &format!("{R}/proof-sha256-after-preflight.json"),
        31492,
    );
    let mut means = BTreeMap::new();
    let mut arm_summary = BTreeMap::new();
    for arm in ARMS {
        let a: Vec<f64> = (0..15)
            .map(|block| {
                rows.iter()
                    .find(|r| r["job"]["arm"] == arm && r["job"]["block"] == block)
                    .unwrap()["mean_ms"]
                    .as_f64()
                    .unwrap()
            })
            .collect();
        let rss: Vec<f64> = rows
            .iter()
            .filter(|r| r["job"]["arm"] == arm)
            .map(|r| r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.)
            .collect();
        arm_summary.insert(arm,json!({"median_process_mean_ms":median(&a),"median_whole_process_maxrss_MiB":median(&rss),"max_whole_process_maxrss_MiB":rss.iter().copied().fold(0.,f64::max)}));
        means.insert(arm, a);
    }
    let off = paired(&means["OFF"], &means["accepted"]);
    let onoff = paired(&means["ON"], &means["OFF"]);
    let onaccepted = paired(&means["ON"], &means["accepted"]);
    let calibration =
        off["lower"].as_f64().unwrap() >= 1. / 1.03 && off["upper"].as_f64().unwrap() <= 1.03;
    let gain = |r: &Value| r["upper"].as_f64().unwrap() < 1. && r["wins"].as_u64().unwrap() >= 10;
    let passed = calibration && gain(&onoff) && gain(&onaccepted);
    save(
        p("latency-analysis.json"),
        &json!({"receipt_count":receipt_count,"rows":rows,"arms":arm_summary,"OFF_over_accepted":off,"ON_over_OFF":onoff,"ON_over_accepted":onaccepted,"calibration_passed":calibration,"pilot_gate_passed":passed,"SDK_requests":6930,"fresh_independent_exports":45,"canonical_ledger_count":31537,"production_adopted":false,"goal_achieved":false,"interval_scope":"nominal96.484375% marginal median under independent exchangeable blocks; not familywise; no exclusions/extensions/reroll"}),
    );
    println!(
        "{}",
        json!({"pilot_gate_passed":passed,"calibration":calibration,"arms":arm_summary,"ON_over_OFF":onoff,"ON_over_accepted":onaccepted})
    );
}
fn diagnostic() {
    let receipt_count = receipts("diagnostic");
    let rows = observations("diagnostic", &p("proof-sha256-after-latency.json"), 31537);
    let mut stats = BTreeMap::new();
    let mut profiles = BTreeMap::new();
    for arm in ARMS {
        let csv = fs::read_to_string(p(&format!("stat/stat-{arm}.csv"))).unwrap();
        let mut counts = BTreeMap::new();
        for l in csv.lines().filter(|l| !l.is_empty() && !l.starts_with('#')) {
            let f: Vec<_> = l.split(';').collect();
            assert!(f.len() >= 5);
            let value = f[0].parse::<u64>().unwrap();
            let running = f[4].parse::<f64>().unwrap();
            assert!(running >= 99.9 && running <= 100.01);
            counts.insert(
                f[2],
                json!({"value":value,"per_request":value as f64/31.,"running_percent":running}),
            );
        }
        assert_eq!(counts.len(), 4);
        stats.insert(arm, json!(counts));
        let name = format!("cycles-{arm}");
        let data = p(&format!("perf/{name}.data"));
        let base = p(&format!("diagnostic-runs"));
        let loss = load(format!("{base}/loss-{name}/stdout.log"));
        assert_eq!(loss["zero_lost"], true);
        let events = fs::read_to_string(format!("{base}/events-{name}/stdout.log")).unwrap();
        assert!(
            events.contains("cycles:u")
                && events.contains("16000000")
                && events.contains("CPU|PERIOD")
                && events.contains("exclude_kernel: 1")
                && events.contains("inherit: 1")
        );
        let metadata = fs::read_to_string(format!("{base}/metadata-{name}/stdout.log")).unwrap();
        assert!(metadata.contains("PERF_RECORD_MMAP") && metadata.contains("PERF_RECORD_COMM"));
        assert!(metadata.contains(
            load(format!("{R}/sdk-identities.json"))[arm]["binary"]
                .as_str()
                .unwrap()
        ));
        let decoded = fs::read_to_string(format!("{base}/decode-{name}/stdout.log")).unwrap();
        assert!(
            decoded.contains("AMD Ryzen 9 9950X")
                && decoded.contains("sample_period, sample_freq } = 16000000")
        );
        let mut n = 0u64;
        let mut unknown = 0;
        let mut pids = BTreeSet::new();
        let mut tids = BTreeSet::new();
        let mut cpus = BTreeSet::new();
        let mut syms = BTreeMap::<String, u64>::new();
        let mut period_total = 0u64;
        for l in decoded
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
        {
            let f: Vec<_> = l.split_whitespace().collect();
            assert!(f.len() >= 6, "bad sample {l}");
            let (pid, tid) = f[0].split_once('/').unwrap();
            let pid = pid.parse::<u32>().unwrap();
            let tid = tid.parse::<u32>().unwrap();
            let cpu = f[1].trim_matches(&['[', ']'][..]).parse::<u32>().unwrap();
            assert!(cpu < 32);
            let time = f[2].trim_end_matches(':').parse::<f64>().unwrap();
            assert!(time > 0.);
            let period = f[3].parse::<u64>().unwrap();
            assert_eq!(period, 16000000);
            let _ip = u64::from_str_radix(f[4].trim_start_matches("0x"), 16).unwrap();
            let sym = f[5..].join(" ");
            if sym.contains("unknown") || sym == "[unknown]" {
                unknown += 1;
            }
            *syms.entry(sym).or_default() += 1;
            n += 1;
            period_total += period;
            pids.insert(pid);
            tids.insert(tid);
            cpus.insert(cpu);
        }
        assert!(n > 100);
        assert_eq!(pids.len(), 1);
        let mut ranked: Vec<_> = syms.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1));
        profiles.insert(arm,json!({"data_sha256":hash(data),"samples":n,"period_total":period_total,"unknown_samples":unknown,"unknown_percent":100.*unknown as f64/n as f64,"pids":pids,"tids":tids,"cpus":cpus,"zero_LOST":true,"header_event_metadata_checks_passed":true,"top_symbols":ranked.into_iter().take(25).collect::<Vec<_>>(),"limits":"period-weighted sampled user cycles; no exclusive wall time, exact IP/ELF mapping or DRAM causality claim"}));
    }
    save(
        p("diagnostic-analysis.json"),
        &json!({"receipt_count":receipt_count,"rows":rows,"stats":stats,"profiles":profiles,"SDK_requests":864,"fresh_independent_exports":6,"canonical_ledger_count":31543,"sampling_claims_limited":true}),
    );
    println!("diagnostic review passed;6fresh exports;ledger31543");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("latency") => latency(),
        Some("diagnostic") => diagnostic(),
        _ => panic!("prepare|latency|diagnostic"),
    }
}
