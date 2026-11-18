include!("recode.rs");
use std::fs;
use serde_json::json;
use sha2::{Digest,Sha256};
fn main(){
 let data=fs::read("../g2-queue-scan175/inputs/1x2.b2q").unwrap();assert_eq!(format!("{:x}",Sha256::digest(&data)),"421d6ba37e8b3a6f5feb23829c174d11bc034cb9bd8874adf672d65038fb0ad0");assert_eq!(&data[..8],b"B2SCAN01");let n=u64::from_le_bytes(data[8..16].try_into().unwrap())as usize;assert_eq!(n,30089);assert_eq!(data.len(),16+160*n);
 let (points,scalars)=data[16..].split_at(128*n);
 assert_eq!(format!("{:x}",Sha256::digest(scalars)),"5f66ad7f79602b0f843c89b75d71371e0998ce524c048ca92af57d8fafaac13a");
 let mut counts=[[0usize;2];11];let mut per_row=Vec::new();
 for (i,(point,scalar)) in points.chunks_exact(128).zip(scalars.chunks_exact(32)).enumerate(){
  let words=phi_split(std::array::from_fn(|k|u64::from_le_bytes(scalar[8*k..8*k+8].try_into().unwrap())));let mut weight=0;
  if point.iter().any(|&v|v!=0){for k in [0,2]{let component=[words[k],words[k+1]&0x7fff_ffff_ffff_ffff,0,0];for(w,c)in counts.iter_mut().enumerate(){if booth_digit(&component,w,12)!=0{c[usize::from(i>=n.div_ceil(2))]+=1;weight+=usize::from(w<10);}}}}
  per_row.push(weight);
 }
 let total=per_row.iter().sum::<usize>();let mut prefix=0;let cut=per_row.iter().position(|x|{prefix+=x;prefix*2>=total}).unwrap()+1;
 println!("{}",serde_json::to_string_pretty(&json!({"n":n,"original_boundary":n.div_ceil(2),"ordinary_arrivals_half_boundary":cut,"ordinary_arrivals_first_at_boundary":prefix,"total_ordinary":total,"per_window":counts,"original_ordinary_per_half":[counts[..10].iter().map(|c|c[0]).sum::<usize>(),counts[..10].iter().map(|c|c[1]).sum::<usize>()]})).unwrap());
}
