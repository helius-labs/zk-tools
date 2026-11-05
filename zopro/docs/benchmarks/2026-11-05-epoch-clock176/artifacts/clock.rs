//! Diagnostic elapsed-time snapshots; enabled only by an explicit driver call.
#[derive(Debug)]
pub struct Snapshot{pub ops:usize,pub expected_epochs:usize,pub kind:u64,pub elapsed_ns:Vec<u64>,pub complete:bool}
pub fn supported()->bool{cfg!(zopro_epoch_clock)}
pub fn configure(enabled:bool,reserve:usize){
 #[cfg(zopro_epoch_clock)]implementation::configure(enabled,reserve);
 #[cfg(not(zopro_epoch_clock))]let _=(enabled,reserve);
}
pub fn take()->Vec<Snapshot>{
 #[cfg(zopro_epoch_clock)]{implementation::take()}
 #[cfg(not(zopro_epoch_clock))]{Vec::new()}
}
#[cfg(zopro_epoch_clock)]pub(crate)use implementation::Clock;
#[cfg(zopro_epoch_clock)]mod implementation{
 use super::Snapshot;use std::{sync::{Mutex,atomic::{AtomicBool,Ordering}},time::Instant};
 static ENABLED:AtomicBool=AtomicBool::new(false);static RECORDS:Mutex<Vec<Snapshot>>=Mutex::new(Vec::new());
 pub(super)fn configure(enabled:bool,reserve:usize){ENABLED.store(false,Ordering::Release);let mut records=RECORDS.lock().unwrap();records.clear();records.reserve(reserve);drop(records);ENABLED.store(enabled,Ordering::Release);}
 pub(super)fn take()->Vec<Snapshot>{ENABLED.store(false,Ordering::Release);std::mem::take(&mut*RECORDS.lock().unwrap())}
 struct Active{ops:usize,expected:usize,kind:u64,started:Instant,marks:Vec<u64>}
 pub(crate)struct Clock(Option<Active>);
 impl Clock{
  #[inline]pub(crate)fn begin(ops:usize,epochs:usize,kind:u64)->Self{if !ENABLED.load(Ordering::Acquire){return Self(None);}let mut marks=Vec::with_capacity(epochs.saturating_add(1));marks.push(0);Self(Some(Active{ops,expected:epochs,kind,started:Instant::now(),marks}))}
  #[inline]pub(crate)fn mark(&mut self){if let Some(a)=self.0.as_mut(){a.marks.push(a.started.elapsed().as_nanos().try_into().expect("epoch duration u64"));}}
  pub(crate)fn finish(self){if let Some(a)=self.0{let complete=a.marks.len()==a.expected+1;RECORDS.lock().unwrap().push(Snapshot{ops:a.ops,expected_epochs:a.expected,kind:a.kind,elapsed_ns:a.marks,complete});}}
 }
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn disabled_recording_is_empty(){configure(false,0);#[cfg(zopro_epoch_clock)]{let mut c=Clock::begin(7,2,0);c.mark();c.finish();}assert!(take().is_empty());}
 #[test]fn complete_epochs_keep_order_and_shape(){configure(true,1);#[cfg(zopro_epoch_clock)]{let mut c=Clock::begin(7,2,0);c.mark();c.mark();c.finish();}let r=take();if supported(){assert_eq!(r.len(),1);assert_eq!(r[0].ops,7);assert_eq!(r[0].expected_epochs,2);assert!(r[0].complete);assert_eq!(r[0].elapsed_ns.len(),3);assert_eq!(r[0].elapsed_ns[0],0);assert!(r[0].elapsed_ns.windows(2).all(|p|p[0]<=p[1]));}else{assert!(r.is_empty());}}
 #[test]fn incomplete_epochs_are_not_reported_complete(){configure(true,1);#[cfg(zopro_epoch_clock)]{let mut c=Clock::begin(9,3,1);c.mark();c.finish();}let r=take();if supported(){assert_eq!(r.len(),1);assert!(!r[0].complete);assert_eq!(r[0].kind,1);assert_eq!(r[0].elapsed_ns.len(),2);}else{assert!(r.is_empty());}}
}
