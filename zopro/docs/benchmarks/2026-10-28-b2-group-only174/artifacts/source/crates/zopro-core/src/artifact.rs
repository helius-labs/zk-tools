//! The zopro key: one file with everything a client needs to prove with a
//! gnark Groth16 proving key over BN254 and no gnark. Built once per key,
//! offline, from gnark's constraint system and keys (`pack`, over a dump
//! that `tools/gnark` writes); read on the device (`Artifact::parse`).
//!
//! Layout, little endian: a 16-byte magic; the format version; the public
//! (the constant one included) and secret input counts and the constraint
//! count; the five vectors' lengths in points; the fixed points and the FFT
//! domain in the kernel's encoding; then six length-prefixed blobs — the
//! key's name, the application's metadata (opaque here), gnark's raw
//! verifying key, the inputs' names (public then secret, newline-separated,
//! the constant one apart), the kernel's solver program and the constraint
//! coefficients, both deflated (the program is the bulk of a key and
//! shrinks six times) — and the vectors A, B, K, Z (G1) and B2 (G2) in
//! gnark's compressed point encoding. The fixed points must be the
//! verifying key's: `pack` and `fixed_points` refuse a key whose are not,
//! since a prover blinds its proofs with them. A host parses the program
//! once and may cache the kernel's solver image. Format 3 uses u64 byte
//! lengths for the six blobs; format 2's u32 lengths remain readable.
use rayon::prelude::*;

use crate::{
    decode_g1, decode_g2, decompress,
    groth16::FixedPoints,
    verify::{compress_g1, compress_g2, VerifyingKey},
    Key, Points,
};

const MAGIC: &[u8; 16] = b"zopro-key\0\0\0\0\0\0\0";
const FORMAT: u32 = 3;
const LEGACY_FORMAT: u32 = 2;
/// The fixed part after the magic: 9 u32s, the fixed points and the
/// domain.
const FIXED: usize = 16 + 9 * 4 + 448 + 64;
/// Deflate at this level: the programs' index streams compress alike from
/// level 4 up, in a tenth of level 9's time.
const DEFLATE_LEVEL: u8 = 6;

/// A parsed key file, borrowing its bytes.
pub struct Artifact<'a> {
    /// Public inputs, the constant one included.
    pub public: u32,
    pub secret: u32,
    pub constraints: u32,
    /// The vectors' lengths in points: a, b, k, z, b2.
    pub points: [u32; 5],
    pub fixed: &'a [u8],
    pub domain: &'a [u8],
    pub name: &'a str,
    /// What the application that built the key keeps in it; opaque here.
    pub metadata: &'a [u8],
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
    pub metadata: &'a [u8],
    pub public: u32,
    pub secret: u32,
    pub constraints: u32,
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

impl Parts<'_> {
    /// A raw image stores the compiled solver separately. Its header needs
    /// no duplicate deflated source program or coefficient table.
    pub fn image_head(&self) -> Result<Vec<u8>, String> {
        pack_head_inner(self, false)
    }

    /// Build and stream a solver image with its u64 byte length. No second
    /// allocation holding the complete serialized plan is needed.
    pub fn write_solver_image(&self, out: &mut impl std::io::Write) -> Result<(), String> {
        let plan = crate::solver::Plan::new(self.program, self.coefficients)?;
        if !plan.matches(self.a.len() / 64, self.b.len() / 64, self.k.len() / 64, self.z.len() / 64 + 1) {
            return Err("the key's program does not match its points".into());
        }
        write_plan(&plan, out)
    }
}

fn write_plan(plan: &crate::solver::Plan, out: &mut impl std::io::Write) -> Result<(), String> {
    out.write_all(&plan.image_len().to_le_bytes()).map_err(|e| e.to_string())?;
    plan.write_image(out).map_err(|e| e.to_string())
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
    check_fixed(parts.fixed, parts.verifying_key)?;
    let mut key = Key::build(
        Points::Kernel([parts.a, parts.b, parts.k, parts.z, parts.b2]),
        parts.domain,
    )?;
    if !key.set_solver(parts.program, parts.coefficients) {
        return Err("the kernel does not solve this circuit".into());
    }
    drop(key);
    let mut out = pack_head(parts)?;
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
    for vector in [&a, &b, &k, &z, &b2] {
        out.extend_from_slice(vector);
    }
    Ok(out)
}

/// The header of a key, without compressing or copying its point vectors.
/// A raw-image writer can append the original kernel-encoded vectors and
/// build the solver separately, without holding those copies in memory.
pub fn pack_head(parts: &Parts) -> Result<Vec<u8>, String> {
    pack_head_inner(parts, true)
}

fn pack_head_inner(parts: &Parts, keep_program: bool) -> Result<Vec<u8>, String> {
    if parts.fixed.len() != 448 || parts.domain.len() != 64 {
        return Err("fixed points are 448 bytes and the domain 64".into());
    }
    if parts.public_names.len().checked_add(1) != Some(parts.public as usize)
        || parts.secret_names.len() != parts.secret as usize
    {
        return Err("the names do not match the input counts".into());
    }
    VerifyingKey::decode_raw(parts.verifying_key)?;
    check_fixed(parts.fixed, parts.verifying_key)?;
    let vectors = [parts.a, parts.b, parts.k, parts.z, parts.b2];
    let mut counts = [0u32; 5];
    for (i, vector) in vectors.iter().enumerate() {
        let size = if i == 4 { 128 } else { 64 };
        if vector.len() % size != 0 {
            return Err("partial point in key vector".into());
        }
        counts[i] = u32::try_from(vector.len() / size).map_err(|_| "too many key points")?;
    }
    let program = if keep_program {
        miniz_oxide::deflate::compress_to_vec(parts.program, DEFLATE_LEVEL)
    } else { Vec::new() };
    let coefficients = if keep_program {
        miniz_oxide::deflate::compress_to_vec(parts.coefficients, DEFLATE_LEVEL)
    } else { Vec::new() };
    let names = parts
        .public_names
        .iter()
        .chain(parts.secret_names)
        .map(|n| format!("{n}\n"))
        .collect::<String>();
    Ok(encode_head([
        parts.public,
        parts.secret,
        parts.constraints,
        counts[0], counts[1], counts[2], counts[3], counts[4],
    ], parts.fixed, parts.domain, [
        parts.name.as_bytes(),
        parts.metadata,
        parts.verifying_key,
        names.as_bytes(),
        &program,
        &coefficients,
    ]))
}

fn encode_head(counts: [u32; 8], fixed: &[u8], domain: &[u8], blobs: [&[u8]; 6]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&FORMAT.to_le_bytes());
    for count in counts {
        out.extend_from_slice(&count.to_le_bytes());
    }
    out.extend_from_slice(fixed);
    out.extend_from_slice(domain);
    for blob in blobs {
        out.extend_from_slice(&(blob.len() as u64).to_le_bytes());
        out.extend_from_slice(blob);
    }
    out
}

impl<'a> Artifact<'a> {
    /// Header for a cached raw image, whose serialized solver already
    /// contains the coefficients and execution plan. The original key can
    /// regenerate this cache if a future solver format becomes incompatible.
    pub fn image_head(&self) -> Vec<u8> {
        let [a, b, k, z, b2] = self.points;
        encode_head([self.public, self.secret, self.constraints, a, b, k, z, b2],
            self.fixed, self.domain,
            [self.name.as_bytes(), self.metadata, self.verifying_key, self.names.as_bytes(), &[], &[]])
    }

    pub fn parse(bytes: &'a [u8]) -> Result<Self, &'static str> {
        let (mut artifact, mut at) = Self::head(bytes)?;
        let mut vector = |points: u32, size: usize| -> Result<&'a [u8], &'static str> {
            let len = (points as usize).checked_mul(size).ok_or("key vector overflow")?;
            let end = at.checked_add(len).ok_or("key vector overflow")?;
            let out = bytes.get(at..end).ok_or("truncated key")?;
            at = end;
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
        let wide = word(0) == FORMAT;
        if !wide && word(0) != LEGACY_FORMAT {
            return Err("a zopro key of another format version");
        }
        let points = [word(4), word(5), word(6), word(7), word(8)];
        let fixed = &bytes[52..500];
        let domain = &bytes[500..564];
        let mut at = FIXED;
        let mut blob = || -> Result<&'a [u8], &'static str> {
            let prefix_end = at.checked_add(if wide { 8 } else { 4 }).ok_or("key blob overflow")?;
            let prefix = bytes.get(at..prefix_end).ok_or("truncated key")?;
            let len = if wide {
                usize::try_from(u64::from_le_bytes(prefix.try_into().unwrap()))
                    .map_err(|_| "key blob length exceeds address space")?
            } else {
                u32::from_le_bytes(prefix.try_into().unwrap()) as usize
            };
            let end = prefix_end.checked_add(len).ok_or("key blob overflow")?;
            let out = bytes.get(prefix_end..end).ok_or("truncated key")?;
            at = end;
            Ok(out)
        };
        let name = std::str::from_utf8(blob()?).map_err(|_| "key name")?;
        let metadata = blob()?;
        let verifying_key = blob()?;
        let names = std::str::from_utf8(blob()?).map_err(|_| "input names")?;
        let program = blob()?;
        let coefficients = blob()?;
        let artifact = Self {
            public: word(1),
            secret: word(2),
            constraints: word(3),
            points,
            fixed,
            domain,
            name,
            metadata,
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

    /// The key's points as the file stores them, compressed: what `key`
    /// decompresses one vector at a time.
    pub fn points(&self) -> Points<'a> {
        Points::Compressed([self.a, self.b, self.k, self.z, self.b2])
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

    /// Writes the points decompressed into the kernel's encoding, a, b, k,
    /// z, b2, one vector at a time: a host's image of the key without the
    /// key loaded (little-endian devices, where the points' memory is that
    /// encoding).
    pub fn write_points(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        fn bytes<T>(points: &[T]) -> &[u8] {
            // SAFETY: plain points, read as their bytes.
            unsafe {
                std::slice::from_raw_parts(points.as_ptr().cast::<u8>(), std::mem::size_of_val(points))
            }
        }
        if !cfg!(target_endian = "little") {
            return Err(std::io::Error::other("a key image on a big-endian device"));
        }
        for vector in [self.a, self.b, self.k, self.z] {
            out.write_all(bytes(&decompress::g1_affine(vector).map_err(std::io::Error::other)?))?;
        }
        out.write_all(bytes(&decompress::g2_affine(self.b2).map_err(std::io::Error::other)?))
    }

    /// The kernel's solver image of the key's program (`Key::solver_image`),
    /// built without the key: a host's image of the key keeps it.
    pub fn solver_image(&self) -> Result<Vec<u8>, String> {
        let plan = crate::solver::Plan::from_deflated(self.program, &inflate(self.coefficients)?)?;
        let [a, b, k, z, _] = self.points;
        if !plan.matches(a as usize, b as usize, k as usize, z as usize + 1) {
            return Err("the key's program does not match its points".into());
        }
        Ok(plan.image())
    }

    /// Stream the solver image with a u64 byte length, without retaining a
    /// serialized copy of the complete plan.
    pub fn write_solver_image(&self, out: &mut impl std::io::Write) -> Result<(), String> {
        let plan = crate::solver::Plan::from_deflated(self.program, &inflate(self.coefficients)?)?;
        let [a, b, k, z, _] = self.points;
        if !plan.matches(a as usize, b as usize, k as usize, z as usize + 1) {
            return Err("the key's program does not match its points".into());
        }
        write_plan(&plan, out)
    }

    /// The kernel's key from `points` (`points()`, or a host's cache of
    /// them decompressed), its solver from the program, or from
    /// `solver_image` when a host cached the kernel's own
    /// (`Key::solver_image`) and the kernel still reads it.
    pub fn key(&self, points: Points, solver_image: Option<&[u8]>) -> Result<Key, String> {
        let owner = match &points {
            Points::Mapped(owner, _) => Some((*owner).clone()),
            _ => None,
        };
        let mut key = Key::build(points, self.domain)?;
        if let Some(image) = solver_image {
            if key.set_solver_image_with_owner(image, owner.as_ref()) {
                return Ok(key);
            }
        }
        let started = crate::time::Instant::now();
        let plan = crate::solver::Plan::from_deflated(self.program, &inflate(self.coefficients)?);
        crate::trace("solver-plan", started);
        if !key.set_solver_plan(plan) {
            return Err("the kernel does not solve the key's program".into());
        }
        Ok(key)
    }

    pub fn verifying_key(&self) -> Result<VerifyingKey, &'static str> {
        VerifyingKey::decode_raw(self.verifying_key)
    }

    /// The fixed points, checked against the verifying key's.
    pub fn fixed_points(&self) -> Result<FixedPoints, &'static str> {
        check_fixed(self.fixed, self.verifying_key)
    }
}

fn inflate(bytes: &[u8]) -> Result<Vec<u8>, &'static str> {
    miniz_oxide::inflate::decompress_to_vec(bytes).map_err(|_| "the key's program is damaged")
}

/// The fixed points in the kernel's encoding, when they are the verifying
/// key's: a prover that blinds with a delta other than the verifying key's
/// can make proofs that verify and reveal the witness.
fn check_fixed(fixed: &[u8], verifying_key: &[u8]) -> Result<FixedPoints, &'static str> {
    let fixed = FixedPoints::decode(fixed).ok_or("fixed points")?;
    if fixed != crate::verify::fixed_points(verifying_key)? {
        return Err("the fixed points are not the verifying key's");
    }
    Ok(fixed)
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
    let value = zopro_bn254::Fq::from_be_bytes_mod_order(raw);
    for (limb, word) in value.0.iter().zip(out.chunks_mut(8)) {
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
