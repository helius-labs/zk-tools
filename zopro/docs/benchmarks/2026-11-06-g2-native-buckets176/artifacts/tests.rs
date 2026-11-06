#[cfg(test)]mod native_tests{
 use super::*;
 #[test]fn native_signed_g2_tails_masks_reuse_and_input_immutability(){
  for n in [0,1,7,8,9,15,16,31,32,33,199,200,201,351,352,353,511]{let mut f=G2Caller::new(n,87);let input=f.incoming.clone();for rounds in [1,3,7]{f.reset();f.run(rounds,true,true);f.check(rounds);}assert_eq!(f.incoming,input);assert_eq!(f.state_payload_bytes(),[262144,327680]);}
 }
 #[test]fn native_signed_state_matches_original_for_all_lane_masks(){
  let mut f=G2Caller::new(33,99);for mask in 0..256u32{for(j,v)in f.owned.iter_mut().enumerate(){v.0=(v.0&0x7fffffff)|if mask>>(j%8)&1!=0{1<<31}else{0};}f.reset();f.run(3,false,true);let want=f.buckets.clone();f.reset();f.run(3,true,true);assert_eq!(f.native.points(),want);}
 }
 #[test]fn native_signed_small_stack_parallel_reuse(){
  use rayon::prelude::*;for workers in [1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();pool.install(||(0..workers).into_par_iter().for_each(|i|{let mut f=G2Caller::new(if i%2==0{200}else{352},1000+i as u64);for n in [1,3]{f.reset();f.run(n,true,true);f.check(n);}assert_eq!(f.state_payload_bytes(),[262144,327680]);}));}
 }
}
