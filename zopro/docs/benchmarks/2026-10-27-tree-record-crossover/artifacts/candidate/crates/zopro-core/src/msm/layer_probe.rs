use super::*;
use zopro_bn254::{G2Config, G2Projective};

/// Diagnostic fixture for a single existing affine-tree layer.
pub struct TreeLayerProbe {
    source: Vec<LimbPoint<Fq2>>,
    original_counts: Vec<usize>,
    starts: Vec<usize>,
    points: Vec<LimbPoint<Fq2>>,
    counts: Vec<usize>,
    scratch: Scratch<G2Config>,
    expected_full: Vec<LimbPoint<Fq2>>,
    expected_counts: Vec<usize>,
    ordinary_pairs: usize,
}

impl TreeLayerProbe {
    pub fn new(pairs: usize, mixed: bool, seed: u64) -> Self {
        assert!(pairs > 0 && pairs <= 1024);
        assert!(seed <= 1);
        let counts = if mixed {
            let q = pairs / 4;
            vec![
                0,
                2 * q + 1,
                2 * q,
                2 * q + 1,
                2 * (pairs - 3 * q) + 1,
                1,
                0,
            ]
        } else {
            vec![2 * pairs]
        };
        let mut starts = Vec::with_capacity(counts.len());
        let mut len = 0;
        for &n in &counts {
            starts.push(len);
            len += n;
        }
        let stride = G2Projective::generator() * Fr::from(1 + seed * 16);
        let mut p = G2Projective::generator() * Fr::from(11 + seed * 1_000_003);
        let raw: Vec<_> = (0..len)
            .map(|_| {
                p += stride;
                p
            })
            .collect();
        let mut input = G2Projective::batch_to_affine(&raw);
        if mixed {
            for (&start, &n) in starts.iter().zip(&counts) {
                for j in 0..n / 2 {
                    let a = input[start + 2 * j];
                    match j % 8 {
                        0 => input[start + 2 * j] = Affine::IDENTITY,
                        1 => input[start + 2 * j + 1] = a,
                        2 => input[start + 2 * j + 1] = -a,
                        _ => {}
                    }
                }
            }
        }
        let source: Vec<_> = input.iter().copied().map(limbs::<G2Config>).collect();
        let ordinary_pairs = starts
            .iter()
            .zip(&counts)
            .map(|(&start, &n)| {
                (0..n / 2)
                    .filter(|&j| {
                        let (a, b) = (input[start + 2 * j], input[start + 2 * j + 1]);
                        !a.is_identity() && !b.is_identity() && a.x != b.x
                    })
                    .count()
            })
            .sum();
        let mut expected_full = source.clone();
        let mut expected_counts = counts.clone();
        reduce_layer_impl::<G2Config, false>(
            &mut expected_full,
            &starts,
            &mut expected_counts,
            &mut Scratch::default(),
        );
        for (bucket, (&start, &n)) in starts.iter().zip(&counts).enumerate() {
            assert_eq!(expected_counts[bucket], n.div_ceil(2));
            for j in 0..n.div_ceil(2) {
                let a = input[start + 2 * j];
                let expected = if 2 * j + 1 < n {
                    (a + input[start + 2 * j + 1]).to_affine()
                } else {
                    a
                };
                assert_eq!(affine::<G2Config>(expected_full[start + j]), expected);
            }
        }
        Self {
            points: source.clone(),
            source,
            counts: counts.clone(),
            original_counts: counts,
            starts,
            scratch: Scratch::default(),
            expected_full,
            expected_counts,
            ordinary_pairs,
        }
    }

    pub fn reset(&mut self) {
        self.points.copy_from_slice(&self.source);
        self.counts.copy_from_slice(&self.original_counts);
    }

    pub fn layer(&mut self, mode: u8) {
        match mode {
            0 | 1 => reduce_layer_impl::<G2Config, false>(
                &mut self.points,
                &self.starts,
                &mut self.counts,
                &mut self.scratch,
            ),
            2 => reduce_layer_impl::<G2Config, true>(
                &mut self.points,
                &self.starts,
                &mut self.counts,
                &mut self.scratch,
            ),
            _ => panic!("unknown layer mode"),
        }
    }

    pub fn check(&self) {
        assert_eq!(self.counts, self.expected_counts);
        assert_eq!(self.points.len(), self.expected_full.len());
        for (actual, expected) in self.points.iter().zip(&self.expected_full) {
            assert!(
                actual.x == expected.x && actual.y == expected.y,
                "complete point-storage mismatch"
            );
        }
    }

    pub fn ordinary_pairs(&self) -> usize {
        self.ordinary_pairs
    }
    pub fn point_slots(&self) -> usize {
        self.source.len()
    }
    pub fn vector_inverse(&self) -> bool {
        self.scratch.vector_inverse
    }
    pub fn record_capacities(&self) -> [usize; 2] {
        [
            self.scratch.groups.capacity() * core::mem::size_of::<Group<Fq2>>(),
            self.scratch.prefix_records.capacity() * core::mem::size_of::<[FqBatch; 2]>(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_probe_complete_storage_matches_both_kernels() {
        for pairs in [1, 4, 7, 8, 9, 15, 16, 17, 32, 64, 128, 256, 512, 1024] {
            for mixed in [false, true] {
                for seed in [0, 1] {
                    let mut probe = TreeLayerProbe::new(pairs, mixed, seed);
                    for inverse in [false, true] {
                        probe.scratch.vector_inverse = inverse;
                        for mode in [0, 2, 1, 2] {
                            probe.reset();
                            probe.layer(mode);
                            probe.check();
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn layer_probe_geometry_and_reset_preserve_input_storage() {
        for pairs in [1, 8, 17, 128, 1024] {
            for mixed in [false, true] {
                let mut p = TreeLayerProbe::new(pairs, mixed, 1);
                assert_eq!(p.point_slots(), 2 * pairs + if mixed { 4 } else { 0 });
                if !mixed {
                    assert_eq!(p.ordinary_pairs(), pairs);
                } else {
                    assert!(p.ordinary_pairs() <= pairs);
                }
                let point_ptr = p.points.as_ptr();
                let count_ptr = p.counts.as_ptr();
                for mode in [0, 1, 2] {
                    p.reset();
                    p.layer(mode);
                    p.check();
                }
                p.reset();
                assert_eq!(p.points.as_ptr(), point_ptr);
                assert_eq!(p.counts.as_ptr(), count_ptr);
                assert_eq!(p.counts, p.original_counts);
                for (a, b) in p.points.iter().zip(&p.source) {
                    assert!(a.x == b.x && a.y == b.y);
                }
            }
        }
    }
}
