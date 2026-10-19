//! gnark's Groth16 verifying key and proof formats, and the verification:
//! the client checks every proof it hands out, and encodes it as gnark's
//! verifier and the chain read it.
use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine};
use ark_ec::{
    pairing::{Pairing, PairingOutput},
    AffineRepr, CurveGroup, VariableBaseMSM,
};
use ark_ff::{BigInteger, One, PrimeField, Zero};

use crate::decompress::{coordinate, largest, largest2};

/// gnark's point flags in a compressed encoding's first byte.
const SMALLEST: u8 = 0b10 << 6;
const LARGEST: u8 = 0b11 << 6;
const INFINITY: u8 = 0b01 << 6;

/// A G1 point from gnark's raw (uncompressed) encoding: x then y, big
/// endian; all zero for the point at infinity.
fn raw_g1(bytes: &[u8]) -> Result<G1Affine, &'static str> {
    if bytes.iter().all(|&b| b == 0) {
        return Ok(G1Affine::identity());
    }
    let point = G1Affine::new_unchecked(coordinate(&bytes[..32])?, coordinate(&bytes[32..64])?);
    if !point.is_on_curve() || !point.is_in_correct_subgroup_assuming_on_curve() {
        return Err("G1 point not on the curve");
    }
    Ok(point)
}

/// A G2 point from gnark's raw encoding: x.c1, x.c0, y.c1, y.c0.
fn raw_g2(bytes: &[u8]) -> Result<G2Affine, &'static str> {
    if bytes.iter().all(|&b| b == 0) {
        return Ok(G2Affine::identity());
    }
    let fq2 = |at: usize| -> Result<Fq2, &'static str> {
        Ok(Fq2::new(
            coordinate(&bytes[at + 32..at + 64])?,
            coordinate(&bytes[at..at + 32])?,
        ))
    };
    let point = G2Affine::new_unchecked(fq2(0)?, fq2(64)?);
    if !point.is_on_curve() || !point.is_in_correct_subgroup_assuming_on_curve() {
        return Err("G2 point not on the curve");
    }
    Ok(point)
}

fn be(value: &Fq, out: &mut Vec<u8>) {
    out.extend_from_slice(&value.into_bigint().to_bytes_be());
}

/// gnark's compressed G1 encoding: x big endian, the first byte's top
/// bits `10` or `11` by whether y is the larger of the two roots, `01`
/// (and zeros) for the point at infinity.
pub fn compress_g1(point: &G1Affine) -> [u8; 32] {
    let mut out = [0u8; 32];
    match point.xy() {
        None => out[0] = INFINITY,
        Some((x, y)) => {
            out.copy_from_slice(&x.into_bigint().to_bytes_be());
            out[0] |= if largest(&y) { LARGEST } else { SMALLEST };
        }
    }
    out
}

/// gnark's compressed G2 encoding: x.c1 then x.c0, the flags as for G1
/// with gnark's order on Fq2.
pub fn compress_g2(point: &G2Affine) -> [u8; 64] {
    let mut out = [0u8; 64];
    match point.xy() {
        None => out[0] = INFINITY,
        Some((x, y)) => {
            out[..32].copy_from_slice(&x.c1.into_bigint().to_bytes_be());
            out[32..].copy_from_slice(&x.c0.into_bigint().to_bytes_be());
            out[0] |= if largest2(&y) { LARGEST } else { SMALLEST };
        }
    }
    out
}

/// A Groth16 verifying key as gnark serializes it (`WriteRawTo`, points
/// uncompressed), with the pairings of its fixed points prepared.
pub struct VerifyingKey {
    alpha: G1Affine,
    beta2: <Bn254 as Pairing>::G2Prepared,
    gamma2: <Bn254 as Pairing>::G2Prepared,
    delta2: <Bn254 as Pairing>::G2Prepared,
    /// The public inputs' points, the constant one's first.
    k: Vec<G1Affine>,
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
        Ok(Self {
            alpha,
            beta2: beta2.into(),
            gamma2: gamma2.into(),
            delta2: delta2.into(),
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
        let mut scalars = Vec::with_capacity(self.k.len());
        scalars.push(Fr::one());
        scalars.extend_from_slice(public);
        let inputs: G1Projective =
            VariableBaseMSM::msm(&self.k, &scalars).expect("matching lengths");
        let g1 = [
            <Bn254 as Pairing>::G1Prepared::from(-proof.ar.into_group()),
            self.alpha.into(),
            inputs.into_affine().into(),
            proof.krs.into(),
        ];
        let g2 = [
            <Bn254 as Pairing>::G2Prepared::from(proof.bs),
            self.beta2.clone(),
            self.gamma2.clone(),
            self.delta2.clone(),
        ];
        Bn254::multi_pairing(g1, g2) == PairingOutput::zero()
    }
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

    /// From the hex coordinates of `to_json`.
    pub fn from_json(json: &serde_json::Value) -> Result<Self, &'static str> {
        let hex = |value: &serde_json::Value| -> Result<Fq, &'static str> {
            let text = value.as_str().ok_or("a coordinate is a string")?;
            let text = text
                .strip_prefix("0x")
                .ok_or("a coordinate starts with 0x")?;
            if text.len() != 64 {
                return Err("a coordinate is 64 hex digits");
            }
            let mut bytes = [0u8; 32];
            for (i, byte) in bytes.iter_mut().enumerate() {
                *byte = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).map_err(|_| "not hex")?;
            }
            coordinate(&bytes)
        };
        let g1 = |value: &serde_json::Value| -> Result<G1Affine, &'static str> {
            let pair = value
                .as_array()
                .filter(|a| a.len() == 2)
                .ok_or("a G1 point is [x, y]")?;
            let point = G1Affine::new_unchecked(hex(&pair[0])?, hex(&pair[1])?);
            if !point.is_on_curve() || !point.is_in_correct_subgroup_assuming_on_curve() {
                return Err("G1 point not on the curve");
            }
            Ok(point)
        };
        let fq2 = |value: &serde_json::Value| -> Result<Fq2, &'static str> {
            let pair = value
                .as_array()
                .filter(|a| a.len() == 2)
                .ok_or("an Fq2 is [c1, c0]")?;
            Ok(Fq2::new(hex(&pair[1])?, hex(&pair[0])?))
        };
        let bs = json["bs"]
            .as_array()
            .filter(|a| a.len() == 2)
            .ok_or("bs is [x, y]")?;
        let bs = G2Affine::new_unchecked(fq2(&bs[0])?, fq2(&bs[1])?);
        if !bs.is_on_curve() || !bs.is_in_correct_subgroup_assuming_on_curve() {
            return Err("G2 point not on the curve");
        }
        Ok(Self {
            ar: g1(&json["ar"])?,
            bs,
            krs: g1(&json["krs"])?,
        })
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
        out.extend_from_slice(&compress_g1(&G1Affine::identity()));
        out
    }

    /// The SDKs' proof JSON: `ar` [x, y], `bs` [[x.c1, x.c0], [y.c1, y.c0]],
    /// `krs` [x, y], each coordinate 0x and 64 hex digits, and the proving
    /// key's digest when given.
    pub fn to_json(&self, proving_key_sha256: Option<&str>) -> serde_json::Value {
        let hex = |value: &Fq| {
            let mut out = Vec::with_capacity(32);
            be(value, &mut out);
            serde_json::Value::String(format!("0x{}", hex_of(&out)))
        };
        let mut json = serde_json::json!({
            "ar": [hex(&self.ar.x), hex(&self.ar.y)],
            "bs": [[hex(&self.bs.x.c1), hex(&self.bs.x.c0)], [hex(&self.bs.y.c1), hex(&self.bs.y.c0)]],
            "krs": [hex(&self.krs.x), hex(&self.krs.y)],
        });
        if let Some(digest) = proving_key_sha256 {
            json["provingKeySha256"] = serde_json::Value::String(digest.into());
        }
        json
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
        out.extend_from_slice(&value.into_bigint().to_bytes_be());
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
