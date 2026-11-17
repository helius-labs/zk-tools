//! Diagnostic-only, one sequential SDK request at a time. No production API.
use std::{sync::{OnceLock,atomic::{AtomicU64,Ordering}},time::Instant};
static EVENTS:[AtomicU64;24]=[const{AtomicU64::new(0)};24];
static ENABLED:OnceLock<bool>=OnceLock::new();static ORIGIN:OnceLock<Instant>=OnceLock::new();
fn enabled()->bool{*ENABLED.get_or_init(||std::env::var("ZOPRO_DIAGNOSTIC_PHASES").as_deref()==Ok("1"))}
pub fn begin(){if enabled(){ORIGIN.get_or_init(Instant::now);for e in &EVENTS{e.store(0,Ordering::Relaxed);}mark(23);}}
pub(crate)fn mark(i:usize){if enabled(){let t=ORIGIN.get_or_init(Instant::now).elapsed().as_nanos().try_into().unwrap();EVENTS[i].store(t,Ordering::Relaxed);}}
pub(crate)fn around<T>(first:usize,last:usize,f:impl FnOnce()->T)->T{mark(first);let result=f();mark(last);result}
pub fn snapshot()->[u64;24]{std::array::from_fn(|i|EVENTS[i].load(Ordering::Relaxed))}
#[cfg(test)]mod tests{use super::*;#[test]fn bounded_event_order_and_reset(){begin();mark(0);around(4,5,||{});let a=snapshot();if enabled(){assert!(a[23]>0&&a[0]>=a[23]&&a[4]>=a[0]&&a[5]>=a[4]);begin();let b=snapshot();assert!(b[23]>=a[5]);assert!(b[..23].iter().all(|&x|x==0));}else{assert_eq!(a,[0;24]);}}}
