use super::*;
use std::hint::black_box;
use zopro_bn254::{G1Config, G1Projective};

pub struct G1StreamProbe {
    source: Vec<Affine<G1Config>>,
    buckets: Vec<Affine<G1Config>>,
    batch: Vec<(u32, Affine<G1Config>)>,
    signed: bool,
    scratch: FqAddScratch,
    expected: Vec<Affine<G1Config>>,
    expected_rounds: usize,
}
impl G1StreamProbe {
    pub fn new(batch_len: usize, signed: bool, seed: u64) -> Self {
        assert!(batch_len <= 512);
        let n = batch_len.max(1).next_power_of_two();
        let generator = G1Projective::generator();
        let raw: Vec<_> = (0..n)
            .map(|i| generator * Fr::from((1u64 << 40) * (i as u64 + 1) + (seed + 1) * 100_000_003))
            .collect();
        let source = G1Projective::batch_to_affine(&raw);
        let inputs: Vec<_> = (0..batch_len)
            .map(|i| generator * Fr::from(11_111 + (seed + 1) * 1337 + i as u64 * 101))
            .collect();
        let inputs = G1Projective::batch_to_affine(&inputs);
        let batch: Vec<_> = inputs
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let index = (i * 37 + seed as usize) % n;
                let flag = if signed && i % 3 != 1 { 1u32 << 31 } else { 0 };
                ((index as u32) | flag, p)
            })
            .collect();
        let mut seen = vec![false; n];
        for &(index, _) in &batch {
            let j = (index & 0x7fff_ffff) as usize;
            assert!(!seen[j]);
            seen[j] = true;
        }
        let scratch = BatchAdd::<G1Config>::default().fq;
        Self {
            buckets: source.clone(),
            source,
            expected: Vec::new(),
            expected_rounds: 0,
            batch,
            signed,
            scratch,
        }
    }
    pub fn reset(&mut self) {
        self.buckets.copy_from_slice(&self.source);
    }
    pub fn set_expected(&mut self, rounds: usize) {
        let mut raw: Vec<_> = self.source.iter().map(|p| p.to_projective()).collect();
        let multiplier = Fr::from(rounds as u64);
        for &(index, p) in &self.batch {
            let p = if self.signed && index >> 31 != 0 {
                -p
            } else {
                p
            };
            let j = (index & 0x7fff_ffff) as usize;
            raw[j] += p * multiplier;
        }
        self.expected = G1Projective::batch_to_affine(&raw);
        self.expected_rounds = rounds;
    }
    pub fn run(&mut self, mode: u8, rounds: usize) {
        match (self.scratch.canonical, self.signed, mode) {
            (true, true, 0 | 1) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_original_impl::<G1Config, true, true>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (true, false, 0 | 1) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_original_impl::<G1Config, true, false>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (false, true, 0 | 1) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_original_impl::<G1Config, false, true>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (false, false, 0 | 1) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_original_impl::<G1Config, false, false>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (true, true, 2) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_impl::<G1Config, true, true>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (true, false, 2) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_impl::<G1Config, true, false>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (false, true, 2) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_impl::<G1Config, false, true>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            (false, false, 2) => {
                for _ in 0..rounds {
                    let p = black_box(&mut *self);
                    stream_g1_impl::<G1Config, false, false>(
                        &mut p.buckets,
                        &p.batch,
                        &mut p.scratch,
                    );
                }
            }
            _ => panic!("unknown mode"),
        }
    }
    pub fn check(&self, rounds: usize) {
        assert_eq!(self.expected_rounds, rounds);
        assert_eq!(self.buckets, self.expected);
    }
    pub fn settings(&self) -> [bool; 4] {
        [
            self.scratch.vector_inverse,
            self.scratch.canonical,
            self.scratch.reserve_groups,
            self.scratch.write_groups,
        ]
    }
    pub fn record_capacity_bytes(&self) -> usize {
        self.scratch.groups.capacity() * core::mem::size_of::<FqAddGroup>()
            + self.scratch.tail.capacity() * core::mem::size_of::<[Fq; 5]>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overlap_matches_independent_complete_curve_oracle() {
        for n in (0..=3 * LANES + 3).chain([199, 200, 203, 352, 355]) {
            for signed in [false, true] {
                for canonical in [false, true] {
                    for vector_inverse in [false, true] {
                        for storage in 0..3 {
                            let mut p = G1StreamProbe::new(n, signed, 3);
                            p.scratch.canonical = canonical;
                            p.scratch.vector_inverse = vector_inverse;
                            p.scratch.reserve_groups = storage > 0;
                            p.scratch.write_groups = storage == 2;
                            p.set_expected(3);
                            for mode in [0, 2, 1, 2] {
                                p.reset();
                                p.run(mode, 3);
                                p.check(3);
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn overlap_reuses_scratch_through_growing_and_shrinking_batches() {
        for signed in [false, true] {
            let mut p = G1StreamProbe::new(355, signed, 7);
            let original = p.batch.clone();
            let ptr = p.buckets.as_ptr();
            for n in [
                0, 1, 7, 8, 9, 15, 16, 17, 24, 25, 200, 203, 352, 355, 203, 24, 17, 8, 1, 0,
            ] {
                p.batch.clear();
                p.batch.extend_from_slice(&original[..n]);
                p.set_expected(5);
                for mode in [2, 0, 2, 1] {
                    p.reset();
                    p.run(mode, 5);
                    p.check(5);
                    assert_eq!(p.buckets.as_ptr(), ptr);
                }
            }
        }
    }
}
