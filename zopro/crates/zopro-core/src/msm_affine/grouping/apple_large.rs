//! Admit classification and point reduction using their live allocations.
use super::{
    ScalarGroups, assemble_groups, canonicalize, grouping_cost, grouping_sample, scalar_words,
};
use crate::msm::{BatchInverse, apple_class_tiles};
use crate::preparation::allocation_bytes;
use hashbrown::HashTable;
use std::hash::BuildHasher;
use zopro_bn254::{Affine, CurveConfig, Fr};

pub(super) fn group<C: CurveConfig>(scalars: &[Fr]) -> Option<ScalarGroups>
where
    C::Base: BatchInverse,
{
    let n = scalars.len();
    if n >= u32::MAX as usize || !crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        return None;
    }
    let words = scalar_words(scalars);
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    let (sample, percent) = grouping_sample(words, zero_aware)?;
    let limit = n.checked_mul(percent)? / 100;
    let mut capacity = (sample.len() * 8).clamp(256, 16384).min(limit);
    drop(sample);

    let index_bytes = grouping_cost::vector::<u32>(n, false)?;
    // A hash resize holds the previous table while it rehashes. A vector
    // growth can likewise hold the previous array while it copies.
    let classify_bytes = |capacity| {
        index_bytes
            .checked_add(grouping_cost::hash_table::<u32>(capacity)?)?
            .checked_add(grouping_cost::vector::<[u64; 4]>(capacity, false)?.checked_mul(2)?)?
            .checked_add(grouping_cost::vector::<usize>(capacity, false)?.checked_mul(2)?)
    };
    let mut reservation = grouping_cost::reserve(n, classify_bytes(capacity))?;
    let mut classes = vec![u32::MAX; n];
    let (values, counts) = {
        let state = ahash::RandomState::new();
        let mut table = HashTable::<u32>::with_capacity(capacity);
        let mut values = Vec::with_capacity(capacity);
        let mut counts = Vec::with_capacity(capacity);
        for (scalar, class) in words.iter().zip(&mut classes) {
            if *scalar == [0; 4] {
                continue;
            }
            let hash = state.hash_one(scalar);
            let index = if let Some(&i) = table.find(hash, |&i| values[i as usize] == *scalar) {
                i
            } else {
                if values.len() == capacity {
                    if capacity == limit {
                        return None;
                    }
                    capacity = capacity.checked_mul(2)?.min(limit);
                    reservation.resize_scalar_grouping(classify_bytes(capacity))?;
                    let extra = capacity - values.len();
                    table.reserve(extra, |&i| state.hash_one(&values[i as usize]));
                    values.reserve_exact(extra);
                    counts.reserve_exact(extra);
                }
                let i = values.len() as u32;
                values.push(*scalar);
                counts.push(0usize);
                table.insert_unique(hash, i, |&i| state.hash_one(&values[i as usize]));
                i
            };
            *class = index;
            counts[index as usize] += 1;
        }
        (values, counts)
    };

    let unique = values.len();
    let nonzero = counts.iter().sum();
    let metadata = allocation_bytes(&values)?.checked_add(allocation_bytes(&counts)?)?;
    let starts = grouping_cost::vector::<usize>(unique + 1, false)?;
    let order = grouping_cost::vector::<u32>(nonzero, false)?;
    let lists = metadata.checked_add(starts)?.checked_add(order)?;
    let assemble = lists
        .checked_add(index_bytes)?
        .checked_add(grouping_cost::vector::<usize>(unique, false)?)?;
    let reduction = lists
        .checked_add(grouping_cost::vector::<Affine<C>>(unique, false)?)?
        .checked_add(apple_class_tiles::workspace::<C>(
            nonzero,
            rayon::current_num_threads(),
        )?)?;
    // Reserve through assembly and its consumer. The classifier's table
    // is already gone, so it does not count against this phase.
    let admitted = reservation.resize_scalar_grouping(Some(assemble.max(reduction)));
    if std::env::var_os("ZOPRO_CPU_GROUP_TRACE").is_some() {
        eprintln!(
            "GROUP_REDUCTION n={n} unique={unique} nonzero={nonzero} peak={} admitted={}",
            assemble.max(reduction),
            admitted.is_some()
        );
    }
    admitted?;
    Some(canonicalize::<false>(assemble_groups(
        classes,
        values,
        counts,
        reservation,
    )))
}
