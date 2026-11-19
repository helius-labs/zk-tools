use std::{fs, time::Instant};
use zopro_bn254::{Fr, G2Affine};
use zopro_core::coalesce_query_probe::b;

fn words(bytes: &[u8]) -> [u64; 4] {
    std::array::from_fn(|i| u64::from_le_bytes(bytes[8*i..8*i+8].try_into().unwrap()))
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let points = zopro_core::decode_g2(&fs::read(&args[1]).unwrap());
    let input: Vec<_> = fs::read(&args[2]).unwrap().chunks_exact(32).map(words).collect();
    let offsets: Vec<_> = fs::read(&args[3]).unwrap().chunks_exact(36)
        .map(|r| (u32::from_le_bytes(r[..4].try_into().unwrap()) as usize, words(&r[4..]))).collect();
    assert_eq!(input.len(), points.len()); assert!(offsets.windows(2).all(|w|w[0].0<w[1].0));
    let enabled = args[4]=="residual";
    let setup = Instant::now();
    let bases: Vec<G2Affine> = offsets.iter().map(|(i,_)| points[*i]).collect();
    let mut values: Vec<_> = offsets.iter().map(|(_,k)| *k).collect();
    let correction = b(&bases, &mut values).0;
    drop(values); drop(bases);
    println!("SETUP {}", setup.elapsed().as_nanos());
    let expected = b(&points, &mut input.clone()).0.to_affine();
    let mut working = input.clone();
    for round in 0..24 {
        working.copy_from_slice(&input);
        let start = Instant::now();
        if enabled {
            for &(index, offset) in &offsets {
                let current = working[index];
                working[index] = if current==offset { [0;4] } else {
                    (Fr::from_canonical(current).unwrap()-Fr::from_canonical(offset).unwrap()).to_canonical()
                };
            }
        }
        let mut result = b(&points, &mut working).0;
        if enabled {result += correction;}
        let elapsed = start.elapsed().as_nanos();
        assert_eq!(result.to_affine(),expected);
        if round>=3 { println!("SAMPLE {}", elapsed); }
    }
}
