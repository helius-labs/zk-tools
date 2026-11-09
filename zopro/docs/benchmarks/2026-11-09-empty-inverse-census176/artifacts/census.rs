//! Diagnostic metadata only. Reset/snapshot require all observed work joined.
use std::sync::atomic::{AtomicBool,AtomicU64,Ordering::Relaxed};
const WIDTH:usize=16;
struct Counters {rows:[[AtomicU64;WIDTH];2],overflow:AtomicBool}
impl Counters {
 const fn new()->Self{Self{rows:[const{[const{AtomicU64::new(0)};WIDTH]};2],overflow:AtomicBool::new(false)}}
 fn add(&self,curve:usize,column:usize,value:u64){let old=self.rows[curve][column].fetch_add(value,Relaxed);if old.checked_add(value).is_none(){self.overflow.store(true,Relaxed);}}
 fn record(&self,curve:usize,m:usize){
  assert!(curve<2);let m=u64::try_from(m).expect("diagnostic count fits u64");
  self.add(curve,0,1);self.add(curve,if m==0{1}else if m<8{2}else{3},1);
  self.add(curve,4,m);self.add(curve,5,m%8);self.add(curve,6,m/8);
  self.rows[curve][7].fetch_max(m,Relaxed);self.add(curve,8+(m%8)as usize,1);
 }
 fn reset(&self){for row in &self.rows{for v in row{v.store(0,Relaxed);}}self.overflow.store(false,Relaxed);}
 fn snapshot(&self)->([[u64;WIDTH];2],bool){(self.rows.each_ref().map(|r|r.each_ref().map(|v|v.load(Relaxed))),self.overflow.load(Relaxed))}
}
static COUNTERS:Counters=Counters::new();
pub(crate) fn record(base_bytes:usize,m:usize){let curve=match base_bytes{32=>0,64=>1,_=>panic!("unsupported diagnostic base field")};COUNTERS.record(curve,m);}
pub fn reset(){COUNTERS.reset();}
pub fn snapshot()->([[u64;WIDTH];2],bool){COUNTERS.snapshot()}

#[cfg(test)]
mod tests {
 use super::*;use std::sync::Arc;
 fn expected(samples:&[(usize,usize)])->([[u64;16];2],bool){let mut rows=[[0u64;16];2];for &(c,m)in samples{let r=&mut rows[c];r[0]+=1;r[1]+=u64::from(m==0);r[2]+=u64::from((1..8).contains(&m));r[3]+=u64::from(m>=8);r[4]+=m as u64;r[5]+=(m%8)as u64;r[6]+=(m/8)as u64;r[7]=r[7].max(m as u64);r[8+m%8]+=1;}(rows,false)}
 #[test]fn exact_metadata_reset_and_overflow(){
  let c=Counters::new();let mut samples=Vec::new();for curve in 0..2{for m in[0,1,2,7,8,9,15,16,31,32,63,64,128,200,352,1024]{c.record(curve,m);samples.push((curve,m));}}
  assert_eq!(c.snapshot(),expected(&samples));c.reset();assert_eq!(c.snapshot(),([[0;16];2],false));
  c.rows[0][0].store(u64::MAX,Relaxed);c.record(0,0);assert!(c.snapshot().1);c.reset();assert_eq!(c.snapshot(),([[0;16];2],false));
  assert_eq!(std::mem::size_of::<Counters>(),264);println!("EMPTY_INVERSE_COUNTER metadata_cases=32;reset=true;overflow_visible=true;counter_storage_bytes=264");
 }
 #[test]fn actual_workers_and_shared_callers(){
  let c=Arc::new(Counters::new());let mut want=Vec::new();let mut workers=0;
  for n in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(n).stack_size(512*1024).build().unwrap();let seen=std::sync::Mutex::new(std::collections::BTreeSet::new());pool.broadcast(|ctx|{seen.lock().unwrap().insert(ctx.index());for j in 0..128{c.record(j%2,(ctx.index()*37+j*11)%513);}});assert_eq!(seen.into_inner().unwrap().len(),n);for i in 0..n{for j in 0..128{want.push((j%2,(i*37+j*11)%513));}}workers+=n;}
  let pool=rayon::ThreadPoolBuilder::new().num_threads(8).stack_size(512*1024).build().unwrap();std::thread::scope(|scope|{let mut handles=Vec::new();for i in 0..4{let pool=&pool;let c=&c;handles.push(scope.spawn(move||pool.install(||for j in 0..128{c.record(j%2,(i*19+j*13)%513);})));}for h in handles{h.join().unwrap();}});
  for i in 0..4{for j in 0..128{want.push((j%2,(i*19+j*13)%513));}}assert_eq!(workers,53);assert_eq!(c.snapshot(),expected(&want));assert_eq!(want.len(),7296);c.reset();assert_eq!(c.snapshot(),([[0;16];2],false));println!("EMPTY_INVERSE_COUNTER workers=53;concurrent_callers=4;records=7296;joined_reset=true");
 }
}
