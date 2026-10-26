use serde_json::{json,Value};
use std::collections::BTreeMap;
pub fn median(mut v:Vec<f64>)->f64 {assert!(!v.is_empty());v.sort_by(f64::total_cmp);let n=v.len();if n%2==0{(v[n/2-1]+v[n/2])/2.0}else{v[n/2]}}
fn percentile(mut v:Vec<u64>,q:f64)->f64 {v.sort_unstable();v[((v.len()-1)as f64*q).round()as usize]as f64}
fn n(v:&Value,k:&str)->u64{v[k].as_u64().expect(k)}
pub fn inspect(v:&Value,workers:usize)->Value {
 let all=v["events"].as_array().unwrap();assert!(all.len()<=32768);let mut spans:BTreeMap<u64,Vec<(u64,u64)>>=BTreeMap::new();
 let mut batches:BTreeMap<u64,(u64,u64,usize)>=BTreeMap::new();let mut kinds:BTreeMap<u64,Vec<(u64,u64)>>=BTreeMap::new();
 let mut changes:BTreeMap<u64,(i64,i64)>=BTreeMap::new();let(mut cancelled,mut count)=(0usize,0usize);let mut total_leaf=0u128;let mut total_delay=0u128;
 for(i,e)in all.iter().enumerate(){assert_eq!(n(e,"id"),i as u64);assert_eq!(n(e,"batch")+n(e,"ordinal"),i as u64);let b=batches.entry(n(e,"batch")).or_insert((n(e,"ready"),n(e,"kind"),0));assert_eq!(b.0,n(e,"ready"));assert_eq!(b.1,n(e,"kind"));assert_eq!(b.2 as u64,n(e,"ordinal"));b.2+=1;
  match n(e,"state") {3=>{cancelled+=1;assert_eq!(n(e,"start"),0);assert_eq!(n(e,"end"),0);continue;},2=>{},_=>panic!("unfinished unit")}
  let(r,s,t,w)=(n(e,"ready"),n(e,"start"),n(e,"end"),n(e,"worker"));assert!(r<=s&&s<=t);assert!(w<workers as u64||w==u32::MAX as u64);
  spans.entry(w).or_default().push((s,t));kinds.entry(n(e,"kind")).or_default().push((s-r,t-s));
  changes.entry(r).or_default().0+=1;{let z=changes.entry(s).or_default();z.0-=1;z.1+=1;}changes.entry(t).or_default().1-=1;
  total_leaf+=(t-s)as u128;total_delay+=(s-r)as u128;count+=1;
 }
 assert!(count>0);for intervals in spans.values_mut(){intervals.sort_unstable();for p in intervals.windows(2){assert!(p[0].1<=p[1].0,"overlapping leaves on one worker");}}
 let lo=*changes.keys().next().unwrap();let hi=*changes.keys().next_back().unwrap();assert!(hi>lo);let tail=hi-(hi-lo)/10;
 let(mut pending,mut running,mut last)=(0i64,0i64,lo);let(mut area_running,mut area_pending)=(0u128,0u128);let(mut pending_low,mut tail_pending_low)=(0u64,0u64);let mut histogram=BTreeMap::<i64,u64>::new();let mut tail_hist=BTreeMap::<i64,u64>::new();let mut peak_pending=0i64;
 for(t,(dp,dr))in changes {let dt=t-last;*histogram.entry(running).or_default()+=dt;area_running+=(running as u128)*dt as u128;area_pending+=(pending as u128)*dt as u128;let td=t.saturating_sub(last.max(tail));*tail_hist.entry(running).or_default()+=td;
  if pending>0&&running<(workers as i64+1)/2{pending_low+=dt;tail_pending_low+=td;}
  pending+=dp;running+=dr;assert!(pending>=0&&running>=0&&running<=workers as i64+1);peak_pending=peak_pending.max(pending);last=t;
 }
 assert_eq!((pending,running),(0,0));assert_eq!(area_running,total_leaf);assert_eq!(area_pending,total_delay);
 let mut by_kind=BTreeMap::new();for(kind,units)in kinds {let delays:Vec<_>=units.iter().map(|x|x.0).collect();let durations:Vec<_>=units.iter().map(|x|x.1).collect();by_kind.insert(kind.to_string(),json!({"units":units.len(),"sum_leaf_elapsed_ms":durations.iter().map(|&x|x as f64).sum::<f64>()/1e6,"median_leaf_us":percentile(durations.clone(),0.5)/1e3,"p95_leaf_us":percentile(durations,0.95)/1e3,"median_declared_ready_delay_us":percentile(delays.clone(),0.5)/1e3,"p95_declared_ready_delay_us":percentile(delays,0.95)/1e3}));}
 json!({"units":count,"cancelled":cancelled,"batches":batches.len(),"interval_ms":(hi-lo)as f64/1e6,"sum_leaf_elapsed_ms":total_leaf as f64/1e6,"mean_observed_running_leaves":total_leaf as f64/(hi-lo)as f64,"mean_declared_pending_units":total_delay as f64/(hi-lo)as f64,"peak_declared_pending_units":peak_pending,"running_histogram_ns":histogram,"last_tenth_running_histogram_ns":tail_hist,"pending_with_less_than_half_workers_observed_running_fraction":pending_low as f64/(hi-lo)as f64,"last_tenth_pending_with_less_than_half_workers_observed_running_fraction":tail_pending_low as f64/(hi-tail)as f64,"kinds":by_kind})
}

#[cfg(test)]mod tests{
 use super::*;
 fn event(id:u64,start:u64,end:u64,worker:u64)->Value{json!({"id":id,"batch":0,"ordinal":id,"ready":0,"start":start,"end":end,"worker":worker,"kind":1,"state":2})}
 #[test]fn known_sweep_integrals(){let v=json!({"events":[event(0,0,10,0),event(1,5,15,1)]});let r=inspect(&v,2);assert_eq!(r["units"],2);assert!((r["mean_observed_running_leaves"].as_f64().unwrap()-4.0/3.0).abs()<1e-12);assert!((r["mean_declared_pending_units"].as_f64().unwrap()-1.0/3.0).abs()<1e-12);assert_eq!(r["running_histogram_ns"]["2"],5);}
 #[test]fn cancelled_units_do_not_contribute(){let mut c=event(2,0,0,u32::MAX as u64);c["state"]=json!(3);let r=inspect(&json!({"events":[event(0,0,10,0),event(1,5,15,1),c]}),2);assert_eq!(r["units"],2);assert_eq!(r["cancelled"],1);}
 #[test]#[should_panic(expected="overlapping leaves")]fn overlap_is_rejected(){inspect(&json!({"events":[event(0,0,10,0),event(1,5,15,0)]}),2);}
}
