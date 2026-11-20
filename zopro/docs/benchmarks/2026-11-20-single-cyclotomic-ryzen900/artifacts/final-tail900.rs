use std::{hint::black_box, time::Instant};
use zopro_bn254::{
    pairing::{final_exponentiation, multi_miller_loop, G2Prepared},
    Fq12, Fr, G1Affine, G2Affine,
};

// Includes outputs of the same Miller recurrence used by proof verification,
// with distinct nonidentity G1/G2 factors for each argument.
fn inputs() -> Vec<Fq12> {
    (1..25)
        .map(|i| {
            let points: Vec<_> = (1..=3)
                .map(|j| {
                    let p = G1Affine::GENERATOR
                        .mul(&Fr::from_u64(i * 7 + j * 19))
                        .to_affine();
                    let q = G2Affine::GENERATOR
                        .mul(&Fr::from_u64(i * 29 + j * 11))
                        .to_affine();
                    (p, G2Prepared::new(&q))
                })
                .collect();
            multi_miller_loop(&[
                (points[0].0, &points[0].1),
                (points[1].0, &points[1].1),
                (points[2].0, &points[2].1),
            ])
        })
        .collect()
}

fn main() {
    let values = inputs();
    let outputs: Vec<_> = values
        .iter()
        .map(|&x| final_exponentiation(x).unwrap())
        .collect();
    for x in &outputs {
        for a in [x.c0, x.c1] {
            for b in [a.c0, a.c1, a.c2] {
                for c in [b.c0, b.c1] {
                    for byte in c.to_be_bytes() {
                        print!("{byte:02x}");
                    }
                }
            }
        }
    }
    println!();
    let mut elapsed = Vec::new();
    for index in 0..240 {
        let index_in_values = index % values.len();
        let input = black_box(values[index_in_values]);
        let started = Instant::now();
        let output = black_box(final_exponentiation(input)).unwrap();
        let ns = started.elapsed().as_nanos();
        assert!(ns > 0 && output == outputs[index_in_values]);
        if index >= 48 {
            elapsed.push(ns);
        }
    }
    for elapsed in elapsed {
        println!("{elapsed}");
    }
}
