use serde_json::json;
use std::{
    alloc::{GlobalAlloc, Layout},
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
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
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 4);
    let selected = match a[2].as_str() {
        "binary" => false,
        "ternary" => true,
        _ => panic!("variant"),
    };
    let rounds: usize = a[3].parse().unwrap();
    assert!(rounds > 0);
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build_global()
        .unwrap();
    let (counts, fresh) = case(&a[1]);
    let mut fixture = ReductionCaller::new(&counts);
    for _ in 0..3 {
        fixture.run(selected, fresh, 8);
        fixture.check();
    }
    let before = ALLOCS.load(Relaxed);
    let start = Instant::now();
    black_box(&mut fixture).run(selected, fresh, rounds);
    let elapsed = start.elapsed();
    let allocations = ALLOCS.load(Relaxed) - before;
    fixture.check();
    if !fresh {
        assert_eq!(allocations, 0);
    }
    let scratch = fixture.scratch_bytes();
    println!(
        "RESULT {}",
        json!({"case":a[1],"variant":a[2],"rounds":rounds,"fresh_buffers":fresh,
        "counts":counts,"ns_per_round":elapsed.as_nanos()as f64/rounds as f64,"elapsed_ns":elapsed.as_nanos(),
        "timed_allocation_calls":allocations,"allocation_calls_per_round":allocations as f64/rounds as f64,
        "scratch_common_bytes":scratch[0],"scratch_binary_bytes":scratch[1],"scratch_ternary_bytes":scratch[2],
        "scratch_fixed_bytes":scratch[3],"scratch_total_bytes":scratch.iter().sum::<usize>(),
        "coordinates":fixture.coordinates(),"verified_projective":true,"SDK_requests":0})
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{Fr, G1Projective};
    use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
    use ark_ff::{AdditiveGroup, BigInt, Field, PrimeField};
    fn word(seed: &mut u64) -> [u64; 4] {
        let mut out = [0; 4];
        for v in &mut out {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            *v = *seed;
        }
        out[3] &= 0x1fffffffffffffff;
        out
    }
    fn check(words: &[[u64; 4]], counts: &[usize]) {
        let mut fixture = ReductionCaller::from_words(words, counts);
        let ordered: Vec<_> = fixture
            .order()
            .iter()
            .map(|&i| Fr::from_bigint(BigInt(words[i as usize])).unwrap())
            .collect();
        let mut at = 0;
        let expected: Vec<_> = counts
            .iter()
            .map(|&n| {
                let sum: Fr = ordered[at..at + n].iter().sum();
                at += n;
                let p = (G1Projective::generator() * sum).into_affine();
                match p.xy() {
                    Some((x, y)) => [x.into_bigint().0, y.into_bigint().0],
                    None => [[0; 4]; 2],
                }
            })
            .collect();
        for selected in [false, true] {
            for fresh in [false, true] {
                fixture.run(selected, fresh, 3);
                fixture.check();
                assert_eq!(fixture.coordinates(), expected);
            }
        }
    }
    #[test]
    fn complete_reductions_match_independent_arkworks() {
        let mut seed = 0x128f961aa734bc21;
        for n in [
            0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26, 47, 48, 49, 257, 384, 511, 2048,
        ] {
            let words: Vec<_> = (0..n).map(|_| word(&mut seed)).collect();
            check(&words, &[n]);
        }
        let counts = [0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26, 47, 48, 49];
        let words: Vec<_> = (0..counts.iter().sum()).map(|_| word(&mut seed)).collect();
        check(&words, &counts);
        let edges = [
            Fr::ZERO,
            Fr::ONE,
            -Fr::ONE,
            Fr::from(2u64),
            -Fr::from(2u64),
            Fr::from(3u64),
        ];
        for a in edges {
            for b in edges {
                for c in edges {
                    let mut w: Vec<_> = (0..48).map(|_| word(&mut seed)).collect();
                    w[24] = a.into_bigint().0;
                    w[25] = b.into_bigint().0;
                    w[26] = c.into_bigint().0;
                    check(&w, &[26, 22]);
                }
            }
        }
        println!("Independent Arkworks: full-width arrays, padded tails, empty/multiple groups,216 boundary triples;both paths/lifetimes");
    }
}
