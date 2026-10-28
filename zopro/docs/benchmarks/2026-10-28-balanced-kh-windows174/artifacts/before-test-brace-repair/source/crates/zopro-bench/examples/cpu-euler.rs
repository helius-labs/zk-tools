//! Offline CPU basis preparation. Run without arguments for usage.
#[cfg(all(target_os = "linux", target_endian = "little"))]
mod command {
    use rayon::prelude::*;
    use sha2::{Digest, Sha256};
    use std::{
        ffi::OsString,
        fs::{self, File, OpenOptions},
        io::{BufWriter, Seek, SeekFrom, Write},
        path::{Path, PathBuf},
        sync::Arc,
        time::Instant,
    };
    use zopro_bn254::{Fr, G1Affine, G1Projective};
    use zopro_prover::{zopro_core::artifact::Artifact, Prover};

    type Result<T> = std::result::Result<T, String>;
    const MAGIC: &[u8; 16] = b"zopro-eul-v1\0\0\0\0";
    const USAGE: &str = "usage: cpu-basis prepare RAW_IMAGE OUTPUT_IMAGE [TILE_POINTS]\n       cpu-basis cache RAW_IMAGE QUERY_CACHE [TILE_POINTS]\n       cpu-basis image RAW_IMAGE QUERY_CACHE OUTPUT_IMAGE [TILE_POINTS]\nTILE_POINTS defaults to 4096. Inputs must be trusted raw-3 or raw-4 images; outputs must not exist.";

    /// Only files successfully created by this invocation are removed on drop.
    struct Temporary {
        path: PathBuf,
        file: File,
    }

    impl Temporary {
        fn new(output: &Path, kind: &str) -> Result<Self> {
            let name = output.file_name().ok_or("output needs a file name")?;
            for counter in 0..1024 {
                let mut suffix = OsString::from(".");
                suffix.push(name);
                suffix.push(format!(".{kind}.{}.{}", std::process::id(), counter));
                let path = output.with_file_name(suffix);
                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(&path)
                {
                    Ok(file) => return Ok(Self { path, file }),
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(e) => return Err(format!("create {}: {e}", path.display())),
                }
            }
            Err("temporary file names are occupied".into())
        }

        fn publish(self, output: &Path) -> Result<()> {
            self.file.sync_all().map_err(|e| e.to_string())?;
            // Same-directory hard linking atomically refuses an existing output,
            // including one created after the initial existence check.
            fs::hard_link(&self.path, output)
                .map_err(|e| format!("publish {}: {e}", output.display()))
        }
    }

    impl Drop for Temporary {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn map(path: &Path) -> Result<memmap2::Mmap> {
        let file = File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
        // The host owns the trusted input and must not mutate it while mapped.
        unsafe { memmap2::Mmap::map(&file) }.map_err(|e| e.to_string())
    }

    fn field(h: &mut Sha256, value: Fr) {
        for word in value.0 {
            h.update(word.to_le_bytes());
        }
    }

    fn montgomery(bytes: &[u8]) -> Result<Fr> {
        let words = core::array::from_fn(|i| {
            u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap())
        });
        Fr::from_canonical(words).ok_or("domain field value outside modulus")?;
        Ok(Fr::from_montgomery(words))
    }

    fn point_bytes(points: &[G1Affine]) -> &[u8] {
        // G1Affine's kernel encoding is two four-limb fields, with no padding.
        unsafe { std::slice::from_raw_parts(points.as_ptr().cast(), points.len() * 64) }
    }

    fn point_fft(values: &mut [G1Projective], generator: Fr, tile: usize, label: &str) {
        let n = values.len();
        let inverse = generator.inverse().unwrap();
        for stage in 1..=n.trailing_zeros() {
            let started = Instant::now();
            let len = 1usize << stage;
            let step = inverse.pow(&[(n / len) as u64]);
            values.par_chunks_mut(len).for_each(|chunk| {
                let (lo, hi) = chunk.split_at_mut(len / 2);
                lo.par_chunks_mut(tile)
                    .zip(hi.par_chunks_mut(tile))
                    .enumerate()
                    .for_each(|(block, (lo, hi))| {
                        let first = block * tile;
                        let mut root = step.pow(&[first as u64]);
                        for (j, (lo, hi)) in lo.iter_mut().zip(hi).enumerate() {
                            let p = if first + j == 0 { *hi } else { *hi * root };
                            let a = *lo;
                            *lo = a + p;
                            *hi = a - p;
                            root *= step;
                        }
                    });
            });
            eprintln!(
                "{label} stage {stage}: {:.3}s",
                started.elapsed().as_secs_f64()
            );
        }
    }

    fn write_cache(raw: &[u8], output: &File, anchor: &Path, tile: usize) -> Result<usize> {
        let wide = raw.get(..16) == Some(b"zopro-raw4\0\0\0\0\0\0");
        if !wide && raw.get(..16) != Some(b"zopro-raw3\0\0\0\0\0\0") {
            return Err("cache preparation requires a raw image".into());
        }
        let length = |at: usize| -> Result<(usize, usize)> {
            let end = at.checked_add(if wide { 8 } else { 4 }).ok_or("raw length overflow")?;
            let bytes = raw.get(at..end).ok_or("truncated raw length")?;
            let n = if wide {
                usize::try_from(u64::from_le_bytes(bytes.try_into().unwrap()))
                    .map_err(|_| "raw length exceeds address space")?
            } else {
                u32::from_le_bytes(bytes.try_into().unwrap()) as usize
            };
            Ok((n, end))
        };
        let (head_len, head_at) = length(16)?;
        let head_end = head_at.checked_add(head_len).ok_or("raw header overflow")?;
        let head = raw.get(head_at..head_end).ok_or("truncated raw header")?;
        let (artifact, used) = Artifact::head(head)?;
        if used != head.len() || artifact.domain.len() != 64 {
            return Err("invalid raw header/domain length".into());
        }
        let counts = artifact.points.map(|v| v as usize);
        let n = counts[3].checked_add(1).ok_or("domain size overflow")?;
        if n < 8 || !n.is_power_of_two() {
            return Err("domain size must be a power of two of at least eight".into());
        }
        let generator = montgomery(&artifact.domain[..32])?;
        let coset = montgomery(&artifact.domain[32..])?;
        if generator.pow(&[n as u64]) != Fr::ONE || generator.pow(&[(n / 2) as u64]) == Fr::ONE {
            return Err("domain generator has the wrong order".into());
        }
        let inv_n = Fr::from(n as u64).inverse().ok_or("invalid domain size")?;
        let divisor = (coset.pow(&[n as u64]) - Fr::ONE)
            .inverse()
            .ok_or("invalid coset divisor")?;
        let c_weight = inv_n * divisor;
        let base = coset.inverse().ok_or("zero coset")?;
        let header_end = head_end
            .checked_next_multiple_of(64)
            .ok_or("header alignment overflow")?;
        if raw
            .get(head_end..header_end)
            .ok_or("truncated raw padding")?
            .iter()
            .any(|&b| b != 0)
        {
            return Err("nonzero raw padding".into());
        }
        let vectors_end = counts
            .iter()
            .enumerate()
            .try_fold(header_end, |at, (i, &count)| {
                at.checked_add(count.checked_mul(if i == 4 { 128 } else { 64 })?)
            })
            .ok_or("raw vectors overflow")?;
        let (solver_len, solver_at) = length(vectors_end)?;
        if solver_at.checked_add(solver_len) != Some(raw.len()) || counts[1] != counts[4] {
            return Err("invalid raw solver length or B query counts".into());
        }
        let preceding = counts[..3]
            .iter()
            .try_fold(0usize, |n, &v| n.checked_add(v))
            .ok_or("query count overflow")?;
        let offset = header_end
            .checked_add(preceding.checked_mul(64).ok_or("query size overflow")?)
            .ok_or("query offset overflow")?;
        let z_len = (n - 1).checked_mul(64).ok_or("Z size overflow")?;
        let z_bytes = raw
            .get(offset..offset.checked_add(z_len).ok_or("Z end overflow")?)
            .ok_or("truncated Z query")?;
        if z_bytes.as_ptr() as usize % std::mem::align_of::<G1Affine>() != 0 {
            return Err("unaligned raw query".into());
        }
        // Bounds/alignment are checked. Coordinates use the trusted native raw
        // encoding, as in the prover's mapped loader; no expanded point copy.
        let z = unsafe { std::slice::from_raw_parts(z_bytes.as_ptr().cast::<G1Affine>(), n - 1) };
        let fingerprint=zopro_prover::zopro_core::basis::euler_cache_fingerprint(z,n,generator)?;

        let scratch = Temporary::new(anchor, "scratch")?;
        let scratch_bytes = n
            .checked_mul(std::mem::size_of::<G1Projective>())
            .ok_or("scratch size overflow")?;
        scratch
            .file
            .set_len(scratch_bytes as u64)
            .map_err(|e| e.to_string())?;
        let mut map =
            unsafe { memmap2::MmapMut::map_mut(&scratch.file) }.map_err(|e| e.to_string())?;
        if map.as_ptr() as usize % std::mem::align_of::<G1Projective>() != 0 {
            return Err("unaligned scratch mapping".into());
        }
        let mut out = BufWriter::with_capacity(64 << 10, output);
        out.write_all(MAGIC).map_err(|e| e.to_string())?;
        out.write_all(&fingerprint).map_err(|e| e.to_string())?;
        out.write_all(&[0; 32]).map_err(|e| e.to_string())?;
        out.write_all(&(n as u64).to_le_bytes())
            .map_err(|e| e.to_string())?;
        let mut payload = Sha256::new();
        for label in ["product", "constraints"] {
            {
                // The exclusively owned file is zero-filled. Field storage and
                // its zero-sized markers accept zero bits; initialize every
                // projective point before the transform reads it.
                let values = unsafe {
                    std::slice::from_raw_parts_mut(map.as_mut_ptr().cast::<G1Projective>(), n)
                };
                values
                    .par_chunks_mut(tile)
                    .enumerate()
                    .for_each(|(block, chunk)| {
                        let first = block * tile;

                        for (j, value) in chunk.iter_mut().enumerate() {
                            let i = first + j;
                            let k=i.reverse_bits() >> (usize::BITS-n.trailing_zeros());
                            let weight=if label=="product"{inv_n*inv_n*inv_n}else{Fr::from(k as u64)*inv_n*inv_n};
                            *value=z.get(i).map_or(G1Projective::IDENTITY,|p|*p*weight);
                        }
                    });
                point_fft(values, generator, tile, label);
                for chunk in values.chunks(tile) {
                    let affine = G1Projective::batch_to_affine(chunk);
                    let data = point_bytes(&affine);
                    payload.update(data);
                    out.write_all(data).map_err(|e| e.to_string())?;
                }
            }
            map.flush().map_err(|e| e.to_string())?;
            // No typed references remain; the owned file has been flushed.
            unsafe { map.unchecked_advise(memmap2::UncheckedAdvice::DontNeed) }
                .map_err(|e| e.to_string())?;
        }
        out.seek(SeekFrom::Start(48)).map_err(|e| e.to_string())?;
        out.write_all(&payload.finalize())
            .map_err(|e| e.to_string())?;
        out.flush().map_err(|e| e.to_string())?;
        Ok(scratch_bytes)
    }

    fn write_image(raw: &Path, cache: &Path, output: &File, tile: usize) -> Result<()> {
        let raw: zopro_prover::zopro_core::Image = Arc::new(map(raw)?);
        let cache: zopro_prover::zopro_core::Image = Arc::new(map(cache)?);
        let mut out = BufWriter::with_capacity(64 << 10, output);
        Prover::write_cpu_basis_image(&raw, &cache, tile, &mut out)?;
        out.flush().map_err(|e| e.to_string())
    }

    pub fn run() -> Result<()> {
        let args: Vec<_> = std::env::args_os().collect();
        let mode = args.get(1).and_then(|s| s.to_str()).ok_or(USAGE)?;
        let paths = match mode {
            "prepare" | "cache" => 2,
            "image" => 3,
            _ => return Err(USAGE.into()),
        };
        if args.len() != paths + 2 && args.len() != paths + 3 {
            return Err(USAGE.into());
        }
        let tile = args
            .get(paths + 2)
            .map(|s| {
                s.to_str()
                    .ok_or("tile is not UTF-8")?
                    .parse::<usize>()
                    .map_err(|_| "invalid tile size")
            })
            .transpose()?
            .unwrap_or(4096);
        if !(1..=65536).contains(&tile) {
            return Err("tile size must be between 1 and 65536".into());
        }
        let input = Path::new(&args[2]);
        let output = Path::new(&args[paths + 1]);
        if fs::symlink_metadata(output).is_ok() {
            return Err(format!("output already exists: {}", output.display()));
        }
        let started = Instant::now();
        let temporary = Temporary::new(output, "output")?;
        let mut scratch_bytes = 0;
        match mode {
            "cache" => scratch_bytes = write_cache(&map(input)?, &temporary.file, output, tile)?,
            "image" => write_image(input, Path::new(&args[3]), &temporary.file, tile)?,
            "prepare" => {
                let cache = Temporary::new(output, "queries")?;
                scratch_bytes = write_cache(&map(input)?, &cache.file, output, tile)?;
                write_image(input, &cache.path, &temporary.file, tile)?;
            }
            _ => unreachable!(),
        }
        temporary.publish(output)?;
        let mut usage = unsafe { std::mem::zeroed::<libc::rusage>() };
        let rss = if unsafe { libc::getrusage(libc::RUSAGE_SELF, &mut usage) } == 0 {
            Some(usage.ru_maxrss)
        } else {
            None
        };
        println!(
            "{}",
            serde_json::json!({"operation":mode,"input":input,"output":output,"tile_points":tile,
            "scratch_file_bytes":scratch_bytes,"output_bytes":output.metadata().map_err(|e|e.to_string())?.len(),
            "seconds":started.elapsed().as_secs_f64(),"peak_rss_kib":rss})
        );
        Ok(())
    }
}

#[cfg(all(target_os = "linux", target_endian = "little"))]
#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =
    zopro_prover::zopro_core::alloc::Allocator;

fn main() -> std::process::ExitCode {
    #[cfg(all(target_os = "linux", target_endian = "little"))]
    let result = command::run();
    #[cfg(not(all(target_os = "linux", target_endian = "little")))]
    let result: Result<(), String> =
        Err("CPU basis preparation currently supports little-endian Linux hosts".into());
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            std::process::ExitCode::FAILURE
        }
    }
}
