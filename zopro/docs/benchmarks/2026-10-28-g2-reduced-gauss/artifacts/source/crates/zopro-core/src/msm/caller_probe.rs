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

#[cfg(test)]
mod reduced_gauss_tests {
    use super::*;
    #[test]
    fn complex_outputs_and_conjugate_scaling_match_scalar_fields() {
        let mut rng = crate::testing::Seeded::new(0x243f_6a88_85a3_08d3);
        let mut fq = || {
            let mut bytes = [0u8; 64];
            for c in bytes.chunks_mut(8) {
                c.copy_from_slice(&rng.next_u64().to_le_bytes());
            }
            Fq::from_random_bytes(&bytes)
        };
        for round in 0..512 {
            let corners = [
                Fq::ZERO,
                Fq::ONE,
                -Fq::ONE,
                Fq::from(2u64),
                -Fq::from(2u64),
                Fq::from(1u64 << 51),
                Fq::from(1u64 << 52),
                Fq::from(u64::MAX),
            ];
            let a: [Fq2; LANES] = core::array::from_fn(|i| {
                if round < 64 {
                    Fq2::new(corners[(round + i) % 8], corners[(round / 8 + i) % 8])
                } else {
                    Fq2::new(fq(), fq())
                }
            });
            let b: [Fq2; LANES] = core::array::from_fn(|i| {
                if round < 64 {
                    Fq2::new(
                        corners[(round / 8 + 3 * i) % 8],
                        corners[(round + 5 * i) % 8],
                    )
                } else {
                    Fq2::new(fq(), fq())
                }
            });
            let factor: [Fq; LANES] = core::array::from_fn(|i| corners[(round + i) % 8]);
            let mut pa = [
                FqBatch::new(&a.map(|v| v.c0)),
                FqBatch::new(&a.map(|v| v.c1)),
            ];
            let pb = [
                FqBatch::new(&b.map(|v| v.c0)),
                FqBatch::new(&b.map(|v| v.c1)),
            ];
            let f = FqBatch::new(&factor);
            let mut scalar = a;
            for _ in 0..8 {
                let product = Fq2::lanes_mul(&pa, &pb);
                let conjugate = Fq2::lanes_scaled_numerator(&pa, &pb, &f);
                let (p0, p1, c0, c1) = (
                    product[0].get(),
                    product[1].get(),
                    conjugate[0].get(),
                    conjugate[1].get(),
                );
                for i in 0..LANES {
                    assert_eq!(Fq2::new(p0[i], p1[i]), scalar[i] * b[i]);
                    assert_eq!(
                        Fq2::new(c0[i], c1[i]),
                        (scalar[i] * b[i].conjugate()).mul_by_fp(&factor[i])
                    );
                }
                // Existing lazy add/sub paths feed changing reduced states.
                pa = Fq2::lanes_add(&product, &Fq2::lanes_sub(&conjugate, &pb));
                scalar = core::array::from_fn(|i| {
                    scalar[i] * b[i] + (scalar[i] * b[i].conjugate()).mul_by_fp(&factor[i]) - b[i]
                });
            }
        }
    }
}
