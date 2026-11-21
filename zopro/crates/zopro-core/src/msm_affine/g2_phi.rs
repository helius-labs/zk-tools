//! Cube-root bucket-Horner MSM for subgroup-valid G2 key queries.
//! Consume exclusive proof-local scalar storage into two signed components.
//! No persistent point tables; bucket/tree workspace is charged by workspace.rs.
use super::{affine_buckets, batch_size, digit, fill_window_impl, merge_buckets};
use rayon::prelude::*;
#[cfg(test)]
use zopro_bn254::Fr;
// Lattice rows (V1X,-V1Y_ABS),(V2X,V2Y) annihilate (1,lambda) mod r.
// Reciprocals are rounded at 320 bits. Truncating input below bits128/64
// contributes <2^-62 to coordinate error; including nearest rounding gives
// component magnitudes <2^126. Thus signed residuals are recovered modulo2^128.
const C1: [u64; 3] = [7978627105577135859, 15644699364383830999, 2];
const C2: [u64; 4] = [
    11953552847224317658,
    8825887400277225869,
    5534624963584316111,
    2,
];
const V1X: u128 = 9931322734385697763;
const V1Y_ABS: u128 = 147946756881789319000765030803803410728;
const V2X: u128 = 147946756881789319010696353538189108491;
const V2Y: u128 = 9931322734385697763;
#[inline(always)]
fn product<const N: usize, const M: usize, const K: usize>(a: [u64; N], b: [u64; M]) -> [u64; K] {
    assert_eq!(K, N + M);
    let mut out = [0u64; K];
    for i in 0..N {
        let mut carry = 0u128;
        for j in 0..M {
            let t = a[i] as u128 * b[j] as u128 + out[i + j] as u128 + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        out[i + M] = carry as u64;
    }
    out
}
#[inline(always)]
fn sign_magnitude(value: u128) -> u128 {
    let sign = ((value as i128) >> 127) as u128;
    let m = (value ^ sign).wrapping_sub(sign);
    assert!(m < 1u128 << 126);
    m | (sign & (1u128 << 127))
}
pub(super) fn phi_split(s: [u64; 4]) -> [u64; 4] {
    let a = product::<2, 3, 5>([s[2], s[3]], C1);
    let b = product::<3, 4, 7>([s[1], s[2], s[3]], C2);
    debug_assert_eq!(a[4], 0);
    debug_assert_eq!(b[6], 0);
    let q1 = a[3] as u128 + u128::from(a[2] >> 63);
    let q2 = (b[4] as u128 | ((b[5] as u128) << 64)) + u128::from(b[3] >> 63);
    let low = s[0] as u128 | ((s[1] as u128) << 64);
    let k1 = low
        .wrapping_sub(q1.wrapping_mul(V1X))
        .wrapping_sub(q2.wrapping_mul(V2X));
    let k2 = q1.wrapping_mul(V1Y_ABS).wrapping_sub(q2.wrapping_mul(V2Y));
    let (a, b) = (sign_magnitude(k1), sign_magnitude(k2));
    [a as u64, (a >> 64) as u64, b as u64, (b >> 64) as u64]
}
use zopro_bn254::{Fq, G2Affine, G2Config, G2Projective};
const BETA: Fq = Fq::from_decimal(
    "21888242871839275220042445260109153167277707414472061641714758635765020556616",
);
#[cfg(test)]
fn phi_point(mut p: G2Affine) -> G2Affine {
    if !p.is_identity() {
        p.x = p.x.mul_by_fp(&BETA);
    }
    p
}
#[inline]
fn hb_digit(s: &[u64; 4], high: bool, w: usize, bits: usize) -> i16 {
    let i = usize::from(high) * 2;
    {
        let d = digit(&[s[i], s[i + 1] & 0x7fffffffffffffff, 0, 0], w, bits);
        if s[i + 1] >> 63 != 0 {
            -d
        } else {
            d
        }
    }
}

fn hb_top_component(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    high: bool,
    w: usize,
    bits: usize,
    top: usize,
) -> Vec<G2Affine> {
    let n = 1usize << top;
    let bucket = |d: i16| d.unsigned_abs() as usize - 1 + if d < 0 { n } else { 0 };
    let mut counts = vec![0usize; 2 * n];
    for (base, s) in bases.iter().zip(short) {
        let d = hb_digit(s, high, w, bits);
        assert!(d.unsigned_abs() as usize <= n);
        if d != 0 && !base.is_identity() {
            counts[bucket(d)] += 1;
        }
    }
    let mut starts = Vec::with_capacity(2 * n + 1);
    starts.push(0);
    for count in counts {
        starts.push(starts.last().unwrap() + count);
    }
    let len = *starts.last().unwrap();
    if len == 0 {
        return vec![G2Affine::IDENTITY; n];
    }
    let mut positions = starts[..2 * n].to_vec();
    let mut order = vec![0u32; len];
    for (i, (base, s)) in bases.iter().zip(short).enumerate() {
        let d = hb_digit(s, high, w, bits);
        if d != 0 && !base.is_identity() {
            let b = bucket(d);
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    let mut reduced = crate::msm::reduce_indexed_buckets(bases, &order, &starts);
    let mut negative = reduced.split_off(n);
    for p in &mut negative {
        p.y = -p.y;
    }
    merge_buckets(&mut reduced, &negative);
    reduced
}
fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize) -> Vec<G2Affine> {
    if w + 1 == 127usize.div_ceil(bits) && 126 - w * bits <= 8 {
        let top = 126 - w * bits;
        // Each component's temporary index/tree workspace is released before the next.
        let mut high = hb_top_component(bases, short, true, w, bits, top);
        for p in &mut high {
            if !p.is_identity() {
                p.x = p.x.mul_by_fp(&BETA);
            }
        }
        let low = hb_top_component(bases, short, false, w, bits, top);
        merge_buckets(&mut high, &low);
        return high;
    }
    #[cfg(all(
        zopro_g2_horner_buckets,
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    if let Some(value) = super::g2_borrowed::try_window(bases, short, w, bits) {
        return value;
    }
    let size = batch_size(bits).max(32);
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    if super::borrowed_scalar::enabled() {
        let mut state = super::borrowed_scalar::Borrowed::new(bits, size);
        state.feed(bases, short.iter().map(|s| hb_digit(s, true, w, bits)));
        phi_buckets(&mut state.inner);
        state.feed(bases, short.iter().map(|s| hb_digit(s, false, w, bits)));
        return affine_buckets(state.inner);
    }
    let mut state = fill_window_impl::<G2Config, false, false>(
        bases,
        short.iter().map(|s| hb_digit(s, true, w, bits)),
        bits,
        size,
    );
    assert!(state.batch.is_empty() && state.queue.is_empty());
    phi_buckets(&mut state);
    for (base, d) in bases
        .iter()
        .zip(short.iter().map(|s| hb_digit(s, false, w, bits)))
    {
        if d == 0 || base.is_identity() {
            continue;
        }
        let bucket = d.unsigned_abs() as usize - 1;
        let point = if d < 0 { -*base } else { *base };
        if state.busy(bucket) {
            state.queue.push((bucket as u32, point));
            if state.queue.len() == size - 1 {
                state.flush_queue();
            }
            continue;
        }
        state.add(bucket, point);
        if state.batch.len() == size {
            state.execute();
            state.take_from_queue();
        }
    }
    state.execute();
    state.flush_queue();
    affine_buckets(state)
}
fn phi_buckets(state: &mut super::Window<G2Config>) {
    for p in &mut state.affine {
        if !p.is_identity() {
            p.x = p.x.mul_by_fp(&BETA);
        }
    }
    for p in &mut state.projective {
        p.x = p.x.mul_by_fp(&BETA);
    }
}
pub(super) fn hb_raw(
    bases: &[G2Affine],
    short: &mut [[u64; 4]],
    bits: usize,
    parts: usize,
) -> G2Projective {
    hb_raw_decomposition(bases, short, bits, parts, false)
}

pub(super) fn hb_raw_decomposition(
    bases: &[G2Affine],
    short: &mut [[u64; 4]],
    bits: usize,
    parts: usize,
    serial: bool,
) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    if serial {
        for word in short.iter_mut() {
            *word = phi_split(*word);
        }
    } else {
        #[cfg(zopro_phi_grain)]
        short
            .par_iter_mut()
            .with_min_len(128)
            .for_each(|s| *s = phi_split(*s));
        #[cfg(not(zopro_phi_grain))]
        short.par_iter_mut().for_each(|s| *s = phi_split(*s));
    }
    from_split(bases, short, bits, parts)
}

pub(super) fn from_split(
    bases: &[G2Affine],
    short: &[[u64; 4]],
    bits: usize,
    parts: usize,
) -> G2Projective {
    assert_eq!(bases.len(), short.len());
    let chunk = bases.len().div_ceil(parts);
    let windows = 127usize.div_ceil(bits);
    let sums: Vec<_> = (0..windows)
        .into_par_iter()
        .map(|w| {
            let mut buckets: Vec<_> = (0..parts)
                .into_par_iter()
                .map(|part| {
                    let start = (part * chunk).min(bases.len());
                    let end = (start + chunk).min(bases.len());
                    hb_window(&bases[start..end], &short[start..end], w, bits)
                })
                .collect();
            let mut combined = buckets.swap_remove(0);
            for b in buckets {
                merge_buckets(&mut combined, &b);
            }
            crate::msm::weighted_affine(&combined)
        })
        .collect();
    let mut result = G2Projective::IDENTITY;
    for sum in sums.iter().rev() {
        for _ in 0..bits {
            result = result.double();
        }
        result += sum;
    }
    result
}

#[cfg(zopro_g2_horner_buckets)]
pub(super) fn raw_exclusive(bases: &[G2Affine], words: &mut [[u64; 4]]) -> G2Projective {
    assert_eq!(bases.len(), words.len());
    if !(1024..(1 << 16)).contains(&bases.len()) {
        return super::g2_horner::raw(bases, words);
    }
    #[cfg(zopro_resource_schedule)]
    if rayon::current_num_threads() >= 32
        && std::env::var("ZOPRO_CPU_G2_RANGES").as_deref() == Ok("4")
    {
        let growth = super::workspace::four_range_growth(bases.len(), rayon::current_num_threads());
        if let Some(_reservation) = crate::preparation::Reservation::analysis(growth) {
            let serial = std::env::var("ZOPRO_CPU_G2_DECOMPOSITION").as_deref() == Ok("serial");
            return hb_raw_decomposition(bases, words, 11, 4, serial);
        }
    }
    hb_raw(bases, words, 12, 2)
}

// This root acts on G1 by the same scalar as BETA does on G2, so both
// groups can use the existing lattice decomposition.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const G1_BETA: Fq =
    Fq::from_decimal("2203960485148121921418603742825762020974279258880205651966");

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(super) fn quotient_g1_msm(
    bases: &[zopro_bn254::G1Affine],
    short: &[[u64; 4]],
) -> zopro_bn254::G1Projective {
    use zopro_bn254::G1Projective;

    assert_eq!(bases.len(), short.len());
    const BITS: usize = 13;
    let mid = bases.len().div_ceil(2);
    let size = batch_size(BITS);
    let sums: Vec<_> = (0..127usize.div_ceil(BITS))
        .into_par_iter()
        .map(|window| {
            let buckets = |range: std::ops::Range<usize>| {
                let points = &bases[range.clone()];
                let words = &short[range];
                let mut state = super::borrowed_scalar::Borrowed::new(BITS, size);
                state.feed(points, words.iter().map(|s| hb_digit(s, true, window, BITS)));
                for point in &mut state.inner.affine {
                    point.x *= G1_BETA;
                }
                for point in &mut state.inner.projective {
                    point.x *= G1_BETA;
                }
                state.feed(points, words.iter().map(|s| hb_digit(s, false, window, BITS)));
                affine_buckets(state.inner)
            };
            let (mut left, right) = rayon::join(|| buckets(0..mid), || buckets(mid..bases.len()));
            merge_buckets(&mut left, &right);
            crate::msm::weighted_affine(&left)
        })
        .collect();
    let mut result = G1Projective::IDENTITY;
    for sum in sums.into_iter().rev() {
        for _ in 0..BITS {
            result = result.double();
        }
        result += sum;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn g1_components_match_the_scalar_multiplication_oracle() {
        use zopro_bn254::{G1Affine, G1Projective};

        let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let mut transformed = G1Affine::GENERATOR;
        transformed.x *= G1_BETA;
        assert_eq!(transformed, (G1Affine::GENERATOR * lambda).to_affine());

        let mut point = G1Projective::IDENTITY;
        let mut scalar = Fr::from(918371u64);
        let mut bases = Vec::new();
        let mut scalars = Vec::new();
        let mut expected = G1Projective::IDENTITY;
        for i in 0..512 {
            point += G1Affine::GENERATOR;
            scalar = scalar * Fr::from(6364136223846793005u64) + Fr::ONE;
            let base = if i % 7 == 0 { G1Affine::IDENTITY } else { point.to_affine() };
            let s = match i % 11 {
                0 => Fr::ZERO,
                1 => Fr::ONE,
                2 => -Fr::ONE,
                3 => -scalar,
                _ => scalar,
            };
            bases.push(base);
            scalars.push(phi_split(s.to_canonical()));
            expected += base * s;
        }
        assert_eq!(quotient_g1_msm(&bases, &scalars), expected);
    }

    #[test]
    fn decomposition_and_endomorphism_match_independent_field_oracles() {
        let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let mut values = vec![
            Fr::ZERO,
            Fr::ONE,
            -Fr::ONE,
            lambda,
            lambda - Fr::ONE,
            lambda + Fr::ONE,
        ];
        for bit in 0..254 {
            let p = Fr::from(2u64).pow(&[bit]);
            values.extend([p - Fr::ONE, p, p + Fr::ONE]);
        }
        let mut x = Fr::from(982371u64);
        for _ in 0..1024 {
            x = x * Fr::from(6364136223846793005u64) + Fr::ONE;
            values.push(x);
        }
        for s in values {
            let p = phi_split(s.to_canonical());
            let component = |i: usize| {
                let hi = p[i + 1] & 0x7fffffffffffffff;
                assert!(hi < 1u64 << 62);
                let m = Fr::from_canonical([p[i], hi, 0, 0]).unwrap();
                if p[i + 1] >> 63 != 0 {
                    -m
                } else {
                    m
                }
            };
            assert_eq!(component(0) + lambda * component(2), s);
        }
        let mut point = G2Projective::IDENTITY;
        for i in 0..96 {
            let p = if i % 3 == 0 { -point } else { point }.to_affine();
            assert_eq!(phi_point(p), (p * lambda).to_affine());
            point += G2Affine::GENERATOR;
        }
    }
}
