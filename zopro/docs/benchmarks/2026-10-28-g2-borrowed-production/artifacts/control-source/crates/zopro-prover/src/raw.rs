//! The raw form of a loaded key, which a host caches in place of the key
//! file so a later load skips point decompression and the solver's build:
//! a magic, the key file's head (without the source program and coefficients
//! already represented in the solver image,
//! length-prefixed), zeros to the next multiple of 64 bytes, the five
//! vectors decompressed into the kernel's encoding, and the kernel's solver
//! image (length-prefixed). Format 4 uses u64 byte lengths for both the head
//! and solver; format 3's u32 lengths remain readable. The vectors' sizes are multiples of 64 bytes, so
//! in a mapped raw form each starts aligned and the key reads its points in
//! place (`Prover::load_image`). `FORMAT` names the layout; a host keeps a
//! raw form only beside the format that wrote it.
use std::io::{self, Write};

use zopro_core::{artifact::Artifact, Image, Points};

use crate::Prover;

pub const FORMAT: &str = "zopro-raw-4";
const MAGIC: &[u8; 16] = b"zopro-raw4\0\0\0\0\0\0";
const LEGACY_MAGIC: &[u8; 16] = b"zopro-raw3\0\0\0\0\0\0";
const ALIGN: usize = 64;

fn is_image(bytes: &[u8]) -> bool {
    bytes.starts_with(MAGIC) || bytes.starts_with(LEGACY_MAGIC)
}

/// What a raw form needs from a load of the key file: its head and the
/// decompressed vectors.
pub struct Source {
    head: Vec<u8>,
    vectors: [Vec<u8>; 5],
}

impl Source {
    /// The raw form of `prover`, loaded from this source.
    pub fn export(self, prover: &Prover) -> Vec<u8> {
        let image = prover.solver_image().unwrap_or_default();
        let [a, b, k, z, b2] = &self.vectors;
        let mut out = Vec::new();
        write(&mut out, &self.head, [a, b, k, z, b2], &image).expect("a vector takes every write");
        out
    }
}

impl Prover {
    /// From a key file, with the source of its raw form, or from a raw
    /// form, with none.
    pub fn load_cacheable(bytes: &[u8]) -> Result<(Self, Option<Source>), String> {
        if is_image(bytes) {
            let (artifact, ranges, image) = parse(bytes).ok_or("a damaged raw key")?;
            let vectors = ranges.map(|(at, len)| &bytes[at..at + len]);
            return Ok((
                Self::from_artifact(&artifact, Points::Kernel(vectors), Some(image))?,
                None,
            ));
        }
        let (_, at) = Artifact::head(bytes)?;
        let artifact = Artifact::parse(bytes)?;
        let vectors = artifact.vectors()?;
        let [a, b, k, z, b2] = &vectors;
        let prover = Self::from_artifact(&artifact, Points::Kernel([a, b, k, z, b2]), None)?;
        let source = Source {
            head: bytes[..at].to_vec(),
            vectors,
        };
        Ok((prover, Some(source)))
    }

    /// From a raw or CPU replacement image a host mapped: the key
    /// reads its points in place, so the system may drop their pages under
    /// memory pressure and read them again rather than the process holding
    /// them, and the load decompresses nothing.
    ///
    /// A host writes the image once, from the key file, renames it into
    /// place and maps it; a damaged or unsupported image fails here, and the host
    /// writes it again.
    pub fn load_image(image: Image) -> Result<Self, String> {
        let bytes = (*image).as_ref();
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        if cpu_basis::is_image(bytes) {
            return Self::load_cpu_basis_image(image);
        }
        if !is_image(bytes) {
            return Err("not a raw zopro key".into());
        }
        let (artifact, ranges, solver) = parse(bytes).ok_or("a damaged raw key")?;
        Self::from_artifact(&artifact, Points::Mapped(&image, ranges), Some(solver))
    }

    /// Writes the raw form of the key file `key` to `out` without loading
    /// the key: each vector decompressed and written in turn, then the
    /// solver's image built and written, so it holds one of them at a time
    /// (a load holds them all: at 16 -> 2 on a phone, ~300 MB against ~500).
    pub fn write_image(key: &[u8], out: &mut impl Write) -> Result<(), String> {
        let (_, at) = Artifact::head(key)?;
        let artifact = Artifact::parse(key)?;
        let io = |e: io::Error| e.to_string();
        write_head(out, &key[..at]).map_err(io)?;
        artifact.write_points(out).map_err(io)?;
        artifact.write_solver_image(out)
    }

    /// Write a raw image directly from dump parts. Callers may map the
    /// vectors and program; this avoids an intermediate compressed point
    /// image and streams the serialized solver instead of copying it.
    pub fn write_dump_image(parts: &zopro_core::artifact::Parts<'_>, out: &mut impl Write) -> Result<(), String> {
        let head = parts.image_head()?;
        write_head(out, &head).map_err(|e| e.to_string())?;
        drop(head);
        for vector in [parts.a, parts.b, parts.k, parts.z, parts.b2] {
            out.write_all(vector).map_err(|e| e.to_string())?;
        }
        parts.write_solver_image(out)
    }
}

fn write(out: &mut impl Write, head: &[u8], vectors: [&[u8]; 5], image: &[u8]) -> io::Result<()> {
    write_head(out, head)?;
    for vector in vectors {
        out.write_all(vector)?;
    }
    write_tail(out, image)
}

/// The magic, the key's head and the padding to the vectors.
fn write_head(out: &mut impl Write, head: &[u8]) -> io::Result<()> {
    let (artifact, end) = Artifact::head(head).map_err(io::Error::other)?;
    if end != head.len() {
        return Err(io::Error::other("trailing raw header data"));
    }
    write_head_with_magic(out, &artifact.image_head(), MAGIC)
}

fn write_head_with_magic(out: &mut impl Write, head: &[u8], magic: &[u8; 16]) -> io::Result<()> {
    out.write_all(magic)?;
    out.write_all(&(head.len() as u64).to_le_bytes())?;
    out.write_all(head)?;
    out.write_all(&[0; ALIGN][..padding(magic.len() + 8 + head.len())])
}

/// The solver image after the vectors.
fn write_tail(out: &mut impl Write, image: &[u8]) -> io::Result<()> {
    out.write_all(&(image.len() as u64).to_le_bytes())?;
    out.write_all(image)
}

/// Zeros from `at` to the next multiple of ALIGN.
fn padding(at: usize) -> usize {
    (ALIGN - at % ALIGN) % ALIGN
}

/// The raw form's key head, its vectors' byte ranges and its solver image,
/// when it is whole.
fn parse(raw: &[u8]) -> Option<(Artifact<'_>, [(usize, usize); 5], &[u8])> {
    if !is_image(raw) {
        return None;
    }
    let wide = raw.starts_with(MAGIC);
    let mut reader = Reader { raw, at: 16 };
    let head_len = reader.length(wide)?;
    let head = reader.take(head_len)?;
    let (artifact, end) = Artifact::head(head).ok()?;
    if end != head.len() {
        return None;
    }
    let pad = reader.take(padding(reader.at))?;
    if pad.iter().any(|&b| b != 0) {
        return None;
    }
    let mut ranges = [(0, 0); 5];
    for (i, &count) in artifact.points.iter().enumerate() {
        let len = (count as usize).checked_mul(if i == 4 { 128 } else { 64 })?;
        let at = reader.at;
        reader.take(len)?;
        ranges[i] = (at, len);
    }
    let image_len = reader.length(wide)?;
    let image = reader.take(image_len)?;
    (reader.at == raw.len()).then_some((artifact, ranges, image))
}

struct Reader<'a> {
    raw: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let out = self.raw.get(self.at..self.at.checked_add(len)?)?;
        self.at += len;
        Some(out)
    }

    fn length(&mut self, wide: bool) -> Option<usize> {
        if wide {
            usize::try_from(u64::from_le_bytes(self.take(8)?.try_into().ok()?)).ok()
        } else {
            Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?) as usize)
        }
    }
}

#[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
#[path = "cpu_basis.rs"]
mod cpu_basis;
#[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
pub use cpu_basis::FORMAT as CPU_BASIS_FORMAT;

#[cfg(test)]
#[path = "raw_tests.rs"]
mod tests;
