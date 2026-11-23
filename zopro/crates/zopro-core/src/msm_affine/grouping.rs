//! Proof-local scalar grouping and its admission policy.
//!
//! Returned groups own scalar values and stable input-index lists. The
//! reservation outlives every owned buffer and all point-reduction consumers.
use super::grouping_cost;
#[cfg(zopro_resource_schedule)]
use crate::solver::{B2Origins, RuntimeAliases};
use rayon::prelude::*;
use zopro_bn254::{CurveConfig, Fr};

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod apple_large;

pub(crate) struct ScalarGroups {
    pub(super) len: usize,
    pub(super) values: Vec<[u64; 4]>,
    pub(super) order: Vec<u32>,
    pub(super) starts: Vec<usize>,
    pub(super) counts: Vec<usize>,
    // Drop the charge after all owned buffers.
    pub(super) _reservation: crate::preparation::Reservation,
}
/// Group before converting representations, so only distinct witness
/// scalars need Montgomery reduction. The grouping is fresh per proof.
pub(crate) fn group_witness(scalars: &[Fr]) -> Option<ScalarGroups> {
    group_witness_inner::<false, false>(scalars, None, grouping_cost::peak, false)
}

pub(crate) fn group_witness_for<C: CurveConfig>(scalars: &[Fr]) -> Option<ScalarGroups>
where
    C::Base: crate::msm::BatchInverse,
{
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    if scalars.len() >= crate::msm::apple_class_tiles::MIN_BASES {
        return apple_large::group::<C>(scalars);
    }
    #[cfg(all(target_os = "android", target_arch = "aarch64"))]
    if crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        if let Some(groups) = group_bounded_android::<C>(scalar_words(scalars)) {
            return Some(canonicalize::<false>(groups));
        }
    }
    group_witness_inner::<false, false>(scalars, None, grouping_cost::peak_for::<C>, false)
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn group_apple_cpu_witness<C: CurveConfig, const EXCLUSIVE: bool>(
    scalars: &[Fr],
) -> Option<ScalarGroups>
where
    C::Base: crate::msm::BatchInverse,
{
    let groups =
        group_witness_inner::<false, EXCLUSIVE>(scalars, None, grouping_cost::peak_for::<C>, true);
    #[cfg(feature = "trace")]
    if let Some(groups) = &groups {
        log::info!(
            "ZOPRO_TRACE cpu-witness-group-prep {} -> {}",
            scalars.len(),
            groups.values.len()
        );
    }
    groups
}

pub(crate) fn group_witness_serial_b2(scalars: &[Fr]) -> Option<ScalarGroups> {
    group_witness_inner::<true, false>(scalars, None, grouping_cost::peak, false)
}

#[cfg(zopro_resource_schedule)]
pub(crate) fn group_witness_b2_origins(
    scalars: &[Fr],
    origins: &B2Origins,
    serial: bool,
) -> Option<ScalarGroups> {
    if serial {
        group_witness_inner::<true, false>(scalars, Some(origins), grouping_cost::peak, false)
    } else {
        group_witness_inner::<false, false>(scalars, Some(origins), grouping_cost::peak, false)
    }
}

#[cfg(zopro_resource_schedule)]
type Origins<'a> = Option<&'a B2Origins>;
#[cfg(not(zopro_resource_schedule))]
type Origins<'a> = Option<&'a ()>;

fn group_witness_inner<const SERIAL_B2: bool, const EXCLUSIVE: bool>(
    scalars: &[Fr],
    origins: Origins<'_>,
    peak: grouping_cost::Peak,
    apple_cpu: bool,
) -> Option<ScalarGroups> {
    if !crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        return None;
    }
    let words = scalar_words(scalars);
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    let groups = group_with_gate::<EXCLUSIVE, SERIAL_B2>(words, zero_aware, origins, peak, apple_cpu)?;
    Some(canonicalize::<SERIAL_B2>(groups))
}

fn scalar_words(scalars: &[Fr]) -> &[[u64; 4]] {
    // SAFETY: Fr is repr(transparent) over four u64 limbs. This shared
    // view only compares/hashes the existing Montgomery representations.
    unsafe { std::slice::from_raw_parts(scalars.as_ptr().cast::<[u64; 4]>(), scalars.len()) }
}

fn canonicalize<const SERIAL_B2: bool>(mut groups: ScalarGroups) -> ScalarGroups {
    let convert = |value: &mut [u64; 4]| {
        *value = Fr::from_montgomery(*value).to_canonical();
    };
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    if SERIAL_B2 && use_vector_b2_decoding() {
        decode_b2_classes(&mut groups.values);
        return groups;
    }
    if SERIAL_B2 {
        groups.values.iter_mut().for_each(convert);
    } else {
        groups.values.par_iter_mut().for_each(convert);
    }
    groups
}

#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
fn use_vector_b2_decoding() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var("ZOPRO_CPU_GROUPED_B2_VECTOR").as_deref() == Ok("1"))
}

#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
fn decode_b2_classes(values: &mut [[u64; 4]]) {
    use zopro_bn254::{ifma8::Ifma8, FrConfig};

    type Batch = Ifma8<FrConfig, true>;
    // Integer one, deliberately in the raw limb representation. A
    // Montgomery multiply by it converts the input to canonical limbs.
    let one = Batch::new(&[Fr::from_montgomery([1, 0, 0, 0]); 8]);
    let (full, tail) = values.as_chunks_mut::<8>();
    for batch in full {
        let input = batch.map(Fr::from_montgomery);
        *batch = Batch::new(&input).mul(&one).get().map(|v| v.0);
    }
    for value in tail {
        *value = Fr::from_montgomery(*value).to_canonical();
    }
}

#[cfg(all(
    test,
    all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )
))]
#[test]
fn vector_b2_classes_match_canonical_conversion_and_preserve_all_tails() {
    use crate::testing::Seeded;

    let mut seed = Seeded::new(0x36417472);
    for len in (0..20).chain(108_893..108_900) {
        let input: Vec<_> = (0..len)
            .map(|i| match i % 7 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                _ => seed.fr(),
            })
            .collect();
        let mut words: Vec<_> = input.iter().map(|f| f.0).collect();
        decode_b2_classes(&mut words);
        assert!(words
            .iter()
            .zip(&input)
            .all(|(actual, value)| *actual == value.to_canonical()));
    }
}

pub(crate) fn scalar_groups(scalars: &[[u64; 4]]) -> Option<ScalarGroups> {
    scalar_groups_policy(
        scalars,
        std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1"),
    )
}

mod diversity;
#[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
pub(super) mod tiled;
pub(super) fn groups_with_diversity_check<C: CurveConfig>(
    scalars: &[[u64; 4]],
) -> Option<ScalarGroups>
where
    C::Base: crate::msm::BatchInverse,
{
    #[cfg(all(target_os = "android", target_arch = "aarch64"))]
    if let Some(groups) = group_bounded_android::<C>(scalars) {
        return Some(groups);
    }
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    group_with_gate::<true, false>(scalars, zero_aware, None, grouping_cost::peak_for::<C>, false)
}

#[cfg(all(target_os = "android", target_arch = "aarch64"))]
fn group_bounded_android<C: CurveConfig>(scalars: &[[u64; 4]]) -> Option<ScalarGroups>
where
    C::Base: crate::msm::BatchInverse,
{
    let n = scalars.len();
    if !((1 << 18)..u32::MAX as usize).contains(&n)
        || std::env::var("ZOPRO_ANDROID_BOUNDED_CLASSES").as_deref() != Ok("1")
    {
        return None;
    }
    let (sample, _) = sample_with_cutoff(scalars, true, 80)?;
    // The sample screens out obviously diverse inputs. A query with tens of
    // thousands of classes can still have a mostly unique sample. Scan all
    // values and stop before adding a class over the cap.
    let limit = n / 6;
    let capacity = (sample.len() * 128).min(limit);
    let reservation = grouping_cost::reserve(n, grouping_cost::bounded_classes::<C>(n, limit))?;
    let mut classes = vec![u32::MAX; n];
    let (values, counts) = classify_up_to(
        scalars, &mut classes, capacity, limit, ahash::RandomState::new(),
    )?;
    #[cfg(feature = "trace")]
    log::info!("ZOPRO_TRACE android-bounded-classes entries={n} classes={} limit={limit}", values.len());
    Some(assemble_groups(classes, values, counts, reservation))
}

type GroupSample<'a> = std::collections::HashSet<&'a [u64; 4], ahash::RandomState>;

fn grouping_sample(scalars: &[[u64; 4]], zero_aware: bool) -> Option<(GroupSample<'_>, usize)> {
    let percent = if (32768..65536).contains(&scalars.len())
        && crate::simd::cpu_option("ZOPRO_CPU_SMALL_GROUPS")
    {
        95
    } else {
        80
    };
    sample_with_cutoff(scalars, zero_aware, percent)
}

fn sample_with_cutoff(
    scalars: &[[u64; 4]],
    zero_aware: bool,
    percent: usize,
) -> Option<(GroupSample<'_>, usize)> {
    let sample = scalars.len().min(2048);
    let stride = scalars.len().div_ceil(sample);
    let unique: GroupSample<'_> = scalars.iter().step_by(stride).take(sample).collect();
    // A strided sample of a partial domain may contain fewer than 2048
    // observations. OFF retains the original denominator for comparison.
    let observed = if zero_aware {
        scalars.len().div_ceil(stride).min(sample)
    } else {
        sample
    };
    if unique.len() * 200 > observed * 199 {
        return None;
    }
    if zero_aware {
        let zeros = scalars
            .iter()
            .step_by(stride)
            .take(sample)
            .filter(|s| **s == [0; 4])
            .count();
        let nonzero = observed - zeros;
        let unique_nonzero = unique.len() - usize::from(zeros != 0);
        // Scattered zeros alone do not justify a full hash map when all
        // sampled nonzero values are distinct and the zero density cannot
        // meet the existing grouping cutoff. Repeated nonzero samples and
        // dense-zero samples retain the original grouping decision.
        let decline = unique_nonzero == nonzero && nonzero * 100 > observed * percent;
        if std::env::var_os("ZOPRO_CPU_GROUP_TRACE").is_some() {
            eprintln!(
                "ZERO_AWARE_GROUP n={} sampled={} zeros={} unique_nonzero={} percent={} decline={}",
                scalars.len(),
                observed,
                zeros,
                unique_nonzero,
                percent,
                decline
            );
        }
        if decline {
            return None;
        }
    }
    Some((unique, percent))
}

fn scalar_groups_policy(scalars: &[[u64; 4]], zero_aware: bool) -> Option<ScalarGroups> {
    group_with_gate::<false, false>(scalars, zero_aware, None, grouping_cost::peak, false)
}

fn group_with_gate<const EXCLUSIVE: bool, const SERIAL_B2: bool>(
    scalars: &[[u64; 4]],
    zero_aware: bool,
    origins: Origins<'_>,
    peak: grouping_cost::Peak,
    apple_cpu: bool,
) -> Option<ScalarGroups> {
    if scalars.len() < 256 || scalars.len() >= u32::MAX as usize {
        return None;
    }
    let apple_small = apple_cpu && (16384..49152).contains(&scalars.len());
    let (unique, percent) = if apple_small
        && std::env::var("ZOPRO_CPU_SMALL_GROUPS").as_deref() != Ok("0")
    {
        // Faster batched affine kernels can benefit from lighter compaction.
        sample_with_cutoff(scalars, zero_aware, 90)
    } else {
        grouping_sample(scalars, zero_aware)
    }?;
    if EXCLUSIVE
        && (1024..65536).contains(&scalars.len())
        && std::env::var("ZOPRO_CPU_GROUP_DIVERSITY").as_deref() == Ok("1")
        && diversity::declines(scalars, scalars.len() * percent / 100)
    {
        return None;
    }
    #[cfg(zopro_resource_schedule)]
    let reservation = match origins {
        Some(plan) => crate::preparation::Reservation::analysis(grouping_cost::add(
            peak(scalars.len(), percent)?,
            plan.scratch_bytes()?,
        ))?,
        None => grouping_cost::reserve(scalars.len(), peak(scalars.len(), percent))?,
    };
    #[cfg(not(zopro_resource_schedule))]
    let reservation = {
        let _ = origins;
        grouping_cost::reserve(scalars.len(), peak(scalars.len(), percent))?
    };
    // Bound the initial allocation by the sample and by the grouping
    // cutoff. Small high-diversity inputs otherwise grow and rehash the
    // entire table several times before either succeeding or declining.
    let reserve_groups = || {
        if apple_cpu && (16384..65536).contains(&scalars.len()) {
            std::env::var("ZOPRO_CPU_RESERVE_GROUPS").as_deref() != Ok("0")
        } else {
            crate::simd::cpu_option("ZOPRO_CPU_RESERVE_GROUPS")
        }
    };
    let capacity = if SERIAL_B2 {
        scalars.len().div_ceil(3).min(scalars.len() * percent / 100)
    } else if reserve_groups() {
        (unique.len() * 16)
            .min(scalars.len() * percent / 100)
            .min(32768)
    } else {
        0
    };
    let mut classes = vec![u32::MAX; scalars.len()];
    #[cfg(zopro_resource_schedule)]
    let (values, counts) = if let Some(plan) = origins {
        let room = capacity.min(plan.residual().div_ceil(3));
        let aliases = plan.aliases(scalars)?;
        classify_nonzero_origins(
            scalars,
            &mut classes,
            room,
            ahash::RandomState::new(),
            aliases,
        )
    } else {
        classify_nonzero(scalars, &mut classes, capacity, ahash::RandomState::new())
    };
    #[cfg(not(zopro_resource_schedule))]
    let (values, counts) =
        classify_nonzero(scalars, &mut classes, capacity, ahash::RandomState::new());
    if values.len() * 100 > scalars.len() * percent {
        return None;
    }
    Some(assemble_groups(classes, values, counts, reservation))
}

fn assemble_groups(
    classes: Vec<u32>,
    values: Vec<[u64; 4]>,
    counts: Vec<usize>,
    reservation: crate::preparation::Reservation,
) -> ScalarGroups {
    let len = classes.len();
    if values.is_empty() {
        return ScalarGroups {
            _reservation: reservation,
            len,
            values,
            order: Vec::new(),
            starts: vec![0],
            counts,
        };
    }
    let mut starts = Vec::with_capacity(counts.len() + 1);
    starts.push(0);
    for &count in &counts {
        starts.push(starts.last().unwrap() + count);
    }
    let mut positions = starts[..counts.len()].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, &class) in classes.iter().enumerate() {
        if class != u32::MAX {
            order[positions[class as usize]] = i as u32;
            positions[class as usize] += 1;
        }
    }
    drop(classes);
    drop(positions);
    ScalarGroups {
        _reservation: reservation,
        len,
        values,
        order,
        starts,
        counts,
    }
}

// Keep one copy of each full scalar in first-occurrence order. The hash table
// stores only indices and checks every limb against that copy on collision.
fn classify_nonzero(
    scalars: &[[u64; 4]],
    classes: &mut [u32],
    capacity: usize,
    state: impl std::hash::BuildHasher,
) -> (Vec<[u64; 4]>, Vec<usize>) {
    classify_up_to(scalars, classes, capacity, usize::MAX, state)
        .expect("the unlimited classifier accepts every class")
}

fn classify_up_to(
    scalars: &[[u64; 4]],
    classes: &mut [u32],
    capacity: usize,
    limit: usize,
    state: impl std::hash::BuildHasher,
) -> Option<(Vec<[u64; 4]>, Vec<usize>)> {
    use hashbrown::{hash_table::Entry, HashTable};

    let mut table = HashTable::<u32>::with_capacity(capacity);
    let mut values = Vec::new();
    let mut counts = Vec::new();
    for (scalar, class) in scalars.iter().zip(classes) {
        if *scalar == [0; 4] {
            continue;
        }
        let hash = state.hash_one(scalar);
        let equal = |&index: &u32| values[index as usize] == *scalar;
        let index = if values.len() == limit {
            *table.find(hash, equal)?
        } else {
            match table.entry(hash, equal, |&index| state.hash_one(&values[index as usize])) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    // The caller bounds the total input length below u32::MAX.
                    let index = values.len() as u32;
                    values.push(*scalar);
                    counts.push(0);
                    entry.insert(index);
                    index
                }
            }
        };
        *class = index;
        counts[index as usize] += 1;
    }
    Some((values, counts))
}

#[cfg(test)]
mod tests;

#[cfg(all(
    test,
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
#[test]
fn serial_b2_scalar_preparation_preserves_groups() {
    let scalars: Vec<_> = (0..113_537)
        .map(|i| match i {
            _ if i % 19 == 0 => Fr::ZERO,
            _ if i % 17 == 0 => Fr::ONE,
            _ => Fr::from((i % 27_013 + 2) as u64),
        })
        .collect();
    let groups = |g: ScalarGroups| (g.len, g.values, g.starts, g.counts, g.order);
    let control = groups(group_witness(&scalars).unwrap());
    let serial = groups(group_witness_serial_b2(&scalars).unwrap());
    assert_eq!(control, serial);
}

#[cfg(zopro_resource_schedule)]
fn classify_nonzero_origins(
    scalars: &[[u64; 4]],
    classes: &mut [u32],
    capacity: usize,
    state: impl std::hash::BuildHasher,
    mut aliases: RuntimeAliases<'_>,
) -> (Vec<[u64; 4]>, Vec<usize>) {
    use hashbrown::{hash_table::Entry, HashTable};
    let mut table = HashTable::<u32>::with_capacity(capacity);
    let mut values = Vec::new();
    let mut counts = Vec::new();
    for (i, (scalar, class)) in scalars.iter().zip(classes).enumerate() {
        if *scalar == [0; 4] {
            continue;
        }
        if let Some(slot) = aliases.slot(i) {
            if *slot == u32::MAX {
                *slot = values.len() as u32;
                *class = *slot;
                values.push(*scalar);
                counts.push(1);
                continue;
            }
            if values[*slot as usize] == *scalar {
                *class = *slot;
                counts[*slot as usize] += 1;
                continue;
            }
        }
        let index = match table.entry(
            state.hash_one(scalar),
            |&j| values[j as usize] == *scalar,
            |&j| state.hash_one(&values[j as usize]),
        ) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let j = values.len() as u32;
                values.push(*scalar);
                counts.push(0);
                e.insert(j);
                j
            }
        };
        *class = index;
        counts[index as usize] += 1;
    }
    (values, counts)
}
