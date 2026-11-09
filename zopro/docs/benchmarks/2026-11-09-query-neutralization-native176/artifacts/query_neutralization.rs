//! Diagnostic-only removal of query work for one exact key and witness.
//! Cached-query durations are not fresh-work prover performance.
use crate::{G1Projective, G2Projective};
use sha2::{Digest, Sha256};
use std::{cell::RefCell, marker::PhantomData, rc::Rc, sync::{Arc, Mutex}, sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering}};

const OWNER: u64 = 1 << 63;
const EXCLUSIVE: u64 = 1 << 62;
const FUSED: u8 = (1 << 0) | (1 << 1) | (1 << 4);
const SEPARATE: u8 = (1 << 0) | (1 << 2) | (1 << 3) | (1 << 4);
static NEXT_KEY: AtomicU64 = AtomicU64::new(1);
pub(crate) fn next_key_id() -> u64 {
    NEXT_KEY.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1)).expect("diagnostic key generation exhausted")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode { Learn, Replay(u8) }
impl Mode {
    fn encode(self) -> Option<u8> { match self { Self::Learn => Some(8), Self::Replay(mask) if mask < 8 => Some(mask), _ => None } }
    fn decode(value: u8) -> Self { if value == 8 { Self::Learn } else { Self::Replay(value) } }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Query { A, Kh, K, H, G2 }
impl Query {
    fn index(self) -> usize { self as usize }
    fn mask(self) -> u8 { match self { Self::A => 1, Self::G2 => 2, _ => 4 } }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PathSignature { pub kind: u8, pub folded: bool, pub lengths: [usize; 6] }
#[derive(Clone, Copy, PartialEq, Eq)]
struct Binding { key: u64, witness: [u8; 32], omit_b: bool }
#[derive(Default)]
struct Cache {
    binding: Option<Binding>, path: Option<PathSignature>, ready: bool, topology: u8,
    g1: [Option<G1Projective>; 4], g2: Option<G2Projective>,
}
#[derive(Default)]
struct Counters {
    requests: AtomicU64, declines: AtomicU64, faults: AtomicU64, hops: AtomicU64,
    calls: [AtomicU64; 5], hits: [AtomicU64; 5], computed: [AtomicU64; 5], misses: [AtomicU64; 5],
}
impl Counters {
    fn reset(&self) {
        for v in [&self.requests, &self.declines, &self.faults, &self.hops].into_iter()
            .chain(self.calls.iter()).chain(self.hits.iter()).chain(self.computed.iter()).chain(self.misses.iter()) { v.store(0, Ordering::Relaxed); }
    }
}
/// Fixed diagnostic state. No point, witness, scalar or key array is retained.
pub struct Controller { active: AtomicU64, mode: AtomicU8, claimed: AtomicBool, cache: Mutex<Cache>, counts: Counters }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub requests: u64, pub declines: u64, pub faults: u64, pub entry_hops: u64,
    pub calls: [u64; 5], pub hits: [u64; 5], pub computed: [u64; 5], pub misses: [u64; 5],
    pub cache_ready: bool, pub learned_topology: u8, pub learned_path_kind: u8,
}
struct Exclusive<'a>(&'a Controller);
impl Drop for Exclusive<'_> { fn drop(&mut self) { self.0.active.store(0, Ordering::Release); } }
thread_local! { static CURRENT: RefCell<Option<Arc<Controller>>> = const { RefCell::new(None) }; }
pub struct Scope { control: Arc<Controller>, previous: Option<Arc<Controller>>, _caller: PhantomData<Rc<()>> }
impl Controller {
    pub fn new() -> Arc<Self> { Arc::new(Self { active: AtomicU64::new(0), mode: AtomicU8::new(0), claimed: AtomicBool::new(false), cache: Mutex::new(Cache::default()), counts: Counters::default() }) }
    fn exclusive(&self) -> Result<Exclusive<'_>, &'static str> {
        self.active.compare_exchange(0, EXCLUSIVE, Ordering::Acquire, Ordering::Relaxed).map_err(|_| "diagnostic owner or request active")?;
        Ok(Exclusive(self))
    }
    pub fn enter(self: &Arc<Self>, mode: Mode) -> Result<Scope, &'static str> {
        let encoded = mode.encode().ok_or("invalid diagnostic mask")?;
        self.active.compare_exchange(0, OWNER, Ordering::Acquire, Ordering::Relaxed).map_err(|_| "diagnostic controller already owned")?;
        if mode == Mode::Learn && self.cache.lock().unwrap().binding.is_some() {
            self.active.store(0, Ordering::Release); return Err("clear before learning again");
        }
        self.mode.store(encoded, Ordering::Relaxed); self.claimed.store(false, Ordering::Relaxed);
        let previous = CURRENT.with(|c| c.replace(Some(self.clone())));
        Ok(Scope { control: self.clone(), previous, _caller: PhantomData })
    }
    pub fn clear(&self) -> Result<(), &'static str> {
        let _guard = self.exclusive()?; *self.cache.lock().unwrap() = Cache::default(); self.counts.reset(); Ok(())
    }
    pub fn reset_counts(&self) -> Result<(), &'static str> { let _guard = self.exclusive()?; self.counts.reset(); Ok(()) }
    pub fn snapshot(&self) -> Result<Snapshot, &'static str> {
        let _guard = self.exclusive()?; let c = &self.counts; let cache = self.cache.lock().unwrap();
        Ok(Snapshot { requests: c.requests.load(Ordering::Relaxed), declines: c.declines.load(Ordering::Relaxed), faults: c.faults.load(Ordering::Relaxed), entry_hops: c.hops.load(Ordering::Relaxed),
            calls: std::array::from_fn(|i| c.calls[i].load(Ordering::Relaxed)), hits: std::array::from_fn(|i| c.hits[i].load(Ordering::Relaxed)), computed: std::array::from_fn(|i| c.computed[i].load(Ordering::Relaxed)), misses: std::array::from_fn(|i| c.misses[i].load(Ordering::Relaxed)),
            cache_ready: cache.ready, learned_topology: cache.topology, learned_path_kind: cache.path.map_or(0, |p| p.kind) })
    }
    pub fn payload_bytes() -> usize { std::mem::size_of::<Self>() }
    pub fn coordinate_payload_bytes() -> usize { 4 * std::mem::size_of::<G1Projective>() + std::mem::size_of::<G2Projective>() }
}
impl Drop for Scope {
    fn drop(&mut self) {
        CURRENT.with(|c| {
            let present = c.replace(self.previous.take());
            if !present.as_ref().is_some_and(|v| Arc::ptr_eq(v, &self.control)) { self.control.counts.faults.fetch_or(1, Ordering::Relaxed); }
        });
        self.control.active.fetch_and(!OWNER, Ordering::AcqRel);
    }
}
pub(crate) struct Request {
    control: Arc<Controller>, mode: Mode, path_set: AtomicBool, valid: AtomicBool,
    seen: AtomicU8, faults: AtomicU64, finished: AtomicBool,
}
impl Request {
    pub(crate) fn begin(key: u64, witness: &[u8], omit_b: bool, eligible: bool) -> Option<Self> {
        let control = CURRENT.with(|c| c.borrow().clone())?;
        control.active.fetch_update(Ordering::Acquire, Ordering::Relaxed, |v| if v & OWNER != 0 { v.checked_add(1) } else { None }).ok()?;
        control.counts.requests.fetch_add(1, Ordering::Relaxed);
        let decline = |control: &Controller| { control.counts.declines.fetch_add(1, Ordering::Relaxed); control.active.fetch_sub(1, Ordering::Release); };
        if !eligible || !omit_b || control.claimed.swap(true, Ordering::Relaxed) { decline(&control); return None; }
        let binding = Binding { key, witness: Sha256::digest(witness).into(), omit_b };
        let mode = Mode::decode(control.mode.load(Ordering::Relaxed));
        let accepted = {
            let mut cache = control.cache.lock().unwrap();
            match mode {
                Mode::Learn if cache.binding.is_none() => { cache.binding = Some(binding); true }
                Mode::Replay(_) => cache.ready && cache.binding == Some(binding),
                _ => false,
            }
        };
        if !accepted { decline(&control); return None; }
        Some(Self { control, mode, path_set: AtomicBool::new(false), valid: AtomicBool::new(false), seen: AtomicU8::new(0), faults: AtomicU64::new(0), finished: AtomicBool::new(false) })
    }
    pub(crate) fn path(&self, signature: PathSignature) {
        if self.path_set.swap(true, Ordering::Relaxed) { self.fault(2); return; }
        let matched = {
            let mut cache = self.control.cache.lock().unwrap();
            if self.mode == Mode::Learn && cache.path.is_none() { cache.path = Some(signature); true } else { cache.path == Some(signature) }
        };
        self.valid.store(matched, Ordering::Release);
        if !matched { self.control.counts.declines.fetch_add(1, Ordering::Relaxed); }
    }
    pub(crate) fn entry_hop(&self) { self.control.counts.hops.fetch_add(1, Ordering::Relaxed); }
    fn fault(&self, bits: u64) { self.faults.fetch_or(bits, Ordering::Relaxed); self.control.counts.faults.fetch_or(bits, Ordering::Relaxed); }
    fn start_query(&self, query: Query) -> bool {
        let bit = 1 << query.index();
        if self.seen.fetch_or(bit, Ordering::Relaxed) & bit != 0 { self.fault(4); }
        self.control.counts.calls[query.index()].fetch_add(1, Ordering::Relaxed);
        matches!(self.mode, Mode::Replay(mask) if mask & query.mask() != 0)
    }
    fn note_computed(&self, query: Query, requested: bool) {
        self.control.counts.computed[query.index()].fetch_add(1, Ordering::Relaxed);
        if requested { self.control.counts.misses[query.index()].fetch_add(1, Ordering::Relaxed); }
    }
    fn g1(&self, query: Query, original: impl FnOnce() -> G1Projective) -> G1Projective {
        if query == Query::G2 { self.fault(8); return original(); }
        let requested = self.start_query(query);
        if requested && self.valid.load(Ordering::Acquire) {
            if let Some(point) = self.control.cache.lock().unwrap().g1[query.index()] {
                self.control.counts.hits[query.index()].fetch_add(1, Ordering::Relaxed); return point;
            }
        }
        self.note_computed(query, requested); let point = original();
        if self.mode == Mode::Learn && self.valid.load(Ordering::Acquire) { self.control.cache.lock().unwrap().g1[query.index()] = Some(point); }
        point
    }
    fn g2(&self, original: impl FnOnce() -> G2Projective) -> G2Projective {
        let requested = self.start_query(Query::G2);
        if requested && self.valid.load(Ordering::Acquire) {
            if let Some(point) = self.control.cache.lock().unwrap().g2 {
                self.control.counts.hits[4].fetch_add(1, Ordering::Relaxed); return point;
            }
        }
        self.note_computed(Query::G2, requested); let point = original();
        if self.mode == Mode::Learn && self.valid.load(Ordering::Acquire) { self.control.cache.lock().unwrap().g2 = Some(point); }
        point
    }
    fn b_group(&self, original: impl FnOnce() -> (G1Projective, G2Projective)) -> (G1Projective, G2Projective) {
        let requested = self.start_query(Query::G2);
        if requested && self.valid.load(Ordering::Acquire) {
            if let Some(point) = self.control.cache.lock().unwrap().g2 {
                self.control.counts.hits[4].fetch_add(1, Ordering::Relaxed); return (G1Projective::IDENTITY, point);
            }
        }
        self.note_computed(Query::G2, requested); let points = original();
        if points.0 != G1Projective::IDENTITY { self.fault(16); }
        if self.mode == Mode::Learn && self.valid.load(Ordering::Acquire) { self.control.cache.lock().unwrap().g2 = Some(points.1); }
        points
    }
    pub(crate) fn finish(&self, success: bool) {
        if self.finished.swap(true, Ordering::Relaxed) { self.fault(32); return; }
        let seen = self.seen.load(Ordering::Relaxed);
        let complete = seen == FUSED || seen == SEPARATE;
        if success && self.valid.load(Ordering::Acquire) && !complete { self.fault(64); }
        if !self.path_set.load(Ordering::Relaxed) { self.fault(128); }
        if self.mode == Mode::Learn {
            let mut cache = self.control.cache.lock().unwrap();
            cache.ready = success && complete && self.valid.load(Ordering::Acquire) && self.faults.load(Ordering::Relaxed) == 0;
            cache.topology = seen;
        }
    }
}
impl Drop for Request {
    fn drop(&mut self) {
        if !self.finished.load(Ordering::Relaxed) {
            self.fault(256);
            if self.mode == Mode::Learn { self.control.cache.lock().unwrap().ready = false; }
        }
        self.control.active.fetch_sub(1, Ordering::Release);
    }
}
#[inline(always)]
pub(crate) fn g1(r: Option<&Request>, query: Query, original: impl FnOnce() -> G1Projective) -> G1Projective { match r { Some(r) => r.g1(query, original), None => original() } }
#[inline(always)]
pub(crate) fn g2(r: Option<&Request>, original: impl FnOnce() -> G2Projective) -> G2Projective { match r { Some(r) => r.g2(original), None => original() } }
#[inline(always)]
pub(crate) fn b_group(r: Option<&Request>, original: impl FnOnce() -> (G1Projective, G2Projective)) -> (G1Projective, G2Projective) { match r { Some(r) => r.b_group(original), None => original() } }

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::Fr;
    fn signature(kind: u8) -> PathSignature { PathSignature { kind, folded: true, lengths: [5,7,9,16,16,9] } }
    fn p1(index: u64) -> G1Projective { G1Projective::generator() * Fr::from(index + 31) }
    fn p2() -> G2Projective { G2Projective::generator() * Fr::from(79u64) }
    fn queries(r: Option<&Request>, fused: bool, forbid: u8) {
        let originals = [p1(0), p1(1), p1(2), p1(3)];
        for query in if fused { vec![Query::A,Query::Kh] } else { vec![Query::A,Query::K,Query::H] } {
            assert_eq!(g1(r,query,||{assert_eq!(forbid & query.mask(),0); originals[query.index()]}), originals[query.index()]);
        }
        assert_eq!(g2(r,||{assert_eq!(forbid&2,0);p2()}),p2());
    }
    fn learn(control:&Arc<Controller>,key:u64,witness:&[u8],fused:bool) {
        let scope=control.enter(Mode::Learn).unwrap(); let r=Request::begin(key,witness,true,true).unwrap();r.path(signature(if fused {1}else{2}));queries(Some(&r),fused,0);r.finish(true);drop(r);drop(scope);assert!(control.snapshot().unwrap().cache_ready);
    }
    #[test]
    fn all_masks_preserve_points_and_distinguish_query_kinds() {
        for fused in [true,false] {
            let c=Controller::new();let key=next_key_id();learn(&c,key,b"exact witness",fused);
            for mask in 0..8 {c.reset_counts().unwrap();let scope=c.enter(Mode::Replay(mask)).unwrap();let r=Request::begin(key,b"exact witness",true,true).unwrap();r.path(signature(if fused {1}else{2}));queries(Some(&r),fused,mask);r.finish(true);drop(r);drop(scope);let s=c.snapshot().unwrap();assert_eq!(s.faults,0);assert_eq!(s.declines,0);assert_eq!(s.requests,1);for q in [Query::A,Query::Kh,Query::K,Query::H,Query::G2] {let active=if fused {q!=Query::K&&q!=Query::H}else{q!=Query::Kh};let expected=u64::from(active);assert_eq!(s.calls[q.index()],expected);assert_eq!(s.hits[q.index()],u64::from(active&&mask&q.mask()!=0));assert_eq!(s.computed[q.index()],u64::from(active&&mask&q.mask()==0));}}
        }
        assert_eq!(Controller::coordinate_payload_bytes(),576);
        println!("QUERY_DIAGNOSTIC fixed_coordinate_bytes={} controller_bytes={} no_point_arrays",Controller::coordinate_payload_bytes(),Controller::payload_bytes());
    }
    #[test]
    fn binding_changes_and_unsupported_modes_run_original() {
        let c=Controller::new();let key=next_key_id();learn(&c,key,b"first",true);
        for (k,witness,omit,eligible) in [(next_key_id(),&b"first"[..],true,true),(key,&b"second"[..],true,true),(key,&b"first"[..],false,true),(key,&b"first"[..],true,false)] {
            c.reset_counts().unwrap();let scope=c.enter(Mode::Replay(7)).unwrap();assert!(Request::begin(k,witness,omit,eligible).is_none());queries(None,true,0);drop(scope);let s=c.snapshot().unwrap();assert_eq!(s.requests,1);assert_eq!(s.declines,1);assert_eq!(s.hits,[0;5]);assert!(s.cache_ready);
        }
        c.reset_counts().unwrap();let scope=c.enter(Mode::Replay(7)).unwrap();let r=Request::begin(key,b"first",true,true).unwrap();r.path(signature(2));queries(Some(&r),false,0);r.finish(true);drop(r);drop(scope);assert_eq!(c.snapshot().unwrap().hits,[0;5]);assert_eq!(c.snapshot().unwrap().declines,1);
        assert!(c.enter(Mode::Replay(8)).is_err());assert!(c.enter(Mode::Learn).is_err());c.clear().unwrap();learn(&c,next_key_id(),b"new binding",false);
    }
    #[test]
    fn incomplete_learn_unwind_duplicate_and_outstanding_requests() {
        let c=Controller::new();let key=next_key_id();{
            let _scope=c.enter(Mode::Learn).unwrap();let r=Request::begin(key,b"x",true,true).unwrap();r.path(signature(1));assert_eq!(g1(Some(&r),Query::A,||p1(0)),p1(0));r.finish(true);
        }assert!(!c.snapshot().unwrap().cache_ready);{let _scope=c.enter(Mode::Replay(7)).unwrap();assert!(Request::begin(key,b"x",true,true).is_none());}
        c.clear().unwrap();let outcome=std::panic::catch_unwind(||{let _scope=c.enter(Mode::Learn).unwrap();let r=Request::begin(key,b"x",true,true).unwrap();r.path(signature(1));g1(Some(&r),Query::A,||panic!("synthetic query unwind"));});assert!(outcome.is_err());assert!(!c.snapshot().unwrap().cache_ready);c.clear().unwrap();
        {let _scope=c.enter(Mode::Learn).unwrap();let r=Request::begin(key,b"x",true,true).unwrap();r.path(signature(1));queries(Some(&r),true,0);g1(Some(&r),Query::A,||p1(0));r.finish(true);}assert!(!c.snapshot().unwrap().cache_ready);assert_ne!(c.snapshot().unwrap().faults&4,0);c.clear().unwrap();
        let scope=c.enter(Mode::Learn).unwrap();let r=Request::begin(key,b"x",true,true).unwrap();r.path(signature(1));drop(scope);assert!(c.snapshot().is_err());assert!(c.reset_counts().is_err());assert!(c.clear().is_err());assert!(c.enter(Mode::Replay(0)).is_err());queries(Some(&r),true,0);r.finish(true);drop(r);assert!(c.snapshot().unwrap().cache_ready);c.clear().unwrap();
        assert!(CURRENT.with(|v|v.borrow().is_none()));
    }
    #[test]
    fn nested_controllers_and_b_group_identity() {
        let a=Controller::new();let b=Controller::new();let ka=next_key_id();let kb=next_key_id();{
            let _outer=a.enter(Mode::Learn).unwrap();assert!(a.enter(Mode::Replay(0)).is_err());{
                let _inner=b.enter(Mode::Learn).unwrap();let r=Request::begin(kb,b"b",true,true).unwrap();r.path(signature(1));queries(Some(&r),true,0);r.finish(true);
            }let r=Request::begin(ka,b"a",true,true).unwrap();r.path(signature(1));g1(Some(&r),Query::A,||p1(0));g1(Some(&r),Query::Kh,||p1(1));assert_eq!(b_group(Some(&r),||(G1Projective::IDENTITY,p2())),(G1Projective::IDENTITY,p2()));r.finish(true);
        }assert!(a.snapshot().unwrap().cache_ready);assert!(b.snapshot().unwrap().cache_ready);
        a.reset_counts().unwrap();{let _scope=a.enter(Mode::Replay(7)).unwrap();let r=Request::begin(ka,b"a",true,true).unwrap();r.path(signature(1));assert_eq!(g1(Some(&r),Query::A,||panic!("A should hit")),p1(0));assert_eq!(g1(Some(&r),Query::Kh,||panic!("KH should hit")),p1(1));assert_eq!(b_group(Some(&r),||panic!("G2 group should hit")),(G1Projective::IDENTITY,p2()));r.finish(true);}assert_eq!(a.snapshot().unwrap().hits,[1,1,0,0,1]);
    }
    #[test]
    fn actual_workers_nested_joins_entry_transport_and_distinct_owners() {
        use rayon::prelude::*;
        let mut total=0usize;
        for workers in [1,4,16,32] {
            let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
            let seen=Arc::new((0..workers).map(|_|AtomicBool::new(false)).collect::<Vec<_>>());let barrier=Arc::new(std::sync::Barrier::new(workers));pool.scope(|scope|{for _ in 0..workers{let seen=seen.clone();let barrier=barrier.clone();scope.spawn(move |_|{seen[rayon::current_thread_index().unwrap()].store(true,Ordering::Relaxed);barrier.wait();});}});assert!(seen.iter().all(|v|v.load(Ordering::Relaxed)));total+=workers;
            let c=Controller::new();let key=next_key_id();let scope=c.enter(Mode::Learn).unwrap();let r=Request::begin(key,b"transport",true,true).unwrap();r.path(signature(1));r.entry_hop();pool.install(||{assert!(CURRENT.with(|v|v.borrow().is_none()));rayon::join(||{g1(Some(&r),Query::A,||p1(0));},||{rayon::join(||{g1(Some(&r),Query::Kh,||p1(1));},||{g2(Some(&r),p2);});});});r.finish(true);drop(r);drop(scope);assert_eq!(c.snapshot().unwrap().entry_hops,1);assert!(c.snapshot().unwrap().cache_ready);
            (0..4usize).into_par_iter().for_each(|owner|{let c=Controller::new();let key=next_key_id();let witness=[owner as u8];learn(&c,key,&witness,false);let _scope=c.enter(Mode::Replay(7)).unwrap();let r=Request::begin(key,&witness,true,true).unwrap();r.path(signature(2));pool.install(||{rayon::join(||{g1(Some(&r),Query::A,||panic!("A hit"));},||{rayon::join(||{g1(Some(&r),Query::K,||panic!("K hit"));g1(Some(&r),Query::H,||panic!("H hit"));},||{g2(Some(&r),||panic!("G2 hit"));});});});r.finish(true);});
            println!("QUERY_DIAGNOSTIC workers={workers} all_actual_indices=true requested_stack_bytes=524288 nested_join=true entry_transport=true distinct_owners=4");
        }
        assert_eq!(total,53);println!("QUERY_DIAGNOSTIC actual_workers=53");
    }
}
