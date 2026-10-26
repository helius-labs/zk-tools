//! Private Linux diagnostic. Wall spans may nest and include parallel work.
//! Calling-thread CPU is not child CPU or exclusive stage CPU.
use std::{marker::PhantomData, rc::Rc, sync::{atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::*}, Mutex, MutexGuard}};
const CAPACITY: usize = 512;
#[derive(Default)]
struct Cell { state: AtomicU64, label: AtomicU64, n: AtomicU64, tid: AtomicU64, wall0: AtomicU64, wall1: AtomicU64, cpu0: AtomicU64, cpu1: AtomicU64 }
static CELLS: std::sync::LazyLock<Box<[Cell]>> = std::sync::LazyLock::new(|| (0..CAPACITY).map(|_| Cell::default()).collect());
static LOCK: Mutex<()> = Mutex::new(());
static ACTIVE: AtomicBool = AtomicBool::new(false);
static NEXT: AtomicUsize = AtomicUsize::new(0);
static LIVE: AtomicUsize = AtomicUsize::new(0);
static GENERATION: AtomicU64 = AtomicU64::new(0);
thread_local! { static TID: u64 = unsafe { libc::syscall(libc::SYS_gettid) as u64 }; }
fn clock(which: libc::clockid_t) -> u64 {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: valid writable timespec and a supported Linux clock ID.
    assert_eq!(unsafe { libc::clock_gettime(which, &mut t) }, 0);
    (t.tv_sec as u64).checked_mul(1_000_000_000).unwrap() + t.tv_nsec as u64
}
pub fn initialize() { std::sync::LazyLock::force(&CELLS); }
pub fn storage_bytes() -> usize { CAPACITY * std::mem::size_of::<Cell>() }
pub struct Session { _guard: MutexGuard<'static, ()> }
impl Session {
    pub fn begin() -> Self {
        let guard = LOCK.lock().unwrap_or_else(|e| e.into_inner());
        assert!(!ACTIVE.load(Acquire));
        assert_eq!(LIVE.load(Acquire), 0, "prior session has unfinished spans");
        initialize();
        GENERATION.fetch_add(1, AcqRel);
        for c in CELLS.iter() { c.state.store(0, Relaxed); }
        NEXT.store(0, Relaxed);
        ACTIVE.store(true, Release);
        Self { _guard: guard }
    }
    pub fn finish(self) -> Result<Vec<Record>, String> {
        ACTIVE.store(false, Release);
        if LIVE.load(Acquire) != 0 { return Err("unfinished stage spans".into()); }
        let count = NEXT.load(Acquire);
        if count > CAPACITY { return Err("stage recorder capacity exceeded".into()); }
        CELLS[..count].iter().enumerate().map(|(id,c)| {
            if c.state.load(Acquire) != 2 { return Err(format!("unfinished record {id}")); }
            let r = Record { id, label: c.label.load(Relaxed), n: c.n.load(Relaxed), tid: c.tid.load(Relaxed), wall0: c.wall0.load(Relaxed), wall1: c.wall1.load(Relaxed), cpu0: c.cpu0.load(Relaxed), cpu1: c.cpu1.load(Relaxed) };
            if r.wall1 < r.wall0 || r.cpu1 < r.cpu0 { return Err("clock order".into()); }
            Ok(r)
        }).collect()
    }
}
impl Drop for Session { fn drop(&mut self) { ACTIVE.store(false, Release); } }
#[derive(Debug)]
pub struct Record { pub id: usize, pub label: u64, pub n: u64, pub tid: u64, pub wall0: u64, pub wall1: u64, pub cpu0: u64, pub cpu1: u64 }
pub struct Span { index: Option<usize>, _same_thread: PhantomData<Rc<()>> }
impl Span {
    #[inline]
    pub fn enter(label: u64, n: usize) -> Self {
        let mut out = Self { index: None, _same_thread: PhantomData };
        let generation = GENERATION.load(Acquire);
        if !ACTIVE.load(Acquire) { return out; }
        LIVE.fetch_add(1, AcqRel);
        if !ACTIVE.load(Acquire) || GENERATION.load(Acquire) != generation {
            LIVE.fetch_sub(1, Release);
            return out;
        }
        let i = NEXT.fetch_add(1, Relaxed);
        if i >= CAPACITY { LIVE.fetch_sub(1, Release); return out; }
        let c = &CELLS[i];
        c.label.store(label, Relaxed); c.n.store(n as u64, Relaxed);
        c.tid.store(TID.with(|t| *t), Relaxed);
        c.wall0.store(clock(libc::CLOCK_MONOTONIC), Relaxed);
        c.cpu0.store(clock(libc::CLOCK_THREAD_CPUTIME_ID), Relaxed);
        c.state.store(1, Release);
        out.index = Some(i);
        out
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        if let Some(i) = self.index {
            let c = &CELLS[i];
            c.cpu1.store(clock(libc::CLOCK_THREAD_CPUTIME_ID), Relaxed);
            c.wall1.store(clock(libc::CLOCK_MONOTONIC), Relaxed);
            c.state.store(2, Release);
            LIVE.fetch_sub(1, Release);
        }
    }
}
pub fn label(id: u64) -> &'static str { match id {
    1=>"sdk.witness", 2=>"sdk.serialization", 3=>"prover.encode", 4=>"prover.solve_rerandomized", 5=>"prover.verify",
    6=>"core.external_entry", 7=>"core.worker_entry", 8=>"core.solve_wires", 9=>"core.public_correction", 10=>"core.canonical_wires",
    11=>"core.gather_queries", 12=>"core.product_fft", 13=>"core.query_join", 14=>"core.encode_parts", 15=>"core.assemble_randomized",
    16=>"g2.query", 17=>"grouping.policy", 18=>"grouping.sample", 19=>"grouping.map", 20=>"grouping.pack",
    21=>"g2.phi_split", 22=>"g2.arithmetic", _=>"test" } }
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn nested_and_parallel_spans_are_complete() {
        let s=Session::begin();
        { let _p=Span::enter(1,42); rayon::scope(|scope| { for _ in 0..16 { scope.spawn(|_| { let _s=Span::enter(2,3); std::hint::black_box((0..1000u64).sum::<u64>()); }); } }); }
        let r=s.finish().unwrap(); assert_eq!(r.len(),17);
        for (i,e) in r.iter().enumerate() { assert_eq!(i,e.id); assert!(e.wall1>=e.wall0); assert!(e.cpu1>=e.cpu0); assert!(e.tid>0); }
        assert!(r[1..].iter().all(|e|e.wall0>=r[0].wall0&&e.wall1<=r[0].wall1));
    }
    #[test] fn exhaustion_is_reported() { let s=Session::begin(); for _ in 0..=CAPACITY { drop(Span::enter(1,0)); } assert!(s.finish().unwrap_err().contains("capacity")); }
    #[test] fn unfinished_span_prevents_reuse_until_dropped() { let s=Session::begin(); let p=Span::enter(1,0); assert!(s.finish().unwrap_err().contains("unfinished")); drop(p); assert!(Session::begin().finish().unwrap().is_empty()); }
    #[test] fn panic_disarms_and_allows_reuse() { assert!(std::panic::catch_unwind(|| { let _s=Session::begin(); let _p=Span::enter(1,0); panic!("expected"); }).is_err()); assert!(Session::begin().finish().unwrap().is_empty()); }
}
