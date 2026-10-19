//! The native CPU MSM: signed-window Pippenger whose buckets are affine
//! points, filled in one pass over the bases in their own order, a batch of
//! additions sharing one inversion (as gnark-crypto's MultiExp does).
//!
//! The lane-form MSM (msm.rs) sorts each window's points into a bucket-
//! ordered copy and reduces it in passes; that suits WebAssembly's SIMD
//! lanes, but a phone's caches do not hold the copies: on a Cortex-A78 it
//! took five times gnark's last-level cache misses per second. Here a
//! window touches the bases once, in order, and its bucket array.
use ark_bn254::Fr;
use ark_ec::{
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AdditiveGroup, VariableBaseMSM,
};
use ark_ff::{BigInt, Field, PrimeField};
use rayon::prelude::*;

/// The base field's product for the batch additions: Fq2's by Karatsuba
/// with lazy reduction on arm64 (ark_bn254::fq2::mul_lazy, 19 ns rather
/// than 21 on an M5 Pro), Fq's as arkworks'.
pub(super) trait BatchField: Field {
    fn times(&self, other: &Self) -> Self;
}

impl BatchField for ark_bn254::Fq {
    #[inline(always)]
    fn times(&self, other: &Self) -> Self {
        *self * other
    }
}

impl BatchField for ark_bn254::Fq2 {
    #[inline(always)]
    fn times(&self, other: &Self) -> Self {
        ark_bn254::fq2::mul_lazy(self, other)
    }
}

/// The window width: gnark's cost model, (bits + 1) (n + 2^c) / c, over the
/// widths whose signed digits fit an i16.
fn width(n: usize) -> usize {
    if let Some(width) = std::env::var("ZOPRO_CPU_WIDTH")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        return width;
    }
    let bits = Fr::MODULUS_BIT_SIZE as usize + 1;
    (4..=15)
        .min_by_key(|&c| bits * (n + (1usize << c)) / c)
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
    match width {
        0..=9 => 0,
        10 => 80,
        11 => 150,
        12 => 200,
        13 => 350,
        14 => 400,
        _ => 500,
    }
}

pub(super) fn msm<P: SWCurveConfig<ScalarField = Fr>>(
    bases: &[Affine<P>],
    scalars: &[Fr],
) -> Projective<P>
where
    P::BaseField: BatchField,
{
    assert_eq!(bases.len(), scalars.len(), "validated MSM dimensions");
    let n = bases.len();
    if n < 256 {
        return Projective::<P>::msm(bases, scalars).unwrap();
    }
    let width = width(n);
    let windows = (Fr::MODULUS_BIT_SIZE as usize + 1).div_ceil(width);
    let digits = recode(scalars, width, windows);
    let parts = split();
    let part = n.div_ceil(parts);
    let pieces: Vec<Projective<P>> = (0..windows * parts)
        .into_par_iter()
        .map(|task| {
            let (w, p) = (task / parts, task % parts);
            let range = p * part..((p + 1) * part).min(n);
            window::<P>(
                &bases[range.clone()],
                &digits[w * n + range.start..w * n + range.end],
                width,
            )
        })
        .collect();
    let sums: Vec<Projective<P>> = pieces.chunks(parts).map(|c| c.iter().sum()).collect();
    let mut result = Projective::<P>::ZERO;
    for sum in sums.iter().rev() {
        for _ in 0..width {
            result.double_in_place();
        }
        result += sum;
    }
    result
}

/// Signed digits, window-major: digits[w * n + i] is scalar i's digit in
/// window w, in (-2^(width-1), 2^(width-1)].
fn recode(scalars: &[Fr], width: usize, windows: usize) -> Vec<i16> {
    let n = scalars.len();
    let radix = 1u64 << width;
    let mask = radix - 1;
    let rows: Vec<Vec<i16>> = scalars
        .par_chunks(4096)
        .map(|chunk| {
            let mut rows = vec![0i16; chunk.len() * windows];
            for (i, s) in chunk.iter().enumerate() {
                let BigInt(words) = s.into_bigint();
                let mut carry = 0u64;
                for w in 0..windows {
                    let offset = w * width;
                    let (word, shift) = (offset / 64, offset % 64);
                    let mut value = if word < 4 { words[word] >> shift } else { 0 };
                    if shift + width > 64 && word < 3 {
                        value |= words[word + 1] << (64 - shift);
                    }
                    let value = (value & mask) + carry;
                    carry = (value + radix / 2) >> width;
                    rows[w * chunk.len() + i] = (value as i64 - (carry * radix) as i64) as i16;
                }
            }
            rows
        })
        .collect();
    let mut digits = vec![0i16; n * windows];
    let mut start = 0;
    for rows in rows {
        let len = rows.len() / windows;
        for w in 0..windows {
            digits[w * n + start..w * n + start + len]
                .copy_from_slice(&rows[w * len..(w + 1) * len]);
        }
        start += len;
    }
    digits
}

/// One window's sum, the sum of m * B_m over its buckets B_m.
fn window<P: SWCurveConfig>(bases: &[Affine<P>], digits: &[i16], width: usize) -> Projective<P>
where
    P::BaseField: BatchField,
{
    let half = 1usize << (width - 1);
    let size = batch_size(width);
    if size == 0 {
        return window_projective(bases, digits, half);
    }
    let mut w = Window::<P>::new(half, size);
    for (base, &digit) in bases.iter().zip(digits) {
        if digit == 0 || base.infinity {
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
    let mut running = Projective::<P>::ZERO;
    let mut total = Projective::<P>::ZERO;
    for k in (0..half).rev() {
        running += w.affine[k];
        running += w.projective[k];
        total += running;
    }
    total
}

/// Narrow windows: few buckets, so projective buckets without batching.
fn window_projective<P: SWCurveConfig>(
    bases: &[Affine<P>],
    digits: &[i16],
    half: usize,
) -> Projective<P> {
    let mut buckets = vec![Projective::<P>::ZERO; half];
    for (base, &digit) in bases.iter().zip(digits) {
        if digit > 0 {
            buckets[digit as usize - 1] += base;
        } else if digit < 0 {
            buckets[digit.unsigned_abs() as usize - 1] -= base;
        }
    }
    let mut running = Projective::<P>::ZERO;
    let mut total = Projective::<P>::ZERO;
    for bucket in buckets.iter().rev() {
        running += bucket;
        total += running;
    }
    total
}

/// A window's buckets and its pending batch of affine additions.
struct Window<P: SWCurveConfig> {
    /// The buckets the batches fill.
    affine: Vec<Affine<P>>,
    /// The rare additions a batch cannot take (doublings, and the queue
    /// when it overflows), in projective form.
    projective: Vec<Projective<P>>,
    /// The buckets in the current batch.
    busy: Vec<u64>,
    /// The batch: bucket and point to add to it.
    batch: Vec<(u32, Affine<P>)>,
    /// Points whose bucket was in the batch when they came.
    queue: Vec<(u32, Affine<P>)>,
    /// The batch inversion's scratch.
    denominators: Vec<P::BaseField>,
    prefixes: Vec<P::BaseField>,
}

impl<P: SWCurveConfig> Window<P>
where
    P::BaseField: BatchField,
{
    fn new(half: usize, size: usize) -> Self {
        Self {
            affine: vec![Affine::<P>::identity(); half],
            projective: vec![Projective::<P>::ZERO; half],
            busy: vec![0; half.div_ceil(64)],
            batch: Vec::with_capacity(size),
            queue: Vec::with_capacity(size),
            denominators: Vec::with_capacity(size),
            prefixes: Vec::with_capacity(size),
        }
    }

    fn busy(&self, bucket: usize) -> bool {
        self.busy[bucket / 64] >> (bucket % 64) & 1 == 1
    }

    /// Adds `point` to a bucket not in the batch: at once when the bucket is
    /// empty, or the sum is a doubling or zero; else into the batch.
    fn add(&mut self, bucket: usize, point: Affine<P>) {
        let current = &mut self.affine[bucket];
        if current.infinity {
            *current = point;
            return;
        }
        if current.x == point.x {
            if current.y == point.y {
                self.projective[bucket] += point;
            } else {
                *current = Affine::<P>::identity();
            }
            return;
        }
        self.busy[bucket / 64] |= 1 << (bucket % 64);
        self.batch.push((bucket as u32, point));
    }

    /// The batch's additions, bucket = bucket + point, with one inversion:
    /// λ = (y2 - y1) / (x2 - x1), x3 = λ² - x1 - x2, y3 = λ (x1 - x3) - y1.
    fn execute(&mut self) {
        if self.batch.is_empty() {
            return;
        }
        self.denominators.clear();
        self.prefixes.clear();
        let mut product = P::BaseField::ONE;
        for (bucket, point) in &self.batch {
            let denominator = point.x - self.affine[*bucket as usize].x;
            self.prefixes.push(product);
            product = product.times(&denominator);
            self.denominators.push(denominator);
        }
        let mut inverse = product.inverse().expect("distinct x in a batch");
        for j in (0..self.batch.len()).rev() {
            let (bucket, point) = self.batch[j];
            let lambda_denominator = inverse.times(&self.prefixes[j]);
            inverse = inverse.times(&self.denominators[j]);
            let current = &mut self.affine[bucket as usize];
            let lambda = (point.y - current.y).times(&lambda_denominator);
            let x = lambda.square() - current.x - point.x;
            let y = lambda.times(&(current.x - x)) - current.y;
            *current = Affine::<P>::new_unchecked(x, y);
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
        for (bucket, point) in self.queue.drain(..) {
            self.projective[bucket as usize] += point;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::{CurveGroup, PrimeGroup};
    use ark_ff::{UniformRand, Zero};
    use rand::{rngs::StdRng, SeedableRng};

    /// Against arkworks, with the special cases the batch must step around:
    /// zero and unit scalars, repeated bases (doublings in a bucket), a base
    /// and its negation (zero sums), the point at infinity.
    fn check<P: SWCurveConfig<ScalarField = Fr>>(sizes: &[usize])
    where
        P::BaseField: BatchField,
    {
        let mut rng = StdRng::seed_from_u64(5);
        let max = *sizes.iter().max().unwrap();
        let g = Projective::<P>::generator();
        let step = Projective::<P>::rand(&mut rng);
        let mut p = Projective::<P>::rand(&mut rng);
        let mut all = Vec::with_capacity(max);
        for i in 0..max {
            all.push(match i % 20 {
                0 => Projective::<P>::zero(),
                1..=3 => g,
                4 => -g,
                _ => {
                    p += step;
                    p
                }
            });
        }
        let bases = Projective::<P>::normalize_batch(&all);
        let scalars: Vec<Fr> = (0..max)
            .map(|i| match i % 17 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                3 => Fr::from(7u64),
                _ => Fr::rand(&mut rng),
            })
            .collect();
        for &n in sizes {
            assert_eq!(
                msm::<P>(&bases[..n], &scalars[..n]),
                Projective::<P>::msm(&bases[..n], &scalars[..n]).unwrap(),
                "{n} points"
            );
        }
        // One scalar repeated: every point of a window in one bucket.
        let same = vec![Fr::from(12345u64); 5000];
        assert_eq!(
            msm::<P>(&bases[..5000], &same),
            Projective::<P>::msm(&bases[..5000], &same).unwrap(),
            "one scalar"
        );
    }

    #[test]
    fn g1_matches_arkworks() {
        check::<ark_bn254::g1::Config>(&[0, 1, 255, 256, 1023, 1024, 4096, 16384, 70000]);
    }

    #[test]
    fn g2_matches_arkworks() {
        check::<ark_bn254::g2::Config>(&[256, 4096, 20000]);
    }
}
