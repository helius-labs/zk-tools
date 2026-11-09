//! Proof-local borrowed pending operands for ordinary G2 phi windows.
use super::{batch_size, digit};
use crate::msm::BatchAdd;
use zopro_bn254::{Field, Fq, G2Affine, G2Config, G2Projective};
const BETA: Fq = Fq::from_decimal(
    "21888242871839275220042445260109153167277707414472061641714758635765020556616",
);
#[inline]
fn component(s: &[u64; 4], high: bool, w: usize, bits: usize) -> i16 {
    let i = usize::from(high) * 2;
    let d = digit(&[s[i], s[i + 1] & 0x7fffffffffffffff, 0, 0], w, bits);
    if s[i + 1] >> 63 != 0 {
        -d
    } else {
        d
    }
}
struct Window<'a> {
    affine: Vec<G2Affine>,
    projective: Vec<G2Projective>,
    projective_indices: Vec<u32>,
    projective_buckets: Vec<usize>,
    busy: Vec<u64>,
    batch: Vec<(u32, &'a G2Affine)>,
    queue: Vec<(u32, &'a G2Affine)>,
    packed: BatchAdd<G2Config>,
}
impl<'a> Window<'a> {
    fn new(half: usize, size: usize, packed: BatchAdd<G2Config>) -> Self {
        Self {
            affine: vec![G2Affine::IDENTITY; half],
            projective: Vec::new(),
            projective_indices: vec![u32::MAX; half],
            projective_buckets: Vec::new(),
            busy: vec![0; half.div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            packed,
        }
    }
    #[inline(always)]
    fn busy(&self, b: usize) -> bool {
        self.busy[b / 64] >> (b % 64) & 1 == 1
    }
    fn projective(&mut self, b: usize, p: G2Affine) {
        let i = self.projective_indices[b];
        if i == u32::MAX {
            self.projective_indices[b] = self.projective.len() as u32;
            self.projective_buckets.push(b);
            self.projective.push(p.to_projective());
        } else {
            self.projective[i as usize] += p;
        }
    }
    #[inline(always)]
    fn add(&mut self, encoded: u32, p: &'a G2Affine) {
        let b = (encoded & 0x7fffffff) as usize;
        let neg = encoded >> 31 != 0;
        let a = &mut self.affine[b];
        if a.is_identity() {
            *a = if neg { -*p } else { *p };
            return;
        }
        if a.x == p.x {
            let signed = if neg { -*p } else { *p };
            if a.y == signed.y {
                self.projective(b, signed)
            } else {
                *a = G2Affine::IDENTITY
            }
            return;
        }
        self.busy[b / 64] |= 1 << (b % 64);
        self.batch.push((encoded, p));
    }
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        self.packed
            .apply_signed_borrowed_g2(&mut self.affine, &self.batch);
        for (i, _) in &self.batch {
            self.busy[(i & 0x7fffffff) as usize / 64] = 0;
        }
        self.batch.clear();
    }
    fn take_from_queue(&mut self) {
        while let Some(&(encoded, p)) = self.queue.last() {
            if self.busy((encoded & 0x7fffffff) as usize) {
                return;
            }
            self.queue.pop();
            self.add(encoded, p);
        }
    }
    fn flush_queue(&mut self) {
        for i in 0..self.queue.len() {
            let (encoded, p) = self.queue[i];
            self.projective(
                (encoded & 0x7fffffff) as usize,
                if encoded >> 31 != 0 { -*p } else { *p },
            );
        }
        self.queue.clear();
    }
    fn feed(
        &mut self,
        bases: &'a [G2Affine],
        short: &[[u64; 4]],
        high: bool,
        w: usize,
        bits: usize,
        size: usize,
    ) {
        for (p, s) in bases.iter().zip(short) {
            let d = component(s, high, w, bits);
            if d == 0 || p.is_identity() {
                continue;
            }
            let b = d.unsigned_abs() as usize - 1;
            let encoded = b as u32 | if d < 0 { 1 << 31 } else { 0 };
            if self.busy(b) {
                self.queue.push((encoded, p));
                if self.queue.len() == size - 1 {
                    self.flush_queue();
                }
                continue;
            }
            self.add(encoded, p);
            if self.batch.len() == size {
                self.execute();
                self.take_from_queue();
            }
        }
        self.execute();
        self.flush_queue();
    }
    fn finish(mut self) -> Vec<G2Affine> {
        let combined: Vec<_> = self
            .projective
            .iter()
            .zip(&self.projective_buckets)
            .map(|(p, &i)| *p + self.affine[i])
            .collect();
        for (p, &i) in G2Projective::batch_to_affine(&combined)
            .into_iter()
            .zip(&self.projective_buckets)
        {
            self.affine[i] = p;
        }
        self.affine
    }
}
pub(super) fn try_window(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    w: usize,
    bits: usize,
) -> Option<Vec<G2Affine>> {
    let packed = BatchAdd::default();
    if !packed.borrowed_g2_enabled() {
        return None;
    }
    let size = batch_size(bits).max(32);
    let mut s = Window::new(1 << (bits - 1), size, packed);
    s.feed(bases, short, true, w, bits, size);
    debug_assert!(s.batch.is_empty() && s.queue.is_empty());
    super::affine_phi(&mut s.affine);
    for p in &mut s.projective {
        p.x = p.x.mul_by_fp(&BETA);
    }
    s.feed(bases, short, false, w, bits, size);
    Some(s.finish())
}