//! Eleven balanced signed spans for the existing two-component G2 query.
use super::*;
use rayon::prelude::*;
const COUNT: usize = 11;
const BITS: usize = 127;
#[derive(Clone, Copy, Debug)]
struct Span {
    start: usize,
    width: usize,
}
fn spans() -> [Span; COUNT] {
    core::array::from_fn(|i| Span {
        start: i * BITS / COUNT,
        width: (i + 1) * BITS / COUNT - i * BITS / COUNT,
    })
}
#[inline(always)]
fn read(s: &[u64; 4], high: bool, span: Span) -> i16 {
    let i = usize::from(high) * 2;
    let magnitude = s[i] as u128 | (((s[i + 1] & 0x7fff_ffff_ffff_ffff) as u128) << 64);
    let d = if span.start == 0 {
        let v = (magnitude & ((1u128 << span.width) - 1)) as i64;
        v - ((v >> (span.width - 1)) << span.width)
    } else {
        let v = ((magnitude >> (span.start - 1)) & ((1u128 << (span.width + 1)) - 1)) as i64;
        ((v + 1) >> 1) - ((v >> span.width) << span.width)
    } as i16;
    if s[i + 1] >> 63 != 0 {
        -d
    } else {
        d
    }
}
fn feed<'a>(
    s: &mut Window<'a>,
    bases: &'a [G2Affine],
    short: &[[u64; 4]],
    high: bool,
    span: Span,
    size: usize,
) {
    for (p, scalar) in bases.iter().zip(short) {
        let d = read(scalar, high, span);
        if d == 0 || p.is_identity() {
            continue;
        }
        let b = d.unsigned_abs() as usize - 1;
        let encoded = b as u32 | if d < 0 { 1 << 31 } else { 0 };
        if s.busy(b) {
            s.queue.push((encoded, p));
            if s.queue.len() == size - 1 {
                s.flush_queue();
            }
            continue;
        }
        s.add(encoded, p);
        if s.batch.len() == size {
            s.execute();
            s.take_from_queue();
        }
    }
    s.execute();
    s.flush_queue();
}
fn window(bases: &[G2Affine], short: &[[u64; 4]], span: Span) -> Vec<G2Affine> {
    let size = batch_size(span.width).max(32);
    let mut state = Window::new(1 << (span.width - 1), size, BatchAdd::default());
    feed(&mut state, bases, short, true, span, size);
    debug_assert!(state.batch.is_empty() && state.queue.is_empty());
    for p in &mut state.affine {
        if !p.is_identity() {
            p.x = p.x.mul_by_fp(&BETA);
        }
    }
    for p in &mut state.projective {
        p.x = p.x.mul_by_fp(&BETA);
    }
    feed(&mut state, bases, short, false, span, size);
    state.finish()
}
pub(super) fn raw(bases: &[G2Affine], words: &mut [[u64; 4]], parts: usize) -> G2Projective {
    assert_eq!(bases.len(), words.len());
    assert!((1..=2).contains(&parts));
    words
        .par_iter_mut()
        .for_each(|s| *s = super::super::g2_phi::phi_split(*s));
    let chunk = bases.len().div_ceil(parts);
    let cover = spans();
    let sums: Vec<_> = cover
        .par_iter()
        .map(|&span| {
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|part| {
                    let start = (part * chunk).min(bases.len());
                    let end = (start + chunk).min(bases.len());
                    window(&bases[start..end], &words[start..end], span)
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for b in buckets {
                super::super::merge_buckets(&mut combined, &b);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = G2Projective::IDENTITY;
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
    use zopro_bn254::Fr;
    #[test]
    fn exact_signed_component_reconstruction_and_geometry() {
        let cover = spans();
        assert_eq!(cover.iter().map(|s| s.width).sum::<usize>(), 127);
        assert_eq!(cover.iter().filter(|s| s.width == 11).count(), 5);
        assert_eq!(cover.iter().filter(|s| s.width == 12).count(), 6);
        assert_eq!(
            cover.iter().map(|s| 1usize << (s.width - 1)).sum::<usize>(),
            17408
        );
        let mut values = vec![0u128, 1, (1u128 << 126) - 1];
        for bit in 0..126 {
            let p = 1u128 << bit;
            values.extend([p - 1, p, p + 1]);
        }
        let mut rng = Seeded::new(0x62616c7068693137);
        values.extend((0..4096).map(|_| {
            ((rng.next_u64() as u128) | ((rng.next_u64() as u128) << 64)) & ((1u128 << 126) - 1)
        }));
        let mut checks = 0usize;
        for magnitude in values {
            for negative in [false, true] {
                for high in [false, true] {
                    let mut words = [0u64; 4];
                    let i = usize::from(high) * 2;
                    words[i] = magnitude as u64;
                    words[i + 1] = (magnitude >> 64) as u64 | if negative { 1 << 63 } else { 0 };
                    let mut sum = 0i128;
                    for span in cover {
                        let bit = |n: usize| -> i128 { ((magnitude >> n) & 1) as i128 };
                        let v = (0..span.width)
                            .map(|b| bit(span.start + b) << b)
                            .sum::<i128>();
                        let mut expected = v + if span.start == 0 {
                            0
                        } else {
                            bit(span.start - 1)
                        } - (bit(span.start + span.width - 1) << span.width);
                        if negative {
                            expected = -expected;
                        }
                        let digit = read(&words, high, span);
                        assert_eq!(digit as i128, expected);
                        assert!(digit.unsigned_abs() as usize <= 1usize << (span.width - 1));
                        sum += (digit as i128) << span.start;
                        checks += 1;
                    }
                    assert_eq!(
                        sum,
                        if negative {
                            -(magnitude as i128)
                        } else {
                            magnitude as i128
                        }
                    );
                }
            }
        }
        assert_eq!(checks, 196988);
    }
    fn fixture(n: usize, adversarial: bool) -> (Vec<G2Affine>, Vec<[u64; 4]>, G2Projective) {
        let g = G2Projective::generator();
        let mut rng = Seeded::new(0x67656e7068693137);
        let coefficients: Vec<_> = (0..n)
            .map(|i| match i % 13 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                _ => Fr::from((i + 13579) as u64),
            })
            .collect();
        let points =
            G2Projective::batch_to_affine(&coefficients.iter().map(|&s| g * s).collect::<Vec<_>>());
        let values: Vec<_> = (0..n)
            .map(|i| {
                if adversarial {
                    match i % 9 {
                        0 => Fr::ZERO,
                        1..=3 => Fr::ONE,
                        4 => -Fr::ONE,
                        5 => Fr::from(2u64),
                        6 => -Fr::from(2u64),
                        _ => rng.fr(),
                    }
                } else {
                    rng.fr()
                }
            })
            .collect();
        let expected = g * values
            .iter()
            .zip(&coefficients)
            .fold(Fr::ZERO, |s, (&a, &b)| s + a * b);
        (
            points,
            values.iter().map(Fr::to_canonical).collect(),
            expected,
        )
    }
    #[test]
    fn balanced_phi_matches_independent_generator_sums() {
        let mut cases = 0;
        for workers in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            for parts in [1, 2] {
                for n in [0, 1, 17, 1024, 4097, 30089] {
                    for adversarial in [false, true] {
                        let (bases, mut words, expected) = fixture(n, adversarial);
                        let actual = pool.install(|| raw(&bases, &mut words, parts));
                        assert_eq!(
                            actual.to_affine(),
                            expected.to_affine(),
                            "workers={workers}parts={parts}n={n}adv={adversarial}"
                        );
                        cases += 1;
                    }
                }
            }
        }
        assert_eq!(cases, 48);
    }
    #[test]
    fn compiled_dispatch_matches_policy() {
        let (bases, mut words, expected) = fixture(1024, false);
        assert!(BatchAdd::<G2Config>::default().borrowed_g2_enabled());
        super::super::BALANCED_CALLS.with(|c| c.set(0));
        let actual = super::super::super::g2_phi::raw_exclusive(&bases, &mut words);
        assert_eq!(actual.to_affine(), expected.to_affine());
        let calls = super::super::BALANCED_CALLS.with(|c| c.get());
        assert_eq!(calls, usize::from(cfg!(zopro_balanced_phi)));
    }
}
