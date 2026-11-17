//! Experimental scalar bridge; raw R256 input is in the inclusive range 0..2p.
use crate::{Fp, FpConfig};

/// Divide a normalized raw root by 16 without changing its Montgomery domain.
#[inline(always)]
pub fn div16_root<C: FpConfig>(value: Fp<C>) -> Fp<C> {
    let k = value.0[0].wrapping_mul(C::INV) & 15;
    let mut words = [0u64; 5];
    let mut carry = 0u128;
    for i in 0..4 {
        let sum = value.0[i] as u128 + k as u128 * C::P[i] as u128 + carry;
        words[i] = sum as u64;
        carry = sum >> 64;
    }
    words[4] = carry as u64;
    let divided = core::array::from_fn(|i| (words[i] >> 4) | (words[i + 1] << 60));
    let (reduced, borrow) = crate::portable::sub4(divided, C::P);
    Fp::from_montgomery(if borrow { divided } else { reduced })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{Field, PrimeField};
    fn bytes(x: [u64; 4]) -> [u8; 32] {
        let mut b = [0; 32];
        for i in 0..4 {
            b[i * 8..i * 8 + 8].copy_from_slice(&x[i].to_le_bytes());
        }
        b
    }
    fn next(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }
    fn root<C: FpConfig, A: PrimeField>() {
        let p = C::P;
        let twice = crate::portable::add4(p, p).0;
        let edges = [
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(p, [1, 0, 0, 0]).0,
            p,
            crate::portable::add4(p, [1, 0, 0, 0]).0,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            twice,
        ];
        let scale = A::from(16).inverse().unwrap();
        let h_inverse = A::from(2).pow([256u64]).inverse().unwrap();
        let mut seed = 0x583a_16f2_d9c4_b071;
        let mut zeros = 0;
        for round in 0..4103 {
            let x = if round < 7 {
                edges[round]
            } else {
                let mut x = core::array::from_fn(|_| next(&mut seed));
                x[3] &= (1u64 << 63) - 1;
                if !crate::portable::less(x, twice) {
                    x = crate::portable::sub4(x, twice).0;
                }
                x
            };
            let got = div16_root(Fp::<C>::from_montgomery(x));
            assert!(crate::portable::less(got.0, p));
            let want = A::from_le_bytes_mod_order(&bytes(x)) * h_inverse * scale;
            assert_eq!(A::from_le_bytes_mod_order(&bytes(got.to_canonical())), want);
            if want.is_zero() {
                zeros += 1;
                assert!(got.is_zero());
            }
        }
        assert_eq!(zeros, 3);
    }
    #[test]
    fn both_prime_raw_scalar_roots() {
        root::<crate::FqConfig, ark_ref::Fq>();
        root::<crate::FrConfig, ark_ref::Fr>();
    }
}
