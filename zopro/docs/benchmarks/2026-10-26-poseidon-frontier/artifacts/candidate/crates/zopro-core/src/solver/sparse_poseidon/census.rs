//! Diagnostic metadata only; never changes the prepared schedule.
use super::*;
use std::sync::atomic::{AtomicUsize,Ordering};
static NEXT:AtomicUsize=AtomicUsize::new(0);
#[derive(Clone,Debug,Default)]
struct Stats {tasks:usize,tagged:usize,max_chain:usize,total:[usize;14],full4:[usize;14],full8:[usize;14],max_ready:[usize;14]}
struct Hashes<'a>{task:&'a[u32],pos:usize}
impl<'a> Hashes<'a>{fn new(ops:usize,task:&'a[u32])->Self{Self{task:if tagged_task(ops,task.first().copied()){task}else{&[]},pos:0}}}
impl Iterator for Hashes<'_>{type Item=usize;fn next(&mut self)->Option<usize>{
 while self.pos<self.task.len(){let t=(self.task[self.pos]>>28)as usize;if t==0{self.pos+=1;continue;}
  assert!((2..=13).contains(&t),"unsupported recognized width");let(partial,_)=parameters(t);
  self.pos=self.pos.checked_add(3*(7*t+partial)).expect("span length");assert!(self.pos<=self.task.len(),"truncated recognized span");return Some(t);
 }None
}}
fn wave<'a>(ops:usize,tasks:impl Iterator<Item=&'a[u32]>+Clone)->Stats{
 let mut s=Stats::default();for task in tasks.clone(){s.tasks+=1;if tagged_task(ops,task.first().copied()){s.tagged+=1;}let mut chain=0;for t in Hashes::new(ops,task){s.total[t]+=1;chain+=1;}s.max_chain=s.max_chain.max(chain);}
 for depth in 0..s.max_chain{let mut ready=[0usize;14];for task in tasks.clone(){if let Some(t)=Hashes::new(ops,task).nth(depth){ready[t]+=1;}}
  for t in 2..=13{s.full4[t]+=ready[t]/4*4;s.full8[t]+=ready[t]/8*8;s.max_ready[t]=s.max_ready[t].max(ready[t]);}
 }s
}
fn owner(ops:usize,task:&[u32])->usize{let code=task.first().copied().expect("nonempty scheduled task");if ops<=INDEX as usize{(code&INDEX)as usize}else{code as usize}}
fn visit(ops:usize,schedule:&Schedule,mut output:impl FnMut(&str,usize,usize,usize,&Stats))->Stats{
 let mut sum=Stats::default();let mut containers=0;
 let mut add=|scope:&str,epoch:usize,key:usize,index:usize,s:&Stats|{output(scope,epoch,key,index,s);sum.tasks+=s.tasks;sum.tagged+=s.tagged;sum.max_chain=sum.max_chain.max(s.max_chain);for t in 2..=13{sum.total[t]+=s.total[t];sum.full4[t]+=s.full4[t];sum.full8[t]+=s.full8[t];sum.max_ready[t]=sum.max_ready[t].max(s.max_ready[t]);}};
 for(e,epoch)in schedule.epochs.iter().enumerate(){let plain=epoch.iter().filter(|task|!schedule.refined.contains_key(&owner(ops,task))).map(Vec::as_slice);add("epoch",e,0,0,&wave(ops,plain));
  for task in epoch{let key=owner(ops,task);if let Some(waves)=schedule.refined.get(&key){containers+=1;for(w,leaves)in waves.iter().enumerate(){add("refined",e,key,w,&wave(ops,leaves.iter().map(Vec::as_slice)));}}}
 }assert_eq!(containers,schedule.refined.len(),"orphaned/duplicate refined container");sum
}
fn line(id:usize,scope:&str,epoch:usize,key:usize,index:usize,s:&Stats){
 eprintln!("POSEIDON_FRONTIER {{\"id\":{},\"scope\":\"{}\",\"epoch\":{},\"owner\":{},\"wave\":{},\"tasks\":{},\"tagged_tasks\":{},\"max_chain\":{},\"hashes\":{:?},\"full4_hashes\":{:?},\"full8_hashes\":{:?},\"max_abstract_frontier\":{:?}}}",id,scope,epoch,key,index,s.tasks,s.tagged,s.max_chain,s.total,s.full4,s.full8,s.max_ready);
}
pub(crate) fn emit(plan:&Plan,schedule:&Schedule){
 if std::env::var("ZOPRO_CPU_POSEIDON_FRONTIER").as_deref()!=Ok("1"){return;}
 let id=NEXT.fetch_add(1,Ordering::Relaxed);
 eprintln!("POSEIDON_FRONTIER_BEGIN {{\"id\":{},\"workers\":{},\"ops\":{},\"constraints\":{},\"wires\":{},\"slots\":{},\"epochs\":{},\"refined_containers\":{},\"encoding_eligible\":{}}}",id,rayon::current_num_threads(),plan.ops.len(),plan.n_constraints,plan.n_wires,plan.n_slots,schedule.epochs.len(),schedule.refined.len(),plan.ops.len()<=INDEX as usize);
 let total=visit(plan.ops.len(),schedule,|scope,e,key,w,s|line(id,scope,e,key,w,s));line(id,"total",0,0,0,&total);
}
#[cfg(test)]mod tests{
 use super::*;
 fn task(widths:&[usize],id:u32)->Vec<u32>{let mut v=vec![TASK|id];for&t in widths{let(p,_)=parameters(t);let start=v.len();v.resize(start+3*(7*t+p),0);v[start]=(t as u32)<<28|id;}v}
 #[test]fn encoding_guard_and_malformed_spans(){assert_eq!(Hashes::new(INDEX as usize+1,&[TASK|(13<<28)]).count(),0);assert_eq!(Hashes::new(99,&[0,3<<28]).count(),0);assert_eq!(Hashes::new(99,&[]).count(),0);assert!(std::panic::catch_unwind(||Hashes::new(99,&[TASK|(3<<28)]).count()).is_err());assert!(std::panic::catch_unwind(||Hashes::new(99,&[TASK|(1<<28)]).count()).is_err());}
 #[test]fn one_chain_does_not_supply_independent_lanes(){let tasks=vec![task(&[3;8],0)];let s=wave(99,tasks.iter().map(Vec::as_slice));assert_eq!(s.total[3],8);assert_eq!(s.full4[3],0);assert_eq!(s.full8[3],0);assert_eq!(s.max_ready[3],1);let tasks:Vec<_>=(0..8).map(|i|task(&[3],i)).collect();let s=wave(99,tasks.iter().map(Vec::as_slice));assert_eq!(s.total[3],8);assert_eq!(s.full8[3],8);assert_eq!(s.max_ready[3],8);}
 #[test]fn mixed_widths_and_chain_tails(){let mut tasks=Vec::new();for i in 0..9{tasks.push(task(if i<8{&[3,4]}else{&[3]},i));}let s=wave(99,tasks.iter().map(Vec::as_slice));assert_eq!(s.total[3],9);assert_eq!(s.total[4],8);assert_eq!(s.full8[3],8);assert_eq!(s.full8[4],8);assert_eq!(s.max_chain,2);let empty:Vec<Vec<u32>>=vec![];assert_eq!(wave(99,empty.iter().map(Vec::as_slice)).tasks,0);}
 #[test]fn refined_container_is_excluded_and_waves_stay_separate(){let mut schedule=Schedule{epochs:vec![vec![task(&[3;8],5),task(&[4],7)]],refined:FxMap::default(),_reservation:None};schedule.refined.insert(5,vec![vec![task(&[3],1);4],vec![task(&[3],2);4]]);let mut scopes=0;let total=visit(99,&schedule,|_,_,_,_,_|scopes+=1);assert_eq!(scopes,3);assert_eq!(total.total[3],8);assert_eq!(total.total[4],1);assert_eq!(total.full4[3],8);assert_eq!(total.full8[3],0);assert_eq!(total.tasks,9);}
}
