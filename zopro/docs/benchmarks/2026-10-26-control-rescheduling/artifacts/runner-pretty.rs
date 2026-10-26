use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::{Command, Stdio},
};
const R: &str = "/tmp/zopro-control-rescheduling";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn managed(k: &str) -> bool {
    ["ZOPRO_", "RAYON_", "MALLOC_", "CROSS_", "CARGO_"]
        .iter()
        .any(|p| k.starts_with(p))
        || ["RUSTFLAGS", "RUSTDOCFLAGS", "RUST_MIN_STACK"].contains(&k)
}
fn run(argv: Vec<String>, env: &BTreeMap<String, String>, dir: &Path) {
    fs::create_dir_all(dir).unwrap();
    assert!(
        !dir.join("receipt.json").exists(),
        "no rerun of successful command"
    );
    save(
        dir.join("command.json"),
        &json!({"argv":argv,"environment":env}),
    );
    let mut c = Command::new(&argv[0]);
    c.args(&argv[1..]);
    for (k, _) in std::env::vars().filter(|(k, _)| managed(k)) {
        c.env_remove(k);
    }
    c.envs(env)
        .stdout(Stdio::from(
            fs::File::create(dir.join("stdout.log")).unwrap(),
        ))
        .stderr(Stdio::from(
            fs::File::create(dir.join("stderr.log")).unwrap(),
        ));
    let status = c.status().unwrap();
    save(
        dir.join("receipt.json"),
        &json!({"success":status.success(),"code":status.code()}),
    );
    assert!(status.success(), "command failed in {}", dir.display());
}
fn prior() -> BTreeSet<String> {
    let mut seen: BTreeSet<String> = read("/tmp/zopro-g2-tree-sdk/context/manifest.json")
        ["prior_proof_sha256"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().into())
        .collect();
    assert_eq!(seen.len(), 5958);
    for root in [
        "zopro-g2-tree-sdk",
        "zopro-g1-quarter-current",
        "zopro-pairing-x-chain-sdk",
    ] {
        for (p, h) in read(format!("/tmp/{root}/proof-inventory.json"))
            .as_object()
            .unwrap()
        {
            assert_eq!(hash(p), h.as_str().unwrap());
            if p.ends_with(".proof") {
                assert!(seen.insert(h.as_str().unwrap().into()));
            }
        }
    }
    assert_eq!(seen.len(), 6793);
    seen
}
fn validate_proof(
    dir: &Path,
    seen: &mut BTreeSet<String>,
    inventory: &mut BTreeMap<String, String>,
) {
    let proof = dir.join("proofs/transfer_confidential_1_2/0.proof");
    let public = proof.with_extension("public");
    assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
    assert_eq!(
        hash(&public),
        "3f0d846c3905d11595d77dd988df829ddc51325eda2b93e4db8f0e17a5928e5b"
    );
    let h = hash(&proof);
    assert!(seen.insert(h.clone()), "proof freshness");
    inventory.insert(proof.display().to_string(), h);
    inventory.insert(public.display().to_string(), hash(public));
    run(
        vec![
            "/tmp/zopro-gnark-verify/verify".into(),
            "/tmp/zopro-keys".into(),
            dir.join("proofs").display().to_string(),
            "/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock".into(),
        ],
        &BTreeMap::new(),
        &dir.join("gnark"),
    );
}
fn validate_clock(log: &Path, rounds: usize, workers: usize) {
    let text = fs::read_to_string(log).unwrap();
    let mut count = 0;
    let mut last = 0;
    let mut summary = None;
    for line in text.lines() {
        if let Some(v) = line.strip_prefix("SAMPLE ") {
            let v: Value = serde_json::from_str(v).unwrap();
            let b = v["begin_ns"].as_u64().unwrap();
            let e = v["end_ns"].as_u64().unwrap();
            assert!(b >= last && e > b);
            let ms = v["ms"].as_f64().unwrap();
            assert!(ms > 0. && (e - b) as f64 + 2. >= ms * 1e6);
            last = e;
            count += 1;
        } else if let Some(v) = line.strip_prefix("SUMMARY ") {
            assert!(summary.is_none());
            summary = Some(serde_json::from_str::<Value>(v).unwrap());
        }
    }
    assert_eq!(count, rounds);
    let s = summary.unwrap();
    assert_eq!(s["rounds"], rounds);
    assert_eq!(s["workers"], workers);
    assert_eq!(s["priming"], 3);
    assert!(s["pid"].as_u64().unwrap() > 0);
}
fn capture(traced: bool) {
    let root = Path::new(R);
    let plan = read(root.join("capture-plan.json"));
    let mut seen = prior();
    let mut inventory = BTreeMap::new();
    if traced {
        let old = read(root.join("preflight-inventory.json"));
        for (p, h) in old.as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            if p.ends_with(".proof") {
                assert!(seen.insert(h.as_str().unwrap().into()));
            }
        }
        assert_eq!(seen.len(), 6796);
    }
    let specs = plan[if traced { "traces" } else { "preflight" }]
        .as_array()
        .unwrap();
    for s in specs {
        let dir = root.join(s["name"].as_str().unwrap());
        let workers = s["workers"].as_u64().unwrap() as usize;
        let rounds = s["rounds"].as_u64().unwrap() as usize;
        let variant = s["variant"].as_str().unwrap();
        let affinity = format!("0-{}", workers - 1);
        let mut env: BTreeMap<String, String> =
            serde_json::from_value(plan["environment"].clone()).unwrap();
        env.insert("RAYON_NUM_THREADS".into(), workers.to_string());
        env.insert(
            "ZOPRO_CPU_PAIRING_X_CHAIN".into(),
            if variant == "on" { "1" } else { "0" }.into(),
        );
        let mut sdk = vec![
            "taskset".into(),
            "-c".into(),
            affinity.clone(),
            format!(
                "{R}/native/{}-sdk",
                if variant == "accepted" {
                    "accepted"
                } else {
                    "candidate"
                }
            ),
            plan["image"].as_str().unwrap().into(),
            plan["request"].as_str().unwrap().into(),
            rounds.to_string(),
            dir.join("proofs/transfer_confidential_1_2")
                .display()
                .to_string(),
        ];
        let argv = if traced {
            fs::create_dir_all(&dir).unwrap();
            let ctl = dir.join("ctl");
            let ack = dir.join("ack");
            let status = Command::new("mkfifo").arg(&ctl).arg(&ack).status().unwrap();
            assert!(status.success());
            env.insert("ZOPRO_BENCH_PERF_CTL".into(), ctl.display().to_string());
            env.insert("ZOPRO_BENCH_PERF_ACK".into(), ack.display().to_string());
            let mut a = vec![
                "sudo".into(),
                "-n".into(),
                "/usr/bin/perf".into(),
                "record".into(),
                "-a".into(),
                "-C".into(),
                affinity,
                "--clockid".into(),
                "mono".into(),
                "--no-buildid-cache".into(),
                "--delay=-1".into(),
                format!("--control=fifo:{},{}", ctl.display(), ack.display()),
                "-m".into(),
                "128".into(),
            ];
            for e in [
                "sched:sched_switch",
                "syscalls:sys_enter_sched_yield",
                "syscalls:sys_exit_sched_yield",
                "syscalls:sys_enter_futex",
                "syscalls:sys_exit_futex",
            ] {
                a.extend(["-e".into(), e.into()]);
            }
            a.extend([
                "-o".into(),
                dir.join("perf.data").display().to_string(),
                "--".into(),
                "sudo".into(),
                "-n".into(),
                "-u".into(),
                "ubuntu".into(),
                "env".into(),
            ]);
            a.extend(env.iter().map(|(k, v)| format!("{k}={v}")));
            a.append(&mut sdk);
            a
        } else {
            sdk
        };
        println!("CAPTURE {}", s["name"]);
        run(argv, &env, &dir);
        validate_clock(&dir.join("stdout.log"), rounds, workers);
        validate_proof(&dir, &mut seen, &mut inventory);
        if traced {
            fs::remove_file(dir.join("ctl")).unwrap();
            fs::remove_file(dir.join("ack")).unwrap();
        }
        save(
            root.join(if traced {
                "proof-inventory.json"
            } else {
                "preflight-inventory.json"
            }),
            &json!(inventory),
        );
    }
    let expected = if traced { 6814 } else { 6796 };
    assert_eq!(seen.len(), expected);
    save(
        root.join(if traced {
            "capture-complete.json"
        } else {
            "preflight-complete.json"
        }),
        &json!({"exports":specs.len(),"next_prior":expected,"proof_binding_freshness_clocks_and_gnark":"pass","sdk_requests":if traced{612}else{12}}),
    );
}
fn decode() {
    let root = Path::new(R);
    assert_eq!(read(root.join("capture-complete.json"))["next_prior"], 6814);
    let plan = read(root.join("capture-plan.json"));
    let mut rows = Vec::new();
    let mut rawpins = BTreeMap::new();
    for s in plan["traces"].as_array().unwrap() {
        let dir = root.join(s["name"].as_str().unwrap());
        if !dir.join("decoded/receipt.json").exists() {
            run(
                vec![
                    "sudo".into(),
                    "-n".into(),
                    "/usr/bin/perf".into(),
                    "script".into(),
                    "--ns".into(),
                    "--show-lost-events".into(),
                    "-i".into(),
                    dir.join("perf.data").display().to_string(),
                    "-F".into(),
                    "pid,tid,cpu,time,event,trace".into(),
                ],
                &BTreeMap::new(),
                &dir.join("decoded"),
            );
        } else {
            assert_eq!(read(dir.join("decoded/receipt.json"))["success"], true);
        }
        let out = dir.join("analysis.json");
        run(
            vec![
                format!("{R}/analyze-pretty"),
                dir.join("decoded/stdout.log").display().to_string(),
                dir.join("stdout.log").display().to_string(),
                s["workers"].to_string(),
                out.display().to_string(),
            ],
            &BTreeMap::new(),
            &dir.join("analyze-pretty"),
        );
        let mut row = read(&out);
        row["case"] = s.clone();
        rows.push(row);
        let p = dir.join("perf.data");
        let out = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(&p)
            .output()
            .unwrap();
        assert!(out.status.success());
        rawpins.insert(
            p.display().to_string(),
            String::from_utf8(out.stdout)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .to_string(),
        );
        let p = dir.join("decoded/stdout.log");
        rawpins.insert(p.display().to_string(), hash(p));
    }
    save(root.join("observations.json"), &json!(rows));
    save(root.join("raw-local-only-sha256.json"), &json!(rawpins));
}
fn freeze() {
    let r = Path::new(R);
    let mut pins = read(r.join("build-pins.json"));
    for name in [
        "native/accepted-sdk",
        "native/candidate-sdk",
        "analyze",
        "runner",
    ] {
        let p = r.join(name);
        pins[p.display().to_string()] = json!(hash(&p));
    }
    let old = read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json");
    let mut preflight = Vec::new();
    for v in ["accepted", "off", "on"] {
        preflight.push(json!({"name":format!("preflight/{v}"),"workers":4,"rounds":1,"variant":v}));
    }
    let mut traces = Vec::new();
    for b in 0..3 {
        let mut vs = vec!["accepted", "off", "on"];
        vs.rotate_left(b);
        if b % 2 == 1 {
            vs.reverse();
        }
        let ws = if b % 2 == 0 { [16, 32] } else { [32, 16] };
        for w in ws {
            for v in &vs {
                traces.push(json!({"name":format!("traces/b{b}-w{w}-{v}"),"block":b,"workers":w,"rounds":31,"variant":v}));
            }
        }
    }
    save(
        r.join("capture-plan.json"),
        &json!({"version":1,"preflight":preflight,"traces":traces,"environment":old["environment"],"image":old["cases"][0]["image"],"request":old["cases"][0]["request"],"prior_count":6793,"diagnostic_only":true}),
    );
    let p = r.join("capture-plan.json");
    pins[p.display().to_string()] = json!(hash(&p));
    save(r.join("capture-pins.json"), &pins);
    let mut steps = Vec::new();
    for m in ["preflight", "trace", "decode"] {
        steps.push(json!({"kind":"command","name":m,"argv":[format!("{R}/runner"),m],"timeout_seconds":1800}));
    }
    save(
        r.join("workflow-capture.json"),
        &json!({"version":1,"source_pins":r.join("capture-pins.json"),"steps":steps}),
    );
    println!("Frozen 3 preflights + 18 scheduling traces;624 SDK requests,21 exports planned");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "freeze" => freeze(),
        "preflight" => capture(false),
        "trace" => capture(true),
        "decode" => decode(),
        _ => panic!("mode"),
    }
}
