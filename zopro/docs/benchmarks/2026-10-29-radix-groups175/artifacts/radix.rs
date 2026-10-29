//! Exact scalar grouping through compact sorted fingerprints.
//! Fingerprints route candidates; equality always compares all four words.
use super::{grouping_cost, ScalarGroups};
use crate::preparation::Reservation;
use std::hash::BuildHasher;

#[cfg(test)]
std::thread_local! {
    static TEST_BUILDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

fn workspace(n: usize, percent: usize) -> Option<usize> {
    use grouping_cost::{add, mul, vector};
    if n < 256 || n >= u32::MAX as usize || percent > 100 {
        return None;
    }
    let u = mul(n, percent)?.checked_div(100)?;
    let entries = vector::<u64>(n, false)?;
    let classes = vector::<u32>(n, false)?;
    let values = vector::<[u64; 4]>(u, true)?;
    let counts = vector::<usize>(u, true)?;
    let starts = vector::<usize>(add(u, 1)?, false)?;
    let positions = vector::<usize>(u, false)?;
    let order = vector::<u32>(n, false)?;
    let sorting = mul(entries, 2)?;
    let roots = add(entries, classes)?;
    let reconstruct = add(add(classes, values)?, counts)?;
    let lists = add(
        add(add(add(add(values, counts)?, classes)?, starts)?, positions)?,
        order,
    )?;
    let sample = grouping_cost::hash_table::<&[u64; 4]>(n.min(2048))?;
    // The explicit histogram payload is included conservatively. This is
    // not a bound on compiler frames, allocator retention or process RSS.
    add(
        add(sample, sorting.max(roots).max(reconstruct).max(lists))?,
        256 * core::mem::size_of::<usize>(),
    )
}

pub(super) fn fits(n: usize, percent: usize) -> bool {
    match (workspace(n, percent), grouping_cost::peak(n, percent)) {
        (Some(needed), Some(existing)) => needed <= existing,
        _ => false,
    }
}

fn radix_pairs(entries: &mut Vec<u64>, scratch: &mut Vec<u64>) {
    debug_assert_eq!(entries.len(), scratch.len());
    let mut bins = [0usize; 256];
    // Stable radix passes sort only the fingerprint. Input indices begin
    // in ascending order and remain ascending within an equal fingerprint.
    for shift in [32, 40, 48, 56] {
        bins.fill(0);
        for &entry in entries.iter() {
            bins[((entry >> shift) & 255) as usize] += 1;
        }
        let mut next = 0;
        for bin in &mut bins {
            let count = *bin;
            *bin = next;
            next += count;
        }
        for &entry in entries.iter() {
            let bin = ((entry >> shift) & 255) as usize;
            scratch[bins[bin]] = entry;
            bins[bin] += 1;
        }
        core::mem::swap(entries, scratch);
    }
}

pub(super) fn build(
    scalars: &[[u64; 4]],
    percent: usize,
    reservation: Reservation,
) -> Option<ScalarGroups> {
    #[cfg(test)]
    TEST_BUILDS.with(|count| count.set(count.get() + 1));
    let state = ahash::RandomState::new();
    build_with(scalars, percent, reservation, |s| state.hash_one(s) as u32)
}

fn build_with(
    scalars: &[[u64; 4]],
    percent: usize,
    reservation: Reservation,
    fingerprint: impl Fn(&[u64; 4]) -> u32,
) -> Option<ScalarGroups> {
    let n = scalars.len();
    let limit = n.checked_mul(percent)?.checked_div(100)?;
    if scalars.iter().all(|s| *s == [0; 4]) {
        return Some(ScalarGroups {
            len: n,
            values: Vec::new(),
            counts: Vec::new(),
            starts: vec![0],
            order: Vec::new(),
            _reservation: reservation,
        });
    }
    let mut entries = Vec::with_capacity(n);
    for (i, scalar) in scalars.iter().enumerate() {
        if *scalar != [0; 4] {
            entries.push(((fingerprint(scalar) as u64) << 32) | i as u64);
        }
    }
    let mut scratch = vec![0u64; entries.len()];
    radix_pairs(&mut entries, &mut scratch);
    drop(scratch);
    let mut classes = vec![u32::MAX; n];
    let mut groups = 0usize;
    let mut first = 0;
    while first < entries.len() {
        let tag = entries[first] >> 32;
        let mut end = first + 1;
        while end < entries.len() && entries[end] >> 32 == tag {
            end += 1;
        }
        let initial = entries[first] as u32 as usize;
        // Frequent equal scalars take a linear scan. For an actual hash
        // collision, sort full keys and then indices; correctness does not
        // depend on random hashing or collision probability.
        if !entries[first + 1..end]
            .iter()
            .all(|e| scalars[*e as u32 as usize] == scalars[initial])
        {
            entries[first..end].sort_unstable_by(|a, b| {
                let (i, j) = (*a as u32 as usize, *b as u32 as usize);
                scalars[i].cmp(&scalars[j]).then_with(|| i.cmp(&j))
            });
        }
        let mut start = first;
        while start < end {
            groups += 1;
            if groups > limit {
                return None;
            }
            let root = entries[start] as u32;
            let scalar = scalars[root as usize];
            let mut stop = start + 1;
            while stop < end && scalars[entries[stop] as u32 as usize] == scalar {
                stop += 1;
            }
            for e in &entries[start..stop] {
                classes[*e as u32 as usize] = root;
            }
            start = stop;
        }
        first = end;
    }
    drop(entries);
    // Each representative is the first original index in its exact class.
    // When visited in input order it already holds the final class ID.
    let mut values = Vec::new();
    let mut counts: Vec<usize> = Vec::new();
    for i in 0..n {
        let root = classes[i];
        if root == u32::MAX {
            continue;
        }
        let class = if root as usize == i {
            let id = values.len() as u32;
            values.push(scalars[i]);
            counts.push(0);
            id
        } else {
            debug_assert!((root as usize) < i);
            classes[root as usize]
        };
        classes[i] = class;
        counts[class as usize] += 1;
    }
    debug_assert_eq!(values.len(), groups);
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
        len: n,
        values,
        counts,
        starts,
        order,
        _reservation: reservation,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn compiled_policy_dispatch_matches_cfg() {
        let words: Vec<_> = (0..1025).map(|i| [(i % 17 + 1) as u64, 3, 5, 7]).collect();
        let before = TEST_BUILDS.with(|n| n.get());
        let groups = super::super::scalar_groups_policy(&words, true).unwrap();
        assert_eq!(groups.values.len(), 17);
        let after = TEST_BUILDS.with(|n| n.get());
        assert_eq!(after - before, usize::from(cfg!(zopro_radix_groups)));
    }

    #[test]
    fn radix_is_stable_and_conserves_indices() {
        for n in [0usize, 1, 2, 7, 255, 256, 257, 1025, 2049] {
            let mut seed = 0x7261646978u64;
            let mut entries: Vec<_> = (0..n)
                .map(|i| {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    let hash = match i % 7 {
                        0 => 0,
                        1 => u32::MAX,
                        2 => 1 << 31,
                        _ => seed as u32,
                    };
                    ((hash as u64) << 32) | i as u64
                })
                .collect();
            let mut expected = entries.clone();
            expected.sort_by_key(|e| e >> 32);
            let mut scratch = vec![0; entries.len()];
            radix_pairs(&mut entries, &mut scratch);
            assert_eq!(entries, expected, "n={n}");
            let mut indices: Vec<_> = entries.iter().map(|e| *e as u32).collect();
            indices.sort_unstable();
            assert_eq!(indices, (0..n as u32).collect::<Vec<_>>());
        }
    }

    #[test]
    fn exact_groups_match_ordered_oracle_with_adversarial_collisions() {
        let mut cases = 0;
        for n in [256usize, 1025, 4097, 32769] {
            for percent in [80usize, 95] {
                for kind in 0..8 {
                    let limit = n * percent / 100;
                    let words: Vec<_> = (0..n)
                        .map(|i| match kind {
                            0 => [0; 4],
                            1 => [u64::MAX, 7, 11, 13],
                            2 => [(i % limit + 1) as u64, 3, 5, 7],
                            3 => [(i % (limit + 1) + 1) as u64, 3, 5, 7],
                            4 => [5, 7, 11, (i / 4 + 1) as u64],
                            5 => {
                                if i % 5 == 0 {
                                    [0; 4]
                                } else {
                                    [(i + 1) as u64, 3, 5, 7]
                                }
                            }
                            6 => {
                                if i % 19 == 0 {
                                    [0; 4]
                                } else {
                                    [(i % 17 + 1) as u64, (i % 17) as u64, 7, 9]
                                }
                            }
                            _ => [
                                (n - i) as u64,
                                0x8000000000000000,
                                (i as u64).wrapping_mul(0x9e3779b97f4a7c15),
                                u64::MAX,
                            ],
                        })
                        .collect();
                    let mut oracle: BTreeMap<[u64; 4], Vec<u32>> = BTreeMap::new();
                    let mut first = vec![];
                    for (i, s) in words.iter().enumerate() {
                        if *s == [0; 4] {
                            continue;
                        }
                        if !oracle.contains_key(s) {
                            first.push(*s);
                        }
                        oracle.entry(*s).or_default().push(i as u32);
                    }
                    for hash in 0..4 {
                        let reservation = Reservation::analysis(workspace(n, percent)).unwrap();
                        let actual = build_with(&words, percent, reservation, |s| match hash {
                            0 => 0,
                            1 => s[0] as u32 & 3,
                            2 => (s[0] ^ s[3]) as u32,
                            _ => {
                                let mut h = s[0].wrapping_mul(0x9e3779b97f4a7c15) ^ s[1];
                                h = h.rotate_left(23) ^ s[2];
                                (h ^ s[3]) as u32
                            }
                        });
                        assert_eq!(
                            actual.is_some(),
                            oracle.len() <= limit,
                            "n={n} percent={percent} kind={kind} hash={hash}"
                        );
                        if let Some(g) = actual {
                            assert_eq!(g.len, n);
                            assert_eq!(g.values, first);
                            assert_eq!(g.counts.len(), first.len());
                            assert_eq!(g.starts.len(), first.len() + 1);
                            let mut flattened = vec![];
                            let mut starts = vec![0];
                            let mut counts = vec![];
                            for value in &first {
                                let ids = &oracle[value];
                                flattened.extend(ids);
                                starts.push(flattened.len());
                                counts.push(ids.len());
                            }
                            assert_eq!(g.order, flattened);
                            assert_eq!(g.starts, starts);
                            assert_eq!(g.counts, counts);
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert_eq!(cases, 256);
        println!("RADIX_EXACT_GROUP_ORACLE_CASES={cases}");
    }

    #[test]
    fn checked_workspace_fits_existing_reservation() {
        let mut cases = 0usize;
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            pool.install(|| {
                for percent in [80, 95] {
                    for n in 256..=65536 {
                        assert!(
                            fits(n, percent),
                            "n={n} percent={percent} workers={workers}"
                        );
                    }
                }
                for n in [
                    65537usize,
                    131072,
                    1 << 20,
                    1 << 24,
                    (u32::MAX - 1) as usize,
                ] {
                    for percent in [80, 95] {
                        assert!(
                            fits(n, percent),
                            "n={n} percent={percent} workers={workers}"
                        );
                    }
                }
            });
            cases += (65536 - 256 + 1) * 2 + 10;
        }
        for n in [0usize, 1, 255, u32::MAX as usize, usize::MAX] {
            assert!(!fits(n, 80));
        }
        assert!(!fits(32768, 101));
        println!("RADIX_RESERVATION_BOUND_CASES={cases}");
    }
}
