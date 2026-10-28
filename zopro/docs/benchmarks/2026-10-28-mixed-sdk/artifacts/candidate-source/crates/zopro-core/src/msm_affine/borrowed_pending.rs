//! Proof-local borrowed operands; the key is never mutated or copied here.
use super::{batch_size, digit, top_window};
use crate::msm::BatchAdd;
use rayon::prelude::*;
use zopro_bn254::{Affine, CurveConfig, Field, Fq, Fr, Projective};

pub(super) fn enabled() -> bool {
    cfg!(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )) && <zopro_bn254::Fq as crate::msm::BatchInverse>::deferred_sign()
        && crate::simd::cpu_option("ZOPRO_CPU_PACKED")
        && crate::simd::cpu_option("ZOPRO_CPU_DIRECT")
}

struct Window<'a, C: CurveConfig<Base = Fq>> {
    affine: Vec<Affine<C>>,
    projective: Vec<Projective<C>>,
    projective_indices: Vec<u32>,
    projective_buckets: Vec<usize>,
    busy: Vec<u64>,
    batch: Vec<(u32, &'a Affine<C>)>,
    queue: Vec<(u32, &'a Affine<C>)>,
    packed: BatchAdd<C>,
    #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    mixed: Vec<crate::msm::mixed_g1::Admitted>,
}
impl<'a, C: CurveConfig<Base = Fq>> Window<'a, C> {
    fn new(half: usize, size: usize) -> Self {
        Self {
            affine: vec![Affine::<C>::IDENTITY; half],
            projective: Vec::new(),
            projective_indices: vec![u32::MAX; half],
            projective_buckets: Vec::new(),
            busy: vec![0; half.div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            packed: BatchAdd::default(),
            #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
            mixed: Vec::new(),
        }
    }
    #[inline(always)]
    fn busy(&self, bucket: usize) -> bool {
        self.busy[bucket / 64] >> (bucket % 64) & 1 == 1
    }
    #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    fn enable_mixed(&mut self,owner:Option<&std::sync::Arc<crate::msm::mixed_g1::Admission>>) {
        if let Some(owner)=owner {if let Some(state)=owner.window() {self.mixed=state;}}
    }
    fn projective(&mut self, bucket: usize, point: Affine<C>) {
        let index = self.projective_indices[bucket];
        if index == u32::MAX {
            self.projective_indices[bucket] = self.projective.len() as u32;
            self.projective_buckets.push(bucket);
            self.projective.push(point.to_projective());
        } else {
            self.projective[index as usize] += point;
        }
    }
    #[inline(always)]
    fn add(&mut self, encoded: u32, point: &'a Affine<C>) {
        let bucket = (encoded & 0x7fffffff) as usize;
        let negative = encoded >> 31 != 0;
        let current = &mut self.affine[bucket];
        if current.is_identity() {
            *current = if negative { -*point } else { *point };
            return;
        }
        if current.x == point.x {
            let signed = if negative { -*point } else { *point };
            if current.y == signed.y {
                self.projective(bucket, signed);
            } else {
                *current = Affine::<C>::IDENTITY;
            }
            return;
        }
        self.busy[bucket / 64] |= 1 << (bucket % 64);
        self.batch.push((encoded, point));
    }
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
        if let Some(state)=self.mixed.first_mut() {
            if state.apply::<C,_>(&mut self.affine,&self.batch,&mut self.queue) {
                for(index,_)in &self.batch {self.busy[(index&0x7fffffff)as usize/64]=0;}
                self.batch.clear();return;
            }
        }
        self.packed
            .apply_signed_borrowed(&mut self.affine, &self.batch);
        for (index, _) in &self.batch {
            self.busy[(index & 0x7fffffff) as usize / 64] = 0;
        }
        self.batch.clear();
    }
    fn take_from_queue(&mut self) {
        while let Some(&(encoded, point)) = self.queue.last() {
            if self.busy((encoded & 0x7fffffff) as usize) {
                return;
            }
            self.queue.pop();
            self.add(encoded, point);
        }
    }
    fn flush_queue(&mut self) {
        for i in 0..self.queue.len() {
            let (encoded, point) = self.queue[i];
            self.projective(
                (encoded & 0x7fffffff) as usize,
                if encoded >> 31 != 0 { -*point } else { *point },
            );
        }
        self.queue.clear();
    }
    fn feed(
        &mut self,
        points: &'a [Affine<C>],
        scalars: &[[u64; 4]],
        w: usize,
        bits: usize,
        size: usize,
    ) {
        for (point, scalar) in points.iter().zip(scalars) {
            let d = digit(scalar, w, bits);
            if d == 0 || point.is_identity() {
                continue;
            }
            let bucket = d.unsigned_abs() as usize - 1;
            let encoded = bucket as u32 | if d < 0 { 1 << 31 } else { 0 };
            if self.busy(bucket) {
                self.queue.push((encoded, point));
                if self.queue.len() == size - 1 {
                    self.flush_queue();
                }
                continue;
            }
            self.add(encoded, point);
            if self.batch.len() == size {
                self.execute();
                self.take_from_queue();
            }
        }
    }
    fn finish(mut self) -> Vec<Affine<C>> {
        self.execute();
        self.flush_queue();
        let combined: Vec<_> = self
            .projective
            .iter()
            .zip(&self.projective_buckets)
            .map(|(p, &i)| *p + self.affine[i])
            .collect();
        for (p, &i) in Projective::<C>::batch_to_affine(&combined)
            .into_iter()
            .zip(&self.projective_buckets)
        {
            self.affine[i] = p;
        }
        self.affine
    }
}

pub(super) fn fused<C: CurveConfig<Base = Fq>>(
    k: &[Affine<C>],
    sk: &[[u64; 4]],
    h: &[Affine<C>],
    sh: &[[u64; 4]],
    bits: usize,
) -> Projective<C> {
    assert_eq!(k.len(), sk.len());
    assert_eq!(h.len(), sh.len());
    assert!((12..=14).contains(&bits));
    assert!(k.len().min(h.len()) >= 1024);
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(bits);
    let size = batch_size(bits).max(32);
    #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    let mixed_admission=crate::msm::mixed_g1::admit(size,windows);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            if w + 1 == windows {
                let (a, b) = rayon::join(
                    || top_window::<C, true>(k, sk, w, bits),
                    || top_window::<C, true>(h, sh, w, bits),
                );
                if let (Some(a), Some(b)) = (a, b) {
                    return a + b;
                }
            }
            let mut state = Window::new(1usize << (bits - 1), size);
            #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
            state.enable_mixed(mixed_admission.as_ref());
            state.feed(k, sk, w, bits, size);
            state.feed(h, sh, w, bits, size);
            crate::msm::weighted_affine(&state.finish())
        })
        .collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..bits {
            result = result.double();
        }
        result += sum;
    }
    result
}

pub(crate) fn ordinary_enabled() -> bool {
    enabled()
}

pub(crate) fn msm<C: CurveConfig<Base = Fq>>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    parts: usize,
    width: usize,
) -> Projective<C> {
    assert_eq!(bases.len(), scalars.len());
    assert!((1024..65536).contains(&bases.len()));
    assert!((1..=2).contains(&parts));
    assert!((10..=14).contains(&width));
    let part = bases.len().div_ceil(parts);
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    let size = batch_size(width).max(32);
    #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    let mixed_admission=crate::msm::mixed_g1::admit(size,windows*parts);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            if let Some(sum) = top_window::<C, true>(bases, scalars, w, width) {
                return sum;
            }
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|p| {
                    let range = p * part..((p + 1) * part).min(bases.len());
                    let mut state = Window::new(1usize << (width - 1), size);
                    #[cfg(all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
                    state.enable_mixed(mixed_admission.as_ref());
                    state.feed(&bases[range.clone()], &scalars[range], w, width, size);
                    state.finish()
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for other in buckets {
                super::merge_buckets(&mut combined, &other);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G1Affine, G1Projective};
    use crate::testing::Seeded;
    #[test]
    fn ordinary_queries_generator_oracle_and_owned_control() {
        let mut rng = Seeded::new(0x626f72726f774131);
        for threads in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for n in [1024, 21006] {
                    let (p, weights) = points(n, 37);
                    for kind in 0..6 {
                        let mut expected = Fr::ZERO;
                        let s: Vec<_> = weights
                            .iter()
                            .enumerate()
                            .map(|(i, w)| {
                                let s = match kind {
                                    0 => Fr::ZERO,
                                    1 => Fr::ONE,
                                    2 => -Fr::ONE,
                                    3 => rng.fr(),
                                    4 => Fr::from((i % 3 + 1) as u64),
                                    _ => {
                                        let bit =
                                            [1, 11, 12, 13, 14, 63, 64, 127, 128, 251, 252, 253]
                                                [i % 12];
                                        let v = Fr::from(2u64).pow(&[bit]);
                                        if i % 2 == 0 {
                                            v - Fr::ONE
                                        } else {
                                            v + Fr::ONE
                                        }
                                    }
                                };
                                expected += s * *w;
                                s.to_canonical()
                            })
                            .collect();
                        let want = (G1Affine::GENERATOR * expected).to_affine();
                        for parts in 1..=2 {
                            for width in 12..=13 {
                                assert_eq!(
                                    msm(&p, &s, parts, width).to_affine(),
                                    want,
                                    "t={threads} n={n} kind={kind} parts={parts} width={width}"
                                );
                                assert_eq!(
                                    super::super::merge_split_width_owned(&p, &s, parts, width)
                                        .to_affine(),
                                    want
                                );
                            }
                        }
                    }
                }
            });
        }
    }
    #[test]
    fn ordinary_trait_dispatch_and_fallback() {
        use crate::msm::BatchInverse;
        use zopro_bn254::{G1Config, G2Affine, G2Config, Fq2};
        let (p, weights) = points(1024, 37);
        let s = vec![Fr::ONE.to_canonical(); p.len()];
        let got = Fq::borrowed_msm(&p, &s, 1, 12);
        if ordinary_enabled() {
            let want = (G1Affine::GENERATOR * weights.iter().fold(Fr::ZERO, |sum, w| sum + *w))
                .to_affine();
            assert_eq!(got.unwrap().to_affine(), want);
        } else {
            assert!(got.is_none());
        }
        for (n, parts, width) in [
            (0, 1, 12),
            (1023, 1, 12),
            (1024, 0, 12),
            (1024, 3, 12),
            (1024, 1, 9),
            (1024, 1, 15),
        ] {
            assert!(Fq::borrowed_msm::<G1Config>(&p[..n], &s[..n], parts, width).is_none());
        }
        assert!(
            Fq2::borrowed_msm::<G2Config>(&vec![G2Affine::IDENTITY; 1024], &s, 1, 12).is_none()
        );
    }
    fn points(n: usize, offset: u64) -> (Vec<G1Affine>, Vec<Fr>) {
        let mut p = G1Affine::GENERATOR * Fr::from(offset);
        let mut raw = Vec::new();
        let mut weights = Vec::new();
        for i in 0..n {
            p += G1Affine::GENERATOR;
            let mut weight = Fr::from(offset + i as u64 + 1);
            let mut q = p;
            if i % 17 == 0 {
                q = G1Projective::IDENTITY;
                weight = Fr::ZERO;
            } else if i % 3 == 0 {
                q = -q;
                weight = -weight;
            } else if i % 11 == 0 {
                q = G1Affine::GENERATOR.to_projective();
                weight = Fr::ONE;
            }
            raw.push(q);
            weights.push(weight);
        }
        (G1Projective::batch_to_affine(&raw), weights)
    }
    #[test]
    fn pending_record_sizes_and_collision_reuse() {
        assert_eq!(core::mem::size_of::<(u32, G1Affine)>(), 72);
        assert_eq!(core::mem::size_of::<(u32, &G1Affine)>(), 16);
        let (p, _) = points(129, 7);
        let mut state = Window::new(32, 8);
        let mut want = vec![G1Projective::IDENTITY; 32];
        for round in 0..3 {
            for (i, point) in p.iter().enumerate() {
                if point.is_identity() {
                    continue;
                }
                let b = if round == 0 { 0 } else { i % 32 };
                let encoded = b as u32 | if i % 2 == 0 { 1 << 31 } else { 0 };
                want[b] += if i % 2 == 0 { -*point } else { *point };
                if state.busy(b) {
                    state.queue.push((encoded, point));
                    if state.queue.len() == 7 {
                        state.flush_queue();
                    }
                } else {
                    state.add(encoded, point);
                    if state.batch.len() == 8 {
                        state.execute();
                        state.take_from_queue();
                    }
                }
            }
            state.execute();
            state.flush_queue();
        }
        assert_eq!(state.finish(), G1Projective::batch_to_affine(&want));
    }
    #[test]
    fn fused_generator_oracle_and_owned_control() {
        let mut rng = Seeded::new(0x626f72726f773236);
        for threads in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for (nk, nh) in [(1024, 1025), (30529, 32768)] {
                    let (k, kw) = points(nk, 7);
                    let (h, hw) = points(nh, 1_000_003);
                    for kind in 0..6 {
                        let mut expected = Fr::ZERO;
                        let mut scalars = |weights: &[Fr]| {
                            weights
                                .iter()
                                .enumerate()
                                .map(|(i, w)| {
                                    let s = match kind {
                                        0 => Fr::ZERO,
                                        1 => Fr::ONE,
                                        2 => -Fr::ONE,
                                        3 => rng.fr(),
                                        4 => Fr::from((i % 3 + 1) as u64),
                                        _ => {
                                            let bit = [
                                                1, 11, 12, 13, 14, 63, 64, 127, 128, 251, 252, 253,
                                            ][i % 12];
                                            let v = Fr::from(2u64).pow(&[bit]);
                                            if i % 2 == 0 {
                                                v - Fr::ONE
                                            } else {
                                                v + Fr::ONE
                                            }
                                        }
                                    };
                                    expected += s * *w;
                                    s.to_canonical()
                                })
                                .collect::<Vec<_>>()
                        };
                        let sk = scalars(&kw);
                        let sh = scalars(&hw);
                        for bits in 12..=14 {
                            let want = (G1Affine::GENERATOR * expected).to_affine();
                            assert_eq!(
                                fused(&k, &sk, &h, &sh, bits).to_affine(),
                                want,
                                "t={threads} n={nk} kind={kind} bits={bits}"
                            );
                            assert_eq!(
                                super::super::fused_kh::owned(&k, &sk, &h, &sh, bits).to_affine(),
                                want
                            );
                        }
                    }
                }
            });
        }
    }
}
