#[path="split.rs"]mod split;
use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{fs,path::Path,collections::BTreeMap};
use zopro_bn254::Fr;
const R:&str="/home/ubuntu/src/zopro-perf-work/component-alias-count176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn normalize(w:[u64;4])->[u64;4]{
 let mut out=[0;4];let mut borrow=false;
 for i in 0..4{let(a,b)=Fr::MODULUS[i].overflowing_sub(w[i]);let(c,d)=a.overflowing_sub(u64::from(borrow));out[i]=c;borrow=b||d;}
 assert!(!borrow);if out.iter().rev().cmp(w.iter().rev()).is_lt(){out}else{w}
}
fn weight(m:u128)->usize{
 let bit=|i:usize|if i<128{((m>>i)&1)as i32}else{0};
 (0..11).filter(|&j|{let v=if j==0{(0..12).map(|k|bit(k)<<k).sum::<i32>()}else{(0..13).map(|k|bit(j*12-1+k)<<k).sum::<i32>()};let d=if j==0{v-(bit(11)<<12)}else{(v+1)/2-(bit(j*12+11)<<12)};d!=0}).count()
}
fn main(){
 let pins=read(format!("{R}/pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let result=read(format!("{R}/runs/result.json"));assert_eq!(result["status"],"complete");assert_eq!(result["steps"].as_array().unwrap().len(),4);
 for s in["compile-tests","alias-tests","compile-study","real-alias-counts"]{let exit=read(format!("{R}/runs/{s}/exit.json"));assert_eq!(exit["returncode"],0);assert_eq!(exit["success"],true);for stream in["stdout","stderr"]{assert_eq!(hash(format!("{R}/runs/{s}/{stream}.log")),exit[format!("{stream}_sha256")].as_str().unwrap());}}
 let tests=read(format!("{R}/runs/alias-tests/tests.json"));assert_eq!(tests["passed_tests"].as_array().unwrap().len(),3);assert_eq!(tests["unfiltered"],true);
 let model=read(format!("{R}/analysis.json"));assert_eq!(read(format!("{R}/runs/real-alias-counts/stdout.log")),model);
 let inputs=read(format!("{R}/inputs.json"));let mut cases=Vec::new();let mut total=0;
 for(name,input)in inputs.as_object().unwrap(){
  let b=fs::read(input["file"].as_str().unwrap()).unwrap();assert_eq!(hash(input["file"].as_str().unwrap()),input["sha256"].as_str().unwrap());assert_eq!(&b[..8],b"B2SCAN01");
  let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+n*160);total+=n;
  let rows=model["rows"].as_array().unwrap().iter().find(|v|v["case"]==name.as_str()).unwrap();assert_eq!(rows["raw_rows"],n);let part=n.div_ceil(2);let mut parts=Vec::new();let mut whole_net=0;
  for(start,expected)in(0..n).step_by(part).zip(rows["parts"].as_array().unwrap()){
   let end=(start+part).min(n);assert_eq!(expected["start"],start);assert_eq!(expected["end"],end);
   let mut signed=Vec::new();let mut exact=Vec::new();let mut active=0;
   for i in start..end{let scalar=&b[16+128*n+32*i..16+128*n+32*(i+1)];let w=std::array::from_fn(|j|u64::from_le_bytes(scalar[8*j..8*j+8].try_into().unwrap()));let identity=b[16+128*i..16+128*(i+1)].iter().all(|&b|b==0);if identity||w==[0;4]{continue;}active+=1;exact.push(w);signed.push(normalize(w));}
   exact.sort_unstable();exact.dedup();signed.sort_unstable();signed.dedup();assert_eq!(expected["active_original_rows"],active);assert_eq!(expected["signed_full_scalar_classes"],signed.len());
   let mut hist=BTreeMap::<(usize,usize),usize>::new();let(mut zeros,mut eligible,mut members,mut formation,mut removed,mut net,mut one)=(0,0,0,0,0,0,0);
   for c in 0..2{let mut magnitudes=Vec::new();for &w in&signed{let s=split::phi_split(w);let m=s[2*c]as u128|(((s[2*c+1]&0x7fff_ffff_ffff_ffff)as u128)<<64);if m==0{zeros+=1;}else{magnitudes.push(m);}}
    magnitudes.sort_unstable();let mut pos=0;while pos<magnitudes.len(){let mut end=pos+1;while end<magnitudes.len()&&magnitudes[end]==magnitudes[pos]{end+=1;}let count=end-pos;let w=weight(magnitudes[pos]);*hist.entry((count,w)).or_default()+=1;if count>=2&&w==1{one+=1;}if count>=2&&w>=2{eligible+=1;members+=count;formation+=count-1;removed+=(count-1)*w;net+=(count-1)*(w-1);}pos=end;}
   }
   let got=json!({"eligible_classes":eligible,"eligible_members":members,"extra_formation_generic_additions":formation,"avoided_bucket_arrivals":removed,"net_generic_additions_saved":net,"duplicate_one_window_classes_skipped":one,"zero_components":zeros,"class_histogram_members_windows_classes":hist.iter().map(|(&(m,w),&c)|vec![m,w,c]).collect::<Vec<_>>()});assert_eq!(got,expected["counts"]);whole_net+=net;
   parts.push(json!({"start":start,"end":end,"exact_full_scalar_classes":exact.len(),"signed_full_scalar_classes":signed.len(),"extra_full_scalar_sign_merges":exact.len()-signed.len(),"reviewed_component_counts":got}));
  }
  assert_eq!(rows["net_additional_generic_additions_saved"],whole_net);cases.push(json!({"case":name,"parts":parts,"net":whole_net}));
 }
 assert_eq!(model["raw_scalar_split_reconstructions"],total);assert_eq!(model["necessary_primary_novelty_gate_passed"],false);
 let phi=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/msm_affine/g2_phi.rs").unwrap();let prefix=phi.split_once("const C1:").unwrap().1.split_once("use zopro_bn254::{Fq,").unwrap().0;let s=fs::read_to_string(format!("{R}/split.rs")).unwrap();assert!(s.starts_with(&format!("const C1:{prefix}")));
 let ledger=read("/home/ubuntu/src/zopro-perf-work/finalize-drain-bundle176/proof-sha256-after.json");assert_eq!(ledger.as_array().unwrap().len(),29953);
 let out=json!({"passed":true,"pins":pins.as_object().unwrap().len(),"successful_workflow_commands":4,"new_tests":3,"production_originals_exact":176,"independent_method":"manual modulus subtraction; sorted scalar and component runs; per-bit Booth window reconstruction; no model import","cases":cases,"raw_rows":total,"new_proofs":0,"proof_ledger":29953,"necessary_primary_novelty_gate_passed":false});
 fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("ReviewPASS:4commands,3tests,{}pins,{}rawrows,176productionfiles; gatefalse",pins.as_object().unwrap().len(),total);
}
