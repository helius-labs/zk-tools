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
const R: &str = "/home/ubuntu/src/zopro-perf-work/sdk-control-copies";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const MAN:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json";
const RECEIPT: &str =
    "/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json";
const ORIGINAL: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk";
const SDK_HASH: &str = "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-sdk-control-copies";
const VARIANTS: [&str; 3] = ["original-x", "copy-001-x", "copy-002-x"];
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
fn check(p: &Value) {
    for (f, h) in p.as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap(), "{f}")
    }
}
fn cmd(name: String, argv: Vec<String>, env: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":900})
}
fn base() -> Value {
    let mut e =
        read("/home/ubuntu/src/zopro-perf-work/center-census/run-plan.json")["environment"].clone();
    e["ZOPRO_BASIS_KEY_DIR"] = json!(r("images"));
    e["ZOPRO_CPU_LAGRANGE_DIR"] = json!(r("lagrange-cache"));
    e
}
fn link() {
    let a = read(RECEIPT);
    check(&a["input_pins"]);
    assert_eq!(hash(ORIGINAL), SDK_HASH);
    fs::copy(
        format!("{Z}/crates/zopro-bench/examples/cpu-euler.rs"),
        r("cpu-euler.rs"),
    )
    .unwrap();
    let libs = a["libraries"].as_object().unwrap();
    for (name, src) in [("raw", "raw.rs"), ("cpu-euler", "cpu-euler.rs")] {
        assert!(!r(name).exists());
        let mut args = vec![
            "--edition=2021".to_string(),
            "-O".into(),
            "-C".into(),
            "target-cpu=native".into(),
            "-L".into(),
            "dependency=/home/ubuntu/src/zopro-perf-work/mixed-sdk/target-control/release/deps"
                .into(),
        ];
        for (k, v) in libs {
            args.extend(["--extern".into(), format!("{k}={}", v.as_str().unwrap())]);
        }
        args.extend(["--extern".into(),"sha2=/home/ubuntu/src/zopro-perf-work/mixed-sdk/target-control/release/deps/libsha2-d6c69002ceb8f69f.rlib".into(),r(src).display().to_string(),"-o".into(),r(name).display().to_string()]);
        save(
            r(format!("link-{name}.json")),
            &json!({"argv":args,"accepted_receipt":RECEIPT,"native_libraries_rebuilt":false}),
        );
        let o = Command::new("rustc").args(args).output().unwrap();
        fs::write(r(format!("link-{name}.stdout.log")), o.stdout).unwrap();
        fs::write(r(format!("link-{name}.stderr.log")), o.stderr).unwrap();
        assert!(o.status.success(), "link {name}");
    }
}
fn restoration() {
    assert!(!r("restore-plan.json").exists());
    fs::create_dir_all(r("images")).unwrap();
    fs::create_dir_all(r("keys")).unwrap();
    fs::create_dir_all(r("lagrange-cache")).unwrap();
    let protocol = read(format!(
        "{Z}/docs/benchmarks/2026-10-27-clean-fft-leaves/artifacts/performance/protocol.json"
    ));
    let manifest = read(MAN);
    let lock = read(format!("{Z}/tools/zolana/zopro-keys.lock"));
    let mut cases = vec![];
    let mut steps = vec![];
    let mut pins = BTreeMap::new();
    for c in protocol["cases"].as_array().unwrap().iter().take(9) {
        let mut c = c.clone();
        let stem = Path::new(c["image"].as_str().unwrap())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let file = format!("{stem}.zopro");
        let key = r("keys").join(&file);
        let raw = r("images").join(format!("{stem}.raw"));
        let image = r("images").join(format!("{stem}.image"));
        let url = format!(
            "https://d3gbdb0egjwcw9.cloudfront.net/{}/{}",
            lock["prefix"].as_str().unwrap(),
            file
        );
        c["image"] = json!(image);
        c["raw"] = json!(raw);
        c["key"] = json!(key);
        c["key_sha256"] = lock["keys"][format!("{stem}.key")]["sha256"].clone();
        assert!(c["key_sha256"].is_string());
        for (suffix, dest) in [("raw", "raw_sha256"), ("euler", "image_sha256")] {
            let old =
                format!("/tmp/zopro-original-constants/small/candidate/{suffix}/{stem}.image");
            c[dest] = manifest["basis_image_hashes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["path"] == old)
                .unwrap()["sha256"]
                .clone();
        }
        steps.push(cmd(
            format!("download-{stem}"),
            vec![
                "curl".into(),
                "--fail".into(),
                "--silent".into(),
                "--show-error".into(),
                "--location".into(),
                "--max-time".into(),
                "300".into(),
                "--output".into(),
                key.display().to_string(),
                url,
            ],
            json!({}),
        ));
        // The key is checked before native parsing, not only after conversion.
        steps.push(cmd(
            format!("check-key-{stem}"),
            vec![
                r("study").display().to_string(),
                "check-file".into(),
                key.display().to_string(),
                c["key_sha256"].as_str().unwrap().into(),
            ],
            json!({}),
        ));
        steps.push(cmd(
            format!("raw-{stem}"),
            vec![
                r("raw").display().to_string(),
                key.display().to_string(),
                raw.display().to_string(),
            ],
            json!({"RAYON_NUM_THREADS":"16"}),
        ));
        steps.push(cmd(
            format!("check-raw-{stem}"),
            vec![
                r("study").display().to_string(),
                "check-file".into(),
                raw.display().to_string(),
                c["raw_sha256"].as_str().unwrap().into(),
            ],
            json!({}),
        ));
        steps.push(cmd(
            format!("euler-{stem}"),
            vec![
                r("cpu-euler").display().to_string(),
                "prepare".into(),
                raw.display().to_string(),
                image.display().to_string(),
            ],
            json!({"RAYON_NUM_THREADS":"16"}),
        ));
        steps.push(cmd(
            format!("check-euler-{stem}"),
            vec![
                r("study").display().to_string(),
                "check-file".into(),
                image.display().to_string(),
                c["image_sha256"].as_str().unwrap().into(),
            ],
            json!({}),
        ));
        cases.push(c);
    }
    save(r("cases.json"), &json!(cases));
    for p in [MAN, RECEIPT, ORIGINAL, VERIFY, X] {
        pin(&mut pins, p)
    }
    for p in [
        "design.md",
        "study.rs",
        "study",
        "raw.rs",
        "raw",
        "cpu-euler.rs",
        "cpu-euler",
        "cases.json",
    ] {
        pin(&mut pins, r(p))
    }
    pin(&mut pins, format!("{Z}/tools/zolana/zopro-keys.lock"));
    for (f, h) in read(MAN)["source_files_sha256"].as_object().unwrap() {
        let p = format!("{Z}/{f}");
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    for (f, h) in read(RECEIPT)["input_pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
        pin(&mut pins, f);
    }
    save(r("restore-pins.json"), &json!(pins));
    save(
        r("restore-plan.json"),
        &json!({"version":1,"source_pins":r("restore-pins.json"),"steps":steps}),
    );
    println!("Prepared54restorationsteps; all historical image hashes required");
}
fn prepare() {
    assert_eq!(read(r("restore/result.json"))["status"], "complete");
    assert!(!r("identities.json").exists());
    check(&read(r("restore-pins.json")));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("restore-pins.json"))).unwrap();
    let mut ids = BTreeMap::new();
    let mut inodes = BTreeSet::new();
    let m = fs::metadata(ORIGINAL).unwrap();
    assert_eq!(hash(ORIGINAL), SDK_HASH);
    for (name, fresh) in [
        (VARIANTS[0], false),
        (VARIANTS[1], true),
        (VARIANTS[2], true),
    ] {
        assert_eq!(name.len(), 10);
        if fresh {
            fs::write(r(name), fs::read(ORIGINAL).unwrap()).unwrap();
            fs::set_permissions(r(name), m.permissions()).unwrap()
        } else {
            fs::hard_link(ORIGINAL, r(name)).unwrap()
        }
        let n = fs::metadata(r(name)).unwrap();
        assert_eq!(hash(r(name)), SDK_HASH);
        assert_eq!(fresh, m.ino() != n.ino());
        assert!(inodes.insert(n.ino()));
        pin(&mut pins, r(name));
        ids.insert(name,json!({"path":r(name),"original":ORIGINAL,"fresh":fresh,"inode":n.ino(),"device":n.dev(),"bytes":n.len(),"sha256":hash(r(name)),"path_bytes":r(name).as_os_str().len()}));
    }
    save(r("identities.json"), &json!(ids));
    pin(&mut pins, r("identities.json"));
    let old = "/home/ubuntu/src/zopro-perf-work/rerandomized-glv-sdk";
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{old}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 13082);
    for (f, h) in read(format!("{old}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        if f.ends_with(".proof") {
            assert_eq!(hash(f), h.as_str().unwrap());
            assert!(prior.insert(h.as_str().unwrap().to_owned()));
        }
    }
    assert_eq!(prior.len(), 13181);
    save(r("prior-proof-sha256.json"), &json!(prior));
    pin(&mut pins, r("prior-proof-sha256.json"));
    for c in read(r("cases.json")).as_array().unwrap() {
        for (k, h) in [
            ("key", "key_sha256"),
            ("raw", "raw_sha256"),
            ("image", "image_sha256"),
        ] {
            assert_eq!(hash(c[k].as_str().unwrap()), c[h].as_str().unwrap());
            pin(&mut pins, c[k].as_str().unwrap());
        }
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    // Both FIFO names are constant across all counter runs.
    for p in ["shared-ctl", "shared-ack"] {
        assert!(Command::new("mkfifo").arg(r(p)).status().unwrap().success());
    }
    let mut steps = vec![];
    let mut jobs = vec![];
    for c in read(r("cases.json")).as_array().unwrap() {
        for v in VARIANTS {
            let name = format!("pre-{}-{v}", c["shape"].as_str().unwrap());
            let mut e = base();
            e["RAYON_NUM_THREADS"] = json!("4");
            let dir = r("proofs").join(&name);
            steps.push(cmd(
                name.clone(),
                vec![
                    "taskset".into(),
                    "-c".into(),
                    "0-3".into(),
                    r(v).display().to_string(),
                    c["image"].as_str().unwrap().into(),
                    c["request"].as_str().unwrap().into(),
                    "1".into(),
                    dir.display().to_string(),
                ],
                e,
            ));
            jobs.push(json!({"name":name,"phase":"preflight","workers":4,"rounds":1,"variant":v,"case":c,"directory":dir}));
        }
    }
    save(r("preflight-jobs.json"), &json!(jobs));
    pin(&mut pins, r("preflight-jobs.json"));
    save(r("preflight-pins.json"), &json!(pins));
    save(
        r("preflight-plan.json"),
        &json!({"version":1,"source_pins":r("preflight-pins.json"),"steps":steps}),
    );
    println!("Prepared27fixedpreflights; three identities frozen");
}
fn capture_plan() {
    assert_eq!(read(r("preflight/result.json"))["status"], "complete");
    assert!(!r("run-plan.json").exists());
    check(&read(r("preflight-pins.json")));
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("preflight-pins.json"))).unwrap();
    pin(&mut pins, r("preflight/result.json"));
    for e in fs::read_dir(r("lagrange-cache")).unwrap() {
        let p = e.unwrap().path();
        assert!(p.is_file());
        pin(&mut pins, p);
    }
    let mut cases = read(r("cases.json")).as_array().unwrap().clone();
    let mut extra = cases[0].clone();
    extra["name"] = json!("1x2-w16");
    extra["workers"] = json!(16);
    extra["cpu_affinity"] = json!("0-15");
    cases.push(extra);
    let mut jobs = vec![];
    let mut steps = vec![];
    let mut push = |c: &Value, v: &str, name: String, phase: &str, block: usize, panel: &str| {
        let rounds = if phase == "latency" && c["shape"] == "1x2" {
            151
        } else {
            31
        };
        let dir = r("proofs").join(&name);
        let mut env = base();
        env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
        let normal = vec![
            "taskset".to_string(),
            "-c".into(),
            c["cpu_affinity"].as_str().unwrap().into(),
            r(v).display().to_string(),
            c["image"].as_str().unwrap().into(),
            c["request"].as_str().unwrap().into(),
            rounds.to_string(),
            dir.display().to_string(),
        ];
        let argv = if phase == "counter" {
            env["ZOPRO_BENCH_PERF_CTL"] = json!(r("shared-ctl"));
            env["ZOPRO_BENCH_PERF_ACK"] = json!(r("shared-ack"));
            let event = if panel == "fetch" {
                "{cycles:u,instructions:u,L1-icache-load-misses:u,iTLB-load-misses:u}"
            } else {
                "{cycles:u,instructions:u,stalled-cycles-frontend:u,L1-dcache-load-misses:u}"
            };
            let mut a = vec!["sudo".into(), "-n".into(), "env".into()];
            for (k, v) in env.as_object().unwrap() {
                a.push(format!("{k}={}", v.as_str().unwrap()));
            }
            a.extend([
                "perf".into(),
                "stat".into(),
                "--no-big-num".into(),
                "-x".into(),
                ";".into(),
                "-e".into(),
                event.into(),
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
        steps.push(cmd(name.clone(), argv, env));
        jobs.push(json!({"name":name,"case":c,"variant":v,"phase":phase,"block":block,"panel":panel,"workers":c["workers"],"rounds":rounds,"directory":dir}));
    };
    for b in 0..15 {
        let mut order: Vec<usize> = (0..cases.len()).collect();
        order.rotate_left(b % cases.len());
        if b % 2 != 0 {
            order.reverse()
        }
        for i in order {
            let c = &cases[i];
            let mut vs = VARIANTS;
            vs.rotate_left((b + i) % 3);
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
                vs.rotate_left((b + i) % 3);
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
    assert_eq!(jobs.len(), 504);
    let mut all = read(r("preflight-jobs.json")).as_array().unwrap().clone();
    all.extend(jobs.clone());
    assert_eq!(all.len(), 531);
    for j in &all {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        steps.push(cmd(
            format!("verify-{}", j["name"].as_str().unwrap()),
            vec![
                VERIFY.into(),
                c["image"].as_str().unwrap().into(),
                c["image_sha256"].as_str().unwrap().into(),
                dir.join("0.proof").display().to_string(),
                dir.join("0.public").display().to_string(),
                c["public_sha256"].as_str().unwrap().into(),
            ],
            json!({}),
        ));
    }
    save(r("jobs.json"), &json!(jobs));
    save(r("all-jobs.json"), &json!(all));
    for p in ["jobs.json", "all-jobs.json"] {
        pin(&mut pins, r(p))
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared450latency+54counter captures then531Gnark verifications; frozen15blocks");
}
fn samples(j: &Value) -> (Vec<f64>, Value) {
    let folder = if j["phase"] == "preflight" {
        "preflight"
    } else {
        "runs"
    };
    let t = fs::read_to_string(
        r(folder)
            .join(j["name"].as_str().unwrap())
            .join("stdout.log"),
    )
    .unwrap();
    let v: Vec<f64> = t
        .lines()
        .filter_map(|l| l.strip_prefix("SAMPLE "))
        .map(|l| {
            serde_json::from_str::<Value>(l).unwrap()["ms"]
                .as_f64()
                .unwrap()
        })
        .collect();
    let s =
        serde_json::from_str(t.lines().find_map(|l| l.strip_prefix("SUMMARY ")).unwrap()).unwrap();
    (v, s)
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    check(&read(r("pins.json")));
    let jobs = read(r("all-jobs.json"));
    let jobs = jobs.as_array().unwrap();
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(seen.len(), 13181);
    let mut inv = BTreeMap::new();
    let mut requests = 0;
    let mut cells: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let mut counter: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for j in jobs {
        let (v, s) = samples(j);
        assert_eq!(v.len(), j["rounds"].as_u64().unwrap() as usize);
        assert_eq!(s["rounds"], j["rounds"]);
        assert_eq!(s["workers"], j["workers"]);
        assert_eq!(s["priming"], 3);
        requests += v.len() + 3;
        let name = j["name"].as_str().unwrap();
        let dir = Path::new(j["directory"].as_str().unwrap());
        let verify =
            fs::read_to_string(r("runs").join(format!("verify-{name}/stdout.log"))).unwrap();
        assert!(verify.contains("VERIFIED proof_sha256="));
        for ext in ["proof", "public"] {
            let p = dir.join(format!("0.{ext}"));
            let h = hash(&p);
            if ext == "proof" {
                assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                assert!(seen.insert(h.clone()));
                assert!(verify.contains(&format!("proof_sha256={h}")))
            } else {
                assert_eq!(h, j["case"]["public_sha256"].as_str().unwrap());
                assert!(verify.contains(&format!("public_sha256={h}")))
            }
            inv.insert(p.display().to_string(), h);
        }
        let key = format!(
            "{}/{}",
            j["case"]["name"].as_str().unwrap(),
            j["variant"].as_str().unwrap()
        );
        let row = json!({"name":name,"block":j["block"],"ms":median(&v),"cpu_ms_per_request":(s["user_cpu_ms"].as_f64().unwrap()+s["system_cpu_ms"].as_f64().unwrap())/v.len()as f64,"RSS_MiB":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"summary":s});
        if j["phase"] == "latency" {
            cells.entry(key).or_default().push(row)
        } else if j["phase"] == "counter" {
            let text = fs::read_to_string(r("runs").join(name).join("stderr.log")).unwrap();
            let mut events = BTreeMap::new();
            for l in text.lines() {
                let f: Vec<_> = l.trim().split(';').collect();
                if f.len() < 5 {
                    continue;
                }
                let event = f[2].strip_suffix(":u").unwrap_or(f[2]);
                if [
                    "cycles",
                    "instructions",
                    "L1-icache-load-misses",
                    "iTLB-load-misses",
                    "stalled-cycles-frontend",
                    "L1-dcache-load-misses",
                ]
                .contains(&event)
                {
                    let count: f64 = f[0].parse().unwrap();
                    let running: f64 = f[4].trim_end_matches('%').parse().unwrap();
                    assert!(running >= 99.9);
                    events.insert(event,json!({"per_request":count/v.len()as f64,"count":count,"running_percent":running}));
                }
            }
            assert_eq!(events.len(), 4);
            let mut row = row;
            row["events"] = json!(events);
            counter
                .entry(format!("{key}/{}", j["panel"].as_str().unwrap()))
                .or_default()
                .push(row);
        }
    }
    assert_eq!(requests, 28044);
    assert_eq!(seen.len(), 13712);
    assert_eq!(inv.len(), 1062);
    save(r("proof-inventory.json"), &json!(inv));
    let mut summary = BTreeMap::new();
    for (k, v) in &cells {
        assert_eq!(v.len(), 15);
        let ms: Vec<_> = v.iter().map(|x| x["ms"].as_f64().unwrap()).collect();
        let cpu: Vec<_> = v
            .iter()
            .map(|x| x["cpu_ms_per_request"].as_f64().unwrap())
            .collect();
        let rss: Vec<_> = v.iter().map(|x| x["RSS_MiB"].as_f64().unwrap()).collect();
        summary.insert(k,json!({"median_ms":median(&ms),"cpu_ms":median(&cpu),"median_RSS_MiB":median(&rss),"max_RSS_MiB":rss.iter().copied().fold(0.,f64::max),"blocks":v}));
    }
    let mut pairs = BTreeMap::new();
    for key in cells.keys().filter(|k| k.ends_with("/original-x")) {
        let case = key.strip_suffix("/original-x").unwrap();
        for v in ["copy-001-x", "copy-002-x"] {
            let cv = &cells[&format!("{case}/{v}")];
            let ov = &cells[key];
            let mut ratios = vec![];
            for b in 0..15 {
                assert_eq!(cv[b]["block"], ov[b]["block"]);
                ratios.push(cv[b]["ms"].as_f64().unwrap() / ov[b]["ms"].as_f64().unwrap());
            }
            let mut s = ratios.clone();
            s.sort_by(f64::total_cmp);
            pairs.insert(format!("{case}/{v}/original-x"),json!({"median":s[7],"low":s[3],"high":s[11],"wins":s.iter().filter(|&&x|x<1.).count(),"ratios":ratios,"nominal_coverage":0.96484375}));
        }
    }
    let mut counter_summary = BTreeMap::new();
    for (k, v) in &counter {
        assert_eq!(v.len(), 3);
        let mut m = BTreeMap::new();
        for event in v[0]["events"].as_object().unwrap().keys() {
            let a: Vec<_> = v
                .iter()
                .map(|x| x["events"][event]["per_request"].as_f64().unwrap())
                .collect();
            m.insert(event, median(&a));
        }
        counter_summary.insert(k, json!({"median_events_per_request":m,"observations":v}));
    }
    let mut refs = BTreeMap::new();
    let mut achieved = 0;
    for c in read(r("cases.json")).as_array().unwrap() {
        let shape = c["shape"].as_str().unwrap();
        if !c["reference"].is_null() {
            let ms = summary[&format!("{}-{}/original-x", shape, "w32")]["median_ms"]
                .as_f64()
                .unwrap();
            let cpu = c["reference"]["cpu_ms"].as_f64().unwrap();
            let metal = c["reference"]["metal_ms"].as_f64().unwrap();
            let ok = ms <= cpu / 10.;
            if ok {
                achieved += 1
            }
            refs.insert(shape.to_owned(),json!({"original_ms":ms,"cpu_reference_ms":cpu,"metal_reference_ms":metal,"cpu_speedup":cpu/ms,"metal_speedup":metal/ms,"CPU10x":ok,"required_ms":cpu/10.}));
        }
    }
    let a = json!({"status":"complete diagnostic; production unchanged","cells":summary,"pairs":pairs,"counters":counter_summary,"m5":refs,"SDK_requests":requests,"new_independent_exports":531,"independent_proof_count":13712,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":achieved,"new_native_tests":0,"inherited_qualification":"unchanged accepted executable bytes","scope":"all eight reference shapes and54 local verified SDK; frontend counters primary16/32 and54/32; no AWS/mobile/memory-pressure claim"});
    save(r("analysis.json"), &a);
    for (k, v) in summary {
        println!(
            "{k}: {:.6}ms {:.2}MiB",
            v["median_ms"].as_f64().unwrap(),
            v["median_RSS_MiB"].as_f64().unwrap()
        );
    }
    for (k, v) in pairs {
        println!("{k}: {} [{},{}]", v["median"], v["low"], v["high"]);
    }
    println!("Verified28044requests/531freshGnarkexports;ledger13712;goal{achieved}/8");
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let q = e.path();
        if e.file_type().unwrap().is_dir() {
            if !["images", "keys", "lagrange-cache"]
                .contains(&e.file_name().to_string_lossy().as_ref())
            {
                walk(&q, out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(q)
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    check(&read(r("pins.json")));
    assert_eq!(read(r("analysis.json"))["independent_proof_count"], 13712);
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let dst = Path::new(D);
    fs::create_dir_all(dst.join("artifacts")).unwrap();
    let mut local = BTreeMap::new();
    for p in files {
        let rel = p.strip_prefix(R).unwrap();
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        if rel.to_string_lossy().starts_with("git-") {
            continue;
        }
        let q = dst.join("artifacts").join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::write(q, bytes).unwrap();
    }
    save(
        dst.join("artifacts/local-executable-pins.json"),
        &json!(local),
    );
    assert!(r("report.md").exists());
    fs::copy(r("report.md"), dst.join("README.md")).unwrap();
    let mut list = vec![];
    walk(dst, &mut list);
    list.sort();
    let mut sums = String::new();
    let mut index = BTreeMap::new();
    for p in list {
        let rel = p.strip_prefix(dst).unwrap().display().to_string();
        let h = hash(&p);
        sums.push_str(&format!("{h}  {rel}\n"));
        index.insert(rel, h);
    }
    fs::write(dst.join("SHA256SUMS"), sums).unwrap();
    save(dst.join("artifact-index.json"), &json!(index));
    println!("Archived{}hashes", index.len());
}
fn state() {
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(head.status.success());
    let head = String::from_utf8(head.stdout).unwrap().trim().to_string();
    let remote = Command::new("git")
        .args([
            "-c",
            "url.https://github.com/.insteadOf=git@github.com:",
            "-c",
            "credential.helper=!gh auth git-credential",
            "ls-remote",
            "origin",
            "refs/heads/sergei/zopro-cpu-perf",
        ])
        .output()
        .unwrap();
    assert!(remote.status.success());
    assert!(String::from_utf8(remote.stdout).unwrap().starts_with(&head));
    let a = read(r("analysis.json"));
    let path = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(path);
    s["previous_completed_experiment"] =
        json!({"commit":s["commit"],"report":s["report"],"status":s["status"]});
    s["commit"] = json!(head);
    s["root"] = json!(R);
    s["report"] = json!(format!("{D}/README.md"));
    s["status"] =
        json!("Accepted SDK backing diagnostic complete and pushed; production unchanged");
    s["sdk_control_copies_diagnostic"] = a;
    s["independent_proof_count"] = json!(13712);
    s["new_independent_exports"] = json!(531);
    s["SDK_requests"] = json!(28044);
    s["owned_live_jobs"] = json!([]);
    s["pushed"] = json!(true);
    s["remote_verified"] = json!(true);
    s["last_goal_turn_classification"] = json!("progress");
    s["previous_status_turn_classification"] =
        json!("no progress; next available safe action executed");
    s["next_action"]=json!("Use full SDK backing/frontend evidence to select the next concrete arithmetic or caller change; no copy cherry-picking or unchanged reroll. Original all8 CPU10x goal remains active.");
    save(path, &s);
    let h = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(h).unwrap();
    fs::write(h,format!("CURRENT COMPLETE/PUSHED — ACCEPTED SDK BYTE-EXACT COPIES\n\nHEAD/remoteverified{head};root{R};report{D}/README.md. Full details analysis.json/report. Accepted SDK14da original+two prospectively fixed byteexactcopies; restored all9raw/eulerimages matchhistoricalhashes;172production sources unchanged.27preflight+450latency+54counter=531SDKcaptures;28044internallyverifiedrequests;531freshGnarkexports/1062members;proofledger13712. All8CPU/Metal comparisons and54,primary16/32and54frontendcounters. No optimization adoption/goalpromotion; goalACTIVE. Read report for findings/nextaction. Allcaptures complete,noownedjobs/cloud. Previous status-only turn no progress; this turn actual diagnostic progress.\n\n{old}")).unwrap();
    println!("State and handoff updated {head}");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("link") => link(),
        Some("restoration") => restoration(),
        Some("prepare") => prepare(),
        Some("capture-plan") => capture_plan(),
        Some("finish") => finish(),
        Some("archive") => archive(),
        Some("state") => state(),
        Some("check-file") => {
            assert_eq!(hash(&a[2]), a[3]);
            println!("SHA256 verified {}", a[2]);
        }
        _ => panic!("link/restoration/prepare/capture-plan/finish/archive/state/check-file"),
    }
}
