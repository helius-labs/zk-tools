//! The final DIF8 stages, evaluated across independent sequences.
use super::*;
const HALF: Fr = Fr::from_decimal(
    "10944121435919637611123202872628637544274182200208017171849102093287904247809",
);
struct Constants {
    i: Pair,
    plus: Pair,
    minus: Pair,
}
impl Constants {
    fn new(roots: &Quad) -> Self {
        let [one, w, i, w3] = roots.get();
        debug_assert_eq!(one, Fr::ONE);
        debug_assert_eq!(i, w.square());
        debug_assert_eq!(w3, w * i);
        Self {
            i: Pair::new(&[i; 8]),
            plus: Pair::new(&[(w + w3) * HALF; 8]),
            minus: Pair::new(&[(w - w3) * HALF; 8]),
        }
    }
}
#[inline(always)]
fn transpose(v: [Pair; 4]) -> [Pair; 4] {
    let t0 = Pair::shuffle_pair::<0, 1, 4, 5>(&v[0], &v[1]);
    let t1 = Pair::shuffle_pair::<2, 3, 6, 7>(&v[0], &v[1]);
    let t2 = Pair::shuffle_pair::<0, 1, 4, 5>(&v[2], &v[3]);
    let t3 = Pair::shuffle_pair::<2, 3, 6, 7>(&v[2], &v[3]);
    [
        Pair::shuffle_pair::<0, 2, 4, 6>(&t0, &t2),
        Pair::shuffle_pair::<1, 3, 5, 7>(&t0, &t2),
        Pair::shuffle_pair::<0, 2, 4, 6>(&t1, &t3),
        Pair::shuffle_pair::<1, 3, 5, 7>(&t1, &t3),
    ]
}
fn tile(values: &mut [Pair; 8], c: &Constants) {
    let [x0, x1, x2, x3] = transpose([values[0], values[2], values[4], values[6]]);
    let [x4, x5, x6, x7] = transpose([values[1], values[3], values[5], values[7]]);
    let (s0, t0) = (x0.add(&x4), x0.sub(&x4));
    let (s1, t1) = (x1.add(&x5), x1.sub(&x5));
    let (s2, t2) = (x2.add(&x6), x2.sub(&x6));
    let (s3, t3) = (x3.add(&x7), x3.sub(&x7));
    let e0 = s0.add(&s2);
    let e1 = s1.add(&s3);
    let e2 = s0.sub(&s2);
    let p0 = s1.sub(&s3).mul(&c.i);
    let p1 = t2.mul(&c.i);
    let p2 = t1.add(&t3).mul(&c.plus);
    let p3 = t1.sub(&t3).mul(&c.minus);
    let even = transpose([e0.add(&e1), e0.sub(&e1), e2.add(&p0), e2.sub(&p0)]);
    let b0 = t0.add(&p1);
    let b1 = t0.sub(&p1);
    let q0 = p2.add(&p3);
    let q1 = p2.sub(&p3);
    let odd = transpose([b0.add(&q0), b0.sub(&q0), b1.add(&q1), b1.sub(&q1)]);
    for j in 0..4 {
        values[2 * j] = even[j];
        values[2 * j + 1] = odd[j];
    }
}
pub(super) fn dif_serial(values: &mut [Pair], roots: &Twiddles, mut level: usize) {
    debug_assert!(values.len() >= 8 && values.len().is_power_of_two());
    debug_assert!(roots.single.is_none());
    let mut gap = values.len() / 2;
    while gap >= 2 {
        let layer = &roots.layers[level][..gap];
        for chunk in values.chunks_exact_mut(2 * gap) {
            let (lo, hi) = chunk.split_at_mut(gap);
            dif_quads(lo, hi, layer);
        }
        gap /= 2;
        level += 1;
    }
    let constants = Constants::new(&roots.layers[level][0]);
    let (tiles, tail) = values.as_chunks_mut::<8>();
    debug_assert!(tail.is_empty());
    for values in tiles {
        tile(values, &constants);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pack(x: &[Fr; 64]) -> [Pair; 8] {
        core::array::from_fn(|row| {
            Pair::new(&core::array::from_fn(|lane| {
                x[(lane / 4 * 4 + row / 2) * 8 + row % 2 * 4 + lane % 4]
            }))
        })
    }
    fn unpack(v: &[Pair; 8]) -> [Fr; 64] {
        let fields = v.map(|p| p.get());
        core::array::from_fn(|k| {
            let (seq, pos) = (k / 8, k % 8);
            fields[seq % 4 * 2 + pos / 4][seq / 4 * 4 + pos % 4]
        })
    }
    fn direct(x: &[Fr], w: Fr, bit_reversed_index: usize) -> Fr {
        let log = x.len().trailing_zeros();
        let k = bit_reversed_index.reverse_bits() >> (usize::BITS - log);
        let step = w.pow(&[k as u64]);
        let mut p = Fr::ONE;
        let mut out = Fr::ZERO;
        for x in x {
            out += *x * p;
            p *= step;
        }
        out
    }
    #[test]
    fn tile_basis_boundaries_and_transpose_match_scalar_dft() {
        let mut rng = crate::testing::Seeded::new(0x66746638636f6c);
        for power in [1, 3, 5, 7] {
            let w = Fr::root_of_unity(8).unwrap().pow(&[power]);
            let c = Constants::new(&Quad::new(&[Fr::ONE, w, w.square(), w.pow(&[3])]));
            for case in 0..192 {
                let x: [Fr; 64] = core::array::from_fn(|i| {
                    if case < 64 {
                        if case == i {
                            Fr::ONE
                        } else {
                            Fr::ZERO
                        }
                    } else {
                        match (i + case) % 16 {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => -Fr::ONE,
                            _ => rng.fr(),
                        }
                    }
                });
                let mut v = pack(&x);
                assert_eq!(unpack(&v), x);
                // Verify both half-wise transposes are exact involutions.
                let t = [v[0], v[2], v[4], v[6]];
                let roundtrip = transpose(transpose(t));
                for i in 0..4 {
                    assert_eq!(roundtrip[i].get(), t[i].get());
                }
                tile(&mut v, &c);
                let got = unpack(&v);
                for seq in 0..8 {
                    for k in 0..8 {
                        assert_eq!(got[seq * 8 + k], direct(&x[seq * 8..seq * 8 + 8], w, k));
                    }
                }
            }
        }
    }
    #[test]
    fn selected_full_dif_leaves_match_direct_transforms_and_short_fallbacks() {
        let mut rng = crate::testing::Seeded::new(0x6674666c656166);
        for n in [8, 16, 32, 64, 128, 256, 512] {
            for power in [1, 3] {
                let w = Fr::root_of_unity(n).unwrap().pow(&[power]);
                let mut p = Fr::ONE;
                let roots = Twiddles::new(
                    (0..n / 2)
                        .map(|_| {
                            let v = p;
                            p *= w;
                            v
                        })
                        .collect(),
                );
                let a: Vec<_> = (0..n)
                    .map(|i| if i % 17 == 0 { Fr::ZERO } else { rng.fr() })
                    .collect();
                let b: Vec<_> = (0..n)
                    .map(|i| if i % 13 == 0 { -Fr::ONE } else { rng.fr() })
                    .collect();
                let mut values: Vec<_> = (0..n / 4)
                    .map(|i| {
                        Pair::new(&core::array::from_fn(|j| {
                            if j < 4 {
                                a[4 * i + j]
                            } else {
                                b[4 * i + j - 4]
                            }
                        }))
                    })
                    .collect();
                super::super::dif_serial(&mut values, &roots, 0);
                for (i, value) in values.iter().enumerate() {
                    let got = value.get();
                    for j in 0..8 {
                        assert_eq!(
                            got[j],
                            direct(if j < 4 { &a } else { &b }, w, 4 * i + j % 4),
                            "n={n},rootpower={power},row={i},lane={j}"
                        );
                    }
                }
            }
        }
    }
}
