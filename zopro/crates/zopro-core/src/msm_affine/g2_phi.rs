//! Cube-root bucket-Horner MSM for subgroup-valid G2 key queries.
//! Consume exclusive proof-local scalar storage into two signed components.
//! No persistent point tables; bucket/tree workspace is charged by workspace.rs.
use super::{affine_buckets, batch_size, digit, fill_window_impl, merge_buckets};
use rayon::prelude::*;
#[cfg(test)]
use zopro_bn254::Fr;
// Lattice rows (V1X,-V1Y_ABS),(V2X,V2Y) annihilate (1,lambda) mod r.
// Reciprocals are rounded at 320 bits. Truncating input below bits128/64
// contributes <2^-62 to coordinate error; including nearest rounding gives
// component magnitudes <2^126. Thus signed residuals are recovered modulo2^128.
const C1: [u64; 3] = [7978627105577135859, 15644699364383830999, 2];
const C2: [u64; 4] = [
    11953552847224317658,
    8825887400277225869,
    5534624963584316111,
    2,
];
const V1X: u128 = 9931322734385697763;
const V1Y_ABS: u128 = 147946756881789319000765030803803410728;
const V2X: u128 = 147946756881789319010696353538189108491;
const V2Y: u128 = 9931322734385697763;
#[inline(always)]
fn product<const N: usize, const M: usize, const K: usize>(a: [u64; N], b: [u64; M]) -> [u64; K] {
    assert_eq!(K, N + M);
    let mut out = [0u64; K];
    for i in 0..N {
        let mut carry = 0u128;
        for j in 0..M {
            let t = a[i] as u128 * b[j] as u128 + out[i + j] as u128 + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        out[i + M] = carry as u64;
    }
    out
}
#[inline(always)]
fn sign_magnitude(value: u128) -> u128 {
    let sign = ((value as i128) >> 127) as u128;
    let m = (value ^ sign).wrapping_sub(sign);
    assert!(m < 1u128 << 126);
    m | (sign & (1u128 << 127))
}
pub(super) fn phi_split(s: [u64; 4]) -> [u64; 4] {
    let a = product::<2, 3, 5>([s[2], s[3]], C1);
    let b = product::<3, 4, 7>([s[1], s[2], s[3]], C2);
    debug_assert_eq!(a[4], 0);
    debug_assert_eq!(b[6], 0);
    let q1 = a[3] as u128 + u128::from(a[2] >> 63);
    let q2 = (b[4] as u128 | ((b[5] as u128) << 64)) + u128::from(b[3] >> 63);
    let low = s[0] as u128 | ((s[1] as u128) << 64);
    let k1 = low
        .wrapping_sub(q1.wrapping_mul(V1X))
        .wrapping_sub(q2.wrapping_mul(V2X));
    let k2 = q1.wrapping_mul(V1Y_ABS).wrapping_sub(q2.wrapping_mul(V2Y));
    let (a, b) = (sign_magnitude(k1), sign_magnitude(k2));
    [a as u64, (a >> 64) as u64, b as u64, (b >> 64) as u64]
}
use zopro_bn254::{Fq, G2Affine, G2Config, G2Projective};
const BETA: Fq = Fq::from_decimal(
    "21888242871839275220042445260109153167277707414472061641714758635765020556616",
);
#[cfg(test)]
fn phi_point(mut p: G2Affine) -> G2Affine {
    if !p.is_identity() {
        p.x = p.x.mul_by_fp(&BETA);
    }
    p
}
#[inline]
fn hb_digit(s: &[u64; 4], high: bool, w: usize, bits: usize) -> i16 {
    let i = usize::from(high) * 2;
    {
        let d = digit(&[s[i], s[i + 1] & 0x7fffffffffffffff, 0, 0], w, bits);
        if s[i + 1] >> 63 != 0 {
            -d
        } else {
            d
        }
    }
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
    let bucket = |d: i16| d.unsigned_abs() as usize - 1 + if d < 0 { n } else { 0 };
    let mut counts = vec![0usize; 2 * n];
    for (base, s) in bases.iter().zip(short) {
        let d = hb_digit(s, high, w, bits);
        assert!(d.unsigned_abs() as usize <= n);
        if d != 0 && !base.is_identity() {
            counts[bucket(d)] += 1;
        }
    }
    let mut starts = Vec::with_capacity(2 * n + 1);
    starts.push(0);
    for count in counts {
        starts.push(starts.last().unwrap() + count);
    }
    let len = *starts.last().unwrap();
    if len == 0 {
        return vec![G2Affine::IDENTITY; n];
    }
    let mut positions = starts[..2 * n].to_vec();
    let mut order = vec![0u32; len];
    for (i, (base, s)) in bases.iter().zip(short).enumerate() {
        let d = hb_digit(s, high, w, bits);
        if d != 0 && !base.is_identity() {
            let b = bucket(d);
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    let mut reduced = crate::msm::reduce_indexed_buckets(bases, &order, &starts);
    let mut negative = reduced.split_off(n);
    for p in &mut negative {
        p.y = -p.y;
    }
    merge_buckets(&mut reduced, &negative);
    reduced
}
fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize) -> Vec<G2Affine> {
    if w + 1 == 127usize.div_ceil(bits) && 126 - w * bits <= 8 {
        let top = 126 - w * bits;
        // Each component's temporary index/tree workspace is released before the next.
        let mut high = hb_top_component(bases, short, true, w, bits, top);
        for p in &mut high {
            if !p.is_identity() {
                p.x = p.x.mul_by_fp(&BETA);
            }
        }
        let low = hb_top_component(bases, short, false, w, bits, top);
        merge_buckets(&mut high, &low);
        return high;
    }
    #[cfg(all(
        zopro_g2_horner_buckets,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    if let Some(value) = super::g2_borrowed::try_window(bases, short, w, bits) {
        return value;
    }
    let size = batch_size(bits).max(32);
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    if super::borrowed_scalar::enabled() {
        let mut state = super::borrowed_scalar::Borrowed::new(bits, size);
        state.feed(bases, short.iter().map(|s| hb_digit(s, true, w, bits)));
        phi_buckets(&mut state.inner);
        state.feed(bases, short.iter().map(|s| hb_digit(s, false, w, bits)));
        return affine_buckets(state.inner);
    }
    let mut state = fill_window_impl::<G2Config, false, false>(
        bases,
        short.iter().map(|s| hb_digit(s, true, w, bits)),
        bits,
        size,
    );
    assert!(state.batch.is_empty() && state.queue.is_empty());
    phi_buckets(&mut state);
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
fn phi_buckets(state: &mut super::Window<G2Config>) {
    for p in &mut state.affine {
        if !p.is_identity() {
            p.x = p.x.mul_by_fp(&BETA);
        }
    }
    for p in &mut state.projective {
        p.x = p.x.mul_by_fp(&BETA);
    }
}
// Balance the last two windows across the remaining 23 bits. This
// keeps comparable work in each window for medium-size G2 queries.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const BALANCED_WIDTHS: [usize; 10] = [13, 13, 13, 13, 13, 13, 13, 13, 11, 12];

// Three point partitions keep the six-worker G2 pool busy through the end
// of the small CPU proof. Give the last three windows the same width.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const THREE_PART_WIDTHS: [usize; 10] = [13, 13, 13, 13, 13, 13, 13, 12, 12, 12];

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn window_offsets<const N: usize>(widths: [usize; N]) -> [(usize, usize); N] {
    let mut offset = 0;
    widths.map(|width| {
        let at = offset;
        offset += width;
        (at, width)
    })
}

/// Overlapping Booth digits telescope at variable radix boundaries too:
/// the new digit adds the preceding window's sign bit as its carry.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(always)]
fn component_digit_at(s: &[u64; 4], high: bool, offset: usize, width: usize) -> i16 {
    let i = 2 * usize::from(high);
    let scalar = u128::from(s[i]) | (u128::from(s[i + 1] & (u64::MAX >> 1)) << 64);
    let window = if offset == 0 { scalar << 1 } else { scalar >> (offset - 1) } as u16;
    let mask = (1u16 << width) - 1;
    let magnitude = ((window >> 1) & mask) + (window & 1);
    let subtract = ((window >> width) & 1) << width;
    let digit = magnitude.wrapping_sub(subtract) as i16;
    if s[i + 1] >> 63 != 0 { -digit } else { digit }
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn balanced_msm(bases: &[G2Affine], short: &mut [[u64; 4]]) -> G2Projective {
    if bases.len() <= (1 << 15)
        && rayon::current_num_threads() == 6
        && std::env::var("ZOPRO_CPU_G2_SCALAR_MERGE").as_deref() != Ok("0")
        && std::env::var("ZOPRO_CPU_G2_THREE_PARTS").as_deref() != Ok("0")
    {
        return three_part_msm(bases, short);
    }
    two_part_msm(bases, short)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn two_part_msm(bases: &[G2Affine], short: &mut [[u64; 4]]) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    short.par_iter_mut().for_each(|s| *s = phi_split(*s));
    let windows = window_offsets(BALANCED_WIDTHS);
    let mid = bases.len().div_ceil(2);
    let scalar_merge = bases.len() <= (1 << 15)
        && rayon::current_num_threads() == 6
        && std::env::var("ZOPRO_CPU_G2_SCALAR_MERGE").as_deref() != Ok("0");
    let sums: Vec<_> = windows
        .into_par_iter()
        .map(|(offset, width)| {
            let size = batch_size(width);
            let part = |start: usize, end: usize| {
                let points = &bases[start..end];
                let words = &short[start..end];
                let mut state = super::borrowed_scalar::Borrowed::new(width, size);
                state.feed(points, words.iter().map(|s| component_digit_at(s, true, offset, width)));
                phi_buckets(&mut state.inner);
                state.feed(points, words.iter().map(|s| component_digit_at(s, false, offset, width)));
                affine_buckets(state.inner)
            };
            let (mut left, right) = rayon::join(|| part(0, mid), || part(mid, bases.len()));
            if scalar_merge {
                super::merge_g2_buckets_scalar(&mut left, &right);
            } else {
                merge_buckets(&mut left, &right);
            }
            crate::msm::weighted_affine(&left)
        })
        .collect();
    let mut result = G2Projective::IDENTITY;
    for (width, sum) in BALANCED_WIDTHS.into_iter().zip(sums).rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

// Keep the small-query layout out of the two-part hot path.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(never)]
pub(super) fn three_part_msm<B: std::borrow::Borrow<G2Affine> + Sync>(
    bases: &[B], short: &mut [[u64; 4]],
) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    short.par_iter_mut().for_each(|s| *s = phi_split(*s));
    let chunk = bases.len().div_ceil(3);
    let sums: Vec<_> = window_offsets(THREE_PART_WIDTHS)
        .into_par_iter()
        .map(|(offset, width)| {
            let size = batch_size(width) * 2 / 3;
            let mut parts: Vec<_> = (0..3)
                .into_par_iter()
                .map(|i| {
                    let start = (i * chunk).min(bases.len());
                    let end = ((i + 1) * chunk).min(bases.len());
                    let points = &bases[start..end];
                    let words = &short[start..end];
                    let mut state = super::borrowed_scalar::Borrowed::new(width, size);
                    let pairs = |high| {
                        points.iter().map(std::borrow::Borrow::borrow)
                            .zip(words.iter().map(move |s| component_digit_at(s, high, offset, width)))
                    };
                    let loose = points.len() < (1 << 16);
                    state.feed_pairs(pairs(true), loose);
                    phi_buckets(&mut state.inner);
                    state.feed_pairs(pairs(false), loose);
                    affine_buckets(state.inner)
                })
                .collect();
            let mut left = parts.swap_remove(0);
            for right in parts {
                super::merge_g2_buckets_scalar(&mut left, &right);
            }
            crate::msm::weighted_affine(&left)
        })
        .collect();
    let mut result = G2Projective::IDENTITY;
    for (width, sum) in THREE_PART_WIDTHS.into_iter().zip(sums).rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}


pub(super) fn hb_raw(
    bases: &[G2Affine],
    short: &mut [[u64; 4]],
    bits: usize,
    parts: usize,
) -> G2Projective {
    hb_raw_decomposition(bases, short, bits, parts, false)
}

pub(super) fn hb_raw_decomposition(
    bases: &[G2Affine],
    short: &mut [[u64; 4]],
    bits: usize,
    parts: usize,
    serial: bool,
) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    if serial {
        for word in short.iter_mut() {
            *word = phi_split(*word);
        }
    } else {
        #[cfg(zopro_phi_grain)]
        short
            .par_iter_mut()
            .with_min_len(128)
            .for_each(|s| *s = phi_split(*s));
        #[cfg(not(zopro_phi_grain))]
        short.par_iter_mut().for_each(|s| *s = phi_split(*s));
    }
    from_split(bases, short, bits, parts)
}

pub(super) fn from_split(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    bits: usize,
    parts: usize,
) -> G2Projective {
    assert_eq!(bases.len(), short.len());
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

#[cfg(zopro_g2_horner_buckets)]
pub(super) fn raw_exclusive(bases: &[G2Affine], words: &mut [[u64; 4]]) -> G2Projective {
    assert_eq!(bases.len(), words.len());
    if !(1024..(1 << 16)).contains(&bases.len()) {
        return super::g2_horner::raw(bases, words);
    }
    #[cfg(zopro_resource_schedule)]
    if rayon::current_num_threads() >= 32
        && std::env::var("ZOPRO_CPU_G2_RANGES").as_deref() == Ok("4")
    {
        let growth = super::workspace::four_range_growth(bases.len(), rayon::current_num_threads());
        if let Some(_reservation) = crate::preparation::Reservation::analysis(growth) {
            let serial = std::env::var("ZOPRO_CPU_G2_DECOMPOSITION").as_deref() == Ok("serial");
            return hb_raw_decomposition(bases, words, 11, 4, serial);
        }
    }
    hb_raw(bases, words, 12, 2)
}

// This root acts on G1 by the same scalar as BETA does on G2, so both
// groups can use the existing lattice decomposition.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const G1_BETA: Fq =
    Fq::from_decimal("2203960485148121921418603742825762020974279258880205651966");

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn quotient_g1_msm(
    bases: &[zopro_bn254::G1Affine],
    short: &[[u64; 4]],
) -> zopro_bn254::G1Projective {
    use zopro_bn254::G1Projective;

    assert_eq!(bases.len(), short.len());
    const BITS: usize = 13;
    let mid = bases.len().div_ceil(2);
    let size = batch_size(BITS);
    let sums: Vec<_> = (0..127usize.div_ceil(BITS))
        .into_par_iter()
        .map(|window| {
            let buckets = |range: std::ops::Range<usize>| {
                let points = &bases[range.clone()];
                let words = &short[range];
                let mut state = super::borrowed_scalar::Borrowed::new(BITS, size);
                state.feed(points, words.iter().map(|s| hb_digit(s, true, window, BITS)));
                for point in &mut state.inner.affine {
                    point.x *= G1_BETA;
                }
                for point in &mut state.inner.projective {
                    point.x *= G1_BETA;
                }
                state.feed(points, words.iter().map(|s| hb_digit(s, false, window, BITS)));
                affine_buckets(state.inner)
            };
            let (mut left, right) = rayon::join(|| buckets(0..mid), || buckets(mid..bases.len()));
            merge_buckets(&mut left, &right);
            crate::msm::weighted_affine(&left)
        })
        .collect();
    let mut result = G1Projective::IDENTITY;
    for sum in sums.into_iter().rev() {
        for _ in 0..BITS {
            result = result.double();
        }
        result += sum;
    }
    result
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const WIDE_FUSED_G1_WIDTHS: [usize; 9] = [14, 14, 14, 14, 14, 14, 14, 14, 15];

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const SINGLE_G1_WIDTHS: [usize; 10] = [13, 13, 13, 13, 13, 13, 13, 13, 13, 10];

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn fused_g1_msm(
    k: &[zopro_bn254::G1Affine],
    k_short: &[[u64; 4]],
    h: &[zopro_bn254::G1Affine],
    h_short: &[[u64; 4]],
) -> zopro_bn254::G1Projective {
    let queries = [(k, k_short), (h, h_short)];
    if ((1 << 14)..(1 << 15)).contains(&k.len())
        && std::env::var("ZOPRO_CPU_FUSED_KH_GEOMETRY").as_deref() != Ok("0")
    {
        if h.len() == (1 << 16) - 1 && rayon::current_num_threads() >= 18 {
            // Wider windows cut the number of passes over the dense quotient.
            #[cfg(feature = "trace")]
            log::info!("ZOPRO_TRACE cpu-fused-kh-wide-windows");
            return g1_endomorphism_msm::<2, 2, 9, true>(queries, WIDE_FUSED_G1_WIDTHS, |width| {
                batch_size(width) * 2 / 3
            });
        }
        if h.len() == (1 << 15) - 1 {
            #[cfg(feature = "trace")]
            log::info!("ZOPRO_TRACE cpu-fused-kh-small-windows");
            if rayon::current_num_threads() == crate::APPLE_SMALL_PAIRED_G1_WORKERS {
                // The paired quotient schedule leaves enough workers for one
                // task per wide window, so this proof skips range merges.
                #[cfg(feature = "trace")]
                log::info!("ZOPRO_TRACE cpu-fused-kh-wide-single-range");
                return g1_endomorphism_msm::<1, 2, 9, true>(
                    queries,
                    WIDE_FUSED_G1_WIDTHS,
                    |width| batch_size(width) * 2 / 3,
                );
            }
            return g1_endomorphism_msm::<3, 2, 10, true>(queries, THREE_PART_WIDTHS, batch_size);
        }
    }
    let size = batch_size(13);
    g1_endomorphism_msm::<3, 2, 10, true>(queries, BALANCED_WIDTHS, |_| size)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn witness_g1_msm(
    bases: &[zopro_bn254::G1Affine],
    short: &[[u64; 4]],
) -> zopro_bn254::G1Projective {
    let queries = [(bases, short)];
    if ((1 << 14)..(1 << 15)).contains(&bases.len())
        && rayon::current_num_threads() >= 18
        && std::env::var("ZOPRO_CPU_WITNESS_G1_LAYOUT").as_deref() != Ok("0")
    {
        // One point range per window avoids merging the witness buckets
        // while the pool is also working on the quotient and G2 queries.
        #[cfg(feature = "trace")]
        log::info!("ZOPRO_TRACE cpu-g1-witness-single-range");
        return g1_endomorphism_msm::<1, 1, 10, true>(queries, THREE_PART_WIDTHS, batch_size);
    }
    let size = batch_size(13);
    let top_size = if ((1 << 14)..(1 << 15)).contains(&bases.len())
        && rayon::current_num_threads() == crate::APPLE_SMALL_PAIRED_G1_WORKERS
    {
        // The ten-bit window has frequent collisions when it uses the
        // thirteen-bit windows' large batch, so its queue spills early.
        48
    } else {
        size
    };
    g1_endomorphism_msm::<2, 1, 10, false>(queries, SINGLE_G1_WIDTHS, |width| {
        if width == 10 { top_size } else { size }
    })
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn g1_endomorphism_msm<const PARTS: usize, const N: usize, const W: usize, const MIXED: bool>(
    queries: [(&[zopro_bn254::G1Affine], &[[u64; 4]]); N],
    widths: [usize; W],
    window_batch_size: impl Fn(usize) -> usize + Sync,
) -> zopro_bn254::G1Projective {
    use zopro_bn254::G1Projective;

    const BITS: usize = 13;
    let queries = queries.map(|(bases, words)| {
        assert_eq!(bases.len(), words.len());
        (bases, words, bases.len().div_ceil(PARTS))
    });
    let sums: Vec<_> = window_offsets(widths)
        .into_par_iter()
        .map(|(offset, width)| {
            let size = window_batch_size(width);
            let digit = |word: &[u64; 4], high| {
                if MIXED {
                    component_digit_at(word, high, offset, width)
                } else {
                    hb_digit(word, high, offset / BITS, BITS)
                }
            };
            let mut partials: Vec<_> = (0..PARTS)
                .into_par_iter()
                .map(|part| {
                    let mut state = super::borrowed_scalar::Borrowed::new(width, size);
                    for high in [true, false] {
                        for &(bases, words, chunk) in &queries {
                            let start = (part * chunk).min(bases.len());
                            let end = (start + chunk).min(bases.len());
                            state.feed(
                                &bases[start..end],
                                words[start..end].iter().map(|s| digit(s, high)),
                            );
                        }
                        if high {
                            // Apply the endomorphism to the high component's
                            // bucket sums before adding the low component.
                            for point in &mut state.inner.affine {
                                if !point.is_identity() {
                                    point.x *= G1_BETA;
                                }
                            }
                            for point in &mut state.inner.projective {
                                point.x *= G1_BETA;
                            }
                        }
                    }
                    affine_buckets(state.inner)
                })
                .collect();
            let mut combined = partials.swap_remove(0);
            for part in partials {
                merge_buckets(&mut combined, &part);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = G1Projective::IDENTITY;
    for (sum, width) in sums.into_iter().zip(widths).rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn balanced_windows_reconstruct_signed_components() {
        for magnitude in [0u128, 1, 4095, 4096, 4097, 8191, 1 << 104, 1 << 125, (1 << 126) - 1] {
            for negative in [false, true] {
                let hi = (magnitude >> 64) as u64 | (u64::from(negative) << 63);
                let s = [magnitude as u64, hi, magnitude as u64, hi];
                for high in [false, true] {
                    let expected = if negative { -(magnitude as i128) } else { magnitude as i128 };
                    for widths in [&BALANCED_WIDTHS[..], &THREE_PART_WIDTHS, &WIDE_FUSED_G1_WIDTHS] {
                        let mut result = 0i128;
                        let mut offset = 0;
                        for &width in widths {
                            let digit = component_digit_at(&s, high, offset, width);
                            assert!(digit.unsigned_abs() <= 1 << (width - 1));
                            result += i128::from(digit) << offset;
                            offset += width;
                        }
                        assert_eq!(result, expected);
                    }
                }
            }
        }
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn balanced_g2_sum_matches_scalar_multiplication() {
        let mut rng = crate::testing::Seeded::new(0x47573257494e);
        let mut point = G2Projective::IDENTITY;
        let mut bases = Vec::new();
        let mut words = Vec::new();
        let mut expected = Fr::ZERO;
        for i in 1..124u64 {
            point += G2Affine::GENERATOR;
            let scalar = match i % 9 {
                0 => Fr::ZERO,
                1 => -Fr::ONE,
                _ => rng.fr(),
            };
            if i % 7 == 0 {
                bases.push(G2Affine::IDENTITY);
            } else if i % 5 == 0 {
                bases.push((-point).to_affine());
                expected -= scalar * Fr::from(i);
            } else {
                bases.push(point.to_affine());
                expected += scalar * Fr::from(i);
            }
            words.push(scalar.to_canonical());
        }
        let expected = G2Affine::GENERATOR * expected;
        for workers in [1, 6] {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();
            let mut scalars = words.clone();
            assert_eq!(pool.install(|| balanced_msm(&bases, &mut scalars)), expected);
        }
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn g1_components_match_the_scalar_multiplication_oracle() {
        use zopro_bn254::{G1Affine, G1Projective};

        let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let mut transformed = G1Affine::GENERATOR;
        transformed.x *= G1_BETA;
        assert_eq!(transformed, (G1Affine::GENERATOR * lambda).to_affine());

        let mut point = G1Projective::IDENTITY;
        let mut scalar = Fr::from(918371u64);
        let mut bases = Vec::new();
        let mut scalars = Vec::new();
        let mut expected = G1Projective::IDENTITY;
        for i in 0..512 {
            point += G1Affine::GENERATOR;
            scalar = scalar * Fr::from(6364136223846793005u64) + Fr::ONE;
            let base = if i % 7 == 0 { G1Affine::IDENTITY } else { point.to_affine() };
            let s = match i % 11 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                3 => -scalar,
                _ => scalar,
            };
            bases.push(base);
            scalars.push(phi_split(s.to_canonical()));
            expected += base * s;
        }
        assert_eq!(quotient_g1_msm(&bases, &scalars), expected);
        assert_eq!(witness_g1_msm(&bases, &scalars), expected);
        assert_eq!(
            g1_endomorphism_msm::<1, 1, 10, true>([(&bases, &scalars)], THREE_PART_WIDTHS, batch_size),
            expected
        );
        let queries = [(&bases[..255], &scalars[..255]), (&bases[255..], &scalars[255..])];
        assert_eq!(
            g1_endomorphism_msm::<2, 2, 9, true>(queries, WIDE_FUSED_G1_WIDTHS, |width| {
                batch_size(width) * 2 / 3
            }),
            expected
        );
        assert_eq!(
            g1_endomorphism_msm::<3, 2, 10, true>(queries, THREE_PART_WIDTHS, batch_size),
            expected
        );
        assert_eq!(
            g1_endomorphism_msm::<1, 2, 9, true>(queries, WIDE_FUSED_G1_WIDTHS, |width| {
                batch_size(width) * 2 / 3
            }),
            expected
        );
        for split in [0, 1, 255, 511, 512] {
            assert_eq!(
                fused_g1_msm(
                    &bases[..split], &scalars[..split], &bases[split..], &scalars[split..],
                ),
                expected,
                "split at {split}",
            );
        }
    }

    #[test]
    fn decomposition_and_endomorphism_match_independent_field_oracles() {
        let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let mut values = vec![
            Fr::ZERO,
            Fr::ONE,
            -Fr::ONE,
            lambda,
            lambda - Fr::ONE,
            lambda + Fr::ONE,
        ];
        for bit in 0..254 {
            let p = Fr::from(2u64).pow(&[bit]);
            values.extend([p - Fr::ONE, p, p + Fr::ONE]);
        }
        let mut x = Fr::from(982371u64);
        for _ in 0..1024 {
            x = x * Fr::from(6364136223846793005u64) + Fr::ONE;
            values.push(x);
        }
        for s in values {
            let p = phi_split(s.to_canonical());
            let component = |i: usize| {
                let hi = p[i + 1] & 0x7fffffffffffffff;
                assert!(hi < 1u64 << 62);
                let m = Fr::from_canonical([p[i], hi, 0, 0]).unwrap();
                if p[i + 1] >> 63 != 0 {
                    -m
                } else {
                    m
                }
            };
            assert_eq!(component(0) + lambda * component(2), s);
        }
        let mut point = G2Projective::IDENTITY;
        for i in 0..96 {
            let p = if i % 3 == 0 { -point } else { point }.to_affine();
            assert_eq!(phi_point(p), (p * lambda).to_affine());
            point += G2Affine::GENERATOR;
        }
    }
}
