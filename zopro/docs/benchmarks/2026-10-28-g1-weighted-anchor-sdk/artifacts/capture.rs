use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor-sdk";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const CONTROL: &str = "/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control-sdk";
const IMAGE:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-25-worker-groups-request-entry/qualification/warm-default-w32-single-request/input/transfer_confidential_1_2.image";
const IH: &str = "16a3e2a686372d531879d4ff99e63eab0ca374d686f05a9d7343d2bf534dd041";
const PH: &str = "3f0d846c3905d11595d77dd988df829ddc51325eda2b93e4db8f0e17a5928e5b";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
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
fn pin(m: &mut BTreeMap<String, String>, p: impl AsRef<Path>) {
    let p = p.as_ref();
    m.insert(p.display().to_string(), hash(p));
}
fn pins() -> BTreeMap<String, String> {
    let mut p: BTreeMap<String, String> =
        serde_json::from_value(read(format!("{R}/build-pins.json"))).unwrap();
    for (f, h) in &p {
        assert_eq!(hash(f), *h);
    }
    for name in [
        "design.md",
        "capture.rs",
        "capture",
        "finish.rs",
        "finish",
        "make-sdk.rs",
        "make-sdk",
        "build-plan.json",
        "native/result.json",
    ] {
        pin(&mut p, Path::new(R).join(name));
    }
    for label in ["off", "on"] {
        let a = read(format!("{R}/frozen-{label}.json"));
        for (f, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            pin(&mut p, f);
        }
        pin(&mut p, format!("{R}/frozen-{label}.json"));
    }
    pin(&mut p, X);
    pin(
        &mut p,
        format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),
    );
    pin(
        &mut p,
        format!("{Z}/crates/zopro-bench/examples/support/perf.rs"),
    );
    p
}

fn prepare() {
    let r = Path::new(R);
    assert!(!r.join("run-plan.json").exists());
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    let mut pins = pins();
    for label in ["off", "on"] {
        let a = read(r.join(format!("native/{label}/artifacts.json")));
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
        let binary = r.join(format!("native/{label}-sdk"));
        assert_eq!(hash(&binary), a["binary_sha256"].as_str().unwrap());
        pin(&mut pins, binary);
        pin(&mut pins, r.join(format!("native/{label}/artifacts.json")));
        pin(&mut pins, r.join(format!("frozen-{label}-cargo.jsonl")));
    }
    let off = read(r.join("native/off/artifacts.json"));
    let on = read(r.join("native/on/artifacts.json"));
    assert_ne!(off["binary_sha256"], on["binary_sha256"]);
    for name in ["zopro_bn254", "zopro_core"] {
        assert_ne!(
            hash(off["libraries"][name].as_str().unwrap()),
            hash(on["libraries"][name].as_str().unwrap())
        );
    }
    assert_eq!(
        hash(CONTROL),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    for (p, h) in
        read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json")
            ["input_pins"]
            .as_object()
            .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    assert_eq!(hash(IMAGE), IH);
    for p in [CONTROL, IMAGE, VERIFY] {
        pin(&mut pins, p);
    }
    let request = format!("{Z}/tools/zolana/fixtures/transfer-1x2.json");
    pin(&mut pins, &request);
    let mut prior: BTreeSet<String> = serde_json::from_value(read(
        "/home/ubuntu/src/zopro-perf-work/scalar-square-sdk/prior-proof-sha256.json",
    ))
    .unwrap();
    assert_eq!(prior.len(), 12884);
    for (p, h) in read("/home/ubuntu/src/zopro-perf-work/scalar-square-sdk/proof-inventory.json")
        .as_object()
        .unwrap()
    {
        if p.ends_with(".proof") {
            assert_eq!(hash(p), h.as_str().unwrap());
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 12983);
    save(r.join("prior-proof-sha256.json"), &json!(prior));
    pin(&mut pins, r.join("prior-proof-sha256.json"));
    pin(&mut pins, r.join("finish.rs"));
    pin(&mut pins, r.join("finish"));
    let base =
        read("/home/ubuntu/src/zopro-perf-work/center-census/run-plan.json")["environment"].clone();
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut profiles = vec![];
    let mut push = |name: String,
                    phase: &str,
                    block: usize,
                    w: usize,
                    label: &str,
                    rounds: usize| {
        let bin = if label == "control" {
            CONTROL.to_string()
        } else {
            r.join(format!("native/{label}-sdk")).display().to_string()
        };
        let dir = r.join("proofs").join(&name);
        let mut env = base.clone();
        env["RAYON_NUM_THREADS"] = json!(w.to_string());
        let mask = if w == 32 {
            "0-31"
        } else if w == 16 {
            "0-15"
        } else {
            "0-3"
        };
        let normal = vec![
            "taskset".to_string(),
            "-c".into(),
            mask.into(),
            bin.clone(),
            IMAGE.into(),
            request.clone(),
            rounds.to_string(),
            dir.display().to_string(),
        ];
        let argv = if phase == "profile" {
            let pd = r.join("profiles").join(&name);
            fs::create_dir_all(&pd).unwrap();
            assert!(std::process::Command::new("mkfifo")
                .arg(pd.join("ctl"))
                .arg(pd.join("ack"))
                .status()
                .unwrap()
                .success());
            env["ZOPRO_BENCH_PERF_CTL"] = json!(pd.join("ctl"));
            env["ZOPRO_BENCH_PERF_ACK"] = json!(pd.join("ack"));
            let mut cmd = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in env.as_object().unwrap() {
                cmd.push(format!("{k}={}", v.as_str().unwrap()));
            }
            cmd.extend([
                "perf".into(),
                "record".into(),
                "--no-buildid-cache".into(),
                "-e".into(),
                "cycles:u".into(),
                "-F".into(),
                "199".into(),
                "-m".into(),
                "512".into(),
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    pd.join("ctl").display(),
                    pd.join("ack").display()
                ),
                "-o".into(),
                pd.join("perf.data").display().to_string(),
                "--".into(),
            ]);
            cmd.extend(normal);
            cmd
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":240}));
        jobs.push(json!({"name":name,"phase":phase,"block":block,"workers":w,"variant":label,"rounds":rounds,"directory":dir}));
    };
    for label in ["control", "off", "on"] {
        push(format!("preflight-{label}"), "preflight", 0, 4, label, 1);
    }
    for block in 0..15 {
        let mut ws = [32, 16];
        if block % 2 != 0 {
            ws.reverse();
        }
        for (i, w) in ws.into_iter().enumerate() {
            let mut labels = vec!["control", "off", "on"];
            labels.rotate_left((block + i) % 3);
            if (block + i) % 2 != 0 {
                labels.reverse();
            }
            for label in labels {
                push(
                    format!("b{block}-w{w}-{label}"),
                    "latency",
                    block,
                    w,
                    label,
                    151,
                );
            }
        }
    }
    for w in [32, 16] {
        for label in ["control", "off", "on"] {
            let name = format!("profile-w{w}-{label}");
            push(name.clone(), "profile", 0, w, label, 251);
            profiles.push(name);
        }
    }
    assert_eq!(jobs.len(), 99);
    for job in &jobs {
        let name = job["name"].as_str().unwrap();
        let dir = job["directory"].as_str().unwrap();
        steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":[VERIFY,IMAGE,IH,format!("{dir}/0.proof"),format!("{dir}/0.public"),PH],"timeout_seconds":120}));
    }
    for name in profiles {
        let file = r.join("profiles").join(&name).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",file],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",file,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",file],"timeout_seconds":120}));
    }
    save(r.join("jobs.json"), &json!(jobs));
    pin(&mut pins, r.join("jobs.json"));
    save(r.join("pins.json"), &json!(pins));
    save(
        r.join("run-plan.json"),
        &json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps}),
    );
    println!(
        "Prepared90latency+3preflight+6profiles;15396SDK99exports;{}pins",
        pins.len()
    );
}

fn main() {
    prepare();
}
