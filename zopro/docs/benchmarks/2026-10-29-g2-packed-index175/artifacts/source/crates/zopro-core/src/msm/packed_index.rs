//! Optional point-bounded indexed leaves spanning adjacent G2 buckets.
use super::*;
use crate::msm_affine::workspace::vector;
use core::mem::MaybeUninit;

struct Layout {
    segments: Vec<(usize, usize)>,
    partial_starts: Vec<usize>,
    tile_starts: Vec<usize>,
}
fn geometry(n: usize, starts: &[usize], tile: usize) -> Option<Layout> {
    if tile == 0
        || starts.is_empty()
        || starts[0] != 0
        || *starts.last()? != n
        || starts.windows(2).any(|w| w[0] > w[1])
    {
        return None;
    }
    let buckets = starts.len() - 1;
    let tiles = n.div_ceil(tile);
    let bound = n.min(tiles.checked_add(buckets)?);
    let mut segments = Vec::new();
    segments.try_reserve_exact(bound).ok()?;
    let mut partial_starts = Vec::new();
    partial_starts.try_reserve_exact(starts.len()).ok()?;
    let mut tile_starts = Vec::new();
    tile_starts.try_reserve_exact(tiles.checked_add(1)?).ok()?;
    partial_starts.push(0);
    tile_starts.push(0);
    for range in starts.windows(2) {
        let mut first = range[0];
        while first < range[1] {
            let boundary = (first / tile).checked_add(1)?.checked_mul(tile)?.min(n);
            let end = range[1].min(boundary);
            segments.push((first, end));
            if end == boundary && end < n {
                tile_starts.push(segments.len());
            }
            first = end;
        }
        partial_starts.push(segments.len());
    }
    if n > 0 {
        tile_starts.push(segments.len());
    }
    debug_assert_eq!(tile_starts.len(), tiles + 1);
    debug_assert!(segments.len() <= bound);
    Some(Layout {
        segments,
        partial_starts,
        tile_starts,
    })
}
fn peak<C: CurveConfig>(n: usize, buckets: usize, workers: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    if n == 0 || buckets == 0 || workers == 0 || n > u32::MAX as usize {
        return None;
    }
    let tile = 2048 / (core::mem::size_of::<C::Base>() / 32);
    let tiles = n.div_ceil(tile);
    let segments = n.min(tiles.checked_add(buckets)?);
    let mut held = vector::<(usize, usize)>(segments, false)?;
    for bytes in [
        vector::<usize>(buckets.checked_add(1)?, false)?,
        vector::<usize>(tiles.checked_add(1)?, false)?,
        vector::<Affine<C>>(segments, false)?,
        vector::<u32>(segments, false)?,
        vector::<usize>(buckets, false)?,
        vector::<Affine<C>>(buckets, false)?,
    ] {
        held = held.checked_add(bytes)?;
    }
    let local = vector::<usize>(buckets.min(tile), false)?.checked_mul(2)?;
    let leaf = workspace_grouping::<C>(n.min(tile), 1)?.checked_add(local)?;
    // Final groups are disjoint over the segment sums. Retained maximum local
    // capacities across for_each_init jobs sum to no more than this total.
    held.checked_add(leaf.checked_mul(workers)?)?
        .checked_add(workspace_grouping::<C>(segments, buckets.div_ceil(256))?)
}
fn fill<C: CurveConfig>(
    bases: &[Affine<C>],
    order: &[u32],
    segments: &[(usize, usize)],
    tiles: &[usize],
    out: &mut [MaybeUninit<Affine<C>>],
) where
    C::Base: BatchInverse,
{
    let tasks = tiles.len() - 1;
    debug_assert_eq!(out.len(), tiles[tasks] - tiles[0]);
    if tasks > 1 {
        let mid = tasks / 2;
        let (left, right) = out.split_at_mut(tiles[mid] - tiles[0]);
        rayon::join(
            || fill(bases, order, segments, &tiles[..=mid], left),
            || fill(bases, order, segments, &tiles[mid..], right),
        );
        return;
    }
    let local = &segments[tiles[0]..tiles[1]];
    let first = local[0].0;
    let end = local.last().unwrap().1;
    let mut points: Vec<_> = order[first..end]
        .iter()
        .map(|&i| limbs::<C>(bases[i as usize]))
        .collect();
    let starts: Vec<_> = local.iter().map(|&(a, _)| a - first).collect();
    let mut counts: Vec<_> = local.iter().map(|&(a, b)| b - a).collect();
    let mut scratch = Scratch::<C>::default();
    while counts.iter().any(|&n| n > 1) {
        reduce_layer(&mut points, &starts, &mut counts, &mut scratch);
    }
    for (i, slot) in out.iter_mut().enumerate() {
        slot.write(if counts[i] == 0 {
            Affine::IDENTITY
        } else {
            affine::<C>(points[starts[i]])
        });
    }
}
pub(super) fn try_reduce<C: CurveConfig>(
    bases: &[Affine<C>],
    order: &[u32],
    starts: &[usize],
) -> Option<Vec<Affine<C>>>
where
    C::Base: BatchInverse,
{
    if !C::Base::TREE_PREFIX {
        return None;
    }
    let buckets = starts.len().checked_sub(1)?;
    let _reservation = crate::preparation::Reservation::analysis(peak::<C>(
        order.len(),
        buckets,
        rayon::current_num_threads(),
    ))?;
    let tile = 2048 / (core::mem::size_of::<C::Base>() / 32);
    let layout = geometry(order.len(), starts, tile)?;
    let mut partials = Vec::<Affine<C>>::new();
    partials.try_reserve_exact(layout.segments.len()).ok()?;
    let mut counts = Vec::new();
    counts.try_reserve_exact(buckets).ok()?;
    counts.extend(layout.partial_starts.windows(2).map(|w| w[1] - w[0]));
    let mut partial_order = Vec::new();
    partial_order
        .try_reserve_exact(layout.segments.len())
        .ok()?;
    partial_order.extend((0..layout.segments.len()).map(|i| i as u32));
    if std::env::var_os("ZOPRO_INDEX_PACK_TRACE").is_some() {
        eprintln!(
            "PACKED_INDEX entered points={} buckets={} tile={} tasks={} segments={}",
            order.len(),
            buckets,
            tile,
            layout.tile_starts.len() - 1,
            layout.segments.len()
        );
    }
    fill(
        bases,
        order,
        &layout.segments,
        &layout.tile_starts,
        &mut partials.spare_capacity_mut()[..layout.segments.len()],
    );
    // SAFETY: disjoint recursive slices cover all segment output slots; every
    // nonforking leaf writes exactly its slice before all joins complete.
    // Affine coordinates have no drop obligations if a leaf panics.
    unsafe {
        partials.set_len(layout.segments.len());
    }
    Some(group_affine(
        &partials,
        &partial_order,
        &layout.partial_starts,
        &counts,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G2Affine, G2Config, G2Projective};
    #[test]
    fn geometry_covers_each_input_and_bucket_once() {
        for tile in [1usize, 7, 1024] {
            for n in [0usize, 1, 7, 1023, 1024, 1025, 2053, 4097] {
                for buckets in [1usize, 2, 257, 1024] {
                    let mut counts = vec![0usize; buckets];
                    for i in 0..n {
                        counts[(i * 17 + i / 7) % buckets] += 1;
                    }
                    let mut starts = vec![0usize];
                    for &c in &counts {
                        starts.push(starts.last().unwrap() + c);
                    }
                    let l = geometry(n, &starts, tile).unwrap();
                    let mut seen = vec![0u8; n];
                    assert_eq!(l.partial_starts.len(), buckets + 1);
                    assert_eq!(l.tile_starts.len(), n.div_ceil(tile) + 1);
                    for b in 0..buckets {
                        let mut covered = 0;
                        for &(a, z) in &l.segments[l.partial_starts[b]..l.partial_starts[b + 1]] {
                            assert!(a < z && a >= starts[b] && z <= starts[b + 1]);
                            covered += z - a;
                            for value in &mut seen[a..z] {
                                *value += 1;
                            }
                        }
                        assert_eq!(covered, counts[b]);
                    }
                    assert!(seen.iter().all(|&n| n == 1));
                    for (i, w) in l.tile_starts.windows(2).enumerate() {
                        let s = &l.segments[w[0]..w[1]];
                        assert_eq!(s[0].0, i * tile);
                        assert_eq!(s.last().unwrap().1, ((i + 1) * tile).min(n));
                        assert!(s.len() <= buckets.min(tile));
                    }
                    assert!(l.segments.len() <= n.div_ceil(tile) + buckets);
                }
            }
        }
    }
    #[test]
    fn grouped_tiles_match_independent_bucket_generator_sums() {
        let g = G2Projective::generator();
        for workers in [1, 4, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            pool.install(|| {
                for n in [1usize, 7, 1023, 1024, 1025, 4097] {
                    for mode in 0..3 {
                        let buckets = if mode == 0 { 1 } else { 257 };
                        let mut groups = vec![Vec::new(); buckets];
                        let mut expected = vec![Fr::ZERO; buckets];
                        let mut points = Vec::new();
                        let mut p = G2Projective::IDENTITY;
                        for i in 0..n {
                            p += g;
                            let (q, w) = match i % 13 {
                                0 => (G2Projective::IDENTITY, Fr::ZERO),
                                1 => (g, Fr::ONE),
                                2 => (-g, -Fr::ONE),
                                _ => {
                                    if i % 5 == 0 {
                                        (-p, -Fr::from((i + 1) as u64))
                                    } else {
                                        (p, Fr::from((i + 1) as u64))
                                    }
                                }
                            };
                            let b = if mode == 2 && i < n - 7.min(n) {
                                0
                            } else {
                                (i * 19) % buckets
                            };
                            points.push(q);
                            groups[b].push(i as u32);
                            expected[b] += w;
                        }
                        let points = G2Projective::batch_to_affine(&points);
                        let mut order = Vec::new();
                        let mut starts = vec![0];
                        for group in groups {
                            order.extend(group);
                            starts.push(order.len());
                        }
                        let actual = try_reduce::<G2Config>(&points, &order, &starts).unwrap();
                        assert_eq!(actual.len(), buckets);
                        for b in 0..buckets {
                            assert_eq!(
                                actual[b],
                                (g * expected[b]).to_affine(),
                                "workers={workers} n={n} mode={mode} bucket={b}"
                            );
                        }
                    }
                }
            });
        }
    }
    #[test]
    fn checked_forecast_and_denied_admission_preserve_inputs() {
        assert!(peak::<G2Config>(usize::MAX, 128, 32).is_none());
        assert!(peak::<G2Config>(1024, usize::MAX, 32).is_none());
        assert!(peak::<G2Config>(1024, 128, usize::MAX).is_none());
        assert!(geometry(1, &[0, 2, 1], 1024).is_none());
        assert!(geometry(1, &[1], 1024).is_none());
        assert!(geometry(0, &[0], 0).is_none());
        for workers in [1, 4, 16, 32, 96] {
            assert!(
                peak::<G2Config>(4097, 128, workers).unwrap()
                    > peak::<G2Config>(1024, 128, workers).unwrap()
            );
        }
        let points = vec![G2Affine::GENERATOR; 7];
        let order = vec![0, 1, 2, 3, 4, 5, 6];
        let starts = vec![0, 0, 7];
        let before = (points.clone(), order.clone(), starts.clone());
        let saved = std::env::var_os("ZOPRO_CPU_PREPARE_MAX_BYTES");
        std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", "0");
        let denied = try_reduce::<G2Config>(&points, &order, &starts);
        if let Some(s) = saved {
            std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", s);
        } else {
            std::env::remove_var("ZOPRO_CPU_PREPARE_MAX_BYTES");
        }
        assert!(denied.is_none());
        assert_eq!((points, order, starts), before);
        assert!(try_reduce::<G2Config>(&[], &[], &[0]).is_none());
    }
}
