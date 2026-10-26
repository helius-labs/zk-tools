use rayon::prelude::*;
use zopro_bn254::Fr;

use crate::simd::FrLanes;

/// A quad: four consecutive elements of a transform, in lane form.
pub(crate) type Quad = FrLanes;
#[cfg(all(
    feature = "inplace-fft",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod buffer;
#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod pair;
#[cfg(all(
    feature = "inplace-fft",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) use buffer::Values;
#[cfg(not(all(
    feature = "inplace-fft",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
)))]
pub(crate) type Values = Vec<Quad>;
/// A GPU transform's tables: roots, inverse roots, forward and reverse
/// weights.
pub type Tables = (Vec<Fr>, Vec<Fr>, Vec<Fr>, Vec<Fr>);

/// One direction's weights as a GPU computes them, as the CPU does: the
/// weight of element i is chunks[i >> low_bits] * lows[i & (2^low_bits - 1)].
pub struct WeightTables {
    pub lows: Vec<Fr>,
    pub chunks: Vec<Fr>,
    pub low_bits: u32,
}

// Quads per parallel task in the wide passes (1024 elements).
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
const TASK: usize = 128;
#[cfg(not(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
)))]
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
    twisted: Option<(Twiddles, Weights)>,
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
    fn tables(&self) -> WeightTables {
        let lows = scalars(&self.lows, self.lows.len() * 4);
        WeightTables {
            low_bits: lows.len().trailing_zeros(),
            lows,
            chunks: self.chunks.iter().map(|chunk| chunk.get()[0]).collect(),
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
    #[cfg(all(zopro_deferred_dif32,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    deferred_dif32: bool,
    // Optional key-only roots in five-stage column order; empty uses layers.
    #[allow(dead_code)]
    columns: Vec<Vec<Quad>>,
    layers: Vec<Vec<Quad>>,
    pair: Quad,
    single: Option<Quad>,
    reservations: Vec<crate::preparation::Reservation>,
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
            #[cfg(all(zopro_deferred_dif32,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
            deferred_dif32: false,
            reservations: Vec::new(),
            columns: Vec::new(),
            layers,
            pair: Quad::new(&[layer[0], layer[1], layer[0], layer[1]]),
            single: None,
        }
    }
    fn empty() -> Self {
        Self {
            #[cfg(all(zopro_deferred_dif32,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
            deferred_dif32: false,
            reservations: Vec::new(),
            columns: Vec::new(),
            layers: Vec::new(),
            pair: Quad::one(),
            single: None,
        }
    }
    /// Each packed five-stage column reads31 roots. Store those roots
    /// together so large transforms read one contiguous region per column.
    #[allow(dead_code)]
    fn prepare_columns(&mut self) {
        if !self.columns.is_empty() {
            return;
        }
        let n = self.layers.first().map_or(0, |layer| layer.len().saturating_mul(8));
        // At most one extra root per existing root; allow collection scratch.
        let Some(reservation) = crate::preparation::Reservation::acquire(n, n.checked_mul(256)) else { return };
        let mut columns = vec![Vec::new(); self.layers.len()];
        for level in (0..self.layers.len()).step_by(5) {
            if level + 4 >= self.layers.len() {
                break;
            }
            let q = self.layers[level].len() / 16;
            if q <= 32 {
                break;
            }
            let roots: Vec<Quad> = (0..q)
                .into_par_iter()
                .flat_map_iter(|column| {
                    let mut roots = Vec::with_capacity(31);
                    for stage in 0..5 {
                        let gap = 16 >> stage;
                        for j in 0..gap {
                            roots.push(self.layers[level + stage][j * q + column]);
                        }
                    }
                    roots
                })
                .collect();
            assert_eq!(roots.len(), 31 * q);
            columns[level] = roots;
        }
        #[cfg(zopro_live_preparation_charge)]
        let reservation = reservation.retain(
            crate::preparation::allocation_bytes(&columns).and_then(|outer| {
                columns.iter().try_fold(outer, |bytes, roots| {
                    bytes.checked_add(crate::preparation::allocation_bytes(roots)?)
                })
            }),
        );
        self.columns = columns;
        self.reservations.push(reservation);
    }
    /// Fold coefficient weights g^i into the DIT butterfly twiddles.
    /// Layer k contributes g^(2^k) for the coefficient's kth bit.
    fn twisted(&self, mut factor: Fr) -> Self {
        let layers = self
            .layers
            .iter()
            .map(|layer| {
                let scale = Quad::new(&[factor; 4]);
                let layer = layer.par_iter().map(|root| root.mul(&scale)).collect();
                factor *= factor;
                layer
            })
            .collect();
        Self {
            #[cfg(all(zopro_deferred_dif32,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
            deferred_dif32: false,
            reservations: Vec::new(),
            columns: Vec::new(),
            layers,
            pair: self.pair.mul(&Quad::new(&[factor; 4])),
            single: Some(Quad::new(&[factor * factor; 4])),
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
fn into_quads(values: Vec<Fr>) -> Values {
    #[cfg(all(
        feature = "inplace-fft",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        Values::pack(values)
    }
    #[cfg(not(all(
        feature = "inplace-fft",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    {
        #[cfg(not(any(
            all(target_arch = "wasm32", target_feature = "simd128"),
            all(
                target_arch = "x86_64",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            )
        )))]
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
}

/// The elements of `quads`, as into_quads takes them.
fn from_quads(quads: Values) -> Vec<Fr> {
    #[cfg(all(
        feature = "inplace-fft",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        quads.unpack()
    }
    #[cfg(not(all(
        feature = "inplace-fft",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    {
        #[cfg(not(any(
            all(target_arch = "wasm32", target_feature = "simd128"),
            all(
                target_arch = "x86_64",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            )
        )))]
        {
            let mut quads = std::mem::ManuallyDrop::new(quads);
            let (ptr, len, capacity) = (quads.as_mut_ptr(), quads.len(), quads.capacity());
            // SAFETY: as into_quads.
            unsafe { Vec::from_raw_parts(ptr.cast::<Fr>(), len * 4, capacity * 4) }
        }
        #[cfg(any(
            all(target_arch = "wasm32", target_feature = "simd128"),
            all(
                target_arch = "x86_64",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            )
        ))]
        scalars(&quads, quads.len() * 4)
    }
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
                twisted: None,
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
        let mut roots = Twiddles::new(powers(generator, n / 2));
        let mut inverse_roots = Twiddles::new(powers(generator.inverse().unwrap(), n / 2));
        #[cfg(all(zopro_deferred_dif32,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
        {
            let enabled = std::env::var("ZOPRO_CPU_DEFERRED_DIF32").as_deref() == Ok("1");
            roots.deferred_dif32 = enabled;
            inverse_roots.deferred_dif32 = enabled;
        }
        let mut twisted = (n >= (1 << 20) && crate::simd::cpu_option("ZOPRO_CPU_TWISTED_FFT")
            || std::env::var("ZOPRO_CPU_TWISTED_FFT").is_ok_and(|v| v == "1"))
        .then(|| crate::preparation::Reservation::acquire(n, n.checked_mul(256)))
        .flatten()
        .map(|reservation| {
            let mut forward = roots.twisted(coset);
            forward.reservations.push(reservation);
            (
                forward,
                Weights::new(n, coset.inverse().unwrap(), n_inv * n_inv * n_inv * den),
            )
        });
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if n >= (1 << 18) && std::env::var("ZOPRO_CPU_FFT_COLUMN_ROOTS").as_deref() == Ok("1") {
            inverse_roots.prepare_columns();
            if let Some((forward, _)) = &mut twisted {
                forward.prepare_columns();
            } else {
                roots.prepare_columns();
            }
        }
        super::trace("plan-roots", started);
        Self {
            n,
            params: (generator, coset, den),
            released: false,
            roots,
            inverse_roots,
            forward_weights,
            reverse_weights,
            twisted,
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
            self.twisted = None;
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
    /// The roots and inverse roots of `tables`, and each direction's
    /// weights in their two small tables rather than n elements (32 MB for
    /// both at 2^19). None below eight elements.
    pub fn compact_tables(&self) -> Option<(Vec<Fr>, Vec<Fr>, WeightTables, WeightTables)> {
        if self.small.is_some() {
            return None;
        }
        Some((
            scalars(&self.roots.layers[0], self.n / 2),
            scalars(&self.inverse_roots.layers[0], self.n / 2),
            self.forward_weights.tables(),
            self.reverse_weights.tables(),
        ))
    }
    pub fn size(&self) -> usize {
        self.n
    }

    /// The coset evaluations of the polynomial with these domain evaluations.
    pub(crate) fn transform(&self, mut values: Vec<Fr>) -> Values {
        assert!(values.len() <= self.n);
        values.resize(self.n, Fr::ZERO);
        if let Some((direct, _)) = &self.small {
            return quads(&direct.coset_evaluations(&direct.coefficients(&values))).into();
        }
        let mut values = into_quads(values);
        dif(&mut values, &self.inverse_roots, 0);
        scale(&mut values, &self.forward_weights);
        dit(&mut values, &self.roots, 0);
        values
    }
    fn basis_weights(&self)->&Weights {
        #[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
        if crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT8") {if let Some((_,w))=&self.twisted{return w;}}
        &self.reverse_weights
    }
    pub(crate) fn basis_query_scale(&self)->Fr {
        self.basis_weights().chunks[0].get()[0]*self.reverse_weights.chunks[0].get()[0].inverse().unwrap()
    }
    pub(crate) fn basis_hash_weights(&self,hash:&mut impl FnMut(Fr)) {
        for q in &self.inverse_roots.layers[0]{for v in q.get(){hash(v);}}
        let weights=self.basis_weights();
        for c in &weights.chunks {let c=c.get()[0];for q in &weights.lows{for low in q.get(){hash(c*low);}}}
        hash(self.reverse_weights.chunks[0].get()[0]);
    }
    pub(crate) fn quotient_query_weights(&self) -> (Vec<Fr>, Vec<Fr>, Fr) {
        assert!(!self.released && self.n >= 8);
        let mut weights = &self.reverse_weights;
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT8") {
            weights = self
                .twisted
                .as_ref()
                .map(|(_, weights)| weights)
                .unwrap_or(weights);
        }
        (
            scalars(&self.inverse_roots.layers[0], self.n / 2),
            weights.expand(self.n),
            self.reverse_weights.chunks[0].get()[0],
        )
    }

    pub(crate) fn fused_product_output(&self) -> bool {
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            return pair::fused_output_enabled(self);
        }
        #[cfg(not(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        {
            false
        }
    }

    pub(crate) fn coset_product_ordered(&self, a: Vec<Fr>, b: Vec<Fr>, fused: bool) -> Vec<Fr> {
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if fused {
            return pair::fused_output_product(self, a, b);
        }
        debug_assert!(!fused);
        self.coset_product(a, b)
    }

    pub(crate) fn coset_product(&self, a: Vec<Fr>, b: Vec<Fr>) -> Vec<Fr> {
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT8") {
            return pair::coset_product(self, a, b);
        }
        let (a, b) = rayon::join(|| self.transform(a), || self.transform(b));
        a.par_iter()
            .zip(b.par_iter())
            .flat_map_iter(|(a, b)| a.mul(b).get())
            .collect()
    }

    /// The quotient's coefficients times the denominator, bit-reversed as
    /// gnark's Z key, the first n - 1, from the domain evaluations a, b, c
    /// (padded to n).
    pub fn quotient(&self, a: Vec<Fr>, b: Vec<Fr>, c: Vec<Fr>) -> Vec<Fr> {
        if self.released {
            let (generator, coset, den) = self.params;
            return Self::new(self.n, generator, coset, den).quotient(a, b, c);
        }
        #[cfg(all(
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if self.small.is_none() && crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT8") {
            return pair::quotient(self, a, b, c);
        }
        if self.small.is_none() && crate::simd::cpu_option("ZOPRO_CPU_QUOTIENT6") {
            return self.quotient_six(a, b, c);
        }
        let (mut a, (b, c)) = rayon::join(
            || self.transform(a),
            || rayon::join(|| self.transform(b), || self.transform(c)),
        );
        a.par_iter_mut()
            .zip(b.par_iter())
            .zip(c.par_iter())
            .for_each(|((a, b), c)| *a = a.mul(b).sub(c));
        let mut h = self.finish(a);
        h.pop();
        h
    }
    // C has degree below n, so its inverse coset transform is exactly its
    // ordinary coefficients. Keep those after its first inverse transform,
    // and subtract them after transforming A*B back. This saves C's forward
    // transform and coset weights without changing the key or polynomial.
    fn quotient_six(&self, a: Vec<Fr>, b: Vec<Fr>, mut c: Vec<Fr>) -> Vec<Fr> {
        assert!(self.small.is_none() && c.len() <= self.n);
        let (mut a, (b, c)) = rayon::join(
            || self.transform(a),
            || {
                rayon::join(
                    || self.transform(b),
                    || {
                        c.resize(self.n, Fr::ZERO);
                        let mut c = into_quads(c);
                        dif(&mut c, &self.inverse_roots, 0);
                        c
                    },
                )
            },
        );
        a.par_iter_mut()
            .zip(b.par_iter())
            .for_each(|(a, b)| *a = a.mul(b));
        drop(b);
        dif(&mut a, &self.inverse_roots, 0);
        scale(&mut a, &self.reverse_weights);
        // The inverse DIF is unnormalized. The first reverse-weight chunk
        // is n^-1 * den in every lane; C needs no inverse coset powers.
        let factor = &self.reverse_weights.chunks[0];
        a.par_iter_mut()
            .zip(c.par_iter())
            .for_each(|(a, c)| *a = a.sub(&c.mul(factor)));
        let mut h = from_quads(a);
        h.pop();
        h
    }
    /// The quotient's coefficients, times the denominator, in bit-reversed
    /// order, from its coset evaluations.
    pub(crate) fn finish(&self, mut values: Values) -> Vec<Fr> {
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
        let n_inv = Fr::from(values.len() as u64)
            .inverse()
            .expect("a nonzero size");
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

    /// Independent polynomial convolution, including reduction modulo the
    /// coset's X^n-coset^n. Arbitrary A/B/C need not satisfy a circuit.
    #[test]
    fn quotient_matches_polynomial_product() {
        for n in [1usize, 2, 4, 8, 16, 32, 256, 1024, 2048, 4096] {
            for root_power in [1, 3] {
                let generator = Fr::root_of_unity(n).unwrap().pow(&[root_power]);
                let coset = Fr::from(5 + root_power);
                let coset_n = coset.pow(&[n as u64]);
                let den = (coset_n - Fr::ONE).inverse().unwrap();
                let plan = Plan::new(n, generator, coset, den);
                let coefficients = |seed: u64| -> Vec<Fr> {
                    (0..n)
                        .map(|i| match (i as u64 + seed) % 5 {
                            0 => Fr::ZERO,
                            1 => Fr::ONE,
                            2 => -Fr::ONE,
                            _ => Fr::from(i as u64 + seed).pow(&[127]),
                        })
                        .collect()
                };
                let (a, b, c) = (coefficients(7), coefficients(19), coefficients(31));
                let mut product = vec![Fr::ZERO; 2 * n - 1];
                for (i, a) in a.iter().enumerate() {
                    for (j, b) in b.iter().enumerate() {
                        product[i + j] += *a * *b;
                    }
                }
                let coefficients: Vec<Fr> = (0..n)
                    .map(|i| {
                        (product[i] + product.get(n + i).copied().unwrap_or(Fr::ZERO) * coset_n
                            - c[i])
                            * den
                    })
                    .collect();
                let log = n.trailing_zeros() as usize;
                let expected: Vec<Fr> = (0..n - 1)
                    .map(|i| coefficients[bit_reverse(i, log)])
                    .collect();
                let domain = |v: &[Fr]| Direct::evaluate(v, generator, Fr::ONE);
                let (a, b, c) = (domain(&a), domain(&b), domain(&c));
                assert_eq!(
                    plan.quotient(a.clone(), b.clone(), c.clone()),
                    expected,
                    "legacy n={n}"
                );
                if n >= 8 {
                    assert_eq!(
                        plan.quotient_six(a.clone(), b.clone(), c.clone()),
                        expected,
                        "six n={n}"
                    );
                    #[cfg(all(
                        feature = "ifma8",
                        target_arch = "x86_64",
                        target_feature = "avx512ifma",
                        target_feature = "avx512vl",
                        target_feature = "avx512dq",
                        not(no_ifma)
                    ))]
                    {
                        type Quotient = fn(&Plan, Vec<Fr>, Vec<Fr>, Vec<Fr>) -> Vec<Fr>;
                        let paths: [Quotient; 4] = [
                            pair::quotient_impl::<1>,
                            pair::quotient_impl::<2>,
                            pair::quotient_impl::<3>,
                            pair::quotient_impl::<4>,
                        ];
                        for (depth, run) in paths.into_iter().enumerate() {
                            assert_eq!(
                                run(&plan, a.clone(), b.clone(), c.clone()),
                                expected,
                                "pair n={n} stages={}",
                                depth + 1
                            );
                        }
                    }
                }
            }
        }
    }

    /// Against the transforms by their definition.
    #[test]
    fn quotient_pads_independent_input_lengths() {
        for n in [8usize, 32, 256, 2048] {
            let generator = Fr::root_of_unity(n).unwrap();
            let coset = Fr::from(5u64);
            let den = (coset.pow(&[n as u64]) - Fr::ONE).inverse().unwrap();
            let plan = Plan::new(n, generator, coset, den);
            let direct = Direct { generator, coset };
            for lengths in [
                [0, 0, 0],
                [1, 3, 5],
                [n - 1, n / 2 + 1, n / 4 - 1],
                [n, 0, n - 3],
            ] {
                let inputs: [Vec<Fr>; 3] = lengths.map(|len| {
                    (0..len)
                        .map(|i| Fr::from(i as u64 + 7).pow(&[19]))
                        .collect()
                });
                let coefficients: [Vec<Fr>; 3] = inputs.clone().map(|mut values| {
                    values.resize(n, Fr::ZERO);
                    direct.coefficients(&values)
                });
                let [a, b, c] = coefficients;
                let mut expected = vec![Fr::ZERO; n];
                let coset_n = coset.pow(&[n as u64]);
                for (i, a) in a.iter().enumerate() {
                    for (j, b) in b.iter().enumerate() {
                        let degree = i + j;
                        expected[degree % n] +=
                            *a * *b * if degree < n { Fr::ONE } else { coset_n };
                    }
                }
                for (value, c) in expected.iter_mut().zip(c) {
                    *value = (*value - c) * den;
                }
                let expected: Vec<_> = (0..n - 1)
                    .map(|i| expected[bit_reverse(i, n.trailing_zeros() as usize)])
                    .collect();
                let [a, b, c] = inputs;
                assert_eq!(plan.quotient(a.clone(), b.clone(), c.clone()), expected);
                #[cfg(all(
                    feature = "ifma8",
                    target_arch = "x86_64",
                    target_feature = "avx512ifma",
                    target_feature = "avx512vl",
                    target_feature = "avx512dq",
                    not(no_ifma)
                ))]
                for run in [
                    pair::quotient_impl::<1>,
                    pair::quotient_impl::<2>,
                    pair::quotient_impl::<3>,
                    pair::quotient_impl::<4>,
                ] {
                    assert_eq!(
                        run(&plan, a.clone(), b.clone(), c.clone()),
                        expected,
                        "n={n} lengths={lengths:?}"
                    );
                }
            }
        }
    }

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
                        let actual = plan.finish(quads(&expected).into());
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

// Fused columns write32 groups of four fields per column. Queries use
// this same permutation; the original cache representation stays natural.
pub(crate) fn product_output_index(n: usize, output: usize) -> usize {
    let q = n / 128;
    let column = output / 128;
    let row = (output % 128) / 4;
    let lane = output % 4;
    (row * q + column) * 4 + lane
}

#[cfg(all(
    test,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod column_root_tests {
    use super::*;
    #[test]
    fn cached_columns_match_independent_quad_products() {
        for n in [8192usize, 262144, 1048576] {
            let generator = Fr::root_of_unity(n).unwrap().pow(&[3]);
            let coset = Fr::from(7);
            let den = denominator(n, coset).unwrap();
            let mut plan = Plan::new(n, generator, coset, den);
            for twisted in [false, true] {
                plan.roots.columns.clear();
                plan.inverse_roots.columns.clear();
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
                let a: Vec<_> = (0..n - 3)
                    .map(|i| Fr::from((i as u64 * 131 + 17) % 65537).square())
                    .collect();
                let b: Vec<_> = (0..n / 2 + 1)
                    .map(|i| Fr::from((i as u64 * 257 + 31) % 131071).square())
                    .collect();
                let left = plan.transform(a.clone());
                let right = plan.transform(b.clone());
                let factor = if twisted {
                    Fr::from(n as u64).square()
                } else {
                    Fr::ONE
                };
                let natural: Vec<_> = left
                    .iter()
                    .zip(right.iter())
                    .flat_map(|(a, b)| a.mul(b).get().map(|x| x * factor))
                    .collect();
                let expected: Vec<_> = (0..n)
                    .map(|i| natural[product_output_index(n, i)])
                    .collect();
                assert_eq!(
                    pair::fused_output_product(&plan, a.clone(), b.clone()),
                    expected
                );
                plan.inverse_roots.prepare_columns();
                if let Some((roots, _)) = &mut plan.twisted {
                    roots.prepare_columns();
                } else {
                    plan.roots.prepare_columns();
                }
                assert_eq!(
                    pair::fused_output_product(&plan, a, b),
                    expected,
                    "n={n}, twisted={twisted}"
                );
                assert_eq!(
                    pair::fused_output_product(&plan, Vec::new(), Vec::new()),
                    vec![Fr::ZERO; n]
                );
            }
        }
    }
}

#[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
impl Plan {
    /// Study-only conversion: release the coset-specific table before
    /// preparing ordinary-domain columns. This plan is then derivative-only.
    pub fn prepare_euler_study(&mut self) {
        assert!(!self.released && self.n >= 8);
        self.twisted = None;
        if self.n >= (1 << 18) && std::env::var("ZOPRO_CPU_FFT_COLUMN_ROOTS").as_deref() == Ok("1") {
            self.roots.prepare_columns();
        }
    }
    pub fn euler_product_study(&self,a:Vec<Fr>,b:Vec<Fr>)->Vec<Fr> {
        pair::euler_product_study(self,a,b)
    }
    pub fn current_product_study(&self,a:Vec<Fr>,b:Vec<Fr>)->Vec<Fr> {
        self.coset_product_ordered(a,b,false)
    }
    pub fn study_configuration(&self)->(bool,bool,usize,usize) {
        (false, self.twisted.is_some(), self.roots.columns.len(), self.inverse_roots.columns.len())
    }
}
#[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
#[cfg(test)]
mod euler_study_tests {
    use super::*;
    #[test]
    fn independent_euler_coefficients_and_partial_rows() {
        let mut count = 0;
        for n in [8usize,16,32,64,128] {
            for power in [1,3] {
                let root = Fr::root_of_unity(n).unwrap().pow(&[power]);
                let coset = Fr::from(5u64);
                let mut plan = Plan::new(n,root,coset,denominator(n,coset).unwrap());
                plan.prepare_euler_study();
                let direct = Direct{generator:root,coset:Fr::ONE};
                for (na,nb) in [(0,0),(1,1),(n/2,n/2+1),(n-3,n),(n,n)] {
                    for seed in 0..4u64 {
                        let make = |len:usize,s:u64|->Vec<Fr>{(0..len).map(|i|match (i as u64+s)%5 {
                            0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,_=>Fr::from(i as u64+s+2).pow(&[127])
                        }).collect()};
                        let a=make(na,seed+3);let b=make(nb,seed+19);
                        let derivative = |v:&[Fr]| {
                            let mut padded=v.to_vec();padded.resize(n,Fr::ZERO);
                            let mut coeff=direct.coefficients(&padded);
                            for (k,c) in coeff.iter_mut().enumerate(){*c *= Fr::from(k as u64);}
                            Direct::evaluate(&coeff,root,Fr::ONE)
                        };
                        let da=derivative(&a);let db=derivative(&b);
                        let expected:Vec<_>=(0..na.max(nb)).map(|i|Fr::from(n as u64)*
                            (da[i]*b.get(i).copied().unwrap_or(Fr::ZERO)+db[i]*a.get(i).copied().unwrap_or(Fr::ZERO))).collect();
                        assert_eq!(plan.euler_product_study(a,b),expected,"n={n} power={power} na={na} nb={nb} seed={seed}");
                        count+=1;
                    }
                }
            }
        }
        assert_eq!(count,200);println!("EULER_SMALL_IDENTITIES={count}");
    }
    #[test]
    fn euler_large_sparse_coefficient_oracle() {
        let mut count=0;
        for n in [8192usize,32768,65536,262144] {
            for power in [1,3] {
                let root=Fr::root_of_unity(n).unwrap().pow(&[power]);let coset=Fr::from(5u64);
                let mut plan=Plan::new(n,root,coset,denominator(n,coset).unwrap());plan.prepare_euler_study();
                let terms_a=[(0,Fr::ONE),(1,Fr::from(7u64)),(n/2+3,Fr::from(13u64)),(n-1,-Fr::ONE)];
                let terms_b=[(2,Fr::from(11u64)),(n/3,Fr::from(19u64)),(n-2,Fr::from(23u64))];
                let eval=|terms:&[(usize,Fr)]| {
                    let mut v=vec![Fr::ZERO;n];let mut d=vec![Fr::ZERO;n];
                    for &(k,c) in terms {let step=root.pow(&[k as u64]);let mut x=c;let f=Fr::from(k as u64);
                        for (v,d) in v.iter_mut().zip(&mut d){*v+=x;*d+=x*f;x*=step;}}
                    (v,d)
                };
                let (a,da)=eval(&terms_a);let (b,db)=eval(&terms_b);
                let expected:Vec<_>=a.iter().zip(&b).zip(&da).zip(&db).map(|(((a,b),da),db)|Fr::from(n as u64)*(*da**b+*a**db)).collect();
                assert_eq!(plan.euler_product_study(a,b),expected,"n={n} power={power}");count+=1;
            }
        }
        assert_eq!(count,8);println!("EULER_LARGE_IDENTITIES={count}");
    }
}


impl Plan {
    pub(crate) fn prepare_euler_basis(&mut self) {
        assert!(!self.released && self.n >= 8);
        self.twisted = None;
        if self.n >= (1 << 18) && std::env::var("ZOPRO_CPU_FFT_COLUMN_ROOTS").as_deref() == Ok("1") {
            self.roots.prepare_columns();
        }
    }
    /// N*(E(A)B+A*E(B)), in original domain order, E(f)=X*f'.
    pub(crate) fn euler_product_values(&self, a:Vec<Fr>, b:Vec<Fr>)->Vec<Fr> {
        assert!(!self.released && self.n>=8 && self.twisted.is_none());
        assert!(a.len()<=self.n && b.len()<=self.n);
        #[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
        { return pair::euler_product_study(self,a,b); }
        #[cfg(not(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma))))]
        {
            let count=a.len().max(b.len());if count==0{return a;}
            let derivative=|v:&[Fr]| {
                let mut values=v.to_vec();values.resize(self.n,Fr::ZERO);
                let mut values=into_quads(values);
                dif(&mut values,&self.inverse_roots,0);
                let log=self.n.trailing_zeros() as usize;
                values.par_iter_mut().enumerate().for_each(|(i,value)| {
                    let weight=Quad::new(&core::array::from_fn(|j|Fr::from(bit_reverse(4*i+j,log)as u64)));
                    *value=value.mul(&weight);
                });
                dit(&mut values,&self.roots,0);
                scalars(&values,self.n)
            };
            // Sequential transforms bound the portable path's live buffers.
            let mut da=derivative(&a);
            for i in 0..count{da[i]*=b.get(i).copied().unwrap_or(Fr::ZERO);}
            let db=derivative(&b);
            for i in 0..count{da[i]+=a.get(i).copied().unwrap_or(Fr::ZERO)*db[i];}
            da.truncate(count);da
        }
    }
    /// Original bit-reversed quotient coefficients become N^2*h(x_i).
    /// Their full N-point MSM against the stored Euler query preserves H
    /// for arbitrary A/B/C, including unsatisfied inputs and padded rows.
    pub(crate) fn original_h_euler_values(&self, mut h:Vec<Fr>)->Vec<Fr> {
        assert!(!self.released && h.len()+1==self.n && self.n>=8);
        h.push(Fr::ZERO);
        let mut values=into_quads(h);
        dit(&mut values,&self.roots,0);
        let factor=Quad::new(&[Fr::from(self.n as u64).square();4]);
        values.par_iter_mut().for_each(|value|*value=value.mul(&factor));
        scalars(&values,self.n)
    }
}

#[cfg(test)]
mod euler_basis_tests {
    use super::*;
    #[test]
    fn euler_product_matches_direct_coefficients_native_or_portable() {
        let mut count=0;
        for n in [8usize,32,128] {
            let root=Fr::root_of_unity(n).unwrap();let coset=Fr::from(5u64);
            let mut plan=Plan::new(n,root,coset,denominator(n,coset).unwrap());plan.prepare_euler_basis();
            let direct=Direct{generator:root,coset:Fr::ONE};
            for len in [0,n/2+1,n] {for seed in [1u64,7,19] {
                let a:Vec<_>=(0..len).map(|i|Fr::from(i as u64+seed).pow(&[13])).collect();
                let b:Vec<_>=(0..len).map(|i|-Fr::from(i as u64+seed+37).pow(&[17])).collect();
                let derivative=|v:&[Fr]|{let mut v=v.to_vec();v.resize(n,Fr::ZERO);let mut c=direct.coefficients(&v);for(k,c)in c.iter_mut().enumerate(){*c*=Fr::from(k as u64);}Direct::evaluate(&c,root,Fr::ONE)};
                let da=derivative(&a);let db=derivative(&b);let expected:Vec<_>=(0..len).map(|i|Fr::from(n as u64)*(da[i]*b[i]+a[i]*db[i])).collect();
                assert_eq!(plan.euler_product_values(a,b),expected);count+=1;
            }}
        }
        assert_eq!(count,27);println!("EULER_PORTABLE_IDENTITIES={count}");
    }
    #[test]
    fn euler_generic_h_preserves_arbitrary_coefficients() {
        let mut count=0;
        for n in [8usize,16,32,128,1024] {for power in [1,3] {
            let root=Fr::root_of_unity(n).unwrap().pow(&[power]);let coset=Fr::from(5u64);
            let mut plan=Plan::new(n,root,coset,denominator(n,coset).unwrap());plan.prepare_euler_basis();
            let mut coeff:Vec<_>=(0..n).map(|i|Fr::from(i as u64+13).pow(&[7])).collect();coeff[n-1]=Fr::ZERO;
            let raw:Vec<_>=(0..n-1).map(|i|coeff[bit_reverse(i,n.trailing_zeros()as usize)]).collect();
            let expected:Vec<_>=Direct::evaluate(&coeff,root,Fr::ONE).into_iter().map(|x|x*Fr::from(n as u64).square()).collect();
            assert_eq!(plan.original_h_euler_values(raw),expected);count+=1;
        }}
        assert_eq!(count,10);println!("EULER_GENERIC_H_IDENTITIES={count}");
    }
}
