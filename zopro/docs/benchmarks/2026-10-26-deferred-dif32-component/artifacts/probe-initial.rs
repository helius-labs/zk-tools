#![allow(unexpected_cfgs)]
use std::{hint::black_box, time::Instant};
use zopro_bn254::{ifma8::Ifma8, Fr, FrConfig};
type V = Ifma8<FrConfig>;

#[inline(always)]
fn reference(values: &mut [V; 32], roots: &[V; 31]) {
    for stage in 0..5 {
        let gap = 16 >> stage;
        for block in (0..32).step_by(2 * gap) {
            for j in 0..gap {
                let low = block + j;
                let high = low + gap;
                let (a, b) = (values[low], values[high]);
                values[low] = a.add(&b);
                values[high] = a.sub(&b).mul(&roots[32 - 2 * gap + j]);
            }
        }
    }
}
#[inline(never)]
fn repeat<const DEFERRED: bool>(values: &mut [V; 32], roots: &[V; 31], iterations: usize) {
    for _ in 0..iterations {
        #[cfg(zopro_deferred_dif32)]
        if DEFERRED {
            V::dif32_deferred(values, |stage, j| roots[32 - 2 * (16 >> stage) + j]);
            continue;
        }
        assert!(!DEFERRED, "candidate missing compile cfg");
        reference(values, roots);
    }
    black_box(values);
}
fn input() -> [V; 32] {
    core::array::from_fn(|row| {
        V::new(&core::array::from_fn(|lane| {
            let mut x = Fr::from((row * 1597 + lane * 173 + 101) as u64);
            for _ in 0..7 {
                x = x.square() + Fr::from(719u64);
            }
            x
        }))
    })
}
fn roots(q: usize, column: usize) -> [V; 31] {
    assert!(q.is_power_of_two() && q <= 16384 && column < q);
    let n = 32 * q;
    let gamma = Fr::TWO_ADIC_ROOT_OF_UNITY.pow(&[(1u64 << 28) / n as u64]);
    core::array::from_fn(|index| {
        let stage = (0..5)
            .find(|s| {
                let gap = 16 >> s;
                index >= 32 - 2 * gap && index < 32 - gap
            })
            .unwrap();
        let gap = 16 >> stage;
        let j = index - (32 - 2 * gap);
        let root = gamma.pow(&[((j * q + column) << stage) as u64]);
        V::new(&[root; 8])
    })
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 5);
    let mode = &args[1];
    let q = args[2].parse::<usize>().unwrap();
    let column = args[3].parse::<usize>().unwrap();
    let n = args[4].parse::<usize>().unwrap();
    assert!(n > 0);
    let roots = black_box(roots(q, column));
    let initial = black_box(input());
    let call = |values: &mut [V; 32], n| match mode.as_str() {
        "reference" => repeat::<false>(values, &roots, n),
        "deferred" => repeat::<true>(values, &roots, n),
        _ => panic!("mode"),
    };
    for _ in 0..3 {
        let mut values = initial;
        call(&mut values, 2000);
        black_box(values);
    }
    let mut values = initial;
    let start = Instant::now();
    call(&mut values, n);
    black_box(&values);
    let ns = start.elapsed().as_nanos();
    let output = values
        .iter()
        .flat_map(|v| v.get().map(|x| x.to_canonical()))
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::json!({"mode":mode,"q":q,"column":column,"iterations":n,"elements_per_tile":256,"elapsed_ns":ns,"ns_per_element":ns as f64/(n*256)as f64,"canonical_words":output})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{AdditiveGroup, BigInt, Field, PrimeField};
    use num_bigint::BigUint;
    use zopro_bn254::{ifma::Ifma, FpConfig};
    const MASK: u64 = (1u64 << 52) - 1;
    fn big(x: [u64; 4]) -> BigUint {
        BigUint::from_bytes_le(&x.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())
    }
    fn digits(x: &BigUint) -> [u64; 5] {
        core::array::from_fn(|j| {
            ((x >> (52 * j)) & BigUint::from(MASK))
                .to_u64_digits()
                .first()
                .copied()
                .unwrap_or(0)
        })
    }
    fn value(x: [u64; 5]) -> BigUint {
        x.into_iter()
            .enumerate()
            .fold(BigUint::from(0u8), |s, (i, x)| {
                s + (BigUint::from(x) << (52 * i))
            })
    }
    fn raw_make<T: Copy, const L: usize>(values: &[BigUint; L]) -> T {
        assert_eq!(std::mem::size_of::<T>(), 40 * L);
        let d: [[u64; L]; 5] =
            core::array::from_fn(|j| core::array::from_fn(|i| digits(&values[i])[j]));
        // Test-only bit construction: one nonzero-sized field is the vector
        // array; all SIMD bit patterns are valid and every digit is normalized.
        unsafe { std::mem::transmute_copy(&d) }
    }
    fn raw_read<T: Copy, const L: usize>(x: &T) -> [BigUint; L] {
        assert_eq!(std::mem::size_of::<T>(), 40 * L);
        // Test-only inspection does not call get/reduce, so malformed raw
        // output cannot be hidden by canonical conversion.
        let d: [[u64; L]; 5] = unsafe { std::mem::transmute_copy(x) };
        assert!(d.iter().flatten().all(|x| *x <= MASK));
        core::array::from_fn(|i| value(core::array::from_fn(|j| d[j][i])))
    }
    struct Model {
        p: BigUint,
        r: BigUint,
        negative_inverse: BigUint,
        coeff_max: u128,
        products: usize,
        stage_max: BigUint,
    }
    impl Model {
        fn new() -> Self {
            let p = big(Fr::MODULUS);
            let r = BigUint::from(1u8) << 260;
            assert!((&p << 6) < r);
            let mask = &r - BigUint::from(1u8);
            let mut inverse = BigUint::from(1u8);
            for _ in 0..9 {
                inverse = (&inverse * (&r + BigUint::from(2u8) - (&p * &inverse & &mask))) & &mask;
            }
            assert_eq!((&p * &inverse) & &mask, BigUint::from(1u8));
            Self {
                p,
                r: r.clone(),
                negative_inverse: r - inverse,
                coeff_max: 0,
                products: 0,
                stage_max: BigUint::from(0u8),
            }
        }
        fn montgomery(&mut self, a: &BigUint, b: &BigUint) -> BigUint {
            let t = a * b;
            assert!(t < &self.p * &self.r);
            let mask = &self.r - BigUint::from(1u8);
            let q = (&t * &self.negative_inverse) & &mask;
            let expected = (&t + q * &self.p) >> 260;
            assert!(expected < (&self.p << 1));
            // A separate bounded coefficient implementation checks every
            // operation in the accepted uniform cancellation-carry CIOS.
            let ad = digits(a);
            let bd = digits(b);
            let p = digits(&self.p);
            let inv = (&self.negative_inverse & BigUint::from(MASK)).to_u64_digits()[0] as u128;
            assert_eq!(inv as u64, FrConfig::INV & MASK);
            let mut c = [0u128; 6];
            let m52 = MASK as u128;
            for bi in bd {
                for j in 0..5 {
                    let z = ad[j] as u128 * bi as u128;
                    c[j] += z & m52;
                    c[j + 1] += z >> 52;
                    self.coeff_max = self.coeff_max.max(c[j]).max(c[j + 1]);
                }
                let m = (c[0] * inv) & m52;
                let low = ((c[0] + m52) >> 52) + ((m * p[0] as u128) >> 52);
                for j in 1..5 {
                    let z = m * p[j] as u128;
                    c[j] += z & m52;
                    c[j + 1] += z >> 52;
                    self.coeff_max = self.coeff_max.max(c[j]).max(c[j + 1]);
                }
                c[1] += low;
                self.coeff_max = self.coeff_max.max(c[1]);
                c = [c[1], c[2], c[3], c[4], c[5], 0];
            }
            for j in 0..4 {
                c[j + 1] += c[j] >> 52;
                c[j] &= m52;
                self.coeff_max = self.coeff_max.max(c[j + 1]);
            }
            assert!(c[..5].iter().all(|v| *v <= m52));
            assert!(self.coeff_max < (32u128 << 52));
            assert_eq!(value(core::array::from_fn(|j| c[j] as u64)), expected);
            self.products += 1;
            expected
        }
        fn transform<const L: usize>(
            &mut self,
            values: &mut [[BigUint; L]; 32],
            roots: &[[BigUint; L]; 31],
        ) {
            for stage in 0..5 {
                let gap = 16 >> stage;
                let bound = &self.p << (stage + 1);
                for block in (0..32).step_by(2 * gap) {
                    for j in 0..gap {
                        let low = block + j;
                        let high = low + gap;
                        for lane in 0..L {
                            let a = values[low][lane].clone();
                            let b = values[high][lane].clone();
                            assert!(a < bound && b < bound);
                            let sum = &a + &b;
                            let difference = &bound + &a - &b;
                            assert!(sum < (&bound << 1) && difference < (&bound << 1));
                            assert!(sum < self.r && difference < self.r);
                            if sum > self.stage_max {
                                self.stage_max = sum.clone();
                            }
                            if difference > self.stage_max {
                                self.stage_max = difference.clone();
                            }
                            let root = &roots[32 - 2 * gap + j][lane];
                            assert!(root < &self.p);
                            values[high][lane] = self.montgomery(&difference, root);
                            values[low][lane] = sum;
                        }
                    }
                }
            }
            for (index, row) in values.iter_mut().enumerate() {
                let shifts = (index | 32).trailing_zeros();
                let bound = &self.p << (shifts + 1);
                for x in row {
                    assert!(*x < bound);
                    for shift in (1..=shifts).rev() {
                        let m = &self.p << shift;
                        if *x >= m {
                            *x -= m;
                        }
                    }
                    assert!(*x < (&self.p << 1));
                }
            }
        }
    }
    fn random_big(seed: &mut u64) -> BigUint {
        let words = core::array::from_fn(|_| {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            *seed
        });
        big(words)
    }
    macro_rules! exact_oracle {($name:ident,$vector:ty,$lanes:expr)=>{
        #[test]fn $name(){type E=$vector;const L:usize=$lanes;let mut model=Model::new();let mut seed=0xf751_229a_ab48_7d17;let mut checked=0;
            for case in 0..128 {let p=model.p.clone();let mut values:[[BigUint;L];32]=core::array::from_fn(|row|core::array::from_fn(|lane|match case {0=>BigUint::from(0u8),1=>(&p<<1)-BigUint::from(1u8),2 if (row+lane)%2==0=>p.clone(),3=>BigUint::from(1u8)<<(case+row+lane)%254,_=>random_big(&mut seed)%(&p<<1)}));let roots:[[BigUint;L];31]=core::array::from_fn(|row|core::array::from_fn(|lane|match case%5 {0=>BigUint::from(0u8),1=>&p-BigUint::from(1u8),2=>BigUint::from(1u8),3 if(row+lane)%2==0=>&p-BigUint::from(1u8),_=>random_big(&mut seed)%&p}));
                let mut actual:[E;32]=core::array::from_fn(|i|raw_make::<E,L>(&values[i]));let roots_actual:[E;31]=core::array::from_fn(|i|raw_make::<E,L>(&roots[i]));E::dif32_deferred(&mut actual,|stage,j|roots_actual[32-2*(16>>stage)+j]);model.transform(&mut values,&roots);
                for i in 0..32 {let raw=raw_read::<E,L>(&actual[i]);assert_eq!(raw,values[i],"raw case{case} row{i}");assert!(raw.iter().all(|x|x<(&p<<1)));assert_eq!(actual[i].compact().expand().get(),actual[i].get());checked+=L;}
            }println!("RAW_MONTGOMERY_OUTPUTS={checked} CIOS_PRODUCTS={} CIOS_COEFF_MAX={} INTERMEDIATE_MAX_BITS={}",model.products,model.coeff_max,model.stage_max.bits());
        }
    }}
    exact_oracle!(exact_integer_four_lanes, Ifma<FrConfig>, 4);
    exact_oracle!(exact_integer_eight_lanes, Ifma8<FrConfig>, 8);
    fn af(x: Fr) -> ark_ref::Fr {
        ark_ref::Fr::new_unchecked(BigInt(x.0))
    }
    macro_rules! fourier_oracle {
        ($name:ident,$vector:ty,$lanes:expr) => {
            #[test]
            fn $name() {
                type E = $vector;
                const L: usize = $lanes;
                let mut checked = 0;
                for (q, column) in [(1usize, 0usize), (256, 173), (16384, 8191)] {
                    let gamma = Fr::TWO_ADIC_ROOT_OF_UNITY.pow(&[(1u64 << 28) / (32 * q) as u64]);
                    let omega = af(gamma).pow([q as u64]);
                    let twist = af(gamma).pow([column as u64]);
                    let root_values: [E; 31] = core::array::from_fn(|i| {
                        let stage = (0..5)
                            .find(|s| {
                                let gap = 16 >> s;
                                i >= 32 - 2 * gap && i < 32 - gap
                            })
                            .unwrap();
                        let gap = 16 >> stage;
                        let j = i - (32 - 2 * gap);
                        E::new(&[gamma.pow(&[((j * q + column) << stage) as u64]); L])
                    });
                    for r in &root_values {
                        assert!(raw_read::<E, L>(r).iter().all(|x| x < &big(Fr::MODULUS)));
                    }
                    for kind in 0..5 {
                        let input: [[Fr; L]; 32] = core::array::from_fn(|row| {
                            core::array::from_fn(|lane| {
                                let x =
                                    Fr::from((row * 1597 + lane * 19 + 101) as u64).pow(&[65537]);
                                match kind {
                                    0 => Fr::ZERO,
                                    1 => Fr::ONE,
                                    2 => {
                                        if row == (lane * 7 % 32) {
                                            x
                                        } else {
                                            Fr::ZERO
                                        }
                                    }
                                    3 => x,
                                    _ => -x,
                                }
                            })
                        });
                        let mut actual = input.map(|row| E::new(&row));
                        E::dif32_deferred(&mut actual, |stage, j| {
                            root_values[32 - 2 * (16 >> stage) + j]
                        });
                        for index in 0..32 {
                            let k = index.reverse_bits() >> (usize::BITS - 5);
                            let w = omega.pow([k as u64]);
                            let scale = twist.pow([k as u64]);
                            let got = actual[index].get();
                            let raw = raw_read::<E, L>(&actual[index]);
                            for lane in 0..L {
                                let mut expected = ark_ref::Fr::ZERO;
                                let mut power = ark_ref::Fr::ONE;
                                for row in &input {
                                    expected += af(row[lane]) * power;
                                    power *= w;
                                }
                                expected *= scale;
                                assert_eq!(
                                    af(got[lane]),
                                    expected,
                                    "q{q} col{column} kind{kind} k{k} lane{lane}"
                                );
                                assert!(raw[lane] < (&big(Fr::MODULUS) << 1));
                                checked += 1;
                            }
                        }
                    }
                }
                println!("DIRECT_FOURIER_FIELD_OUTPUTS={checked}");
            }
        };
    }
    fourier_oracle!(direct_fourier_four_lanes, Ifma<FrConfig>, 4);
    fourier_oracle!(direct_fourier_eight_lanes, Ifma8<FrConfig>, 8);
    #[test]
    fn repeated_tiles_match_existing_field_arithmetic() {
        let roots = roots(256, 173);
        let mut a = input();
        let mut b = a;
        for step in 0..20 {
            repeat::<false>(&mut a, &roots, 1);
            repeat::<true>(&mut b, &roots, 1);
            for i in 0..32 {
                assert_eq!(a[i].get(), b[i].get(), "iteration{step} row{i}");
            }
        }
    }
    #[test]
    #[should_panic(expected = "uniform R260")]
    fn mixed_representation_is_rejected() {
        type M = Ifma8<FrConfig, true>;
        let mut values = [M::one(); 32];
        M::dif32_deferred(&mut values, |_, _| M::one());
    }
}
