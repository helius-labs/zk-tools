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
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
}
#[global_allocator]
static A: Counted = Counted;
fn measure<T: Caller>(f: &mut T, kind: &str, size: usize, rounds: usize) {
    let mut profile = perf::Control::from_env().unwrap();
    for _ in 0..3 {
        f.reset();
        black_box(&mut *f).run(8);
        f.check(8);
    }
    f.reset();
    if let Some(p) = profile.as_mut() {
        p.enable().unwrap();
    }
    let before = perf::Usage::now().unwrap();
    let allocations = ALLOCS.load(Relaxed);
    let started = Instant::now();
    black_box(&mut *f).run(rounds);
    let elapsed = started.elapsed().as_nanos();
    let allocations = ALLOCS.load(Relaxed) - allocations;
    let after = perf::Usage::now().unwrap();
    if let Some(p) = profile.as_mut() {
        p.disable().unwrap();
    }
    assert_eq!(allocations, 0);
    f.check(rounds);
    println!(
        "RESULT {}",
        json!({"kind":kind,"size":size,"iterations":rounds,"prime_calls":3,"prime_iterations":8,"elapsed_ns":elapsed,"ns_per_round":elapsed as f64/rounds as f64,"timed_allocations":allocations,"capacity_bytes":f.capacity_bytes(),"fixture_reference_bytes":f.reference_bytes(),"output_sha256":format!("{:x}",Sha256::digest(f.bytes())),"internally_verified":true,"usage":after.difference(&before),"SDK_requests":0})
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let size = a[2].parse().unwrap();
    let rounds = a[3].parse().unwrap();
    assert!(rounds > 0);
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    match a[1].as_str() {
        "g1-owned" => measure(&mut CallerG1::new(size, false, 17), &a[1], size, rounds),
        "g1-borrowed-signed" => {
            CallerG1::new(size, true, 17).with_borrowed(|f| measure(f, &a[1], size, rounds))
        }
        "g2-prefix" => measure(&mut CallerG2::new(size, 17), &a[1], size, rounds),
        _ => panic!("kind"),
    }
}
