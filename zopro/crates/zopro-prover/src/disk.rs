//! A native cache for a versioned key on disk.
use std::{
    fs::File,
    io::{BufWriter, Write},
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
    /// Load an immutable key, keeping its solver image and GPU pipelines beside it.
    /// If the image cannot be created or opened, load directly from the key file.
    ///
    /// # Safety
    ///
    /// The key and cache directory must be trusted. Nobody may modify or
    /// truncate a mapped file while it is mapped, including after this
    /// function returns. Publish updated files by atomic rename. Use a
    /// versioned key path or remove the old image when replacing a key.
    pub unsafe fn load_cached(key: &Path) -> Result<Self, String> {
        zopro_core::set_gpu_pipeline_cache(&key.with_file_name("zopro-gpu.cache"));
        let image = image_path(key);
        if image.is_file() {
            // SAFETY: the caller guarantees the mapped file is immutable.
            match unsafe { load_image(&image) } {
                Ok(prover) => return Ok(prover),
                Err(error) => log::warn!("zopro: {}: {error}; regenerating image", image.display()),
            }
        }
        // SAFETY: the caller guarantees the key file is immutable.
        let source = unsafe { map(key) }?;
        let cached = write_image(&source, &image).and_then(|()| {
            // SAFETY: the caller guarantees the mapped file is immutable.
            unsafe { load_image(&image) }
        });
        match cached {
            Ok(prover) => Ok(prover),
            Err(error) => {
                log::warn!(
                    "zopro: {}: {error}; loading the key directly",
                    image.display()
                );
                Self::load(&source)
            }
        }
    }
}

unsafe fn map(path: &Path) -> Result<Mmap, String> {
    let file = File::open(path).map_err(|error| error.to_string())?;
    // SAFETY: the caller guarantees the mapped file is immutable.
    unsafe { Mmap::map(&file) }.map_err(|error| error.to_string())
}

unsafe fn load_image(path: &Path) -> Result<Prover, String> {
    // SAFETY: the caller guarantees the mapped file is immutable.
    Prover::load_image(Arc::new(unsafe { map(path) }?))
}

fn write_image(key: &[u8], image: &Path) -> Result<(), String> {
    let parent = image
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let io = |error: std::io::Error| error.to_string();
    // Each writer gets its own file. No writer can truncate an image already mapped
    // by a reader, and a process that fails here leaves the previous image intact.
    let mut file = tempfile::NamedTempFile::new_in(parent).map_err(io)?;
    let mut out = BufWriter::new(file.as_file_mut());
    Prover::write_image(key, &mut out)?;
    out.flush().map_err(io)?;
    drop(out);
    file.as_file().sync_all().map_err(io)?;
    file.persist(image).map_err(|error| error.to_string())?;
    Ok(())
}
