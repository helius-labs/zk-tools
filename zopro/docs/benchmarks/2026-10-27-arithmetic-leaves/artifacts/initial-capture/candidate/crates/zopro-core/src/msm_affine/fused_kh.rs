use rayon::prelude::*;
use zopro_bn254::{Field,Fq,Fr,G1Config,G1Affine,G1Projective};
use super::{Window,batch_size,digit,affine_buckets,top_window};
use crate::msm::BatchInverse;
fn feed(state:&mut Window<G1Config>,points:&[G1Affine],scalars:&[[u64;4]],w:usize,bits:usize,size:usize){
#[cfg(zopro_arithmetic_trace)] let _trace_serial=crate::arithmetic_trace::serial(21,points.len(),w);

 for (base,s) in points.iter().zip(scalars){
  let d=digit(s,w,bits);if d==0||base.is_identity(){continue;}
  let bucket=d.unsigned_abs()as usize-1;
  let p=if state.defer_sign||d>0{*base}else{-*base};
  let encoded=bucket as u32|if state.defer_sign&&d<0{1<<31}else{0};
  if state.busy(bucket){state.queue.push((encoded,p));if state.queue.len()==size-1{state.flush_queue();}continue;}
  state.add_signed(encoded,p);if state.batch.len()==size{state.execute();state.take_from_queue();}
 }
}
// Only a borrowed pair of existing base/scalar slices. One bucket set per window;
// state remains valid across the segment boundary, with pending batches shared.
pub(crate)fn fused(k:&[G1Affine],sk:&[[u64;4]],h:&[G1Affine],sh:&[[u64;4]],bits:usize)->G1Projective{
#[cfg(zopro_arithmetic_trace)] let _trace_kernel=crate::arithmetic_trace::root(2,k.len(),h.len());
#[cfg(zopro_arithmetic_trace)] let trace_kernel_id=_trace_kernel.id();


 assert_eq!(k.len(),sk.len());assert_eq!(h.len(),sh.len());assert!((12..=14).contains(&bits));
 if k.len().min(h.len())<1024{let(a,b)=rayon::join(||super::msm_raw(k,sk),||super::msm_raw(h,sh));return a+b;}
 let windows=(Fr::MODULUS_BITS as usize+1).div_ceil(bits);let size=batch_size(bits).max(32);
 let sums:Vec<_>=(0..windows).into_par_iter().map(|w|{
#[cfg(zopro_arithmetic_trace)] let _trace_window=crate::arithmetic_trace::scope(trace_kernel_id,11,w,0,false);
#[cfg(zopro_arithmetic_trace)] let trace_window_id=_trace_window.id();


  if w+1==windows {
   let(a,b)=rayon::join(||{
#[cfg(zopro_arithmetic_trace)] let _trace_top=crate::arithmetic_trace::scope(trace_window_id,15,0,0,false);
top_window::<G1Config,true>(k,sk,w,bits)
},||{
#[cfg(zopro_arithmetic_trace)] let _trace_top=crate::arithmetic_trace::scope(trace_window_id,15,1,0,false);
top_window::<G1Config,true>(h,sh,w,bits)
});
   if let(Some(a),Some(b))=(a,b){return a+b;}
  }
  let mut state=Window::<G1Config>::new(1usize<<(bits-1),size);state.defer_sign=<Fq as BatchInverse>::deferred_sign();
  feed(&mut state,k,sk,w,bits,size);feed(&mut state,h,sh,w,bits,size);state.execute();state.flush_queue();
  crate::msm::weighted_affine(&affine_buckets(state))
 }).collect();
 let mut result=G1Projective::IDENTITY;for sum in sums.iter().rev(){for _ in 0..bits{result=result.double();}result+=sum;}result
}


// Only internal rerandomized solving consumes K+H together. The public
// five-part API and nonmatching product normalization retain separate sums.
pub(crate) fn eligible(omit_b: bool, scale: Fr, k: usize, h: usize) -> bool {
    omit_b && scale == Fr::ONE && (1024..65536).contains(&k)
        && (1024..65536).contains(&h) && k + h < 65536
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::Seeded;

    #[test]
    fn internal_combination_guard() {
        assert!(eligible(true, Fr::ONE, 30529, 32768));
        for (omit, scale, k, h) in [
            (false, Fr::ONE, 30529, 32768),
            (true, Fr::from(7u64), 30529, 32768),
            (true, Fr::ONE, 1023, 32768),
            (true, Fr::ONE, 30529, 65536),
            (true, Fr::ONE, 32768, 32768),
            (true, Fr::ONE, usize::MAX, usize::MAX),
        ] { assert!(!eligible(omit, scale, k, h)); }
    }

    fn points(n: usize, offset: u64) -> (Vec<G1Affine>, Vec<Fr>) {
        let mut p = G1Affine::GENERATOR * Fr::from(offset);
        let mut raw = Vec::with_capacity(n);
        let mut weights = Vec::with_capacity(n);
        for i in 0..n {
            p += G1Affine::GENERATOR;
            let (mut q, mut weight) = (p, Fr::from(offset + i as u64 + 1));
            if i % 17 == 0 { q = G1Projective::IDENTITY; weight = Fr::ZERO; }
            else if i % 3 == 0 { q = -q; weight = -weight; }
            else if i % 11 == 0 { q = G1Affine::GENERATOR.to_projective(); weight = Fr::ONE; }
            raw.push(q); weights.push(weight);
        }
        (G1Projective::batch_to_affine(&raw), weights)
    }

    #[test]
    fn combined_queries_generator_oracle() {
        let mut rng = Seeded::new(0x6b68_3230_3236);
        for (nk, nh) in [(0,0),(0,7),(7,0),(3,7),(1023,1025),(1024,1025),
                          (8191,8192),(30529,32768)] {
            let (k, kw) = points(nk, 7);
            let (h, hw) = points(nh, 1_000_003);
            for kind in 0..6 {
                let mut expected = Fr::ZERO;
                let mut scalars = |weights: &[Fr], is_h: bool| -> Vec<[u64;4]> {
                    weights.iter().enumerate().map(|(i,w)| {
                        let s = match kind {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => -Fr::ONE,
                            3 => rng.fr(),
                            4 => if !is_h && i%4 != 0 { Fr::from((i*1597%65536) as u64) } else { rng.fr() },
                            _ => { let bit=[1,11,12,13,14,63,64,127,128,251,252,253][i%12];
                                   let v=Fr::from(2u64).pow(&[bit]);
                                   if i%2==0 {v-Fr::ONE} else {v+Fr::ONE} }
                        };
                        expected += s * *w; s.to_canonical()
                    }).collect()
                };
                let sk=scalars(&kw,false); let sh=scalars(&hw,true);
                let want=(G1Affine::GENERATOR*expected).to_affine();
                for bits in 12..=14 {
                    assert_eq!(fused(&k,&sk,&h,&sh,bits).to_affine(),want,
                        "nk={nk} nh={nh} kind={kind} bits={bits}");
                }
            }
        }
    }
}
