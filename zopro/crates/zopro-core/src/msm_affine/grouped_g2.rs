//! Combine bases with equal coefficients before the native G2 MSM.
use rayon::prelude::*;
use zopro_bn254::{Fq2, G2Affine, G2Projective};

type Coefficient = [u64; 4];
const MAX_SCRATCH: usize = 32 << 20;
const BATCH: usize = 128;

pub(crate) fn msm(bases: &[G2Affine], scalars: &[Coefficient]) -> Option<G2Projective> {
    let n = scalars.len();
    // Limit the index arrays and keep the u32 indices valid.
    if bases.len() != n || !(256..=1 << 19).contains(&n) {
        return None;
    }

    let mut order = Vec::new();
    order.try_reserve_exact(n).ok()?;
    order.extend(
        scalars
            .iter()
            .enumerate()
            .filter_map(|(i, &scalar)| (scalar != [0; 4]).then_some(i as u32)),
    );
    if order.is_empty() {
        return Some(G2Projective::IDENTITY);
    }
    order.par_sort_unstable_by_key(|&i| scalars[i as usize]);

    let mut starts = Vec::new();
    starts.try_reserve_exact(order.len() + 1).ok()?;
    starts.push(0u32);
    for (i, pair) in order.windows(2).enumerate() {
        if scalars[pair[0] as usize] != scalars[pair[1] as usize] {
            starts.push((i + 1) as u32);
        }
    }

    let terms = starts.len();
    if terms * 8 > order.len() * 7 {
        return None;
    }
    let indices = (order.capacity() + starts.capacity()) * size_of::<u32>();
    let output = terms * (size_of::<G2Affine>() + size_of::<Coefficient>());
    // Per-worker point sums, prefix products and affine output for one batch.
    let batch_bytes =
        BATCH * (size_of::<G2Projective>() + size_of::<Fq2>() + size_of::<G2Affine>());
    if indices + output + rayon::current_num_threads() * batch_bytes > MAX_SCRATCH {
        return None;
    }

    starts.push(order.len() as u32);
    let mut compact = Vec::new();
    let mut words = Vec::new();
    compact.try_reserve_exact(terms).ok()?;
    words.try_reserve_exact(terms).ok()?;
    compact.resize(terms, G2Affine::IDENTITY);
    words.extend(
        starts[..terms]
            .iter()
            .map(|&i| scalars[order[i as usize] as usize]),
    );
    compact
        .par_chunks_mut(BATCH)
        .enumerate()
        .for_each(|(index, output)| {
            let first = index * BATCH;
            let mut sums = [G2Projective::IDENTITY; BATCH];
            for (sum, range) in sums
                .iter_mut()
                .zip(starts[first..=first + output.len()].windows(2))
            {
                for &point in &order[range[0] as usize..range[1] as usize] {
                    *sum += bases[point as usize];
                }
            }
            output.copy_from_slice(&G2Projective::batch_to_affine(&sums[..output.len()]));
        });
    drop(starts);
    drop(order);
    Some(super::msm_canonical(&compact, &words))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::Fr;

    #[test]
    fn repeated_scalars_preserve_the_g2_query() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        pool.install(|| {
            let generator = G2Affine::GENERATOR;
            let mut points = Vec::new();
            let mut scalars = Vec::new();
            let mut expected = Fr::ZERO;
            for i in 0..1027u64 {
                let base = if i % 5 == 0 {
                    Fr::ZERO
                } else {
                    Fr::from(i % 37) - Fr::from(18)
                };
                let scalar = if i % 3 == 0 {
                    Fr::ZERO
                } else {
                    Fr::from(i % 241) - Fr::from(120)
                };
                points.push((generator * base).to_affine());
                scalars.push(scalar.to_canonical());
                expected += base * scalar;
            }
            assert_eq!(msm(&points, &scalars).unwrap(), generator * expected);
            assert_eq!(
                msm(&points, &vec![[0; 4]; points.len()]).unwrap(),
                G2Projective::IDENTITY
            );
            let all_distinct: Vec<_> = (1..=points.len() as u64)
                .map(|i| Fr::from(i).to_canonical())
                .collect();
            assert!(msm(&points, &all_distinct).is_none());
            let mut sparse = all_distinct;
            sparse[100..].fill([0; 4]);
            assert!(msm(&points, &sparse).is_none());
        });
    }
}
