//! Write singleton scalar classes directly; only repeated classes need a tree.
use super::*;
use std::mem::MaybeUninit;

#[inline]
pub(super) fn fill<C: CurveConfig, F: Fn(u32) -> Affine<C>>(
    get: &F,
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
    first: usize,
    output: &mut [MaybeUninit<Affine<C>>],
    points: &mut Vec<LimbPoint<C::Base>>,
    scratch: &mut Scratch<C>,
) where
    C::Base: BatchInverse,
{
    const GROUP: usize = 256;
    assert!(output.len() <= GROUP);
    points.clear();
    let repeated = counts[first..first + output.len()]
        .iter().copied().filter(|&n| n > 1).sum();
    // The old path reserves every input point in this same output tile.
    // Eliding 0/1 classes cannot increase this requested capacity. Existing
    // capacity is reused, and the original reservation remains conservative.
    points.reserve_exact(repeated);
    let mut local_starts = [0usize; GROUP];
    let mut local_counts = [0usize; GROUP];
    for (j, out) in output.iter_mut().enumerate() {
        let n = counts[first + j];
        local_starts[j] = points.len();
        match n {
            0 => { out.write(Affine::IDENTITY); }
            1 => { out.write(get(order[starts[first + j]])); }
            _ => {
                local_counts[j] = n;
                for &index in &order[starts[first + j]..starts[first + j] + n] {
                    points.push(limbs::<C>(get(index)));
                }
            }
        }
    }
    let active = &mut local_counts[..output.len()];
    while active.iter().any(|&n| n > 1) {
        reduce_layer(points, &local_starts[..output.len()], active, scratch);
    }
    for (j, out) in output.iter_mut().enumerate() {
        if counts[first + j] > 1 {
            out.write(affine::<C>(points[local_starts[j]]));
        }
    }
    // Every slot is initialized once. If get/reduction panics, the caller's
    // Vec length stays unpublished, as in the original spare-capacity pass.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn cases<C: CurveConfig>() where C::Base: BatchInverse {
        let g = Projective::<C>::generator();
        let weights: Vec<_> = (0..127).map(|i| match i % 11 {
            0 => Fr::ZERO, 1 => Fr::ONE, 2 => -Fr::ONE,
            _ => Fr::from((i / 2 + 17) as u64),
        }).collect();
        let bases = Projective::<C>::batch_to_affine(
            &weights.iter().map(|s| g * *s).collect::<Vec<_>>());
        let original = bases.clone();
        let mut points = Vec::new();
        let mut scratch = Scratch::<C>::default();
        let mut previous_bound = 0;
        for size in [0usize, 1, 7, 255, 256, 257, 513] {
            for pattern in 0..4 {
                let counts: Vec<_> = (0..size).map(|i| match pattern {
                    0 => 0, 1 => 1, 2 => [0, 1, 2, 3, 7, 8, 9, 17][i % 8],
                    _ => if i % 2 == 0 { 1 } else { 2 },
                }).collect();
                let mut starts = vec![0];
                for &n in &counts { starts.push(starts.last().unwrap() + n); }
                let order: Vec<_> = (0..*starts.last().unwrap())
                    .map(|i| ((i * 37 + pattern * 19) % bases.len()) as u32).collect();
                let expected = Projective::<C>::batch_to_affine(&counts.iter().enumerate().map(|(j, &n)| {
                    let sum = order[starts[j]..starts[j] + n].iter()
                        .fold(Fr::ZERO, |s, &i| s + weights[i as usize]);
                    g * sum
                }).collect::<Vec<_>>());
                let reads = AtomicUsize::new(0);
                let get = |i: u32| { reads.fetch_add(1, Ordering::Relaxed); bases[i as usize] };
                let actual = group_affine_impl(get, &order, &starts, &counts);
                assert_eq!(actual, expected);
                assert_eq!(reads.load(Ordering::Relaxed), order.len());
                assert_eq!(group_affine(&bases, &order, &starts, &counts), expected);
                assert_eq!(group_affine_prepared(&bases[..63], &bases[63..], &order, &starts, &counts), expected);
                // Exercise retained scratch across increasing, shrinking,
                // empty, singleton-only and exceptional mixed tiles.
                for first in (0..size).step_by(256) {
                    let end = (first + 256).min(size);
                    let mut out = Vec::<Affine<C>>::with_capacity(end - first);
                    fill(&|i| bases[i as usize], &order, &starts, &counts, first,
                        &mut out.spare_capacity_mut()[..end - first], &mut points, &mut scratch);
                    let repeated: usize = counts[first..end].iter().copied().filter(|&n| n > 1).sum();
                    assert_eq!(points.len(), repeated);
                    previous_bound = previous_bound.max(starts[end] - starts[first]);
                    assert!(points.capacity() <= previous_bound);
                    // SAFETY: fill initialized each of these exact slots.
                    unsafe { out.set_len(end - first); }
                    assert_eq!(out, expected[first..end]);
                }
            }
        }
        assert_eq!(bases, original);
    }

    #[test]
    fn zero_singleton_repeated_partial_prepared_and_reused_scratch() {
        cases::<zopro_bn254::G1Config>();
        cases::<zopro_bn254::G2Config>();
    }

    #[test]
    fn every_worker_small_stacks_and_getter_panic() {
        use zopro_bn254::{G1Affine, G1Config, G2Affine};
        let mut workers_seen = 0;
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(workers)
                .stack_size(512 * 1024).build().unwrap();
            let indices = pool.broadcast(|ctx| {
                let counts = [0, 1, 2, 3, 1, 0, 9];
                let mut starts = vec![0];
                for n in counts { starts.push(starts.last().unwrap() + n); }
                let order: Vec<u32> = (0..*starts.last().unwrap()).map(|i| (i % 2) as u32).collect();
                for negative in [false, true] {
                    let a = G1Affine::GENERATOR;
                    let b = if negative { -a } else { a };
                    let result = group_affine(&[a, b], &order, &starts, &counts);
                    for (j, &n) in counts.iter().enumerate() {
                        let want = order[starts[j]..starts[j] + n].iter()
                            .fold(Projective::<G1Config>::IDENTITY, |p, &i| p + [a, b][i as usize]);
                        assert_eq!(result[j], want.to_affine());
                    }
                    let a = G2Affine::GENERATOR;
                    let b = if negative { -a } else { a };
                    let result = group_affine(&[a, b], &order, &starts, &counts);
                    for (j, &n) in counts.iter().enumerate() {
                        let want = order[starts[j]..starts[j] + n].iter()
                            .fold(Projective::<zopro_bn254::G2Config>::IDENTITY, |p, &i| p + [a, b][i as usize]);
                        assert_eq!(result[j], want.to_affine());
                    }
                }
                ctx.index()
            });
            assert_eq!(indices, (0..workers).collect::<Vec<_>>());
            workers_seen += indices.len();
            assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| pool.install(|| {
                group_affine_impl::<G1Config, _>(|_| panic!("expected getter panic"), &[0], &[0, 1], &[1])
            }))).is_err());
        }
        assert_eq!(workers_seen, 53);
    }
}
