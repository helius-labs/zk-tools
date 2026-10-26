use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    pins.insert(p.to_str().unwrap().into(), hash(p));
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    let mode = std::env::args().nth(1).unwrap();
    assert_eq!(read(root.join("perf/result.json"))["status"], "complete");
    if mode == "extract" {
        let config = read(root.join("perf-plan.json"));
        let rows = read(root.join("perf/rows.json"));
        let mut pins = BTreeMap::new();
        let mut runs = Vec::new();
        let mut binaries = Vec::new();
        let mut copies = Vec::new();
        let mut steps = Vec::new();
        let mut samples = 0u64;
        for row in rows
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["phase"] == "profile" && r["case"] == "1x2-w32")
        {
            let label = row["variant"].as_str().unwrap();
            let binary = config["variants"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["name"] == label)
                .unwrap()["binary"]
                .as_str()
                .unwrap();
            assert_eq!(hash(binary), config["pins"][binary].as_str().unwrap());
            assert_eq!(row["profile"]["lost_records"], 0);
            assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
            let base = root.join(format!("perf/profile/b0/1x2-w32/{label}"));
            let src = base.join("perf.data");
            let dst = root.join(format!("instruction-inputs/{label}/perf.data"));
            fs::create_dir_all(dst.parent().unwrap()).unwrap();
            assert!(!dst.exists());
            let before = fs::metadata(&src).unwrap();
            let out = Command::new("sudo")
                .args(["-n", "sha256sum"])
                .arg(&src)
                .output()
                .unwrap();
            assert!(out.status.success());
            let digest = String::from_utf8(out.stdout)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .to_owned();
            assert!(Command::new("sudo")
                .args(["-n", "cat"])
                .arg(&src)
                .stdout(Stdio::from(fs::File::create(&dst).unwrap()))
                .status()
                .unwrap()
                .success());
            assert_eq!(hash(&dst), digest);
            let after = fs::metadata(&src).unwrap();
            assert_eq!(before.len(), after.len());
            assert_eq!(before.modified().unwrap(), after.modified().unwrap());
            copies.push(
                json!({"original":src,"user_owned_copy":dst,"sha256":digest,"same_bytes":true}),
            );
            let compact = base.join("symbols/stdout.log");
            for p in [
                &dst,
                &compact,
                Path::new(binary),
                &root.join(format!("{label}-symbols.txt")),
            ] {
                add(p, &mut pins);
            }
            steps.push(json!({"kind":"command","name":format!("metadata-{label}"),"argv":["perf","script","--show-mmap-events","--show-task-events","-i",dst,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
            steps.push(json!({"kind":"command","name":format!("objdump-{label}"),"argv":["objdump","-d","-w","--insn-width=16",binary],"timeout_seconds":120}));
            binaries.push(json!({"name":label,"path":binary,"disassembly":root.join(format!("instruction-extraction/objdump-{label}/stdout.log")),"symbols":root.join(format!("{label}-symbols.txt"))}));
            runs.push(json!({"name":format!("primary-{label}"),"binary":label,"metadata":root.join(format!("instruction-extraction/metadata-{label}/stdout.log")),"samples":compact,"expected_pid":row["profile"]["sampled_pids"][0],"expected_samples":row["profile"]["samples"],"expected_period":row["profile"]["period"],"expected_tids":row["profile"]["sampled_tids"]}));
            samples += row["profile"]["samples"].as_u64().unwrap();
        }
        assert_eq!(runs.len(), 3);
        assert_eq!(samples, 49666);
        for p in [
            root.join("instruction-audit.rs"),
            root.join("perf-plan.json"),
            root.join("perf/rows.json"),
            Path::new("/tmp/zopro-instruction-census/target/release/xtask").into(),
        ] {
            add(p, &mut pins);
        }
        save(
            root.join("instruction-raw-copy-audit.json"),
            &json!({"copies":copies,"raw_copies_local_only":"Original nine raw captures will be archived with identical hashes; these three user-owned read copies need no duplicate archive bytes.","selection":"All three primary captures, selected after full SDK rejection to inspect actual G2 overhead; no new captures, timings, source, gates or observation exclusions.","samples":samples}),
        );
        add(root.join("instruction-raw-copy-audit.json"), &mut pins);
        save(
            root.join("instruction-template.json"),
            &json!({"version":1,"pins":{},"binaries":binaries,"runs":runs}),
        );
        add(root.join("instruction-template.json"), &mut pins);
        save(root.join("instruction-extraction-pins.json"), &json!(pins));
        save(
            root.join("instruction-extraction-plan.json"),
            &json!({"version":1,"source_pins":root.join("instruction-extraction-pins.json"),"environment":{},"steps":steps}),
        );
        println!("Froze six offline commands over three retained primarycaptures{samples}samples; no newSDK");
    } else if mode == "analyze" {
        let result = read(root.join("instruction-extraction/result.json"));
        assert_eq!(result["status"], "complete");
        let mut pins: BTreeMap<String, String> =
            serde_json::from_value(read(root.join("instruction-extraction-pins.json"))).unwrap();
        for (p, h) in &pins {
            assert_eq!(&hash(p), h);
        }
        let plan = read(root.join("instruction-extraction-plan.json"));
        for step in plan["steps"].as_array().unwrap() {
            let base = root
                .join("instruction-extraction")
                .join(step["name"].as_str().unwrap());
            let exit = read(base.join("exit.json"));
            assert_eq!(exit["success"], true);
            assert_eq!(exit["returncode"], 0);
            for stream in ["stdout", "stderr"] {
                let p = base.join(format!("{stream}.log"));
                assert_eq!(hash(&p), exit[format!("{stream}_sha256")].as_str().unwrap());
                add(p, &mut pins);
            }
            for name in ["command.json", "exit.json"] {
                add(base.join(name), &mut pins);
            }
        }
        add(root.join("instruction-extraction-plan.json"), &mut pins);
        let mut config = read(root.join("instruction-template.json"));
        config["pins"] = json!(pins);
        save(root.join("instruction-config.json"), &config);
        add(root.join("instruction-config.json"), &mut pins);
        save(root.join("instruction-analysis-pins.json"), &json!(pins));
        save(
            root.join("instruction-analysis-plan.json"),
            &json!({"version":1,"source_pins":root.join("instruction-analysis-pins.json"),"environment":{},"steps":[{"kind":"command","name":"primary-instructions","argv":["/tmp/zopro-instruction-census/target/release/xtask","instructions",root.join("instruction-config.json"),root.join("instructions")],"timeout_seconds":120}]}),
        );
        println!("Froze qualified reader for allthree primarycaptures");
    } else {
        panic!("extract|analyze")
    }
}
