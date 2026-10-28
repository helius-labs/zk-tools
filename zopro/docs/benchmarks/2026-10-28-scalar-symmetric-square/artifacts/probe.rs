use ark_ff::{BigInt, Field};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    fs,
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
use zopro_bn254::{Fp, FpConfig, FqConfig, FrConfig};
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
#[inline(never)]
fn calculate<C: FpConfig>(
    initial: [Fp<C>; 4],
    constant: [Fp<C>; 4],
    kind: &str,
    rounds: usize,
) -> [Fp<C>; 4] {
    let mut x = black_box(initial);
    let c = black_box(constant);
    match kind {
        "dependent" => {
            for _ in 0..rounds {
                x[0] = (x[0] + c[0]).square();
                black_box(&mut x);
            }
        }
        "four" => {
            for _ in 0..rounds {
                for i in 0..4 {
                    x[i] = (x[i] + c[i]).square();
                }
                black_box(&mut x);
            }
        }
        "fifth" => {
            for _ in 0..rounds {
                for i in 0..4 {
                    let y = x[i] + c[i];
                    let z = y.square();
                    x[i] = z.square() * y;
                }
                black_box(&mut x);
            }
        }
        _ => panic!("unknown kind"),
    }
    black_box(x)
}
fn fixture(path: &str) {
    let mut v = json!({});
    macro_rules! field{($name:expr,$config:ty,$ark:ty)=>{{
        let mut seed=0x1b70_946a_35fc_ed83u64;
        let mut next=||{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed};
        let mut input=vec![];for _ in 0..8{let a=[next(),next(),next(),next()&((1<<61)-1)];assert!(Fp::<$config>::from_canonical(a).is_some());input.push(a);}
        let initial:[Fp<$config>;4]=core::array::from_fn(|i|Fp::from_canonical(input[i]).unwrap());let constant:[Fp<$config>;4]=core::array::from_fn(|i|Fp::from_canonical(input[4+i]).unwrap());
        let raw_initial=initial.map(|x|x.0);let raw_constant=constant.map(|x|x.0);let mut outputs=json!({});
        for rounds in[262144usize,4194304]{for kind in["dependent","four","fifth"]{let mut x=raw_initial.map(|a|<$ark>::new_unchecked(BigInt(a)));let c=raw_constant.map(|a|<$ark>::new_unchecked(BigInt(a)));let streams=if kind=="dependent"{1}else{4};for _ in 0..rounds{for i in 0..streams{let y=x[i]+c[i];x[i]=if kind=="fifth"{y.square().square()*y}else{y.square()};}}outputs[format!("{kind}-{rounds}")]=json!(x.map(|a|a.0.0));}}
        v[$name]=json!({"initial_montgomery":raw_initial,"constant_montgomery":raw_constant,"outputs":outputs});
    }}}
    field!("fq", FqConfig, ark_bn254::Fq);
    field!("fr", FrConfig, ark_bn254::Fr);
    fs::write(path, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
    println!("Prepared independent Arkworks recurrence fixture for both fields,3 contexts,2 round counts");
}
fn run<C: FpConfig>(fixture: &Value, field: &str, kind: &str, rounds: usize) {
    assert!([262144, 4194304].contains(&rounds));
    let data = &fixture[field];
    let initial: [[u64; 4]; 4] =
        serde_json::from_value(data["initial_montgomery"].clone()).unwrap();
    let constant: [[u64; 4]; 4] =
        serde_json::from_value(data["constant_montgomery"].clone()).unwrap();
    let expected: [[u64; 4]; 4] =
        serde_json::from_value(data["outputs"][format!("{kind}-{rounds}")].clone()).unwrap();
    let initial = initial.map(Fp::<C>::from_montgomery);
    let constant = constant.map(Fp::<C>::from_montgomery);
    let mut control = perf::Control::from_env().unwrap();
    for _ in 0..3 {
        assert_eq!(
            calculate(initial, constant, kind, rounds).map(|x| x.0),
            expected
        );
    }
    if let Some(c) = control.as_mut() {
        c.enable().unwrap();
    }
    let before = perf::Usage::now().unwrap();
    let allocs = ALLOCS.load(Ordering::Relaxed);
    let started = Instant::now();
    let out = calculate(initial, constant, kind, rounds);
    let elapsed = started.elapsed().as_nanos();
    let allocations = ALLOCS.load(Ordering::Relaxed) - allocs;
    let after = perf::Usage::now().unwrap();
    if let Some(c) = control.as_mut() {
        c.disable().unwrap();
    }
    assert_eq!(allocations, 0);
    assert_eq!(out.map(|x| x.0), expected);
    let mut h = Sha256::new();
    for x in out {
        for w in x.0 {
            h.update(w.to_le_bytes());
        }
    }
    let updates = rounds * if kind == "dependent" { 1 } else { 4 };
    println!(
        "RESULT {}",
        json!({"field":field,"kind":kind,"rounds":rounds,"updates":updates,"elapsed_ns":elapsed,"mean_ns_per_update":elapsed as f64/updates as f64,"timed_allocations":allocations,"verified_against_Arkworks":true,"output_sha256":format!("{:x}",h.finalize()),"usage":after.difference(&before)})
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "fixture" => fixture(&a[2]),
        "run" => {
            let f: Value = serde_json::from_slice(&fs::read(&a[2]).unwrap()).unwrap();
            let rounds = a[5].parse().unwrap();
            match a[3].as_str() {
                "fq" => run::<FqConfig>(&f, &a[3], &a[4], rounds),
                "fr" => run::<FrConfig>(&f, &a[3], &a[4], rounds),
                _ => panic!("field"),
            }
        }
        _ => panic!("mode"),
    }
}
