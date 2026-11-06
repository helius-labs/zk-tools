//! Exact variable-span Booth reconstruction, without field/curve or timing claims.
#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub width: usize,
}
pub fn spans() -> [Span; 11] {
    core::array::from_fn(|i| match i {
        0..=7 => Span {
            start: i * 12,
            width: 12,
        },
        8 => Span {
            start: 96,
            width: 11,
        },
        9 => Span {
            start: 107,
            width: 11,
        },
        10 => Span {
            start: 118,
            width: 12,
        },
        _ => unreachable!(),
    })
}
/// Same offset/carry recipe as accepted balanced K/H; an unsigned magnitude
/// below2^126 is represented in the first two limbs. No recoded array.
pub fn digit_at(s: &[u64; 4], span: Span) -> i16 {
    let Span { start, width } = span;
    if start == 0 {
        let value = s[0] & ((1u64 << width) - 1);
        return (value as i64 - ((value >> (width - 1)) << width) as i64) as i16;
    }
    let offset = start - 1;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { s[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        value |= s[word + 1] << (64 - shift);
    }
    value &= (1u64 << (width + 1)) - 1;
    (((value + 1) >> 1) as i64 - ((value >> width) << width) as i64) as i16
}
pub fn component(s: &[u64; 4], high: bool, span: Span) -> i16 {
    let i = usize::from(high) * 2;
    let d = digit_at(&[s[i], s[i + 1] & 0x7fffffffffffffff, 0, 0], span);
    if s[i + 1] >> 63 != 0 {
        -d
    } else {
        d
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn encode(n: u128, neg: bool) -> [u64; 2] {
        assert!(n < (1u128 << 126));
        [n as u64, (n >> 64) as u64 | ((neg as u64) << 63)]
    }
    fn independent_digit(n: u128, neg: bool, span: Span) -> i16 {
        let bit = |i: usize| if i < 128 { ((n >> i) & 1) as i128 } else { 0 };
        let mut d = if span.start == 0 {
            0
        } else {
            bit(span.start - 1)
        };
        for k in 0..span.width {
            let weight = 1i128 << k;
            if k + 1 == span.width {
                d -= bit(span.start + k) * weight
            } else {
                d += bit(span.start + k) * weight
            }
        }
        if neg {
            -d as i16
        } else {
            d as i16
        }
    }
    fn check(n: u128, neg: bool) {
        let pair = encode(n, neg);
        let words = [pair[0], pair[1], pair[0], pair[1]];
        let mut total = 0i128;
        for (i, span) in spans().into_iter().enumerate() {
            let d = component(&words, false, span);
            assert_eq!(d, independent_digit(n, neg, span));
            assert_eq!(d, component(&words, true, span));
            if i == 10 {
                assert!(d.unsigned_abs() <= 256);
                assert!(d == 0 || (d < 0) == neg)
            } else {
                assert!((d as i32).abs() <= 1 << (span.width - 1));
            }
            total += (d as i128) << span.start;
        }
        assert_eq!(total, if neg { -(n as i128) } else { n as i128 });
        // Horner must use each LOWER span's width; using12uniformly is incorrect.
        let cover = spans();
        let mut horner = component(&words, false, cover[10]) as i128;
        for i in (0..10).rev() {
            horner = (horner << cover[i].width) + component(&words, false, cover[i]) as i128;
        }
        assert_eq!(horner, total);
    }
    #[test]
    fn all_small_values_both_signs_and_components() {
        for n in 0..4096 {
            check(n, false);
            check(n, true);
        }
    }
    #[test]
    fn every_bit_boundary_and_top_carry() {
        let max = (1u128 << 126) - 1;
        for k in 0..126 {
            let p = 1u128 << k;
            for n in [p.saturating_sub(1), p, (p + 1).min(max), max ^ (p - 1)] {
                check(n, false);
                check(n, true);
            }
        }
        let p = encode(max, false);
        assert_eq!(digit_at(&[p[0], p[1], 0, 0], spans()[10]), 256);
    }
    #[test]
    fn random_full_width_reconstructs_exact_integer() {
        let mut state = 0x6d697865645f746fu64;
        for _ in 0..50000 {
            let mut next = || {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state
            };
            let n = ((next() as u128) << 64 | next() as u128) & ((1u128 << 126) - 1);
            check(n, false);
            check(n, true);
        }
    }
    #[test]
    fn fixed_geometry_preserves_contiguous_offsets_and_shrinks_buckets() {
        let s = spans();
        assert_eq!(s.len(), 11);
        for i in 1..11 {
            assert_eq!(s[i].start, s[i - 1].start + s[i - 1].width);
        }
        let ordinary: usize = s[..10].iter().map(|s| 1usize << (s.width - 1)).sum();
        assert_eq!(ordinary, 18432);
        assert_eq!(ordinary + 256, 18688);
        assert_eq!(10 * 2048 + 64 - (ordinary + 256), 1856);
    }
}
