use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
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
fn main() {
    let r = Path::new(R);
    let mut pins: BTreeMap<String, String> = serde_json::from_value(
        read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json")["pins"].clone(),
    )
    .unwrap();
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in base["source_files_sha256"].as_object().unwrap() {
        let p = Path::new("/home/ubuntu/src/zk-tools/zopro").join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), h.as_str().unwrap().into());
    }
    let mut steps = Vec::new();
    for (name, art) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
        ),
        (
            "candidate",
            "/tmp/zopro-pairing-x-chain-sdk/native/on/artifacts.json",
        ),
    ] {
        let a = read(art);
        pins.insert(art.into(), hash(art));
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins.insert(p.clone(), h.as_str().unwrap().into());
        }
        let mut cargo = String::new();
        for (n, p) in a["libraries"].as_object().unwrap() {
            cargo.push_str(
                &serde_json::to_string(
                    &json!({"reason":"compiler-artifact","target":{"name":n},"filenames":[p]}),
                )
                .unwrap(),
            );
            cargo.push('\n');
        }
        let file = r.join(format!("{name}-libraries.jsonl"));
        fs::write(&file, cargo).unwrap();
        pins.insert(file.display().to_string(), hash(&file));
        steps.push(json!({"kind":"link-sdk","name":name,"artifacts":file,"driver":r.join("driver.rs"),"additional_sources":[r.join("support/perf.rs")]}));
    }
    for rel in [
        "design.md",
        "driver.rs",
        "support/perf.rs",
        "runner.rs",
        "analyze.rs",
        "setup.rs",
        "sched-switch-format.txt",
        "yield-format.txt",
    ] {
        let p = r.join(rel);
        pins.insert(p.display().to_string(), hash(p));
    }
    for p in [
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
        "/usr/bin/perf",
    ] {
        pins.insert(p.into(), hash(p));
    }
    let deps = "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/deps";
    let sj = format!("{deps}/libserde_json-cdebb9338c2784ad.rlib");
    let sha = format!("{deps}/libsha2-16b8f2c03bdf5faa.rlib");
    for p in [&sj, &sha] {
        pins.insert(p.clone(), hash(p));
    }
    for (name, file, test) in [
        ("analyzer-tests", "analyze.rs", true),
        ("analyzer-build", "analyze.rs", false),
        ("runner-build", "runner.rs", false),
    ] {
        let output = if name == "analyzer-tests" {
            "analyzer-tests"
        } else if file == "analyze.rs" {
            "analyze"
        } else {
            "runner"
        };
        let mut argv = vec![
            "rustc".to_string(),
            "--edition=2021".into(),
            "-O".into(),
            "-L".into(),
            format!("dependency={deps}"),
            "--extern".into(),
            format!("serde_json={sj}"),
            "--extern".into(),
            format!("sha2={sha}"),
        ];
        if test {
            argv.push("--test".into());
        }
        argv.extend([
            r.join(file).display().to_string(),
            "-o".into(),
            r.join(output).display().to_string(),
        ]);
        steps.push(json!({"kind":"command","name":name,"argv":argv}));
        if test {
            steps.push(json!({"kind":"command","name":"analyzer-test-run","argv":[r.join(output),"--test-threads=1"],"unfiltered":true,"required_tests":["tests::nanoseconds_and_fields","tests::disjoint_clipping","tests::yield_off_cpu_and_capacity","tests::missing_cpu_and_right_censor","tests::continuity_is_detected"]}));
        }
    }
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "pin {p}");
    }
    save(r.join("build-pins.json"), &json!(pins));
    save(
        r.join("workflow-build.json"),
        &json!({"version":1,"source_pins":r.join("build-pins.json"),"steps":steps}),
    );
    println!(
        "Pinned {} inputs; link exact native artifacts + observer tests before capture",
        pins.len()
    );
}
