use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-records176";
fn main() {
    assert!(!std::path::Path::new(R).join("codegen.json").exists());
    let review: Value =
        serde_json::from_slice(&fs::read(format!("{R}/review.json")).unwrap()).unwrap();
    assert_eq!(review["review_passed"], true);
    let exe = format!("{R}/oracle-tests-native");
    let h = format!("{:x}", Sha256::digest(fs::read(&exe).unwrap()));
    assert_eq!(h, review["outputs_sha256"]["oracle-tests-native"]);
    let args = ["-d", "-C", "--no-show-raw-insn", &exe];
    let output = Command::new("objdump").args(args).output().unwrap();
    fs::write(format!("{R}/codegen-objdump.stdout.log"), &output.stdout).unwrap();
    fs::write(format!("{R}/codegen-objdump.stderr.log"), &output.stderr).unwrap();
    assert!(output.status.success());
    let mut functions = BTreeMap::<String, String>::new();
    let mut selected = None;
    for l in std::str::from_utf8(&output.stdout).unwrap().lines() {
        if l.starts_with('0') && l.ends_with(">:") {
            selected = if (l.contains("zopro_core::msm::reduce_layer") && l.contains("G1Config"))
                || l.contains("zopro_core::msm::g1_tree_records_probe")
                || l.contains("zopro_core::msm::weighted_sum") && l.contains("G1Config")
            {
                Some(l.to_owned())
            } else {
                None
            };
        }
        if let Some(name) = &selected {
            functions
                .entry(name.clone())
                .or_default()
                .push_str(&format!("{l}\n"));
        }
    }
    assert!(!functions.is_empty());
    let mut records = Vec::new();
    let mut total_ifma = 0;
    for (name, body) in &functions {
        let mut instructions = 0;
        let mut ifma = 0;
        let mut rsp_sub = 0;
        let mut pushes = 0;
        let mut calls = Vec::new();
        for l in body.lines().skip(1) {
            let Some((_, instruction)) = l.split_once(':') else {
                continue;
            };
            let i = instruction.trim();
            if i.is_empty() {
                continue;
            }
            instructions += 1;
            let mnemonic = i.split_whitespace().next().unwrap();
            if mnemonic == "vpmadd52luq" || mnemonic == "vpmadd52huq" {
                ifma += 1;
            }
            if instructions <= 16 {
                if mnemonic == "push" {
                    pushes += 8;
                }
                if mnemonic == "sub" && i.ends_with(",%rsp") {
                    let n = i.split_once("$0x").unwrap().1.split(',').next().unwrap();
                    rsp_sub += usize::from_str_radix(n, 16).unwrap();
                }
            }
            if mnemonic == "call" {
                calls.push(i.to_owned());
            }
        }
        total_ifma += ifma;
        records.push(json!({"symbol":name,"static_instructions":instructions,"IFMA_sites":ifma,"prologue_rsp_sub_bytes":rsp_sub,"prologue_push_bytes":pushes,"calls":calls,"not_whole_thread_stack_or_dynamic_spill_cost":true}));
    }
    assert!(total_ifma > 0);
    fs::write(
        format!("{R}/selected-disassembly.json"),
        serde_json::to_vec(&functions).unwrap(),
    )
    .unwrap();
    let result = json!({"passed":true,"actual_native_oracle_final_consumer":exe,"consumer_sha256":h,"argv":args,"command":"objdump","exit_status":0,"stdout_sha256":format!("{:x}",Sha256::digest(&output.stdout)),"functions":records,"IFMA_sites_total":total_ifma,"timing_driver_not_built":true,"performance_claim":false,"stack_tests_inherited_from_completed_workflow":true});
    fs::write(
        format!("{R}/codegen.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Final-native-oracle consumer codegenPASS {}functions/{total_ifma}staticIFMA sites; no performance or spillcost claim",functions.len());
}
