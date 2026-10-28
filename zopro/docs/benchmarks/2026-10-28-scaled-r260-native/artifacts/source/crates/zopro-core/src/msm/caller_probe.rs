//! Diagnostic adapters; production stream bodies are unchanged.
use super::*;
use std::hint::black_box;
use zopro_bn254::{G1Config, G2Config};

pub trait Caller {
    fn reset(&mut self);
    fn run(&mut self, rounds: usize);
    fn check(&self, rounds: usize);
    fn bytes(&self) -> Vec<u8>;
    fn capacity_bytes(&self) -> usize;
    fn reference_bytes(&self) -> usize {
        0
    }
}
struct Fixture<C: CurveConfig>
where
    C::Base: BatchInverse,
{
    source: Vec<Affine<C>>,
    buckets: Vec<Affine<C>>,
    batch: Vec<(u32, Affine<C>)>,
    scratch: BatchAdd<C>,
    signed: bool,
    coefficients: Vec<u64>,
    steps: Vec<u64>,
}
fn check<C: CurveConfig>(
    buckets: &[Affine<C>],
    coefficients: &[u64],
    steps: &[u64],
    batch: &[(u32, Affine<C>)],
    signed: bool,
    rounds: usize,
) {
    let mut scalars: Vec<_> = coefficients.iter().map(|c| Fr::from(*c)).collect();
    for (i, (index, _)) in batch.iter().enumerate() {
        let sign = if signed && index >> 31 != 0 {
            -Fr::ONE
        } else {
            Fr::ONE
        };
        scalars[(index & 0x7fffffff) as usize] +=
            sign * Fr::from(steps[i]) * Fr::from(rounds as u64);
    }
    let g = Projective::<C>::generator();
    let expected =
        Projective::<C>::batch_to_affine(&scalars.iter().map(|s| g * *s).collect::<Vec<_>>());
    assert_eq!(buckets, expected);
}
impl<C: CurveConfig> Fixture<C>
where
    C::Base: BatchInverse,
{
    fn new(n: usize, signed: bool, seed: u64) -> Self {
        assert!(n <= 512);
        let size = n.max(1).next_power_of_two();
        let g = Projective::<C>::generator();
        let coefficients: Vec<_> = (0..size)
            .map(|i| (1u64 << 48) * (i as u64 + 1) + (seed + 1) * 100000003)
            .collect();
        let source = Projective::<C>::batch_to_affine(
            &coefficients
                .iter()
                .map(|c| g * Fr::from(*c))
                .collect::<Vec<_>>(),
        );
        let steps: Vec<_> = (0..n)
            .map(|i| 11111 + (seed + 1) * 1337 + i as u64 * 101)
            .collect();
        let incoming = Projective::<C>::batch_to_affine(
            &steps.iter().map(|c| g * Fr::from(*c)).collect::<Vec<_>>(),
        );
        let batch = incoming
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let flag = if signed && i % 3 != 1 { 1u32 << 31 } else { 0 };
                ((((i * 37 + seed as usize) % size) as u32) | flag, p)
            })
            .collect();
        Self {
            buckets: source.clone(),
            source,
            batch,
            scratch: BatchAdd::default(),
            signed,
            coefficients,
            steps,
        }
    }
    fn check(&self, n: usize) {
        check(
            &self.buckets,
            &self.coefficients,
            &self.steps,
            &self.batch,
            self.signed,
            n,
        )
    }
}
pub struct CallerG1(Fixture<G1Config>);
impl CallerG1 {
    pub fn new(n: usize, signed: bool, seed: u64) -> Self {
        let s = Fixture::new(n, signed, seed);
        assert!(
            s.scratch.direct
                && s.scratch.fq.canonical
                && s.scratch.fq.vector_inverse
                && s.scratch.fq.write_groups
        );
        Self(s)
    }
    pub fn with_borrowed<R>(&mut self, action: impl FnOnce(&mut BorrowedG1<'_>) -> R) -> R {
        let f = &mut self.0;
        let borrowed = f.batch.iter().map(|(i, p)| (*i, p)).collect();
        let mut caller = BorrowedG1 {
            source: &f.source,
            buckets: &mut f.buckets,
            batch: borrowed,
            original: &f.batch,
            scratch: &mut f.scratch,
            coefficients: &f.coefficients,
            steps: &f.steps,
            signed: f.signed,
        };
        action(&mut caller)
    }
}
impl Caller for CallerG1 {
    fn reset(&mut self) {
        self.0.buckets.copy_from_slice(&self.0.source)
    }
    fn run(&mut self, n: usize) {
        for _ in 0..n {
            let s = black_box(&mut self.0);
            if s.signed {
                s.scratch.apply_signed(&mut s.buckets, &s.batch)
            } else {
                s.scratch.apply(&mut s.buckets, &s.batch)
            }
        }
    }
    fn check(&self, n: usize) {
        self.0.check(n)
    }
    fn bytes(&self) -> Vec<u8> {
        format!("{:?}", self.0.buckets).into_bytes()
    }
    fn capacity_bytes(&self) -> usize {
        self.0.scratch.fq.groups.capacity() * std::mem::size_of::<FqAddGroup>()
            + self.0.scratch.fq.tail.capacity() * std::mem::size_of::<[Fq; 5]>()
    }
}
pub struct BorrowedG1<'a> {
    source: &'a [Affine<G1Config>],
    buckets: &'a mut [Affine<G1Config>],
    batch: Vec<(u32, &'a Affine<G1Config>)>,
    original: &'a [(u32, Affine<G1Config>)],
    scratch: &'a mut BatchAdd<G1Config>,
    coefficients: &'a [u64],
    steps: &'a [u64],
    signed: bool,
}
impl Caller for BorrowedG1<'_> {
    fn reset(&mut self) {
        self.buckets.copy_from_slice(self.source)
    }
    fn run(&mut self, n: usize) {
        for _ in 0..n {
            let s = black_box(&mut *self);
            s.scratch.apply_signed_borrowed(s.buckets, &s.batch)
        }
    }
    fn check(&self, n: usize) {
        check(
            self.buckets,
            self.coefficients,
            self.steps,
            self.original,
            self.signed,
            n,
        )
    }
    fn bytes(&self) -> Vec<u8> {
        format!("{:?}", self.buckets).into_bytes()
    }
    fn capacity_bytes(&self) -> usize {
        self.scratch.fq.groups.capacity() * std::mem::size_of::<FqAddGroup>()
            + self.scratch.fq.tail.capacity() * std::mem::size_of::<[Fq; 5]>()
    }
    fn reference_bytes(&self) -> usize {
        self.batch.capacity() * std::mem::size_of::<(u32, &Affine<G1Config>)>()
    }
}
pub struct CallerG2(Fixture<G2Config>);
impl CallerG2 {
    pub fn new(n: usize, seed: u64) -> Self {
        let s = Fixture::new(n, false, seed);
        assert!(
            s.scratch.direct
                && s.scratch.fq2.canonical
                && s.scratch.fq2.vector_inverse
                && s.scratch.fq2.prefix_enabled
        );
        Self(s)
    }
}
impl Caller for CallerG2 {
    fn reset(&mut self) {
        self.0.buckets.copy_from_slice(&self.0.source)
    }
    fn run(&mut self, n: usize) {
        for _ in 0..n {
            let s = black_box(&mut self.0);
            s.scratch.apply(&mut s.buckets, &s.batch)
        }
    }
    fn check(&self, n: usize) {
        self.0.check(n)
    }
    fn bytes(&self) -> Vec<u8> {
        format!("{:?}", self.0.buckets).into_bytes()
    }
    fn capacity_bytes(&self) -> usize {
        self.0.scratch.fq2.prefix_records.capacity() * std::mem::size_of::<[FqBatch; 2]>()
            + self.0.scratch.fq2.tail.capacity() * std::mem::size_of::<[Fq2; 5]>()
            + {#[cfg(all(zopro_g2_scaled_r260,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] {self.0.scratch.fq2.scaled_prefix_records.capacity()*std::mem::size_of::<g2_scaled::Record>()} #[cfg(not(all(zopro_g2_scaled_r260,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma))))] {0}}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const LENGTHS: [usize; 13] = [0, 1, 7, 8, 9, 31, 32, 128, 199, 200, 351, 352, 512];
    #[test]
    fn owned_g1_generator_oracle_tails_and_reuse() {
        for signed in [false, true] {
            for n in LENGTHS {
                let mut f = CallerG1::new(n, signed, 19);
                for rounds in [1, 3, 17] {
                    f.reset();
                    f.run(rounds);
                    f.check(rounds);
                }
            }
        }
    }
    #[test]
    fn borrowed_signed_g1_generator_oracle_tails_and_reuse() {
        for n in LENGTHS {
            let mut f = CallerG1::new(n, true, 29);
            f.with_borrowed(|f| {
                for rounds in [1, 3, 17] {
                    f.reset();
                    f.run(rounds);
                    f.check(rounds);
                }
                assert_eq!(
                    f.reference_bytes(),
                    n * std::mem::size_of::<(u32, &Affine<G1Config>)>()
                );
            });
        }
    }
    #[test]
    fn owned_g2_prefix_generator_oracle_tails_and_reuse() {
        for n in LENGTHS {
            let mut f = CallerG2::new(n, 23);
            for rounds in [1, 3, 17] {
                f.reset();
                f.run(rounds);
                f.check(rounds);
            }
        }
    }
}
