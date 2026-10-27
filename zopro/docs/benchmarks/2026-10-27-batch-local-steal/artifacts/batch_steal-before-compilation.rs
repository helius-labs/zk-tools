//! Safe adapter to the existing Crossbeam batch-transfer operation.
use crossbeam_deque::{Steal,Stealer,Worker};

#[inline]
pub(super) fn transfer<T>(victim:&Stealer<T>,local:&Worker<T>)->(Steal<T>,usize){
    let job=victim.steal_batch_and_pop(local);
    let available=if job.is_success(){local.len()}else{0};
    (job,available)
}

#[cfg(test)]
mod tests{
 use super::*;
 use std::sync::{Arc,atomic::{AtomicBool,AtomicUsize,Ordering}};
 #[test]
 fn transfer_uses_existing_batch_bound_and_preserves_every_item(){
  for fifo in [false,true]{let source=if fifo{Worker::new_fifo()}else{Worker::new_lifo()};for i in 0..64{source.push(i)}let local=Worker::new_lifo();let(job,remaining)=transfer(&source.stealer(),&local);assert!(remaining>0&&remaining<32);let first=job.success().unwrap();let mut values=vec![first];while let Some(v)=local.pop(){values.push(v)}while let Some(v)=source.pop(){values.push(v)}values.sort_unstable();assert_eq!(values,(0..64).collect::<Vec<_>>());}
 }
 #[test]
 fn concurrent_transfers_deliver_each_item_once(){
  for thieves in [1,2,8]{let count=32768;let source=Worker::new_lifo();let done=Arc::new(AtomicBool::new(false));let seen=Arc::new((0..count).map(|_|AtomicUsize::new(0)).collect::<Vec<_>>());let mut handles=Vec::new();for _ in 0..thieves{let s=source.stealer();let done=done.clone();let seen=seen.clone();handles.push(std::thread::spawn(move||{let local=Worker::new_lifo();let started=std::time::Instant::now();loop{assert!(started.elapsed().as_secs()<30,"transfer test stalled");if let Some(i)=local.pop(){assert_eq!(seen[i].fetch_add(1,Ordering::Relaxed),0);continue}match transfer(&s,&local).0{Steal::Success(i)=>{assert_eq!(seen[i].fetch_add(1,Ordering::Relaxed),0);},Steal::Retry=>{},Steal::Empty=>{if done.load(Ordering::Acquire)&&s.is_empty(){break}std::thread::yield_now();}}}}));}for i in 0..count{source.push(i)}done.store(true,Ordering::Release);for h in handles{h.join().unwrap()}assert!(source.is_empty());for n in seen.iter(){assert_eq!(n.load(Ordering::Relaxed),1);}}
 }
 #[test]
 fn nested_scopes_panics_and_pool_reuse(){
  for workers in [1,2,4,8]{let pool=crate::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();assert!(pool.install(||crate::registry::WorkerThread::with(|w|w.unwrap().registry().batch_local_steal)));let count=4096;let seen=(0..count).map(|_|AtomicUsize::new(0)).collect::<Vec<_>>();pool.scope(|s|{for i in 0..count{s.spawn(|_|{crate::join(||{seen[i].fetch_add(1,Ordering::Relaxed);},||{crate::scope_fifo(|f|f.spawn_fifo(|_|{}));});});}});assert!(seen.iter().all(|v|v.load(Ordering::Relaxed)==1));assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||pool.install(||crate::join(||panic!("expected panic"),||{})))).is_err());assert_eq!(pool.install(||crate::join(||17,||25)),(17,25));}
 }
 #[test]
 fn concurrent_callers_and_fifo_pools_complete(){
  for workers in [1,4]{#[allow(deprecated)]let pool=Arc::new(crate::ThreadPoolBuilder::new().num_threads(workers).breadth_first().build().unwrap());let mut handles=Vec::new();for caller in 0..4{let pool=pool.clone();handles.push(std::thread::spawn(move||pool.install(||{let seen=Arc::new(AtomicUsize::new(0));crate::scope(|s|for i in 0..1024{let seen=seen.clone();s.spawn(move |_|{seen.fetch_add(i+caller,Ordering::Relaxed);});});assert_eq!(seen.load(Ordering::Relaxed),(0..1024).map(|i|i+caller).sum());})));}for h in handles{h.join().unwrap()}}
 }
}
