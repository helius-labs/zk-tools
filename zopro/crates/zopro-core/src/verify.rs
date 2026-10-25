//! gnark's Groth16 verifying key and proof formats, and the verification:
//! the client checks every proof it hands out, and encodes it as gnark's
//! verifier and the chain read it.
use zopro_bn254::{
    pairing::{final_exponentiation, multi_miller_loop, pairing, G2Prepared},
    Fq, Fq12, Fq2, Fr, G1Affine, G1Projective, G2Affine,
};

use crate::decompress::{coordinate, largest, largest2};

/// gnark's point flags in a compressed encoding's first byte.
const SMALLEST: u8 = 0b10 << 6;
const LARGEST: u8 = 0b11 << 6;
const INFINITY: u8 = 0b01 << 6;

/// A G1 point from gnark's raw (uncompressed) encoding: x then y, big
/// endian; all zero for the point at infinity.
fn raw_g1(bytes: &[u8]) -> Result<G1Affine, &'static str> {
    if bytes.iter().all(|&b| b == 0) {
        return Ok(G1Affine::IDENTITY);
    }
    let point = G1Affine::new(coordinate(&bytes[..32])?, coordinate(&bytes[32..64])?);
    // G1 has cofactor one: on the curve is in the group.
    if !point.is_on_curve() {
        return Err("G1 point not on the curve");
    }
    Ok(point)
}

/// A G2 point from gnark's raw encoding: x.c1, x.c0, y.c1, y.c0.
fn raw_g2(bytes: &[u8]) -> Result<G2Affine, &'static str> {
    if bytes.iter().all(|&b| b == 0) {
        return Ok(G2Affine::IDENTITY);
    }
    let fq2 = |at: usize| -> Result<Fq2, &'static str> {
        Ok(Fq2::new(
            coordinate(&bytes[at + 32..at + 64])?,
            coordinate(&bytes[at..at + 32])?,
        ))
    };
    let point = G2Affine::new(fq2(0)?, fq2(64)?);
    if !point.is_on_curve() || !point.is_in_subgroup() {
        return Err("G2 point not on the curve");
    }
    Ok(point)
}

fn be(value: &Fq, out: &mut Vec<u8>) {
    out.extend_from_slice(&value.to_be_bytes());
}

/// gnark's compressed G1 encoding: x big endian, the first byte's top
/// bits `10` or `11` by whether y is the larger of the two roots, `01`
/// (and zeros) for the point at infinity.
pub fn compress_g1(point: &G1Affine) -> [u8; 32] {
    let mut out = [0u8; 32];
    if point.is_identity() {
        out[0] = INFINITY;
    } else {
        out.copy_from_slice(&point.x.to_be_bytes());
        out[0] |= if largest(&point.y) { LARGEST } else { SMALLEST };
    }
    out
}

/// gnark's compressed G2 encoding: x.c1 then x.c0, the flags as for G1
/// with gnark's order on Fq2.
pub fn compress_g2(point: &G2Affine) -> [u8; 64] {
    let mut out = [0u8; 64];
    if point.is_identity() {
        out[0] = INFINITY;
    } else {
        out[..32].copy_from_slice(&point.x.c1.to_be_bytes());
        out[32..].copy_from_slice(&point.x.c0.to_be_bytes());
        out[0] |= if largest2(&point.y) { LARGEST } else { SMALLEST };
    }
    out
}

/// A Groth16 verifying key as gnark serializes it (`WriteRawTo`, points
/// uncompressed), with the pairings of its fixed points prepared.
#[derive(Clone)]
pub struct VerifyingKey {
    /// The fixed pairing e(-alpha, beta), prepared once with the key.
    negative_alpha_beta: Fq12,
    gamma2: G2Prepared,
    delta2: G2Prepared,
    /// The public inputs' points, the constant one's first.
    k: Vec<G1Affine>,
    /// Fixed-base windows, excluding the constant input.
    k_windows: Option<Vec<G1Affine>>,
}

/// A raw verifying key's alpha, beta and delta, which a proving key's
/// fixed points must be.
pub fn fixed_points(bytes: &[u8]) -> Result<crate::groth16::FixedPoints, &'static str> {
    let short = "verifying key too short";
    let g1 = |at: usize| bytes.get(at..at + 64).ok_or(short).and_then(raw_g1);
    let g2 = |at: usize| bytes.get(at..at + 128).ok_or(short).and_then(raw_g2);
    Ok(crate::groth16::FixedPoints {
        alpha: g1(0)?,
        beta: g1(64)?,
        delta: g1(384)?,
        beta2: g2(128)?,
        delta2: g2(448)?,
    })
}

impl VerifyingKey {
    /// From gnark's raw encoding: alpha, beta (G1), beta, gamma (G2), delta
    /// (G1), delta (G2), the K points (a big-endian count), then the
    /// commitment lists, which must be empty: the circuits commit to
    /// nothing.
    pub fn decode_raw(bytes: &[u8]) -> Result<Self, &'static str> {
        let short = "verifying key too short";
        let g1 = |at: usize| bytes.get(at..at + 64).ok_or(short).and_then(raw_g1);
        let g2 = |at: usize| bytes.get(at..at + 128).ok_or(short).and_then(raw_g2);
        let alpha = g1(0)?;
        let _beta = g1(64)?;
        let beta2 = g2(128)?;
        let gamma2 = g2(256)?;
        let _delta = g1(384)?;
        let delta2 = g2(448)?;
        let count =
            u32::from_be_bytes(bytes.get(576..580).ok_or(short)?.try_into().unwrap()) as usize;
        let mut k = Vec::with_capacity(count);
        let mut at = 580;
        for _ in 0..count {
            k.push(g1(at)?);
            at += 64;
        }
        let committed = u32::from_be_bytes(bytes.get(at..at + 4).ok_or(short)?.try_into().unwrap());
        let commitment_keys =
            u32::from_be_bytes(bytes.get(at + 4..at + 8).ok_or(short)?.try_into().unwrap());
        if committed != 0 || commitment_keys != 0 {
            return Err("a verifying key with commitments");
        }
        if at + 8 != bytes.len() {
            return Err("verifying key with trailing bytes");
        }
        if k.is_empty() {
            return Err("verifying key without public inputs");
        }
        let k_windows = (k.len() <= 17 && crate::simd::cpu_option("ZOPRO_CPU_PREPARE_PUBLIC"))
            .then(|| prepare_public_windows(&k[1..]));
        Ok(Self {
            k_windows,
            negative_alpha_beta: pairing(&-alpha, &beta2),
            gamma2: G2Prepared::new(&gamma2),
            delta2: G2Prepared::new(&delta2),
            k,
        })
    }

    /// The public inputs the key expects, the constant one apart.
    pub fn public_inputs(&self) -> usize {
        self.k.len() - 1
    }

    /// Whether `proof` proves `public`: e(Ar, Bs) = e(alpha, beta)
    /// e(sum of the inputs' K, gamma) e(Krs, delta).
    pub fn verify(&self, proof: &Proof, public: &[Fr]) -> bool {
        if public.len() != self.public_inputs() {
            return false;
        }
        let inputs = if let Some(windows) = &self.k_windows {
            public_inputs_prepared(self.k[0], windows, public)
        } else {
            let mut scalars = Vec::with_capacity(self.k.len());
            scalars.push(Fr::ONE);
            scalars.extend_from_slice(public);
            zopro_bn254::curve::msm(&self.k, &scalars).to_affine()
        };
        let bs = G2Prepared::new(&proof.bs);
        let pairs = [
            (-proof.ar, &bs),
            (inputs, &self.gamma2),
            (proof.krs, &self.delta2),
        ];
        final_exponentiation(proof_miller_product(
            &pairs,
            crate::simd::cpu_option("ZOPRO_CPU_VERIFY_PARALLEL"),
        )) == Some(self.negative_alpha_beta)
    }
}

// The product of independent Miller loops equals their shared recurrence.
// Use available native workers after proving; one worker retains the shared
// loop, which does less total squaring. Final exponentiation is unchanged.
fn proof_miller_product(pairs: &[(G1Affine, &G2Prepared); 3], parallel: bool) -> Fq12 {
    #[cfg(not(target_arch = "wasm32"))]
    if parallel && rayon::current_num_threads() >= 3 {
        let (a, (b, c)) = rayon::join(
            || multi_miller_loop(&pairs[..1]),
            || {
                rayon::join(
                    || multi_miller_loop(&pairs[1..2]),
                    || multi_miller_loop(&pairs[2..]),
                )
            },
        );
        return a * b * c;
    }
    let _ = parallel;
    multi_miller_loop(pairs)
}

// Four-bit fixed-base windows: 64 * 15 points per public input. The
// preparation cap above bounds this table to 960 KiB per verifying key.
fn prepare_public_windows(bases: &[G1Affine]) -> Vec<G1Affine> {
    let mut points = Vec::with_capacity(bases.len() * 64 * 15);
    for base in bases {
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
    }
    G1Projective::batch_to_affine(&points)
}

fn public_inputs_prepared(constant: G1Affine, windows: &[G1Affine], public: &[Fr]) -> G1Affine {
    let mut result = constant.to_projective();
    for (table, scalar) in windows.chunks_exact(64 * 15).zip(public) {
        let words = scalar.to_canonical();
        for window in 0..64 {
            let digit = ((words[window / 16] >> (4 * (window % 16))) & 15) as usize;
            if digit != 0 {
                result += table[window * 15 + digit - 1];
            }
        }
    }
    result.to_affine()
}

/// A Groth16 proof's three points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Proof {
    pub ar: G1Affine,
    pub bs: G2Affine,
    pub krs: G1Affine,
}

impl Proof {
    /// From `groth16::assemble`'s 256 bytes: Ar, Krs (x, y) then Bs (x.c1,
    /// x.c0, y.c1, y.c0), big endian.
    pub fn from_raw(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != 256 {
            return Err("a raw proof is 256 bytes");
        }
        Ok(Self {
            ar: raw_g1(&bytes[..64])?,
            krs: raw_g1(&bytes[64..128])?,
            bs: raw_g2(&bytes[128..256])?,
        })
    }

    /// `from_raw`'s 256 bytes.
    pub fn to_raw(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(256);
        for v in [&self.ar.x, &self.ar.y, &self.krs.x, &self.krs.y] {
            be(v, &mut out);
        }
        for v in [&self.bs.x.c1, &self.bs.x.c0, &self.bs.y.c1, &self.bs.y.c0] {
            be(v, &mut out);
        }
        out
    }

    /// gnark's compressed proof (`Proof.WriteTo`): Ar, Bs, Krs compressed,
    /// no commitments (a zero count) and the commitment proof of knowledge
    /// at infinity: 164 bytes.
    pub fn to_gnark(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(164);
        out.extend_from_slice(&compress_g1(&self.ar));
        out.extend_from_slice(&compress_g2(&self.bs));
        out.extend_from_slice(&compress_g1(&self.krs));
        out.extend_from_slice(&0u32.to_be_bytes());
        out.extend_from_slice(&compress_g1(&G1Affine::IDENTITY));
        out
    }
}

/// gnark's binary public witness (`witness.MarshalBinary` of `Public()`):
/// big-endian counts of public and secret values, the vector's length, then
/// the values big endian.
pub fn public_witness(values: &[Fr]) -> Vec<u8> {
    let mut out = Vec::with_capacity(12 + 32 * values.len());
    out.extend_from_slice(&(values.len() as u32).to_be_bytes());
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(&(values.len() as u32).to_be_bytes());
    for value in values {
        out.extend_from_slice(&value.to_be_bytes());
    }
    out
}

pub fn hex_of(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parallel_miller_product_matches_shared_loop() {
        use zopro_bn254::{G2Affine, G2Projective};
        let mut rng = crate::testing::Seeded::new(0x1234568ab);
        for threads in [1, 3, 8] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for _ in 0..8 {
                    let p: [_; 3] = core::array::from_fn(|_| {
                        (G1Projective::generator() * rng.fr()).to_affine()
                    });
                    let q: [_; 3] = core::array::from_fn(|_| {
                        (G2Projective::generator() * rng.fr()).to_affine()
                    });
                    let qs = q.map(|q| G2Prepared::new(&q));
                    let identity = G2Prepared::new(&G2Affine::IDENTITY);
                    for mask in 0..64 {
                        let pairs = core::array::from_fn(|i| {
                            (
                                if mask >> i & 1 == 0 {
                                    p[i]
                                } else {
                                    G1Affine::IDENTITY
                                },
                                if mask >> (i + 3) & 1 == 0 {
                                    &qs[i]
                                } else {
                                    &identity
                                },
                            )
                        });
                        let expected = multi_miller_loop(&pairs);
                        let actual = proof_miller_product(&pairs, true);
                        assert_eq!(actual, expected);
                        assert_eq!(final_exponentiation(actual), final_exponentiation(expected));
                    }
                }
            });
        }
    }

    #[test]
    fn prepared_public_inputs_match_reference() {
        let g = G1Affine::GENERATOR;
        for bases in [
            vec![],
            vec![g],
            vec![g, G1Affine::IDENTITY, (g * Fr::from(7u64)).to_affine()],
        ] {
            let windows = prepare_public_windows(&bases);
            let all: Vec<_> = std::iter::once(g).chain(bases.iter().copied()).collect();
            for value in [
                Fr::ZERO,
                Fr::ONE,
                -Fr::ONE,
                Fr::from(15u64),
                Fr::from(16u64),
                crate::random_fr(),
            ] {
                let public: Vec<_> = (0..bases.len())
                    .map(|i| value + Fr::from(i as u64))
                    .collect();
                let scalars: Vec<_> = std::iter::once(Fr::ONE)
                    .chain(public.iter().copied())
                    .collect();
                assert_eq!(
                    public_inputs_prepared(g, &windows, &public),
                    zopro_bn254::curve::msm(&all, &scalars).to_affine()
                );
            }
        }
    }
}
