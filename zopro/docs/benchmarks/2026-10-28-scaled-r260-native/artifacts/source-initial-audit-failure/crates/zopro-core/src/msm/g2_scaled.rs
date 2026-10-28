//! Experimental native affine batch over scaled coordinates, host storage.
use super::*;
use zopro_bn254::ifma8::scaled::{self as w, Wide};
pub(super) type Record = (Wide<2>, Wide<3>);
#[inline(always)]
fn px(v: [Fq2; LANES]) -> [Wide<4>; 2] {
    [w::import_x(&v.map(|v| v.c0)), w::import_x(&v.map(|v| v.c1))]
}
#[inline(always)]
fn py(v: [Fq2; LANES]) -> [Wide<2>; 2] {
    [w::import_y(&v.map(|v| v.c0)), w::import_y(&v.map(|v| v.c1))]
}
#[inline(always)]
fn dx(b: &[Wide<4>; 2], a: &[Wide<4>; 2]) -> [Wide<8>; 2] {
    [w::dx(&b[0], &a[0]), w::dx(&b[1], &a[1])]
}
pub(super) fn apply<C: CurveConfig<Base = Fq2>>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, Affine<C>)],
    records: &mut Vec<Record>,
) {
    assert_eq!(batch.len() % LANES, 0);
    records.clear();
    if batch.is_empty() {
        return;
    }
    // Existing queued-batch contract supplies distinct indices. Exceptional
    // values take a complete whole-call scalar path before vector mutation.
    if batch.iter().any(|(id, b)| {
        let a = buckets[*id as usize];
        a.is_identity() || b.is_identity() || a.x == b.x
    }) {
        for &(id, b) in batch {
            let i = id as usize;
            buckets[i] = (buckets[i] + b).to_affine();
        }
        return;
    }
    records.reserve(batch.len() / LANES);
    let mut chains = Wide::<2>::one();
    for k in (0..batch.len()).step_by(LANES) {
        let ax = px(core::array::from_fn(|i| buckets[batch[k + i].0 as usize].x));
        let bx = px(core::array::from_fn(|i| batch[k + i].1.x));
        let norm = w::norm(&dx(&bx, &ax));
        records.push((chains, norm));
        chains = chains.mul_norm(&norm);
    }
    let mut inverse = chains.inverse_nonzero();
    for j in (0..batch.len() / LANES).rev() {
        let k = j * LANES;
        let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch[k + i].0 as usize]);
        let b: [Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = px(a.map(|p| p.x));
        let ay = py(a.map(|p| p.y));
        let bx = px(b.map(|p| p.x));
        let by = py(b.map(|p| p.y));
        let denominator = dx(&bx, &ax);
        let dy = [w::dy(&by[0], &ay[0]), w::dy(&by[1], &ay[1])];
        let factor = records[j].0.mul(&inverse);
        inverse = inverse.mul_norm(&records[j].1);
        let slope = w::slope(&dy, &denominator, &factor);
        let square = w::square(&slope);
        let x = core::array::from_fn::<_, 2, _>(|i| w::xcoord(&square[i], &ax[i], &bx[i]));
        let d = core::array::from_fn(|i| w::difference(&ax[i], &x[i]));
        let y = w::finish(&slope, &d, &ay);
        let x0 = w::export_x(&x[0]);
        let x1 = w::export_x(&x[1]);
        let y0 = w::export_y(&y[0]);
        let y1 = w::export_y(&y[1]);
        for i in 0..LANES {
            buckets[batch[k + i].0 as usize] =
                Affine::new(Fq2::new(x0[i], x1[i]), Fq2::new(y0[i], y1[i]));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{G2Affine, G2Config, G2Projective};
    fn random(seed: &mut u64) -> Fr {
        let mut b = [0; 32];
        for c in b.chunks_mut(8) {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            c.copy_from_slice(&seed.to_le_bytes());
        }
        Fr::from_le_bytes_mod_order(&b)
    }
    #[test]
    fn full_width_generator_outputs_and_reuse() {
        let g = G2Projective::generator();
        let mut seed = 0x941ca2246f1a7853;
        let mut outputs = 0;
        for n in [8, 16, 32, 64, 200] {
            let mut scalars: Vec<_> = (0..n).map(|_| random(&mut seed)).collect();
            let steps: Vec<_> = (0..n).map(|_| random(&mut seed)).collect();
            let mut actual =
                G2Projective::batch_to_affine(&scalars.iter().map(|s| g * *s).collect::<Vec<_>>());
            let incoming =
                G2Projective::batch_to_affine(&steps.iter().map(|s| g * *s).collect::<Vec<_>>());
            let batch: Vec<_> = incoming
                .iter()
                .enumerate()
                .map(|(i, p)| (i as u32, *p))
                .collect();
            let mut records = vec![];
            for _ in 0..8 {
                apply::<G2Config>(&mut actual, &batch, &mut records);
                for (i, s) in scalars.iter_mut().enumerate() {
                    *s += steps[i];
                }
                let expected = G2Projective::batch_to_affine(
                    &scalars.iter().map(|s| g * *s).collect::<Vec<_>>(),
                );
                assert_eq!(actual, expected);
                outputs += n;
                assert_eq!(records.len(), n / 8);
            }
        }
        assert_eq!(outputs, 2560);
    }
    #[test]
    fn whole_call_exception_fallback_before_vector_mutation() {
        let g = G2Affine::GENERATOR;
        let z = G2Affine::IDENTITY;
        for special in 0..4 {
            let mut actual: Vec<_> = (0..8)
                .map(|i| (g * Fr::from((i + 7) as u64)).to_affine())
                .collect();
            let mut batch: Vec<_> = (0..8)
                .map(|i| (i as u32, (g * Fr::from((i + 300) as u64)).to_affine()))
                .collect();
            match special {
                0 => actual[3] = z,
                1 => batch[3].1 = z,
                2 => batch[3].1 = actual[3],
                _ => batch[3].1 = -actual[3],
            }
            let expected: Vec<_> = actual
                .iter()
                .zip(&batch)
                .map(|(a, (_, b))| (*a + *b).to_affine())
                .collect();
            let mut records = vec![];
            apply::<G2Config>(&mut actual, &batch, &mut records);
            assert_eq!(actual, expected);
            assert_eq!(records.capacity(), 0);
        }
        let mut actual: Vec<Affine<G2Config>> = vec![];
        let mut records = vec![];
        apply::<G2Config>(&mut actual, &[], &mut records);
        assert_eq!(records.capacity(), 0);
    }
}
