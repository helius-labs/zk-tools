use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma::Ifma, ifma8::Ifma8, Fq, Fq2, FqConfig};

type V = Ifma8<FqConfig, true>;
type H = Ifma<FqConfig, true>;

trait Complex: Copy {
    fn new(x: &[Fq2; 8]) -> Self;
    fn get(&self) -> [Fq2; 8];
    fn add(&self, b: &Self) -> Self;
    fn sub(&self, b: &Self) -> Self;
    fn mul(&self, b: &Self) -> Self;
    fn mul_conjugate(&self, b: &Self) -> Self;
    fn square(&self) -> Self;
    fn scale(&self, b: &V) -> Self;
    fn norm(&self) -> V;
}

#[derive(Clone, Copy)]
struct Split([V; 2]);
impl Complex for Split {
    #[inline(always)]
    fn new(x: &[Fq2; 8]) -> Self {
        Self([V::new(&x.map(|v| v.c0)), V::new(&x.map(|v| v.c1))])
    }
    #[inline(always)]
    fn get(&self) -> [Fq2; 8] {
        let a = self.0[0].get();
        let b = self.0[1].get();
        core::array::from_fn(|i| Fq2::new(a[i], b[i]))
    }
    #[inline(always)]
    fn add(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i].add(&b.0[i])))
    }
    #[inline(always)]
    fn sub(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i].sub(&b.0[i])))
    }
    #[inline(always)]
    fn mul(&self, b: &Self) -> Self {
        Self([
            self.0[0].difference2(&b.0[0], &self.0[1], &b.0[1]),
            self.0[0].sum2(&b.0[1], &self.0[1], &b.0[0]),
        ])
    }
    #[inline(always)]
    fn mul_conjugate(&self, b: &Self) -> Self {
        Self([
            self.0[0].sum2(&b.0[0], &self.0[1], &b.0[1]),
            self.0[1].difference2(&b.0[0], &self.0[0], &b.0[1]),
        ])
    }
    #[inline(always)]
    fn square(&self) -> Self {
        let im = self.0[0].mul(&self.0[1]);
        Self([
            self.0[0].add(&self.0[1]).mul(&self.0[0].sub(&self.0[1])),
            im.add(&im),
        ])
    }
    #[inline(always)]
    fn scale(&self, b: &V) -> Self {
        Self(self.0.map(|a| a.mul(b)))
    }
    #[inline(always)]
    fn norm(&self) -> V {
        self.0[0].sum2(&self.0[0], &self.0[1], &self.0[1])
    }
}

// Each vector contains real lanes 0..4 and imaginary lanes 4..8.
#[derive(Clone, Copy)]
struct Interleaved([V; 2]);
#[inline(always)]
fn packed_mul(a: &V, b: &V, conjugate: bool) -> V {
    let (ar, ai) = a.halves();
    let (br, bi) = b.halves();
    let ar = V::from_halves(&ar, &ar);
    let neg = H::zero().sub(if conjugate { &bi } else { &ai });
    let c = if conjugate {
        V::from_halves(&ai, &ai)
    } else {
        V::from_halves(&neg, &ai)
    };
    let d = V::from_halves(&bi, &br);
    let b = if conjugate {
        V::from_halves(&br, &neg)
    } else {
        *b
    };
    ar.sum2(&b, &c, &d)
}
impl Complex for Interleaved {
    #[inline(always)]
    fn new(x: &[Fq2; 8]) -> Self {
        Self(core::array::from_fn(|j| {
            V::new(&core::array::from_fn(|i| {
                if i < 4 {
                    x[j * 4 + i].c0
                } else {
                    x[j * 4 + i - 4].c1
                }
            }))
        }))
    }
    #[inline(always)]
    fn get(&self) -> [Fq2; 8] {
        let a = self.0.map(|v| v.get());
        core::array::from_fn(|i| Fq2::new(a[i / 4][i % 4], a[i / 4][i % 4 + 4]))
    }
    #[inline(always)]
    fn add(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i].add(&b.0[i])))
    }
    #[inline(always)]
    fn sub(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i].sub(&b.0[i])))
    }
    #[inline(always)]
    fn mul(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| {
            packed_mul(&self.0[i], &b.0[i], false)
        }))
    }
    #[inline(always)]
    fn mul_conjugate(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| {
            packed_mul(&self.0[i], &b.0[i], true)
        }))
    }
    #[inline(always)]
    fn square(&self) -> Self {
        Self(self.0.map(|a| {
            let (re, im) = a.halves();
            let left = V::from_halves(&re.add(&im), &re);
            let right = V::from_halves(&re.sub(&im), &im);
            let (re, im) = left.mul(&right).halves();
            V::from_halves(&re, &im.add(&im))
        }))
    }
    #[inline(always)]
    fn scale(&self, b: &V) -> Self {
        let (lo, hi) = b.halves();
        Self([
            self.0[0].mul(&V::from_halves(&lo, &lo)),
            self.0[1].mul(&V::from_halves(&hi, &hi)),
        ])
    }
    #[inline(always)]
    fn norm(&self) -> V {
        let norms = self.0.map(|a| {
            let (re, im) = a.halves();
            re.sum2(&re, &im, &im)
        });
        V::from_halves(&norms[0], &norms[1])
    }
}

// The accepted G2 reverse-pass algebra, with the norm-prefix inverse factor
// supplied by the caller. This function does not perform an inversion.
#[inline(always)]
fn finish<C: Complex>(ax: C, ay: C, bx: C, by: C, factor: V) -> (C, C) {
    let dx = bx.sub(&ax);
    let dy = by.sub(&ay);
    let slope = dy.mul_conjugate(&dx).scale(&factor);
    let x = slope.square().sub(&ax.add(&bx));
    let y = slope.mul(&ax.sub(&x)).sub(&ay);
    (x, y)
}

fn input(offset: u64) -> [Fq2; 8] {
    core::array::from_fn(|i| {
        let mut x = Fq::from(offset + i as u64 * 173 + 1);
        for _ in 0..7 {
            x = x.square() + Fq::from(7247u64);
        }
        Fq2::new(x, x.square() + Fq::from(91u64))
    })
}

// Non-inlined outer loops remain visible to linked assembly and perf audit.
#[inline(never)]
fn chain<C: Complex>(iterations: usize) -> (u128, Vec<[u64; 4]>) {
    let mut a = C::new(&input(1));
    let b = black_box(C::new(&input(3)));
    let c = black_box(C::new(&input(11)));
    let start = Instant::now();
    for _ in 0..iterations {
        a = a.mul(&b).add(&c);
    }
    black_box(&a);
    let ns = start.elapsed().as_nanos();
    (ns, words(&[a]))
}
#[inline(never)]
fn four_streams<C: Complex>(iterations: usize) -> (u128, Vec<[u64; 4]>) {
    let mut a = core::array::from_fn::<_, 4, _>(|i| C::new(&input(i as u64 + 1)));
    let b = black_box(C::new(&input(3)));
    let c = black_box(C::new(&input(11)));
    let start = Instant::now();
    for _ in 0..iterations {
        for state in &mut a {
            *state = state.mul(&b).add(&c);
        }
    }
    black_box(&a);
    let ns = start.elapsed().as_nanos();
    (ns, words(&a))
}
#[inline(never)]
fn scaled_product<C: Complex>(iterations: usize) -> (u128, Vec<[u64; 4]>) {
    let mut a = C::new(&input(1));
    let b = black_box(C::new(&input(3)));
    let c = black_box(C::new(&input(11)));
    let mut factor = V::new(&input(19).map(|v| v.c0));
    let step = black_box(V::new(&input(23).map(|v| v.c0)));
    let start = Instant::now();
    for _ in 0..iterations {
        a = a.mul(&b).scale(&factor).add(&c);
        factor = factor.mul(&step);
    }
    black_box((&a, &factor));
    let ns = start.elapsed().as_nanos();
    let mut out = words(&[a]);
    out.extend(factor.get().map(|v| v.to_canonical()));
    (ns, out)
}
#[inline(never)]
fn affine_expression<C: Complex>(iterations: usize) -> (u128, Vec<[u64; 4]>) {
    let mut x = C::new(&input(1));
    let mut y = C::new(&input(3));
    let bx = black_box(C::new(&input(11)));
    let by = black_box(C::new(&input(19)));
    let scale = black_box(V::new(&input(23).map(|v| v.c0)));
    let start = Instant::now();
    for _ in 0..iterations {
        let d = bx.sub(&x);
        let factor = d.norm().mul(&scale);
        (x, y) = finish(x, y, bx, by, factor);
    }
    black_box((&x, &y));
    let ns = start.elapsed().as_nanos();
    (ns, words(&[x, y]))
}
fn words<C: Complex>(states: &[C]) -> Vec<[u64; 4]> {
    states
        .iter()
        .flat_map(|s| s.get())
        .flat_map(|s| [s.c0.to_canonical(), s.c1.to_canonical()])
        .collect()
}
fn run<C: Complex>(pattern: &str, n: usize) -> (u128, Vec<[u64; 4]>) {
    match pattern {
        "chain" => chain::<C>(n),
        "four-streams" => four_streams::<C>(n),
        "scaled-product" => scaled_product::<C>(n),
        "affine-expression" => affine_expression::<C>(n),
        _ => panic!("unknown pattern"),
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);
    let mode = &args[1];
    let pattern = &args[2];
    let n = args[3].parse::<usize>().unwrap();
    assert!(n > 0);
    let run = |n| match mode.as_str() {
        "reference" => run::<Split>(pattern, n),
        "interleaved" => run::<Interleaved>(pattern, n),
        _ => panic!("unknown mode"),
    };
    for _ in 0..3 {
        black_box(run(2000));
    }
    let (ns, out) = run(n);
    let elements = 8 * if pattern == "four-streams" { 4 } else { 1 };
    println!(
        "{}",
        serde_json::json!({"mode":mode,"pattern":pattern,"iterations":n,"elements_per_iteration":elements,"elapsed_ns":ns,"ns_per_element":ns as f64/(n*elements)as f64,"canonical_words":out})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{Field, PrimeField};
    use zopro_bn254::{Fr, G2Projective};
    fn aq(x: Fq) -> ark_ref::Fq {
        // Interpret raw Montgomery integers independently, including lazy inputs.
        let bytes =
            x.0.into_iter()
                .flat_map(u64::to_le_bytes)
                .collect::<Vec<_>>();
        ark_ref::Fq::from_le_bytes_mod_order(&bytes)
            * ark_ref::Fq::from(2u64).pow([256u64]).inverse().unwrap()
    }
    fn a2(x: Fq2) -> ark_ref::Fq2 {
        ark_ref::Fq2::new(aq(x.c0), aq(x.c1))
    }
    fn check(got: [Fq2; 8], expected: [ark_ref::Fq2; 8]) {
        for i in 0..8 {
            assert_eq!(a2(got[i]), expected[i], "lane {i}");
        }
    }
    fn plus_raw(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        let mut c = 0u128;
        core::array::from_fn(|i| {
            let v = a[i] as u128 + b[i] as u128 + c;
            c = v >> 64;
            v as u64
        })
    }
    #[test]
    fn independent_arkworks_operations_and_lazy_chains() {
        let p = Fq::MODULUS;
        let mut cases = 0;
        for k in 0..2048 {
            let mut a = input(k * 101 + 1);
            let mut b = input(k * 73 + 7);
            for i in 0..8 {
                if (k + i as u64) % 3 == 0 {
                    a[i].c0 = Fq::from_montgomery(plus_raw(a[i].c0.0, p));
                }
                if (k + i as u64) % 5 == 0 {
                    b[i].c1 = Fq::from_montgomery(plus_raw(b[i].c1.0, p));
                }
            }
            let f = V::new(&input(k + 21).map(|v| v.c0));
            let af = f.get().map(aq);
            let mut state = Interleaved::new(&a);
            let other = Interleaved::new(&b);
            let aa = a.map(a2);
            let bb = b.map(a2);
            check(
                state.mul(&other).get(),
                core::array::from_fn(|i| aa[i] * bb[i]),
            );
            check(
                state.mul_conjugate(&other).get(),
                core::array::from_fn(|i| aa[i] * ark_ref::Fq2::new(bb[i].c0, -bb[i].c1)),
            );
            check(state.square().get(), aa.map(|v| v.square()));
            check(
                state.scale(&f).get(),
                core::array::from_fn(|i| aa[i] * ark_ref::Fq2::new(af[i], ark_ref::Fq::ZERO)),
            );
            let n = state.norm().get();
            for i in 0..8 {
                assert_eq!(aq(n[i]), aa[i].c0.square() + aa[i].c1.square());
            }
            let mut expected = aa;
            for _ in 0..8 {
                state = state.mul(&other);
                expected = core::array::from_fn(|i| expected[i] * bb[i]);
                check(state.get(), expected);
            }
            cases += 8 * 13;
        }
        println!("ARK_FQ2_ELEMENT_CHECKS={cases}");
    }
    #[test]
    fn modulus_corners_and_half_order() {
        let p = Fq::MODULUS;
        let two = plus_raw(p, p);
        let sub1 = |mut a: [u64; 4]| {
            for x in &mut a {
                let (v, b) = x.overflowing_sub(1);
                *x = v;
                if !b {
                    break;
                }
            }
            a
        };
        let raw = [
            [0; 4],
            [1, 0, 0, 0],
            sub1(p),
            p,
            plus_raw(p, [1, 0, 0, 0]),
            sub1(two),
        ];
        let mut checks = 0;
        for ai in 0..raw.len() {
            for bi in 0..raw.len() {
                let a = core::array::from_fn(|i| {
                    Fq2::new(
                        Fq::from_montgomery(raw[(ai + i) % raw.len()]),
                        Fq::from_montgomery(raw[(ai + i + 1) % raw.len()]),
                    )
                });
                let b = core::array::from_fn(|i| {
                    Fq2::new(
                        Fq::from_montgomery(raw[(bi + i + 2) % raw.len()]),
                        Fq::from_montgomery(raw[(bi + i + 3) % raw.len()]),
                    )
                });
                let aa = a.map(a2);
                let bb = b.map(a2);
                let a = Interleaved::new(&a);
                let b = Interleaved::new(&b);
                check(a.get(), aa);
                check(a.mul(&b).get(), core::array::from_fn(|i| aa[i] * bb[i]));
                check(a.square().get(), aa.map(|v| v.square()));
                checks += 24;
            }
        }
        println!("CORNER_FQ2_CHECKS={checks}");
    }
    #[test]
    fn complete_curve_finish_matches_independent_arkworks() {
        let generator = G2Projective::generator();
        let mut checks = 0;
        for case in 0..48 {
            let a = core::array::from_fn::<_, 8, _>(|i| {
                generator
                    .mul(&Fr::from((case * 31 + i + 1) as u64))
                    .to_affine()
            });
            let b = core::array::from_fn::<_, 8, _>(|i| {
                let p = generator
                    .mul(&Fr::from((case * 7 + i + 10001) as u64))
                    .to_affine();
                if i % 2 == 0 {
                    -p
                } else {
                    p
                }
            });
            let norm: [Fq; 8] = core::array::from_fn(|i| {
                let d = b[i].x - a[i].x;
                d.c0.square() + d.c1.square()
            });
            let factor = V::new(&norm.map(|v| v.inverse().unwrap()));
            let (x, y) = finish(
                Interleaved::new(&a.map(|v| v.x)),
                Interleaved::new(&a.map(|v| v.y)),
                Interleaved::new(&b.map(|v| v.x)),
                Interleaved::new(&b.map(|v| v.y)),
                factor,
            );
            let (x, y) = (x.get(), y.get());
            for i in 0..8 {
                let pa = ark_ref::G2Affine::new_unchecked(a2(a[i].x), a2(a[i].y));
                let pb = ark_ref::G2Affine::new_unchecked(a2(b[i].x), a2(b[i].y));
                let sum = ark_ref::G2Affine::from(pa + pb);
                assert_eq!(a2(x[i]), sum.x);
                assert_eq!(a2(y[i]), sum.y);
                checks += 1;
            }
        }
        println!("INDEPENDENT_CURVE_ADDITIONS={checks}");
    }
    #[test]
    fn changing_state_workloads_match_all_output_words() {
        for p in [
            "chain",
            "four-streams",
            "scaled-product",
            "affine-expression",
        ] {
            for n in [1, 2, 7, 31, 128] {
                assert_eq!(run::<Split>(p, n).1, run::<Interleaved>(p, n).1, "{p} {n}");
            }
        }
    }
}
