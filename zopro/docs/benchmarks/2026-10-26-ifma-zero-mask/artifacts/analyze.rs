use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn u16at(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes(b[p..p + 2].try_into().unwrap())
}
fn u64at(b: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(b[p..p + 8].try_into().unwrap())
}
fn slice(b: &[u8], address: u64, len: usize) -> &[u8] {
    assert_eq!(&b[..7], b"\x7fELF\x02\x01\x01");
    assert_eq!(u16at(b, 18), 62);
    let off = u64at(b, 32) as usize;
    let size = u16at(b, 54) as usize;
    for i in 0..u16at(b, 56) as usize {
        let p = off + i * size;
        if u32::from_le_bytes(b[p..p + 4].try_into().unwrap()) != 1 {
            continue;
        }
        let start = u64at(b, p + 16);
        if let Some(delta) = address.checked_sub(start) {
            if delta + len as u64 <= u64at(b, p + 32) {
                let at = (u64at(b, p + 8) + delta) as usize;
                return &b[at..at + len];
            }
        }
    }
    panic!("ELFaddressnotfilebacked:{address:x}")
}
fn normalize(a: &str, b: &[u8]) -> String {
    let text = a.split('#').next().unwrap().trim();
    if let Some(at) = text.find("(%rip)") {
        let target = a
            .split('#')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap();
        let target = u64::from_str_radix(target, 16).unwrap();
        let size = if text.contains("{1to8}")
            || text.contains("{1to4}")
            || text.starts_with("vpbroadcastq")
            || text.starts_with("vbroadcastsd")
        {
            8
        } else if text.contains("{1to16}")
            || text.starts_with("vpbroadcastd")
            || text.starts_with("vbroadcastss")
        {
            4
        } else if text.contains("%zmm") {
            64
        } else if text.contains("%ymm") {
            32
        } else if text.contains("%xmm") {
            16
        } else {
            8
        };
        let value = slice(b, target, size)
            .iter()
            .map(|v| format!("{v:02x}"))
            .collect::<String>();
        let start = text[..at]
            .rfind(|c: char| c.is_whitespace() || c == ',')
            .map(|i| i + 1)
            .unwrap_or(0);
        format!("{}<data{size}:{value}>{}", &text[..start], &text[at..])
    } else {
        text.into()
    }
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    assert_eq!(read(root.join("build/result.json"))["status"], "complete");
    for (p, h) in read(root.join("build-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut groups = Vec::new();
    let mut metadata = Vec::new();
    let mut sequences = BTreeMap::new();
    let mut asm = String::new();
    let wrappers = read(root.join("wrappers.json"));
    let field_names = wrappers
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["name"].as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let mut isa_names = Vec::new();
    for n in [4, 8] {
        for name in [
            format!("zero_raw{n}"),
            format!("zero_mask52_{n}"),
            format!("zero_feed52_{n}"),
            format!("zero_feedraw{n}"),
            format!("zero_mask48_{n}"),
        ] {
            isa_names.push(name);
        }
    }
    for (label, path, names) in [
        ("accepted", root.join("accepted-probe"), &field_names),
        ("candidate", root.join("candidate-probe"), &field_names),
        ("isa", root.join("isa"), &isa_names),
    ] {
        let b = fs::read(&path).unwrap();
        let digest = hash(&path);
        let out = Command::new("nm")
            .args(["-S", "--defined-only"])
            .arg(&path)
            .output()
            .unwrap();
        assert!(out.status.success());
        let nm = String::from_utf8(out.stdout).unwrap();
        fs::write(root.join(format!("{label}-symbols.txt")), &nm).unwrap();
        let mut selected = 0;
        for line in nm.lines() {
            let f = line.split_whitespace().collect::<Vec<_>>();
            if f.len() != 4 || !names.iter().any(|n| n == f[3]) {
                continue;
            }
            let name = f[3];
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            assert!(size > 0);
            let args = vec![
                "-d".to_owned(),
                "-w".into(),
                "--insn-width=16".into(),
                format!("--start-address=0x{start:x}"),
                format!("--stop-address=0x{:x}", start + size),
                path.to_str().unwrap().into(),
            ];
            let out = Command::new("objdump").args(&args).output().unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            let mut inst = Vec::new();
            let mut cursor = start;
            for line in text.lines() {
                if let Some((address, tail)) = line.split_once(':') {
                    let address = address.trim();
                    if address.is_empty() || !address.chars().all(|c| c.is_ascii_hexdigit()) {
                        continue;
                    }
                    let fields = tail
                        .split('\t')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<_>>();
                    assert_eq!(fields.len(), 2);
                    let ip = u64::from_str_radix(address, 16).unwrap();
                    assert_eq!(ip, cursor);
                    let raw = fields[0]
                        .split_whitespace()
                        .map(|w| u8::from_str_radix(w, 16).unwrap())
                        .collect::<Vec<_>>();
                    assert_eq!(slice(&b, ip, raw.len()), raw);
                    cursor += raw.len() as u64;
                    inst.push(fields[1].to_owned());
                }
            }
            assert_eq!(cursor, start + size);
            let normalized = inst.iter().map(|s| normalize(s, &b)).collect::<Vec<_>>();
            let ns = normalized.join("\n");
            let mut subs = Vec::new();
            for s in &inst {
                if let Some(v) = s
                    .strip_prefix("sub")
                    .and_then(|s| s.trim().strip_prefix("$0x"))
                    .and_then(|s| s.split_once(','))
                {
                    if v.1.trim() == "%rsp" {
                        subs.push(u64::from_str_radix(v.0, 16).unwrap());
                    }
                }
            }
            metadata.push(json!({"label":label,"symbol":name,"binary_sha256":digest,"start":start,"size_bytes":size,"instructions":inst.len(),"and_sites":inst.iter().filter(|s|s.starts_with("vpand")).count(),"ifma_sites":inst.iter().filter(|s|s.starts_with("vpmadd52")).count(),"calls":inst.iter().filter(|s|s.starts_with("call")).count(),"rsp_operand_lines":inst.iter().filter(|s|s.contains("(%rsp)")).count(),"literal_stack_subtractions":subs,"raw_code_sha256":format!("{:x}",Sha256::digest(slice(&b,start,size as usize))),"normalized_instruction_sha256":format!("{:x}",Sha256::digest(ns.as_bytes())),"objdump_args":args}));
            assert!(sequences
                .insert(format!("{label}:{name}"), normalized)
                .is_none());
            asm.push_str(&format!("\n{label}:{name}\n{text}"));
            selected += 1;
        }
        assert_eq!(selected, names.len());
        groups.push(json!({"label":label,"binary_sha256":digest,"functions":selected}));
    }
    let mut pairs = Vec::new();
    let mut same = 0;
    let mut and_saved = 0isize;
    for name in &field_names {
        let a = metadata
            .iter()
            .find(|v| v["label"] == "accepted" && v["symbol"] == *name)
            .unwrap();
        let c = metadata
            .iter()
            .find(|v| v["label"] == "candidate" && v["symbol"] == *name)
            .unwrap();
        let eq = sequences[&format!("accepted:{name}")] == sequences[&format!("candidate:{name}")];
        same += usize::from(eq);
        let saving =
            a["and_sites"].as_u64().unwrap() as isize - c["and_sites"].as_u64().unwrap() as isize;
        and_saved += saving;
        pairs.push(json!({"symbol":name,"normalized_equal":eq,"accepted_bytes":a["size_bytes"],"candidate_bytes":c["size_bytes"],"accepted_and_sites":a["and_sites"],"candidate_and_sites":c["and_sites"],"and_saved":saving,"accepted_ifma":a["ifma_sites"],"candidate_ifma":c["ifma_sites"],"accepted_stack":a["literal_stack_subtractions"],"candidate_stack":c["literal_stack_subtractions"]}));
    }
    let mut toy = Vec::new();
    for n in [4, 8] {
        toy.push(json!({"lanes":n,"feed_mask52_vs_raw_equal":sequences[&format!("isa:zero_feed52_{n}")]==sequences[&format!("isa:zero_feedraw{n}")],"return_mask52_vs_raw_equal":sequences[&format!("isa:zero_mask52_{n}")]==sequences[&format!("isa:zero_raw{n}")]}));
    }
    fs::write(root.join("codegen.txt"), asm).unwrap();
    fs::write(root.join("codegen.json"),serde_json::to_vec_pretty(&json!({"binaries":groups,"functions":metadata,"pairs":pairs,"isa_pairs":toy,"field_functions_normalized_equal":same,"field_functions":field_names.len(),"total_and_sites_saved":and_saved,"normalization":"EveryinstructionbyteboundtoELF, exactfunctionextent. Removeaddress/comments; RIPdisplacement replacedbyactualreferencedconstantbytes8/16/32/64. Otherregisters/opcodes/immediates retained; noperformanceclaimfromstaticcounts."})).unwrap()).unwrap();
    println!(
        "FIELD equivalent{same}/24; summed ANDsite saving{and_saved}; no timing/proof collected"
    );
}
