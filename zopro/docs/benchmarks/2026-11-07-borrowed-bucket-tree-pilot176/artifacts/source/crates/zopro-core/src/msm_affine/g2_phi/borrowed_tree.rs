//! Proof-local balanced borrowed G2 bucket fills. No persistent point state.
use super::{hb_digit, BETA};
use crate::{msm::BatchAdd, preparation::Reservation};
use rayon::prelude::*;
use zopro_bn254::{G2Affine, G2Config};

const BUCKETS: usize = 2048;
const BAND: usize = 256;
#[cfg(test)]
static WINDOW_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}
fn mul(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(b)
}

/// Additional requested-capacity bound; the original proof forecast is retained.
pub(super) fn extra_bytes(n: usize, workers: usize) -> Option<usize> {
    use super::super::workspace::vector as v;
    if !(1024..65536).contains(&n) || workers == 0 {
        return None;
    }
    let part = n.div_ceil(2);
    let output = add(add(part, 2 * BAND)?.div_ceil(2), 7)?;
    let pairs = add(add(part, BAND)?.div_ceil(2), 7)?.next_multiple_of(8);
    let routes = add(
        v::<u32>(mul(part, 2)?, false)?,
        mul(v::<usize>(2 * BUCKETS + 1, false)?, 3)?,
    )?;
    // Count returned affine buckets as well, conservatively above the old floor.
    let held = mul(add(routes, v::<G2Affine>(BUCKETS, false)?)?, 20)?;
    let mut leaf = mul(v::<G2Affine>(output, false)?, 2)?;
    leaf = add(leaf, v::<(u32, &G2Affine)>(pairs, false)?)?;
    leaf = add(leaf, crate::msm::workspace_batch::<G2Config>(pairs)?)?;
    leaf = add(leaf, core::mem::size_of::<State>())?;
    leaf = add(leaf, core::mem::size_of::<[G2Affine; BAND]>() * 3)?;
    leaf = add(leaf, core::mem::size_of::<[usize; BAND]>() * 6)?;
    add(held, mul(leaf, workers)?)
}

pub(super) fn admit(n: usize) -> Option<Reservation> {
    if !cfg!(zopro_borrowed_bucket_tree) || !BatchAdd::<G2Config>::default().borrowed_g2_enabled() {
        return None;
    }
    Reservation::analysis(extra_bytes(n, rayon::current_num_threads()))
}

#[derive(Clone, Copy)]
struct Leaf<'a> {
    point: &'a G2Affine,
    negative: bool,
}
impl Leaf<'_> {
    fn value(self) -> G2Affine {
        if self.negative {
            -*self.point
        } else {
            *self.point
        }
    }
}

// The zero-length output carries no old references. Allocation reuse is an
// optimization; Rust's lifetime checking, not that optimization, provides safety.
fn empty<'old, 'new>(batch: Vec<(u32, &'old G2Affine)>) -> Vec<(u32, &'new G2Affine)> {
    batch
        .into_iter()
        .map(|_| -> (u32, &'new G2Affine) { unreachable!("take(0)") })
        .take(0)
        .collect()
}
#[derive(Default)]
struct PairStorage(Vec<(u32, &'static G2Affine)>);
impl PairStorage {
    fn take<'a>(&mut self) -> Vec<(u32, &'a G2Affine)> {
        debug_assert!(self.0.is_empty());
        empty(core::mem::take(&mut self.0))
    }
    fn restore(&mut self, batch: Vec<(u32, &G2Affine)>) {
        self.0 = empty(batch);
    }
}
#[derive(Default)]
struct State {
    current: Vec<G2Affine>,
    next: Vec<G2Affine>,
    pairs: PairStorage,
    adder: BatchAdd<G2Config>,
    #[cfg(test)]
    stats: Stats,
}
#[cfg(test)]
#[derive(Default, Clone, Debug)]
struct Stats {
    calls: usize,
    real_pairs: usize,
    dummy_pairs: usize,
    exceptional: usize,
}

fn append<'a>(
    out: &mut Vec<G2Affine>,
    batch: &mut Vec<(u32, &'a G2Affine)>,
    left: G2Affine,
    right: Leaf<'a>,
) -> bool {
    if left.is_identity() {
        out.push(right.value());
        return true;
    }
    if right.point.is_identity() {
        out.push(left);
        return true;
    }
    if left.x == right.point.x {
        out.push((left + right.value()).to_affine());
        return true;
    }
    let index = u32::try_from(out.len()).unwrap();
    assert!(index < 0x8000_0000);
    out.push(left);
    batch.push((
        index | if right.negative { 1 << 31 } else { 0 },
        right.point,
    ));
    false
}
fn execute(
    out: &mut Vec<G2Affine>,
    batch: &mut Vec<(u32, &G2Affine)>,
    adder: &mut BatchAdd<G2Config>,
) -> (usize, usize) {
    let real_pairs = batch.len();
    if real_pairs == 0 {
        return (0, 0);
    }
    let real_outputs = out.len();
    let (encoded, right) = batch[0];
    let left = out[(encoded & 0x7fff_ffff) as usize];
    let padding = (8 - real_pairs % 8) % 8;
    for _ in 0..padding {
        let index = u32::try_from(out.len()).unwrap();
        out.push(left);
        batch.push((index | (encoded & 0x8000_0000), right));
    }
    assert_eq!(batch.len() % 8, 0);
    adder.apply_signed_borrowed_g2(out, batch);
    out.truncate(real_outputs);
    (real_pairs, padding)
}

fn stage<'a>(
    counts: &[usize; BAND],
    get: impl Fn(usize, usize) -> Leaf<'a>,
    state: &mut State,
) -> [G2Affine; BAND] {
    let mut counts = *counts;
    let mut starts = [0usize; BAND];
    let mut batch = state.pairs.take();
    let slots = counts.iter().map(|n| n.div_ceil(2)).sum::<usize>();
    let pair_count = counts.iter().map(|n| n / 2).sum::<usize>();
    state.current.clear();
    state.next.clear();
    state.current.reserve_exact(slots + 7);
    batch.reserve_exact(pair_count.next_multiple_of(8));
    let mut exceptional = 0;
    for b in 0..BAND {
        starts[b] = state.current.len();
        for i in (0..counts[b]).step_by(2) {
            let left = get(b, i).value();
            if i + 1 < counts[b] {
                exceptional +=
                    usize::from(append(&mut state.current, &mut batch, left, get(b, i + 1)));
            } else {
                state.current.push(left);
            }
        }
        counts[b] = counts[b].div_ceil(2);
    }
    let metrics = execute(&mut state.current, &mut batch, &mut state.adder);
    #[cfg(test)]
    {
        state.stats.calls += usize::from(metrics.0 > 0);
        state.stats.real_pairs += metrics.0;
        state.stats.dummy_pairs += metrics.1;
        state.stats.exceptional += exceptional;
    }
    #[cfg(not(test))]
    let _ = (metrics, exceptional);
    state.pairs.restore(batch);
    while counts.iter().any(|&n| n > 1) {
        let slots = counts.iter().map(|n| n.div_ceil(2)).sum::<usize>();
        let pair_count = counts.iter().map(|n| n / 2).sum::<usize>();
        state.next.clear();
        state.next.reserve_exact(slots + 7);
        let mut batch = state.pairs.take();
        batch.reserve_exact(pair_count.next_multiple_of(8));
        let mut next_starts = [0usize; BAND];
        let mut exceptional = 0;
        for b in 0..BAND {
            next_starts[b] = state.next.len();
            for i in (0..counts[b]).step_by(2) {
                let left = state.current[starts[b] + i];
                if i + 1 < counts[b] {
                    let right = Leaf {
                        point: &state.current[starts[b] + i + 1],
                        negative: false,
                    };
                    exceptional += usize::from(append(&mut state.next, &mut batch, left, right));
                } else {
                    state.next.push(left);
                }
            }
            counts[b] = counts[b].div_ceil(2);
        }
        let metrics = execute(&mut state.next, &mut batch, &mut state.adder);
        #[cfg(test)]
        {
            state.stats.calls += usize::from(metrics.0 > 0);
            state.stats.real_pairs += metrics.0;
            state.stats.dummy_pairs += metrics.1;
            state.stats.exceptional += exceptional;
        }
        #[cfg(not(test))]
        let _ = (metrics, exceptional);
        state.pairs.restore(batch);
        core::mem::swap(&mut state.current, &mut state.next);
        starts = next_starts;
    }
    core::array::from_fn(|b| {
        if counts[b] == 0 {
            G2Affine::IDENTITY
        } else {
            state.current[starts[b]]
        }
    })
}

struct Route {
    order: Vec<u32>,
    starts: Vec<usize>,
}
impl Route {
    fn new(bases: &[G2Affine], short: &[[u64; 4]], w: usize, high: bool) -> Self {
        let mut counts = vec![0usize; BUCKETS];
        for (p, s) in bases.iter().zip(short) {
            let d = hb_digit(s, high, w, 12);
            if d != 0 && !p.is_identity() {
                counts[d.unsigned_abs() as usize - 1] += 1;
            }
        }
        let mut starts = Vec::with_capacity(BUCKETS + 1);
        starts.push(0);
        for n in counts {
            starts.push(starts.last().unwrap() + n);
        }
        let mut positions = starts[..BUCKETS].to_vec();
        let mut order = vec![0u32; starts[BUCKETS]];
        for (i, (p, s)) in bases.iter().zip(short).enumerate() {
            let d = hb_digit(s, high, w, 12);
            if d != 0 && !p.is_identity() {
                let b = d.unsigned_abs() as usize - 1;
                order[positions[b]] = i as u32 | if d < 0 { 1 << 31 } else { 0 };
                positions[b] += 1;
            }
        }
        Self { order, starts }
    }
    fn count(&self, bucket: usize) -> usize {
        self.starts[bucket + 1] - self.starts[bucket]
    }
    fn leaf<'a>(&self, bases: &'a [G2Affine], bucket: usize, i: usize) -> Leaf<'a> {
        let encoded = self.order[self.starts[bucket] + i];
        Leaf {
            point: &bases[(encoded & 0x7fff_ffff) as usize],
            negative: encoded >> 31 != 0,
        }
    }
}

pub(super) fn window(bases: &[G2Affine], short: &[[u64; 4]], w: usize) -> Vec<G2Affine> {
    assert_eq!(bases.len(), short.len());
    assert!(bases.len() < 65536 && w < 10);
    #[cfg(test)]
    WINDOW_CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let high = Route::new(bases, short, w, true);
    let low = Route::new(bases, short, w, false);
    let mut out = vec![G2Affine::IDENTITY; BUCKETS];
    out.par_chunks_mut(BAND)
        .enumerate()
        .for_each_init(State::default, |state, (band, output)| {
            let start = band * BAND;
            let hc = core::array::from_fn(|b| high.count(start + b));
            let mut roots = stage(&hc, |b, i| high.leaf(bases, start + b, i), state);
            for root in &mut roots {
                if !root.is_identity() {
                    root.x = root.x.mul_by_fp(&BETA);
                }
            }
            let lc = core::array::from_fn(|b| low.count(start + b) + usize::from(hc[b] > 0));
            let result = stage(
                &lc,
                |b, i| {
                    if hc[b] > 0 && i == 0 {
                        Leaf {
                            point: &roots[b],
                            negative: false,
                        }
                    } else {
                        low.leaf(bases, start + b, i - usize::from(hc[b] > 0))
                    }
                },
                state,
            );
            output.copy_from_slice(&result);
        });
    out
}

#[cfg(test)]
mod tests;
