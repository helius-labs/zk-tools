//! Disjoint exact scalar classes, with the original stable output layout.
use super::{grouping_cost, ScalarGroups};
use crate::preparation::Reservation;
use rayon::prelude::*;
use std::collections::HashMap;

const SHARDS: usize = 8;
const MIN_INPUT: usize = 8192;

fn workspace(n: usize, populations: &[usize; SHARDS]) -> Option<usize> {
    use grouping_cost::{add, hash_table, vector};
    let nonzero = populations.iter().try_fold(0usize, |a, &b| add(a, b))?;
    if nonzero > n {
        return None;
    }
    let pairs = vector::<(u32, u32)>(nonzero, false)?;
    let maps = populations
        .iter()
        .try_fold(0usize, |a, &b| add(a, hash_table::<([u64; 4], u32)>(b)?))?;
    let build = add(pairs, maps)?;
    let reconstruct = add(
        add(pairs, vector::<u32>(n, false)?)?,
        add(vector::<[u64; 4]>(n, true)?, vector::<usize>(n, true)?)?,
    )?;
    // Include the fixed histogram/offset arrays and parallel segment descriptors.
    let descriptors = 4usize
        .checked_mul(SHARDS)?
        .checked_mul(std::mem::size_of::<usize>())?
        .checked_add(SHARDS.checked_mul(std::mem::size_of::<&mut [(u32, u32)]>())?)?;
    add(
        add(
            hash_table::<&[u64; 4]>(n.min(2048))?,
            build.max(reconstruct),
        )?,
        descriptors,
    )
}

fn segments<'a>(
    pairs: &'a mut [(u32, u32)],
    populations: [usize; SHARDS],
) -> [&'a mut [(u32, u32)]; SHARDS] {
    let mut remaining = pairs;
    populations.map(|len| {
        let (head, tail) = std::mem::take(&mut remaining).split_at_mut(len);
        remaining = tail;
        head
    })
}

pub(super) fn try_group(
    scalars: &[[u64; 4]],
    percent: usize,
    capacity: usize,
    reservation: Reservation,
) -> Result<Option<ScalarGroups>, Reservation> {
    if scalars.len() < MIN_INPUT
        || scalars.len() >= u32::MAX as usize
        || rayon::current_num_threads() == 1
    {
        return Err(reservation);
    }
    let mut populations = [0usize; SHARDS];
    for scalar in scalars {
        if *scalar != [0; 4] {
            populations[(scalar[0] as usize) & (SHARDS - 1)] += 1;
        }
    }
    match (
        workspace(scalars.len(), &populations),
        grouping_cost::peak(scalars.len(), percent),
    ) {
        (Some(needed), Some(held)) if needed <= held => {}
        _ => return Err(reservation),
    }
    let mut positions = [0usize; SHARDS];
    for i in 1..SHARDS {
        positions[i] = positions[i - 1] + populations[i - 1];
    }
    let nonzero: usize = populations.iter().sum();
    let mut pairs = vec![(0u32, 0u32); nonzero];
    for (index, scalar) in scalars.iter().enumerate() {
        if *scalar != [0; 4] {
            let shard = (scalar[0] as usize) & (SHARDS - 1);
            pairs[positions[shard]] = (index as u32, index as u32);
            positions[shard] += 1;
        }
    }
    let cardinality: usize = segments(&mut pairs, populations)
        .into_par_iter()
        .map(|entries| {
            let mut map = HashMap::with_capacity_and_hasher(
                (capacity / SHARDS).min(entries.len()),
                ahash::RandomState::new(),
            );
            for (original, representative) in entries {
                *representative = *map.entry(scalars[*original as usize]).or_insert(*original);
            }
            // The map and any retired growth allocation drop before reconstruction.
            map.len()
        })
        .sum();
    let limit = scalars.len() / 100 * percent + scalars.len() % 100 * percent / 100;
    if cardinality > limit {
        return Ok(None);
    }
    let mut classes = vec![u32::MAX; scalars.len()];
    for &(original, representative) in &pairs {
        classes[original as usize] = representative;
    }
    let mut values = Vec::new();
    let mut counts = Vec::<usize>::new();
    for i in 0..classes.len() {
        let representative = classes[i];
        if representative == u32::MAX {
            continue;
        }
        let class = if representative as usize == i {
            let class = values.len() as u32;
            values.push(scalars[i]);
            counts.push(0);
            class
        } else {
            debug_assert!((representative as usize) < i);
            classes[representative as usize]
        };
        classes[i] = class;
        counts[class as usize] += 1;
    }
    debug_assert_eq!(values.len(), cardinality);
    drop(pairs);
    let mut starts = Vec::with_capacity(counts.len() + 1);
    starts.push(0);
    for &count in &counts {
        starts.push(starts.last().unwrap() + count);
    }
    let mut positions = starts[..counts.len()].to_vec();
    let mut order = vec![0u32; nonzero];
    for (index, &class) in classes.iter().enumerate() {
        if class != u32::MAX {
            order[positions[class as usize]] = index as u32;
            positions[class as usize] += 1;
        }
    }
    drop(classes);
    drop(positions);
    Ok(Some(ScalarGroups {
        len: scalars.len(),
        values,
        counts,
        starts,
        order,
        _reservation: reservation,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn check(words: &[[u64; 4]], percent: usize, must_run: bool) {
        let charge =
            grouping_cost::reserve(words.len(), percent).expect("bounded fixture admitted");
        let result = try_group(words, percent, 0, charge);
        let Ok(actual) = result else {
            assert!(!must_run);
            return;
        };
        let mut oracle = BTreeMap::<[u64; 4], Vec<u32>>::new();
        let mut first = Vec::new();
        for (i, &word) in words.iter().enumerate() {
            if word == [0; 4] {
                continue;
            }
            if !oracle.contains_key(&word) {
                first.push(word);
            }
            oracle.entry(word).or_default().push(i as u32);
        }
        if oracle.len() * 100 > words.len() * percent {
            assert!(actual.is_none());
            return;
        }
        let actual = actual.expect("exact cutoff accepts fixture");
        assert_eq!(actual.values, first);
        assert_eq!(actual.len, words.len());
        assert_eq!(actual.starts.len(), first.len() + 1);
        for (i, value) in first.iter().enumerate() {
            assert_eq!(
                &actual.order[actual.starts[i]..actual.starts[i + 1]],
                &oracle[value]
            );
            assert_eq!(actual.counts[i], oracle[value].len());
        }
        assert_eq!(*actual.starts.last().unwrap(), actual.order.len());
        assert_eq!(
            actual.order.len(),
            oracle.values().map(Vec::len).sum::<usize>()
        );
    }

    #[test]
    fn stable_layout_matches_independent_ordered_map() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        pool.install(|| {
            for n in [8192usize, 8193, 32768, 65537] {
                for percent in [80usize, 95] {
                    for kind in 0..8 {
                        let limit = n * percent / 100;
                        let words: Vec<_> = (0..n)
                            .map(|i| match kind {
                                0 => [0; 4],
                                1 => [u64::MAX, u64::MAX, u64::MAX, u64::MAX],
                                2 => {
                                    if i % 3 == 0 {
                                        [0; 4]
                                    } else {
                                        [(i % 17 + 1) as u64, 7, 9, 11]
                                    }
                                }
                                3 => [((i % 37 + 1) * 8) as u64, i as u64 % 7, 13, 17],
                                4 | 5 => {
                                    [((i % (limit + usize::from(kind == 5))) + 1) as u64, 0, 0, 0]
                                }
                                6 => [
                                    i as u64 + 1,
                                    (i as u64).wrapping_mul(0x9e3779b97f4a7c15),
                                    !i as u64,
                                    u64::MAX,
                                ],
                                _ => [((n - 1 - i) % 1027 + 1) as u64, 3, 5, 7],
                            })
                            .collect();
                        check(&words, percent, true);
                    }
                }
            }
        });
    }

    #[test]
    fn nested_and_concurrent_pools_preserve_stable_groups() {
        for workers in [1usize, 2, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            let words: Vec<_> = (0..8193).map(|i| [(i % 71) as u64, 0, 0, 0]).collect();
            pool.install(|| {
                rayon::join(
                    || check(&words, 80, workers > 1),
                    || check(&words, 95, workers > 1),
                );
            });
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        std::thread::scope(|scope| {
            for offset in 0..4 {
                let pool = &pool;
                scope.spawn(move || {
                    let words: Vec<_> = (0..16385)
                        .map(|i| [((i + offset) % 73) as u64, 11, 13, 17])
                        .collect();
                    pool.install(|| check(&words, 80, true));
                });
            }
        });
    }

    #[test]
    fn bounded_forecast_and_small_input_fallback() {
        assert!(workspace(usize::MAX, &[usize::MAX; SHARDS]).is_none());
        assert!(workspace(1, &[1; SHARDS]).is_none());
        for n in [8192usize, 32768, 65537, 1 << 20] {
            let mut balanced = [n / SHARDS; SHARDS];
            balanced[0] += n % SHARDS;
            let mut skewed = [0; SHARDS];
            skewed[0] = n;
            for counts in [balanced, skewed, [0; SHARDS]] {
                let needed = workspace(n, &counts).unwrap();
                assert!(needed <= grouping_cost::peak(n, 80).unwrap());
                assert!(
                    needed
                        >= grouping_cost::vector::<(u32, u32)>(counts.iter().sum(), false).unwrap()
                );
            }
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        pool.install(|| check(&vec![[1, 0, 0, 0]; 8191], 80, false));
    }
}
