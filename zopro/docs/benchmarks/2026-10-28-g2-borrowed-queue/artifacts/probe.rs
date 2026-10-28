use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    time::Instant,
};
use zopro_bn254::{Field, Fr, G2Affine, G2Projective};
#[path = "perf.rs"]
mod perf;
struct Alloc;
#[cfg(zopro_queue_heap_probe)]
mod heap {
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    pub static LIVE: AtomicUsize = AtomicUsize::new(0);
    pub static PEAK: AtomicUsize = AtomicUsize::new(0);
    pub static CALLS: AtomicUsize = AtomicUsize::new(0);
    pub static BYTES: AtomicUsize = AtomicUsize::new(0);
    pub fn add(n: usize) {
        CALLS.fetch_add(1, Relaxed);
        BYTES.fetch_add(n, Relaxed);
        let v = LIVE.fetch_add(n, Relaxed) + n;
        PEAK.fetch_max(v, Relaxed);
    }
    pub fn remove(n: usize) {
        LIVE.fetch_sub(n, Relaxed);
    }
    pub fn reset() -> usize {
        let v = LIVE.load(Relaxed);
        PEAK.store(v, Relaxed);
        CALLS.store(0, Relaxed);
        BYTES.store(0, Relaxed);
        v
    }
    pub fn report(before: usize) -> serde_json::Value {
        let after = LIVE.load(Relaxed);
        let peak = PEAK.load(Relaxed);
        let calls = CALLS.load(Relaxed);
        let bytes = BYTES.load(Relaxed);
        serde_json::json!({"live_before":before,"live_after":after,"peak_live":peak,"peak_extra":peak.saturating_sub(before),"allocation_requests":calls,"requested_bytes":bytes})
    }
}
unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { zopro_core::alloc::Allocator.alloc(l) };
        #[cfg(zopro_queue_heap_probe)]
        if !p.is_null() {
            heap::add(l.size())
        }
        p
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        let p = unsafe { zopro_core::alloc::Allocator.alloc_zeroed(l) };
        #[cfg(zopro_queue_heap_probe)]
        if !p.is_null() {
            heap::add(l.size())
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        #[cfg(zopro_queue_heap_probe)]
        heap::remove(l.size());
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        let q = unsafe { zopro_core::alloc::Allocator.realloc(p, l, n) };
        #[cfg(zopro_queue_heap_probe)]
        if !q.is_null() {
            heap::remove(l.size());
            heap::add(n)
        }
        q
    }
}
#[global_allocator]
static ALLOC: Alloc = Alloc;
fn cpus(t: &str) -> Vec<usize> {
    match t {
        "solo" => vec![2],
        "physical16" => (0..16).collect(),
        "logical32" => (0..32).collect(),
        "test2" => vec![0, 1],
        _ => panic!("topology"),
    }
}
fn affinity(cpus: &[usize]) {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        for c in cpus {
            libc::CPU_SET(*c, &mut set);
        }
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set),
            0
        );
    }
}
fn actual_affinity() -> Vec<usize> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set),
            0
        );
        (0..libc::CPU_SETSIZE as usize)
            .filter(|c| libc::CPU_ISSET(*c, &set))
            .collect()
    }
}
fn pool(cpus: &[usize]) -> rayon::ThreadPool {
    let mapping = cpus.to_vec();
    rayon::ThreadPoolBuilder::new()
        .num_threads(cpus.len())
        .start_handler(move |i| affinity(&[mapping[i]]))
        .build()
        .unwrap()
}
fn workers(p: &rayon::ThreadPool) -> Value {
    let values = p.broadcast(|ctx| {
        let a = actual_affinity();
        assert_eq!(a.len(), 1);
        json!({"index":ctx.index(),"cpu":a[0],"tid":unsafe{libc::syscall(libc::SYS_gettid)}})
    });
    json!(values)
}
fn random(seed: &mut u64) -> Fr {
    let mut b = [0; 32];
    for c in b.chunks_mut(8) {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        c.copy_from_slice(&seed.to_le_bytes());
    }
    Fr::from_le_bytes_mod_order(&b)
}
struct Fixture {
    bases: Vec<G2Affine>,
    original: Vec<[u64; 4]>,
    words: Vec<[u64; 4]>,
    expected: G2Affine,
}
impl Fixture {
    fn new(n: usize, seed: u64) -> Self {
        let g = G2Projective::generator();
        let offset = 1000003 + seed * 65537;
        let mut point = g * Fr::from(offset);
        let mut points = Vec::with_capacity(n);
        let mut expected = Fr::ZERO;
        let mut original = Vec::with_capacity(n);
        let mut random_seed = 0x951c742bda638f07 ^ seed;
        for i in 0..n {
            points.push(point);
            point += g;
            let s = random(&mut random_seed);
            expected += s * Fr::from(offset + i as u64);
            original.push(s.to_canonical());
        }
        Self {
            bases: G2Projective::batch_to_affine(&points),
            words: original.clone(),
            original,
            expected: (g * expected).to_affine(),
        }
    }
    fn one(&mut self, p: &rayon::ThreadPool) -> G2Projective {
        self.words.copy_from_slice(&self.original);
        let b = black_box(&self.bases);
        p.install(|| zopro_core::queue_probe_g2(b, black_box(&mut self.words)))
    }
    fn check(&self, result: G2Projective) {
        assert_eq!(result.to_affine(), self.expected);
    }
}
fn run(n: usize, mode: usize, topology: &str, rounds: usize, seed: u64) -> Value {
    let cpus = cpus(topology);
    affinity(&cpus);
    let p = pool(&cpus);
    let threads = workers(&p);
    let mut f = Fixture::new(n, seed);
    for _ in 0..3 {
        let r = f.one(&p);
        f.check(r);
    }
    let mut results = Vec::with_capacity(rounds);
    let mut control = perf::Control::from_env().unwrap();
    #[cfg(zopro_queue_heap_probe)]
    let live_before = heap::reset();
    if let Some(c) = control.as_mut() {
        c.enable().unwrap();
    }
    let before = perf::Usage::now().unwrap();
    let start = Instant::now();
    for _ in 0..rounds {
        results.push(f.one(&p));
    }
    let wall = start.elapsed().as_nanos() as u64;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = control.as_mut() {
        c.disable().unwrap();
    }
    #[cfg(zopro_queue_heap_probe)]
    let memory = heap::report(live_before);
    #[cfg(not(zopro_queue_heap_probe))]
    let memory = Value::Null;
    for r in results {
        f.check(r);
    }
    assert_eq!(workers(&p), threads);
    assert_eq!(actual_affinity(), cpus);
    let usage = after.difference(&before);
    let cpu_ms = usage["user_cpu_ms"].as_f64().unwrap() + usage["system_cpu_ms"].as_f64().unwrap();
    let digest = format!(
        "{:x}",
        Sha256::digest(format!("{:?}", f.expected).as_bytes())
    );
    json!({"kind":"g2-msm","size":n,"mode":mode,"topology":topology,"worker_count":cpus.len(),"cpus":cpus,"workers":threads,"rounds":rounds,"prime_calls":3,"input_seed":seed,"total_MSM_calls":rounds,"wall_ns":wall,"ns_per_MSM":wall as f64/rounds as f64,"CPU_ms_per_MSM":cpu_ms/rounds as f64,"usage":usage,"output_sha256":digest,"internally_verified":true,"includes_scalar_reset":true,"heap_instrumented":cfg!(zopro_queue_heap_probe),"heap":memory,"SDK_requests":0,"proof_exports":0})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 6);
    assert_eq!(a[1], "g2-msm");
    let n = a[2].parse().unwrap();
    assert!([4096, 16384, 30089].contains(&n));
    let mode = a[3].parse().unwrap();
    assert!(mode < 3);
    let rounds = a[5].parse().unwrap();
    assert!([1, 7, 256].contains(&rounds));
    let seed = std::env::var("ZOPRO_MSM_SEED").unwrap().parse().unwrap();
    println!("RESULT {}", run(n, mode, &a[4], rounds, seed));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_MSM_oracles_reset_and_affinity() {
        for t in ["solo", "test2"] {
            for n in [7, 1024] {
                let v = run(n, 0, t, 2, 23);
                assert_eq!(v["internally_verified"], true);
                assert_eq!(v["worker_count"], cpus(t).len());
            }
        }
    }
    #[cfg(zopro_queue_heap_probe)]
    #[test]
    fn heap_accounting_balances_requested_buffer() {
        use std::sync::atomic::Ordering::Relaxed;
        let before = heap::LIVE.load(Relaxed);
        let v = std::hint::black_box(Vec::<u8>::with_capacity(4096));
        assert_eq!(heap::LIVE.load(Relaxed), before + 4096);
        drop(v);
        assert_eq!(heap::LIVE.load(Relaxed), before);
    }
}
