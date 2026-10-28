use ark_ff::Field;
use std::{
    hint::black_box,
    ops::{Add, MulAssign},
    time::Instant,
};
fn bench<T: Copy + MulAssign + Add<Output = T>>(name: &str, a: T, b: T) {
    let (a, b) = (black_box(a), black_box(b));
    let n = 10_000_000;
    let mut x = a;
    let t = Instant::now();
    for _ in 0..n {
        x *= b;
    }
    black_box(x);
    println!(
        "{name} dependent_ns={:.2}",
        t.elapsed().as_nanos() as f64 / n as f64
    );
    let mut x = [a, b, a + b, a];
    let t = Instant::now();
    for _ in 0..n / 4 {
        for v in &mut x {
            *v *= b;
        }
    }
    black_box(x);
    println!(
        "{name} independent_ns={:.2}",
        t.elapsed().as_nanos() as f64 / n as f64
    );
}
fn main() {
    let a = zopro_bn254::Fq::from_decimal(
        "123456789012345678901234567890123456789012345678901234567890",
    );
    let b = zopro_bn254::Fq::from_decimal(
        "3456789012345678901234567890123456789012345678901234567890123",
    );
    bench("zopro fq", a, b);
    bench(
        "ark fq",
        ark_ref::Fq::new_unchecked(ark_ff::BigInt(a.0)),
        ark_ref::Fq::new_unchecked(ark_ff::BigInt(b.0)),
    );
    bench(
        "zopro fq2",
        zopro_bn254::Fq2::new(a, b),
        zopro_bn254::Fq2::new(b, a),
    );
    bench(
        "ark fq2",
        ark_ref::Fq2::new(
            ark_ref::Fq::new_unchecked(ark_ff::BigInt(a.0)),
            ark_ref::Fq::new_unchecked(ark_ff::BigInt(b.0)),
        ),
        ark_ref::Fq2::new(
            ark_ref::Fq::new_unchecked(ark_ff::BigInt(b.0)),
            ark_ref::Fq::new_unchecked(ark_ff::BigInt(a.0)),
        ),
    );
    black_box(ark_ref::Fq::ONE);
    let n = 100_000;
    let mut x = black_box(a);
    let t = Instant::now();
    for _ in 0..n {
        x = (x + b).inverse().unwrap();
    }
    black_box(x);
    println!(
        "zopro inverse_ns={:.0}",
        t.elapsed().as_nanos() as f64 / n as f64
    );
    let mut x = black_box(ark_ref::Fq::new_unchecked(ark_ff::BigInt(a.0)));
    let bb = black_box(ark_ref::Fq::new_unchecked(ark_ff::BigInt(b.0)));
    let t = Instant::now();
    for _ in 0..n {
        x = (x + bb).inverse().unwrap();
    }
    black_box(x);
    println!(
        "ark inverse_ns={:.0}",
        t.elapsed().as_nanos() as f64 / n as f64
    );
    let mut x = black_box(a);
    let mut e = zopro_bn254::Fq::MODULUS;
    e[0] -= 2;
    let t = Instant::now();
    for _ in 0..n / 10 {
        x = (x + b).pow(&e);
    }
    black_box(x);
    println!(
        "zopro fermat_inverse_ns={:.0}",
        t.elapsed().as_nanos() as f64 / (n / 10) as f64
    );
}
