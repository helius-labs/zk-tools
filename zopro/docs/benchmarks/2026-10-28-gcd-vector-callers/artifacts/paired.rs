use core::arch::x86_64::*;
const MASK: u64 = (1 << 31) - 1;

/// Compute both complete GCD integer rows using eight radix-32 digits.
/// Every coefficient row has L1 norm <=2^31. Each signed digit sum fits
/// i64, and carrying in radix32 stays in [-2^31,2^31-1]. Thus adding the
/// incoming carry cannot overflow i64. Modular coefficient rows are NOT
/// passed here; their 62-bit bounds require the unchanged scalar path.
#[inline(always)]
pub(crate) fn update(a: &[u64; 4], b: &[u64; 4], rows: &mut [i64; 4]) -> ([u64; 4], [u64; 4]) {
    debug_assert!(rows[0].unsigned_abs() + rows[1].unsigned_abs() <= 1 << 31);
    debug_assert!(rows[2].unsigned_abs() + rows[3].unsigned_abs() <= 1 << 31);
    let [f0, g0, f1, g1] = *rows;
    // SAFETY: this prototype is compiled only for the pinned native CPU
    // with AVX2/AVX512F. Each input is exactly32 bytes, loadu permits its
    // ordinary alignment, and each result buffer is exactly64 bytes.
    let (c0, c1) = unsafe {
        let av = _mm512_cvtepu32_epi64(_mm256_loadu_si256(a.as_ptr().cast()));
        let bv = _mm512_cvtepu32_epi64(_mm256_loadu_si256(b.as_ptr().cast()));
        let ashift = _mm512_slli_epi64::<32>(av);
        let bshift = _mm512_slli_epi64::<32>(bv);
        // Multiplication by the unsigned low32 bits followed by the
        // masked 2^32 correction implements a signed coefficient exactly
        // modulo2^64, including both signs of the pure2^31 boundary.
        let r0 = _mm512_add_epi64(
            _mm512_mul_epu32(av, _mm512_set1_epi64(f0 as u32 as i64)),
            _mm512_mul_epu32(bv, _mm512_set1_epi64(g0 as u32 as i64)),
        );
        let r0 = _mm512_mask_sub_epi64(r0, (f0 >> 63) as u8, r0, ashift);
        let r0 = _mm512_mask_sub_epi64(r0, (g0 >> 63) as u8, r0, bshift);
        let r1 = _mm512_add_epi64(
            _mm512_mul_epu32(av, _mm512_set1_epi64(f1 as u32 as i64)),
            _mm512_mul_epu32(bv, _mm512_set1_epi64(g1 as u32 as i64)),
        );
        let r1 = _mm512_mask_sub_epi64(r1, (f1 >> 63) as u8, r1, ashift);
        let r1 = _mm512_mask_sub_epi64(r1, (g1 >> 63) as u8, r1, bshift);
        let mut c0 = [0i64; 8];
        let mut c1 = [0i64; 8];
        _mm512_storeu_si512(c0.as_mut_ptr().cast(), r0);
        _mm512_storeu_si512(c1.as_mut_ptr().cast(), r1);
        (c0, c1)
    };
    let (mut t0, mut t1) = ([0u64; 5], [0u64; 5]);
    let (mut carry0, mut carry1) = (0i64, 0i64);
    for j in 0..8 {
        let x = c0[j] + carry0;
        let y = c1[j] + carry1;
        t0[j / 2] |= (x as u32 as u64) << ((j % 2) * 32);
        t1[j / 2] |= (y as u32 as u64) << ((j % 2) * 32);
        carry0 = x >> 32;
        carry1 = y >> 32;
    }
    t0[4] = carry0 as u64;
    t1[4] = carry1 as u64;
    #[inline(always)]
    fn finish(mut t: [u64; 5], f: &mut i64, g: &mut i64) -> [u64; 4] {
        if (t[4] as i64) < 0 {
            *f = -*f;
            *g = -*g;
            let mut carry = 1u128;
            for word in &mut t {
                carry += (!*word) as u128;
                *word = carry as u64;
                carry >>= 64;
            }
        }
        debug_assert_eq!(t[0] & MASK, 0);
        debug_assert_eq!(t[4] >> 31, 0);
        core::array::from_fn(|j| (t[j] >> 31) | (t[j + 1] << 33))
    }
    let (mut f0, mut g0, mut f1, mut g1) = (f0, g0, f1, g1);
    let aa = finish(t0, &mut f0, &mut g0);
    let bb = finish(t1, &mut f1, &mut g1);
    *rows = [f0, g0, f1, g1];
    (aa, bb)
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
        } else if x.iter().rev().cmp(y.iter().rev()).is_lt() {
            (sub(y, x), g < 0)
        } else {
            (sub(x, y), f < 0)
        };
        let negative = negative && mag != [0; 5];
        assert_eq!(mag[0] & MASK, 0);
        assert_eq!(mag[4] >> 31, 0);
        (
            core::array::from_fn(|j| (mag[j] >> 31) | (mag[j + 1] << 33)),
            if negative { -f } else { f },
            if negative { -g } else { g },
        )
    }
    fn check(a: [u64; 4], b: [u64; 4], rows: [i64; 4]) {
        let e0 = oracle(a, b, rows[0], rows[1]);
        let e1 = oracle(a, b, rows[2], rows[3]);
        let mut got_rows = rows;
        let got = update(&a, &b, &mut got_rows);
        assert_eq!(got, (e0.0, e1.0));
        assert_eq!(got_rows, [e0.1, e0.2, e1.1, e1.2]);
        assert_eq!((got, got_rows), super::super::reference_pair(a, b, rows));
    }
    #[test]
    fn paired_rows_match_original_and_independent_unsigned_magnitudes() {
        let mut seed = 0x31ca_5120_6c44_32a1u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut pairs = 0;
        for bits in 1..=256 {
            for _ in 0..128 {
                let n = (bits + 63) / 64;
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
                let rows = super::super::reference_matrix(a, b);
                check(a, b, rows);
                pairs += 1;
            }
        }
        // Multiples of2^31 admit arbitrary bounded rows; these include
        // signs and zero combinations not reached by ordinary unit GCDs.
        for _ in 0..32768 {
            let a = [next() & !MASK, next(), next(), next()];
            let b = [next() & !MASK, next(), next(), next()];
            let mut rows = [0; 4];
            for k in [0, 2] {
                let f = (next() % ((1 << 31) + 1)) as i64;
                let g = (next() % ((1 << 31) - f as u64 + 1)) as i64;
                rows[k] = if next() & 1 != 0 { -f } else { f };
                rows[k + 1] = if next() & 1 != 0 { -g } else { g };
            }
            check(a, b, rows);
            pairs += 1;
        }
        for a in [
            [0; 4],
            [u64::MAX; 4],
            [0xffff_ffff_8000_0000; 4],
            [1 << 31, 0, 0, 0],
        ] {
            for b in [
                [0; 4],
                [u64::MAX; 4],
                [0xffff_ffff_8000_0000; 4],
                [1 << 31, 0, 0, 0],
            ] {
                for f in [-(1 << 31), 0, 1 << 31] {
                    for g in [-(1 << 31), 0, 1 << 31] {
                        if (f as i64).unsigned_abs() + (g as i64).unsigned_abs() <= 1 << 31 {
                            check(a, b, [f, g, g, f]);
                            pairs += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(pairs, 65616);
    }
    #[test]
    fn signed_radix32_carry_bounds_cover_extremes() {
        let b = 1i128 << 31;
        for c in [-b, b - 1, 0] {
            for v in [-((1i128 << 32) - 1) * b, 0, ((1i128 << 32) - 1) * b] {
                let z = v + c;
                assert!(z >= i64::MIN as i128 && z <= i64::MAX as i128);
                let q = z >> 32;
                assert!(q >= -b && q < b);
            }
        }
    }
}
