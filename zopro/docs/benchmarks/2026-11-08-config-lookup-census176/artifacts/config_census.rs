//! Diagnostic-only environment call ownership; original results are returned.
use std::{alloc::{GlobalAlloc,Layout},cell::Cell,ffi::{OsStr,OsString},sync::atomic::{AtomicBool,Ordering}};
include!("config_sites.rs");
pub const METRICS:[&str;12]=["calls","present","absent","not_unicode","alloc_calls","alloc_bytes","zeroed_calls","zeroed_bytes","realloc_calls","realloc_requested_bytes","returned_len","returned_capacity"];
pub const TOTAL_METRICS:[&str;6]=["alloc_calls","alloc_bytes","zeroed_calls","zeroed_bytes","realloc_calls","realloc_requested_bytes"];
const SLOTS:usize=(SITES.len()+NAMES.len())*METRICS.len();const NONE:usize=usize::MAX;
static ENABLED:AtomicBool=AtomicBool::new(false);
struct State{site:Cell<usize>,name:Cell<usize>,counts:[Cell<u64>;SLOTS],totals:[Cell<u64>;6]}
impl State{const fn new()->Self{Self{site:Cell::new(NONE),name:Cell::new(NONE),counts:[const{Cell::new(0)};SLOTS],totals:[const{Cell::new(0)};6]}}fn add(&self,site:usize,name:usize,k:usize,n:u64){for p in[site,SITES.len()+name]{let c=&self.counts[p*METRICS.len()+k];c.set(c.get()+n);}}}
std::thread_local!{static STATE:State=const{State::new()};}
struct Scope{old_site:usize,old_name:usize,site:usize,name:usize}
impl Drop for Scope{fn drop(&mut self){let _=STATE.try_with(|s|{s.site.set(self.old_site);s.name.set(self.old_name);});}}
fn begin(site:usize,key:&OsStr)->Option<Scope>{if !ENABLED.load(Ordering::Relaxed){return None;}assert!(site<SITES.len());let name=key.to_str().and_then(|key|NAMES.iter().position(|&name|name==key)).unwrap_or(0);STATE.with(|s|{s.add(site,name,0,1);Some(Scope{old_site:s.site.replace(site),old_name:s.name.replace(name),site,name})})}
fn result(scope:&Option<Scope>,status:usize,len:usize,capacity:usize){if let Some(g)=scope{STATE.with(|s|{s.add(g.site,g.name,status,1);s.add(g.site,g.name,10,len as u64);s.add(g.site,g.name,11,capacity as u64);});}}
pub fn var_at<const SITE:usize,S:AsRef<OsStr>>(key:S)->Result<String,std::env::VarError>{let scope=begin(SITE,key.as_ref());let out=std::env::var(key);match &out{Ok(s)=>result(&scope,1,s.len(),s.capacity()),Err(std::env::VarError::NotPresent)=>result(&scope,2,0,0),Err(std::env::VarError::NotUnicode(s))=>result(&scope,3,s.len(),s.capacity())}out}
pub fn var_os_at<const SITE:usize,S:AsRef<OsStr>>(key:S)->Option<OsString>{let scope=begin(SITE,key.as_ref());let out=std::env::var_os(key);if let Some(s)=&out{result(&scope,1,s.len(),s.capacity())}else{result(&scope,2,0,0)}out}
pub fn enable(on:bool){ENABLED.store(on,Ordering::Relaxed);}
pub fn clear_thread(){assert!(!ENABLED.load(Ordering::Relaxed));STATE.with(|s|{assert_eq!(s.site.get(),NONE);for c in &s.counts{c.set(0);}for c in &s.totals{c.set(0);}});}
pub fn take_thread()->(Vec<u64>,[u64;6]){assert!(!ENABLED.load(Ordering::Relaxed));STATE.with(|s|{assert_eq!(s.site.get(),NONE);(s.counts.iter().map(Cell::get).collect(),std::array::from_fn(|i|s.totals[i].get()))})}
fn allocated(kind:usize,n:usize){if !ENABLED.load(Ordering::Relaxed){return;}let _=STATE.try_with(|s|{let k=kind*2;s.totals[k].set(s.totals[k].get()+1);s.totals[k+1].set(s.totals[k+1].get()+n as u64);let site=s.site.get();if site!=NONE{s.add(site,s.name.get(),4+k,1);s.add(site,s.name.get(),5+k,n as u64);}});}
pub struct TrackingAllocator;
unsafe impl GlobalAlloc for TrackingAllocator{
 unsafe fn alloc(&self,l:Layout)->*mut u8{let p=unsafe{crate::alloc::Allocator.alloc(l)};if !p.is_null(){allocated(0,l.size());}p}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{let p=unsafe{crate::alloc::Allocator.alloc_zeroed(l)};if !p.is_null(){allocated(1,l.size());}p}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{crate::alloc::Allocator.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{let p=unsafe{crate::alloc::Allocator.realloc(p,l,n)};if !p.is_null(){allocated(2,n);}p}
}
pub const COUNTER_PAYLOAD_BYTES_PER_THREAD:usize=SLOTS*8+6*8+2*core::mem::size_of::<usize>();
#[cfg(test)]mod tests{
 use super::*;use std::os::unix::ffi::OsStringExt;
 #[global_allocator]static ALLOCATOR:TrackingAllocator=TrackingAllocator;
 #[test]fn environment_results_allocator_scope_reset_and_thread_isolation(){
  let key="ZOPRO_CONFIG_CENSUS_TEST_VALUE";let saved=std::env::var_os(key);enable(false);unsafe{std::env::remove_var(key);}clear_thread();enable(true);assert!(matches!(var_at::<0,_>(key),Err(std::env::VarError::NotPresent)));unsafe{std::env::set_var(key,"1");}assert_eq!(var_at::<0,_>(key).unwrap(),"1");unsafe{std::env::set_var(key,"");}assert_eq!(var_at::<0,_>(key).unwrap(),"");unsafe{std::env::set_var(key,OsString::from_vec(vec![0xff,0xfe]));}assert!(matches!(var_at::<0,_>(key),Err(std::env::VarError::NotUnicode(_))));enable(false);let (counts,totals)=take_thread();assert_eq!(counts[0],4);assert_eq!(counts[1],2);assert_eq!(counts[2],1);assert_eq!(counts[3],1);assert!(counts[4]>=2);assert_eq!(counts[5],3);assert!(totals[0]>=counts[4]);
  unsafe{std::env::set_var(key,"0");}clear_thread();enable(true);let thread=std::thread::spawn(move||{let out=var_os_at::<0,_>(key).unwrap();assert_eq!(out,"0");STATE.with(|s|{assert_eq!(s.counts[0].get(),1);assert!(s.counts[4].get()>=1);assert_eq!(s.site.get(),NONE);});});thread.join().unwrap();enable(false);let(main,_)=take_thread();assert_eq!(main[0],0);assert_eq!(main[4],0);
  clear_thread();let _=var_at::<0,_>(key).unwrap();assert_eq!(take_thread().0[0],0);match saved{Some(s)=>unsafe{std::env::set_var(key,s)},None=>unsafe{std::env::remove_var(key)}};
 }
}
