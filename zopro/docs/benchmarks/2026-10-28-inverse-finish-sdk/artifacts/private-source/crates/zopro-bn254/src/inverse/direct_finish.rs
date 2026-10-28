// Direct canonical residue of the bounded signed Euclidean linear form.
// All operands are ordinary integer words, not Montgomery operations.
#[inline(always)]
fn direct_finish_reduce<C: FpConfig>(u: &[u64; 4], v: &[u64; 4], f: i64, g: i64) -> [u64; 4] {
    debug_assert!(f != i64::MIN && g != i64::MIN);
    debug_assert!(f == 0 || g == 0 || (f < 0) != (g < 0));
    debug_assert!(C::P[3] as u128 * 3 > 1u128 << 63 && C::P[3] < 1u64 << 63);
    let mut a = linear(u, f, v, g);
    let negative = (a[4] as i64) < 0;
    if negative {
        let mut carry = 1u128;
        for limb in &mut a {
            carry += (!*limb) as u128;
            *limb = carry as u64;
            carry >>= 64;
        }
    }
    // d is odd/not a power of two in both supported fields; thus MAX/d
    // equals floor(2^128/d). This reciprocal is compile-time constant.
    let mu = const { u128::MAX / (C::P[3] as u128) };
    let (m0, m1) = (mu as u64, (mu >> 64) as u64);
    let p00 = (a[3] as u128) * (m0 as u128);
    let p01 = (a[3] as u128) * (m1 as u128);
    let p10 = (a[4] as u128) * (m0 as u128);
    let p11 = (a[4] as u128) * (m1 as u128);
    let mid = (p00 >> 64) + (p01 as u64 as u128) + (p10 as u64 as u128);
    let high = (mid >> 64) + (p01 >> 64) + (p10 >> 64) + p11;
    debug_assert!(high <= u64::MAX as u128);
    let q = high as u64;
    let mut product = [0u64; 5];
    let mut carry = 0u128;
    for i in 0..4 {
        let t = (q as u128) * (C::P[i] as u128) + carry;
        product[i] = t as u64;
        carry = t >> 64;
    }
    product[4] = carry as u64;
    let mut borrow = false;
    for i in 0..5 {
        let (t, b0) = a[i].overflowing_sub(product[i]);
        let (t, b1) = t.overflowing_sub(u64::from(borrow));
        a[i] = t;
        borrow = b0 || b1;
    }
    for _ in 0..3 {
        if (a[4] as i64) >= 0 {
            break;
        }
        let mut carry = 0u128;
        for i in 0..4 {
            carry += a[i] as u128 + C::P[i] as u128;
            a[i] = carry as u64;
            carry >>= 64;
        }
        a[4] = a[4].wrapping_add(carry as u64);
    }
    debug_assert_eq!(a[4], 0);
    let mut result = portable::reduce_once([a[0], a[1], a[2], a[3]], C::P);
    if negative && result != [0; 4] {
        result = portable::sub4(C::P, result).0;
    }
    result
}

#[cfg(test)]
mod direct_finish_tests {
    use super::*;
    use ark_ff::{BigInteger, PrimeField};
    fn check<C: FpConfig, F: PrimeField>() {
        fn words(x: [u64; 4]) -> [u8; 32] {
            let mut b = [0; 32];
            for (i, w) in x.into_iter().enumerate() {
                b[i * 8..i * 8 + 8].copy_from_slice(&w.to_le_bytes());
            }
            b
        }
        let from = |x: [u64; 4]| F::from_le_bytes_mod_order(&words(x));
        let signed = |x: i64| {
            let a = F::from(x.unsigned_abs());
            if x < 0 {
                -a
            } else {
                a
            }
        };
        let mut cases = 0;
        let p1 = portable::sub4(C::P, [1, 0, 0, 0]).0;
        let half =
            core::array::from_fn(|i| (C::P[i] >> 1) | if i < 3 { C::P[i + 1] << 63 } else { 0 });
        for u in [[0; 4], [1, 0, 0, 0], p1, half] {
            for v in [[0; 4], [1, 0, 0, 0], p1, half] {
                for f in [0, 1, 2, 3, i32::MAX as i64, i64::MAX - 1, i64::MAX] {
                    for g in [0, -1, -2, -3, -(i32::MAX as i64), -i64::MAX + 1, -i64::MAX] {
                        for (f, g) in [(f, g), (-f, -g)] {
                            let expected = from(u) * signed(f) + from(v) * signed(g);
                            let got = direct_finish_reduce::<C>(&u, &v, f, g);
                            assert_eq!(got.as_slice(), expected.into_bigint().as_ref());
                            cases += 1;
                        }
                    }
                }
            }
        }
        let mut seed = 0x494e56464e5348u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for _ in 0..32768 {
            let mut bytes = [0u8; 64];
            for chunk in bytes.chunks_mut(8) {
                chunk.copy_from_slice(&next().to_le_bytes());
            }
            let u = Fp::<C>::from_random_bytes(&bytes).to_canonical();
            for chunk in bytes.chunks_mut(8) {
                chunk.copy_from_slice(&next().to_le_bytes());
            }
            let v = Fp::<C>::from_random_bytes(&bytes).to_canonical();
            let f = (next() & i64::MAX as u64) as i64;
            let g = -((next() & i64::MAX as u64) as i64);
            let expected = from(u) * signed(f) + from(v) * signed(g);
            let got = direct_finish_reduce::<C>(&u, &v, f, g);
            assert_eq!(got.as_slice(), expected.into_bigint().as_ref());
            cases += 1;
        }
        eprintln!("DIRECT_FINISH_RESIDUES={cases}");
        let _ = F::ONE.into_bigint().to_bytes_le();
    }
    #[test]
    fn direct_signed_finish_matches_independent_fullwidth_fields() {
        check::<crate::FqConfig, ark_ref::Fq>();
        check::<crate::FrConfig, ark_ref::Fr>();
    }
}
