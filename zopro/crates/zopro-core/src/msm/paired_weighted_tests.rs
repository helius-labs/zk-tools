use super::*;
use zopro_bn254::{Fr,G1Config,G2Config};
use crate::testing::Seeded;
fn fixture<C:CurveConfig>(n:usize,kind:usize)->(Vec<Affine<C>>,Vec<Fr>)where C::Base:BatchInverse{
 let mut rng=Seeded::new(0x7177656967687465+kind as u64);
 let coefficients=(0..n).map(|i|match kind{0=>rng.fr(),1=>Fr::ZERO,2=>Fr::ONE,3=>if i%2==0{Fr::ONE}else{-Fr::ONE},4=>if(i/256)%2==0{Fr::ZERO}else{rng.fr()},_=>match i%5{0|1=>Fr::ONE,2=>-Fr::from(2u64),3=>Fr::ZERO,_=>Fr::from((i%127+1)as u64)}}).collect::<Vec<_>>();
 let points=coefficients.iter().map(|&x|(Projective::<C>::generator()*x).to_affine()).collect();(points,coefficients)
}
fn pair_check<C:CurveConfig>()where C::Base:BatchInverse{
 let mut tree=Vec::new();for kind in 0..6{let(points,coefficients)=fixture::<C>(512,kind);let saved=points.clone();for inverse in[false,true]{let mut s=scratch::<C>();s.vector_inverse=inverse;let got=forest::<C>(&points,&mut tree,&mut s);
  for g in 0..2{let mut original=vec![LimbPoint::<C::Base>::infinity();511];for i in 0..256{original[i]=limbs::<C>(points[g*256+i]);}let mut old=Scratch::<C>::default();old.vector_inverse=inverse;let expected=weighted_sum(&mut original,256,&mut old);assert_eq!(got[g].0.to_affine(),expected.to_affine());assert_eq!(got[g].1,affine::<C>(original[0]));
   let part=&coefficients[g*256..(g+1)*256];let total=part.iter().copied().fold(Fr::ZERO,|a,b|a+b);let weighted=part.iter().enumerate().fold(Fr::ZERO,|a,(i,b)|a+*b*Fr::from((i+1)as u64));assert_eq!(got[g].0.to_affine(),(Projective::<C>::generator()*weighted).to_affine());assert_eq!(got[g].1,(Projective::<C>::generator()*total).to_affine());}
  assert!(tree.capacity()<=NODES);assert!(s.pairs.capacity()<=256);assert!(s.groups.capacity()<=if C::Base::TREE_PREFIX{3}else{32});assert!(s.prefix_records.capacity()<=if C::Base::TREE_PREFIX{32}else{0});assert_eq!(points,saved);
 }}
}
#[test]fn paired_weighted_outputs_totals_exceptions_reuse_and_both_inverses(){pair_check::<G1Config>();pair_check::<G2Config>();}
fn whole_check<C:CurveConfig>()where C::Base:BatchInverse{
 let(points,coefficients)=fixture::<C>(2048,5);let before=points.clone();for n in[0usize,1,128,256,512,768,1024,1536,2048]{let got=weighted_affine::<C>(&points[..n]);let expected=coefficients[..n].iter().enumerate().fold(Fr::ZERO,|a,(i,b)|a+*b*Fr::from((i+1)as u64));assert_eq!(got.to_affine(),(Projective::<C>::generator()*expected).to_affine());assert_eq!(try_weighted::<C>(&points[..n]).is_some(),n>=512&&n%512==0);}
 assert_eq!(points,before);assert_eq!(workspace::<C>(usize::MAX),None);assert_eq!(workspace::<C>(usize::MAX&!511),None);
 for power in 9..=20{let n=1usize<<power;assert!(workspace::<C>(n).unwrap()<=workspace_weighted::<C>(n).unwrap());}
}
#[test]fn paired_weighted_complete_boundaries_fallback_overflow_and_budget_compatibility(){whole_check::<G1Config>();whole_check::<G2Config>();}
#[test]fn paired_weighted_actual_workers_small_stacks_and_shared_pool(){
 let(points,weights)=fixture::<G1Config>(1024,0);let expected=(zopro_bn254::G1Projective::generator()*weights.iter().enumerate().fold(Fr::ZERO,|a,(i,b)|a+*b*Fr::from((i+1)as u64))).to_affine();
 let(p2,w2)=fixture::<G2Config>(512,0);let expected2=(zopro_bn254::G2Projective::generator()*w2.iter().enumerate().fold(Fr::ZERO,|a,(i,b)|a+*b*Fr::from((i+1)as u64))).to_affine();
 for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();pool.broadcast(|_|{assert_eq!(try_weighted::<G1Config>(&points).unwrap().to_affine(),expected);assert_eq!(try_weighted::<G2Config>(&p2).unwrap().to_affine(),expected2);});}
}
