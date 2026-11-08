//! Immutable option ownership for one tree reduction operation.
use super::*;

#[derive(Clone, Copy)]
pub(super) struct Factory {
    captured: Option<bool>,
}

impl Factory {
    pub(super) fn new() -> Self {
        // Only experiment controls are process-latched. The arithmetic option
        // below is read afresh for every operation owner.
        static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        static TRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let enabled = *ENABLED.get_or_init(|| {
            std::env::var("ZOPRO_CPU_TREE_OPTION_OWNER").as_deref() == Ok("1")
        });
        let owner = Self::from_source(enabled, || {
            crate::simd::cpu_option("ZOPRO_CPU_TREE_VECTOR_INVERSE")
        });
        if *TRACE.get_or_init(|| std::env::var_os("ZOPRO_CPU_TREE_OWNER_TRACE").is_some()) {
            eprintln!("TREE_OWNER captured={:?}", owner.captured);
        }
        owner
    }

    fn from_source(enabled: bool, source: impl FnOnce() -> bool) -> Self {
        Self {
            captured: if enabled { Some(source()) } else { None },
        }
    }

    pub(super) fn make<C: CurveConfig>(&self) -> Scratch<C>
    where
        C::Base: BatchInverse,
    {
        match self.captured {
            None => Scratch::default(),
            Some(vector_inverse) => Scratch {
                vector_inverse,
                pairs: Vec::new(),
                groups: Vec::new(),
                prefix_records: Vec::new(),
                denominators: [C::Base::ZERO; LANES],
                products: [Default::default(); LANES],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use zopro_bn254::{G1Config, G2Config};

    #[test]
    fn captures_once_and_next_owner_observes_changes() {
        let value = Cell::new(true);
        let reads = Cell::new(0usize);
        let source = || {
            reads.set(reads.get() + 1);
            value.get()
        };
        let first = Factory::from_source(true, source);
        value.set(false);
        for _ in 0..100 {
            assert!(first.make::<G1Config>().vector_inverse);
            assert!(first.make::<G2Config>().vector_inverse);
        }
        assert_eq!(reads.get(), 1);
        let second = Factory::from_source(true, source);
        assert!(!second.make::<G1Config>().vector_inverse);
        assert_eq!(reads.get(), 2);
        let disabled = Factory::from_source(false, || panic!("disabled capture read"));
        assert_eq!(disabled.captured, None);
    }

    fn curve<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let generator = Projective::<C>::generator();
        for workers in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .stack_size(512 * 1024)
                .build()
                .unwrap();
            pool.install(|| {
                for n in [0usize, 1, 2, 7, 8, 9, 17, 257] {
                    let weights: Vec<_> = (0..n)
                        .map(|i| match i % 7 {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => -Fr::ONE,
                            3 | 4 => Fr::from(37u64),
                            _ => Fr::from((i * 19 + 71) as u64),
                        })
                        .collect();
                    let points = Projective::<C>::batch_to_affine(
                        &weights.iter().map(|s| generator * *s).collect::<Vec<_>>(),
                    );
                    let order: Vec<_> = (0..n as u32).rev().collect();
                    let mut starts = vec![0usize];
                    for size in [0, 1, 3, 7, 17, 64, 165] {
                        starts.push((starts.last().unwrap() + size).min(n));
                    }
                    starts.push(n);
                    let counts: Vec<_> = starts.windows(2).map(|r| r[1] - r[0]).collect();
                    let sums = group_affine(&points, &order, &starts, &counts);
                    let indexed = reduce_indexed_buckets(&points, &order, &starts);
                    for (j, range) in starts.windows(2).enumerate() {
                        let total = order[range[0]..range[1]]
                            .iter()
                            .fold(Fr::ZERO, |s, &i| s + weights[i as usize]);
                        let expected = (generator * total).to_affine();
                        assert_eq!(sums[j], expected);
                        assert_eq!(indexed[j], expected);
                    }
                    let total = weights.iter().enumerate().fold(Fr::ZERO, |s, (i, v)| {
                        s + *v * Fr::from((i + 1) as u64)
                    });
                    // The weighted tree requires a power-of-two bucket count.
                    // Padding contributes zero without changing the weights.
                    let mut buckets = points.clone();
                    buckets.resize(n.max(1).next_power_of_two(), Affine::<C>::IDENTITY);
                    assert_eq!(weighted_affine(&buckets).to_affine(), (generator * total).to_affine());
                }
            });
        }
    }

    #[test]
    fn complete_reductions_threaded_generator_oracle() {
        curve::<G1Config>();
        curve::<G2Config>();
    }
}
