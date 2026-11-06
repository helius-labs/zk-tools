// Private performance adapter: no production arithmetic implementation.
pub struct WeightedCaller {
    bases: Vec<G1Affine>,
    expected: G1Affine,
    output: G1Projective,
}
impl WeightedCaller {
    pub fn new(n: usize) -> Self {
        let words: Vec<_> = (0..n)
            .map(|i| {
                (Fr::from(1u64 << 48) * Fr::from(i as u64 + 1)
                    + Fr::from(1800000054u64)
                    + Fr::from(i as u64) * Fr::from(137u64))
                .to_canonical()
            })
            .collect();
        Self::from_words(&words)
    }
    pub fn from_words(words: &[[u64; 4]]) -> Self {
        assert!(words.chunks(256).all(|c| c.len().is_power_of_two()));
        let values: Vec<_> = words
            .iter()
            .map(|w| Fr::from_canonical(*w).unwrap())
            .collect();
        let g = G1Projective::generator();
        let bases =
            G1Projective::batch_to_affine(&values.iter().map(|s| g * *s).collect::<Vec<_>>());
        let sum = values
            .iter()
            .enumerate()
            .fold(Fr::ZERO, |a, (i, &s)| a + Fr::from(i as u64 + 1) * s);
        Self {
            bases,
            expected: (g * sum).to_affine(),
            output: G1Projective::IDENTITY,
        }
    }
    pub fn run(&mut self, rounds: usize) {
        for _ in 0..rounds {
            let s = black_box(&mut *self);
            s.output = weighted_affine::<G1Config>(&s.bases);
            black_box(&s.output);
        }
    }
    pub fn check(&self) {
        assert_eq!(self.output.to_affine(), self.expected);
    }
    pub fn coordinates(&self) -> [[u64; 4]; 2] {
        let p = self.output.to_affine();
        [p.x.to_canonical(), p.y.to_canonical()]
    }
    #[cfg(all(
        zopro_resource_schedule,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    pub fn source_forecast_bytes(&self) -> Option<usize> {
        workspace_weighted::<G1Config>(self.bases.len())
    }
}
