#[path="model-fixed.rs"]mod model;
use model::{Word,Query};use serde_json::{json,Value};use std::fs;
fn main(){
 let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),4,"probe QUERY INPUT OUTPUT");let b=fs::read(&args[2]).unwrap();
 let(q,words,ids)=if args[1]=="A"{
  assert_eq!(&b[..8],b"ACOLUMN1");let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+33*n);
  let words=b[16..].chunks_exact(33).map(|v|std::array::from_fn(|i|u64::from_le_bytes(v[8*i..8*i+8].try_into().unwrap()))).collect::<Vec<Word>>();
  let ids=b[16..].chunks_exact(33).map(|v|{assert!(v[32]<=1);v[32]!=0}).collect::<Vec<_>>();(Query::A,words,ids)
 }else{
  assert_eq!(args[1],"B");assert_eq!(&b[..8],b"B2SCAN01");let n=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;assert_eq!(b.len(),16+160*n);
  let words=b[16+128*n..].chunks_exact(32).map(|v|std::array::from_fn(|i|u64::from_le_bytes(v[8*i..8*i+8].try_into().unwrap()))).collect::<Vec<Word>>();
  let ids=b[16..16+128*n].chunks_exact(128).map(|v|v.iter().all(|x|*x==0)).collect::<Vec<_>>();(Query::B,words,ids)
 };
 for &w in &words{let _=model::scalar(w);}
 let v=model::census(q,&words,&ids);fs::write(&args[3],serde_json::to_vec_pretty(&v).unwrap()).unwrap();println!("COALESCE {}",json!({"query":args[1],"rows":words.len(),"global_grouping_admitted":v["global_grouping_admitted"],"net_generic_pairs_saved":v.get("net_generic_pairs_saved").unwrap_or(&Value::Null),"new_proofs":0,"new_timing_samples":0}));
}
