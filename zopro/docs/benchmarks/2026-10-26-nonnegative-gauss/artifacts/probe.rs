use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma8::Ifma8, Fq, FqConfig};

type V = Ifma8<FqConfig, true>;
type Complex = [V; 2];

#[inline(always)]
fn ordinary(a: &Complex, b: &Complex) -> Complex {
    [
        a[0].difference2(&b[0], &a[1], &b[1]),
        a[0].sum2(&b[1], &a[1], &b[0]),
    ]
}

#[inline(always)]
fn multiply<const STRATEGY: u8>(a: &Complex, b: &Complex) -> Complex {
    #[cfg(zopro_gauss_positive)]
    {
        if STRATEGY == 1 {
            return V::complex_product_streamed(&a[0], &a[1], &b[0], &b[1]);
        }
        if STRATEGY == 2 {
            return V::complex_product_gauss(&a[0], &a[1], &b[0], &b[1]);
        }
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
pub fn complex_old_codegen(a: &Complex, b: &Complex) -> Complex {
    ordinary(a, b)
}

#[cfg(zopro_gauss_positive)]
#[no_mangle]
#[inline(never)]
pub fn complex_streamed_codegen(a: &Complex, b: &Complex) -> Complex {
    V::complex_product_streamed(&a[0], &a[1], &b[0], &b[1])
}

#[cfg(zopro_gauss_positive)]
#[no_mangle]
#[inline(never)]
pub fn complex_gauss_codegen(a: &Complex, b: &Complex) -> Complex {
    V::complex_product_gauss(&a[0], &a[1], &b[0], &b[1])
}
#[inline(never)]
fn run<const STRATEGY: u8>(pattern: &str, iterations: usize) -> (u128, Vec<V>) {
    let b = [vector(701), vector(907)];
    let factor = vector(211);
    let ax = [vector(307), vector(401)];
    let ay = [vector(503), vector(601)];
    let mut z = [vector(101), vector(103)];
    let mut streams = [
        [vector(1001), vector(1003)],
        [vector(1101), vector(1103)],
        [vector(1201), vector(1203)],
        [vector(1301), vector(1303)],
    ];
    let started = Instant::now();
    match pattern {
        "chain" => {
            for _ in 0..iterations {
                z = multiply::<STRATEGY>(&z, &b);
            }
        }
        "streams4" => {
            for _ in 0..iterations {
                for stream in &mut streams {
                    *stream = multiply::<STRATEGY>(stream, &b);
                }
            }
        }
        "scaled" => {
            for _ in 0..iterations {
                z = multiply::<STRATEGY>(&z, &b);
                z = [z[0].mul(&factor), z[1].mul(&factor)];
            }
        }
        "affine-expression" => {
            for _ in 0..iterations {
                let mut slope = multiply::<STRATEGY>(&z, &b);
                slope = [slope[0].mul(&factor), slope[1].mul(&factor)];
                let cross = slope[0].mul(&slope[1]);
                let square = [
                    slope[0].add(&slope[1]).mul(&slope[0].sub(&slope[1])),
                    cross.add(&cross),
                ];
                let dx = [ax[0].sub(&square[0]), ax[1].sub(&square[1])];
                z = multiply::<STRATEGY>(&slope, &dx);
                z = [z[0].sub(&ay[0]), z[1].sub(&ay[1])];
            }
        }
        _ => unreachable!(),
    }
    black_box((&z, &streams));
    let elapsed = started.elapsed().as_nanos();
    let result = if pattern == "streams4" {
        streams.into_iter().flatten().collect()
    } else {
        z.to_vec()
    };
    (elapsed, black_box(result))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 2 && args[1] == "codegen" {
        let a = [vector(101), vector(103)];
        let b = [vector(701), vector(907)];
        black_box(complex_old_codegen(&a, &b));
        #[cfg(zopro_gauss_positive)]
        {
            black_box(complex_streamed_codegen(&a, &b));
            black_box(complex_gauss_codegen(&a, &b));
        }
        return Ok(());
    }
    if args.len() != 4 {
        return Err("usage: component old|streamed|gauss PATTERN ITERATIONS".into());
    }
    let strategy: u8 = match args[1].as_str() {
        "old" => 0,
        "streamed" if cfg!(zopro_gauss_positive) => 1,
        "gauss" if cfg!(zopro_gauss_positive) => 2,
        _ => return Err("unsupported mode".into()),
    };
    let pattern = args[2].as_str();
    if !["chain", "streams4", "scaled", "affine-expression"].contains(&pattern) {
        return Err("unsupported pattern".into());
    }
    let iterations: usize = args[3].parse()?;
    if iterations == 0 {
        return Err("iterations must be positive".into());
    }
    for _ in 0..3 {
        black_box(match strategy {
            0 => run::<0>(pattern, 2000),
            1 => run::<1>(pattern, 2000),
            2 => run::<2>(pattern, 2000),
            _ => unreachable!(),
        });
    }
    let (elapsed_ns, result) = match strategy {
        0 => run::<0>(pattern, iterations),
        1 => run::<1>(pattern, iterations),
        2 => run::<2>(pattern, iterations),
        _ => unreachable!(),
    };
    let words: Vec<u64> = result
        .into_iter()
        .flat_map(|v| v.get())
        .flat_map(|v| v.to_canonical())
        .collect();
    let streams = if pattern == "streams4" { 4. } else { 1. };
    println!("ROW {{\"pattern\":\"{pattern}\",\"strategy\":{strategy},\"iterations\":{iterations},\"primes\":3,\"elapsed_ns\":{elapsed_ns},\"ns_per_lane\":{},\"output_words\":{words:?}}}", elapsed_ns as f64 / (iterations as f64 * 8. * streams));
    Ok(())
}
include!("probe-tests.rs");
