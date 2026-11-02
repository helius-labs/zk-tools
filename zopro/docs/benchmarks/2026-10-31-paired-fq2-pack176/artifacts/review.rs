use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/paired-fq2-pack176";
fn r(s:impl AsRef<Path>)->PathBuf{Path::new(R).join(s)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn close(a:f64,b:f64){assert!((a-b).abs()<=1e-12*(1.+a.abs()+b.abs()),"{a} != {b}");}
fn rank(v:&[f64])->Value{assert_eq!(v.len(),15);let mut s=v.to_vec();s.sort_by(f64::total_cmp);json!({"median":s[7],"low":s[3],"high":s[11],"wins":v.iter().filter(|&&v|v<1.).count()})}
fn verify_stat(a:&Value,b:&Value){for k in["median","low","high"]{close(a[k].as_f64().unwrap(),b[k].as_f64().unwrap());}assert_eq!(a["wins"],b["wins"]);}
fn main(){
 let a=read(r("analysis.json"));assert_eq!(a["complete"],true);
 let mut checked_pins=0;for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());checked_pins+=1;}
 let mut commands=0;for step in read(r("build-plan.json"))["steps"].as_array().unwrap().iter().take(4){let name=step["name"].as_str().unwrap();assert_eq!(read(r(format!("native-initial/{name}/exit.json")))["success"],true);commands+=1;}
 assert_eq!(read(r("native-initial/field-off/exit.json"))["success"],false);
 for(dir,plan)in[("native","build-plan-rest.json"),("runs","run-plan.json")] {assert_eq!(read(r(format!("{dir}/result.json")))["status"],"complete");for step in read(r(plan))["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let exit=read(r(format!("{dir}/{name}/exit.json")));assert_eq!(exit["success"],true,"{dir}/{name}: {exit}");assert_eq!(hash(r(format!("{dir}/{name}/stdout.log"))),exit["stdout_sha256"]);assert_eq!(hash(r(format!("{dir}/{name}/stderr.log"))),exit["stderr_sha256"]);commands+=1;}}
 let jobs=read(r("jobs.json"));let contexts=read(r("contexts.json"));assert_eq!(contexts.as_array().unwrap().len(),6);assert_eq!(jobs.as_array().unwrap().len(),292);
 let mut observations:BTreeMap<(String,String),Vec<(usize,f64)>>=BTreeMap::new();let mut signatures=BTreeMap::new();let mut all_calls=0u64;let mut primes=0u64;
 for j in jobs.as_array().unwrap(){let name=j["name"].as_str().unwrap();let output=fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();let result=output.lines().filter_map(|l|l.strip_prefix("RESULT ")).collect::<Vec<_>>();assert_eq!(result.len(),1);let out:Value=serde_json::from_str(result[0]).unwrap();
  for k in["kind","size","topology","mode"]{assert_eq!(j[k],out[k]);}assert_eq!(out["internally_verified"],true);assert_eq!(out["timed_allocations"],0);assert_eq!(out["iterations_per_worker"],j["rounds"]);assert_eq!(out["prime_calls"],3);assert_eq!(out["prime_iterations_per_worker"],1024);assert_eq!(out["SDK_requests"],0);
  let n=out["worker_count"].as_u64().unwrap();all_calls+=out["total_kernel_calls"].as_u64().unwrap();primes+=n*3072;let sig=json!([out["output_sha256"],out["capacity_bytes_per_worker"],out["fixture_reference_bytes_per_worker"]]);let key=format!("{}/{}/{}",j["context"],j["rounds"],j["mode"]);let _=key;
  let key=format!("{}/{}",j["context"],j["rounds"]);if let Some(old)=signatures.insert(key,sig.clone()){assert_eq!(sig,old);}
  if j["phase"]=="latency"{observations.entry((j["context"].as_str().unwrap().to_owned(),j["variant"].as_str().unwrap().to_owned())).or_default().push((j["block"].as_u64().unwrap()as usize,out["wall_ns_per_wave"].as_f64().unwrap()));}
 }
 assert_eq!(all_calls,a["measured_kernel_calls_including_profiles_and_preflights"].as_u64().unwrap());assert_eq!(primes,a["priming_kernel_calls"].as_u64().unwrap());
 for v in observations.values(){assert_eq!(v.len(),15);for(i,(b,_))in v.iter().enumerate(){assert_eq!(i,*b);}}
 let mut checks=BTreeMap::new();let mut ratios=BTreeMap::new();
 for c in contexts.as_array().unwrap(){let ctx=c["name"].as_str().unwrap();for(n,d)in[("off","frozen"),("on","off"),("on","frozen")] {let ns=&observations[&(ctx.to_owned(),n.to_owned())];let ds=&observations[&(ctx.to_owned(),d.to_owned())];let v=(0..15).map(|b|ns[b].1/ds[b].1).collect::<Vec<_>>();let st=rank(&v);let key=format!("{ctx}/{n}/{d}");verify_stat(&st,&a["pairs"][&key]);checks.insert(key.clone(),if n=="off"{st["low"].as_f64().unwrap()>=1./1.03&&st["high"].as_f64().unwrap()<=1.03}else{st["high"].as_f64().unwrap()<=1.03});ratios.insert(key,v);}}
 for group in["all6","logical32_2"]{let selected=contexts.as_array().unwrap().iter().filter(|c|group=="all6"||c["topology"]=="logical32").collect::<Vec<_>>();for control in["off","frozen"]{let v=(0..15).map(|b|(selected.iter().map(|c|ratios[&format!("{}/on/{control}",c["name"].as_str().unwrap())][b].ln()).sum::<f64>()/selected.len()as f64).exp()).collect::<Vec<_>>();let st=rank(&v);verify_stat(&st,&a["aggregates"][format!("{group}/on/{control}")]);checks.insert(format!("{group}/on/{control}/gain"),st["high"].as_f64().unwrap()<1.&&st["wins"].as_u64().unwrap()>=10);}}
 assert_eq!(json!(checks),a["checks"]);let pass=checks.values().all(|&v|v);assert_eq!(pass,a["component_gate"]);assert_eq!(a["proof_ledger"],29255);assert_eq!(a["adopted"],false);assert_eq!(a["production_changed"],false);
 let code=read(r("codegen.json"));assert!(code.as_array().unwrap().iter().any(|c|c["variant"]=="on"&&c["nm"].as_str().unwrap().contains("paired_g2_borrowed")));
 let mut summaries=vec![];for c in code.as_array().unwrap(){let asm=fs::read_to_string(c["assembly"].as_str().unwrap()).unwrap();assert_eq!(hash(c["assembly"].as_str().unwrap()),c["sha256"]);let instr=asm.lines().filter(|l|l.trim_start().split_once(':').is_some_and(|(a,_)|a.chars().all(|c|c.is_ascii_hexdigit()))).count();summaries.push(json!({"variant":c["variant"],"nm":c["nm"],"IFMA_sites":asm.matches("vpmadd52").count(),"instruction_lines":instr,"stack_base_operand_lines":asm.lines().filter(|l|l.contains("%rsp")||l.contains("%rbp")).count(),"shuffle_lines":asm.lines().filter(|l|l.contains("vshufi64x2")||l.contains("vpermt2q")||l.contains("vpermi2q")).count()}));}
 fs::write(r("codegen-summary.json"),serde_json::to_vec_pretty(&summaries).unwrap()).unwrap();
 let m=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");for(p,h)in m["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),h.as_str().unwrap());}
 let ledger=read("/home/ubuntu/src/zopro-perf-work/ifma-logjumps176/proof-sha256-after.json");assert_eq!(ledger.as_array().unwrap().len(),29255);let unique=ledger.as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<std::collections::BTreeSet<_>>();assert_eq!(unique.len(),29255);
 let review=json!({"passed":true,"checked_pins":checked_pins,"successful_commands":commands,"checked_callers":292,"statistics_and_all_gates_independently_recomputed":true,"production176_exact":true,"new_proofs":0,"ledger":29255,"component_gate":pass,"adopted":false});fs::write(r("review.json"),serde_json::to_vec_pretty(&review).unwrap()).unwrap();println!("{review}");
}
