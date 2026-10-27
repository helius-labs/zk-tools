use super::*;

pub trait ShortFixture: Sized {
    fn new(pairs: usize, mixed: bool, seed: u64) -> Self;
    fn reset(&mut self);
    fn layer(&mut self, mode: u8);
    fn check(&self);
    fn ordinary_pairs(&self) -> usize;
    fn vector_inverse(&self) -> bool;
    fn record_capacity_bytes(&self) -> usize;
}

impl<C: CurveConfig> ShortFixture for ShortLayerProbe<C>
where
    C::Base: BatchInverse,
{
    fn new(pairs: usize, mixed: bool, seed: u64) -> Self {
        Self::new(pairs, mixed, seed)
    }
    fn reset(&mut self) {
        self.reset()
    }
    fn layer(&mut self, mode: u8) {
        self.layer(mode)
    }
    fn check(&self) {
        self.check()
    }
    fn ordinary_pairs(&self) -> usize {
        self.ordinary_pairs()
    }
    fn vector_inverse(&self) -> bool {
        self.vector_inverse()
    }
    fn record_capacity_bytes(&self) -> usize {
        self.record_capacity_bytes()
    }
}

pub struct ShortLayerProbe<C: CurveConfig>
where
    C::Base: BatchInverse,
{
    source: Vec<LimbPoint<C::Base>>,
    original_counts: Vec<usize>,
    starts: Vec<usize>,
    points: Vec<LimbPoint<C::Base>>,
    counts: Vec<usize>,
    scratch: Scratch<C>,
    expected_full: Vec<LimbPoint<C::Base>>,
    expected_counts: Vec<usize>,
    ordinary_pairs: usize,
}

impl<C: CurveConfig> ShortLayerProbe<C>
where
    C::Base: BatchInverse,
{
    pub fn new(pairs: usize, mixed: bool, seed: u64) -> Self {
        assert!(pairs <= 64);
        let counts = if mixed {
            vec![
                0,
                2 * pairs / 3 + 1,
                2 * pairs / 3,
                2 * pairs - 4 * pairs / 3 + 1,
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
        let stride = Projective::<C>::generator() * Fr::from(1 + seed * 16);
        let mut p = Projective::<C>::generator() * Fr::from(11 + seed * 1_000_003);
        let raw: Vec<_> = (0..len)
            .map(|_| {
                p += stride;
                p
            })
            .collect();
        let mut input = Projective::<C>::batch_to_affine(&raw);
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
        let source: Vec<_> = input.iter().copied().map(limbs::<C>).collect();
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
        reduce_layer_original::<C>(
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
                assert_eq!(affine::<C>(expected_full[start + j]), expected);
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
            0 | 1 => reduce_layer_original::<C>(
                &mut self.points,
                &self.starts,
                &mut self.counts,
                &mut self.scratch,
            ),
            2 => reduce_layer::<C>(
                &mut self.points,
                &self.starts,
                &mut self.counts,
                &mut self.scratch,
            ),
            _ => panic!("unknown mode"),
        }
    }
    pub fn check(&self) {
        assert_eq!(self.counts, self.expected_counts);
        assert_eq!(self.points.len(), self.expected_full.len());
        for (a, b) in self.points.iter().zip(&self.expected_full) {
            assert!(a.x == b.x && a.y == b.y, "backing point storage mismatch");
        }
    }
    pub fn ordinary_pairs(&self) -> usize {
        self.ordinary_pairs
    }
    pub fn vector_inverse(&self) -> bool {
        self.scratch.vector_inverse
    }
    pub fn record_capacity_bytes(&self) -> usize {
        self.scratch.groups.capacity() * core::mem::size_of::<Group<C::Base>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G1Config, G2Config};
    #[test]
    fn active_inverse_matches_independent_scalar_elements() {
        let mut rng = 0x91bcfe5397a08d65u64;
        for sample in 0..1024 {
            for used in 1..LANES {
                let mut values = [Fq::ONE; LANES];
                for (i, v) in values[..used].iter_mut().enumerate() {
                    let words = core::array::from_fn(|_| {
                        rng ^= rng << 13;
                        rng ^= rng >> 7;
                        rng ^= rng << 17;
                        rng
                    });
                    *v = match (sample + i) % 16 {
                        0 => Fq::ONE,
                        1 => -Fq::ONE,
                        _ => Fq::from_canonical(words).unwrap_or(Fq::from(rng | 1)),
                    };
                    if v.is_zero() {
                        *v = Fq::ONE;
                    }
                }
                let got = invert_short_lanes(values, used);
                for i in 0..LANES {
                    assert_eq!(got[i], values[i].inverse().unwrap());
                    assert_eq!(got[i] * values[i], Fq::ONE);
                }
            }
        }
    }
    fn layers<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        for mixed in [false, true] {
            for seed in [0, 1, 7, 31] {
                for pairs in (0..=3 * LANES + 3).chain([64, 7, 1, 0]) {
                    let mut probe = ShortLayerProbe::<C>::new(pairs, mixed, seed);
                    let point_ptr = probe.points.as_ptr();
                    let count_ptr = probe.counts.as_ptr();
                    for inverse in [false, true] {
                        probe.scratch.vector_inverse = inverse;
                        for mode in [0, 2, 1, 2] {
                            probe.reset();
                            probe.layer(mode);
                            probe.check();
                        }
                    }
                    probe.reset();
                    assert_eq!(point_ptr, probe.points.as_ptr());
                    assert_eq!(count_ptr, probe.counts.as_ptr());
                    if !mixed {
                        assert_eq!(probe.ordinary_pairs, pairs);
                    }
                }
            }
        }
    }
    #[test]
    fn short_tree_g1_complete_storage_and_reuse() {
        layers::<G1Config>();
    }
    #[test]
    fn short_tree_g2_complete_storage_and_reuse() {
        layers::<G2Config>();
    }
}
