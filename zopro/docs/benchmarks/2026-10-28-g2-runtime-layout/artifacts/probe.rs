use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    collections::BTreeMap,
    hint::black_box,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
        Mutex,
    },
    time::Instant,
};
use zopro_core::{Caller, CallerG1, CallerG2};
#[path = "perf.rs"]
mod perf;

const PAGE: usize = 4096;
const GUARD: usize = 16;
static MODE: AtomicUsize = AtomicUsize::new(0);
static TARGET: AtomicUsize = AtomicUsize::new(0);
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
static LOGGING: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy)]
struct Event {
    ptr: usize,
    size: usize,
    align: usize,
    op: u8,
}
impl Event {
    const ZERO: Self = Self {
        ptr: 0,
        size: 0,
        align: 0,
        op: 0,
    };
}
#[derive(Clone, Copy)]
struct Journal {
    len: usize,
    overflow: bool,
    items: [Event; 512],
}
static JOURNAL: Mutex<Journal> = Mutex::new(Journal {
    len: 0,
    overflow: false,
    items: [Event::ZERO; 512],
});
#[derive(Clone, Copy)]
struct Block {
    ptr: usize,
    base: usize,
    size: usize,
    backing: usize,
}
impl Block {
    const ZERO: Self = Self {
        ptr: 0,
        base: 0,
        size: 0,
        backing: 0,
    };
}
static FORCED: Mutex<[Block; 64]> = Mutex::new([Block::ZERO; 64]);

fn record(ptr: *mut u8, l: Layout, op: u8) {
    if ptr.is_null() || l.size() < 1024 || !LOGGING.load(Relaxed) {
        return;
    }
    if let Ok(mut journal) = JOURNAL.lock() {
        let i = journal.len;
        if i == journal.items.len() {
            journal.overflow = true;
            return;
        }
        journal.items[i] = Event {
            ptr: ptr as usize,
            size: l.size(),
            align: l.align(),
            op,
        };
        journal.len += 1;
    }
}
fn intercepted(l: Layout) -> bool {
    MODE.load(Relaxed) != 0
        && TARGET.load(Relaxed) > 0
        && l.size() == TARGET.load(Relaxed)
        && l.align() == 64
}
unsafe fn backing(size: usize, offset: usize, zero: bool) -> Option<Block> {
    let bytes = size.checked_add(PAGE + 2 * GUARD)?;
    let layout = Layout::from_size_align(bytes, 64).ok()?;
    // SAFETY: System owns this full backing block; payload and both guards are
    // disjoint, and offset is one of the predeclared 64-byte-aligned page offsets.
    let base = unsafe { System.alloc(layout) };
    if base.is_null() {
        return None;
    }
    let first = base as usize + GUARD;
    let delta = (offset + PAGE - (first & (PAGE - 1))) & (PAGE - 1);
    let ptr = first + delta;
    unsafe {
        std::ptr::write_bytes(base, 0xa5, GUARD);
        std::ptr::write_bytes(base.add(bytes - GUARD), 0x5a, GUARD);
        if zero {
            std::ptr::write_bytes(ptr as *mut u8, 0, size);
        }
    }
    Some(Block {
        ptr,
        base: base as usize,
        size,
        backing: bytes,
    })
}
unsafe fn valid(b: Block) -> bool {
    // SAFETY: a registry entry owns the entire still-live System allocation.
    unsafe {
        (0..GUARD).all(|i| {
            *((b.base + i) as *const u8) == 0xa5
                && *((b.base + b.backing - GUARD + i) as *const u8) == 0x5a
        })
    }
}
unsafe fn release(b: Block) {
    if !unsafe { valid(b) } {
        std::process::abort()
    }
    let Ok(layout) = Layout::from_size_align(b.backing, 64) else {
        std::process::abort()
    };
    unsafe { System.dealloc(b.base as *mut u8, layout) }
}
fn owned(ptr: *mut u8) -> bool {
    match FORCED.lock() {
        Ok(reg) => reg.iter().any(|b| b.ptr == ptr as usize),
        Err(_) => std::process::abort(),
    }
}
fn take(ptr: *mut u8) -> Option<Block> {
    let Ok(mut reg) = FORCED.lock() else {
        std::process::abort()
    };
    let i = reg.iter().position(|b| b.ptr == ptr as usize)?;
    Some(std::mem::replace(&mut reg[i], Block::ZERO))
}
struct Observed;
impl Observed {
    unsafe fn allocate(&self, l: Layout, zero: bool) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        let ptr = if intercepted(l) {
            let offset = if MODE.load(Relaxed) == 1 { 0 } else { 2048 };
            let Some(b) = (unsafe { backing(l.size(), offset, zero) }) else {
                return std::ptr::null_mut();
            };
            let Ok(mut reg) = FORCED.lock() else {
                unsafe { release(b) };
                return std::ptr::null_mut();
            };
            if let Some(i) = reg.iter().position(|b| b.ptr == 0) {
                reg[i] = b;
                b.ptr as *mut u8
            } else {
                unsafe { release(b) };
                std::ptr::null_mut()
            }
        } else if zero {
            unsafe { zopro_core::alloc::Allocator.alloc_zeroed(l) }
        } else {
            unsafe { zopro_core::alloc::Allocator.alloc(l) }
        };
        record(ptr, l, 1);
        ptr
    }
}
// SAFETY: natural blocks retain the original allocator/layout. Forced blocks
// are tracked by pointer, so deallocation never depends on later mode changes.
// Reallocation copies only the smaller live extent and preserves old ownership
// on allocation failure. No path unwinds through GlobalAlloc.
unsafe impl GlobalAlloc for Observed {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        unsafe { self.allocate(l, false) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        unsafe { self.allocate(l, true) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        record(p, l, 2);
        if l.align() == 64 {
            if let Some(b) = take(p) {
                if b.size != l.size() {
                    std::process::abort()
                }
                unsafe { release(b) };
                return;
            }
        }
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, size: usize) -> *mut u8 {
        let Ok(next) = Layout::from_size_align(size, l.align()) else {
            return std::ptr::null_mut();
        };
        if intercepted(next) || (l.align() == 64 && owned(p)) {
            let q = unsafe { self.allocate(next, false) };
            if q.is_null() {
                return q;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(p, q, l.size().min(size));
                self.dealloc(p, l)
            };
            q
        } else {
            ALLOCS.fetch_add(1, Relaxed);
            let q = unsafe { zopro_core::alloc::Allocator.realloc(p, l, size) };
            if !q.is_null() {
                record(p, l, 2);
                record(q, next, 1)
            }
            q
        }
    }
}
#[global_allocator]
static ALLOCATOR: Observed = Observed;
fn configure(mode: usize, size: usize) {
    assert!(mode <= 2);
    assert!(FORCED.lock().unwrap().iter().all(|b| b.ptr == 0));
    LOGGING.store(false, Relaxed);
    MODE.store(mode, Relaxed);
    TARGET.store(size, Relaxed);
    *JOURNAL.lock().unwrap() = Journal {
        len: 0,
        overflow: false,
        items: [Event::ZERO; 512],
    };
    LOGGING.store(true, Relaxed);
}
fn snapshot(
    kind: &str,
    size: usize,
    mode: usize,
    fixture: usize,
    marker: usize,
) -> serde_json::Value {
    let j = *JOURNAL.lock().unwrap();
    assert!(!j.overflow);
    let mut live = BTreeMap::new();
    let mut preexisting_frees = 0usize;
    for e in &j.items[..j.len] {
        if e.op == 1 {
            assert!(live.insert(e.ptr, *e).is_none())
        } else if live.remove(&e.ptr).is_none() {
            preexisting_frees += 1;
        }
    }
    let targets: Vec<_> = live
        .values()
        .filter(|e| e.size == size / 8 * 640 && e.align == 64)
        .collect();
    assert_eq!(targets.len(), if kind == "g2" { 1 } else { 0 });
    if let Some(prefix) = targets.first() {
        if mode != 0 {
            assert_eq!(prefix.ptr % PAGE, if mode == 1 { 0 } else { 2048 });
        }
    }
    let prefix = targets.first().map(|e| e.ptr);
    let buffers:Vec<_>=live.values().map(|e|json!({"size":e.size,"align":e.align,"page_offset":e.ptr%PAGE,"relative_pages_to_prefix":prefix.map(|p|(e.ptr/PAGE)as i64-(p/PAGE)as i64),"prefix_request":prefix==Some(e.ptr)})).collect();
    json!({"events":j.len,"overflow":false,"preexisting_allocation_frees":preexisting_frees,"fixture_page_offset":fixture%PAGE,"stack_marker_page_offset":marker%PAGE,"prefix_page_offset":prefix.map(|p|p%PAGE),"live_buffers":buffers,"forced_blocks_live":FORCED.lock().unwrap().iter().filter(|b|b.ptr!=0).count()})
}
fn measure<T: Caller>(
    mut f: T,
    kind: &str,
    size: usize,
    mode: usize,
    rounds: usize,
    mut control: Option<perf::Control>,
) {
    for _ in 0..3 {
        f.reset();
        black_box(&mut f).run(1024);
        f.check(1024);
    }
    f.reset();
    LOGGING.store(false, Relaxed);
    let marker = black_box(0u64);
    let fixture = std::ptr::from_ref(&f) as usize;
    let marker_ptr = std::ptr::from_ref(&marker) as usize;
    if let Some(c) = control.as_mut() {
        c.enable().unwrap()
    }
    let usage = perf::Usage::now().unwrap();
    let count = ALLOCS.load(Relaxed);
    let start = Instant::now();
    black_box(&mut f).run(rounds);
    let elapsed = start.elapsed().as_nanos();
    let allocations = ALLOCS.load(Relaxed) - count;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = control.as_mut() {
        c.disable().unwrap()
    }
    assert_eq!(allocations, 0);
    let layout = snapshot(kind, size, mode, fixture, marker_ptr);
    f.check(rounds);
    println!(
        "RESULT {}",
        json!({"kind":kind,"size":size,"mode":mode,"iterations":rounds,"prime_iterations":1024,"prime_calls":3,"ns_per_batch":elapsed as f64/rounds as f64,"elapsed_ns":elapsed,"timed_allocations":allocations,"capacity_bytes":f.capacity_bytes(),"internally_verified":true,"output_sha256":format!("{:x}",Sha256::digest(f.bytes())),"layout":layout,"usage":after.difference(&usage),"SDK_requests":0})
    );
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 5);
    let size: usize = args[2].parse().unwrap();
    let mode: usize = args[3].parse().unwrap();
    let rounds: usize = args[4].parse().unwrap();
    assert!([200, 352].contains(&size));
    assert!([16384, 262144].contains(&rounds));
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    let control = perf::Control::from_env().unwrap();
    configure(mode, size / 8 * 640);
    match args[1].as_str() {
        "g2" => measure(CallerG2::new(size, 17), "g2", size, mode, rounds, control),
        "g1" => {
            assert_eq!((size, mode), (200, 0));
            measure(
                CallerG1::new(size, false, 17),
                "g1",
                size,
                mode,
                rounds,
                control,
            )
        }
        _ => panic!("kind"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backing_offsets_zeroing_and_bounds() {
        for size in [16000, 28160] {
            for offset in [0, 2048] {
                for zero in [false, true] {
                    let b = unsafe { backing(size, offset, zero) }.unwrap();
                    assert_eq!(b.ptr % PAGE, offset);
                    if zero {
                        assert!((0..size).all(|i| unsafe { *((b.ptr + i) as *const u8) == 0 }));
                    }
                    for i in 0..size {
                        unsafe {
                            *((b.ptr + i) as *mut u8) = (i * 137) as u8;
                        }
                    }
                    assert!(unsafe { valid(b) });
                    unsafe { release(b) };
                }
            }
        }
    }
    #[test]
    fn forced_resize_and_real_callers_preserve_outputs() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .unwrap();
        for size in [16000, 28160] {
            for mode in [1, 2] {
                configure(mode, size);
                let l = Layout::from_size_align(size, 64).unwrap();
                let p = unsafe { ALLOCATOR.alloc_zeroed(l) };
                assert!(!p.is_null());
                for i in 0..size {
                    unsafe {
                        *p.add(i) = (i * 113) as u8;
                    }
                }
                let q = unsafe { ALLOCATOR.realloc(p, l, size * 2) };
                assert!(!q.is_null());
                for i in 0..size {
                    assert_eq!(unsafe { *q.add(i) }, (i * 113) as u8);
                }
                let twice = Layout::from_size_align(size * 2, 64).unwrap();
                let p = unsafe { ALLOCATOR.realloc(q, twice, size) };
                assert!(!p.is_null());
                for i in 0..size {
                    assert_eq!(unsafe { *p.add(i) }, (i * 113) as u8);
                }
                unsafe { ALLOCATOR.dealloc(p, l) };
                LOGGING.store(false, Relaxed);
            }
        }
        for size in [200, 352] {
            for mode in [0, 1, 2] {
                configure(mode, size / 8 * 640);
                let mut f = CallerG2::new(size, 17);
                f.run(32);
                LOGGING.store(false, Relaxed);
                let _ = snapshot("g2", size, mode, std::ptr::from_ref(&f) as usize, 0);
                f.check(32);
                drop(f);
            }
        }
        configure(0, 16000);
        let mut f = CallerG1::new(200, false, 17);
        f.run(32);
        LOGGING.store(false, Relaxed);
        let _ = snapshot("g1", 200, 0, std::ptr::from_ref(&f) as usize, 0);
        f.check(32);
        drop(f);
        configure(0, 0);
        LOGGING.store(false, Relaxed);
    }
}
