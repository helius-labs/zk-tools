//! Diagnostic-only coarse proof phases; no field values or key data are recorded.
use std::{cell::RefCell,marker::PhantomData,rc::Rc,sync::{Arc,OnceLock},sync::atomic::{AtomicU8,AtomicU32,AtomicU64,Ordering},time::Instant};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(usize)]
pub enum Phase { Request,Prover,Encode,RawSolve,Pipeline,Solver,PrepareScalars,Fft,Queries,A,Kh,G2,K,H,B1,Serialize,Finalize,Verify }
pub const PHASES:[Phase;18]=[Phase::Request,Phase::Prover,Phase::Encode,Phase::RawSolve,Phase::Pipeline,Phase::Solver,Phase::PrepareScalars,Phase::Fft,Phase::Queries,Phase::A,Phase::Kh,Phase::G2,Phase::K,Phase::H,Phase::B1,Phase::Serialize,Phase::Finalize,Phase::Verify];
impl Phase {
    pub fn name(self)->&'static str { ["request","prover","encode","raw_solve","pipeline","solver","prepare_scalars","fft","queries","a","kh","g2","k","h","b1","serialize","finalize","verify"][self as usize] }
    pub fn parent(self)->Option<Phase> { use Phase::*;Some(match self {Request=>return None,Prover=>Request,Encode|RawSolve|Verify=>Prover,Pipeline|Finalize=>RawSolve,Solver|PrepareScalars|Fft|Queries|Serialize=>Pipeline,A|Kh|G2|K|H|B1=>Queries}) }
}
#[derive(Default)]
struct Slot { state:AtomicU8,start:AtomicU64,end:AtomicU64,start_tid:AtomicU64,end_tid:AtomicU64,start_cpu:AtomicU32,end_cpu:AtomicU32 }
const OWNED:u64=1<<63;
const LOCKED:u64=1<<62;
pub struct Recorder { active:AtomicU64,origin:AtomicU64,faults:AtomicU32,entry_hops:AtomicU32,slots:[Slot;18] }
#[derive(Clone,Copy,Debug,Default)]
pub struct Entry {pub state:u8,pub start_ns:u64,pub end_ns:u64,pub start_tid:u64,pub end_tid:u64,pub start_cpu:u32,pub end_cpu:u32}
#[derive(Clone,Copy,Debug)]
pub struct Snapshot {pub faults:u32,pub entry_hops:u32,pub entries:[Entry;18]}
static CLOCK:OnceLock<Instant>=OnceLock::new();
fn now()->u64 {CLOCK.get_or_init(Instant::now).elapsed().as_nanos().min(u64::MAX as u128)as u64}
fn tid()->u64 {unsafe{libc::syscall(libc::SYS_gettid)as u64}}
fn cpu()->u32 {unsafe{libc::sched_getcpu()as u32}}
thread_local!{static CURRENT:RefCell<Option<Arc<Recorder>>>=const{RefCell::new(None)};}
pub fn current()->Option<Arc<Recorder>>{CURRENT.with(|c|c.borrow().clone())}
impl Recorder {
    pub fn new()->Arc<Self>{Arc::new(Self{active:AtomicU64::new(0),origin:AtomicU64::new(now()),faults:AtomicU32::new(0),entry_hops:AtomicU32::new(0),slots:std::array::from_fn(|_|Slot::default())})}
    pub fn reset(&self)->Result<(),&'static str>{
        self.active.compare_exchange(0,LOCKED,Ordering::Acquire,Ordering::Relaxed).map_err(|_|"recorder active")?;
        for s in &self.slots{s.state.store(0,Ordering::Relaxed);s.start.store(0,Ordering::Relaxed);s.end.store(0,Ordering::Relaxed);s.start_tid.store(0,Ordering::Relaxed);s.end_tid.store(0,Ordering::Relaxed);s.start_cpu.store(0,Ordering::Relaxed);s.end_cpu.store(0,Ordering::Relaxed);}
        self.faults.store(0,Ordering::Relaxed);self.entry_hops.store(0,Ordering::Relaxed);self.origin.store(now(),Ordering::Relaxed);self.active.store(0,Ordering::Release);Ok(())
    }
    pub fn snapshot(&self)->Result<Snapshot,&'static str>{
        self.active.compare_exchange(0,LOCKED,Ordering::Acquire,Ordering::Relaxed).map_err(|_|"recorder active")?;
        let out=Snapshot{faults:self.faults.load(Ordering::Relaxed),entry_hops:self.entry_hops.load(Ordering::Relaxed),entries:std::array::from_fn(|i|{let s=&self.slots[i];Entry{state:s.state.load(Ordering::Relaxed),start_ns:s.start.load(Ordering::Relaxed),end_ns:s.end.load(Ordering::Relaxed),start_tid:s.start_tid.load(Ordering::Relaxed),end_tid:s.end_tid.load(Ordering::Relaxed),start_cpu:s.start_cpu.load(Ordering::Relaxed),end_cpu:s.end_cpu.load(Ordering::Relaxed)}})};
        self.active.store(0,Ordering::Release);Ok(out)
    }
    pub(crate) fn mark_entry_hop(&self){self.entry_hops.fetch_add(1,Ordering::Relaxed);}
    pub fn payload_bytes()->usize{std::mem::size_of::<Self>()}
}
pub struct Session {record:Option<Arc<Recorder>>,previous:Option<Arc<Recorder>>,_caller:PhantomData<Rc<()>>}
impl Session {
    pub fn enter(record:Option<&Arc<Recorder>>)->Result<Self,&'static str>{
        if let Some(r)=record{r.active.compare_exchange(0,OWNED,Ordering::Acquire,Ordering::Relaxed).map_err(|_|"recorder already active")?;}
        let record=record.cloned();let previous=CURRENT.with(|c|c.replace(record.clone()));Ok(Self{record,previous,_caller:PhantomData})
    }
}
impl Drop for Session {fn drop(&mut self){CURRENT.with(|c|{let present=c.replace(self.previous.take());match(&present,&self.record){(None,None)=>{},(Some(a),Some(b))if Arc::ptr_eq(a,b)=>{},_=>{if let Some(r)=&self.record{r.faults.fetch_or(4,Ordering::Relaxed);}}}});if let Some(r)=&self.record{let before=r.active.fetch_and(!OWNED,Ordering::AcqRel);if before&!OWNED!=0{r.faults.fetch_or(8,Ordering::Relaxed);}}}}
pub struct Span<'a>{record:Option<&'a Recorder>,phase:Phase}
impl<'a> Span<'a>{
    pub fn new(record:Option<&'a Recorder>,phase:Phase)->Self{
        if let Some(r)=record{let s=&r.slots[phase as usize];if r.active.fetch_update(Ordering::Acquire,Ordering::Relaxed,|v|if v&OWNED!=0{v.checked_add(1)}else{None}).is_err(){r.faults.fetch_or(1,Ordering::Relaxed);return Self{record:None,phase};}if s.state.compare_exchange(0,1,Ordering::Relaxed,Ordering::Relaxed).is_err(){r.faults.fetch_or(1,Ordering::Relaxed);r.active.fetch_sub(1,Ordering::Release);return Self{record:None,phase};}s.start_tid.store(tid(),Ordering::Relaxed);s.start_cpu.store(cpu(),Ordering::Relaxed);s.start.store(now().saturating_sub(r.origin.load(Ordering::Relaxed)),Ordering::Relaxed);}
        Self{record,phase}
    }
}
impl Drop for Span<'_>{fn drop(&mut self){if let Some(r)=self.record{let s=&r.slots[self.phase as usize];s.end.store(now().saturating_sub(r.origin.load(Ordering::Relaxed)),Ordering::Relaxed);s.end_cpu.store(cpu(),Ordering::Relaxed);s.end_tid.store(tid(),Ordering::Relaxed);let aborted=std::thread::panicking();if aborted{r.faults.fetch_or(2,Ordering::Relaxed);}s.state.store(if aborted{3}else{2},Ordering::Release);r.active.fetch_sub(1,Ordering::Release);}}}

#[cfg(test)]mod tests {
    use super::*;
    #[test]fn disabled_nested_sessions_and_reuse(){
        assert!(current().is_none());let r=Recorder::new();r.reset().unwrap();{let _outer=Session::enter(Some(&r)).unwrap();assert!(Arc::ptr_eq(&current().unwrap(),&r));assert!(r.reset().is_err());assert!(r.snapshot().is_err());assert!(Session::enter(Some(&r)).is_err());{let _off=Session::enter(None).unwrap();assert!(current().is_none());let current=current();let _s=Span::new(current.as_deref(),Phase::Solver);}assert!(Arc::ptr_eq(&current().unwrap(),&r));let _s=Span::new(Some(&r),Phase::Request);}assert!(current().is_none());let s=r.snapshot().unwrap();assert_eq!(s.faults,0);assert_eq!(s.entries[Phase::Request as usize].state,2);for _ in 0..3{r.reset().unwrap();let session=Session::enter(Some(&r)).unwrap();{let _s=Span::new(Some(&r),Phase::Request);}drop(session);assert_eq!(r.snapshot().unwrap().faults,0);}
    }
    #[test]fn duplicate_phase_and_unwind_are_reported(){let r=Recorder::new();{let _session=Session::enter(Some(&r)).unwrap();{let _a=Span::new(Some(&r),Phase::Request);let _b=Span::new(Some(&r),Phase::Request);}}assert_ne!(r.snapshot().unwrap().faults&1,0);r.reset().unwrap();let result=std::panic::catch_unwind(||{let _session=Session::enter(Some(&r)).unwrap();let _s=Span::new(Some(&r),Phase::Request);panic!("recorder unwind test");});assert!(result.is_err());assert!(current().is_none());let s=r.snapshot().unwrap();assert_ne!(s.faults&2,0);assert_eq!(s.entries[0].state,3);}
    #[test]fn proof_local_parallel_callbacks_and_independent_requests(){
        for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();std::thread::scope(|threads|{for _ in 0..4{let pool=&pool;threads.spawn(move||{let r=Recorder::new();let session=Session::enter(Some(&r)).unwrap();{let _root=Span::new(Some(&r),Phase::Request);let captured=current().unwrap();pool.install(||rayon::join(||{let _s=Span::new(Some(&captured),Phase::A);},||{let _s=Span::new(Some(&captured),Phase::G2);}));}drop(session);let s=r.snapshot().unwrap();assert_eq!(s.faults,0);for p in[Phase::Request,Phase::A,Phase::G2]{let e=s.entries[p as usize];assert_eq!(e.state,2);assert!(e.end_ns>=e.start_ns);assert!(e.start_tid>0&&e.end_tid>0);assert!(e.start_ns>=s.entries[0].start_ns&&e.end_ns<=s.entries[0].end_ns);}});}});}
    }
    #[test]fn nested_distinct_recorders_and_fixed_metadata_size(){let a=Recorder::new();let b=Recorder::new();{let _a=Session::enter(Some(&a)).unwrap();{let _b=Session::enter(Some(&b)).unwrap();assert!(Arc::ptr_eq(&current().unwrap(),&b));let _s=Span::new(Some(&b),Phase::Request);}assert!(Arc::ptr_eq(&current().unwrap(),&a));let _s=Span::new(Some(&a),Phase::Request);}assert_eq!(a.snapshot().unwrap().faults,0);assert_eq!(b.snapshot().unwrap().faults,0);assert_eq!(a.snapshot().unwrap().entries.len(),18);assert!(Recorder::payload_bytes()<2048);println!("COARSE_RECORDER_PAYLOAD_BYTES={}",Recorder::payload_bytes());
        a.reset().unwrap();let session=Session::enter(Some(&a)).unwrap();let span=Span::new(Some(&a),Phase::Request);drop(session);assert!(a.reset().is_err());assert!(a.snapshot().is_err());drop(span);assert_ne!(a.snapshot().unwrap().faults&8,0);a.reset().unwrap();assert_eq!(a.snapshot().unwrap().faults,0);
    }
}

#[cfg(test)]mod explicit_entry_transport_test{
 use super::*;
 #[test]fn borrowed_context_crosses_worker_entry_without_worker_tls(){for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();let r=Recorder::new();let session=Session::enter(Some(&r)).unwrap();{let _root=Span::new(Some(&r),Phase::Request);let captured=current();r.mark_entry_hop();pool.install(||{assert!(current().is_none());let trace=captured.as_deref();let _canonical=Span::new(trace,Phase::Pipeline);rayon::join(||{let _a=Span::new(trace,Phase::A);},||{let _g2=Span::new(trace,Phase::G2);});});}drop(session);let s=r.snapshot().unwrap();assert_eq!(s.faults,0);assert_eq!(s.entry_hops,1);for p in[Phase::Request,Phase::Pipeline,Phase::A,Phase::G2]{assert_eq!(s.entries[p as usize].state,2);}}}
}
