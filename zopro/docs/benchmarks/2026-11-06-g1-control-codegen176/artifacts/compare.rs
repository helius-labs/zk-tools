use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g1-control-codegen176";
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn u16le(b:&[u8],i:usize)->usize{u16::from_le_bytes(b[i..i+2].try_into().unwrap())as usize}
fn u32le(b:&[u8],i:usize)->u64{u32::from_le_bytes(b[i..i+4].try_into().unwrap())as u64}
fn u64le(b:&[u8],i:usize)->u64{u64::from_le_bytes(b[i..i+8].try_into().unwrap())}
fn mapped<'a>(b:&'a[u8],addr:u64,n:usize)->&'a[u8]{
 assert_eq!(&b[..6],b"\x7fELF\x02\x01");assert_eq!(u16le(b,18),62);
 let base=u64le(b,32)as usize;let stride=u16le(b,54);let count=u16le(b,56);
 for i in 0..count{let q=base+i*stride;if u32le(b,q)!=1{continue}let v=u64le(b,q+16);let size=u64le(b,q+32);if addr>=v&&addr.checked_add(n as u64).unwrap()<=v+size{let off=u64le(b,q+8)+(addr-v);return &b[off as usize..off as usize+n]}}
 panic!("unmapped ELF input address {addr:x}, size{n}");
}
fn load_width(m:&str,ops:&str)->Option<usize>{
 if m=="vpbroadcastq"||ops.contains("{1to8}"){return Some(8)}
 if m=="vpbroadcastd"||ops.contains("{1to16}"){return Some(4)}
 if m=="vbroadcasti128"||m=="vbroadcastf128"{return Some(16)}
 if m.starts_with('v'){if ops.contains("%zmm"){Some(64)}else if ops.contains("%ymm"){Some(32)}else if ops.contains("%xmm"){Some(16)}else{None}}else{None}
}
fn callee(s:&str)->String{let symbol=s.split_once('<').unwrap().1.trim_end_matches('>');let (base,offset)=symbol.split_once("+0x").map(|(a,b)|(a,format!("+0x{b}"))).unwrap_or((symbol,String::new()));format!("{}{}",rustc_demangle::try_demangle(base).map(|x|format!("{x:#}")).unwrap_or(base.into()),offset)}
fn normalize(label:&str,row:&Value)->Value{
 let binary=fs::read(row["binary"].as_str().unwrap()).unwrap();assert_eq!(h(&binary),row["binary_sha256"].as_str().unwrap());
 let bytes=fs::read(format!("{R}/{label}/assembly.txt")).unwrap();assert_eq!(h(&bytes),row["objdump"]["stdout_sha256"].as_str().unwrap());
 let start=row["address"].as_u64().unwrap();let end=start+row["bytes"].as_u64().unwrap();let s=String::from_utf8(bytes).unwrap();let mut lines=vec![];let mut refs=vec![];let mut calls=vec![];let mut ptrs=BTreeMap::<u64,usize>::new();let mut unknown=vec![];
 for line in s.lines(){let Some((a,t))=line.trim().split_once(':')else{continue};let Ok(pc)=u64::from_str_radix(a,16)else{continue};assert!(pc>=start&&pc<end);let text=t.trim();let(m,rest)=text.split_once(char::is_whitespace).unwrap_or((text,""));let ops=rest.split_once('#').map(|(a,_)|a).unwrap_or(rest).split_whitespace().collect::<String>();
  let normalized=if ops.contains("(%rip)"){
   let comment=rest.split_once('#').unwrap().1.trim();let target=u64::from_str_radix(comment.split_whitespace().next().unwrap(),16).unwrap();let at=ops.find("(%rip)").unwrap();let after=&ops[at+6..];let memory_start=ops[..at].rfind(',').map_or(0,|i|i+1);let before=&ops[..memory_start];
   if m=="lea"{let next=ptrs.len();let id=*ptrs.entry(target).or_insert(next);let payload=mapped(&binary,target,24);refs.push(json!({"offset":pc-start,"kind":"address_not_dereferenced_by_this_instruction","target":target,"pointer_alias_id":id,"first24bytes_sha256":h(payload),"bytes_are_not_a_semantic_object_decode":true}));format!("{m} RIP_ADDRESS_{id}{after}")}
   else if let Some(n)=load_width(m,&ops){let payload=mapped(&binary,target,n);let digest=h(payload);refs.push(json!({"offset":pc-start,"kind":"constant_load","target":target,"read_bytes":n,"payload_sha256":digest}));format!("{m} {before}RIP_CONST_{n}_{digest}{after}")}
   else{unknown.push(json!({"offset":pc-start,"instruction":text}));format!("{m} UNRESOLVED_RIP:{ops}")}
  }else if (m.starts_with('j')||m=="call")&&rest.contains('<'){
   let target=u64::from_str_radix(rest.split_whitespace().next().unwrap(),16).unwrap();if target>=start&&target<end{format!("{m} LOCAL+{:x}",target-start)}else{let name=callee(rest);calls.push(json!({"offset":pc-start,"target":target,"signature":name,"callee_body_identity_not_checked":true}));format!("{m} EXTERNAL:{name}")}
  }else{format!("{m} {ops}")};
  lines.push(format!("{:x} {normalized}",pc-start));
 }
 assert_eq!(lines.len()as u64,row["instruction_sites"].as_u64().unwrap());let text=lines.join("\n")+"\n";fs::write(format!("{R}/{label}/normalized.txt"),&text).unwrap();
 json!({"normalized_sha256":h(text.as_bytes()),"lines":lines,"RIP_references":refs,"external_calls":calls,"unresolved_RIP":unknown,"base_mod64":start%64,"scope":"exact offsets/mnemonics/registers/immediates/local branch offsets and resolved constant bytes; external call signatures and address alias graph only, not callee bodies or pointed-to object semantics"})
}
fn main(){assert!(!Path::new(R).join("comparison.json").exists());let raw=fs::read(format!("{R}/analysis.json")).unwrap();let a:Value=serde_json::from_slice(&raw).unwrap();let mut rows=BTreeMap::new();for label in["original","two_chain_control","descriptor_control"]{rows.insert(label,normalize(label,&a["rows"][label]));}
 let original=rows["original"]["lines"].as_array().unwrap();let mut comparisons=BTreeMap::new();for label in["two_chain_control","descriptor_control"]{let other=rows[label]["lines"].as_array().unwrap();assert_eq!(original.len(),other.len());let differences=original.iter().zip(other).enumerate().filter(|(_, (a,b))|a!=b).map(|(i,(a,b))|json!({"instruction_index":i,"original":a,"other":b})).collect::<Vec<_>>();comparisons.insert(label,json!({"all_normalized_instructions_equal":differences.is_empty(),"different_instruction_rows":differences.len(),"differences":differences}));}
 let result=json!({"input_analysis_sha256":h(&raw),"rows":rows,"comparisons":comparisons,"new_runtime_candidate":false,"new_timing":false,"no_causal_latency_or_alignment_claim":true});fs::write(format!("{R}/comparison.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();
 for(label,c)in result["comparisons"].as_object().unwrap(){println!("{label}: equal={} differences={} unresolved_rip={} alignmentmod64={}",c["all_normalized_instructions_equal"],c["different_instruction_rows"],result["rows"][label]["unresolved_RIP"].as_array().unwrap().len(),result["rows"][label]["base_mod64"]);}
}
