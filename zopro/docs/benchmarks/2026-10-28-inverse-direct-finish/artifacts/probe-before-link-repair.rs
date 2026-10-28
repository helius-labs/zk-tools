use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    fs,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
use zopro_bn254::{ifma8::Ifma8, Fp, FpConfig, FqConfig, FrConfig};
#[path = "perf.rs"]
mod perf;
struct Counted;
static ALLOCS: AtomicU64 = AtomicU64::new(0);
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc_zeroed(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static A: Counted = Counted;
fn calculate<C: FpConfig>(input: &[Fp<C>], out: &mut [Fp<C>], batch: bool, rounds: usize) {
    for _ in 0..rounds {
        if batch {
            for (a, b) in input.chunks_exact(8).zip(out.chunks_exact_mut(8)) {
                let lanes = core::array::from_fn(|j| a[j]);
                b.copy_from_slice(&Ifma8::new(&lanes).inverse_nonzero().get());
            }
        } else {
            for (a, b) in input.iter().zip(out.iter_mut()) {
                *b = a.inverse().unwrap();
            }
        }
        std::hint::black_box(&mut *out);
    }
}
fn run<C: FpConfig>(fixture: &Value, field: &str, kind: &str, rounds: usize) {
    let batch = kind == "batch8";
    assert!(batch || kind == "scalar");
    let input: Vec<_> = fixture[field]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            Fp::<C>::from_canonical(serde_json::from_value(v["input"].clone()).unwrap()).unwrap()
        })
        .collect();
    assert_eq!(input.len(), 4096);
    let expected: Vec<_> = fixture[field]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            Fp::<C>::from_canonical(serde_json::from_value(v["inverse"].clone()).unwrap()).unwrap()
        })
        .collect();
    let mut out = vec![Fp::ZERO; input.len()];
    let mut control = perf::Control::from_env().unwrap();
    for _ in 0..3 {
        calculate(&input, &mut out, batch, rounds);
    }
    assert_eq!(out, expected);
    if let Some(c) = control.as_mut() {
        c.enable().unwrap();
    }
    let usage = perf::Usage::now().unwrap();
    let before = ALLOCS.load(Ordering::Relaxed);
    let started = Instant::now();
    calculate(&input, &mut out, batch, rounds);
    let elapsed = started.elapsed().as_nanos();
    let allocs = ALLOCS.load(Ordering::Relaxed) - before;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = control.as_mut() {
        c.disable().unwrap();
    }
    assert_eq!(allocs, 0);
    assert_eq!(out, expected);
    let mut h = Sha256::new();
    for x in out {
        for w in x.to_canonical() {
            h.update(w.to_le_bytes());
        }
    }
    println!(
        "RESULT {}",
        json!({"field":field,"kind":kind,"rounds":rounds,"inputs":input.len(),"elements":rounds*input.len(),"elapsed_ns":elapsed,"mean_ns_per_element":elapsed as f64/(rounds*input.len())as f64,"timed_allocations":allocs,"verified_against_Arkworks":true,"output_sha256":format!("{:x}",h.finalize()),"usage":after.difference(&usage)})
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    let fixture: Value = serde_json::from_slice(&fs::read(&a[1]).unwrap()).unwrap();
    let rounds = a[4].parse().unwrap();
    match a[2].as_str() {
        "fq" => run::<FqConfig>(&fixture, &a[2], &a[3], rounds),
        "fr" => run::<FrConfig>(&fixture, &a[2], &a[3], rounds),
        _ => panic!("field"),
    }
}
