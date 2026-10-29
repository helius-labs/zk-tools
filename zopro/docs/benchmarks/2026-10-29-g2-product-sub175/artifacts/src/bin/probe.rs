use serde_json::json;
use sha2::{Digest, Sha256};
use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma8::Ifma8, Field, FpConfig, Fq, Fq2, FqConfig};
type V = Ifma8<FqConfig, true>;
type Pair = [V; 2];
#[inline(always)]
fn add(a: Pair, b: Pair) -> Pair {
    [a[0].add(&b[0]), a[1].add(&b[1])]
}
#[inline(always)]
fn sub(a: Pair, b: Pair) -> Pair {
    [a[0].sub(&b[0]), a[1].sub(&b[1])]
}
#[inline(always)]
fn scale(a: Pair, t: V) -> Pair {
    [a[0].mul(&t), a[1].mul(&t)]
}
#[inline(always)]
fn mul(a: Pair, b: Pair) -> Pair {
    [
        a[0].difference2(&b[0], &a[1], &b[1]),
        a[0].sum2(&b[1], &a[1], &b[0]),
    ]
}
#[inline(always)]
fn square(a: Pair) -> Pair {
    let p = a[0].mul(&a[1]);
    [a[0].add(&a[1]).mul(&a[0].sub(&a[1])), p.add(&p)]
}
#[inline(always)]
fn product_sub<const ALT: bool>(a: Pair, b: Pair, c: Pair) -> Pair {
    #[cfg(product_sub_kernel)]
    if ALT {
        return [
            a[0].difference2_sub_canonical(&b[0], &a[1], &b[1], &c[0]),
            a[0].sum2_sub_canonical(&b[1], &a[1], &b[0], &c[1]),
        ];
    }
    sub(mul(a, b), c)
}
#[inline(never)]
fn chain<const ALT: bool>(
    mut x: Pair,
    b: Pair,
    c: Pair,
    t: V,
    n: usize,
    scaled: bool,
) -> [Pair; 4] {
    for _ in 0..n {
        if scaled {
            x = scale(x, t);
        }
        x = product_sub::<ALT>(x, b, c);
    }
    [black_box(x); 4]
}
#[inline(never)]
fn streams<const ALT: bool>(mut x: [Pair; 4], b: Pair, c: Pair, n: usize) -> [Pair; 4] {
    for _ in 0..n {
        for x in &mut x {
            *x = product_sub::<ALT>(*x, b, c);
        }
    }
    black_box(x)
}
#[inline(never)]
fn affine<const ALT: bool>(mut x: Pair, ay: Pair, bx: Pair, by: Pair, t: V, n: usize) -> [Pair; 4] {
    for _ in 0..n {
        let dx = sub(bx, x);
        let dy = sub(by, ay);
        let slope = scale(
            [
                dy[0].sum2(&dx[0], &dy[1], &dx[1]),
                dy[1].difference2(&dx[0], &dy[0], &dx[1]),
            ],
            t,
        );
        let outx = sub(square(slope), add(x, bx));
        let outy = product_sub::<ALT>(slope, sub(x, outx), ay);
        x = add(outx, outy);
    }
    [black_box(x); 4]
}
fn pack(x: [Fq2; 8]) -> Pair {
    [V::new(&x.map(|a| a.c0)), V::new(&x.map(|a| a.c1))]
}
fn get(x: Pair) -> [Fq2; 8] {
    let [a, b] = x.map(|v| v.get());
    core::array::from_fn(|i| Fq2::new(a[i], b[i]))
}
fn execute<const ALT: bool>(
    pattern: &str,
    x: [Pair; 4],
    b: Pair,
    c: Pair,
    d: Pair,
    t: V,
    n: usize,
) -> [Pair; 4] {
    match pattern {
        "chain" => chain::<ALT>(x[0], b, c, t, n, false),
        "streams4" => streams::<ALT>(x, b, c, n),
        "scaled" => chain::<ALT>(x[0], b, c, t, n, true),
        "affine" => affine::<ALT>(x[0], c, b, d, t, n),
        _ => panic!("pattern"),
    }
}
fn run<const ALT: bool>(mode: &str, pattern: &str, n: usize) {
    let mut seed = 0x8e45d9182bcafd31u64;
    let mut next = || {
        let mut bytes = [0u8; 64];
        for q in bytes.chunks_mut(8) {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            q.copy_from_slice(&seed.to_le_bytes());
        }
        Fq::from_le_bytes_mod_order(&bytes)
    };
    let raw: [[Fq2; 8]; 4] =
        core::array::from_fn(|_| core::array::from_fn(|_| Fq2::new(next(), next())));
    let rb = core::array::from_fn(|_| Fq2::new(next(), next()));
    let rc = core::array::from_fn(|_| Fq2::new(next(), next()));
    let rd = core::array::from_fn(|_| Fq2::new(next(), next()));
    let rt = core::array::from_fn(|_| next());
    let x = raw.map(pack);
    let b = pack(rb);
    let c = pack(rc);
    let d = pack(rd);
    let t = V::new(&rt);
    for _ in 0..3 {
        black_box(execute::<ALT>(pattern, black_box(x), b, c, d, t, 2000));
    }
    let start = Instant::now();
    let output = execute::<ALT>(pattern, black_box(x), b, c, d, t, n);
    let elapsed = start.elapsed();
    let actual = output.map(get);
    let mut expected = raw;
    for stream in 0..4 {
        for lane in 0..8 {
            let initial = raw[if pattern == "streams4" { stream } else { 0 }][lane];
            expected[stream][lane] = if pattern == "affine" {
                let mut x = initial;
                for _ in 0..n {
                    let dx = rb[lane] - x;
                    let dy = rd[lane] - rc[lane];
                    let slope = (dy * dx.conjugate()).mul_by_fp(&rt[lane]);
                    let outx = slope.square() - x - rb[lane];
                    let outy = slope * (x - outx) - rc[lane];
                    x = outx + outy;
                }
                x
            } else {
                let factor = if pattern == "scaled" {
                    rb[lane].mul_by_fp(&rt[lane])
                } else {
                    rb[lane]
                };
                let p = factor.pow(&[n as u64]);
                let geom = if factor == Fq2::ONE {
                    Fq2::new(Fq::from(n as u64), Fq::ZERO)
                } else {
                    (p - Fq2::ONE) * (factor - Fq2::ONE).inverse().unwrap()
                };
                initial * p - rc[lane] * geom
            };
            assert_eq!(actual[stream][lane], expected[stream][lane]);
        }
    }
    let bytes: Vec<_> = actual
        .iter()
        .flatten()
        .flat_map(|v| [v.c0, v.c1])
        .flat_map(|v| v.to_canonical().into_iter().flat_map(u64::to_le_bytes))
        .collect();
    let divisor = n * 8 * if pattern == "streams4" { 4 } else { 1 };
    println!(
        "RESULT {}",
        json!({"mode":mode,"pattern":pattern,"iterations":n,"prime_calls":3,"prime_iterations":2000,"elapsed_ns":elapsed.as_nanos(),"ns_per_unit":elapsed.as_nanos()as f64/divisor as f64,"output_sha256":format!("{:x}",Sha256::digest(bytes)),"checked_Fq2_elements":32,"internally_verified":true,"SDK_requests":0})
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let n = a[3].parse().unwrap();
    assert!(n > 0);
    if a[1] == "candidate" {
        run::<true>(&a[1], &a[2], n)
    } else {
        assert_eq!(a[1], "ordinary");
        run::<false>(&a[1], &a[2], n)
    }
}
