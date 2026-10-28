//! Bucket-Horner MSM for subgroup-valid G2 queries of a Groth16 key.
//! No key-derived tables. Scratch is a canonical scalar copy below 2 MiB;
//! temporary point buckets/tree storage follow the existing MSM kernels.
use super::{
    affine_buckets, batch_size, digit, fill_window_impl, merge_buckets, msm_grouped, msm_raw,
    ScalarGroups,
};
use rayon::prelude::*;
#[cfg(test)]
use zopro_bn254::Fr;
const MIN_QUERY: usize = 1024;
const MAX_QUERY: usize = 1 << 16;
const FROB_MU: u128 = 147946756881789318990833708069417712966;
#[inline]
fn frob_split(n: [u64; 4]) -> [u64; 4] {
    let recip = [0x2cb62031c8e01941u64, 0x4ccef014a773d2d5, 2];
    let mut product = [0u64; 7];
    for i in 0..4 {
        let mut carry = 0u128;
        for j in 0..3 {
            let t = n[i] as u128 * recip[j] as u128 + product[i + j] as u128 + carry;
            product[i + j] = t as u64;
            carry = t >> 64;
        }
        let mut j = i + 3;
        while carry != 0 {
            let t = product[j] as u128 + carry;
            product[j] = t as u64;
            carry = t >> 64;
            j += 1;
        }
    }
    let mut q = product[4] as u128 | ((product[5] as u128) << 64);
    let a = [FROB_MU as u64, (FROB_MU >> 64) as u64];
    let b = [q as u64, (q >> 64) as u64];
    let mut p = [0u64; 4];
    for i in 0..2 {
        let mut carry = 0u128;
        for j in 0..2 {
            let t = a[i] as u128 * b[j] as u128 + p[i + j] as u128 + carry;
            p[i + j] = t as u64;
            carry = t >> 64;
        }
        let mut j = i + 2;
        while carry != 0 {
            let t = p[j] as u128 + carry;
            p[j] = t as u64;
            carry = t >> 64;
            j += 1;
        }
    }
    let mut rem = [0u64; 4];
    let mut borrow = false;
    for i in 0..4 {
        let (t, b0) = n[i].overflowing_sub(p[i]);
        let (t, b1) = t.overflowing_sub(borrow as u64);
        rem[i] = t;
        borrow = b0 || b1;
    }
    assert!(!borrow);
    assert_eq!((rem[2], rem[3]), (0, 0));
    let mut a = rem[0] as u128 | ((rem[1] as u128) << 64);
    if a >= FROB_MU {
        a -= FROB_MU;
        q += 1;
    }
    assert!(a < FROB_MU && a >> 127 == 0 && q >> 127 == 0);
    [a as u64, (a >> 64) as u64, q as u64, (q >> 64) as u64]
}

use zopro_bn254::{Fq, Fq2, G2Affine, G2Config, G2Projective};
const HB_X: Fq2 = Fq2::new(
    Fq::from_decimal(
        "21575463638280843010398324269430826099269044274347216827212613867836435027261",
    ),
    Fq::from_decimal(
        "10307601595873709700152284273816112264069230130616436755625194854815875713954",
    ),
);
const HB_Y: Fq2 = Fq2::new(
    Fq::from_decimal(
        "2821565182194536844548159561693502659359617185244120367078079554186484126554",
    ),
    Fq::from_decimal(
        "3505843767911556378687030309984248845540243509899259641013678093033130930403",
    ),
);
#[inline]
fn hb_digit(s: &[u64; 4], high: bool, w: usize, bits: usize) -> i16 {
    let i = usize::from(high) * 2;
    digit(&[s[i], s[i + 1], 0, 0], w, bits)
}
fn hb_top_component(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    high: bool,
    w: usize,
    bits: usize,
    top: usize,
) -> Vec<G2Affine> {
    let n = 1usize << top;
    let mut counts = vec![0usize; n];
    for (base, s) in bases.iter().zip(short) {
        let d = hb_digit(s, high, w, bits);
        assert!(d >= 0 && d as usize <= n);
        if d != 0 && !base.is_identity() {
            counts[d as usize - 1] += 1;
        }
    }
    let mut starts = Vec::with_capacity(n + 1);
    starts.push(0);
    for count in counts {
        starts.push(starts.last().unwrap() + count);
    }
    if *starts.last().unwrap() == 0 {
        return vec![G2Affine::IDENTITY; n];
    }
    let mut positions = starts[..n].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, (base, s)) in bases.iter().zip(short).enumerate() {
        let d = hb_digit(s, high, w, bits);
        if d != 0 && !base.is_identity() {
            let b = d as usize - 1;
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    crate::msm::reduce_indexed_buckets(bases, &order, &starts)
}
fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize) -> Vec<G2Affine> {
    if w + 1 == 128usize.div_ceil(bits) && 127 - w * bits <= 8 {
        let top = 127 - w * bits;
        // Each component's temporary index/tree workspace is released before the next.
        let mut high = hb_top_component(bases, short, true, w, bits, top);
        for p in &mut high {
            if !p.is_identity() {
                p.x = p.x.conjugate() * HB_X;
                p.y = p.y.conjugate() * HB_Y;
            }
        }
        let low = hb_top_component(bases, short, false, w, bits, top);
        merge_buckets(&mut high, &low);
        return high;
    }
    let size = batch_size(bits).max(32);
    let mut state = fill_window_impl::<G2Config, false, false>(
        bases,
        short.iter().map(|s| hb_digit(s, true, w, bits)),
        bits,
        size,
    );
    assert!(state.batch.is_empty() && state.queue.is_empty());
    for p in &mut state.affine {
        if !p.is_identity() {
            p.x = p.x.conjugate() * HB_X;
            p.y = p.y.conjugate() * HB_Y;
        }
    }
    for p in &mut state.projective {
        p.x = p.x.conjugate() * HB_X;
        p.y = p.y.conjugate() * HB_Y;
        p.z = p.z.conjugate();
    }
    for (base, d) in bases
        .iter()
        .zip(short.iter().map(|s| hb_digit(s, false, w, bits)))
    {
        if d == 0 || base.is_identity() {
            continue;
        }
        let bucket = d.unsigned_abs() as usize - 1;
        let point = if d < 0 { -*base } else { *base };
        if state.busy(bucket) {
            #[cfg(zopro_msm_geometry)]
            state.geometry.queued();
            state.queue.push((bucket as u32, point));
            if state.queue.len() == size - 1 {
                state.flush_queue();
            }
            continue;
        }
        state.add(bucket, point);
        if state.batch.len() == size {
            state.execute();
            state.take_from_queue();
        }
    }
    state.execute();
    state.flush_queue();
    affine_buckets(state)
}
fn hb_raw(bases: &[G2Affine], short: &mut [[u64; 4]], bits: usize, parts: usize) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    short.par_iter_mut().for_each(|s| *s = frob_split(*s));
    let chunk = bases.len().div_ceil(parts);
    let windows = 128usize.div_ceil(bits);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|part| {
                    let start = (part * chunk).min(bases.len());
                    let end = (start + chunk).min(bases.len());
                    hb_window(&bases[start..end], &short[start..end], w, bits)
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
    result
}

#[inline]
pub(super) fn raw(bases: &[G2Affine], words: &[[u64; 4]]) -> G2Projective {
    if !(MIN_QUERY..MAX_QUERY).contains(&bases.len()) {
        return msm_raw(bases, words);
    }
    // B1 reads the caller's canonical scalars concurrently.
    let mut short = words.to_vec();
    hb_raw(bases, &mut short, 11, 2)
}
#[inline]
pub(super) fn grouped(bases: &[G2Affine], groups: &ScalarGroups) -> G2Projective {
    assert_eq!(bases.len(), groups.len);
    if !(MIN_QUERY..MAX_QUERY).contains(&bases.len())
        || !(MIN_QUERY..MAX_QUERY).contains(&groups.values.len())
    {
        return msm_grouped(bases, groups);
    }
    let points = crate::msm::group_affine(bases, &groups.order, &groups.starts, &groups.counts);
    let mut short = groups.values.clone();
    hb_raw(&points, &mut short, 11, 2)
}
#[cfg(test)]
mod tests {
    use super::*;
    fn bases(n: usize) -> (Vec<G2Affine>, Vec<Fr>) {
        let weights: Vec<_> = (0..n)
            .map(|i| {
                let value = Fr::from((i % 23 + 3) as u64);
                if i % 17 == 0 {
                    Fr::ZERO
                } else if i % 3 == 0 {
                    -value
                } else {
                    value
                }
            })
            .collect();
        let points = weights
            .iter()
            .map(|&s| (G2Affine::GENERATOR * s).to_affine())
            .collect();
        (points, weights)
    }
    #[test]
    fn query_horner_matches_weighted_generator_at_scalar_boundaries() {
        let (bases, weights) = bases(1027);
        let mu = Fr::from_decimal("147946756881789318990833708069417712966");
        for kind in 0..7 {
            let values: Vec<_> = (0..bases.len())
                .map(|i| match kind {
                    0 => Fr::ZERO,
                    1 => Fr::ONE,
                    2 => -Fr::ONE,
                    3 => mu - Fr::ONE,
                    4 => mu,
                    5 => mu + Fr::ONE,
                    _ => Fr::from((i + 91) as u64).pow(&[127]),
                })
                .collect();
            let words: Vec<_> = values.iter().map(Fr::to_canonical).collect();
            let before = words.clone();
            let scalar: Fr = values
                .iter()
                .zip(&weights)
                .fold(Fr::ZERO, |acc, (&s, &w)| acc + s * w);
            assert_eq!(
                raw(&bases, &words).to_affine(),
                (G2Affine::GENERATOR * scalar).to_affine()
            );
            assert_eq!(words, before, "B1's canonical values must be unchanged");
        }
    }
    #[test]
    fn query_horner_preserves_shared_group_values() {
        let (bases, weights) = bases(4096);
        let values: Vec<_> = (0..bases.len())
            .map(|i| Fr::from((i % 1500 + 101) as u64).pow(&[137]))
            .collect();
        let words: Vec<_> = values.iter().map(Fr::to_canonical).collect();
        let groups = super::super::scalar_groups(&words).unwrap();
        assert!(groups.values.len() >= MIN_QUERY);
        let before = groups.values.clone();
        let scalar: Fr = values
            .iter()
            .zip(&weights)
            .fold(Fr::ZERO, |acc, (&s, &w)| acc + s * w);
        assert_eq!(
            grouped(&bases, &groups).to_affine(),
            (G2Affine::GENERATOR * scalar).to_affine()
        );
        assert_eq!(groups.values, before);
    }
}
