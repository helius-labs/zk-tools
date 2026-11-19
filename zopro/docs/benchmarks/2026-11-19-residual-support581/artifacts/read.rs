//! Compute whole-pack support for a leave-one-out public row reference.
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Default)]
struct Census {
    active: Vec<bool>,
    rows: usize,
}

impl Census {
    fn push(&mut self, row: usize, live: bool) {
        if row % 4 == 0 {
            self.active.push(false);
        }
        self.rows += usize::from(live);
        if live {
            let index = self.active.len() - 1;
            self.active[index] = true;
        }
    }

    fn finish(mut self, fixture: usize, mode: &str, rows: usize) {
        let packs = rows.next_power_of_two() / 4;
        self.active.resize(packs, false);
        let initial = self.active.iter().filter(|v| **v).count();
        let width = packs / 32;
        let mut masks = std::collections::BTreeMap::<u32, usize>::new();
        for column in 0..width {
            let mut mask = 0;
            for stripe in 0..32 {
                mask |= u32::from(self.active[stripe * width + column]) << stripe;
            }
            *masks.entry(mask).or_default() += 1;
        }
        let top: usize = masks.values().copied().max().unwrap();
        println!("rows {fixture} {mode} {rows} {} {initial} {packs} {} {width} {top}", self.rows, masks.len());
        for level in 0..5 {
            let gap = packs >> (level + 1);
            let mut uses = [0usize; 3];
            for block in self.active.chunks_exact_mut(2 * gap) {
                let (left, right) = block.split_at_mut(gap);
                for (lo, hi) in left.iter_mut().zip(right) {
                    let count = usize::from(*lo) + usize::from(*hi);
                    uses[count] += 1;
                    *lo = count > 0;
                    *hi = *lo;
                }
            }
            assert_eq!(uses.iter().sum::<usize>(), packs / 2);
            println!("level {fixture} {mode} {level} {} {} {}", uses[0], uses[1], uses[2]);
        }
    }
}

fn main() {
    let mut readers: Vec<_> = std::env::args_os().skip(1)
        .map(|path| BufReader::new(File::open(path).unwrap())).collect();
    assert_eq!(readers.len(), 4);
    let mut counts = Vec::new();
    for stream in &mut readers {
        let mut header = [0;8]; stream.read_exact(&mut header).unwrap();
        counts.push(usize::try_from(u64::from_le_bytes(header)).unwrap());
    }
    let n = counts[0];
    assert!(counts.iter().all(|v| *v==n) && n >= 128);
    let mut ordinary: [Census; 4] = Default::default();
    let mut centered: [Census; 4] = Default::default();
    let zero = [0u8;64];
    let mut row = [[0u8;64];4];
    for i in 0..n {
        for (input, stream) in row.iter_mut().zip(&mut readers) {
            stream.read_exact(input).unwrap();
        }
        for k in 0..4 {
            let a=&row[(k+1)%4]; let b=&row[(k+2)%4]; let c=&row[(k+3)%4];
            let reference = if a==b || a==c {a} else if b==c {b} else {&zero};
            ordinary[k].push(i,row[k]!=zero);
            centered[k].push(i,row[k]!=*reference);
        }
    }
    for reader in &mut readers { assert_eq!(reader.read(&mut [0]).unwrap(),0); }
    for (k,(ordinary,centered)) in ordinary.into_iter().zip(centered).enumerate() {
        ordinary.finish(k,"ordinary",n); centered.finish(k,"centered",n);
    }
}
