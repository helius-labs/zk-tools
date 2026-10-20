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
        .unwrap_or(if plan.n >= (1 << 19) { 4 } else { 1 })
        .clamp(1, 4);
    match depth {
        4 => quotient_impl::<4>(plan, a, b, c),
        3 => quotient_impl::<3>(plan, a, b, c),
        2 => quotient_impl::<2>(plan, a, b, c),
        _ => quotient_impl::<1>(plan, a, b, c),
    }
}
pub(super) fn quotient_impl<const DEPTH: usize>(
    plan: &Plan,
    mut a: Vec<Fr>,
    mut b: Vec<Fr>,
    mut c: Vec<Fr>,
) -> Vec<Fr> {
    assert!(a.len() <= plan.n && b.len() <= plan.n && c.len() <= plan.n);
    a.resize(plan.n, Fr::ZERO);
    b.resize(plan.n, Fr::ZERO);
    c.resize(plan.n, Fr::ZERO);
    let mut values: Vec<Pair> = a
        .par_chunks_exact(4)
        .zip(b.par_chunks_exact(4))
        .map(|(a, b)| {
            Quad::pair(
                &Quad::new(a.try_into().unwrap()),
                &Quad::new(b.try_into().unwrap()),
            )
        })
        .collect();
    drop(a);
    drop(b);
    dif::<DEPTH>(&mut values, &plan.inverse_roots, 0);
    values
        .par_chunks_mut(plan.forward_weights.lows.len())
        .zip(&plan.forward_weights.chunks)
        .for_each(|(values, chunk)| {
            let chunk = broadcast(chunk);
            for (value, low) in values.iter_mut().zip(&plan.forward_weights.lows) {
                *value = value.mul(&broadcast(low)).mul(&chunk);
            }
        });
    dit::<DEPTH>(&mut values, &plan.roots, 0);
    values
        .par_iter_mut()
        .zip(c.par_chunks_exact(4))
        .for_each(|(value, c)| {
            let (a, b) = Quad::unpair(value);
            *value = Quad::pair(&a.mul(&b), &Quad::new(c.try_into().unwrap()));
        });
    drop(c);
    dif::<DEPTH>(&mut values, &plan.inverse_roots, 0);
    let mut out = vec![Fr::ZERO; plan.n];
    out.par_chunks_mut(4 * plan.reverse_weights.lows.len())
        .zip(values.par_chunks(plan.reverse_weights.lows.len()))
        .zip(&plan.reverse_weights.chunks)
        .for_each(|((out, values), chunk)| {
            for ((out, value), low) in out
                .chunks_exact_mut(4)
                .zip(values)
                .zip(&plan.reverse_weights.lows)
            {
                let (a, c) = Quad::unpair(value);
                out.copy_from_slice(
                    &a.mul(low)
                        .mul(chunk)
                        .sub(&c.mul(&plan.reverse_weights.chunks[0]))
                        .get(),
                );
            }
        });
    out.pop();
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
fn dit_within(a: &mut Pair, b: &mut Pair, pair: &Pair) {
    let (lo, hi) = (
        Pair::shuffle_pair::<0, 2, 4, 6>(a, b),
        Pair::shuffle_pair::<1, 3, 5, 7>(a, b),
    );
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
            dit_within(&mut a[0], &mut b[0], &broadcast(&twiddles.pair));
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
