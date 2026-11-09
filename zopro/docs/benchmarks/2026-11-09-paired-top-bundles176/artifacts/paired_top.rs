//! Concurrent high/low leading components; ordinary windows retain their bodies.
use super::*;

pub(super) fn try_raw(
    bases: &[G2Affine],
    short: &mut [[u64; 4]],
    bits: usize,
    parts: usize,
) -> Option<G2Projective> {
    assert_eq!(bases.len(), short.len());
    if !(MIN_QUERY..MAX_QUERY).contains(&bases.len()) || bits != 12 || parts != 2 {
        return None;
    }
    let charge = super::super::workspace::paired_top_extra(bases.len(), parts)?;
    let _owner = crate::preparation::Reservation::analysis(Some(charge))?;
    #[cfg(test)]
    ADMITTED.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    short.par_iter_mut().for_each(|s| *s = phi_split(*s));
    let chunk = bases.len().div_ceil(parts);
    let windows = 127usize.div_ceil(bits);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|part| {
                    let start = (part * chunk).min(bases.len());
                    let end = (start + chunk).min(bases.len());
                    if w + 1 != windows {
                        return hb_window(&bases[start..end], &short[start..end], w, bits);
                    }
                    let top = 126 - w * bits;
                    let (mut high, low) = rayon::join(
                        || hb_top_component(&bases[start..end], &short[start..end], true, w, bits, top),
                        || hb_top_component(&bases[start..end], &short[start..end], false, w, bits, top),
                    );
                    for p in &mut high {
                        if !p.is_identity() {
                            p.x = p.x.mul_by_fp(&BETA);
                        }
                    }
                    merge_buckets(&mut high, &low);
                    #[cfg(test)]
                    PAIRED_PARTS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    high
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for b in buckets {
                merge_buckets(&mut combined, &b);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = G2Projective::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..bits {
            result = result.double();
        }
        result += sum;
    }
    Some(result)
}

#[cfg(test)]
static ADMITTED: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]
static PAIRED_PARTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]
mod tests;
