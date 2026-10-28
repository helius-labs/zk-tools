use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Instant,
};
use zopro_core::{Caller, CallerG1, CallerG2};
#[path = "perf.rs"]
mod perf;
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
struct Counted;
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.alloc_zeroed(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.realloc(p, l, n) }
    }
}
#[global_allocator]
static A: Counted = Counted;
fn measure<T: Caller>(f: &mut T, kind: &str, n: usize, mode: usize, rounds: usize) {
    let mut control = perf::Control::from_env().unwrap();
    for _ in 0..3 {
        f.reset();
        f.run(1024);
        f.check(1024);
    }
    f.reset();
    if let Some(c) = control.as_mut() {
        c.enable().unwrap();
    }
    let usage = perf::Usage::now().unwrap();
    let before = ALLOCS.load(Relaxed);
    let start = Instant::now();
    black_box(&mut *f).run(rounds);
    let elapsed = start.elapsed().as_nanos();
    let allocations = ALLOCS.load(Relaxed) - before;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = control.as_mut() {
        c.disable().unwrap();
    }
    assert_eq!(allocations, 0);
    f.check(rounds);
    println!(
        "RESULT {}",
        json!({"kind":kind,"size":n,"mode":mode,"iterations":rounds,"prime_calls":3,"prime_iterations":1024,"ns_per_batch":elapsed as f64/rounds as f64,"elapsed_ns":elapsed,"timed_allocations":allocations,"capacity_bytes":f.capacity_bytes(),"fixture_reference_bytes":f.reference_bytes(),"internally_verified":true,"output_sha256":format!("{:x}",Sha256::digest(f.bytes())),"usage":after.difference(&usage),"SDK_requests":0})
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    let n = a[2].parse().unwrap();
    let mode: usize = a[3].parse().unwrap();
    let rounds = a[4].parse().unwrap();
    assert!([200, 352].contains(&n));
    assert!(mode <= 2);
    assert!([4096, 262144].contains(&rounds));
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    match a[1].as_str() {
        "g1-owned" => {
            let mut f = CallerG1::new(n, false, 17);
            f.set_weighted(mode == 2);
            measure(&mut f, &a[1], n, mode, rounds)
        }
        "g1-borrowed-signed" => {
            let mut f = CallerG1::new(n, true, 17);
            f.set_weighted(mode == 2);
            f.with_borrowed(|b| measure(b, &a[1], n, mode, rounds));
        }
        "g2-prefix" => {
            let mut f = CallerG2::new(n, 17);
            measure(&mut f, &a[1], n, mode, rounds)
        }
        _ => panic!("kind"),
    }
}
