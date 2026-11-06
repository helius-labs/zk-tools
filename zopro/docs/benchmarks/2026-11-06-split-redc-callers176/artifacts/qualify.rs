use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-callers176";
const N: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-native176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(f: &str, v: &Value) {
    fs::write(format!("{R}/{f}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("qualification.json").exists());
    assert!(!Path::new(R).join("disassembly").exists());
    let first = read(format!("{R}/build/completed.json"));
    assert_eq!(first.as_array().unwrap().len(), 4);
    assert_eq!(
        read(format!("{R}/build/driver-OFF/exit.json"))["success"],
        false
    );
    let repair = read(format!("{R}/build-repair-field-search/result.json"));
    assert_eq!(repair["status"], "complete");
    assert_eq!(repair["steps"].as_array().unwrap().len(), 8);
    let mut pins = BTreeMap::new();
    for f in ["build-pins.json", "repair-pins.json"] {
        for (p, h) in read(format!("{R}/{f}")).as_object().unwrap() {
            assert_eq!(h, &json!(hash(p)));
            pins.insert(p.clone(), h.as_str().unwrap().to_string());
        }
    }
    let original = read(format!("{R}/build-plan.json"));
    let repairplan = read(format!("{R}/repair-plan.json"));
    let mut receipts = vec![];
    let mut names = BTreeSet::new();
    for (plan, dir, count) in [
        (&original, "build", 4),
        (&repairplan, "build-repair-field-search", 8),
    ] {
        for s in plan["steps"].as_array().unwrap().iter().take(count) {
            let name = s["name"].as_str().unwrap();
            assert!(names.insert(name));
            let d = format!("{R}/{dir}/{name}");
            let e = read(format!("{d}/exit.json"));
            assert_eq!(e["success"], true);
            assert_eq!(e["returncode"], 0);
            assert_eq!(e["stdout_sha256"], hash(format!("{d}/stdout.log")));
            assert_eq!(e["stderr_sha256"], hash(format!("{d}/stderr.log")));
            let cmd = read(format!("{d}/command.json"));
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                s["argv"].as_array().unwrap()
            );
            for f in ["exit.json", "command.json", "stdout.log", "stderr.log"] {
                let p = format!("{d}/{f}");
                pins.insert(p.clone(), hash(&p));
            }
            receipts.push(
                json!({"name":name,"directory":d,"exit_sha256":hash(format!("{d}/exit.json"))}),
            );
        }
    }
    assert_eq!(names.len(), 12);
    let off = fs::read_to_string(format!("{R}/build/core-test-OFF/stdout.log")).unwrap();
    assert!(off.contains("1 passed; 0 failed; 0 ignored; 0 measured; 125 filtered out"));
    let on = fs::read_to_string(format!(
        "{R}/build-repair-field-search/core-test-ON/stdout.log"
    ))
    .unwrap();
    assert!(on.contains("126 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    let onstep = repairplan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "core-test-ON")
        .unwrap();
    assert_eq!(onstep["unfiltered"], true);
    let audit = read(format!("{R}/source-audit.json"));
    let native = read(format!("{N}/source-audit.json"));
    assert_eq!(audit["source_files"], 178);
    assert_eq!(audit["native_source_originals_exact"], 175);
    for (name, h) in audit["hashes"].as_object().unwrap() {
        let p = format!("{R}/source/{name}");
        assert_eq!(h, &json!(hash(&p)));
        if let Some(expected) = native["hashes"].get(name) {
            let mut bytes = fs::read(&p).unwrap();
            if let Some(s) = audit["suffixes"].get(name) {
                let s = s.as_str().unwrap();
                assert!(bytes.ends_with(s.as_bytes()));
                bytes.truncate(bytes.len() - s.len());
            }
            assert_eq!(
                format!("{:x}", Sha256::digest(bytes)),
                expected.as_str().unwrap()
            );
        }
    }
    let mut ids = BTreeMap::new();
    fs::create_dir(format!("{R}/disassembly")).unwrap();
    let mut rows = vec![];
    for role in ["OFF", "ON"] {
        let field = read(format!("{N}/field-{role}.json"));
        let core = read(format!("{R}/core-{role}.json"));
        let driver = read(format!("{R}/driver-{role}.json"));
        for v in [&field, &core, &driver] {
            for k in ["source", "frozen"] {
                assert_eq!(v["sha256"], hash(v[k].as_str().unwrap()));
                pins.insert(
                    v[k].as_str().unwrap().to_string(),
                    v["sha256"].as_str().unwrap().to_string(),
                );
            }
        }
        let ds = repairplan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == format!("driver-{role}"))
            .unwrap();
        let a = ds["argv"].as_array().unwrap();
        assert!(a.contains(&json!(format!(
            "zopro_bn254={}",
            field["frozen"].as_str().unwrap()
        ))));
        assert!(a.contains(&json!(format!(
            "zopro_core={}",
            core["source"].as_str().unwrap()
        ))));
        assert_eq!(
            a.iter().filter(|v| *v == "zopro_split_redc_planes").count(),
            if role == "ON" { 1 } else { 0 }
        );
        assert!(!a.iter().any(|v| v == "zopro_validated_g2"));
        ids.insert(role,json!({"binary":driver["source"],"binary_sha256":driver["sha256"],"frozen_binary":driver["frozen"],"field":field,"core":core}));
        let binary = driver["source"].as_str().unwrap();
        let nm = Command::new("/usr/bin/nm")
            .args(["-S", "--defined-only", binary])
            .output()
            .unwrap();
        assert!(nm.status.success());
        assert!(nm.stderr.is_empty());
        let symbols = String::from_utf8(nm.stdout).unwrap();
        fs::write(format!("{R}/disassembly/{role}-symbols.txt"), &symbols).unwrap();
        for (kind, needle) in [
            ("G1", "msm::stream_g1_impl"),
            ("G2", "msm::g2_borrowed::apply"),
            ("inverse", "inverse_nonzero"),
        ] {
            let selected = symbols
                .lines()
                .filter_map(|l| {
                    let f = l.split_whitespace().collect::<Vec<_>>();
                    if f.len() != 4 {
                        return None;
                    }
                    let n = rustc_demangle::try_demangle(f[3]).ok()?.to_string();
                    if !n.contains(needle)
                        || kind == "G1" && (!n.contains("G1Config") || !n.contains('&'))
                        || kind == "inverse" && !n.contains("FqBatch")
                    {
                        return None;
                    }
                    Some((
                        u64::from_str_radix(f[0], 16).unwrap(),
                        u64::from_str_radix(f[1], 16).unwrap(),
                        f[3].to_string(),
                        n,
                    ))
                })
                .collect::<Vec<_>>();
            assert_eq!(selected.len(), 1, "{role}/{kind}");
            let (start, size, raw, name) = &selected[0];
            let args = vec![
                "-d".to_string(),
                "-Mintel".into(),
                "--no-show-raw-insn".into(),
                format!("--disassemble={raw}"),
                binary.into(),
            ];
            let out = Command::new("/usr/bin/objdump")
                .args(&args)
                .output()
                .unwrap();
            assert!(out.status.success() && out.stderr.is_empty());
            let txt = String::from_utf8(out.stdout).unwrap();
            let p = format!("{R}/disassembly/{role}-{kind}.txt");
            fs::write(&p, &txt).unwrap();
            let ins = txt
                .lines()
                .filter_map(|l| {
                    let (a, s) = l.trim().split_once(':')?;
                    let ip = u64::from_str_radix(a, 16).ok()?;
                    if ip < *start || ip >= *start + *size {
                        return None;
                    }
                    Some(s.trim())
                })
                .collect::<Vec<_>>();
            assert!(!ins.is_empty());
            let count = |m: &str| {
                ins.iter()
                    .filter(|s| s.split_whitespace().next() == Some(m))
                    .count()
            };
            let ifmas = ins.iter().filter(|s| s.starts_with("vpmadd52")).count();
            let stats = json!({"role":role,"kind":kind,"raw_symbol":raw,"name":name,"start":start,"size_bytes":size,"instruction_rows":ins.len(),"ifma":ifmas,"expected_nominal_ifma":if kind=="G1"{800}else if kind=="G2"{1450}else{500},"add":count("vpaddq"),"sub":count("vpsubq"),"right_logical":count("vpsrlq"),"right_arithmetic":count("vpsraq"),"vector_rsp_rbp_operand_rows":ins.iter().filter(|s|s.starts_with('v')&&(s.contains("[rsp")||s.contains("[rbp"))).count(),"literal_rsp_subtractions":ins.iter().filter(|s|s.starts_with("sub    rsp,")).collect::<Vec<_>>(),"disassembly_sha256":hash(&p),"decode_argv":args,"new_decode":true});
            rows.push(stats);
            pins.insert(p.clone(), hash(&p));
        }
    }
    let previous =
        read("/home/ubuntu/src/zopro-perf-work/g2-validated-batch176/qualification.json");
    for row in previous["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["role"] == "FROZEN")
    {
        let p = row["inherited_disassembly"].as_str().unwrap();
        assert_eq!(row["disassembly_sha256"], hash(p));
        pins.insert(p.to_string(), hash(p));
        let mut r = row.clone();
        r["new_decode"] = json!(false);
        rows.push(r);
    }
    assert_eq!(rows.len(), 9);
    let result = json!({"status":"pass","source_files":178,"native177_recoverable":true,"runtime_core_original":true,"coherent_pairs_verified":true,"core_OFF_new_tests":1,"core_OFF_inherited_filtered":125,"core_ON_unfiltered_tests":126,"successful_unique_workflow_commands":12,"failed_driver_link_stage_preserved":true,"receipts":receipts,"identities":ids,"rows":rows,"new_decodes":6,"frozen_decodes_inherited":3,"new_profiles":0,"new_proofs":0,"proof_ledger":30846,"production_changed":false,"adopted":false,"timing_measured":false,"limits":"Static consumer codegen and correctness only; stack operands/code sizes are not dynamic spill cost, latency, peak memory or minimal RAM."});
    save("qualification.json", &result);
    save("qualification-pins.json", &json!(pins));
    for r in result["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["role"] != "FROZEN")
    {
        println!(
            "{} {}: rows{} IFMA{} add{} sub{} stack{}",
            r["role"],
            r["kind"],
            r["instruction_rows"],
            r["ifma"],
            r["add"],
            r["sub"],
            r["vector_rsp_rbp_operand_rows"]
        );
    }
}
