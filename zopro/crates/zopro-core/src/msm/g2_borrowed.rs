//! Signed borrowed G2 prefix, retaining accepted arithmetic and scratch.
use super::*;
pub(super) fn apply<C: CurveConfig<Base = Fq2>, const FAST: bool>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, &Affine<C>)],
    s: &mut Fq2AddScratch,
) {
    let full = batch.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.prefix_records.clear();
    s.prefix_records.reserve(full / LANES);
    for k in (0..full).step_by(LANES) {
        let ax = fq2_pack(core::array::from_fn(|i| {
            buckets[(batch[k + i].0 & 0x7fffffff) as usize].x
        }));
        let bx = fq2_pack(core::array::from_fn(|i| batch[k + i].1.x));
        let denominator = core::array::from_fn(|i| canonical_sub::<FAST>(&bx[i], &ax[i]));
        let norm = Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains, norm]);
        chains = chains.mul(&norm);
    }
    s.tail.clear();
    let mut inverse =
        if let Some(value) = direct_chain_inverse(&chains, full == batch.len(), s.vector_inverse) {
            value
        } else {
            let mut tail = chains.get();
            for k in full..batch.len() {
                let (encoded, b) = batch[k];
                let index = encoded & 0x7fffffff;
                let b = if encoded >> 31 != 0 { -*b } else { *b };
                let a = buckets[index as usize];
                let denominator = b.x - a.x;
                let norm = denominator.c0.square() + denominator.c1.square();
                let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
                tail[k % LANES] *= norm;
                s.tail
                    .push([a.x, a.y, a.x + b.x, Fq2::new(norm, Fq::ZERO), numerator]);
            }
            let mut inverse = invert_lanes(tail, s.vector_inverse);
            for k in (full..batch.len()).rev() {
                let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
                let slope = numerator.mul_by_fp(&inverse[k % LANES]);
                inverse[k % LANES] *= norm.c0;
                let x = slope.square() - xsum;
                let y = slope * (ax - x) - ay;
                buckets[(batch[k].0 & 0x7fffffff) as usize] = Affine::new(x, y);
            }
            FqBatch::new(&inverse)
        };
    for j in (0..full / LANES).rev() {
        let k = j * LANES;
        let a: [Affine<C>; LANES] =
            core::array::from_fn(|i| buckets[(batch[k + i].0 & 0x7fffffff) as usize]);
        let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = fq2_pack(a.map(|p| p.x));
        let ay = fq2_pack(a.map(|p| p.y));
        let bx = fq2_pack(b.map(|p| p.x));
        let dy=if FAST{
            let by=fq2_pack(b.map(|p|p.y));
            let negative=signs::<true,C>(&batch[k..k+LANES]);
            core::array::from_fn(|i|by[i].signed_difference_canonical(&ay[i],negative))
        }else{
            let by=fq2_pack(core::array::from_fn(|i|if batch[k+i].0>>31!=0{-b[i].y}else{b[i].y}));
            core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]))
        };
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let factor = s.prefix_records[j][0].mul(&inverse);
        inverse = inverse.mul(&s.prefix_records[j][1]);
        let slope = Fq2::lanes_scaled_numerator(&dy, &denominator, &factor);
        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &Fq2::lanes_add(&ax, &bx));
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);
        let (x, y) = (fq2_unpack(x), fq2_unpack(y));
        for i in 0..LANES {
            buckets[(batch[k + i].0 & 0x7fffffff) as usize] = Affine::new(x[i], y[i]);
        }
    }
}

impl<C: CurveConfig<Base = Fq2>> BatchAdd<C> {
    pub(crate) fn borrowed_g2_enabled(&self) -> bool {
        self.direct
            && self.fq2.canonical
            && self.fq2.vector_inverse
            && self.fq2.prefix_enabled
            && crate::simd::cpu_option("ZOPRO_CPU_PACKED")
    }
    pub(crate) fn apply_signed_borrowed_g2(
        &mut self,
        buckets: &mut [Affine<C>],
        batch: &[(u32, &Affine<C>)],
    ) {
        if self.direct && self.fq2.canonical && self.fq2.vector_inverse && self.fq2.prefix_enabled {
            return apply::<C, true>(buckets, batch, &mut self.fq2);
        }
        let owned: Vec<_> = batch
            .iter()
            .map(|&(id, p)| (id & 0x7fffffff, if id >> 31 != 0 { -*p } else { *p }))
            .collect();
        self.apply(buckets, &owned);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::G2Config;
    #[test]
    fn signed_borrowed_prefix_tails_and_reuse() {
        let g = Projective::<G2Config>::generator();
        for n in [0usize, 1, 7, 8, 9, 15, 16, 32, 200, 352, 511] {
            let size = n.max(1).next_power_of_two();
            let initial = Projective::<G2Config>::batch_to_affine(
                &(0..size)
                    .map(|i| g * Fr::from((i + 12345) as u64))
                    .collect::<Vec<_>>(),
            );
            let incoming = Projective::<G2Config>::batch_to_affine(
                &(0..n)
                    .map(|i| g * Fr::from((i + 900001) as u64))
                    .collect::<Vec<_>>(),
            );
            for sign_mode in 0..3 {
                let batch: Vec<_> = incoming
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        (
                            ((i * 37 % size) as u32)
                                | if sign_mode == 1 || sign_mode == 2 && i % 3 == 0 {
                                    1 << 31
                                } else {
                                    0
                                },
                            p,
                        )
                    })
                    .collect();
                let mut actual = initial.clone();
                let mut expected = initial.clone();
                let mut s = Fq2AddScratch {
                    canonical: true,
                    vector_inverse: true,
                    ..Default::default()
                };
                for _ in 0..3 {
                    for &(id, p) in &batch {
                        let i = (id & 0x7fffffff) as usize;
                        let b = if id >> 31 != 0 { -*p } else { *p };
                        assert_ne!(expected[i].x, b.x);
                        expected[i] = (expected[i] + b).to_affine();
                    }
                    apply::<G2Config, true>(&mut actual, &batch, &mut s);
                    assert_eq!(actual, expected);
                }
            }
        }
    }
}

#[cfg(test)]mod vector_sign_tests{
 use super::*;use zopro_bn254::{G2Config,Fr};
 #[test]fn all_masks_tails_reuse_and_small_stack_pools(){
  let g=Projective::<G2Config>::generator();let weights=(1..=64).map(|i|Fr::from(i+70000)).collect::<Vec<_>>();let initial=Projective::<G2Config>::batch_to_affine(&weights.iter().map(|s|g**s).collect::<Vec<_>>());let incoming=Projective::<G2Config>::batch_to_affine(&(0..64).map(|i|g*Fr::from(i+900001)).collect::<Vec<_>>());let original=incoming.clone();
  let reference=Projective::<G2Config>::batch_to_affine(&(0..7).flat_map(|step|(0..64).map(move|i|{let initial=Fr::from(i as u64+70001);let delta=Fr::from(i as u64+900001)*Fr::from(if step<=3{step as u64}else{step as u64-3});g*(if step<=3{initial+delta}else{initial-delta})})).collect::<Vec<_>>());
  for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();pool.install(||{use rayon::prelude::*;(0..256usize).into_par_iter().for_each(|mask|{for n in[0,1,7,8,9,15,16,31,32,33,63,64]{let mut buckets=initial.clone();let batch=(0..n).map(|i|(i as u32|if mask>>(i%8)&1!=0{1<<31}else{0},&incoming[i])).collect::<Vec<_>>();let mut scratch=Fq2AddScratch{canonical:true,vector_inverse:true,..Default::default()};for pass in 1..=3{apply::<G2Config,true>(&mut buckets,&batch,&mut scratch);for i in 0..64{let step=if i>=n{0}else if mask>>(i%8)&1!=0{pass+3}else{pass};assert_eq!(buckets[i],reference[step*64+i]);}}}});});}
  assert_eq!(incoming,original);
 }
}
