use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf}};
use serde_json::{json,Value};
use sha2::{Sha256,Digest};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn log(dir:&Path,stream:&str)->String{
 let e=read(dir.join("exit.json"));assert_eq!(e["returncode"],0);assert_eq!(e["success"],true);
 let p=dir.join(format!("{stream}.log"));assert_eq!(hash(&p),e[format!("{stream}_sha256")]);
 fs::read_to_string(p).unwrap()
}
fn main(){
 let a:Vec<_>=std::env::args().collect();let root=Path::new(&a[1]);let out=Path::new(&a[2]);let pre=a[3]=="pre";
 let jobs=read(root.join("config/jobs.json"));let c=read(root.join("config/case.json"));let mut rows=vec![];
 let mut ledger:BTreeSet<String>=serde_json::from_value(read(root.join("config/seed.json"))).unwrap();assert_eq!(ledger.len(),64421);
 for j in jobs.as_array().unwrap().iter().filter(|j| !pre||j["pre"]==true){
  let name=j["name"].as_str().unwrap();let arm=j["arm"].as_str().unwrap();let n=j["n"].as_u64().unwrap() as usize;let b=j["block"].as_u64().unwrap();
  let phase=if j["pre"]==true{"pre".to_string()}else{format!("b{b}")};
  let capture=out.join(&phase).join(name);let raw=log(&capture,"stdout");let offline=out.join(format!("{phase}-review"));
  let samples:Vec<Value>=raw.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).map(|s|serde_json::from_str(s).unwrap()).collect();
  assert_eq!(samples.len(),n);let sums:Vec<Value>=raw.lines().filter_map(|l|l.strip_prefix("SUMMARY ")).map(|s|serde_json::from_str(s).unwrap()).collect();assert_eq!(sums.len(),1);
  let s=&sums[0];assert_eq!(s["priming"],3);assert_eq!(s["rounds"],n);let w=j["w"].as_u64().unwrap();
  let mut workers=BTreeMap::new();
  if arm!="aeglos"{assert_eq!(s["placement_before"],s["placement_after"]);assert_eq!(s["workers"],w);
   for x in s["placement_before"]["worker_rows"].as_array().unwrap(){let i=x["index"].as_u64().unwrap();assert_eq!(x["allowed"],json!([i]));workers.insert(x["tid"].as_u64().unwrap(),i);}assert_eq!(workers.len(),w as usize);
  }else{assert_eq!(s["threads"],w);}
  let mut pdata=out.join("proofs").join(name);if arm=="aeglos"{pdata=pdata.join(c["id"].as_str().unwrap());}
  let ph=hash(pdata.join("0.proof"));assert_eq!(fs::metadata(pdata.join("0.proof")).unwrap().len(),164);assert!(ledger.insert(ph.clone()));assert_eq!(hash(pdata.join("0.public")),c["public_sha256"]);
  let verdict=log(&offline.join(format!("verify-{name}")),"stdout");assert!(verdict.contains(&format!("VERIFIED proof_sha256={ph} ")));
  let mut spans:BTreeMap<String,Vec<f64>>=BTreeMap::new();
  if arm=="trace"{
    assert!(log(&offline.join(format!("loss-{name}")),"stdout").contains("zero-lost"));
    let trace=log(&offline.join(format!("decode-{name}")),"stdout");let mut by=BTreeMap::<String,Vec<(u64,f64)>>::new();let mut pids=BTreeSet::new();
    for line in trace.lines().filter(|v|!v.trim().is_empty()&&!v.starts_with('#')){
     let f:Vec<_>=line.split_whitespace().collect();let (p,t)=f[0].split_once('/').unwrap();let pid=p.parse::<u64>().unwrap();let tid=t.parse::<u64>().unwrap();pids.insert(pid);
     let cpu=f[1].trim_matches(['[',']']).parse::<u64>().unwrap();assert!(cpu<w);
     if tid!=pid{assert_eq!(workers.get(&tid),Some(&cpu));}
     let time=f[2].trim_end_matches(':').parse::<f64>().unwrap()*1000.;assert_eq!(f[3],"1");
     let label=f[4].strip_prefix(&format!("zp251b{b}:")).unwrap().trim_end_matches(':');
     by.entry(label.into()).or_default().push((tid,time));
    }
    assert_eq!(pids.len(),1);assert_eq!(by.len(),12);
    for values in by.values(){assert_eq!(values.len(),n);}
    for i in 0..n{
     let interval=|tag:&str|{let (t,x)=by[&format!("{tag}_enter")][i];let (u,y)=by[&format!("{tag}_exit")][i];assert_eq!(t,u);assert!(y>=x);(x,y)};
     let (s0,s1)=interval("sdk");let(k0,k1)=interval("canonical");let(v0,v1)=interval("solver");let(f0,f1)=interval("fft");let(a0,a1)=interval("a");let(g0,g1)=interval("g2");
     assert!(s0<=k0 && k0<=v0 && v1<=f0 && f1<=a0.min(g0) && a1.max(g1)<=k1 && k1<=s1);
     if i>0{assert!(by["sdk_exit"][i-1].1<=s0);}
     let total=samples[i]["ms"].as_f64().unwrap();let residual=total-(k1-k0);assert!(residual>=-0.05);
     for(key,value)in[("solve",v1-v0),("before_solver_inside_canonical",v0-k0),("between_solve_and_fft",f0-v1),("fft",f1-f0),("post_fft_to_canonical_exit",k1-f1),("outside_canonical",residual),("a_span",a1-a0),("g2_span",g1-g0),("g2_after_a_return",(g1-a1).max(0.)),("canonical_after_later_query",k1-a1.max(g1))]{spans.entry(key.into()).or_default().push(value);}
    }
  }
  for(i,v)in samples.iter().enumerate(){assert_eq!(v["round"],i);if arm=="aeglos"{assert_eq!(v["cache_hit"],true);for k in ["witness_ms","fft_ms","msm_ms","assembly_ms","scalars_ms"]{spans.entry(k.into()).or_default().push(v[k].as_f64().unwrap());}}spans.entry("full".into()).or_default().push(v[if arm=="aeglos"{"verified_ms"}else{"ms"}].as_f64().unwrap());}
  let means=spans.into_iter().map(|(k,v)|(k,v.iter().sum::<f64>()/n as f64)).collect::<BTreeMap<_,_>>();
  rows.push(json!({"job":j,"means":means,"proof_sha256":ph,"rss_mib":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"cpu_ms":(s["user_cpu_ms"].as_f64().unwrap()+s["system_cpu_ms"].as_f64().unwrap())/n as f64}));
 }
 assert_eq!(rows.len(),if pre{6}else{24});let dest=PathBuf::from(&a[4]);fs::write(dest,serde_json::to_vec_pretty(&json!({"passed":true,"rows":rows,"unique":ledger.len()})).unwrap()).unwrap();
 if !pre{fs::write(&a[5],serde_json::to_vec_pretty(&ledger).unwrap()).unwrap();}
}
