//! Keep signed references to key points until the scalar batch uses them.
use super::{BatchInverse, Window, apply_scalar_batch};
use zopro_bn254::{Affine, CurveConfig};

pub(super) fn enabled() -> bool {
    std::env::var("ZOPRO_APPLE_BORROW_BATCH").as_deref() != Ok("0")
        && !crate::simd::cpu_option("ZOPRO_CPU_PACKED")
        && !crate::simd::cpu_option("ZOPRO_CPU_MASKED_SIGN")
}

pub(super) struct Borrowed<'a, C: CurveConfig>
where
    C::Base: BatchInverse,
{
    pub(super) inner: Window<C>,
    batch: Vec<(u32, &'a Affine<C>)>,
    queue: Vec<(u32, &'a Affine<C>)>,
    size: usize,
}

impl<'a, C: CurveConfig> Borrowed<'a, C>
where
    C::Base: BatchInverse,
{
    pub(super) fn new(width: usize, size: usize) -> Self {
        let mut inner = Window::new(1usize << (width - 1), 0);
        inner.denominators.reserve_exact(size);
        inner.prefixes.reserve_exact(size);
        Self {
            inner,
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            size,
        }
    }

    #[inline(always)]
    fn add(&mut self, encoded: u32, point: &'a Affine<C>) {
        if self.inner.stage_pending(encoded, point) {
            self.batch.push((encoded, point));
        }
    }

    fn execute(&mut self, loose: bool) {
        if self.batch.is_empty() {
            return;
        }
        let state = &mut self.inner;
        if loose && C::Base::loose_batch(
            &mut state.affine,
            &mut state.busy,
            &self.batch,
            &mut state.denominators,
            &mut state.prefixes,
        ) {
            self.batch.clear();
            return;
        }
        apply_scalar_batch::<C, _, true>(
            &mut state.affine,
            &mut state.busy,
            &self.batch,
            &mut state.denominators,
            &mut state.prefixes,
        );
        self.batch.clear();
    }

    fn flush(&mut self) {
        for &(encoded, point) in &self.queue {
            self.inner.add_projective(
                (encoded & 0x7fff_ffff) as usize,
                if encoded >> 31 != 0 { -*point } else { *point },
            );
        }
        self.queue.clear();
    }

    pub(super) fn feed(&mut self, bases: &'a [Affine<C>], digits: impl Iterator<Item = i16>) {
        self.feed_pairs(bases.iter().zip(digits), bases.len() < (1 << 16));
    }

    pub(super) fn feed_pairs(
        &mut self, pairs: impl Iterator<Item = (&'a Affine<C>, i16)>, loose: bool,
    ) {
        for (point, d) in pairs {
            if d == 0 || point.is_identity() {
                continue;
            }
            let bucket = d.unsigned_abs() as usize - 1;
            let encoded = bucket as u32 | (u32::from(d < 0) << 31);
            if self.inner.busy(bucket) {
                self.queue.push((encoded, point));
                if self.queue.len() == self.size - 1 {
                    self.flush();
                }
                continue;
            }
            self.add(encoded, point);
            if self.batch.len() == self.size {
                self.execute(loose);
                while let Some(&(encoded, point)) = self.queue.last() {
                    if self.inner.busy((encoded & 0x7fff_ffff) as usize) {
                        break;
                    }
                    self.queue.pop();
                    self.add(encoded, point);
                }
            }
        }
        self.execute(loose);
        self.flush();
    }
}
