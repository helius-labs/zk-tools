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
