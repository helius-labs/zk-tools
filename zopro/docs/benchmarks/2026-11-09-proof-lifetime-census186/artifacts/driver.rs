//! Instrumented lifetime diagnostic. Its durations are not latency benchmarks.
use std::{alloc::{GlobalAlloc,Layout},fs,io::{self,Read},path::Path,sync::{Arc,atomic::{AtomicUsize,Ordering}},time::Instant};
static BASE:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
static LIVE:AtomicUsize=AtomicUsize::new(0);static PEAK:AtomicUsize=AtomicUsize::new(0);static VOLUME:AtomicUsize=AtomicUsize::new(0);static ALLOCS:AtomicUsize=AtomicUsize::new(0);static FREES:AtomicUsize=AtomicUsize::new(0);static REALLOCS:AtomicUsize=AtomicUsize::new(0);static FAILURES:AtomicUsize=AtomicUsize::new(0);
struct Track;
#[global_allocator]static TRACK:Track=Track;
fn add(counter:&AtomicUsize,n:usize)->usize{match counter.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|v|v.checked_add(n)){Ok(v)=>v+n,Err(_)=>std::process::abort()}}
fn sub(counter:&AtomicUsize,n:usize){if counter.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|v|v.checked_sub(n)).is_err(){std::process::abort();}}
fn allocated(p:*mut u8,size:usize){if p.is_null(){add(&FAILURES,1);}else{add(&ALLOCS,1);add(&VOLUME,size);let live=add(&LIVE,size);PEAK.fetch_max(live,Ordering::Relaxed);}}
// SAFETY: forwarding preserves the exact backend pointers/layouts. Accounting
// uses only allocation-free atomics; failed alloc/realloc never changes LIVE.
unsafe impl GlobalAlloc for Track{
 unsafe fn alloc(&self,l:Layout)->*mut u8{let p=unsafe{BASE.alloc(l)};allocated(p,l.size());p}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{let p=unsafe{BASE.alloc_zeroed(l)};allocated(p,l.size());p}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){sub(&LIVE,l.size());add(&FREES,1);unsafe{BASE.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,size:usize)->*mut u8{let next=unsafe{BASE.realloc(p,l,size)};if next.is_null(){add(&FAILURES,1);}else{add(&REALLOCS,1);add(&VOLUME,size);if size>=l.size(){let live=add(&LIVE,size-l.size());PEAK.fetch_max(live,Ordering::Relaxed);}else{sub(&LIVE,l.size()-size);}}next}
}
#[derive(Clone,Copy,Default)]struct Counts{live:usize,peak:usize,volume:usize,allocs:usize,frees:usize,reallocs:usize,failures:usize}
fn counts()->Counts{Counts{live:LIVE.load(Ordering::Relaxed),peak:PEAK.load(Ordering::Relaxed),volume:VOLUME.load(Ordering::Relaxed),allocs:ALLOCS.load(Ordering::Relaxed),frees:FREES.load(Ordering::Relaxed),reallocs:REALLOCS.load(Ordering::Relaxed),failures:FAILURES.load(Ordering::Relaxed)}}
impl Counts{fn json(self)->serde_json::Value{serde_json::json!({"live_requested_bytes":self.live,"cumulative_requested_highwater_bytes":self.peak,"cumulative_requested_volume_bytes":self.volume,"successful_allocations":self.allocs,"frees":self.frees,"successful_reallocations":self.reallocs,"failures":self.failures})}}
#[repr(C)]#[derive(Clone,Copy,Default)]struct Heap{arena:usize,ordblks:usize,smblks:usize,hblks:usize,hblkhd:usize,usmblks:usize,fsmblks:usize,uordblks:usize,fordblks:usize,keepcost:usize}
unsafe extern "C"{fn mallinfo2()->Heap;}
#[derive(Clone,Copy,Default)]struct Resident{rss_kib:u64,hwm_kib:u64,anon_kib:u64,file_kib:u64,vm_kib:u64,threads:u64}
fn resident()->io::Result<Resident>{let mut file=fs::File::open("/proc/self/status")?;let mut buf=[0u8;16384];let n=file.read(&mut buf)?;if n==buf.len(){return Err(io::Error::other("status buffer exhausted"));}let text=std::str::from_utf8(&buf[..n]).map_err(io::Error::other)?;let mut r=Resident::default();for line in text.lines(){let Some((key,value))=line.split_once(':')else{continue};let target=match key{"VmRSS"=>&mut r.rss_kib,"VmHWM"=>&mut r.hwm_kib,"RssAnon"=>&mut r.anon_kib,"RssFile"=>&mut r.file_kib,"VmSize"=>&mut r.vm_kib,"Threads"=>&mut r.threads,_=>continue};*target=value.split_whitespace().next().unwrap().parse().map_err(io::Error::other)?;}Ok(r)}
#[derive(Clone,Copy)]struct Usage(libc::rusage);
fn usage()->io::Result<Usage>{let mut u=std::mem::MaybeUninit::<libc::rusage>::uninit();if unsafe{libc::getrusage(libc::RUSAGE_SELF,u.as_mut_ptr())}!=0{return Err(io::Error::last_os_error());}Ok(Usage(unsafe{u.assume_init()}))}
fn ms(t:libc::timeval)->f64{t.tv_sec as f64*1000.+t.tv_usec as f64/1000.}
#[derive(Clone,Copy)]struct Frame{stage:&'static str,index:usize,c:Counts,heap:Heap,resident:Resident,usage:Usage}
fn frame(stage:&'static str,index:usize)->io::Result<Frame>{let c=counts();let heap=unsafe{mallinfo2()};let resident=resident()?;let usage=usage()?;Ok(Frame{stage,index,c,heap,resident,usage})}
impl Frame{fn json(self)->serde_json::Value{serde_json::json!({"stage":self.stage,"index":self.index,"counts":self.c.json(),"glibc":{"arena_bytes":self.heap.arena,"inuse_bytes":self.heap.uordblks,"free_bytes":self.heap.fordblks,"mmap_bytes":self.heap.hblkhd,"mmap_regions":self.heap.hblks,"fastbin_free_bytes":self.heap.fsmblks,"top_releasable_bytes":self.heap.keepcost},"linux":{"RSS_KiB":self.resident.rss_kib,"HWM_KiB":self.resident.hwm_kib,"anonymous_KiB":self.resident.anon_kib,"file_KiB":self.resident.file_kib,"VM_KiB":self.resident.vm_kib,"threads":self.resident.threads,"rusage_HWM_KiB":self.usage.0.ru_maxrss},"cumulative_CPU_ms":ms(self.usage.0.ru_utime)+ms(self.usage.0.ru_stime)})}}
#[derive(Clone,Copy)]struct Request{index:usize,wall_ms:f64,before:Usage,after:Usage,counts_before:Counts,counts_after:Counts}
impl Request{fn json(self)->serde_json::Value{let user=ms(self.after.0.ru_utime)-ms(self.before.0.ru_utime);let sys=ms(self.after.0.ru_stime)-ms(self.before.0.ru_stime);serde_json::json!({"index":self.index,"instrumented_wall_ms":self.wall_ms,"user_CPU_ms":user,"system_CPU_ms":sys,"aggregate_CPU_service_over_wall":(user+sys)/self.wall_ms,"minor_faults":self.after.0.ru_minflt-self.before.0.ru_minflt,"major_faults":self.after.0.ru_majflt-self.before.0.ru_majflt,"voluntary_switches":self.after.0.ru_nvcsw-self.before.0.ru_nvcsw,"involuntary_switches":self.after.0.ru_nivcsw-self.before.0.ru_nivcsw,"requested_before":self.counts_before.json(),"requested_after":self.counts_after.json()})}}
fn self_check(){
 // No pool or concurrent allocation is created by this mode. Each backend
 // operation is checked against its logical requested-size delta and payload.
 let start=counts();unsafe{
 let a=Layout::from_size_align(256,64).unwrap();let p=TRACK.alloc_zeroed(a);assert!(!p.is_null());assert_eq!(p as usize%64,0);assert!((0..256).all(|i|p.add(i).read()==0));assert_eq!(counts().live,start.live+256);for i in 0..256{p.add(i).write((i^93)as u8);}
 let q=TRACK.realloc(p,a,262144);assert!(!q.is_null());assert_eq!(q as usize%64,0);assert!((0..256).all(|i|q.add(i).read()==(i^93)as u8));assert_eq!(counts().live,start.live+262144);
 let large=Layout::from_size_align(262144,64).unwrap();let maximum_aligned=(isize::MAX as usize)&!63;let failed=TRACK.realloc(q,large,maximum_aligned);assert!(failed.is_null());assert_eq!(counts().live,start.live+262144);assert!((0..256).all(|i|q.add(i).read()==(i^93)as u8));
 let s=TRACK.realloc(q,large,64);assert!(!s.is_null());assert!((0..64).all(|i|s.add(i).read()==(i^93)as u8));assert_eq!(counts().live,start.live+64);TRACK.dealloc(s,Layout::from_size_align(64,64).unwrap());assert_eq!(counts().live,start.live);
 let fail=TRACK.alloc(Layout::from_size_align(maximum_aligned,64).unwrap());assert!(fail.is_null());assert_eq!(counts().live,start.live);
 }assert_eq!(std::mem::size_of::<Heap>(),10*std::mem::size_of::<usize>());println!("LIFETIME_ALLOCATOR_SELF_CHECK_PASS alignment64 zeroing grow shrink cross128KiB failureownership countrestoration");
}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let args=std::env::args_os().collect::<Vec<_>>();if args.len()==2&&args[1]=="--self-check"{self_check();return Ok(());}if args.len()!=5{return Err("usage: lifetime-sdk IMAGE REQUEST_JSON ROUNDS PROOF_DIRECTORY".into());}let rounds:usize=args[3].to_str().unwrap().parse()?;if rounds!=51{return Err("frozen diagnostic requires51requests".into());}
 let mut frames=Vec::with_capacity(rounds+6);let mut requests=Vec::with_capacity(rounds);let placement_before=placement_setup()?;frames.push(frame("pool",0)?);let file=fs::File::open(&args[1])?;let image=unsafe{memmap2::Mmap::map(&file)?};drop(file);frames.push(frame("mapped",0)?);let prover=zopro_prover::Prover::load_image(Arc::new(image))?;let mut sdk=zopro_zolana::Prover::new(prover)?;let request=fs::read_to_string(&args[2])?;frames.push(frame("loaded",0)?);
 for i in 0..3{std::hint::black_box(sdk.prove(&request)?);frames.push(frame("prime",i)?);}let mut last=None;for index in 0..rounds{let counts_before=counts();let before=usage()?;let t=Instant::now();let proof=sdk.prove(&request)?;let wall_ms=t.elapsed().as_secs_f64()*1000.;let after=usage()?;let counts_after=counts();last=Some(proof);requests.push(Request{index,wall_ms,before,after,counts_before,counts_after});frames.push(frame("proof",index)?);}
 let placement_after=placement_snapshot("fixed",&placement_before["main_allowed"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize).collect::<Vec<_>>(),placement_before["workers"].as_u64().unwrap()as usize)?;assert_eq!(placement_before,placement_after);let dir=Path::new(&args[4]);fs::create_dir_all(dir)?;let proof=last.as_ref().unwrap();fs::write(dir.join("0.proof"),&proof.proof)?;fs::write(dir.join("0.public"),&proof.public_inputs)?;
 for f in &frames{println!("FRAME {}",f.json());}for r in &requests{println!("REQUEST {}",r.json());}println!("CENSUS_SUMMARY {}",serde_json::json!({"rounds":rounds,"priming":3,"frames":frames.len(),"requests":requests.len(),"frame_capacity":frames.capacity(),"request_capacity":requests.capacity(),"observer_reserved_bytes":frames.capacity()*std::mem::size_of::<Frame>()+requests.capacity()*std::mem::size_of::<Request>(),"instrumented_not_performance_estimate":true,"logical_requested_counts_not_physical_or_total_process_memory":true,"counts_include_original_allocator_backing_only_as_requested_Rust_layouts":true,"placement_before":placement_before,"placement_after":placement_after,"sched_schedstats":fs::read_to_string("/proc/sys/kernel/sched_schedstats")?.trim()}));Ok(())
}
include!("placement.rs");
