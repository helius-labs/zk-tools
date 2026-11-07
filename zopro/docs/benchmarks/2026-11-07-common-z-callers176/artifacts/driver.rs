mod perf_support;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout},
    sync::{Arc, Barrier},
    cell::Cell,
    time::Instant,
};
use zopro_core::{TreeCaller, WeightedCaller};
thread_local! { static ALLOC_COUNT: Cell<usize> = const { Cell::new(0) }; }
fn record_allocation(){ let _=ALLOC_COUNT.try_with(|n|n.set(n.get().wrapping_add(1))); }
fn thread_allocations()->usize{ALLOC_COUNT.with(Cell::get)}
struct Counted;
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        record_allocation();
        unsafe { zopro_core::alloc::Allocator.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        record_allocation();
        unsafe { zopro_core::alloc::Allocator.alloc_zeroed(l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        record_allocation();
        unsafe { zopro_core::alloc::Allocator.realloc(p, l, n) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
}
#[global_allocator]
static A: Counted = Counted;
fn case(name: &str) -> (Vec<usize>, bool, bool) {
    match name {
        "small16" => (vec![16],false,false),
        "groups3x4" => (vec![3;4],false,false),
        "small-mixed" => (vec![3,4,0,1,2],false,false),
        "tile257" => (vec![257], true, false),
        "tile384" => (vec![384], true, false),
        "tile511" => (vec![511], true, false),
        "tile2048" => (vec![2048], true, false),
        "group35973" => (vec![35973], false, false),
        "groups31x256" => (vec![31; 256], false, false),
        "tails" => (
            vec![0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26, 47, 48, 49],
            false,
            false,
        ),
        "weighted128" => (vec![128], true, true),
        "weighted256" => (vec![256], true, true),
        _ => panic!("case"),
    }
}
enum Fixture {
    Tree(TreeCaller, bool),
    Weighted(WeightedCaller),
}
impl Fixture {
    fn new(counts: &[usize], fresh: bool, weighted: bool) -> Self {
        if weighted {
            Self::Weighted(WeightedCaller::new(counts[0]))
        } else {
            Self::Tree(TreeCaller::new(counts), fresh)
        }
    }
    fn run(&mut self, n: usize) {
        match self {
            Self::Tree(f, fresh) => f.run(*fresh, n),
            Self::Weighted(f) => f.run(n),
        }
    }
    fn check(&self) {
        match self {
            Self::Tree(f, _) => f.check(),
            Self::Weighted(f) => f.check(),
        }
    }
    fn coordinates(&self) -> Vec<[[u64; 4]; 2]> {
        match self {
            Self::Tree(f, _) => f.coordinates(),
            Self::Weighted(f) => vec![f.coordinates()],
        }
    }
    fn capacity(&self) -> Option<[usize; 5]> {
        match self {
            Self::Tree(f, _) => Some(f.scratch_bytes()),
            Self::Weighted(_) => None,
        }
    }
    fn forecast(&self) -> Option<usize> {
        match self {
            Self::Weighted(f) => f.source_forecast_bytes(),
            _ => None,
        }
    }
    fn digest(&self) -> String {
        let mut h = Sha256::new();
        for p in self.coordinates() {
            for c in p {
                for w in c {
                    h.update(w.to_le_bytes());
                }
            }
        }
        format!("{:x}", h.finalize())
    }
}
fn run_case(name: &str, workers: usize, rounds: usize, arm: &str, measure: bool) -> Value {
    let (counts, fresh, weighted) = case(name);
    assert!(rounds > 0);
    let pool = Arc::new(
        rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 * 1024)
            .build()
            .unwrap(),
    );
    let ready = Arc::new(Barrier::new(workers + 1));
    let start = Arc::new(Barrier::new(workers + 1));
    let finish = Arc::new(Barrier::new(workers + 1));
    let release = Arc::new(Barrier::new(workers + 1));
    let coordinator = {
        let pool = pool.clone();
        let ready = ready.clone();
        let start = start.clone();
        let finish = finish.clone();
        let release = release.clone();
        let counts = counts.clone();
        std::thread::Builder::new().name("broadcast-coordinator".into()).stack_size(512*1024).spawn(move||pool.broadcast(|context|{
            let id=context.index();assert_eq!(rayon::current_num_threads(),workers);assert_eq!(rayon::current_thread_index(),Some(id));let cpu=if workers==1{2}else{id};
            // SAFETY: current worker receives one valid initialized CPU set.
            unsafe{let mut set:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut set);libc::CPU_SET(cpu,&mut set);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&set),&set),0);}
            let tid=unsafe{libc::syscall(libc::SYS_gettid)}as u64;let mut fixture=Fixture::new(&counts,fresh,weighted);
            for _ in 0..3{fixture.run(8);fixture.check();}let expected=fixture.digest();let before=fixture.capacity();let forecast=fixture.forecast();
            assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);ready.wait();let before_alloc=thread_allocations();start.wait();fixture.run(rounds);let worker_allocations=thread_allocations()-before_alloc;finish.wait();release.wait();if !fresh{assert_eq!(worker_allocations,0);}
            fixture.check();let hash=fixture.digest();assert_eq!(hash,expected);let capacity=fixture.capacity();assert_eq!(capacity,before);assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);assert_eq!(rayon::current_num_threads(),workers);
            json!({"id":id,"tid":tid,"cpu":cpu,"pool_workers":workers,"output_sha256":hash,"scratch_capacity_bytes":capacity,"source_forecast_bytes":forecast,"timed_worker_allocation_calls":worker_allocations})
        })).unwrap()
    };
    let mut control = if measure {
        perf_support::Control::from_env().unwrap()
    } else {
        None
    };
    ready.wait();
    if let Some(c) = &mut control {
        c.enable().unwrap();
    }
    let usage_before = if measure {
        Some(perf_support::Usage::now().unwrap())
    } else {
        None
    };
    let begin = if measure { Some(Instant::now()) } else { None };
    start.wait();
    finish.wait();
    let elapsed = begin.map(|t| t.elapsed());
    let usage = usage_before.map(|before| perf_support::Usage::now().unwrap().difference(&before));
    if let Some(c) = &mut control {
        c.disable().unwrap();
    }
    release.wait();
    let rows = coordinator.join().unwrap();
    let allocations:usize=rows.iter().map(|r|r["timed_worker_allocation_calls"].as_u64().unwrap()as usize).sum();
    assert_eq!(rows.len(), workers);
    assert!(rows
        .windows(2)
        .all(|w| w[0]["output_sha256"] == w[1]["output_sha256"]));
    if !fresh {
        assert_eq!(allocations, 0);
    }
    json!({"pid":std::process::id(),"case":name,"variant":arm,"workers":workers,"rounds_per_worker":rounds,"priming_rounds_per_worker":24,"fresh_buffers":fresh,"weighted":weighted,"counts":counts,"elapsed_ns":elapsed.map(|t|t.as_nanos()),"ns_per_wave":elapsed.map(|t|t.as_nanos()as f64/rounds as f64),"timed_allocations":if measure{Some(allocations)}else{None},"allocation_calls_per_worker_round":if measure{Some(allocations as f64/(rounds*workers)as f64)}else{None},"process_usage":usage,"worker_rows":rows,"output_sha256":rows[0]["output_sha256"],"complete_projective_oracle":true,"benchmark_Rayon_broadcast":true,"allocation_count_scope":"sum of worker TLS deltas, excludes main/coordinator","SDK_requests":0,"proofs":0})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    let workers: usize = a[2].parse().unwrap();
    assert!([1, 16, 32].contains(&workers));
    let rounds = a[3].parse().unwrap();
    assert!(["frozen", "fresh", "OFF", "ON"].contains(&a[4].as_str()));
    println!("RESULT {}", run_case(&a[1], workers, rounds, &a[4], true));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn broadcast_harness_is_inside_requested_pool_and_checks_after_release() {
        for workers in [1, 4, 16, 32] {
            for case in ["tile384", "tails", "weighted128", "weighted256"] {
                let v = run_case(case, workers, 3, "ON", false);
                assert!(v["elapsed_ns"].is_null());
                assert!(v["ns_per_wave"].is_null());
                assert!(v["process_usage"].is_null());
                assert_eq!(v["workers"], workers);
                assert_eq!(v["benchmark_Rayon_broadcast"], true);
                for row in v["worker_rows"].as_array().unwrap() {
                    assert_eq!(row["pool_workers"], workers);
                    let a=row["timed_worker_allocation_calls"].as_u64().unwrap();
                    if case=="tails"{assert_eq!(a,0);}else{assert!(a>0);}
                    if case.starts_with("weighted") {
                        assert!(row["scratch_capacity_bytes"].is_null());
                        assert!(row["source_forecast_bytes"].as_u64().unwrap() > 0);
                    } else {
                        assert_eq!(row["scratch_capacity_bytes"].as_array().unwrap().len(), 5);
                    }
                }
            }
        }
        println!(
            "BROADCAST_HARNESS_PASS 16contexts/1,4,16,32workers/512KiB/zero timing observations"
        );
    }
}
