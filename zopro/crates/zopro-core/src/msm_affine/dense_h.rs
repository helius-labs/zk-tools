use super::*;

// Consecutive little-endian bit fields covering the 254 scalar bits.
const WIDTHS: [usize; 19] = [
    14, 13, 14, 13, 13, 14, 13, 13, 14, 13, 13, 14, 13, 13, 14, 13, 13, 14, 13,
];
// The last sign bit is above the 254-bit canonical scalar, so no top carry remains.
const SIGNED18: [usize; 18] = [
    15, 14, 14, 14, 14, 14, 15, 14, 14, 14, 14, 14, 15, 14, 14, 14, 14, 14,
];
const SIGNED17: [usize; 17] = [15; 17];

#[inline(always)]
fn mixed_booth(words: &[u64; 4], offset: usize, bits: usize) -> i16 {
    // Bring in the previous window's sign bit as the low overlap bit.
    let start = offset.saturating_sub(1);
    let limb = start / 64;
    let shift = start % 64;
    let mut raw = words[limb] >> shift;
    if shift + bits + usize::from(offset != 0) > 64 {
        raw |= words[limb + 1] << (64 - shift);
    }
    if offset == 0 {
        raw <<= 1;
    }
    let overlap = raw & ((1 << (bits + 1)) - 1);
    (((overlap + 1) >> 1) as i32 - (((overlap >> bits) as i32) << bits)) as i16
}

pub(super) fn try_sum<C: CurveConfig>(
    bases: &[Affine<C>],
    words: &[[u64; 4]],
) -> Option<Projective<C>>
where
    C::Base: BatchInverse,
{
    if bases.len() < 1 << 18 || !crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED") {
        return None;
    }
    let configured = std::env::var("ZOPRO_CPU_DENSE_H_WINDOWS").ok()?;
    let (widths, signed): (&[usize], bool) = match configured.as_str() {
        "19" => (&WIDTHS, false),
        "18s" => (&SIGNED18, true),
        "17s" => (&SIGNED17, true),
        _ => return None,
    };
    let split = rayon::current_num_threads() > 1
        && std::env::var("ZOPRO_CPU_H_WINDOW_RANGES").as_deref() == Ok("2");
    let requested_ranges = if split { 2 } else { 1 };
    for ranges in (1..=requested_ranges).rev() {
        let extra =
            super::workspace::dense_h_layout_extra::<C>(bases.len(), ranges, widths, signed);
        if let Some(_reservation) = crate::preparation::Reservation::analysis(extra) {
            return Some(match (signed, ranges) {
                (true, _) => signed_sum(bases, words, widths, ranges == 2),
                (false, 2) => sum::<C, true>(bases, words),
                (false, _) => sum::<C, false>(bases, words),
            });
        }
    }
    None
}

fn sum<C: CurveConfig, const SPLIT: bool>(bases: &[Affine<C>], words: &[[u64; 4]]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), words.len());
    let widths = &WIDTHS;
    let mut offsets = [0usize; WIDTHS.len()];
    let mut end = 0;
    for (slot, &bits) in offsets.iter_mut().zip(widths) {
        *slot = end;
        end += bits;
    }
    assert_eq!(end, Fr::MODULUS_BITS as usize);
    let sums: Vec<_> = widths
        .par_iter()
        .zip(&offsets)
        .map(|(&bits, &offset)| {
            let lane = offset / 64;
            let shift = offset % 64;
            let mask = (1u64 << bits) - 1;
            let batch = batch_size(bits + 1).max(32);
            let fill = |bases: &[Affine<C>], words: &[[u64; 4]]| {
                let buckets = if shift + bits <= 64 {
                    fill_window_impl::<C, false, true>(
                        bases,
                        words.iter().map(|v| ((v[lane] >> shift) & mask) as i16),
                        bits + 1,
                        batch,
                    )
                } else {
                    fill_window_impl::<C, false, true>(
                        bases,
                        words.iter().map(|v| {
                            (((v[lane] >> shift) | (v[lane + 1] << (64 - shift))) & mask) as i16
                        }),
                        bits + 1,
                        batch,
                    )
                };
                affine_buckets(buckets)
            };
            let buckets = if SPLIT {
                let mid = bases.len().div_ceil(2);
                let (mut left, right) = rayon::join(
                    || fill(&bases[..mid], &words[..mid]),
                    || fill(&bases[mid..], &words[mid..]),
                );
                merge_buckets(&mut left, &right);
                left
            } else {
                fill(bases, words)
            };
            crate::msm::weighted_affine(&buckets)
        })
        .collect();
    let mut result = *sums.last().unwrap();
    for j in (0..sums.len() - 1).rev() {
        for _ in 0..widths[j] {
            result = result.double();
        }
        result += sums[j];
    }
    result
}

fn signed_sum<C: CurveConfig>(
    bases: &[Affine<C>],
    words: &[[u64; 4]],
    widths: &[usize],
    split: bool,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), words.len());
    let mut offsets = [0usize; SIGNED18.len()];
    let mut end = 0;
    for (offset, &bits) in offsets.iter_mut().zip(widths) {
        *offset = end;
        end += bits;
    }
    assert_eq!(end, Fr::MODULUS_BITS as usize + 1);
    let sums: Vec<_> = widths
        .par_iter()
        .zip(&offsets[..widths.len()])
        .map(|(&bits, &offset)| {
            let fill = |bases: &[Affine<C>], words: &[[u64; 4]]| {
                let digits = words.iter().map(|v| mixed_booth(v, offset, bits));
                affine_buckets(fill_window(bases, digits, bits, batch_size(bits).max(32)))
            };
            let buckets = if split {
                let mid = bases.len().div_ceil(2);
                let (mut left, right) = rayon::join(
                    || fill(&bases[..mid], &words[..mid]),
                    || fill(&bases[mid..], &words[mid..]),
                );
                merge_buckets(&mut left, &right);
                left
            } else {
                fill(bases, words)
            };
            crate::msm::weighted_affine(&buckets)
        })
        .collect();
    let mut result = *sums.last().unwrap();
    for j in (0..sums.len() - 1).rev() {
        for _ in 0..widths[j] {
            result = result.double();
        }
        result += sums[j];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G1Affine, G1Config};
    #[test]
    fn mixed_signed_digits_and_msm_match_scalar_oracle() {
        for widths in [&SIGNED17[..], &SIGNED18[..]] {
            let mut words = Vec::new();
            let mut bases = Vec::new();
            let mut expected = Fr::ZERO;
            for i in 0..267 {
                let mut x = Fr::ONE;
                for _ in 0..i % 254 {
                    x = x + x;
                }
                x = match i % 5 {
                    0 => -Fr::ONE,
                    1 => x - Fr::ONE,
                    2 => -x,
                    3 => Fr::ZERO,
                    _ => x + Fr::ONE,
                };
                let word = x.to_canonical();
                let mut recovered = Fr::ZERO;
                let mut place = Fr::ONE;
                let mut offset = 0;
                for &bits in widths {
                    let digit = mixed_booth(&word, offset, bits);
                    assert!(digit.unsigned_abs() as usize <= 1 << (bits - 1));
                    let term = place * Fr::from(digit.unsigned_abs() as u64);
                    recovered += if digit < 0 { -term } else { term };
                    for _ in 0..bits {
                        place = place + place;
                    }
                    offset += bits;
                }
                assert_eq!(recovered, x);
                let (p, v) = match i % 4 {
                    0 => (G1Affine::IDENTITY, Fr::ZERO),
                    1 => (-G1Affine::GENERATOR, -x),
                    _ => (G1Affine::GENERATOR, x),
                };
                bases.push(p);
                words.push(word);
                expected += v;
            }
            for split in [false, true] {
                assert_eq!(
                    signed_sum(&bases, &words, widths, split).to_affine(),
                    (G1Affine::GENERATOR * expected).to_affine()
                );
            }
        }
    }

    #[test]
    fn mixed_windows_reconstruct_generator_sum() {
        let mut bases = vec![G1Affine::GENERATOR; 1031];
        let mut words = Vec::new();
        let mut expected = Fr::ZERO;
        for (i, p) in bases.iter_mut().enumerate() {
            let value = match i % 31 {
                0 => Fr::ZERO,
                1 => -Fr::ONE,
                _ => Fr::from((i + 1) as u64).pow(&[47]),
            };
            if i % 7 == 0 {
                *p = G1Affine::IDENTITY;
            } else if i % 3 == 0 {
                *p = -*p;
                expected -= value;
            } else {
                expected += value;
            }
            words.push(value.to_canonical());
        }
        let oracle = (G1Affine::GENERATOR * expected).to_affine();
        assert_eq!(sum::<G1Config, false>(&bases, &words).to_affine(), oracle);
        assert_eq!(sum::<G1Config, true>(&bases, &words).to_affine(), oracle);
    }
}
