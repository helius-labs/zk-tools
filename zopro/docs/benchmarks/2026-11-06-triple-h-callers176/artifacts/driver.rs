mod perf_support;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Arc, Barrier,
    },
    time::Instant,
};
use zopro_core::ReductionCaller;
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
fn case(name: &str) -> (Vec<usize>, bool) {
    match name {
        "tile257" => (vec![257], true),
        "tile384" => (vec![384], true),
        "tile511" => (vec![511], true),
        "tile2048" => (vec![2048], true),
        "group35973" => (vec![35973], false),
        "groups31x256" => (vec![31; 256], false),
        "tails" => (vec![0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26, 47, 48, 49], false),
        _ => panic!("case"),
    }
}
fn digest(f: &ReductionCaller) -> String {
    let mut h = Sha256::new();
    for p in f.coordinates() {
        for c in p {
            for w in c {
                h.update(w.to_le_bytes());
            }
        }
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    let (counts, fresh) = case(&a[1]);
    let selected = match a[2].as_str() {
        "binary" => false,
        "ternary" => true,
        _ => panic!("route"),
    };
    let workers: usize = a[3].parse().unwrap();
    assert!([1, 16, 32].contains(&workers));
    let rounds: usize = a[4].parse().unwrap();
    assert!(rounds > 0);
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    let ready = Arc::new(Barrier::new(workers + 1));
    let start = Arc::new(Barrier::new(workers + 1));
    let finish = Arc::new(Barrier::new(workers + 1));
    let release = Arc::new(Barrier::new(workers + 1));
    let mut jobs = Vec::with_capacity(workers);
    for id in 0..workers {
        let counts = counts.clone();
        let ready = ready.clone();
        let start = start.clone();
        let finish = finish.clone();
        let release = release.clone();
        jobs.push(std::thread::Builder::new().name(format!("reduction-{id}")).stack_size(512*1024).spawn(move||{
            let cpu=if workers==1{2}else{id};
            // SAFETY: initialized CPU set; current thread and valid set size.
            unsafe{let mut set:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut set);libc::CPU_SET(cpu,&mut set);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&set),&set),0);}
            let tid=unsafe{libc::syscall(libc::SYS_gettid)}as u64;
            assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);
            let mut f=ReductionCaller::new(&counts);
            for _ in 0..3{f.run(selected,fresh,8);f.check();}
            let before=digest(&f);let capacity_before=f.scratch_bytes();
            ready.wait();start.wait();black_box(&mut f).run(selected,fresh,rounds);finish.wait();release.wait();
            f.check();let hash=digest(&f);assert_eq!(hash,before);let capacity=f.scratch_bytes();assert_eq!(capacity,capacity_before);
            assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);
            json!({"id":id,"tid":tid,"cpu":cpu,"output_sha256":hash,"scratch_capacity_bytes":capacity})
        }).unwrap());
    }
    let mut control = perf_support::Control::from_env().unwrap();
    ready.wait();
    if let Some(c) = &mut control {
        c.enable().unwrap();
    }
    let usage_before = perf_support::Usage::now().unwrap();
    let allocations_before = ALLOCS.load(Relaxed);
    let begin = Instant::now();
    start.wait();
    finish.wait();
    let elapsed = begin.elapsed();
    let allocations = ALLOCS.load(Relaxed) - allocations_before;
    let usage = perf_support::Usage::now()
        .unwrap()
        .difference(&usage_before);
    if let Some(c) = &mut control {
        c.disable().unwrap();
    }
    release.wait();
    let rows: Vec<_> = jobs.into_iter().map(|j| j.join().unwrap()).collect();
    assert!(rows
        .windows(2)
        .all(|w| w[0]["output_sha256"] == w[1]["output_sha256"]));
    if !fresh {
        assert_eq!(allocations, 0);
    }
    println!(
        "RESULT {}",
        json!({"case":a[1],"variant":a[2],"workers":workers,"rounds_per_worker":rounds,"priming_rounds_per_worker":24,"fresh_buffers":fresh,"counts":counts,"elapsed_ns":elapsed.as_nanos(),"ns_per_wave":elapsed.as_nanos()as f64/rounds as f64,"timed_allocations":allocations,"allocation_calls_per_worker_round":allocations as f64/(rounds*workers)as f64,"process_usage":usage,"output_sha256":rows[0]["output_sha256"],"worker_rows":rows,"complete_projective_oracle":true,"SDK_requests":0,"proofs":0})
    );
}
