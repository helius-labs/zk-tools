// Independent sorted-row count certificate. No model.rs import.
use serde_json::{json,Value};use std::{fs,path::Path};use zopro_bn254::Fr;
mod split{include!("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs");pub fn full(s:&[u64;4],w:usize)->i16{booth_digit(s,w,12)}}
type Word=[u64;4];const ROOT:&str="/home/ubuntu/src/zopro-perf-work/bounded-coalesce176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn d(q:&str,w:Word)->Vec<i16>{if q=="A"{(0..22).map(|i|split::full(&w,i)).collect()}else{let s=split::phi_split(w);(0..11).flat_map(|i|[split::digit_of(&s,true,i,12),split::digit_of(&s,false,i,12)]).collect()}}
fn main(){
 let input=read(Path::new(ROOT).join("inputs.json"));let mut results=serde_json::Map::new();
 for(name,v)in input.as_object().unwrap(){
  let q=v["query"].as_str().unwrap();let b=fs::read(v["file"].as_str().unwrap()).unwrap();let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;
  let mut words=Vec::<Word>::new();let mut identities=vec![];
  if q=="A"{assert_eq!(&b[..8],b"ACOLUMN1");assert_eq!(b.len(),16+33*n);for row in b[16..].chunks_exact(33){words.push(std::array::from_fn(|i|u64::from_le_bytes(row[8*i..8*i+8].try_into().unwrap())));assert!(row[32]<=1);identities.push(row[32]!=0);}}
  else{assert_eq!(&b[..8],b"B2SCAN01");assert_eq!(b.len(),16+160*n);for row in b[16+128*n..].chunks_exact(32){words.push(std::array::from_fn(|i|u64::from_le_bytes(row[8*i..8*i+8].try_into().unwrap())));}identities=b[16..16+128*n].chunks_exact(128).map(|v|v.iter().all(|x|*x==0)).collect();}
  let lambda=split::phi_lambda();let digits=words.iter().map(|&w|{let digits=d(q,w);let mut value=Fr::ZERO;for(i,&digit)in digits.iter().enumerate(){let x=Fr::from(digit.unsigned_abs()as u64)*if digit<0{-Fr::ONE}else{Fr::ONE};value+=x*Fr::from(2u64).pow(&[(12*if q=="A"{i}else{i/2})as u64])*if q=="B"&&i%2==0{lambda}else{Fr::ONE};}assert_eq!(value,Fr::from_canonical(w).unwrap());digits}).collect::<Vec<_>>();
  let old=read(Path::new(ROOT).join(format!("{name}.json")));assert_eq!(old["original_rows"],n);
  // Independently reproduce the exact sample/full decision via HashSets.
  use std::collections::HashSet;let sample=n.min(2048);let stride=n.div_ceil(sample);let observed=n.div_ceil(stride).min(sample);let sampled=words.iter().step_by(stride).take(sample).collect::<HashSet<_>>();let zeros=words.iter().step_by(stride).take(sample).filter(|x|**x==[0;4]).count();let percent=if(32768..65536).contains(&n){95}else{80};let nz=observed-zeros;
  let admitted=n>=256&&sampled.len()*200<=observed*199&&!(sampled.len()-usize::from(zeros!=0)==nz&&nz*100>observed*percent)&&words.iter().filter(|x|**x!=[0;4]).collect::<HashSet<_>>().len()*100<=n*percent;
  assert_eq!(old["global_grouping_admitted"],admitted);
  if admitted{assert_eq!(old["bounded_path_selected"],false);results.insert(name.clone(),json!({"bypassed":true,"every_original_scalar_reconstructed":true}));continue}
  let parts=if q=="A"{1}else{2};let length=n.div_ceil(parts);
  let mut sorted=vec![];
  for i in 0..n{if words[i]!=[0;4]&&!identities[i]{let part=i/length;let tile=(i-part*length)/1024;sorted.push((part,tile,words[i],i));}}
  sorted.sort_unstable();let nonzero=sorted.len();let(mut formations,mut classes,mut savings)=(0usize,0usize,0usize);let mut start=0;
  while start<sorted.len(){let mut end=start+1;while end<sorted.len()&&sorted[end].0==sorted[start].0&&sorted[end].1==sorted[start].1&&sorted[end].2==sorted[start].2{end+=1}
   let weight=digits[sorted[start].3].iter().filter(|x|**x!=0).count();if end-start>1&&weight>1{let repeats=end-start-1;formations+=repeats;classes+=1;savings+=repeats*(weight-1);}start=end;
  }
  let mut original_pairs=0;let mut original_arrivals=0;
  for part in 0..parts{let range=part*length..((part+1)*length).min(n);
   let windows=if q=="A"{22}else{11};for window in 0..windows{let top=window+1==windows;let mut counts=vec![0usize;if top{if q=="A"{4}else{256}}else{2048}];
    for i in range.clone(){if identities[i]{continue}for component in 0..if q=="A"{1}else{2}{let c=if q=="A"{window}else{2*window+component};let digit=digits[i][c];if digit==0{continue}let bin=if top&&q=="B"{component*128+usize::from(digit<0)*64+digit.unsigned_abs()as usize-1}else{digit.unsigned_abs()as usize-1};counts[bin]+=1;}}
    original_pairs+=counts.iter().map(|n|n.saturating_sub(1)).sum::<usize>();original_arrivals+=counts.iter().sum::<usize>();
   }
  }
  assert_eq!(old["formation_additions"],formations);assert_eq!(old["useful_repeated_classes"],classes);assert_eq!(old["net_generic_pairs_saved"],savings);assert_eq!(old["emitted_terms"],nonzero-formations);assert_eq!(old["original_generic_pairs"],original_pairs);assert_eq!(old["original_arrivals"],original_arrivals);
  assert_eq!(old["candidate_generic_pairs_including_formation"],original_pairs-savings);
  let point_bytes=classes*if q=="A"{64}else{128};let index_bytes=4*(formations+classes);
  results.insert(name.clone(),json!({"bypassed":false,"original_generic_pairs":original_pairs,"formation_additions":formations,"useful_repeated_classes":classes,"net_generic_pairs_saved":savings,"emitted_terms":nonzero-formations,"every_original_scalar_reconstructed":true,"sorted_row_certificate":true,"hypothetical_sparse_overlay_payload_bytes":{"derived_points":point_bytes,"u32_skip_and_representative_indices":index_bytes,"sum":point_bytes+index_bytes},"payload_excludes_tile_builder_curve_scratch_and_overhead":true}));
 }
 fs::write(Path::new(ROOT).join("review.json"),serde_json::to_vec_pretty(&json!({"complete":true,"cases":results,"shared_coalescer_model":false,"source_digit_helpers_shared":true,"new_proofs":0,"new_witness_solves":0,"new_latency_samples":0})).unwrap()).unwrap();println!("Independent sorted-row certificate and full scalar reconstructions passed");
}
