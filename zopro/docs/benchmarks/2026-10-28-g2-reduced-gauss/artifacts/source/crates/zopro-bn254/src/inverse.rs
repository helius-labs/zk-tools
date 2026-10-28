//! Batched binary GCD: carry 31 low bits and 33 leading bits through each
//! small GCD, then apply its matrix to the full integers and coefficients.
//! Algorithm: Thomas Pornin, Optimized Binary GCD for Modular Inversion.
//! https://github.com/pornin/bingcd/blob/main/doc/bingcd.pdf
use crate::{portable, Fp, FpConfig};
use core::hint::select_unpredictable;

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

/// The signed 320-bit linear combination. Coefficients have row L1 norm at most 2^62,
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

/// (f*a+g*b)/2^SHIFT modulo p. A Montgomery step cancels the
/// low SHIFT bits. A matrix row's L1 norm is at most 2^SHIFT,
/// so the quotient lies in (-p,2p). SHIFT is 31 or 62.
#[inline(always)]
fn coefficient_update<C: FpConfig, const SHIFT: usize>(
    a: &[u64; 4],
    b: &[u64; 4],
    f: i64,
    g: i64,
) -> [u64; 4] {
    let mut t = linear(a, f, b, g);
    let m = t[0].wrapping_mul(C::INV) & ((1u64 << SHIFT) - 1);
    let mut carry = 0u128;
    for j in 0..4 {
        let v = (t[j] as u128) + (m as u128) * (C::P[j] as u128) + carry;
        t[j] = v as u64;
        carry = v >> 64;
    }
    t[4] = t[4].wrapping_add(carry as u64);
    debug_assert_eq!(t[0] & ((1u64 << SHIFT) - 1), 0);
    let mut result = core::array::from_fn(|j| (t[j] >> SHIFT) | (t[j + 1] << (64 - SHIFT)));
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
    // Compose pairs of integer-update matrices before updating modular
    // coefficients. Each row has L1 norm <=2^31, hence the product's
    // rows have norm <=2^62 and fit i64 without losing their signs.
    let mut pending = false;
    let mut matrix = [1i64, 0, 0, 1];
    while a != [0; 4] {
        let bits = bit_len(&a).max(bit_len(&b));
        // Exact one-word operands finish with ordinary Euclid, applying
        // any pending matrix first to preserve the coefficient invariant.
        if bits <= 64 {
            if pending {
                let old_u = u;
                u = coefficient_update::<C, 31>(&old_u, &v, matrix[0], matrix[1]);
                v = coefficient_update::<C, 31>(&old_u, &v, matrix[2], matrix[3]);
            }
            return Some(finish::<C>(a[0], b[0], u, v));
        }
        #[cfg(not(zopro_gcd_balanced_runs))]
        let (mut f0, mut g0, mut f1, mut g1) = {
        let (mut small_a, mut small_b) = (approximate(&a, bits), approximate(&b, bits));
        let (mut f0, mut g0, mut f1, mut g1) = (1i64, 0i64, 0i64, 1i64);
        for _ in 0..BITS {
            // Odd steps set A to |A-B|/2 and B to min(A,B); even
            // steps halve A. Compute the difference before the selections
            // so A does not depend on the updated B, which remains odd.
            let odd = small_a & 1;
            let smaller = small_a < small_b;
            let swap = odd & u64::from(smaller);
            let previous_a = small_a;
            let difference = small_a.wrapping_sub(small_b);
            let magnitude = select_unpredictable(smaller, difference.wrapping_neg(), difference);
            small_a = select_unpredictable(odd != 0, magnitude, small_a) >> 1;
            small_b = select_unpredictable(swap != 0, previous_a, small_b);
            let (previous_f0, previous_g0) = (f0, g0);
            f0 = select_unpredictable(swap != 0, f1, f0);
            f1 = select_unpredictable(swap != 0, previous_f0, f1);
            g0 = select_unpredictable(swap != 0, g1, g0);
            g1 = select_unpredictable(swap != 0, previous_g0, g1);
            let odd_mask = 0u64.wrapping_sub(odd);
            f0 -= f1 & odd_mask as i64;
            g0 -= g1 & odd_mask as i64;
            f1 *= 2;
            g1 *= 2;
        }
            (f0, g0, f1, g1)
        };
        #[cfg(zopro_gcd_balanced_runs)]
        let (_, _, mut f0, mut g0, mut f1, mut g1) =
            zero_run_matrix(approximate(&a, bits), approximate(&b, bits));
        let old_a = a;
        a = integer_update(&old_a, &b, &mut f0, &mut g0);
        b = integer_update(&old_a, &b, &mut f1, &mut g1);
        if pending {
            let [m00, m01, m10, m11] = matrix;
            let combined = [
                f0 * m00 + g0 * m10,
                f0 * m01 + g0 * m11,
                f1 * m00 + g1 * m10,
                f1 * m01 + g1 * m11,
            ];
            debug_assert!(combined[0].unsigned_abs() + combined[1].unsigned_abs() <= 1 << 62);
            debug_assert!(combined[2].unsigned_abs() + combined[3].unsigned_abs() <= 1 << 62);
            let old_u = u;
            u = coefficient_update::<C, 62>(&old_u, &v, combined[0], combined[1]);
            v = coefficient_update::<C, 62>(&old_u, &v, combined[2], combined[3]);
            pending = false;
        } else {
            matrix = [f0, g0, f1, g1];
            pending = true;
        }
        rounds += 1;
        assert!(rounds <= 32, "batched GCD did not converge");
    }
    if pending {
        v = coefficient_update::<C, 31>(&u, &v, matrix[2], matrix[3]);
    }
    debug_assert_eq!(b, [1, 0, 0, 0]);
    let result = Fp::from_montgomery(v);
    debug_assert_eq!(*x * result, Fp::ONE);
    Some(result)
}

#[inline]
fn finish<C: FpConfig>(a: u64, b: u64, u: [u64; 4], v: [u64; 4]) -> Fp<C> {
    if b == 1 {
        return Fp::from_montgomery(v);
    }
    let (mut r0, mut r1) = (a, b);
    let (mut f0, mut f1) = (1i64, 0i64);
    let (mut g0, mut g1) = (0i64, 1i64);
    loop {
        let q = r0 / r1;
        let rem = r0 % r1;
        // Skip the zero remainder's coefficients, whose magnitude could be 2^64.
        // Every nonzero remainder's coefficients fit signed63 bits.
        if rem == 0 {
            debug_assert_eq!(r1, 1);
            break;
        }
        (r0, r1) = (r1, rem);
        (f0, f1) = (f1, f0 - (q as i64) * f1);
        (g0, g1) = (g1, g0 - (q as i64) * g1);
    }
    let (f0, g0) = (f1, g1);

    // The two Bezout coefficients have opposite signs and magnitude <2^63.
    // L=f*u+g*v lies in (-2^63*p,2^63*p). A radix64 Montgomery
    // reduction returns L/2^64 in (-p/2,3p/2), needing at most one correction.
    let mut t = linear(&u, f0, &v, g0);
    let m = t[0].wrapping_mul(C::INV);
    let mut carry = 0u128;
    for j in 0..4 {
        let z = t[j] as u128 + (m as u128) * (C::P[j] as u128) + carry;
        t[j] = z as u64;
        carry = z >> 64;
    }
    t[4] = t[4].wrapping_add(carry as u64);
    debug_assert_eq!(t[0], 0);
    let mut result = [t[1], t[2], t[3], t[4]];
    if (t[4] as i64) < 0 {
        result = portable::add4(result, C::P).0;
    }
    result = portable::reduce_once(result, C::P);
    let scale = Fp::<C>::from_montgomery(const { portable::power_of_two(320, C::P) });
    Fp::from_montgomery(result) * scale
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
        // Raw Montgomery powers include matrices with a coefficient
        // equal to +2^31 after a run of 31 even steps.
        for bit in 0..254 {
            let mut words = [0; 4];
            words[bit / 64] = 1 << (bit % 64);
            values.push(Fp::from_montgomery(words));
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
    fn one_word_finish() {
        fn check<C: FpConfig>() {
            let expected = Fp::<C>::from(7).pow(&portable::sub4(C::P, [2, 0, 0, 0]).0);
            let field =
                |x: u64| Fp::<C>::from_montgomery([x, 0, 0, 0]) * Fp::<C>::from_montgomery(C::R2);
            let mut pairs = vec![
                (1, u64::MAX),
                (u64::MAX, 1),
                (2, u64::MAX),
                (u64::MAX, u64::MAX - 1),
                (u64::MAX - 1, u64::MAX),
                (1 << 63, (1 << 63) - 1),
                ((1 << 63) - 1, 1 << 63),
                (7540113804746346429, 12200160415121876738),
                (12200160415121876738, 7540113804746346429),
            ];
            let mut seed = 0x739abcd27u64;
            for _ in 0..4096 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let a = seed;
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let b = seed | 1;
                let (mut x, mut y) = (a, b);
                while y != 0 {
                    (x, y) = (y, x % y);
                }
                if x == 1 {
                    pairs.push((a, b));
                }
            }
            for (a, b) in pairs {
                let u = (field(a) * expected).0;
                let v = (field(b) * expected).0;
                assert_eq!(finish::<C>(a, b, u, v), expected, "a={a} b={b}");
            }
        }
        check::<crate::FqConfig>();
        check::<crate::FrConfig>();
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

// Matrix row encoding E=f+2^32*g modulo2^64, with row L1 norm<=2^31.
// The f=+2^31 pure row needs explicit decoding; f=-2^31 decodes normally.
// A pure g=+2^31 row decodes with the opposite whole-row sign. The existing
// full integer_update normalizes that sign and both coefficients before
// matrix composition. No mixed row can reach either ambiguity at this bound.
#[inline(always)]
pub(crate) fn decode_row(e:u64)->(i64,i64) {
    let low=e as u32 as i32 as i64;
    let f=select_unpredictable(e==1u64<<31,1i64<<31,low);
    let g=(e.wrapping_sub(f as u64) as i64)>>32;
    (f,g)
}
#[inline(always)]
pub(crate) fn zero_run_matrix(mut a:u64,mut b:u64)->(u64,u64,i64,i64,i64,i64) {
    let (mut row0,mut row1)=(1u64,1u64<<32);
    let mut remaining=BITS as u32;
    let first=a.trailing_zeros().min(remaining);
    a >>= first; row1 <<= first; remaining-=first;
    while remaining!=0 {
        debug_assert_eq!(a&1,1);debug_assert_eq!(b&1,1);
        let smaller=a<b;
        let difference=a.wrapping_sub(b);
        let magnitude=select_unpredictable(smaller,difference.wrapping_neg(),difference);
        b=select_unpredictable(smaller,a,b);
        let previous_row0=row0;
        row0=select_unpredictable(smaller,row1,row0);
        row1=select_unpredictable(smaller,previous_row0,row1);
        row0=row0.wrapping_sub(row1);
        let zeros=magnitude.trailing_zeros().min(remaining);
        debug_assert!(zeros>0 && zeros<=31);
        a=magnitude>>zeros;row1 <<= zeros;remaining-=zeros;
    }
    let(f0,g0)=decode_row(row0);let(f1,g1)=decode_row(row1);
    (a,b,f0,g0,f1,g1)
}
