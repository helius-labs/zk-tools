use serde_json::json;
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Instant,
};
use zopro_prover::zopro_core::{self, TreeLayerProbe};
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
    let mixed = args[2] == "1";
    let seed: u64 = args[3].parse()?;
    let paired = args[4] == "paired";
    let allowed = affinity();
    assert_eq!(allowed, vec![2]);
    let tid = unsafe { libc::syscall(libc::SYS_gettid) };
    let mut control = perf::Control::from_env()?;
    if paired {
        assert!(control.is_none());
    }
    let mut fixture = TreeLayerProbe::new(pairs, mixed, seed);
    assert!(fixture.vector_inverse());
    for mode in [0, 1, 2] {
        for _ in 0..3 {
            fixture.reset();
            fixture.layer(mode);
            fixture.check();
        }
    }
    let iterations = if paired {
        (16384 / pairs).max(64)
    } else {
        if pairs == 16 {
            131072
        } else {
            8192
        }
    };
    let stages = if paired { 21 } else { 1 };
    let mut rows = Vec::with_capacity(stages);
    for block in 0..if paired { 7 } else { 1 } {
        let mut modes = if paired {
            vec![0u8, 1, 2]
        } else {
            vec![match args[4].as_str() {
                "control-a" => 0,
                "prefix" => 2,
                _ => panic!("mode"),
            }]
        };
        if paired {
            modes.rotate_left((block + pairs + usize::from(mixed) + seed as usize) % 3);
            if block % 2 == 1 {
                modes.reverse();
            }
        }
        for mode in modes {
            for _ in 0..3 {
                fixture.reset();
                fixture.layer(mode);
                fixture.check();
            }
            let usage0 = perf::Usage::now()?;
            if let Some(c) = control.as_mut() {
                c.enable()?;
            }
            let allocations = CALLS.load(Relaxed);
            let mut kernel_ns = 0u128;
            let wall = Instant::now();
            for _ in 0..iterations {
                fixture.reset();
                let start = Instant::now();
                black_box(&mut fixture).layer(mode);
                kernel_ns += start.elapsed().as_nanos();
                black_box(&fixture);
            }
            let caller_ns = wall.elapsed().as_nanos();
            let allocations = CALLS.load(Relaxed) - allocations;
            if let Some(c) = control.as_mut() {
                c.disable()?;
            }
            let usage1 = perf::Usage::now()?;
            fixture.check();
            assert_eq!(allocations, 0);
            assert_eq!(affinity(), allowed);
            assert_eq!(unsafe { libc::syscall(libc::SYS_gettid) }, tid);
            rows.push((
                block,
                mode,
                kernel_ns,
                caller_ns,
                allocations,
                usage0,
                usage1,
            ));
        }
    }
    assert_eq!(rows.len(), stages);
    println!(
        "AFFINITY {}",
        json!({"before":allowed,"after":affinity(),"tid":tid,"cpu":unsafe{libc::sched_getcpu()}})
    );
    for (block, mode, kernel_ns, caller_ns, allocations, usage0, usage1) in rows {
        let mut out = usage1.difference(&usage0);
        out["block"] = json!(block);
        out["mode"] = json!(["control-a", "control-b", "prefix"][mode as usize]);
        out["requested_pairs"] = json!(pairs);
        out["ordinary_pairs"] = json!(fixture.ordinary_pairs());
        out["point_slots"] = json!(fixture.point_slots());
        out["mixed"] = json!(mixed);
        out["seed"] = json!(seed);
        out["iterations"] = json!(iterations);
        out["kernel_ns_per_layer"] = json!(kernel_ns as f64 / iterations as f64);
        out["caller_ns_per_layer"] = json!(caller_ns as f64 / iterations as f64);
        out["timed_allocation_calls"] = json!(allocations);
        out["record_capacities_bytes_both_retained"] = json!(fixture.record_capacities());
        out["all_output_storage_and_live_counts_correct"] = json!(true);
        println!("STAGE {out}");
    }
    Ok(())
}
