use zopro_bn254::Fr;
use rayon::prelude::*;

use crate::simd::FrLanes;

/// A quad: four consecutive elements of a transform, in lane form.
pub(crate) type Quad = FrLanes;
/// A GPU transform's tables: roots, inverse roots, forward and reverse
/// weights.
pub type Tables = (Vec<Fr>, Vec<Fr>, Vec<Fr>, Vec<Fr>);

// Quads per parallel task in the wide passes (1024 elements).
const TASK: usize = 256;

// Cached twiddles and coset powers for a prepared proving key, in quads: a
// transform converts its values to lane form once, runs every layer on
// four elements at a time and converts back once. An inverse DIF transform
// emits bit-reversed coefficients; the forward DIT transform consumes that
// order directly. The final inverse transform already matches gnark's
// bit-reversed Z key, so no proof-time permutation is needed. Domains
// below eight elements, which only tests use, take the direct transform.
pub struct Plan {
    n: usize,
    /// The generator, coset generator and denominator the tables come from,
    /// to rebuild them after `release_tables`.
    params: (Fr, Fr, Fr),
    /// Whether the tables were released: a GPU holds its own copy.
    released: bool,
    roots: Twiddles,
    inverse_roots: Twiddles,
    forward_weights: Weights,
    reverse_weights: Weights,
    small: Option<(Direct, Fr)>,
}

/// A transform's weights, factor * base^bitrev(i) for element i, in two
/// small tables rather than a vector of n (32 MB for both directions at
/// 2^19): element i = c 2^low + t, the chunk c's 2^low elements, has
/// bitrev(i) = bitrev_low(t) 2^(log - low) + bitrev_(log - low)(c), so its
/// weight is chunks[c] * lows[t]. Scaling multiplies by both, one product
/// more per element.
struct Weights {
    /// base^(bitrev_low(t) 2^(log - low)), as quads.
    lows: Vec<Quad>,
    /// factor * base^bitrev_(log - low)(c), each in all four lanes.
    chunks: Vec<Quad>,
}
impl Weights {
    fn new(n: usize, base: Fr, factor: Fr) -> Self {
        let log = n.trailing_zeros() as usize;
        let low = log.min((4 * TASK).trailing_zeros() as usize);
        let lows = bit_reversed_powers(base.pow(&[1u64 << (log - low)]), low);
        let chunks = bit_reversed_powers(base, log - low)
            .into_iter()
            .map(|power| Quad::new(&[factor * power; 4]))
            .collect();
        Self {
            lows: quads(&lows),
            chunks,
        }
    }
    fn empty() -> Self {
        Self {
            lows: Vec::new(),
            chunks: Vec::new(),
        }
    }
    /// The n weights, for a GPU's tables.
    fn expand(&self, n: usize) -> Vec<Fr> {
        let lows = scalars(&self.lows, self.lows.len() * 4);
        let mut out = vec![Fr::ZERO; n];
        out.par_chunks_mut(lows.len())
            .zip(&self.chunks)
            .for_each(|(out, chunk)| {
                let chunk = chunk.get()[0];
                for (out, low) in out.iter_mut().zip(&lows) {
                    *out = chunk * *low;
                }
            });
        out
    }
}

/// base^bitrev_bits(i) for i below 2^bits.
fn bit_reversed_powers(base: Fr, bits: usize) -> Vec<Fr> {
    let mut powers = Vec::with_capacity(1 << bits);
    let mut power = Fr::ONE;
    for _ in 0..1usize << bits {
        powers.push(power);
        power *= base;
    }
    (0..1usize << bits)
        .map(|i| powers[bit_reverse(i, bits)])
        .collect()
}

fn bit_reverse(i: usize, bits: usize) -> usize {
    if bits == 0 {
        0
    } else {
        i.reverse_bits() >> (usize::BITS as usize - bits)
    }
}

// One direction's twiddles. layers[k] holds generator^(j·2^k), four j per
// quad, for the layers whose butterflies are four or more elements apart;
// `pair` is (1, t, 1, t) for the layer two apart; the layer one apart
// multiplies by one.
struct Twiddles {
    layers: Vec<Vec<Quad>>,
    pair: Quad,
}
impl Twiddles {
    fn new(mut layer: Vec<Fr>) -> Self {
        let mut layers = Vec::new();
        while layer.len() >= 4 {
            let next = layer.iter().step_by(2).copied().collect();
            layers.push(quads(&layer));
            layer = next;
        }
        debug_assert_eq!(layer.len(), 2);
        Self {
            layers,
            pair: Quad::new(&[layer[0], layer[1], layer[0], layer[1]]),
        }
    }
    fn empty() -> Self {
        Self {
            layers: Vec::new(),
            pair: Quad::one(),
        }
    }
}

/// `values` as quads, the last padded with zeros.
pub(crate) fn quads(values: &[Fr]) -> Vec<Quad> {
    values
        .par_chunks(4)
        .map(|chunk| {
            let mut four = [Fr::ZERO; 4];
            four[..chunk.len()].copy_from_slice(chunk);
            Quad::new(&four)
        })
        .collect()
}

/// `values` as quads, natively in place: there a quad's memory is its four
/// elements', so a transform holds its values once. A length or capacity
/// not a multiple of four, or WebAssembly's lane form, takes a copy.
fn into_quads(values: Vec<Fr>) -> Vec<Quad> {
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    if values.len().is_multiple_of(4) && values.capacity().is_multiple_of(4) {
        const _: () = assert!(
            std::mem::size_of::<Quad>() == 4 * std::mem::size_of::<Fr>()
                && std::mem::align_of::<Quad>() == std::mem::align_of::<Fr>()
        );
        let mut values = std::mem::ManuallyDrop::new(values);
        let (ptr, len, capacity) = (values.as_mut_ptr(), values.len(), values.capacity());
        // SAFETY: a quad is four elements in a row (Lanes, repr(transparent)
        // over [Fr; 4]), so the allocation holds len / 4 quads and its
        // layout is capacity / 4 quads'.
        return unsafe { Vec::from_raw_parts(ptr.cast::<Quad>(), len / 4, capacity / 4) };
    }
    quads(&values)
}

/// The elements of `quads`, as into_quads takes them.
fn from_quads(quads: Vec<Quad>) -> Vec<Fr> {
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
    {
        let mut quads = std::mem::ManuallyDrop::new(quads);
        let (ptr, len, capacity) = (quads.as_mut_ptr(), quads.len(), quads.capacity());
        // SAFETY: as into_quads.
        unsafe { Vec::from_raw_parts(ptr.cast::<Fr>(), len * 4, capacity * 4) }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    scalars(&quads, quads.len() * 4)
}

/// The first `n` elements of `quads`.
pub(crate) fn scalars(quads: &[Quad], n: usize) -> Vec<Fr> {
    let mut values: Vec<Fr> = quads.par_iter().flat_map_iter(|quad| quad.get()).collect();
    values.truncate(n);
    values
}

/// The quotient's denominator for a coset: 1 / (coset^n - 1).
pub fn denominator(n: usize, coset: Fr) -> Option<Fr> {
    (coset.pow(&[n as u64]) - Fr::ONE).inverse()
}

impl Plan {
    pub fn new(n: usize, generator: Fr, coset: Fr, den: Fr) -> Self {
        assert!(n.is_power_of_two());
        if n < 8 {
            return Self {
                n,
                params: (generator, coset, den),
                released: false,
                roots: Twiddles::empty(),
                inverse_roots: Twiddles::empty(),
                forward_weights: Weights::empty(),
                reverse_weights: Weights::empty(),
                small: Some((Direct { generator, coset }, den)),
            };
        }
        let powers = |base: Fr, len: usize| {
            let mut out = vec![Fr::ONE; len];
            out.par_chunks_mut(256).enumerate().for_each(|(i, chunk)| {
                let mut power = base.pow(&[(i * 256) as u64]);
                for value in chunk {
                    *value = power;
                    power *= base;
                }
            });
            out
        };
        let started = crate::time::Instant::now();
        let n_inv = Fr::from(n as u64).inverse().unwrap();
        let forward_weights = Weights::new(n, coset, n_inv);
        let reverse_weights = Weights::new(n, coset.inverse().unwrap(), n_inv * den);
        let started = super::trace("plan-weights", started);
        let roots = Twiddles::new(powers(generator, n / 2));
        let inverse_roots = Twiddles::new(powers(generator.inverse().unwrap(), n / 2));
        super::trace("plan-roots", started);
        Self {
            n,
            params: (generator, coset, den),
            released: false,
            roots,
            inverse_roots,
            forward_weights,
            reverse_weights,
            small: None,
        }
    }
    /// Drops the CPU tables once a GPU holds its own (160 MB at 2^20, which
    /// a phone counts against its memory limit); a CPU quotient after this
    /// rebuilds them for the one call.
    pub fn release_tables(&mut self) {
        if self.small.is_none() {
            self.roots = Twiddles::empty();
            self.inverse_roots = Twiddles::empty();
            self.forward_weights = Weights::empty();
            self.reverse_weights = Weights::empty();
            self.released = true;
        }
    }
    /// The tables a GPU transform needs: the first n/2 powers of the
    /// generator and of its inverse, and both bit-reversed weight vectors.
    /// None below eight elements.
    pub fn tables(&self) -> Option<Tables> {
        if self.small.is_some() {
            return None;
        }
        Some((
            scalars(&self.roots.layers[0], self.n / 2),
            scalars(&self.inverse_roots.layers[0], self.n / 2),
            self.forward_weights.expand(self.n),
            self.reverse_weights.expand(self.n),
        ))
    }
    pub fn size(&self) -> usize {
        self.n
    }

    /// The coset evaluations of the polynomial with these domain evaluations.
    pub(crate) fn transform(&self, mut values: Vec<Fr>) -> Vec<Quad> {
        assert!(values.len() <= self.n);
        values.resize(self.n, Fr::ZERO);
        if let Some((direct, _)) = &self.small {
            return quads(&direct.coset_evaluations(&direct.coefficients(&values)));
        }
        let mut values = into_quads(values);
        dif(&mut values, &self.inverse_roots, 0);
        scale(&mut values, &self.forward_weights);
        dit(&mut values, &self.roots, 0);
        values
    }
    /// The quotient's coefficients times the denominator, bit-reversed as
    /// gnark's Z key, the first n - 1, from the domain evaluations a, b, c
    /// (padded to n).
    pub fn quotient(&self, a: Vec<Fr>, b: Vec<Fr>, c: Vec<Fr>) -> Vec<Fr> {
        if self.released {
            let (generator, coset, den) = self.params;
            return Self::new(self.n, generator, coset, den).quotient(a, b, c);
        }
        let (mut a, (b, c)) = rayon::join(
            || self.transform(a),
            || rayon::join(|| self.transform(b), || self.transform(c)),
        );
        a.par_iter_mut()
            .zip(&b)
            .zip(&c)
            .for_each(|((a, b), c)| *a = a.mul(b).sub(c));
        let mut h = self.finish(a);
        h.pop();
        h
    }
    /// The quotient's coefficients, times the denominator, in bit-reversed
    /// order, from its coset evaluations.
    pub(crate) fn finish(&self, mut values: Vec<Quad>) -> Vec<Fr> {
        if let Some((direct, den)) = &self.small {
            let mut values = direct.coset_coefficients(&scalars(&values, self.n));
            for v in &mut values {
                *v *= *den;
            }
            let log = self.n.trailing_zeros();
            for i in 0..self.n {
                let j = if log == 0 {
                    0
                } else {
                    i.reverse_bits() >> (usize::BITS - log)
                };
                if i < j {
                    values.swap(i, j);
                }
            }
            return values;
        }
        assert_eq!(values.len() * 4, self.n);
        dif(&mut values, &self.inverse_roots, 0);
        scale(&mut values, &self.reverse_weights);
        from_quads(values)
    }
}

// values[i] *= the i-th weight; a task's quads are a weights chunk's.
fn scale(values: &mut [Quad], weights: &Weights) {
    values
        .par_chunks_mut(weights.lows.len())
        .zip(&weights.chunks)
        .for_each(|(values, chunk)| {
            for (value, low) in values.iter_mut().zip(&weights.lows) {
                *value = value.mul(low).mul(chunk);
            }
        });
}

// lo, hi = lo + hi, (lo - hi) * twiddle, quad by quad.
#[inline(always)]
fn dif_quads(lo: &mut [Quad], hi: &mut [Quad], twiddles: &[Quad]) {
    for j in 0..lo.len() {
        let difference = lo[j].sub(&hi[j]);
        lo[j] = lo[j].add(&hi[j]);
        hi[j] = difference.mul(&twiddles[j]);
    }
}

// lo, hi = lo + hi * twiddle, lo - hi * twiddle, quad by quad.
#[inline(always)]
fn dit_quads(lo: &mut [Quad], hi: &mut [Quad], twiddles: &[Quad]) {
    for j in 0..lo.len() {
        let product = hi[j].mul(&twiddles[j]);
        hi[j] = lo[j].sub(&product);
        lo[j] = lo[j].add(&product);
    }
}

// The layers two and one apart act within a quad: on two quads a and b at
// a time, the butterflies' two sides are gathered into one quad each,
// combined, and scattered back.
#[inline(always)]
fn dif_within(a: &mut Quad, b: &mut Quad, pair: &Quad) {
    // Two apart: (a0 a1 b0 b1) with (a2 a3 b2 b3), twiddles (1, t, 1, t).
    let (lo, hi) = (
        Quad::shuffle::<0, 1, 4, 5>(a, b),
        Quad::shuffle::<2, 3, 6, 7>(a, b),
    );
    let sum = lo.add(&hi);
    let difference = lo.sub(&hi).mul(pair);
    *a = Quad::shuffle::<0, 1, 4, 5>(&sum, &difference);
    *b = Quad::shuffle::<2, 3, 6, 7>(&sum, &difference);
    // One apart: (a0 a2 b0 b2) with (a1 a3 b1 b3), twiddle one.
    let (lo, hi) = (
        Quad::shuffle::<0, 2, 4, 6>(a, b),
        Quad::shuffle::<1, 3, 5, 7>(a, b),
    );
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Quad::shuffle::<0, 4, 1, 5>(&sum, &difference);
    *b = Quad::shuffle::<2, 6, 3, 7>(&sum, &difference);
}
#[inline(always)]
fn dit_within(a: &mut Quad, b: &mut Quad, pair: &Quad) {
    let (lo, hi) = (
        Quad::shuffle::<0, 2, 4, 6>(a, b),
        Quad::shuffle::<1, 3, 5, 7>(a, b),
    );
    let (sum, difference) = (lo.add(&hi), lo.sub(&hi));
    *a = Quad::shuffle::<0, 4, 1, 5>(&sum, &difference);
    *b = Quad::shuffle::<2, 6, 3, 7>(&sum, &difference);
    let (lo, hi) = (
        Quad::shuffle::<0, 1, 4, 5>(a, b),
        Quad::shuffle::<2, 3, 6, 7>(a, b),
    );
    let product = hi.mul(pair);
    let (sum, difference) = (lo.add(&product), lo.sub(&product));
    *a = Quad::shuffle::<0, 1, 4, 5>(&sum, &difference);
    *b = Quad::shuffle::<2, 3, 6, 7>(&sum, &difference);
}

// `level` selects the twiddle layer of the outermost butterflies.
fn dif(values: &mut [Quad], twiddles: &Twiddles, level: usize) {
    if values.len() > TASK {
        let mid = values.len() / 2;
        let (lo, hi) = values.split_at_mut(mid);
        lo.par_chunks_mut(TASK)
            .zip(hi.par_chunks_mut(TASK))
            .zip(twiddles.layers[level].par_chunks(TASK))
            .for_each(|((lo, hi), twiddles)| dif_quads(lo, hi, twiddles));
        rayon::join(
            || dif(lo, twiddles, level + 1),
            || dif(hi, twiddles, level + 1),
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
            dif_within(&mut a[0], &mut b[0], &twiddles.pair);
        }
    }
}
fn dit(values: &mut [Quad], twiddles: &Twiddles, level: usize) {
    if values.len() > TASK {
        let mid = values.len() / 2;
        let (lo, hi) = values.split_at_mut(mid);
        rayon::join(
            || dit(lo, twiddles, level + 1),
            || dit(hi, twiddles, level + 1),
        );
        lo.par_chunks_mut(TASK)
            .zip(hi.par_chunks_mut(TASK))
            .zip(twiddles.layers[level].par_chunks(TASK))
            .for_each(|((lo, hi), twiddles)| dit_quads(lo, hi, twiddles));
    } else {
        for pair in values.as_chunks_mut::<2>().0 {
            let (a, b) = pair.split_at_mut(1);
            dit_within(&mut a[0], &mut b[0], &twiddles.pair);
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

/// The transforms by their definition, for domains below eight elements:
/// the subgroup of `generator` and its coset by `coset`.
pub(crate) struct Direct {
    generator: Fr,
    coset: Fr,
}

impl Direct {
    fn powers(x: Fr, n: usize) -> Vec<Fr> {
        let mut out = Vec::with_capacity(n);
        let mut p = Fr::ONE;
        for _ in 0..n {
            out.push(p);
            p *= x;
        }
        out
    }

    /// Sum over i of values[i] point_i^j, for the points w^i scaled by
    /// `scale`.
    fn evaluate(values: &[Fr], root: Fr, scale: Fr) -> Vec<Fr> {
        let n = values.len();
        let roots = Self::powers(root, n);
        let scales = Self::powers(scale, n);
        (0..n)
            .map(|j| {
                (0..n)
                    .map(|i| values[i] * scales[i] * roots[(i * j) % n])
                    .fold(Fr::ZERO, |a, b| a + b)
            })
            .collect()
    }

    /// The coefficients of the polynomial with these evaluations over the
    /// subgroup.
    pub(crate) fn coefficients(&self, values: &[Fr]) -> Vec<Fr> {
        let n_inv = Fr::from(values.len() as u64).inverse().expect("a nonzero size");
        let inverse = self.generator.inverse().expect("a nonzero generator");
        Self::evaluate(values, inverse, Fr::ONE)
            .into_iter()
            .map(|c| c * n_inv)
            .collect()
    }

    /// The polynomial's evaluations over the coset.
    pub(crate) fn coset_evaluations(&self, coefficients: &[Fr]) -> Vec<Fr> {
        Self::evaluate(coefficients, self.generator, self.coset)
    }

    /// The coefficients of the polynomial with these coset evaluations.
    pub(crate) fn coset_coefficients(&self, values: &[Fr]) -> Vec<Fr> {
        let coset_inv = self.coset.inverse().expect("a nonzero coset");
        let mut c = self.coefficients(values);
        let mut scale = Fr::ONE;
        for v in &mut c {
            *v *= scale;
            scale *= coset_inv;
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Against the transforms by their definition.
    #[test]
    fn matches_direct_transforms() {
        for n in [1, 2, 4, 8, 16, 256, 1024] {
            for root_power in [1, 3] {
                let generator = Fr::root_of_unity(n).unwrap().pow(&[root_power]);
                for offset in [5, 7] {
                    let coset = Fr::from(offset as u64);
                    let direct = Direct { generator, coset };
                    let den = (coset.pow(&[n as u64]) - Fr::ONE).inverse().unwrap();
                    let plan = Plan::new(n, generator, coset, den);
                    for len in [0, n / 2, n] {
                        let input: Vec<Fr> = (0..len)
                            .map(|i| match i % 4 {
                                0 => Fr::ZERO,
                                1 => -Fr::ONE,
                                _ => Fr::from(i as u64 + 1).pow(&[127]),
                            })
                            .collect();
                        let mut padded = input.clone();
                        padded.resize(n, Fr::ZERO);
                        let expected = direct.coset_evaluations(&direct.coefficients(&padded));
                        assert_eq!(
                            scalars(&plan.transform(input), n),
                            expected,
                            "n {n} len {len}"
                        );
                        let actual = plan.finish(quads(&expected));
                        let coefficients: Vec<Fr> = direct
                            .coset_coefficients(&expected)
                            .into_iter()
                            .map(|v| v * den)
                            .collect();
                        let log = n.trailing_zeros();
                        for (i, value) in actual.iter().enumerate() {
                            let j = if log == 0 {
                                0
                            } else {
                                i.reverse_bits() >> (usize::BITS - log)
                            };
                            assert_eq!(*value, coefficients[j], "n {n} len {len} i {i}");
                        }
                    }
                }
            }
        }
    }
}
