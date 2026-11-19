//! Benchmark-only intervention for a single immutable key and one generated witness.
use crate::{Fr, G1Projective, G2Projective};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Default)]
pub(crate) struct Probe {
    settings: OnceLock<u8>,
    witness: OnceLock<Box<[Fr]>>,
    pub(crate) a: Saved<G1Projective>,
    pub(crate) kh: Saved<G1Projective>,
    pub(crate) b2: Saved<G2Projective>,
}

impl Probe {
    pub(crate) fn enter(&self, witness: &[Fr]) -> Result<u8, String> {
        let mask = *self.settings.get_or_init(|| {
            std::env::var("ZOPRO_EXPERIMENT_580_MASK").unwrap().parse().unwrap()
        });
        if mask > 7 {
            return Err("invalid intervention mask".into());
        }
        if self.witness.get_or_init(|| witness.into()).as_ref() != witness {
            return Err("580 requires one identical generated witness per process".into());
        }
        Ok(mask)
    }
}

pub(crate) struct Saved<T> {
    point: OnceLock<T>,
    evaluations: AtomicUsize,
    reuses: AtomicUsize,
}

impl<T> Default for Saved<T> {
    fn default() -> Self {
        Self { point: OnceLock::new(), evaluations: AtomicUsize::new(0), reuses: AtomicUsize::new(0) }
    }
}

impl<T: Copy> Saved<T> {
    pub(crate) fn measure(&self, reuse: bool, calculate: impl FnOnce() -> T) -> T {
        if reuse {
            if let Some(point) = self.point.get() {
                self.reuses.fetch_add(1, Ordering::Relaxed);
                return *point;
            }
        }
        self.evaluations.fetch_add(1, Ordering::Relaxed);
        let point = calculate();
        if reuse {
            assert!(self.point.set(point).is_ok(), "benchmark requires serial SDK requests");
        }
        point
    }

    fn counts(&self) -> [usize; 2] {
        [self.evaluations.load(Ordering::Relaxed), self.reuses.load(Ordering::Relaxed)]
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        if let Some(mask) = self.settings.get() {
            eprintln!("ABLATION {mask} {:?} {:?} {:?}", self.a.counts(), self.kh.counts(), self.b2.counts());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interventions_bind_the_witness_and_count_every_branch() {
        let probe = Probe::default();
        assert!(probe.settings.set(7).is_ok());
        assert_eq!(probe.enter(&[Fr::ONE]).unwrap(), 7);
        assert!(probe.enter(&[Fr::ZERO]).is_err());
        let saved = Saved::default();
        assert_eq!(saved.measure(false, || 2), 2);
        assert_eq!(saved.measure(true, || 3), 3);
        assert_eq!(saved.measure(true, || panic!("unexpected recomputation")), 3);
        assert_eq!(saved.counts(), [2,1]);
    }
}
