//! Balanced ordinary width-12 windows; borrowed inputs and existing kernels.
use super::*;
const COUNT: usize = 22;
const BITS: usize = 255;
#[derive(Clone, Copy, Debug)]
struct Span {
    start: usize,
    width: usize,
}
fn spans() -> [Span; COUNT] {
    core::array::from_fn(|i| {
        let start = i * BITS / COUNT;
        Span {
            start,
            width: (i + 1) * BITS / COUNT - start,
        }
    })
}
#[inline(always)]
fn digit_at(s: &[u64; 4], span: Span) -> i16 {
    let Span { start, width } = span;
    if start == 0 {
        let value = s[0] & ((1u64 << width) - 1);
        return (value as i64 - ((value >> (width - 1)) << width) as i64) as i16;
    }
    let offset = start - 1;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { s[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        value |= s[word + 1] << (64 - shift);
    }
    value &= (1u64 << (width + 1)) - 1;
    (((value + 1) >> 1) as i64 - ((value >> width) << width) as i64) as i16
}
fn feed<'a, C: CurveConfig<Base = Fq>>(
    state: &mut Window<'a, C>,
    points: &'a [Affine<C>],
    scalars: &[[u64; 4]],
    span: Span,
    size: usize,
) {
    // Same borrowed pending/collision algorithm; only the digit reader differs.
    for (point, scalar) in points.iter().zip(scalars) {
        let d = digit_at(scalar, span);
        if d == 0 || point.is_identity() {
            continue;
        }
        let bucket = d.unsigned_abs() as usize - 1;
        let encoded = bucket as u32 | if d < 0 { 1 << 31 } else { 0 };
        if state.busy(bucket) {
            state.queue.push((encoded, point));
            if state.queue.len() == size - 1 {
                state.flush_queue();
            }
            continue;
        }
        state.add(encoded, point);
        if state.batch.len() == size {
            state.execute();
            state.take_from_queue();
        }
    }
}
pub(super) fn msm<C: CurveConfig<Base = Fq>>(
    bases: &[Affine<C>],
    scalars: &[[u64; 4]],
    parts: usize,
) -> Projective<C> {
    assert_eq!(bases.len(), scalars.len());
    assert!((1..=2).contains(&parts));
    let part = bases.len().div_ceil(parts);
    let cover = spans();
    let sums: Vec<_> = cover
        .par_iter()
        .map(|&span| {
            let size = batch_size(span.width).max(32);
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|p| {
                    let range = p * part..((p + 1) * part).min(bases.len());
                    let mut state = Window::new(1usize << (span.width - 1), size);
                    feed(
                        &mut state,
                        &bases[range.clone()],
                        &scalars[range],
                        span,
                        size,
                    );
                    state.finish()
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for other in buckets {
                super::super::merge_buckets(&mut combined, &other);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = Projective::<C>::IDENTITY;
    for (span, sum) in cover.iter().zip(&sums).rev() {
        for _ in 0..span.width {
            result = result.double();
        }
        result += sum;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::Seeded;
    use zopro_bn254::{G1Affine, G1Config, G1Projective};
    fn add_signed(sum: &mut [u64; 5], digit: i16, start: usize) {
        let mut term = [0u64; 5];
        let j = start / 64;
        let bit = start % 64;
        let v = digit.unsigned_abs() as u64;
        term[j] = v << bit;
        if bit != 0 && j + 1 < 5 {
            term[j + 1] = v >> (64 - bit);
        }
        if digit >= 0 {
            let mut carry = 0u128;
            for i in 0..5 {
                let x = sum[i] as u128 + term[i] as u128 + carry;
                sum[i] = x as u64;
                carry = x >> 64;
            }
        } else {
            let mut borrow = false;
            for i in 0..5 {
                let (a, b) = sum[i].overflowing_sub(term[i]);
                let (c, d) = a.overflowing_sub(u64::from(borrow));
                sum[i] = c;
                borrow = b || d;
            }
        }
    }
    #[test]
    fn variable_booth_digits_reconstruct_exact_integers() {
        let cover = spans();
        assert_eq!(cover.len(), 22);
        assert_eq!(cover.iter().map(|s| s.width).sum::<usize>(), 255);
        assert_eq!(cover.iter().filter(|s| s.width == 11).count(), 9);
        assert_eq!(cover.iter().filter(|s| s.width == 12).count(), 13);
        assert_eq!(
            cover.iter().map(|s| 1usize << (s.width - 1)).sum::<usize>(),
            35840
        );
        let mut rng = Seeded::new(0x62616c616e636564);
        let mut inputs = vec![Fr::ZERO, Fr::ONE, -Fr::ONE];
        for bit in 0..254 {
            let x = Fr::from(2u64).pow(&[bit]);
            inputs.extend([x, x - Fr::ONE, x + Fr::ONE]);
        }
        inputs.extend((0..4096).map(|_| rng.fr()));
        for s in &inputs {
            let words = s.to_canonical();
            let bit = |n: usize| -> i64 {
                if n >= 256 {
                    0
                } else {
                    ((words[n / 64] >> (n % 64)) & 1) as i64
                }
            };
            let mut reconstructed = [0; 5];
            for &span in &cover {
                let unsigned = (0..span.width)
                    .map(|i| bit(span.start + i) << i)
                    .sum::<i64>();
                let incoming = if span.start == 0 {
                    0
                } else {
                    bit(span.start - 1)
                };
                let expected =
                    unsigned + incoming - (bit(span.start + span.width - 1) << span.width);
                let d = digit_at(&words, span);
                assert_eq!(d as i64, expected);
                assert!(d.unsigned_abs() as usize <= 1usize << (span.width - 1));
                add_signed(&mut reconstructed, d, span.start);
            }
            assert_eq!(&reconstructed[..4], &words);
            assert_eq!(reconstructed[4], 0);
            let top = digit_at(&words, cover[21]);
            assert!(top >= 0 && top <= 2048);
        }
        println!(
            "ORDINARY_BALANCED_EXACT_SCALARS={} DIGITS={}",
            inputs.len(),
            inputs.len() * 22
        );
    }
    fn points(n: usize, base: u64) -> (Vec<G1Affine>, Vec<Fr>) {
        let g = G1Projective::generator();
        let mut current = g * Fr::from(base);
        let mut p = Vec::with_capacity(n);
        let mut weights = Vec::with_capacity(n);
        for i in 0..n {
            current += G1Affine::GENERATOR;
            let (mut point, mut weight) = (current, Fr::from(base + i as u64 + 1));
            if i % 17 == 0 {
                point = G1Projective::IDENTITY;
                weight = Fr::ZERO;
            } else if i % 11 == 0 {
                let sign = if i % 2 == 0 { Fr::ONE } else { -Fr::ONE };
                point = g * sign;
                weight = sign;
            } else if i % 3 == 0 {
                point = -point;
                weight = -weight;
            }
            p.push(point);
            weights.push(weight);
        }
        (G1Projective::batch_to_affine(&p), weights)
    }
    #[test]
    fn ordinary_balanced_generator_oracle_and_owned_control() {
        for threads in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                let mut rng = Seeded::new(0x6f7264696e617279);
                for n in [0, 1, 17, 1024, 21006, 35973, 65535] {
                    let (bases, weights) = points(n, 37);
                    for kind in 0..6 {
                        let mut expected = Fr::ZERO;
                        let scalars: Vec<_> = weights
                            .iter()
                            .enumerate()
                            .map(|(i, w)| {
                                let scalar = match kind {
                                    0 => Fr::ZERO,
                                    1 => Fr::ONE,
                                    2 => -Fr::ONE,
                                    3 => rng.fr(),
                                    4 => Fr::from((i % 5) as u64),
                                    _ => {
                                        let bit = [
                                            10, 11, 12, 21, 22, 23, 63, 64, 127, 128, 242, 243,
                                            252, 253,
                                        ][i % 14];
                                        let x = Fr::from(2u64).pow(&[bit]);
                                        if i % 2 == 0 {
                                            x - Fr::ONE
                                        } else {
                                            x + Fr::ONE
                                        }
                                    }
                                };
                                expected += scalar * *w;
                                scalar.to_canonical()
                            })
                            .collect();
                        let want = (G1Affine::GENERATOR * expected).to_affine();
                        for parts in [1, 2] {
                            assert_eq!(
                                msm::<G1Config>(&bases, &scalars, parts).to_affine(),
                                want,
                                "threads={threads} n={n} parts={parts} kind={kind}"
                            );
                            if n >= 1024 {
                                assert_eq!(
                                    super::super::super::merge_split_width_owned::<G1Config>(
                                        &bases, &scalars, parts, 12
                                    )
                                    .to_affine(),
                                    want,
                                    "owned threads={threads} n={n} parts={parts} kind={kind}"
                                );
                            }
                        }
                    }
                }
            });
        }
    }
}
