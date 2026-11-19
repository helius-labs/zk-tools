// Encodes the most frequent public reference pairs with bounded training memory.
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

type Pair = [u8; 64];
const ZERO: Pair = [0; 64];

fn rows(path: &Path, mut visit: impl FnMut(Pair)) -> u64 {
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut header = [0; 8];
    reader.read_exact(&mut header).unwrap();
    let count = u64::from_le_bytes(header);
    assert_eq!(reader.get_ref().metadata().unwrap().len(), 8 + count * 64);
    let mut pair = ZERO;
    for _ in 0..count {
        reader.read_exact(&mut pair).unwrap();
        visit(pair);
    }
    count
}

fn encode(input: &Path, dest: &Path, width: u8) {
    assert!(matches!(width, 1 | 4 | 8));
    let mut candidates = BTreeMap::new();
    let count = rows(input, |value| {
        if value == ZERO { return; }
        if let Some(n) = candidates.get_mut(&value) {
            *n += 1u64;
        } else if candidates.len() < 511 {
            candidates.insert(value, 1u64);
        } else {
            candidates.retain(|_, n| { *n -= 1; *n > 0 });
        }
    });
    for n in candidates.values_mut() { *n = 0; }
    rows(input, |value| {
        if let Some(n) = candidates.get_mut(&value) { *n += 1; }
    });
    let mut selected: Vec<_> = candidates.into_iter().collect();
    selected.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    selected.truncate((1usize << width)-1);
    let mut writer = BufWriter::new(File::create(dest).unwrap());
    writer.write_all(b"EUP1").unwrap();
    writer.write_all(&count.to_le_bytes()).unwrap();
    writer.write_all(&[width, selected.len() as u8]).unwrap();
    let mut ids = BTreeMap::new();
    for (i,(v,_)) in selected.iter().enumerate() {
        writer.write_all(v).unwrap();
        ids.insert(*v, (i+1) as u8);
    }
    let mut byte = 0u8;
    let mut shift = 0u8;
    rows(input, |v| {
        byte |= ids.get(&v).copied().unwrap_or(0) << shift;
        shift += width;
        if shift == 8 { writer.write_all(&[byte]).unwrap(); byte = 0; shift = 0; }
    });
    if shift != 0 { writer.write_all(&[byte]).unwrap(); }
    writer.flush().unwrap();
}

fn decode(src: &Path, dest: &Path) {
    let mut input = BufReader::new(File::open(src).unwrap());
    let mut header = [0;14]; input.read_exact(&mut header).unwrap();
    assert_eq!(&header[..4], b"EUP1");
    let count = u64::from_le_bytes(header[4..12].try_into().unwrap());
    let width = header[12]; let entries = header[13];
    assert!(matches!(width, 1 | 4 | 8) && (entries as usize) < 1usize << width);
    let expected = 14 + entries as u64 * 64 + (count * width as u64).div_ceil(8);
    assert_eq!(expected, input.get_ref().metadata().unwrap().len());
    let mut table = vec![ZERO; entries as usize + 1];
    for v in &mut table[1..] { input.read_exact(v).unwrap(); }
    let mut output = BufWriter::new(File::create(dest).unwrap());
    output.write_all(&header[4..12]).unwrap();
    let mut byte = [0u8]; let mut kept = 0;
    for i in 0..count {
        let shift = ((i%8) as u8 * width) % 8;
        if shift == 0 { input.read_exact(&mut byte).unwrap(); }
        let id = (byte[0] >> shift) as usize & ((1usize << width)-1);
        output.write_all(&table[id]).unwrap(); kept += u64::from(id != 0);
    }
    output.flush().unwrap();
    println!("{count} {entries} {kept} {expected}");
}
fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    assert_eq!(args.len(), 4);
    encode(Path::new(&args[0]), Path::new(&args[1]), args[3].to_str().unwrap().parse().unwrap());
    decode(Path::new(&args[1]), Path::new(&args[2]));
}
