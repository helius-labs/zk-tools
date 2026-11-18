use std::{collections::{BTreeMap,BTreeSet},fs,path::Path};
use serde_json::{json,Value};
use sha2::{Sha256,Digest};

#[derive(Debug)]
struct Call {name:String,start:i64,end:i64,n:Option<usize>}
fn read(p:impl AsRef<Path>)->String{fs::read_to_string(p).unwrap()}
fn sha(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn intervals(text:&str, prefix:&str, workers:&BTreeMap<u64,u64>)->Vec<Call>{
 let mut stack=BTreeMap::<u64,Vec<Call>>::new();let mut result=vec![];let mut pids=BTreeSet::new();
 for l in text.lines().filter(|s|!s.is_empty()&&!s.starts_with('#')){
  let f:Vec<_>=l.split_whitespace().collect();let(pid,tid)=f[0].split_once('/').unwrap();
  let(pid,tid)=(pid.parse::<u64>().unwrap(),tid.parse::<u64>().unwrap());pids.insert(pid);
  let cpu=f[1].trim_matches(['[',']']).parse::<u64>().unwrap();assert!(cpu<16);if pid!=tid{assert_eq!(workers.get(&tid),Some(&cpu));}
  let t=f[2].trim_end_matches(':').replace('.',"").parse::<i64>().unwrap();assert_eq!(f[3],"1");
  let(tag,act)=f[4].trim_end_matches(':').strip_prefix(prefix).unwrap().rsplit_once('_').unwrap();let s=stack.entry(tid).or_default();
  match act{
   "enter"=>{let arg=|key:&str|f.iter().find_map(|v|v.strip_prefix(key)).map(|s|s.parse::<usize>().unwrap());
    let n=arg("bases=");assert_eq!(n,arg("scalars="));assert_eq!(n.is_some(),tag=="g1raw");
    s.push(Call{name:tag.into(),start:t,end:t,n});}
   "exit"=>{let mut c=s.pop().unwrap();assert_eq!(c.name,tag);assert!(t>=c.start);c.end=t;result.push(c);}
   _=>panic!()
  }
 }
 assert_eq!(pids.len(),1);assert!(stack.values().all(Vec::is_empty));result
}
fn overlap(c:&Call,d:&Call)->f64{(c.end.min(d.end)-c.start.max(d.start)).max(0) as f64/1e6}
fn main(){
 let mut rows=vec![];let mut hashes=BTreeSet::new();
 for b in 0..3{for(k,arm)in ["late","full","group"].into_iter().enumerate(){for obs in ["plain","trace"]{
  let name=format!("b{b}-{arm}-{obs}");let out=Path::new("output");
  let log=read(out.join(format!("{name}.log")));let s:Value=serde_json::from_str(log.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();
  assert_eq!(s["workers"],16);assert_eq!(s["placement_before"],s["placement_after"]);assert_eq!(s["priming"],3);assert_eq!(s["rounds"],17);assert_eq!(s["overlapped_queries"],if arm=="late"{0}else{17});
  let workers=s["placement_before"]["worker_rows"].as_array().unwrap().iter().map(|r|{assert_eq!(r["allowed"],json!([r["index"]]));(r["tid"].as_u64().unwrap(),r["index"].as_u64().unwrap())}).collect::<BTreeMap<_,_>>();assert_eq!(workers.len(),16);
  let samples:Vec<f64>=log.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).enumerate().map(|(i,l)|{let v:Value=serde_json::from_str(l).unwrap();assert_eq!(v["round"],i);v["ms"].as_f64().unwrap()}).collect();assert_eq!(samples.len(),17);
  let p=out.join(&name);let h=sha(p.join("0.proof"));assert_eq!(fs::metadata(p.join("0.proof")).unwrap().len(),164);let public=sha(p.join("0.public"));assert_eq!(public,"b0d48d1dbe96148b0f30d7ba3b1d74e65d5bdf3f8a45f9d7cd27b03c0e939095");
  assert!(read(out.join(format!("{name}.verify"))).contains(&format!("VERIFIED proof_sha256={h} public_sha256={public} ")));assert!(hashes.insert(h.clone()));
  let mut traces=vec![];
  if obs=="trace"{
   let calls=intervals(&read(out.join(format!("{name}.trace"))),&format!("zp268b{}:",b*3+k),&workers);
   let sdks:Vec<_>=calls.iter().filter(|c|c.name=="sdk").collect();assert_eq!(sdks.len(),17);let mut accounted=0;
   for(sdk,wall)in sdks.into_iter().zip(&samples){
    let req:Vec<_>=calls.iter().filter(|c|c.start>=sdk.start&&c.end<=sdk.end).collect();accounted+=req.len();
    let all=|tag:&str|req.iter().copied().filter(|c|c.name==tag).collect::<Vec<_>>();
    let one=|tag:&str|{let xs=all(tag);assert_eq!(xs.len(),1);xs[0]};
    let fft=one("fft");let sol=one("solver");let owner=one(if arm=="late"{"msm"}else{"overlap"});let groups=all("group");let a=all("g1groups");let g=one("g2groups");let _gr=one("g2raw");
    assert_eq!(req.len(),14);assert_eq!(groups.len(),3);assert_eq!(a.len(),2);
    let raws=all("g1raw");let lens:BTreeSet<_>=raws.iter().map(|c|c.n.unwrap()).collect();assert_eq!(lens,BTreeSet::from([43958,66488,554226]));
    let h=raws.into_iter().find(|c|c.n==Some(554226)).unwrap();assert!(fft.end<=h.start);
    let wend=g.end.max(a.iter().map(|e|e.end).max().unwrap());
    let gfirst=groups.iter().map(|e|e.start).min().unwrap();
    let glast=groups.iter().map(|e|e.end).max().unwrap();
    let marks=[sdk.start,sol.start,sol.end,fft.start,fft.end,h.start,h.end,sdk.end];
    assert!(marks.windows(2).all(|x|x[0]<=x[1]));
    let stages=["prefix","solve","prepare","fft","fft_to_h","h","after_h"].into_iter().zip(marks.windows(2)).map(|(k,p)|(k,(p[1]-p[0])as f64/1e6)).collect::<BTreeMap<_,_>>();
    traces.push(json!({"stages":stages,"full":wall,"owner_ms":(owner.end-owner.start)as f64/1e6,"first_group_minus_fft_start":(gfirst-fft.start)as f64/1e6,"last_group_minus_fft_end":(glast-fft.end)as f64/1e6,"group_fft_overlap_summed_ms":groups.iter().map(|c|overlap(c,fft)).sum::<f64>(),"g2_fft_overlap":overlap(g,fft),"witness_end_minus_h_start":(wend-h.start)as f64/1e6,"h_after_witness":h.end>=wend,"h_starts_after_witness":h.start>=wend}));
   }assert_eq!(accounted,calls.len());
  }
  rows.push(json!({"block":b,"arm":arm,"observer":obs,"ms":samples.iter().sum::<f64>()/17.,"rss":s["whole_process_maxrss_kib"].as_f64().unwrap()/1024.,"traces":traces,"proof":h}));
 }}}
 fs::write("rows.json",serde_json::to_vec(&rows).unwrap()).unwrap();fs::write("hashes.json",serde_json::to_vec_pretty(&hashes).unwrap()).unwrap();
}
