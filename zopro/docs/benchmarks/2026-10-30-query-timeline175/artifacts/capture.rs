use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/query-timeline175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g2-balanced-sdk175";
const SDK: &str = "/home/ubuntu/src/zopro-perf-work/query-timeline175/sdk/runs/timeline-sdk";
const ACCEPTED: &str =
    "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
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
fn pin(pins: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    pins.insert(p.as_ref().display().to_string(), hash(&p));
}
fn verify(j: &Value) -> Value {
    let c = &j["case"];
    let d = Path::new(j["directory"].as_str().unwrap());
    json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],d.join("0.proof"),d.join("0.public"),c["public_sha256"]],"timeout_seconds":900})
}
fn prepare() {
    assert!(!r("plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    assert_eq!(read(r("sdk/runs/result.json"))["status"], "complete");
    let mut pins = BTreeMap::new();
    let build = read(r("build-pins.json"));
    for (p, h) in build.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().to_string());
    }
    let libs = read(r("frozen-control.json"));
    for (p, h) in libs["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().to_string());
    }
    let current = Path::new(Z).join("docs/benchmarks/2026-10-29-current-cpu-summary");
    assert_eq!(
        read(current.join("data.json"))["historical_distinct_Gnark_ledger"],
        24943
    );
    fs::create_dir_all(r("inherited-summary")).unwrap();
    for f in ["README.md", "data.json", "source-hashes.json"] {
        fs::copy(current.join(f), r("inherited-summary").join(f)).unwrap();
        pin(&mut pins, r("inherited-summary").join(f));
    }
    fs::copy(
        Path::new(OLD).join("proof-sha256-after.json"),
        r("prior-proof-sha256.json"),
    )
    .unwrap();
    assert_eq!(
        read(r("prior-proof-sha256.json")).as_array().unwrap().len(),
        24943
    );
    let old_cases = read(Path::new(OLD).join("cases.json"));
    let base = read(Path::new(OLD).join("plan.json"));
    let env = base["steps"][0]["environment"].clone();
    let original = old_cases
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "1x2-w32")
        .unwrap();
    let mut cases = vec![];
    for (workers, affinity) in [(1, "2"), (16, "0-15"), (32, "0-31")] {
        let mut c = original.clone();
        c["name"] = json!(format!("1x2-w{workers}"));
        c["workers"] = json!(workers);
        c["cpu_affinity"] = json!(affinity);
        cases.push(c);
    }
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut add = |c: &Value, v: &str, block: usize, panel: &str| {
        let n = if panel == "preflight" {
            format!("preflight-{}-{v}", c["name"].as_str().unwrap())
        } else {
            format!("b{block}-{}-{v}", c["name"].as_str().unwrap())
        };
        let rounds = if panel == "preflight" { 1 } else { 151 };
        let mut e = env.clone();
        e["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
        e["ZOPRO_QUERY_TIMELINE"] = json!(if v == "traced" { "1" } else { "0" });
        let binary = if v == "accepted" { ACCEPTED } else { SDK };
        let dir = r(format!("proofs/{n}"));
        let j = json!({"name":n,"case":c,"variant":v,"block":block,"panel":panel,"rounds":rounds,"directory":dir,"binary":binary});
        steps.push(json!({"kind":"command","name":n,"argv":["taskset","-c",c["cpu_affinity"],binary,c["image"],c["request"],rounds.to_string(),dir],"environment":e,"timeout_seconds":900}));
        if panel == "preflight" {
            steps.push(verify(&j));
        }
        jobs.push(j);
    };
    for c in &cases {
        for v in ["accepted", "OFF", "traced"] {
            add(c, v, 0, "preflight");
        }
    }
    for b in 0..31 {
        let mut cs = cases.clone();
        let len = cs.len();
        cs.rotate_left(b % len);
        if b % 2 == 1 {
            cs.reverse();
        }
        let mut vs = vec!["accepted", "OFF", "traced"];
        vs.rotate_left(b % 3);
        if b % 2 == 1 {
            vs.reverse();
        }
        for c in &cs {
            for v in &vs {
                add(c, v, b, "latency");
            }
        }
    }
    for j in &jobs {
        if j["panel"] != "preflight" {
            steps.push(verify(j));
        }
    }
    assert_eq!(jobs.len(), 288);
    assert_eq!(steps.len(), 576);
    for p in [
        "capture.rs",
        "capture-helper",
        "analysis.rs",
        "publication.rs",
        "design.md",
        "capture-protocol.md",
        "review.rs",
        "review",
        "query_timeline.rs",
        "source-audit.json",
        "prepare-sdk.rs",
        "prepare-sdk",
        "sdk/source-audit.json",
        "sdk/cpu-sdk.rs",
        "sdk/support/perf.rs",
        "sdk/runs/timeline/artifacts.json",
        "native/control-core/tests.json",
        "native/control-prover/tests.json",
        "prior-proof-sha256.json",
    ] {
        pin(&mut pins, r(p));
    }
    for p in [
        SDK,
        ACCEPTED,
        VERIFY,
        "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask",
    ] {
        pin(&mut pins, p);
    }
    for key in ["image", "raw", "key", "request"] {
        pin(&mut pins, original[key].as_str().unwrap());
    }
    for path in [
        Path::new(OLD).join("cases.json"),
        Path::new(OLD).join("plan.json"),
        Path::new(OLD).join("proof-sha256-after.json"),
    ] {
        pin(&mut pins, path);
    }
    save(r("cases.json"), &json!(cases));
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("cases.json"));
    pin(&mut pins, r("jobs.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared576steps/288processes/43002internally verified requests planned;288fresh Gnark exports required; no capture yet");
}
fn median(x: &[f64]) -> f64 {
    let mut x = x.to_vec();
    x.sort_by(f64::total_cmp);
    if x.len() % 2 == 1 {
        x[x.len() / 2]
    } else {
        (x[x.len() / 2 - 1] + x[x.len() / 2]) / 2.
    }
}
fn stats(x: &[f64]) -> Value {
    assert_eq!(x.len(), 31);
    let mut sorted = x.to_vec();
    sorted.sort_by(f64::total_cmp);
    json!({"median":sorted[15],"low":sorted[9],"high":sorted[21],"wins":x.iter().filter(|&&v|v<1.).count(),"ratios":x,"nominal_individual_coverage":0.9705506265163422})
}
include!("analysis.rs");
include!("publication.rs");
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        Some("publish") => publish(),
        _ => panic!("prepare/finish/publish"),
    }
}
