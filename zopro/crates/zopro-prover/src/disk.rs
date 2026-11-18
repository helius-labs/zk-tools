//! Native loading with a versioned, disk-backed solver cache.
use std::{
    fs::{self, File},
    io::{self, BufWriter, Read, Seek, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use memmap2::Mmap;

use crate::{Prover, raw};

/// The cache image beside `key`, named for both the point and solver layouts.
pub fn image_path(key: &Path) -> PathBuf {
    let mut name = key.file_name().unwrap_or_default().to_os_string();
    name.push(format!(
        ".{}.solver{}",
        raw::FORMAT,
        zopro_core::solver_image_version()
    ));
    key.with_file_name(name)
}

impl Prover {
    /// Load a trusted key, reusing its on-disk solver image when possible.
    /// The prover's mapping is backed by its own anonymous file.
    pub fn load_cached(key: &Path) -> Result<Self, String> {
        load(key, Ok)
    }
}

/// Load using the cache, with protocol-specific validation on every path.
/// Rejected images are rebuilt from the source key.
pub fn load<T>(key: &Path, validate: impl Fn(Prover) -> Result<T, String>) -> Result<T, String> {
    let parent = key
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut source = regular_file(key).map_err(io_error)?;
    zopro_core::set_gpu_pipeline_cache(&key.with_file_name("zopro-gpu.cache"));
    let image = image_path(key);
    match regular_file(&image) {
        Ok(mut cached) => {
            let hit = (|| {
                let snapshot = snapshot(&mut cached, parent).map_err(io_error)?;
                let mapped = map_owned(&snapshot).map_err(io_error)?;
                let header = raw::source_header(&mapped).ok_or("invalid solver image header")?;
                match_header(&mut source, header).map_err(io_error)?;
                validate(Prover::load_image(Arc::new(mapped))?)
            })();
            match hit {
                Ok(prover) => return Ok(prover),
                Err(error) => log::warn!("zopro: {}: {error}; regenerating image", image.display()),
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => log::warn!("zopro: {}: {error}", image.display()),
    }

    let rebuilt = (|| {
        let mut snapshot = anonymous_file(parent).map_err(io_error)?;
        {
            let bytes = key_bytes(&mut source, key, parent)?;
            let mut out = BufWriter::new(&mut snapshot);
            Prover::write_image(bytes.as_ref(), &mut out)?;
            out.flush().map_err(io_error)?;
        }
        let mapped = map_owned(&snapshot).map_err(io_error)?;
        let prover = validate(Prover::load_image(Arc::new(mapped))?)?;
        if let Err(error) = publish(&mut snapshot, &image, parent) {
            log::warn!(
                "zopro: {}: {error}; using the private image",
                image.display()
            );
        }
        Ok::<_, String>(prover)
    })();
    match rebuilt {
        Ok(prover) => Ok(prover),
        Err(error) => {
            log::warn!(
                "zopro: {}: {error}; loading the key directly",
                image.display()
            );
            validate(Prover::load(key_bytes(&mut source, key, parent)?.as_ref())?)
        }
    }
}

enum KeyBytes {
    Mapped(Mmap),
    Owned(Vec<u8>),
}

impl AsRef<[u8]> for KeyBytes {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Mapped(bytes) => bytes,
            Self::Owned(bytes) => bytes,
        }
    }
}

fn io_error(error: io::Error) -> String {
    error.to_string()
}

fn regular_file(path: &Path) -> io::Result<File> {
    let invalid = || io::Error::new(io::ErrorKind::InvalidInput, "expected a regular key file");
    if !fs::metadata(path)?.is_file() {
        return Err(invalid());
    }
    let file = File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(invalid());
    }
    Ok(file)
}

fn anonymous_file(parent: &Path) -> io::Result<File> {
    // Android's process temp directory is not always writable. The key's
    // directory is normally available there. tempfile unlinks before returning.
    tempfile::tempfile().or_else(|_| tempfile::tempfile_in(parent))
}

fn copy_file(source: &mut File, destination: &mut impl Write) -> io::Result<()> {
    let len = source.metadata()?.len();
    if io::copy(&mut source.take(len), destination)? != len {
        return Err(io::ErrorKind::UnexpectedEof.into());
    }
    Ok(())
}

fn snapshot(source: &mut File, parent: &Path) -> io::Result<File> {
    let mut copy = anonymous_file(parent)?;
    copy_file(source, &mut copy)?;
    Ok(copy)
}

fn key_bytes(source: &mut File, key: &Path, parent: &Path) -> Result<KeyBytes, String> {
    source.rewind().map_err(io_error)?;
    match snapshot(source, parent).and_then(|file| map_owned(&file)) {
        Ok(mapped) => Ok(KeyBytes::Mapped(mapped)),
        Err(error) => {
            log::warn!("zopro: {}: {error}; reading key into memory", key.display());
            let mut bytes = Vec::new();
            source.rewind().map_err(io_error)?;
            copy_file(source, &mut bytes).map_err(io_error)?;
            Ok(KeyBytes::Owned(bytes))
        }
    }
}

fn map_owned(file: &File) -> io::Result<Mmap> {
    // SAFETY: only freshly written anonymous files from this module are mapped.
    // The module never exposes their descriptors or changes their contents
    // after mapping. The original key and on-disk cache can be replaced or
    // truncated independently of the mapping.
    unsafe { Mmap::map(file) }
}

fn match_header(source: &mut File, header: &[u8]) -> io::Result<()> {
    let mut buffer = [0; 8192];
    for chunk in header.chunks(buffer.len()) {
        source.read_exact(&mut buffer[..chunk.len()])?;
        if chunk != &buffer[..chunk.len()] {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "cached image belongs to a different key",
            ));
        }
    }
    Ok(())
}

fn publish(source: &mut File, image: &Path, parent: &Path) -> io::Result<()> {
    source.rewind()?;
    let mut staged = tempfile::NamedTempFile::new_in(parent)?;
    io::copy(source, staged.as_file_mut())?;
    staged.as_file().sync_all()?;
    staged.persist(image)?;
    Ok(())
}
