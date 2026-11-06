//! Private H-only complete unweighted G1 reduction. No production route.
use super::*;
use std::{hint::black_box, mem::size_of};
use zopro_bn254::{G1Affine, G1Config, G1Projective};

#[derive(Clone, Copy)]
struct TripleRecord {
    prefix: FqBatch,
    denominator: FqBatch,
}
#[derive(Default)]
struct Ternary {
    triples: Vec<usize>,
    pairs: Vec<usize>,
    records: Vec<TripleRecord>,
    pair_records: Vec<TripleRecord>,
}
impl Ternary {
    fn bytes(&self) -> usize {
        (self.triples.capacity() + self.pairs.capacity()) * size_of::<usize>()
            + (self.records.capacity() + self.pair_records.capacity()) * size_of::<TripleRecord>()
    }
    // Regions are disjoint and sorted. Rejection leaves points/counts intact.
    fn layer(
        &mut self,
        points: &mut [LimbPoint<Fq>],
        starts: &[usize],
        counts: &mut [usize],
    ) -> bool {
        if starts.len() != counts.len() {
            return false;
        }
        let mut previous_end = 0;
        let mut triples = 0;
        for (&start, &n) in starts.iter().zip(counts.iter()) {
            let Some(end) = start.checked_add(n) else {
                return false;
            };
            if start < previous_end || end > points.len() {
                return false;
            }
            previous_end = end;
            triples += n / 3;
        }
        if triples < LANES {
            return false;
        }
        self.triples.clear();
        self.pairs.clear();
        self.records.clear();
        self.pair_records.clear();
        self.triples.reserve_exact(triples);
        for (&start, &n) in starts.iter().zip(counts.iter()) {
            if points[start..start + n].iter().any(LimbPoint::is_infinity) {
                return false;
            }
            for j in (0..n / 3 * 3).step_by(3) {
                self.triples.push(start + j);
            }
            if n % 3 == 2 {
                self.pairs.push(start + n - 2);
            }
        }
        self.records
            .reserve_exact(self.triples.len().div_ceil(LANES));
        self.pair_records
            .reserve_exact(self.pairs.len().div_ceil(LANES));
        let zero = Fq::to_limbs(Fq::ZERO);
        let one = Fq::to_limbs(Fq::ONE);
        // Off-curve padding is arithmetic-only: H=2 for triples, dx=1 for
        // pairs. It is never included in identity checks or output stores.
        let dummy_a = LimbPoint::<Fq> { x: zero, y: zero };
        let dummy_b = LimbPoint::<Fq> { x: one, y: zero };
        let mut chains = FqBatch::one();
        for row in self.triples.chunks(LANES) {
            let a = core::array::from_fn(|i| row.get(i).map(|&k| &points[k]).unwrap_or(&dummy_a));
            let b =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 1]).unwrap_or(&dummy_b));
            let r =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 2]).unwrap_or(&dummy_b));
            let ax = Fq::load(a.map(|p| &p.x));
            let ay = Fq::load(a.map(|p| &p.y));
            let bx = Fq::load(b.map(|p| &p.x));
            let by = Fq::load(b.map(|p| &p.y));
            let rx = Fq::load(r.map(|p| &p.x));
            let d = bx.sub(&ax);
            let n = by.sub(&ay);
            let h = d.mul(&d).difference2(&ax.add(&bx).add(&rx), &n, &n);
            self.records.push(TripleRecord {
                prefix: chains,
                denominator: h,
            });
            chains = chains.mul(&h);
        }
        for row in self.pairs.chunks(LANES) {
            let a = core::array::from_fn(|i| row.get(i).map(|&k| &points[k]).unwrap_or(&dummy_a));
            let b =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 1]).unwrap_or(&dummy_b));
            let d = Fq::load(b.map(|p| &p.x)).sub(&Fq::load(a.map(|p| &p.x)));
            self.pair_records.push(TripleRecord {
                prefix: chains,
                denominator: d,
            });
            chains = chains.mul(&d);
        }
        // A zero in any active lane or either record family rejects the whole
        // layer BEFORE ANY point/count write. No per-batch export is needed.
        let Some(mut inverse) = chains.inverse_checked() else {
            return false;
        };
        for (j, g) in self.pair_records.iter().enumerate().rev() {
            let row = &self.pairs[j * LANES..self.pairs.len().min((j + 1) * LANES)];
            let a = core::array::from_fn(|i| row.get(i).map(|&k| &points[k]).unwrap_or(&dummy_a));
            let b =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 1]).unwrap_or(&dummy_b));
            let ax = Fq::load(a.map(|p| &p.x));
            let ay = Fq::load(a.map(|p| &p.y));
            let bx = Fq::load(b.map(|p| &p.x));
            let by = Fq::load(b.map(|p| &p.y));
            let inv = g.prefix.mul(&inverse);
            inverse = inverse.mul(&g.denominator);
            let slope = by.sub(&ay).mul(&inv);
            let x = slope.mul(&slope).sub(&ax.add(&bx));
            let y = slope.mul(&ax.sub(&x)).sub(&ay);
            let (x, y) = (Fq::store(&x), Fq::store(&y));
            for (i, &k) in row.iter().enumerate() {
                points[k] = LimbPoint { x: x[i], y: y[i] };
            }
        }
        for (j, g) in self.records.iter().enumerate().rev() {
            let row = &self.triples[j * LANES..self.triples.len().min((j + 1) * LANES)];
            let a = core::array::from_fn(|i| row.get(i).map(|&k| &points[k]).unwrap_or(&dummy_a));
            let b =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 1]).unwrap_or(&dummy_b));
            let r =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 2]).unwrap_or(&dummy_b));
            let ax = Fq::load(a.map(|p| &p.x));
            let ay = Fq::load(a.map(|p| &p.y));
            let bx = Fq::load(b.map(|p| &p.x));
            let by = Fq::load(b.map(|p| &p.y));
            let rx = Fq::load(r.map(|p| &p.x));
            let ry = Fq::load(r.map(|p| &p.y));
            let inv = g.prefix.mul(&inverse);
            inverse = inverse.mul(&g.denominator);
            let d = bx.sub(&ax);
            let n = by.sub(&ay);
            let s = ax.add(&bx).add(&rx);
            let u = d.sum2(&ay.add(&ry), &n, &rx.sub(&ax));
            let q = u.mul(&inv);
            let w = d.mul(&q);
            let v = n.mul(&q);
            let x = w.mul(&w).sub(&v.add(&v)).add(&ax.add(&bx)).sub(&rx);
            let y = w
                .mul(&rx.sub(&x).sub(&s.add(&s)).add(&v))
                .add(&ay.add(&by).add(&ry));
            let (x, y) = (Fq::store(&x), Fq::store(&y));
            for (i, &k) in row.iter().enumerate() {
                points[k] = LimbPoint { x: x[i], y: y[i] };
            }
        }
        // All original coordinates have now been consumed. Compaction cannot
        // overwrite an unread source because j <= 3j, and groups are disjoint.
        for (&start, n) in starts.iter().zip(counts.iter_mut()) {
            let old = *n;
            for j in 0..old / 3 {
                points[start + j] = points[start + 3 * j];
            }
            if old % 3 != 0 {
                points[start + old / 3] = points[start + old - old % 3];
            }
            *n = old.div_ceil(3);
        }
        true
    }
}
