use std::{collections::{BTreeMap,BTreeSet},fs,path::Path};
use serde_json::{Value,json};
use sha2::{Sha256,Digest};
fn sha(data:&[u8])->String{format!("{:x}",Sha256::digest(data))}
fn file(p:impl AsRef<Path>)->Vec<u8>{fs::read(p).unwrap()}
fn text(p:impl AsRef<Path>)->String{fs::read_to_string(p).unwrap()}
fn verify(proofs:&mut BTreeSet<String>,p:&Path,log:&Path,public:&str)->String{
 let raw=file(p.join("0.proof"));assert_eq!(raw.len(),164);
 let h=sha(&raw);assert!(text(log).contains(&format!("VERIFIED proof_sha256={h} public_sha256={public} ")));
 assert_eq!(sha(&file(p.join("0.public"))),public);assert!(proofs.insert(h.clone()));h
}
struct Call { tag:String,start:u64,end:u64 }
fn trace(data:&str, worker:&BTreeMap<u64,u64>,b:u64,samples:&[f64])->Value{
 let mut stack:BTreeMap<u64,Vec<(String,u64)>>=BTreeMap::new();let mut calls=vec![];let mut pids=BTreeSet::new();
 for line in data.lines().filter(|l|!l.trim().is_empty()&&!l.starts_with('#')){
  let f:Vec<_>=line.split_whitespace().collect();let(p,t)=f[0].split_once('/').unwrap();let(pid,tid)=(p.parse::<u64>().unwrap(),t.parse::<u64>().unwrap());
  let cpu=f[1].trim_matches(['[',']']).parse::<u64>().unwrap();pids.insert(pid);assert!(cpu<worker.len()as u64);
  if pid!=tid{assert_eq!(worker.get(&tid),Some(&cpu));}
  let ts=f[2].trim_end_matches(':').replace('.',"").parse::<u64>().unwrap();assert_eq!(f[3],"1");
  let(tag,op)=f[4].trim_end_matches(':').strip_prefix(&format!("zp265b{b}:")).unwrap().rsplit_once('_').unwrap();
  let s=stack.entry(tid).or_default();match op{"enter"=>s.push((tag.into(),ts)),"exit"=>{let(k,start)=s.pop().unwrap();assert_eq!(k,tag);assert!(start<=ts);calls.push(Call{tag:k,start,end:ts});},_=>panic!()}
 }
 assert_eq!(pids.len(),1);assert!(stack.values().all(Vec::is_empty));
 let sdks:Vec<_>=calls.iter().filter(|e|e.tag=="sdk").collect();assert_eq!(sdks.len(),samples.len());
 let mut rounds=vec![];let mut accounted=0;
 for(sdk,&wall)in sdks.into_iter().zip(samples){
  let req:Vec<_>=calls.iter().filter(|c|c.start>=sdk.start&&c.end<=sdk.end).collect();accounted+=req.len();
  let mut counts=BTreeMap::new();for c in &req{*counts.entry(c.tag.as_str()).or_insert(0usize)+=1}
  assert_eq!(counts,BTreeMap::from([("sdk",1),("solver",1),("fft",1),("msm",1),("group",3),("g1raw",3),("g1groups",2),("g2groups",1),("g2raw",1)]));
  let one=|tag:&str|*req.iter().find(|c|c.tag==tag).unwrap();
  let(s,f,m)=(one("solver"),one("fft"),one("msm"));
  let marks=[sdk.start,s.start,s.end,f.start,f.end,m.start,m.end,sdk.end];assert!(marks.windows(2).all(|w|w[0]<=w[1]));
  let mut stage=BTreeMap::new();for(k,pair)in ["before_solve","solve","between_solve_fft","fft","before_msm","msm","after_msm"].iter().zip(marks.windows(2)){stage.insert(*k,(pair[1]-pair[0])as f64/1e6);}
  stage.insert("outside_sdk",wall-(sdk.end-sdk.start)as f64/1e6);
  let groups:Vec<_>=req.iter().filter(|c|c.tag=="group").collect();
  let g2=one("g2groups");let g2raw=one("g2raw");assert!(m.start<=g2.start&&g2.start<=g2raw.start&&g2raw.end<=g2.end&&g2.end<=m.end);
  rounds.push(json!({"stages":stage,"full":wall,"first_group_after_fft":(groups.iter().map(|c|c.start).min().unwrap()-f.end)as f64/1e6,"group_wall_ms":groups.iter().map(|c|(c.end-c.start)as f64/1e6).collect::<Vec<_>>(),"g2_grouped_ms":(g2.end-g2.start)as f64/1e6,"g2_raw_ms":(g2raw.end-g2raw.start)as f64/1e6}));
 }
 assert_eq!(accounted,calls.len());json!(rounds)
}
fn main(){
 let orig=file("../fft-phi264/host-ledger.json");assert_eq!(sha(&orig),"b3e5168758bcbae140c4080d620e787b9202b0633c2c13a8024fbeb117bad693");
 let mut ledger:BTreeSet<String>=serde_json::from_slice(&orig).unwrap();let public="b0d48d1dbe96148b0f30d7ba3b1d74e65d5bdf3f8a45f9d7cd27b03c0e939095";
 verify(&mut ledger,Path::new("output/b0-w16-trace"),Path::new("exploratory.verify"),public);
 let base=Path::new("sparse/output");let mut rows=vec![];
 for b in 0..3 {for w in [16,32]{for arm in ["plain","trace"]{
  let n=format!("b{b}-w{w}-{arm}");let log=text(base.join(format!("{n}.log")));
  let s:Value=serde_json::from_str(log.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();
  assert_eq!(s["placement_before"],s["placement_after"]);assert_eq!(s["workers"],w);assert_eq!(s["priming"],3);
  let samples:Vec<f64>=log.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).enumerate().map(|(i,l)|{let v:Value=serde_json::from_str(l).unwrap();assert_eq!(v["round"],i);v["ms"].as_f64().unwrap()}).collect();assert_eq!(samples.len(),17);
  let h=verify(&mut ledger,&base.join(&n),&base.join(format!("{n}.verify")),public);
  let workers=s["placement_before"]["worker_rows"].as_array().unwrap().iter().map(|r|{assert_eq!(r["allowed"],json!([r["index"]]));(r["tid"].as_u64().unwrap(),r["index"].as_u64().unwrap())}).collect();
  let phases=if arm=="trace"{trace(&text(base.join(format!("{n}.trace"))),&workers,b,&samples)}else{Value::Null};
  rows.push(json!({"block":b,"workers":w,"arm":arm,"full_ms":samples.iter().sum::<f64>()/17.,"rss_mib":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"cpu_ms":(s["user_cpu_ms"].as_f64().unwrap()+s["system_cpu_ms"].as_f64().unwrap())/17.,"rounds":phases,"proof":h}));
 }}}
 assert_eq!(ledger.len(),65247);fs::write("ledger.json",serde_json::to_vec_pretty(&ledger).unwrap()).unwrap();fs::write("rows.json",serde_json::to_vec(&rows).unwrap()).unwrap();
}
