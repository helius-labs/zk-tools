use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-production";
fn number(b: &[u8], p: usize, n: usize) -> usize {
    b[p..p + n]
        .iter()
        .enumerate()
        .map(|(i, v)| (*v as usize) << (i * 8))
        .sum()
}
fn sections(b: &[u8]) -> BTreeMap<String, (usize, usize, usize)> {
    assert_eq!(&b[..6], b"\x7fELF\x02\x01");
    let start = number(b, 40, 8);
    let size = number(b, 58, 2);
    let count = number(b, 60, 2);
    let strings = number(b, start + size * number(b, 62, 2) + 24, 8);
    let mut out = BTreeMap::new();
    for i in 0..count {
        let p = start + size * i;
        if number(b, p + 4, 4) == 8 {
            continue;
        }
        let s = strings + number(b, p, 4);
        let e = s + b[s..].iter().position(|&b| b == 0).unwrap();
        out.insert(
            String::from_utf8(b[s..e].to_vec()).unwrap(),
            (
                number(b, p + 24, 8),
                number(b, p + 32, 8),
                number(b, p + 8, 8),
            ),
        );
    }
    out
}
fn main() {
    let mut pairs = BTreeMap::new();
    for (label, old, new) in [
        (
            "measured-to-clean",
            "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk/borrow-sdk",
            format!("{R}/native/candidate-sdk"),
        ),
        (
            "accepted-to-control",
            "/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk",
            format!("{R}/native/control-sdk"),
        ),
    ] {
        let a = fs::read(old).unwrap();
        let b = fs::read(&new).unwrap();
        let sa = sections(&a);
        let sb = sections(&b);
        let mut records = BTreeMap::new();
        for (name, (p, n, flags)) in sa {
            if let Some(&(q, m, _)) = sb.get(&name) {
                let x = &a[p..p + n];
                let y = &b[q..q + m];
                let mut first = vec![];
                let mut count = 0;
                if n == m {
                    for (i, (x, y)) in x.iter().zip(y).enumerate() {
                        if x != y {
                            count += 1;
                            if first.len() < 32 {
                                first.push(json!({"offset":i,"old":x,"new":y}));
                            }
                        }
                    }
                }
                records.insert(name,json!({"allocated":flags&2!=0,"identical":x==y,"old_bytes":n,"new_bytes":m,"old_sha256":format!("{:x}",Sha256::digest(x)),"new_sha256":format!("{:x}",Sha256::digest(y)),"different_bytes_if_same_size":if n==m{json!(count)}else{json!(null)},"first_differences":first}));
            }
        }
        let text = records[".text"]["identical"].clone();
        let rodata = records[".rodata"]["identical"].clone();
        let changed: Vec<_> = records
            .iter()
            .filter(|(_, v)| v["allocated"] == true && v["identical"] == false)
            .map(|(k, _)| k.clone())
            .collect();
        println!("{label} text{text} rodata{rodata} allocated_differences{changed:?}");
        pairs.insert(label,json!({"old":old,"new":new,"sections":records,"text_identical":text,"rodata_identical":rodata,"changed_allocated_sections":changed,"old_sha256":format!("{:x}",Sha256::digest(&a)),"new_sha256":format!("{:x}",Sha256::digest(&b))}));
    }
    fs::write(
        format!("{R}/code-audit.json"),
        serde_json::to_vec_pretty(&json!(pairs)).unwrap(),
    )
    .unwrap();
}
