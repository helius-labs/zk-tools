use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Barrier, Once,
    },
    time::Instant,
};
use zopro_core::{Caller, CallerG1, CallerG2};
#[path = "perf.rs"]
mod perf;
struct Counted;
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
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
fn pool() {
    static P: Once = Once::new();
    P.call_once(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .unwrap()
    });
}
fn cpus(topology: &str) -> Vec<usize> {
    match topology {
        "solo" => vec![2],
        "smt" => vec![2, 18],
        "separate" => vec![2, 3],
        "physical16" => (0..16).collect(),
        "logical32" => (0..32).collect(),
        _ => panic!("topology"),
    }
}
fn affinity(cpu: usize) {
    unsafe {
        let mut mask: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut mask);
        libc::CPU_SET(cpu, &mut mask);
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&mask), &mask),
            0
        );
        let mut actual: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&actual), &mut actual),
            0
        );
        for n in 0..libc::CPU_SETSIZE as usize {
            assert_eq!(libc::CPU_ISSET(n, &actual), n == cpu);
        }
        assert_eq!(libc::sched_getcpu(), cpu as i32);
    }
}
fn cpu_now() -> u64 {
    unsafe {
        let mut t: libc::timespec = std::mem::zeroed();
        assert_eq!(
            libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut t),
            0
        );
        t.tv_sec as u64 * 1_000_000_000 + t.tv_nsec as u64
    }
}
struct Gates {
    ready: Barrier,
    start: Barrier,
    done: Barrier,
    release: Barrier,
}
#[derive(Debug)]
struct Worker {
    cpu: usize,
    tid: i64,
    elapsed_ns: u64,
    cpu_ns: u64,
    capacity_bytes: usize,
    reference_bytes: usize,
    digest: String,
}
fn worker<T: Caller>(f: &mut T, cpu: usize, rounds: usize, prime: usize, g: &Gates) -> Worker {
    for _ in 0..3 {
        f.reset();
        f.run(prime);
        f.check(prime);
    }
    f.reset();
    let capacity = f.capacity_bytes();
    let refs = f.reference_bytes();
    let tid = unsafe { libc::syscall(libc::SYS_gettid) };
    g.ready.wait();
    g.start.wait();
    assert_eq!(unsafe { libc::sched_getcpu() }, cpu as i32);
    let c0 = cpu_now();
    let start = Instant::now();
    black_box(&mut *f).run(rounds);
    let elapsed = start.elapsed().as_nanos() as u64;
    let c1 = cpu_now();
    assert_eq!(unsafe { libc::sched_getcpu() }, cpu as i32);
    g.done.wait();
    g.release.wait();
    f.check(rounds);
    assert_eq!(capacity, f.capacity_bytes());
    assert_eq!(refs, f.reference_bytes());
    Worker {
        cpu,
        tid,
        elapsed_ns: elapsed,
        cpu_ns: c1 - c0,
        capacity_bytes: capacity,
        reference_bytes: refs,
        digest: format!("{:x}", Sha256::digest(f.bytes())),
    }
}
fn run(kind: &str, n: usize, mode: usize, topology: &str, rounds: usize, prime: usize) -> Value {
    pool();
    let cpus = cpus(topology);
    let count = cpus.len();
    let gates = Gates {
        ready: Barrier::new(count + 1),
        start: Barrier::new(count + 1),
        done: Barrier::new(count + 1),
        release: Barrier::new(count + 1),
    };
    let mut control = perf::Control::from_env().unwrap();
    let (wall_ns, usage, allocations, workers) = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(count);
        for &cpu in &cpus {
            let g = &gates;
            handles.push(scope.spawn(move || {
                affinity(cpu);
                match kind {
                    "g1-owned" => {
                        let mut f = CallerG1::new(n, false, 17);
                        f.set_weighted(mode == 2);
                        worker(&mut f, cpu, rounds, prime, g)
                    }
                    "g1-borrowed-signed" => {
                        let mut f = CallerG1::new(n, true, 17);
                        f.set_weighted(mode == 2);
                        f.with_borrowed(|b| worker(b, cpu, rounds, prime, g))
                    }
                    "g2-prefix" => {
                        let mut f = CallerG2::new(n, 17);
                        worker(&mut f, cpu, rounds, prime, g)
                    }
                    _ => panic!("kind"),
                }
            }));
        }
        gates.ready.wait();
        if let Some(c) = control.as_mut() {
            c.enable().unwrap();
        }
        let usage = perf::Usage::now().unwrap();
        let before = ALLOCS.load(Relaxed);
        let start = Instant::now();
        gates.start.wait();
        gates.done.wait();
        let wall = start.elapsed().as_nanos() as u64;
        let allocations = ALLOCS.load(Relaxed) - before;
        let after = perf::Usage::now().unwrap();
        if let Some(c) = control.as_mut() {
            c.disable().unwrap();
        }
        gates.release.wait();
        let workers: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        (wall, after.difference(&usage), allocations, workers)
    });
    assert_eq!(allocations, 0, "allocation inside common timing interval");
    let first = &workers[0];
    for w in &workers {
        assert_eq!(w.digest, first.digest);
        assert_eq!(w.capacity_bytes, first.capacity_bytes);
        assert_eq!(w.reference_bytes, first.reference_bytes);
        assert!(w.elapsed_ns <= wall_ns);
    }
    let values:Vec<_>=workers.iter().map(|w|json!({"cpu":w.cpu,"tid":w.tid,"elapsed_ns":w.elapsed_ns,"ns_per_batch":w.elapsed_ns as f64/rounds as f64,"thread_cpu_ns":w.cpu_ns,"capacity_bytes":w.capacity_bytes,"reference_bytes":w.reference_bytes,"digest":w.digest})).collect();
    let sum_cpu: u64 = workers.iter().map(|w| w.cpu_ns).sum();
    let sum_wall: u64 = workers.iter().map(|w| w.elapsed_ns).sum();
    json!({"kind":kind,"size":n,"mode":mode,"topology":topology,"cpus":cpus,"workers":values,"worker_count":count,"iterations_per_worker":rounds,"total_kernel_calls":count*rounds,"prime_calls":3,"prime_iterations_per_worker":prime,"wall_ns":wall_ns,"wall_ns_per_wave":wall_ns as f64/rounds as f64,"throughput_ns_per_call":wall_ns as f64/(rounds*count) as f64,"mean_worker_ns_per_batch":sum_wall as f64/(rounds*count) as f64,"thread_cpu_ns_per_call":sum_cpu as f64/(rounds*count) as f64,"timed_allocations":allocations,"internally_verified":true,"output_sha256":first.digest,"capacity_bytes_per_worker":first.capacity_bytes,"fixture_reference_bytes_per_worker":first.reference_bytes,"usage":usage,"SDK_requests":0})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 6);
    let n = a[2].parse().unwrap();
    let mode = a[3].parse().unwrap();
    let rounds = a[5].parse().unwrap();
    assert!(mode <= 2);
    assert!([8192, 262144].contains(&rounds));
    assert!(
        (a[1] == "g1-owned" && n == 200)
            || (a[1] == "g1-borrowed-signed" && n == 352)
            || (a[1] == "g2-prefix" && n == 352)
    );
    let result = run(&a[1], n, mode, &a[4], rounds, 1024);
    println!("RESULT {result}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_workers_preserve_oracles_affinity_barriers_and_reuse() {
        pool();
        let mut digests = std::collections::BTreeMap::new();
        for (kind, n) in [
            ("g1-owned", 200),
            ("g1-borrowed-signed", 352),
            ("g2-prefix", 352),
        ] {
            for topology in ["solo", "smt", "separate", "physical16", "logical32"] {
                for mode in 0..3 {
                    let v = run(kind, n, mode, topology, 8, 8);
                    assert_eq!(v["worker_count"], cpus(topology).len());
                    assert_eq!(v["timed_allocations"], 0);
                    let h = v["output_sha256"].as_str().unwrap().to_string();
                    if let Some(old) = digests.insert(kind, h.clone()) {
                        assert_eq!(old, h);
                    }
                }
            }
        }
    }
}
