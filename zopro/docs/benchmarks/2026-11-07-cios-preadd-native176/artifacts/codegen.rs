use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/cios-preadd-native176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/cios-outer-loop176";
const REPO: &str = "/home/ubuntu/src/zk-tools/zopro";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
#[allow(dead_code)]
mod inherited_graph {
    include!("/home/ubuntu/src/zopro-perf-work/fq8-dependency176/audit.rs");
    pub fn inspect(text: &str) -> serde_json::Value {
        let ins = decode(text);
        let prefix_len = ins
            .iter()
            .position(|i| i.op.starts_with('j'))
            .unwrap_or(ins.len());
        let prefix = &ins[..prefix_len];
        let mut scores = Vec::new();
        for source in [0, 4] {
            let nodes = graph(prefix, source);
            check_relaxation(&nodes);
            scores.push(json!({"hypothetical_source_weight":source,"completion_cycles_in_model":nodes.iter().map(|n|n.ready).max().unwrap()}));
        }
        json!({"prefix_instructions":prefix_len,"function_instructions":ins.len(),"IFMA_sites":ins.iter().filter(|i|i.op.starts_with("vpmadd52")).count(),"calls":ins.iter().filter(|i|i.op.starts_with("call")).count(),"prefix_stack_operand_rows":prefix.iter().filter(|i|i.args.iter().any(|s|s.contains("[rsp")||s.contains("[rbp"))).count(),"function_stack_operand_rows":ins.iter().filter(|i|i.args.iter().any(|s|s.contains("[rsp")||s.contains("[rbp"))).count(),"prefix_ZMM_stack_store_rows":prefix.iter().filter(|i|i.op.starts_with("vmov")&&i.args.first().is_some_and(|s|s.contains("ZMMWORD")&&(s.contains("[rsp")||s.contains("[rbp")))).count(),"scores":scores})
    }
}
#[allow(dead_code)]
mod inherited_elf {
    include!("/home/ubuntu/src/zopro-perf-work/cios-outer-loop176/analyze.rs");
    pub fn info(bytes: &[u8], name: &str) -> serde_json::Value {
        let (start, size, sha) = symbol(bytes, name);
        serde_json::json!({"start":start,"size_bytes":size,"body_sha256":sha})
    }
}
fn main() {
    assert!(!Path::new(&format!("{R}/codegen.json")).exists());
    for (p, h) in load(format!("{R}/codegen-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let manifest=load(format!("{REPO}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut changed = Vec::new();
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(format!("{REPO}/{p}")), h.as_str().unwrap());
        if hash(format!("{R}/source/{p}")) == h.as_str().unwrap() {
            exact += 1;
        } else {
            changed.push(p.clone());
        }
    }
    assert_eq!(exact, 173);
    assert_eq!(
        changed,
        [
            "crates/zopro-bn254/src/ifma.rs",
            "crates/zopro-bn254/src/ifma8.rs",
            "scripts/ifma8.py"
        ]
    );
    let plan = load(format!("{R}/build-plan.json"));
    let mut receipts = 0;
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        let e = load(format!("{R}/build/{name}/exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let command = load(format!("{R}/build/{name}/command.json"));
        let a = command["argv"].as_array().unwrap();
        let wanted = step["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - wanted.len()..], wanted);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(format!("{R}/build/{name}/{stream}.log")),
                e[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        if name.starts_with("probe-") {
            for flag in ["-O", "target-cpu=native", "lto=fat", "codegen-units=1"] {
                assert!(wanted.contains(&json!(flag)));
            }
        }
        if name == "field-ON" {
            assert!(wanted.contains(&json!("zopro_cios_preadd_local")));
        }
        if name == "field-OFF" {
            assert!(!wanted.contains(&json!("zopro_cios_preadd_local")));
        }
        receipts += 1;
    }
    assert_eq!(receipts, 14);
    assert_eq!(load(format!("{R}/build/result.json"))["status"], "complete");
    let tests = fs::read_to_string(format!("{R}/build/field-tests-ON/stdout.log")).unwrap();
    assert!(tests.contains("37 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    for name in [
        "ifma::tests::preadd_native_chain_matches_arkworks",
        "ifma8::tests::preadd_native_chain_matches_arkworks",
    ] {
        assert!(tests.contains(&format!("test {name} ... ok")));
    }
    let stderr = fs::read_to_string(format!("{R}/build/field-tests-ON/stderr.log")).unwrap();
    for row in [
        "PRE_ADD_NATIVE_LANES=98312 WIDTH=8",
        "PRE_ADD_NATIVE_LANES=49156 WIDTH=4",
    ] {
        assert_eq!(stderr.matches(row).count(), 1);
    }
    let mut panels = BTreeMap::new();
    let mut native_graph_improves = true;
    let mut unchanged_counts = true;
    for mode in ["accepted", "OFF", "ON"] {
        let path = if mode == "accepted" {
            format!("{P}/probe-accepted")
        } else {
            format!("{R}/probe-{mode}")
        };
        let bytes = fs::read(path).unwrap();
        for name in [
            "probe_fq8_mul",
            "probe_fq8_sum2",
            "probe_fr8_mul",
            "probe_fq4_mul",
        ] {
            let decode = if mode == "accepted" {
                format!("{P}/continuation/decode-accepted-{name}/stdout.log")
            } else {
                format!("{R}/build/decode-{mode}-{name}/stdout.log")
            };
            let info = inherited_elf::info(&bytes, name);
            let mut v = inherited_graph::inspect(&fs::read_to_string(decode).unwrap());
            v["elf"] = info;
            panels.insert(format!("{mode}/{name}"), v);
        }
    }
    for name in ["probe_fq8_mul", "probe_fq8_sum2"] {
        let accepted = &panels[&format!("accepted/{name}")];
        let off = &panels[&format!("OFF/{name}")];
        let on = &panels[&format!("ON/{name}")];
        assert_eq!(accepted["IFMA_sites"], off["IFMA_sites"]);
        unchanged_counts &= on["IFMA_sites"] == off["IFMA_sites"];
        for index in 0..2 {
            assert_eq!(accepted["scores"][index], off["scores"][index]);
            native_graph_improves &= on["scores"][index]["completion_cycles_in_model"]
                .as_u64()
                .unwrap()
                < off["scores"][index]["completion_cycles_in_model"]
                    .as_u64()
                    .unwrap();
        }
    }
    let controls=vec!["probe_fq4_mul","probe_fr8_mul"].into_iter().map(|name|{let off=&panels[&format!("OFF/{name}")];let on=&panels[&format!("ON/{name}")];assert_eq!(off["IFMA_sites"],on["IFMA_sites"]);assert_eq!(off["elf"]["size_bytes"],on["elf"]["size_bytes"]);json!({"name":name,"same_size_and_IFMA_count":true,"exact_function_bytes_equal":off["elf"]["body_sha256"]==on["elf"]["body_sha256"]})}).collect::<Vec<_>>();
    let no_calls = panels.values().all(|v| v["calls"] == 0);
    let no_zmm_stores = panels
        .values()
        .all(|v| v["prefix_ZMM_stack_store_rows"] == 0);
    let selected = native_graph_improves && unchanged_counts && no_calls && no_zmm_stores;
    let out = json!({"panels":panels,"controls":controls,"native_graph_improves_all_four_comparisons":native_graph_improves,"unchanged_IFMA_counts":unchanged_counts,"no_calls":no_calls,"no_prefix_ZMM_stack_stores":no_zmm_stores,"fixed_caller_measurement_selected":selected,"source_parent_files_exact":exact,"source_changed_parents":changed,"new_native_field_tests_passed":37,"new_native_chain_lane_outputs":147468,"successful_build_test_decode_receipts":receipts,"inherited_graph_toys_not_replayed":true,"memory_dependencies_ignored_in_graph":true,"graph_is_not_latency":true,"new_proofs":0,"new_timings":0,"adopted":false});
    fs::write(
        format!("{R}/codegen.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("37native tests PASS;147468new chain-lane outputs; native dependency model improves={native_graph_improves}; caller selected={selected}");
}
