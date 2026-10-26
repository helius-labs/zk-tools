use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
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
fn run(argv: Vec<String>, d: &Path) {
    assert!(!d.exists());
    fs::create_dir_all(d).unwrap();
    save(d.join("command.json"), &json!(argv));
    let status = Command::new(&argv[0])
        .args(&argv[1..])
        .stdout(Stdio::from(fs::File::create(d.join("stdout.log")).unwrap()))
        .stderr(Stdio::from(fs::File::create(d.join("stderr.log")).unwrap()))
        .status()
        .unwrap();
    save(
        d.join("receipt.json"),
        &json!({"success":status.success(),"code":status.code()}),
    );
    assert!(status.success(), "{}", d.display());
}
fn main() {
    let r = Path::new(R);
    let mode = std::env::args().nth(1).unwrap();
    if mode == "run" {
        let plan = read(r.join("capture-plan.json"));
        let old = read(r.join("observations.json"));
        let mut rows = Vec::new();
        let mut pins = json!({});
        for (s, pretty) in plan["traces"]
            .as_array()
            .unwrap()
            .iter()
            .zip(old.as_array().unwrap())
        {
            let d = r.join(s["name"].as_str().unwrap());
            let raw = d.join("raw-events.log");
            let meta = d.join("raw-metadata.json");
            run(
                vec![
                    "sudo".into(),
                    "-n".into(),
                    format!("{R}/raw-reader"),
                    d.join("perf.data").display().to_string(),
                    raw.display().to_string(),
                    meta.display().to_string(),
                ],
                &d.join("raw-reader"),
            );
            let out = d.join("analysis-raw.json");
            run(
                vec![
                    format!("{R}/analyze-raw"),
                    raw.display().to_string(),
                    d.join("stdout.log").display().to_string(),
                    s["workers"].to_string(),
                    out.display().to_string(),
                ],
                &d.join("analyze-raw"),
            );
            let mut row = read(&out);
            row["case"] = s.clone();
            row["raw_metadata"] = read(meta);
            row["pretty_event_count"] = pretty["event_count"].clone();
            row["raw_matches_pretty_count"] = json!(
                row["event_count"].as_u64().unwrap()
                    == pretty["event_count"].as_u64().unwrap()
                        + pretty["unparsed_noncomment_lines"].as_u64().unwrap()
            );
            row["trace_usable"] = json!(
                row["trace_usable"] == true
                    && row["raw_metadata"]["lost_samples"] == 0
                    && row["raw_matches_pretty_count"] == true
            );
            rows.push(row);
            pins[raw.display().to_string()] = json!(hash(&raw));
        }
        save(r.join("observations-raw.json"), &json!(rows));
        save(r.join("numeric-events-local-only-sha256.json"), &pins);
        println!("Analyzed all18 existing captures; no SDK calls");
        return;
    }
    let mut pins = read(r.join("reader-run-pins.json"));
    for f in [
        "raw.rs",
        "analyze-raw.rs",
        "raw-post.rs",
        "raw-reader-amendment.md",
    ] {
        let p = r.join(f);
        pins[p.display().to_string()] = json!(hash(&p));
    }
    let mut steps = Vec::new();
    if mode == "build" {
        let old = read(r.join("workflow-build.json"));
        let template = old["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == "runner-build")
            .unwrap();
        for (name, src, out, test) in [
            ("raw-tests-build", "raw.rs", "raw-tests", true),
            ("raw-tests-run", "", "", false),
            ("raw-reader-build", "raw.rs", "raw-reader", false),
            (
                "raw-analyzer-tests-build",
                "analyze-raw.rs",
                "raw-analyzer-tests",
                true,
            ),
            ("raw-analyzer-tests-run", "", "", false),
            ("raw-analyzer-build", "analyze-raw.rs", "analyze-raw", false),
            ("raw-post-build", "raw-post.rs", "raw-post", false),
        ] {
            if name.ends_with("tests-run") {
                let binary = if name == "raw-tests-run" {
                    "raw-tests"
                } else {
                    "raw-analyzer-tests"
                };
                steps.push(json!({"kind":"command","name":name,"argv":[r.join(binary),"--test-threads=1"],"unfiltered":true}));
                continue;
            }
            let mut s = template.clone();
            s["name"] = json!(name);
            let a = s["argv"].as_array_mut().unwrap();
            for v in a.iter_mut() {
                if *v == json!(r.join("runner.rs")) {
                    *v = json!(r.join(src));
                } else if *v == json!(r.join("runner")) {
                    *v = json!(r.join(out));
                }
            }
            if test {
                a.insert(1, json!("--test"));
            }
            steps.push(s);
        }
        save(r.join("raw-build-pins.json"), &pins);
        save(
            r.join("workflow-raw-build.json"),
            &json!({"version":1,"source_pins":r.join("raw-build-pins.json"),"steps":steps}),
        );
    } else {
        assert_eq!(mode, "freeze");
        for f in ["raw-reader", "analyze-raw", "raw-post"] {
            let p = r.join(f);
            pins[p.display().to_string()] = json!(hash(&p));
        }
        save(r.join("raw-run-pins.json"), &pins);
        save(
            r.join("workflow-raw-run.json"),
            &json!({"version":1,"source_pins":r.join("raw-run-pins.json"),"steps":[{"kind":"command","name":"retained-raw-analysis","argv":[r.join("raw-post"),"run"],"timeout_seconds":1800}]}),
        );
    }
}
