//! Group scalar classes and reduce their points into proof-local tiles.
use super::{grouping_cost, grouping_sample};
use crate::msm::BatchInverse;
use hashbrown::{hash_table::Entry, HashTable};
use rayon::prelude::*;
use std::hash::BuildHasher;
use zopro_bn254::{Affine, CurveConfig, Fr, Projective};

const MAX_SHARDS: usize = 32;

struct Members {
    words: Vec<[u64; 4]>,
    starts: Vec<usize>,
    counts: Vec<usize>,
    order: Vec<u32>,
}

fn allocate<T>(n: usize) -> Option<Vec<T>> {
    let mut out = Vec::new();
    out.try_reserve_exact(n).ok()?;
    Some(out)
}

fn members(keys: &[[u64; 4]], indices: &[u32]) -> Option<Members> {
    let mut table = HashTable::<u32>::new();
    let hasher = ahash::RandomState::new();
    table.try_reserve(indices.len(), |_: &u32| 0).ok()?;
    let mut words: Vec<[u64; 4]> = allocate(indices.len())?;
    let mut counts = allocate(indices.len())?;
    let mut labels = allocate(indices.len())?;
    for &i in indices {
        let key = keys[i as usize];
        let class = match table.entry(
            hasher.hash_one(key),
            |&class| words[class as usize] == key,
            |&class| hasher.hash_one(words[class as usize]),
        ) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let class = words.len() as u32;
                words.push(key);
                counts.push(0usize);
                entry.insert(class);
                class
            }
        };
        labels.push(class);
        counts[class as usize] += 1;
    }
    drop(table);

    let mut starts = allocate(counts.len() + 1)?;
    starts.push(0);
    for &n in &counts {
        starts.push(starts.last().unwrap() + n);
    }
    let mut cursor = allocate(counts.len())?;
    cursor.extend_from_slice(&starts[..counts.len()]);
    let mut order = allocate(indices.len())?;
    order.resize(indices.len(), 0);
    for (&i, c) in indices.iter().zip(labels) {
        order[cursor[c as usize]] = i;
        cursor[c as usize] += 1;
    }
    Some(Members {
        words,
        starts,
        counts,
        order,
    })
}

fn route(key: &[u64; 4], mask: usize) -> usize {
    // Equal scalar words select the same shard. The table separately hashes
    // and verifies the complete word array within each shard.
    let folded =
        key[0] ^ key[1].rotate_right(19) ^ key[2].rotate_right(37) ^ key[3].rotate_right(51);
    ((folded ^ (folded >> 32)) as usize) & mask
}

pub(super) fn partition(
    keys: &[[u64; 4]],
    shards: usize,
) -> Option<(Vec<u32>, [usize; MAX_SHARDS + 1])> {
    if !shards.is_power_of_two() || shards > MAX_SHARDS || keys.len() >= u32::MAX as usize {
        return None;
    }
    let mask = shards - 1;
    let mut offsets = [0usize; MAX_SHARDS + 1];
    for key in keys {
        if *key != [0; 4] {
            offsets[route(key, mask) + 1] += 1;
        }
    }
    for part in 0..shards {
        offsets[part + 1] += offsets[part];
    }
    let mut order = Vec::new();
    order.try_reserve_exact(offsets[shards]).ok()?;
    order.resize(offsets[shards], 0);
    let mut next = offsets;
    for (i, key) in keys.iter().enumerate() {
        if *key == [0; 4] {
            continue;
        }
        let shard = route(key, mask);
        order[next[shard]] = i as u32;
        next[shard] += 1;
    }

    Some((order, offsets))
}
fn scratch_bytes<C: CurveConfig>(n: usize, shards: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    use core::mem::size_of;
    let bytes_per_row = 2 * size_of::<[u64; 4]>()
        + size_of::<Affine<C>>()
        + 4 * size_of::<u32>()
        + 3 * size_of::<usize>();
    // Account for both live hash table allocations and their per-shard metadata.
    let buckets = n.checked_mul(3)?.checked_add(shards.checked_mul(8)?)?;
    let hash = buckets.checked_mul(2 * (size_of::<u32>() + 1))?;
    let partition = grouping_cost::vector::<u32>(n, false)?.checked_mul(2)?;
    let tiles = n.div_ceil(256).checked_add(shards)?;
    n.checked_mul(bytes_per_row)?
        .checked_add(hash)?
        .checked_add(partition)?
        .checked_add(crate::msm::workspace_grouping_linear::<C>(n, tiles)?)?
        .checked_add(shards.checked_mul(1280)?)?
        .checked_add(tiles.checked_mul(256)?)
}

/// Return None before entering the ordinary grouping path when tiling is inapplicable.
pub(crate) fn prove<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[Fr],
    shards: usize,
) -> Option<Projective<C>>
where
    C::Base: BatchInverse,
{
    let n = scalars.len();
    if bases.len() != n
        || !(65_536..262_144).contains(&n)
        || !matches!(shards, 16 | 32)
        || rayon::current_num_threads() < 4
        || !crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
    {
        return None;
    }
    // SAFETY: the initialized Montgomery field representation consists of four u64 limbs.
    let keys = unsafe { std::slice::from_raw_parts(scalars.as_ptr().cast::<[u64; 4]>(), n) };
    let zero_aware = std::env::var("ZOPRO_CPU_ZERO_AWARE_GROUPS").as_deref() == Ok("1");
    let (sample, percent) = grouping_sample(keys, zero_aware)?;
    drop(sample);
    let ordinary = grouping_cost::peak(n, percent)?;
    let bytes = ordinary.checked_add(scratch_bytes::<C>(n, shards)?)?;
    let charge = crate::preparation::Reservation::analysis(Some(bytes))?;
    let (indices, offsets) = partition(keys, shards)?;
    let groups = (0..shards)
        .into_par_iter()
        .map(|s| {
            let mut group = members(keys, &indices[offsets[s]..offsets[s + 1]])?;
            for word in &mut group.words {
                *word = Fr::from_montgomery(*word).to_canonical();
            }
            Some(group)
        })
        .collect::<Option<Vec<_>>>()?;
    drop(indices);
    let terms = groups.iter().map(|g| g.words.len()).sum::<usize>();
    if terms * 100 > n * percent {
        return None;
    }
    let mut views = allocate(groups.len())?;
    views.extend(groups.iter().map(|g| crate::msm::class_tiles::ClassShard {
        words: &g.words,
        starts: &g.starts,
        counts: &g.counts,
        order: &g.order,
    }));
    let (points, words) = crate::msm::class_tiles::compact(bases, &views)?;
    drop(views);
    drop(groups);
    #[cfg(zopro_live_preparation_charge)]
    let charge = charge.retain(Some(ordinary));
    let point = super::super::msm_raw(&points, &words);
    drop(charge);
    Some(point)
}

#[test]
fn tiled_groups_preserve_the_weighted_sum_on_both_curves() {
    fn check<C: CurveConfig>(base: Affine<C>, shards: usize)
    where
        C::Base: BatchInverse,
    {
        let n = 65_536;
        let scalars: Vec<_> = (0..n).map(|i| Fr::from((i % 137) as u64)).collect();
        let points: Vec<_> = (0..n)
            .map(|i| if i % 17 == 0 { Affine::IDENTITY } else { base })
            .collect();
        let sum: Fr = scalars
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| (i % 17 != 0).then_some(x))
            .fold(Fr::ZERO, |a, b| a + b);
        assert!(prove(&points, &scalars, shards).unwrap() == base * sum);
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap()
        .install(|| {
            check(zopro_bn254::G1Affine::GENERATOR, 32);
            check(zopro_bn254::G2Affine::GENERATOR, 16);
        });
}
