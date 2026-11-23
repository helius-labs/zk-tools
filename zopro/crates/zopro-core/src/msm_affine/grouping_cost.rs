//! Checked costs for optional scalar grouping, including its consumers.
use super::*;
use core::mem::size_of;
pub(super) fn add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}
pub(super) fn mul(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(b)
}
pub(crate) fn vector<T>(n: usize, growing: bool) -> Option<usize> {
    if n == 0 {
        return Some(0);
    }
    let capacity = if growing {
        n.max(4).checked_next_power_of_two()?
    } else {
        n
    };
    add(mul(capacity, size_of::<T>())?, 64)
}
pub(super) fn hash_table<T>(n: usize) -> Option<usize> {
    let buckets = add(mul(n, 8)?, 6)?
        .checked_div(7)?
        .checked_next_power_of_two()?
        .max(4);
    add(
        add(mul(buckets, add(size_of::<T>(), 1)?)?, 80)?,
        add(mul(buckets / 2, add(size_of::<T>(), 1)?)?, 80)?,
    )
}
pub(super) fn limits<C: CurveConfig>(n: usize, u: usize) -> Option<(usize, usize)>
where
    C::Base: BatchInverse,
{
    limits_with_class_cap::<C>(n, u, n)
}

#[cfg(all(target_os = "android", target_arch = "aarch64"))]
pub(super) fn bounded_classes<C: CurveConfig>(n: usize, cap: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    Some(limits_with_class_cap::<C>(n, cap, cap)?.0)
}

fn limits_with_class_cap<C: CurveConfig>(n: usize, u: usize, cap: usize) -> Option<(usize, usize)>
where
    C::Base: BatchInverse,
{
    let sample = hash_table::<&[u64; 4]>(n.min(2048))?;
    let values = vector::<[u64; 4]>(cap, true)?;
    let counts = vector::<usize>(cap, true)?;
    let build = add(
        add(add(hash_table::<([u64; 4], u32)>(cap)?, values)?, counts)?,
        vector::<u32>(n, false)?,
    )?;
    let lists = add(
        add(
            add(add(values, counts)?, vector::<u32>(n, false)?)?,
            mul(vector::<usize>(add(cap, 1)?, false)?, 2)?,
        )?,
        vector::<u32>(n, false)?,
    )?;
    let build = add(sample, build.max(lists))?;
    let workers = rayon::current_num_threads().max(1);
    let classes = mul(workers, 256)?.min(u);
    // A live map_init state retains its largest assigned chunk. Across all
    // states those representative chunks contain at most workers*256 distinct
    // classes, plus all n-u duplicate inputs. Optimize this linear upper bound
    // over u at its endpoints and its only kink, not over fixture statistics.
    let metadata = |v: usize| -> Option<usize> {
        let each = add(
            add(mul(2, size_of::<[u64; 4]>())?, mul(3, size_of::<usize>())?)?,
            size_of::<Affine<C>>(),
        )?;
        add(
            add(
                add(mul(v, each)?, mul(n, size_of::<u32>())?)?,
                size_of::<usize>(),
            )?,
            5 * 64,
        )
    };
    let mut construction = 0;
    for v in [1usize, u, classes.max(1)] {
        let active = add(n.checked_sub(v)?, v.min(mul(workers, 256)?))?;
        construction = construction.max(add(
            metadata(v)?,
            crate::msm::workspace_grouping_linear::<C>(active, workers)?,
        )?);
    }
    Some((build.max(construction), metadata(u)?))
}

/// A ScalarGroups value can feed G1 and G2 concurrently. Charge both worst
/// consumer peaks; this deliberately counts shared metadata conservatively.
pub(super) fn peak(n: usize, percent: usize) -> Option<usize> {
    if n == 0 || percent > 100 {
        return None;
    }
    let u = mul(n, percent)?.checked_div(100)?;
    if u == 0 {
        return None;
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    if n >= crate::msm::apple_class_tiles::MIN_BASES {
        let workers = rayon::current_num_threads();
        return add(
            large_peak::<zopro_bn254::G1Config>(n, u, workers)?,
            large_peak::<zopro_bn254::G2Config>(n, u, workers)?,
        );
    }
    add(
        limits::<zopro_bn254::G1Config>(n, u)?.0,
        limits::<zopro_bn254::G2Config>(n, u)?.0,
    )
}

pub(super) type Peak = fn(usize, usize) -> Option<usize>;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn large_peak<C: CurveConfig>(n: usize, u: usize, workers: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    let sample = hash_table::<&[u64; 4]>(n.min(2048))?;
    let vectors = |entries| {
        add(vector::<[u64; 4]>(entries, true)?, vector::<usize>(entries, true)?)
    };
    let indices = vector::<u32>(n, false)?;
    let starts = vector::<usize>(add(u, 1)?, false)?;
    // Classification stores only an index in the hash table. Its vectors
    // can grow before classification discovers that grouping is unhelpful.
    let classify = add(add(hash_table::<u32>(n)?, mul(vectors(n)?, 2)?)?, indices)?;
    let lists = add(add(vectors(u)?, mul(starts, 2)?)?, mul(indices, 2)?)?;
    let reduction = add(
        add(add(add(vectors(u)?, starts)?, indices)?, vector::<Affine<C>>(u, false)?)?,
        crate::msm::apple_class_tiles::workspace::<C>(n, workers)?,
    )?;
    add(sample, classify.max(lists).max(reduction))
}

/// Cost model for a group used by one curve. Other hosts retain their
/// existing admission limits.
pub(super) fn peak_for<C: CurveConfig>(n: usize, percent: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    #[cfg(any(all(gpu, target_os = "ios"), all(target_os = "macos", target_arch = "aarch64")))]
    {
        if percent > 100 {
            return None;
        }
        let u = n.checked_mul(percent)?.checked_div(100)?;
        if u == 0 {
            return None;
        }
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        if n >= crate::msm::apple_class_tiles::MIN_BASES {
            return large_peak::<C>(n, u, rayon::current_num_threads());
        }
        Some(limits::<C>(n, u)?.0)
    }
    #[cfg(not(any(all(gpu, target_os = "ios"), all(target_os = "macos", target_arch = "aarch64"))))]
    peak(n, percent)
}

pub(super) fn reserve(n: usize, peak: Option<usize>) -> Option<crate::preparation::Reservation> {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let reservation = crate::preparation::Reservation::scalar_grouping(peak);
    #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
    let reservation = crate::preparation::Reservation::analysis(peak);
    if std::env::var_os("ZOPRO_CPU_GROUP_TRACE").is_some() {
        eprintln!(
            "GROUP_ADMISSION n={n} peak={peak:?} admitted={}",
            reservation.is_some()
        );
    }
    reservation
}
