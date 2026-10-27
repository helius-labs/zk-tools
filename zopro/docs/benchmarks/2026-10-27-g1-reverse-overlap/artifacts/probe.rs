use serde_json::json;
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Instant,
};
use zopro_prover::zopro_core::{self, G1StreamProbe};
#[path = "support/perf.rs"]
mod perf;
static CALLS: AtomicUsize = AtomicUsize::new(0);
struct Counted;
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        CALLS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        CALLS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.alloc_zeroed(l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        CALLS.fetch_add(1, Relaxed);
        unsafe { zopro_core::alloc::Allocator.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
}
#[global_allocator]
static ALLOCATOR: Counted = Counted;
fn affinity() -> Vec<usize> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set),
            0
        );
        (0..libc::CPU_SETSIZE as usize)
            .filter(|&i| libc::CPU_ISSET(i, &set))
            .collect()
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 5);
    let pairs: usize = args[1].parse()?;
    let signed = args[2] == "1";
    let seed: u64 = args[3].parse()?;
    let paired = args[4] == "paired";
    let allowed = affinity();
    assert_eq!(allowed, vec![2]);
    let tid = unsafe { libc::syscall(libc::SYS_gettid) };
    let mut ctl = perf::Control::from_env()?;
    if paired {
        assert!(ctl.is_none());
    }
    let mut fixture = G1StreamProbe::new(pairs, signed, seed);
    assert_eq!(fixture.settings(), [true, true, true, true]);
    fixture.set_expected(3);
    for mode in [0, 1, 2] {
        fixture.reset();
        fixture.run(mode, 3);
        fixture.check(3);
    }
    let iterations = if paired { 1024 } else { 131072 };
    fixture.set_expected(iterations);
    let mut rows = Vec::with_capacity(if paired { 21 } else { 1 });
    for block in 0..if paired { 7 } else { 1 } {
        let mut modes = if paired {
            vec![0u8, 1, 2]
        } else {
            vec![match args[4].as_str() {
                "control-a" => 0,
                "overlap" => 2,
                _ => panic!("profile mode"),
            }]
        };
        if paired {
            modes.rotate_left((block + pairs + seed as usize + usize::from(signed)) % 3);
            if block % 2 == 1 {
                modes.reverse();
            }
        }
        for mode in modes {
            fixture.reset();
            fixture.run(mode, 3);
            fixture.reset();
            let u0 = perf::Usage::now()?;
            if let Some(c) = ctl.as_mut() {
                c.enable()?;
            }
            let allocations = CALLS.load(Relaxed);
            let t = Instant::now();
            black_box(&mut fixture).run(mode, iterations);
            let elapsed_ns = t.elapsed().as_nanos();
            black_box(&fixture);
            let allocations = CALLS.load(Relaxed) - allocations;
            if let Some(c) = ctl.as_mut() {
                c.disable()?;
            }
            let u1 = perf::Usage::now()?;
            fixture.check(iterations);
            assert_eq!(allocations, 0);
            assert_eq!(affinity(), allowed);
            assert_eq!(unsafe { libc::syscall(libc::SYS_gettid) }, tid);
            rows.push((block, mode, elapsed_ns, allocations, u0, u1));
        }
    }
    println!(
        "AFFINITY {}",
        json!({"before":allowed,"after":affinity(),"tid":tid,"cpu":unsafe{libc::sched_getcpu()}})
    );
    for (block, mode, elapsed_ns, allocations, u0, u1) in rows {
        let mut out = u1.difference(&u0);
        out["block"] = json!(block);
        out["mode"] = json!(["control-a", "control-b", "overlap"][mode as usize]);
        out["pairs"] = json!(pairs);
        out["signed"] = json!(signed);
        out["seed"] = json!(seed);
        out["iterations"] = json!(iterations);
        out["kernel_ns_per_layer"] = json!(elapsed_ns as f64 / iterations as f64);
        out["caller_ns_per_layer"] = json!(elapsed_ns as f64 / iterations as f64);
        out["timed_allocation_calls"] = json!(allocations);
        out["record_capacity_bytes"] = json!(fixture.record_capacity_bytes());
        out["complete_storage_and_counts_correct"] = json!(true);
        println!("STAGE {out}");
    }
    Ok(())
}
