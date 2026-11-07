use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/cios-outer-loop176";
const REPO: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn word(b: &[u8], off: usize, size: usize) -> u64 {
    let mut w = [0; 8];
    w[..size].copy_from_slice(&b[off..off + size]);
    u64::from_le_bytes(w)
}
fn symbol(b: &[u8], wanted: &str) -> (u64, u64, String) {
    assert_eq!(&b[..6], b"\x7fELF\x02\x01");
    let shoff = word(b, 40, 8) as usize;
    let stride = word(b, 58, 2) as usize;
    assert_eq!(stride, 64);
    for i in 0..word(b, 60, 2) as usize {
        let s = shoff + i * stride;
        if word(b, s + 4, 4) != 2 {
            continue;
        }
        let strings = shoff + word(b, s + 40, 4) as usize * stride;
        let st = word(b, strings + 24, 8) as usize;
        let start = word(b, s + 24, 8) as usize;
        let size = word(b, s + 32, 8) as usize;
        let ent = word(b, s + 56, 8) as usize;
        assert_eq!(ent, 24);
        for p in (start..start + size).step_by(ent) {
            let np = st + word(b, p, 4) as usize;
            let len = b[np..].iter().position(|x| *x == 0).unwrap();
            if &b[np..np + len] != wanted.as_bytes() {
                continue;
            }
            assert_eq!(b[p + 4] & 15, 2);
            let addr = word(b, p + 8, 8);
            let len = word(b, p + 16, 8);
            let sec = shoff + word(b, p + 6, 2) as usize * stride;
            let off = word(b, sec + 24, 8) + addr - word(b, sec + 16, 8);
            return (
                addr,
                len,
                format!(
                    "{:x}",
                    Sha256::digest(&b[off as usize..(off + len) as usize])
                ),
            );
        }
    }
    panic!("missing symbol {wanted}");
}
fn main() {
    assert!(!Path::new(&format!("{R}/analysis.json")).exists());
    for (p, h) in read(format!("{R}/analysis-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let manifest=read(format!("{REPO}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut exact = 0;
    let mut recovered = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(format!("{REPO}/{p}")), h.as_str().unwrap());
        if !p.starts_with("crates/zopro-bn254/") {
            continue;
        }
        let local = format!("{R}/source/{p}");
        if hash(&local) == h.as_str().unwrap() {
            exact += 1;
            continue;
        }
        assert!(p.ends_with("/ifma.rs") || p.ends_with("/ifma8.rs"));
        let mut text = fs::read_to_string(local).unwrap();
        assert_eq!(text.matches("($i:expr)").count(), 1);
        text = text.replacen("($i:expr)", "($i:literal)", 1);
        let start = text
            .find("            if cfg!(zopro_cios_outer_loop)")
            .unwrap();
        let end = start + text[start..].find("            if MIXED {").unwrap();
        text.replace_range(start..end,"            row!(0);\n            row!(1);\n            row!(2);\n            row!(3);\n");
        assert_eq!(
            text,
            fs::read_to_string(format!("{REPO}/{p}")).unwrap(),
            "inverse edit {p}"
        );
        recovered += 1;
    }
    assert_eq!((exact, recovered), (16, 2));
    let mut successes = 0;
    let mut failures = 0;
    for (plan, dir) in [
        ("build-plan.json", "build"),
        ("continuation-plan.json", "continuation"),
    ] {
        for step in read(format!("{R}/{plan}"))["steps"].as_array().unwrap() {
            let name = step["name"].as_str().unwrap();
            let base = format!("{R}/{dir}/{name}");
            if !Path::new(&format!("{base}/exit.json")).exists() {
                assert_eq!(dir, "build");
                continue;
            }
            let exit = read(format!("{base}/exit.json"));
            let command = read(format!("{base}/command.json"));
            let actual = command["argv"].as_array().unwrap();
            let expected = step["argv"].as_array().unwrap();
            assert_eq!(&actual[actual.len() - expected.len()..], expected);
            for stream in ["stdout", "stderr"] {
                assert_eq!(
                    hash(format!("{base}/{stream}.log")),
                    exit[format!("{stream}_sha256")].as_str().unwrap()
                );
            }
            if exit["success"] == true {
                assert_eq!(exit["returncode"], 0);
                successes += 1;
            } else {
                assert_eq!((dir, name), ("build", "probe-accepted"));
                failures += 1;
            }
            if name.starts_with("probe-") && exit["success"] == true {
                for flag in ["-O", "target-cpu=native", "lto=fat", "codegen-units=1"] {
                    assert!(expected.contains(&json!(flag)), "missing {flag}");
                }
                assert!(expected.contains(&json!(format!("{R}/probe-fixed.rs"))));
            }
        }
    }
    assert_eq!((successes, failures), (17, 1));
    assert_eq!(
        read(format!("{R}/continuation/result.json"))["status"],
        "complete"
    );
    let mut panels = BTreeMap::new();
    for mode in ["accepted", "OFF", "ON"] {
        let binary = fs::read(format!("{R}/probe-{mode}")).unwrap();
        for name in [
            "probe_fq8_mul",
            "probe_fq8_sum2",
            "probe_fr8_mul",
            "probe_fq4_mul",
        ] {
            let (start, size, body_sha) = symbol(&binary, name);
            let decoded =
                fs::read_to_string(format!("{R}/continuation/decode-{mode}-{name}/stdout.log"))
                    .unwrap();
            let mut ins = Vec::new();
            for line in decoded.lines() {
                let Some((a, tail)) = line.trim().split_once(':') else {
                    continue;
                };
                let Ok(addr) = u64::from_str_radix(a, 16) else {
                    continue;
                };
                let mut words = tail.split_whitespace();
                let Some(op) = words.next() else {
                    continue;
                };
                assert!(addr >= start && addr < start + size);
                ins.push((addr, op.to_string(), words.collect::<Vec<_>>().join(" ")));
            }
            assert!(!ins.is_empty());
            let mut backedges = Vec::new();
            let mut ifma_backedges = 0;
            for (addr, op, args) in &ins {
                if !op.starts_with('j') {
                    continue;
                }
                let target = args.split_whitespace().next().unwrap();
                let Ok(target) = u64::from_str_radix(target.trim_start_matches("0x"), 16) else {
                    continue;
                };
                if target >= *addr {
                    continue;
                }
                assert!(ins.iter().any(|i| i.0 == target));
                let ifmas = ins
                    .iter()
                    .filter(|i| i.0 >= target && i.0 <= *addr && i.1.starts_with("vpmadd52"))
                    .count();
                if ifmas > 0 {
                    ifma_backedges += 1;
                }
                backedges.push(json!({"branch_offset":addr-start,"target_offset":target-start,"IFMA_sites_in_address_range":ifmas}));
            }
            let calls = ins.iter().filter(|i| i.1.starts_with("call")).count();
            panels.insert(format!("{mode}/{name}"),json!({"start":start,"size_bytes":size,"body_sha256":body_sha,"instructions":ins.len(),"IFMA_sites":ins.iter().filter(|i|i.1.starts_with("vpmadd52")).count(),"calls":calls,"backedges":backedges,"IFMA_backedges":ifma_backedges}));
        }
    }
    let mut sizes = BTreeMap::new();
    for mode in ["accepted", "OFF", "ON"] {
        sizes.insert(
            mode,
            ["probe_fq8_mul", "probe_fq8_sum2"]
                .iter()
                .map(|n| {
                    panels[&format!("{mode}/{n}")]["size_bytes"]
                        .as_u64()
                        .unwrap()
                })
                .sum::<u64>(),
        );
    }
    let loops = ["probe_fq8_mul", "probe_fq8_sum2"].iter().all(|n| {
        panels[&format!("ON/{n}")]["IFMA_backedges"]
            .as_u64()
            .unwrap()
            > 0
    });
    let no_calls = ["probe_fq8_mul", "probe_fq8_sum2"]
        .iter()
        .all(|n| panels[&format!("ON/{n}")]["calls"] == 0);
    let smaller = sizes["ON"] < sizes["OFF"] && sizes["ON"] < sizes["accepted"];
    let qualified = loops && no_calls && smaller;
    let out = json!({"panels":panels,"combined_Fq8_bytes":sizes,"both_Fq8_IFMA_loops_retained":loops,"no_Fq8_calls":no_calls,"combined_Fq8_smaller_than_both_controls":smaller,"static_selection_passed":qualified,"source_files_production_verified":176,"field_snapshot_exact":exact,"field_snapshot_recovered":recovered,"successful_build_decode_commands":successes,"preserved_failed_compile_commands":failures,"new_arithmetic_executions":0,"new_tests":0,"new_timings":0,"new_proofs":0,"adopted":false,"decision":if qualified{"static structure qualifies for separate arithmetic qualification"}else{"close fixed source recipe before arithmetic execution or timing"}});
    fs::write(
        format!("{R}/analysis.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("static selection={qualified}; Fq8 combined bytes {sizes:?}; arithmetic/timing/proofs=0; {successes} successful build/decode receipts and {failures} failed compile retained");
}
