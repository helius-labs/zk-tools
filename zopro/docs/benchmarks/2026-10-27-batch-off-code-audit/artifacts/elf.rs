use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
fn number(b: &[u8], p: usize, n: usize) -> u64 {
    let s = b.get(p..p.checked_add(n).unwrap()).unwrap();
    s.iter()
        .enumerate()
        .map(|(i, x)| (*x as u64) << (8 * i))
        .sum()
}
fn hex(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
#[derive(Debug)]
struct Section {
    kind: u64,
    flags: u64,
    address: u64,
    offset: usize,
    size: usize,
}
fn sections(b: &[u8]) -> BTreeMap<String, Section> {
    assert!(b.len() >= 64 && &b[..7] == b"\x7fELF\x02\x01\x01");
    assert_eq!(number(b, 18, 2), 62);
    let offset = number(b, 40, 8) as usize;
    let stride = number(b, 58, 2) as usize;
    let count = number(b, 60, 2) as usize;
    let strings = number(b, 62, 2) as usize;
    assert!(stride >= 64 && count > 0 && count < 65535 && strings < count);
    assert!(
        offset
            .checked_add(stride.checked_mul(count).unwrap())
            .unwrap()
            <= b.len()
    );
    let p = offset + strings * stride;
    let begin = number(b, p + 24, 8) as usize;
    let len = number(b, p + 32, 8) as usize;
    let names = &b[begin..begin.checked_add(len).unwrap()];
    let mut out = BTreeMap::new();
    for i in 1..count {
        let p = offset + i * stride;
        let n = number(b, p, 4) as usize;
        let end = n + names[n..].iter().position(|x| *x == 0).unwrap();
        let name = std::str::from_utf8(&names[n..end]).unwrap().to_owned();
        let s = Section {
            kind: number(b, p + 4, 4),
            flags: number(b, p + 8, 8),
            address: number(b, p + 16, 8),
            offset: number(b, p + 24, 8) as usize,
            size: number(b, p + 32, 8) as usize,
        };
        if s.kind != 8 {
            assert!(s.offset.checked_add(s.size).unwrap() <= b.len());
        }
        assert!(out.insert(name, s).is_none());
    }
    out
}
fn main() {
    let ids: Value = serde_json::from_slice(
        &fs::read("/tmp/zopro-control-build-audit/identities.json").unwrap(),
    )
    .unwrap();
    let ap = ids["variants"]["accepted"]["binary"].as_str().unwrap();
    let bp = ids["variants"]["fresh"]["binary"].as_str().unwrap();
    let a = fs::read(ap).unwrap();
    let b = fs::read(bp).unwrap();
    let sa = sections(&a);
    let sb = sections(&b);
    assert_eq!(sa.keys().collect::<Vec<_>>(), sb.keys().collect::<Vec<_>>());
    let mut rows = Vec::new();
    for (name, x) in &sa {
        let y = &sb[name];
        assert_eq!(
            (x.kind, x.flags, x.address, x.offset, x.size),
            (y.kind, y.flags, y.address, y.offset, y.size)
        );
        if x.kind == 8 {
            rows.push(json!({"name":name,"kind":"NOBITS","size":x.size,"file_bytes":false,"layout_equal":true}));
            continue;
        }
        let aa = &a[x.offset..x.offset + x.size];
        let bb = &b[y.offset..y.offset + y.size];
        let diffs: Vec<_> = aa
            .iter()
            .zip(bb)
            .enumerate()
            .filter_map(|(i, (a, b))| (a != b).then_some(i))
            .collect();
        let mut ranges = Vec::new();
        for &i in &diffs {
            if let Some((_, end)) = ranges.last_mut() {
                if *end == i {
                    *end += 1;
                    continue;
                }
            }
            ranges.push((i, i + 1));
        }
        let snippets:Vec<_>=ranges.iter().take(24).map(|&(start,end)|{let begin=start.saturating_sub(24);let stop=(end+64).min(aa.len());json!({"section_offset":start,"file_offset":x.offset+start,"virtual_address":x.address+start as u64,"length":end-start,"accepted_excerpt":String::from_utf8_lossy(&aa[begin..stop]),"fresh_excerpt":String::from_utf8_lossy(&bb[begin..stop])})}).collect();
        rows.push(json!({"name":name,"file_offset":x.offset,"virtual_address":x.address,"size":x.size,"allocated":x.flags&2!=0,"executable":x.flags&4!=0,"layout_equal":true,"bytes_equal":aa==bb,"accepted_sha256":hex(aa),"fresh_sha256":hex(bb),"different_bytes":diffs.len(),"difference_runs":ranges.len(),"first24_difference_runs":snippets}));
        println!(
            "{name}: {} bytes, {} different bytes, {} runs",
            x.size,
            diffs.len(),
            ranges.len()
        );
    }
    fs::write("/tmp/zopro-control-build-audit/sections.json",serde_json::to_vec_pretty(&json!({"accepted":ap,"fresh":bp,"file_size_equal":a.len()==b.len(),"section_layout_equal":true,"rows":rows,"static_only":true,"new_proofs":0})).unwrap()).unwrap();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn x86_64_section_offsets_are_not_virtual_addresses() {
        let mut b = vec![0u8; 512];
        b[..7].copy_from_slice(b"\x7fELF\x02\x01\x01");
        for (p, v, n) in [
            (18, 62u64, 2),
            (40, 64, 8),
            (58, 64, 2),
            (60, 3, 2),
            (62, 1, 2),
            (128, 1, 4),
            (132, 3, 4),
            (152, 256, 8),
            (160, 20, 8),
            (192, 8, 4),
            (196, 1, 4),
            (200, 6, 8),
            (208, 0x1234, 8),
            (216, 320, 8),
            (224, 4, 8),
        ] {
            b[p..p + n].copy_from_slice(&v.to_le_bytes()[..n]);
        }
        let names = b"\0.shstr\0.text\0";
        b[256..256 + names.len()].copy_from_slice(names);
        b[320..324].copy_from_slice(&[1, 2, 3, 4]);
        let s = sections(&b);
        assert_eq!(s[".text"].address, 0x1234);
        assert_eq!(s[".text"].offset, 320);
        assert_eq!(
            &b[s[".text"].offset..s[".text"].offset + s[".text"].size],
            &[1, 2, 3, 4]
        );
    }
}
