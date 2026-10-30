//! Diagnostic-only fixed-capacity stage journal. Never contains proof values.
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::Instant,
};
const LIMIT: usize = 256;
static ENABLED: AtomicBool = AtomicBool::new(false);
static OVERFLOW: AtomicBool = AtomicBool::new(false);
static RECORDS: Mutex<Vec<[u64; 13]>> = Mutex::new(Vec::new());

/// Allocate before priming/timing in both OFF and traced processes.
pub fn prepare() {
    assert!(!ENABLED.load(Ordering::Acquire));
    let mut records = RECORDS.lock().unwrap();
    records.clear();
    if records.capacity() < LIMIT {
        records.reserve_exact(LIMIT);
    }
    OVERFLOW.store(false, Ordering::Release);
}
/// Called only at quiescent SDK boundaries after the three primes.
pub fn begin(enabled: bool) {
    assert!(!ENABLED.load(Ordering::Acquire));
    assert!(RECORDS.lock().unwrap().capacity() >= LIMIT);
    ENABLED.store(enabled, Ordering::Release);
}
/// Disable after timing and transfer records without timed serialization.
pub fn take() -> Vec<[u64; 13]> {
    ENABLED.store(false, Ordering::Release);
    assert!(!OVERFLOW.load(Ordering::Acquire), "timeline overflow");
    std::mem::take(&mut *RECORDS.lock().unwrap())
}

#[derive(Clone, Copy)]
pub(crate) struct Start(Option<Instant>);
impl Start {
    pub(crate) fn new() -> Self {
        Self(ENABLED.load(Ordering::Acquire).then(Instant::now))
    }
    pub(crate) fn stamp(self) -> u64 {
        self.0.map_or(0, |t| {
            u64::try_from(t.elapsed().as_nanos()).expect("timeline duration")
        })
    }
    pub(crate) fn measure<T>(self, f: impl FnOnce() -> T) -> (T, [u64; 2]) {
        let before = self.stamp();
        let value = f();
        let after = self.stamp();
        (value, [before, after])
    }
    pub(crate) fn finish(self, marks: [u64; 7], queries: [[u64; 2]; 3]) {
        if self.0.is_none() {
            return;
        }
        let mut record = [0; 13];
        record[..7].copy_from_slice(&marks);
        for (i, q) in queries.into_iter().enumerate() {
            record[7 + 2 * i..9 + 2 * i].copy_from_slice(&q);
        }
        assert!(valid(&record), "invalid timeline ordering");
        let mut records = RECORDS.lock().unwrap();
        if records.len() == LIMIT {
            OVERFLOW.store(true, Ordering::Release);
        } else {
            records.push(record);
        }
    }
}
fn valid(r: &[u64; 13]) -> bool {
    r[..7].windows(2).all(|p| p[0] <= p[1])
        && r[7..]
            .chunks_exact(2)
            .all(|q| r[4] <= q[0] && q[0] <= q[1] && q[1] <= r[5])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stage_order_and_query_containment() {
        let good = [1, 2, 3, 4, 5, 12, 13, 5, 8, 6, 11, 5, 12];
        assert!(valid(&good));
        for i in 0..6 {
            let mut r = good;
            r[i] = r[i + 1] + 1;
            assert!(!valid(&r));
        }
        for i in [7, 9, 11] {
            let mut r = good;
            r[i] = 4;
            assert!(!valid(&r));
            let mut r = good;
            r[i + 1] = 13;
            assert!(!valid(&r));
            let mut r = good;
            r[i] = r[i + 1] + 1;
            assert!(!valid(&r));
        }
    }
    #[test]
    fn bounded_journal_parallel_spans_and_disabled_reset() {
        prepare();
        begin(false);
        let t = Start::new();
        assert_eq!(t.stamp(), 0);
        let (v, q) = t.measure(|| 73);
        assert_eq!((v, q), (73, [0, 0]));
        t.finish([0; 7], [[0; 2]; 3]);
        assert!(take().is_empty());
        prepare();
        begin(true);
        let t = Start::new();
        let mut marks = [0; 7];
        for m in &mut marks[..5] {
            *m = t.stamp();
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let ((a, b), c) = pool.install(|| {
            rayon::join(
                || rayon::join(|| t.measure(|| 11), || t.measure(|| 13)),
                || t.measure(|| 17),
            )
        });
        assert_eq!((a.0, b.0, c.0), (11, 13, 17));
        marks[5] = t.stamp();
        marks[6] = t.stamp();
        for _ in 0..LIMIT {
            t.finish(marks, [a.1, b.1, c.1]);
        }
        assert_eq!(RECORDS.lock().unwrap().len(), LIMIT);
        t.finish(marks, [a.1, b.1, c.1]);
        assert!(OVERFLOW.load(Ordering::Acquire));
        ENABLED.store(false, Ordering::Release);
        prepare();
        begin(true);
        t.finish(marks, [a.1, b.1, c.1]);
        let out = take();
        assert_eq!(out.len(), 1);
        assert!(valid(&out[0]));
        assert!(!ENABLED.load(Ordering::Acquire));
    }
}
