use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
fn main() {
    let bytes = fs::read(format!("{R}/caller.disassembly.txt")).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    let mut functions = BTreeMap::<String, String>::new();
    let mut name = None;
    for line in text.lines() {
        if line.starts_with('0') && line.ends_with(">:") {
            name = if line.contains("h_only_probe")
                && !line.contains("drop_glue")
                && !line.contains("grow_one")
            {
                Some(line.to_owned())
            } else {
                None
            };
        }
        if let Some(name) = &name {
            functions
                .entry(name.clone())
                .or_default()
                .push_str(&format!("{line}\n"));
        }
    }
    assert_eq!(functions.len(), 4);
    let mut records = Vec::new();
    for (name, body) in &functions {
        let mut mnemonics = BTreeMap::<String, usize>::new();
        let mut prologue_sub_bytes = 0;
        let mut pushes = 0;
        let mut instruction_count = 0;
        for line in body.lines().skip(1) {
            let Some((_, ins)) = line.split_once(':') else {
                continue;
            };
            let ins = ins.trim();
            let Some(mnemonic) = ins.split_whitespace().next() else {
                continue;
            };
            instruction_count += 1;
            *mnemonics.entry(mnemonic.to_owned()).or_default() += 1;
            if instruction_count <= 12 {
                if mnemonic == "push" {
                    pushes += 8;
                }
                if mnemonic == "sub" && ins.ends_with(",%rsp") {
                    let n = ins.split_once("$0x").unwrap().1.split(',').next().unwrap();
                    prologue_sub_bytes += usize::from_str_radix(n, 16).unwrap();
                }
            }
        }
        let ifma = mnemonics.get("vpmadd52luq").copied().unwrap_or(0)
            + mnemonics.get("vpmadd52huq").copied().unwrap_or(0);
        if name.contains("Ternary") {
            assert_eq!(prologue_sub_bytes, 0x1348);
            assert_eq!(pushes, 48);
            assert!(ifma > 0);
        }
        records.push(json!({"symbol":name,"static_instructions":instruction_count,"static_IFMA_sites":ifma,"prologue_rsp_sub_bytes":prologue_sub_bytes,"prologue_push_bytes":pushes,"frame_includes_spills_and_outgoing_arguments":true,"not_a_dynamic_spill_cost_or_total_thread_stack_bound":true,"mnemonics":mnemonics}));
    }
    let original: Value =
        serde_json::from_slice(&fs::read(format!("{R}/review.json")).unwrap()).unwrap();
    let caller = fs::read(format!("{R}/caller")).unwrap();
    let caller_hash = format!("{:x}", Sha256::digest(&caller));
    assert_eq!(caller_hash, original["binaries_sha256"]["caller"]);
    let out = json!({"passed":true,"caller_sha256":caller_hash,"disassembly_sha256":format!("{:x}",Sha256::digest(bytes)),"actual_decode_argv":["objdump","-d","-C","--no-show-raw-insn","caller"],"decode_exit_status":0,"saved_decode_reused_without_rerunning":true,"functions":records,"performance_claim":false,"no_timing_or_PMU_or_profiles":true});
    fs::write(
        format!("{R}/codegen.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    fs::write(
        format!("{R}/selected-disassembly.json"),
        serde_json::to_vec(&functions).unwrap(),
    )
    .unwrap();
    println!("Final-consumer codegen PASS: 4functions; H-only layer prologue4936+48push bytes; native IFMA present; no speed inference");
}
