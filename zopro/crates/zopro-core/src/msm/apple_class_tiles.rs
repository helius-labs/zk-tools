//! Bound point-reduction scratch for large scalar groups on Apple silicon.
use super::*;
use core::mem::{MaybeUninit, size_of};

pub(crate) const MIN_BASES: usize = 1 << 18;
const TILE_POINTS: usize = 2048;

/// An oversized class takes fewer than twice as many tiles as its length
/// divided by the tile size. Each worker retains one tile's scratch.
pub(crate) fn workspace<C: CurveConfig>(n: usize, workers: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    let partials = n.checked_mul(2)? / TILE_POINTS;
    let oversized = n / (TILE_POINTS + 1);
    let tables = partials
        .checked_mul(size_of::<&[u32]>() + size_of::<Affine<C>>())?
        .checked_add(oversized.checked_mul(size_of::<(usize, Affine<C>)>())?)?
        .checked_add(oversized.checked_mul(size_of::<(usize, usize)>())?)?
        .checked_add(4 * 64)?;
    // Include the prior scratch allocation during a growth operation.
    let scratch = workspace_grouping_linear::<C>(workers.checked_mul(TILE_POINTS)?, workers)?;
    tables.checked_add(scratch.checked_mul(2)?)
}

pub(super) fn group<C: CurveConfig>(
    bases: &[Affine<C>],
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    let get = |index: u32| bases[index as usize];
    let n = counts.len();
    let oversized = sum_oversized::<C>(&get, order, starts, counts);
    let mut result = Vec::with_capacity(n);
    result.spare_capacity_mut()[..n]
        .par_chunks_mut(CLASS_TILE)
        .enumerate()
        .for_each_init(
            || (Vec::new(), Scratch::default()),
            |scratch, (tile, output)| {
                let first = tile * CLASS_TILE;
                let end = first + output.len();
                let starts = &starts[first..=end];
                let counts = &counts[first..end];
                if starts[output.len()] - starts[0] <= TILE_POINTS {
                    return reduce_class_tile(&get, order, starts, counts, output, scratch);
                }
                let next = oversized.partition_point(|&(class, _)| class < first);
                let mut oversized = oversized[next..].iter();
                let mut i = 0;
                while i < output.len() {
                    if counts[i] > TILE_POINTS {
                        let &(class, sum) = oversized.next().expect("oversized class sum");
                        debug_assert_eq!(class, first + i);
                        output[i].write(sum);
                        i += 1;
                    } else {
                        let end = i + starts[i..]
                            .partition_point(|&offset| offset - starts[i] <= TILE_POINTS)
                            - 1;
                        reduce_class_tile(
                            &get,
                            order,
                            &starts[i..=end],
                            &counts[i..end],
                            &mut output[i..end],
                            scratch,
                        );
                        i = end;
                    }
                }
            },
        );
    // SAFETY: the joined pass initializes each output exactly once.
    unsafe { result.set_len(n) };
    result
}

fn sum_oversized<C: CurveConfig>(
    get: &(impl Fn(u32) -> Affine<C> + Sync),
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
) -> Vec<(usize, Affine<C>)>
where
    C::Base: BatchInverse,
{
    let large_count = counts.iter().filter(|&&count| count > TILE_POINTS).count();
    if large_count == 0 {
        return Vec::new();
    }
    let mut classes = Vec::with_capacity(large_count);
    let mut tiles = Vec::with_capacity(2 * order.len() / TILE_POINTS);
    for (i, &count) in counts.iter().enumerate() {
        if count > TILE_POINTS {
            classes.push((i, tiles.len()));
            tiles.extend(order[starts[i]..starts[i + 1]].chunks(TILE_POINTS));
        }
    }
    let partials: Vec<_> = tiles
        .par_iter()
        .map_init(
            || (Vec::new(), Scratch::default()),
            |scratch, &tile| {
                let mut result = [MaybeUninit::uninit()];
                reduce_class_tile(
                    get,
                    tile,
                    &[0, tile.len()],
                    &[tile.len()],
                    &mut result,
                    scratch,
                );
                // SAFETY: reduce_class_tile always writes its one output here.
                unsafe { result[0].assume_init() }
            },
        )
        .collect();
    let mut sums = Vec::with_capacity(large_count);
    for (i, &(class, start)) in classes.iter().enumerate() {
        let end = classes.get(i + 1).map_or(partials.len(), |&(_, end)| end);
        let mut sum = Projective::<C>::IDENTITY;
        for point in &partials[start..end] {
            sum += *point;
        }
        sums.push((class, sum.to_affine()));
    }
    sums
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G1Config, G2Config};

    fn check<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let counts: Vec<_> = [TILE_POINTS, TILE_POINTS + 1, 1, TILE_POINTS * 3 + 7]
            .into_iter()
            .chain(std::iter::repeat_n(1, CLASS_TILE + 8))
            .chain([TILE_POINTS * 2, TILE_POINTS - 1])
            .collect();
        let mut starts = vec![0];
        for count in &counts {
            starts.push(starts.last().unwrap() + count);
        }
        let len = *starts.last().unwrap();
        let generator = Affine::<C>::GENERATOR;
        let bases: Vec<_> = (0..len)
            .map(|i| match i % 5 {
                1 => Affine::IDENTITY,
                2 | 4 => -generator,
                _ => generator,
            })
            .collect();
        let order: Vec<_> = (0..len as u32).rev().collect();
        let expected = group_affine_impl(|i| bases[i as usize], &order, &starts, &counts);
        assert_eq!(group(&bases, &order, &starts, &counts), expected);
    }

    #[test]
    fn split_classes_match_unbounded_reduction() {
        check::<G1Config>();
        check::<G2Config>();
    }
}
