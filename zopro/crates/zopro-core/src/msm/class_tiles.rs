//! Exact scalar-group tiles over disjoint shard metadata and final point rows.
use super::*;
use core::mem::MaybeUninit;

pub(crate) struct ClassShard<'a> {
    pub(crate) words: &'a [[u64; 4]],
    pub(crate) starts: &'a [usize],
    pub(crate) counts: &'a [usize],
    pub(crate) order: &'a [u32],
}

struct Tile<'a, C: CurveConfig> {
    shard: ClassShard<'a>,
    points: &'a mut [MaybeUninit<Affine<C>>],
    words: &'a mut [MaybeUninit<[u64; 4]>],
}

type CompactRows<C> = (Vec<Affine<C>>, Vec<[u64; 4]>);

pub(crate) fn compact<C: CurveConfig>(
    bases: &[Affine<C>],
    shards: &[ClassShard<'_>],
) -> Option<CompactRows<C>>
where
    C::Base: BatchInverse,
{
    let terms = shards
        .iter()
        .try_fold(0usize, |n, s| n.checked_add(s.words.len()))?;
    let jobs = shards.iter().try_fold(0usize, |n, s| {
        n.checked_add(s.words.len().div_ceil(CLASS_TILE))
    })?;
    let mut output_points = Vec::new();
    let mut output_words = Vec::new();
    let mut tiles = Vec::new();
    output_points.try_reserve_exact(terms).ok()?;
    output_words.try_reserve_exact(terms).ok()?;
    tiles.try_reserve_exact(jobs).ok()?;
    let mut points = &mut output_points.spare_capacity_mut()[..terms];
    let mut words = &mut output_words.spare_capacity_mut()[..terms];
    for shard in shards {
        let n = shard.words.len();
        assert_eq!(shard.counts.len(), n);
        assert_eq!(shard.starts.len(), n + 1);
        for first in (0..n).step_by(CLASS_TILE) {
            let end = (first + CLASS_TILE).min(n);
            let (points_tile, p_rest) = points.split_at_mut(end - first);
            let (words_tile, w_rest) = words.split_at_mut(end - first);
            points = p_rest;
            words = w_rest;
            tiles.push(Tile {
                shard: ClassShard {
                    words: &shard.words[first..end],
                    starts: &shard.starts[first..=end],
                    counts: &shard.counts[first..end],
                    order: shard.order,
                },
                points: points_tile,
                words: words_tile,
            });
        }
    }
    assert!(points.is_empty() && words.is_empty());
    let get = |i: u32| bases[i as usize];
    tiles.into_par_iter().for_each_init(
        || (Vec::<LimbPoint<C::Base>>::new(), Scratch::<C>::default()),
        |state, tile| {
            let s = tile.shard;
            reduce_class_tile(&get, s.order, s.starts, s.counts, tile.points, state);
            for (dst, &value) in tile.words.iter_mut().zip(s.words) {
                dst.write(value);
            }
        },
    );
    // SAFETY: the joined indexed pass wrote every output exactly once
    // through the disjoint borrowed slices. These types have no drop work.
    unsafe {
        output_points.set_len(terms);
        output_words.set_len(terms);
    }
    Some((output_points, output_words))
}
