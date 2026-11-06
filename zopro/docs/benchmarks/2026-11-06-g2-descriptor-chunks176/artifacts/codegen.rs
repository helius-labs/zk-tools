use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,process::Command,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-descriptor-chunks176";
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let id:Value=serde_json::from_slice(&fs::read(format!("{R}/identities.json")).unwrap()).unwrap();
 let binary=format!("{R}/candidate-bin");assert_eq!(hash(&binary),id["ON"]["binary_sha256"].as_str().unwrap());
 assert!(!std::path::Path::new(R).join("linked-code").exists());fs::create_dir(format!("{R}/linked-code")).unwrap();
 let nm=Command::new("nm").args(["-S","--size-sort",&binary]).output().unwrap();assert!(nm.status.success());
 let text=String::from_utf8(nm.stdout).unwrap();let mut all=BTreeMap::new();
 for (label,wanted) in [("OFF","::msm::g2_borrowed::apply::<"),("ON","::msm::descriptor_chunks::apply::<")]{
  let symbols=text.lines().filter_map(|l|{let f=l.split_whitespace().collect::<Vec<_>>();if f.len()!=4{return None}let demangled=rustc_demangle::try_demangle(f[3]).map(|x|format!("{x:#}")).unwrap_or(f[3].into());if demangled.contains(wanted){Some((f[3],demangled,u64::from_str_radix(f[0],16).unwrap(),u64::from_str_radix(f[1],16).unwrap()))}else{None}}).collect::<Vec<_>>();assert_eq!(symbols.len(),1);
  let(raw,name,address,size)=&symbols[0];let out=Command::new("objdump").args(["-d","--no-show-raw-insn",&format!("--disassemble={raw}"),&binary]).output().unwrap();assert!(out.status.success());let s=String::from_utf8(out.stdout).unwrap();
  let path=format!("{R}/linked-code/{label}-G2.txt");fs::write(&path,&s).unwrap();
  let(mut insn,mut ifma,mut bounds,mut jumps,mut rsp)=(0,0,0,0,0);
  for line in s.lines(){let Some((a,t))=line.trim().split_once(':')else{continue};if a.is_empty()||!a.chars().all(|c|c.is_ascii_hexdigit()){continue}let Some(m)=t.split_whitespace().next()else{continue};insn+=1;if m.starts_with("vpmadd52"){ifma+=1}if m=="call"&&t.contains("panic_bounds_check"){bounds+=1}if m.starts_with('j')&&m!="jmp"{jumps+=1}if t.contains("(%rsp)"){rsp+=1}}
  all.insert(label,json!({"symbol":name,"address":address,"bytes":size,"instruction_sites":insn,"IFMA_sites":ifma,"panic_bounds_check_call_sites":bounds,"conditional_jump_sites":jumps,"RSP_operand_lines":rsp,"assembly_sha256":hash(&path)}));
 }
 let same_address=all["OFF"]["address"]==all["ON"]["address"];
 let value=json!({"binary_sha256":hash(&binary),"variants":all,"merged_at_same_address":same_address,"scope":"linked static code only; not dynamic executed checks, spills, stack bounds, performance or causal attribution"});
 fs::write(format!("{R}/codegen.json"),serde_json::to_vec_pretty(&value).unwrap()).unwrap();println!("{value}");
}
