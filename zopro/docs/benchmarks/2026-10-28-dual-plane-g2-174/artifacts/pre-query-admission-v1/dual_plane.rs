//! Joint low/high streaming; no transformed input-point cache.
use super::*;
use core::mem::size_of;

// Charge the complete new leaf, not merely a delta from existing forecasts.
// The token also covers normalization, output separation and serial merging.
fn forecast(bits: usize) -> Option<usize> {
    use crate::msm_affine::grouping_cost::{add, mul, vector as v};
    if !(10..=14).contains(&bits) {
        return None;
    }
    let buckets = 1usize.checked_shl(bits as u32)?;
    let batch = batch_size(bits).max(32).checked_mul(2)?;
    let merge = 512usize.min(buckets / 2);
    let mut cost = size_of::<Window<'static>>();
    for bytes in [
        v::<G2Affine>(buckets, false)?,
        v::<G2Projective>(buckets, true)?,
        v::<u32>(buckets, false)?,
        v::<usize>(buckets, true)?,
        v::<u64>(buckets.div_ceil(64), false)?,
        mul(v::<(u32, &G2Affine)>(batch, false)?, 2)?,
        crate::msm::workspace_batch::<G2Config>(batch.max(merge))?,
        v::<G2Projective>(buckets, false)?,
        v::<zopro_bn254::Fq2>(buckets, false)?,
        v::<G2Affine>(buckets, false)?,
        v::<G2Affine>(buckets / 2, false)?,
        v::<(u32, &G2Affine)>(merge, false)?,
    ] {
        cost = add(cost, bytes)?;
    }
    // Prefix-record reserve may grow from the fill batch to the merge batch.
    add(
        cost,
        mul(
            v::<[crate::simd::FqBatch; 2]>(
                batch.max(merge).div_ceil(crate::simd::BATCH_LANES),
                true,
            )?,
            2,
        )?,
    )
}

fn finish(mut state: Window<'_>) -> (Vec<G2Affine>, BatchAdd<G2Config>) {
    let combined: Vec<_> = state
        .projective
        .iter()
        .zip(&state.projective_buckets)
        .map(|(p, &i)| *p + state.affine[i])
        .collect();
    for (p, &i) in G2Projective::batch_to_affine(&combined)
        .into_iter()
        .zip(&state.projective_buckets)
    {
        state.affine[i] = p;
    }
    (state.affine, state.packed)
}

pub(super) fn try_window(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    w: usize,
    bits: usize,
) -> Option<Vec<G2Affine>> {
    checked_window(bases, short, w, bits, forecast(bits))
}
fn checked_window(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    w: usize,
    bits: usize,
    peak: Option<usize>,
) -> Option<Vec<G2Affine>> {
    if bases.len() != short.len() || !(10..=14).contains(&bits) || w >= 127usize.div_ceil(bits) {
        return None;
    }
    if w + 1 == 127usize.div_ceil(bits) && 126 - w * bits <= 8 {
        return None;
    }
    let packed = BatchAdd::default();
    if !packed.borrowed_g2_enabled() {
        return None;
    }
    let _credit = crate::preparation::Reservation::analysis(peak)?;
    let half = 1usize << (bits - 1);
    let size = 2 * batch_size(bits).max(32);
    let mut state = Window::new(2 * half, size, packed);
    for (p, s) in bases.iter().zip(short) {
        if p.is_identity() {
            continue;
        }
        // Distinct bucket planes share one pending batch and inverse stage.
        for high in [true, false] {
            let d = component(s, high, w, bits);
            if d == 0 {
                continue;
            }
            let bucket = d.unsigned_abs() as usize - 1 + if high { 0 } else { half };
            let encoded = bucket as u32 | if d < 0 { 1 << 31 } else { 0 };
            if state.busy(bucket) {
                state.queue.push((encoded, p));
                if state.queue.len() == size - 1 {
                    state.flush_queue();
                }
                continue;
            }
            state.add(encoded, p);
            if state.batch.len() == size {
                state.execute();
                state.take_from_queue();
            }
        }
    }
    state.execute();
    state.flush_queue();
    let (mut high, mut packed) = finish(state);
    // split_off allocates exactly the ordinary output length. Its capacity,
    // rather than an uncharged two-plane allocation, survives this token.
    let mut low = high.split_off(half);
    for p in &mut high {
        if !p.is_identity() {
            p.x = p.x.mul_by_fp(&BETA);
        }
    }
    let mut pairs = Vec::with_capacity(512.min(half));
    // Nonforking, same512 merge grain as merge_buckets. The full new-leaf
    // reservation remains live until both temporary planes/scratch drop.
    for (left, right) in low.chunks_mut(512).zip(high.chunks(512)) {
        pairs.clear();
        for (i, (a, b)) in left.iter_mut().zip(right).enumerate() {
            if a.is_identity() {
                *a = *b;
            } else if b.is_identity() {
            } else if a.x == b.x {
                *a = (*a + *b).to_affine();
            } else {
                pairs.push((i as u32, b));
            }
        }
        if !pairs.is_empty() {
            packed.apply_signed_borrowed_g2(left, &pairs);
        }
    }
    Some(low)
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::Fr;
    #[test]
    fn all_buckets_match_complete_signed_component_oracle() {
        for bits in [10, 12, 14] {
            let half = 1usize << (bits - 1);
            let mut seed = 0x6275636b65747331u64;
            for n in [0, 1, 17, 401, 1024] {
                let g = G2Projective::generator();
                let bases = G2Projective::batch_to_affine(
                    &(0..n)
                        .map(|i| match i % 11 {
                            0 => G2Projective::IDENTITY,
                            1..=4 => g,
                            5..=7 => -g,
                            _ => g * Fr::from(i as u64 + 101),
                        })
                        .collect::<Vec<_>>(),
                );
                let short: Vec<_> = (0..n)
                    .map(|i| {
                        let mut bytes = [0; 32];
                        for c in bytes.chunks_mut(8) {
                            seed ^= seed << 13;
                            seed ^= seed >> 7;
                            seed ^= seed << 17;
                            c.copy_from_slice(&seed.to_le_bytes());
                        }
                        let s = match i % 9 {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => -Fr::ONE,
                            _ => Fr::from_le_bytes_mod_order(&bytes),
                        };
                        crate::msm_affine::g2_phi::phi_split(s.to_canonical())
                    })
                    .collect();
                for w in [0, 3] {
                    let mut expected = vec![G2Projective::IDENTITY; half];
                    for (p, s) in bases.iter().zip(&short) {
                        for high in [true, false] {
                            let d = component(s, high, w, bits);
                            if d == 0 {
                                continue;
                            }
                            let mut point = *p;
                            if high && !point.is_identity() {
                                point.x = point.x.mul_by_fp(&BETA);
                            }
                            if d < 0 {
                                point = -point;
                            }
                            expected[d.unsigned_abs() as usize - 1] += point;
                        }
                    }
                    let got =
                        try_window(&bases, &short, w, bits).expect("admitted small native test");
                    assert_eq!(got.len(), half);
                    assert_eq!(got.capacity(), half);
                    assert_eq!(got, G2Projective::batch_to_affine(&expected));
                }
            }
        }
    }
    #[test]
    fn checked_costs_and_denial_preserve_inputs() {
        for bits in 10..=14 {
            let n = forecast(bits).unwrap();
            assert!(n >= (1usize << bits) * size_of::<G2Affine>());
        }
        assert!(forecast(usize::MAX).is_none());
        assert!(forecast(9).is_none());
        let bases = [G2Affine::GENERATOR];
        let words = [[1, 0, 2, 0]];
        assert!(checked_window(&bases, &words, 0, 12, Some(usize::MAX)).is_none());
        assert!(try_window(&bases, &words, 10, 12).is_none());
        assert_eq!(bases, [G2Affine::GENERATOR]);
        assert_eq!(words, [[1, 0, 2, 0]]);
    }
}
