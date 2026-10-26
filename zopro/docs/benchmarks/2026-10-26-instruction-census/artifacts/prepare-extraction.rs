use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
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
fn source(p: &Path, pins: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            source(&p, pins)
        } else {
            add(&p, pins);
        }
    }
}
fn main() {
    let r = Path::new("/tmp/zopro-instruction-census");
    let parent = Path::new("/tmp/zopro-fused-product-overlap");
    let archive=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-fused-product-overlap/artifacts");
    let index = read(archive.join("artifact-index.json"));
    let cfg = read(parent.join("perf-plan-v2.json"));
    let rows = read(archive.join("perf/rows.json"));
    assert_eq!(
        read(archive.join("closeout.json"))["profile_samples"],
        297412
    );
    let mut pins = BTreeMap::new();
    let mut binaries = BTreeMap::new();
    let mut runs = Vec::new();
    let mut steps = Vec::new();
    let mut sample_total = 0;
    for row in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["phase"] == "profile")
    {
        let case = row["case"].as_str().unwrap();
        let variant = row["variant"].as_str().unwrap();
        let block = row["block"].as_u64().unwrap();
        assert_eq!(row["profile"]["lost_records"], 0);
        assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
        let path = cfg["variants"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == variant)
            .unwrap()["binary"]
            .as_str()
            .unwrap();
        assert_eq!(hash(path), cfg["pins"][path].as_str().unwrap());
        let binary = if variant == "accepted" {
            "accepted"
        } else {
            "candidate"
        };
        if let Some(old) = binaries.insert(binary, path.to_owned()) {
            assert_eq!(old, path);
        }
        let name = format!("{case}-{variant}");
        let base = format!("perf/profile/b{block}/{case}/{variant}");
        let raw = archive.join(format!("{base}/perf.data"));
        let samples = archive.join(format!("{base}/symbols/stdout.log"));
        for p in [&raw, &samples] {
            let rel = p.strip_prefix(archive).unwrap().to_str().unwrap();
            assert_eq!(hash(p), index["sha256"][rel].as_str().unwrap());
            add(p, &mut pins);
        }
        steps.push(json!({"kind":"command","name":format!("metadata-{name}"),"argv":["perf","script","--show-mmap-events","--show-task-events","-i",raw,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));
        runs.push(json!({"name":name,"binary":binary,"metadata":r.join("extraction").join(format!("metadata-{name}/stdout.log")),"samples":samples,"expected_pid":row["profile"]["sampled_pids"][0],"expected_samples":row["profile"]["samples"],"expected_period":row["profile"]["period"],"expected_tids":row["profile"]["sampled_tids"]}));
        sample_total += row["profile"]["samples"].as_u64().unwrap();
    }
    assert_eq!(runs.len(), 9);
    assert_eq!(binaries.len(), 2);
    assert_eq!(sample_total, 297412);
    let mut definitions = Vec::new();
    for (name, path) in &binaries {
        add(path, &mut pins);
        for (label, args) in [
            ("objdump", vec!["objdump", "-d", "-w", "--insn-width=16"]),
            ("nm", vec!["nm", "-S", "-C", "--defined-only"]),
            ("elf", vec!["readelf", "-lW"]),
        ] {
            let mut argv = args.iter().map(|s| json!(s)).collect::<Vec<_>>();
            argv.push(json!(path));
            steps.push(json!({"kind":"command","name":format!("{label}-{name}"),"argv":argv,"timeout_seconds":120}));
        }
        definitions.push(json!({"name":name,"path":path,"disassembly":r.join(format!("extraction/objdump-{name}/stdout.log")),"symbols":r.join(format!("extraction/nm-{name}/stdout.log"))}));
    }
    for (tool, label) in [
        ("perf", "perf-version"),
        ("objdump", "objdump-version"),
        ("rustc", "compiler-version"),
    ] {
        steps.push(json!({"kind":"command","name":label,"argv":[tool,if tool=="rustc"{"-Vv"}else{"--version"}]}));
    }
    source(
        Path::new("/home/ubuntu/src/zk-tools/zopro/xtask/src"),
        &mut pins,
    );
    for p in [
        "/home/ubuntu/src/zk-tools/zopro/xtask/Cargo.toml",
        "/home/ubuntu/src/zk-tools/zopro/xtask/Cargo.lock",
        "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask",
        "/tmp/zopro-instruction-census/target/release/xtask",
    ] {
        add(p, &mut pins);
    }
    assert_eq!(
        hash("/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask"),
        "deaba4ab1d782d9173859f3a8a542a3f5d2409af3fb67a80eedb32836530d616"
    );
    for name in [
        "experiment.json",
        "prepare-extraction.rs",
        "tool-tests.log",
        "tool-clippy.log",
        "tool-build.log",
    ] {
        add(r.join(name), &mut pins);
    }
    for p in [
        parent.join("perf-plan-v2.json"),
        archive.join("artifact-index.json"),
        archive.join("perf/rows.json"),
        archive.join("closeout.json"),
    ] {
        add(p, &mut pins);
    }
    fs::write(
        r.join("analysis-template.json"),
        serde_json::to_vec_pretty(
            &json!({"version":1,"pins":{},"binaries":definitions,"runs":runs}),
        )
        .unwrap(),
    )
    .unwrap();
    add(r.join("analysis-template.json"), &mut pins);
    fs::write(
        r.join("extraction-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(r.join("extraction-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("extraction-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    let provenance = json!({"source_commit":"25cef6ed118ca891f52dadf29d0bfc49f16766dc","parent_root":parent,"archive":archive,"samples":sample_total,"captures":9,"new_captures":0,"new_sdk_calls":0,"new_gnark_exports":0,"old_qualified_xtask_unchanged":true,"reader_binary_sha256":hash(r.join("target/release/xtask")),"reader_unit_tests":30,"formatting_clippy_build":"passed; isolatedtarget","all_raw_and_compact_input_hashes_match_parent_index":true});
    fs::write(
        r.join("source-provenance.json"),
        serde_json::to_vec_pretty(&provenance).unwrap(),
    )
    .unwrap();
    println!("Frozen9existingcaptures{sample_total}samples,2exactbinaries,{}pins,{}metadata/disassemblycommands",pins.len(),steps.len());
}
