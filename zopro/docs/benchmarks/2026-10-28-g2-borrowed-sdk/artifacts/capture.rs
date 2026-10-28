use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk";
const P: &str = "/home/ubuntu/src/zopro-perf-work/sdk-control-copies";
const C: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const ORIGINAL: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk";
const VARIANTS: [&str; 4] = ["accepted", "frozen", "off", "on"];
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
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn name(v: &str, heap: bool) -> String {
    format!(
        "{}-{}",
        match v {
            "accepted" => "accept",
            "frozen" => "frozen",
            "off" => "offset",
            "on" => "borrow",
            _ => panic!("variant"),
        },
        if heap { "mem" } else { "sdk" }
    )
}
fn base() -> Value {
    let mut e =
        read("/home/ubuntu/src/zopro-perf-work/center-census/run-plan.json")["environment"].clone();
    e["ZOPRO_BASIS_KEY_DIR"] = json!(format!("{P}/images"));
    e["ZOPRO_CPU_LAGRANGE_DIR"] = json!(format!("{P}/lagrange-cache"));
    e
}
fn main() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native-links/result.json"))["status"], "complete");
    assert_eq!(
        read(r("control-locations.json"))["native_code_or_arithmetic_changed"],
        false
    );
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (f, h) in &pins {
        assert_eq!(hash(f), *h)
    }
    assert_eq!(
        hash(ORIGINAL),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    pin(&mut pins, ORIGINAL);
    let mut ids = BTreeMap::new();
    let mut inodes = BTreeSet::new();
    for v in VARIANTS {
        for heap in [false, true] {
            let source = if v == "accepted" && !heap {
                PathBuf::from(ORIGINAL)
            } else {
                r("native-links").join(format!(
                    "{}-sdk",
                    if heap { format!("heap-{v}") } else { v.into() }
                ))
            };
            let alias = r(name(v, heap));
            assert_eq!(alias.file_name().unwrap().len(), 10);
            fs::hard_link(&source, &alias).unwrap();
            let meta = fs::metadata(&alias).unwrap();
            assert_eq!(hash(&source), hash(&alias));
            assert!(inodes.insert(meta.ino()));
            pin(&mut pins, &source);
            pin(&mut pins, &alias);
            ids.insert(format!("{v}/{}",if heap{"heap"}else{"ordinary"}),json!({"source":source,"path":alias,"inode":meta.ino(),"device":meta.dev(),"sha256":hash(&alias),"bytes":meta.len(),"path_bytes":alias.as_os_str().len()}));
        }
    }
    save(r("identities.json"), &json!(ids));
    for label in [
        "frozen",
        "off",
        "on",
        "heap-accepted",
        "heap-frozen",
        "heap-off",
        "heap-on",
    ] {
        let p = r(format!("native-links/{label}/artifacts.json"));
        let a = read(&p);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (f, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(f), h.as_str().unwrap());
            pin(&mut pins, f);
        }
        pin(&mut pins, p);
    }
    let mut cases = read(format!("{P}/cases.json")).as_array().unwrap().clone();
    assert_eq!(cases.len(), 9);
    for c in &cases {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    let mut extra = cases[0].clone();
    extra["name"] = json!("1x2-w16");
    extra["workers"] = json!(16);
    extra["cpu_affinity"] = json!("0-15");
    cases.push(extra);
    save(r("cases.json"), &json!(cases));
    for e in fs::read_dir(format!("{P}/lagrange-cache")).unwrap() {
        let p = e.unwrap().path();
        assert!(p.is_file());
        pin(&mut pins, p);
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{P}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 13181);
    for (f, h) in read(format!("{P}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        if f.ends_with(".proof") {
            assert_eq!(hash(f), h.as_str().unwrap());
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 13712);
    save(r("prior-proof-sha256.json"), &json!(prior));
    for p in ["shared-ctl", "shared-ack"] {
        assert!(Command::new("mkfifo").arg(r(p)).status().unwrap().success());
    }
    for (p, args) in [
        (
            "host-before.txt",
            vec![
                "ps",
                "-eo",
                "pid,ppid,comm,pcpu,pmem,etimes",
                "--sort=-pcpu",
            ],
        ),
        ("compiler.txt", vec!["rustc", "-vV"]),
        ("lscpu.txt", vec!["lscpu"]),
    ] {
        let o = Command::new(args[0]).args(&args[1..]).output().unwrap();
        assert!(o.status.success());
        fs::write(r(p), o.stdout).unwrap();
        pin(&mut pins, r(p));
    }
    let mut steps = vec![];
    let mut jobs = vec![];
    let mut profiles = vec![];
    let mut push = |case: &Value, v: &str, job: String, phase: &str, b: usize, panel: &str| {
        let mut c = case.clone();
        let w = if phase == "preflight" {
            4
        } else {
            c["workers"].as_u64().unwrap()
        };
        let mask = if w == 4 {
            "0-3"
        } else if w == 16 {
            "0-15"
        } else {
            "0-31"
        };
        c["workers"] = json!(w);
        c["cpu_affinity"] = json!(mask);
        let rounds = match phase {
            "preflight" | "resource" => 1,
            "profile" => 251,
            "latency" if c["shape"] == "1x2" => 151,
            _ => 31,
        };
        let mut env = base();
        env["RAYON_NUM_THREADS"] = json!(w.to_string());
        let dir = r("proofs").join(&job);
        let normal = vec![
            "taskset".to_string(),
            "-c".into(),
            mask.into(),
            r(name(v, phase == "resource")).display().to_string(),
            c["image"].as_str().unwrap().into(),
            c["request"].as_str().unwrap().into(),
            rounds.to_string(),
            dir.display().to_string(),
        ];
        let argv = if ["counter", "profile"].contains(&phase) {
            env["ZOPRO_BENCH_PERF_CTL"] = json!(r("shared-ctl"));
            env["ZOPRO_BENCH_PERF_ACK"] = json!(r("shared-ack"));
            let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in env.as_object().unwrap() {
                a.push(format!("{k}={}", v.as_str().unwrap()));
            }
            if phase == "counter" {
                let events = if panel == "fetch" {
                    "{cycles:u,instructions:u,L1-icache-load-misses:u,iTLB-load-misses:u}"
                } else {
                    "{cycles:u,instructions:u,stalled-cycles-frontend:u,L1-dcache-load-misses:u}"
                };
                a.extend([
                    "perf".into(),
                    "stat".into(),
                    "--no-big-num".into(),
                    "-x".into(),
                    ";".into(),
                    "-e".into(),
                    events.into(),
                ]);
            } else {
                let pd = r("profiles").join(&job);
                fs::create_dir_all(&pd).unwrap();
                profiles.push(job.clone());
                a.extend([
                    "perf".into(),
                    "record".into(),
                    "--no-buildid-cache".into(),
                    "-e".into(),
                    "cycles:u".into(),
                    "-F".into(),
                    "199".into(),
                    "-m".into(),
                    "512".into(),
                    "-o".into(),
                    pd.join("perf.data").display().to_string(),
                ]);
            }
            a.extend([
                "--delay=-1".into(),
                format!(
                    "--control=fifo:{},{}",
                    r("shared-ctl").display(),
                    r("shared-ack").display()
                ),
                "--".into(),
            ]);
            a.extend(normal);
            a
        } else {
            normal
        };
        steps.push(json!({"kind":"command","name":job,"argv":argv,"environment":env,"timeout_seconds":900}));
        jobs.push(json!({"name":job,"case":c,"workers":w,"rounds":rounds,"variant":v,"phase":phase,"block":b,"panel":panel,"directory":dir}));
    };
    for c in cases.iter().take(9) {
        for v in VARIANTS {
            push(
                c,
                v,
                format!("pre-{}-{v}", c["shape"].as_str().unwrap()),
                "preflight",
                0,
                "",
            );
        }
    }
    for b in 0..15 {
        let mut order: Vec<usize> = (0..10).collect();
        order.rotate_left(b % 10);
        if b % 2 != 0 {
            order.reverse()
        }
        for i in order {
            let c = &cases[i];
            let mut vs = VARIANTS;
            vs.rotate_left((b + i) % 4);
            if (b + i) % 2 != 0 {
                vs.reverse()
            }
            for v in vs {
                push(
                    c,
                    v,
                    format!("b{b:02}-{}-{v}", c["name"].as_str().unwrap()),
                    "latency",
                    b,
                    "",
                );
            }
        }
    }
    for b in 0..3 {
        for (i, c) in [&cases[0], &cases[9], &cases[8]].into_iter().enumerate() {
            for panel in ["fetch", "stall"] {
                let mut vs = VARIANTS;
                vs.rotate_left((b + i) % 4);
                if (b + i) % 2 != 0 {
                    vs.reverse()
                }
                for v in vs {
                    push(
                        c,
                        v,
                        format!("p{b}-{}-{v}-{panel}", c["name"].as_str().unwrap()),
                        "counter",
                        b,
                        panel,
                    );
                }
            }
        }
    }
    for c in [&cases[0], &cases[9]] {
        for v in ["accepted", "off", "on"] {
            push(
                c,
                v,
                format!("profile-{}-{v}", c["name"].as_str().unwrap()),
                "profile",
                0,
                "",
            );
        }
    }
    for b in 0..3 {
        for (i, c) in [&cases[0], &cases[9], &cases[8]].into_iter().enumerate() {
            let mut vs = VARIANTS;
            vs.rotate_left((b + i) % 4);
            if (b + i) % 2 != 0 {
                vs.reverse()
            }
            for v in vs {
                push(
                    c,
                    v,
                    format!("m{b}-{}-{v}", c["name"].as_str().unwrap()),
                    "resource",
                    b,
                    "",
                );
            }
        }
    }
    assert_eq!(jobs.len(), 750);
    assert_eq!(profiles.len(), 6);
    for j in &jobs {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        steps.push(json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":180}));
    }
    for p in profiles {
        let file = r("profiles").join(&p).join("perf.data");
        steps.push(json!({"kind":"command","name":format!("owner-{p}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",file],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{p}"),"argv":["perf","script","--header","--show-lost-events","-i",file,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{p}"),"argv":["perf","script","-D","-i",file],"timeout_seconds":120}));
    }
    assert_eq!(steps.len(), 1518);
    save(r("jobs.json"), &json!(jobs));
    for p in [
        "design.md",
        "capture.rs",
        "capture",
        "finish.rs",
        "finish",
        "close.rs",
        "close",
        "source-audit.json",
        "identities.json",
        "cases.json",
        "prior-proof-sha256.json",
        "jobs.json",
        "sdk-control-sections.rs",
        "sdk-control-sections",
        "sdk-control-locations.rs",
        "sdk-control-locations",
        "control-sections.json",
        "control-locations.json",
        "qualification-inheritance.json",
        "native-links/result.json",
    ] {
        pin(&mut pins, r(p));
    }
    for p in [
        VERIFY,
        "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask",
    ] {
        pin(&mut pins, p);
    }
    pin(&mut pins, format!("{P}/analysis.json"));
    pin(&mut pins, format!("{C}/analysis.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared750SDKcapture/750Gnark/18decode steps;39060requests;ledger13712->14462 only after verification");
}
