use std::{fs,collections::HashMap};
use zopro_bn254::{Fr,FrConfig,FpConfig};
use serde_json::{json,Value};
use zopro_prover::zopro_core::artifact::Artifact;
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn negative(x:[u64;4])->[u64;4]{if x==[0;4]{return x;}let mut out=[0;4];let mut borrow=false;for i in 0..4{let(t,b0)=FrConfig::P[i].overflowing_sub(x[i]);let(t,b1)=t.overflowing_sub(borrow as u64);out[i]=t;borrow=b0||b1;}assert!(!borrow);out}
fn weight(mut state:u64)->Fr{let mut bytes=[0u8;64];for c in bytes.chunks_mut(8){state^=state<<13;state^=state>>7;state^=state<<17;c.copy_from_slice(&state.to_le_bytes());}Fr::from_random_bytes(&bytes)}
fn stats(values:&[Fr])->Value{
 let mut equal=HashMap::<[u64;4],usize>::new();
 let mut signed=HashMap::<[u64;4],(u8,usize,usize,[Fr;2])>::new();let mut zeros=0usize;let mut lhs=[Fr::ZERO;2];let mut checks=0;
 for (i,s) in values.iter().enumerate(){let x=s.to_canonical();let nx=negative(x);assert_eq!(nx,(-*s).to_canonical());assert_eq!(negative(nx),x);checks+=2;if x==[0;4]{zeros+=1;continue;}
  *equal.entry(x).or_default()+=1;
  let flip=x.iter().rev().cmp(nx.iter().rev()).is_gt();let rep=if flip{nx}else{x};let rf=Fr::from_le_bytes_mod_order(&rep.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());assert_eq!(*s,if flip{-rf}else{rf});checks+=1;
  let v=signed.entry(rep).or_insert((0,0,0,[Fr::ZERO;2]));v.0|=if flip{2}else{1};if flip{v.2+=1;}else{v.1+=1;}
  for k in 0..2{let w=weight((i as u64+1).wrapping_mul([0x9e3779b97f4a7c15u64,0xd6e8feb86659fd93][k]));lhs[k]+=*s*w;v.3[k]+=if flip{-w}else{w};}
 }
 let mut rhs=[Fr::ZERO;2];let mut both=0usize;let mut affected=0usize;
 for (rep,(signs,pos,neg,sum)) in &signed{if *signs==3{both+=1;affected+=pos+neg;}let rf=Fr::from_le_bytes_mod_order(&rep.into_iter().flat_map(|x|x.to_le_bytes()).collect::<Vec<_>>());for k in 0..2{rhs[k]+=rf*sum[k];}}
 assert_eq!(lhs,rhs);assert_eq!(equal.len()-signed.len(),both);
 json!({"positions":values.len(),"zero_positions":zeros,"nonzero_equality_classes":equal.len(),"signed_classes":signed.len(),"extra_class_reductions":both,"opposite_class_original_positions":affected,"unique_class_reduction_fraction":both as f64/equal.len().max(1)as f64,"per_scalar_integer_and_sign_checks":checks,"independent_weighted_field_checks":2,"signed_point_sum_count_model":"Each extra class reduction replaces two effective base/scalar rows by one. Forming the signed base sum costs a point addition before any window savings; this census does not measure that cost."})
}
fn main(){
 let mut synthetic=vec![Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(2u64),-Fr::from(2u64),Fr::ONE];for i in 1..129{let x=weight(i);synthetic.extend([x,-x,x]);}let st=stats(&synthetic);assert_eq!(st["extra_class_reductions"],130);println!("SYNTHETIC {st}");
 let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),2);let cases:Value=serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();let pool=rayon::ThreadPoolBuilder::new().num_threads(32).build().unwrap();pool.install(||{
 for c in cases.as_array().unwrap(){let shape=c["shape"].as_str().unwrap();let bytes=fs::read(c["key"].as_str().unwrap()).unwrap();let request=fs::read_to_string(c["request"].as_str().unwrap()).unwrap();
 let sdk=zopro_zolana::Prover::load(&bytes).unwrap();let witness=sdk.witness(&request).unwrap();let image=sdk.solver_image().unwrap();drop(sdk);let artifact=Artifact::parse(&bytes).unwrap();let key=artifact.key(artifact.points(),Some(&image)).unwrap();let raw=artifact.key(artifact.points(),None).unwrap();let witness:Vec<_>=witness.iter().flat_map(|v|v.0.iter().flat_map(|w|w.to_le_bytes())).collect();let values=key.msm_scalars(&witness).unwrap();let raw_values=raw.msm_scalars(&witness).unwrap();assert_eq!(values,raw_values);drop(raw);drop(key);
 for (query,i) in [("A_G1",0),("B_G2",1),("rawK_diagnostic",2)]{println!("ROW {}",json!({"shape":shape,"query":query,"stats":stats(&values[i]),"raw_and_solver_image_solutions_equal":true}));}
 }
 });println!("COMPLETE ninefreshshape-solves,cached_and_rawsolutions_agree,27queries,54independentfieldfunctionalchecks,noSDKproofs,noarrayexports");
}
