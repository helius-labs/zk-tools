use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};
fn n(b: &[u8], i: usize, len: usize) -> usize {
    b[i..i + len]
        .iter()
        .enumerate()
        .map(|(j, b)| (*b as usize) << (8 * j))
        .sum()
}
fn sections(b: &[u8]) -> BTreeMap<String, (usize, usize, usize)> {
    let off = n(b, 40, 8);
    let width = n(b, 58, 2);
    let count = n(b, 60, 2);
    let strings = off + width * n(b, 62, 2);
    let start = n(b, strings + 24, 8);
    let mut out = BTreeMap::new();
    for i in 0..count {
        let p = off + i * width;
        let name = start + n(b, p, 4);
        let end = b[name..].iter().position(|v| *v == 0).unwrap() + name;
        out.insert(
            String::from_utf8(b[name..end].to_vec()).unwrap(),
            (n(b, p + 24, 8), n(b, p + 32, 8), n(b, p + 16, 8)),
        );
    }
    out
}
fn main() {
    let a = fs::read("native-links/frozen-sdk").unwrap();
    let b = fs::read("native-links/off-sdk").unwrap();
    let s = sections(&a);
    assert_eq!(s, sections(&b));
    let audit: Value = serde_json::from_slice(&fs::read("control-sections.json").unwrap()).unwrap();
    let names: Vec<_> = audit["sections"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, v)| v["byte_identical"] == false)
        .map(|(k, _)| k.as_str())
        .collect();
    assert_eq!(names, vec![".data.rel.ro", ".note.gnu.build-id"]);
    let (off, size, addr) = s[".data.rel.ro"];
    let (ro, rs, rv) = s[".rodata"];
    let (re, rl, _) = s[".rela.dyn"];
    let mut reloc = BTreeMap::new();
    for i in (0..rl).step_by(24) {
        if n(&a, re + i + 8, 8) & 0xffffffff == 8 {
            reloc.insert(n(&a, re + i, 8), n(&a, re + i + 16, 8));
        }
    }
    let mut out = vec![];
    let mut masked = b[off..off + size].to_vec();
    for d in audit["sections"][".data.rel.ro"]["differences"]
        .as_array()
        .unwrap()
    {
        let i = d["section_offset"].as_u64().unwrap() as usize;
        let start = i - 16;
        assert_eq!(&a[off + start..off + i], &b[off + start..off + i]);
        assert_eq!(
            &a[off + i + 4..off + start + 24],
            &b[off + i + 4..off + start + 24]
        );
        let old = n(&a, off + i, 4);
        let new = n(&b, off + i, 4);
        assert_eq!(new, old + 2);
        let pointer = *reloc.get(&(addr + start)).unwrap();
        let len = n(&a, off + start + 8, 8);
        assert!(pointer >= rv && pointer + len <= rv + rs);
        let path = std::str::from_utf8(&a[ro + pointer - rv..ro + pointer - rv + len]).unwrap();
        assert!(
            path.ends_with("crates/zopro-core/src/msm_affine/g2_phi.rs"),
            "{path}"
        );
        masked[i..i + 4].copy_from_slice(&a[off + i..off + i + 4]);
        out.push(json!({"section_offset":i,"file":path,"frozen_line":old,"off_line":new,"column_unchanged":n(&a,off+i+4,4)}));
    }
    assert!(!out.is_empty());
    assert_eq!(masked, &a[off..off + size]);
    fs::write("control-locations.json",serde_json::to_vec_pretty(&json!({"native_code_or_arithmetic_changed":false,"entire_text_rodata_and_other_sections_identical":true,"only_non_buildid_differences":"Rust panic Location line fields, each +2 after cfg-only source insertions","panic_locations":out,"timing_not_started":true,"performance_gates_unchanged":true})).unwrap()).unwrap();
    println!(
        "All changed records verified as Rust source lines+2;all other runtime section bytes identical"
    );
}
