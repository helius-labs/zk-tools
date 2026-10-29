use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma8::Ifma8, Field, Fr, FrConfig};
type Pair = Ifma8<FrConfig>;
include!("original-within.rs");
struct Roots {
    w: Fr,
    twiddle: Pair,
    pair: Pair,
    factor: Pair,
}
impl Roots {
    fn new(power: u64) -> Self {
        let w = Fr::root_of_unity(8).unwrap().pow(&[power]);
        let i = w.square();
        let w3 = i * w;
        let half = Fr::from(2u64).inverse().unwrap();
        Self {
            w,
            twiddle: Pair::new(&[Fr::ONE, w, i, w3, Fr::ONE, w, i, w3]),
            pair: Pair::new(&[Fr::ONE, i, Fr::ONE, i, Fr::ONE, i, Fr::ONE, i]),
            factor: Pair::new(&[
                i,
                i,
                (w + w3) * half,
                (w - w3) * half,
                i,
                i,
                (w + w3) * half,
                (w - w3) * half,
            ]),
        }
    }
}
#[inline(always)]
fn original(a: &mut Pair, b: &mut Pair, r: &Roots) {
    let difference = a.sub(b);
    *a = a.add(b);
    *b = difference.mul(&r.twiddle);
    dif_within(a, b, &r.pair);
}
#[inline(always)]
fn factored(a: &mut Pair, b: &mut Pair, r: &Roots) {
    let s = a.add(b);
    let t = a.sub(b);
    let sr = Pair::shuffle_pair::<2, 3, 0, 1>(&s, &s);
    let sd = s.sub(&sr);
    let se = s.add(&sr);
    let tr = Pair::shuffle_pair::<2, 3, 0, 1>(&t, &t);
    let tu = t.add(&tr);
    let tv = t.sub(&tr);
    let p0 = Pair::shuffle_pair::<1, 6, 0, 0>(&sd, &t);
    let p1 = Pair::shuffle_pair::<1, 5, 1, 5>(&tu, &tv);
    let p = Pair::shuffle_pair::<0, 1, 4, 5>(&p0, &p1).mul(&r.factor);
    let ea = Pair::shuffle_pair::<0, 0, 4, 4>(&se, &sd);
    let eb = Pair::shuffle_pair::<1, 1, 4, 4>(&se, &p);
    *a = Pair::shuffle_pair::<0, 5, 2, 7>(&ea.add(&eb), &ea.sub(&eb));
    let b0 = Pair::shuffle_pair::<0, 0, 0, 0>(&t, &t);
    let p1 = Pair::shuffle_pair::<1, 1, 1, 1>(&p, &p);
    let base = Pair::shuffle_pair::<0, 1, 6, 7>(&b0.add(&p1), &b0.sub(&p1));
    let p2 = Pair::shuffle_pair::<2, 2, 2, 2>(&p, &p);
    let p3 = Pair::shuffle_pair::<3, 3, 3, 3>(&p, &p);
    let term = Pair::shuffle_pair::<0, 1, 6, 7>(&p2.add(&p3), &p2.sub(&p3));
    *b = Pair::shuffle_pair::<0, 5, 2, 7>(&base.add(&term), &base.sub(&term));
}
fn pack(x: [Fr; 16]) -> [Pair; 2] {
    [
        Pair::new(&core::array::from_fn(|i| x[if i < 4 { i } else { i + 4 }])),
        Pair::new(&core::array::from_fn(|i| {
            x[if i < 4 { i + 4 } else { i + 8 }]
        })),
    ]
}
fn unpack(v: &[Pair; 2]) -> [Fr; 16] {
    let a = v[0].get();
    let b = v[1].get();
    core::array::from_fn(|i| {
        let h = i / 8;
        let j = i % 8;
        if j < 4 {
            a[4 * h + j]
        } else {
            b[4 * h + j - 4]
        }
    })
}
fn reverse(i: usize) -> usize {
    ((i & 1) << 2) | (i & 2) | ((i & 4) >> 2)
}
fn direct(x: &[Fr; 8], w: Fr) -> [Fr; 8] {
    core::array::from_fn(|i| {
        let k = reverse(i);
        let step = w.pow(&[k as u64]);
        let mut t = Fr::ONE;
        let mut s = Fr::ZERO;
        for x in x {
            s += *x * t;
            t *= step;
        }
        s
    })
}
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn field(seed: &mut u64) -> Fr {
    let mut b = [0u8; 32];
    for c in b.chunks_mut(8) {
        c.copy_from_slice(&next(seed).to_le_bytes());
    }
    Fr::from_le_bytes_mod_order(&b)
}
fn fixtures(seed: u64) -> Vec<[Fr; 16]> {
    let mut seed = seed | 1;
    (0..64)
        .map(|_| core::array::from_fn(|_| field(&mut seed)))
        .collect()
}
type Matrix = [[Fr; 8]; 8];
fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
    core::array::from_fn(|i| {
        core::array::from_fn(|j| (0..8).fold(Fr::ZERO, |s, k| s + a[i][k] * b[k][j]))
    })
}
fn matrix_power(w: Fr, mut n: usize) -> Matrix {
    let mut a: Matrix =
        core::array::from_fn(|i| core::array::from_fn(|j| w.pow(&[(reverse(i) * j) as u64])));
    let mut v: Matrix =
        core::array::from_fn(|i| core::array::from_fn(|j| if i == j { Fr::ONE } else { Fr::ZERO }));
    while n != 0 {
        if n & 1 != 0 {
            v = matmul(&v, &a);
        }
        a = matmul(&a, &a);
        n >>= 1;
    }
    v
}
fn check(v: &[[Pair; 2]], initial: &[[Fr; 16]], r: &Roots, rounds: usize) {
    let base = rounds / 64;
    let lo = matrix_power(r.w, base);
    let hi = matrix_power(r.w, base + 1);
    for (i, (v, x)) in v.iter().zip(initial).enumerate() {
        let m = if i < rounds % 64 { &hi } else { &lo };
        let got = unpack(v);
        for h in 0..2 {
            for j in 0..8 {
                let want = (0..8).fold(Fr::ZERO, |s, k| s + m[j][k] * x[8 * h + k]);
                assert_eq!(got[8 * h + j], want, "fixture{i} half{h} lane{j}");
            }
        }
    }
}
fn run<const NEW: bool>(v: &mut [[Pair; 2]], r: &Roots, rounds: usize) {
    for i in 0..rounds {
        let data = black_box(&mut v[i & 63]);
        let [a, b] = data;
        if NEW {
            factored(a, b, r)
        } else {
            original(a, b, r)
        }
        black_box(&*data);
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 5);
    let mode = &args[1];
    assert!(["A", "B", "candidate"].contains(&mode.as_str()));
    let power = args[2].parse::<u64>().unwrap();
    assert!([1, 3, 5, 7].contains(&power));
    let rounds = args[3].parse::<usize>().unwrap();
    assert!(rounds > 64);
    let seed = args[4].parse().unwrap();
    let roots = Roots::new(power);
    let initial = fixtures(seed);
    let packed: Vec<_> = initial.iter().copied().map(pack).collect();
    let mut values = packed.clone();
    for _ in 0..3 {
        if mode == "candidate" {
            run::<true>(&mut values, &roots, 64)
        } else {
            run::<false>(&mut values, &roots, 64)
        }
    }
    check(&values, &initial, &roots, 192);
    values.copy_from_slice(&packed);
    let started = Instant::now();
    if mode == "candidate" {
        run::<true>(&mut values, &roots, rounds)
    } else {
        run::<false>(&mut values, &roots, rounds)
    }
    let ns = started.elapsed().as_nanos();
    check(&values, &initial, &roots, rounds);
    let checksum = unpack(&values[0])[0].to_canonical();
    println!("{{\"mode\":\"{mode}\",\"power\":{power},\"rounds\":{rounds},\"seed\":{seed},\"priming_calls\":192,\"ns\":{ns},\"ns_per_call\":{},\"checked_outputs\":1024,\"checksum\":{:?}}}",ns as f64/rounds as f64,checksum);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basis_matrices_match_independent_dft_all_primitive_roots() {
        for p in [1, 3, 5, 7] {
            let r = Roots::new(p);
            for lane in 0..16 {
                let mut x = [Fr::ZERO; 16];
                x[lane] = Fr::ONE;
                let mut a = pack(x);
                let mut b = a;
                let [aa, ab] = &mut a;
                original(aa, ab, &r);
                let [ba, bb] = &mut b;
                factored(ba, bb, &r);
                let expected: [Fr; 16] = core::array::from_fn(|i| {
                    let h = i / 8;
                    direct(&x[8 * h..8 * h + 8].try_into().unwrap(), r.w)[i % 8]
                });
                assert_eq!(unpack(&a), expected);
                assert_eq!(unpack(&b), expected);
            }
        }
    }
    #[test]
    fn full_width_boundaries_and_independent_sequences_match() {
        let mut seed = 77413;
        for p in [1, 3, 5, 7] {
            let r = Roots::new(p);
            for k in 0..1024 {
                let x = core::array::from_fn(|i| match (i + k) % 16 {
                    0 => Fr::ZERO,
                    1 => Fr::ONE,
                    2 => -Fr::ONE,
                    _ => field(&mut seed),
                });
                let mut a = pack(x);
                let mut b = a;
                let [aa, ab] = &mut a;
                original(aa, ab, &r);
                let [ba, bb] = &mut b;
                factored(ba, bb, &r);
                let expected: [Fr; 16] = core::array::from_fn(|i| {
                    let h = i / 8;
                    direct(&x[8 * h..8 * h + 8].try_into().unwrap(), r.w)[i % 8]
                });
                assert_eq!(unpack(&a), expected);
                assert_eq!(unpack(&b), expected);
            }
        }
    }
    #[test]
    fn changing_recurrence_checks_all_fixtures_against_matrix_power() {
        for p in [1, 3, 5, 7] {
            let r = Roots::new(p);
            let initial = fixtures(p + 129);
            for rounds in [1, 7, 63, 64, 65, 129, 8191] {
                let mut a: Vec<_> = initial.iter().copied().map(pack).collect();
                let mut b = a.clone();
                run::<false>(&mut a, &r, rounds);
                run::<true>(&mut b, &r, rounds);
                check(&a, &initial, &r, rounds);
                check(&b, &initial, &r, rounds);
            }
        }
    }
}
