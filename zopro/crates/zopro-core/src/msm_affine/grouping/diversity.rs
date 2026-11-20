//! A bounded lower bound on distinct nonzero scalars before constructing the grouping map.
pub(super) fn declines(scalars: &[[u64; 4]], cutoff: usize) -> bool {
    let bits = (scalars.len() * 8).next_power_of_two().min(1 << 18);
    let Some(_reservation) = crate::preparation::Reservation::analysis(Some(bits / 8 + 64)) else {
        return false;
    };
    occupied_exceeds(scalars, bits, cutoff)
}

fn occupied_exceeds(scalars: &[[u64; 4]], bits: usize, cutoff: usize) -> bool {
    let mut marks = Vec::<u64>::new();
    if marks.try_reserve_exact(bits / 64).is_err() {
        return false;
    }
    marks.resize(bits / 64, 0);
    let shift = u64::BITS - bits.ilog2();
    for s in scalars {
        // Equal scalars select equal bits. Ignoring all zero low limbs can only
        // lower the bound, including when their high limbs are nonzero.
        if s[0] != 0 {
            let pos = (s[0].wrapping_mul(0x9e3779b97f4a7c15) >> shift) as usize;
            marks[pos >> 6] |= 1 << (pos & 63);
        }
    }
    marks.iter().map(|v| v.count_ones() as usize).sum::<usize>() > cutoff
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    #[test]
    fn certificate_is_conservative_under_collisions_zeros_and_thresholds() {
        let mut rng = 8191u64;
        for n in [1024usize, 8192, 30089, 32768, 65535] {
            let cutoff = n * if n >= 32768 { 95 } else { 80 } / 100;
            let bits = (8 * n).next_power_of_two().min(1 << 18);
            for d in [1, cutoff, cutoff + 1, n] {
                let vals: Vec<_> = (0..n)
                    .map(|i| {
                        let v = i % d + 1;
                        [v as u64, 0, 0, 0]
                    })
                    .collect();
                let bound = occupied_exceeds(&vals, bits, cutoff);
                assert!(!bound || d > cutoff);
                if d == n {
                    assert!(bound);
                }
            }
            let x: Vec<_> = (0..n)
                .map(|i| {
                    rng ^= rng << 13;
                    rng ^= rng >> 7;
                    rng ^= rng << 17;
                    if i % 5 == 0 {
                        [0, 0, 0, 0]
                    } else {
                        [rng % 17, rng, 0, 0]
                    }
                })
                .collect();
            let exact = x
                .iter()
                .filter(|&&s| s != [0; 4])
                .collect::<BTreeSet<_>>()
                .len();
            for k in [0, 16, 17, cutoff, exact] {
                assert!(!occupied_exceeds(&x, bits, k) || exact > k);
            }
            assert!(!occupied_exceeds(&x, bits, 17));
        }
    }
}
