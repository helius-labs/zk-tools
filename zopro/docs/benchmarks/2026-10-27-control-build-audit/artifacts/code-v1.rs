use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, process::Command};
mod elf {
    include!("sections.rs");
    pub fn lookup(b: &[u8], address: u64, width: usize) -> Option<(String, Vec<u8>)> {
        for (name, s) in sections(b) {
            if s.kind == 8 {
                continue;
            }
            if let Some(delta) = address.checked_sub(s.address) {
                if delta
                    .checked_add(width as u64)
                    .is_some_and(|end| end <= s.size as u64)
                {
                    let begin = s.offset + delta as usize;
                    return Some((name, b[begin..begin + width].to_vec()));
                }
            }
        }
        None
    }
}
fn read(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn text(command: &str, args: &[&str]) -> String {
    let out = Command::new(command).args(args).output().unwrap();
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn instructions(s: &str) -> BTreeMap<u64, String> {
    let mut out = BTreeMap::new();
    for line in s.lines() {
        if let Some((a, b)) = line.split_once(':') {
            let a = a.trim();
            if a.is_empty() || !a.bytes().all(|b| b.is_ascii_hexdigit()) {
                continue;
            }
            let b = b.trim();
            if b.is_empty() {
                continue;
            }
            assert!(out
                .insert(u64::from_str_radix(a, 16).unwrap(), b.to_owned())
                .is_none());
        }
    }
    out
}
fn plain(s: &str) -> String {
    s.split('#')
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn shape(s: &str) -> String {
    let mut s = plain(s);
    if let Some(begin) = s.find("[rip") {
        let end = begin + s[begin..].find(']').unwrap();
        s.replace_range(begin..=end, "[rip+<relocation>]");
    }
    s
}
fn target(s: &str) -> u64 {
    u64::from_str_radix(
        s.split_once('#')
            .unwrap()
            .1
            .split_whitespace()
            .next()
            .unwrap(),
        16,
    )
    .unwrap()
}
fn width(s: &str) -> Option<usize> {
    for (word, n) in [
        ("ZMMWORD PTR", 64),
        ("YMMWORD PTR", 32),
        ("XMMWORD PTR", 16),
        ("QWORD PTR", 8),
        ("DWORD PTR", 4),
        ("WORD PTR", 2),
        ("BYTE PTR", 1),
    ] {
        if s.contains(word) {
            return Some(n);
        }
    }
    None
}
fn functions(s: &str) -> BTreeMap<u64, (u64, String)> {
    let mut out = BTreeMap::new();
    for line in s.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        if f.len() != 4 || !matches!(f[2], "t" | "T") {
            continue;
        }
        let address = u64::from_str_radix(f[0], 16).unwrap();
        let size = u64::from_str_radix(f[1], 16).unwrap();
        out.entry(address).or_insert((size, f[3].to_owned()));
    }
    out
}
fn main() {
    let ids = read("/tmp/zopro-control-build-audit/identities.json");
    let a_path = ids["variants"]["accepted"]["binary"].as_str().unwrap();
    let b_path = ids["variants"]["fresh"]["binary"].as_str().unwrap();
    let a = fs::read(a_path).unwrap();
    let b = fs::read(b_path).unwrap();
    let a_dis = text(
        "objdump",
        &[
            "-d",
            "--section=.text",
            "--no-show-raw-insn",
            "-M",
            "intel",
            a_path,
        ],
    );
    let b_dis = text(
        "objdump",
        &[
            "-d",
            "--section=.text",
            "--no-show-raw-insn",
            "-M",
            "intel",
            b_path,
        ],
    );
    fs::write(
        "/tmp/zopro-control-build-audit/accepted-local-only.asm",
        &a_dis,
    )
    .unwrap();
    fs::write(
        "/tmp/zopro-control-build-audit/fresh-local-only.asm",
        &b_dis,
    )
    .unwrap();
    let aa = instructions(&a_dis);
    let bb = instructions(&b_dis);
    assert_eq!(aa.keys().collect::<Vec<_>>(), bb.keys().collect::<Vec<_>>());
    let af = functions(&text("nm", &["-S", "--defined-only", a_path]));
    let bf = functions(&text("nm", &["-S", "--defined-only", b_path]));
    assert_eq!(af, bf);
    let mut shape_changes = Vec::new();
    let mut references = Vec::new();
    let mut function_stats = BTreeMap::<String, Value>::new();
    for (address, asm) in &aa {
        let other = &bb[address];
        let owner = af
            .range(..=*address)
            .next_back()
            .map(|(start, (size, name))| {
                assert!(*address < start + size);
                name.clone()
            })
            .unwrap_or_else(|| "<padding>".into());
        let row=function_stats.entry(owner.clone()).or_insert(json!({"instructions":0,"raw_instruction_differences":0,"shape_differences":0,"RIP_references":0,"RIP_address_differences":0,"rodata_read_differences":0}));
        row["instructions"] = json!(row["instructions"].as_u64().unwrap() + 1);
        if plain(asm) != plain(other) {
            row["raw_instruction_differences"] =
                json!(row["raw_instruction_differences"].as_u64().unwrap() + 1);
        }
        if shape(asm) != shape(other) {
            row["shape_differences"] = json!(row["shape_differences"].as_u64().unwrap() + 1);
            shape_changes
                .push(json!({"address":address,"function":owner,"accepted":asm,"fresh":other}));
        }
        if asm.contains("[rip") {
            assert!(other.contains("[rip"));
            let at = target(asm);
            let bt = target(other);
            row["RIP_references"] = json!(row["RIP_references"].as_u64().unwrap() + 1);
            if at != bt {
                row["RIP_address_differences"] =
                    json!(row["RIP_address_differences"].as_u64().unwrap() + 1);
            }
            let operation = asm.split_whitespace().next().unwrap();
            let memory_write = plain(asm)
                .split_once(' ')
                .is_some_and(|(_, rest)| rest.split(',').next().unwrap().contains("[rip"));
            let kind = if operation == "lea" {
                "address-only"
            } else if matches!(operation, "call" | "jmp") {
                "indirect-control"
            } else if memory_write {
                "memory-destination"
            } else {
                "read"
            };
            let w = width(asm);
            assert_eq!(w, width(other));
            let left = w.and_then(|w| elf::lookup(&a, at, w));
            let right = w.and_then(|w| elf::lookup(&b, bt, w));
            let rodata = kind == "read"
                && left.as_ref().is_some_and(|x| x.0 == ".rodata")
                && right.as_ref().is_some_and(|x| x.0 == ".rodata");
            if rodata && left.as_ref().unwrap().1 != right.as_ref().unwrap().1 {
                row["rodata_read_differences"] =
                    json!(row["rodata_read_differences"].as_u64().unwrap() + 1);
            }
            references.push(json!({"address":address,"function":owner,"operation":operation,"kind":kind,"width":w,"accepted_target":at,"fresh_target":bt,"accepted_section":left.as_ref().map(|x|&x.0),"fresh_section":right.as_ref().map(|x|&x.0),"file_bytes_equal":match(&left,&right){(Some(x),Some(y))=>Some(x.1==y.1),_=>None},"accepted_bytes_sha256":left.as_ref().map(|x|hash(&x.1)),"fresh_bytes_sha256":right.as_ref().map(|x|hash(&x.1)),"readonly_rodata_load":rodata}));
        }
    }
    let rodata_reads = references
        .iter()
        .filter(|x| x["readonly_rodata_load"] == true)
        .count();
    let changed_literals = references
        .iter()
        .filter(|x| x["readonly_rodata_load"] == true && x["file_bytes_equal"] == false)
        .count();
    let summary = json!({"instruction_addresses_equal":true,"text_function_names_addresses_sizes_equal":true,"instructions":aa.len(),"functions":af.len(),"shape_difference_count":shape_changes.len(),"shape_differences":shape_changes,"RIP_references":references.len(),"changed_RIP_addresses":references.iter().filter(|x|x["accepted_target"]!=x["fresh_target"]).count(),"readonly_rodata_loads":rodata_reads,"different_rodata_load_bytes":changed_literals,"function_stats":function_stats,"full_assembly_local_only_sha256":{"accepted":hash(a_dis.as_bytes()),"fresh":hash(b_dis.as_bytes())},"scope":"Static entire .text; only RIP displacement normalized; other registers/offsets/constants/branch addresses preserved. File-image loads do not prove runtime relocations or causal performance."});
    fs::write(
        "/tmp/zopro-control-build-audit/code.json",
        serde_json::to_vec_pretty(&summary).unwrap(),
    )
    .unwrap();
    fs::write(
        "/tmp/zopro-control-build-audit/RIP-references.json",
        serde_json::to_vec_pretty(&references).unwrap(),
    )
    .unwrap();
    println!("{} instructions, {} functions, {} non-RIP shape differences; {} RIP refs, {} readonly loads, {} changed literal payloads",aa.len(),af.len(),shape_changes.len(),references.len(),rodata_reads,changed_literals);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_RIP_displacement_is_normalized() {
        assert_eq!(
            shape("vmovdqa64 zmm0,ZMMWORD PTR [rip+0x123] # ab <data>"),
            shape("vmovdqa64 zmm0,ZMMWORD PTR [rip+0x999] # cd <data>")
        );
        assert_ne!(
            shape("mov rax,QWORD PTR [rsp+0x10]"),
            shape("mov rax,QWORD PTR [rsp+0x20]")
        );
        assert_ne!(shape("add rax,0x10"), shape("add rax,0x11"));
        assert_ne!(shape("jne 123 <branch>"), shape("jne 124 <branch>"));
    }
    #[test]
    fn read_width_and_targets_preserve_operand_meaning() {
        assert_eq!(width("vpbroadcastq zmm0,QWORD PTR [rip+0x123]"), Some(8));
        assert_eq!(width("vmovdqu64 zmm0,ZMMWORD PTR [rip+0x123]"), Some(64));
        assert_eq!(target("mov eax,DWORD PTR [rip+0x123] # abc <data>"), 0xabc);
    }
}
