use super::*;

/// A and B fit in N words. Matrix-row L1 norm is at most 2^31,
/// hence the signed combination fits N+1 words and its absolute
/// quotient by 2^31 again fits N words. Modular coefficients use
/// their original four-word update; only GCD integers shrink.
#[inline(always)]
pub(super) fn integer_update<const N: usize>(
    a: &[u64; 4],
    b: &[u64; 4],
    f: &mut i64,
    g: &mut i64,
) -> [u64; 4] {
    debug_assert!((2..=4).contains(&N));
    debug_assert!(a[N..].iter().all(|&v| v == 0) && b[N..].iter().all(|&v| v == 0));
    debug_assert!(f.unsigned_abs() + g.unsigned_abs() <= 1 << 31);
    let mut t = [0u64; 5];
    let mut carry = 0i128;
    for j in 0..N {
        let v = (a[j] as i128) * (*f as i128) + (b[j] as i128) * (*g as i128) + carry;
        t[j] = v as u64;
        carry = v >> 64;
    }
    t[N] = carry as u64;
    if (t[N] as i64) < 0 {
        *f = -*f;
        *g = -*g;
        let mut carry = 1u128;
        for limb in &mut t[..=N] {
            carry += (!*limb) as u128;
            *limb = carry as u64;
            carry >>= 64;
        }
    }
    debug_assert_eq!(t[0] & MASK, 0);
    debug_assert_eq!(t[N] >> BITS, 0);
    core::array::from_fn(|j| {
        if j < N {
            (t[j] >> BITS) | (t[j + 1] << (64 - BITS))
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn mul(a: [u64; 4], m: u64) -> [u64; 5] {
        let mut t = [0; 5];
        let mut c = 0u128;
        for j in 0..4 {
            let z = a[j] as u128 * m as u128 + c;
            t[j] = z as u64;
            c = z >> 64;
        }
        t[4] = c as u64;
        t
    }
    fn add(a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
        let mut t = [0; 5];
        let mut c = 0u128;
        for j in 0..5 {
            c += a[j] as u128 + b[j] as u128;
            t[j] = c as u64;
            c >>= 64;
        }
        assert_eq!(c, 0);
        t
    }
    fn sub(a: [u64; 5], b: [u64; 5]) -> [u64; 5] {
        let mut t = [0; 5];
        let mut c = 0u128;
        for j in 0..5 {
            let rhs = b[j] as u128 + c;
            let lhs = a[j] as u128;
            t[j] = lhs.wrapping_sub(rhs) as u64;
            c = u128::from(lhs < rhs);
        }
        assert_eq!(c, 0);
        t
    }
    fn oracle(a: [u64; 4], b: [u64; 4], f: i64, g: i64) -> ([u64; 4], i64, i64) {
        let x = mul(a, f.unsigned_abs());
        let y = mul(b, g.unsigned_abs());
        let (mag, negative) = if (f < 0) == (g < 0) {
            (add(x, y), f < 0)
        } else {
            let cmp = x.iter().rev().cmp(y.iter().rev());
            if cmp.is_lt() {
                (sub(y, x), g < 0)
            } else {
                (sub(x, y), f < 0)
            }
        };
        assert_eq!(mag[0] & MASK, 0);
        assert_eq!(mag[4] >> BITS, 0);
        let out = core::array::from_fn(|j| (mag[j] >> BITS) | (mag[j + 1] << (64 - BITS)));
        (
            out,
            if negative { -f } else { f },
            if negative { -g } else { g },
        )
    }
    fn check(a: [u64; 4], b: [u64; 4], f: i64, g: i64, n: usize) {
        let expected = oracle(a, b, f, g);
        let (mut of, mut og) = (f, g);
        let old = super::super::integer_update(&a, &b, &mut of, &mut og);
        assert_eq!((old, of, og), expected);
        let (mut nf, mut ng) = (f, g);
        let new = match n {
            2 => integer_update::<2>(&a, &b, &mut nf, &mut ng),
            3 => integer_update::<3>(&a, &b, &mut nf, &mut ng),
            4 => integer_update::<4>(&a, &b, &mut nf, &mut ng),
            _ => unreachable!(),
        };
        assert_eq!((new, nf, ng), expected);
    }
    #[test]
    fn shortened_integer_updates_match_unsigned_oracle_and_original() {
        let mut seed = 0x61c4_e31b_2551_6a01u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut cases = 0;
        for bits in 65..=254 {
            let n = (bits + 63) / 64;
            for _ in 0..128 {
                let mut a = core::array::from_fn(|_| next());
                let mut b = core::array::from_fn(|_| next());
                for j in n..4 {
                    a[j] = 0;
                    b[j] = 0;
                }
                let high = bits - (n - 1) * 64;
                if high < 64 {
                    a[n - 1] &= (1 << high) - 1;
                    b[n - 1] &= (1 << high) - 1;
                }
                a[n - 1] |= 1 << (high - 1);
                b[0] |= 1;
                let (_, _, f0, g0, f1, g1) =
                    zero_run_matrix(approximate(&a, bits), approximate(&b, bits));
                check(a, b, f0, g0, n);
                check(a, b, f1, g1, n);
                cases += 2;
            }
        }
        for n in 2..=4 {
            let mut a = [0; 4];
            for x in &mut a[..n] {
                *x = u64::MAX;
            }
            let b = a;
            for sign in [-1, 1] {
                check(a, b, sign * (1 << 31), 0, n);
                check(a, b, 0, sign * (1 << 31), n);
                cases += 2;
            }
            check([0; 4], b, 1 << 31, 0, n);
            cases += 1;
        }
        assert_eq!(cases, 48655);
    }
}
