#![deny(unsafe_op_in_unsafe_fn)]
use ark_ff::{BigInt, Field, PrimeField};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::{
        atomic::{AtomicUsize, Ordering::Relaxed},
        Barrier,
    },
    time::Instant,
};
use zopro_bn254::{Fp, FpConfig, FqConfig, FrConfig};
#[path = "ordinary.rs"]
mod ordinary;
#[path = "paired.rs"]
mod paired;
#[path = "perf.rs"]
mod perf;
#[path = "portable.rs"]
mod portable;
#[path = "selected.rs"]
mod selected;
struct Counted;
static ALLOCS: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Relaxed);
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static A: Counted = Counted;
fn affinity(cpu: usize) {
    unsafe {
        let mut m: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut m);
        libc::CPU_SET(cpu, &mut m);
        assert_eq!(libc::sched_setaffinity(0, std::mem::size_of_val(&m), &m), 0);
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
fn cpus(t: &str) -> Vec<usize> {
    match t {
        "solo" => vec![2],
        "physical16" => (0..16).collect(),
        "logical32" => (0..32).collect(),
        _ => panic!("topology"),
    }
}
fn fixtures<C: FpConfig>() -> Vec<Fp<C>> {
    let mut seed = 0x7630_32a1_5120_2091u64;
    let mut v = Vec::new();
    while v.len() < 4096 {
        let mut bytes = [0; 64];
        for chunk in bytes.chunks_exact_mut(8) {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            chunk.copy_from_slice(&seed.to_le_bytes());
        }
        let x = Fp::<C>::from_random_bytes(&bytes);
        if !x.is_zero() {
            v.push(x);
        }
    }
    v
}
fn expected<C: FpConfig>(input: &[Fp<C>], field: &str) -> Vec<Fp<C>> {
    input
        .iter()
        .map(|x| {
            let repr = x.to_canonical();
            let words = if field == "fq" {
                ark_ref::Fq::from_bigint(BigInt(repr))
                    .unwrap()
                    .inverse()
                    .unwrap()
                    .into_bigint()
                    .0
            } else {
                ark_ref::Fr::from_bigint(BigInt(repr))
                    .unwrap()
                    .inverse()
                    .unwrap()
                    .into_bigint()
                    .0
            };
            let result = Fp::<C>::from_canonical(words).unwrap();
            assert_eq!(*x * result, Fp::ONE);
            result
        })
        .collect()
}
#[inline(never)]
fn calculate<C: FpConfig, const MODE: usize>(input: &[Fp<C>], out: &mut [Fp<C>], passes: usize) {
    for _ in 0..passes {
        for (x, y) in black_box(input).iter().zip(black_box(&mut *out)) {
            *y = match MODE {
                0 => black_box(x).inverse().unwrap(),
                1 => ordinary::inverse(black_box(x)).unwrap(),
                2 => selected::inverse(black_box(x)).unwrap(),
                _ => unreachable!(),
            };
        }
        black_box(&mut *out);
    }
}
fn calculate_mode<C: FpConfig>(input: &[Fp<C>], out: &mut [Fp<C>], passes: usize, mode: usize) {
    match mode {
        0 => calculate::<C, 0>(input, out, passes),
        1 => calculate::<C, 1>(input, out, passes),
        2 => calculate::<C, 2>(input, out, passes),
        _ => panic!("mode"),
    }
}
fn digest<C: FpConfig>(v: &[Fp<C>]) -> String {
    let mut h = Sha256::new();
    for x in v {
        for w in x.0 {
            h.update(w.to_le_bytes());
        }
    }
    format!("{:x}", h.finalize())
}
fn run<C: FpConfig>(field: &str, mode: usize, topology: &str, passes: usize) -> Value {
    let input = fixtures::<C>();
    let want = expected(&input, field);
    let input_sha = digest(&input);
    let expected_sha = digest(&want);
    let cpus = cpus(topology);
    let count = cpus.len();
    let ready = Barrier::new(count + 1);
    let start = Barrier::new(count + 1);
    let done = Barrier::new(count + 1);
    let release = Barrier::new(count + 1);
    let mut control = perf::Control::from_env().unwrap();
    let (wall, allocations, usage, workers) = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(count);
        for &cpu in &cpus {
            let (input, want, ready, start, done, release) =
                (&input, &want, &ready, &start, &done, &release);
            handles.push(scope.spawn(move||{affinity(cpu);let mut out=vec![Fp::<C>::ZERO;input.len()];for _ in 0..3{calculate_mode(input,&mut out,1,mode);assert_eq!(out,*want);out.fill(Fp::ZERO);}let tid=unsafe{libc::syscall(libc::SYS_gettid)};ready.wait();start.wait();let c0=cpu_now();let clock=Instant::now();calculate_mode(input,&mut out,passes,mode);let elapsed=clock.elapsed().as_nanos()as u64;let cpu_time=cpu_now()-c0;assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);done.wait();release.wait();assert_eq!(out,*want);for(x,y)in input.iter().zip(&out){assert_eq!(*x**y,Fp::ONE);}json!({"cpu":cpu,"tid":tid,"elapsed_ns":elapsed,"thread_cpu_ns":cpu_time,"ns_per_inverse":elapsed as f64/(passes*input.len())as f64,"output_sha256":digest(&out)})}));
        }
        ready.wait();
        if let Some(c) = control.as_mut() {
            c.enable().unwrap();
        }
        let u0 = perf::Usage::now().unwrap();
        let before = ALLOCS.load(Relaxed);
        let clock = Instant::now();
        start.wait();
        done.wait();
        let wall = clock.elapsed().as_nanos() as u64;
        let allocations = ALLOCS.load(Relaxed) - before;
        let u1 = perf::Usage::now().unwrap();
        if let Some(c) = control.as_mut() {
            c.disable().unwrap();
        }
        release.wait();
        let workers: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        (wall, allocations, u1.difference(&u0), workers)
    });
    assert_eq!(allocations, 0);
    for w in &workers {
        assert_eq!(w["output_sha256"], expected_sha);
        assert!(w["elapsed_ns"].as_u64().unwrap() <= wall);
    }
    let inversions = passes * input.len();
    let total_cpu: u64 = workers
        .iter()
        .map(|w| w["thread_cpu_ns"].as_u64().unwrap())
        .sum();
    json!({"field":field,"mode":mode,"topology":topology,"passes":passes,"input_count":input.len(),"prime_passes":3,"worker_count":count,"cpus":cpus,"workers":workers,"inversions_per_worker":inversions,"total_inversions":inversions*count,"wall_ns":wall,"wall_ns_per_inverse_wave":wall as f64/inversions as f64,"throughput_ns_per_inverse":wall as f64/(inversions*count)as f64,"thread_cpu_ns_per_inverse":total_cpu as f64/(inversions*count)as f64,"timed_allocations":allocations,"input_sha256":input_sha,"output_sha256":expected_sha,"Arkworks_verified":true,"product_identity_verified":true,"usage":usage,"SDK_requests":0})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    assert!(std::is_x86_feature_detected!("avx512f") && std::is_x86_feature_detected!("avx2"));
    let mode = a[2].parse().unwrap();
    let passes = a[4].parse().unwrap();
    assert!(mode <= 2);
    assert!([16, 128].contains(&passes));
    let v = match a[1].as_str() {
        "fq" => run::<FqConfig>(&a[1], mode, &a[3], passes),
        "fr" => run::<FrConfig>(&a[1], mode, &a[3], passes),
        _ => panic!("field"),
    };
    println!("RESULT {v}");
}
