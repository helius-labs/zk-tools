use std::{alloc::{GlobalAlloc,Layout},sync::atomic::{AtomicUsize,Ordering::Relaxed},time::Instant,hint::black_box};
use zopro_prover::zopro_core::{self,PoseidonBatchProbe};use serde_json::json;
#[path="support/perf.rs"]mod perf;
static CALLS:AtomicUsize=AtomicUsize::new(0);
struct Counted;
unsafe impl GlobalAlloc for Counted{
 unsafe fn alloc(&self,l:Layout)->*mut u8{CALLS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.alloc(l)}}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{CALLS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.alloc_zeroed(l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{CALLS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.realloc(p,l,n)}}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{zopro_core::alloc::Allocator.dealloc(p,l)}}
}
#[global_allocator]static ALLOCATOR:Counted=Counted;
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<_>=std::env::args().collect();assert_eq!(a.len(),7);let groups:usize=a[1].parse()?;let c=a[2]=="1";let reverse=a[3]=="1";let ragged=a[4]=="1";let mode=match a[5].as_str(){"control-a"|"control-b"=>0,"batch"=>1,_=>panic!("mode")};let rounds:usize=a[6].parse()?;assert!(rounds>0);let mut control=perf::Control::from_env()?;
 let mut probe=PoseidonBatchProbe::new(groups,reverse,c,ragged);probe.reset(0);probe.run(2)?;let expected=probe.snapshot();probe.reset(0);for _ in 0..3{black_box(&mut probe).run(mode)?;}assert_eq!(probe.snapshot(),expected);
 if let Some(p)=control.as_mut(){p.enable()?;}let usage0=perf::Usage::now()?;let allocations=CALLS.load(Relaxed);let start=Instant::now();for _ in 0..rounds{black_box(&mut probe).run(mode)?;}let elapsed=start.elapsed().as_secs_f64()*1000.;let calls=CALLS.load(Relaxed)-allocations;let usage1=perf::Usage::now()?;if let Some(p)=control.as_mut(){p.disable()?;}
 assert_eq!(probe.snapshot(),expected);assert_eq!(calls,0,"no timed allocation");let mut out=usage1.difference(&usage0);out["groups"]=json!(groups);out["materialize_c"]=json!(c);out["reverse"]=json!(reverse);out["ragged"]=json!(ragged);out["mode"]=json!(a[5]);out["iterations"]=json!(rounds);out["primes"]=json!(3);out["hashes_per_iteration"]=json!(probe.hashes());out["ms"]=json!(elapsed);out["ns_per_hash"]=json!(elapsed*1e6/(rounds*probe.hashes())as f64);out["buffer_elements_checked"]=json!(probe.buffer_elements());out["timed_allocation_calls"]=json!(calls);out["all_outputs_match_interpreter"]=json!(true);println!("SUMMARY {out}");Ok(())
}
