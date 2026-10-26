use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, format!("{}\n", serde_json::to_string_pretty(v).unwrap())).unwrap()
}
fn command(name: String, argv: Vec<String>, environment: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":300})
}
fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| s.to_string()).collect()
}
fn engine_command(engine: &str, rounds: usize, proof_dir: &str, case: &Value) -> Vec<String> {
    if engine == "aeglos" {
        return strings(&[
            "/usr/local/bin/aeglos-ablation",
            "aeglos",
            "/gnark-keys/transfer_confidential_1_2.key",
            "/fixtures/transfer-1x2.json",
            &rounds.to_string(),
        ]);
    }
    let binary = if engine == "accepted" {
        "/experiment/zopro-accepted"
    } else {
        "/experiment/zopro-candidate"
    };
    let stem = Path::new(case["image"].as_str().unwrap())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    strings(&[
        binary,
        case["image"].as_str().unwrap(),
        case["request"].as_str().unwrap(),
        &rounds.to_string(),
        &format!("{proof_dir}/{stem}"),
    ])
}
fn environment(engine: &str, workers: usize, proof_dir: &str) -> Value {
    json!({"RAYON_NUM_THREADS":workers.to_string(),"GOMAXPROCS":workers.to_string(),"AEGLOS_CPU_THREADS":workers.to_string(),"AEGLOS_BENCH_DISABLE_SHIFTED":"0","ZOPRO_BENCH_PROOFS":proof_dir,"ZOPRO_CPU_BOUNDED_STEAL_SEARCH":if engine=="on"{"1"}else{"0"}})
}
fn verify(name: &str, proof_dir: &str) -> Value {
    command(
        format!("verify-{name}"),
        strings(&[
            "/usr/local/bin/gnark-verify",
            "/keys",
            proof_dir,
            "/manifest/zopro-keys.lock",
        ]),
        json!({}),
    )
}
fn main() {
    let root = Path::new("/tmp/zopro-bounded-steal-search/aws");
    let context = root.join("context");
    let mut perf = read("/tmp/zopro-bounded-steal-search/perf-plan.json");
    perf["blocks"] = json!(3);
    perf["profile_blocks"] = json!(1);
    perf["checks"] = json!([["/experiment/review", "preflight", "/output"]]);
    perf["variants"] = json!([{ "name":"accepted","binary":"/experiment/zopro-accepted"},{"name":"off","binary":"/experiment/zopro-candidate","environment":{"ZOPRO_CPU_BOUNDED_STEAL_SEARCH":"0"}},{"name":"on","binary":"/experiment/zopro-candidate","environment":{"ZOPRO_CPU_BOUNDED_STEAL_SEARCH":"1"}}]);
    let cases = perf["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["name"] == "1x2-w32" || c["name"] == "merge-54x1-w32")
        .flat_map(|case| {
            [32usize, 64, 96].map(|workers| {
                let mut c = case.clone();
                let shape = c["shape"].as_str().unwrap().to_string();
                c["name"] = json!(format!("{shape}-w{workers}"));
                c["workers"] = json!(workers);
                c["rounds"] = json!(if shape == "1x2" { 31 } else { 9 });
                c["cpu_affinity"] = json!("0-95");
                c["image"] = json!(format!(
                    "/basis-latest/{}",
                    Path::new(c["image"].as_str().unwrap())
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                ));
                c["request"] = json!(format!(
                    "/fixtures/{}",
                    Path::new(c["request"].as_str().unwrap())
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                ));
                c
            })
        })
        .collect::<Vec<_>>();
    perf["profile_cases"] = json!(cases.iter().map(|c| c["name"].clone()).collect::<Vec<_>>());
    perf["cases"] = json!(cases);
    perf["pins"] = json!({});
    perf["prior_proof_sha256"] = read(root.join("prior-proofs.json"));
    perf["environment"]["ZOPRO_BASIS_KEY_DIR"] = json!("/basis-latest");
    perf["environment"]["ZOPRO_CPU_LAGRANGE_DIR"] = json!("/output/lagrange-cache");
    perf["perf"] = json!("/usr/lib/linux-tools/6.8.0-147-generic/perf");
    perf["perf_prefix"] = json!([]);
    perf["child_prefix"] = json!([]);
    perf["verifier"] = json!([
        "/usr/local/bin/gnark-verify",
        "/keys",
        "{proof_dir}",
        "/manifest/zopro-keys.lock"
    ]);
    save(context.join("perf-template.json"), &perf);
    let env = perf["environment"].clone();
    let mut preflight_steps = Vec::new();
    let mut preflight_jobs = Vec::new();
    for case in cases.iter().filter(|c| c["workers"] == 32) {
        for engine in ["accepted", "off", "on"] {
            let name = format!("{}-{engine}", case["shape"].as_str().unwrap());
            let dir = format!("/output/preflight-proofs/{name}");
            preflight_steps.push(command(
                name.clone(),
                engine_command(engine, 1, &dir, case),
                environment(engine, 4, &dir),
            ));
            preflight_steps.push(verify(&name, &dir));
            preflight_jobs.push(json!({"name":name,"engine":engine,"workers":4,"rounds":1,"proof_dir":dir,"public_sha256":case["public_sha256"],"output":"preflight"}));
        }
    }
    let primary = cases.iter().find(|c| c["shape"] == "1x2").unwrap();
    let name = "aeglos-primary";
    let dir = "/output/preflight-proofs/aeglos-primary";
    preflight_steps.push(command(
        name.into(),
        engine_command("aeglos", 1, dir, primary),
        environment("aeglos", 4, dir),
    ));
    preflight_steps.push(verify(name, dir));
    preflight_jobs.push(json!({"name":name,"engine":"aeglos","workers":4,"rounds":1,"proof_dir":dir,"public_sha256":primary["public_sha256"],"output":"preflight"}));
    preflight_steps.push(command(
        "review-preflight".into(),
        strings(&["/experiment/review", "preflight", "/output"]),
        json!({}),
    ));
    save(
        context.join("preflight.json"),
        &json!({"version":1,"source_pins":"/experiment/input-pins.json","environment":env,"steps":preflight_steps}),
    );
    save(
        context.join("preflight-jobs.json"),
        &json!({"jobs":preflight_jobs,"sdk":28,"exports":7}),
    );
    let mut comparisons = BTreeMap::new();
    let mut jobs = Vec::new();
    for kind in ["latency", "stat"] {
        let mut steps = Vec::new();
        let rounds = if kind == "latency" { 31 } else { 201 };
        let output = format!("comparison-{kind}");
        for block in 0..3 {
            let mut workers_order = vec![32usize, 64, 96];
            if block % 2 == 1 {
                workers_order.reverse();
            }
            for (wi, workers) in workers_order.into_iter().enumerate() {
                let mut engines = vec!["accepted", "off", "on", "aeglos"];
                engines.rotate_left((block + wi) % 4);
                if block % 2 == 1 {
                    engines.reverse();
                }
                for engine in engines {
                    let name = format!("{kind}-b{block}-w{workers}-{engine}");
                    let dir = format!("/output/comparison-proofs/{name}");
                    let mut e = environment(engine, workers, &dir);
                    let mut argv = strings(&["taskset", "-c", "0-95"]);
                    argv.extend(engine_command(engine, rounds, &dir, primary));
                    if kind == "stat" {
                        let counters = format!("/output/counters/{name}");
                        let ctl = format!("{counters}/ctl");
                        let ack = format!("{counters}/ack");
                        steps.push(command(
                            format!("channels-{name}"),
                            strings(&[
                                "/bin/sh",
                                "-c",
                                &format!("set -eu\nmkdir -p {counters}\nmkfifo {ctl} {ack}"),
                            ]),
                            json!({}),
                        ));
                        e["ZOPRO_BENCH_PERF_CTL"] = json!(ctl);
                        e["ZOPRO_BENCH_PERF_ACK"] = json!(ack);
                        e["CROSS_PERF_CTL"] = json!(ctl);
                        e["CROSS_PERF_ACK"] = json!(ack);
                        let mut stat = strings(&[
                            "/usr/lib/linux-tools/6.8.0-147-generic/perf",
                            "stat",
                            "--no-big-num",
                            "-x",
                            ";",
                            "-e",
                            "cycles:u,instructions:u,branches:u,branch-misses:u",
                            "--delay=-1",
                            &format!("--control=fifo:{ctl},{ack}"),
                            "-o",
                            &format!("{counters}/stat.csv"),
                            "--",
                        ]);
                        stat.extend(argv);
                        argv = stat;
                    }
                    steps.push(command(name.clone(), argv, e));
                    steps.push(verify(&name, &dir));
                    jobs.push(json!({"name":name,"kind":kind,"block":block,"workers":workers,"engine":engine,"rounds":rounds,"priming":3,"proof_dir":dir,"public_sha256":primary["public_sha256"],"output":output}));
                }
            }
        }
        comparisons.insert(kind,json!({"version":1,"source_pins":"/experiment/input-pins.json","environment":env,"steps":steps}));
    }
    assert_eq!(jobs.len(), 72);
    assert_eq!(
        jobs.iter()
            .map(|j| j["rounds"].as_u64().unwrap() + 3)
            .sum::<u64>(),
        8568
    );
    for (kind, plan) in comparisons {
        save(context.join(format!("comparison-{kind}.json")), &plan);
    }
    save(
        context.join("comparison-jobs.json"),
        &json!({"sdk":8568,"exports":72,"jobs":jobs}),
    );
    println!(
        "CONTEXT_PLANS perf54latency18profiles2286SDK;comparison72process8568SDK;preflight28SDK7exports;AWS10882SDK151exports"
    );
}
