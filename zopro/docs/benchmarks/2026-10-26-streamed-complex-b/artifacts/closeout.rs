use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::Command,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
fn receipt(p: impl AsRef<Path>) {
    let p = p.as_ref();
    let exit = read(p.join("exit.json"));
    assert_eq!(exit["success"], true, "{}", p.display());
    assert_eq!(exit["returncode"], 0);
    for stream in ["stdout", "stderr"] {
        assert_eq!(
            hash(p.join(format!("{stream}.log"))),
            exit[format!("{stream}_sha256")].as_str().unwrap()
        );
    }
}
fn row(p: impl AsRef<Path>) -> Value {
    let s = fs::read_to_string(p).unwrap();
    let rows = s
        .lines()
        .filter_map(|l| l.strip_prefix("ROW "))
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 1);
    serde_json::from_str(rows[0]).unwrap()
}
fn main() {
    let r = Path::new("/tmp/zopro-streamed-complex-b");
    let pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("profile-input-pins.json"))).unwrap();
    assert_eq!(pins.len(), 218);
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for stage in ["build-v2", "timing", "profiles"] {
        assert_eq!(
            read(r.join(stage).join("result.json"))["status"],
            "complete"
        );
    }
    receipt(r.join("build/field-tests"));
    assert_eq!(
        read(r.join("build/field-tests/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        33
    );
    receipt(r.join("build-v2/component-oracles"));
    assert_eq!(
        read(r.join("build-v2/component-oracles/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    let oracle = fs::read_to_string(r.join("build-v2/component-oracles/stdout.log")).unwrap();
    let mut raw = 0u64;
    let mut chains = 0u64;
    let mut max = 0i128;
    for l in oracle.lines() {
        let f = l
            .split_whitespace()
            .filter_map(|w| w.split_once('='))
            .collect::<BTreeMap<_, _>>();
        if l.starts_with("EXACT_STREAMED ") {
            raw += f["outputs"].parse::<u64>().unwrap();
            max = max.max(f["max_coefficient"].parse().unwrap());
            assert_eq!(f["bound"], "144115188075855872");
        }
        if l.starts_with("ARK_STREAMED ") {
            chains += f["chain_outputs"].parse::<u64>().unwrap();
        }
    }
    assert_eq!(raw, 91136);
    assert_eq!(chains, 729088);
    assert!(max < 32 * (1i128 << 52));
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    assert_eq!(base["source_files_sha256"].as_object().unwrap().len(), 170);
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(
        hash("/tmp/zopro-grouping-module/native/candidate-sdk"),
        "03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d"
    );
    fs::copy(
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        r.join("accepted-manifest.json"),
    )
    .unwrap();
    let order = read(r.join("timing-order.json"));
    let mut seen = BTreeSet::new();
    for v in order.as_array().unwrap() {
        let name = v["name"].as_str().unwrap();
        assert!(seen.insert(name));
        let p = r.join("timing").join(name);
        receipt(&p);
        let cmd = read(p.join("command.json"));
        let argv = cmd["argv"].as_array().unwrap();
        let fields = [
            "taskset",
            "-c",
            "2",
            if v["variant"] == "accepted" {
                r.join("accepted-probe")
            } else {
                r.join("target/release/probe")
            }
            .to_str()
            .unwrap(),
            v["mode"].as_str().unwrap(),
            v["pattern"].as_str().unwrap(),
            "100000",
        ]
        .map(|s| json!(s));
        assert!(argv.ends_with(&fields));
    }
    assert_eq!(seen.len(), 496);
    assert_eq!(
        read(r.join("component-result.json"))["raw_word_comparisons_vs_three_controls"],
        41664
    );
    let variants = [
        ("accepted", r.join("accepted-probe"), 0),
        ("fresh", r.join("target/release/probe"), 0),
        ("shared", r.join("target/release/probe"), 1),
        ("streamed", r.join("target/release/probe"), 2),
    ];
    let mut records = Vec::new();
    let mut outputs = Vec::new();
    let mut total = 0;
    for (label, binary, strategy) in variants {
        for step in ["capture", "decode", "loss"] {
            receipt(r.join("profiles").join(format!("{step}-{label}")));
        }
        let capture = r.join(format!("profiles/capture-{label}"));
        let got = row(capture.join("stdout.log"));
        assert_eq!(got["strategy"], strategy);
        assert_eq!(got["iterations"], 20000000);
        assert_eq!(got["primes"], 3);
        assert_eq!(got["pattern"], "affine-expression");
        assert_eq!(got["output_words"].as_array().unwrap().len(), 64);
        outputs.push(got["output_words"].clone());
        let cmd = read(capture.join("command.json"));
        let argv = cmd["argv"].as_array().unwrap();
        assert!(argv.iter().any(|a| a == "--no-buildid-cache"));
        assert_eq!(argv.last().unwrap(), "20000000");
        let o = Command::new("nm").arg("-S").arg(&binary).output().unwrap();
        assert!(o.status.success());
        let plain = String::from_utf8(o.stdout).unwrap();
        let plain = plain
            .lines()
            .filter_map(|l| {
                let f = l.split_whitespace().collect::<Vec<_>>();
                (f.len() == 4).then(|| (f[0].to_owned(), f[3].to_owned()))
            })
            .collect::<BTreeMap<_, _>>();
        let o = Command::new("nm")
            .args(["-S", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let names = String::from_utf8(o.stdout).unwrap();
        let names = names
            .lines()
            .filter_map(|l| {
                let f = l.split_whitespace().collect::<Vec<_>>();
                if f.len() < 4 {
                    return None;
                }
                plain.get(f[0]).map(|raw| (raw.clone(), f[3..].join(" ")))
            })
            .collect::<BTreeMap<_, _>>();
        let decoded =
            fs::read_to_string(r.join(format!("profiles/decode-{label}/stdout.log"))).unwrap();
        let dump = fs::read_to_string(r.join(format!("profiles/loss-{label}/stdout.log"))).unwrap();
        let lost = dump.matches("PERF_RECORD_LOST").count();
        assert_eq!(lost, 0);
        let raw_count = dump.matches("PERF_RECORD_SAMPLE(").count();
        let mut count = 0;
        let mut periods = 0u64;
        let mut pids = BTreeSet::new();
        let mut symbols = BTreeMap::<String, u64>::new();
        for l in decoded
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        {
            let f = l.split_whitespace().collect::<Vec<_>>();
            assert!(f.len() >= 7, "{l}");
            let (pid, tid) = f[1].split_once('/').unwrap();
            assert_eq!(pid, tid);
            pids.insert(pid.parse::<u32>().unwrap());
            assert_eq!(f[4], "cycles:u:");
            let period = f[3].parse::<u64>().unwrap();
            periods += period;
            let sym = names.get(f[6]).cloned().unwrap_or_else(|| f[6].into());
            *symbols.entry(sym).or_default() += period;
            count += 1;
        }
        assert_eq!(count, raw_count);
        assert_eq!(pids.len(), 1);
        assert!(count > 1000);
        total += count;
        let mut symbols=symbols.into_iter().map(|(symbol,period)|json!({"symbol":symbol,"period":period,"share":period as f64/periods as f64})).collect::<Vec<_>>();
        symbols.sort_by(|a, b| b["period"].as_u64().cmp(&a["period"].as_u64()));
        records.push(json!({"variant":label,"samples":count,"raw_sample_events":raw_count,"pids":pids,"lost_event_records":lost,"total_period":periods,"symbols":symbols,"profile_ns_per_lane":got["ns_per_lane"]}));
    }
    for out in &outputs[1..] {
        assert_eq!(out, &outputs[0]);
    }
    fs::write(r.join("profile-analysis.json"),serde_json::to_vec_pretty(&json!({"profiles":records,"samples":total,"lost_event_records":0,"all_output_words_equal":true,"profile_word_comparisons_vs_three_controls":192,"scope":"Whole component process including setup/primes/output; not SDK warm intervals, causal spills or actual G2 caller profiles."})).unwrap()).unwrap();

    assert_eq!(read(r.join("component-result.json"))["passed"], true);
    assert_eq!(read(r.join("codegen-screen.json"))["passed"], true);
    for (file, relocate) in [
        ("build-input-pins.json", true),
        ("build-input-pins-v2.json", false),
    ] {
        let original: BTreeMap<String, String> =
            serde_json::from_value(read(r.join(file))).unwrap();
        for (path, digest) in original {
            let p = if relocate && path == r.join("probe.rs").to_str().unwrap() {
                r.join("probe-initial.rs")
            } else {
                path.into()
            };
            assert_eq!(hash(&p), digest, "originalpin{}", p.display());
        }
    }
    let close = json!({"status":"closed_verified","adopted":false,"performance_gate_passed":read(r.join("component-result.json"))["passed"],"failed_checks":read(r.join("component-result.json"))["failures"].as_array().unwrap().len(),"pins_rehashed":pins.len(),"native_field_tests":33,"new_oracle_tests":4,"exact_raw_outputs":raw,"ark_chain_outputs":chains,"max_modeled_prepacking_coefficient":max.to_string(),"coefficient_bound":"144115188075855872","latency_processes":496,"latency_iterations":49600000u64,"prime_iterations":2976000,"latency_words_compared":41664,"profiles":4,"profile_samples":total,"lost_events":0,"profile_words_compared":192,"new_sdk_proofs":0,"new_gnark_exports":0,"accepted_170_source_files_and_qualified_sdk_unchanged":true,"candidate_scope":"Only two field files with additive methods; no core dispatch or scheduler change. Initial inline previous coefficient tests retained; relative include repair before all timing; new row model and raw shared comparison qualified in build-v2."});
    fs::write(
        r.join("closeout.json"),
        serde_json::to_vec_pretty(&close).unwrap(),
    )
    .unwrap();
    println!("CLOSED496processes,33+4tests,91136exact+729088Arkoutputs,4profiles{total}samples0lost; component positive, not adopted; acceptedunchanged");
}
