//! gnark's Groth16 proof assembly over the kernel's parts, with fresh
//! blinding: Ar = A + alpha + r delta, Bs = B2 + beta2 + s delta2,
//! Krs = K + Z - r s delta + s Ar + r Bs1 with Bs1 = B + beta + s delta.
use ark_bn254::{Fq, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField, UniformRand};
use rand::rngs::OsRng;

use crate::{decode_g1, decode_g2};

/// The proving key's fixed points, as a key image head carries them.
#[derive(Clone, Copy)]
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
    out.extend_from_slice(&v.into_bigint().to_bytes_be());
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
    let mut rng = OsRng;
    let (r, s) = (Fr::rand(&mut rng), Fr::rand(&mut rng));
    let delta: G1Projective = fixed.delta.into_group();
    let ar = a.into_group() + fixed.alpha.into_group() + delta * r;
    let bs1 = b.into_group() + fixed.beta.into_group() + delta * s;
    let bs: G2Projective = b2.into_group() + fixed.beta2.into_group() + fixed.delta2.into_group() * s;
    let krs = k.into_group() + z.into_group() + delta * (-(r * s)) + ar * s + bs1 * r;
    let (ar, krs, bs) = (ar.into_affine(), krs.into_affine(), bs.into_affine());
    let mut out = Vec::with_capacity(256);
    for v in [ar.x, ar.y, krs.x, krs.y] {
        be(v, &mut out);
    }
    for v in [bs.x.c1, bs.x.c0, bs.y.c1, bs.y.c0] {
        be(v, &mut out);
    }
    out
}
