//! Private, bounded observation of declared independent arithmetic units.
use std::{cell::Cell, sync::{atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering::*}, Mutex, MutexGuard, OnceLock}, time::Instant};
pub const CAPACITY: usize = 32768;
static SERIAL: Mutex<()> = Mutex::new(());
static TRACE: OnceLock<Trace> = OnceLock::new();
thread_local! { static IN_LEAF: Cell<bool> = const { Cell::new(false) }; }
struct Node { ready: AtomicU64, start: AtomicU64, end: AtomicU64, kind: AtomicU32, worker: AtomicU32, state: AtomicU32, batch: AtomicU32, ordinal: AtomicU32 }
impl Node { fn new() -> Self { Self { ready:AtomicU64::new(0),start:AtomicU64::new(0),end:AtomicU64::new(0),kind:AtomicU32::new(0),worker:AtomicU32::new(u32::MAX),state:AtomicU32::new(0),batch:AtomicU32::new(0),ordinal:AtomicU32::new(0) } } }
struct Trace { epoch: Instant, enabled: AtomicBool, overflow: AtomicBool, next: AtomicUsize, limit: AtomicUsize, nodes: Box<[Node]> }
impl Trace { fn now(&self) -> u64 { self.epoch.elapsed().as_nanos().try_into().expect("clock range") } }
pub fn initialize() { TRACE.get_or_init(||Trace { epoch:Instant::now(), enabled:AtomicBool::new(false),overflow:AtomicBool::new(false),next:AtomicUsize::new(0),limit:AtomicUsize::new(CAPACITY),nodes:(0..CAPACITY).map(|_|Node::new()).collect() }); }
pub fn storage_bytes() -> usize { CAPACITY * std::mem::size_of::<Node>() }
pub struct Session { trace: &'static Trace, _serial: MutexGuard<'static, ()>, finished: bool }
#[derive(Debug,Clone)]
pub struct Event { pub id:usize, pub ready:u64, pub start:u64, pub end:u64, pub kind:u32, pub worker:u32, pub state:u32, pub batch:u32, pub ordinal:u32 }
impl Session {
    pub fn begin() -> Self {
        initialize();
        let serial=SERIAL.lock().unwrap_or_else(|e|e.into_inner());
        let t=TRACE.get().unwrap();
        assert!(!t.enabled.load(Acquire));
        t.next.store(0,Relaxed);t.overflow.store(false,Relaxed);t.limit.store(CAPACITY,Relaxed);
        t.enabled.store(true,Release);
        Self {trace:t,_serial:serial,finished:false}
    }
    pub fn finish(mut self) -> Result<Vec<Event>,String> {
        self.trace.enabled.store(false,Release);self.finished=true;
        if self.trace.overflow.load(Acquire) {return Err("bounded diagnostic buffer exhausted".into());}
        let n=self.trace.next.load(Acquire);
        let mut out=Vec::with_capacity(n);
        for (i,e) in self.trace.nodes[..n].iter().enumerate() {
            let state=e.state.load(Acquire);
            if state!=2 && state!=3 {return Err(format!("unfinished diagnostic unit {i}: {state}"));}
            let event=Event {id:i,ready:e.ready.load(Relaxed),start:e.start.load(Relaxed),end:e.end.load(Relaxed),kind:e.kind.load(Relaxed),worker:e.worker.load(Relaxed),state,batch:e.batch.load(Relaxed),ordinal:e.ordinal.load(Relaxed)};
            if state==2 && !(event.ready<=event.start && event.start<=event.end) {return Err(format!("unordered diagnostic unit {i}"));}
            out.push(event);
        }
        Ok(out)
    }
}
impl Drop for Session {fn drop(&mut self) {if !self.finished {self.trace.enabled.store(false,Release);}}}
pub(crate) struct Batch { trace:Option<&'static Trace>, first:usize, count:usize }
impl Batch {
    pub(crate) fn new(kind:u32,count:usize) -> Self {
        let off=Self{trace:None,first:0,count:0};
        let Some(t)=TRACE.get() else {return off;};
        if !t.enabled.load(Acquire) || count==0 {return off;}
        let first=t.next.fetch_add(count,Relaxed);
        if first.checked_add(count).is_none_or(|end|end>t.limit.load(Relaxed)) {t.overflow.store(true,Release);return off;}
        let ready=t.now();
        for (i,n) in t.nodes[first..first+count].iter().enumerate() {
            n.ready.store(ready,Relaxed);n.start.store(0,Relaxed);n.end.store(0,Relaxed);n.kind.store(kind,Relaxed);n.worker.store(u32::MAX,Relaxed);n.batch.store(first as u32,Relaxed);n.ordinal.store(i as u32,Relaxed);n.state.store(0,Release);
        }
        Self{trace:Some(t),first,count}
    }
    pub(crate) fn start(&self,i:usize) -> Leaf {
        let Some(t)=self.trace else {return Leaf{trace:None,index:0};};
        assert!(i<self.count);let index=self.first+i;let n=&t.nodes[index];
        IN_LEAF.with(|v|assert!(!v.replace(true),"arithmetic observation contains a nested leaf"));
        assert_eq!(n.state.swap(1,AcqRel),0,"unit executed twice");
        n.worker.store(rayon::current_thread_index().map(|x|x as u32).unwrap_or(u32::MAX),Relaxed);n.start.store(t.now(),Relaxed);
        Leaf{trace:Some(t),index}
    }
    pub(crate) fn cancel(&self,i:usize) {if let Some(t)=self.trace {assert!(i<self.count);assert_eq!(t.nodes[self.first+i].state.swap(3,AcqRel),0);}}
}
pub(crate) struct Leaf {trace:Option<&'static Trace>,index:usize}
impl Drop for Leaf {fn drop(&mut self) {if let Some(t)=self.trace {let n=&t.nodes[self.index];n.end.store(t.now(),Relaxed);n.state.store(2,Release);IN_LEAF.with(|v|assert!(v.replace(false)));}}}
pub(crate) fn family<C: zopro_bn254::CurveConfig>(g1:u32,g2:u32)->u32 {if std::mem::size_of::<C::Base>()==32 {g1}else{g2}}

#[cfg(test)] mod tests {
    use super::*;use rayon::prelude::*;
    #[test] fn parallel_units_are_unique_and_ordered() {let session=Session::begin();let batch=Batch::new(17,4096);(0..4096).into_par_iter().for_each(|i|{let _leaf=batch.start(i);std::hint::black_box(i*i);});let out=session.finish().unwrap();assert_eq!(out.len(),4096);for(i,e)in out.iter().enumerate(){assert_eq!(e.id,i);assert_eq!(e.ordinal,i as u32);assert_eq!(e.kind,17);assert_eq!(e.state,2);assert!(e.ready<=e.start&&e.start<=e.end);}}
    #[test] fn cancellation_is_distinct_from_execution() {let session=Session::begin();let batch=Batch::new(7,3);batch.cancel(1);{let _l=batch.start(0);}{let _l=batch.start(2);}let out=session.finish().unwrap();assert_eq!(out.iter().map(|e|e.state).collect::<Vec<_>>(),[2,3,2]);assert_eq!(out[1].start,0);}
    #[test] fn exhaustion_is_reported_without_out_of_bounds_writes() {let session=Session::begin();session.trace.limit.store(2,Relaxed);let batch=Batch::new(7,3);{let _l=batch.start(200);}assert!(session.finish().unwrap_err().contains("exhausted"));}
    #[test] fn panic_disarms_and_allows_reuse() {let result=std::panic::catch_unwind(||{let _session=Session::begin();let batch=Batch::new(8,1);let _leaf=batch.start(0);panic!("probe");});assert!(result.is_err());let session=Session::begin();let batch=Batch::new(9,1);{let _leaf=batch.start(0);}assert_eq!(session.finish().unwrap()[0].kind,9);}
}
