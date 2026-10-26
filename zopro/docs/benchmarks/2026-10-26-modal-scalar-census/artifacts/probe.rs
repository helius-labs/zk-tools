use std::{fs,collections::HashMap};
use zopro_bn254::{Fr};
use serde_json::{json,Value};
use zopro_prover::zopro_core::artifact::Artifact;
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn weight(mut state:u64)->Fr{let mut bytes=[0u8;64];for c in bytes.chunks_mut(8){state^=state<<13;state^=state>>7;state^=state<<17;c.copy_from_slice(&state.to_le_bytes());}Fr::from_random_bytes(&bytes)}
fn stats(values:&[Fr])->Value{
 let mut classes=HashMap::<[u64;4],(Fr,usize,[Fr;2])>::new();let mut lhs=[Fr::ZERO;2];let mut totals=[Fr::ZERO;2];
 for(i,&s) in values.iter().enumerate(){let x=s.to_canonical();let e=classes.entry(x).or_insert((s,0,[Fr::ZERO;2]));e.1+=1;for k in 0..2{let w=weight((i as u64+1).wrapping_mul([0x9e3779b97f4a7c15u64,0xd6e8feb86659fd93][k]));lhs[k]+=s*w;totals[k]+=w;e.2[k]+=w;}}
 let zero=classes.get(&[0;4]).map_or(0,|v|v.1);let one=classes.get(&Fr::ONE.to_canonical()).map_or(0,|v|v.1);let minus=classes.get(&(-Fr::ONE).to_canonical()).map_or(0,|v|v.1);
 let mut mode=Fr::ZERO;let mut frequency=zero;for(_, (s,n,_))in &classes{if *n>frequency||*n==frequency&&mode!=Fr::ZERO&&s.to_canonical().iter().rev().cmp(mode.to_canonical().iter().rev()).is_lt(){mode=*s;frequency=*n;}}
 let mut rhs=totals.map(|x|x*mode);let mut transformed=0usize;let mut translated_adds=0usize;let mut original_adds=0usize;let mut visited=0usize;
 for(s,n,sums)in classes.values(){visited+=n;if *s!=Fr::ZERO{original_adds+=n-1;}if *s!=mode{translated_adds+=n-1;transformed+=1;for k in 0..2{rhs[k]+=(*s-mode)*sums[k];}}}
 assert_eq!(visited,values.len());assert_eq!(lhs,rhs);assert!(translated_adds<=original_adds);
 let saved=original_adds-translated_adds;
 if mode!=Fr::ZERO{assert_eq!(saved,frequency-zero-usize::from(zero==0));}else{assert_eq!(saved,0);}
 json!({"positions":values.len(),"zero_occurrences":zero,"one_occurrences":one,"minusone_occurrences":minus,"classes_including_zero_if_present":classes.len(),"mode_kind":if mode==Fr::ZERO{"zero"}else if mode==Fr::ONE{"one"}else if mode==-Fr::ONE{"minusone"}else{"other"},"mode_occurrences":frequency,"baseline_group_formation_additions":original_adds,"translated_group_formation_additions":translated_adds,"modeled_saved_group_formation_additions":saved,"formation_reduction_fraction":saved as f64/original_adds.max(1)as f64,"translated_effective_scalar_classes":transformed,"baseline_effective_scalar_classes":classes.len()-usize::from(zero!=0),"setup_point_additions_if_prepared":values.len().saturating_sub(1),"modeled_setup_amortization_proofs":if saved>0{json!(values.len().saturating_sub(1)as f64/saved as f64)}else{Value::Null},"independent_weighted_field_checks":2,"original_positions_accounted_once":visited,"correction_scalar_multiplication":"Required when mode is nonzero; not timed or modeled as free","persistent_point_count_if_prepared":1})
}
fn main(){
 let tests=[vec![Fr::ZERO;32],vec![Fr::ONE;32],(0..32).map(|i|if i<25{Fr::ONE}else{weight(i)}).collect(),(0..32).map(|i|if i<25{Fr::ZERO}else{weight(i)}).collect()];for(i,v)in tests.iter().enumerate(){let st=stats(v);if i==0||i==3{assert_eq!(st["mode_kind"],"zero");}else{assert_eq!(st["mode_kind"],"one");}println!("SYNTHETIC {}",serde_json::json!({"case":i,"stats":st}));}
 let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),2);let cases:Value=serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();let pool=rayon::ThreadPoolBuilder::new().num_threads(32).build().unwrap();pool.install(||{
 for c in cases.as_array().unwrap(){let shape=c["shape"].as_str().unwrap();let bytes=fs::read(c["key"].as_str().unwrap()).unwrap();let request=fs::read_to_string(c["request"].as_str().unwrap()).unwrap();
 let sdk=zopro_zolana::Prover::load(&bytes).unwrap();let witness=sdk.witness(&request).unwrap();let image=sdk.solver_image().unwrap();drop(sdk);let artifact=Artifact::parse(&bytes).unwrap();let key=artifact.key(artifact.points(),Some(&image)).unwrap();let raw=artifact.key(artifact.points(),None).unwrap();let witness:Vec<_>=witness.iter().flat_map(|v|v.0.iter().flat_map(|w|w.to_le_bytes())).collect();let values=key.msm_scalars(&witness).unwrap();let raw_values=raw.msm_scalars(&witness).unwrap();assert_eq!(values,raw_values);drop(raw);drop(key);
 for (query,i) in [("A_G1",0),("B_G2",1),("rawK_diagnostic",2)]{println!("ROW {}",json!({"shape":shape,"query":query,"stats":stats(&values[i]),"raw_and_solver_image_solutions_equal":true}));}
 }
 });println!("COMPLETE ninefreshshape-solves,cached_and_rawsolutions_agree,27queries,54independenttranslationfunctionalchecks,noSDKproofs,noarrayexports");
}
