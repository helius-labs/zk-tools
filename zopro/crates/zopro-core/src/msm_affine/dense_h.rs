use super::*;

// Consecutive little-endian bit fields covering the 254 scalar bits.
pub(super) const WIDTHS: [usize; 19] = [
    14, 13, 14, 13, 13, 14, 13, 13, 14, 13, 13, 14, 13, 13, 14, 13, 13, 14, 13,
];
pub(super) fn try_sum<C: CurveConfig>(
    bases: &[Affine<C>],
    words: &[[u64; 4]],
) -> Option<Projective<C>>
where
    C::Base: BatchInverse,
{
    if bases.len() < 1 << 18
        || std::env::var("ZOPRO_CPU_DENSE_H_WINDOWS").as_deref() != Ok("19")
        || !crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED")
    {
        return None;
    }
    let growth = super::workspace::dense_h_extra::<C>(bases.len());
    let _reservation = crate::preparation::Reservation::analysis(growth)?;
    Some(sum(bases, words))
}

fn sum<C: CurveConfig>(bases: &[Affine<C>], words: &[[u64; 4]]) -> Projective<C>
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
            crate::msm::weighted_affine(&affine_buckets(buckets))
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
        {
            assert_eq!(
                sum::<G1Config>(&bases, &words).to_affine(),
                (G1Affine::GENERATOR * expected).to_affine()
            );
        }
    }
}
