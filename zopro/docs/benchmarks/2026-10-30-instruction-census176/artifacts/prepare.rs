use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/instruction-census176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const X: &str = "/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn command(name: impl Into<String>, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name.into(),"argv":argv,"timeout_seconds":180})
}
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    let mode = std::env::args().nth(1).expect("extract/analyze");
    if mode == "analyze" {
        assert!(!r.join("analysis-config.json").exists());
        assert_eq!(read(r.join("extraction/result.json"))["status"], "complete");
        let mut config = read(r.join("analysis-template.json"));
        let mut pins: BTreeMap<String, String> =
            serde_json::from_value(read(r.join("extraction-pins.json"))).unwrap();
        for (f, h) in &pins {
            assert_eq!(hash(f), *h);
        }
        for b in config["binaries"].as_array().unwrap() {
            for key in ["path", "disassembly", "symbols"] {
                pin(&mut pins, b[key].as_str().unwrap());
            }
        }
        for run in config["runs"].as_array().unwrap() {
            for key in ["metadata", "samples"] {
                pin(&mut pins, run[key].as_str().unwrap());
            }
        }
        pin(&mut pins, r.join("extraction/result.json"));
        config["pins"] = json!(pins);
        save(r.join("analysis-config.json"), &config);
        pin(&mut pins, r.join("analysis-config.json"));
        save(r.join("analysis-pins.json"), &json!(pins));
        save(
            r.join("analysis-plan.json"),
            &json!({"version":1,"source_pins":r.join("analysis-pins.json"),"steps":[command("instruction-attribution",vec![X.into(),"instructions".into(),r.join("analysis-config.json").display().to_string(),r.join("analysis").display().to_string()])]}),
        );
        println!("Prepared offline instruction attribution for four qualified captures");
        return;
    }
    assert_eq!(mode, "extract");
    assert!(!r.join("extraction-plan.json").exists());
    let manifest_file=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let m = read(&manifest_file);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(), 176);
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let file = Path::new(Z).join(f);
        assert_eq!(hash(&file), h.as_str().unwrap());
        pin(&mut pins, file);
    }
    for file in[manifest_file,p.join("analysis.json"),p.join("identities.json"),Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-default175/artifacts/post-publication-review.json"),r.join("prepare.rs"),r.join("prepare"),r.join("design.md"),PathBuf::from(X),PathBuf::from("/usr/bin/perf"),PathBuf::from("/usr/bin/objdump"),PathBuf::from("/usr/bin/nm")]{pin(&mut pins,file);}
    let a = read(p.join("analysis.json"));
    let ids = read(p.join("identities.json"));
    assert_eq!(a["final_artifact_gate_passed"], true);
    let mut steps = vec![];
    let mut binaries = vec![];
    let mut runs = vec![];
    for (label, variant) in [("prior", "accepted"), ("current", "ON")] {
        let binary = ids[variant]["binary"].as_str().unwrap();
        assert_eq!(hash(binary), ids[variant]["sha256"]);
        pin(&mut pins, binary);
        steps.push(command(
            format!("objdump-{label}"),
            vec![
                "objdump".into(),
                "-d".into(),
                "-w".into(),
                "--insn-width=16".into(),
                binary.into(),
            ],
        ));
        steps.push(command(
            format!("nm-{label}"),
            vec![
                "nm".into(),
                "-S".into(),
                "-C".into(),
                "--defined-only".into(),
                binary.into(),
            ],
        ));
        binaries.push(json!({"name":label,"path":binary,"disassembly":r.join(format!("extraction/objdump-{label}/stdout.log")),"symbols":r.join(format!("extraction/nm-{label}/stdout.log"))}));
        for workers in [16, 32] {
            let capture = format!("cycles-1x2-w{workers}-{variant}");
            let name = format!("1x2-w{workers}-{label}");
            let profile = &a["profiles"][&capture];
            assert_eq!(profile["attribution_qualified"], true);
            assert_eq!(profile["zero_lost"], true);
            let data = p.join(format!("perf/{capture}.data"));
            pin(&mut pins, &data);
            for kind in ["samples", "metadata"] {
                let mut argv = vec!["perf".into(), "script".into()];
                if kind == "metadata" {
                    argv.extend(["--show-mmap-events".into(), "--show-task-events".into()]);
                }
                argv.extend([
                    "-i".into(),
                    data.display().to_string(),
                    "-F".into(),
                    "pid,tid,period,ip,sym".into(),
                ]);
                steps.push(command(format!("{kind}-{name}"), argv));
            }
            runs.push(json!({"name":name,"binary":label,"metadata":r.join(format!("extraction/metadata-{name}/stdout.log")),"samples":r.join(format!("extraction/samples-{name}/stdout.log")),"expected_pid":profile["PIDs"][0],"expected_samples":profile["samples"],"expected_period":profile["period"],"expected_tids":profile["TIDs"].as_array().unwrap().len()}));
        }
    }
    for tool in ["perf", "objdump", "nm"] {
        steps.push(command(
            format!("{tool}-version"),
            vec![tool.into(), "--version".into()],
        ));
    }
    assert_eq!(steps.len(), 15);
    assert_eq!(runs.len(), 4);
    save(
        r.join("analysis-template.json"),
        &json!({"version":1,"binaries":binaries,"runs":runs,"pins":{}}),
    );
    pin(&mut pins, r.join("analysis-template.json"));
    save(r.join("extraction-pins.json"), &json!(pins));
    save(
        r.join("extraction-plan.json"),
        &json!({"version":1,"source_pins":r.join("extraction-pins.json"),"steps":steps}),
    );
    println!("Prepared 15 offline extraction steps, four qualified captures, exact current176 source and binary/perf inputs");
}
