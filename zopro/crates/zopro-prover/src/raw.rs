//! The raw form of a loaded key, which a host caches in place of the key
//! file so a later load skips point decompression and the solver's build:
//! a magic, the key file's head (everything before its vectors,
//! length-prefixed), the five vectors decompressed into the kernel's
//! encoding, and the kernel's solver image (length-prefixed). `FORMAT`
//! names the layout; a host keeps a raw form only beside the format that
//! wrote it.
use zopro_core::artifact::Artifact;

use crate::Prover;

pub const FORMAT: &str = "zopro-raw-2";
const MAGIC: &[u8; 16] = b"zopro-raw2\0\0\0\0\0\0";

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
        let size =
            24 + self.head.len() + self.vectors.iter().map(Vec::len).sum::<usize>() + image.len();
        let mut out = Vec::with_capacity(size);
        out.extend_from_slice(MAGIC);
        out.extend_from_slice(&(self.head.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.head);
        for vector in &self.vectors {
            out.extend_from_slice(vector);
        }
        out.extend_from_slice(&(image.len() as u32).to_le_bytes());
        out.extend_from_slice(&image);
        out
    }
}

impl Prover {
    /// From a key file, with the source of its raw form, or from a raw
    /// form, with none.
    pub fn load_cacheable(bytes: &[u8]) -> Result<(Self, Option<Source>), String> {
        if bytes.starts_with(MAGIC) {
            let (artifact, vectors, image) = parse(bytes).ok_or("a damaged raw key")?;
            return Ok((
                Self::from_artifact(&artifact, &vectors, Some(&image))?,
                None,
            ));
        }
        let (_, at) = Artifact::head(bytes)?;
        let artifact = Artifact::parse(bytes)?;
        let vectors = artifact.vectors()?;
        let prover = Self::from_artifact(&artifact, &vectors, None)?;
        let source = Source {
            head: bytes[..at].to_vec(),
            vectors,
        };
        Ok((prover, Some(source)))
    }
}

/// The raw form's key head, vectors and solver image, when it is whole.
fn parse(raw: &[u8]) -> Option<(Artifact<'_>, [Vec<u8>; 5], Vec<u8>)> {
    let mut reader = Reader { raw, at: 16 };
    let head_len = reader.length()?;
    let head = reader.take(head_len)?;
    let (artifact, end) = Artifact::head(head).ok()?;
    if end != head.len() {
        return None;
    }
    let [a, b, k, z, b2] = artifact.points.map(|n| n as usize);
    let vectors = [
        reader.take(a * 64)?.to_vec(),
        reader.take(b * 64)?.to_vec(),
        reader.take(k * 64)?.to_vec(),
        reader.take(z * 64)?.to_vec(),
        reader.take(b2 * 128)?.to_vec(),
    ];
    let image_len = reader.length()?;
    let image = reader.take(image_len)?.to_vec();
    (reader.at == raw.len()).then_some((artifact, vectors, image))
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
