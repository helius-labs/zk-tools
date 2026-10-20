use crate::simd::{FqBatch, FqLimbs, BATCH_LANES as LANES};
use rayon::prelude::*;
use zopro_bn254::{Affine, CurveConfig, Field, Fq, Fq2, Fr, Projective};
// Signed-window Pippenger. Reduce each bucket as an affine addition tree,
// sharing one inversion across all buckets at each level. Keep bucket storage
// contiguous: thousands of small allocations contend on the WASM allocator.
pub(super) fn msm<C: CurveConfig>(bases: &[Affine<C>], scalars: &[Fr]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), scalars.len(), "validated MSM dimensions");
    if bases.len() < 256 {
        return zopro_bn254::curve::msm(bases, scalars);
    }
    // Wider windows mean fewer digits per scalar, so fewer bucket additions,
    // at the cost of more buckets to sum per window; past 2^16 points the
    // additions dominate.
    let width: usize = match bases.len() {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        // 15 is the most i16 digits hold: a signed digit reaches 2^(width-1).
        _ => 15,
    };
    // Experiment: ZOPRO_MSM_WIDTH overrides the width past 2^16 points.
    #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
    let width = match std::env::var("ZOPRO_MSM_WIDTH") {
        Ok(value) if bases.len() >= 65536 => value.parse().unwrap(),
        _ => width,
    };
    // Leave room for the final carry when recoding into signed digits.
    let count = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    let radix = 1u64 << width;
    let mask = radix - 1;
    // Preserve indexed writes into one allocation. Collecting a flattened
    // parallel iterator creates temporary vectors and contends on the WASM heap.
    debug_assert!(width <= 15, "signed digits of this width overflow i16");
    let mut digits = vec![0i16; scalars.len() * count];
    // The point at infinity's digits stay zero: the windows skip it with
    // the zero digits, without a check of their own.
    digits
        .par_chunks_mut(count)
        .zip(scalars.par_iter().zip(bases))
        .for_each(|(digits, (s, base))| {
            if base.is_identity() {
                return;
            }
            let words = s.to_canonical();
            let mut carry = 0;
            for (i, digit) in digits.iter_mut().enumerate() {
                let offset = i * width;
                let word = offset / 64;
                let shift = offset % 64;
                let mut value = words[word] >> shift;
                if shift + width > 64 && word < 3 {
                    value |= words[word + 1] << (64 - shift);
                }
                let value = (value & mask) + carry;
                carry = (value + radix / 2) >> width;
                *digit = (value as i32 - (carry * radix) as i32) as i16;
            }
        });
    let window_sum = |i: usize| -> Projective<C> {
        let bucket_count = (radix / 2) as usize;
        let mut counts = vec![0usize; bucket_count];
        for digits in digits.chunks(count) {
            let digit = digits[i];
            if digit != 0 {
                counts[digit.unsigned_abs() as usize - 1] += 1;
            }
        }
        let mut starts = vec![0; bucket_count + 1];
        for j in 0..bucket_count {
            starts[j + 1] = starts[j] + counts[j];
        }
        let mut positions = starts[..bucket_count].to_vec();

        let mut points = vec![LimbPoint::<C::Base>::infinity(); starts[bucket_count]];
        for (base, digits) in bases.iter().zip(digits.chunks(count)) {
            let digit = digits[i];
            if digit != 0 {
                let j = digit.unsigned_abs() as usize - 1;
                // Never the point at infinity: its digits are zero.
                let point = if digit < 0 { -*base } else { *base };
                points[positions[j]] = LimbPoint {
                    x: C::Base::to_limbs(point.x),
                    y: C::Base::to_limbs(point.y),
                };
                positions[j] += 1;
            }
        }
        sum_buckets(&mut points, &starts, &mut counts)
    };
    // Each window sorts a copy of its points: all windows at once held up to
    // `count` copies (hundreds of MB for G2 on a wide key, counted against a
    // phone's memory limit). They go ZOPRO_MSM_WINDOWS at a time (default:
    // all natively, as measured best on a Mac; phones set fewer); each
    // window's bucket groups still run in parallel. In WebAssembly's 4 GB
    // heap the copies at once are held to COPIES_BUDGET: sixteen threads'
    // G2 copies of a 54 → 1 merge (2 GB) took the heap to 4052 MB, and one
    // allocation more trapped a worker and hung the proof.
    let copy = bases.len() * std::mem::size_of::<LimbPoint<C::Base>>();
    let default = if cfg!(target_arch = "wasm32") {
        (COPIES_BUDGET / copy.max(1)).clamp(1, count)
    } else {
        count
    };
    let at_once = std::env::var("ZOPRO_MSM_WINDOWS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
        .max(1);
    let windows: Vec<usize> = (0..count).collect();
    let sums: Vec<Projective<C>> = windows
        .chunks(at_once)
        .flat_map(|chunk| chunk.par_iter().map(|&i| window_sum(i)).collect::<Vec<_>>())
        .collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

const COPIES_BUDGET: usize = 512 << 20;

// Smaller bucket groups let idle workers help a long-running window and keep
// each affine inversion batch's working set small. Each task owns disjoint
// point and count slices; no shared writes or additional prepared key data.
fn sum_buckets<C: CurveConfig>(
    points: &mut [LimbPoint<C::Base>],
    starts: &[usize],
    counts: &mut [usize],
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    const GROUP: usize = 256;
    if counts.len() <= GROUP || rayon::current_num_threads() == 1 {
        return sum_bucket_group(points, starts, counts).0;
    }
    let mut tasks = Vec::with_capacity(counts.len() / GROUP);
    let mut remaining = points;
    for (i, counts) in counts.chunks_mut(GROUP).enumerate() {
        let offset = starts[i * GROUP];
        let size = starts[(i + 1) * GROUP] - offset;
        let (chunk, rest) = remaining.split_at_mut(size);
        remaining = rest;
        let starts: Vec<_> = starts[i * GROUP..(i + 1) * GROUP]
            .iter()
            .map(|s| s - offset)
            .collect();
        tasks.push((chunk, starts, counts));
    }
    let partials: Vec<_> = tasks
        .into_par_iter()
        .map(|(points, starts, counts)| sum_bucket_group(points, &starts, counts))
        .collect();
    // Local weights restart at one. Group i therefore needs an additional
    // i * GROUP times its unweighted total. Reverse running sums compute
    // those offsets without a separate scalar multiplication for each group.
    let mut sum = Projective::<C>::IDENTITY;
    let mut offset = Projective::<C>::IDENTITY;
    let mut running = Projective::<C>::IDENTITY;
    for (weighted, total) in partials.into_iter().rev() {
        sum += weighted;
        offset += running;
        running += total;
    }
    for _ in 0..GROUP.ilog2() {
        offset = offset.double();
    }
    sum + offset
}

/// Sum each scalar-equivalence class independently. Only worker-local
/// groups hold point copies; the global ordering uses four-byte indices.
pub(super) fn group_affine<C: CurveConfig>(
    bases: &[Affine<C>],
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    const GROUP: usize = 256;
    let groups: Vec<Vec<Affine<C>>> = counts
        .par_chunks(GROUP)
        .enumerate()
        .map(|(i, counts)| {
            let first = i * GROUP;
            let offset = starts[first];
            let end = starts[first + counts.len()];
            let mut points: Vec<_> = order[offset..end]
                .iter()
                .map(|&index| limbs::<C>(bases[index as usize]))
                .collect();
            let starts: Vec<_> = starts[first..first + counts.len()]
                .iter()
                .map(|s| s - offset)
                .collect();
            let mut counts = counts.to_vec();
            let mut scratch = Scratch::<C>::default();
            while counts.iter().any(|&count| count > 1) {
                reduce_layer(&mut points, &starts, &mut counts, &mut scratch);
            }
            starts
                .iter()
                .zip(counts)
                .map(|(&start, count)| {
                    if count == 0 {
                        Affine::IDENTITY
                    } else {
                        affine::<C>(points[start])
                    }
                })
                .collect()
        })
        .collect();
    groups.into_iter().flatten().collect()
}

/// Reduce high-occupancy digit buckets in independent point tiles first.
/// The second tree sees only tile sums, keeping thin final windows in SIMD.
pub(super) fn reduce_indexed_buckets<C: CurveConfig>(
    bases: &[Affine<C>],
    order: &[u32],
    starts: &[usize],
) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    let tile = 2048 / (core::mem::size_of::<C::Base>() / 32);
    let mut tasks = Vec::new();
    let mut partial_starts = Vec::with_capacity(starts.len());
    partial_starts.push(0);
    for range in starts.windows(2) {
        for start in (range[0]..range[1]).step_by(tile) {
            tasks.push((start, (start + tile).min(range[1])));
        }
        partial_starts.push(tasks.len());
    }
    let partials: Vec<Affine<C>> = tasks
        .into_par_iter()
        .map(|(first, end)| {
            let mut points: Vec<_> = order[first..end]
                .iter()
                .map(|&i| limbs::<C>(bases[i as usize]))
                .collect();
            let mut count = [points.len()];
            let mut scratch = Scratch::<C>::default();
            while count[0] > 1 {
                reduce_layer(&mut points, &[0], &mut count, &mut scratch);
            }
            affine::<C>(points[0])
        })
        .collect();
    let counts: Vec<_> = partial_starts.windows(2).map(|r| r[1] - r[0]).collect();
    let partial_order: Vec<_> = (0..partials.len() as u32).collect();
    group_affine(&partials, &partial_order, &partial_starts, &counts)
}

/// Reduces already accumulated affine buckets through the same weighted
/// affine tree used by the lane MSM, splitting work into cache-sized groups.
pub(super) fn weighted_affine<C: CurveConfig>(buckets: &[Affine<C>]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    const GROUP: usize = 256;
    let partials: Vec<_> = buckets
        .par_chunks(GROUP)
        .map(|buckets| {
            let mut points: Vec<_> = buckets.iter().copied().map(limbs::<C>).collect();
            let starts: Vec<_> = (0..buckets.len()).collect();
            let mut counts = vec![1; buckets.len()];
            sum_bucket_group(&mut points, &starts, &mut counts)
        })
        .collect();
    let mut sum = Projective::<C>::IDENTITY;
    let mut offset = Projective::<C>::IDENTITY;
    let mut running = Projective::<C>::IDENTITY;
    for (weighted, total) in partials.into_iter().rev() {
        sum += weighted;
        offset += running;
        running += total;
    }
    for _ in 0..GROUP.ilog2() {
        offset = offset.double();
    }
    sum + offset
}

fn sum_bucket_group<C: CurveConfig>(
    points: &mut [LimbPoint<C::Base>],
    starts: &[usize],
    counts: &mut [usize],
) -> (Projective<C>, Affine<C>)
where
    C::Base: BatchInverse,
{
    let mut scratch = Scratch::default();
    while counts.iter().any(|n| *n > 1) {
        reduce_layer(points, starts, counts, &mut scratch);
    }
    let n = counts.len();
    let mut buckets = vec![LimbPoint::<C::Base>::infinity(); n * 2 - 1];
    for j in 0..n {
        if counts[j] != 0 {
            buckets[j] = points[starts[j]];
        }
    }
    let weighted = weighted_sum(&mut buckets, n, &mut scratch);
    (weighted, affine::<C>(buckets[0]))
}

// A bucket point, coordinates in limb form; the identity is (0, 0), which
// is not on either curve.
#[derive(Clone, Copy)]
struct LimbPoint<F: BatchInverse> {
    x: F::Limbs,
    y: F::Limbs,
}
impl<F: BatchInverse> LimbPoint<F> {
    #[inline(always)]
    fn infinity() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
    #[inline(always)]
    fn is_infinity(&self) -> bool {
        F::limbs_zero(&self.x) && F::limbs_zero(&self.y)
    }
}
#[inline(always)]
fn limbs<C: CurveConfig>(point: Affine<C>) -> LimbPoint<C::Base>
where
    C::Base: BatchInverse,
{
    if point.is_identity() {
        LimbPoint::infinity()
    } else {
        LimbPoint {
            x: C::Base::to_limbs(point.x),
            y: C::Base::to_limbs(point.y),
        }
    }
}
#[inline(always)]
fn affine<C: CurveConfig>(point: LimbPoint<C::Base>) -> Affine<C>
where
    C::Base: BatchInverse,
{
    if point.is_infinity() {
        Affine::IDENTITY
    } else {
        Affine::new(C::Base::from_limbs(point.x), C::Base::from_limbs(point.y))
    }
}

// Buffers of one layer, reused across the layers of a bucket group.
struct Scratch<C: CurveConfig>
where
    C::Base: BatchInverse,
{
    // The first point of each pair the affine formula adds, in bucket order.
    pairs: Vec<usize>,
    // Groups of four pairs in lane form, then the scalar tail.
    groups: Vec<Group<C::Base>>,
    denominators: Vec<C::Base>,
    products: Vec<<C::Base as BatchInverse>::Product>,
    sums: Vec<LimbPoint<C::Base>>,
}
// Lane groups retain the scaled numerator instead of every input coordinate
// and prefix. The reverse pass obtains the slope directly from the norm inverse.
#[derive(Clone, Copy)]
struct Group<F: BatchInverse> {
    ax: F::Lanes,
    ay: F::Lanes,
    xsum: F::Lanes,
    norm: FqBatch,
    numerator: F::Lanes,
}
impl<C: CurveConfig> Default for Scratch<C>
where
    C::Base: BatchInverse,
{
    fn default() -> Self {
        Self {
            pairs: Vec::new(),
            groups: Vec::new(),
            denominators: Vec::new(),
            products: Vec::new(),
            sums: Vec::new(),
        }
    }
}

// Adds each bucket's points in pairs. Pair k joins prefix chain k % LANES: four
// chains of one batch inversion, so that a group of four pairs runs every
// operation on four lanes, in lane form from loading its coordinates to its
// sums. Pairs with an identity, a doubling or opposite points take the
// complete formulas instead.
fn reduce_layer<C: CurveConfig>(
    points: &mut [LimbPoint<C::Base>],
    starts: &[usize],
    counts: &mut [usize],
    s: &mut Scratch<C>,
) where
    C::Base: BatchInverse,
{
    s.pairs.clear();
    for index in 0..counts.len() {
        let offset = starts[index];
        for j in (0..counts[index].saturating_sub(1)).step_by(2) {
            let (a, b) = (&points[offset + j], &points[offset + j + 1]);
            if !a.is_infinity() && !b.is_infinity() && a.x != b.x {
                s.pairs.push(offset + j);
            }
        }
    }
    type F<C> = <C as CurveConfig>::Base;
    let m = s.pairs.len();
    let full = m / LANES * LANES;
    s.groups.clear();
    let mut chains = FqBatch::one();
    for k in (0..full).step_by(LANES) {
        let a: [&LimbPoint<F<C>>; LANES] = core::array::from_fn(|i| &points[s.pairs[k + i]]);
        let b: [&LimbPoint<F<C>>; LANES] = core::array::from_fn(|i| &points[s.pairs[k + i] + 1]);
        let (ax, ay) = (F::<C>::load(a.map(|p| &p.x)), F::<C>::load(a.map(|p| &p.y)));
        let (bx, by) = (F::<C>::load(b.map(|p| &p.x)), F::<C>::load(b.map(|p| &p.y)));
        let denominator = F::<C>::lanes_sub(&bx, &ax);
        let norm = F::<C>::lanes_norm(&denominator);
        let numerator =
            F::<C>::lanes_scaled_numerator(&F::<C>::lanes_sub(&by, &ay), &denominator, &chains);
        chains = chains.mul(&norm);
        s.groups.push(Group {
            ax,
            ay,
            xsum: F::<C>::lanes_add(&ax, &bx),
            norm,
            numerator,
        });
    }
    s.denominators.clear();
    s.products.clear();
    let mut tail = chains.get();
    for k in full..m {
        let (a, b) = (
            affine::<C>(points[s.pairs[k]]),
            affine::<C>(points[s.pairs[k] + 1]),
        );
        let denominator = b.x - a.x;
        s.denominators.push(denominator);
        s.products
            .push(F::<C>::prefix(&mut tail[k % LANES], denominator));
    }
    let mut inverses = invert_lanes(tail);
    for k in (full..m).rev() {
        let t = k - full;
        F::<C>::reverse(
            &mut inverses[k % LANES],
            s.denominators[t],
            &mut s.products[t],
        );
    }
    let mut inverses = FqBatch::new(&inverses);
    s.sums.clear();
    s.sums.resize(m, LimbPoint::infinity());
    for (j, group) in s.groups.iter().enumerate().rev() {
        let slope = F::<C>::lanes_scale(&group.numerator, &inverses);
        inverses = inverses.mul(&group.norm);
        let x = F::<C>::lanes_sub(&F::<C>::lanes_square(&slope), &group.xsum);
        let y = F::<C>::lanes_sub(
            &F::<C>::lanes_mul(&slope, &F::<C>::lanes_sub(&group.ax, &x)),
            &group.ay,
        );
        let (x, y) = (F::<C>::store(&x), F::<C>::store(&y));
        for i in 0..LANES {
            s.sums[j * LANES + i] = LimbPoint { x: x[i], y: y[i] };
        }
    }
    for k in full..m {
        let t = k - full;
        let (a, b) = (
            affine::<C>(points[s.pairs[k]]),
            affine::<C>(points[s.pairs[k] + 1]),
        );
        let slope = (b.y - a.y) * F::<C>::denominator_inverse(s.denominators[t], s.products[t]);
        let x = slope.square() - a.x - b.x;
        let y = slope * (a.x - x) - a.y;
        s.sums[k] = limbs::<C>(Affine::new(x, y));
    }
    // Each bucket keeps its pair sums in front, in order. A write lands
    // before the pairs still to be read.
    let mut cursor = 0;
    for index in 0..counts.len() {
        let offset = starts[index];
        let n = counts[index];
        for j in 0..n / 2 {
            points[offset + j] = if s.pairs.get(cursor) == Some(&(offset + 2 * j)) {
                cursor += 1;
                s.sums[cursor - 1]
            } else {
                let (a, b) = (points[offset + 2 * j], points[offset + 2 * j + 1]);
                if a.is_infinity() {
                    b
                } else if b.is_infinity() {
                    a
                } else {
                    // Doubling and opposite points need the complete formulas.
                    limbs::<C>((affine::<C>(a) + affine::<C>(b)).to_affine())
                }
            };
        }
        if !n.is_multiple_of(2) {
            points[offset + n / 2] = points[offset + n - 1];
        }
        counts[index] = n.div_ceil(2);
    }
}

// The inverses of four chains with one field inversion: inverting is far
// dearer than the few products that share it. Every denominator is
// nonzero, so is every chain.
fn invert_lanes(c: [Fq; LANES]) -> [Fq; LANES] {
    let mut prefixes = [Fq::ONE; LANES];
    let mut product = Fq::ONE;
    for i in 0..LANES {
        prefixes[i] = product;
        product *= c[i];
    }
    let mut inverse = product.inverse().unwrap();
    let mut result = [Fq::ZERO; LANES];
    for i in (0..LANES).rev() {
        result[i] = inverse * prefixes[i];
        inverse *= c[i];
    }
    result
}

/// Reuses the lane-form affine addition kernel for a streaming Pippenger
/// batch. The pair storage and inversion scratch persist across batches.
pub(super) struct BatchAdd<C: CurveConfig>
where
    C::Base: BatchInverse,
{
    points: Vec<LimbPoint<C::Base>>,
    starts: Vec<usize>,
    counts: Vec<usize>,
    scratch: Scratch<C>,
    direct: bool,
    fq: FqAddScratch,
    fq2: Fq2AddScratch,
}
impl<C: CurveConfig> Default for BatchAdd<C>
where
    C::Base: BatchInverse,
{
    fn default() -> Self {
        Self {
            points: Vec::new(),
            starts: Vec::new(),
            counts: Vec::new(),
            scratch: Scratch::default(),
            direct: crate::simd::cpu_option("ZOPRO_CPU_DIRECT"),
            fq: FqAddScratch::default(),
            fq2: Fq2AddScratch::default(),
        }
    }
}
impl<C: CurveConfig> BatchAdd<C>
where
    C::Base: BatchInverse,
{
    pub(super) fn apply(&mut self, buckets: &mut [Affine<C>], batch: &[(u32, Affine<C>)]) {
        if self.direct {
            C::Base::stream_add(buckets, batch, self);
        } else {
            self.apply_generic(buckets, batch);
        }
    }
    fn apply_generic(&mut self, buckets: &mut [Affine<C>], batch: &[(u32, Affine<C>)]) {
        self.points.clear();
        self.starts.clear();
        self.counts.clear();
        for (j, (index, point)) in batch.iter().enumerate() {
            self.starts.push(j * 2);
            self.counts.push(2);
            self.points.push(limbs::<C>(buckets[*index as usize]));
            self.points.push(limbs::<C>(*point));
        }
        reduce_layer(
            &mut self.points,
            &self.starts,
            &mut self.counts,
            &mut self.scratch,
        );
        for (j, (index, _)) in batch.iter().enumerate() {
            buckets[*index as usize] = affine::<C>(self.points[j * 2]);
        }
    }
}

#[derive(Clone, Copy)]
struct FqAddGroup {
    ax: FqBatch,
    ay: FqBatch,
    xsum: FqBatch,
    denominator: FqBatch,
    numerator: FqBatch,
}
#[derive(Default)]
struct FqAddScratch {
    groups: Vec<FqAddGroup>,
    tail: Vec<[Fq; 5]>,
}
fn stream_g1<C: CurveConfig<Base = Fq>>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, Affine<C>)],
    s: &mut FqAddScratch,
) {
    let full = batch.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.groups.clear();
    for k in (0..full).step_by(LANES) {
        let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch[k + i].0 as usize]);
        let b: [Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = FqBatch::new(&a.map(|p| p.x));
        let ay = FqBatch::new(&a.map(|p| p.y));
        let bx = FqBatch::new(&b.map(|p| p.x));
        let by = FqBatch::new(&b.map(|p| p.y));
        let denominator = bx.sub(&ax);
        let numerator = by.sub(&ay).mul(&chains);
        chains = chains.mul(&denominator);
        s.groups.push(FqAddGroup {
            ax,
            ay,
            xsum: ax.add(&bx),
            denominator,
            numerator,
        });
    }
    let mut tail = chains.get();
    s.tail.clear();
    for k in full..batch.len() {
        let (index, b) = batch[k];
        let a = buckets[index as usize];
        let denominator = b.x - a.x;
        let numerator = (b.y - a.y) * tail[k % LANES];
        tail[k % LANES] *= denominator;
        s.tail.push([a.x, a.y, a.x + b.x, denominator, numerator]);
    }
    let mut inverse = invert_lanes(tail);
    for k in (full..batch.len()).rev() {
        let [ax, ay, xsum, denominator, numerator] = s.tail[k - full];
        let slope = numerator * inverse[k % LANES];
        inverse[k % LANES] *= denominator;
        let x = slope.square() - xsum;
        let y = slope * (ax - x) - ay;
        buckets[batch[k].0 as usize] = Affine::new(x, y);
    }
    let mut inverse = FqBatch::new(&inverse);
    for (j, g) in s.groups.iter().enumerate().rev() {
        let slope = g.numerator.mul(&inverse);
        inverse = inverse.mul(&g.denominator);
        let x = slope.mul(&slope).sub(&g.xsum);
        let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);
        let (x, y) = (x.get(), y.get());
        for i in 0..LANES {
            buckets[batch[j * LANES + i].0 as usize] = Affine::new(x[i], y[i]);
        }
    }
}

#[derive(Clone, Copy)]
struct Fq2AddGroup {
    ax: [FqBatch; 2],
    ay: [FqBatch; 2],
    xsum: [FqBatch; 2],
    norm: FqBatch,
    numerator: [FqBatch; 2],
}
#[derive(Default)]
struct Fq2AddScratch {
    groups: Vec<Fq2AddGroup>,
    tail: Vec<[Fq2; 5]>,
}
#[inline(always)]
fn fq2_pack(values: [Fq2; LANES]) -> [FqBatch; 2] {
    [
        FqBatch::new(&values.map(|v| v.c0)),
        FqBatch::new(&values.map(|v| v.c1)),
    ]
}
#[inline(always)]
fn fq2_unpack(values: [FqBatch; 2]) -> [Fq2; LANES] {
    let (c0, c1) = (values[0].get(), values[1].get());
    core::array::from_fn(|i| Fq2::new(c0[i], c1[i]))
}
fn stream_g2<C: CurveConfig<Base = Fq2>>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, Affine<C>)],
    s: &mut Fq2AddScratch,
) {
    let full = batch.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.groups.clear();
    for k in (0..full).step_by(LANES) {
        let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch[k + i].0 as usize]);
        let b: [Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = fq2_pack(a.map(|p| p.x));
        let ay = fq2_pack(a.map(|p| p.y));
        let bx = fq2_pack(b.map(|p| p.x));
        let by = fq2_pack(b.map(|p| p.y));
        let denominator = Fq2::lanes_sub(&bx, &ax);
        let norm = Fq2::lanes_norm(&denominator);
        let dy = Fq2::lanes_sub(&by, &ay);
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        let numerator = [
            dy[0].sum2(&denominator[0], &dy[1], &denominator[1]),
            dy[1].difference2(&denominator[0], &dy[0], &denominator[1]),
        ];
        #[cfg(not(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        let numerator = {
            let conjugate = [denominator[0], FqBatch::zero().sub(&denominator[1])];
            Fq2::lanes_mul(&dy, &conjugate)
        };
        let numerator = [numerator[0].mul(&chains), numerator[1].mul(&chains)];
        chains = chains.mul(&norm);
        let xsum = [ax[0].add(&bx[0]), ax[1].add(&bx[1])];
        s.groups.push(Fq2AddGroup {
            ax,
            ay,
            xsum,
            norm,
            numerator,
        });
    }
    let mut tail = chains.get();
    s.tail.clear();
    for k in full..batch.len() {
        let (index, b) = batch[k];
        let a = buckets[index as usize];
        let denominator = b.x - a.x;
        let norm = denominator.c0.square() + denominator.c1.square();
        let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
        tail[k % LANES] *= norm;
        s.tail
            .push([a.x, a.y, a.x + b.x, Fq2::new(norm, Fq::ZERO), numerator]);
    }
    let mut inverse = invert_lanes(tail);
    for k in (full..batch.len()).rev() {
        let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
        let slope = numerator.mul_by_fp(&inverse[k % LANES]);
        inverse[k % LANES] *= norm.c0;
        let x = slope.square() - xsum;
        let y = slope * (ax - x) - ay;
        buckets[batch[k].0 as usize] = Affine::new(x, y);
    }
    let mut inverse = FqBatch::new(&inverse);
    for (j, g) in s.groups.iter().enumerate().rev() {
        let slope = [g.numerator[0].mul(&inverse), g.numerator[1].mul(&inverse)];
        inverse = inverse.mul(&g.norm);
        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &g.xsum);
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&g.ax, &x)), &g.ay);
        let (x, y) = (fq2_unpack(x), fq2_unpack(y));
        for i in 0..LANES {
            buckets[batch[j * LANES + i].0 as usize] = Affine::new(x[i], y[i]);
        }
    }
}

// W(B) = 2*W(B[0]+B[1], B[2]+B[3], ...) - sum(B[0], B[2], ...).
// Reduce the odd-weight sums from all preceding levels together with the
// current paired buckets, sharing one inverse per level.
fn weighted_sum<C: CurveConfig>(
    points: &mut [LimbPoint<C::Base>],
    n: usize,
    scratch: &mut Scratch<C>,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    debug_assert!(n.is_power_of_two());
    debug_assert_eq!(points.len(), 2 * n - 1);
    let mut starts = Vec::with_capacity(n.ilog2() as usize + 1);
    let mut counts = Vec::with_capacity(n.ilog2() as usize + 1);
    starts.push(0);
    counts.push(n);
    let mut next = n;
    while counts[0] > 1 {
        let half = counts[0] / 2;
        for i in 0..half {
            points[next + i] = points[2 * i];
        }
        reduce_layer(points, &starts, &mut counts, scratch);
        starts.push(next);
        counts.push(half);
        next += half;
    }
    let mut sum = affine::<C>(points[0]).to_projective();
    for start in starts[1..].iter().rev() {
        sum = sum.double();
        sum -= affine::<C>(points[*start]);
    }
    sum
}

// In Fq2 = Fq[u]/(u^2 + 1), invert x + y*u using
// (x - y*u)/(x^2 + y^2). Batch the norms in Fq instead of multiplying
// Fq2 prefix products. Keep the norm beside each prefix so the reverse
// pass does not compute it again. Exceptional pairs are excluded above.

pub(super) trait BatchInverse: Field {
    fn stream_add<C: CurveConfig<Base = Self>>(
        buckets: &mut [Affine<C>],
        batch: &[(u32, Affine<C>)],
        s: &mut BatchAdd<C>,
    ) {
        s.apply_generic(buckets, batch);
    }
    type Product: Copy + Send + Default;
    fn prefix(accumulator: &mut Fq, value: Self) -> Self::Product;
    fn reverse(inverse: &mut Fq, value: Self, product: &mut Self::Product);
    fn denominator_inverse(value: Self, product: Self::Product) -> Self;
    /// One element as bucket points store it.
    type Limbs: Copy + Default + PartialEq + Send + Sync;
    fn to_limbs(value: Self) -> Self::Limbs;
    fn from_limbs(limbs: Self::Limbs) -> Self;
    fn limbs_zero(limbs: &Self::Limbs) -> bool;
    /// Four elements in lane form.
    type Lanes: Copy;
    fn load(values: [&Self::Limbs; LANES]) -> Self::Lanes;
    fn store(lanes: &Self::Lanes) -> [Self::Limbs; LANES];
    fn lanes_add(a: &Self::Lanes, b: &Self::Lanes) -> Self::Lanes;
    fn lanes_sub(a: &Self::Lanes, b: &Self::Lanes) -> Self::Lanes;
    /// Scale (dy / dx)'s numerator by the preceding norm product.
    fn lanes_scaled_numerator(dy: &Self::Lanes, dx: &Self::Lanes, prefix: &FqBatch) -> Self::Lanes;
    fn lanes_scale(value: &Self::Lanes, factor: &FqBatch) -> Self::Lanes;
    fn lanes_mul(a: &Self::Lanes, b: &Self::Lanes) -> Self::Lanes;
    fn lanes_square(a: &Self::Lanes) -> Self::Lanes;
    /// What the inversion chains multiply by: the value, or its Fq norm.
    fn lanes_norm(value: &Self::Lanes) -> FqBatch;
    /// 1/value from the inverse of its norm.
    fn lanes_inverse(value: &Self::Lanes, inverse_norm: &FqBatch) -> Self::Lanes;
}
impl BatchInverse for Fq {
    fn stream_add<C: CurveConfig<Base = Self>>(
        buckets: &mut [Affine<C>],
        batch: &[(u32, Affine<C>)],
        s: &mut BatchAdd<C>,
    ) {
        stream_g1(buckets, batch, &mut s.fq);
    }
    type Product = Fq;
    #[inline(always)]
    fn prefix(accumulator: &mut Fq, value: Self) -> Fq {
        let prefix = *accumulator;
        *accumulator *= value;
        prefix
    }
    #[inline(always)]
    fn reverse(inverse: &mut Fq, value: Self, product: &mut Fq) {
        *product *= *inverse;
        *inverse *= value;
    }
    #[inline(always)]
    fn denominator_inverse(_: Self, product: Fq) -> Self {
        product
    }
    type Limbs = FqLimbs;
    #[inline(always)]
    fn to_limbs(value: Self) -> FqLimbs {
        FqLimbs::new(value)
    }
    #[inline(always)]
    fn from_limbs(limbs: FqLimbs) -> Self {
        limbs.get()
    }
    #[inline(always)]
    fn limbs_zero(limbs: &FqLimbs) -> bool {
        limbs.is_zero()
    }
    type Lanes = FqBatch;
    #[inline(always)]
    fn load(values: [&FqLimbs; LANES]) -> FqBatch {
        FqBatch::load(values)
    }
    #[inline(always)]
    fn store(lanes: &FqBatch) -> [FqLimbs; LANES] {
        lanes.store()
    }
    #[inline(always)]
    fn lanes_add(a: &FqBatch, b: &FqBatch) -> FqBatch {
        a.add(b)
    }
    #[inline(always)]
    fn lanes_scaled_numerator(dy: &FqBatch, _: &FqBatch, prefix: &FqBatch) -> FqBatch {
        dy.mul(prefix)
    }
    #[inline(always)]
    fn lanes_scale(value: &FqBatch, factor: &FqBatch) -> FqBatch {
        value.mul(factor)
    }
    #[inline(always)]
    fn lanes_sub(a: &FqBatch, b: &FqBatch) -> FqBatch {
        a.sub(b)
    }
    #[inline(always)]
    fn lanes_mul(a: &FqBatch, b: &FqBatch) -> FqBatch {
        a.mul(b)
    }
    #[inline(always)]
    fn lanes_square(a: &FqBatch) -> FqBatch {
        a.mul(a)
    }
    #[inline(always)]
    fn lanes_norm(value: &FqBatch) -> FqBatch {
        *value
    }
    #[inline(always)]
    fn lanes_inverse(_: &FqBatch, inverse_norm: &FqBatch) -> FqBatch {
        *inverse_norm
    }
}
impl BatchInverse for Fq2 {
    fn stream_add<C: CurveConfig<Base = Self>>(
        buckets: &mut [Affine<C>],
        batch: &[(u32, Affine<C>)],
        s: &mut BatchAdd<C>,
    ) {
        if crate::simd::cpu_option("ZOPRO_CPU_DIRECT_G2") {
            stream_g2(buckets, batch, &mut s.fq2);
        } else {
            s.apply_generic(buckets, batch);
        }
    }
    type Product = [Fq; 2];
    #[inline(always)]
    fn prefix(accumulator: &mut Fq, value: Self) -> Self::Product {
        let norm = Fq::sum_of_products(&value.c0, &value.c0, &value.c1, &value.c1);
        let prefix = *accumulator;
        *accumulator *= norm;
        [prefix, norm]
    }
    #[inline(always)]
    fn reverse(inverse: &mut Fq, _: Self, product: &mut Self::Product) {
        product[0] *= *inverse;
        *inverse *= product[1];
    }
    #[inline(always)]
    fn denominator_inverse(value: Self, product: Self::Product) -> Self {
        Fq2::new(value.c0 * product[0], -(value.c1 * product[0]))
    }
    // c0 and c1 of four elements. Fq2 = Fq[u]/(u^2 + 1).
    type Limbs = [FqLimbs; 2];
    #[inline(always)]
    fn to_limbs(value: Self) -> [FqLimbs; 2] {
        [FqLimbs::new(value.c0), FqLimbs::new(value.c1)]
    }
    #[inline(always)]
    fn from_limbs(limbs: [FqLimbs; 2]) -> Self {
        Fq2::new(limbs[0].get(), limbs[1].get())
    }
    #[inline(always)]
    fn limbs_zero(limbs: &[FqLimbs; 2]) -> bool {
        limbs[0].is_zero() && limbs[1].is_zero()
    }
    type Lanes = [FqBatch; 2];
    #[inline(always)]
    fn load(values: [&[FqLimbs; 2]; LANES]) -> [FqBatch; 2] {
        [
            FqBatch::load(values.map(|v| &v[0])),
            FqBatch::load(values.map(|v| &v[1])),
        ]
    }
    #[inline(always)]
    fn store(lanes: &[FqBatch; 2]) -> [[FqLimbs; 2]; LANES] {
        let (c0, c1) = (lanes[0].store(), lanes[1].store());
        core::array::from_fn(|i| [c0[i], c1[i]])
    }
    #[inline(always)]
    fn lanes_add(a: &[FqBatch; 2], b: &[FqBatch; 2]) -> [FqBatch; 2] {
        [a[0].add(&b[0]), a[1].add(&b[1])]
    }
    #[inline(always)]
    fn lanes_scaled_numerator(
        dy: &[FqBatch; 2],
        dx: &[FqBatch; 2],
        prefix: &FqBatch,
    ) -> [FqBatch; 2] {
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        let value = [
            dy[0].sum2(&dx[0], &dy[1], &dx[1]),
            dy[1].difference2(&dx[0], &dy[0], &dx[1]),
        ];
        #[cfg(not(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        let value = Self::lanes_mul(dy, &[dx[0], FqBatch::zero().sub(&dx[1])]);
        Self::lanes_scale(&value, prefix)
    }
    #[inline(always)]
    fn lanes_scale(value: &[FqBatch; 2], factor: &FqBatch) -> [FqBatch; 2] {
        [value[0].mul(factor), value[1].mul(factor)]
    }
    #[inline(always)]
    fn lanes_sub(a: &[FqBatch; 2], b: &[FqBatch; 2]) -> [FqBatch; 2] {
        [a[0].sub(&b[0]), a[1].sub(&b[1])]
    }
    // Karatsuba: three Fq products.
    #[inline(always)]
    fn lanes_mul(a: &[FqBatch; 2], b: &[FqBatch; 2]) -> [FqBatch; 2] {
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            [
                a[0].difference2(&b[0], &a[1], &b[1]),
                a[0].sum2(&b[1], &a[1], &b[0]),
            ]
        }
        #[cfg(not(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        {
            let v0 = a[0].mul(&b[0]);
            let v1 = a[1].mul(&b[1]);
            let v2 = a[0].add(&a[1]).mul(&b[0].add(&b[1]));
            [v0.sub(&v1), v2.sub(&v0).sub(&v1)]
        }
    }
    // (c0 + c1 u)^2 = (c0 + c1)(c0 - c1) + 2 c0 c1 u.
    #[inline(always)]
    fn lanes_square(a: &[FqBatch; 2]) -> [FqBatch; 2] {
        let product = a[0].mul(&a[1]);
        [a[0].add(&a[1]).mul(&a[0].sub(&a[1])), product.add(&product)]
    }
    #[inline(always)]
    fn lanes_norm(value: &[FqBatch; 2]) -> FqBatch {
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            value[0].sum2(&value[0], &value[1], &value[1])
        }
        #[cfg(not(all(
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        {
            value[0].mul(&value[0]).add(&value[1].mul(&value[1]))
        }
    }
    // (c0 - c1 u) / norm.
    #[inline(always)]
    fn lanes_inverse(value: &[FqBatch; 2], inverse_norm: &FqBatch) -> [FqBatch; 2] {
        [
            value[0].mul(inverse_norm),
            FqBatch::zero().sub(&value[1].mul(inverse_norm)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let mut seed = 41u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let scalars: Vec<Fr> = (0..16385)
            .map(|i| match i % 20 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                _ => Fr::from_le_bytes_mod_order(
                    &[
                        next().to_le_bytes(),
                        next().to_le_bytes(),
                        next().to_le_bytes(),
                        next().to_le_bytes(),
                    ]
                    .concat(),
                ),
            })
            .collect();
        let bases: Vec<Affine<C>> = (0..16385)
            .map(|i| match i % 20 {
                0 => Affine::IDENTITY,
                1..=3 => Projective::<C>::generator().to_affine(),
                4 => (-Projective::<C>::generator()).to_affine(),
                _ => (Projective::<C>::generator() * Fr::from(next())).to_affine(),
            })
            .collect();
        for n in [
            0, 1, 255, 256, 511, 1023, 1024, 1025, 4095, 4096, 16383, 16384, 16385,
        ] {
            assert_eq!(
                msm::<C>(&bases[..n], &scalars[..n]),
                zopro_bn254::curve::msm(&bases[..n], &scalars[..n]),
                "size {n}"
            );
        }
        let generator = Projective::<C>::generator().to_affine();
        for scalar in [Fr::ZERO, Fr::ONE, -Fr::ONE] {
            let scalars = vec![scalar; 4096];
            // Force exceptional additions at multiple levels of a bucket tree.
            for bases in [
                vec![generator; 4096],
                (0..4096)
                    .map(|i| if i % 2 == 0 { generator } else { -generator })
                    .collect(),
                vec![Affine::IDENTITY; 4096],
            ] {
                assert_eq!(
                    msm::<C>(&bases, &scalars),
                    zopro_bn254::curve::msm(&bases, &scalars)
                );
            }
        }
    }

    #[test]
    fn extension_field_batch_inverses() {
        let mut seed = 79u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let mut field = || {
            Fq::from_le_bytes_mod_order(
                &[
                    next().to_le_bytes(),
                    next().to_le_bytes(),
                    next().to_le_bytes(),
                    next().to_le_bytes(),
                ]
                .concat(),
            )
        };
        let values: Vec<_> = (0..1024)
            .map(|i| match i % 8 {
                0 => Fq2::ZERO,
                1 => Fq2::ONE,
                2 => -Fq2::ONE,
                3 => Fq2::new(Fq::ZERO, field()),
                4 => Fq2::new(field(), Fq::ZERO),
                _ => Fq2::new(field(), field()),
            })
            .collect();
        let mut accumulator = Fq::ONE;
        let mut products: Vec<_> = values
            .iter()
            .map(|value| {
                if value.is_zero() {
                    Default::default()
                } else {
                    Fq2::prefix(&mut accumulator, *value)
                }
            })
            .collect();
        let mut inverse = accumulator.inverse().unwrap();
        for (value, product) in values.iter().zip(&mut products).rev() {
            if !value.is_zero() {
                Fq2::reverse(&mut inverse, *value, product);
            }
        }
        for (value, product) in values.into_iter().zip(products) {
            if !value.is_zero() {
                assert_eq!(
                    Fq2::denominator_inverse(value, product),
                    value.inverse().unwrap()
                );
            }
        }
    }
    #[test]
    fn g1() {
        check::<zopro_bn254::G1Config>();
    }
    #[test]
    fn g2() {
        check::<zopro_bn254::G2Config>();
    }
}
