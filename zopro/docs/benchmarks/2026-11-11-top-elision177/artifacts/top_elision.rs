//! Fixed-input diagnostic only. One immutable fixture per process.
use std::sync::{OnceLock, atomic::{AtomicUsize, Ordering}};
use zopro_bn254::G2Affine;
static SAVED: [OnceLock<Vec<G2Affine>>; 4] = [const { OnceLock::new() }; 4];
static CALLS: AtomicUsize = AtomicUsize::new(0);
static EXECUTED: AtomicUsize = AtomicUsize::new(0);
static MODE: OnceLock<bool> = OnceLock::new();
pub(super) fn call(id: usize, compute: impl FnOnce() -> Vec<G2Affine>) -> Vec<G2Affine> {
    assert!(id < 4);
    CALLS.fetch_add(1, Ordering::Relaxed);
    let elide = *MODE.get_or_init(|| match std::env::var("ZOPRO_DIAGNOSTIC_ELIDE_TOP").as_deref() {
        Ok("0") => false, Ok("1") => true, _ => panic!("explicit diagnostic mode required"),
    });
    if elide {
        if let Some(saved) = SAVED[id].get() { return saved.clone(); }
    }
    EXECUTED.fetch_add(1, Ordering::Relaxed);
    let result = compute();
    if SAVED[id].get().is_none() { assert!(SAVED[id].set(result.clone()).is_ok()); }
    result
}
pub(crate) fn status() -> (usize, usize, usize, usize) {
    (CALLS.load(Ordering::Relaxed), EXECUTED.load(Ordering::Relaxed),
     SAVED.iter().filter(|s|s.get().is_some()).count(),
     SAVED.iter().filter_map(|s|s.get()).map(|v|v.len()*std::mem::size_of::<G2Affine>()).sum())
}
