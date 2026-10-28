//! Private complete unweighted G1 reduction experiment.
use super::*;
use std::{hint::black_box, mem::size_of};
use zopro_bn254::{G1Affine, G1Config, G1Projective};

#[derive(Clone, Copy)]
struct TripleRecord {
    prefix: FqBatch,
    denominator: FqBatch,
    first: FqBatch,
    second: FqBatch,
    ax: FqBatch,
    ay: FqBatch,
    xsum: FqBatch,
    rx: FqBatch,
}
#[derive(Default)]
struct Ternary {
    triples: Vec<usize>,
    pairs: Vec<usize>,
    records: Vec<TripleRecord>,
    pair_records: Vec<Group<Fq>>,
}
impl Ternary {
    fn bytes(&self) -> usize {
        (self.triples.capacity() + self.pairs.capacity()) * size_of::<usize>()
            + self.records.capacity() * size_of::<TripleRecord>()
            + self.pair_records.capacity() * size_of::<Group<Fq>>()
    }
    // No output is touched until every denominator is known nonzero. A false
    // result leaves the original input and counts ready for reduce_layer.
    fn layer(
        &mut self,
        points: &mut [LimbPoint<Fq>],
        starts: &[usize],
        counts: &mut [usize],
    ) -> bool {
        let triples: usize = counts.iter().map(|n| n / 3).sum();
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
        let dummy_a = LimbPoint::<Fq> { x: zero, y: zero };
        let dummy_b = LimbPoint::<Fq> { x: one, y: zero };
        let mut chains = FqBatch::one();
        for row in self.triples.chunks(LANES) {
            let a: [&LimbPoint<Fq>; LANES] =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k]).unwrap_or(&dummy_a));
            let b: [&LimbPoint<Fq>; LANES] =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 1]).unwrap_or(&dummy_b));
            let r: [&LimbPoint<Fq>; LANES] =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 2]).unwrap_or(&dummy_b));
            let ax = Fq::load(a.map(|p| &p.x));
            let ay = Fq::load(a.map(|p| &p.y));
            let bx = Fq::load(b.map(|p| &p.x));
            let by = Fq::load(b.map(|p| &p.y));
            let rx = Fq::load(r.map(|p| &p.x));
            let ry = Fq::load(r.map(|p| &p.y));
            let d = bx.sub(&ax);
            let n = by.sub(&ay);
            let d2 = d.mul(&d);
            let xsum = ax.add(&bx);
            let h = d2.difference2(&xsum.add(&rx), &n, &n);
            let u = d.sum2(&ay.add(&ry), &n, &rx.sub(&ax));
            let denominator = d.mul(&h);
            if denominator.get().iter().any(Field::is_zero) {
                return false;
            }
            self.records.push(TripleRecord {
                prefix: chains,
                denominator,
                first: n.mul(&h),
                second: d2.mul(&u),
                ax,
                ay,
                xsum,
                rx,
            });
            chains = chains.mul(&denominator);
        }
        // Pair remainders join the SAME inversion chains. Padding denominators
        // are nonzero and padded outputs are never stored.
        for row in self.pairs.chunks(LANES) {
            let a: [&LimbPoint<Fq>; LANES] =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k]).unwrap_or(&dummy_a));
            let b: [&LimbPoint<Fq>; LANES] =
                core::array::from_fn(|i| row.get(i).map(|&k| &points[k + 1]).unwrap_or(&dummy_b));
            let ax = Fq::load(a.map(|p| &p.x));
            let ay = Fq::load(a.map(|p| &p.y));
            let bx = Fq::load(b.map(|p| &p.x));
            let by = Fq::load(b.map(|p| &p.y));
            let denominator = bx.sub(&ax);
            if denominator.get().iter().any(Field::is_zero) {
                return false;
            }
            self.pair_records.push(Group {
                ax,
                ay,
                xsum: ax.add(&bx),
                norm: denominator,
                numerator: by.sub(&ay).mul(&chains),
            });
            chains = chains.mul(&denominator);
        }
        let mut inverse = chains.inverse_nonzero();
        for (j, g) in self.pair_records.iter().enumerate().rev() {
            let slope = g.numerator.mul(&inverse);
            inverse = inverse.mul(&g.norm);
            let x = slope.mul(&slope).sub(&g.xsum);
            let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);
            let (x, y) = (Fq::store(&x), Fq::store(&y));
            for (i, &k) in self.pairs[j * LANES..].iter().take(LANES).enumerate() {
                points[k] = LimbPoint { x: x[i], y: y[i] };
            }
        }
        for (j, g) in self.records.iter().enumerate().rev() {
            let factor = g.prefix.mul(&inverse);
            inverse = inverse.mul(&g.denominator);
            let l = g.first.mul(&factor);
            let w = g.second.mul(&factor);
            let k = w.sub(&l);
            let xs = l.mul(&l).sub(&g.xsum);
            let x = k.mul(&k).sub(&xs).sub(&g.rx);
            let y = w.sum2(&xs.sub(&x), &l, &x.sub(&g.ax)).add(&g.ay);
            let (x, y) = (Fq::store(&x), Fq::store(&y));
            for (i, &k) in self.triples[j * LANES..].iter().take(LANES).enumerate() {
                points[k] = LimbPoint { x: x[i], y: y[i] };
            }
        }
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

struct Work {
    points: Vec<LimbPoint<Fq>>,
    counts: Vec<usize>,
    output: Vec<G1Affine>,
    binary: Scratch<G1Config>,
    ternary: Ternary,
}
impl Default for Work {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            counts: Vec::new(),
            output: Vec::new(),
            binary: Scratch::default(),
            ternary: Ternary::default(),
        }
    }
}
impl Work {
    fn bytes(&self) -> [usize; 4] {
        [
            self.points.capacity() * size_of::<LimbPoint<Fq>>()
                + self.counts.capacity() * size_of::<usize>()
                + self.output.capacity() * size_of::<G1Affine>(),
            self.binary.pairs.capacity() * size_of::<usize>()
                + self.binary.groups.capacity() * size_of::<Group<Fq>>()
                + self.binary.prefix_records.capacity() * size_of::<[FqBatch; 2]>(),
            self.ternary.bytes(),
            size_of::<Self>(),
        ]
    }
    fn reduce<const SELECTED: bool>(
        &mut self,
        bases: &[G1Affine],
        order: &[u32],
        starts: &[usize],
        counts: &[usize],
    ) {
        self.points.clear();
        self.points.reserve_exact(order.len());
        for &index in order {
            self.points.push(limbs::<G1Config>(bases[index as usize]));
        }
        self.counts.clear();
        self.counts.extend_from_slice(counts);
        while self.counts.iter().any(|&n| n > 1) {
            if SELECTED
                && self
                    .ternary
                    .layer(&mut self.points, starts, &mut self.counts)
            {
                continue;
            }
            reduce_layer::<G1Config>(&mut self.points, starts, &mut self.counts, &mut self.binary);
        }
        self.output.clear();
        for (&start, &n) in starts.iter().zip(&self.counts) {
            self.output.push(if n == 0 {
                G1Affine::IDENTITY
            } else {
                affine::<G1Config>(self.points[start])
            });
        }
    }
}

pub struct ReductionCaller {
    bases: Vec<G1Affine>,
    order: Vec<u32>,
    starts: Vec<usize>,
    counts: Vec<usize>,
    expected: Vec<G1Affine>,
    work: Work,
    last_bytes: [usize; 4],
}
impl ReductionCaller {
    pub fn new(counts: &[usize]) -> Self {
        let n: usize = counts.iter().sum();
        let values: Vec<_> = (0..n)
            .map(|i| Fr::from((1u64 << 48) * (i as u64 + 1) + 1800000054 + i as u64 * 137))
            .collect();
        let g = G1Projective::generator();
        let bases =
            G1Projective::batch_to_affine(&values.iter().map(|s| g * *s).collect::<Vec<_>>());
        Self::from_bases(bases, counts)
    }
    pub fn from_words(words: &[[u64; 4]], counts: &[usize]) -> Self {
        assert_eq!(words.len(), counts.iter().sum());
        let g = G1Projective::generator();
        let bases = G1Projective::batch_to_affine(
            &words
                .iter()
                .map(|w| g * Fr::from_canonical(*w).unwrap())
                .collect::<Vec<_>>(),
        );
        Self::from_bases(bases, counts)
    }
    fn from_bases(bases: Vec<G1Affine>, counts: &[usize]) -> Self {
        let n = bases.len();
        let mut order: Vec<u32> = (0..n as u32).collect();
        // A deterministic bijection, independent of either reduction algorithm.
        let mut state = 0x6a09e667f3bcc909u64;
        for i in (1..n).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            order.swap(i, state as usize % (i + 1));
        }
        let mut starts = Vec::new();
        let mut offset = 0;
        let mut expected = Vec::new();
        for &count in counts {
            starts.push(offset);
            let mut sum = G1Projective::IDENTITY;
            for &i in &order[offset..offset + count] {
                sum += bases[i as usize];
            }
            expected.push(sum.to_affine());
            offset += count;
        }
        Self {
            bases,
            order,
            starts,
            counts: counts.to_vec(),
            expected,
            work: Work::default(),
            last_bytes: [0; 4],
        }
    }
    pub fn run(&mut self, selected: bool, fresh: bool, rounds: usize) {
        if selected {
            self.run_impl::<true>(fresh, rounds)
        } else {
            self.run_impl::<false>(fresh, rounds)
        }
    }
    fn run_impl<const SELECTED: bool>(&mut self, fresh: bool, rounds: usize) {
        for _ in 0..rounds {
            let s = black_box(&mut *self);
            if fresh {
                let mut w = Work::default();
                w.reduce::<SELECTED>(&s.bases, &s.order, &s.starts, &s.counts);
                s.last_bytes = w.bytes();
                s.last_bytes[0] += s.work.output.capacity() * size_of::<G1Affine>();
                s.last_bytes[3] += size_of::<Work>();
                // Keep the just-computed output alive, without cloning it.
                std::mem::swap(&mut s.work.output, &mut w.output);
            } else {
                s.work
                    .reduce::<SELECTED>(&s.bases, &s.order, &s.starts, &s.counts);
                s.last_bytes = s.work.bytes();
            }
            black_box(&s.work.output);
        }
    }
    pub fn check(&self) {
        assert_eq!(self.work.output, self.expected);
    }
    pub fn order(&self) -> &[u32] {
        &self.order
    }
    pub fn scratch_bytes(&self) -> [usize; 4] {
        self.last_bytes
    }
    pub fn coordinates(&self) -> Vec<[[u64; 4]; 2]> {
        self.work
            .output
            .iter()
            .map(|p| [p.x.to_canonical(), p.y.to_canonical()])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_reduction_matches_projective_for_tails_and_exceptions() {
        for counts in [
            vec![],
            vec![0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26],
            vec![257],
            vec![384],
            vec![511],
            vec![2048],
            vec![31; 256],
        ] {
            let mut fixture = ReductionCaller::new(&counts);
            for selected in [false, true] {
                for fresh in [false, true] {
                    fixture.run(selected, fresh, 3);
                    fixture.check();
                }
            }
        }
        let values = [
            Fr::ZERO,
            Fr::ONE,
            -Fr::ONE,
            Fr::from(2),
            -Fr::from(2),
            Fr::from(3),
        ];
        for &a in &values {
            for &b in &values {
                for &c in &values {
                    let mut words = vec![Fr::from(17).to_canonical(); 48];
                    words[24] = a.to_canonical();
                    words[25] = b.to_canonical();
                    words[26] = c.to_canonical();
                    let mut fixture = ReductionCaller::from_words(&words, &[26, 22]);
                    for selected in [false, true] {
                        fixture.run(selected, false, 1);
                        fixture.check();
                    }
                }
            }
        }
    }
    #[test]
    fn guards_leave_inputs_and_counts_unchanged_before_binary_fallback() {
        let g = G1Projective::generator();
        for triple in [
            [Fr::ZERO, Fr::ONE, Fr::from(3)],
            [Fr::ONE, Fr::ONE, Fr::from(3)],
            [Fr::ONE, -Fr::ONE, Fr::from(3)],
            [Fr::ONE, Fr::from(2), Fr::from(3)],
            [Fr::ONE, Fr::from(2), -Fr::from(3)],
        ] {
            let mut values: Vec<_> = (1..=48).map(|i| Fr::from(i as u64 + 100)).collect();
            values[24..27].copy_from_slice(&triple);
            let original: Vec<_> = values
                .iter()
                .map(|v| limbs::<G1Config>((g * *v).to_affine()))
                .collect();
            let mut points = original.clone();
            let mut counts = [48];
            let mut t = Ternary::default();
            assert!(!t.layer(&mut points, &[0], &mut counts));
            assert_eq!(counts, [48]);
            assert!(points
                .iter()
                .zip(original)
                .all(|(a, b)| a.x == b.x && a.y == b.y));
        }
    }
}
