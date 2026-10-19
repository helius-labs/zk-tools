//! The zopro key: one file with everything a client needs to prove with a
//! gnark proving key and no gnark. Built once per key, offline, from
//! gnark's key (`pack`); read on the device (`load`).
//!
//! Layout, little endian: a 16-byte magic; the format version; the
//! circuit's input and output counts and flags; the public (the constant
//! one included) and secret input counts and the constraint count; the five
//! vectors' lengths in points; the gnark key's digest; the fixed points and
//! the FFT domain in the kernel's encoding; then five length-prefixed blobs
//! — the key's name, gnark's raw verifying key, the inputs' names (public
//! then secret, newline-separated, the constant one apart), the kernel's
//! solver program and the constraint coefficients, both deflated (the
//! program is the bulk of a key and shrinks six times) — and the vectors
//! A, B, K, Z (G1) and B2 (G2) in gnark's compressed point encoding. The
//! file comes out smaller than gnark's key. A host parses the program once
//! and may cache the kernel's solver image.
use ark_ff::PrimeField;
use rayon::prelude::*;

use crate::{
    decode_g1, decode_g2, decompress,
    groth16::FixedPoints,
    verify::{compress_g1, compress_g2, VerifyingKey},
    Key,
};

const MAGIC: &[u8; 16] = b"zopro-key1\0\0\0\0\0\0";
const FORMAT: u32 = 1;
/// The fixed part after the magic: 14 u32s, the digest, the fixed points
/// and the domain.
const FIXED: usize = 16 + 14 * 4 + 32 + 448 + 64;
/// Deflate at this level: the programs' index streams compress alike from
/// level 4 up, in a tenth of level 9's time.
const DEFLATE_LEVEL: u8 = 6;

/// A parsed key file, borrowing its bytes.
pub struct Artifact<'a> {
    pub inputs: u32,
    pub outputs: u32,
    pub flags: u32,
    /// Public inputs, the constant one included.
    pub public: u32,
    pub secret: u32,
    pub constraints: u32,
    pub digest: [u8; 32],
    /// The vectors' lengths in points: a, b, k, z, b2.
    pub points: [u32; 5],
    pub fixed: &'a [u8],
    pub domain: &'a [u8],
    pub name: &'a str,
    pub verifying_key: &'a [u8],
    /// Public names then secret names, one per line.
    pub names: &'a str,
    /// The kernel's solver program and the constraint coefficients, deflated.
    pub program: &'a [u8],
    pub coefficients: &'a [u8],
    pub a: &'a [u8],
    pub b: &'a [u8],
    pub k: &'a [u8],
    pub z: &'a [u8],
    pub b2: &'a [u8],
}

/// What `pack` takes: the kernel's inputs as gnark hands them over (the
/// dump's files), plus the names.
pub struct Parts<'a> {
    pub name: &'a str,
    pub inputs: u32,
    pub outputs: u32,
    pub flags: u32,
    pub public: u32,
    pub secret: u32,
    pub constraints: u32,
    pub digest: [u8; 32],
    /// The kernel's encoding, 448 bytes.
    pub fixed: &'a [u8],
    /// The kernel's encoding, 64 bytes.
    pub domain: &'a [u8],
    /// gnark's raw verifying key.
    pub verifying_key: &'a [u8],
    pub public_names: &'a [String],
    pub secret_names: &'a [String],
    /// The vectors in the kernel's encoding.
    pub a: &'a [u8],
    pub b: &'a [u8],
    pub k: &'a [u8],
    pub z: &'a [u8],
    pub b2: &'a [u8],
    /// The kernel's solver program and the constraint coefficients.
    pub program: &'a [u8],
    pub coefficients: &'a [u8],
}

/// The key file for `parts`: the vectors compressed, the program and
/// coefficients deflated, everything checked to parse and the program to
/// solve.
pub fn pack(parts: &Parts) -> Result<Vec<u8>, String> {
    if parts.fixed.len() != 448 || parts.domain.len() != 64 {
        return Err("fixed points are 448 bytes and the domain 64".into());
    }
    if parts.public_names.len() + 1 != parts.public as usize
        || parts.secret_names.len() != parts.secret as usize
    {
        return Err("the names do not match the input counts".into());
    }
    VerifyingKey::decode_raw(parts.verifying_key)?;
    FixedPoints::decode(parts.fixed).ok_or("fixed points")?;
    let mut key = Key::build(parts.a, parts.b, parts.k, parts.z, parts.b2, parts.domain)?;
    if !key.set_solver(parts.program, parts.coefficients) {
        return Err("the kernel does not solve this circuit".into());
    }
    drop(key);
    let program = miniz_oxide::deflate::compress_to_vec(parts.program, DEFLATE_LEVEL);
    let coefficients = miniz_oxide::deflate::compress_to_vec(parts.coefficients, DEFLATE_LEVEL);
    let g1 = |bytes: &[u8]| -> Vec<u8> {
        decode_g1(bytes)
            .par_iter()
            .flat_map_iter(compress_g1)
            .collect()
    };
    let b2: Vec<u8> = decode_g2(parts.b2)
        .par_iter()
        .flat_map_iter(compress_g2)
        .collect();
    let (a, b, k, z) = (g1(parts.a), g1(parts.b), g1(parts.k), g1(parts.z));
    let names = parts
        .public_names
        .iter()
        .chain(parts.secret_names)
        .map(|n| format!("{n}\n"))
        .collect::<String>();
    let mut out = Vec::with_capacity(
        FIXED
            + a.len()
            + b.len()
            + k.len()
            + z.len()
            + b2.len()
            + program.len()
            + names.len()
            + 1024,
    );
    out.extend_from_slice(MAGIC);
    for v in [
        FORMAT,
        0,
        parts.inputs,
        parts.outputs,
        parts.flags,
        parts.public,
        parts.secret,
        parts.constraints,
        (parts.a.len() / 64) as u32,
        (parts.b.len() / 64) as u32,
        (parts.k.len() / 64) as u32,
        (parts.z.len() / 64) as u32,
        (parts.b2.len() / 128) as u32,
        0,
    ] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&parts.digest);
    out.extend_from_slice(parts.fixed);
    out.extend_from_slice(parts.domain);
    for blob in [
        parts.name.as_bytes(),
        parts.verifying_key,
        names.as_bytes(),
        &program,
        &coefficients,
    ] {
        out.extend_from_slice(&(blob.len() as u32).to_le_bytes());
        out.extend_from_slice(blob);
    }
    for vector in [&a, &b, &k, &z, &b2] {
        out.extend_from_slice(vector);
    }
    Ok(out)
}

impl<'a> Artifact<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, &'static str> {
        let (mut artifact, mut at) = Self::head(bytes)?;
        let mut vector = |points: u32, size: usize| -> Result<&'a [u8], &'static str> {
            let len = points as usize * size;
            let out = bytes.get(at..at + len).ok_or("truncated key")?;
            at += len;
            Ok(out)
        };
        let [a, b, k, z, b2] = artifact.points;
        artifact.a = vector(a, 32)?;
        artifact.b = vector(b, 32)?;
        artifact.k = vector(k, 32)?;
        artifact.z = vector(z, 32)?;
        artifact.b2 = vector(b2, 64)?;
        if at != bytes.len() {
            return Err("a zopro key with trailing bytes");
        }
        Ok(artifact)
    }

    /// The key without its vectors (empty here), and where they begin: a
    /// host that caches the vectors decompressed keeps the head with them.
    pub fn head(bytes: &'a [u8]) -> Result<(Self, usize), &'static str> {
        if bytes.len() < FIXED || &bytes[..16] != MAGIC {
            return Err("not a zopro key");
        }
        let word = |i: usize| u32::from_le_bytes(bytes[16 + 4 * i..20 + 4 * i].try_into().unwrap());
        if word(0) != FORMAT {
            return Err("a zopro key of another format version");
        }
        let points = [word(8), word(9), word(10), word(11), word(12)];
        let mut digest = [0u8; 32];
        digest.copy_from_slice(&bytes[72..104]);
        let fixed = &bytes[104..552];
        let domain = &bytes[552..616];
        let mut at = FIXED;
        let mut blob = || -> Result<&'a [u8], &'static str> {
            let len = u32::from_le_bytes(
                bytes
                    .get(at..at + 4)
                    .ok_or("truncated key")?
                    .try_into()
                    .unwrap(),
            ) as usize;
            let out = bytes.get(at + 4..at + 4 + len).ok_or("truncated key")?;
            at += 4 + len;
            Ok(out)
        };
        let name = std::str::from_utf8(blob()?).map_err(|_| "key name")?;
        let verifying_key = blob()?;
        let names = std::str::from_utf8(blob()?).map_err(|_| "input names")?;
        let program = blob()?;
        let coefficients = blob()?;
        let artifact = Self {
            inputs: word(2),
            outputs: word(3),
            flags: word(4),
            public: word(5),
            secret: word(6),
            constraints: word(7),
            digest,
            points,
            fixed,
            domain,
            name,
            verifying_key,
            names,
            program,
            coefficients,
            a: &[],
            b: &[],
            k: &[],
            z: &[],
            b2: &[],
        };
        Ok((artifact, at))
    }

    /// The inputs' names: public (the constant one apart) then secret.
    pub fn input_names(&self) -> Vec<&'a str> {
        self.names.lines().collect()
    }

    /// The vectors decompressed into the kernel's encoding: a, b, k, z, b2.
    /// Seconds for a wide key on a phone; a host may cache them.
    pub fn vectors(&self) -> Result<[Vec<u8>; 5], &'static str> {
        Ok([
            kernel_g1(&decompress::g1(self.a)?),
            kernel_g1(&decompress::g1(self.b)?),
            kernel_g1(&decompress::g1(self.k)?),
            kernel_g1(&decompress::g1(self.z)?),
            kernel_g2(&decompress::g2(self.b2)?),
        ])
    }

    /// The kernel's key from decompressed `vectors` (`vectors()`, or a
    /// host's cache of them), its solver from the program, or from
    /// `solver_image` when a host cached the kernel's own
    /// (`Key::solver_image`) and the kernel still reads it.
    pub fn key(&self, vectors: &[Vec<u8>; 5], solver_image: Option<&[u8]>) -> Result<Key, String> {
        let [a, b, k, z, b2] = vectors;
        let mut key = Key::build(a, b, k, z, b2, self.domain)?;
        if let Some(image) = solver_image {
            if key.set_solver_image(image) {
                return Ok(key);
            }
        }
        let inflate = |bytes: &[u8]| {
            miniz_oxide::inflate::decompress_to_vec(bytes)
                .map_err(|_| "the key's program is damaged")
        };
        let (program, coefficients) = (inflate(self.program)?, inflate(self.coefficients)?);
        if !key.set_solver(&program, &coefficients) {
            return Err("the kernel does not solve the key's program".into());
        }
        Ok(key)
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey, &'static str> {
        VerifyingKey::decode_raw(self.verifying_key)
    }

    pub fn fixed_points(&self) -> Result<FixedPoints, &'static str> {
        FixedPoints::decode(self.fixed).ok_or("fixed points")
    }
}

/// Raw big-endian G1 points (64 bytes) in the kernel's encoding: each
/// coordinate's Montgomery limbs, little endian.
fn kernel_g1(raw: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; raw.len()];
    raw.par_chunks(32)
        .zip(out.par_chunks_mut(32))
        .for_each(|(raw, out)| kernel_fq(raw, out));
    out
}

fn kernel_g2(raw: &[u8]) -> Vec<u8> {
    // gnark's raw G2 is x.c1, x.c0, y.c1, y.c0; the kernel's x.c0, x.c1,
    // y.c0, y.c1.
    let mut out = vec![0u8; raw.len()];
    raw.par_chunks(128)
        .zip(out.par_chunks_mut(128))
        .for_each(|(raw, out)| {
            kernel_fq(&raw[..32], &mut out[32..64]);
            kernel_fq(&raw[32..64], &mut out[..32]);
            kernel_fq(&raw[64..96], &mut out[96..128]);
            kernel_fq(&raw[96..128], &mut out[64..96]);
        });
    out
}

fn kernel_fq(raw: &[u8], out: &mut [u8]) {
    if raw.iter().all(|&b| b == 0) {
        out.fill(0);
        return;
    }
    let value = ark_bn254::Fq::from_be_bytes_mod_order(raw);
    for (limb, word) in value.0 .0.iter().zip(out.chunks_mut(8)) {
        word.copy_from_slice(&limb.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_what_is_not_a_key() {
        assert!(Artifact::parse(b"zopro-key1").is_err());
        let mut bytes = vec![0u8; FIXED];
        bytes[..16].copy_from_slice(MAGIC);
        bytes[16..20].copy_from_slice(&FORMAT.to_le_bytes());
        assert_eq!(Artifact::parse(&bytes).err(), Some("truncated key"));
    }
}
