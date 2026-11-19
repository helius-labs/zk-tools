use std::hint::black_box;
use std::time::Instant;
use zopro_bn254::{Fr, FrConfig, ifma8::Ifma8};

#[inline(never)]
fn generic(v: &mut [Fr]) {
    for w in v {
        *w = Fr::from_montgomery(w.to_canonical());
    }
}

#[inline(never)]
fn reduced(v: &mut [Fr]) {
    for w in v {
        *w = Fr::from_montgomery(w.to_canonical_reduced());
    }
}

#[inline(never)]
fn vector(v: &mut [Fr]) {
    type Batch = Ifma8<FrConfig, true>;
    let factor = Batch::new(&[Fr::from_montgomery([1, 0, 0, 0]); 8]);
    let (batches, rest) = v.as_chunks_mut::<8>();
    for batch in batches {
        *batch = Batch::new(batch).mul(&factor).get();
    }
    reduced(rest);
}

fn main() {
    type Convert = fn(&mut [Fr]);
    let mut state = 0x553f2ba0221c758du64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let input: Vec<_> = (0..65536).map(|i| match i % 8 {
        0 => Fr::ONE,
        1 => Fr::ZERO,
        2 => -Fr::ONE,
        _ => Fr::from_random_bytes(&core::array::from_fn(|_| next() as u8)),
    }).collect();
    let mut reference = input.clone();
    generic(&mut reference);
    let variants: [(&str,Convert);3] = [
        ("generic", generic), ("reduced", reduced), ("vector", vector),
    ];
    let mut buffer = input.clone();
    for block in 0..12 {
        for offset in 0..variants.len() {
            let index = if block%2==0 { (offset + block/2)%3 } else { (2-offset + block/2)%3 };
            let (mode, convert) = variants[index];
            let mut elapsed=0u128;
            for _ in 0..16 {
                buffer.copy_from_slice(&input);
                let now=Instant::now();
                convert(black_box(&mut buffer));
                elapsed+=now.elapsed().as_nanos();
                assert_eq!(black_box(&buffer),&reference);
            }
            println!("{block} {mode} {:.5}", elapsed as f64 / (16*input.len()) as f64);
        }
    }
}
