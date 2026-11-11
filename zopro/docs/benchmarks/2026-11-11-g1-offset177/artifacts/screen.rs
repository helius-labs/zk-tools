use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-offset177";
const SEED: &str =
    "/home/ubuntu/src/zopro-perf-work/gather-backing177/proof-sha256-after-capture.json";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(
        !p.as_ref().exists(),
        "already exists: {}",
        p.as_ref().display()
    );
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
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
fn command(name: &str, argv: Vec<String>, environment: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":1800})
}
fn receipt(folder: &str, s: &Value) {
    let base = r(format!("{folder}/{}", s["name"].as_str().unwrap()));
    let exit = read(base.join("exit.json"));
    assert_eq!(exit["success"], true);
    assert_eq!(exit["returncode"], 0);
    for stream in ["stdout", "stderr"] {
        assert_eq!(
            hash(base.join(format!("{stream}.log"))),
            exit[format!("{stream}_sha256")]
        );
    }
    let c = read(base.join("command.json"));
    assert_eq!(
        &c["argv"].as_array().unwrap()[4..],
        s["argv"].as_array().unwrap()
    );
    for (k, v) in s["environment"].as_object().into_iter().flatten() {
        // xtask records the benchmark environment allowlist. Compiler selection
        // is retained in the build plan, fingerprints and explicit link receipt.
        if !["PATH", "RUSTC", "RUSTUP_TOOLCHAIN"].contains(&k.as_str()) {
            assert_eq!(c["environment"][k], *v);
        }
    }
}
fn add(
    c: &Value,
    arm: &str,
    block: usize,
    phase: &str,
    base: &Value,
    ids: &Value,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
) {
    let name = format!("{phase}-{}-{arm}-{block:02}", c["name"].as_str().unwrap());
    let rounds = if phase == "preflight" {
        1
    } else if c["shape"] == "1x2" {
        101
    } else {
        31
    };
    let mut env = base.clone();
    env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
    let dir = r(format!("proofs/{name}"));
    let binary = ids[arm]["copies"][block % 3]["path"].as_str().unwrap();
    let argv = vec![
        "/usr/bin/taskset".into(),
        "-c".into(),
        c["cpu_affinity"].as_str().unwrap().into(),
        binary.into(),
        c["image"].as_str().unwrap().into(),
        c["request"].as_str().unwrap().into(),
        rounds.to_string(),
        dir.display().to_string(),
    ];
    steps.push(command(&name, argv, env.clone()));
    jobs.push(json!({"name":name,"case":c,"arm":arm,"block":block,"rounds":rounds,"proof_dir":dir,"environment":env,"binary":binary}));
}
fn verify(j: &Value) -> Value {
    let c = &j["case"];
    let dir = j["proof_dir"].as_str().unwrap();
    command(
        &format!("verify-{}", j["name"].as_str().unwrap()),
        vec![
            VERIFY.into(),
            c["image"].as_str().unwrap().into(),
            c["image_sha256"].as_str().unwrap().into(),
            format!("{dir}/0.proof"),
            format!("{dir}/0.public"),
            c["public_sha256"].as_str().unwrap().into(),
        ],
        json!({}),
    )
}
fn prepare() {
    let checkpoint = read(r("runtime-consumers.json"));
    let mut pins = BTreeMap::<String, String>::new();
    for arm in ["REF", "OFF", "ON"] {
        let p = checkpoint[arm]["path"].as_str().unwrap();
        assert_eq!(hash(p), checkpoint[arm]["sha256"]);
        pins.insert(p.into(), hash(p));
    }
    for label in ["control", "candidate"] {
        let folder = format!("native-{label}");
        assert_eq!(
            read(r(format!("{folder}/result.json")))["status"],
            "complete"
        );
        for step in read(r(format!("{label}-plan.json")))["steps"]
            .as_array()
            .unwrap()
        {
            receipt(&folder, step);
        }
    }
    for (p, h) in read(r("build-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().into());
    }
    for (file, needle) in [
        ("candidate-core", "130 passed; 0 failed"),
        ("candidate-prover", "6 passed; 0 failed"),
    ] {
        assert!(
            fs::read_to_string(r(format!("native-candidate/{file}/stdout.log")))
                .unwrap()
                .contains(needle)
        );
    }
    let cases = read("/home/ubuntu/src/zopro-perf-work/large-class-sdk176/measure-cases.json");
    let pre: Vec<_> = cases
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
        .cloned()
        .collect();
    assert_eq!(pre.len(), 9);
    let main: Vec<_> = ["1x2-w16", "1x2-w32", "2x2-w32", "4x2-w32", "8x2-w32", "12x2-w32", "16x2-w32", "24x2-w32", "merge-24x1-w32", "merge-54x1-w32"]
        .iter()
        .map(|name| {
            cases
                .as_array()
                .unwrap()
                .iter()
                .find(|c| c["name"] == *name)
                .unwrap()
                .clone()
        })
        .collect();
    for c in &pre {
        for k in ["image", "request"] {
            let p = c[k].as_str().unwrap();
            let h = hash(p);
            if k == "image" {
                assert_eq!(h, c["image_sha256"]);
            }
            pins.insert(p.into(), h);
        }
    }
    let base=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json")["environment"].clone();
    assert!(base.is_object());
    let seed: BTreeSet<String> = serde_json::from_value(read(SEED)).unwrap();
    assert_eq!(seed.len(), 52000);
    // Two independent integer constructions of the binomial coverage.
    let mut row = vec![1u64];
    for _ in 0..30 {
        let mut next = vec![1; row.len() + 1];
        for i in 1..row.len() {
            next[i] = row[i - 1] + row[i];
        }
        row = next;
    }
    let mut choose = 1u64;
    let mut total = 0u64;
    for k in 0..=30 {
        assert_eq!(choose, row[k]);
        if (10..=20).contains(&k) {
            total += choose;
        }
        if k < 30 {
            choose = choose * (30 - k as u64) / (k as u64 + 1);
        }
    }
    save(
        r("statistics-model.json"),
        &json!({"blocks":30,"lower_rank_zero_based":9,"median_ranks_zero_based":[14,15],"upper_rank_zero_based":20,"marginal_coverage":total as f64/(1u64<<30)as f64,"two_integer_models_agree":true,"joint_confidence_claim":false}),
    );
    check_copies(&checkpoint);
    for arm in ["REF","OFF","ON"] { for f in checkpoint[arm]["copies"].as_array().unwrap() {let p=f["path"].as_str().unwrap(); pins.insert(p.into(),hash(p));}}
    for phase in ["preflight", "main"] {
        let (mut jobs, mut steps) = (vec![], vec![]);
        if phase == "preflight" {
            for c in &pre {
                for arm in ["REF", "OFF", "ON"] {
                    add(c, arm, 0, phase, &base, &checkpoint, &mut jobs, &mut steps);
                }
            }
        } else {
            for b in 0..30 {
                let mut order = (0..10).collect::<Vec<_>>();
                let mut arms = vec!["REF", "OFF", "ON"];
                order.rotate_left(b % 10);
                arms.rotate_left((b / 3) % 3);
                if b % 2 == 1 { order.reverse(); }
                if (b / 3) % 2 == 1 { arms.reverse(); }
                for i in order {
                    for arm in &arms {
                        add(
                            &main[i],
                            arm,
                            b,
                            phase,
                            &base,
                            &checkpoint,
                            &mut jobs,
                            &mut steps,
                        );
                    }
                }
            }
        }
        assert_eq!(jobs.len(), if phase == "preflight" { 27 } else { 900 });
        assert_eq!(
            jobs.iter()
                .map(|j| j["rounds"].as_u64().unwrap() + 3)
                .sum::<u64>(),
            if phase == "preflight" { 108 } else { 43200 }
        );
        steps.extend(jobs.iter().map(verify));
        save(r(format!("{phase}-jobs.json")), &json!(jobs));
        save(
            r(format!("{phase}-plan.json")),
            &json!({"version":1,"source_pins":r("runtime-pins.json"),"steps":steps}),
        );
    }
    for p in [
        SEED,
        VERIFY,
        "/usr/bin/taskset",
        "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask",
    ] {
        pins.insert(p.into(), hash(p));
    }
    for f in [
        "screen.rs",
        "screen",
        "independent.rs",
        "independent",
        "screen-design.md",
        "runtime-consumers.json",
        "prepare-screen.rs",
        "statistics-model.json",
        "preflight-jobs.json",
        "preflight-plan.json",
        "main-jobs.json",
        "main-plan.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    save(r("runtime-pins.json"), &json!(pins));
    println!("Frozen both protocols, consumers, inputs, independent receipt/proof reader and statistics. 27+900 exports planned; no SDK calls yet.");
}
fn review(phase: &str) {
    assert!(["preflight", "main"].contains(&phase));
    check_copies(&read(r("runtime-consumers.json")));
    let folder = format!("{phase}-run");
    assert_eq!(
        read(r(format!("{folder}/result.json")))["status"],
        "complete"
    );
    for (p, h) in read(r("runtime-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for step in read(r(format!("{phase}-plan.json")))["steps"]
        .as_array()
        .unwrap()
    {
        receipt(&folder, step);
    }
    let mut ledger: BTreeSet<String> = serde_json::from_value(read(if phase == "preflight" {
        PathBuf::from(SEED)
    } else {
        r("proof-sha256-after-preflight.json")
    }))
    .unwrap();
    assert_eq!(
        ledger.len(),
        if phase == "preflight" { 52000 } else { 52027 }
    );
    let mut rows = vec![];
    let mut samples_count = 0u64;
    for j in read(r(format!("{phase}-jobs.json"))).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let out = fs::read_to_string(r(format!("{folder}/{name}/stdout.log"))).unwrap();
        let samples: Vec<f64> = out
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .enumerate()
            .map(|(i, s)| {
                let v: Value = serde_json::from_str(s).unwrap();
                assert_eq!(v["round"], i);
                let x = v["ms"].as_f64().unwrap();
                assert!(x > 0. && x.is_finite());
                x
            })
            .collect();
        let rounds = j["rounds"].as_u64().unwrap();
        assert_eq!(samples.len() as u64, rounds);
        samples_count += rounds;
        let sums: Vec<_> = out
            .lines()
            .filter_map(|l| l.strip_prefix("SUMMARY "))
            .collect();
        assert_eq!(sums.len(), 1);
        let sum: Value = serde_json::from_str(sums[0]).unwrap();
        assert_eq!(sum["rounds"], rounds);
        assert_eq!(sum["priming"], 3);
        assert_eq!(sum["workers"], j["case"]["workers"]);
        assert_eq!(sum["placement_before"], sum["placement_after"]);
        let p = &sum["placement_before"];
        assert_eq!(p["mode"], "fixed");
        let workers = sum["workers"].as_u64().unwrap();
        assert_eq!(p["tasks"], workers + 1);
        assert_eq!(p["main_allowed"], json!((0..workers).collect::<Vec<_>>()));
        let worker_rows = p["worker_rows"].as_array().unwrap();
        assert_eq!(worker_rows.len() as u64, workers);
        let mut tids = BTreeSet::new();
        for (i, w) in worker_rows.iter().enumerate() {
            assert_eq!(w["index"], i);
            assert_eq!(w["allowed"], json!([i]));
            assert!(tids.insert(w["tid"].as_u64().unwrap()));
        }
        let dir = Path::new(j["proof_dir"].as_str().unwrap());
        assert_eq!(fs::metadata(dir.join("0.proof")).unwrap().len(), 164);
        let ph = hash(dir.join("0.proof"));
        let pu = hash(dir.join("0.public"));
        assert_eq!(pu, j["case"]["public_sha256"]);
        let verifier = fs::read_to_string(r(format!("{folder}/verify-{name}/stdout.log"))).unwrap();
        for needle in [
            format!("proof_sha256={ph}"),
            format!("public_sha256={pu}"),
            format!(
                "image_sha256={}",
                j["case"]["image_sha256"].as_str().unwrap()
            ),
        ] {
            assert!(verifier.contains(&needle));
        }
        assert!(ledger.insert(ph.clone()));
        let mean = samples.iter().sum::<f64>() / rounds as f64;
        let cpu = (sum["user_cpu_ms"].as_f64().unwrap() + sum["system_cpu_ms"].as_f64().unwrap())
            / rounds as f64;
        rows.push(json!({"name":name,"context":j["case"]["name"],"arm":j["arm"],"block":j["block"],"ms":mean,"cpu_ms":cpu,"summary":sum,"proof_sha256":ph}));
    }
    assert_eq!(
        ledger.len(),
        if phase == "preflight" { 52027 } else { 52927 }
    );
    assert_eq!(samples_count, if phase == "preflight" { 27 } else { 40500 });
    save(
        r(format!("proof-sha256-after-{phase}.json")),
        &json!(ledger),
    );
    save(r(format!("{phase}-rows.json")), &json!(rows));
    if phase == "main" {
        assess(&rows);
    }
    save(
        r(format!("{phase}-review.json")),
        &json!({"verified":true,"new_exports":rows.len(),"canonical_count":ledger.len(),"samples_checked":samples_count}),
    );
    println!(
        "Verified {phase}: {} exports; ledger{}; {samples_count} samples",
        rows.len(),
        ledger.len()
    );
}
fn bounds(mut v: Vec<f64>) -> Value {
    assert_eq!(v.len(), 30);
    assert!(v.iter().all(|x| x.is_finite() && *x > 0.));
    v.sort_by(f64::total_cmp);
    json!({"lower":v[9],"median":(v[14]+v[15])/2.,"upper":v[20]})
}
fn assess(rows: &[Value]) {
    let mut gates = vec![];
    let mut timings = vec![];
    let mut diagnostics = vec![];
    for context in ["1x2-w16", "1x2-w32", "2x2-w32", "4x2-w32", "8x2-w32", "12x2-w32", "16x2-w32", "24x2-w32", "merge-24x1-w32", "merge-54x1-w32"] {
        let arm = |a: &str| {
            let mut v: Vec<_> = rows
                .iter()
                .filter(|v| v["context"] == context && v["arm"] == a)
                .collect();
            v.sort_by_key(|v| v["block"].as_u64().unwrap());
            assert_eq!(v.len(), 30);
            for (i, r) in v.iter().enumerate() {
                assert_eq!(r["block"], i);
            }
            v
        };
        let on = arm("ON");
        for a in ["REF", "OFF", "ON"] {
            let v = arm(a);
            timings.push(json!({"context":context,"arm":a,"ms":bounds(v.iter().map(|r|r["ms"].as_f64().unwrap()).collect()),"cpu_ms":bounds(v.iter().map(|r|r["cpu_ms"].as_f64().unwrap()).collect())}));
        }
        for comparator in ["REF", "OFF"] {
            let other = arm(comparator);
            for metric in ["ms", "cpu_ms"] {
                let b = bounds(
                    on.iter()
                        .zip(&other)
                        .map(|(a, b)| a[metric].as_f64().unwrap() / b[metric].as_f64().unwrap())
                        .collect(),
                );
                diagnostics.push(json!({"context":context,"comparator":comparator,"metric":metric,"ratio":b}));
                let gain = context == "1x2-w32" && metric == "ms";
                let upper = b["upper"].as_f64().unwrap();
                if comparator == "REF" || gain { gates.push(json!({"context":context,"comparator":comparator,"metric":metric,"ratio":b,"passed":if gain{upper<1.}else{upper<=1.02},"limit":if gain{1.}else{1.02},"strict":gain})); }
            }
            let peak = |v: &Vec<&Value>| {
                v.iter()
                    .map(|v| v["summary"]["whole_process_maxrss_kib"].as_f64().unwrap())
                    .fold(0., f64::max)
            };
            let a = peak(&on);
            let b = peak(&other);
            if comparator == "REF" { gates.push(json!({"context":context,"comparator":comparator,"metric":"peak_rss_kib","ON":a,"control":b,"passed":a<=b*1.05+1024.})); }
        }
    }
    assert_eq!(gates.len(), 31);
    let passed = gates.iter().filter(|g| g["passed"] == true).count();
    save(
        r("assessment.json"),
        &json!({"checks_passed":passed,"checks":gates,"timings":timings,"diagnostic_contrasts":diagnostics,"backings":backing_summary(rows),"adopted":false,"full_goal_achieved":false,"marginal_intervals_only":true}),
    );
    println!("Progression checks {passed}/31");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("preflight-review") => review("preflight"),
        Some("main-review") => review("main"),
        _ => panic!("prepare/preflight-review/main-review"),
    }
}

fn check_copies(ids: &Value) {
    use std::os::unix::fs::{MetadataExt,PermissionsExt};
    let mut distinct=BTreeSet::new();
    for arm in ["REF","OFF","ON"] { for f in ids[arm]["copies"].as_array().unwrap() {
        let p=f["path"].as_str().unwrap();let m=fs::metadata(p).unwrap();
        assert_eq!(hash(p),f["sha256"]);assert_eq!(m.dev(),f["device"].as_u64().unwrap());
        assert_eq!(m.ino(),f["inode"].as_u64().unwrap());assert_eq!(m.permissions().mode()as u64,f["mode"].as_u64().unwrap());
        assert!(distinct.insert((m.dev(),m.ino())));
    }} assert_eq!(distinct.len(),9);
}
fn backing_summary(rows:&[Value])->Value {
 let contexts:BTreeSet<_>=rows.iter().map(|v|v["context"].as_str().unwrap()).collect();let mut results=vec![];let mut flags=vec![];
 for context in contexts { for comparator in ["REF","OFF"] { let mut medians=vec![];
  for copy in 0..3u64 {let mut ratios=vec![];for block in(0..30u64).filter(|b|b%3==copy){let get=|arm:&str|rows.iter().find(|r|r["context"]==context&&r["block"]==block&&r["arm"]==arm).unwrap()["ms"].as_f64().unwrap();ratios.push(get("ON")/get(comparator));}ratios.sort_by(f64::total_cmp);assert_eq!(ratios.len(),10);let median=(ratios[4]+ratios[5])/2.;medians.push(median);results.push(json!({"context":context,"copy":copy,"comparator":comparator,"paired_ratios_sorted":ratios,"median_ratio":median}));}
  if context=="1x2-w32"{let spread=medians.iter().copied().fold(f64::NEG_INFINITY,f64::max)/medians.iter().copied().fold(f64::INFINITY,f64::min);flags.push(json!({"comparator":comparator,"max_min_median_ratio":spread,"needs_diagnosis":spread>1.02}));}
 }}json!({"strata":results,"primary_spread_flags":flags,"all_backings_retained":true,"stratum_intervals_claimed":false})
}
