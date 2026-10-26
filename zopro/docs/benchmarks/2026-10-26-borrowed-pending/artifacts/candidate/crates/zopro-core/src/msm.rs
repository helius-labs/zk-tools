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
    group_affine_impl(|index| bases[index as usize], order, starts, counts)
}

pub(super) fn group_affine_prepared<C: CurveConfig>(
    bases: &[Affine<C>],
    sums: &[Affine<C>],
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    group_affine_impl(
        |index| {
            let index = index as usize;
            if index < bases.len() {
                bases[index]
            } else {
                sums[index - bases.len()]
            }
        },
        order,
        starts,
        counts,
    )
}

fn group_affine_impl<C: CurveConfig, F: Fn(u32) -> Affine<C> + Sync>(
    get: F,
    order: &[u32],
    starts: &[usize],
    counts: &[usize],
) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    const GROUP: usize = 256;
    let n = counts.len();
    let mut result: Vec<Affine<C>> = Vec::with_capacity(n);
    result.spare_capacity_mut()[..n]
        .par_chunks_mut(GROUP)
        .enumerate()
        .for_each_init(
            || (Vec::<LimbPoint<C::Base>>::new(), Scratch::<C>::default()),
            |(points, scratch), (i, output)| {
                let first = i * GROUP;
                let offset = starts[first];
                let end = starts[first + output.len()];
                points.clear();
                points.reserve_exact(end - offset);
                for &index in &order[offset..end] {
                    points.push(limbs::<C>(get(index)));
                }
                let mut local_starts = [0usize; GROUP];
                let mut local_counts = [0usize; GROUP];
                for j in 0..output.len() {
                    local_starts[j] = starts[first + j] - offset;
                    local_counts[j] = counts[first + j];
                }
                let local_starts = &local_starts[..output.len()];
                let local_counts = &mut local_counts[..output.len()];
                while local_counts.iter().any(|&count| count > 1) {
                    reduce_layer(points, local_starts, local_counts, scratch);
                }
                for (j, out) in output.iter_mut().enumerate() {
                    out.write(if local_counts[j] == 0 {
                        Affine::IDENTITY
                    } else {
                        affine::<C>(points[local_starts[j]])
                    });
                }
            },
        );
    // SAFETY: the completed indexed parallel pass wrote all n output slots
    // exactly once. Affine coordinates have no drop obligations on panic.
    unsafe { result.set_len(n) };
    result
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
        .map_init(
            || (Vec::<LimbPoint<C::Base>>::new(), Scratch::<C>::default()),
            |(tree, scratch), buckets| {
                let n = buckets.len();
                tree.resize(2 * n - 1, LimbPoint::<C::Base>::infinity());
                for (out, &point) in tree.iter_mut().zip(buckets) {
                    *out = limbs::<C>(point);
                }
                // weighted_sum writes every internal tree slot before reading it.
                let weighted = weighted_sum(tree, n, scratch);
                (weighted, affine::<C>(tree[0]))
            },
        )
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
    vector_inverse: bool,
    pairs: Vec<usize>,
    // Groups of four pairs in lane form, then the scalar tail.
    groups: Vec<Group<C::Base>>,
    denominators: [C::Base; LANES],
    products: [<C::Base as BatchInverse>::Product; LANES],
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
            vector_inverse: crate::simd::cpu_option("ZOPRO_CPU_TREE_VECTOR_INVERSE"),
            pairs: Vec::new(),
            groups: Vec::new(),
            denominators: [C::Base::ZERO; LANES],
            products: [Default::default(); LANES],
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
    s.groups.reserve_exact(full / LANES);

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
    let mut inverses = if let Some(value)=direct_chain_inverse(&chains,full==m,s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..m {
            let (a, b) = (
                affine::<C>(points[s.pairs[k]]),
                affine::<C>(points[s.pairs[k] + 1]),
            );
            let denominator = b.x - a.x;
            {
                s.denominators[k - full] = denominator;
                s.products[k - full] = F::<C>::prefix(&mut tail[k % LANES], denominator);
            }
        }
        let mut inverses = invert_lanes(tail, s.vector_inverse);
        for k in (full..m).rev() {
            let t = k - full;
            F::<C>::reverse(
                &mut inverses[k % LANES],
                s.denominators[t],
                &mut s.products[t],
            );
        }
        FqBatch::new(&inverses)
    };
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
            {
                points[s.pairs[j * LANES + i]] = LimbPoint { x: x[i], y: y[i] };
            }
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
        {
            points[s.pairs[k]] = limbs::<C>(Affine::new(x, y));
        }
    }
    // All vector inputs were captured before the reverse pass. Each ordinary
    // pair writes only its first input slot, disjoint from every other pair
    // and exceptional pair. Scalar tails are likewise disjoint. Compaction
    // writes before the next unread pair, so no result vector is needed.
    // Each bucket keeps its pair sums in front, in order. A write lands
    // before the pairs still to be read.
    let mut cursor = 0;
    for index in 0..counts.len() {
        let offset = starts[index];
        let n = counts[index];
        for j in 0..n / 2 {
            points[offset + j] = if s.pairs.get(cursor) == Some(&(offset + 2 * j)) {
                cursor += 1;
                {
                    points[offset + 2 * j]
                }
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
fn invert_lanes(c: [Fq; LANES], vector_inverse: bool) -> [Fq; LANES] {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    if vector_inverse {
        return FqBatch::new(&c).inverse_nonzero().get();
    }
    let _ = vector_inverse;
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
        let vector_inverse = crate::simd::cpu_option("ZOPRO_CPU_VECTOR_INVERSE");
        let canonical = crate::simd::cpu_option("ZOPRO_CPU_CANONICAL_BATCH");
        let store_mode = std::env::var("ZOPRO_CPU_G1_SCRATCH_STORE")
            .ok()
            .and_then(|v| v.parse::<u8>().ok())
            .filter(|v| *v <= 2)
            .unwrap_or(
                if cfg!(all(
                    feature = "ifma8",
                    target_arch = "x86_64",
                    target_feature = "avx512ifma",
                    target_feature = "avx512vl",
                    target_feature = "avx512dq",
                    not(no_ifma)
                )) {
                    2
                } else {
                    0
                },
            );
        Self {
            points: Vec::new(),
            starts: Vec::new(),
            counts: Vec::new(),
            scratch: Scratch::default(),
            direct: crate::simd::cpu_option("ZOPRO_CPU_DIRECT"),
            fq: FqAddScratch {
                write_groups: store_mode == 2,
                reserve_groups: store_mode > 0,
                vector_inverse,
                canonical,
                ..Default::default()
            },
            fq2: Fq2AddScratch {
                vector_inverse,
                canonical,
                #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] prefix_enabled:std::env::var("ZOPRO_CPU_G2_PREFIX_RECORDS").as_deref()==Ok("1"),
                #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] prefix_trace:std::env::var_os("ZOPRO_CPU_G2_PREFIX_TRACE").is_some(),
                ..Default::default()
            },
        }
    }
}
impl<C: CurveConfig> BatchAdd<C>
where
    C::Base: BatchInverse,
{
    pub(super) fn apply_signed(&mut self, buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)]) {
        if self.direct {C::Base::stream_add_signed(buckets,batch,self);}
        else {let decoded=decode_signed_batch(batch);self.apply_generic(buckets,&decoded);}
    }
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
            {
            // Batch length is actual work, not a key-wide upper bound.
            self.points.reserve_exact(batch.len()*2);
            self.starts.reserve_exact(batch.len());
            self.counts.reserve_exact(batch.len());
        }
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

impl<C: CurveConfig<Base = Fq>> BatchAdd<C> {
    pub(super) fn apply_signed_borrowed(&mut self, buckets: &mut [Affine<C>], batch: &[(u32, &Affine<C>)]) {
        if self.fq.canonical { stream_g1_impl::<C,true,true>(buckets,batch,&mut self.fq); }
        else { stream_g1_impl::<C,false,true>(buckets,batch,&mut self.fq); }
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
    write_groups: bool,
    reserve_groups: bool,
    vector_inverse: bool,
    canonical: bool,
    groups: Vec<FqAddGroup>,
    tail: Vec<[Fq; 5]>,
}
#[inline(always)]
fn canonical_add<const FAST: bool>(a: &FqBatch, b: &FqBatch) -> FqBatch {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    if FAST {
        return a.add_canonical(b);
    }
    a.add(b)
}
#[inline(always)]
fn canonical_sub<const FAST: bool>(a: &FqBatch, b: &FqBatch) -> FqBatch {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    if FAST {
        return a.sub_canonical(b);
    }
    a.sub(b)
}
fn stream_g1<C: CurveConfig<Base = Fq>>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, Affine<C>)],
    s: &mut FqAddScratch,
) {
    if s.canonical {
        stream_g1_impl::<C, true, false>(buckets, batch, s)
    } else {
        stream_g1_impl::<C, false, false>(buckets, batch, s)
    }
}

#[inline(always)]
fn batch_index<const SIGNED:bool>(index:u32)->usize {if SIGNED {(index&0x7fffffff)as usize}else{index as usize}}
#[inline(always)]
fn signs<const SIGNED:bool,C:CurveConfig>(batch:&[(u32,impl std::borrow::Borrow<Affine<C>> )])->u8 {
    if !SIGNED {return 0;}let mut bits=0u8;for i in 0..LANES {bits|=((batch[i].0>>31)as u8)<<i;}bits
}
#[inline(always)]
fn scaled_signed_dy<const FAST:bool,const SIGNED:bool>(by:&FqBatch,ay:&FqBatch,chains:&FqBatch,negative:u8)->FqBatch {
    #[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    if SIGNED {return by.signed_difference_scaled(ay,chains,negative);}
    if SIGNED {let values=by.get();let signed=core::array::from_fn(|i|if negative>>i&1!=0{-values[i]}else{values[i]});return canonical_sub::<FAST>(&FqBatch::new(&signed),ay).mul(chains);}
    canonical_sub::<FAST>(by,ay).mul(chains)
}
fn stream_g1_impl<C: CurveConfig<Base = Fq>, const FAST: bool, const SIGNED: bool>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, impl std::borrow::Borrow<Affine<C>> + Copy)],
    s: &mut FqAddScratch,
) {
    use std::borrow::Borrow;
    let full = batch.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.groups.clear();
    if s.write_groups {
        // Reserve all records before taking any pointers. Each record is
        // Copy and its length is published only after all five fields are
        // initialized. A panic before then leaves no partial record in len.
        s.groups.reserve(full / LANES);
        for k in (0..full).step_by(LANES) {
            let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch_index::<SIGNED>(batch[k + i].0)]);
            let b: [Affine<C>; LANES] = core::array::from_fn(|i| *batch[k + i].1.borrow());
            // SAFETY: reserve covers full/LANES records, k/LANES is in
            // that range, and no allocation changes during these writes.
            // addr_of_mut! creates no reference to uninitialized storage.
            unsafe {
                let slot = s.groups.as_mut_ptr().add(k / LANES);
                let ax = FqBatch::new(&a.map(|p| p.x));
                core::ptr::addr_of_mut!((*slot).ax).write(ax);
                let bx = FqBatch::new(&b.map(|p| p.x));
                let denominator = canonical_sub::<FAST>(&bx, &ax);
                core::ptr::addr_of_mut!((*slot).xsum).write(canonical_add::<FAST>(&ax, &bx));
                core::ptr::addr_of_mut!((*slot).denominator).write(denominator);
                let ay = FqBatch::new(&a.map(|p| p.y));
                core::ptr::addr_of_mut!((*slot).ay).write(ay);
                let by = FqBatch::new(&b.map(|p| p.y));
                let numerator = scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&chains,signs::<SIGNED,C>(&batch[k..k+LANES]));
                core::ptr::addr_of_mut!((*slot).numerator).write(numerator);
                chains = chains.mul(&denominator);
                // All five fields of this record are now initialized.
                s.groups.set_len(k / LANES + 1);
            }
        }
    } else {
        if s.reserve_groups {
            s.groups.reserve(full / LANES);
        }
        for k in (0..full).step_by(LANES) {
            let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch_index::<SIGNED>(batch[k + i].0)]);
            let b: [Affine<C>; LANES] = core::array::from_fn(|i| *batch[k + i].1.borrow());
            let ax = FqBatch::new(&a.map(|p| p.x));
            let ay = FqBatch::new(&a.map(|p| p.y));
            let bx = FqBatch::new(&b.map(|p| p.x));
            let by = FqBatch::new(&b.map(|p| p.y));
            // Host coordinates are canonical; only the resulting differences are lazy.
            let denominator = canonical_sub::<FAST>(&bx, &ax);
            let numerator = scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&chains,signs::<SIGNED,C>(&batch[k..k+LANES]));
            chains = chains.mul(&denominator);
            s.groups.push(FqAddGroup {
                ax,
                ay,
                xsum: canonical_add::<FAST>(&ax, &bx),
                denominator,
                numerator,
            });
        }
    }
    s.tail.clear();
    let mut inverse = if let Some(value)=direct_chain_inverse(&chains,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..batch.len() {
            let (index, b) = batch[k];
            let b = *b.borrow();
            let b = if SIGNED && index>>31!=0 {-b}else{b};
            let a = buckets[batch_index::<SIGNED>(index)];
            let denominator = b.x - a.x;
            let numerator = (b.y - a.y) * tail[k % LANES];
            tail[k % LANES] *= denominator;
            s.tail.push([a.x, a.y, a.x + b.x, denominator, numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, denominator, numerator] = s.tail[k - full];
            let slope = numerator * inverse[k % LANES];
            inverse[k % LANES] *= denominator;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch_index::<SIGNED>(batch[k].0)] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
    };
    for (j, g) in s.groups.iter().enumerate().rev() {
        let slope = g.numerator.mul(&inverse);
        inverse = inverse.mul(&g.denominator);
        let x = slope.mul(&slope).sub(&g.xsum);
        let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);
        let (x, y) = (x.get(), y.get());
        for i in 0..LANES {
            buckets[batch_index::<SIGNED>(batch[j * LANES + i].0)] = Affine::new(x[i], y[i]);
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
    vector_inverse: bool,
    canonical: bool,
    groups: Vec<Fq2AddGroup>,
    #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] prefix_records:Vec<[FqBatch;2]>,
    #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] prefix_enabled:bool,
    #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] prefix_trace:bool,
    #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] prefix_traced:bool,
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
    if s.canonical {
        stream_g2_impl::<C, true>(buckets, batch, s)
    } else {
        stream_g2_impl::<C, false>(buckets, batch, s)
    }
}
fn stream_g2_impl<C: CurveConfig<Base = Fq2>, const FAST: bool>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, Affine<C>)],
    s: &mut Fq2AddScratch,
) {
    #[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] {
        if s.prefix_trace && !s.prefix_traced {eprintln!("G2_PREFIX_RECORDS enabled={} batch={}",s.prefix_enabled,batch.len());s.prefix_traced=true;}
        if s.prefix_enabled {return g2_prefix::apply::<C,FAST>(buckets,batch,s);}
    }
    let full = batch.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.groups.clear();
    s.groups.reserve_exact(full / LANES);

    for k in (0..full).step_by(LANES) {
        let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch[k + i].0 as usize]);
        let b: [Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = fq2_pack(a.map(|p| p.x));
        let ay = fq2_pack(a.map(|p| p.y));
        let bx = fq2_pack(b.map(|p| p.x));
        let by = fq2_pack(b.map(|p| p.y));
        let denominator = core::array::from_fn(|i| canonical_sub::<FAST>(&bx[i], &ax[i]));
        let norm = Fq2::lanes_norm(&denominator);
        let dy: [FqBatch; 2] = core::array::from_fn(|i| canonical_sub::<FAST>(&by[i], &ay[i]));
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
        let xsum = core::array::from_fn(|i| canonical_add::<FAST>(&ax[i], &bx[i]));
        s.groups.push(Fq2AddGroup {
            ax,
            ay,
            xsum,
            norm,
            numerator,
        });
    }
    s.tail.clear();
    let mut inverse = if let Some(value)=direct_chain_inverse(&chains,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
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
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
            let slope = numerator.mul_by_fp(&inverse[k % LANES]);
            inverse[k % LANES] *= norm.c0;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch[k].0 as usize] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
    };
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
    {
        // A power-of-two usize has at most usize::BITS reduction levels.
        let mut starts = [0usize; usize::BITS as usize];
        let mut counts = [0usize; usize::BITS as usize];
        counts[0] = n;
        let mut levels = 1;
        let mut next = n;
        while counts[0] > 1 {
            let half = counts[0] / 2;
            for i in 0..half {
                points[next + i] = points[2 * i];
            }
            reduce_layer(points, &starts[..levels], &mut counts[..levels], scratch);
            starts[levels] = next;
            counts[levels] = half;
            levels += 1;
            next += half;
        }
        let mut sum = affine::<C>(points[0]).to_projective();
        for start in starts[1..levels].iter().rev() {
            sum = sum.double();
            sum -= affine::<C>(points[*start]);
        }
        return sum;
    }
}

// In Fq2 = Fq[u]/(u^2 + 1), invert x + y*u using
// (x - y*u)/(x^2 + y^2). Batch the norms in Fq instead of multiplying
// Fq2 prefix products. Keep the norm beside each prefix so the reverse
// pass does not compute it again. Exceptional pairs are excluded above.

fn decode_signed_batch<C:CurveConfig>(batch:&[(u32,Affine<C>)])->Vec<(u32,Affine<C>)> {
    batch.iter().map(|&(index,p)|(index&0x7fffffff,if index>>31!=0{-p}else{p})).collect()
}

pub(super) trait BatchInverse: Field {
    fn deferred_sign()->bool {false}
    fn stream_add_signed<C:CurveConfig<Base=Self>>(buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut BatchAdd<C>) {
        s.apply(buckets,&decode_signed_batch(batch));
    }

    fn negate_if(value: Self, negative: bool) -> Self;

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
    #[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    fn deferred_sign()->bool {std::env::var("ZOPRO_CPU_DEFER_SIGNED_Y").as_deref()==Ok("1")}
    #[cfg(all(feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    fn stream_add_signed<C:CurveConfig<Base=Self>>(buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut BatchAdd<C>) {
        if s.fq.canonical {stream_g1_impl::<C,true,true>(buckets,batch,&mut s.fq)}else{stream_g1_impl::<C,false,true>(buckets,batch,&mut s.fq)}
    }

    #[inline(always)]
    fn negate_if(value: Self, negative: bool) -> Self {
        use zopro_bn254::{FpConfig, FqConfig};
        let nonzero = (value.0[0] | value.0[1] | value.0[2] | value.0[3]) != 0;
        let mask = 0u64.wrapping_sub(u64::from(negative) & u64::from(nonzero));
        let mut carry = (mask & 1) as u128;
        let words = core::array::from_fn(|i| {
            let sum = ((value.0[i] ^ mask) as u128) + ((FqConfig::P[i] & mask) as u128) + carry;
            carry = sum >> 64;
            sum as u64
        });
        Fq::from_montgomery(words)
    }

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
    #[inline(always)]
    fn negate_if(value: Self, negative: bool) -> Self {
        Fq2::new(
            Fq::negate_if(value.c0, negative),
            Fq::negate_if(value.c1, negative),
        )
    }

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
    fn g1_scratch_stores_match_reference() {
        use zopro_bn254::G1Config;
        let g = Projective::<G1Config>::generator();
        for n in [0, 1, 7, 8, 9, 19, 152, 352] {
            let mut p = g;
            let mut q = g.mul(&Fr::from(5000u64));
            let points: Vec<_> = (0..n)
                .map(|i| {
                    let result = if i % 5 == 0 { -p } else { p };
                    p += g;
                    result.to_affine()
                })
                .collect();
            let batch: Vec<_> = (0..n)
                .map(|i| {
                    let result = if i % 3 == 0 { -q } else { q };
                    q += g;
                    ((i * 37 % n) as u32, result.to_affine())
                })
                .collect();
            for canonical in [false, true] {
                for vector_inverse in [false, true] {
                    for mode in 0..3 {
                        let mut actual = points.clone();
                        let mut expected = points.clone();
                        let mut scratch = FqAddScratch {
                            canonical,
                            vector_inverse,
                            write_groups: mode == 2,
                            reserve_groups: mode > 0,
                            ..Default::default()
                        };
                        for _ in 0..2 {
                            for &(index, b) in &batch {
                                assert_ne!(expected[index as usize].x, b.x);
                                expected[index as usize] =
                                    (expected[index as usize] + b).to_affine();
                            }
                            stream_g1(&mut actual, &batch, &mut scratch);
                            assert_eq!(actual, expected, "n={n} mode={mode}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn conditional_negation() {
        use zopro_bn254::{FpConfig, FqConfig};
        let mut seed = 19u64;
        for i in 0..1024 {
            let mut bytes = [0; 64];
            for chunk in bytes.chunks_mut(8) {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                chunk.copy_from_slice(&seed.to_le_bytes());
            }
            let value = match i {
                0 => Fq::ZERO,
                1 => Fq::ONE,
                2 => -Fq::ONE,
                3 => Fq::from_montgomery([
                    FqConfig::P[0] - 1,
                    FqConfig::P[1],
                    FqConfig::P[2],
                    FqConfig::P[3],
                ]),
                _ => Fq::from_random_bytes(&bytes),
            };
            assert_eq!(Fq::negate_if(value, false), value);
            assert_eq!(Fq::negate_if(value, true), -value);
            let extension = Fq2::new(value, if i % 2 == 0 { Fq::ZERO } else { value });
            assert_eq!(Fq2::negate_if(extension, false), extension);
            assert_eq!(Fq2::negate_if(extension, true), -extension);
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


#[cfg(all(target_arch = "x86_64", target_os = "linux", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
pub(crate) struct SubsetG2Sums {
    buckets: Vec<zopro_bn254::G2Affine>,
    batch: Vec<(u32, zopro_bn254::G2Affine)>,
    scratch: BatchAdd<zopro_bn254::G2Config>,
    dummy: [zopro_bn254::G2Affine; 3],
}
#[cfg(all(target_arch = "x86_64", target_os = "linux", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
impl SubsetG2Sums {
    pub(crate) fn new(dummy: [zopro_bn254::G2Affine; 3]) -> Self {
        let mut buckets = vec![zopro_bn254::G2Affine::IDENTITY; 258];
        buckets[256] = dummy[0];
        buckets[257] = dummy[1];
        Self {
            buckets,
            batch: Vec::with_capacity(258),
            scratch: BatchAdd::default(),
            dummy,
        }
    }
    #[inline(always)]
    fn push_at(&mut self, bit: usize, p: zopro_bn254::G2Affine) {
        if p.is_identity() {
            return;
        }
        let a = self.buckets[bit];
        if a.is_identity() {
            self.buckets[bit] = p;
        } else if a.x == p.x {
            self.buckets[bit] = if a.y == p.y {
                (a + p).to_affine()
            } else {
                zopro_bn254::G2Affine::IDENTITY
            };
        } else {
            self.batch.push((bit as u32, p));
        }
    }
    #[inline(always)]
    pub(crate) fn push(&mut self, bit: usize, p: zopro_bn254::G2Affine) {
        debug_assert!(bit < 256);
        self.push_at(bit, p);
    }
    pub(crate) fn flush(&mut self, pad: bool) {
        // Each real bit is visited once per group, with two separate optional
        // dummy buckets. Queued indices are distinct and all x differences
        // have been checked; dummy results never enter the weighted output.
        if pad {
            self.push_at(256, self.dummy[2]);
            self.push_at(257, self.dummy[2]);
        }
        self.scratch.apply(&mut self.buckets, &self.batch);
        self.batch.clear();
    }
    pub(crate) fn finish(self) -> zopro_bn254::G2Projective {
        let mut result = zopro_bn254::G2Projective::IDENTITY;
        for p in self.buckets[..256].iter().rev() {
            result = result.double();
            result += p;
        }
        result
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
pub(crate) struct SubsetG1Sums {
    buckets: Vec<zopro_bn254::G1Affine>,
    batch: Vec<(u32, zopro_bn254::G1Affine)>,
    scratch: BatchAdd<zopro_bn254::G1Config>,
    dummy: [zopro_bn254::G1Affine; 3],
}
#[cfg(all(target_arch = "x86_64", target_os = "linux", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
impl SubsetG1Sums {
    pub(crate) fn new(dummy: [zopro_bn254::G1Affine; 3]) -> Self {
        let mut buckets = vec![zopro_bn254::G1Affine::IDENTITY; 258];
        buckets[256] = dummy[0];
        buckets[257] = dummy[1];
        Self {
            buckets,
            batch: Vec::with_capacity(258),
            scratch: BatchAdd::default(),
            dummy,
        }
    }
    #[inline(always)]
    fn push_at(&mut self, bit: usize, p: zopro_bn254::G1Affine) {
        if p.is_identity() {
            return;
        }
        let a = self.buckets[bit];
        if a.is_identity() {
            self.buckets[bit] = p;
        } else if a.x == p.x {
            self.buckets[bit] = if a.y == p.y {
                (a + p).to_affine()
            } else {
                zopro_bn254::G1Affine::IDENTITY
            };
        } else {
            self.batch.push((bit as u32, p));
        }
    }
    #[inline(always)]
    pub(crate) fn push(&mut self, bit: usize, p: zopro_bn254::G1Affine) {
        debug_assert!(bit < 256);
        self.push_at(bit, p);
    }
    pub(crate) fn flush(&mut self, pad: bool) {
        // Each real bit is visited once per group, with two separate optional
        // dummy buckets. Queued indices are distinct and all x differences
        // have been checked; dummy results never enter the weighted output.
        if pad {
            self.push_at(256, self.dummy[2]);
            self.push_at(257, self.dummy[2]);
        }
        self.scratch.apply(&mut self.buckets, &self.batch);
        self.batch.clear();
    }
    pub(crate) fn finish(self) -> zopro_bn254::G1Projective {
        let mut result = zopro_bn254::G1Projective::IDENTITY;
        for p in self.buckets[..256].iter().rev() {
            result = result.double();
            result += p;
        }
        result
    }
}

#[cfg(test)]
mod deferred_sign_reference_tests {
    use super::*;
    use zopro_bn254::{G1Affine,G1Config};
    #[test]
    fn borrowed_signed_stream_tails_and_reuse_match_generator() {
        for n in [0usize,1,7,8,9,15,16,199,352,511] {
            let original:Vec<_>=(0..n).map(|i|(G1Affine::GENERATOR*Fr::from((i+7)as u64)).to_affine()).collect();
            let inputs:Vec<_>=(0..n).map(|i|(G1Affine::GENERATOR*Fr::from((i+n+1_000_003)as u64)).to_affine()).collect();
            for mode in 0..3 {
                let batch:Vec<_>=inputs.iter().enumerate().map(|(i,p)|(((n-1-i)as u32)|if mode==1||mode==2&&i%3!=0{1<<31}else{0},p)).collect();
                for canonical in [false,true] {for vector_inverse in [false,true] {for storage in 0..3 {
                    let mut got=original.clone();let mut work=BatchAdd::<G1Config>::default();
                    work.fq.canonical=canonical;work.fq.vector_inverse=vector_inverse;
                    work.fq.reserve_groups=storage>0;work.fq.write_groups=storage==2;
                    for round in 1..=3 {
                        work.apply_signed_borrowed(&mut got,&batch);
                        for(i,p)in got.iter().enumerate(){
                            let input_index=n-1-i;
                            let w=Fr::from((input_index+n+1_000_003)as u64)*Fr::from(round as u64);
                            let negative=mode==1||mode==2&&input_index%3!=0;
                            let expected=Fr::from((i+7)as u64)+if negative{-w}else{w};
                            assert_eq!(*p,(G1Affine::GENERATOR*expected).to_affine(),"n={n} mode={mode} canonical={canonical} inverse={vector_inverse} storage={storage} round={round}");
                        }
                    }
                }}}
            }
        }
    }
    #[test]
    fn signed_stream_matches_complete_curve_additions_including_tails() {
        for n in [1usize,7,8,15,16,199,352,511] {
            let original:Vec<_>=(0..n).map(|i|(G1Affine::GENERATOR*Fr::from((i+7)as u64)).to_affine()).collect();
            let inputs:Vec<_>=(0..n).map(|i|(G1Affine::GENERATOR*Fr::from((i+n+103)as u64)).to_affine()).collect();
            for mode in 0..3 {
                let batch:Vec<_>=inputs.iter().enumerate().map(|(i,&p)|(((n-1-i)as u32)|if mode==1||mode==2&&i%3!=0{1<<31}else{0},p)).collect();
                let mut expected=original.clone();for &(index,p)in &batch {let i=(index&0x7fffffff)as usize;assert_ne!(original[i].x,p.x,"stream batch requires distinct x");expected[i]=(expected[i]+if index>>31!=0{-p}else{p}).to_affine();}
                for direct in [false,true] {for canonical in [false,true] {let mut got=original.clone();let mut work=BatchAdd::<G1Config>::default();work.direct=direct;work.fq.canonical=canonical;work.apply_signed(&mut got,&batch);assert_eq!(got,expected,"n={n} mode={mode} direct={direct} canonical={canonical}");}}
            }
        }
    }
}

#[cfg(test)]
mod scratch_capacity_tests {
    use super::*;
    fn check<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let g = Projective::<C>::generator();
        let sizes: Vec<_> = [257usize]
            .into_iter()
            .chain(0..34)
            .chain([129, 128, 127, 17, 9, 7, 1, 0, 258, 3])
            .collect();
        for vector_inverse in [false, true] {
            let mut scratch = Scratch::<C>::default();
            scratch.vector_inverse = vector_inverse;
            for kind in 0..4 {
                for &n in &sizes {
                    let a: Vec<_> = (0..n).map(|i| Fr::from((i + 2) as u64)).collect();
                    let b: Vec<_> = (0..n)
                        .map(|i| match kind {
                            0 => Fr::from((i + n + 39) as u64),
                            1 => {
                                if i % 3 == 0 {
                                    -a[i]
                                } else {
                                    a[i]
                                }
                            }
                            2 => Fr::ZERO,
                            _ => {
                                if i % 3 == 0 {
                                    Fr::ZERO
                                } else if i % 3 == 1 {
                                    -a[i]
                                } else {
                                    Fr::from((i + n + 39) as u64)
                                }
                            }
                        })
                        .collect();
                    let points: Vec<_> = a
                        .iter()
                        .zip(&b)
                        .flat_map(|(&x, &y)| [(g * x).to_affine(), (g * y).to_affine()])
                        .map(limbs::<C>)
                        .collect();
                    let mut points = points;
                    let starts: Vec<_> = (0..n).map(|i| 2 * i).collect();
                    let mut counts = vec![2; n];
                    reduce_layer(&mut points, &starts, &mut counts, &mut scratch);
                    for i in 0..n {
                        assert_eq!(counts[i], 1);
                        assert_eq!(
                            affine::<C>(points[2 * i]),
                            (g * (a[i] + b[i])).to_affine(),
                            "n={n} kind={kind} inverse={vector_inverse} i={i}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn scratch_g1_reused_across_sizes_and_all_scalar_tail_lengths() {
        check::<zopro_bn254::G1Config>();
    }
    #[test]
    fn scratch_g2_reused_across_sizes_and_all_scalar_tail_lengths() {
        check::<zopro_bn254::G2Config>();
    }
}

#[cfg(test)]
mod inplace_scratch_tests {
    use super::*;
    fn check<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let g = Projective::<C>::generator();
        let mut scratch = Scratch::<C>::default();
        for inverse in [false, true] {
            scratch.vector_inverse = inverse;
            for kind in 0..4 {
                for shift in 0..17 {
                    let mut points = Vec::new();
                    let mut starts = Vec::new();
                    let mut counts = Vec::new();
                    let mut wants = Vec::new();
                    for bucket in 0..49 {
                        let n = (bucket * 7 + shift) % 34;
                        starts.push(points.len());
                        counts.push(n);
                        let mut want = Fr::ZERO;
                        for j in 0..n {
                            let a = Fr::from(((bucket * 43 + j / 2 * 19 + 1) % 997 + 1) as u64);
                            let scalar = match kind {
                                0 => a,
                                1 => {
                                    if j % 2 == 1 {
                                        -a
                                    } else {
                                        a
                                    }
                                }
                                2 => {
                                    if j % 3 == 0 {
                                        Fr::ZERO
                                    } else {
                                        a
                                    }
                                }
                                _ => Fr::from(((j * 17 + bucket * 11 + 7) % 313 + 1) as u64),
                            };
                            want += scalar;
                            points.push(limbs::<C>((g * scalar).to_affine()));
                        }
                        wants.push(want);
                    }
                    while counts.iter().any(|&n| n > 1) {
                        reduce_layer(&mut points, &starts, &mut counts, &mut scratch);
                    }
                    for i in 0..counts.len() {
                        let got = if counts[i] == 0 {
                            Affine::IDENTITY
                        } else {
                            affine::<C>(points[starts[i]])
                        };
                        assert_eq!(
                            got,
                            (g * wants[i]).to_affine(),
                            "kind={kind} shift={shift} bucket={i} inverse={inverse}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn inplace_g1_mixed_buckets_odd_counts_and_reused_layers() {
        check::<zopro_bn254::G1Config>();
    }
    #[test]
    fn inplace_g2_mixed_buckets_odd_counts_and_reused_layers() {
        check::<zopro_bn254::G2Config>();
    }
}

#[cfg(test)]
mod reused_reduction_tests {
    use super::*;
    fn weighted<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let g = Projective::<C>::generator();
        for kind in 0..4 {
            let scalars: Vec<_> = (0..4096)
                .map(|i| match kind {
                    0 => Fr::ZERO,
                    1 => Fr::ONE,
                    2 => {
                        if i % 2 == 0 {
                            Fr::ONE
                        } else {
                            -Fr::ONE
                        }
                    }
                    _ => Fr::from(((i * 37 + 19) % 997 + 1) as u64),
                })
                .collect();
            let bases = Projective::<C>::batch_to_affine(
                &scalars.iter().map(|&s| g * s).collect::<Vec<_>>(),
            );
            for n in [
                0usize, 1, 2, 4, 8, 16, 32, 64, 128, 256, 257, 258, 260, 264, 272, 288, 320, 384,
                512, 768, 1024, 1280, 2048, 4096,
            ] {
                let want = g * scalars[..n]
                    .iter()
                    .enumerate()
                    .fold(Fr::ZERO, |a, (i, &s)| a + Fr::from((i + 1) as u64) * s);
                assert_eq!(
                    weighted_affine::<C>(&bases[..n]).to_affine(),
                    want.to_affine(),
                    "kind={kind} n={n}"
                );
            }
        }
    }
    fn grouped<C: CurveConfig>()
    where
        C::Base: BatchInverse,
    {
        let g = Projective::<C>::generator();
        for n in [0usize, 1, 7, 255, 256, 257, 511, 512, 513, 1025] {
            let mut starts = vec![0];
            let mut counts = Vec::new();
            let mut order = Vec::new();
            let mut scalars = Vec::new();
            let mut wants = Vec::new();
            for i in 0..n {
                let count = (i * 7 + 3) % 19;
                let mut want = Fr::ZERO;
                for j in 0..count {
                    let a = Fr::from((i / 2 + 1) as u64);
                    let a = if j % 5 == 0 {
                        Fr::ZERO
                    } else if j % 2 == 0 {
                        a
                    } else {
                        -a
                    };
                    want += a;
                    order.push(scalars.len() as u32);
                    scalars.push(a);
                }
                counts.push(count);
                starts.push(order.len());
                wants.push(want);
            }
            let bases = Projective::<C>::batch_to_affine(
                &scalars.iter().map(|&a| g * a).collect::<Vec<_>>(),
            );
            let got = group_affine(&bases, &order, &starts, &counts);
            assert_eq!(got.len(), n);
            for (i, p) in got.iter().enumerate() {
                assert_eq!(*p, (g * wants[i]).to_affine(), "n={n} group={i}");
            }
        }
    }
    #[test]
    fn reused_weighted_g1_independent_oracle() {
        weighted::<zopro_bn254::G1Config>();
    }
    #[test]
    fn reused_weighted_g2_independent_oracle() {
        weighted::<zopro_bn254::G2Config>();
    }
    #[test]
    fn direct_group_output_g1_empty_and_partial_chunks() {
        grouped::<zopro_bn254::G1Config>();
    }
    #[test]
    fn direct_group_output_g2_empty_and_partial_chunks() {
        grouped::<zopro_bn254::G2Config>();
    }
}

// Typed requested-capacity planning helpers; no kernel allocation changes.
#[cfg(all(
    zopro_resource_schedule,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) fn workspace_batch<C: CurveConfig>(n: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    use crate::msm_affine::workspace::vector as v;
    let groups = n / LANES;
    let tail = n.min(LANES - 1);
    if core::mem::size_of::<C::Base>() == 32 {
        // reserve(full/LANES) can start with a non-power-of-two capacity;
        // later growth may double it. A push-only next-power bound is unsound.
        v::<FqAddGroup>(groups.checked_mul(2)?, false)?.checked_add(v::<[Fq; 5]>(tail, true)?)
    } else {
        v::<Fq2AddGroup>(groups, false)?.checked_add(v::<[Fq2; 5]>(tail, true)?)
    }
}
#[cfg(all(
    zopro_resource_schedule,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) fn workspace_weighted<C: CurveConfig>(n: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    use crate::msm_affine::workspace::vector as v;
    if n == 0 {
        return Some(0);
    }
    let group = n.min(256);
    let pairs = group / 2;
    let per = v::<LimbPoint<C::Base>>(2 * group - 1, false)?
        .checked_add(v::<usize>(pairs, true)?)?
        .checked_add(v::<Group<C::Base>>(pairs / LANES, false)?)?;
    per.checked_mul(n.div_ceil(256))?
        .checked_add(v::<(Projective<C>, Affine<C>)>(n.div_ceil(256), false)?)
}
#[cfg(all(
    zopro_resource_schedule,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) fn workspace_grouping<C: CurveConfig>(n: usize, jobs: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    use crate::msm_affine::workspace::vector as v;
    if n == 0 {
        return Some(0);
    }
    // All disjoint input points may be live across suspended group/tile tasks.
    // Pair-index growth <2x and reserved lane groups are charged by their types.
    let points = v::<LimbPoint<C::Base>>(n, false)?;
    // Rounding is per job, not once for the concatenation. The sum of
    // power-of-two capacities is <2*sum(pair lengths), plus tiny minima.
    let pairs = v::<usize>(n.checked_add(jobs.checked_mul(4)?)?, false)?;
    let groups = v::<Group<C::Base>>(n / (2 * LANES), false)?;
    points
        .checked_add(pairs)?
        .checked_add(groups)?
        .checked_add(jobs.checked_mul(3 * 64)?)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn workspace_grouping_linear<C: CurveConfig>(n: usize, jobs: usize) -> Option<usize>
where
    C::Base: BatchInverse,
{
    // Exact-sized point storage, push-grown pair indices and reserve_exact
    // lane groups. Per-job minima/margins are charged before aggregation.
    let each = core::mem::size_of::<LimbPoint<C::Base>>()
        .checked_add(core::mem::size_of::<usize>())?
        .checked_add(core::mem::size_of::<Group<C::Base>>().div_ceil(2 * LANES))?;
    n.checked_mul(each)?
        .checked_add(jobs.checked_mul(4 * core::mem::size_of::<usize>() + 3 * 64)?)
}

// Full batches need no scalar-tail factors. Keep their chains in the
// existing vector representation through the existing product-tree inverse.
#[inline(always)]
fn direct_chain_inverse(chains:&FqBatch, complete:bool, vector_inverse:bool)->Option<FqBatch> {
    #[cfg(all(zopro_direct_chain_inverse,feature="ifma8",target_arch="x86_64",
        target_feature="avx512ifma",target_feature="avx512vl",
        target_feature="avx512dq",not(no_ifma)))]
    if complete && vector_inverse {return Some(chains.inverse_nonzero());}
    let _=(chains,complete,vector_inverse);
    None
}

#[cfg(all(test,feature="ifma8",target_arch="x86_64",
    target_feature="avx512ifma",target_feature="avx512vl",
    target_feature="avx512dq",not(no_ifma)))]
mod direct_chain_inverse_tests {
    use super::*;
    #[test]
    fn complete_lazy_chains_match_scalar_inverse_oracle() {
        let mut seed=Fq::from(982371u64);
        let mut checks=0;
        for case in 0..513 {
            let mut expected=[Fq::ONE;LANES];
            let mut chains=FqBatch::one();
            for round in 0..1+case%7 {
                let factors=core::array::from_fn(|i| {
                    seed=seed.square()+Fq::from((case*31+round*17+i+1)as u64);
                    let value=match case {
                        0=>Fq::ONE,
                        1=>-Fq::ONE,
                        2 if i%2==0=>Fq::ONE,
                        _=>if seed.is_zero(){Fq::ONE}else{seed},
                    };
                    expected[i]*=value;
                    value
                });
                chains=chains.mul(&FqBatch::new(&factors));
            }
            assert!(direct_chain_inverse(&chains,false,true).is_none());
            assert!(direct_chain_inverse(&chains,true,false).is_none());
            let direct=direct_chain_inverse(&chains,true,true);
            assert_eq!(direct.is_some(),cfg!(zopro_direct_chain_inverse));
            let got=direct.unwrap_or_else(||FqBatch::new(&invert_lanes(chains.get(),true))).get();
            for i in 0..LANES {
                assert_eq!(got[i],expected[i].inverse().unwrap(),"case={case} lane={i}");
                checks+=1;
            }
        }
        println!("DIRECT_CHAIN_INVERSE_ORACLES={checks}");
    }
}

#[cfg(all(zopro_g2_prefix_records,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
mod g2_prefix;
