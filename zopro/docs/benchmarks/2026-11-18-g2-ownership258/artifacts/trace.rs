use std::collections::{BTreeMap,BTreeSet};
#[derive(Debug)]
struct Call {tag:String, win:Option<usize>, part:Option<usize>, tid:u64, start:i64, end:i64}
fn ns(s:&str)->i64 {
 let (sec,frac)=s.trim_end_matches(':').split_once('.').unwrap();
 sec.parse::<i64>().unwrap()*1_000_000_000 + frac.parse::<i64>().unwrap()*10_i64.pow(9-frac.len() as u32)
}
pub fn analyze(trace:&str, prefix:&str, n:usize, workers:&BTreeMap<u64,u64>) -> BTreeMap<String,Vec<f64>> {
 let mut starts:BTreeMap<u64,Vec<Call>>=BTreeMap::new();
 let mut calls=vec![];let mut pids=BTreeSet::new();
 for l in trace.lines().map(str::trim).filter(|s|!s.is_empty()&&!s.starts_with('#')) {
  let f:Vec<_>=l.split_whitespace().collect();
  let (p,t)=f[0].split_once('/').unwrap();let pid=p.parse::<u64>().unwrap();pids.insert(pid);
  let tid=t.parse::<u64>().unwrap();let cpu=f[1].trim_matches(['[',']']).parse::<u64>().unwrap();
  assert!(cpu<workers.len() as u64);if tid!=pid{assert_eq!(workers.get(&tid),Some(&cpu));}
  assert_eq!(f[3],"1");let time=ns(f[2]);let name=f[4].trim_end_matches(':').strip_prefix(prefix).unwrap();
  let (tag,action)=name.rsplit_once('_').unwrap();let stack=starts.entry(tid).or_default();
  if action=="enter" {
   let field=|key:&str| f[5..].iter().find_map(|s|s.strip_prefix(key)).map(|v|v.parse::<usize>().unwrap());
   let win=if tag=="weight"{stack.iter().rev().find(|e|e.tag=="window").unwrap().win}else{field("win=")};
   let part=field("part=");
   assert_eq!(win.is_some(),matches!(tag,"window"|"part"|"weight"));assert_eq!(part.is_some(),tag=="part");
   stack.push(Call{tag:tag.into(),win,part,tid,start:time,end:0});
  }else{assert_eq!(action,"exit");let mut e=stack.pop().unwrap();assert_eq!(e.tag,tag);assert!(time>=e.start);e.end=time;calls.push(e);}
 }
 assert_eq!(pids.len(),1);assert!(starts.values().all(Vec::is_empty));assert_eq!(calls.len(),47*n);
 let sdks:Vec<_>=calls.iter().filter(|c|c.tag=="sdk").collect();assert_eq!(sdks.len(),n);let mut last=0;
 let mut spans=BTreeMap::<String,Vec<f64>>::new();
 for sdk in sdks {
  assert!(sdk.start>=last);last=sdk.end;
  let req:Vec<_>=calls.iter().filter(|c|sdk.start<=c.start && c.end<=sdk.end).collect();
  assert_eq!(req.len(),47);
  let find=|tag:&str,w:Option<usize>,p:Option<usize>|{
   let mut it=req.iter().copied().filter(|c| c.tag==tag && c.win==w && c.part==p);
   let c=it.next().unwrap();assert!(it.next().is_none());c
  };
  let g=find("g2",None,None);let raw=find("raw",None,None);
  assert!(g.start<=raw.start&&raw.end<=g.end);
  let mut parts=vec![];let mut windows=vec![];let mut weights=vec![];
  for w in 0..11{
   let window=find("window",Some(w),None);let weight=find("weight",Some(w),None);
   assert_eq!(window.tid,weight.tid);
   assert!(raw.start<=window.start&&window.end<=raw.end&&weight.end<=window.end);
   for p in 0..2{let c=find("part",Some(w),Some(p));assert!(window.start<=c.start&&c.end<=weight.start);parts.push(c);}
   windows.push(window);weights.push(weight);
  }
  let window=*windows.iter().max_by_key(|c|c.end).unwrap();let w=window.win.unwrap();let weight=weights[w];
  let part=*parts.iter().filter(|c| c.win==Some(w)).max_by_key(|c|c.end).unwrap();
  for k in 0..11 {
   let l=parts[2*k];let r=parts[2*k+1];
   assert_eq!((l.win,l.part,r.win,r.part),(Some(k),Some(0),Some(k),Some(1)));
   let (ld,rd)=(l.end-l.start,r.end-r.start);
   for (key,v) in [("p0",ld),("p1",rd),("p1_minus_p0_due_start",r.start-l.start),("p1_minus_p0_due_duration",rd-ld),("abs_end_gap",(r.end-l.end).abs())] {
    spans.entry(format!("w{k:02}_{key}_ms")).or_default().push(v as f64/1e6);
   }
   for (key,b) in [("p1_finishes_later",r.end>l.end),("p1_takes_longer",rd>ld),("window_finishes_last",w==k)] {
    spans.entry(format!("w{k:02}_{key}")).or_default().push(f64::from(b));
   }
  }
  spans.entry("last_window_critical_half_1".into()).or_default().push(f64::from(part.part==Some(1)));
  let t=[g.start,raw.start,window.start,part.start,part.end,weight.start,weight.end,window.end,raw.end,g.end];
  assert!(t.windows(2).all(|v|v[0]<=v[1]));
  let names=["before_raw","raw_to_last_window_entry","last_window_to_critical_part_start","critical_part","critical_part_end_to_weight","weight_of_last_window","weight_to_window_end","last_window_to_raw_end","raw_to_g2_end"];
  for (name,pair) in names.into_iter().zip(t.windows(2)){spans.entry(name.into()).or_default().push((pair[1]-pair[0]) as f64/1e6);}
  let first=parts.iter().map(|c|c.start).min().unwrap();let last_start=parts.iter().map(|c|c.start).max().unwrap();
  let last_end=parts.iter().map(|c|c.end).max().unwrap();let first_end=parts.iter().map(|c|c.end).min().unwrap();
  let mut ev:Vec<_>=parts.iter().flat_map(|c|[(c.start,1),(c.end,-1)]).collect();ev.sort();
  let mut active=0;let mut max_active=0;for (_,delta)in ev{active+=delta;max_active=max_active.max(active);}assert_eq!(active,0);
  for (name,d) in [("g2",g.end-g.start),("first_part_after_g2",first-g.start),("all_part_start_spread",last_start-first),("all_part_end_spread",last_end-first_end),("after_all_parts",g.end-last_end),("critical_part_start_after_first",part.start-first),("mean_part_elapsed",parts.iter().map(|c|c.end-c.start).sum::<i64>()/22)]{
   spans.entry(name.into()).or_default().push(d as f64/1e6);
  }
  for (name,v) in [("max_open_parts",max_active as usize),("distinct_part_workers",parts.iter().map(|c|c.tid).collect::<BTreeSet<_>>().len()),("last_window_index",w),("last_window_is_top",usize::from(w==10))]{spans.entry(name.into()).or_default().push(v as f64);}
 }
 spans
}
