use std::{hint::black_box, time::Instant};
#[path = "support/perf.rs"]
mod perf;
type Words = [[u64; 4]; 8];
trait Ops {
    type V: Copy;
    fn new(v: Words) -> Self::V;
    fn get(v: Self::V) -> Words;
    fn add(a: Self::V, b: Self::V) -> Self::V;
    fn sub(a: Self::V, b: Self::V) -> Self::V;
    fn mul(a: Self::V, b: Self::V) -> Self::V;
}
macro_rules! backend {
    ($name:ident,$lib:ident,$field:ident,$mixed:literal,$add:ident,$sub:ident) => {
        struct $name;
        impl Ops for $name {
            type V = $lib::ifma8::Ifma8<$lib::$field, $mixed>;
            #[inline(always)]
            fn new(v: Words) -> Self::V {
                Self::V::new(&v.map($lib::Fp::from_montgomery))
            }
            #[inline(always)]
            fn get(v: Self::V) -> Words {
                v.get().map(|x| x.0)
            }
            #[inline(always)]
            fn add(a: Self::V, b: Self::V) -> Self::V {
                a.$add(&b)
            }
            #[inline(always)]
            fn sub(a: Self::V, b: Self::V) -> Self::V {
                a.$sub(&b)
            }
            #[inline(always)]
            fn mul(a: Self::V, b: Self::V) -> Self::V {
                a.mul(&b)
            }
        }
    };
}
backend!(Fq256Frozen, frozen_bn254, FqConfig, true, add, sub);
backend!(Fq260Frozen, frozen_bn254, FqConfig, false, add, sub);
backend!(Fr256Frozen, frozen_bn254, FrConfig, true, add, sub);
backend!(Fr260Frozen, frozen_bn254, FrConfig, false, add, sub);
backend!(Fq256Fresh, zopro_bn254, FqConfig, true, add, sub);
backend!(Fq260Fresh, zopro_bn254, FqConfig, false, add, sub);
backend!(Fr256Fresh, zopro_bn254, FrConfig, true, add, sub);
backend!(Fr260Fresh, zopro_bn254, FrConfig, false, add, sub);
backend!(Fq256Lazy, zopro_bn254, FqConfig, true, add_lazy, sub_lazy);
backend!(Fq260Lazy, zopro_bn254, FqConfig, false, add_lazy, sub_lazy);
backend!(Fr256Lazy, zopro_bn254, FrConfig, true, add_lazy, sub_lazy);
backend!(Fr260Lazy, zopro_bn254, FrConfig, false, add_lazy, sub_lazy);

#[inline(never)]
fn kernel<O: Ops, const P: u8>(mut s: [O::V; 4], b: [O::V; 3], iterations: usize) -> [O::V; 4] {
    match P {
        0 => {
            for _ in 0..iterations {
                s[0] = O::add(s[0], b[0]);
            }
        }
        1 => {
            for _ in 0..iterations {
                s[0] = O::sub(s[0], b[1]);
            }
        }
        2 => {
            for _ in 0..iterations {
                s[0] = O::add(s[0], b[0]);
                s[1] = O::sub(s[1], b[1]);
                s[2] = O::add(s[2], b[2]);
                s[3] = O::sub(s[3], b[0]);
            }
        }
        3 => {
            for _ in 0..iterations {
                let slope = O::mul(s[0], b[0]);
                let x = O::sub(O::mul(slope, slope), O::add(b[1], b[2]));
                s[0] = O::sub(O::mul(slope, O::sub(b[1], x)), b[2]);
            }
        }
        _ => unreachable!(),
    }
    s
}
fn inputs<C: frozen_bn254::FpConfig>(block: u64) -> [Words; 7] {
    let mut seed = 0x6c696e6561725f31u64.wrapping_add(block);
    core::array::from_fn(|_| {
        core::array::from_fn(|_| {
            let mut bytes = [0u8; 64];
            for chunk in bytes.chunks_mut(8) {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                chunk.copy_from_slice(&seed.to_le_bytes());
            }
            frozen_bn254::Fp::<C>::from_random_bytes(&bytes).0
        })
    })
}
fn execute<O: Ops, F: Ops, const P: u8>(
    input: [Words; 7],
    iterations: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let states: [O::V; 4] = core::array::from_fn(|i| O::new(input[i]));
    let b: [O::V; 3] = core::array::from_fn(|i| O::new(input[i + 4]));
    let refs: [F::V; 4] = core::array::from_fn(|i| F::new(input[i]));
    let rb: [F::V; 3] = core::array::from_fn(|i| F::new(input[i + 4]));
    let want = kernel::<F, P>(black_box(refs), black_box(rb), black_box(iterations)).map(F::get);
    let prime_want = kernel::<F, P>(black_box(refs), black_box(rb), black_box(512)).map(F::get);
    for _ in 0..3 {
        let out = kernel::<O, P>(black_box(states), black_box(b), black_box(512));
        assert_eq!(out.map(O::get), prime_want);
    }
    let mut control = perf::Control::from_env()?;
    if let Some(c) = control.as_mut() {
        c.enable()?;
    }
    let before = perf::Usage::now()?;
    let started = Instant::now();
    let out = black_box(kernel::<O, P>(
        black_box(states),
        black_box(b),
        black_box(iterations),
    ));
    let ms = started.elapsed().as_secs_f64() * 1000.;
    let after = perf::Usage::now()?;
    if let Some(c) = control.as_mut() {
        c.disable()?;
    }
    let checksum = out.map(O::get);
    assert_eq!(checksum, want);
    let mut summary = after.difference(&before);
    summary["pid"] = std::process::id().into();
    summary["iterations"] = iterations.into();
    summary["priming"] = 3.into();
    let ns = ms * 1e6 / (iterations * 8 * if P == 2 { 4 } else { 1 }) as f64;
    println!(
        "RESULT {}",
        serde_json::json!({"elapsed_ms":ms,"ns_per_lane_iteration":ns,"pattern":P,"iterations":iterations,"summary":summary,"checksum":checksum,"frozen_reference_equal":true,"sdk_proofs":0,"gnark_exports":0})
    );
    Ok(())
}
fn dispatch<O: Ops, F: Ops>(
    input: [Words; 7],
    p: u8,
    n: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    match p {
        0 => execute::<O, F, 0>(input, n),
        1 => execute::<O, F, 1>(input, n),
        2 => execute::<O, F, 2>(input, n),
        3 => execute::<O, F, 3>(input, n),
        _ => panic!("pattern0..3"),
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(
        a.len(),
        7,
        "probe FIELD RADIX MODE PATTERN ITERATIONS BLOCK"
    );
    let p = a[4].parse()?;
    let n = a[5].parse()?;
    let block = a[6].parse()?;
    assert!(n > 0);
    match (a[1].as_str(), a[2].as_str(), a[3].as_str()) {
        ("fq", "256", "frozen") => {
            dispatch::<Fq256Frozen, Fq256Frozen>(inputs::<frozen_bn254::FqConfig>(block), p, n)
        }
        ("fq", "256", "fresh") => {
            dispatch::<Fq256Fresh, Fq256Frozen>(inputs::<frozen_bn254::FqConfig>(block), p, n)
        }
        ("fq", "256", "lazy") => {
            dispatch::<Fq256Lazy, Fq256Frozen>(inputs::<frozen_bn254::FqConfig>(block), p, n)
        }
        ("fq", "260", "frozen") => {
            dispatch::<Fq260Frozen, Fq260Frozen>(inputs::<frozen_bn254::FqConfig>(block), p, n)
        }
        ("fq", "260", "fresh") => {
            dispatch::<Fq260Fresh, Fq260Frozen>(inputs::<frozen_bn254::FqConfig>(block), p, n)
        }
        ("fq", "260", "lazy") => {
            dispatch::<Fq260Lazy, Fq260Frozen>(inputs::<frozen_bn254::FqConfig>(block), p, n)
        }
        ("fr", "256", "frozen") => {
            dispatch::<Fr256Frozen, Fr256Frozen>(inputs::<frozen_bn254::FrConfig>(block), p, n)
        }
        ("fr", "256", "fresh") => {
            dispatch::<Fr256Fresh, Fr256Frozen>(inputs::<frozen_bn254::FrConfig>(block), p, n)
        }
        ("fr", "256", "lazy") => {
            dispatch::<Fr256Lazy, Fr256Frozen>(inputs::<frozen_bn254::FrConfig>(block), p, n)
        }
        ("fr", "260", "frozen") => {
            dispatch::<Fr260Frozen, Fr260Frozen>(inputs::<frozen_bn254::FrConfig>(block), p, n)
        }
        ("fr", "260", "fresh") => {
            dispatch::<Fr260Fresh, Fr260Frozen>(inputs::<frozen_bn254::FrConfig>(block), p, n)
        }
        ("fr", "260", "lazy") => {
            dispatch::<Fr260Lazy, Fr260Frozen>(inputs::<frozen_bn254::FrConfig>(block), p, n)
        }
        _ => panic!("field/radix/mode"),
    }
}
