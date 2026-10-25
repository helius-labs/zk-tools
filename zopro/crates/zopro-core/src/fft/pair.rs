//! Two independent transforms share the lower/upper four IFMA lanes.
use super::*;
use zopro_bn254::{ifma8::Ifma8, FrConfig};
type Pair = Ifma8<FrConfig>;
const TASK: usize = 64;

#[inline(always)]
fn broadcast(q: &Quad) -> Pair {
    Quad::pair(q, q)
}

pub(super) fn quotient(plan: &Plan, a: Vec<Fr>, b: Vec<Fr>, c: Vec<Fr>) -> Vec<Fr> {
    let depth = std::env::var("ZOPRO_CPU_FFT_STAGES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .or_else(|| {
            if std::env::var("ZOPRO_CPU_FFT4").is_ok_and(|v| v == "1") {
                Some(4)
            } else if std::env::var("ZOPRO_CPU_FFT3").is_ok_and(|v| v == "1") {
                Some(3)
            } else {
                std::env::var("ZOPRO_CPU_FFT2")
                    .ok()
                    .map(|v| if v == "0" { 1 } else { 2 })
            }
        })
        .unwrap_or(if plan.n >= (1 << 15) { 5 } else { 1 })
        .clamp(1, 5);
    match depth {
        5 => quotient_impl::<5>(plan, a, b, c),
        4 => quotient_impl::<4>(plan, a, b, c),
        3 => quotient_impl::<3>(plan, a, b, c),
        2 => quotient_impl::<2>(plan, a, b, c),
        _ => quotient_impl::<1>(plan, a, b, c),
    }
}
pub(super) fn quotient_impl<const DEPTH: usize>(
    plan: &Plan,
    a: Vec<Fr>,
    b: Vec<Fr>,
    mut c: Vec<Fr>,
) -> Vec<Fr> {
    let started = crate::time::Instant::now();
    assert!(a.len() <= plan.n && b.len() <= plan.n && c.len() <= plan.n);
    c.reserve(plan.n - c.len());
    let pack = |i: usize| {
        let mut lanes = [Fr::ZERO; 8];
        for j in 0..4 {
            lanes[j] = a.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
            lanes[4 + j] = b.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
        }
        Pair::new(&lanes)
    };
    let mut values: Vec<Pair> = (0..plan.n / 4).into_par_iter().map(pack).collect();
    drop(a);
    drop(b);
    let started = crate::trace("fft-pack", started);
    dif::<DEPTH>(&mut values, &plan.inverse_roots, 0);
    let started = crate::trace("fft-inverse-ab", started);
    if plan.twisted.is_none() {
        values
            .par_chunks_mut(plan.forward_weights.lows.len())
            .zip(&plan.forward_weights.chunks)
            .for_each(|(values, chunk)| {
                let chunk = broadcast(chunk);
                for (value, low) in values.iter_mut().zip(&plan.forward_weights.lows) {
                    *value = value.mul(&broadcast(low)).mul(&chunk);
                }
            });
    }
    let started = crate::trace("fft-weights", started);
    let (roots, reverse_weights) = plan
        .twisted
        .as_ref()
        .map(|(roots, weights)| (roots, weights))
        .unwrap_or((&plan.roots, &plan.reverse_weights));
    dit::<DEPTH>(&mut values, roots, 0);
    let started = crate::trace("fft-forward-ab", started);
    values.par_iter_mut().enumerate().for_each(|(i, value)| {
        let (a, b) = Quad::unpair(value);
        let c = if 4 * i >= c.len() {
            Quad::zero()
        } else {
            Quad::new(&core::array::from_fn(|j| {
                c.get(4 * i + j).copied().unwrap_or(Fr::ZERO)
            }))
        };
        *value = Quad::pair(&a.mul(&b), &c);
    });
    // C is consumed by the paired transform above. Its allocation already
    // has room for the final coefficients, so reuse it for the output.
    let mut out = c;
    let started = crate::trace("fft-product-pack-c", started);
    dif::<DEPTH>(&mut values, &plan.inverse_roots, 0);
    let started = crate::trace("fft-inverse-product-c", started);
    // SAFETY: the reserve above covers n fields. C is no longer read, and
    // the output pass initializes every field before the length is extended.
    let output = unsafe {
        std::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<std::mem::MaybeUninit<Fr>>(), plan.n)
    };
    output
        .par_chunks_mut(4 * reverse_weights.lows.len())
        .zip(values.par_chunks(reverse_weights.lows.len()))
        .zip(&reverse_weights.chunks)
        .for_each(|((out, values), chunk)| {
            for ((out, value), low) in out
                .chunks_exact_mut(4)
                .zip(values)
                .zip(&reverse_weights.lows)
            {
                let (a, c) = Quad::unpair(value);
                let fields = a
                    .mul(low)
                    .mul(chunk)
                    .sub(&c.mul(&plan.reverse_weights.chunks[0]))
                    .get();
                for (out, value) in out.iter_mut().zip(fields) {
                    out.write(value);
                }
            }
        });
    // SAFETY: all n output fields were initialized by the completed pass.
    unsafe { out.set_len(plan.n) };
    out.pop();
    crate::trace("fft-output", started);
    out
}

// lo, hi = lo + hi, (lo - hi) * twiddle, quad by quad.
#[inline(always)]
fn dif_quads(lo: &mut [Pair], hi: &mut [Pair], twiddles: &[Quad]) {
    for j in 0..lo.len() {
        let difference = lo[j].sub(&hi[j]);
        lo[j] = lo[j].add(&hi[j]);
        hi[j] = difference.mul(&broadcast(&twiddles[j]));
    }
}

// lo, hi = lo + hi * twiddle, lo - hi * twiddle, quad by quad.
#[inline(always)]
fn dit_quads(lo: &mut [Pair], hi: &mut [Pair], twiddles: &[Quad]) {
    for j in 0..lo.len() {
        let product = hi[j].mul(&broadcast(&twiddles[j]));
        hi[j] = lo[j].sub(&product);
        lo[j] = lo[j].add(&product);
    }
}

// The layers two and one apart act within a quad: on two quads a and b at
// a time, the butterflies' two sides are gathered into one quad each,
// combined, and scattered back.
#[inline(always)]
fn dif_within(a: &mut Pair, b: &mut Pair, pair: &Pair) {
    // Two apart: (a0 a1 b0 b1) with (a2 a3 b2 b3), twiddles (1, t, 1, t).
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let sum = lo.add(&hi);
    let difference = lo.sub(&hi).mul(pair);
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
    // One apart: (a0 a2 b0 b2) with (a1 a3 b1 b3), twiddle one.
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
}
#[inline(always)]
fn dit_within(a: &mut Pair, b: &mut Pair, pair: &Pair, single: Option<&Pair>) {
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
    let hi = single.map_or(hi, |root| hi.mul(root));
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 1, 4, 5>(a, b),
        Pair::shuffle_pair::<2, 3, 6, 7>(a, b),
    );
    let product = hi.mul(pair);
    let (sum, difference) = (lo.add(&product), lo.sub(&product));
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &difference);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &difference);
}

// `level` selects the twiddle layer of the outermost butterflies.
fn dif<const DEPTH: usize>(values: &mut [Pair], twiddles: &Twiddles, level: usize) {
    if DEPTH >= 5 && values.len() >= 32 * TASK {
        let q = values.len() / 32;
        let mut pieces = values.chunks_exact_mut(q);
        let v0 = pieces.next().unwrap();
        let v1 = pieces.next().unwrap();
        let v2 = pieces.next().unwrap();
        let v3 = pieces.next().unwrap();
        let v4 = pieces.next().unwrap();
        let v5 = pieces.next().unwrap();
        let v6 = pieces.next().unwrap();
        let v7 = pieces.next().unwrap();
        let v8 = pieces.next().unwrap();
        let v9 = pieces.next().unwrap();
        let v10 = pieces.next().unwrap();
        let v11 = pieces.next().unwrap();
        let v12 = pieces.next().unwrap();
        let v13 = pieces.next().unwrap();
        let v14 = pieces.next().unwrap();
        let v15 = pieces.next().unwrap();
        let v16 = pieces.next().unwrap();
        let v17 = pieces.next().unwrap();
        let v18 = pieces.next().unwrap();
        let v19 = pieces.next().unwrap();
        let v20 = pieces.next().unwrap();
        let v21 = pieces.next().unwrap();
        let v22 = pieces.next().unwrap();
        let v23 = pieces.next().unwrap();
        let v24 = pieces.next().unwrap();
        let v25 = pieces.next().unwrap();
        let v26 = pieces.next().unwrap();
        let v27 = pieces.next().unwrap();
        let v28 = pieces.next().unwrap();
        let v29 = pieces.next().unwrap();
        let v30 = pieces.next().unwrap();
        let v31 = pieces.next().unwrap();
        fused_32::<false>(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18,
            v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, twiddles, level,
        );
        [
            &mut *v0, &mut *v1, &mut *v2, &mut *v3, &mut *v4, &mut *v5, &mut *v6, &mut *v7,
            &mut *v8, &mut *v9, &mut *v10, &mut *v11, &mut *v12, &mut *v13, &mut *v14, &mut *v15,
            &mut *v16, &mut *v17, &mut *v18, &mut *v19, &mut *v20, &mut *v21, &mut *v22, &mut *v23,
            &mut *v24, &mut *v25, &mut *v26, &mut *v27, &mut *v28, &mut *v29, &mut *v30, &mut *v31,
        ]
        .into_par_iter()
        .for_each(|q| dif::<DEPTH>(q, twiddles, level + 5));
        return;
    }
    if DEPTH >= 4 && values.len() >= 16 * TASK {
        let q = values.len() / 16;
        let mut pieces = values.chunks_exact_mut(q);
        let v0 = pieces.next().unwrap();
        let v1 = pieces.next().unwrap();
        let v2 = pieces.next().unwrap();
        let v3 = pieces.next().unwrap();
        let v4 = pieces.next().unwrap();
        let v5 = pieces.next().unwrap();
        let v6 = pieces.next().unwrap();
        let v7 = pieces.next().unwrap();
        let v8 = pieces.next().unwrap();
        let v9 = pieces.next().unwrap();
        let v10 = pieces.next().unwrap();
        let v11 = pieces.next().unwrap();
        let v12 = pieces.next().unwrap();
        let v13 = pieces.next().unwrap();
        let v14 = pieces.next().unwrap();
        let v15 = pieces.next().unwrap();
        fused_sixteen::<false>(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, twiddles, level,
        );
        [
            &mut *v0, &mut *v1, &mut *v2, &mut *v3, &mut *v4, &mut *v5, &mut *v6, &mut *v7,
            &mut *v8, &mut *v9, &mut *v10, &mut *v11, &mut *v12, &mut *v13, &mut *v14, &mut *v15,
        ]
        .into_par_iter()
        .for_each(|q| dif::<DEPTH>(q, twiddles, level + 4));
        return;
    }
    if DEPTH >= 3 && values.len() >= 8 * TASK {
        let q = values.len() / 8;
        let mut quarters = values.chunks_exact_mut(q);
        let a = quarters.next().unwrap();
        let b = quarters.next().unwrap();
        let c = quarters.next().unwrap();
        let d = quarters.next().unwrap();
        let e = quarters.next().unwrap();
        let f = quarters.next().unwrap();
        let g = quarters.next().unwrap();
        let h = quarters.next().unwrap();
        fused_eight::<false>(a, b, c, d, e, f, g, h, twiddles, level);
        [
            &mut *a, &mut *b, &mut *c, &mut *d, &mut *e, &mut *f, &mut *g, &mut *h,
        ]
        .into_par_iter()
        .for_each(|q| dif::<DEPTH>(q, twiddles, level + 3));
        return;
    }
    if DEPTH >= 2 && values.len() >= 4 * TASK {
        let quarter = values.len() / 4;
        let (ab, cd) = values.split_at_mut(2 * quarter);
        let (a, b) = ab.split_at_mut(quarter);
        let (c, d) = cd.split_at_mut(quarter);
        fused_layer::<false>(a, b, c, d, twiddles, level);
        rayon::join(
            || {
                rayon::join(
                    || dif::<DEPTH>(a, twiddles, level + 2),
                    || dif::<DEPTH>(b, twiddles, level + 2),
                );
            },
            || {
                rayon::join(
                    || dif::<DEPTH>(c, twiddles, level + 2),
                    || dif::<DEPTH>(d, twiddles, level + 2),
                );
            },
        );
        return;
    }
    if values.len() > TASK {
        let mid = values.len() / 2;
        let (lo, hi) = values.split_at_mut(mid);
        lo.par_chunks_mut(TASK)
            .zip(hi.par_chunks_mut(TASK))
            .zip(twiddles.layers[level].par_chunks(TASK))
            .for_each(|((lo, hi), twiddles)| dif_quads(lo, hi, twiddles));
        rayon::join(
            || dif::<DEPTH>(lo, twiddles, level + 1),
            || dif::<DEPTH>(hi, twiddles, level + 1),
        );
    } else {
        let mut gap = values.len() / 2;
        let mut level = level;
        while gap >= 1 {
            let layer = &twiddles.layers[level][..gap];
            for chunk in values.chunks_exact_mut(gap * 2) {
                let (lo, hi) = chunk.split_at_mut(gap);
                dif_quads(lo, hi, layer);
            }
            gap /= 2;
            level += 1;
        }
        for pair in values.as_chunks_mut::<2>().0 {
            let (a, b) = pair.split_at_mut(1);
            dif_within(&mut a[0], &mut b[0], &broadcast(&twiddles.pair));
        }
    }
}
fn dit<const DEPTH: usize>(values: &mut [Pair], twiddles: &Twiddles, level: usize) {
    if DEPTH >= 5 && values.len() >= 32 * TASK {
        let q = values.len() / 32;
        let mut pieces = values.chunks_exact_mut(q);
        let v0 = pieces.next().unwrap();
        let v1 = pieces.next().unwrap();
        let v2 = pieces.next().unwrap();
        let v3 = pieces.next().unwrap();
        let v4 = pieces.next().unwrap();
        let v5 = pieces.next().unwrap();
        let v6 = pieces.next().unwrap();
        let v7 = pieces.next().unwrap();
        let v8 = pieces.next().unwrap();
        let v9 = pieces.next().unwrap();
        let v10 = pieces.next().unwrap();
        let v11 = pieces.next().unwrap();
        let v12 = pieces.next().unwrap();
        let v13 = pieces.next().unwrap();
        let v14 = pieces.next().unwrap();
        let v15 = pieces.next().unwrap();
        let v16 = pieces.next().unwrap();
        let v17 = pieces.next().unwrap();
        let v18 = pieces.next().unwrap();
        let v19 = pieces.next().unwrap();
        let v20 = pieces.next().unwrap();
        let v21 = pieces.next().unwrap();
        let v22 = pieces.next().unwrap();
        let v23 = pieces.next().unwrap();
        let v24 = pieces.next().unwrap();
        let v25 = pieces.next().unwrap();
        let v26 = pieces.next().unwrap();
        let v27 = pieces.next().unwrap();
        let v28 = pieces.next().unwrap();
        let v29 = pieces.next().unwrap();
        let v30 = pieces.next().unwrap();
        let v31 = pieces.next().unwrap();
        [
            &mut *v0, &mut *v1, &mut *v2, &mut *v3, &mut *v4, &mut *v5, &mut *v6, &mut *v7,
            &mut *v8, &mut *v9, &mut *v10, &mut *v11, &mut *v12, &mut *v13, &mut *v14, &mut *v15,
            &mut *v16, &mut *v17, &mut *v18, &mut *v19, &mut *v20, &mut *v21, &mut *v22, &mut *v23,
            &mut *v24, &mut *v25, &mut *v26, &mut *v27, &mut *v28, &mut *v29, &mut *v30, &mut *v31,
        ]
        .into_par_iter()
        .for_each(|q| dit::<DEPTH>(q, twiddles, level + 5));
        fused_32::<true>(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18,
            v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, twiddles, level,
        );
        return;
    }
    if DEPTH >= 4 && values.len() >= 16 * TASK {
        let q = values.len() / 16;
        let mut pieces = values.chunks_exact_mut(q);
        let v0 = pieces.next().unwrap();
        let v1 = pieces.next().unwrap();
        let v2 = pieces.next().unwrap();
        let v3 = pieces.next().unwrap();
        let v4 = pieces.next().unwrap();
        let v5 = pieces.next().unwrap();
        let v6 = pieces.next().unwrap();
        let v7 = pieces.next().unwrap();
        let v8 = pieces.next().unwrap();
        let v9 = pieces.next().unwrap();
        let v10 = pieces.next().unwrap();
        let v11 = pieces.next().unwrap();
        let v12 = pieces.next().unwrap();
        let v13 = pieces.next().unwrap();
        let v14 = pieces.next().unwrap();
        let v15 = pieces.next().unwrap();
        [
            &mut *v0, &mut *v1, &mut *v2, &mut *v3, &mut *v4, &mut *v5, &mut *v6, &mut *v7,
            &mut *v8, &mut *v9, &mut *v10, &mut *v11, &mut *v12, &mut *v13, &mut *v14, &mut *v15,
        ]
        .into_par_iter()
        .for_each(|q| dit::<DEPTH>(q, twiddles, level + 4));
        fused_sixteen::<true>(
            v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, twiddles, level,
        );
        return;
    }
    if DEPTH >= 3 && values.len() >= 8 * TASK {
        let q = values.len() / 8;
        let mut quarters = values.chunks_exact_mut(q);
        let a = quarters.next().unwrap();
        let b = quarters.next().unwrap();
        let c = quarters.next().unwrap();
        let d = quarters.next().unwrap();
        let e = quarters.next().unwrap();
        let f = quarters.next().unwrap();
        let g = quarters.next().unwrap();
        let h = quarters.next().unwrap();
        [
            &mut *a, &mut *b, &mut *c, &mut *d, &mut *e, &mut *f, &mut *g, &mut *h,
        ]
        .into_par_iter()
        .for_each(|q| dit::<DEPTH>(q, twiddles, level + 3));
        fused_eight::<true>(a, b, c, d, e, f, g, h, twiddles, level);
        return;
    }
    if DEPTH >= 2 && values.len() >= 4 * TASK {
        let quarter = values.len() / 4;
        let (ab, cd) = values.split_at_mut(2 * quarter);
        let (a, b) = ab.split_at_mut(quarter);
        let (c, d) = cd.split_at_mut(quarter);
        rayon::join(
            || {
                rayon::join(
                    || dit::<DEPTH>(a, twiddles, level + 2),
                    || dit::<DEPTH>(b, twiddles, level + 2),
                );
            },
            || {
                rayon::join(
                    || dit::<DEPTH>(c, twiddles, level + 2),
                    || dit::<DEPTH>(d, twiddles, level + 2),
                );
            },
        );
        fused_layer::<true>(a, b, c, d, twiddles, level);
        return;
    }
    if values.len() > TASK {
        let mid = values.len() / 2;
        let (lo, hi) = values.split_at_mut(mid);
        rayon::join(
            || dit::<DEPTH>(lo, twiddles, level + 1),
            || dit::<DEPTH>(hi, twiddles, level + 1),
        );
        lo.par_chunks_mut(TASK)
            .zip(hi.par_chunks_mut(TASK))
            .zip(twiddles.layers[level].par_chunks(TASK))
            .for_each(|((lo, hi), twiddles)| dit_quads(lo, hi, twiddles));
    } else {
        for pair in values.as_chunks_mut::<2>().0 {
            let (a, b) = pair.split_at_mut(1);
            dit_within(
                &mut a[0],
                &mut b[0],
                &broadcast(&twiddles.pair),
                twiddles.single.as_ref().map(broadcast).as_ref(),
            );
        }
        // Butterflies `gap` quads apart use the layer of stride n / (8 gap).
        let mut gap = 1;
        while gap < values.len() {
            let layer =
                &twiddles.layers[twiddles.layers.len() - 1 - gap.trailing_zeros() as usize][..gap];
            for chunk in values.chunks_exact_mut(gap * 2) {
                let (lo, hi) = chunk.split_at_mut(gap);
                dit_quads(lo, hi, layer);
            }
            gap *= 2;
        }
    }
}

// Four small disjoint tiles fit in L1 while two stages run, avoiding a
// complete write/read round trip for each stage on large transforms.
fn fused_layer<const FORWARD: bool>(
    a: &mut [Pair],
    b: &mut [Pair],
    c: &mut [Pair],
    d: &mut [Pair],
    twiddles: &Twiddles,
    level: usize,
) {
    const TILE: usize = 8;
    let q = a.len();
    a.par_chunks_mut(TILE)
        .zip(b.par_chunks_mut(TILE))
        .zip(c.par_chunks_mut(TILE))
        .zip(d.par_chunks_mut(TILE))
        .enumerate()
        .for_each(|(i, (((a, b), c), d))| {
            let i = i * TILE;
            let r0 = &twiddles.layers[level][i..i + a.len()];
            let r1 = &twiddles.layers[level][q + i..q + i + a.len()];
            let r2 = &twiddles.layers[level + 1][i..i + a.len()];
            if FORWARD {
                dit_quads(a, b, r2);
                dit_quads(c, d, r2);
                dit_quads(a, c, r0);
                dit_quads(b, d, r1);
            } else {
                dif_quads(a, c, r0);
                dif_quads(b, d, r1);
                dif_quads(a, b, r2);
                dif_quads(c, d, r2);
            }
        });
}

fn fused_eight<const FORWARD: bool>(
    a: &mut [Pair],
    b: &mut [Pair],
    c: &mut [Pair],
    d: &mut [Pair],
    e: &mut [Pair],
    f: &mut [Pair],
    g: &mut [Pair],
    h: &mut [Pair],
    twiddles: &Twiddles,
    level: usize,
) {
    const TILE: usize = 4;
    let q = a.len();
    a.par_chunks_mut(TILE)
        .zip(b.par_chunks_mut(TILE))
        .zip(c.par_chunks_mut(TILE))
        .zip(d.par_chunks_mut(TILE))
        .zip(e.par_chunks_mut(TILE))
        .zip(f.par_chunks_mut(TILE))
        .zip(g.par_chunks_mut(TILE))
        .zip(h.par_chunks_mut(TILE))
        .enumerate()
        .for_each(|(column, (((((((a, b), c), d), e), f), g), h))| {
            let j = column * TILE;
            let len = a.len();
            let r0_0 = &twiddles.layers[level + 0][0 * q + j..0 * q + j + len];
            let r0_1 = &twiddles.layers[level + 0][1 * q + j..1 * q + j + len];
            let r0_2 = &twiddles.layers[level + 0][2 * q + j..2 * q + j + len];
            let r0_3 = &twiddles.layers[level + 0][3 * q + j..3 * q + j + len];
            let r1_0 = &twiddles.layers[level + 1][0 * q + j..0 * q + j + len];
            let r1_1 = &twiddles.layers[level + 1][1 * q + j..1 * q + j + len];
            let r2_0 = &twiddles.layers[level + 2][0 * q + j..0 * q + j + len];
            if FORWARD {
                dit_quads(a, b, r2_0);
                dit_quads(c, d, r2_0);
                dit_quads(e, f, r2_0);
                dit_quads(g, h, r2_0);
                dit_quads(a, c, r1_0);
                dit_quads(b, d, r1_1);
                dit_quads(e, g, r1_0);
                dit_quads(f, h, r1_1);
                dit_quads(a, e, r0_0);
                dit_quads(b, f, r0_1);
                dit_quads(c, g, r0_2);
                dit_quads(d, h, r0_3);
            } else {
                dif_quads(a, e, r0_0);
                dif_quads(b, f, r0_1);
                dif_quads(c, g, r0_2);
                dif_quads(d, h, r0_3);
                dif_quads(a, c, r1_0);
                dif_quads(b, d, r1_1);
                dif_quads(e, g, r1_0);
                dif_quads(f, h, r1_1);
                dif_quads(a, b, r2_0);
                dif_quads(c, d, r2_0);
                dif_quads(e, f, r2_0);
                dif_quads(g, h, r2_0);
            }
        });
}

fn fused_sixteen<const FORWARD: bool>(
    v0: &mut [Pair],
    v1: &mut [Pair],
    v2: &mut [Pair],
    v3: &mut [Pair],
    v4: &mut [Pair],
    v5: &mut [Pair],
    v6: &mut [Pair],
    v7: &mut [Pair],
    v8: &mut [Pair],
    v9: &mut [Pair],
    v10: &mut [Pair],
    v11: &mut [Pair],
    v12: &mut [Pair],
    v13: &mut [Pair],
    v14: &mut [Pair],
    v15: &mut [Pair],
    twiddles: &Twiddles,
    level: usize,
) {
    const TILE: usize = 2;
    let q = v0.len();
    v0.par_chunks_mut(TILE)
        .zip(v1.par_chunks_mut(TILE))
        .zip(v2.par_chunks_mut(TILE))
        .zip(v3.par_chunks_mut(TILE))
        .zip(v4.par_chunks_mut(TILE))
        .zip(v5.par_chunks_mut(TILE))
        .zip(v6.par_chunks_mut(TILE))
        .zip(v7.par_chunks_mut(TILE))
        .zip(v8.par_chunks_mut(TILE))
        .zip(v9.par_chunks_mut(TILE))
        .zip(v10.par_chunks_mut(TILE))
        .zip(v11.par_chunks_mut(TILE))
        .zip(v12.par_chunks_mut(TILE))
        .zip(v13.par_chunks_mut(TILE))
        .zip(v14.par_chunks_mut(TILE))
        .zip(v15.par_chunks_mut(TILE))
        .enumerate()
        .for_each(
            |(
                column,
                (
                    (
                        (
                            (
                                (
                                    ((((((((((v0, v1), v2), v3), v4), v5), v6), v7), v8), v9), v10),
                                    v11,
                                ),
                                v12,
                            ),
                            v13,
                        ),
                        v14,
                    ),
                    v15,
                ),
            )| {
                let start = column * TILE;
                let len = v0.len();
                let r0_0 = &twiddles.layers[level + 0][0 * q + start..0 * q + start + len];
                let r0_1 = &twiddles.layers[level + 0][1 * q + start..1 * q + start + len];
                let r0_2 = &twiddles.layers[level + 0][2 * q + start..2 * q + start + len];
                let r0_3 = &twiddles.layers[level + 0][3 * q + start..3 * q + start + len];
                let r0_4 = &twiddles.layers[level + 0][4 * q + start..4 * q + start + len];
                let r0_5 = &twiddles.layers[level + 0][5 * q + start..5 * q + start + len];
                let r0_6 = &twiddles.layers[level + 0][6 * q + start..6 * q + start + len];
                let r0_7 = &twiddles.layers[level + 0][7 * q + start..7 * q + start + len];
                let r1_0 = &twiddles.layers[level + 1][0 * q + start..0 * q + start + len];
                let r1_1 = &twiddles.layers[level + 1][1 * q + start..1 * q + start + len];
                let r1_2 = &twiddles.layers[level + 1][2 * q + start..2 * q + start + len];
                let r1_3 = &twiddles.layers[level + 1][3 * q + start..3 * q + start + len];
                let r2_0 = &twiddles.layers[level + 2][0 * q + start..0 * q + start + len];
                let r2_1 = &twiddles.layers[level + 2][1 * q + start..1 * q + start + len];
                let r3_0 = &twiddles.layers[level + 3][0 * q + start..0 * q + start + len];
                if FORWARD {
                    dit_quads(v0, v1, r3_0);
                    dit_quads(v2, v3, r3_0);
                    dit_quads(v4, v5, r3_0);
                    dit_quads(v6, v7, r3_0);
                    dit_quads(v8, v9, r3_0);
                    dit_quads(v10, v11, r3_0);
                    dit_quads(v12, v13, r3_0);
                    dit_quads(v14, v15, r3_0);
                    dit_quads(v0, v2, r2_0);
                    dit_quads(v1, v3, r2_1);
                    dit_quads(v4, v6, r2_0);
                    dit_quads(v5, v7, r2_1);
                    dit_quads(v8, v10, r2_0);
                    dit_quads(v9, v11, r2_1);
                    dit_quads(v12, v14, r2_0);
                    dit_quads(v13, v15, r2_1);
                    dit_quads(v0, v4, r1_0);
                    dit_quads(v1, v5, r1_1);
                    dit_quads(v2, v6, r1_2);
                    dit_quads(v3, v7, r1_3);
                    dit_quads(v8, v12, r1_0);
                    dit_quads(v9, v13, r1_1);
                    dit_quads(v10, v14, r1_2);
                    dit_quads(v11, v15, r1_3);
                    dit_quads(v0, v8, r0_0);
                    dit_quads(v1, v9, r0_1);
                    dit_quads(v2, v10, r0_2);
                    dit_quads(v3, v11, r0_3);
                    dit_quads(v4, v12, r0_4);
                    dit_quads(v5, v13, r0_5);
                    dit_quads(v6, v14, r0_6);
                    dit_quads(v7, v15, r0_7);
                } else {
                    dif_quads(v0, v8, r0_0);
                    dif_quads(v1, v9, r0_1);
                    dif_quads(v2, v10, r0_2);
                    dif_quads(v3, v11, r0_3);
                    dif_quads(v4, v12, r0_4);
                    dif_quads(v5, v13, r0_5);
                    dif_quads(v6, v14, r0_6);
                    dif_quads(v7, v15, r0_7);
                    dif_quads(v0, v4, r1_0);
                    dif_quads(v1, v5, r1_1);
                    dif_quads(v2, v6, r1_2);
                    dif_quads(v3, v7, r1_3);
                    dif_quads(v8, v12, r1_0);
                    dif_quads(v9, v13, r1_1);
                    dif_quads(v10, v14, r1_2);
                    dif_quads(v11, v15, r1_3);
                    dif_quads(v0, v2, r2_0);
                    dif_quads(v1, v3, r2_1);
                    dif_quads(v4, v6, r2_0);
                    dif_quads(v5, v7, r2_1);
                    dif_quads(v8, v10, r2_0);
                    dif_quads(v9, v11, r2_1);
                    dif_quads(v12, v14, r2_0);
                    dif_quads(v13, v15, r2_1);
                    dif_quads(v0, v1, r3_0);
                    dif_quads(v2, v3, r3_0);
                    dif_quads(v4, v5, r3_0);
                    dif_quads(v6, v7, r3_0);
                    dif_quads(v8, v9, r3_0);
                    dif_quads(v10, v11, r3_0);
                    dif_quads(v12, v13, r3_0);
                    dif_quads(v14, v15, r3_0);
                }
            },
        );
}

pub(super) fn coset_product(plan: &Plan, a: Vec<Fr>, b: Vec<Fr>) -> Vec<Fr> {
    let depth = product_depth(plan);
    // Keep small domains and explicit alternate fusion depths on their
    // existing path. Native five-stage products use compact raw storage.
    if depth == 5 && plan.n >= (1 << 15) && crate::simd::cpu_option("ZOPRO_CPU_FFT_PACKED") {
        return stored_product::<PackedPair>(plan, a, b);
    }
    match depth {
        5 => product_impl::<5>(plan, a, b),
        4 => product_impl::<4>(plan, a, b),
        3 => product_impl::<3>(plan, a, b),
        2 => product_impl::<2>(plan, a, b),
        _ => product_impl::<1>(plan, a, b),
    }
}

fn product_impl<const DEPTH: usize>(plan: &Plan, a: Vec<Fr>, b: Vec<Fr>) -> Vec<Fr> {
    let started = crate::time::Instant::now();
    assert!(a.len() <= plan.n && b.len() <= plan.n);
    let pack = |i: usize| {
        let mut lanes = [Fr::ZERO; 8];
        for j in 0..4 {
            lanes[j] = a.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
            lanes[4 + j] = b.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
        }
        Pair::new(&lanes)
    };
    let mut values: Vec<Pair> = (0..plan.n / 4).into_par_iter().map(pack).collect();
    let mut out = a;
    out.reserve(plan.n - out.len());
    drop(b);
    let started = crate::trace("fft-pack", started);
    dif::<DEPTH>(&mut values, &plan.inverse_roots, 0);
    let started = crate::trace("fft-inverse-ab", started);
    if plan.twisted.is_none() {
        values
            .par_chunks_mut(plan.forward_weights.lows.len())
            .zip(&plan.forward_weights.chunks)
            .for_each(|(values, chunk)| {
                let chunk = broadcast(chunk);
                for (value, low) in values.iter_mut().zip(&plan.forward_weights.lows) {
                    *value = value.mul(&broadcast(low)).mul(&chunk);
                }
            });
    }
    let started = crate::trace("fft-weights", started);
    let (roots, _reverse_weights) = plan
        .twisted
        .as_ref()
        .map(|(roots, weights)| (roots, weights))
        .unwrap_or((&plan.roots, &plan.reverse_weights));
    dit::<DEPTH>(&mut values, roots, 0);
    let started = crate::trace("fft-forward-ab", started);
    // SAFETY: output has capacity n, no input uses A any longer, and the
    // complete parallel pass below initializes all n fields before set_len.
    let output = unsafe {
        std::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<std::mem::MaybeUninit<Fr>>(), plan.n)
    };
    output
        .par_chunks_mut(4)
        .zip(values.par_iter())
        .for_each(|(out, value)| {
            let (a, b) = Quad::unpair(value);
            for (out, value) in out.iter_mut().zip(a.mul(&b).get()) {
                out.write(value);
            }
        });
    unsafe { out.set_len(plan.n) };
    crate::trace("fft-product-output", started);
    out
}

// Keep 5 butterfly layers in a column tile before the next outer pass.
fn fused_32<const FORWARD: bool>(
    v0: &mut [Pair],
    v1: &mut [Pair],
    v2: &mut [Pair],
    v3: &mut [Pair],
    v4: &mut [Pair],
    v5: &mut [Pair],
    v6: &mut [Pair],
    v7: &mut [Pair],
    v8: &mut [Pair],
    v9: &mut [Pair],
    v10: &mut [Pair],
    v11: &mut [Pair],
    v12: &mut [Pair],
    v13: &mut [Pair],
    v14: &mut [Pair],
    v15: &mut [Pair],
    v16: &mut [Pair],
    v17: &mut [Pair],
    v18: &mut [Pair],
    v19: &mut [Pair],
    v20: &mut [Pair],
    v21: &mut [Pair],
    v22: &mut [Pair],
    v23: &mut [Pair],
    v24: &mut [Pair],
    v25: &mut [Pair],
    v26: &mut [Pair],
    v27: &mut [Pair],
    v28: &mut [Pair],
    v29: &mut [Pair],
    v30: &mut [Pair],
    v31: &mut [Pair],
    twiddles: &Twiddles,
    level: usize,
) {
    const TILE: usize = 1;
    let q = v0.len();
    v0.par_chunks_mut(TILE)
        .zip(v1.par_chunks_mut(TILE))
        .zip(v2.par_chunks_mut(TILE).zip(v3.par_chunks_mut(TILE)))
        .zip(
            v4.par_chunks_mut(TILE)
                .zip(v5.par_chunks_mut(TILE))
                .zip(v6.par_chunks_mut(TILE).zip(v7.par_chunks_mut(TILE))),
        )
        .zip(
            v8.par_chunks_mut(TILE)
                .zip(v9.par_chunks_mut(TILE))
                .zip(v10.par_chunks_mut(TILE).zip(v11.par_chunks_mut(TILE)))
                .zip(
                    v12.par_chunks_mut(TILE)
                        .zip(v13.par_chunks_mut(TILE))
                        .zip(v14.par_chunks_mut(TILE).zip(v15.par_chunks_mut(TILE))),
                ),
        )
        .zip(
            v16.par_chunks_mut(TILE)
                .zip(v17.par_chunks_mut(TILE))
                .zip(v18.par_chunks_mut(TILE).zip(v19.par_chunks_mut(TILE)))
                .zip(
                    v20.par_chunks_mut(TILE)
                        .zip(v21.par_chunks_mut(TILE))
                        .zip(v22.par_chunks_mut(TILE).zip(v23.par_chunks_mut(TILE))),
                )
                .zip(
                    v24.par_chunks_mut(TILE)
                        .zip(v25.par_chunks_mut(TILE))
                        .zip(v26.par_chunks_mut(TILE).zip(v27.par_chunks_mut(TILE)))
                        .zip(
                            v28.par_chunks_mut(TILE)
                                .zip(v29.par_chunks_mut(TILE))
                                .zip(v30.par_chunks_mut(TILE).zip(v31.par_chunks_mut(TILE))),
                        ),
                ),
        )
        .enumerate()
        .for_each(
            |(
                column,
                (
                    (
                        (((v0, v1), (v2, v3)), ((v4, v5), (v6, v7))),
                        (((v8, v9), (v10, v11)), ((v12, v13), (v14, v15))),
                    ),
                    (
                        (((v16, v17), (v18, v19)), ((v20, v21), (v22, v23))),
                        (((v24, v25), (v26, v27)), ((v28, v29), (v30, v31))),
                    ),
                ),
            )| {
                let start = column * TILE;
                let len = v0.len();
                let r0_0 = &twiddles.layers[level + 0][0 * q + start..0 * q + start + len];
                let r0_1 = &twiddles.layers[level + 0][1 * q + start..1 * q + start + len];
                let r0_2 = &twiddles.layers[level + 0][2 * q + start..2 * q + start + len];
                let r0_3 = &twiddles.layers[level + 0][3 * q + start..3 * q + start + len];
                let r0_4 = &twiddles.layers[level + 0][4 * q + start..4 * q + start + len];
                let r0_5 = &twiddles.layers[level + 0][5 * q + start..5 * q + start + len];
                let r0_6 = &twiddles.layers[level + 0][6 * q + start..6 * q + start + len];
                let r0_7 = &twiddles.layers[level + 0][7 * q + start..7 * q + start + len];
                let r0_8 = &twiddles.layers[level + 0][8 * q + start..8 * q + start + len];
                let r0_9 = &twiddles.layers[level + 0][9 * q + start..9 * q + start + len];
                let r0_10 = &twiddles.layers[level + 0][10 * q + start..10 * q + start + len];
                let r0_11 = &twiddles.layers[level + 0][11 * q + start..11 * q + start + len];
                let r0_12 = &twiddles.layers[level + 0][12 * q + start..12 * q + start + len];
                let r0_13 = &twiddles.layers[level + 0][13 * q + start..13 * q + start + len];
                let r0_14 = &twiddles.layers[level + 0][14 * q + start..14 * q + start + len];
                let r0_15 = &twiddles.layers[level + 0][15 * q + start..15 * q + start + len];
                let r1_0 = &twiddles.layers[level + 1][0 * q + start..0 * q + start + len];
                let r1_1 = &twiddles.layers[level + 1][1 * q + start..1 * q + start + len];
                let r1_2 = &twiddles.layers[level + 1][2 * q + start..2 * q + start + len];
                let r1_3 = &twiddles.layers[level + 1][3 * q + start..3 * q + start + len];
                let r1_4 = &twiddles.layers[level + 1][4 * q + start..4 * q + start + len];
                let r1_5 = &twiddles.layers[level + 1][5 * q + start..5 * q + start + len];
                let r1_6 = &twiddles.layers[level + 1][6 * q + start..6 * q + start + len];
                let r1_7 = &twiddles.layers[level + 1][7 * q + start..7 * q + start + len];
                let r2_0 = &twiddles.layers[level + 2][0 * q + start..0 * q + start + len];
                let r2_1 = &twiddles.layers[level + 2][1 * q + start..1 * q + start + len];
                let r2_2 = &twiddles.layers[level + 2][2 * q + start..2 * q + start + len];
                let r2_3 = &twiddles.layers[level + 2][3 * q + start..3 * q + start + len];
                let r3_0 = &twiddles.layers[level + 3][0 * q + start..0 * q + start + len];
                let r3_1 = &twiddles.layers[level + 3][1 * q + start..1 * q + start + len];
                let r4_0 = &twiddles.layers[level + 4][0 * q + start..0 * q + start + len];
                if FORWARD {
                    dit_quads(v0, v1, r4_0);
                    dit_quads(v2, v3, r4_0);
                    dit_quads(v4, v5, r4_0);
                    dit_quads(v6, v7, r4_0);
                    dit_quads(v8, v9, r4_0);
                    dit_quads(v10, v11, r4_0);
                    dit_quads(v12, v13, r4_0);
                    dit_quads(v14, v15, r4_0);
                    dit_quads(v16, v17, r4_0);
                    dit_quads(v18, v19, r4_0);
                    dit_quads(v20, v21, r4_0);
                    dit_quads(v22, v23, r4_0);
                    dit_quads(v24, v25, r4_0);
                    dit_quads(v26, v27, r4_0);
                    dit_quads(v28, v29, r4_0);
                    dit_quads(v30, v31, r4_0);
                    dit_quads(v0, v2, r3_0);
                    dit_quads(v1, v3, r3_1);
                    dit_quads(v4, v6, r3_0);
                    dit_quads(v5, v7, r3_1);
                    dit_quads(v8, v10, r3_0);
                    dit_quads(v9, v11, r3_1);
                    dit_quads(v12, v14, r3_0);
                    dit_quads(v13, v15, r3_1);
                    dit_quads(v16, v18, r3_0);
                    dit_quads(v17, v19, r3_1);
                    dit_quads(v20, v22, r3_0);
                    dit_quads(v21, v23, r3_1);
                    dit_quads(v24, v26, r3_0);
                    dit_quads(v25, v27, r3_1);
                    dit_quads(v28, v30, r3_0);
                    dit_quads(v29, v31, r3_1);
                    dit_quads(v0, v4, r2_0);
                    dit_quads(v1, v5, r2_1);
                    dit_quads(v2, v6, r2_2);
                    dit_quads(v3, v7, r2_3);
                    dit_quads(v8, v12, r2_0);
                    dit_quads(v9, v13, r2_1);
                    dit_quads(v10, v14, r2_2);
                    dit_quads(v11, v15, r2_3);
                    dit_quads(v16, v20, r2_0);
                    dit_quads(v17, v21, r2_1);
                    dit_quads(v18, v22, r2_2);
                    dit_quads(v19, v23, r2_3);
                    dit_quads(v24, v28, r2_0);
                    dit_quads(v25, v29, r2_1);
                    dit_quads(v26, v30, r2_2);
                    dit_quads(v27, v31, r2_3);
                    dit_quads(v0, v8, r1_0);
                    dit_quads(v1, v9, r1_1);
                    dit_quads(v2, v10, r1_2);
                    dit_quads(v3, v11, r1_3);
                    dit_quads(v4, v12, r1_4);
                    dit_quads(v5, v13, r1_5);
                    dit_quads(v6, v14, r1_6);
                    dit_quads(v7, v15, r1_7);
                    dit_quads(v16, v24, r1_0);
                    dit_quads(v17, v25, r1_1);
                    dit_quads(v18, v26, r1_2);
                    dit_quads(v19, v27, r1_3);
                    dit_quads(v20, v28, r1_4);
                    dit_quads(v21, v29, r1_5);
                    dit_quads(v22, v30, r1_6);
                    dit_quads(v23, v31, r1_7);
                    dit_quads(v0, v16, r0_0);
                    dit_quads(v1, v17, r0_1);
                    dit_quads(v2, v18, r0_2);
                    dit_quads(v3, v19, r0_3);
                    dit_quads(v4, v20, r0_4);
                    dit_quads(v5, v21, r0_5);
                    dit_quads(v6, v22, r0_6);
                    dit_quads(v7, v23, r0_7);
                    dit_quads(v8, v24, r0_8);
                    dit_quads(v9, v25, r0_9);
                    dit_quads(v10, v26, r0_10);
                    dit_quads(v11, v27, r0_11);
                    dit_quads(v12, v28, r0_12);
                    dit_quads(v13, v29, r0_13);
                    dit_quads(v14, v30, r0_14);
                    dit_quads(v15, v31, r0_15);
                } else {
                    dif_quads(v0, v16, r0_0);
                    dif_quads(v1, v17, r0_1);
                    dif_quads(v2, v18, r0_2);
                    dif_quads(v3, v19, r0_3);
                    dif_quads(v4, v20, r0_4);
                    dif_quads(v5, v21, r0_5);
                    dif_quads(v6, v22, r0_6);
                    dif_quads(v7, v23, r0_7);
                    dif_quads(v8, v24, r0_8);
                    dif_quads(v9, v25, r0_9);
                    dif_quads(v10, v26, r0_10);
                    dif_quads(v11, v27, r0_11);
                    dif_quads(v12, v28, r0_12);
                    dif_quads(v13, v29, r0_13);
                    dif_quads(v14, v30, r0_14);
                    dif_quads(v15, v31, r0_15);
                    dif_quads(v0, v8, r1_0);
                    dif_quads(v1, v9, r1_1);
                    dif_quads(v2, v10, r1_2);
                    dif_quads(v3, v11, r1_3);
                    dif_quads(v4, v12, r1_4);
                    dif_quads(v5, v13, r1_5);
                    dif_quads(v6, v14, r1_6);
                    dif_quads(v7, v15, r1_7);
                    dif_quads(v16, v24, r1_0);
                    dif_quads(v17, v25, r1_1);
                    dif_quads(v18, v26, r1_2);
                    dif_quads(v19, v27, r1_3);
                    dif_quads(v20, v28, r1_4);
                    dif_quads(v21, v29, r1_5);
                    dif_quads(v22, v30, r1_6);
                    dif_quads(v23, v31, r1_7);
                    dif_quads(v0, v4, r2_0);
                    dif_quads(v1, v5, r2_1);
                    dif_quads(v2, v6, r2_2);
                    dif_quads(v3, v7, r2_3);
                    dif_quads(v8, v12, r2_0);
                    dif_quads(v9, v13, r2_1);
                    dif_quads(v10, v14, r2_2);
                    dif_quads(v11, v15, r2_3);
                    dif_quads(v16, v20, r2_0);
                    dif_quads(v17, v21, r2_1);
                    dif_quads(v18, v22, r2_2);
                    dif_quads(v19, v23, r2_3);
                    dif_quads(v24, v28, r2_0);
                    dif_quads(v25, v29, r2_1);
                    dif_quads(v26, v30, r2_2);
                    dif_quads(v27, v31, r2_3);
                    dif_quads(v0, v2, r3_0);
                    dif_quads(v1, v3, r3_1);
                    dif_quads(v4, v6, r3_0);
                    dif_quads(v5, v7, r3_1);
                    dif_quads(v8, v10, r3_0);
                    dif_quads(v9, v11, r3_1);
                    dif_quads(v12, v14, r3_0);
                    dif_quads(v13, v15, r3_1);
                    dif_quads(v16, v18, r3_0);
                    dif_quads(v17, v19, r3_1);
                    dif_quads(v20, v22, r3_0);
                    dif_quads(v21, v23, r3_1);
                    dif_quads(v24, v26, r3_0);
                    dif_quads(v25, v27, r3_1);
                    dif_quads(v28, v30, r3_0);
                    dif_quads(v29, v31, r3_1);
                    dif_quads(v0, v1, r4_0);
                    dif_quads(v2, v3, r4_0);
                    dif_quads(v4, v5, r4_0);
                    dif_quads(v6, v7, r4_0);
                    dif_quads(v8, v9, r4_0);
                    dif_quads(v10, v11, r4_0);
                    dif_quads(v12, v13, r4_0);
                    dif_quads(v14, v15, r4_0);
                    dif_quads(v16, v17, r4_0);
                    dif_quads(v18, v19, r4_0);
                    dif_quads(v20, v21, r4_0);
                    dif_quads(v22, v23, r4_0);
                    dif_quads(v24, v25, r4_0);
                    dif_quads(v26, v27, r4_0);
                    dif_quads(v28, v29, r4_0);
                    dif_quads(v30, v31, r4_0);
                }
            },
        );
}

#[cfg(test)]
mod wide_tests {
    use super::*;
    #[test]
    fn fused_output_order_matches_quad_reference() {
        for n in [8192usize, 65536] {
            let generator = Fr::root_of_unity(n).unwrap().pow(&[3]);
            let coset = Fr::from(7u64);
            let den = denominator(n, coset).unwrap();
            let mut plan = Plan::new(n, generator, coset, den);
            for (a_len, b_len) in [(0, 0), (1, 3), (n - 3, n / 2 + 1)] {
                let a: Vec<_> = (0..a_len)
                    .map(|i| Fr::from(i as u64 + 17).pow(&[79]))
                    .collect();
                let b: Vec<_> = (0..b_len)
                    .map(|i| Fr::from(i as u64 + 31).pow(&[113]))
                    .collect();
                let left = plan.transform(a.clone());
                let right = plan.transform(b.clone());
                let reference: Vec<_> = left
                    .iter()
                    .zip(right.iter())
                    .flat_map(|(a, b)| a.mul(b).get())
                    .collect();
                for twisted in [false, true] {
                    plan.twisted = twisted.then(|| {
                        (
                            plan.roots.twisted(coset),
                            Weights::new(
                                n,
                                coset.inverse().unwrap(),
                                Fr::from(n as u64).inverse().unwrap().pow(&[3]) * den,
                            ),
                        )
                    });
                    let factor = if twisted {
                        Fr::from(n as u64).square()
                    } else {
                        Fr::ONE
                    };
                    let mut seen = vec![false; n];
                    let expected: Vec<_> = (0..n)
                        .map(|i| {
                            let source = super::super::product_output_index(n, i);
                            assert!(!seen[source]);
                            seen[source] = true;
                            reference[source] * factor
                        })
                        .collect();
                    assert!(seen.into_iter().all(|x| x));
                    assert_eq!(
                        fused_output_product(&plan, a.clone(), b.clone()),
                        expected,
                        "n={n} twisted={twisted}"
                    );
                }
            }
        }
    }

    #[test]
    fn packed_product_matches_quad_reference() {
        for n in [8usize, 256, 16384, 65536] {
            let generator = Fr::root_of_unity(n).unwrap().pow(&[3]);
            let coset = Fr::from(7u64);
            let den = denominator(n, coset).unwrap();
            let mut plan = Plan::new(n, generator, coset, den);
            for (a_len, b_len) in [(0, 0), (1, 3), (n - 3, n / 2 + 1)] {
                let a: Vec<_> = (0..a_len)
                    .map(|i| Fr::from(i as u64 + 17).pow(&[79]))
                    .collect();
                let b: Vec<_> = (0..b_len)
                    .map(|i| Fr::from(i as u64 + 31).pow(&[113]))
                    .collect();
                let left = plan.transform(a.clone());
                let right = plan.transform(b.clone());
                let expected: Vec<_> = left
                    .iter()
                    .zip(right.iter())
                    .flat_map(|(a, b)| a.mul(b).get())
                    .collect();
                for twisted in [false, true] {
                    plan.twisted = twisted.then(|| {
                        (
                            plan.roots.twisted(coset),
                            Weights::new(
                                n,
                                coset.inverse().unwrap(),
                                Fr::from(n as u64).inverse().unwrap().pow(&[3]) * den,
                            ),
                        )
                    });
                    let factor = if twisted {
                        Fr::from(n as u64).square()
                    } else {
                        Fr::ONE
                    };
                    let expected: Vec<_> = expected.iter().map(|x| *x * factor).collect();
                    assert_eq!(
                        stored_product::<PackedPair>(&plan, a.clone(), b.clone()),
                        expected,
                        "packed n={n} twisted={twisted}"
                    );
                    assert_eq!(
                        stored_product::<Pair>(&plan, a.clone(), b.clone()),
                        expected,
                        "looped n={n} twisted={twisted}"
                    );
                }
            }
        }
    }

    #[test]
    fn wider_fusion_matches_unfused_transforms() {
        for n in [16384usize, 65536] {
            let generator = Fr::root_of_unity(n).unwrap().pow(&[3]);
            let coset = Fr::from(7u64);
            let den = denominator(n, coset).unwrap();
            let mut plan = Plan::new(n, generator, coset, den);
            let a: Vec<_> = (0..n - 3)
                .map(|i| Fr::from(i as u64 + 19).pow(&[127]))
                .collect();
            let b: Vec<_> = (0..n / 2 + 1)
                .map(|i| Fr::from(i as u64 + 31).pow(&[79]))
                .collect();
            let c: Vec<_> = (0..n / 4 - 1)
                .map(|i| Fr::from(i as u64 + 37).pow(&[53]))
                .collect();
            let expected = plan.quotient_six(a.clone(), b.clone(), c.clone());
            for twisted in [false, true] {
                plan.twisted = twisted.then(|| {
                    (
                        plan.roots.twisted(coset),
                        Weights::new(
                            n,
                            coset.inverse().unwrap(),
                            Fr::from(n as u64).inverse().unwrap().pow(&[3]) * den,
                        ),
                    )
                });
                assert_eq!(
                    quotient_impl::<5>(&plan, a.clone(), b.clone(), c.clone()),
                    expected,
                    "n={n} twisted={twisted} depth=5"
                );

                let product = product_impl::<1>(&plan, a.clone(), b.clone());
                assert_eq!(
                    product_impl::<5>(&plan, a.clone(), b.clone()),
                    product,
                    "n={n} twisted={twisted} product5"
                );
            }
        }
    }
}

// A numerical storage format, preserving the IFMA Montgomery radix.
type PackedPair = zopro_bn254::ifma8::PackedIfma8<FrConfig>;
trait PairStorage: Copy + Send + Sync {
    fn load(&self) -> Pair;
    fn store(value: Pair) -> Self;
}
#[cfg(test)]
impl PairStorage for Pair {
    #[inline(always)]
    fn load(&self) -> Pair {
        *self
    }
    #[inline(always)]
    fn store(value: Pair) -> Self {
        value
    }
}
impl PairStorage for PackedPair {
    #[inline(always)]
    fn load(&self) -> Pair {
        self.expand()
    }
    #[inline(always)]
    fn store(value: Pair) -> Self {
        value.compact()
    }
}

fn stored_product<S: PairStorage>(plan: &Plan, a: Vec<Fr>, b: Vec<Fr>) -> Vec<Fr> {
    let started = crate::time::Instant::now();
    assert!(a.len() <= plan.n && b.len() <= plan.n);
    let mut values: Vec<S> = (0..plan.n / 4)
        .into_par_iter()
        .map(|i| {
            let mut lanes = [Fr::ZERO; 8];
            for j in 0..4 {
                lanes[j] = a.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
                lanes[4 + j] = b.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
            }
            S::store(Pair::new(&lanes))
        })
        .collect();
    let mut out = a;
    out.reserve(plan.n - out.len());
    drop(b);
    let started = crate::trace("fft-pack", started);
    stored_transform::<S, false>(&mut values, &plan.inverse_roots, 0);
    let started = crate::trace("fft-inverse-ab", started);
    if plan.twisted.is_none() {
        values
            .par_chunks_mut(plan.forward_weights.lows.len())
            .zip(&plan.forward_weights.chunks)
            .for_each(|(values, chunk)| {
                let chunk = broadcast(chunk);
                for (value, low) in values.iter_mut().zip(&plan.forward_weights.lows) {
                    *value = S::store(value.load().mul(&broadcast(low)).mul(&chunk));
                }
            });
    }
    let started = crate::trace("fft-weights", started);
    let roots = plan
        .twisted
        .as_ref()
        .map(|(roots, _)| roots)
        .unwrap_or(&plan.roots);
    stored_transform::<S, true>(&mut values, roots, 0);
    let started = crate::trace("fft-forward-ab", started);
    // SAFETY: reserve covers all n fields; A is consumed, and the joined
    // output pass initializes every output before extending the length.
    let output = unsafe {
        std::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<std::mem::MaybeUninit<Fr>>(), plan.n)
    };
    output
        .par_chunks_mut(4)
        .zip(values.par_iter())
        .for_each(|(out, value)| {
            let (a, b) = Quad::unpair(&value.load());
            for (out, field) in out.iter_mut().zip(a.mul(&b).get()) {
                out.write(field);
            }
        });
    // SAFETY: every field was initialized by the completed pass above.
    unsafe { out.set_len(plan.n) };
    crate::trace("fft-product-output", started);
    out
}

fn stored_transform<S: PairStorage, const FORWARD: bool>(
    values: &mut [S],
    roots: &Twiddles,
    level: usize,
) {
    if values.len() <= 1024 {
        // Expand a cache-sized leaf once; its inner butterflies use the
        // unchanged lane and fused native kernels without extra packing.
        let mut local: Vec<Pair> = values.iter().map(S::load).collect();
        if FORWARD {
            dit::<5>(&mut local, roots, level);
        } else {
            dif::<5>(&mut local, roots, level);
        }
        for (out, value) in values.iter_mut().zip(local) {
            *out = S::store(value);
        }
        return;
    }
    let q = values.len() / 32;
    let mut pieces = values.chunks_exact_mut(q);
    let v0 = pieces.next().unwrap();
    let v1 = pieces.next().unwrap();
    let v2 = pieces.next().unwrap();
    let v3 = pieces.next().unwrap();
    let v4 = pieces.next().unwrap();
    let v5 = pieces.next().unwrap();
    let v6 = pieces.next().unwrap();
    let v7 = pieces.next().unwrap();
    let v8 = pieces.next().unwrap();
    let v9 = pieces.next().unwrap();
    let v10 = pieces.next().unwrap();
    let v11 = pieces.next().unwrap();
    let v12 = pieces.next().unwrap();
    let v13 = pieces.next().unwrap();
    let v14 = pieces.next().unwrap();
    let v15 = pieces.next().unwrap();
    let v16 = pieces.next().unwrap();
    let v17 = pieces.next().unwrap();
    let v18 = pieces.next().unwrap();
    let v19 = pieces.next().unwrap();
    let v20 = pieces.next().unwrap();
    let v21 = pieces.next().unwrap();
    let v22 = pieces.next().unwrap();
    let v23 = pieces.next().unwrap();
    let v24 = pieces.next().unwrap();
    let v25 = pieces.next().unwrap();
    let v26 = pieces.next().unwrap();
    let v27 = pieces.next().unwrap();
    let v28 = pieces.next().unwrap();
    let v29 = pieces.next().unwrap();
    let v30 = pieces.next().unwrap();
    let v31 = pieces.next().unwrap();
    if FORWARD {
        [
            &mut *v0, &mut *v1, &mut *v2, &mut *v3, &mut *v4, &mut *v5, &mut *v6, &mut *v7,
            &mut *v8, &mut *v9, &mut *v10, &mut *v11, &mut *v12, &mut *v13, &mut *v14, &mut *v15,
            &mut *v16, &mut *v17, &mut *v18, &mut *v19, &mut *v20, &mut *v21, &mut *v22, &mut *v23,
            &mut *v24, &mut *v25, &mut *v26, &mut *v27, &mut *v28, &mut *v29, &mut *v30, &mut *v31,
        ]
        .into_par_iter()
        .for_each(|part| stored_transform::<S, FORWARD>(part, roots, level + 5));
    }
    stored_columns::<S, FORWARD>(
        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19,
        v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, roots, level,
    );
    if !FORWARD {
        [
            &mut *v0, &mut *v1, &mut *v2, &mut *v3, &mut *v4, &mut *v5, &mut *v6, &mut *v7,
            &mut *v8, &mut *v9, &mut *v10, &mut *v11, &mut *v12, &mut *v13, &mut *v14, &mut *v15,
            &mut *v16, &mut *v17, &mut *v18, &mut *v19, &mut *v20, &mut *v21, &mut *v22, &mut *v23,
            &mut *v24, &mut *v25, &mut *v26, &mut *v27, &mut *v28, &mut *v29, &mut *v30, &mut *v31,
        ]
        .into_par_iter()
        .for_each(|part| stored_transform::<S, FORWARD>(part, roots, level + 5));
    }
}

// Keep the fixed column scratch out of Rayon's recursive iterator frames.
// This leaf does not fork or allocate; the mathematical kernel is unchanged.
#[inline(never)]
fn stored_column_leaf<S: PairStorage, const FORWARD: bool>(
    values: [&mut S; 32], roots: &Twiddles, level: usize, q: usize,
    column: usize, cached: bool,
) {
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31] = values;
    let mut local = [
        v0.load(),
        v1.load(),
        v2.load(),
        v3.load(),
        v4.load(),
        v5.load(),
        v6.load(),
        v7.load(),
        v8.load(),
        v9.load(),
        v10.load(),
        v11.load(),
        v12.load(),
        v13.load(),
        v14.load(),
        v15.load(),
        v16.load(),
        v17.load(),
        v18.load(),
        v19.load(),
        v20.load(),
        v21.load(),
        v22.load(),
        v23.load(),
        v24.load(),
        v25.load(),
        v26.load(),
        v27.load(),
        v28.load(),
        v29.load(),
        v30.load(),
        v31.load(),
    ];
    if cached {
        column_kernel::<true, FORWARD>(&mut local, roots, level, q, column);
    } else {
        column_kernel::<false, FORWARD>(&mut local, roots, level, q, column);
    }
    *v0 = S::store(local[0]);
    *v1 = S::store(local[1]);
    *v2 = S::store(local[2]);
    *v3 = S::store(local[3]);
    *v4 = S::store(local[4]);
    *v5 = S::store(local[5]);
    *v6 = S::store(local[6]);
    *v7 = S::store(local[7]);
    *v8 = S::store(local[8]);
    *v9 = S::store(local[9]);
    *v10 = S::store(local[10]);
    *v11 = S::store(local[11]);
    *v12 = S::store(local[12]);
    *v13 = S::store(local[13]);
    *v14 = S::store(local[14]);
    *v15 = S::store(local[15]);
    *v16 = S::store(local[16]);
    *v17 = S::store(local[17]);
    *v18 = S::store(local[18]);
    *v19 = S::store(local[19]);
    *v20 = S::store(local[20]);
    *v21 = S::store(local[21]);
    *v22 = S::store(local[22]);
    *v23 = S::store(local[23]);
    *v24 = S::store(local[24]);
    *v25 = S::store(local[25]);
    *v26 = S::store(local[26]);
    *v27 = S::store(local[27]);
    *v28 = S::store(local[28]);
    *v29 = S::store(local[29]);
    *v30 = S::store(local[30]);
    *v31 = S::store(local[31]);
}

fn stored_columns<S: PairStorage, const FORWARD: bool>(
    v0: &mut [S],
    v1: &mut [S],
    v2: &mut [S],
    v3: &mut [S],
    v4: &mut [S],
    v5: &mut [S],
    v6: &mut [S],
    v7: &mut [S],
    v8: &mut [S],
    v9: &mut [S],
    v10: &mut [S],
    v11: &mut [S],
    v12: &mut [S],
    v13: &mut [S],
    v14: &mut [S],
    v15: &mut [S],
    v16: &mut [S],
    v17: &mut [S],
    v18: &mut [S],
    v19: &mut [S],
    v20: &mut [S],
    v21: &mut [S],
    v22: &mut [S],
    v23: &mut [S],
    v24: &mut [S],
    v25: &mut [S],
    v26: &mut [S],
    v27: &mut [S],
    v28: &mut [S],
    v29: &mut [S],
    v30: &mut [S],
    v31: &mut [S],
    roots: &Twiddles,
    level: usize,
) {
    let cached = !roots.columns.is_empty();
    let q = v0.len();
    v0.par_chunks_mut(1)
        .zip(v1.par_chunks_mut(1))
        .zip(v2.par_chunks_mut(1).zip(v3.par_chunks_mut(1)))
        .zip(
            v4.par_chunks_mut(1)
                .zip(v5.par_chunks_mut(1))
                .zip(v6.par_chunks_mut(1).zip(v7.par_chunks_mut(1))),
        )
        .zip(
            v8.par_chunks_mut(1)
                .zip(v9.par_chunks_mut(1))
                .zip(v10.par_chunks_mut(1).zip(v11.par_chunks_mut(1)))
                .zip(
                    v12.par_chunks_mut(1)
                        .zip(v13.par_chunks_mut(1))
                        .zip(v14.par_chunks_mut(1).zip(v15.par_chunks_mut(1))),
                ),
        )
        .zip(
            v16.par_chunks_mut(1)
                .zip(v17.par_chunks_mut(1))
                .zip(v18.par_chunks_mut(1).zip(v19.par_chunks_mut(1)))
                .zip(
                    v20.par_chunks_mut(1)
                        .zip(v21.par_chunks_mut(1))
                        .zip(v22.par_chunks_mut(1).zip(v23.par_chunks_mut(1))),
                )
                .zip(
                    v24.par_chunks_mut(1)
                        .zip(v25.par_chunks_mut(1))
                        .zip(v26.par_chunks_mut(1).zip(v27.par_chunks_mut(1)))
                        .zip(
                            v28.par_chunks_mut(1)
                                .zip(v29.par_chunks_mut(1))
                                .zip(v30.par_chunks_mut(1).zip(v31.par_chunks_mut(1))),
                        ),
                ),
        )
        .enumerate()
        .for_each(
            |(
                column,
                (
                    (
                        (((v0, v1), (v2, v3)), ((v4, v5), (v6, v7))),
                        (((v8, v9), (v10, v11)), ((v12, v13), (v14, v15))),
                    ),
                    (
                        (((v16, v17), (v18, v19)), ((v20, v21), (v22, v23))),
                        (((v24, v25), (v26, v27)), ((v28, v29), (v30, v31))),
                    ),
                ),
            )| {
                stored_column_leaf::<S, FORWARD>([
                    &mut v0[0], &mut v1[0], &mut v2[0], &mut v3[0],
                    &mut v4[0], &mut v5[0], &mut v6[0], &mut v7[0],
                    &mut v8[0], &mut v9[0], &mut v10[0], &mut v11[0],
                    &mut v12[0], &mut v13[0], &mut v14[0], &mut v15[0],
                    &mut v16[0], &mut v17[0], &mut v18[0], &mut v19[0],
                    &mut v20[0], &mut v21[0], &mut v22[0], &mut v23[0],
                    &mut v24[0], &mut v25[0], &mut v26[0], &mut v27[0],
                    &mut v28[0], &mut v29[0], &mut v30[0], &mut v31[0],
                ], roots, level, q, column, cached);
            },
        );
}

fn product_depth(plan: &Plan) -> usize {
    std::env::var("ZOPRO_CPU_FFT_STAGES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .or_else(|| {
            if std::env::var("ZOPRO_CPU_FFT4").is_ok_and(|v| v == "1") {
                Some(4)
            } else if std::env::var("ZOPRO_CPU_FFT3").is_ok_and(|v| v == "1") {
                Some(3)
            } else {
                std::env::var("ZOPRO_CPU_FFT2")
                    .ok()
                    .map(|v| if v == "0" { 1 } else { 2 })
            }
        })
        .unwrap_or(if plan.n >= (1 << 15) { 5 } else { 1 })
        .clamp(1, 5)
}

pub(super) fn fused_output_product(plan: &Plan, a: Vec<Fr>, b: Vec<Fr>) -> Vec<Fr> {
    if plan.n / 4 <= 1024 {
        return stored_product::<PackedPair>(plan, a, b);
    }
    type S = PackedPair;
    let started = crate::time::Instant::now();
    assert!(a.len() <= plan.n && b.len() <= plan.n);
    let mut values: Vec<S> = (0..plan.n / 4)
        .into_par_iter()
        .map(|i| {
            let mut lanes = [Fr::ZERO; 8];
            for j in 0..4 {
                lanes[j] = a.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
                lanes[4 + j] = b.get(4 * i + j).copied().unwrap_or(Fr::ZERO);
            }
            S::store(Pair::new(&lanes))
        })
        .collect();
    let mut out = a;
    out.reserve(plan.n - out.len());
    drop(b);
    let started = crate::trace("fft-pack", started);
    stored_transform::<S, false>(&mut values, &plan.inverse_roots, 0);
    let started = crate::trace("fft-inverse-ab", started);
    if plan.twisted.is_none() {
        values
            .par_chunks_mut(plan.forward_weights.lows.len())
            .zip(&plan.forward_weights.chunks)
            .for_each(|(values, chunk)| {
                let chunk = broadcast(chunk);
                for (value, low) in values.iter_mut().zip(&plan.forward_weights.lows) {
                    *value = S::store(value.load().mul(&broadcast(low)).mul(&chunk));
                }
            });
    }
    let started = crate::trace("fft-weights", started);
    let roots = plan
        .twisted
        .as_ref()
        .map(|(roots, _)| roots)
        .unwrap_or(&plan.roots);

    let cached = !roots.columns.is_empty();
    let q = values.len() / 32;
    values
        .par_chunks_mut(q)
        .for_each(|part| stored_transform::<S, true>(part, roots, 5));
    // SAFETY: reserve covers n fields, inputs have been packed, and each
    // output column owns128 distinct fields initialized before set_len.
    let output = unsafe {
        std::slice::from_raw_parts_mut(out.as_mut_ptr().cast::<std::mem::MaybeUninit<Fr>>(), plan.n)
    };
    output
        .par_chunks_mut(128)
        .enumerate()
        .for_each(|(column, out)| {
            let mut local: [Pair; 32] = core::array::from_fn(|row| values[row * q + column].load());
            if cached {
                column_kernel::<true, true>(&mut local, roots, 0, q, column);
            } else {
                column_kernel::<false, true>(&mut local, roots, 0, q, column);
            }
            for (out, value) in out.chunks_exact_mut(4).zip(local) {
                let (a, b) = Quad::unpair(&value);
                for (out, value) in out.iter_mut().zip(a.mul(&b).get()) {
                    out.write(value);
                }
            }
        });
    // SAFETY: every field was initialized by the completed pass above.
    unsafe { out.set_len(plan.n) };
    crate::trace("fft-product-output", started);
    out
}

pub(super) fn fused_output_enabled(plan: &Plan) -> bool {
    plan.n >= (1 << 15)
        && product_depth(plan) == 5
        && crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT8")
        && crate::simd::cpu_option("ZOPRO_CPU_FFT_PACKED")
        && crate::simd::cpu_option("ZOPRO_CPU_FFT_FUSED_OUTPUT")
}

#[inline(always)]
fn column_kernel<const CACHED: bool, const FORWARD: bool>(
    local: &mut [Pair; 32],
    roots: &Twiddles,
    level: usize,
    q: usize,
    column: usize,
) {
    for order in 0..5 {
        let stage = if FORWARD { 4 - order } else { order };
        let gap = 16 >> stage;
        for block in (0..32).step_by(2 * gap) {
            for j in 0..gap {
                let low = block + j;
                let high = low + gap;
                let root = if CACHED {
                    broadcast(&roots.columns[level][31 * column + 32 - 2 * gap + j])
                } else {
                    broadcast(&roots.layers[level + stage][j * q + column])
                };
                let (a, b) = (local[low], local[high]);
                if FORWARD {
                    let product = b.mul(&root);
                    local[low] = a.add(&product);
                    local[high] = a.sub(&product);
                } else {
                    local[low] = a.add(&b);
                    local[high] = a.sub(&b).mul(&root);
                }
            }
        }
    }
}



// Isolated derivative component study; not used by proving entry points.
// Unnormalized inverse FFT means the returned prefix is
// n * (E(A) * B + A * E(B)), where E(f) = X f'.
pub(super) fn euler_product_study(plan: &Plan, mut a: Vec<Fr>, b: Vec<Fr>) -> Vec<Fr> {
    assert!(plan.n >= 8 && a.len() <= plan.n && b.len() <= plan.n);
    assert!(!plan.released && plan.twisted.is_none());
    let count = a.len().max(b.len());
    a.resize(count, Fr::ZERO);
    if count == 0 { return a; }
    let mut values: Vec<PackedPair> = (0..plan.n / 4).into_par_iter().map(|i| {
        let mut lanes = [Fr::ZERO; 8];
        for j in 0..4 {
            lanes[j] = a.get(4*i+j).copied().unwrap_or(Fr::ZERO);
            lanes[4+j] = b.get(4*i+j).copied().unwrap_or(Fr::ZERO);
        }
        PackedPair::store(Pair::new(&lanes))
    }).collect();
    stored_transform::<PackedPair, false>(&mut values, &plan.inverse_roots, 0);
    if cfg!(zopro_euler_leading_weights) && plan.n <= 1usize<<28 {
        let log=plan.n.trailing_zeros() as usize;let low=log.min(10);
        // <=8KiB of ordinary integer weights replaces <=40KiB of field weights.
        let lows:Vec<[u64;4]>=(0..(1usize<<low)/4).map(|i|
            core::array::from_fn(|j|(bit_reverse(4*i+j,low)<<(log-low))as u64)).collect();
        values.par_chunks_mut(lows.len()).enumerate().for_each(|(i,chunk)| {
            let high=bit_reverse(i,log-low) as u64;
            for (value,low) in chunk.iter_mut().zip(&lows) {
                let factors=core::array::from_fn(|j|low[j%4]+high);
                *value=PackedPair::store(value.load().mul_u28(&factors));
            }
        });
    } else {
    // k in bit-reversed order splits additively into <=1024 low-index
    // weights and one scalar per chunk. No n-element weight table.
    let log = plan.n.trailing_zeros() as usize;
    let low = log.min(10);
    let lows: Vec<Quad> = (0..(1usize << low)/4).map(|i| {
        Quad::new(&core::array::from_fn(|j| {
            Fr::from((bit_reverse(4*i+j, low) << (log-low)) as u64)
        }))
    }).collect();
    values.par_chunks_mut(lows.len()).enumerate().for_each(|(i, chunk)| {
        let high = Fr::from(bit_reverse(i, log-low) as u64);
        let high = Quad::new(&[high;4]);
        for (value, low) in chunk.iter_mut().zip(&lows) {
            *value = PackedPair::store(value.load().mul(&broadcast(&low.add(&high))));
        }
    });
    drop(lows);
    }
    stored_transform::<PackedPair, true>(&mut values, &plan.roots, 0);
    euler_combine_products(&mut a, &b, &values);
    a
}

// Rejoin two packed pairs into eight independent derivative rows before
// the existing fused field product. No extra allocation or retained state.
#[inline]
fn euler_combine_products(a: &mut [Fr], b: &[Fr], values: &[PackedPair]) {
    if cfg!(zopro_euler_pointwise8) {
        a.par_chunks_mut(8).zip(values.par_chunks(2)).enumerate().for_each(|(i,(out,values))| {
            let (da0,db0) = values[0].load().halves();
            let (da1,db1) = values[1].load().halves();
            let da = Pair::from_halves(&da0,&da1);
            let db = Pair::from_halves(&db0,&db1);
            let aa = Pair::new(&core::array::from_fn(|j| out.get(j).copied().unwrap_or(Fr::ZERO)));
            let bb = Pair::new(&core::array::from_fn(|j| b.get(8*i+j).copied().unwrap_or(Fr::ZERO)));
            let product = da.sum2(&bb,&aa,&db).get();
            out.copy_from_slice(&product[..out.len()]);
        });
    } else {
    a.par_chunks_mut(4).zip(values.par_iter()).enumerate().for_each(|(i,(out,value))| {
        let (da,db) = Quad::unpair(&value.load());
        let aa = Quad::new(&core::array::from_fn(|j| out.get(j).copied().unwrap_or(Fr::ZERO)));
        let bb = Quad::new(&core::array::from_fn(|j| b.get(4*i+j).copied().unwrap_or(Fr::ZERO)));
        let product = da.mul(&bb).add(&aa.mul(&db)).get();
        out.copy_from_slice(&product[..out.len()]);
    });
    }
}

#[cfg(test)]
mod euler_pointwise_tests {
    use super::*;
    #[test]
    fn eight_rows_match_scalar_products_with_partial_inputs() {
        let lengths=[0,1,2,3,4,5,7,8,9,15,16,17,31,32,33,127,128,129,1023,1024,1025,2047,2048,2049,4095,4096,4097];
        let mut seed=0x45554c4552505438u64;
        let mut random=|| {
            let mut bytes=[0u8;64];
            for b in bytes.chunks_mut(8) {seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}
            Fr::from_random_bytes(&bytes)
        };
        let base:Vec<[Fr;4]>=(0..4104).map(|_|core::array::from_fn(|_|random())).collect();
        let edges=[Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(2u64)];
        let mut checked=0usize;
        for n in lengths {for nb in [0,n/3,n] {for pattern in 0..3 {
            let count=(n+7)/8*8;
            let rows:Vec<[Fr;4]>=(0..count).map(|i|core::array::from_fn(|j|
                match pattern {0=>Fr::ZERO,1=>edges[(i+3*j)%4],_=>base[i][j]})).collect();
            let mut a:Vec<_>=rows[..n].iter().map(|v|v[0]).collect();
            let b:Vec<_>=rows[..nb].iter().map(|v|v[1]).collect();
            let values:Vec<PackedPair>=rows.chunks(4).map(|chunk| {
                PackedPair::store(Pair::new(&core::array::from_fn(|j|chunk[j%4][if j<4 {2}else{3}])))
            }).collect();
            let expected:Vec<_>=(0..n).map(|i|rows[i][2]*b.get(i).copied().unwrap_or(Fr::ZERO)+a[i]*rows[i][3]).collect();
            euler_combine_products(&mut a,&b,&values);
            assert_eq!(a,expected,"n={n} nb={nb} pattern={pattern}");
            checked+=n;
        }}}
        eprintln!("EULER_POINTWISE_FIELD_COMPARISONS={checked}");
    }
}
