use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/stream-ibs175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
const XTASK: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn r(s: impl AsRef<Path>) -> PathBuf {
    Path::new(ROOT).join(s)
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
        assert_eq!(hash(f), h.as_str().unwrap(), "pin {f}");
    }
}
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 0 {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.
    } else {
        v[v.len() / 2]
    }
}
fn prepare() {
    assert!(!r("run-plan.json").exists());
    let manifest = read(MANIFEST);
    let mut pins = BTreeMap::new();
    pin(&mut pins, MANIFEST);
    for (f, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let p = Path::new(Z).join(f);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    assert_eq!(
        manifest["source_files_sha256"].as_object().unwrap().len(),
        175
    );
    let sdk = manifest["native_binary"].as_str().unwrap();
    assert_eq!(hash(sdk), manifest["native_binary_sha256"]);
    pin(&mut pins, sdk);
    let cases = read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");
    let case = cases
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == "1x2-w32")
        .unwrap();
    let image = case["image"].as_str().unwrap();
    assert_eq!(hash(image), case["image_sha256"]);
    let request = case["request"].as_str().unwrap();
    pin(&mut pins, image);
    pin(&mut pins, request);
    save(r("case.json"), case);
    pin(&mut pins, r("case.json"));
    for p in [
        XTASK,
        VERIFY,
        "/usr/bin/perf",
        "/usr/bin/objdump",
        "/lib/x86_64-linux-gnu/libgcc_s.so.1",
        "/lib/x86_64-linux-gnu/libc.so.6",
        "/lib64/ld-linux-x86-64.so.2",
    ] {
        pin(&mut pins, p);
    }
    for p in [
        "decode.rs",
        "legacy-decoder.rs",
        "decode",
        "decoder-build.log",
        "decoder-build-v2.log",
        "decoder-tests.log",
        "decoder-incomplete-test.log",
        "flow.rs",
        "flow",
        "flow-build.log",
        "design.md",
        "symbols.txt",
        "disassembly.txt",
        "hardware.json",
        "kernel.txt",
        "perf-version.txt",
        "references/installed-amd-ibs.h",
        "capability/analysis.json",
        "capability/inherited-op-analysis.json",
        "capability/raw.txt",
        "capability/samples.txt",
        "capability/capture.stdout.log",
        "capability/capture.stderr.log",
    ] {
        pin(&mut pins, r(p));
    }
    for p in ["raw.txt", "samples.txt"] {
        pin(
            &mut pins,
            format!("/home/ubuntu/src/zopro-perf-work/off-ibs/capability/{p}"),
        );
    }
    assert_eq!(read(r("capability/analysis.json"))["samples"], 31);
    assert_eq!(
        read(r("capability/inherited-op-analysis.json"))["samples"],
        239
    );
    let mut caps = BTreeMap::new();
    for pmu in ["ibs_op", "ibs_fetch"] {
        for sub in ["caps", "format"] {
            for e in fs::read_dir(format!("/sys/bus/event_source/devices/{pmu}/{sub}")).unwrap() {
                let p = e.unwrap().path();
                caps.insert(p.display().to_string(), fs::read_to_string(p).unwrap());
            }
        }
    }
    save(r("capabilities.json"), &json!(caps));
    pin(&mut pins, r("capabilities.json"));
    let mut prior: BTreeSet<String> = serde_json::from_value(read(
        "/home/ubuntu/src/zopro-perf-work/service-trace175/prior-proof-sha256.json",
    ))
    .unwrap();
    assert_eq!(prior.len(), 17786);
    let inventory = read("/home/ubuntu/src/zopro-perf-work/service-trace175/proof-inventory.json");
    check(&inventory);
    for (f, h) in inventory.as_object().unwrap() {
        if f.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().to_owned()));
        }
    }
    assert_eq!(prior.len(), 17805);
    save(r("prior-proof-sha256.json"), &json!(prior));
    pin(&mut pins, r("prior-proof-sha256.json"));
    let mut jobs = Vec::new();
    let mut steps = Vec::new();
    for block in 0..3 {
        for mode in if block % 2 == 0 {
            ["op", "fetch"]
        } else {
            ["fetch", "op"]
        } {
            let name = format!("b{block}-{mode}");
            let dir = r("captures").join(&name);
            fs::create_dir_all(dir.join("proofs")).unwrap();
            let ctl = dir.join("ctl");
            let ack = dir.join("ack");
            assert!(Command::new("mkfifo")
                .arg(&ctl)
                .arg(&ack)
                .status()
                .unwrap()
                .success());
            let event = if mode == "op" {
                "ibs_op/cnt_ctl=1/"
            } else {
                "ibs_fetch/rand_en=1/"
            };
            let mut argv = vec![
                json!("sudo"),
                json!("-n"),
                json!("perf"),
                json!("record"),
                json!("--no-buildid-cache"),
                json!("-e"),
                json!(event),
                json!("-c"),
                json!("524288"),
                json!("-m"),
                json!("1024"),
                json!("--raw-samples"),
            ];
            if mode == "op" {
                argv.extend([json!("-d"), json!("-W")]);
            }
            argv.extend([
                json!("--delay=-1"),
                json!(format!(
                    "--control=fifo:{},{}",
                    ctl.display(),
                    ack.display()
                )),
                json!("-o"),
                json!(dir.join("perf.data")),
                json!("--"),
                json!("env"),
            ]);
            for (k, v) in manifest["environment"].as_object().unwrap() {
                argv.push(json!(format!("{k}={}", v.as_str().unwrap())));
            }
            argv.extend([
                json!(format!("ZOPRO_BENCH_PERF_CTL={}", ctl.display())),
                json!(format!("ZOPRO_BENCH_PERF_ACK={}", ack.display())),
            ]);
            argv.extend(
                json!([
                    "taskset",
                    "-c",
                    "0-31",
                    sdk,
                    image,
                    request,
                    "61",
                    dir.join("proofs")
                ])
                .as_array()
                .unwrap()
                .iter()
                .cloned(),
            );
            steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":180}));
            jobs.push(json!({"name":name,"mode":mode,"block":block,"directory":dir}));
        }
    }
    for j in &jobs {
        let name = j["name"].as_str().unwrap();
        let dir = Path::new(j["directory"].as_str().unwrap());
        steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":[VERIFY,image,case["image_sha256"],dir.join("proofs/0.proof"),dir.join("proofs/0.public"),case["public_sha256"]],"timeout_seconds":120}));
    }
    for j in &jobs {
        let name = j["name"].as_str().unwrap();
        let dir = Path::new(j["directory"].as_str().unwrap());
        let mode = j["mode"].as_str().unwrap();
        let fields = if mode == "op" {
            "pid,tid,time,ip,weight,data_src,sym,symoff,dso"
        } else {
            "pid,tid,time,ip,sym,symoff,dso"
        };
        steps.push(json!({"kind":"command","name":format!("raw-{name}"),"argv":["sudo","-n","perf","report","-D","-i",dir.join("perf.data")],"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("script-{name}"),"argv":["sudo","-n","perf","script","--ns","--header","--show-lost-events","--no-demangle","-i",dir.join("perf.data"),"-F",fields],"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":[r("decode"),json!(mode),r("runs").join(format!("raw-{name}/stdout.log")),r("runs").join(format!("script-{name}/stdout.log")),r("disassembly.txt"),dir.join("analysis.json")],"timeout_seconds":180}));
    }
    save(r("jobs.json"), &json!(jobs));
    pin(&mut pins, r("jobs.json"));
    save(
        r("declaration.json"),
        &json!({"blocks":3,"rounds":61,"primes":3,"SDK_requests":384,"Gnark_exports":6,"prior_proofs":17805,"expected_next_ledger":17811,"accepted_source_files":175,"source_changed":false,"adopted":false,"goal_achieved":false,"all8_CPU10x_rows":0,"events":["ibs_op/cnt_ctl=1/","ibs_fetch/rand_en=1/"],"period":524288,"workers":32,"affinity":"0-31","no_capture_overlap":true,"no_extensions_or_reruns":true,"order":"six captures, six Gnark, then raw/script/decode","limits":"three descriptive process blocks; instrumented latency; no new performance/adoption/RAM qualification"}),
    );
    pin(&mut pins, r("declaration.json"));
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Prepared30steps:6captures,384SDK,6Gnark; unchangedaccepted175");
}
fn finish() {
    assert_eq!(read(r("runs/result.json"))["status"], "complete");
    check(&read(r("pins.json")));
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    let mut inventory = BTreeMap::new();
    let mut raw_hashes = BTreeMap::new();
    let mut rows = Vec::new();
    let mut sample_counts = BTreeMap::<String, u64>::new();
    let case = read(r("case.json"));
    for j in read(r("jobs.json")).as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let dir = Path::new(j["directory"].as_str().unwrap());
        let text = fs::read_to_string(r("runs").join(name).join("stdout.log")).unwrap();
        let samples: Vec<Value> = text
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(samples.len(), 61);
        let summary: Value = serde_json::from_str(
            text.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["rounds"], 61);
        assert_eq!(summary["workers"], 32);
        let proof = dir.join("proofs/0.proof");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        let digest = hash(&proof);
        assert!(prior.insert(digest.clone()));
        let public = proof.with_extension("public");
        assert_eq!(hash(&public), case["public_sha256"]);
        let verified =
            fs::read_to_string(r("runs").join(format!("verify-{name}/stdout.log"))).unwrap();
        assert!(verified.contains(&format!("VERIFIED proof_sha256={digest} ")));
        pin(&mut inventory, proof);
        pin(&mut inventory, public);
        let a = read(dir.join("analysis.json"));
        assert_eq!(a["lost_records"], 0);
        assert_eq!(a["user_pids"].as_array().unwrap().len(), 1);
        let mode = j["mode"].as_str().unwrap();
        *sample_counts.entry(mode.into()).or_default() += a["samples"].as_u64().unwrap();
        for cat in ["G1feed", "G2feed"] {
            assert!(
                a["groups"][format!("user/{cat}")]["samples"]
                    .as_u64()
                    .unwrap()
                    > 100
            );
        }
        let mut row = j.clone();
        row["instrumented_median_ms"] = json!(median(
            samples.iter().map(|v| v["ms"].as_f64().unwrap()).collect()
        ));
        row["summary"] = summary;
        row["IBS"] = a;
        rows.push(row);
        for p in [
            dir.join("perf.data"),
            r("runs").join(format!("raw-{name}/stdout.log")),
            r("runs").join(format!("script-{name}/stdout.log")),
        ] {
            pin(&mut raw_hashes, p);
        }
    }
    assert_eq!(prior.len(), 17811);
    assert_eq!(inventory.len(), 12);
    save(r("proof-inventory.json"), &json!(inventory));
    save(r("raw-local-only-sha256.json"), &json!(raw_hashes));
    save(r("rows.json"), &json!(rows));
    save(
        r("analysis.json"),
        &json!({"status":"complete current175 streaming op/fetch diagnostic","SDK_requests":384,"new_independent_exports":6,"independent_proof_count":17811,"sample_counts":sample_counts,"production_changed":false,"adopted":false,"goal_achieved":false,"goal_status":"active/unmet0of8","inherited_M5":read(MANIFEST)["m5"],"rows":rows}),
    );
    println!("Closed384SDK/6Gnark; ledger17811; samples{sample_counts:?}");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare|finish"),
    }
}
