//! Diagnostic only: one immutable fixture per process, never a proving path.
use std::sync::{OnceLock, atomic::{AtomicUsize, Ordering}};
use zopro_bn254::{G2Affine, G2Projective};
static FILLS: [OnceLock<Vec<G2Affine>>; 20] = [const { OnceLock::new() }; 20];
static SUMS: [OnceLock<G2Projective>; 10] = [const { OnceLock::new() }; 10];
static COUNTS: [AtomicUsize; 4] = [const { AtomicUsize::new(0) }; 4];
static MODE: OnceLock<usize> = OnceLock::new();
fn mode() -> usize {
    *MODE.get_or_init(|| {
        let m = std::env::var("ZOPRO_DIAGNOSTIC_ORDINARY_MODE").unwrap().parse().unwrap();
        assert!(m < 4); m
    })
}
pub(super) fn fill(w: usize, part: usize, compute: impl FnOnce() -> Vec<G2Affine>) -> Vec<G2Affine> {
    assert!(w < 10 && part < 2);
    let slot = &FILLS[2*w+part];
    COUNTS[0].fetch_add(1, Ordering::Relaxed);
    if mode() & 1 != 0 { if let Some(v) = slot.get() { return v.clone(); } }
    COUNTS[1].fetch_add(1, Ordering::Relaxed);
    let v = compute(); assert_eq!(v.len(), 2048);
    if slot.get().is_none() { assert!(slot.set(v.clone()).is_ok()); }
    v
}
pub(super) fn reduce(w: usize, compute: impl FnOnce() -> G2Projective) -> G2Projective {
    assert!(w < 10);
    let slot = &SUMS[w];
    COUNTS[2].fetch_add(1, Ordering::Relaxed);
    if mode() & 2 != 0 { if let Some(v) = slot.get() { return *v; } }
    COUNTS[3].fetch_add(1, Ordering::Relaxed);
    let v = compute();
    if slot.get().is_none() { assert!(slot.set(v).is_ok()); }
    v
}
pub(crate) fn status() -> [usize; 7] {
    [COUNTS[0].load(Ordering::Relaxed), COUNTS[1].load(Ordering::Relaxed),
     COUNTS[2].load(Ordering::Relaxed), COUNTS[3].load(Ordering::Relaxed),
     FILLS.iter().filter(|s| s.get().is_some()).count(),
     SUMS.iter().filter(|s| s.get().is_some()).count(),
     FILLS.iter().filter_map(|s| s.get()).map(|v| v.len()*std::mem::size_of::<G2Affine>()).sum::<usize>()
      + SUMS.iter().filter(|s| s.get().is_some()).count()*std::mem::size_of::<G2Projective>()]
}
pub(crate) fn check(calls: usize) -> [usize; 7] {
    let s = status();
    assert_eq!(s, [20*calls, if mode()&1 != 0 {20} else {20*calls},
        10*calls, if mode()&2 != 0 {10} else {10*calls},20,10,
        20*2048*std::mem::size_of::<G2Affine>()+10*std::mem::size_of::<G2Projective>()]);
    s
}
