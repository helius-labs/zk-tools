use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/tmp/zopro-g1-reference-staging";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn pretty(s: &str) -> String {
    rustc_demangle::try_demangle(s)
        .map(|s| format!("{s:#}"))
        .unwrap_or_else(|_| s.to_owned())
}
fn normalize(asm: &str, local: &BTreeMap<u64, usize>) -> String {
    let asm = asm.split('#').next().unwrap().trim();
    let mut w = asm.split_whitespace();
    let Some(op) = w.next() else {
        return String::new();
    };
    let rest = w.collect::<Vec<_>>().join(" ");
    if op.starts_with('j') || op == "call" {
        if let Some(target) = rest
            .split_whitespace()
            .next()
            .and_then(|s| u64::from_str_radix(s, 16).ok())
        {
            if let Some(i) = local.get(&target) {
                return format!("{op} <local:{i}>");
            }
            let dest = rest
                .split_once('<')
                .and_then(|(_, s)| s.strip_suffix('>'))
                .unwrap_or("unknown-external-target");
            return format!("{op} <{}>", pretty(dest));
        }
    }
    let mut out = format!("{op} {rest}");
    if let Some(begin) = out.find("[rip") {
        if let Some(end) = out[begin..].find(']') {
            out.replace_range(begin..begin + end + 1, "[rip+<relocation>]");
        }
    }
    out
}
fn instructions(text: &str) -> Vec<(u64, String)> {
    text.lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(':')?;
            let a = a.trim();
            if a.is_empty() || !a.bytes().all(|x| x.is_ascii_hexdigit()) {
                return None;
            }
            let asm = b.trim();
            if asm.is_empty() {
                return None;
            }
            Some((u64::from_str_radix(a, 16).ok()?, asm.to_owned()))
        })
        .collect()
}
fn main() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native-v2/result.json"))["status"], "complete");
    let manifest = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    let mut rows = Vec::new();
    let mut bodies = BTreeMap::<String, BTreeMap<String, Vec<String>>>::new();
    for (variant, path) in [
        (
            "accepted",
            manifest["native_binary"].as_str().unwrap().to_owned(),
        ),
        (
            "fresh",
            read(r.join("native-v2/fresh/artifacts.json"))["binary"]
                .as_str()
                .unwrap()
                .to_owned(),
        ),
        (
            "candidate",
            read(r.join("native-v2/candidate/artifacts.json"))["binary"]
                .as_str()
                .unwrap()
                .to_owned(),
        ),
    ] {
        let out = Command::new("nm")
            .args(["-S", "--defined-only", &path])
            .output()
            .unwrap();
        assert!(out.status.success());
        let symbols = String::from_utf8(out.stdout).unwrap();
        let mut selected = BTreeMap::new();
        for line in symbols.lines() {
            let f: Vec<_> = line.split_whitespace().collect();
            if f.len() != 4 {
                continue;
            }
            let name = pretty(f[3]);
            if !name.contains("::msm::stream_g1_impl") {
                continue;
            }
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let bytes = u64::from_str_radix(f[1], 16).unwrap();
            let out = Command::new("objdump")
                .args(["-d", "--no-show-raw-insn", "-M", "intel"])
                .arg(format!("--start-address={start}"))
                .arg(format!("--stop-address={}", start + bytes))
                .arg(&path)
                .output()
                .unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            let code = instructions(&text);
            assert!(!code.is_empty());
            let local: BTreeMap<_, _> =
                code.iter().enumerate().map(|(i, (a, _))| (*a, i)).collect();
            let normalized: Vec<_> = code.iter().map(|(_, s)| normalize(s, &local)).collect();
            let id = selected.len();
            fs::write(r.join(format!("{variant}-g1-{id}.asm.txt")), &text).unwrap();
            fs::write(
                r.join(format!("{variant}-g1-{id}.normalized.txt")),
                normalized.join("\n"),
            )
            .unwrap();
            let mut counts = BTreeMap::<String, u64>::new();
            for (_, asm) in &code {
                let op = asm.split_whitespace().next().unwrap();
                *counts.entry("instructions".into()).or_default() += 1;
                if asm.contains('[') {
                    *counts
                        .entry("memory_operand_instructions".into())
                        .or_default() += 1
                }
                if asm.contains("[rsp") || asm.contains("[rbp") {
                    *counts
                        .entry("stack_reference_instructions".into())
                        .or_default() += 1
                }
                if op.starts_with("vpmadd52") {
                    *counts.entry("IFMA_sites".into()).or_default() += 1
                }
                if [
                    "vperm",
                    "vpunpck",
                    "vshuf",
                    "vpshuf",
                    "vinsert",
                    "vextract",
                    "vpbroadcast",
                ]
                .iter()
                .any(|p| op.starts_with(p))
                {
                    *counts.entry("shuffle_broadcast_sites".into()).or_default() += 1
                }
            }
            rows.push(json!({"variant":variant,"binary":path,"binary_sha256":hash(&path),"symbol":name,"bytes":bytes,"counts":counts,"disassembly":r.join(format!("{variant}-g1-{id}.asm.txt")),"normalized_sha256":hash(r.join(format!("{variant}-g1-{id}.normalized.txt")))}));
            assert!(selected.insert(name, normalized).is_none());
        }
        assert!(!selected.is_empty());
        bodies.insert(variant.into(), selected);
    }
    let mut comparisons = Vec::new();
    for (name, body) in &bodies["candidate"] {
        let fresh = &bodies["fresh"][name];
        let accepted = &bodies["accepted"][name];
        comparisons.push(json!({"symbol":name,"candidate_fresh_normalized_shape_equal":body==fresh,"fresh_accepted_normalized_shape_equal":fresh==accepted,"candidate_instructions":body.len(),"fresh_instructions":fresh.len(),"accepted_instructions":accepted.len(),"first_difference_index":body.iter().zip(fresh).position(|(a,b)|a!=b)}));
    }
    let all_same = comparisons
        .iter()
        .all(|c| c["candidate_fresh_normalized_shape_equal"] == true);
    fs::write(r.join("codegen.json"),serde_json::to_vec_pretty(&json!({"before_any_SDK_capture":true,"static_only":true,"rows":rows,"comparisons":comparisons,"all_stream_instruction_shapes_equal":all_same,"normalization":"preserve registers, stack offsets, constants and local control-flow instruction indices; normalize external symbol hashes and RIP displacements only","not_dynamic_spill_counts_or_cache_wall_causality":true})).unwrap()).unwrap();
    println!(
        "ActualSDKG1staticcomparison:all_instruction_shapes_equal={all_same};{}symbols",
        comparisons.len()
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_stack_offsets_registers_and_constants() {
        let m = BTreeMap::new();
        assert_ne!(
            normalize("vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x40]", &m),
            normalize("vmovdqa64 zmm0,ZMMWORD PTR [rsp+0x80]", &m)
        );
        assert_ne!(normalize("add rax,0x12", &m), normalize("add rax,0x13", &m));
        assert_ne!(normalize("add rax,0x12", &m), normalize("add rbx,0x12", &m));
    }
    #[test]
    fn preserves_local_jump_destination() {
        let m = BTreeMap::from([(16, 2), (32, 4)]);
        assert_eq!(normalize("jne 10 <own+0x1>", &m), "jne <local:2>");
        assert_ne!(
            normalize("jne 10 <own+0x1>", &m),
            normalize("jne 20 <own+0x2>", &m)
        );
    }
    #[test]
    fn normalizes_only_RIP_reference_and_external_symbol_address() {
        let m = BTreeMap::new();
        assert_eq!(
            normalize("vmovdqa64 zmm0,ZMMWORD PTR [rip+0x124] # constant", &m),
            normalize("vmovdqa64 zmm0,ZMMWORD PTR [rip+0x999] # constant", &m)
        );
        assert_eq!(
            normalize("call 100 <callee>", &m),
            normalize("call 200 <callee>", &m)
        );
        assert_ne!(
            normalize("call 100 <callee>", &m),
            normalize("call 200 <other>", &m)
        );
    }
}
