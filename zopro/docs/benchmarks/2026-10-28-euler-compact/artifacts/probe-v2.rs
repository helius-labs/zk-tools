use rayon::prelude::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    alloc::{GlobalAlloc, Layout},
    fs,
    hint::black_box,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Instant,
};
use zopro_bn254::{
    ifma::Ifma,
    ifma8::{Ifma8, PackedIfma8},
    Fr, FrConfig,
};
type Pair = Ifma8<FrConfig>;
type PackedPair = PackedIfma8<FrConfig>;
struct Quad(Ifma<FrConfig>);
impl Quad {
    fn new(v: &[Fr; 4]) -> Self {
        Self(Ifma::new(v))
    }
    fn mul(&self, b: &Self) -> Self {
        Self(self.0.mul(&b.0))
    }
    fn add(&self, b: &Self) -> Self {
        Self(self.0.add(&b.0))
    }
    fn get(&self) -> [Fr; 4] {
        self.0.get()
    }
    fn unpair(p: &Pair) -> (Self, Self) {
        let (a, b) = p.halves();
        (Self(a), Self(b))
    }
}
trait PairStorage {
    fn load(&self) -> Pair;
}
impl PairStorage for PackedPair {
    fn load(&self) -> Pair {
        self.expand()
    }
}
include!("retained-output.rs");
#[path = "kernel-v2.rs"]
mod kernel;
#[path = "perf.rs"]
mod perf;
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
struct Fixture {
    a: Vec<Fr>,
    b: Vec<Fr>,
    values: Vec<PackedPair>,
    expected: Vec<Fr>,
    out: Vec<Fr>,
}
fn expected(a: &[Fr], b: &[Fr], values: &[PackedPair]) -> Vec<Fr> {
    a.iter()
        .enumerate()
        .map(|(i, &a)| {
            let d = values[i / 4].expand().get();
            d[i % 4] * b.get(i).copied().unwrap_or(Fr::ZERO) + a * d[i % 4 + 4]
        })
        .collect()
}
impl Fixture {
    fn load(path: &str, general: bool) -> Self {
        let bytes = fs::read(path).unwrap();
        assert_eq!(&bytes[..16], b"zopro-euler-in 1\0");
        let mut at = 16;
        let mut word = || {
            let n = u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap()) as usize;
            at += 8;
            n
        };
        let (na, nb, nv, size) = (word(), word(), word(), word());
        assert_eq!(size, core::mem::size_of::<PackedPair>());
        assert_eq!(size, 256);
        assert_eq!(bytes.len(), 48 + (na + nb) * 32 + nv * size);
        let mut fields = |n| {
            (0..n)
                .map(|_| {
                    let words = core::array::from_fn(|_| {
                        let w = u64::from_le_bytes(bytes[at..at + 8].try_into().unwrap());
                        at += 8;
                        w
                    });
                    assert!(Fr::from_canonical(words).is_some());
                    Fr::from_montgomery(words)
                })
                .collect::<Vec<_>>()
        };
        let mut a = fields(na);
        let mut b = fields(nb);
        // SAFETY: this immutable, hash-pinned local frame was written by the
        // same host's typed PackedIfma8 capture; all vector bits are valid.
        let values = bytes[at..]
            .chunks_exact(size)
            .map(|s| unsafe { core::ptr::read_unaligned::<PackedPair>(s.as_ptr().cast()) })
            .collect::<Vec<_>>();
        assert_eq!(values.len(), nv);
        if general {
            b.resize(a.len(), Fr::ZERO);
            for (i, a) in a.iter_mut().enumerate() {
                if *a == Fr::ZERO {
                    *a = Fr::ONE;
                }
                b[i] = *a + Fr::ONE;
                if b[i] == Fr::ZERO {
                    b[i] = *a + Fr::from(2u64);
                }
                assert!(b[i] != *a && b[i] != Fr::ZERO);
            }
        }
        let expected = expected(&a, &b, &values);
        let out = a.clone();
        Self {
            a,
            b,
            values,
            expected,
            out,
        }
    }
    fn reset(&mut self) {
        self.out.copy_from_slice(&self.a)
    }
    fn run(&mut self, mode: &str) {
        match mode {
            "accepted" | "fresh" => euler_combine_products(&mut self.out, &self.b, &self.values),
            "tiled" => kernel::tiled(&mut self.out, &self.b, &self.values),
            #[cfg(not(accepted_control))]
            "compact" => kernel::compact(&mut self.out, &self.b, &self.values),
            _ => panic!("mode"),
        }
    }
    fn check(&self) {
        assert_eq!(self.out, self.expected);
    }
    fn digest(&self) -> String {
        let mut h = Sha256::new();
        for f in &self.out {
            for w in f.to_canonical() {
                h.update(w.to_le_bytes());
            }
        }
        format!("{:x}", h.finalize())
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    let rounds: usize = a[4].parse().unwrap();
    assert!(rounds > 0);
    let general = a[3] == "general";
    assert!(general || a[3] == "actual");
    let workers: usize = std::env::var("RAYON_NUM_THREADS").unwrap().parse().unwrap();
    rayon::ThreadPoolBuilder::new()
        .num_threads(workers)
        .build_global()
        .unwrap();
    rayon::broadcast(|_| {});
    let mut profile = perf::Control::from_env().unwrap();
    let mut f = Fixture::load(&a[1], general);
    for _ in 0..3 {
        f.reset();
        f.run(&a[2]);
        f.check();
    }
    if let Some(control) = profile.as_mut() {
        control.enable().unwrap();
    }
    let before_usage = perf::Usage::now().unwrap();
    let mut elapsed = 0u128;
    let mut allocations = 0usize;
    for _ in 0..rounds {
        f.reset();
        let before = ALLOCS.load(Relaxed);
        let now = Instant::now();
        black_box(&mut f).run(&a[2]);
        elapsed += now.elapsed().as_nanos();
        allocations += ALLOCS.load(Relaxed) - before;
        black_box(&f.out);
    }
    let after_usage = perf::Usage::now().unwrap();
    if let Some(control) = profile.as_mut() {
        control.disable().unwrap();
    }
    f.check();
    println!(
        "RESULT {}",
        json!({"fixture":a[3],"mode":a[2],"workers":workers,"iterations":rounds,"prime_calls":3,"rows":f.a.len(),"elapsed_ns":elapsed,"ns_per_call":elapsed as f64/rounds as f64,"timed_allocations":allocations,"output_sha256":f.digest(),"verified_against_scalar_reference":true,"SDK_requests":0,"reset_in_timer":false,"fixed_index_scratch_bytes":core::mem::size_of::<[u16;kernel::TILE]>()*2,"whole_loop_usage_including_reset":after_usage.difference(&before_usage)})
    );
}

#[cfg(all(test, not(accepted_control)))]
mod tests {
    use super::*;
    fn fields(n: usize, mode: usize) -> (Vec<Fr>, Vec<Fr>, Vec<PackedPair>) {
        let mut seed = 0x8c4712a3u64;
        let mut next = || {
            let mut bytes = [0; 64];
            for c in bytes.chunks_mut(8) {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                c.copy_from_slice(&seed.to_le_bytes());
            }
            Fr::from_random_bytes(&bytes)
        };
        let a: Vec<_> = (0..n).map(|_| next()).collect();
        let b: Vec<_> = (0..n)
            .map(|i| match mode {
                0 => next(),
                1 => a[i],
                2 => {
                    if i % 3 == 0 {
                        a[i]
                    } else {
                        Fr::ZERO
                    }
                }
                _ => {
                    if i % 2 == 0 {
                        a[i]
                    } else {
                        -a[i]
                    }
                }
            })
            .collect();
        let values = (0..n.div_ceil(8) * 2)
            .map(|_| Pair::new(&core::array::from_fn(|_| next())).compact())
            .collect();
        (a, b, values)
    }
    #[test]
    fn every_mask_unequal_derivatives_and_partial_tile_matches_scalar() {
        for mask in 0u16..256 {
            let (mut a, mut b, values) = fields(256, 0);
            for i in 0..256 {
                if mask >> (i % 8) & 1 != 0 {
                    b[i] = a[i];
                } else {
                    b[i] = -a[i];
                }
                if i == 0 && mask % 3 == 0 {
                    a[i] = Fr::ZERO;
                }
                if i == 1 && mask % 7 == 0 {
                    b[i] = Fr::ZERO;
                }
            }
            let oracle = expected(&a, &b, &values);
            kernel::compact(&mut a, &b, &values);
            assert_eq!(a, oracle);
        }
        for n in [
            0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 63, 64, 65, 127, 128, 129, 255, 256, 257, 263, 264,
            265, 511, 512, 513, 1025,
        ] {
            for mode in 0..4 {
                let (a, b, values) = fields(n, mode);
                for nb in [0, n / 2, n] {
                    let oracle = expected(&a, &b[..nb], &values);
                    let mut out = a.clone();
                    let pointer = out.as_ptr();
                    kernel::compact(&mut out, &b[..nb], &values);
                    assert_eq!(out, oracle);
                    assert_eq!(out.as_ptr(), pointer);
                    out.copy_from_slice(&a);
                    kernel::compact(&mut out, &b[..nb], &values);
                    assert_eq!(out, oracle);
                }
            }
        }
    }
    #[test]
    fn native_compaction_matches_fullwidth_arkworks() {
        use ark_ff::PrimeField;
        let (mut a, mut b, values) = fields(777, 1);
        for i in 0..a.len() {
            match i % 7 {
                0 => {
                    a[i] = Fr::ZERO;
                    b[i] = Fr::ZERO
                }
                1 => a[i] = Fr::ZERO,
                2 => b[i] = Fr::ZERO,
                3 => b[i] = -a[i],
                _ => {}
            }
        }
        let cv = |v: Fr| {
            ark_bn254::Fr::from_le_bytes_mod_order(
                &v.to_canonical()
                    .into_iter()
                    .flat_map(u64::to_le_bytes)
                    .collect::<Vec<_>>(),
            )
        };
        let oracle: Vec<_> = a
            .iter()
            .enumerate()
            .map(|(i, &a)| {
                let d = values[i / 4].expand().get();
                cv(d[i % 4]) * cv(b[i]) + cv(a) * cv(d[i % 4 + 4])
            })
            .collect();
        kernel::compact(&mut a, &b, &values);
        for (v, w) in a.into_iter().zip(oracle) {
            assert_eq!(cv(v), w);
        }
    }
}
