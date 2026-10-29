//! Diagnostic-only serial CPU and elapsed spans over the original arithmetic.
use std::{
    cell::{Cell, RefCell},
    marker::PhantomData,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        OnceLock,
    },
    time::Instant,
};
pub type Event = [u64; 10];
pub const LIMIT: usize = 1024;
static ENABLED: AtomicBool = AtomicBool::new(false);
static CLOCK: OnceLock<Instant> = OnceLock::new();
static ORIGIN: AtomicU64 = AtomicU64::new(0);
static API: AtomicU64 = AtomicU64::new(0);
thread_local! {static EVENTS:RefCell<Vec<Event>>=const{RefCell::new(Vec::new())};static PARENT:Cell<u64>=const{Cell::new(0)};static DROPPED:Cell<u64>=const{Cell::new(0)};}
fn wall() -> u64 {
    CLOCK.get().unwrap().elapsed().as_nanos() as u64 - ORIGIN.load(Ordering::Relaxed)
}
fn cpu() -> u64 {
    let mut ts: libc::timespec = unsafe { std::mem::zeroed() };
    assert_eq!(
        unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut ts) },
        0
    );
    ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
}
pub struct Scope {
    slot: Option<usize>,
    id: u64,
    old: u64,
    serial: bool,
    _local: PhantomData<Rc<()>>,
}
impl Scope {
    pub fn id(&self) -> u64 {
        self.id
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        if let Some(index) = self.slot {
            let c = if self.serial { cpu() } else { 0 };
            let end = wall();
            EVENTS.with(|v| {
                let mut v = v.borrow_mut();
                assert_eq!(v[index][6], 0);
                v[index][6] = end;
                v[index][9] = c;
            });
            PARENT.with(|p| {
                assert_eq!(p.get(), self.id);
                p.set(self.old);
            });
        }
    }
}
pub fn parent() -> u64 {
    PARENT.with(|p| {
        let id = p.get();
        if id == 0 {
            API.load(Ordering::Relaxed)
        } else {
            id
        }
    })
}
#[inline]
pub fn scope(parent: u64, kind: u64, a: usize, b: usize, serial: bool) -> Scope {
    if !ENABLED.load(Ordering::Relaxed) {
        return Scope {
            slot: None,
            id: 0,
            old: 0,
            serial: false,
            _local: PhantomData,
        };
    }
    let worker = rayon::current_thread_index().map_or(0, |x| x + 1);
    let old = PARENT.with(Cell::get);
    let available = EVENTS.with(|v| {
        let v = v.borrow();
        v.len() < LIMIT
    });
    if !available {
        DROPPED.with(|x| x.set(x.get() + 1));
        return Scope {
            slot: None,
            id: 0,
            old,
            serial: false,
            _local: PhantomData,
        };
    }
    let start = wall();
    let c = if serial { cpu() } else { 0 };
    let (slot, id) = EVENTS.with(|v| {
        let mut v = v.borrow_mut();
        assert!(v.capacity() >= LIMIT);
        let slot = v.len();
        let id = ((worker as u64) << 32) | (slot as u64 + 1);
        v.push([
            id,
            parent,
            kind,
            a as u64,
            b as u64,
            start,
            0,
            worker as u64,
            c,
            0,
        ]);
        (slot, id)
    });
    PARENT.with(|p| p.set(id));
    Scope {
        slot: Some(slot),
        id,
        old,
        serial,
        _local: PhantomData,
    }
}
#[inline]
pub fn serial(kind: u64, a: usize, b: usize) -> Scope {
    scope(parent(), kind, a, b, true)
}
pub fn root(kind: u64, a: usize, b: usize) -> Scope {
    scope(API.load(Ordering::Relaxed), kind, a, b, false)
}
pub fn kernel(kind: u64, a: usize, b: usize) -> Scope {
    scope(parent(), kind, a, b, false)
}
pub fn api() -> Scope {
    let s = scope(0, 0, 0, 0, false);
    API.store(s.id(), Ordering::Relaxed);
    s
}
fn clear() {
    PARENT.with(|x| {
        assert_eq!(x.get(), 0);
    });
    DROPPED.with(|x| x.set(0));
    EVENTS.with(|v| {
        let mut v = v.borrow_mut();
        v.clear();
        if v.capacity() < LIMIT {
            v.reserve_exact(LIMIT);
            v.resize(LIMIT, [0; 10]);
            v.clear();
        }
    });
}
pub fn reset(enabled: bool) {
    ENABLED.store(false, Ordering::SeqCst);
    CLOCK.get_or_init(Instant::now);
    rayon::broadcast(|_| clear());
    clear();
    API.store(0, Ordering::Relaxed);
    ORIGIN.store(
        CLOCK.get().unwrap().elapsed().as_nanos() as u64,
        Ordering::Relaxed,
    );
    ENABLED.store(enabled, Ordering::SeqCst);
}
pub fn snapshot() -> Vec<(Vec<Event>, u64)> {
    ENABLED.store(false, Ordering::SeqCst);
    let take = || {
        PARENT.with(|x| assert_eq!(x.get(), 0));
        (EVENTS.with(|v| v.borrow().clone()), DROPPED.with(Cell::get))
    };
    let mut all = rayon::broadcast(|_| take());
    all.push(take());
    all
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_parent_cpu_disabled_and_unwind() {
        reset(true);
        let api_id;
        {
            let api = api();
            api_id = api.id();
            let root_scope = root(1, 7, 8);
            let id = root_scope.id();
            rayon::broadcast(|_| {
                let _part = scope(id, 13, 0, 0, false);
                let _leaf = serial(30, 8, 0);
                std::hint::black_box((0..8192u64).fold(0u64, |a, b| a.wrapping_add(b)));
            });
            {
                let query = root(6, 0, 0);
                let _leaf = serial(23, 0, 0);
                assert_ne!(query.id(), id);
            }
            let result = std::panic::catch_unwind(|| {
                let _leaf = serial(32, 8, 0);
                panic!("unwind");
            });
            assert!(result.is_err());
            assert!(api.id() != 0);
        }
        let all = snapshot();
        let events: Vec<_> = all.iter().flat_map(|(v, _)| v.iter()).collect();
        assert!(all.iter().all(|(_, d)| *d == 0));
        let ids: std::collections::HashSet<_> = events.iter().map(|e| e[0]).collect();
        assert_eq!(ids.len(), events.len());
        for e in &events {
            assert!(e[6] >= e[5]);
            assert_eq!(e[0] >> 32, e[7]);
            if e[1] != 0 {
                let p = events.iter().find(|p| p[0] == e[1]).unwrap();
                assert!(p[5] <= e[5] && e[6] <= p[6]);
            }
            if e[8] != 0 {
                assert!(e[9] >= e[8]);
            }
            if e[2] == 6 {
                assert_eq!(e[1], api_id);
            }
        }
        reset(false);
        {
            let _a = api();
            let _b = serial(30, 1, 0);
        }
        assert!(snapshot().iter().all(|(v, d)| v.is_empty() && *d == 0));
    }
    #[test]
    fn bounded_capacity_preserves_execution_and_reports_drops() {
        reset(true);
        for _ in 0..LIMIT + 3 {
            let _s = scope(0, 90, 0, 0, false);
        }
        let all = snapshot();
        let main = all.last().unwrap();
        assert_eq!(main.0.len(), LIMIT);
        assert_eq!(main.1, 3);
        reset(false);
    }
}
