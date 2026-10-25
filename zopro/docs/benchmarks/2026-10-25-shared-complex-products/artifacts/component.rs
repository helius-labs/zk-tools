use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma8::Ifma8, Fq, FqConfig};

type V = Ifma8<FqConfig, true>;
type Complex = [V; 2];

#[inline(always)]
fn ordinary(a: &Complex, b: &Complex) -> Complex {
    [a[0].difference2(&b[0], &a[1], &b[1]), a[0].sum2(&b[1], &a[1], &b[0])]
}

#[inline(always)]
fn multiply<const SHARED: bool>(a: &Complex, b: &Complex) -> Complex {
    #[cfg(shared_complex)]
    if SHARED {
        return V::complex_product_shared(&a[0], &a[1], &b[0], &b[1]);
    }
    ordinary(a, b)
}

fn vector(seed: u64) -> V {
    let mut state = black_box(seed);
    V::new(&core::array::from_fn(|_| {
        let mut bytes = [0u8; 64];
        for chunk in bytes.chunks_mut(8) {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            chunk.copy_from_slice(&state.to_le_bytes());
        }
        Fq::from_random_bytes(&bytes)
    }))
}

#[no_mangle]
#[inline(never)]
pub fn complex_old_codegen(a: &Complex, b: &Complex) -> Complex { ordinary(a, b) }

#[cfg(shared_complex)]
#[no_mangle]
#[inline(never)]
pub fn complex_shared_codegen(a: &Complex, b: &Complex) -> Complex {
    V::complex_product_shared(&a[0], &a[1], &b[0], &b[1])
}

#[inline(never)]
fn run<const SHARED: bool>(pattern: &str, iterations: usize) -> (u128, Vec<V>) {
    let b = [vector(701), vector(907)];
    let factor = vector(211);
    let ax = [vector(307), vector(401)];
    let ay = [vector(503), vector(601)];
    let mut z = [vector(101), vector(103)];
    let mut streams = [
        [vector(1001), vector(1003)], [vector(1101), vector(1103)],
        [vector(1201), vector(1203)], [vector(1301), vector(1303)],
    ];
    let started = Instant::now();
    match pattern {
        "chain" => for _ in 0..iterations { z = multiply::<SHARED>(&z, &b); },
        "streams4" => for _ in 0..iterations {
            for stream in &mut streams { *stream = multiply::<SHARED>(stream, &b); }
        },
        "scaled" => for _ in 0..iterations {
            z = multiply::<SHARED>(&z, &b);
            z = [z[0].mul(&factor), z[1].mul(&factor)];
        },
        "affine-expression" => for _ in 0..iterations {
            let mut slope = multiply::<SHARED>(&z, &b);
            slope = [slope[0].mul(&factor), slope[1].mul(&factor)];
            let cross = slope[0].mul(&slope[1]);
            let square = [slope[0].add(&slope[1]).mul(&slope[0].sub(&slope[1])), cross.add(&cross)];
            let dx = [ax[0].sub(&square[0]), ax[1].sub(&square[1])];
            z = multiply::<SHARED>(&slope, &dx);
            z = [z[0].sub(&ay[0]), z[1].sub(&ay[1])];
        },
        _ => unreachable!(),
    }
    black_box((&z, &streams));
    let elapsed = started.elapsed().as_nanos();
    let result = if pattern == "streams4" { streams.into_iter().flatten().collect() } else { z.to_vec() };
    (elapsed, black_box(result))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "codegen" {
        let a = [vector(101), vector(103)];
        let b = [vector(701), vector(907)];
        black_box(complex_old_codegen(&a, &b));
        #[cfg(shared_complex)]
        black_box(complex_shared_codegen(&a, &b));
        return Ok(());
    }
    if args.len() != 4 { return Err("usage: component old|shared PATTERN ITERATIONS".into()); }
    let shared = match args[1].as_str() {
        "old" => false, "shared" if cfg!(shared_complex) => true,
        _ => return Err("unsupported mode".into()),
    };
    let pattern = args[2].as_str();
    if !["chain", "streams4", "scaled", "affine-expression"].contains(&pattern) {
        return Err("unsupported pattern".into());
    }
    let iterations: usize = args[3].parse()?;
    if iterations == 0 { return Err("iterations must be positive".into()); }
    for _ in 0..3 {
        black_box(if shared { run::<true>(pattern, 2000) } else { run::<false>(pattern, 2000) });
    }
    let (elapsed_ns, result) = if shared { run::<true>(pattern, iterations) } else { run::<false>(pattern, iterations) };
    let words: Vec<u64> = result.into_iter().flat_map(|v| v.get()).flat_map(|v| v.to_canonical()).collect();
    let streams = if pattern == "streams4" { 4. } else { 1. };
    println!("ROW {{\"pattern\":\"{pattern}\",\"shared\":{shared},\"iterations\":{iterations},\"primes\":3,\"elapsed_ns\":{elapsed_ns},\"ns_per_lane\":{},\"output_words\":{words:?}}}", elapsed_ns as f64 / (iterations as f64 * 8. * streams));
    Ok(())
}
