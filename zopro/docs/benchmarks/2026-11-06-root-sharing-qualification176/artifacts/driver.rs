use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Barrier,
    },
    time::Instant,
};
use zopro_bn254::Fq;
mod bulk;
mod fixture;
mod perf;
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
struct Counted;
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}
#[global_allocator]
static A: Counted = Counted;
fn affinity(cpu: usize) {
    unsafe {
        let mut mask: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut mask);
        libc::CPU_SET(cpu, &mut mask);
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&mask), &mask),
            0
        );
    }
}
fn individual(roots: &[[Fq; 32]], worker: usize, rounds: usize, sums: &mut [[u64; 4]; 32]) {
    for r in 0..rounds {
        let root = std::hint::black_box(roots[r % fixture::ROWS][worker]);
        let inv = root.inverse().unwrap();
        fixture::accumulate(&mut sums[worker], inv);
    }
}
fn shared<const N: usize>(roots: &[[Fq; 32]], rounds: usize, sums: &mut [[u64; 4]; 32]) {
    for r in 0..rounds {
        let inverse = bulk::bulk(std::hint::black_box(&roots[r % fixture::ROWS]), N).unwrap();
        for i in 0..N {
            fixture::accumulate(&mut sums[i], inverse[i]);
        }
    }
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(args.len(), 4);
    let workers = args[1].parse::<usize>().unwrap();
    let rounds = args[2].parse::<usize>().unwrap();
    let label = args[3].as_str();
    assert!([1, 16, 32].contains(&workers));
    assert!(rounds > 0);
    assert!(["control-a", "control-b", "shared"].contains(&label));
    let sharing = label == "shared";
    let coordinator = if workers == 1 { 0 } else { 2 };
    let roots = fixture::fixture();
    let fixture_hash = fixture::hash(&roots);
    let ready = Barrier::new(workers + 1);
    let start = Barrier::new(workers + 1);
    let finish = Barrier::new(workers + 1);
    let (rows, elapsed, allocations, usage) = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for worker in 0..workers {
            let roots = &roots;
            let ready = &ready;
            let start = &start;
            let finish = &finish;
            handles.push(
                std::thread::Builder::new()
                    .stack_size(512 * 1024)
                    .spawn_scoped(scope, move || {
                        let cpu = if workers == 1 { 2 } else { worker };
                        affinity(cpu);
                        let tid = unsafe { libc::syscall(libc::SYS_gettid) };
                        let active = !sharing || worker == coordinator;
                        let mut sums = [[0u64; 4]; 32];
                        for _ in 0..3 {
                            let mut dummy = [[0u64; 4]; 32];
                            if active {
                                if !sharing || workers == 1 {
                                    individual(roots, worker, fixture::ROWS, &mut dummy);
                                } else if workers == 16 {
                                    shared::<16>(roots, fixture::ROWS, &mut dummy);
                                } else {
                                    shared::<32>(roots, fixture::ROWS, &mut dummy);
                                }
                            }
                            std::hint::black_box(dummy);
                        }
                        ready.wait();
                        start.wait();
                        if active {
                            if !sharing || workers == 1 {
                                individual(roots, worker, rounds, &mut sums);
                            } else if workers == 16 {
                                shared::<16>(roots, rounds, &mut sums);
                            } else {
                                shared::<32>(roots, rounds, &mut sums);
                            }
                        }
                        finish.wait();
                        assert_eq!(unsafe { libc::sched_getcpu() }, cpu as i32);
                        (worker, cpu, tid, active, sums)
                    })
                    .unwrap(),
            );
        }
        ready.wait();
        let mut control = perf::Control::from_env().unwrap();
        if let Some(c) = control.as_mut() {
            c.enable().unwrap();
        }
        let before = perf::Usage::now().unwrap();
        let alloc0 = ALLOCS.load(Ordering::Relaxed);
        let time = Instant::now();
        start.wait();
        finish.wait();
        let elapsed = time.elapsed().as_nanos() as u64;
        let allocations = ALLOCS.load(Ordering::Relaxed) - alloc0;
        let after = perf::Usage::now().unwrap();
        if let Some(c) = control.as_mut() {
            c.disable().unwrap();
        }
        let rows = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<_>>();
        (rows, elapsed, allocations, after.difference(&before))
    });
    assert_eq!(allocations, 0, "timed allocation");
    let mut sums = [[0u64; 4]; 32];
    let mut workers_json = vec![];
    for (worker, cpu, tid, active, result) in rows {
        workers_json.push(json!({"worker":worker,"cpu":cpu,"tid":tid,"active":active}));
        for i in 0..32 {
            for j in 0..4 {
                sums[i][j] = sums[i][j].wrapping_add(result[i][j]);
            }
        }
    }
    // Validation only. These inverses never feed the timed implementation.
    for i in 0..workers {
        let mut cycle = [0u64; 4];
        let mut prefix = [0u64; 4];
        for r in 0..fixture::ROWS {
            let inverse = roots[r][i].inverse().unwrap();
            fixture::accumulate(&mut cycle, inverse);
            if r < rounds % fixture::ROWS {
                fixture::accumulate(&mut prefix, inverse);
            }
        }
        let expected = std::array::from_fn::<_, 4, _>(|j| {
            cycle[j]
                .wrapping_mul((rounds / fixture::ROWS) as u64)
                .wrapping_add(prefix[j])
        });
        assert_eq!(sums[i], expected);
    }
    let mut h = Sha256::new();
    for sum in &sums[..workers] {
        for w in sum {
            h.update(w.to_le_bytes());
        }
    }
    println!(
        "ROOT_WAVE {}",
        json!({"label":label,"mode":if sharing{"optimistic-shared"}else{"independent"},"workers":workers,"roots_per_wave":workers,"rounds":rounds,"elapsed_ns":elapsed,"ns_per_wave":elapsed as f64/rounds as f64,"timed_allocations":allocations,"output_sha256":format!("{:x}",h.finalize()),"checksum_kind":"four wrapping-u64 sums of canonical Montgomery outputs per root; fixed wrapper independently qualified","fixture_sha256":fixture_hash,"fixture_rows":fixture::ROWS,"fixture_bytes":fixture::ROWS*32*32,"pid":std::process::id(),"worker_rows":workers_json,"coordinator":coordinator,"active_workers":if sharing{1}else{workers},"parked_workers":if sharing{workers-1}else{0},"zero_request_publication_or_queue_cost":true,"worker_start_end_barriers_inside_clock":true,"priming_passes":3,"priming_rounds":fixture::ROWS,"scratch_stack_bytes_per_benchmark_thread":512*1024,"usage":usage,"proofs":0,"new_production_threads":0})
    );
}
