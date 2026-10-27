//! Lossless storage for G1's existing expanded batch values.
use super::FqBatch;

#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
#[derive(Clone, Copy)]
pub(super) struct Stored(zopro_bn254::ifma8::PackedIfma8<zopro_bn254::FqConfig, true>);
#[cfg(not(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
)))]
#[derive(Clone, Copy)]
pub(super) struct Stored(FqBatch);

impl Stored {
    #[inline(always)]
    pub(super) fn new(value: FqBatch) -> Self {
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            Self(value.compact_storage())
        }
        #[cfg(not(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        {
            Self(value)
        }
    }
    #[inline(always)]
    pub(super) fn expand(&self) -> FqBatch {
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            FqBatch::expand_storage(&self.0)
        }
        #[cfg(not(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        {
            self.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simd::BATCH_LANES;
    use zopro_bn254::{Field, Fq};
    #[test]
    fn stored_values_match_scalar_arithmetic_across_lazy_chains() {
        for seed in 0..32u64 {
            let mut expected: [Fq; BATCH_LANES] =
                core::array::from_fn(|lane| match (seed + lane as u64) % 4 {
                    0 => Fq::ZERO,
                    1 => Fq::ONE,
                    2 => -Fq::ONE,
                    _ => -Fq::from(seed * 101 + lane as u64 + 2),
                });
            let mut stored = Stored::new(FqBatch::new(&expected));
            for round in 0..17u64 {
                assert_eq!(stored.expand().get(), expected);
                let factors = core::array::from_fn(|i| Fq::from(seed + round + i as u64 + 2));
                let subtract = core::array::from_fn(|i| -Fq::from(round + i as u64 + 1));
                let value = stored
                    .expand()
                    .mul(&FqBatch::new(&factors))
                    .sub(&FqBatch::new(&subtract));
                for i in 0..BATCH_LANES {
                    expected[i] = expected[i] * factors[i] - subtract[i];
                }
                stored = Stored::new(value);
            }
            assert_eq!(stored.expand().get(), expected);
        }
    }
    #[test]
    fn record_size_and_conservative_forecast_contract() {
        let expanded = core::mem::size_of::<FqBatch>();
        let stored = core::mem::size_of::<Stored>();
        if BATCH_LANES == 8 {
            assert_eq!((expanded, stored), (320, 256));
        } else {
            assert_eq!(stored, expanded);
        }
        assert_eq!(core::mem::size_of::<super::super::FqAddGroup>(), 5 * stored);
        assert_eq!(core::mem::size_of::<[FqBatch; 5]>(), 5 * expanded);
        assert!(
            core::mem::size_of::<[FqBatch; 5]>()
                >= core::mem::size_of::<super::super::FqAddGroup>()
        );
    }
}
