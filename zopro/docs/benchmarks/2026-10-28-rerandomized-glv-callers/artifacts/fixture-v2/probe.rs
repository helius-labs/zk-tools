use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
use zopro_bn254::{
    Affine, CurveConfig, Fq, Fr, G1Affine, G1Config, G1Projective, G2Affine, G2Config,
    G2Projective, Projective,
};
mod candidate;
mod original;
mod perf;
use original::FixedBase;
struct Counter;
static ALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(l.size() as u64, Ordering::Relaxed);
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(n as u64, Ordering::Relaxed);
        unsafe { System.realloc(p, l, n) }
    }
}
#[global_allocator]
static ALLOCATOR: Counter = Counter;
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn random_scalar(seed: &mut u64) -> Fr {
    loop {
        let w = [
            next(seed),
            next(seed),
            next(seed),
            next(seed) & 0x3fff_ffff_ffff_ffff,
        ];
        if let Some(s) = Fr::from_canonical(w) {
            return s;
        }
    }
}
fn boundary_scalars() -> Vec<Fr> {
    let mut v = vec![
        Fr::ZERO,
        Fr::ONE,
        -Fr::ONE,
        Fr::from(15u64),
        Fr::from(16u64),
        Fr::from(17u64),
    ];
    for k in [1, 63, 64, 125, 126, 127, 128, 252, 253] {
        let s = Fr::from(2u64).pow(&[k]);
        v.extend([s - Fr::ONE, s, s + Fr::ONE]);
    }
    v
}
fn aq(x: ark_bn254::Fq) -> Fq {
    let b = x.into_bigint().to_bytes_be();
    let b: [u8; 32] = b.try_into().unwrap();
    Fq::from_be_bytes(&b).unwrap()
}
fn oracle_g1(s: Fr) -> G1Projective {
    let s = ark_bn254::Fr::from_be_bytes_mod_order(&s.to_be_bytes());
    let p = (ark_bn254::G1Affine::generator() * s).into_affine();
    if p.infinity {
        G1Projective::IDENTITY
    } else {
        G1Affine::new(aq(p.x), aq(p.y)).to_projective()
    }
}
fn oracle_g2(s: Fr) -> G2Projective {
    let s = ark_bn254::Fr::from_be_bytes_mod_order(&s.to_be_bytes());
    let p = (ark_bn254::G2Affine::generator() * s).into_affine();
    if p.infinity {
        G2Projective::IDENTITY
    } else {
        G2Affine::new(
            zopro_bn254::Fq2::new(aq(p.x.c0), aq(p.x.c1)),
            zopro_bn254::Fq2::new(aq(p.y.c0), aq(p.y.c1)),
        )
        .to_projective()
    }
}
fn pin(cpus: &[usize]) {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        for &c in cpus {
            libc::CPU_SET(c, &mut set);
        }
        assert_eq!(
            libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set),
            0
        );
    }
}
fn affinity() -> Vec<usize> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        assert_eq!(
            libc::sched_getaffinity(0, std::mem::size_of_val(&set), &mut set),
            0
        );
        (0..libc::CPU_SETSIZE as usize)
            .filter(|&c| libc::CPU_ISSET(c, &set))
            .collect()
    }
}
#[derive(Clone, Copy)]
struct Fixture {
    a: G1Projective,
    b: G2Projective,
    c: G1Projective,
    l: Fr,
    r: Fr,
    uv: Fr,
    want: (G1Projective, G1Projective, G2Projective),
}
type Output = (G1Projective, G1Projective, G2Projective);
fn fixtures() -> Vec<Fixture> {
    let mut seed = 0xa413_37f1_8b23_f621;
    (0..16)
        .map(|i| {
            let k = Fr::from((i * 17 + 5) as u64);
            let h = Fr::from((i * 19 + 7) as u64);
            let c = Fr::from((i + 13) as u64);
            let l = random_scalar(&mut seed);
            let r = random_scalar(&mut seed);
            let uv = l * r;
            Fixture {
                a: G1Projective::generator() * k,
                b: G2Projective::generator() * h,
                c: G1Projective::generator() * c,
                l,
                r,
                uv,
                want: (
                    oracle_g1(k * l),
                    oracle_g1(c + k * r),
                    oracle_g2(h * l + Fr::from(37u64) * uv),
                ),
            }
        })
        .collect()
}
fn pair(mode: usize, f: &Fixture) -> (G1Projective, G1Projective) {
    if mode == 0 {
        original::pair_g1(f.a, f.l, f.r)
    } else {
        candidate::pair_g1(f.a, f.l, f.r)
    }
}
fn joint(
    mode: usize,
    f: &Fixture,
    delta: G2Affine,
    prepared: &FixedBase<G2Config>,
) -> G2Projective {
    if mode == 0 {
        original::joint_g2(f.b, f.l, delta, f.uv, Some(prepared))
    } else {
        candidate::joint_g2(f.b, f.l, delta, f.uv, Some(prepared))
    }
}
fn call(
    case: &str,
    mode: usize,
    f: &Fixture,
    delta: G2Affine,
    prepared: &FixedBase<G2Config>,
) -> Output {
    match case {
        "g1" => {
            let (a, c) = pair(mode, f);
            (a, f.c + c, G2Projective::IDENTITY)
        }
        "g2" => (
            G1Projective::IDENTITY,
            G1Projective::IDENTITY,
            joint(mode, f, delta, prepared),
        ),
        "w16" | "w32" => {
            let ((a, c), b) = rayon::join(|| pair(mode, f), || joint(mode, f, delta, prepared));
            (a, f.c + c, b)
        }
        _ => panic!("case"),
    }
}
fn check(case: &str, got: Output, f: &Fixture) {
    match case {
        "g1" => assert_eq!((got.0, got.1), (f.want.0, f.want.1)),
        "g2" => assert_eq!(got.2, f.want.2),
        _ => assert_eq!(got, f.want),
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let case = &args[1];
    let mode: usize = args[2].parse().unwrap();
    assert!(mode <= 1);
    let rounds: usize = args[3].parse().unwrap();
    assert!(rounds > 0);
    let cpus: Vec<_> = match case.as_str() {
        "g1" | "g2" => vec![2],
        "w16" => (0..16).collect(),
        "w32" => (0..32).collect(),
        _ => panic!("case"),
    };
    pin(&cpus);
    let workers = cpus.len();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .start_handler({
            let cpus = cpus.clone();
            move |_| pin(&cpus)
        })
        .build()
        .unwrap();
    let worker_affinity = pool.broadcast(|_| affinity());
    assert!(worker_affinity.iter().all(|a| a == &cpus));
    let f = fixtures();
    let delta = (G2Affine::GENERATOR * Fr::from(37u64)).to_affine();
    let prepared = FixedBase::new(delta);
    for _ in 0..3 {
        pool.install(|| {
            for x in &f {
                check(case, call(case, mode, black_box(x), delta, &prepared), x)
            }
        });
    }
    let mut output = vec![
        (
            G1Projective::IDENTITY,
            G1Projective::IDENTITY,
            G2Projective::IDENTITY
        );
        rounds
    ];
    let mut control = perf::Control::from_env().unwrap();
    if let Some(c) = &mut control {
        c.enable().unwrap();
    }
    let a0 = ALLOCS.load(Ordering::Relaxed);
    let b0 = BYTES.load(Ordering::Relaxed);
    let u0 = perf::Usage::now().unwrap();
    let start = Instant::now();
    pool.install(|| {
        for (i, out) in output.iter_mut().enumerate() {
            *out = black_box(call(
                case,
                mode,
                black_box(&f[i % f.len()]),
                delta,
                black_box(&prepared),
            ));
        }
    });
    let elapsed = start.elapsed().as_nanos();
    let u1 = perf::Usage::now().unwrap();
    let allocs = ALLOCS.load(Ordering::Relaxed) - a0;
    let bytes = BYTES.load(Ordering::Relaxed) - b0;
    if let Some(c) = &mut control {
        c.disable().unwrap();
    }
    for (i, &out) in output.iter().enumerate() {
        check(case, out, &f[i % f.len()]);
    }
    let after = pool.broadcast(|_| affinity());
    assert_eq!(after, worker_affinity);
    assert_eq!(affinity(), cpus);
    println!(
        "{}",
        serde_json::json!({"case":case,"mode":mode,"rounds":rounds,"elapsed_ns":elapsed,"ns_per_call":elapsed as f64/rounds as f64,"usage":u1.difference(&u0),"timed_allocation_calls":allocs,"timed_requested_bytes":bytes,"worker_affinity":after,"all_outputs_verified":true,"SDK_requests":0})
    );
}
