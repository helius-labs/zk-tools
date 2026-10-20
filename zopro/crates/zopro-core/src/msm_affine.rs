//! The native CPU MSM: signed-window Pippenger whose buckets are affine
//! points, filled in one pass over the bases in their own order, a batch of
//! additions sharing one inversion (as gnark-crypto's MultiExp does).
//!
//! The lane-form MSM (msm.rs) sorts each window's points into a bucket-
//! ordered copy and reduces it in passes; that suits WebAssembly's SIMD
//! lanes, but a phone's caches do not hold the copies: on a Cortex-A78 it
//! took five times gnark's last-level cache misses per second. Here a
//! window touches the bases once, in order, and its bucket array.
use crate::msm::{BatchAdd, BatchInverse};
use rayon::prelude::*;
use zopro_bn254::{Affine, CurveConfig, Field, Fr, Projective};

/// The window width, over the widths whose signed digits fit an i16. The
/// work is the windows' sum of n batch additions and 2^c bucket sums, a
/// sum (projective) weighing two additions, as single-threaded G2 MSMs on
/// an M5 Pro fit. The windows run as the pool's tasks, so among the widths
/// within 15 % of the least work the one whose windows take the fewest
/// rounds on the pool's threads wins: 8 → 2's G2 MSM (190 k points) at
/// width 14 took 19 windows on 18 threads, two rounds, 94 ms a proof; at
/// 15, 17 windows, one round, 74 ms (24 → 2: 240 and 181). Below that the
/// least work: 2 → 2 (54 k) at 11 took 43 ms a proof against 12's 31.
fn width(n: usize) -> usize {
    if n >= (1 << 18) {
        if let Some(width) = std::env::var("ZOPRO_CPU_LARGE_WIDTH")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
        {
            return width.clamp(10, 15);
        }
    }
    if let Some(width) = std::env::var("ZOPRO_CPU_WIDTH")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        return width;
    }
    // Wide bucket tables compete with each other and inversion scratch in
    // the CPU caches. Native IFMA's affine reduction handles the narrow
    // final digit efficiently, so large MSMs can use smaller tables.
    if n >= (1 << 18) && crate::simd::cpu_option("ZOPRO_CPU_CACHE_WIDTH") {
        return 13;
    }
    let bits = Fr::MODULUS_BITS as usize + 1;
    let threads = rayon::current_num_threads().max(1);
    let work = |c: usize| (n + (2usize << c)) * bits.div_ceil(c);
    let least = (4..=15).map(work).min().unwrap();
    (4..=15)
        .filter(|&c| work(c) <= least + least * 15 / 100)
        .min_by_key(|&c| (bits.div_ceil(c).div_ceil(threads), work(c)))
        .unwrap()
}

/// Point ranges per window (ZOPRO_CPU_SPLIT, experiment): more tasks than
/// windows for the pool, each with its own buckets.
fn split() -> usize {
    std::env::var("ZOPRO_CPU_SPLIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
        .max(1)
}

/// Additions per shared inversion, by width (gnark's): wide enough that the
/// inversion is cheap per addition, narrow enough that a random batch rarely
/// meets a bucket twice.
fn batch_size(width: usize) -> usize {
    let size: usize = match width {
        0..=9 => 0,
        10 => 80,
        11 => 150,
        12 => 200,
        13 => 350,
        14 => 400,
        _ => 500,
    };
    if crate::simd::cpu_option("ZOPRO_CPU_ALIGN_BATCH") {
        size.next_multiple_of(crate::simd::BATCH_LANES)
    } else {
        size
    }
}

pub(super) fn msm<C: CurveConfig>(bases: &[Affine<C>], scalars: &[Fr]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    let canonical: Vec<[u64; 4]> = scalars.par_iter().map(Fr::to_canonical).collect();
    msm_canonical(bases, &canonical)
}

/// `scalars` in place as their canonical limbs, for msm_canonical: a
/// proof's scalars, which it needs no more after its MSMs.
pub(super) fn canonical(scalars: &mut [Fr]) -> &[[u64; 4]] {
    scalars
        .par_iter_mut()
        .for_each(|s| *s = Fr::from_montgomery(s.to_canonical()));
    // SAFETY: Fr is repr(transparent) over its four limbs.
    unsafe { std::slice::from_raw_parts(scalars.as_ptr().cast::<[u64; 4]>(), scalars.len()) }
}

/// The MSM of scalars given as canonical limbs. Each window computes its
/// digits from them as it goes (`digit`), so the MSM holds no digits: 63
/// MB for 16 -> 2's five MSMs at once, at no cost in time on a phone.
pub(super) fn msm_canonical<C: CurveConfig>(
    bases: &[Affine<C>],
    canonical: &[[u64; 4]],
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    if crate::simd::cpu_option("ZOPRO_CPU_DEDUP") {
        if let Some((bases, scalars)) = group_scalars(bases, canonical) {
            return msm_raw(&bases, &scalars);
        }
    }
    msm_raw(bases, canonical)
}

/// Combine equal scalars using an affine addition tree. Inspect a small
/// sample first, so random FFT coefficients keep their streaming path.
fn group_scalars<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
) -> Option<(Vec<Affine<C>>, Vec<[u64; 4]>)>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), scalars.len());
    if scalars.len() < 256 || scalars.len() >= u32::MAX as usize {
        return None;
    }
    let sample = scalars.len().min(2048);
    let unique: std::collections::HashSet<_> = scalars
        .iter()
        .step_by(scalars.len().div_ceil(sample))
        .take(sample)
        .collect();
    if unique.len() * 200 > sample * 199 {
        return None;
    }
    let mut groups = std::collections::HashMap::new();
    let mut values = Vec::new();
    let mut classes = vec![u32::MAX; scalars.len()];
    let mut counts: Vec<usize> = Vec::new();
    for ((base, scalar), class) in bases.iter().zip(scalars).zip(&mut classes) {
        if *scalar == [0; 4] || base.is_identity() {
            continue;
        }
        let index = *groups.entry(*scalar).or_insert_with(|| {
            values.push(*scalar);
            counts.push(0);
            (values.len() - 1) as u32
        });
        *class = index;
        counts[index as usize] += 1;
    }
    drop(groups);
    if values.len() * 10 > scalars.len() * 8 {
        return None;
    }
    if values.is_empty() {
        return Some((Vec::new(), Vec::new()));
    }
    let mut starts = Vec::with_capacity(counts.len() + 1);
    starts.push(0);
    for &count in &counts {
        starts.push(starts.last().unwrap() + count);
    }
    let mut positions = starts[..counts.len()].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, &class) in classes.iter().enumerate() {
        if class != u32::MAX {
            order[positions[class as usize]] = i as u32;
            positions[class as usize] += 1;
        }
    }
    drop(classes);
    drop(positions);
    Some((
        crate::msm::group_affine(bases, &order, &starts, &counts),
        values,
    ))
}

fn msm_raw<C: CurveConfig>(bases: &[Affine<C>], canonical: &[[u64; 4]]) -> Projective<C>
where
    C::Base: BatchInverse,
{
    assert_eq!(bases.len(), canonical.len(), "validated MSM dimensions");
    let n = bases.len();
    if n == 0 {
        return Projective::<C>::IDENTITY;
    }
    // Small MSMs benefit from shorter streaming tasks on a wide pool.
    // Keep the large quotient MSM's existing path; splitting it did not
    // show a repeatable gain. Portable and ARM defaults are unchanged.
    let default_parts = if (1024..(1 << 18)).contains(&n)
        && rayon::current_num_threads() >= 16
        && crate::simd::cpu_option("ZOPRO_CPU_MERGE_SPLIT")
    {
        2
    } else {
        1
    };
    let merge_parts = std::env::var("ZOPRO_CPU_MERGE_SPLIT")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default_parts)
        .clamp(1, 32);
    if merge_parts > 1 && n >= 1024 {
        return merge_split(bases, canonical, merge_parts);
    }
    let width = width(n);
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    let parts = split();
    let part = n.div_ceil(parts);
    let pieces: Vec<Projective<C>> = (0..windows * parts)
        .into_par_iter()
        .map(|task| {
            let (w, p) = (task / parts, task % parts);
            let range = p * part..((p + 1) * part).min(n);
            if let Some(sum) =
                top_window::<C>(&bases[range.clone()], &canonical[range.clone()], w, width)
            {
                return sum;
            }
            window::<C>(
                &bases[range.clone()],
                canonical[range].iter().map(|s| digit(s, w, width)),
                width,
            )
        })
        .collect();
    let sums: Vec<Projective<C>> = pieces.chunks(parts).map(|c| c.iter().sum()).collect();
    let mut result = Projective::<C>::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..width {
            result = result.double();
        }
        result += sum;
    }
    result
}

/// Scalar `words`' signed digit in window `w`, in (-2^(width-1),
/// 2^(width-1)]: the window's bits plus the carry out of the window below,
/// which is one when that window's bits are at least half the radix, none
/// when they are less than half less one, and the carry into it when they
/// are just that (rarely: one in 2^width). The point at infinity's windows
/// skip it.
#[inline(always)]
fn digit(words: &[u64; 4], w: usize, width: usize) -> i16 {
    let half = 1u64 << (width - 1);
    let bits = |w: usize| -> u64 {
        let offset = w * width;
        let (word, shift) = (offset / 64, offset % 64);
        let mut value = if word < 4 { words[word] >> shift } else { 0 };
        if shift + width > 64 && word < 3 {
            value |= words[word + 1] << (64 - shift);
        }
        value & ((1u64 << width) - 1)
    };
    let mut carry = 0;
    let mut below = w;
    while below > 0 {
        let b = bits(below - 1);
        if b != half - 1 {
            carry = (b >= half) as u64;
            break;
        }
        below -= 1;
    }
    let value = bits(w) + carry;
    let out = (value + half) >> width;
    (value as i64 - (out << width) as i64) as i16
}

fn window<C: CurveConfig>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    width: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    let half = 1usize << (width - 1);
    let size = batch_size(width);
    if size == 0 {
        return window_projective(bases, digits, half);
    }
    let w = fill_window::<C>(bases, digits, width, size);
    if crate::simd::cpu_option("ZOPRO_CPU_REDUCE") {
        return crate::msm::weighted_affine(&affine_buckets(w));
    }
    let mut running = Projective::<C>::IDENTITY;
    let mut total = Projective::<C>::IDENTITY;
    for k in (0..half).rev() {
        running += w.affine[k];
        let index = w.projective_indices[k];
        if index != u32::MAX {
            running += w.projective[index as usize];
        }
        total += running;
    }
    total
}

fn fill_window<C: CurveConfig>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    width: usize,
    size: usize,
) -> Window<C>
where
    C::Base: BatchInverse,
{
    let mut w = Window::<C>::new(1usize << (width - 1), size);
    for (base, digit) in bases.iter().zip(digits) {
        if digit == 0 || base.is_identity() {
            continue;
        }
        let bucket = digit.unsigned_abs() as usize - 1;
        let point = if digit < 0 { -*base } else { *base };
        if w.busy(bucket) {
            w.queue.push((bucket as u32, point));
            if w.queue.len() == size - 1 {
                w.flush_queue();
            }
            continue;
        }
        w.add(bucket, point);
        if w.batch.len() == size {
            w.execute();
            w.take_from_queue();
        }
    }
    w.execute();
    w.flush_queue();
    w
}

fn affine_buckets<C: CurveConfig>(mut w: Window<C>) -> Vec<Affine<C>>
where
    C::Base: BatchInverse,
{
    let combined: Vec<_> = w
        .projective
        .iter()
        .zip(&w.projective_buckets)
        .map(|(point, &i)| *point + w.affine[i])
        .collect();
    for (point, &i) in Projective::<C>::batch_to_affine(&combined)
        .into_iter()
        .zip(&w.projective_buckets)
    {
        w.affine[i] = point;
    }
    w.affine
}

/// Merge disjoint equal-weight buckets using the streaming affine kernel.
fn merge_buckets<C: CurveConfig>(left: &mut [Affine<C>], right: &[Affine<C>])
where
    C::Base: BatchInverse,
{
    assert_eq!(left.len(), right.len());
    left.par_chunks_mut(512)
        .zip(right.par_chunks(512))
        .for_each(|(left, right)| {
            let mut pairs = Vec::with_capacity(left.len());
            for (i, (a, &b)) in left.iter_mut().zip(right).enumerate() {
                if a.is_identity() {
                    *a = b;
                } else if b.is_identity() {
                } else if a.x == b.x {
                    *a = (*a + b).to_affine();
                } else {
                    pairs.push((i as u32, b));
                }
            }
            if !pairs.is_empty() {
                BatchAdd::<C>::default().apply(left, &pairs);
            }
        });
}

fn merge_split<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    parts: usize,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    let part = bases.len().div_ceil(parts);
    let width = width(bases.len());
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    let size = batch_size(width).max(32);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            if let Some(sum) = top_window::<C>(bases, scalars, w, width) {
                return sum;
            }
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|p| {
                    let range = p * part..((p + 1) * part).min(bases.len());
                    affine_buckets(fill_window::<C>(
                        &bases[range.clone()],
                        scalars[range].iter().map(|s| digit(s, w, width)),
                        width,
                        size,
                    ))
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for other in buckets {
                merge_buckets(&mut combined, &other);
            }
            crate::msm::weighted_affine(&combined)
        })
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

/// A short final digit uses only a few buckets. Streaming these collisions
/// would send most points through the scalar projective fallback.
fn top_window<C: CurveConfig>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    w: usize,
    width: usize,
) -> Option<Projective<C>>
where
    C::Base: BatchInverse,
{
    let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
    if bases.len() < 1024 || w + 1 != windows {
        return None;
    }
    let bits = Fr::MODULUS_BITS as usize - w * width;
    if bits > 8 || !crate::simd::cpu_option("ZOPRO_CPU_TOP_TREE") {
        return None;
    }
    let buckets = 1usize << bits;
    let mut counts = vec![0usize; buckets];
    for (base, s) in bases.iter().zip(scalars) {
        let d = digit(s, w, width);
        debug_assert!(d >= 0 && d as usize <= buckets);
        if d != 0 && !base.is_identity() {
            counts[d as usize - 1] += 1;
        }
    }
    let mut starts = Vec::with_capacity(buckets + 1);
    starts.push(0);
    for count in counts {
        starts.push(starts.last().unwrap() + count);
    }
    if *starts.last().unwrap() == 0 {
        return Some(Projective::IDENTITY);
    }
    let mut positions = starts[..buckets].to_vec();
    let mut order = vec![0u32; *starts.last().unwrap()];
    for (i, (base, s)) in bases.iter().zip(scalars).enumerate() {
        let d = digit(s, w, width);
        if d != 0 && !base.is_identity() {
            let b = d as usize - 1;
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    let buckets = crate::msm::reduce_indexed_buckets(bases, &order, &starts);
    Some(crate::msm::weighted_affine(&buckets))
}

/// Narrow windows: few buckets, so projective buckets without batching.
fn window_projective<C: CurveConfig>(
    bases: &[Affine<C>],
    digits: impl Iterator<Item = i16>,
    half: usize,
) -> Projective<C> {
    let mut buckets = vec![Projective::<C>::IDENTITY; half];
    for (base, digit) in bases.iter().zip(digits) {
        if digit > 0 {
            buckets[digit as usize - 1] += base;
        } else if digit < 0 {
            buckets[digit.unsigned_abs() as usize - 1] -= base;
        }
    }
    let mut running = Projective::<C>::IDENTITY;
    let mut total = Projective::<C>::IDENTITY;
    for bucket in buckets.iter().rev() {
        running += bucket;
        total += running;
    }
    total
}

/// A window's buckets and its pending batch of affine additions.
struct Window<C: CurveConfig>
where
    C::Base: BatchInverse,
{
    /// The buckets the batches fill.
    affine: Vec<Affine<C>>,
    /// The rare additions a batch cannot take (doublings, and the queue
    /// when it overflows), in projective form.
    projective: Vec<Projective<C>>,
    projective_indices: Vec<u32>,
    projective_buckets: Vec<usize>,
    /// The buckets in the current batch.
    busy: Vec<u64>,
    /// The batch: bucket and point to add to it.
    batch: Vec<(u32, Affine<C>)>,
    /// Points whose bucket was in the batch when they came.
    queue: Vec<(u32, Affine<C>)>,
    /// The batch inversion's scratch.
    denominators: Vec<C::Base>,
    prefixes: Vec<C::Base>,
    packed: BatchAdd<C>,
    use_packed: bool,
}

impl<C: CurveConfig> Window<C>
where
    C::Base: BatchInverse,
{
    fn new(half: usize, size: usize) -> Self {
        Self {
            affine: vec![Affine::<C>::IDENTITY; half],
            projective: Vec::new(),
            projective_indices: vec![u32::MAX; half],
            projective_buckets: Vec::new(),
            busy: vec![0; half.div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            denominators: Vec::with_capacity(size),
            prefixes: Vec::with_capacity(size),
            packed: BatchAdd::default(),
            use_packed: crate::simd::cpu_option("ZOPRO_CPU_PACKED"),
        }
    }

    fn busy(&self, bucket: usize) -> bool {
        self.busy[bucket / 64] >> (bucket % 64) & 1 == 1
    }

    /// Adds `point` to a bucket not in the batch: at once when the bucket is
    /// empty, or the sum is a doubling or zero; else into the batch.
    fn add(&mut self, bucket: usize, point: Affine<C>) {
        let current = &mut self.affine[bucket];
        if current.is_identity() {
            *current = point;
            return;
        }
        if current.x == point.x {
            if current.y == point.y {
                self.add_projective(bucket, point);
            } else {
                *current = Affine::<C>::IDENTITY;
            }
            return;
        }
        self.busy[bucket / 64] |= 1 << (bucket % 64);
        self.batch.push((bucket as u32, point));
    }

    /// Most buckets never need the exceptional projective path. Keep only
    /// those that do, rather than a second dense point array per window.
    fn add_projective(&mut self, bucket: usize, point: Affine<C>) {
        let index = self.projective_indices[bucket];
        if index == u32::MAX {
            self.projective_indices[bucket] = self.projective.len() as u32;
            self.projective_buckets.push(bucket);
            self.projective.push(point.to_projective());
        } else {
            self.projective[index as usize] += point;
        }
    }

    /// The batch's additions, bucket = bucket + point, with one inversion:
    /// λ = (y2 - y1) / (x2 - x1), x3 = λ² - x1 - x2, y3 = λ (x1 - x3) - y1.
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        if self.use_packed {
            self.packed.apply(&mut self.affine, &self.batch);
            for (bucket, _) in &self.batch {
                self.busy[*bucket as usize / 64] = 0;
            }
            self.batch.clear();
            return;
        }
        self.denominators.clear();
        self.prefixes.clear();
        let mut product = C::Base::ONE;
        for (bucket, point) in &self.batch {
            let denominator = point.x - self.affine[*bucket as usize].x;
            self.prefixes.push(product);
            product *= denominator;
            self.denominators.push(denominator);
        }
        let mut inverse = product.inverse().expect("distinct x in a batch");
        for j in (0..self.batch.len()).rev() {
            let (bucket, point) = self.batch[j];
            let lambda_denominator = inverse * self.prefixes[j];
            inverse = inverse * self.denominators[j];
            let current = &mut self.affine[bucket as usize];
            let lambda = (point.y - current.y) * lambda_denominator;
            let x = lambda.square() - current.x - point.x;
            let y = lambda * (current.x - x) - current.y;
            *current = Affine::<C>::new(x, y);
            self.busy[bucket as usize / 64] = 0;
        }
        self.batch.clear();
    }

    /// The queued points whose buckets are free again, newest first, into
    /// the next batch.
    fn take_from_queue(&mut self) {
        while let Some(&(bucket, point)) = self.queue.last() {
            if self.busy(bucket as usize) {
                return;
            }
            self.queue.pop();
            self.add(bucket as usize, point);
        }
    }

    /// The queue's points into the projective buckets, unbatched.
    fn flush_queue(&mut self) {
        for i in 0..self.queue.len() {
            let (bucket, point) = self.queue[i];
            self.add_projective(bucket as usize, point);
        }
        self.queue.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::Seeded;
    use zopro_bn254::{G1Config, G2Config};

    #[test]
    fn equal_scalars_group_correctly() {
        fn check<C: CurveConfig>()
        where
            C::Base: BatchInverse,
        {
            let mut rng = Seeded::new(91);
            let choices = [
                Fr::ZERO,
                Fr::ONE,
                -Fr::ONE,
                rng.fr(),
                rng.fr(),
                rng.fr(),
                rng.fr(),
            ];
            let g = Projective::<C>::generator();
            let mut point = g;
            let bases: Vec<_> = (0..512)
                .map(|i| {
                    point += g;
                    match i % 19 {
                        0 => Affine::IDENTITY,
                        1 => g.to_affine(),
                        2 => (-g).to_affine(),
                        _ => point.to_affine(),
                    }
                })
                .collect();
            let scalars: Vec<_> = (0..512)
                .map(|i| choices[i % choices.len()].to_canonical())
                .collect();
            let (grouped, values) = group_scalars(&bases, &scalars).expect("repeated scalars");
            assert_eq!(
                msm_raw(&grouped, &values).to_affine(),
                msm_raw(&bases, &scalars).to_affine()
            );
            let zero = vec![[0; 4]; 512];
            let (grouped, values) = group_scalars(&bases, &zero).unwrap();
            assert!(grouped.is_empty() && values.is_empty());
        }
        check::<G1Config>();
        check::<G2Config>();
    }

    /// Each window's digit against recoding the scalar from its lowest
    /// window up, the carry passed along: for scalars whose windows are
    /// just below half the radix in runs, where a digit's carry comes from
    /// windows further down, and for random ones; and the digits rebuild
    /// the scalar.
    #[test]
    fn digits_match_a_sequential_recoding() {
        let mut seeded = Seeded::new(7);
        for width in 4..=15usize {
            let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
            let half = 1u64 << (width - 1);
            let mut scalars: Vec<[u64; 4]> = (0..64).map(|_| seeded.fr().to_canonical()).collect();
            for run in [1, 2, 3, 7, windows] {
                for start in [0, 1, windows / 2] {
                    // Below 2^252, so still below the modulus.
                    let mut words = seeded.fr().to_canonical();
                    words[3] &= (1 << 60) - 1;
                    for w in start..(start + run).min(windows - 1) {
                        for bit in 0..width {
                            let at = w * width + bit;
                            if at < 252 {
                                let one = (half - 1) >> bit & 1 == 1;
                                let (word, shift) = (at / 64, at % 64);
                                words[word] = words[word] & !(1 << shift) | (one as u64) << shift;
                            }
                        }
                    }
                    scalars.push(words);
                }
            }
            for words in &scalars {
                let mut carry = 0u64;
                let mut rebuilt = Fr::ZERO;
                let radix = Fr::from(1u64 << width);
                for w in (0..windows).rev() {
                    rebuilt = rebuilt * radix + {
                        let d = digit(words, w, width) as i64;
                        if d < 0 {
                            -Fr::from(d.unsigned_abs())
                        } else {
                            Fr::from(d as u64)
                        }
                    };
                }
                for w in 0..windows {
                    let offset = w * width;
                    let (word, shift) = (offset / 64, offset % 64);
                    let mut value = if word < 4 { words[word] >> shift } else { 0 };
                    if shift + width > 64 && word < 3 {
                        value |= words[word + 1] << (64 - shift);
                    }
                    let value = (value & ((1 << width) - 1)) + carry;
                    carry = (value + half) >> width;
                    let expected = (value as i64 - (carry << width) as i64) as i16;
                    assert_eq!(
                        digit(words, w, width),
                        expected,
                        "width {width}, window {w}"
                    );
                }
                assert_eq!(rebuilt.to_canonical(), *words, "width {width}");
            }
        }
    }

    /// Against the sum of the products, with the special cases the batch
    /// must step around: zero and unit scalars, repeated bases (doublings in
    /// a bucket), a base and its negation (zero sums), the point at
    /// infinity; past `naive` points against the lane-form MSM.
    fn check<C: CurveConfig>(sizes: &[usize], naive: usize)
    where
        C::Base: crate::msm::BatchInverse,
    {
        let mut rng = Seeded::new(5);
        let max = *sizes.iter().max().unwrap();
        let g = Projective::<C>::generator();
        let step = g.mul(&rng.fr());
        let mut p = g.mul(&rng.fr());
        let mut all = Vec::with_capacity(max);
        for i in 0..max {
            all.push(match i % 20 {
                0 => Projective::<C>::IDENTITY,
                1..=3 => g,
                4 => -g,
                _ => {
                    p += step;
                    p
                }
            });
        }
        let bases = Projective::<C>::batch_to_affine(&all);
        let scalars: Vec<Fr> = (0..max)
            .map(|i| match i % 17 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                3 => Fr::from(7u64),
                _ => rng.fr(),
            })
            .collect();
        let reference = |bases: &[Affine<C>], scalars: &[Fr]| -> Projective<C> {
            if bases.len() <= naive {
                bases.iter().zip(scalars).map(|(b, s)| b.mul(s)).sum()
            } else {
                crate::msm::msm(bases, scalars)
            }
        };
        for &n in sizes {
            assert_eq!(
                msm::<C>(&bases[..n], &scalars[..n]),
                reference(&bases[..n], &scalars[..n]),
                "{n} points"
            );
        }
        // One scalar repeated: every point of a window in one bucket.
        let same = vec![Fr::from(12345u64); 5000];
        assert_eq!(
            msm::<C>(&bases[..5000], &same),
            reference(&bases[..5000], &same),
            "one scalar"
        );
    }

    #[test]
    fn g1_matches_reference() {
        check::<G1Config>(&[0, 1, 255, 256, 1023, 1024, 4096, 16384, 70000], 1024);
    }

    #[test]
    fn g2_matches_reference() {
        check::<G2Config>(&[1, 200, 256, 4096, 20000], 256);
    }
}
