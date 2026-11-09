//! Optional two-pass splitting of oversized scalar-equivalence classes.
use super::*;
fn forecast<C: CurveConfig>(
    classes: usize,
    partials: usize,
    workers: usize,
    tile: usize,
) -> Option<usize>
where
    C::Base: BatchInverse,
{
    let vector = |n: usize, each: usize| n.checked_mul(each)?.checked_add(64);
    vector(classes.checked_add(1)?, core::mem::size_of::<usize>())?
        .checked_add(vector(partials, core::mem::size_of::<(usize, usize)>())?)?
        .checked_add(vector(partials, core::mem::size_of::<Affine<C>>())?)?
        .checked_add(workspace_grouping_linear::<C>(tile, 1)?.checked_mul(workers.min(partials))?)?
        .checked_add(256)
}
pub(super) fn try_reduce<C: CurveConfig, F: Fn(u32) -> Affine<C> + Sync>(
    get: &F,
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
) -> Option<Vec<Affine<C>>>
where
    C::Base: BatchInverse,
{
    let tile = 2048 / (core::mem::size_of::<C::Base>() / 32);
    let mut partials_len = 0usize;
    for &count in counts {
        if count > tile {
            partials_len = partials_len.checked_add(count.div_ceil(tile))?;
        }
    }
    if partials_len == 0 {
        return None;
    }
    let extra = forecast::<C>(
        counts.len(),
        partials_len,
        rayon::current_num_threads(),
        tile,
    )?;
    let _charge = crate::preparation::Reservation::analysis(Some(extra))?;
    let mut partial_starts = Vec::with_capacity(counts.len() + 1);
    partial_starts.push(0);
    let mut tasks = Vec::with_capacity(partials_len);
    for (i, &count) in counts.iter().enumerate() {
        if count > tile {
            let begin = starts[i];
            let end = starts[i + 1];
            for first in (begin..end).step_by(tile) {
                tasks.push((first, (first + tile).min(end)));
            }
        }
        partial_starts.push(tasks.len());
    }
    assert_eq!(tasks.len(), partials_len);
    let partials: Vec<Affine<C>> = tasks
        .into_par_iter()
        .map_init(
            || (Vec::<LimbPoint<C::Base>>::new(), Scratch::<C>::default()),
            |(points, scratch), (first, end)| {
                points.clear();
                points.reserve_exact(end - first);
                for &index in &order[first..end] {
                    points.push(limbs::<C>(get(index)));
                }
                let mut count = [points.len()];
                while count[0] > 1 {
                    reduce_layer(points, &[0], &mut count, scratch);
                }
                if count[0] == 0 {
                    Affine::IDENTITY
                } else {
                    affine::<C>(points[0])
                }
            },
        )
        .collect();
    const GROUP: usize = 256;
    let n = counts.len();
    let mut result: Vec<Affine<C>> = Vec::with_capacity(n);
    result.spare_capacity_mut()[..n]
        .par_chunks_mut(GROUP)
        .enumerate()
        .for_each_init(
            || (Vec::<LimbPoint<C::Base>>::new(), Scratch::<C>::default()),
            |(points, scratch), (chunk, output)| {
                let first = chunk * GROUP;
                let mut local_starts = [0usize; GROUP];
                let mut local_counts = [0usize; GROUP];
                let needed = (first..first + output.len())
                    .map(|i| {
                        if counts[i] > tile {
                            partial_starts[i + 1] - partial_starts[i]
                        } else {
                            counts[i]
                        }
                    })
                    .sum();
                points.clear();
                points.reserve_exact(needed);
                for j in 0..output.len() {
                    let i = first + j;
                    local_starts[j] = points.len();
                    if counts[i] > tile {
                        for &p in &partials[partial_starts[i]..partial_starts[i + 1]] {
                            points.push(limbs::<C>(p));
                        }
                    } else {
                        for &index in &order[starts[i]..starts[i + 1]] {
                            points.push(limbs::<C>(get(index)));
                        }
                    }
                    local_counts[j] = points.len() - local_starts[j];
                }
                let starts = &local_starts[..output.len()];
                let counts = &mut local_counts[..output.len()];
                while counts.iter().any(|&n| n > 1) {
                    reduce_layer(points, starts, counts, scratch);
                }
                for (j, out) in output.iter_mut().enumerate() {
                    out.write(if counts[j] == 0 {
                        Affine::IDENTITY
                    } else {
                        affine::<C>(points[starts[j]])
                    });
                }
            },
        );
    // Each completed disjoint output chunk initializes all its own slots. Affine
    // has no drop obligation; a getter panic leaves the published length at zero.
    unsafe { result.set_len(n) };
    Some(result)
}
#[cfg(test)]
include!("large_class_tests.rs");
