use super::*;
mod arithmetic {
    include!("/home/ubuntu/src/zopro-perf-work/common-z-small-native176/kernel.rs");
    include!("common_z_segmented.rs");
}
pub(super) fn finish<C: CurveConfig<Base = Fq>>(
    points: &mut [LimbPoint<Fq>],
    starts: &[usize],
    counts: &mut [usize],
) -> bool {
    if !counts.iter().any(|&n| n >= 3) {
        return false;
    }
    let mut total = 0usize;
    for &n in counts.iter() {
        total += n;
        if total > 16 {
            return false;
        }
    }
    let mut input = [(Fq::ZERO, Fq::ZERO); 16];
    let mut compact = [0usize; 16];
    let mut mapping = [0usize; 16];
    let mut at = 0;
    let mut groups = 0;
    for (j, (&start, &n)) in starts.iter().zip(counts.iter()).enumerate() {
        if n == 0 {
            continue;
        }
        mapping[groups] = j;
        compact[groups] = n;
        groups += 1;
        for point in &points[start..start + n] {
            if point.is_infinity() {
                return false;
            }
            input[at] = (
                <Fq as BatchInverse>::from_limbs(point.x),
                <Fq as BatchInverse>::from_limbs(point.y),
            );
            at += 1;
        }
    }
    assert_eq!(at, total);
    let Some(output) = arithmetic::segments(&input[..total], &compact[..groups]) else {
        return false;
    };
    for j in 0..groups {
        let original = mapping[j];
        let (x, y) = output[j];
        points[starts[original]] = limbs::<C>(Affine::new(x, y));
        counts[original] = 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G1Affine, G1Config, G1Projective};
    fn complete(p: &[G1Affine]) -> G1Affine {
        p.iter()
            .fold(G1Projective::IDENTITY, |a, p| a + *p)
            .to_affine()
    }
    fn walk(n: usize) -> Vec<G1Affine> {
        let g = G1Affine::GENERATOR;
        let mut p = g.mul(&Fr::from_canonical([0x5123a99af71e2345, 0x212d28b2, 0, 0]).unwrap());
        let step = g.mul_limbs(&[137]);
        (0..n)
            .map(|_| {
                let q = p.to_affine();
                p += step;
                q
            })
            .collect()
    }
    #[test]
    fn segmented_frontier_preserves_outputs_and_transactional_fallback() {
        let base = walk(32);
        let mut patterns = Vec::new();
        // Exhaust compositions through16 with at most4 nonempty groups, including
        // odd carries and singleton groups; zero groups are inserted separately.
        for a in 0..=16 {
            for b in 0..=16 - a {
                for c in 0..=16 - a - b {
                    for d in 0..=16 - a - b - c {
                        if a + b + c + d <= 16 && [a, b, c, d].iter().any(|&n| n >= 3) {
                            patterns.push(vec![a, 0, b, c, 0, d]);
                        }
                    }
                }
            }
        }
        patterns.extend([
            vec![2; 8],
            vec![1; 16],
            vec![3, 3, 3, 3, 1, 1, 1, 1],
            vec![17],
        ]);
        for counts in patterns {
            let n = counts.iter().sum::<usize>();
            let mut starts = Vec::new();
            let mut values = Vec::new();
            let mut source_at = 0;
            let mut points = Vec::new();
            for &count in &counts {
                starts.push(points.len());
                let p = base[source_at..source_at + count].to_vec();
                source_at += count;
                points.extend(p.iter().map(|&p| limbs::<G1Config>(p)));
                values.push(p);
                points.extend([LimbPoint::<Fq>::infinity(); 3]);
            }
            let saved = points.clone();
            let mut current = counts.clone();
            let used = finish::<G1Config>(&mut points, &starts, &mut current);
            let expected = n <= 16 && counts.iter().any(|&n| n >= 3);
            assert_eq!(used, expected);
            for j in 0..points.len() {
                if used && starts.iter().zip(&counts).any(|(&s, &n)| n != 0 && s == j) {
                    continue;
                }
                assert_eq!(points[j].x, saved[j].x);
                assert_eq!(points[j].y, saved[j].y);
            }
            for (j, p) in values.iter().enumerate() {
                if used && counts[j] != 0 {
                    assert_eq!(affine::<G1Config>(points[starts[j]]), complete(p));
                    assert_eq!(current[j], 1);
                } else {
                    assert_eq!(current[j], counts[j]);
                }
            }
        }
        let g = G1Affine::GENERATOR;
        let two = g.mul_limbs(&[2]).to_affine();
        let four = g.mul_limbs(&[4]).to_affine();
        for p in [
            vec![g, g, g],
            vec![g, -g, g],
            vec![g, two, four, -g],
            vec![g, two, -four, g],
            vec![g, two, G1Affine::IDENTITY, four],
        ] {
            let mut points = p.iter().copied().map(limbs::<G1Config>).collect::<Vec<_>>();
            let saved = points.clone();
            let mut counts = [p.len()];
            assert!(!finish::<G1Config>(&mut points, &[0], &mut counts));
            assert_eq!(counts, [p.len()]);
            for (a, b) in points.iter().zip(saved) {
                assert_eq!(a.x, b.x);
                assert_eq!(a.y, b.y);
            }
        }
    }
    #[test]
    fn production_grouping_and_indexed_paths_preserve_complete_results() {
        let counts = [4097, 3, 0, 1, 7, 16, 4, 31];
        let n = counts.iter().sum();
        let bases = walk(n);
        let order = (0..n as u32).collect::<Vec<_>>();
        let mut starts = vec![0];
        for &count in &counts {
            starts.push(starts.last().unwrap() + count);
        }
        let expected = starts
            .windows(2)
            .map(|r| complete(&bases[r[0]..r[1]]))
            .collect::<Vec<_>>();
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 << 10)
                .build()
                .unwrap();
            pool.install(|| {
                assert_eq!(
                    group_affine::<G1Config>(&bases, &order, &starts, &counts),
                    expected
                );
                assert_eq!(
                    reduce_indexed_buckets::<G1Config>(&bases, &order, &starts),
                    expected
                );
            });
        }
    }
}
