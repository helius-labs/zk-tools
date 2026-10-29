//! Diagnostic-only flow counters around the unchanged borrowed Window methods.
use super::*;
use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};
static QUERY: AtomicU64 = AtomicU64::new(0);
thread_local! {static CONTEXT:Cell<(u64,usize,usize)>=const{Cell::new((0,usize::MAX,usize::MAX))};}
pub(super) fn enabled() -> bool {
    std::env::var_os("ZOPRO_GLS4_CENSUS").is_some_and(|v| v == "1")
}
pub(super) fn query() -> u64 {
    if enabled() {
        QUERY.fetch_add(1, Ordering::Relaxed) + 1
    } else {
        0
    }
}
pub(super) struct Scope((u64, usize, usize));
pub(super) fn context(q: u64, w: usize, p: usize) -> Scope {
    Scope(CONTEXT.with(|c| c.replace((q, w, p))))
}
impl Drop for Scope {
    fn drop(&mut self) {
        CONTEXT.with(|c| c.set(self.0));
    }
}

#[derive(Default, Debug)]
struct Counts {
    input: usize,
    skip_zero: usize,
    skip_identity: usize,
    nonzero: usize,
    direct: usize,
    push: usize,
    pop: usize,
    flushed: usize,
    assigned: usize,
    doubled: usize,
    cancelled: usize,
    ordinary: usize,
    executed: usize,
    batches: usize,
    proj_init: usize,
    proj_update: usize,
    max_digit: usize,
    distinct: usize,
    max_arrivals: usize,
    sum_squared_arrivals: u64,
}
#[derive(Debug)]
struct Record {
    count: Counts,
    arrivals: [u32; 1024],
    batch_hist: [usize; 153],
}
impl Record {
    fn new(input: usize) -> Self {
        Self {
            count: Counts {
                input,
                ..Default::default()
            },
            arrivals: [0; 1024],
            batch_hist: [0; 153],
        }
    }
    fn check(&mut self) {
        let c = &mut self.count;
        assert_eq!(c.input, c.skip_zero + c.skip_identity + c.nonzero);
        assert_eq!(c.nonzero, c.direct + c.push);
        assert_eq!(c.push, c.pop + c.flushed);
        assert_eq!(
            c.direct + c.pop,
            c.assigned + c.doubled + c.cancelled + c.ordinary
        );
        assert_eq!(c.ordinary, c.executed);
        assert_eq!(c.doubled + c.flushed, c.proj_init + c.proj_update);
        assert_eq!(c.batches, self.batch_hist.iter().sum());
        assert_eq!(
            c.executed,
            self.batch_hist.iter().enumerate().map(|(n, k)| n * k).sum()
        );
        assert_eq!(c.nonzero, self.arrivals.iter().map(|n| *n as usize).sum());
        c.distinct = self.arrivals.iter().filter(|n| **n > 0).count();
        c.max_arrivals = *self.arrivals.iter().max().unwrap() as usize;
        c.sum_squared_arrivals = self
            .arrivals
            .iter()
            .map(|n| u64::from(*n) * u64::from(*n))
            .sum();
    }
    fn emit(&self, component: usize) {
        let (q, w, p) = CONTEXT.with(Cell::get);
        assert!(q > 0 && w < 6 && p < 4);
        let c = &self.count;
        eprintln!(concat!("GLS4_CENSUS {{\"query\":{},\"window\":{},\"part\":{},\"component\":{},",
            "\"input\":{},\"skip_zero\":{},\"skip_identity\":{},\"nonzero\":{},\"direct\":{},\"push\":{},\"pop\":{},\"flushed\":{},",
            "\"assigned\":{},\"doubled\":{},\"cancelled\":{},\"ordinary\":{},\"executed\":{},\"batches\":{},\"proj_init\":{},\"proj_update\":{},",
            "\"max_digit\":{},\"distinct\":{},\"max_arrivals\":{},\"sum_squared_arrivals\":{},\"batch_hist\":{:?},\"flow_checked\":true}}"),
            q,w,p,component,c.input,c.skip_zero,c.skip_identity,c.nonzero,c.direct,c.push,c.pop,c.flushed,
            c.assigned,c.doubled,c.cancelled,c.ordinary,c.executed,c.batches,c.proj_init,c.proj_update,
            c.max_digit,c.distinct,c.max_arrivals,c.sum_squared_arrivals,self.batch_hist);
    }
}
fn add<'a>(state: &mut Window<'a>, encoded: u32, p: &'a G2Affine, r: &mut Record) {
    let b = (encoded & 0x7fffffff) as usize;
    let was_identity = state.affine[b].is_identity();
    let old_batch = state.batch.len();
    let existed = state.projective_indices[b] != u32::MAX;
    state.add(encoded, p);
    if state.batch.len() == old_batch + 1 {
        r.count.ordinary += 1;
    } else {
        assert_eq!(state.batch.len(), old_batch);
        if was_identity {
            r.count.assigned += 1;
        } else if state.affine[b].is_identity() {
            r.count.cancelled += 1;
        } else {
            r.count.doubled += 1;
            if existed {
                r.count.proj_update += 1;
            } else {
                r.count.proj_init += 1;
            }
        }
    }
}
fn execute(state: &mut Window<'_>, r: &mut Record) {
    let n = state.batch.len();
    if n > 0 {
        assert!(n < r.batch_hist.len());
        r.count.batches += 1;
        r.count.executed += n;
        r.batch_hist[n] += 1;
    }
    state.execute();
    assert!(state.batch.is_empty());
}
fn take<'a>(state: &mut Window<'a>, r: &mut Record) {
    while let Some(&(encoded, p)) = state.queue.last() {
        if state.busy((encoded & 0x7fffffff) as usize) {
            return;
        }
        state.queue.pop();
        r.count.pop += 1;
        add(state, encoded, p, r);
    }
}
fn flush(state: &mut Window<'_>, r: &mut Record) {
    let mut seen = [0u64; 16];
    let before = state.projective.len();
    let mut created = 0;
    for &(encoded, _) in &state.queue {
        let b = (encoded & 0x7fffffff) as usize;
        assert!(b < 1024);
        let bit = 1u64 << (b % 64);
        if state.projective_indices[b] == u32::MAX && seen[b / 64] & bit == 0 {
            r.count.proj_init += 1;
            created += 1;
        } else {
            r.count.proj_update += 1;
        }
        seen[b / 64] |= bit;
        r.count.flushed += 1;
    }
    state.flush_queue();
    assert!(state.queue.is_empty());
    assert_eq!(state.projective.len() - before, created);
}
fn observed<'a>(
    state: &mut Window<'a>,
    bases: &'a [G2Affine],
    short: &[[u64; 5]],
    component: usize,
    w: usize,
    size: usize,
) -> Record {
    assert_eq!(bases.len(), short.len());
    assert!(state.batch.is_empty() && state.queue.is_empty());
    let mut r = Record::new(bases.len());
    for (p, s) in bases.iter().zip(short) {
        let d = component_digit(s, component, w, BITS);
        if d == 0 {
            r.count.skip_zero += 1;
            continue;
        }
        if p.is_identity() {
            r.count.skip_identity += 1;
            continue;
        }
        let b = d.unsigned_abs() as usize - 1;
        assert!(b < 1024);
        r.count.nonzero += 1;
        r.arrivals[b] += 1;
        r.count.max_digit = r.count.max_digit.max(b + 1);
        let encoded = b as u32 | if d < 0 { 1 << 31 } else { 0 };
        if state.busy(b) {
            state.queue.push((encoded, p));
            r.count.push += 1;
            if state.queue.len() == size - 1 {
                flush(state, &mut r);
            }
            continue;
        }
        r.count.direct += 1;
        add(state, encoded, p, &mut r);
        if state.batch.len() == size {
            execute(state, &mut r);
            take(state, &mut r);
        }
    }
    execute(state, &mut r);
    flush(state, &mut r);
    assert!(state.batch.is_empty() && state.queue.is_empty());
    assert!(state.busy.iter().all(|v| *v == 0));
    r.check();
    r
}
pub(super) fn run_feed<'a>(
    state: &mut Window<'a>,
    bases: &'a [G2Affine],
    short: &[[u64; 5]],
    component: usize,
    w: usize,
    size: usize,
) {
    let r = observed(state, bases, short, component, w, size);
    r.emit(component);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn same_state(a: &Window<'_>, b: &Window<'_>) {
        assert_eq!(a.affine, b.affine);
        assert_eq!(a.projective_indices, b.projective_indices);
        assert_eq!(a.projective_buckets, b.projective_buckets);
        assert_eq!(a.busy, b.busy);
        assert!(
            a.batch.is_empty() && b.batch.is_empty() && a.queue.is_empty() && b.queue.is_empty()
        );
        assert_eq!(a.projective.len(), b.projective.len());
        for (x, y) in a.projective.iter().zip(&b.projective) {
            assert_eq!(x.x, y.x);
            assert_eq!(x.y, y.y);
            assert_eq!(x.z, y.z);
        }
    }
    #[test]
    fn observed_state_matches_original_horner_feeds() {
        assert!(!enabled());
        let g = G2Projective::generator();
        let mut cursor = G2Projective::IDENTITY;
        let mut projective = Vec::new();
        for i in 0..200 {
            cursor += g;
            projective.push(if i % 17 == 0 {
                G2Projective::IDENTITY
            } else if i % 5 == 0 {
                -cursor
            } else {
                cursor
            });
        }
        let points = G2Projective::batch_to_affine(&projective);
        let x_factor = PSI_X.mul_by_fp(&BETA.square());
        for n in [0usize, 7, 33, 200] {
            for clumped in [false, true] {
                for w in [0usize, 5] {
                    let short: Vec<_> = (0..n)
                        .map(|i| {
                            split(
                                if clumped {
                                    Fr::ONE
                                } else {
                                    Fr::from((i + 7) as u64).pow(&[137])
                                }
                                .to_canonical(),
                            )
                        })
                        .collect();
                    let mut a = Window::new(1024, 32, BatchAdd::default());
                    let mut b = Window::new(1024, 32, BatchAdd::default());
                    for component in [3, 2, 1, 0] {
                        let row = observed(&mut a, &points[..n], &short, component, w, 32);
                        assert_eq!(row.count.input, n);
                        b.feed_gls4(&points[..n], &short, component, w, 32);
                        same_state(&a, &b);
                        match component {
                            3 | 1 => {
                                a.phi();
                                b.phi();
                            }
                            2 => {
                                a.compound(&x_factor);
                                b.compound(&x_factor);
                            }
                            _ => {}
                        }
                    }
                    assert_eq!(a.finish(), b.finish());
                }
            }
        }
    }
    #[test]
    fn explicit_exception_and_queue_counts() {
        let g = G2Projective::generator();
        let points: Vec<_> = [1u64, 1, 2, 3, 4, 5, 0, 1]
            .into_iter()
            .map(|i| (g * Fr::from(i)).to_affine())
            .collect();
        let mut words = vec![[1, 0, 0, 0, 0]; 8];
        words[1][4] = 1 << 4;
        words[7][0] = 0;
        let mut state = Window::new(1024, 32, BatchAdd::default());
        state.affine[0] = g.to_affine();
        let row = observed(&mut state, &points, &words, 0, 0, 32);
        let c = &row.count;
        assert_eq!(
            (c.input, c.skip_zero, c.skip_identity, c.nonzero),
            (8, 1, 1, 6)
        );
        assert_eq!((c.direct, c.push, c.pop, c.flushed), (4, 2, 0, 2));
        assert_eq!(
            (c.assigned, c.doubled, c.cancelled, c.ordinary),
            (1, 1, 1, 1)
        );
        assert_eq!(
            (c.executed, c.batches, c.proj_init, c.proj_update),
            (1, 1, 1, 2)
        );
        assert_eq!(state.finish()[0], (g * Fr::from(15)).to_affine());
    }
}
