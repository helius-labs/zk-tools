mod kernel;
use serde_json::json;
use std::{
    fs,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use zopro_bn254::{Field, Fq, Fr, G1Affine, G1Projective};
const R: &str = "/home/ubuntu/src/zopro-perf-work/common-z-small-native176";
fn complete(p: &[G1Affine]) -> G1Affine {
    p.iter()
        .fold(G1Projective::IDENTITY, |a, p| a + *p)
        .to_affine()
}
fn main() {
    let mut rng = 0x6a09e667f3bcc909u64;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng
    };
    let mut product_cases = 0;
    for active in 0..=8 {
        for _ in 0..32 {
            let a = std::array::from_fn(|j| {
                if j < active {
                    Fq::from_canonical([next(), next(), next(), next() & ((1u64 << 61) - 1)])
                        .unwrap()
                } else {
                    Fq::ONE
                }
            });
            assert!(a.iter().all(|a| !a.is_zero()));
            kernel::check_products(a);
            product_cases += 1;
        }
    }
    let mut inputs = Vec::new();
    let mut expected = Vec::new();
    let mut flags = Vec::new();
    for seed in 0..8u64 {
        let offset = Fr::from_canonical([
            0x4711ea5184f09a23u64.wrapping_mul(seed + 1),
            0x5192906ea3714f8b + seed,
            0,
            0,
        ])
        .unwrap();
        let step = Fr::from_u64(67 + 2 * seed);
        let mut p = G1Affine::GENERATOR.mul(&offset);
        let dp = G1Affine::GENERATOR.mul(&step);
        let walk = (0..17)
            .map(|_| {
                let q = p.to_affine();
                p += dp;
                q
            })
            .collect::<Vec<_>>();
        for n in 0..=17 {
            let input = walk[..n].to_vec();
            let want = G1Affine::GENERATOR
                .mul(
                    &(offset * Fr::from_u64(n as u64)
                        + step * Fr::from_u64((n * n.saturating_sub(1) / 2) as u64)),
                )
                .to_affine();
            assert_eq!(complete(&input), want);
            inputs.push(input);
            expected.push(want);
            flags.push((2..=16).contains(&n));
        }
    }
    let g = G1Affine::GENERATOR;
    let two = g.mul_limbs(&[2]).to_affine();
    let four = g.mul_limbs(&[4]).to_affine();
    for p in [
        vec![g, g],
        vec![g, -g],
        vec![G1Affine::IDENTITY, g],
        vec![g, two, four, -g],
        vec![g, two, -four, g],
        vec![g, two, G1Affine::IDENTITY, four],
    ] {
        expected.push(complete(&p));
        inputs.push(p);
        flags.push(false);
    }
    let mut outputs = 0;
    for ((p, want), &flag) in inputs.iter().zip(&expected).zip(&flags) {
        let saved = p.clone();
        let (out, used) = kernel::sum(p);
        assert_eq!(&out, want);
        assert_eq!(used, flag);
        assert_eq!(*p, saved);
        outputs += 1;
    }
    let inputs = Arc::new(inputs);
    let expected = Arc::new(expected);
    let flags = Arc::new(flags);
    let calls = AtomicUsize::new(0);
    let mut workers_seen = 0;
    for workers in [1, 16, 32] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .stack_size(512 << 10)
            .build()
            .unwrap();
        let seen = pool.broadcast(|cx| {
            assert!(cx.index() < workers);
            for _ in 0..3 {
                for ((p, want), &flag) in inputs.iter().zip(expected.iter()).zip(flags.iter()) {
                    let (out, used) = kernel::sum(p);
                    assert_eq!(&out, want);
                    assert_eq!(used, flag);
                    calls.fetch_add(1, Ordering::Relaxed);
                }
            }
            cx.index()
        });
        assert_eq!(seen, (0..workers).collect::<Vec<_>>());
        workers_seen += seen.len();
    }
    let result = json!({"passed":true,"product_butterfly_vectors":product_cases,"complete_cases":outputs,"worker_complete_calls":calls.load(Ordering::Relaxed),"actual_workers":workers_seen,"worker_counts":[1,16,32],"stack_bytes":512<<10,"new_timings":0,"new_proofs":0,"production_changed":false,"scope":"new standalone G1 SIMD algorithm, not real caller integration, portable, pressure or performance qualification"});
    let path = format!("{R}/qualification.json");
    assert!(!std::path::Path::new(&path).exists());
    fs::write(path, serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!("{result}");
}
