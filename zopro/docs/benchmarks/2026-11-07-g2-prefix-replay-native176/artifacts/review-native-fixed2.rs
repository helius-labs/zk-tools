use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-prefix-replay176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn load(s: &str) -> Value {
    serde_json::from_slice(&fs::read(s).unwrap()).unwrap()
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn hash(s: &str) -> String {
    let mut f = fs::File::open(s).unwrap();
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
fn replace(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "inverse not unique {a}");
    *s = s.replacen(a, b, 1);
}
fn main() {
    let manifest=load(&format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut exact = 0;
    let mut field = 0;
    let mut changed = BTreeMap::new();
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(&format!("{Z}/{p}")), h.as_str().unwrap());
        let newpath = format!("{R}/source/{p}");
        let got = hash(&newpath);
        if p.starts_with("crates/zopro-bn254/") {
            assert_eq!(got, h.as_str().unwrap());
            field += 1;
        }
        if got == h.as_str().unwrap() {
            exact += 1;
            continue;
        }
        let mut source = fs::read_to_string(&newpath).unwrap();
        match p.as_str() {
            "crates/zopro-core/src/msm.rs" => {
                replace(
                    &mut source,
                    "prefix_records:Vec<g2_prefix_record::Record>",
                    "prefix_records:Vec<[FqBatch;2]>",
                );
                replace(&mut source,"\n#[cfg(all(zopro_g2_prefix_records,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nmod g2_prefix_record;\n","");
            }
            "crates/zopro-core/src/msm/g2_prefix.rs" => {
                replace(
                    &mut source,
                    "g2_prefix_record::Record::new(chains,norm)",
                    "[chains,norm]",
                );
                replace(
                    &mut source,
                    "s.prefix_records[j].prefix",
                    "s.prefix_records[j][0]",
                );
                replace(
                    &mut source,
                    "s.prefix_records[j].norm(&denominator)",
                    "s.prefix_records[j][1]",
                );
            }
            "crates/zopro-core/src/msm/g2_borrowed.rs" => {
                replace(
                    &mut source,
                    "g2_prefix_record::Record::new(chains, norm)",
                    "[chains, norm]",
                );
                replace(
                    &mut source,
                    "s.prefix_records[j].prefix",
                    "s.prefix_records[j][0]",
                );
                replace(
                    &mut source,
                    "s.prefix_records[j].norm(&denominator)",
                    "s.prefix_records[j][1]",
                );
            }
            _ => panic!("unplanned changed parent {p}"),
        };
        assert_eq!(digest(source.as_bytes()), h.as_str().unwrap());
        changed.insert(
            p,
            json!({"snapshot_sha256":got,"recovered_original_sha256":h}),
        );
    }
    assert_eq!(exact, 173);
    assert_eq!(changed.len(), 3);
    let child = format!("{R}/source/crates/zopro-core/src/msm/g2_prefix_record.rs");
    let current = fs::read_to_string(&child).unwrap();
    let first = fs::read_to_string(format!("{R}/prefix-record-first.rs")).unwrap();
    assert_eq!(
        first.split("#[cfg(test)]").next(),
        current.split("#[cfg(test)]").next()
    );
    assert!(current.contains("Fq2::lanes_norm(denominator)"));
    let runtime = current.split("#[cfg(test)]").next().unwrap();
    assert!(!runtime.contains("unsafe"));
    assert!(!runtime.contains("Vec<"));
    let pins = load(&format!("{R}/build-pins-fixed.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let plan = load(&format!("{R}/build-plan-fixed.json"));
    assert_eq!(
        load(&format!("{R}/build-fixed-runs/result.json"))["status"],
        "complete"
    );
    let mut tests = BTreeMap::new();
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let base = format!("{R}/build-fixed-runs/{n}");
        let e = load(&format!("{base}/exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let cmd = load(&format!("{base}/command.json"));
        let got = cmd["argv"].as_array().unwrap();
        let wanted = s["argv"].as_array().unwrap();
        assert_eq!(&got[got.len() - wanted.len()..], wanted);
        for (k, v) in s["environment"].as_object().unwrap() {
            if k == "RUSTC" || k == "RUSTUP_TOOLCHAIN" {
                // Workflow applies these, but command receipts retain managed
                // variables only. Preserve that evidence boundary explicitly.
                assert!(cmd["environment"].get(k).is_none());
            } else {
                assert_eq!(&cmd["environment"][k], v);
            }
        }
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(&format!("{base}/{stream}.log")),
                e[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        let output = fs::read_to_string(format!("{base}/stdout.log")).unwrap();
        assert!(output.contains(
            "test result: ok. 126 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
        ));
        assert!(output.contains("prefix_record_size_and_signed_owned_reuse ... ok"));
        tests.insert(n, 126);
    }
    let off = plan["steps"][0]["environment"]["RUSTFLAGS"]
        .as_str()
        .unwrap();
    let on = plan["steps"][1]["environment"]["RUSTFLAGS"]
        .as_str()
        .unwrap();
    assert_eq!(on, format!("{off} --cfg zopro_g2_prefix_replay"));
    assert_eq!(off, manifest["rustflags"].as_str().unwrap());
    let failed = load(&format!("{R}/build-runs/core-off/exit.json"));
    assert_eq!(failed["returncode"], 101);
    let first_completed = format!("{R}/build-runs/completed.json");
    if Path::new(&first_completed).exists() {
        assert_eq!(load(&first_completed).as_array().unwrap().len(), 0);
    }
    assert!(!Path::new(&format!("{R}/build-runs/core-on")).exists());
    let mut rows = vec![];
    for g in [0u64, 1, 4, 25, 44, 64, 128] {
        rows.push(json!({"eight_point_groups":g,"OFF_logical_record_bytes":g*640,"ON_logical_record_bytes":g*320,"OFF_variable_IFMA_proxy":g*1450,"ON_variable_IFMA_proxy":g*1600,"added_VF":g}));
    }
    let out = format!("{R}/native-review.json");
    assert!(!Path::new(&out).exists());
    fs::write(out,serde_json::to_vec_pretty(&json!({"review_passed":true,"native_correctness_qualified":true,"production_changed":false,"accepted_source_files":176,"unchanged_parents":exact,"reversible_parents":changed,"new_children":1,"unchanged_field_parents":field,"tests":tests,"first_failed_compile_preserved":true,"test_only_repair_runtime_prefix_exact":true,"actual_OFF_flags_match_accepted":true,"actual_ON_adds_only_replay_cfg":true,"layout_forecast":rows,"memory_limits":"logical records only; sameVec header/count; not process peak, actual stack or minimumRAM. Generic trees/G1/tails/scheduler unchanged.","new_proofs":0,"new_timings":0,"performance_qualified":false,"native_source_not_adopted":true})).unwrap()).unwrap();
    println!("PASS173exact+3recovered+1child;126OFF/126ONcoretests;no performance claim");
}
