use serde_json::json;
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Instant,
};
use zopro_bn254::{G1Config, G2Config};
use zopro_prover::zopro_core::{self, ShortFixture, ShortLayerProbe};
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
fn run<P: ShortFixture>(
    pairs: usize,
    seed: u64,
    paired: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let allowed = affinity();
    assert_eq!(allowed, vec![2]);
    let tid = unsafe { libc::syscall(libc::SYS_gettid) };
    let mut ctl = perf::Control::from_env()?;
    if paired {
        assert!(ctl.is_none());
    }
    let mut fixtures: Vec<_> = (0..16)
        .map(|i| P::new(pairs, false, seed * 16 + i))
        .collect();
    for f in &mut fixtures {
        assert!(f.vector_inverse());
        assert_eq!(f.ordinary_pairs(), pairs);
        for mode in [0, 1, 2] {
            for _ in 0..3 {
                f.reset();
                f.layer(mode);
                f.check();
            }
        }
    }
    let iterations = if paired { 4096 } else { 65536 };
    let mut rows = Vec::with_capacity(if paired { 21 } else { 1 });
    for block in 0..if paired { 7 } else { 1 } {
        let mut modes = if paired { vec![0u8, 1, 2] } else { vec![2] };
        if paired {
            modes.rotate_left((block + pairs + seed as usize) % 3);
            if block % 2 == 1 {
                modes.reverse();
            }
        }
        for mode in modes {
            for f in &mut fixtures {
                for _ in 0..3 {
                    f.reset();
                    f.layer(mode);
                }
            }
            let u0 = perf::Usage::now()?;
            if let Some(c) = ctl.as_mut() {
                c.enable()?;
            }
            let allocations = CALLS.load(Relaxed);
            let mut kernel_ns = 0u128;
            let start = Instant::now();
            for i in 0..iterations {
                let f = &mut fixtures[i % 16];
                f.reset();
                let t = Instant::now();
                black_box(&mut *f).layer(mode);
                kernel_ns += t.elapsed().as_nanos();
                black_box(&*f);
            }
            let caller_ns = start.elapsed().as_nanos();
            let allocations = CALLS.load(Relaxed) - allocations;
            if let Some(c) = ctl.as_mut() {
                c.disable()?;
            }
            let u1 = perf::Usage::now()?;
            for f in &fixtures {
                f.check();
            }
            assert_eq!(allocations, 0);
            assert_eq!(affinity(), allowed);
            assert_eq!(unsafe { libc::syscall(libc::SYS_gettid) }, tid);
            rows.push((block, mode, kernel_ns, caller_ns, allocations, u0, u1));
        }
    }
    println!(
        "AFFINITY {}",
        json!({"before":allowed,"after":affinity(),"tid":tid,"cpu":unsafe{libc::sched_getcpu()}})
    );
    for (block, mode, kernel_ns, caller_ns, allocations, u0, u1) in rows {
        let mut out = u1.difference(&u0);
        out["block"] = json!(block);
        out["mode"] = json!(["control-a", "control-b", "short"][mode as usize]);
        out["pairs"] = json!(pairs);
        out["seed"] = json!(seed);
        out["iterations"] = json!(iterations);
        out["kernel_ns_per_layer"] = json!(kernel_ns as f64 / iterations as f64);
        out["caller_ns_per_layer"] = json!(caller_ns as f64 / iterations as f64);
        out["timed_allocation_calls"] = json!(allocations);
        out["record_capacity_bytes"] = json!(fixtures
            .iter()
            .map(|f| f.record_capacity_bytes())
            .sum::<usize>());
        out["complete_storage_and_counts_correct"] = json!(true);
        println!("STAGE {out}");
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 5);
    let pairs = args[2].parse()?;
    let seed = args[3].parse()?;
    let paired = args[4] == "paired";
    match args[1].as_str() {
        "g1" => run::<ShortLayerProbe<G1Config>>(pairs, seed, paired),
        "g2" => run::<ShortLayerProbe<G2Config>>(pairs, seed, paired),
        _ => panic!("curve"),
    }
}
