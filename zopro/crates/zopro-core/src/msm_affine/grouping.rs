//! Proof-local scalar grouping and its admission policy.
//!
//! Returned groups own scalar values and stable input-index lists. The
//! reservation outlives every owned buffer and all point-reduction consumers.
use super::grouping_cost;
use rayon::prelude::*;
use zopro_bn254::Fr;

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
    group_witness_inner::<false>(scalars)
}

pub(crate) fn group_witness_serial_b2(scalars: &[Fr]) -> Option<ScalarGroups> {
    group_witness_inner::<true>(scalars)
}

fn group_witness_inner<const SERIAL_B2: bool>(scalars: &[Fr]) -> Option<ScalarGroups> {
    if !crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        return None;
    }
    // SAFETY: Fr is repr(transparent) over four u64 limbs. This shared
    // view only compares/hashes the existing Montgomery representations.
    let words =
        unsafe { std::slice::from_raw_parts(scalars.as_ptr().cast::<[u64; 4]>(), scalars.len()) };
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    let mut groups = group_with_gate::<false, SERIAL_B2>(words, zero_aware)?;
    let convert = |value: &mut [u64; 4]| {
        *value = Fr::from_montgomery(*value).to_canonical();
    };
    if SERIAL_B2 {
        groups.values.iter_mut().for_each(convert);
    } else {
        groups.values.par_iter_mut().for_each(convert);
    }
    Some(groups)
}

pub(crate) fn scalar_groups(scalars: &[[u64; 4]]) -> Option<ScalarGroups> {
    scalar_groups_policy(
        scalars,
        std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1"),
    )
}

mod diversity;
pub(super) fn groups_with_diversity_check(scalars: &[[u64; 4]]) -> Option<ScalarGroups> {
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    group_with_gate::<true, false>(scalars, zero_aware)
}

type GroupSample<'a> = std::collections::HashSet<&'a [u64; 4], ahash::RandomState>;

fn grouping_sample(scalars: &[[u64; 4]], zero_aware: bool) -> Option<(GroupSample<'_>, usize)> {
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
    let percent = if (32768..65536).contains(&scalars.len())
        && crate::simd::cpu_option("ZOPRO_CPU_SMALL_GROUPS")
    {
        95
    } else {
        80
    };
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
    group_with_gate::<false, false>(scalars, zero_aware)
}

fn group_with_gate<const EXCLUSIVE: bool, const SERIAL_B2: bool>(scalars: &[[u64; 4]], zero_aware: bool) -> Option<ScalarGroups> {
    if scalars.len() < 256 || scalars.len() >= u32::MAX as usize {
        return None;
    }
    let (unique, percent) = grouping_sample(scalars, zero_aware)?;
    if EXCLUSIVE && (1024..65536).contains(&scalars.len())
        && std::env::var("ZOPRO_CPU_GROUP_DIVERSITY").as_deref() == Ok("1")
        && diversity::declines(scalars, scalars.len() * percent / 100)
    {
        return None;
    }
    let reservation = grouping_cost::reserve(scalars.len(), percent)?;
    // Bound the initial allocation by the sample and by the grouping
    // cutoff. Small high-diversity inputs otherwise grow and rehash the
    // entire table several times before either succeeding or declining.
    let capacity = if SERIAL_B2 {
        scalars.len().div_ceil(3).min(scalars.len() * percent / 100)
    } else if crate::simd::cpu_option("ZOPRO_CPU_RESERVE_GROUPS") {
        (unique.len() * 16)
            .min(scalars.len() * percent / 100)
            .min(32768)
    } else {
        0
    };
    let mut classes = vec![u32::MAX; scalars.len()];
    let (values, counts) = classify_nonzero(scalars, &mut classes, capacity, ahash::RandomState::new());
    if values.len() * 100 > scalars.len() * percent {
        return None;
    }
    if values.is_empty() {
        return Some(ScalarGroups {
            _reservation: reservation,
            len: scalars.len(),
            values,
            order: Vec::new(),
            starts: vec![0],
            counts,
        });
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
    Some(ScalarGroups {
        _reservation: reservation,
        len: scalars.len(),
        values,
        order,
        starts,
        counts,
    })
}

// Keep one copy of each full scalar in first-occurrence order. The hash table
// stores only indices and checks every limb against that copy on collision.
fn classify_nonzero(
    scalars: &[[u64; 4]],
    classes: &mut [u32],
    capacity: usize,
    state: impl std::hash::BuildHasher,
) -> (Vec<[u64; 4]>, Vec<usize>) {
    use hashbrown::{hash_table::Entry, HashTable};

    let mut table = HashTable::<u32>::with_capacity(capacity);
    let mut values = Vec::new();
    let mut counts = Vec::new();
    for (scalar, class) in scalars.iter().zip(classes) {
        if *scalar == [0; 4] {
            continue;
        }
        let index = match table.entry(
            state.hash_one(scalar),
            |&index| values[index as usize] == *scalar,
            |&index| state.hash_one(&values[index as usize]),
        ) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                // The caller bounds the total input length below u32::MAX.
                let index = values.len() as u32;
                values.push(*scalar);
                counts.push(0);
                entry.insert(index);
                index
            }
        };
        *class = index;
        counts[index as usize] += 1;
    }
    (values, counts)
}

#[cfg(test)]
mod tests;


#[cfg(all(test, target_arch = "x86_64", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", not(no_ifma)))]
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
