use serde_json::json;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
fn n(b: &[u8], i: usize, len: usize) -> usize {
    b[i..i + len]
        .iter()
        .enumerate()
        .map(|(j, b)| (*b as usize) << (8 * j))
        .sum()
}
fn sections(b: &[u8]) -> BTreeMap<String, (usize, usize)> {
    assert_eq!(&b[..6], b"\x7fELF\x02\x01");
    let off = n(b, 40, 8);
    let width = n(b, 58, 2);
    let count = n(b, 60, 2);
    let strings = off + width * n(b, 62, 2);
    let start = n(b, strings + 24, 8);
    let mut out = BTreeMap::new();
    for i in 0..count {
        let p = off + i * width;
        if n(b, p + 4, 4) == 8 {
            continue;
        }
        let name = start + n(b, p, 4);
        let end = b[name..].iter().position(|v| *v == 0).unwrap() + name;
        out.insert(
            String::from_utf8(b[name..end].to_vec()).unwrap(),
            (n(b, p + 24, 8), n(b, p + 32, 8)),
        );
    }
    out
}
fn main() {
    let a = fs::read("native/frozen-sdk").unwrap();
    let b = fs::read("native/off-sdk").unwrap();
    let sa = sections(&a);
    let sb = sections(&b);
    assert_eq!(sa.keys().collect::<Vec<_>>(), sb.keys().collect::<Vec<_>>());
    let mut out = BTreeMap::new();
    for (name, (pa, na)) in sa {
        let (pb, nb) = sb[&name];
        let x = &a[pa..pa + na];
        let y = &b[pb..pb + nb];
        let mut diff = vec![];
        if na == nb {
            for (i, (x, y)) in x.iter().zip(y).enumerate() {
                if x != y {
                    diff.push(json!({"section_offset":i,"frozen":x,"off":y}));
                }
            }
        }
        let same = x == y;
        out.insert(name.clone(),json!({"byte_identical":same,"size_frozen":na,"size_off":nb,"sha256_frozen":format!("{:x}",Sha256::digest(x)),"sha256_off":format!("{:x}",Sha256::digest(y)),"differences":diff}));
    }
    for name in [".text", ".rodata"] {
        assert_eq!(out[name]["byte_identical"], true, "{name}");
    }
    fs::write("control-sections.json",serde_json::to_vec_pretty(&json!({"all_text_and_rodata_byte_identical":true,"timing_not_started":true,"sections":out})).unwrap()).unwrap();
    println!("Full .text/.rodata byteidentical; all section differences recorded");
}
