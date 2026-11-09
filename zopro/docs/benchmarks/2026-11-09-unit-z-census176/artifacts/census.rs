//! Diagnostic-only call classification; not linked into a timed candidate.
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use crate::{CurveConfig, G1Config, G2Config};
pub const LABELS: [&str; 9] = ["calls", "right_identity", "left_identity", "ordinary_unit", "equal_unit", "opposite_unit", "ordinary_nonunit", "equal_nonunit", "opposite_nonunit"];
static ACTIVE: AtomicBool = AtomicBool::new(false);
static COUNTS: [[AtomicU64; 9]; 2] = [const { [const { AtomicU64::new(0) }; 9] }; 2];
#[inline]
pub(crate) fn bump<C: CurveConfig>(event: usize) {
    if !ACTIVE.load(Ordering::Relaxed) { return; }
    let curve = if std::any::TypeId::of::<C>() == std::any::TypeId::of::<G1Config>() { 0 }
        else { assert_eq!(std::any::TypeId::of::<C>(),std::any::TypeId::of::<G2Config>()); 1 };
    COUNTS[curve][event].fetch_add(1, Ordering::Relaxed);
}
pub fn begin() {
    assert!(!ACTIVE.load(Ordering::Acquire));
    for curve in &COUNTS { for n in curve { n.store(0, Ordering::Relaxed); } }
    ACTIVE.store(true, Ordering::Release);
}
pub fn finish() -> [[u64; 9]; 2] {
    assert!(ACTIVE.swap(false,Ordering::AcqRel));
    snapshot()
}
pub fn snapshot() -> [[u64; 9]; 2] {
    core::array::from_fn(|i|core::array::from_fn(|j|COUNTS[i][j].load(Ordering::Relaxed)))
}
