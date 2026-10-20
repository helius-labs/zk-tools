//! The raw form of a loaded key, which a host caches in place of the key
//! file so a later load skips point decompression and the solver's build:
//! a magic, the key file's head (everything before its vectors,
//! length-prefixed), zeros to the next multiple of 64 bytes, the five
//! vectors decompressed into the kernel's encoding, and the kernel's solver
//! image (length-prefixed). The vectors' sizes are multiples of 64 bytes, so
//! in a mapped raw form each starts aligned and the key reads its points in
//! place (`Prover::load_image`). `FORMAT` names the layout; a host keeps a
//! raw form only beside the format that wrote it.
use std::io::{self, Write};

use zopro_core::{artifact::Artifact, Image, Points};

use crate::Prover;

pub const FORMAT: &str = "zopro-raw-3";
const MAGIC: &[u8; 16] = b"zopro-raw3\0\0\0\0\0\0";
const ALIGN: usize = 64;

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
        if bytes.starts_with(MAGIC) {
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

    /// From a raw form a host mapped (`write_image` wrote it): the key
    /// reads its points in place, so the system may drop their pages under
    /// memory pressure and read them again rather than the process holding
    /// them, and the load decompresses nothing.
    ///
    /// A host writes the image once, from the key file, renames it into
    /// place and maps it; a damaged or older image fails here, and the host
    /// writes it again.
    pub fn load_image(image: Image) -> Result<Self, String> {
        let bytes = (*image).as_ref();
        if !bytes.starts_with(MAGIC) {
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
        let image = artifact.solver_image()?;
        write_tail(out, &image).map_err(io)
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
    out.write_all(MAGIC)?;
    out.write_all(&(head.len() as u32).to_le_bytes())?;
    out.write_all(head)?;
    out.write_all(&[0; ALIGN][..padding(MAGIC.len() + 4 + head.len())])
}

/// The solver image after the vectors.
fn write_tail(out: &mut impl Write, image: &[u8]) -> io::Result<()> {
    out.write_all(&(image.len() as u32).to_le_bytes())?;
    out.write_all(image)
}

/// Zeros from `at` to the next multiple of ALIGN.
fn padding(at: usize) -> usize {
    at.next_multiple_of(ALIGN) - at
}

/// The raw form's key head, its vectors' byte ranges and its solver image,
/// when it is whole.
fn parse(raw: &[u8]) -> Option<(Artifact<'_>, [(usize, usize); 5], &[u8])> {
    let mut reader = Reader { raw, at: 16 };
    let head_len = reader.length()?;
    let head = reader.take(head_len)?;
    let (artifact, end) = Artifact::head(head).ok()?;
    if end != head.len() {
        return None;
    }
    let pad = reader.take(padding(reader.at))?;
    if pad.iter().any(|&b| b != 0) {
        return None;
    }
    let [a, b, k, z, b2] = artifact.points.map(|n| n as usize);
    let ranges = [a * 64, b * 64, k * 64, z * 64, b2 * 128].map(|len| {
        let at = reader.at;
        reader.take(len).map(|_| (at, len))
    });
    let ranges = [ranges[0]?, ranges[1]?, ranges[2]?, ranges[3]?, ranges[4]?];
    let image_len = reader.length()?;
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

    fn length(&mut self) -> Option<usize> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?) as usize)
    }
}
