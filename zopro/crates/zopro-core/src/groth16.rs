//! gnark's Groth16 proof assembly over the kernel's parts, with fresh
//! blinding: Ar = A + alpha + r delta, Bs = B2 + beta2 + s delta2,
//! Krs = K + Z - r s delta + s Ar + r Bs1 with Bs1 = B + beta + s delta.
use zopro_bn254::{Fq, G1Affine, G1Projective, G2Affine, G2Projective};

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

fn be(v: Fq, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_be_bytes());
}

/// The proof from `Key::solve_parts`' or `Key::parts`' 384 bytes (A, B, K,
/// Z, B2 in the kernel's encoding): Ar, Krs, Bs in gnark's raw big-endian
/// encoding, 256 bytes.
pub fn assemble(parts: &[u8], fixed: &FixedPoints) -> Vec<u8> {
    assert_eq!(parts.len(), 384, "the kernel's five parts");
    let (a, b, k, z) = (
        decode_g1(&parts[..64])[0],
        decode_g1(&parts[64..128])[0],
        decode_g1(&parts[128..192])[0],
        decode_g1(&parts[192..256])[0],
    );
    let b2 = decode_g2(&parts[256..384])[0];
    let (r, s) = (crate::random_fr(), crate::random_fr());
    let delta: G1Projective = fixed.delta.to_projective();
    let ar = a.to_projective() + fixed.alpha.to_projective() + delta * r;
    let bs1 = b.to_projective() + fixed.beta.to_projective() + delta * s;
    let bs: G2Projective =
        b2.to_projective() + fixed.beta2.to_projective() + fixed.delta2.to_projective() * s;
    let krs = k.to_projective() + z.to_projective() + delta * (-(r * s)) + ar * s + bs1 * r;
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
