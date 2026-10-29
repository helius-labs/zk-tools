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
        #[cfg(zopro_gls4_census)]
        if census::enabled(){return census::run_feed(self,bases,short,component,w,size);}
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
fn window(bases: &[G2Affine], short: &[[u64; 5]], w: usize, x_factor: &Fq2) -> Vec<G2Affine> {
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
    #[cfg(zopro_gls4_census)]
    let query_id=census::query();
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
                    #[cfg(zopro_gls4_census)]
                    let _context=census::context(query_id,w,part);
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
        eprintln!("GLS4_HORNER entered bits=11 parts=4 points={}", bases.len());
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

#[cfg(zopro_gls4_census)]
mod census;
