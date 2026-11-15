use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
mod validate;
const R: &str = "/home/ubuntu/src/zopro-perf-work/msm-local180";
const I: &str = "/home/ubuntu/src/zopro-perf-work/msm-full180";
const SEED: &str = "f40afdcc062125e33e69e523708915956f1aa21bfc25b2dfd2604e81d28e0446";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(n: &str, v: Value) {
    let p = format!("{R}/{n}");
    assert!(!Path::new(&p).exists(), "already exists: {p}");
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap()
}
fn mean(v: &[f64]) -> f64 {
    assert!(!v.is_empty());
    assert!(v.iter().all(|v| v.is_finite()));
    v.iter().sum::<f64>() / v.len() as f64
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn state() -> Value {
    let mut m = BTreeMap::new();
    for p in [
        "/proc/loadavg",
        "/proc/meminfo",
        "/proc/swaps",
        "/proc/vmstat",
        "/proc/stat",
        "/proc/pressure/cpu",
        "/proc/pressure/memory",
        "/proc/pressure/io",
        "/proc/self/cgroup",
    ] {
        if let Ok(s) = fs::read_to_string(p) {
            m.insert(p, s);
        }
    }
    json!(m)
}
fn verify_pins() {
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
}
fn prepare() {
    assert_eq!(hash(format!("{I}/proof-sha256-after-preflight.json")), SEED);
    let original = read(format!("{I}/preflight-jobs.json"));
    let plan = read(format!("{I}/preflight-plan.json"));
    let mut pins = read(format!("{I}/preflight-pins.json"))
        .as_object()
        .unwrap()
        .clone();
    for (p, h) in &pins {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut late = vec![];
    let mut add = |workers: usize, block: usize, panel: &str, mode: &str, arm: &str| {
        let base_name = format!("{mode}-{arm}");
        let orig = original
            .as_array()
            .unwrap()
            .iter()
            .find(|j| j["name"] == base_name)
            .unwrap();
        let mut j = orig.clone();
        let name = format!("{panel}-w{workers}-{base_name}-b{block:02}");
        let dir = format!("{R}/proofs/{name}/transfer_confidential_1_2");
        let rounds = if panel == "preflight" { 1 } else { 51 };
        j["name"] = json!(name);
        j["proof_dir"] = json!(dir);
        j["workers"] = json!(workers);
        j["rounds"] = json!(rounds);
        j["block"] = json!(block);
        j["panel"] = json!(panel);
        let mut step = plan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == base_name)
            .unwrap()
            .clone();
        step["name"] = json!(name);
        step["argv"][2] = json!(format!("0-{}", workers - 1));
        step["environment"]["RAYON_NUM_THREADS"] = json!(workers.to_string());
        if mode == "zopro" {
            step["argv"][6] = json!(rounds.to_string());
            step["argv"][7] = json!(dir);
        } else {
            step["argv"][7] = json!(rounds.to_string());
            step["environment"]["AEGLOS_CPU_THREADS"] = json!(workers.to_string());
            step["environment"]["GOMAXPROCS"] = json!(workers.to_string());
            step["environment"]["ZOPRO_BENCH_PROOFS"] = json!(format!("{R}/proofs/{name}"));
        }
        let mut verify = plan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == format!("verify-{base_name}"))
            .unwrap()
            .clone();
        verify["name"] = json!(format!("verify-{name}"));
        verify["argv"][3] = json!(format!("{dir}/0.proof"));
        verify["argv"][4] = json!(format!("{dir}/0.public"));
        let inspect = json!({"kind":"command","name":format!("inspect-{name}"),"argv":[format!("{R}/local"),"inspect",name],"timeout_seconds":60});
        jobs.push(j);
        steps.push(step);
        if panel == "preflight" {
            steps.extend([verify, inspect]);
        } else {
            late.extend([verify, inspect]);
        }
    };
    for w in [16, 32] {
        for mode in ["zopro", "sorted", "windowed"] {
            for arm in if mode == "zopro" {
                vec!["REF", "OFF", "ON"]
            } else {
                vec!["REF", "FRESH", "OFF", "ON"]
            } {
                add(w, 0, "preflight", mode, arm);
            }
        }
    }
    for block in 0..9 {
        let mut counts = [16, 32];
        if block % 2 == 1 {
            counts.reverse();
        }
        let mut modes = ["zopro", "sorted", "windowed"];
        modes.rotate_left(block % 3);
        if block % 2 == 1 {
            modes.reverse();
        }
        for mode in modes {
            let mut arms = if mode == "zopro" {
                vec!["REF", "OFF", "ON"]
            } else {
                vec!["REF", "FRESH", "OFF", "ON"]
            };
            let n = arms.len();
            arms.rotate_left(block % n);
            if block % 2 == 1 {
                arms.reverse();
            }
            for w in counts {
                for arm in &arms {
                    add(w, block, "main", mode, arm);
                }
            }
        }
    }
    drop(add);
    assert_eq!(jobs.len(), 220);
    assert_eq!(
        jobs.iter()
            .map(|j| j["rounds"].as_u64().unwrap() + 3)
            .sum::<u64>(),
        10780
    );
    steps.push(json!({"kind":"command","name":"after-capture-state","argv":[format!("{R}/local"),"snapshot","after"],"timeout_seconds":60}));
    steps.extend(late);
    steps.push(json!({"kind":"command","name":"review","argv":[format!("{R}/local"),"review"],"timeout_seconds":120}));
    save("jobs.json", json!(jobs));
    for p in [
        format!("{R}/local"),
        format!("{R}/local.rs"),
        format!("{R}/validate.rs"),
        format!("{R}/make-validator.rs"),
        format!("{R}/protocol.md"),
        format!("{R}/jobs.json"),
        format!("{I}/proof-sha256-after-preflight.json"),
        format!("{I}/preflight-plan.json"),
    ] {
        pins.insert(p.clone(), json!(hash(p)));
    }
    save("pins.json", json!(pins));
    save(
        "plan.json",
        json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":steps}),
    );
    println!("Frozen220 processes10780SDKcalls; canonical55220->55440; no capture yet");
}
fn review() {
    verify_pins();
    let seed = read(format!("{I}/proof-sha256-after-preflight.json"));
    let mut ledger: BTreeSet<String> = serde_json::from_value(seed).unwrap();
    assert_eq!(ledger.len(), 55220);
    let jobs = read(format!("{R}/jobs.json"));
    let mut rows = vec![];
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let v = read(format!("{R}/checks/{name}.json"));
        assert_eq!(v["passed"], true);
        assert!(ledger.insert(v["proof_sha256"].as_str().unwrap().into()));
        if j["panel"] != "main" {
            continue;
        }
        let z = j["engine"] == "zopro";
        let ss = v["samples"].as_array().unwrap();
        let times = ss
            .iter()
            .map(|s| s[if z { "ms" } else { "verified_ms" }].as_f64().unwrap())
            .collect::<Vec<_>>();
        let mut pm: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        let mut geometry = BTreeSet::new();
        for (i, p) in v["phases"].as_array().unwrap().iter().skip(3).enumerate() {
            let mut sums = BTreeMap::<String, f64>::new();
            let mut counted = 0.;
            for row in p["rows"].as_array().unwrap() {
                let phase = row["phase"].as_str().unwrap();
                let dur = (row["end_ns"].as_u64().unwrap() - row["begin_ns"].as_u64().unwrap())
                    as f64
                    / 1e6;
                *sums.entry(phase.into()).or_default() += dur;
                if !z || !["A", "KH", "G2"].contains(&phase) {
                    counted += dur;
                }
            }
            let total = p["total_ns"].as_u64().unwrap() as f64 / 1e6;
            assert!(counted <= total + 1e-6);
            assert!(total <= times[i] + 1e-3);
            sums.insert("observer_total".into(), total);
            sums.insert("unassigned_within_observer".into(), total - counted);
            sums.insert("request_outside_observer".into(), times[i] - total);
            if z {
                let rr = p["rows"].as_array().unwrap();
                let query_end = rr[3]["end_ns"].as_u64().unwrap();
                for k in 4..7 {
                    sums.insert(
                        format!("{}_completion_tail", rr[k]["phase"].as_str().unwrap()),
                        (query_end - rr[k]["end_ns"].as_u64().unwrap()) as f64 / 1e6,
                    );
                }
            } else {
                let g=p["rows"].as_array().unwrap().iter().map(|r|json!({"phase":r["phase"],"items":r["items"],"job_items":r["job_items"]})).collect::<Vec<_>>();
                geometry.insert(serde_json::to_string(&g).unwrap());
            }
            for (k, n) in sums {
                pm.entry(k).or_default().push(n);
            }
        }
        let phase_means = pm
            .into_iter()
            .map(|(k, v)| (k, mean(&v)))
            .collect::<BTreeMap<_, _>>();
        let usage = &v["summary"];
        let cpu = (usage["user_cpu_ms"].as_f64().unwrap()
            + usage["system_cpu_ms"].as_f64().unwrap())
            / 51.;
        rows.push(json!({"job":j,"median_ms":median(&times),"mean_ms":mean(&times),"cpu_ms_per_request":cpu,"usage":usage,"phase_mean_ms":phase_means,"work_geometries":geometry.iter().map(|s|serde_json::from_str::<Value>(s).unwrap()).collect::<Vec<_>>() }));
    }
    assert_eq!(rows.len(), 198);
    assert_eq!(ledger.len(), 55440);
    let mut gates = vec![];
    let mut contexts = vec![];
    let mut scaling = vec![];
    let get = |mode: &str, w: usize, arm: &str, b: usize| {
        rows.iter()
            .find(|r| {
                r["job"]["mode"] == mode
                    && r["job"]["workers"] == w
                    && r["job"]["arm"] == arm
                    && r["job"]["block"] == b
            })
            .unwrap()
    };
    for mode in ["zopro", "sorted", "windowed"] {
        for w in [16, 32] {
            let pairs = if mode == "zopro" {
                vec![("OFF", "REF"), ("ON", "OFF")]
            } else {
                vec![("FRESH", "REF"), ("OFF", "FRESH"), ("ON", "OFF")]
            };
            for (num, den) in pairs {
                let raw = (0..9)
                    .map(|b| {
                        get(mode, w, num, b)["median_ms"].as_f64().unwrap()
                            / get(mode, w, den, b)["median_ms"].as_f64().unwrap()
                    })
                    .collect::<Vec<_>>();
                let mut ratios = raw.clone();
                ratios.sort_by(f64::total_cmp);
                gates.push(json!({"mode":mode,"workers":w,"ratio":format!("{num}/{den}"),"block_ratios":raw,"median":ratios[4],"interval":[ratios[1],ratios[7]],"passed":ratios[1]>=0.95&&ratios[7]<=1.05}));
            }
            for arm in if mode == "zopro" {
                vec!["REF", "OFF", "ON"]
            } else {
                vec!["REF", "FRESH", "OFF", "ON"]
            } {
                let rs = (0..9).map(|b| get(mode, w, arm, b)).collect::<Vec<_>>();
                let mut phases = BTreeMap::new();
                for key in rs[0]["phase_mean_ms"].as_object().unwrap().keys() {
                    phases.insert(
                        key.clone(),
                        mean(
                            &rs.iter()
                                .map(|r| r["phase_mean_ms"][key].as_f64().unwrap())
                                .collect::<Vec<_>>(),
                        ),
                    );
                }
                contexts.push(json!({"mode":mode,"workers":w,"arm":arm,"median_of_process_medians_ms":median(&rs.iter().map(|r|r["median_ms"].as_f64().unwrap()).collect::<Vec<_>>()),"pooled_mean_ms":mean(&rs.iter().map(|r|r["mean_ms"].as_f64().unwrap()).collect::<Vec<_>>()),"cpu_ms_per_request":mean(&rs.iter().map(|r|r["cpu_ms_per_request"].as_f64().unwrap()).collect::<Vec<_>>()),"max_rss_mib":rs.iter().map(|r|r["usage"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max),"phase_mean_ms":phases}));
            }
        }
        for arm in ["REF", "ON"] {
            let ratios = (0..9)
                .map(|b| {
                    get(mode, 32, arm, b)["median_ms"].as_f64().unwrap()
                        / get(mode, 16, arm, b)["median_ms"].as_f64().unwrap()
                })
                .collect::<Vec<_>>();
            scaling.push(json!({"mode":mode,"arm":arm,"median_32_over_16":median(&ratios),"improving_blocks":ratios.iter().filter(|&&v|v<1.).count(),"block_ratios":ratios}));
        }
    }
    save("rows.json", json!(rows));
    save("proof-sha256-after-capture.json", json!(ledger));
    save(
        "review.json",
        json!({"status":"complete","sdk_calls":10780,"exports":220,"canonical_count":55440,"canonical_sha256":hash(format!("{R}/proof-sha256-after-capture.json")),"all_gates_pass":gates.iter().all(|v|v["passed"]==true),"gates":gates,"contexts":contexts,"scaling":scaling,"production_change":false}),
    );
    println!("COMPLETE220 verified exports; canonical55440; interpret individual diagnostic gates");
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args[1].as_str() {
        "prepare" => prepare(),
        "inspect" => {
            let v = validate::inspect(&args[2]);
            fs::create_dir_all(format!("{R}/checks")).unwrap();
            save(&format!("checks/{}.json", args[2]), v);
        }
        "snapshot" => {
            save(&format!("{}.json", args[2]), state());
            if args[2] == "before" {
                let o = Command::new("lscpu")
                    .args(["-p=CPU,CORE,SOCKET,ONLINE"])
                    .output()
                    .unwrap();
                assert!(o.status.success());
                fs::write(format!("{R}/topology.csv"), o.stdout).unwrap();
            }
        }
        "review" => review(),
        _ => panic!("unknown operation"),
    }
}
