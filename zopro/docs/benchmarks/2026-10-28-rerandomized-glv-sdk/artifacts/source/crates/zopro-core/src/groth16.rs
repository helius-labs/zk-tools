//! gnark's Groth16 proof assembly over the kernel's parts, with fresh
//! blinding: Ar = A + alpha + r delta, Bs = B2 + beta2 + s delta2,
//! Krs = K + Z + s Ar + r (B + beta).
use zopro_bn254::{
    Affine, CurveConfig, Fq, Fr, G1Affine, G1Config, G1Projective, G2Affine, G2Config,
    G2Projective, Projective,
};

use crate::{decode_g1, decode_g2};

/// The proving key's fixed points: the verifying key's alpha, beta and
/// delta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FixedPoints {
    pub alpha: G1Affine,
    pub beta: G1Affine,
    pub delta: G1Affine,
    pub beta2: G2Affine,
    pub delta2: G2Affine,
}

impl FixedPoints {
    /// From the kernel's encoding: alpha, beta, delta (64 bytes each),
    /// beta2, delta2 (128 each), 448 bytes.
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 448 {
            return None;
        }
        Some(Self {
            alpha: decode_g1(&bytes[..64])[0],
            beta: decode_g1(&bytes[64..128])[0],
            delta: decode_g1(&bytes[128..192])[0],
            beta2: decode_g2(&bytes[192..320])[0],
            delta2: decode_g2(&bytes[320..448])[0],
        })
    }
}

/// Fixed proving-key points with optional native multiplication tables.
/// Only key constants are retained; assembly draws fresh blinding scalars.
#[derive(Clone)]
pub struct PreparedFixedPoints(std::sync::Arc<Prepared>);

struct Prepared {
    fixed: FixedPoints,
    windows: Option<(FixedBase<G1Config>, FixedBase<G2Config>)>,
}

impl PreparedFixedPoints {
    pub fn new(fixed: FixedPoints) -> Self {
        let windows = crate::simd::cpu_option("ZOPRO_CPU_PREPARE_BLINDING")
            .then(|| (FixedBase::new(fixed.delta), FixedBase::new(fixed.delta2)));
        Self(std::sync::Arc::new(Prepared { fixed, windows }))
    }

    pub fn assemble(&self, parts: &[u8]) -> Vec<u8> {
        assemble_blinded(
            parts,
            &self.0.fixed,
            crate::random_fr(),
            crate::random_fr(),
            self.0.windows.as_ref(),
        )
    }
}

pub(crate) struct FixedBase<C: CurveConfig>(Vec<Affine<C>>);
impl<C: CurveConfig> FixedBase<C> {
    const POINTS: usize = 64 * 15;
    // Construction holds projective points, inversion prefixes, and output.
    pub(crate) fn preparation_bytes() -> Option<usize> {
        Self::POINTS.checked_mul(std::mem::size_of::<Projective<C>>()
            + std::mem::size_of::<C::Base>() + std::mem::size_of::<Affine<C>>())?
            .checked_add(3 * 64)
    }

    #[cfg(zopro_live_preparation_charge)]
    pub(crate) fn allocation_bytes(&self) -> Option<usize> {
        crate::preparation::allocation_bytes(&self.0)
    }

    pub(crate) fn new(base: Affine<C>) -> Self {
        let mut points = Vec::with_capacity(Self::POINTS);
        let mut shifted = base.to_projective();
        for _ in 0..64 {
            let mut multiple = shifted;
            for _ in 1..16 {
                points.push(multiple);
                multiple += shifted;
            }
            for _ in 0..4 {
                shifted = shifted.double();
            }
        }
        Self(Projective::batch_to_affine(&points))
    }

    pub(crate) fn mul(&self, scalar: Fr) -> Projective<C> {
        let words = scalar.to_canonical();
        let mut result = Projective::IDENTITY;
        for window in 0..64 {
            let digit = ((words[window / 16] >> (4 * (window % 16))) & 15) as usize;
            if digit != 0 {
                result += self.0[window * 15 + digit - 1];
            }
        }
        result
    }
}

// Fresh odd-multiple tables share one doubling chain for ar*s + bs1*r.
// They live only for this proof; no witness-dependent state is retained.
fn blinding_digits<const W: usize>(scalar: zopro_bn254::Fr) -> ([i8; 256], usize) {
    let mut words = scalar.to_canonical();
    let mut digits = [0; 256];
    let mut used = 0;
    while words != [0; 4] {
        assert!(used < 256);
        if words[0] & 1 != 0 {
            let mut digit = (words[0] & ((1 << W) - 1)) as i16;
            if digit > (1 << (W - 1)) {
                digit -= 1 << W;
            }
            digits[used] = digit as i8;
            let mut carry = digit.unsigned_abs() as u64;
            for word in &mut words {
                let (v, next) = if digit > 0 {
                    word.overflowing_sub(carry)
                } else {
                    word.overflowing_add(carry)
                };
                *word = v;
                carry = u64::from(next);
            }
            assert_eq!(carry, 0);
        }
        for j in 0..3 {
            words[j] = (words[j] >> 1) | (words[j + 1] << 63);
        }
        words[3] >>= 1;
        used += 1;
    }
    (digits, used)
}
fn joint_blinding<const W: usize>(
    a: G1Projective,
    b: G1Projective,
    s: zopro_bn254::Fr,
    r: zopro_bn254::Fr,
) -> G1Projective {
    let (sd, sn) = blinding_digits::<W>(s);
    let (rd, rn) = blinding_digits::<W>(r);
    let count = 1usize << (W - 2);
    let mut points = Vec::with_capacity(2 * count);
    for base in [a, b] {
        let step = base.double();
        let mut point = base;
        for _ in 0..count {
            points.push(point);
            point += step;
        }
    }
    let table = G1Projective::batch_to_affine(&points);
    let mut out = G1Projective::IDENTITY;
    for i in (0..sn.max(rn)).rev() {
        out = out.double();
        for (d, offset) in [(sd[i], 0), (rd[i], count)] {
            if d != 0 {
                let p = table[offset + (d.unsigned_abs() as usize) / 2];
                out += if d < 0 { -p } else { p };
            }
        }
    }
    out
}

fn be(v: Fq, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_be_bytes());
}

/// The proof from `Key::solve_parts`' or `Key::parts`' 384 bytes (A, B, K,
/// Z, B2 in the kernel's encoding): Ar, Krs, Bs in gnark's raw big-endian
/// encoding, 256 bytes.
pub fn assemble(parts: &[u8], fixed: &FixedPoints) -> Vec<u8> {
    assemble_blinded(parts, fixed, crate::random_fr(), crate::random_fr(), None)
}

fn assemble_blinded(
    parts: &[u8],
    fixed: &FixedPoints,
    r: zopro_bn254::Fr,
    s: zopro_bn254::Fr,
    windows: Option<&(FixedBase<G1Config>, FixedBase<G2Config>)>,
) -> Vec<u8> {
    assert_eq!(parts.len(), 384, "the kernel's five parts");
    let (a, b, k, z) = (
        decode_g1(&parts[..64])[0],
        decode_g1(&parts[64..128])[0],
        decode_g1(&parts[128..192])[0],
        decode_g1(&parts[192..256])[0],
    );
    let b2 = decode_g2(&parts[256..384])[0];
    let delta: G1Projective = fixed.delta.to_projective();
    let (delta_r, delta2_s) = windows.map_or_else(
        || (delta * r, fixed.delta2 * s),
        |(g1, g2)| (g1.mul(r), g2.mul(s)),
    );
    let ar = a.to_projective() + fixed.alpha.to_projective() + delta_r;
    let bs1 = b.to_projective() + fixed.beta.to_projective();
    let bs: G2Projective = b2.to_projective() + fixed.beta2.to_projective() + delta2_s;
    // The s*delta term in the blinded B1 cancels -r*s*delta.
    let variable = if crate::simd::cpu_option("ZOPRO_CPU_JOINT_BLINDING") {
        joint_blinding::<5>(ar, bs1, s, r)
    } else {
        ar * s + bs1 * r
    };
    let krs = k.to_projective() + z.to_projective() + variable;
    let (ar, krs, bs) = (ar.to_affine(), krs.to_affine(), bs.to_affine());
    let mut out = Vec::with_capacity(256);
    for v in [ar.x, ar.y, krs.x, krs.y] {
        be(v, &mut out);
    }
    for v in [bs.x.c1, bs.x.c0, bs.y.c1, bs.y.c0] {
        be(v, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::Fr;

    #[test]
    fn joint_blinding_matches_independent_scalar_sum() {
        let mut rng = crate::testing::Seeded::new(0x473829);
        let values = [
            Fr::ZERO,
            Fr::ONE,
            -Fr::ONE,
            Fr::from(15u64),
            Fr::from(16u64),
            Fr::from(17u64),
            rng.fr(),
        ];
        let generator = G1Projective::generator();
        let points = values.map(|value| generator * value);
        for (i, &a) in points.iter().enumerate() {
            for (j, &b) in points.iter().enumerate() {
                for s in values {
                    for r in values {
                        let expected = generator * (values[i] * s + values[j] * r);
                        assert_eq!(a * s + b * r, expected);
                        assert_eq!(joint_blinding::<5>(a, b, s, r), expected);
                    }
                }
            }
        }
    }

    #[test]
    fn blinding_matches_groth16_formula() {
        let g1 = G1Affine::GENERATOR;
        let g2 = G2Affine::GENERATOR;
        let (a, b, k, z) = (
            g1 * Fr::from(2u64),
            g1 * Fr::from(3u64),
            g1 * Fr::from(5u64),
            g1 * Fr::from(7u64),
        );
        let b2 = g2 * Fr::from(11u64);
        let fixed = FixedPoints {
            alpha: (g1 * Fr::from(13u64)).to_affine(),
            beta: (g1 * Fr::from(17u64)).to_affine(),
            delta: (g1 * Fr::from(19u64)).to_affine(),
            beta2: (g2 * Fr::from(23u64)).to_affine(),
            delta2: (g2 * Fr::from(29u64)).to_affine(),
        };
        let mut parts = Vec::new();
        for point in [a, b, k, z] {
            crate::output_g1(&mut parts, point);
        }
        crate::output_g2(&mut parts, b2);
        let windows = (FixedBase::new(fixed.delta), FixedBase::new(fixed.delta2));
        let values = [
            Fr::ZERO,
            Fr::ONE,
            -Fr::ONE,
            Fr::from(2u64),
            crate::random_fr(),
        ];
        for r in values {
            for s in values {
                let delta = fixed.delta.to_projective();
                let ar = a + fixed.alpha + delta * r;
                let bs1 = b + fixed.beta + delta * s;
                let bs = b2 + fixed.beta2 + fixed.delta2 * s;
                let krs = k + z - delta * (r * s) + ar * s + bs1 * r;
                let expected = crate::verify::Proof {
                    ar: ar.to_affine(),
                    krs: krs.to_affine(),
                    bs: bs.to_affine(),
                };
                assert_eq!(
                    assemble_blinded(&parts, &fixed, r, s, None),
                    expected.to_raw()
                );
                assert_eq!(
                    assemble_blinded(&parts, &fixed, r, s, Some(&windows)),
                    expected.to_raw()
                );
            }
        }
    }
}

// Exact uniform sampling for the conditional proof distribution described
// in groth16_rerandomize.rs. The ordinary assembly API retains its sampler.
fn uniform_fr() -> Fr {
    loop {
        let mut bytes = [0u8; 32];
        getrandom::getrandom(&mut bytes).expect("the system's randomness");
        bytes[31] &= 0x3f;
        let words = std::array::from_fn(|i| {
            u64::from_le_bytes(bytes[8 * i..8 * i + 8].try_into().unwrap())
        });
        if let Some(value) = Fr::from_canonical(words) {
            return value;
        }
    }
}
impl PreparedFixedPoints {
    pub(crate) fn rerandomization_key_valid(&self) -> bool {
        !self.0.fixed.delta.is_identity() && !self.0.fixed.delta2.is_identity()
    }
    pub(crate) fn assemble_rerandomized(&self, parts: &[u8]) -> Option<Vec<u8>> {
        assert_eq!(parts.len(), 384);
        let a0 = decode_g1(&parts[..64])[0].to_projective() + self.0.fixed.alpha;
        if a0.is_identity() {
            return None;
        }
        let u = loop {
            let u = uniform_fr();
            if !u.is_zero() {
                break u;
            }
        };
        let v = uniform_fr();
        self.rerandomized_with(parts, u, v)
    }
    fn rerandomized_with(&self, parts: &[u8], u: Fr, v: Fr) -> Option<Vec<u8>> {
        assert_eq!(parts.len(), 384);
        let inverse = u.inverse()?;
        let a0 = decode_g1(&parts[..64])[0].to_projective() + self.0.fixed.alpha;
        if a0.is_identity() {
            return None;
        }
        // The B1 bytes are intentionally not read.
        let c0 = decode_g1(&parts[128..192])[0].to_projective() + decode_g1(&parts[192..256])[0];
        let b0 = decode_g2(&parts[256..384])[0].to_projective() + self.0.fixed.beta2;
        let (a, c, b) = self.windowed_outputs(a0, b0, c0, u, inverse, v);
        let (a, c, b) = (a.to_affine(), c.to_affine(), b.to_affine());
        let mut out = Vec::with_capacity(256);
        for x in [a.x, a.y, c.x, c.y, b.x.c1, b.x.c0, b.y.c1, b.y.c0] {
            be(x, &mut out);
        }
        Some(out)
    }
    pub(crate) fn assemble_nonzero(&self, parts: &[u8]) -> Vec<u8> {
        loop {
            let proof = assemble_blinded(
                parts,
                &self.0.fixed,
                uniform_fr(),
                uniform_fr(),
                self.0.windows.as_ref(),
            );
            if proof[..64].iter().any(|&x| x != 0) {
                return proof;
            }
        }
    }
}

#[cfg(test)]
mod rerandomized_tests {
    use super::*;
    #[test]
    fn transformation_matches_independent_exponent_oracle_and_handles_zero() {
        let g = G1Affine::GENERATOR;
        let g2 = G2Affine::GENERATOR;
        let fixed = FixedPoints {
            alpha: (g * Fr::from(13u64)).to_affine(),
            beta: (g * Fr::from(17u64)).to_affine(),
            delta: (g * Fr::from(19u64)).to_affine(),
            beta2: (g2 * Fr::from(17u64)).to_affine(),
            delta2: (g2 * Fr::from(19u64)).to_affine(),
        };
        for prepared in [false, true] {
            let p = if prepared {
                PreparedFixedPoints(std::sync::Arc::new(Prepared {
                    fixed,
                    windows: Some((FixedBase::new(fixed.delta), FixedBase::new(fixed.delta2))),
                }))
            } else {
                PreparedFixedPoints(std::sync::Arc::new(Prepared {
                    fixed,
                    windows: None,
                }))
            };
            let values = [
                Fr::ZERO,
                Fr::ONE,
                -Fr::ONE,
                Fr::from(2u64),
                Fr::from(123456789u64).pow(&[81]),
            ];
            for a in [Fr::ZERO, Fr::ONE, -Fr::from(13u64), Fr::from(97u64)] {
                let mut parts = Vec::new();
                for x in [a, Fr::from(5u64), Fr::from(7u64), Fr::from(11u64)] {
                    crate::output_g1(&mut parts, g * x);
                }
                crate::output_g2(&mut parts, g2 * Fr::from(5u64));
                for u in values {
                    for v in values {
                        let got = p.rerandomized_with(&parts, u, v);
                        let a0 = a + Fr::from(13u64);
                        if a0.is_zero() || u.is_zero() {
                            assert!(got.is_none());
                            continue;
                        }
                        let aa = (g * (a0 * u.inverse().unwrap())).to_affine();
                        let cc = (g * (Fr::from(18u64) + v * a0)).to_affine();
                        let bb = (g2 * (u * (Fr::from(22u64) + v * Fr::from(19u64)))).to_affine();
                        let mut expected = Vec::new();
                        for x in [aa.x, aa.y, cc.x, cc.y, bb.x.c1, bb.x.c0, bb.y.c1, bb.y.c0] {
                            be(x, &mut expected);
                        }
                        assert_eq!(got.unwrap(), expected);
                        // Validity invariant with the public pairing exponent fixed.
                        assert_eq!(
                            a0 * Fr::from(22u64) - Fr::from(18u64) * Fr::from(19u64),
                            (a0 * u.inverse().unwrap())
                                * (u * (Fr::from(22u64) + v * Fr::from(19u64)))
                                - (Fr::from(18u64) + v * a0) * Fr::from(19u64)
                        );
                        let mut poisoned = parts.clone();
                        poisoned[64..128].fill(255);
                        assert_eq!(p.rerandomized_with(&poisoned, u, v).unwrap(), expected);
                    }
                }
                if (a + Fr::from(13u64)).is_zero() {
                    assert!(p.assemble_rerandomized(&parts).is_none());
                    for _ in 0..4 {
                        assert!(p.assemble_nonzero(&parts)[..64].iter().any(|&x| x != 0));
                    }
                }
            }
        }
    }
    #[test]
    fn rerandomization_rejects_zero_delta() {
        let g = G1Affine::GENERATOR;
        let g2 = G2Affine::GENERATOR;
        let fixed = FixedPoints {
            alpha: g,
            beta: g,
            delta: G1Affine::IDENTITY,
            beta2: g2,
            delta2: g2,
        };
        assert!(!PreparedFixedPoints::new(fixed).rerandomization_key_valid());
        assert!(!PreparedFixedPoints::new(FixedPoints {
            delta: g,
            delta2: G2Affine::IDENTITY,
            ..fixed
        })
        .rerandomization_key_valid());
    }
}

include!("groth16_rerandomize.rs");

#[cfg(zopro_glv_blinding)]
#[path = "groth16/glv_blinding.rs"]
mod glv_blinding;
