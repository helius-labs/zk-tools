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
    let sample = hash_table::<&[u64; 4]>(n.min(2048))?;
    let values = vector::<[u64; 4]>(n, true)?;
    let counts = vector::<usize>(n, true)?;
    let build = add(
        add(add(hash_table::<([u64; 4], u32)>(n)?, values)?, counts)?,
        vector::<u32>(n, false)?,
    )?;
    let lists = add(
        add(
            add(add(values, counts)?, vector::<u32>(n, false)?)?,
            mul(vector::<usize>(add(n, 1)?, false)?, 2)?,
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
    if n == 0 || percent > 100 { return None; }
    let u = mul(n, percent)?.checked_div(100)?;
    if u == 0 { return None; }
    add(limits::<zopro_bn254::G1Config>(n, u)?.0,
        limits::<zopro_bn254::G2Config>(n, u)?.0)
}

pub(super) fn reserve(n: usize, percent: usize) -> Option<crate::preparation::Reservation> {
    let peak = peak(n, percent);
    let reservation = crate::preparation::Reservation::analysis(peak);
    if std::env::var_os("ZOPRO_CPU_GROUP_TRACE").is_some() {
        eprintln!("GROUP_ADMISSION n={n} peak={peak:?} admitted={}", reservation.is_some());
    }
    reservation
}
