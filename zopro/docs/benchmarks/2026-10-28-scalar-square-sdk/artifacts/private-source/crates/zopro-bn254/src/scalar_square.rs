//! Four-limb symmetric square and Montgomery reduction.
//! Adapted from arkworks-rs/algebra v0.5.0, ff/src/fields/models/fp/
//! montgomery_backend.rs ordinary Rust square fallback, by arkworks contributors.
//! Upstream license: MIT OR Apache-2.0; see the accompanying upstream MIT license.
use crate::{portable, FpConfig};

#[inline(always)]
fn mac(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let t = a as u128 + b as u128 * c as u128 + *carry as u128;
    *carry = (t >> 64) as u64;
    t as u64
}

#[inline(always)]
fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let t = *a as u128 + b as u128 + carry as u128;
    *a = t as u64;
    (t >> 64) as u64
}

#[inline(always)]
fn wide(a: [u64; 4]) -> [u64; 8] {
    let mut r = [0u64; 8];
    let mut carry = 0;
    for i in 0..3 {
        for j in (i + 1)..4 {
            r[i + j] = mac(r[i + j], a[i], a[j], &mut carry);
        }
        r[4 + i] = carry;
        carry = 0;
    }
    r[7] = r[6] >> 63;
    for i in (2..=6).rev() {
        r[i] = (r[i] << 1) | (r[i - 1] >> 63);
    }
    r[1] <<= 1;
    for i in 0..4 {
        r[2 * i] = mac(r[2 * i], a[i], a[i], &mut carry);
        carry = adc(&mut r[2 * i + 1], 0, carry);
    }
    debug_assert_eq!(carry, 0);
    r
}

/// Canonical BN254 Montgomery input and output; p < 2^254.
#[inline(always)]
pub(crate) fn square<C: FpConfig>(a: [u64; 4]) -> [u64; 4] {
    debug_assert!(portable::less(a, C::P));
    debug_assert!(C::P[3] < 1u64 << 62);
    let mut r = wide(a);
    let mut carry2 = 0;
    for i in 0..4 {
        let k = r[i].wrapping_mul(C::INV);
        let mut carry = 0;
        let _ = mac(r[i], k, C::P[0], &mut carry);
        for j in 1..4 {
            r[i + j] = mac(r[i + j], k, C::P[j], &mut carry);
        }
        carry2 = adc(&mut r[4 + i], carry, carry2);
    }
    debug_assert_eq!(carry2, 0);
    portable::reduce_once([r[4], r[5], r[6], r[7]], C::P)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fp, FqConfig, FrConfig};
    use ark_ff::{BigInt, BigInteger, Field};

    fn next(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }

    #[test]
    fn full_512_bit_square_matches_independent_product() {
        let check = |a: [u64; 4]| {
            let (lo, hi) = BigInt(a).mul(&BigInt(a));
            let expected: [u64; 8] =
                core::array::from_fn(|i| if i < 4 { lo.0[i] } else { hi.0[i - 4] });
            assert_eq!(wide(a), expected, "input {a:?}");
        };
        check([0; 4]);
        check([u64::MAX; 4]);
        for bit in 0..256 {
            let mut a = [0; 4];
            a[bit / 64] = 1 << (bit % 64);
            check(a);
            check(a.map(|x| !x));
        }
        for i in 0..4 {
            let mut a = [u64::MAX; 4];
            a[i] = 0;
            check(a);
        }
        let mut seed = 0x53ac_f724_de90_318b;
        for _ in 0..16384 {
            check(core::array::from_fn(|_| next(&mut seed)));
        }
    }

    #[test]
    fn montgomery_square_and_fifth_match_independent_fields() {
        macro_rules! field {
            ($config:ty, $ark:ty) => {{
                let mut inputs = vec![
                    [0; 4],
                    <$config>::R,
                    portable::sub4(<$config>::P, [1, 0, 0, 0]).0,
                ];
                for bit in 0..254 {
                    let mut a = [0; 4];
                    a[bit / 64] = 1 << (bit % 64);
                    if portable::less(a, <$config>::P) {
                        inputs.push(a);
                        inputs.push(portable::sub4(<$config>::P, a).0);
                    }
                }
                let mut seed = 0x692e_07a1_d845_39cf;
                for _ in 0..8192 {
                    let mut a = core::array::from_fn(|_| next(&mut seed));
                    a[3] &= (1 << 62) - 1;
                    inputs.push(portable::reduce_once(a, <$config>::P));
                }
                for a in inputs {
                    let x = Fp::<$config>::from_montgomery(a);
                    let reference = <$ark>::new_unchecked(BigInt(a));
                    let result = square::<$config>(a);
                    assert_eq!(result, reference.square().0 .0);
                    assert_eq!(result, (x * x).0);
                    assert_eq!(result, x.square().0);
                    assert!(portable::less(result, <$config>::P));
                    let mut y = x;
                    let mut r = reference;
                    for _ in 0..8 {
                        y = Fp::from_montgomery(square::<$config>(y.0));
                        r = r.square();
                        assert_eq!(y.0, r.0 .0);
                        let y2 = Fp::<$config>::from_montgomery(square::<$config>(y.0));
                        let fifth = Fp::<$config>::from_montgomery(square::<$config>(y2.0)) * y;
                        assert_eq!(fifth.0, (r.square().square() * r).0 .0);
                        assert!(portable::less(fifth.0, <$config>::P));
                    }
                }
            }};
        }
        field!(FqConfig, ark_ref::Fq);
        field!(FrConfig, ark_ref::Fr);
    }
}
