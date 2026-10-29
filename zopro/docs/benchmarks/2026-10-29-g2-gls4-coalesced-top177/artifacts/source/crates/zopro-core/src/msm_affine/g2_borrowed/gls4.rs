//! One borrowed bucket state for four GLV/GLS components.
use super::*;
use rayon::prelude::*;
use zopro_bn254::{Fq2, Fr};
mod lattice;
use lattice::{component_digit, split};
const BITS: usize = 11;
const PARTS: usize = 4;
const WINDOWS: usize = 6;
const PSI_X: Fq2 = Fq2::new(
    Fq::from_decimal(
        "21575463638280843010398324269430826099269044274347216827212613867836435027261",
    ),
    Fq::from_decimal(
        "10307601595873709700152284273816112264069230130616436755625194854815875713954",
    ),
);
const PSI_Y: Fq2 = Fq2::new(
    Fq::from_decimal(
        "2821565182194536844548159561693502659359617185244120367078079554186484126554",
    ),
    Fq::from_decimal(
        "3505843767911556378687030309984248845540243509899259641013678093033130930403",
    ),
);

impl<'a> Window<'a> {
    fn feed_gls4(
        &mut self,
        bases: &'a [G2Affine],
        short: &[[u64; 5]],
        component: usize,
        w: usize,
        size: usize,
    ) {
        for (p, s) in bases.iter().zip(short) {
            let d = component_digit(s, component, w, BITS);
            if d == 0 || p.is_identity() {
                continue;
            }
            let b = d.unsigned_abs() as usize - 1;
            let encoded = b as u32 | if d < 0 { 1 << 31 } else { 0 };
            if self.busy(b) {
                self.queue.push((encoded, p));
                if self.queue.len() == size - 1 {
                    self.flush_queue();
                }
                continue;
            }
            self.add(encoded, p);
            if self.batch.len() == size {
                self.execute();
                self.take_from_queue();
            }
        }
        self.execute();
        self.flush_queue();
    }
    fn phi(&mut self) {
        debug_assert!(self.batch.is_empty() && self.queue.is_empty());
        for p in &mut self.affine {
            if !p.is_identity() {
                p.x = p.x.mul_by_fp(&BETA);
            }
        }
        for p in &mut self.projective {
            p.x = p.x.mul_by_fp(&BETA);
        }
    }
    fn compound(&mut self, x_factor: &Fq2) {
        debug_assert!(self.batch.is_empty() && self.queue.is_empty());
        for p in &mut self.affine {
            if !p.is_identity() {
                p.x = p.x.conjugate() * *x_factor;
                p.y = p.y.conjugate() * PSI_Y;
            }
        }
        for p in &mut self.projective {
            p.x = p.x.conjugate() * *x_factor;
            p.y = p.y.conjugate() * PSI_Y;
            p.z = p.z.conjugate();
        }
    }
}
// This is the original signed indexed top algorithm adapted only to GLS4 digits.
fn top_component(bases: &[G2Affine], short: &[[u64; 5]], component: usize) -> Vec<G2Affine> {
    const N: usize = 1 << (BITS - 1);
    let bucket = |d: i16| d.unsigned_abs() as usize - 1 + if d < 0 { N } else { 0 };
    let mut counts = vec![0usize; 2 * N];
    for (base, s) in bases.iter().zip(short) {
        let d = component_digit(s, component, WINDOWS - 1, BITS);
        assert!(d.unsigned_abs() as usize <= N);
        if d != 0 && !base.is_identity() {
            counts[bucket(d)] += 1;
        }
    }
    let mut starts = Vec::with_capacity(2 * N + 1);
    starts.push(0);
    for &count in &counts {
        starts.push(starts.last().unwrap() + count);
    }
    let len = *starts.last().unwrap();
    if len == 0 {
        return vec![G2Affine::IDENTITY; N];
    }
    let mut positions = starts[..2 * N].to_vec();
    let mut order = vec![0u32; len];
    for (i, (base, s)) in bases.iter().zip(short).enumerate() {
        let d = component_digit(s, component, WINDOWS - 1, BITS);
        if d != 0 && !base.is_identity() {
            let b = bucket(d);
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    let mut reduced = crate::msm::group_affine(bases, &order, &starts, &counts);
    let mut negative = reduced.split_off(N);
    for p in &mut negative {
        p.y = -p.y;
    }
    super::super::merge_buckets(&mut reduced, &negative);
    reduced
}
fn top_window(bases: &[G2Affine], short: &[[u64; 5]], x_factor: &Fq2) -> Vec<G2Affine> {
    let mut combined = top_component(bases, short, 3);
    for component in (0..3).rev() {
        for p in &mut combined {
            if !p.is_identity() {
                if component == 1 {
                    p.x = p.x.conjugate() * *x_factor;
                    p.y = p.y.conjugate() * PSI_Y;
                } else {
                    p.x = p.x.mul_by_fp(&BETA);
                }
            }
        }
        let next = top_component(bases, short, component);
        super::super::merge_buckets(&mut combined, &next);
    }
    combined
}

fn window(bases: &[G2Affine], short: &[[u64; 5]], w: usize, x_factor: &Fq2) -> Vec<G2Affine> {
    if w + 1 == WINDOWS {
        return top_window(bases, short, x_factor);
    }
    let size = batch_size(BITS).max(32);
    let packed = BatchAdd::default();
    let mut state = Window::new(1 << (BITS - 1), size, packed);
    state.feed_gls4(bases, short, 3, w, size);
    state.phi();
    state.feed_gls4(bases, short, 2, w, size);
    state.compound(x_factor);
    state.feed_gls4(bases, short, 1, w, size);
    state.phi();
    state.feed_gls4(bases, short, 0, w, size);
    state.finish()
}
fn run_prepared(bases: &[G2Affine], short: &[[u64; 5]]) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    if bases.is_empty() {
        return G2Projective::IDENTITY;
    }
    let chunk = bases.len().div_ceil(PARTS);
    // Protocol-only constants, calculated once per invocation; included in timing.
    let x_factor = PSI_X.mul_by_fp(&BETA.square());
    let sums: Vec<_> = (0..WINDOWS)
        .into_par_iter()
        .map(|w| {
            let mut pieces: Vec<_> = (0..PARTS)
                .into_par_iter()
                .map(|part| {
                    let start = (part * chunk).min(bases.len());
                    let end = (start + chunk).min(bases.len());
                    window(&bases[start..end], &short[start..end], w, &x_factor)
                })
                .collect();
            let mut combined = pieces.swap_remove(0);
            for piece in pieces {
                super::super::merge_buckets(&mut combined, &piece);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = G2Projective::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..BITS {
            result = result.double();
        }
        result += sum;
    }
    result
}
pub(crate) fn try_raw(bases: &[G2Affine], words: &[[u64; 4]]) -> Option<G2Projective> {
    assert_eq!(bases.len(), words.len());
    if !(1024..65536).contains(&bases.len()) {
        return None;
    }
    let _reservation = crate::preparation::Reservation::analysis(
        super::super::workspace::gls4_horner_peak(bases.len(), rayon::current_num_threads()),
    )?;
    let mut short = Vec::new();
    short.try_reserve_exact(words.len()).ok()?;
    words
        .par_iter()
        .map(|s| split(*s))
        .collect_into_vec(&mut short);
    if std::env::var_os("ZOPRO_GLS4_HORNER_TRACE").is_some() {
        eprintln!(
            "GLS4_COALESCED entered bits=11 parts=4 points={}",
            bases.len()
        );
    }
    Some(run_prepared(bases, &short))
}

#[cfg(test)]
mod tests {
    use super::*;
    const L: Fr = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
    const M: Fr = Fr::from_decimal("147946756881789318990833708069417712966");
    fn random(seed: &mut u64) -> Fr {
        let mut bytes = [0; 64];
        for chunk in bytes.chunks_mut(8) {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            chunk.copy_from_slice(&seed.to_le_bytes());
        }
        Fr::from_random_bytes(&bytes)
    }
    fn decode(s: &[u64; 5], i: usize) -> Fr {
        let v = Fr::from(s[i]) + Fr::from((s[4] >> i) & 1) * Fr::from(2).pow(&[64]);
        if s[4] >> (i + 4) & 1 != 0 {
            -v
        } else {
            v
        }
    }
    #[test]
    fn decomposition_and_digits_reconstruct_fullwidth() {
        let mut values = vec![Fr::ZERO, Fr::ONE, -Fr::ONE, L, M, L - Fr::ONE, M + Fr::ONE];
        for bit in 0..254 {
            let v = Fr::from(2).pow(&[bit]);
            values.extend([v - Fr::ONE, v, v + Fr::ONE]);
        }
        let mut seed = 0x474c5334484f524eu64;
        for _ in 0..8192 {
            values.push(random(&mut seed));
        }
        for scalar in values {
            let s = split(scalar.to_canonical());
            assert_eq!(s[4] >> 8, 0);
            let c: [Fr; 4] = core::array::from_fn(|i| decode(&s, i));
            assert_eq!(c[0] + L * c[1] + M * c[2] + L * M * c[3], scalar);
            for i in 0..4 {
                let mut integer = 0i128;
                for w in (0..WINDOWS).rev() {
                    integer = integer * (1 << BITS) + i128::from(component_digit(&s, i, w, BITS));
                }
                let magnitude = s[i] as u128 | (((s[4] >> i) & 1) as u128) << 64;
                let expected = if s[4] >> (i + 4) & 1 != 0 {
                    -(magnitude as i128)
                } else {
                    magnitude as i128
                };
                assert_eq!(integer, expected);
            }
        }
    }
    #[test]
    fn affine_and_projective_horner_maps_match_scalars() {
        let x_factor = PSI_X.mul_by_fp(&BETA.square());
        for i in 0..33u64 {
            let p = G2Projective::generator() * Fr::from(i);
            let q = G2Projective::generator() * Fr::from(i + 57);
            let mut state = Window::new(1, 32, BatchAdd::default());
            state.affine[0] = p.to_affine();
            state.projective.push(q);
            state.projective_buckets.push(0);
            state.projective_indices[0] = 0;
            state.phi();
            assert_eq!(state.affine[0], (p * L).to_affine());
            assert_eq!(state.projective[0], q * L);
            state.compound(&x_factor);
            assert_eq!(state.affine[0], (p * M).to_affine());
            assert_eq!(state.projective[0], q * M);
            state.phi();
            assert_eq!(state.affine[0], (p * L * M).to_affine());
            assert_eq!(state.projective[0], q * L * M);
            assert_eq!(state.finish()[0], ((p + q) * L * M).to_affine());
        }
    }
    #[test]
    fn full_msm_matches_independent_generator_sums() {
        let mut seed = 0x474c53344d534d31u64;
        for n in [0usize, 1, 7, 33, 1024, 2049] {
            let g = G2Projective::generator();
            let mut cursor = G2Projective::IDENTITY;
            let mut points = Vec::new();
            let mut weights = Vec::new();
            for i in 0..n {
                cursor += g;
                let mut p = cursor;
                let mut w = Fr::from((i + 1) as u64);
                if i % 17 == 0 {
                    p = G2Projective::IDENTITY;
                    w = Fr::ZERO;
                } else if i % 5 == 0 {
                    p = -p;
                    w = -w;
                }
                points.push(p);
                weights.push(w);
            }
            let points = G2Projective::batch_to_affine(&points);
            for mode in 0..4 {
                let scalars: Vec<_> = (0..n)
                    .map(|i| match mode {
                        0 => Fr::ZERO,
                        1 => {
                            if i % 2 == 0 {
                                Fr::ONE
                            } else {
                                -Fr::ONE
                            }
                        }
                        2 => [L, M, L * M, -L, -M][i % 5],
                        _ => random(&mut seed),
                    })
                    .collect();
                let short: Vec<_> = scalars.iter().map(|s| split(s.to_canonical())).collect();
                let expected = scalars
                    .iter()
                    .zip(&weights)
                    .fold(Fr::ZERO, |sum, (&s, &w)| sum + s * w);
                assert_eq!(
                    run_prepared(&points, &short).to_affine(),
                    (g * expected).to_affine(),
                    "n={n} mode={mode}"
                );
            }
        }
    }
    #[test]
    fn indexed_top_signed_edges_match_generator_sum() {
        let n = 2053;
        let g = G2Projective::generator();
        let mut cursor = G2Projective::IDENTITY;
        let mut points = Vec::new();
        let mut weights = Vec::new();
        let mut digits = Vec::new();
        let mut short = Vec::new();
        for i in 0..n {
            cursor += g;
            let mut p = cursor;
            let mut weight = Fr::from((i + 1) as u64);
            if i % 17 == 0 {
                p = G2Projective::IDENTITY;
                weight = Fr::ZERO;
            } else if i % 5 == 0 {
                p = -p;
                weight = -weight;
            }
            let mut s = [0u64; 5];
            let mut ds = [0i16; 4];
            for c in 0..4 {
                let d = (i * (2 * c + 1) + c * 127) % 1025;
                // The largest magnitude rounds up to digit1024 via Booth carry.
                let magnitude = if d == 1024 {
                    (1u128 << 65) - 1
                } else {
                    (d as u128) << 55
                };
                s[c] = magnitude as u64;
                s[4] |= ((magnitude >> 64) as u64) << c;
                let negative = (i + c) % 3 == 0;
                if negative {
                    s[4] |= 1 << (c + 4);
                }
                ds[c] = if negative { -(d as i16) } else { d as i16 };
            }
            points.push(p);
            weights.push(weight);
            short.push(s);
            digits.push(ds);
        }
        let points = G2Projective::batch_to_affine(&points);
        for c in 0..4 {
            let result = top_component(&points, &short, c);
            assert_eq!(result.len(), 1024);
            let expected = weights.iter().zip(&digits).fold(Fr::ZERO, |sum, (&w, d)| {
                let f = Fr::from(d[c].unsigned_abs() as u64);
                sum + w * if d[c] < 0 { -f } else { f }
            });
            assert_eq!(
                crate::msm::weighted_affine(&result).to_affine(),
                (g * expected).to_affine(),
                "component={c}"
            );
        }
        assert!(top_component(&[], &[], 0).iter().all(G2Affine::is_identity));
    }
    #[test]
    fn hybrid_msm_custom_rayon_pools_match_generator_sum() {
        let n = 2049;
        let g = G2Projective::generator();
        let mut seed = 0x485942524944544fu64;
        let mut cursor = G2Projective::IDENTITY;
        let mut points = Vec::new();
        let mut short = Vec::new();
        let mut expected = Fr::ZERO;
        for i in 0..n {
            cursor += g;
            let p = if i % 13 == 0 {
                G2Projective::IDENTITY
            } else if i % 3 == 0 {
                -cursor
            } else {
                cursor
            };
            let s = random(&mut seed);
            let w = if i % 13 == 0 {
                Fr::ZERO
            } else if i % 3 == 0 {
                -Fr::from((i + 1) as u64)
            } else {
                Fr::from((i + 1) as u64)
            };
            points.push(p);
            short.push(split(s.to_canonical()));
            expected += s * w;
        }
        let points = G2Projective::batch_to_affine(&points);
        for threads in [1, 4, 16, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            let actual = pool.install(|| run_prepared(&points, &short));
            assert_eq!(
                actual.to_affine(),
                (g * expected).to_affine(),
                "threads={threads}"
            );
        }
    }

    #[test]
    fn coalesced_top_skewed_complete_curve_groups() {
        let g = G2Projective::generator();
        for threads in [1, 4, 32] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for n in [0usize, 1, 7, 1025, 4097] {
                    let mut points = Vec::new();
                    let mut short = Vec::new();
                    let mut expected = Fr::ZERO;
                    for i in 0..n {
                        let (p, weight) = match i % 5 {
                            0 => (G2Affine::IDENTITY, Fr::ZERO),
                            1 | 2 => (G2Affine::GENERATOR, Fr::ONE),
                            _ => (-G2Affine::GENERATOR, -Fr::ONE),
                        };
                        let d = if i % 2 == 0 { 1024u64 } else { 1 };
                        let magnitude = if d == 1024 {
                            (1u128 << 65) - 1
                        } else {
                            1u128 << 55
                        };
                        let negative = i % 3 == 0;
                        let mut s = [0u64; 5];
                        s[0] = magnitude as u64;
                        s[4] = (magnitude >> 64) as u64 | if negative { 1 << 4 } else { 0 };
                        points.push(p);
                        short.push(s);
                        expected += weight * if negative { -Fr::from(d) } else { Fr::from(d) };
                    }
                    let buckets = top_component(&points, &short, 0);
                    assert_eq!(buckets.len(), 1024);
                    assert!(buckets[1..1023].iter().all(G2Affine::is_identity));
                    assert_eq!(
                        crate::msm::weighted_affine(&buckets).to_affine(),
                        (g * expected).to_affine(),
                        "threads={threads} n={n}"
                    );
                }
            });
        }
    }

    #[test]
    fn checked_admission_and_fallback_preserve_words() {
        use super::super::super::workspace::gls4_horner_peak;
        assert!(gls4_horner_peak(usize::MAX, 32).is_none());
        assert!(gls4_horner_peak(1024, usize::MAX).is_none());
        for threads in [1, 4, 16, 32, 96] {
            assert!(
                gls4_horner_peak(65535, threads).unwrap()
                    > gls4_horner_peak(1024, threads).unwrap()
            );
        }
        let points = vec![G2Affine::GENERATOR; 1024];
        let words = vec![Fr::ONE.to_canonical(); 1024];
        let before = words.clone();
        let saved = std::env::var_os("ZOPRO_CPU_PREPARE_MAX_BYTES");
        std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", "0");
        let denied = try_raw(&points, &words);
        if let Some(value) = saved {
            std::env::set_var("ZOPRO_CPU_PREPARE_MAX_BYTES", value);
        } else {
            std::env::remove_var("ZOPRO_CPU_PREPARE_MAX_BYTES");
        }
        assert!(denied.is_none());
        assert_eq!(words, before);
        assert!(try_raw(&[], &[]).is_none());
    }
}
