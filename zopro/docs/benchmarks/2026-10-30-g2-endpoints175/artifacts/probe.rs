use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{alloc::{GlobalAlloc,Layout},fs,hint::black_box,sync::{atomic::{AtomicUsize,Ordering::Relaxed},Arc,Barrier},time::Instant};
use zopro_core::g2_replay::{Fixture,Runner};
#[path="perf.rs"]mod perf;
struct Counted;static ALLOCS:AtomicUsize=AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counted{
 unsafe fn alloc(&self,l:Layout)->*mut u8{ALLOCS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.alloc(l)}}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{ALLOCS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.alloc_zeroed(l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{ALLOCS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.realloc(p,l,n)}}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{zopro_core::alloc::Allocator.dealloc(p,l)}}
}
#[global_allocator]static A:Counted=Counted;
fn affinity(cpu:usize){unsafe{let mut mask:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut mask);libc::CPU_SET(cpu,&mut mask);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask),0);let mut got:libc::cpu_set_t=std::mem::zeroed();assert_eq!(libc::sched_getaffinity(0,std::mem::size_of_val(&got),&mut got),0);for n in 0..libc::CPU_SETSIZE as usize{assert_eq!(libc::CPU_ISSET(n,&got),n==cpu)}assert_eq!(libc::sched_getcpu(),cpu as i32);}}
fn cpu_now()->u64{unsafe{let mut t:libc::timespec=std::mem::zeroed();assert_eq!(libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID,&mut t),0);t.tv_sec as u64*1_000_000_000+t.tv_nsec as u64}}
struct Gates{ready:Barrier,start:Barrier,done:Barrier,release:Barrier}
fn run_rounds(runners:&mut[Runner<'_>],rounds:usize){for _ in 0..rounds{for runner in runners.iter_mut(){black_box(&mut *runner).run(1);}}}
fn main(){
 assert!(std::env::var_os("ZOPRO_G2_CAPTURE_DIR").is_none());let args=std::env::args().collect::<Vec<_>>();assert_eq!(args.len(),5);let case:Value=serde_json::from_slice(&fs::read(&args[1]).unwrap()).unwrap();let workers:usize=args[2].parse().unwrap();let rounds:usize=args[3].parse().unwrap();let prime:usize=args[4].parse().unwrap();assert!([1,16,32].contains(&workers)&&rounds>0&&prime>0);
 let mut file_pins=vec![];let input=case["fixtures"].as_array().unwrap().iter().map(|f|{let p=f["path"].as_str().unwrap();let b=fs::read(p).unwrap();let h=format!("{:x}",Sha256::digest(&b));assert_eq!(h,f["sha256"]);file_pins.push(json!({"path":p,"sha256":h}));b}).collect::<Vec<_>>();assert!(!input.is_empty()&&input.len()<=40);let input=Arc::new(input);
 let gates=Arc::new(Gates{ready:Barrier::new(workers+1),start:Barrier::new(workers+1),done:Barrier::new(workers+1),release:Barrier::new(workers+1)});let mut control=perf::Control::from_env().unwrap();let mut handles=vec![];
 for worker in 0..workers{let input=input.clone();let gates=gates.clone();handles.push(std::thread::spawn(move||{
  let cpu=if workers==1{2}else{worker};affinity(cpu);let tid=unsafe{libc::syscall(libc::SYS_gettid)};
  let mut fixtures=input.iter().map(|b|Fixture::decode(b).unwrap()).collect::<Vec<_>>();let mut runners=fixtures.iter_mut().map(Fixture::runner).collect::<Vec<_>>();for _ in 0..3{run_rounds(&mut runners,prime);for r in &runners{r.check();}}
  let capacities=runners.iter().map(Runner::capacity_bytes).collect::<Vec<_>>();gates.ready.wait();gates.start.wait();let before=cpu_now();let start=Instant::now();run_rounds(&mut runners,rounds);let elapsed=start.elapsed().as_nanos()as u64;let cpu_ns=cpu_now()-before;gates.done.wait();gates.release.wait();
  assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);let mut digest=Sha256::new();for(r,capacity)in runners.iter().zip(&capacities){r.check();assert_eq!(r.capacity_bytes(),*capacity);digest.update(r.bytes());}
  json!({"cpu":cpu,"tid":tid,"elapsed_ns":elapsed,"CPU_ns":cpu_ns,"capacities":capacities,"digest":format!("{:x}",digest.finalize())})
 }));}
 gates.ready.wait();if let Some(c)=&mut control{c.enable().unwrap();}let before=perf::Usage::now().unwrap();let alloc0=ALLOCS.load(Relaxed);let clock=Instant::now();gates.start.wait();gates.done.wait();let elapsed=clock.elapsed().as_nanos()as u64;let allocations=ALLOCS.load(Relaxed)-alloc0;let after=perf::Usage::now().unwrap();if let Some(c)=&mut control{c.disable().unwrap();}gates.release.wait();let worker_rows=handles.into_iter().map(|h|h.join().unwrap()).collect::<Vec<_>>();assert_eq!(allocations,0);for w in &worker_rows{assert_eq!(w["digest"],worker_rows[0]["digest"]);assert_eq!(w["capacities"],worker_rows[0]["capacities"]);}
 let calls=rounds*input.len();println!("RESULT {}",json!({"case":case["name"],"workers":workers,"rounds":rounds,"prime_rounds":prime,"primes":3,"fixtures":input.len(),"calls_per_worker":calls,"all_calls_including_primes":(rounds+3*prime)*input.len()*workers,"wave_ns":elapsed,"ns_per_batch_wave":elapsed as f64/calls as f64,"timed_allocations":allocations,"usage":after.difference(&before),"worker_rows":worker_rows,"input_pins":file_pins,"outputs_match_captured_and_inherited_Arkworks_oracles":true}));
}
