//! Count matched residual input pairs for a fixed reference and alternative choices.
use std::fs::File;
use std::io::{BufReader, Read};

const POLICIES: usize = 6;
const SAMPLES: usize = 64;
type Pair = [u8; 64];

#[derive(Default)]
struct Counts {
    total: [usize; POLICIES],
    sample: [usize; POLICIES],
    block: [usize; POLICIES],
    block_best: usize,
    row_best: usize,
}

fn references<'a>(a: &'a Pair, b: &'a Pair, c: &'a Pair, zero: &'a Pair) -> [&'a Pair; POLICIES] {
    let majority = if a == b || a == c { Some(a) } else if b == c { Some(b) } else { None };
    [majority.unwrap_or(zero), a, b, c, zero, majority.unwrap_or(a)]
}

fn best(counts: &[usize; POLICIES]) -> usize {
    (1..POLICIES).fold(0, |best, i| if counts[i] > counts[best] { i } else { best })
}

fn main() {
    let mut files: Vec<_> = std::env::args_os().skip(1)
        .map(|p| BufReader::new(File::open(p).unwrap())).collect();
    assert_eq!(files.len(), 4);
    let mut n = None;
    for f in &mut files {
        let mut header = [0; 8];
        f.read_exact(&mut header).unwrap();
        let count = usize::try_from(u64::from_le_bytes(header)).unwrap();
        assert_eq!(*n.get_or_insert(count), count);
    }
    let n = n.unwrap();
    assert!(n >= SAMPLES);
    let positions: Vec<_> = (0..SAMPLES).map(|j| (2*j+1)*n/(2*SAMPLES)).collect();
    let zero = [0;64];
    let mut rows = [[0;64];4];
    let mut counts: [Counts;4] = Default::default();
    let mut next_sample = 0;
    for i in 0..n {
        for (r, f) in rows.iter_mut().zip(files.iter_mut()) { f.read_exact(r).unwrap(); }
        let sampled = positions.get(next_sample) == Some(&i);
        next_sample += usize::from(sampled);
        for (j, dst) in counts.iter_mut().enumerate() {
            let options = references(&rows[(j+1)%4], &rows[(j+2)%4], &rows[(j+3)%4], &zero);
            let mut any = false;
            for (p, value) in options.iter().enumerate() {
                if rows[j] == **value {
                    any = true;
                    dst.total[p] += 1;
                    dst.block[p] += 1;
                    dst.sample[p] += usize::from(sampled);
                }
            }
            dst.row_best += usize::from(any);
            if (i+1)%256 == 0 || i+1 == n {
                dst.block_best += dst.block[best(&dst.block)];
                dst.block = [0; POLICIES];
            }
        }
    }
    assert_eq!(next_sample, SAMPLES);
    for file in &mut files { assert_eq!(file.read(&mut [0]).unwrap(), 0); }
    for (j, c) in counts.iter().enumerate() {
        let sample_choice = best(&c.sample);
        let whole = c.total[best(&c.total)];
        assert!(whole <= c.block_best && c.block_best <= c.row_best && c.row_best <= n);
        print!("{j} {n} {sample_choice} {} {whole} {} {}", c.total[sample_choice], c.block_best, c.row_best);
        for val in c.total { print!(" {val}"); }
        println!();
    }
}

#[test]
fn fallback_and_tie_order() {
    let z = [0; 64]; let a = [1; 64]; let b = [2; 64]; let c = [3; 64];
    assert_eq!(references(&a,&b,&c,&z), [&z,&a,&b,&c,&z,&a]);
    assert_eq!(references(&a,&b,&b,&z), [&b,&a,&b,&b,&z,&b]);
    assert_eq!(best(&[2,2,1,0,0,0]), 0);
}
