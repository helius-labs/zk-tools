use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::{BTreeMap,BTreeSet},fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf177";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(name:&str,value:Value){let p=format!("{R}/{name}");assert!(!Path::new(&p).exists());fs::write(p,serde_json::to_vec_pretty(&value).unwrap()).unwrap();}
fn near(a:f64,b:f64){assert!((a-b).abs()<=1e-11*a.abs().max(b.abs()).max(1.),"{a} != {b}");}
fn order(v:&[f64],k:usize)->f64{let mut copy=v.to_vec();*copy.select_nth_unstable_by(k,f64::total_cmp).1}
fn interval(v:&[f64])->[f64;3]{assert_eq!(v.len(),30);[order(v,9),(order(v,14)+order(v,15))/2.,order(v,20)]}
fn plan(){
 let mut totals=vec![];let mut contexts=BTreeSet::new();
 let consumers=read(format!("{R}/runtime-consumers.json"));
 for phase in["preflight","main"]{
  let jobs=read(format!("{R}/{phase}-jobs.json"));let jobs=jobs.as_array().unwrap();let plan=read(format!("{R}/{phase}-plan.json"));let steps=plan["steps"].as_array().unwrap();
  assert_eq!(steps.len(),jobs.len()*2);assert_eq!(jobs.len(),if phase=="preflight"{27}else{900});let mut calls=0;let mut samples=0;let mut cells=BTreeMap::<(String,String,u64),usize>::new();
  for(i,j)in jobs.iter().enumerate(){let case=&j["case"];let n=case["name"].as_str().unwrap();let arm=j["arm"].as_str().unwrap();let b=j["block"].as_u64().unwrap();let rounds=j["rounds"].as_u64().unwrap();
   assert_eq!(rounds,if phase=="preflight"{1}else if n.starts_with("1x2-"){101}else{31});
   let expected=&consumers[arm]["copies"][(b%3)as usize]["path"];assert_eq!(&j["binary"],expected);assert_eq!(steps[i]["argv"][3],*expected);assert_eq!(steps[i]["argv"][6],rounds.to_string());assert_eq!(steps[i]["name"],j["name"]);assert_eq!(steps[i+jobs.len()]["name"],format!("verify-{}",j["name"].as_str().unwrap()));
   assert_eq!(steps[i]["argv"][1],"-c");assert_eq!(steps[i]["argv"][2],case["cpu_affinity"]);assert_eq!(j["environment"]["RAYON_NUM_THREADS"],case["workers"].as_u64().unwrap().to_string());
   if phase=="main"{assert!(b<30);contexts.insert(n.to_owned());*cells.entry((n.into(),arm.into(),b%3)).or_default()+=1;}
   calls+=rounds+3;samples+=rounds;
  }
  if phase=="main"{assert_eq!(contexts.len(),10);assert_eq!(cells.len(),90);assert!(cells.values().all(|n|*n==10));}
  assert_eq!(calls,if phase=="preflight"{108}else{43200});assert_eq!(samples,if phase=="preflight"{27}else{40500});
  totals.push(json!({"phase":phase,"exports":jobs.len(),"calls":calls,"samples":samples,"all_copy_cells_balanced":phase=="main"}));
 }
 // Independent population-count enumeration on half-binomial terms.
 let c=|n:u64,k:u64|{let mut x=1u64;for i in 1..=k{x=x*(n+1-i)/i;}x};let mass:(u64,u64)=((10..=20).map(|k|c(30,k)).sum(),1u64<<30);
 let statistics=read(format!("{R}/statistics-model.json"));near(statistics["marginal_coverage"].as_f64().unwrap(),mass.0 as f64/mass.1 as f64);
 assert_eq!(statistics["lower_rank_zero_based"],9);assert_eq!(statistics["upper_rank_zero_based"],20);
 save("independent-plan-review.json",json!({"passed":true,"totals":totals,"contexts":contexts,"coverage_numerator":mass.0,"coverage_denominator":mass.1,"coverage":mass.0 as f64/mass.1 as f64,"new_SDK_calls":0}));println!("Independent plan review passed: 927exports/43308calls; all90copy cells balanced");
}
fn finish(){
 let jobs=read(format!("{R}/main-jobs.json"));let rows=read(format!("{R}/main-rows.json"));let rows=rows.as_array().unwrap();let mut data=BTreeMap::<(String,String,u64),(f64,f64,f64)>::new();
 for j in jobs.as_array().unwrap(){let name=j["name"].as_str().unwrap();let text=fs::read_to_string(format!("{R}/main-run/{name}/stdout.log")).unwrap();let mut count=0;let mut total=0.;let mut summary=None;
  for l in text.lines(){if let Some(s)=l.strip_prefix("SAMPLE "){let v:Value=serde_json::from_str(s).unwrap();assert_eq!(v["round"],count);total+=v["ms"].as_f64().unwrap();count+=1;}else if let Some(s)=l.strip_prefix("SUMMARY "){assert!(summary.is_none());summary=Some(serde_json::from_str::<Value>(s).unwrap());}}
  assert_eq!(count,j["rounds"].as_u64().unwrap());let sum=summary.unwrap();assert_eq!(sum["rounds"],count);let mean=total/count as f64;let cpu=(sum["user_cpu_ms"].as_f64().unwrap()+sum["system_cpu_ms"].as_f64().unwrap())/count as f64;let rss=sum["whole_process_maxrss_kib"].as_f64().unwrap();let row=rows.iter().find(|v|v["name"]==name).unwrap();near(row["ms"].as_f64().unwrap(),mean);near(row["cpu_ms"].as_f64().unwrap(),cpu);assert!(data.insert((j["case"]["name"].as_str().unwrap().into(),j["arm"].as_str().unwrap().into(),j["block"].as_u64().unwrap()),(mean,cpu,rss)).is_none());
 }
 assert_eq!(data.len(),900);let assessment=read(format!("{R}/assessment.json"));let mut reproduced=vec![];
 for g in assessment["checks"].as_array().unwrap(){let ctx=g["context"].as_str().unwrap();let cmp=g["comparator"].as_str().unwrap();let metric=g["metric"].as_str().unwrap();let on=(0..30).map(|b|data[&(ctx.into(),"ON".into(),b)]).collect::<Vec<_>>();let other=(0..30).map(|b|data[&(ctx.into(),cmp.into(),b)]).collect::<Vec<_>>();
  let pass=if metric=="peak_rss_kib"{let a=on.iter().map(|r|r.2).fold(0.,f64::max);let b=other.iter().map(|r|r.2).fold(0.,f64::max);near(a,g["ON"].as_f64().unwrap());near(b,g["control"].as_f64().unwrap());a<=b*1.05+1024.}else{let v=on.iter().zip(&other).map(|(a,b)|if metric=="ms"{a.0/b.0}else{assert_eq!(metric,"cpu_ms");a.1/b.1}).collect::<Vec<_>>();let values=interval(&v);for(i,key)in["lower","median","upper"].iter().enumerate(){near(values[i],g["ratio"][key].as_f64().unwrap());}let gain=ctx=="1x2-w32"&&metric=="ms";assert_eq!(g["strict"],gain);if gain{values[2]<1.}else{values[2]<=1.02}};
  assert_eq!(g["passed"],pass);reproduced.push(json!({"context":ctx,"comparator":cmp,"metric":metric,"passed":pass}));
 }
 assert_eq!(reproduced.len(),31);let pass=reproduced.iter().filter(|g|g["passed"]==true).count();assert_eq!(assessment["checks_passed"],pass);
 let mut comparisons=vec![];for(shape,cpu,metal)in[("1x2",37.,24.),("2x2",59.,30.),("4x2",109.,48.),("8x2",189.,74.),("12x2",284.,101.),("16x2",335.,124.),("24x2",528.,176.),("merge-24x1",507.,175.)]{for arm in["REF","OFF","ON"]{let context=format!("{shape}-w32");let v=(0..30).map(|b|data[&(context.clone(),arm.into(),b)].0).collect::<Vec<_>>();let ms=interval(&v)[1];let rss=(0..30).map(|b|data[&(context.clone(),arm.into(),b)].2).fold(0.,f64::max)/1024.;comparisons.push(json!({"shape":shape,"workers":32,"arm":arm,"ms":ms,"peak_RSS_MiB":rss,"M5_CPU_ms":cpu,"M5_Metal_ms":metal,"CPU_speedup":cpu/ms,"Metal_speedup":metal/ms,"CPU10x_limit_ms":cpu/10.,"descriptive_CPU10x":ms<=cpu/10.}));}}
 let ledger=read(format!("{R}/proof-sha256-after-main.json"));assert_eq!(ledger.as_array().unwrap().len(),53899);
 save("independent-main-review.json",json!({"passed":true,"raw_processes_recomputed":900,"checks_reproduced":reproduced,"checks_passed":pass,"M5_comparisons":comparisons,"canonical_count":53899,"canonical_sha256":hash(format!("{R}/proof-sha256-after-main.json")),"performance_adopted":false,"rank_implementation":"selection rather than full sort","scope":"same-agent separate arithmetic/decision reader, not an external audit"}));println!("Independent main reader passed: {pass}/31 performance checks; all8 CPU/Metal comparisons saved");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("plan")=>plan(),Some("finish")=>finish(),_=>panic!("plan/finish")}}
