//! Batched binary GCD: carry 31 low bits and 33 leading bits through each
//! small GCD, then apply its matrix to the full integers and coefficients.
//! Algorithm: Thomas Pornin, Optimized Binary GCD for Modular Inversion.
//! https://github.com/pornin/bingcd/blob/main/doc/bingcd.pdf
use crate::{portable, Fp, FpConfig};

const BITS: usize = 31;
const MASK: u64 = (1 << BITS) - 1;

#[inline(always)]
fn bit_len(x: &[u64; 4]) -> usize {
    for i in (0..4).rev() {
        if x[i] != 0 {
            return i * 64 + 64 - x[i].leading_zeros() as usize;
        }
    }
    0
}

#[inline(always)]
fn approximate(x: &[u64; 4], bits: usize) -> u64 {
    if bits <= 64 {
        return x[0];
    }
    let shift = bits - 33;
    let (word, offset) = (shift / 64, shift % 64);
    let mut high = x[word] >> offset;
    if offset != 0 && word < 3 {
        high |= x[word + 1] << (64 - offset);
    }
    (high << BITS) | (x[0] & MASK)
}

/// The signed 320-bit linear combination. Coefficients fit signed 33 bits,
/// so each 128-bit intermediate and the signed high word have ample room.
#[inline(always)]
fn linear(a: &[u64; 4], f: i64, b: &[u64; 4], g: i64) -> [u64; 5] {
    let mut out = [0; 5];
    let mut carry = 0i128;
    for j in 0..4 {
        let t = (a[j] as i128) * (f as i128) + (b[j] as i128) * (g as i128) + carry;
        out[j] = t as u64;
        carry = t >> 64;
    }
    out[4] = carry as u64;
    out
}

#[inline(always)]
fn integer_update(a: &[u64; 4], b: &[u64; 4], f: &mut i64, g: &mut i64) -> [u64; 4] {
    let mut t = linear(a, *f, b, *g);
    if (t[4] as i64) < 0 {
        *f = -*f;
        *g = -*g;
        let mut carry = 1u128;
        for limb in &mut t {
            carry += (!*limb) as u128;
            *limb = carry as u64;
            carry >>= 64;
        }
    }
    debug_assert_eq!(t[0] & MASK, 0, "GCD matrix must cancel its low bits");
    debug_assert_eq!(t[4] >> BITS, 0);
    core::array::from_fn(|j| (t[j] >> BITS) | (t[j + 1] << (64 - BITS)))
}

/// (f*a+g*b)/2^31 modulo p. One radix-31 Montgomery step makes the
/// numerator divisible before shifting; its quotient lies in (-p,2p).
#[inline(always)]
fn coefficient_update<C: FpConfig>(a: &[u64; 4], b: &[u64; 4], f: i64, g: i64) -> [u64; 4] {
    let mut t = linear(a, f, b, g);
    let m = t[0].wrapping_mul(C::INV) & MASK;
    let mut carry = 0u128;
    for j in 0..4 {
        let v = (t[j] as u128) + (m as u128) * (C::P[j] as u128) + carry;
        t[j] = v as u64;
        carry = v >> 64;
    }
    t[4] = t[4].wrapping_add(carry as u64);
    debug_assert_eq!(t[0] & MASK, 0);
    let mut result = core::array::from_fn(|j| (t[j] >> BITS) | (t[j + 1] << (64 - BITS)));
    if (t[4] as i64) < 0 {
        result = portable::add4(result, C::P).0;
    }
    portable::reduce_once(result, C::P)
}

pub(crate) fn inverse<C: FpConfig>(x: &Fp<C>) -> Option<Fp<C>> {
    if x.0 == [0; 4] {
        return None;
    }
    let (mut a, mut b) = (x.0, C::P);
    let (mut u, mut v) = (C::R2, [0; 4]);
    let mut rounds = 0;
    while a != [0; 4] {
        let bits = bit_len(&a).max(bit_len(&b));
        let (mut small_a, mut small_b) = (approximate(&a, bits), approximate(&b, bits));
        let (mut f0, mut g0, mut f1, mut g1) = (1i64, 0i64, 0i64, 1i64);
        for _ in 0..BITS {
            if small_a & 1 != 0 {
                if small_a < small_b {
                    core::mem::swap(&mut small_a, &mut small_b);
                    core::mem::swap(&mut f0, &mut f1);
                    core::mem::swap(&mut g0, &mut g1);
                }
                small_a -= small_b;
                f0 -= f1;
                g0 -= g1;
            }
            small_a >>= 1;
            f1 *= 2;
            g1 *= 2;
        }
        let old_a = a;
        a = integer_update(&old_a, &b, &mut f0, &mut g0);
        b = integer_update(&old_a, &b, &mut f1, &mut g1);
        let old_u = u;
        u = coefficient_update::<C>(&old_u, &v, f0, g0);
        v = coefficient_update::<C>(&old_u, &v, f1, g1);
        rounds += 1;
        assert!(rounds <= 32, "batched GCD did not converge");
    }
    debug_assert_eq!(b, [1, 0, 0, 0]);
    let result = Fp::from_montgomery(v);
    debug_assert_eq!(*x * result, Fp::ONE);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check<C: FpConfig>() {
        assert_eq!(inverse(&Fp::<C>::ZERO), None);
        let mut values = vec![Fp::<C>::ONE, -Fp::<C>::ONE];
        let mut power = Fp::<C>::ONE;
        for _ in 0..256 {
            values.push(power);
            values.push(-power);
            power = power.double();
        }
        let mut seed = 0x876543219abcdefu64;
        for _ in 0..10000 {
            let mut bytes = [0; 64];
            for chunk in bytes.chunks_mut(8) {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                chunk.copy_from_slice(&seed.to_le_bytes());
            }
            values.push(Fp::<C>::from_random_bytes(&bytes));
        }
        for x in values {
            if x == Fp::ZERO {
                continue;
            }
            let got = inverse(&x).unwrap();
            assert_eq!(got * x, Fp::ONE);
            assert_eq!(got, x.pow(&portable::sub4(C::P, [2, 0, 0, 0]).0));
        }
    }
    #[test]
    fn fq() {
        check::<crate::FqConfig>();
    }
    #[test]
    fn fr() {
        check::<crate::FrConfig>();
    }
}
