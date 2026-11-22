//! The native CPU MSM: windowed Pippenger whose buckets are affine
//! points, filled in one pass over the bases in their own order, a batch of
//! additions sharing one inversion (as gnark-crypto's MultiExp does).
//!
//! The lane-form MSM (msm.rs) sorts each window's points into a bucket-
//! ordered copy and reduces it in passes; that suits WebAssembly's SIMD
//! lanes, but a phone's caches do not hold the copies: on a Cortex-A78 it
//! took five times gnark's last-level cache misses per second. Here a
//! window touches the bases once, in order, and its bucket array.
use crate::msm::{BatchAdd, BatchInverse};
use rayon::prelude::*;
use zopro_bn254::{Affine, CurveConfig, Field, Fq, Fr, Projective};
pub(super) mod borrowed_pending;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod borrowed_scalar;
mod grouping;
mod grouping_cost;
#[cfg(zopro_resource_schedule)]
pub(super) use grouping::group_witness_b2_origins;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub(super) use grouping::group_witness_serial_b2;
#[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
pub(super) use grouping::tiled::prove as tiled_grouped_msm;
pub(super) use grouping::{group_witness, group_witness_for, scalar_groups, ScalarGroups};

#[cfg(all(
    zopro_g2_horner_buckets,
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
mod g2_horner;

#[cfg(all(
    zopro_g2_horner_buckets,
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
#[inline]
pub(super) fn g2_query_raw(
    bases: &[zopro_bn254::G2Affine],
    words: &[[u64; 4]],
) -> zopro_bn254::G2Projective {
    g2_horner::raw(bases, words)
}
#[cfg(all(
    zopro_g2_horner_buckets,
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
#[inline]
pub(super) fn g2_query_grouped(
    bases: &[zopro_bn254::G2Affine],
    groups: &ScalarGroups,
) -> zopro_bn254::G2Projective {
    g2_horner::grouped(bases, groups)
}

/// The window width, over the widths whose signed digits fit an i16. The
/// work is the windows' sum of n batch additions and 2^c bucket sums, a
/// sum (projective) weighing two additions, as single-threaded G2 MSMs on
/// an M5 Pro fit. The windows run as the pool's tasks, so among the widths
/// within 15 % of the least work the one whose windows take the fewest
/// rounds on the pool's threads wins: 8 → 2's G2 MSM (190 k points) at
/// width 14 took 19 windows on 18 threads, two rounds, 94 ms a proof; at
/// 15, 17 windows, one round, 74 ms (24 → 2: 240 and 181). Below that the
/// least work: 2 → 2 (54 k) at 11 took 43 ms a proof against 12's 31.
fn width(n: usize) -> usize {
    if n >= (1 << 18) {
        if let Some(width) = std::env::var("ZOPRO_CPU_LARGE_WIDTH")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
        {
            return width.clamp(10, 15);
        }
    }
    if let Some(width) = std::env::var("ZOPRO_CPU_WIDTH")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        return width;
    }
    // Wide bucket tables compete with each other and inversion scratch in
    // the CPU caches. Native IFMA's affine reduction handles the narrow
    // final digit efficiently, so large MSMs can use smaller tables.
    if n >= (1 << 18) && crate::simd::cpu_option("ZOPRO_CPU_CACHE_WIDTH") {
        return 13;
    }
    let bits = Fr::MODULUS_BITS as usize + 1;
    let threads = rayon::current_num_threads().max(1);
    let work = |c: usize| (n + (2usize << c)) * bits.div_ceil(c);
    let least = (4..=15).map(work).min().unwrap();
    (4..=15)
        .filter(|&c| work(c) <= least + least * 15 / 100)
        .min_by_key(|&c| (bits.div_ceil(c).div_ceil(threads), work(c)))
        .unwrap()
}

/// Point ranges per window (ZOPRO_CPU_SPLIT, experiment): more tasks than
/// windows for the pool, each with its own buckets.
fn split() -> usize {
    std::env::var("ZOPRO_CPU_SPLIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1)
}

/// Additions per shared inversion, by width (gnark's): wide enough that the
/// inversion is cheap per addition, narrow enough that a random batch rarely
/// meets a bucket twice.
fn batch_size(width: usize) -> usize {
    let size: usize = match width {
        0..=9 => 0,
        10 => 80,
        11 => 150,
        12 => 200,
        13 => 350,
        14 => 400,
        _ => 500,
    };
    if crate::simd::cpu_option("ZOPRO_CPU_ALIGN_BATCH") {
        size.next_multiple_of(crate::simd::BATCH_LANES)
    } else {
        size
    }
}

pub(super) fn msm<C: CurveConfig>(bases: &[Affine<C>], scalars: &[Fr]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    if scalars.len() >= (1 << 16) {
        if let Some(groups) = group_witness_for::<C>(scalars) {
            return msm_grouped(bases, &groups);
        }
        let canonical: Vec<[u64; 4]> = scalars.par_iter().map(Fr::to_canonical).collect();
        return msm_raw(bases, &canonical);
    }
    let canonical: Vec<[u64; 4]> = scalars.par_iter().map(Fr::to_canonical).collect();
    msm_canonical(bases, &canonical)
}

/// `scalars` in place as their canonical limbs, for msm_canonical: a
/// proof's scalars, which it needs no more after its MSMs.
pub(super) fn canonical(scalars: &mut [Fr]) -> &[[u64; 4]] {
    #[cfg(zopro_canonical_task_grain)]
    scalars
        .par_iter_mut()
        .with_min_len(128)
        .for_each(|s| *s = Fr::from_montgomery(s.to_canonical()));
    #[cfg(not(zopro_canonical_task_grain))]
    scalars
        .par_iter_mut()
        .for_each(|s| *s = Fr::from_montgomery(s.to_canonical()));
    // SAFETY: Fr is repr(transparent) over its four limbs.
    unsafe { std::slice::from_raw_parts(scalars.as_ptr().cast::<[u64; 4]>(), scalars.len()) }
}

/// The MSM of scalars given as canonical limbs. Each window computes its
/// digits from them as it goes (`digit`), so the MSM holds no digits: 63
/// MB for 16 -> 2's five MSMs at once, at no cost in time on a phone.
pub(super) fn msm_canonical<C: CurveConfig>(
    bases: &[Affine<C>],
    canonical: &[[u64; 4]],
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    if crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        if let Some(groups) = grouping::groups_with_diversity_check::<C>(canonical) {
            return msm_grouped(bases, &groups);
        }
    }
    msm_raw(bases, canonical)
}

/// Medium quotient queries can combine two short scalar components
/// before reducing their buckets.
pub(super) fn msm_h_in_place(
    bases: &[zopro_bn254::G1Affine],
    scalars: &mut [Fr],
) -> zopro_bn254::G1Projective {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    if bases.len() == (1 << 16) - 1
        && rayon::current_num_threads() >= 16
        && borrowed_scalar::enabled()
        && std::env::var("ZOPRO_CPU_G1_H_ENDO").as_deref() != Ok("0")
    {
        #[cfg(feature = "trace")]
        log::info!("ZOPRO_TRACE cpu-quotient-g1-endomorphism");
        let short = encode_exclusive(scalars, |s| g2_phi::phi_split(s.to_canonical()));
        return g2_phi::quotient_g1_msm(bases, short);
    }
    msm_canonical(bases, canonical(scalars))
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn apple_fused_kh_enabled(k: usize, h: usize) -> bool {
    let limit = match h {
        32767 | 65535 => 65536,
        131071 => 131072,
        _ => return false,
    };
    (1024..limit).contains(&k)
        && borrowed_scalar::enabled()
        && std::env::var("ZOPRO_CPU_FUSED_KH").as_deref() != Ok("0")
}

/// Compact K while the quotient is running, retaining its workspace reservation.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) struct PreparedAppleK {
    points: Vec<zopro_bn254::G1Affine>,
    groups: ScalarGroups,
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn prepare_apple_k(
    bases: &[zopro_bn254::G1Affine],
    scalars: &[Fr],
    accepted_groups: std::ops::Range<usize>,
    apple_groups: bool,
) -> Option<PreparedAppleK> {
    let mut groups = if apple_groups {
        grouping::group_apple_cpu_witness::<zopro_bn254::G1Config, false>(scalars)
    } else {
        group_witness_for::<zopro_bn254::G1Config>(scalars)
    }?;
    if !accepted_groups.contains(&groups.values.len()) {
        return None;
    }
    let points = crate::msm::group_affine(bases, &groups.order, &groups.starts, &groups.counts);
    groups.values.par_iter_mut().for_each(|v| *v = g2_phi::phi_split(*v));
    Some(PreparedAppleK { points, groups })
}

/// Combine the K and H contributions for rerandomized proofs once the
/// quotient is ready. The scalar arrays belong to this proof.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn apple_fused_kh(
    k: &[zopro_bn254::G1Affine],
    sk: &mut [Fr],
    h: &[zopro_bn254::G1Affine],
    sh: &mut [Fr],
    prepared: Option<PreparedAppleK>,
    paired: bool,
) -> zopro_bn254::G1Projective {
    #[cfg(feature = "trace")]
    log::info!("ZOPRO_TRACE cpu-fused-kh-endomorphism");
    let split = |s: &Fr| g2_phi::phi_split(s.to_canonical());
    let h_words = encode_exclusive(sh, split);
    if let Some(PreparedAppleK { points, groups }) = prepared {
        return g2_phi::fused_g1_msm(&points, &groups.values, h, h_words, paired);
    }
    let k_words = encode_exclusive(sk, split);
    g2_phi::fused_g1_msm(k, k_words, h, h_words, paired)
}

/// Use the shorter endomorphism components after compacting a G1 witness query.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn msm_g1_witness(
    bases: &[zopro_bn254::G1Affine],
    words: &[[u64; 4]],
) -> zopro_bn254::G1Projective {
    if !borrowed_scalar::enabled() || !crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        return msm_canonical(bases, words);
    }
    let Some(groups) = grouping::groups_with_diversity_check::<zopro_bn254::G1Config>(words)
    else {
        return msm_raw(bases, words);
    };
    g1_witness_grouped(bases, groups, false)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn g1_witness_grouped(
    bases: &[zopro_bn254::G1Affine],
    mut groups: ScalarGroups,
    paired: bool,
) -> zopro_bn254::G1Projective {
    if !(1024..32768).contains(&groups.values.len()) {
        return msm_grouped(bases, &groups);
    }
    #[cfg(feature = "trace")]
    log::info!("ZOPRO_TRACE cpu-g1-witness-endomorphism");
    let points = crate::msm::group_affine(bases, &groups.order, &groups.starts, &groups.counts);
    groups.values.par_iter_mut().for_each(|s| *s = g2_phi::phi_split(*s));
    g2_phi::witness_g1_msm(&points, &groups.values, paired)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn apple_a_from_fr(
    bases: &[zopro_bn254::G1Affine],
    scalars: &mut [Fr],
    paired: bool,
) -> zopro_bn254::G1Projective {
    if let Some(groups) = grouping::group_apple_cpu_witness::<zopro_bn254::G1Config, true>(scalars) {
        if borrowed_scalar::enabled()
            && std::env::var("ZOPRO_CPU_WITNESS_G1_ENDO").as_deref() != Ok("0")
        {
            return g1_witness_grouped(bases, groups, paired);
        }
        return msm_grouped(bases, &groups);
    }
    msm_raw(bases, canonical(scalars))
}

pub(super) fn msm_grouped<C: CurveConfig>(
    bases: &[Affine<C>],
    groups: &ScalarGroups,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), groups.len, "validated MSM dimensions");
    let bases = crate::msm::group_affine(bases, &groups.order, &groups.starts, &groups.counts);
    msm_raw(&bases, &groups.values)
}

/// Compact the CPU's G2 query while the small Metal proof runs its G1 work.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn try_apple_compact_g2(
    bases: &[zopro_bn254::G2Affine],
    scalars: &[Fr],
    borrow_singletons: bool,
) -> Option<zopro_bn254::G2Projective> {
    if !(32768..65536).contains(&bases.len())
        || std::env::var("ZOPRO_METAL_COMPACT_G2").as_deref() == Ok("0")
    {
        return None;
    }
    let mut groups = match group_witness_for::<zopro_bn254::G2Config>(scalars) {
        Some(groups) => groups,
        None if borrow_singletons => {
            // The scalar classifier already declined this query.
            let words: Vec<_> = scalars.par_iter().map(Fr::to_canonical).collect();
            return Some(msm_raw(bases, &words));
        }
        None => return None,
    };
    if borrow_singletons
        && (16384..49152).contains(&groups.values.len())
        && borrowed_scalar::enabled()
    {
        // The grouping reservation covers the reduced points and the tree.
        // The borrowed route also builds compact index lists and references.
        let extra = || {
            let duplicates = groups.order.len().checked_sub(groups.values.len())?;
            let starts = grouping_cost::vector::<usize>(duplicates.checked_add(1)?, false)?;
            let counts = grouping_cost::vector::<usize>(duplicates, false)?;
            let order = grouping_cost::vector::<u32>(duplicates.checked_mul(2)?, false)?;
            let points = grouping_cost::vector::<&zopro_bn254::G2Affine>(groups.values.len(), false)?;
            starts.checked_add(counts)?.checked_add(order)?.checked_add(points)
        };
        if let Some(_reservation) = grouping_cost::reserve(scalars.len(), extra()) {
            #[cfg(feature = "trace")]
            log::info!("ZOPRO_TRACE metal-borrowed-g2");
            return Some(apple_b2_borrowed_groups(bases, groups));
        }
    }
    if !((1 << 14)..(1 << 15)).contains(&groups.values.len())
        || !borrowed_scalar::enabled()
        || std::env::var("ZOPRO_CPU_G2_BALANCED").as_deref() == Ok("0")
    {
        return Some(msm_grouped(bases, &groups));
    }
    #[cfg(feature = "trace")]
    log::info!("ZOPRO_TRACE metal-compact-g2-windows");
    let points = crate::msm::group_affine(bases, &groups.order, &groups.starts, &groups.counts);
    Some(g2_phi::balanced_msm(&points, &mut groups.values))
}

/// Admit small Metal queries with enough repeats to benefit from compaction.
#[cfg(gpu)]
pub(super) fn metal_compact_candidate(scalars: &[Fr]) -> bool {
    if scalars.len() < (1 << 14) {
        return false;
    }
    let stride = scalars.len().div_ceil(2048);
    let unique: ahash::AHashSet<_> = scalars.iter().step_by(stride).collect();
    if unique.len() * 4 <= scalars.len().div_ceil(stride) * 3 {
        return true;
    }
    drop(unique);
    // The stride can miss repeats concentrated in runs. Check the larger
    // small queries exactly, stopping as soon as they exceed the stricter
    // density limit. Charge the table before allocating it.
    if !(32768..65536).contains(&scalars.len()) {
        return false;
    }
    let limit = scalars.len() * 64 / 100;
    let capacity = limit + 1;
    let Some(_reservation) = grouping_cost::reserve(
        scalars.len(),
        grouping_cost::hash_table::<&Fr>(capacity),
    ) else {
        return false;
    };
    let mut distinct = ahash::AHashSet::new();
    if distinct.try_reserve(capacity).is_err() {
        return false;
    }
    scalars.iter().all(|value| {
        distinct.insert(value);
        distinct.len() <= limit
    })
}

#[cfg(gpu)]
pub(super) fn group_for_metal(
    bases: &[zopro_bn254::G1Affine],
    scalars: &[Fr],
    compact_overlap: bool,
) -> Option<(
    Vec<zopro_bn254::G1Affine>,
    Vec<[u64; 4]>,
    crate::preparation::Reservation,
)> {
    let min = if compact_overlap { 1 << 14 } else { 1 << 16 };
    if bases.len() != scalars.len() || bases.len() < min {
        return None;
    }
    let groups = group_witness_for::<zopro_bn254::G1Config>(scalars)?;
    if groups.values.is_empty() {
        return None;
    }
    let ScalarGroups {
        values,
        order,
        starts,
        counts,
        _reservation: reservation,
        ..
    } = groups;
    let points = crate::msm::group_affine(bases, &order, &starts, &counts);
    drop((order, starts, counts));
    // Release compaction scratch credit before the other queries start.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let reservation = {
        use crate::preparation::allocation_bytes;
        let retained = allocation_bytes(&points)
            .zip(allocation_bytes(&values))
            .and_then(|(points, values)| points.checked_add(values));
        reservation.retain(retained)
    };
    Some((points, values, reservation))
}

fn msm_grouped_prepared<C: CurveConfig>(
    bases: &[Affine<C>],
    sums: &[Affine<C>],
    groups: &ScalarGroups,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), groups.len);
    let combined = crate::msm::group_affine_prepared(
        bases,
        sums,
        &groups.order,
        &groups.starts,
        &groups.counts,
    );
    msm_raw(&combined, &groups.values)
}

pub(super) fn msm_in_place<C: CurveConfig>(bases: &[Affine<C>], scalars: &mut [Fr]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    if let Some(groups) = group_witness_for::<C>(scalars) {
        msm_grouped(bases, &groups)
    } else {
        msm_raw(bases, canonical(scalars))
    }
}

/// Treat Montgomery words as scalar integers, then undo their common R
/// factor on the single output point. This avoids reducing every scalar.
pub(super) fn msm_montgomery<C: CurveConfig>(bases: &[Affine<C>], scalars: &[Fr]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    // SAFETY: Fr is repr(transparent) over its four initialized limbs.
    let words =
        unsafe { std::slice::from_raw_parts(scalars.as_ptr().cast::<[u64; 4]>(), scalars.len()) };
    #[cfg(all(
        zopro_resource_schedule,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    let result = dense_h::try_sum(bases, words).unwrap_or_else(|| msm_raw(bases, words));
    #[cfg(not(all(
        zopro_resource_schedule,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    let result = msm_raw(bases, words);
    let inverse_r = Fr::from_montgomery([1, 0, 0, 0]);
    result.mul(&inverse_r)
}

/// Combine equal scalars using an affine addition tree. Inspect a small
/// sample first, so random FFT coefficients keep their streaming path.
#[cfg(test)]
fn group_scalars<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
) -> Option<(Vec<Affine<C>>, Vec<[u64; 4]>)>
where
    C::Base: BatchInverse,
{
    let groups = scalar_groups(scalars)?;
    Some((
        crate::msm::group_affine(bases, &groups.order, &groups.starts, &groups.counts),
        groups.values,
    ))
}

/// MSM of canonical words after the caller has already chosen grouping.
pub(super) fn msm_raw<C: CurveConfig>(bases: &[Affine<C>], canonical: &[[u64; 4]]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), canonical.len(), "validated MSM dimensions");
    let n = bases.len();
    if n == 0 {
        return Projective::<C>::IDENTITY;
    }
    if (1024..65536).contains(&n) && C::Base::deferred_sign() {
        let bits = std::env::var("ZOPRO_CPU_DEFER_SIGN_WIDTH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(13usize)
            .clamp(10, 14);
        let parts = std::env::var("ZOPRO_CPU_DEFER_SIGN_PARTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2usize)
            .clamp(1, 2);
        return merge_split_width(bases, canonical, parts, bits);
    }
    // Individual scalar multiplications in the unsigned fallback are
    // useful for reference checks, but small production MSMs keep their
    // existing shared-window path.
    if n >= 256 && crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED") {
        let delta = std::env::var("ZOPRO_CPU_UNSIGNED_DELTA")
            .ok()
            .and_then(|v| v.parse::<isize>().ok())
            .unwrap_or(0)
            .clamp(-4, 4);
        let explicit_bits = std::env::var("ZOPRO_CPU_UNSIGNED_WIDTH")
            .ok()
            .and_then(|v| v.parse::<usize>().ok());
        let mut bits = explicit_bits
            .unwrap_or_else(|| width(n).saturating_add_signed(delta))
            .clamp(3, 15);
        if explicit_bits.is_none() && crate::simd::cpu_option("ZOPRO_CPU_UNSIGNED_CACHE") {
            let point = core::mem::size_of::<Affine<C>>();
            while bits > 3 {
                let working =
                    (1usize << bits) * (point + 4) + batch_size(bits + 1) * (point * 7 / 2 + 48);
                if working <= 400 * 1024 {
                    break;
                }
                bits -= 1;
            }
        }
        let default_parts = if (1024..(1 << 18)).contains(&n)
            && rayon::current_num_threads() >= 16
            && crate::simd::cpu_option("ZOPRO_CPU_MERGE_SPLIT")
        {
            2
        } else {
            1
        };
        let parts = std::env::var("ZOPRO_CPU_UNSIGNED_PARTS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(default_parts)
            .clamp(1, 32);
        return msm_unsigned_raw(bases, canonical, bits, parts);
    }
    // Two ranges give wide Apple silicon pools shorter tasks through the
    // last window. Cap the range to the query sizes tested on that target.
    let split_apple = cfg!(all(target_os = "macos", target_arch = "aarch64"))
        && ((1 << 15)..(1 << 19)).contains(&n);
    let split_ifma = (1024..(1 << 18)).contains(&n)
        && crate::simd::cpu_option("ZOPRO_CPU_MERGE_SPLIT");
    let default_parts = if rayon::current_num_threads() >= 16 && (split_apple || split_ifma)
    {
        2
    } else {
        1
    };
    let merge_parts = std::env::var("ZOPRO_CPU_MERGE_SPLIT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default_parts)
        .clamp(1, 32);
    if merge_parts > 1 && n >= 1024 {
        return merge_split(bases, canonical, merge_parts);
    }
    let width = width(n);
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    let parts = split();
    let part = n.div_ceil(parts);
    let pieces: Vec<Projective<C>> = (0..windows * parts)
        .into_par_iter()
        .map(|task| {
            let (w, p) = (task / parts, task % parts);
            let range = p * part..((p + 1) * part).min(n);
            if let Some(sum) =
                top_window::<C, true>(&bases[range.clone()], &canonical[range.clone()], w, width)
            {
                return sum;
            }
            window::<C>(
                &bases[range.clone()],
                canonical[range].iter().map(|s| digit(s, w, width)),
                width,
            )
        })
        .collect();
    let sums: Vec<Projective<C>> = pieces.chunks(parts).map(|c| c.iter().sum()).collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

/// Positive radix digits use an extra bucket bit in exchange for no
/// carry propagation and no conditional negation of point coordinates.
/// Scalar words are canonical Fr values, hence have at most 254 bits.
#[inline(always)]
fn unsigned_digit(words: &[u64; 4], w: usize, bits: usize) -> i16 {
    let offset = w * bits;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { words[word] >> shift } else { 0 };
    if shift + bits > 64 && word < 3 {
        value |= words[word + 1] << (64 - shift);
    }
    (value & ((1u64 << bits) - 1) & 0x7fff) as i16
}

fn msm_unsigned_raw<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    bits: usize,
    parts: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), scalars.len());
    assert!((3..=15).contains(&bits));
    let n = bases.len();
    if n < 256 {
        return bases.iter().zip(scalars).map(|(p, s)| p.mul_limbs(s)).sum();
    }
    let parts = parts.clamp(1, 32);
    let part = n.div_ceil(parts);
    let windows = (Fr::MODULUS_BITS as usize).div_ceil(bits);
    // The final bucket is unused padding. A power-of-two table preserves
    // the existing affine tree's bucket-group layout and weights.
    let size = batch_size(bits + 1).max(32);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            if let Some(sum) = top_window::<C, false>(bases, scalars, w, bits) {
                return sum;
            }
            if parts == 1 {
                return crate::msm::weighted_affine(&affine_buckets(fill_window_impl::<
                    C,
                    false,
                    true,
                >(
                    bases,
                    scalars.iter().map(|s| unsigned_digit(s, w, bits)),
                    bits + 1,
                    size,
                )));
            }
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .filter_map(|p| {
                    let first = p * part;
                    let end = ((p + 1) * part).min(n);
                    (first < end).then(|| {
                        affine_buckets(fill_window_impl::<C, false, true>(
                            &bases[first..end],
                            scalars[first..end]
                                .iter()
                                .map(|s| unsigned_digit(s, w, bits)),
                            bits + 1,
                            size,
                        ))
                    })
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for other in buckets {
                merge_buckets(&mut combined, &other);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..bits {
            result = result.double();
        }
        result += sum;
    }
    result
}

/// Scalar `words`' signed digit in window `w`, in (-2^(width-1),
/// 2^(width-1)]: the window's bits plus the carry out of the window below,
/// which is one when that window's bits are at least half the radix, none
/// when they are less than half less one, and the carry into it when they
/// are just that (rarely: one in 2^width). The point at infinity's windows
/// skip it.
#[inline(always)]
fn digit(words: &[u64; 4], w: usize, width: usize) -> i16 {
    #[cfg(zopro_booth_digits)]
    {
        booth_digit(words, w, width)
    }
    #[cfg(not(zopro_booth_digits))]
    {
    let half = 1u64 << (width - 1);
    let bits = |w: usize| -> u64 {
        let offset = w * width;
        let (word, shift) = (offset / 64, offset % 64);
        let mut value = if word < 4 { words[word] >> shift } else { 0 };
        if shift + width > 64 && word < 3 {
            value |= words[word + 1] << (64 - shift);
        }
        value & ((1u64 << width) - 1)
    };
    let mut carry = 0;
    let mut below = w;
    while below > 0 {
        let b = bits(below - 1);
        if b != half - 1 {
            carry = (b >= half) as u64;
            break;
        }
        below -= 1;
    }
    let value = bits(w) + carry;
    let out = (value + half) >> width;
    (value as i64 - (out << width) as i64) as i16
    }
}

/// Overlapping radix-2^width Booth digits. Incoming carry is the previous
/// unsigned window's sign bit, so no backwards carry-chain scan is needed.
/// Digits lie in [-2^(width-1),+2^(width-1)] and telescope to the input.
#[inline(always)]
fn booth_digit(words: &[u64; 4], w: usize, width: usize) -> i16 {
    if w == 0 {
        let value = words[0] & ((1u64 << width) - 1);
        return (value as i64 - ((value >> (width - 1)) << width) as i64) as i16;
    }
    let offset = w * width - 1;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { words[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        value |= words[word + 1] << (64 - shift);
    }
    value &= (1u64 << (width + 1)) - 1;
    (((value + 1) >> 1) as i64 - ((value >> width) << width) as i64) as i16
}

fn window<C: CurveConfig>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    width: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    let half = 1usize << (width - 1);
    let size = batch_size(width);
    if size == 0 {
        return window_projective(bases, digits, half);
    }
    let w = fill_window::<C>(bases, digits, width, size);
    if crate::simd::cpu_option("ZOPRO_CPU_REDUCE") {
        return crate::msm::weighted_affine(&affine_buckets(w));
    }
    let mut running = Projective::<C>::IDENTITY;
    let mut total = Projective::<C>::IDENTITY;
    for k in (0..half).rev() {
        running += w.affine[k];
        let index = w.projective_indices[k];
        if index != u32::MAX {
            running += w.projective[index as usize];
        }
        total += running;
    }
    total
}

fn fill_window<C: CurveConfig>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    width: usize,
    size: usize,
) -> Window<C>
where
    C::Base: BatchInverse,
{
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    if borrowed_scalar::enabled() {
        let mut state = borrowed_scalar::Borrowed::new(width, size);
        state.feed(bases, digits);
        return state.inner;
    }
    if crate::simd::cpu_option("ZOPRO_CPU_MASKED_SIGN") {
        fill_window_impl::<C, true, false>(bases, digits, width, size)
    } else {
        fill_window_impl::<C, false, false>(bases, digits, width, size)
    }
}

fn fill_window_impl<C: CurveConfig, const MASKED: bool, const POSITIVE: bool>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    width: usize,
    size: usize,
) -> Window<C>
where
    C::Base: BatchInverse,
{
    fill_window_deferred::<C, MASKED, POSITIVE>(
        bases,
        digits,
        width,
        size,
        !POSITIVE && C::Base::deferred_sign(),
    )
}
fn fill_window_deferred<C: CurveConfig, const MASKED: bool, const POSITIVE: bool>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    width: usize,
    size: usize,
    defer: bool,
) -> Window<C>
where
    C::Base: BatchInverse,
{
    let mut w = Window::<C>::new(1usize << (width - 1), size);
    w.defer_sign = defer;
    for (base, digit) in bases.iter().zip(digits) {
        if digit == 0 || base.is_identity() {
            continue;
        }
        debug_assert!(!POSITIVE || digit >= 0);
        let bucket = if POSITIVE {
            digit as usize - 1
        } else {
            digit.unsigned_abs() as usize - 1
        };
        let point = if POSITIVE || defer {
            *base
        } else if MASKED {
            Affine::new(base.x, C::Base::negate_if(base.y, digit < 0))
        } else if digit < 0 {
            -*base
        } else {
            *base
        };
        let encoded=bucket as u32|if defer&&digit<0 {1<<31}else{0};
        if w.busy(bucket) {
            w.queue.push((encoded, point));
            if w.queue.len() == size - 1 {
                w.flush_queue();
            }
            continue;
        }
        w.add_signed(encoded, point);
        if w.batch.len() == size {
            w.execute();
            w.take_from_queue();
        }
    }
    w.execute();
    w.flush_queue();
    w
}

fn affine_buckets<C: CurveConfig>(mut w: Window<C>) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    let combined: Vec<_> = w
        .projective
        .iter()
        .zip(&w.projective_buckets)
        .map(|(point, &i)| *point + w.affine[i])
        .collect();
    for (point, &i) in Projective::<C>::batch_to_affine(&combined)
        .into_iter()
        .zip(&w.projective_buckets)
    {
        w.affine[i] = point;
    }
    w.affine
}

/// Merge disjoint equal-weight buckets using the streaming affine kernel.
fn merge_buckets<C: CurveConfig>(left: &mut [Affine<C>], right: &[Affine<C>])
where
    C::Base: BatchInverse,
{
    assert_eq!(left.len(), right.len());
    left.par_chunks_mut(512)
        .zip(right.par_chunks(512))
        .for_each(|(left, right)| {
            let pairs = bucket_pairs(left, right, |point| *point);
            if !pairs.is_empty() {
                BatchAdd::<C>::default().apply(left, &pairs);
            }
        });
}

// Handle exceptional sums before putting distinct-x pairs into a shared inversion.
fn bucket_pairs<'a, C: CurveConfig, P>(
    left: &mut [Affine<C>],
    right: &'a [Affine<C>],
    point: impl Fn(&'a Affine<C>) -> P,
) -> Vec<(u32, P)> {
    let mut pairs = Vec::with_capacity(left.len());
    for (i, (a, b)) in left.iter_mut().zip(right).enumerate() {
        if a.is_identity() {
            *a = *b;
        } else if b.is_identity() {
        } else if a.x == b.x {
            *a = (*a + *b).to_affine();
        } else {
            pairs.push((i as u32, point(b)));
        }
    }
    pairs
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn merge_g2_buckets_scalar(left: &mut [zopro_bn254::G2Affine], right: &[zopro_bn254::G2Affine]) {
    assert_eq!(left.len(), right.len());
    const CHUNK: usize = 512;
    left.par_chunks_mut(CHUNK)
        .zip(right.par_chunks(CHUNK))
        .for_each(|(left, right)| {
            let pairs = bucket_pairs(left, right, |point| point);
            if !pairs.is_empty() {
                let mut busy = [0u64; CHUNK / 64];
                let mut denominators = Vec::with_capacity(pairs.len());
                let mut prefixes = Vec::with_capacity(pairs.len());
                apply_scalar_batch::<zopro_bn254::G2Config, _, false>(
                    left,
                    &mut busy,
                    &pairs,
                    &mut denominators,
                    &mut prefixes,
                );
            }
        });
}

fn merge_split<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    parts: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    merge_split_width(bases, scalars, parts, width(bases.len()))
}
fn merge_split_width<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    parts: usize,
    width: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    if let Some(result) = C::Base::borrowed_msm(bases, scalars, parts, width) {
        return result;
    }
    merge_split_width_owned(bases, scalars, parts, width)
}
fn merge_split_width_owned<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    parts: usize,
    width: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    let part = bases.len().div_ceil(parts);
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    let size = batch_size(width).max(32);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            if let Some(sum) = top_window::<C, true>(bases, scalars, w, width) {
                return sum;
            }
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|p| {
                    let range = p * part..((p + 1) * part).min(bases.len());
                    affine_buckets(fill_window::<C>(
                        &bases[range.clone()],
                        scalars[range].iter().map(|s| digit(s, w, width)),
                        width,
                        size,
                    ))
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for other in buckets {
                merge_buckets(&mut combined, &other);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

/// A short final digit uses only a few buckets. Streaming these collisions
/// would send most points through the scalar projective fallback.
fn top_window<C: CurveConfig, const SIGNED: bool>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    w: usize,
    width: usize,
) -> Option<Projective<C>>
where
    C::Base: BatchInverse,
{
    let windows = (Fr::MODULUS_BITS as usize + usize::from(SIGNED)).div_ceil(width);
    if bases.len() < 1024 || w + 1 != windows {
        return None;
    }
    let bits = Fr::MODULUS_BITS as usize - w * width;
    if bits > 8 || !crate::simd::cpu_option("ZOPRO_CPU_TOP_TREE") {
        return None;
    }
    let buckets = 1usize << bits;
    let mut counts = vec![0usize; buckets];
    for (base, s) in bases.iter().zip(scalars) {
        let d = if SIGNED {
            digit(s, w, width)
        } else {
            unsigned_digit(s, w, width)
        };
        debug_assert!(d >= 0 && d as usize <= buckets);
        if d != 0 && !base.is_identity() {
            counts[d as usize - 1] += 1;
        }
    }
    let mut starts = Vec::with_capacity(buckets + 1);
    starts.push(0);
    for count in counts {
        starts.push(starts.last().unwrap() + count);
    }
    if *starts.last().unwrap() == 0 {
        return Some(Projective::IDENTITY);
    }
    let mut positions = starts[..buckets].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, (base, s)) in bases.iter().zip(scalars).enumerate() {
        let d = if SIGNED {
            digit(s, w, width)
        } else {
            unsigned_digit(s, w, width)
        };
        if d != 0 && !base.is_identity() {
            let b = d as usize - 1;
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    let buckets = crate::msm::reduce_indexed_buckets(bases, &order, &starts);
    Some(crate::msm::weighted_affine(&buckets))
}

/// Narrow windows: few buckets, so projective buckets without batching.
fn window_projective<C: CurveConfig>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    half: usize,
) -> Projective<C> {
    let mut buckets = vec![Projective::<C>::IDENTITY; half];
    for (base, digit) in bases.iter().zip(digits) {
        if digit > 0 {
            buckets[digit as usize - 1] += base;
        } else if digit < 0 {
            buckets[digit.unsigned_abs() as usize - 1] -= base;
        }
    }
    let mut running = Projective::<C>::IDENTITY;
    let mut total = Projective::<C>::IDENTITY;
    for bucket in buckets.iter().rev() {
        running += bucket;
        total += running;
    }
    total
}

/// A window's buckets and its pending batch of affine additions.
struct Window<C: CurveConfig>
where
    C::Base: BatchInverse,
{
    /// The buckets the batches fill.
    affine: Vec<Affine<C>>,
    /// The rare additions a batch cannot take (doublings, and the queue
    /// when it overflows), in projective form.
    projective: Vec<Projective<C>>,
    projective_indices: Vec<u32>,
    projective_buckets: Vec<usize>,
    /// The buckets in the current batch.
    busy: Vec<u64>,
    /// The batch: bucket and point to add to it.
    batch: Vec<(u32, Affine<C>)>,
    /// Points whose bucket was in the batch when they came.
    queue: Vec<(u32, Affine<C>)>,
    /// The batch inversion's scratch.
    denominators: Vec<C::Base>,
    prefixes: Vec<<C::Base as BatchInverse>::Product>,
    packed: BatchAdd<C>,
    use_packed: bool,
    defer_sign: bool,
}

impl<C: CurveConfig> Window<C>
where
    C::Base: BatchInverse,
{
    fn new(half: usize, size: usize) -> Self {
        Self {
            affine: vec![Affine::<C>::IDENTITY; half],
            projective: Vec::new(),
            projective_indices: vec![u32::MAX; half],
            projective_buckets: Vec::new(),
            busy: vec![0; half.div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            denominators: Vec::with_capacity(size),
            prefixes: Vec::with_capacity(size),
            packed: BatchAdd::default(),
            use_packed: crate::simd::cpu_option("ZOPRO_CPU_PACKED"),
            defer_sign: false,
        }
    }

    #[inline(always)]
    fn busy(&self, bucket: usize) -> bool {
        self.busy[bucket / 64] >> (bucket % 64) & 1 == 1
    }

    /// Adds `point` to a bucket not in the batch: at once when the bucket is
    /// empty, or the sum is a doubling or zero; else into the batch.
    #[inline(always)]
    fn add(&mut self, bucket: usize, point: Affine<C>) {
        self.add_signed(u32::try_from(bucket).expect("bucket index fits u32"), point);
    }
    #[inline(always)]
    fn add_signed(&mut self, encoded: u32, point: Affine<C>) {
        if self.stage_pending::<false>(encoded, &point) {
            self.batch.push((encoded, point));
        }
    }

    #[inline(always)]
    fn stage_pending<const CHECK_LOW_WORD: bool>(&mut self, encoded: u32, point: &Affine<C>) -> bool {
        let bucket = (encoded & 0x7fffffff) as usize;
        let negative = encoded >> 31 != 0;
        let current = &mut self.affine[bucket];
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        if CHECK_LOW_WORD {
            // A nonzero word excludes the identity. Distinct words exclude
            // the coincident-x cases that need the complete formulas.
            let low = C::Base::affine_x_word(&current.x);
            if low != 0 && low != C::Base::affine_x_word(&point.x) {
                self.busy[bucket / 64] |= 1 << (bucket % 64);
                return true;
            }
        }
        if current.is_identity() {
            *current = if negative { -*point } else { *point };
            return false;
        }
        if current.x == point.x {
            let point=if negative { -*point } else { *point };
            if current.y == point.y {
                self.add_projective(bucket, point);
            } else {
                *current = Affine::<C>::IDENTITY;
            }
            return false;
        }
        self.busy[bucket / 64] |= 1 << (bucket % 64);
        true
    }

    /// Most buckets never need the exceptional projective path. Keep only
    /// those that do, rather than a second dense point array per window.
    fn add_projective(&mut self, bucket: usize, point: Affine<C>) {
        let index = self.projective_indices[bucket];
        if index == u32::MAX {
            self.projective_indices[bucket] = self.projective.len() as u32;
            self.projective_buckets.push(bucket);
            self.projective.push(point.to_projective());
        } else {
            self.projective[index as usize] += point;
        }
    }

    /// The batch's additions, bucket = bucket + point, with one inversion:
    /// λ = (y2 - y1) / (x2 - x1), x3 = λ² - x1 - x2, y3 = λ (x1 - x3) - y1.
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        if self.defer_sign && self.use_packed {
            self.packed.apply_signed(&mut self.affine, &self.batch);
            for (index, _) in &self.batch {
                self.busy[(index & 0x7fffffff) as usize / 64] = 0;
            }
            self.batch.clear();
            return;
        }
        if self.defer_sign {
            for (index, p) in &mut self.batch {
                if *index >> 31 != 0 {
                    *p = -*p;
                }
                *index &= 0x7fffffff;
            }
        }
        if self.use_packed {
            self.packed.apply(&mut self.affine, &self.batch);
            for (bucket, _) in &self.batch {
                self.busy[*bucket as usize / 64] = 0;
            }
            self.batch.clear();
            return;
        }
        apply_scalar_batch::<C, _, false>(
            &mut self.affine,
            &mut self.busy,
            &self.batch,
            &mut self.denominators,
            &mut self.prefixes,
        );
        self.batch.clear();
    }

    /// The queued points whose buckets are free again, newest first, into
    /// the next batch.
    fn take_from_queue(&mut self) {
        while let Some(&(bucket, point)) = self.queue.last() {
            if self.busy((bucket&0x7fffffff)as usize) {
                return;
            }
            self.queue.pop();
            self.add_signed(bucket, point);
        }
    }

    /// The queue's points into the projective buckets, unbatched.
    fn flush_queue(&mut self) {
        for i in 0..self.queue.len() {
            let (bucket, point) = self.queue[i];
            self.add_projective(
                (bucket & 0x7fffffff) as usize,
                if bucket >> 31 != 0 { -point } else { point },
            );
        }
        self.queue.clear();
    }
}

fn apply_scalar_batch<C: CurveConfig, P: std::borrow::Borrow<Affine<C>>, const SIGNED: bool>(
    affine: &mut [Affine<C>],
    busy: &mut [u64],
    batch: &[(u32, P)],
    denominators: &mut Vec<C::Base>,
    prefixes: &mut Vec<<C::Base as BatchInverse>::Product>,
) where
    C::Base: BatchInverse,
{
    let index = |encoded: u32| if SIGNED { encoded & 0x7fff_ffff } else { encoded } as usize;
    denominators.clear();
    prefixes.clear();
    // G2 uses the denominators' norms so its batch also inverts in Fq.
    let mut product = Fq::ONE;
    for (encoded, point) in batch {
        let denominator = point.borrow().x - affine[index(*encoded)].x;
        prefixes.push(C::Base::prefix(&mut product, denominator));
        denominators.push(denominator);
    }
    let mut inverse = product.inverse().expect("distinct x in a batch");
    for (j, (encoded, point)) in batch.iter().enumerate().rev() {
        let bucket = index(*encoded);
        let denominator = denominators[j];
        let mut prefix = prefixes[j];
        C::Base::reverse(&mut inverse, denominator, &mut prefix);
        let lambda_denominator = C::Base::denominator_inverse(denominator, prefix);
        let point = point.borrow();
        let current = &mut affine[bucket];
        let y = if SIGNED && *encoded >> 31 != 0 { -point.y } else { point.y };
        let lambda = (y - current.y) * lambda_denominator;
        let x = lambda.square() - current.x - point.x;
        let y = lambda * (current.x - x) - current.y;
        *current = Affine::<C>::new(x, y);
        busy[bucket / 64] = 0;
    }
}

/// A key-only partition. The proof checks equality before selecting a
/// precomputed four-point sum; unequal groups retain every original term.
struct EqualLayout {
    n: usize,
    groups: Vec<[u32; 4]>,
    rest: Vec<u32>,
}
impl EqualLayout {
    fn for_wires(indices: &[u32], groups: &[[usize; 4]], n_values: usize) -> Option<Self> {
        let mut positions = vec![u32::MAX; n_values];
        for (i, &wire) in indices.iter().enumerate() {
            positions[wire as usize] = i as u32;
        }
        let groups: Vec<_> = groups
            .iter()
            .filter_map(|g| {
                let positions = g.map(|wire| positions[wire]);
                positions
                    .iter()
                    .all(|&i| i != u32::MAX)
                    .then_some(positions)
            })
            .collect();
        (groups.len() >= 256 && groups.len() * 16 >= indices.len())
            .then(|| EqualLayout::new(indices.len(), groups))
    }

    fn new(n: usize, groups: Vec<[u32; 4]>) -> Self {
        assert!(n
            .checked_add(groups.len())
            .is_some_and(|n| n < u32::MAX as usize));
        let mut used = vec![false; n];
        for group in &groups {
            for &index in group {
                assert!((index as usize) < n && !used[index as usize]);
                used[index as usize] = true;
            }
        }
        let rest = (0..n).filter(|&i| !used[i]).map(|i| i as u32).collect();
        Self { n, groups, rest }
    }
    fn source_groups(&self, input: &[Fr]) -> Option<ScalarGroups> {
        assert_eq!(input.len(), self.n);
        let mut scalars = Vec::with_capacity(self.n);
        let mut sources = Vec::with_capacity(self.n);
        for (i, group) in self.groups.iter().enumerate() {
            let values = group.map(|index| input[index as usize]);
            if values[1..].iter().all(|v| *v == values[0]) {
                scalars.push(values[0]);
                sources.push((self.n + i) as u32);
            } else {
                scalars.extend(values);
                sources.extend(group);
            }
        }
        for &index in &self.rest {
            scalars.push(input[index as usize]);
            sources.push(index);
        }
        if scalars.len() == self.n {
            return None;
        }
        let mut grouped = if let Some(g) = group_witness(&scalars) {
            g
        } else {
            if scalars.len() > self.n * 3 / 4 {
                return None;
            }
            let n = scalars.len();
            let reservation = grouping_cost::reserve(n, grouping_cost::peak(n, 100))?;
            let values = scalars.par_iter().map(Fr::to_canonical).collect();
            ScalarGroups {
                _reservation: reservation,
                len: n,
                values,
                order: (0..n as u32).collect(),
                starts: (0..=n).collect(),
                counts: vec![1; n],
            }
        };
        grouped
            .order
            .iter_mut()
            .for_each(|i| *i = sources[*i as usize]);
        grouped.len = self.n;
        Some(grouped)
    }

    #[cfg(test)]
    fn scalars(&self, input: &[Fr]) -> (Vec<Fr>, Vec<bool>) {
        assert_eq!(input.len(), self.n);
        let mut scalars = Vec::with_capacity(self.n);
        let mut equal = Vec::with_capacity(self.groups.len());
        for group in &self.groups {
            let values = group.map(|index| input[index as usize]);
            let same = values[1..].iter().all(|value| *value == values[0]);
            equal.push(same);
            if same {
                scalars.push(values[0]);
            } else {
                scalars.extend_from_slice(&values);
            }
        }
        scalars.extend(self.rest.iter().map(|&i| input[i as usize]));
        (scalars, equal)
    }
    fn prepare<C: CurveConfig>(&self, bases: &[Affine<C>]) -> Vec<Affine<C>>
    where
        C::Base: BatchInverse,
    {
        assert_eq!(bases.len(), self.n);
        let order: Vec<_> = self.groups.iter().flatten().copied().collect();
        let starts: Vec<_> = (0..=self.groups.len()).map(|i| 4 * i).collect();
        crate::msm::group_affine(bases, &order, &starts, &vec![4; self.groups.len()])
    }
    #[cfg(test)]
    fn bases<C: CurveConfig>(
        &self,
        original: &[Affine<C>],
        sums: &[Affine<C>],
        equal: &[bool],
        count: usize,
    ) -> Vec<Affine<C>> {
        assert_eq!(original.len(), self.n);
        assert_eq!(sums.len(), self.groups.len());
        assert_eq!(equal.len(), self.groups.len());
        let mut bases = Vec::with_capacity(count);
        for ((group, &sum), &same) in self.groups.iter().zip(sums).zip(equal) {
            if same {
                bases.push(sum);
            } else {
                bases.extend(group.map(|i| original[i as usize]));
            }
        }
        bases.extend(self.rest.iter().map(|&i| original[i as usize]));
        assert_eq!(bases.len(), count);
        bases
    }
}

type PreparedG1 = (EqualLayout, Vec<zopro_bn254::G1Affine>);
type PreparedB = (
    EqualLayout,
    Vec<zopro_bn254::G1Affine>,
    Vec<zopro_bn254::G2Affine>,
);
pub(super) struct PreparedEqualWitness {
    a: Option<PreparedG1>,
    b: Option<PreparedB>,
    k: Option<(bool, PreparedG1)>,
    pub(super) reservation: Option<crate::preparation::Reservation>,
}
impl PreparedEqualWitness {
    pub(super) fn new(
        a: &[zopro_bn254::G1Affine],
        b: &[zopro_bn254::G1Affine],
        b2: &[zopro_bn254::G2Affine],
        a_indices: &[u32],
        b_indices: &[u32],
        groups: &[[usize; 4]],
        n_values: usize,
    ) -> Option<Self> {
        assert_eq!(a.len(), a_indices.len());
        assert_eq!(b.len(), b_indices.len());
        assert_eq!(b2.len(), b_indices.len());
        let (a_layout, b_layout) = rayon::join(
            || EqualLayout::for_wires(a_indices, groups, n_values),
            || EqualLayout::for_wires(b_indices, groups, n_values),
        );
        let (a, b) = rayon::join(
            || {
                a_layout.map(|layout| {
                    let sums = layout.prepare(a);
                    (layout, sums)
                })
            },
            || {
                b_layout.map(|layout| {
                    let (b, b2) = rayon::join(|| layout.prepare(b), || layout.prepare(b2));
                    (layout, b, b2)
                })
            },
        );
        (a.is_some() || b.is_some()).then_some(Self {
            reservation: None,
            a,
            b,
            k: None,
        })
    }
    pub(super) fn prepare_k(
        &mut self,
        bases: &[zopro_bn254::G1Affine],
        indices: &[u32],
        groups: &[[usize; 4]],
        n_values: usize,
        folded: bool,
    ) {
        assert_eq!(bases.len(), indices.len());
        self.k = EqualLayout::for_wires(indices, groups, n_values).map(|layout| {
            let sums = layout.prepare(bases);
            (folded, (layout, sums))
        });
    }
    pub(super) fn msm_k(
        &self,
        bases: &[zopro_bn254::G1Affine],
        scalars: &mut [Fr],
        folded: bool,
    ) -> zopro_bn254::G1Projective {
        if let Some((prepared_folded, (layout, sums))) = &self.k {
            if *prepared_folded == folded {
                if let Some(groups) = layout.source_groups(scalars) {
                    return msm_grouped_prepared(bases, sums, &groups);
                }
            }
        }
        msm_in_place(bases, scalars)
    }
    pub(super) fn msm_a(
        &self,
        bases: &[zopro_bn254::G1Affine],
        scalars: &mut [Fr],
    ) -> zopro_bn254::G1Projective {
        if let Some((layout, sums)) = &self.a {
            if let Some(groups) = layout.source_groups(scalars) {
                return msm_grouped_prepared(bases, sums, &groups);
            }
        }
        msm_in_place(bases, scalars)
    }
    pub(super) fn msms_b(
        &self,
        bases: &[zopro_bn254::G1Affine],
        bases2: &[zopro_bn254::G2Affine],
        scalars: &mut [Fr],
    ) -> (zopro_bn254::G1Projective, zopro_bn254::G2Projective) {
        if let Some((layout, sums, sums2)) = &self.b {
            if let Some(groups) = layout.source_groups(scalars) {
                return rayon::join(
                    || msm_grouped_prepared(bases, sums, &groups),
                    || msm_grouped_prepared(bases2, sums2, &groups),
                );
            }
        }
        if let Some(groups) = group_witness(scalars) {
            rayon::join(
                || msm_grouped(bases, &groups),
                || msm_grouped(bases2, &groups),
            )
        } else {
            let scalars = canonical(scalars);
            rayon::join(|| msm_raw(bases, scalars), || msm_raw(bases2, scalars))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::Seeded;
    use zopro_bn254::{G1Config, G2Config};

    #[test]
    fn prepared_k_tracks_the_selected_query_basis() {
        use zopro_bn254::{G1Affine, G1Projective};
        let n = 1027usize;
        let generator = G1Projective::generator();
        let mut p = generator;
        let projective: Vec<_> = (0..n)
            .map(|_| {
                let v = p;
                p += generator;
                v
            })
            .collect();
        let raw = G1Projective::batch_to_affine(&projective);
        let folded = G1Projective::batch_to_affine(
            &projective
                .iter()
                .map(|p| *p * Fr::from(7u64))
                .collect::<Vec<_>>(),
        );
        let layout = EqualLayout::new(
            n,
            (0..256)
                .map(|i| core::array::from_fn(|j| (4 * i + j) as u32))
                .collect(),
        );
        let sums = layout.prepare(&folded);
        let prepared = PreparedEqualWitness {
            reservation: None,
            a: None,
            b: None,
            k: Some((true, (layout, sums))),
        };
        let mut rng = Seeded::new(0x654321);
        for pattern in 0..3 {
            let mut scalars: Vec<_> = (0..n).map(|_| rng.fr()).collect();
            if pattern < 2 {
                for group in scalars[..1024].chunks_exact_mut(4) {
                    let value = rng.fr();
                    for x in &mut group[..if pattern == 0 { 4 } else { 3 }] {
                        *x = value;
                    }
                }
            }
            let weight = scalars
                .iter()
                .enumerate()
                .map(|(i, s)| *s * Fr::from((i + 1) as u64))
                .fold(Fr::ZERO, |a, b| a + b);
            let expected = (generator * weight).to_affine();
            let expected_folded = (generator * (weight * Fr::from(7u64))).to_affine();
            assert_ne!(expected, G1Affine::IDENTITY);
            // The two query vectors deliberately have the same length.
            assert_eq!(
                prepared
                    .msm_k(&raw, &mut scalars.clone(), false)
                    .to_affine(),
                expected
            );
            assert_eq!(
                prepared
                    .msm_k(&folded, &mut scalars.clone(), true)
                    .to_affine(),
                expected_folded
            );
        }
    }

    #[test]
    fn prepared_group_sources_match_independent_scalar_sum() {
        fn check<C: CurveConfig>()
        where
            C::Base: BatchInverse,
        {
            let n = 1027usize;
            let generator = Projective::<C>::generator();
            let mut point = generator;
            let projective: Vec<_> = (0..n)
                .map(|_| {
                    let p = point;
                    point += generator;
                    p
                })
                .collect();
            let bases = Projective::<C>::batch_to_affine(&projective);
            let groups = (0..256)
                .map(|i| core::array::from_fn(|j| ((4 * i + j) * 17 % n) as u32))
                .collect();
            let layout = EqualLayout::new(n, groups);
            let sums = layout.prepare(&bases);
            let mut rng = Seeded::new(0x1234567890);
            for pattern in 0..7 {
                let mut values: Vec<_> = (0..n).map(|_| rng.fr()).collect();
                for (i, group) in layout.groups.iter().enumerate() {
                    let value = match pattern {
                        0 => Fr::ZERO,
                        1 => Fr::ONE,
                        2 => -Fr::ONE,
                        _ => rng.fr(),
                    };
                    for (j, &wire) in group.iter().enumerate() {
                        if pattern < 4 || (pattern == 4 && i % 5 != 0) || (pattern == 5 && j < 3) {
                            values[wire as usize] = value;
                        }
                    }
                }
                if pattern == 0 {
                    values.fill(Fr::ZERO);
                }
                let expected = generator
                    * values
                        .iter()
                        .enumerate()
                        .map(|(i, s)| *s * Fr::from((i + 1) as u64))
                        .fold(Fr::ZERO, |a, b| a + b);
                let grouped = layout.source_groups(&values);
                assert_eq!(grouped.is_some(), pattern <= 4);
                let actual = if let Some(groups) = grouped {
                    assert_eq!(groups.len, n);
                    assert!(groups.order.iter().all(|i| (*i as usize) < n + sums.len()));
                    msm_grouped_prepared(&bases, &sums, &groups)
                } else {
                    msm_in_place(&bases, &mut values.clone())
                };
                assert_eq!(actual.to_affine(), expected.to_affine());
            }
        }
        check::<G1Config>();
        check::<G2Config>();
    }

    #[test]
    fn prepared_equal_groups_match_unmodified_msm() {
        fn check<C: CurveConfig>()
        where
            C::Base: BatchInverse,
        {
            let mut rng = Seeded::new(0xf654321);
            let n = 41;
            let mut bases: Vec<_> = (0..n)
                .map(|_| (Projective::<C>::generator() * rng.fr()).to_affine())
                .collect();
            bases[1] = Affine::IDENTITY;
            bases[9] = bases[3];
            bases[15] = -bases[12];
            let layout = EqualLayout::new(n, vec![[3, 1, 9, 7], [2, 20, 4, 13], [12, 15, 40, 39]]);
            let sums = layout.prepare(&bases);
            for pattern in 0..8 {
                let mut values: Vec<_> = (0..n).map(|_| rng.fr()).collect();
                for (k, g) in layout.groups.iter().enumerate() {
                    let same = match pattern {
                        0 => Fr::ZERO,
                        1 => Fr::ONE,
                        2 => -Fr::ONE,
                        _ => rng.fr(),
                    };
                    for (j, &i) in g.iter().enumerate() {
                        if pattern < 4 || j < 3 {
                            values[i as usize] = same;
                        }
                    }
                    if pattern >= 5 {
                        values[g[(k + pattern) % 4] as usize] += Fr::ONE;
                    }
                }
                let (scalars, equal) = layout.scalars(&values);
                let compact = layout.bases(&bases, &sums, &equal, scalars.len());
                let expected: Projective<C> = bases.iter().zip(&values).map(|(p, s)| *p * *s).sum();
                let actual: Projective<C> =
                    compact.iter().zip(&scalars).map(|(p, s)| *p * *s).sum();
                assert_eq!(actual.to_affine(), expected.to_affine());
                assert_eq!(msm(&compact, &scalars).to_affine(), expected.to_affine());
                assert_eq!(scalars.len(), n - 3 * equal.iter().filter(|v| **v).count());
            }
            let empty = EqualLayout::new(n, Vec::new());
            let (scalars, equal) = empty.scalars(&vec![Fr::ONE; n]);
            assert_eq!(empty.bases(&bases, &[], &equal, n), bases);
            assert_eq!(scalars, vec![Fr::ONE; n]);
        }
        check::<G1Config>();
        check::<G2Config>();
    }

    #[test]
    fn unsigned_digits_match_bit_reference() {
        let mut rng = Seeded::new(0x7298);
        let mut values: Vec<_> = (0..128).map(|_| rng.fr().to_canonical()).collect();
        values.extend([
            Fr::ZERO.to_canonical(),
            Fr::ONE.to_canonical(),
            (-Fr::ONE).to_canonical(),
        ]);
        for bit in 0..254 {
            let mut words = [0u64; 4];
            words[bit / 64] = 1 << (bit % 64);
            values.push(words);
        }
        for bits in 3..=15 {
            for words in &values {
                for w in 0..254usize.div_ceil(bits) {
                    let mut expected = 0u16;
                    for bit in 0..bits {
                        let at = w * bits + bit;
                        if at < 256 {
                            expected |= (((words[at / 64] >> (at % 64)) & 1) as u16) << bit;
                        }
                    }
                    assert_eq!(unsigned_digit(words, w, bits), expected as i16);
                }
            }
        }
    }
    #[test]
    fn unsigned_msm_matches_reference() {
        fn check<C: CurveConfig>()
        where
            C::Base: BatchInverse,
        {
            let mut rng = Seeded::new(0x83721);
            let g = Projective::<C>::generator();
            let mut p = g;
            for n in [0, 1, 17, 257, 1025] {
                let bases: Vec<_> = (0..n)
                    .map(|i| {
                        p += g;
                        match i % 19 {
                            0 => Affine::IDENTITY,
                            1 => g.to_affine(),
                            2 => (-g).to_affine(),
                            _ => p.to_affine(),
                        }
                    })
                    .collect();
                let scalars: Vec<_> = (0..n)
                    .map(|i| match i % 17 {
                        0 => Fr::ZERO,
                        1 => Fr::ONE,
                        2 => -Fr::ONE,
                        _ => rng.fr(),
                    })
                    .collect();
                let expected = zopro_bn254::curve::msm(&bases, &scalars).to_affine();
                let words: Vec<_> = scalars.iter().map(Fr::to_canonical).collect();
                for bits in [4, 10, 13] {
                    for parts in [1, 3] {
                        assert_eq!(
                            msm_unsigned_raw(&bases, &words, bits, parts).to_affine(),
                            expected,
                            "n={n} bits={bits} parts={parts}"
                        );
                    }
                }
            }
        }
        check::<G1Config>();
        check::<G2Config>();
    }
    #[test]
    fn equal_scalars_group_correctly() {
        fn check<C: CurveConfig>()
        where
            C::Base: BatchInverse,
        {
            let mut rng = Seeded::new(91);
            let choices = [
                Fr::ZERO,
                Fr::ONE,
                -Fr::ONE,
                rng.fr(),
                rng.fr(),
                rng.fr(),
                rng.fr(),
            ];
            let g = Projective::<C>::generator();
            let mut point = g;
            let bases: Vec<_> = (0..512)
                .map(|i| {
                    point += g;
                    match i % 19 {
                        0 => Affine::IDENTITY,
                        1 => g.to_affine(),
                        2 => (-g).to_affine(),
                        _ => point.to_affine(),
                    }
                })
                .collect();
            let scalars: Vec<_> = (0..512)
                .map(|i| choices[i % choices.len()].to_canonical())
                .collect();
            let (grouped, values) = group_scalars(&bases, &scalars).expect("repeated scalars");
            assert_eq!(
                msm_raw(&grouped, &values).to_affine(),
                msm_raw(&bases, &scalars).to_affine()
            );
            let witness: Vec<_> = (0..512).map(|i| choices[i % choices.len()]).collect();
            let expected = zopro_bn254::curve::msm(&bases, &witness).to_affine();
            assert_eq!(msm(&bases, &witness).to_affine(), expected);
            let mut in_place = witness.clone();
            assert_eq!(msm_in_place(&bases, &mut in_place).to_affine(), expected);
            let zero = vec![[0; 4]; 512];
            let (grouped, values) = group_scalars(&bases, &zero).unwrap();
            assert!(grouped.is_empty() && values.is_empty());
            assert_eq!(msm_raw(&bases, &zero).to_affine(), Affine::<C>::IDENTITY);
        }
        check::<G1Config>();
        check::<G2Config>();
    }

    /// Each window's digit against recoding the scalar from its lowest
    /// window up, the carry passed along: for scalars whose windows are
    /// just below half the radix in runs, where a digit's carry comes from
    /// windows further down, and for random ones; and the digits rebuild
    /// the scalar.
    #[test]
    fn digits_match_a_sequential_recoding() {
        let mut seeded = Seeded::new(7);
        for width in 4..=15usize {
            let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
            let half = 1u64 << (width - 1);
            let mut scalars: Vec<[u64; 4]> = (0..64).map(|_| seeded.fr().to_canonical()).collect();
            for run in [1, 2, 3, 7, windows] {
                for start in [0, 1, windows / 2] {
                    // Below 2^252, so still below the modulus.
                    let mut words = seeded.fr().to_canonical();
                    words[3] &= (1 << 60) - 1;
                    for w in start..(start + run).min(windows - 1) {
                        for bit in 0..width {
                            let at = w * width + bit;
                            if at < 252 {
                                let one = (half - 1) >> bit & 1 == 1;
                                let (word, shift) = (at / 64, at % 64);
                                words[word] = words[word] & !(1 << shift) | (one as u64) << shift;
                            }
                        }
                    }
                    scalars.push(words);
                }
            }
            for words in &scalars {
                let mut carry = 0u64;
                let mut rebuilt = Fr::ZERO;
                let radix = Fr::from(1u64 << width);
                for w in (0..windows).rev() {
                    rebuilt = rebuilt * radix + {
                        let d = digit(words, w, width) as i64;
                        if d < 0 {
                            -Fr::from(d.unsigned_abs())
                        } else {
                            Fr::from(d as u64)
                        }
                    };
                }
                for w in 0..windows {
                    let offset = w * width;
                    let (word, shift) = (offset / 64, offset % 64);
                    let mut value = if word < 4 { words[word] >> shift } else { 0 };
                    if shift + width > 64 && word < 3 {
                        value |= words[word + 1] << (64 - shift);
                    }
                    let raw = value & ((1 << width) - 1);
                    let value = raw + carry;
                    #[cfg(not(zopro_booth_digits))]
                    { carry = (value + half) >> width; }
                    #[cfg(zopro_booth_digits)]
                    { carry = raw >> (width-1); }
                    let expected = (value as i64 - (carry << width) as i64) as i16;
                    assert_eq!(
                        digit(words, w, width),
                        expected,
                        "width {width}, window {w}"
                    );
                }
                assert_eq!(rebuilt.to_canonical(), *words, "width {width}");
            }
        }
    }

    /// Against the sum of the products, with the special cases the batch
    /// must step around: zero and unit scalars, repeated bases (doublings in
    /// a bucket), a base and its negation (zero sums), the point at
    /// infinity; past `naive` points against the lane-form MSM.
    fn check<C: CurveConfig>(sizes: &[usize], naive: usize)
    where
        C::Base: crate::msm::BatchInverse,
    {
        let mut rng = Seeded::new(5);
        let max = *sizes.iter().max().unwrap();
        let g = Projective::<C>::generator();
        let step = g.mul(&rng.fr());
        let mut p = g.mul(&rng.fr());
        let mut all = Vec::with_capacity(max);
        for i in 0..max {
            all.push(match i % 20 {
                0 => Projective::<C>::IDENTITY,
                1..=3 => g,
                4 => -g,
                _ => {
                    p += step;
                    p
                }
            });
        }
        let bases = Projective::<C>::batch_to_affine(&all);
        let scalars: Vec<Fr> = (0..max)
            .map(|i| match i % 17 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                3 => Fr::from(7u64),
                _ => rng.fr(),
            })
            .collect();
        let reference = |bases: &[Affine<C>], scalars: &[Fr]| -> Projective<C> {
            if bases.len() <= naive {
                bases.iter().zip(scalars).map(|(b, s)| b.mul(s)).sum()
            } else {
                crate::msm::msm(bases, scalars)
            }
        };
        for &n in sizes {
            let expected = reference(&bases[..n], &scalars[..n]);
            assert_eq!(msm::<C>(&bases[..n], &scalars[..n]), expected, "{n} points");
            if n <= 1024 || n == max {
                assert_eq!(
                    msm_montgomery::<C>(&bases[..n], &scalars[..n]),
                    expected,
                    "{n} points with Montgomery words"
                );
            }
        }
        // One scalar repeated: every point of a window in one bucket.
        let same = vec![Fr::from(12345u64); 5000];
        assert_eq!(
            msm::<C>(&bases[..5000], &same),
            reference(&bases[..5000], &same),
            "one scalar"
        );
    }

    #[test]
    fn g1_matches_reference() {
        check::<G1Config>(&[0, 1, 255, 256, 1023, 1024, 4096, 16384, 70000], 1024);
    }

    #[test]
    fn g2_matches_reference() {
        check::<G2Config>(&[1, 200, 256, 4096, 20000], 256);
    }
}

#[cfg(test)]
mod deferred_window_reference_tests {
    use super::*;
    use zopro_bn254::{G1Affine, G1Config};

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn borrowed_bucket_shortcut_preserves_full_comparisons() {
        let generator = G1Affine::GENERATOR;
        let another = (generator * Fr::from(17u64)).to_affine();
        let x_word_collision = |high| {
            G1Affine::new(Fq::from_montgomery([7, high, 0, 0]), Fq::ONE)
        };
        for (current, incoming) in [
            (G1Affine::IDENTITY, generator),
            (generator, another),
            (generator, generator),
            (generator, -generator),
            (x_word_collision(1), x_word_collision(2)),
            (G1Affine::new(Fq::ZERO, Fq::ONE), generator),
        ] {
            for encoded in [0, 1 << 31] {
                let mut ordinary = Window::<G1Config>::new(1, 0);
                let mut borrowed = Window::<G1Config>::new(1, 0);
                ordinary.affine[0] = current;
                borrowed.affine[0] = current;
                assert_eq!(
                    borrowed.stage_pending::<true>(encoded, &incoming),
                    ordinary.stage_pending::<false>(encoded, &incoming),
                );
                assert_eq!(borrowed.affine, ordinary.affine);
                assert_eq!(borrowed.busy, ordinary.busy);
                assert_eq!(borrowed.projective_buckets, ordinary.projective_buckets);
                assert_eq!(borrowed.projective, ordinary.projective);
            }
        }
    }

    #[test]
    fn encoded_sign_windows_match_weighted_generator_and_scalar_fallback() {
        for n in [31usize, 1027] {
            let weights: Vec<_> = (0..n)
                .map(|i| {
                    if i % 17 == 0 {
                        Fr::ZERO
                    } else if i % 3 == 0 {
                        -Fr::from((i % 31 + 1) as u64)
                    } else {
                        Fr::from((i % 31 + 1) as u64)
                    }
                })
                .collect();
            let points: Vec<_> = weights
                .iter()
                .map(|&s| (G1Affine::GENERATOR * s).to_affine())
                .collect();
            for width in [10usize, 12, 14] {
                for kind in 0..4 {
                    let digits: Vec<i16> = (0..n)
                        .map(|i| match kind {
                            0 => 0,
                            1 => 1,
                            2 => {
                                if i % 2 == 0 {
                                    -1
                                } else {
                                    1
                                }
                            }
                            _ => {
                                let d = (i * 131 + 17) % ((1 << (width - 1)) - 1) + 1;
                                if i % 3 == 0 {
                                    -(d as i16)
                                } else {
                                    d as i16
                                }
                            }
                        })
                        .collect();
                    let scalar = weights.iter().zip(&digits).fold(Fr::ZERO, |sum, (&a, &d)| {
                        sum + a * if d < 0 {
                            -Fr::from(d.unsigned_abs() as u64)
                        } else {
                            Fr::from(d as u64)
                        }
                    });
                    let window = fill_window_deferred::<G1Config, true, false>(
                        &points,
                        digits.into_iter(),
                        width,
                        batch_size(width).max(32),
                        true,
                    );
                    let got = crate::msm::weighted_affine(&affine_buckets(window)).to_affine();
                    assert_eq!(
                        got,
                        (G1Affine::GENERATOR * scalar).to_affine(),
                        "n={n} width={width} kind={kind}"
                    );
                }
            }
        }
        let mut window = Window::<G1Config>::new(16, 32);
        window.defer_sign = true;
        window.use_packed = false;
        let mut expected = window.affine.clone();
        for i in 0..16 {
            let a = (G1Affine::GENERATOR * Fr::from((i + 7) as u64)).to_affine();
            let b = (G1Affine::GENERATOR * Fr::from((i + 103) as u64)).to_affine();
            window.affine[i] = a;
            expected[i] = (a + if i % 2 == 0 { -b } else { b }).to_affine();
            window.add_signed(i as u32 | if i % 2 == 0 { 1 << 31 } else { 0 }, b);
        }
        window.execute();
        assert_eq!(window.affine, expected);
    }
}

#[cfg(any(
    all(target_os = "macos", target_arch = "aarch64"),
    all(
        zopro_g2_horner_buckets,
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        feature = "ifma8",
        not(no_ifma)
    )
))]
mod g2_phi;
#[cfg(all(
    zopro_resource_schedule,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) mod workspace;

pub(super) fn canonical_exclusive(scalars: &mut [Fr]) -> &mut [[u64; 4]] {
    encode_exclusive(scalars, Fr::to_canonical)
}

// Reuse proof-local field storage as words. This input is no longer used
// as field values after encoding.
fn encode_exclusive(
    scalars: &mut [Fr],
    encode: impl Fn(&Fr) -> [u64; 4] + Sync,
) -> &mut [[u64; 4]] {
    scalars
        .par_iter_mut()
        .for_each(|s| *s = Fr::from_montgomery(encode(s)));
    // SAFETY: Fr is repr(transparent) over its four limbs.
    unsafe {
        std::slice::from_raw_parts_mut(scalars.as_mut_ptr().cast::<[u64; 4]>(), scalars.len())
    }
}

#[cfg(all(
    zopro_g2_horner_buckets,
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
#[inline(never)]
pub(super) fn owned_grouped_phi(
    bases: &[zopro_bn254::G2Affine],
    groups: ScalarGroups,
    frontend: Option<&rayon::ThreadPool>,
) -> zopro_bn254::G2Projective {
    use rayon::prelude::*;
    if !(1024..65536).contains(&groups.values.len()) {
        return crate::msm_grouped_b2(bases, &groups);
    }
    let ScalarGroups {
        len,
        mut values,
        starts,
        order,
        counts,
        _reservation,
    } = groups;
    assert_eq!(bases.len(), len);
    // B1 is omitted. The B2 query owns its canonical coefficients.
    let (points, ()) = rayon::join(
        move || crate::msm::group_affine(bases, &order, &starts, &counts),
        || {
            values
                .par_iter_mut()
                .with_min_len(128)
                .for_each(|s| *s = g2_phi::phi_split(*s))
        },
    );
    // Group index vectors have dropped. Keep the reservation through the MSM.
    #[cfg(all(zopro_resource_schedule, target_os = "linux"))]
    if let Some(pool) = frontend.filter(|p| p.current_num_threads() < rayon::current_num_threads())
    {
        let bytes =
            workspace::grouped_auxiliary_window_bytes(values.len(), pool.current_num_threads());
        if let Some(_scratch) = crate::preparation::Reservation::analysis(bytes) {
            return pool.install(|| g2_phi::from_split(&points, &values, 12, 2));
        }
    }
    #[cfg(not(all(zopro_resource_schedule, target_os = "linux")))]
    let _ = frontend;
    g2_phi::from_split(&points, &values, 12, 2)
}

// The four-query CPU prover gives up these proof-local scalar words.
// The ordinary five-part path continues to share immutable scalars with B1.
pub(super) fn g2_query_exclusive(
    bases: &[zopro_bn254::G2Affine],
    words: &mut [[u64; 4]],
    grouping: bool,
) -> zopro_bn254::G2Projective {
    assert_eq!(bases.len(), words.len());
    let groups = (grouping && crate::simd::cpu_option("ZOPRO_CPU_DEDUP"))
        .then(|| grouping::groups_with_diversity_check::<zopro_bn254::G2Config>(words))
        .flatten();
    g2_exclusive_with_groups(bases, groups, || words, false)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn apple_g2_three_parts_enabled() -> bool {
    std::env::var("ZOPRO_CPU_G2_BALANCED").as_deref() != Ok("0")
        && std::env::var("ZOPRO_CPU_G2_SCALAR_MERGE").as_deref() != Ok("0")
        && std::env::var("ZOPRO_CPU_G2_THREE_PARTS").as_deref() != Ok("0")
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn apple_b2_from_fr(
    bases: &[zopro_bn254::G2Affine],
    scalars: &mut [Fr],
    grouping: bool,
    three_parts: bool,
) -> zopro_bn254::G2Projective {
    assert_eq!(bases.len(), scalars.len());
    let groups = grouping
        .then(|| grouping::group_apple_cpu_witness::<zopro_bn254::G2Config, true>(scalars))
        .flatten();
    g2_exclusive_with_groups(
        bases,
        groups,
        || canonical_exclusive(scalars),
        three_parts,
    )
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn apple_b2_borrowed_groups(
    bases: &[zopro_bn254::G2Affine],
    mut groups: ScalarGroups,
) -> zopro_bn254::G2Projective {
    // Classify the singletons as references to the key. Only the repeated
    // classes need a point sum and storage owned by this proof.
    let duplicates = groups.order.len() - groups.values.len();
    let mut starts = Vec::with_capacity(duplicates + 1);
    let mut order = Vec::with_capacity(2 * duplicates);
    let mut counts = Vec::with_capacity(duplicates);
    starts.push(0);
    for (i, &count) in groups.counts.iter().enumerate() {
        if count > 1 {
            order.extend_from_slice(&groups.order[groups.starts[i]..groups.starts[i + 1]]);
            starts.push(order.len());
            counts.push(count);
        }
    }
    let sums = crate::msm::group_affine(bases, &order, &starts, &counts);
    let mut summed = sums.iter();
    let points: Vec<_> = groups.counts.iter().enumerate().map(|(i, &count)| {
        if count == 1 {
            &bases[groups.order[groups.starts[i]] as usize]
        } else {
            summed.next().expect("non-singleton sum")
        }
    }).collect();
    g2_phi::three_part_msm(&points, &mut groups.values, false)
}

fn g2_exclusive_with_groups<'a>(
    bases: &[zopro_bn254::G2Affine],
    groups: Option<ScalarGroups>,
    words: impl FnOnce() -> &'a mut [[u64; 4]],
    three_parts: bool,
) -> zopro_bn254::G2Projective {
    let _ = three_parts;
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let limit = if three_parts { 1 << 17 } else { 1 << 16 };
    #[cfg(all(zopro_g2_horner_buckets, target_arch = "x86_64"))]
    let limit = 1 << 16;
    #[cfg(any(
        all(target_os = "macos", target_arch = "aarch64"),
        all(
            zopro_g2_horner_buckets,
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            feature = "ifma8",
            not(no_ifma)
        )
    ))]
    if (1024..limit).contains(&bases.len()) {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let larger_batches = (61440..65536).contains(&bases.len());
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let compute = |bases: &[zopro_bn254::G2Affine], words: &mut [[u64; 4]]| {
            let workers = rayon::current_num_threads();
            let balanced_limit = if three_parts { limit } else { 1 << 15 };
            if ((1 << 14)..balanced_limit).contains(&bases.len())
                && (three_parts || workers == 6 || workers >= 16)
                && borrowed_scalar::enabled()
                && std::env::var("ZOPRO_CPU_G2_BALANCED").as_deref() != Ok("0")
            {
                #[cfg(feature = "trace")]
                log::info!("ZOPRO_TRACE cpu-g2-balanced-windows");
                if three_parts {
                    return g2_phi::three_part_msm(bases, words, larger_batches && workers == 5);
                }
                return g2_phi::balanced_msm(bases, words);
            }
            g2_phi::hb_raw(bases, words, 12, 2)
        };
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        let compute = g2_phi::raw_exclusive;
        if let Some(mut g) = groups {
            if !(1024..limit).contains(&g.values.len()) {
                return crate::msm_grouped_b2(bases, &g);
            }
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            if three_parts && rayon::current_num_threads() == 7
                && ((1 << 14)..limit).contains(&g.values.len())
                // Few repeated classes keep the references and their
                // temporary index arrays within the grouping reservation.
                && g.order.len() - g.values.len() <= g.values.len() / 8
                && borrowed_scalar::enabled()
                && std::env::var("ZOPRO_CPU_G2_BALANCED").as_deref() != Ok("0")
            {
                return apple_b2_borrowed_groups(bases, g);
            }
            let points = crate::msm::group_affine(bases, &g.order, &g.starts, &g.counts);
            return compute(&points, &mut g.values);
        }
        return compute(bases, words());
    }
    match groups {
        Some(g) => crate::msm_grouped_b2(bases, &g),
        None => crate::msm_raw_b2(bases, words()),
    }
}
#[cfg(test)]
mod exclusive_g2_tests {
    use super::*;
    use zopro_bn254::{G2Affine, G2Projective};
    #[test]
    fn exclusive_queries_match_generator_oracle() {
        for n in [0, 7, 1023, 1024, 8192, 30089, 65535, 65536] {
            let mut point = G2Projective::IDENTITY;
            let mut points = Vec::with_capacity(n);
            let mut weights = Vec::with_capacity(n);
            for i in 0..n {
                point += G2Affine::GENERATOR;
                let (mut p, mut w) = (point, Fr::from((i + 1) as u64));
                if i % 17 == 0 {
                    p = G2Projective::IDENTITY;
                    w = Fr::ZERO;
                } else if i % 3 == 0 {
                    p = -p;
                    w = -w;
                } else if i % 11 == 0 {
                    p = G2Affine::GENERATOR.to_projective();
                    w = Fr::ONE;
                }
                points.push(p);
                weights.push(w);
            }
            let points = G2Projective::batch_to_affine(&points);
            for kind in 0..4 {
                let mut state = Fr::from(928371u64).pow(&[91]);
                let mut expected = Fr::ZERO;
                let words: Vec<_> = weights
                    .iter()
                    .enumerate()
                    .map(|(i, w)| {
                        state = state * Fr::from(6364136223846793005u64) + Fr::from((i + 7) as u64);
                        let s = match kind {
                            0 => state,
                            1 => {
                                if i % 16 == 0 {
                                    Fr::ZERO
                                } else {
                                    state
                                }
                            }
                            2 => Fr::from((i % 2048) as u64),
                            3 => {
                                let p = Fr::from(2u64)
                                    .pow(&[[11, 12, 63, 64, 125, 126, 127, 128, 253][i % 9]]);
                                if i % 2 == 0 {
                                    p - Fr::ONE
                                } else {
                                    p + Fr::ONE
                                }
                            }
                            _ => unreachable!(),
                        };
                        expected += s * *w;
                        s.to_canonical()
                    })
                    .collect();
                let want = (G2Affine::GENERATOR * expected).to_affine();
                for grouping in [false, true] {
                    let mut input = words.clone();
                    assert_eq!(
                        g2_query_exclusive(&points, &mut input, grouping).to_affine(),
                        want,
                        "n={n}kind={kind}"
                    );
                }
            }
        }
    }
    #[test]
    fn exclusive_conversion_matches_standard() {
        let mut values: Vec<_> = (0..1027)
            .map(|i| Fr::from((i + 1) as u64).pow(&[91]))
            .collect();
        let expected: Vec<_> = values.iter().map(Fr::to_canonical).collect();
        assert_eq!(canonical_exclusive(&mut values), expected);
    }
}

#[cfg(any(
    test,
    all(
        zopro_real_kh,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )
))]
pub(super) mod fused_kh;

#[cfg(all(
    zopro_g2_horner_buckets,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod g2_borrowed;

#[cfg(all(
    zopro_resource_schedule,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod dense_h;
