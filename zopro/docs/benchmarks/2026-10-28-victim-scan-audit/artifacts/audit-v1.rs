use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::{BufRead,BufReader,Read},path::Path,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/victim-scan-audit";
const B:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal";
const I:&str="/home/ubuntu/src/zopro-perf-work/off-ibs";
const A:&str="/home/ubuntu/src/zopro-perf-work/batch-off-code-audit";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
#[derive(Clone,Debug)]pub struct Function{label:String,at:u64,size:u64,name:String,instructions:BTreeMap<u64,String>}
fn contains(f:&Function,at:u64)->bool{at>=f.at&&at-f.at<f.size}
fn mapping(line:&str,binary:&str)->Option<u64>{
 if !line.contains(": PERF_RECORD_MMAP2 ")||!line.ends_with(&format!(": r--p {binary}")){return None}
 let part=line.split_once(": [")?.1;let (address,rest)=part.split_once('(')?;
 let (_,rest)=rest.split_once(" @ ")?;if rest.split_whitespace().next()? != "0"{return None}
 u64::from_str_radix(address.strip_prefix("0x")?,16).ok()
}
fn load_bias(path:&Path,binary:&str)->u64{for line in BufReader::new(fs::File::open(path).unwrap()).lines(){if let Some(v)=mapping(&line.unwrap(),binary){return v}}panic!("offset zero SDK mapping missing")}
mod inherited{
 #![allow(dead_code)]
 include!("decoder-inherited.rs");
 pub fn histogram(raw:&std::path::Path,script:&std::path::Path,bias:u64,functions:&[super::Function],expected:u64)->Value{
  let mut records=raw_records(BufReader::new(File::open(raw).unwrap())).unwrap();assert_eq!(records.len()as u64,expected);
  let mut all=0;let mut counts=BTreeMap::<String,BTreeMap<u64,(u64,u64,u64,u64)>>::new();
  for line in BufReader::new(File::open(script).unwrap()).lines(){let line=line.unwrap();let Some(s)=parse_sample(&line).unwrap()else{continue};let r=records.remove(&s.key).unwrap();assert_eq!((r.weight,r.source),(s.weight,s.source));all+=1;
   let Some(at)=r.key.ip.checked_sub(bias)else{continue};let Some(f)=functions.iter().find(|f|super::contains(f,at))else{continue};
   assert!(f.instructions.contains_key(&at),"sample does not name instruction {at:x}");
   let v=counts.entry(f.label.clone()).or_default().entry(at).or_default();v.0+=1;
   if r.data3&1!=0{if r.data3&(1<<21)!=0{v.2+=1}else{v.1+=1;if r.data3&(1<<14)==0{v.3+=r.weight;}}}
  }
  assert_eq!(all,expected);assert!(records.is_empty());
  let rows:BTreeMap<_,_>=functions.iter().map(|f|{let mut rows=Vec::new();if let Some(c)=counts.get(&f.label){for(at,(all,load,pf,weight))in c{rows.push(json!({"address":at,"offset":at-f.at,"instruction":f.instructions[at],"operation_samples":all,"demand_load_samples":load,"software_prefetch_samples":pf,"positive_miss_weight_sum":weight}));}}
   let sum=|field:&str|rows.iter().map(|v|v[field].as_u64().unwrap()).sum::<u64>();
   (f.label.clone(),json!({"operation_samples":sum("operation_samples"),"demand_load_samples":sum("demand_load_samples"),"instructions":rows}))}).collect();
  json!({"samples_joined":all,"functions":rows})
 }
}
fn functions(role:&str,binary:&str)->Vec<Function>{
 let names=fs::read_to_string(format!("{A}/{role}-nm.txt")).unwrap();let mut result=Vec::new();
 for line in names.lines(){let words:Vec<_>=line.split_whitespace().collect();if words.len()<4{continue}let name=words[3..].join(" ");
  let label=if name=="<rayon_core::registry::WorkerThread>::wait_until_cold"{"wait"}else if name=="<crossbeam_deque::deque::Stealer<rayon_core::job::JobRef>>::steal"{"single_steal"}else if name.starts_with("core::iter::adapters::filter::filter_try_fold::<")&&name.contains("<rayon_core::registry::WorkerThread>::steal::") {"scan"}else{continue};
  let at=u64::from_str_radix(words[0],16).unwrap();let size=u64::from_str_radix(words[1],16).unwrap();let out=Command::new("objdump").args(["-d","-C","--no-show-raw-insn","-M","intel",&format!("--start-address={at}"),&format!("--stop-address={}",at+size),binary]).output().unwrap();assert!(out.status.success());let s=String::from_utf8(out.stdout).unwrap();fs::write(format!("{R}/{role}-{label}.asm"),&s).unwrap();
  let mut instructions=BTreeMap::new();for line in s.lines(){let Some((address,ins))=line.split_once(':')else{continue};let Ok(addr)=u64::from_str_radix(address.trim(),16)else{continue};if addr>=at&&addr<at+size{assert!(instructions.insert(addr,ins.trim().to_owned()).is_none());}}
  assert!(!instructions.is_empty());result.push(Function{label:label.into(),at,size,name,instructions});
 }
 assert_eq!(result.len(),if role=="control"{2}else{3});result
}
fn main(){let r=Path::new(R);assert!(!r.join("analysis.json").exists());let mut pins=BTreeMap::new();
 for (p,h) in read(format!("{I}/decode-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.clone(),h.as_str().unwrap().to_owned());}
 assert_eq!(hash(r.join("decoder-inherited.rs")),hash(format!("{I}/decoder-v3.rs")));
 let registry=fs::read_to_string(r.join("registry-control.rs")).unwrap();let mut off=fs::read_to_string(r.join("registry-off.rs")).unwrap();assert!(off.contains("match if self.registry.batch_local_steal {"));
 for(s,t)in [("    pub(super) batch_local_steal: bool,\n",""),("            batch_local_steal: std::env::var(\"ZOPRO_CPU_BATCH_LOCAL_STEAL\").is_ok_and(|s| s == \"1\"),\n",""),("if self.registry.batch_local_steal {\n                        let (job, available) = batch_steal::transfer(&victim.stealer, &self.worker);\n                        if available != 0 {\n                            self.registry.sleep.new_internal_jobs(available.min(u32::MAX as usize) as u32, true);\n                        }\n                        job\n                    } else { victim.stealer.steal() }","victim.stealer.steal()"),("\n#[path=\"batch_steal.rs\"]\nmod batch_steal;\n","")]{assert_eq!(off.matches(s).count(),1);off=off.replacen(s,t,1);}assert_eq!(off,registry);
 let a=read(format!("{I}/analysis.json"));let raw_pins=read(format!("{I}/raw-local-only-sha256.json"));let mut binaries=BTreeMap::new();let mut selected=BTreeMap::new();let mut metadata=BTreeMap::new();
 for role in ["control","candidate"]{let v=read(format!("{B}/native/{role}/artifacts.json"));let path=v["binary"].as_str().unwrap();assert_eq!(hash(path),v["binary_sha256"].as_str().unwrap());let f=functions(role,path);metadata.insert(role,f.iter().map(|f|json!({"label":f.label,"address":f.at,"size":f.size,"symbol":f.name,"instruction_count":f.instructions.len()})).collect::<Vec<_>>());binaries.insert(role,path.to_owned());selected.insert(role,f);}
 let mut observations=Vec::new();for j in a["rows"].as_array().unwrap(){let name=j["name"].as_str().unwrap();let role=if j["variant"]=="control"{"control"}else{"candidate"};let dir=if name=="b0-control"{"runs"}else{"decode-runs"};let raw=std::path::PathBuf::from(format!("{I}/{dir}/raw-{name}/stdout.log"));let script=std::path::PathBuf::from(format!("{I}/{dir}/script-{name}/stdout.log"));
  for p in [&raw,&script]{let p=p.display().to_string();assert_eq!(hash(&p),raw_pins[&p].as_str().unwrap());pins.insert(p.clone(),raw_pins[&p].as_str().unwrap().into());}
  let bias=load_bias(&raw,&binaries[role]);let obs=inherited::histogram(&raw,&script,bias,&selected[role],j["IBS"]["samples"].as_u64().unwrap());
  let expected=|pattern:&str|j["IBS"]["demand_symbols"].as_object().unwrap().iter().filter(|(s,_)|s.starts_with("user/")&&s.contains(pattern)).map(|(_,v)|v.as_u64().unwrap()).sum::<u64>();
  assert_eq!(obs["functions"]["wait"]["demand_load_samples"].as_u64().unwrap(),expected("WorkerThread15wait_until_cold"));
  if role=="candidate"{assert_eq!(obs["functions"]["scan"]["demand_load_samples"].as_u64().unwrap(),expected("WorkerThread5steal0"));}
  observations.push(json!({"name":name,"variant":j["variant"],"load_bias":bias,"histogram":obs}));
 }
 for name in ["audit.rs","audit","design.md","decoder-inherited.rs","registry-control.rs","registry-off.rs"]{let p=r.join(name);pins.insert(p.display().to_string(),hash(p));}
 save(r.join("pins.json"),&json!(pins));save(r.join("analysis.json"),&json!({"status":"complete offline audit","binaries":binaries,"functions":metadata,"observations":observations,"source_recovered_exactly":true,"SDK_requests":0,"new_exports":0,"proof_ledger_unchanged":12524,"production_changed":false,"adopted":false,"goal_achieved":false,"limits":"Selected functions and existing samples; added OFF work and attribution changes are not a measured causal latency fraction or improvement to accepted production"}));println!("Audited five function instances and six existing captures; no SDK/capture/verification; ledger12524 unchanged");
}
#[cfg(test)]mod tests{use super::*;
 #[test]fn only_sdk_offset_zero_mapping_sets_bias(){let b="/a/sdk";assert_eq!(mapping("0 123 0x1 [0xa8]: PERF_RECORD_MMAP2 7/7: [0xabc000(0x10000) @ 0 09:00 1 2]: r--p /a/sdk",b),Some(0xabc000));assert_eq!(mapping("0 123 0x1 [0xa8]: PERF_RECORD_MMAP2 7/7: [0xabd000(0x10000) @ 0x1000 09:00 1 2]: r--p /a/sdk",b),None);assert_eq!(mapping("0 123 0x1 [0xa8]: PERF_RECORD_MMAP2 7/7: [0xabc000(0x10000) @ 0 09:00 1 2]: r--p /other",b),None);}
 #[test]fn instruction_ranges_exclude_underflow_and_end(){let f=Function{label:"x".into(),at:0x100,size:0x20,name:"x".into(),instructions:BTreeMap::new()};assert!(!contains(&f,0xff));assert!(contains(&f,0x100));assert!(contains(&f,0x11f));assert!(!contains(&f,0x120));}
}
