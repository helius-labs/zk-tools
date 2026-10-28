//! G1 and G2 MSMs through wgpu: one WGSL kernel set for Vulkan (Android), WebGPU
//! (the browser) and Metal. As zopro-metal: the host recodes the scalars
//! into signed digits and counting-sorts the pairs into bucket runs; the
//! GPU sums each run in shards, level by level, reduces each window in two
//! stages, and the host combines the windows. Native hosts wait on the
//! device; the browser's asynchronous path is to come.
use crate::time::Instant;

/// The clock: std's natively, the page's in a browser, where std has none.
pub(crate) mod time {
    #[cfg(not(target_arch = "wasm32"))]
    pub use std::time::Instant;
    #[cfg(target_arch = "wasm32")]
    pub use web_time::Instant;
}

use zopro_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use rayon::prelude::*;

pub mod quotient;
pub use quotient::Quotient;

const G1_SHADER: &str = concat!(
    include_str!("../shaders/field.wgsl"),
    "\n",
    include_str!("../shaders/xyzz.wgsl"),
    "\n",
    include_str!("../shaders/buckets.wgsl"),
);
const G2_SHADER: &str = concat!(
    include_str!("../shaders/field.wgsl"),
    "\n",
    include_str!("../shaders/fp2.wgsl"),
    "\n",
    include_str!("../shaders/xyzz2.wgsl"),
    "\n",
    include_str!("../shaders/buckets2.wgsl"),
);

/// A curve the shaders have kernels for, with its point encodings.
pub trait Curve:
    'static
    + Copy
    + Send
    + Sync
    + PartialEq
    + std::fmt::Debug
    + std::ops::Add<Output = Self>
    + std::ops::AddAssign
{
    type Affine: Copy + Send + Sync;
    const IDENTITY: Self;
    fn double(&self) -> Self;
    const AFFINE_BYTES: usize;
    const XYZZ_BYTES: usize;
    const SHADER: &'static str;
    /// The WGSL module's name, the key it is cached by.
    const LABEL: &'static str;
    /// shards_first, shards_tree, reduce_blocks.
    const ENTRIES: [&'static str; 3];
    /// The same three kernels pre-compiled for Vulkan (glsl/), when written.
    const SPIRV: Option<[&'static [u8]; 3]>;
    /// Whether the pre-compiled reduction writes each thread's (V, S) pair
    /// for the host to weigh, instead of reducing them on the GPU.
    const SPIRV_PAIRS: bool;
    fn encode(point: &Self::Affine, out: &mut Vec<u8>);
    fn xyzz_to_projective(bytes: &[u8]) -> Self;
}

fn fq_bytes(v: &Fq, out: &mut Vec<u8>) {
    for w in v.0 {
        out.extend_from_slice(&w.to_le_bytes());
    }
}

impl Curve for G1Projective {
    type Affine = G1Affine;
    const IDENTITY: Self = Self::IDENTITY;
    fn double(&self) -> Self {
        G1Projective::double(self)
    }
    const AFFINE_BYTES: usize = 64;
    const XYZZ_BYTES: usize = 128;
    const SHADER: &'static str = G1_SHADER;
    const LABEL: &'static str = G1_LABEL;
    const ENTRIES: [&'static str; 3] = ["shards_first", "shards_tree", "reduce_blocks"];
    const SPIRV: Option<[&'static [u8]; 3]> = Some([
        include_bytes!("../glsl/out/shards_first.spv"),
        include_bytes!("../glsl/out/shards_tree.spv"),
        include_bytes!("../glsl/out/reduce_blocks.spv"),
    ]);
    const SPIRV_PAIRS: bool = false;
    fn encode(point: &G1Affine, out: &mut Vec<u8>) {
        fq_bytes(&point.x, out);
        fq_bytes(&point.y, out);
    }
    fn xyzz_to_projective(bytes: &[u8]) -> Self {
        let (x, y, zz, zzz) = (
            fq(&bytes[..32]),
            fq(&bytes[32..64]),
            fq(&bytes[64..96]),
            fq(&bytes[96..128]),
        );
        if zz.is_zero() {
            return Self::IDENTITY;
        }
        Self::new(x * zz, y * zzz, zz)
    }
}

impl Curve for G2Projective {
    type Affine = G2Affine;
    const IDENTITY: Self = Self::IDENTITY;
    fn double(&self) -> Self {
        G2Projective::double(self)
    }
    const AFFINE_BYTES: usize = 128;
    const XYZZ_BYTES: usize = 256;
    const SHADER: &'static str = G2_SHADER;
    const LABEL: &'static str = G2_LABEL;
    const ENTRIES: [&'static str; 3] = ["shards_first_g2", "shards_tree_g2", "reduce_blocks_g2"];
    const SPIRV: Option<[&'static [u8]; 3]> = Some([
        include_bytes!("../glsl/out/shards_first_g2.spv"),
        include_bytes!("../glsl/out/shards_tree_g2.spv"),
        include_bytes!("../glsl/out/reduce_pairs_g2.spv"),
    ]);
    const SPIRV_PAIRS: bool = true;
    fn encode(point: &G2Affine, out: &mut Vec<u8>) {
        for v in [point.x.c0, point.x.c1, point.y.c0, point.y.c1] {
            fq_bytes(&v, out);
        }
    }
    fn xyzz_to_projective(bytes: &[u8]) -> Self {
        let fq2 = |at: usize| Fq2::new(fq(&bytes[at..at + 32]), fq(&bytes[at + 32..at + 64]));
        let (x, y, zz, zzz) = (fq2(0), fq2(64), fq2(128), fq2(192));
        if zz.is_zero() {
            return Self::IDENTITY;
        }
        Self::new(x * zz, y * zzz, zz)
    }
}

pub type G1Msm = Msm<G1Projective>;
pub type G2Msm = Msm<G2Projective>;
const WORKGROUP: u32 = 64;
const FIRST_SHARD: usize = 32;
/// The first level's shard length: shorter with the pre-compiled kernels,
/// where more, shorter shards ran the OnePlus 13's accumulation 13 % faster
/// overall (ZOPRO_FIRST_SHARD overrides, for kernel experiments).
fn first_shard(spirv: bool) -> usize {
    std::env::var("ZOPRO_FIRST_SHARD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(if spirv {
            SPIRV_FIRST_SHARD
        } else {
            FIRST_SHARD
        })
}
const SPIRV_FIRST_SHARD: usize = 16;
/// Invocations per workgroup the first level's kernel declares
/// (ZOPRO_FIRST_GROUP, with a kernel from ZOPRO_SPV_DIR).
fn first_group() -> u32 {
    std::env::var("ZOPRO_FIRST_GROUP")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(WORKGROUP)
}
const SHARD: usize = 32;
/// The tree levels' shard length (ZOPRO_SHARD overrides, for kernel
/// experiments).
fn shard() -> usize {
    std::env::var("ZOPRO_SHARD")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(SHARD)
}
/// Threads per window in the reduction, measured best (with 256 and 128
/// close); ZOPRO_REDUCE_PER_WINDOW overrides natively.
const REDUCE_THREADS_PER_WINDOW: usize = 512;
/// Threads per window when the host weighs the reduction's pairs.
const PAIRS_PER_WINDOW: usize = 128;

fn width_for(n: usize) -> usize {
    let width = match n {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        _ => 15,
    };
    // As the Metal driver's: capped at 13 bits (ZOPRO_GPU_WIDTH overrides).
    let cap = std::env::var("ZOPRO_GPU_WIDTH")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(13)
        .clamp(7, 15);
    width.min(cap)
}

/// Stage times of one MSM, in milliseconds.
#[derive(Debug, Clone, Copy, Default)]
pub struct Timings {
    pub sort_ms: f64,
    pub gpu_ms: f64,
    pub combine_ms: f64,
}

/// A submitted MSM: finish it for its result.
pub struct Pending {
    submitted: Instant,
    timings: Timings,
    submission: wgpu::SubmissionIndex,
    timer: Option<profile::Timer>,
}

/// Per-kernel GPU times from timestamp queries, where the device has them
/// and profiling is on before the device is made (`set_profiling`, or
/// ZOPRO_WGPU_PROFILE=1 natively): every pass of an MSM or a quotient
/// writes its beginning and end, and `profile_report` gives the sums by
/// kernel since the last report. A browser rounds timestamps to 0.1 ms.
pub mod profile {
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    };

    static ON: AtomicBool = AtomicBool::new(false);
    static TIMES: Mutex<Vec<(&'static str, f64, u32)>> = Mutex::new(Vec::new());

    pub fn set_profiling(on: bool) {
        ON.store(on, Ordering::SeqCst);
    }

    pub(crate) fn wanted() -> bool {
        ON.load(Ordering::SeqCst)
            || (cfg!(not(target_arch = "wasm32"))
                && std::env::var("ZOPRO_WGPU_PROFILE").is_ok_and(|v| v == "1"))
    }

    pub(crate) fn record(kernel: &'static str, ms: f64) {
        let mut times = TIMES.lock().unwrap();
        match times.iter_mut().find(|(name, ..)| *name == kernel) {
            Some((_, total, count)) => {
                *total += ms;
                *count += 1;
            }
            None => times.push((kernel, ms, 1)),
        }
    }

    /// A host stage's time, when profiling.
    pub fn record_cpu(stage: &'static str, ms: f64) {
        if wanted() {
            record(stage, ms);
        }
    }

    /// `kernel total_ms passes` per line, and clears them.
    pub fn report() -> String {
        let mut times = TIMES.lock().unwrap();
        let out = times
            .iter()
            .map(|(name, total, count)| format!("{name} {total:.2} ms {count} passes"))
            .collect::<Vec<_>>()
            .join("\n");
        times.clear();
        out
    }

    /// The passes of one submission: their names, the query set they write,
    /// and the buffers it resolves into.
    pub(crate) struct Timer {
        set: wgpu::QuerySet,
        resolve: wgpu::Buffer,
        pub(crate) staging: wgpu::Buffer,
        names: Vec<&'static str>,
        period: f32,
    }

    /// One timer per submission: destroyed rather than left to a browser's
    /// collector (five 54 → 1 merges in a row ran Chrome's GPU out of
    /// memory).
    impl Drop for Timer {
        fn drop(&mut self) {
            self.set.destroy();
            self.resolve.destroy();
            self.staging.destroy();
        }
    }

    impl Timer {
        /// Room for `passes` passes, when profiling and the device times.
        pub(crate) fn new(gpu: &crate::Gpu, passes: usize) -> Option<Self> {
            let period = gpu.timestamp_period?;
            let count = (2 * passes) as u32;
            let device = &gpu.device;
            Some(Self {
                set: device.create_query_set(&wgpu::QuerySetDescriptor {
                    label: Some("zopro profile"),
                    ty: wgpu::QueryType::Timestamp,
                    count,
                }),
                resolve: device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: 8 * count as u64,
                    usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                }),
                staging: device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: 8 * count as u64,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                names: Vec::with_capacity(passes),
                period,
            })
        }

        /// A pass's descriptor, its times written as `kernel`'s.
        pub(crate) fn pass(&mut self, kernel: &'static str) -> wgpu::ComputePassDescriptor<'_> {
            let i = self.names.len() as u32;
            self.names.push(kernel);
            wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                    query_set: &self.set,
                    beginning_of_pass_write_index: Some(2 * i),
                    end_of_pass_write_index: Some(2 * i + 1),
                }),
            }
        }

        /// Copies the times out after the passes.
        pub(crate) fn resolve(&self, encoder: &mut wgpu::CommandEncoder) {
            let count = 2 * self.names.len() as u32;
            encoder.resolve_query_set(&self.set, 0..count, &self.resolve, 0);
            encoder.copy_buffer_to_buffer(&self.resolve, 0, &self.staging, 0, 8 * count as u64);
        }

        /// Records the times once the submission is done.
        pub(crate) async fn read(
            self,
            gpu: &crate::Gpu,
            submission: wgpu::SubmissionIndex,
        ) -> Result<(), String> {
            crate::mapped(gpu, &self.staging, Some(submission)).await?;
            let data = self
                .staging
                .slice(..)
                .get_mapped_range()
                .map_err(|e| format!("timestamps: {e}"))?
                .to_vec();
            self.staging.unmap();
            let ticks: Vec<u64> = data
                .chunks_exact(8)
                .map(|c| u64::from_le_bytes(c.try_into().unwrap()))
                .collect();
            for (i, name) in self.names.iter().enumerate() {
                let ns = ticks[2 * i + 1].saturating_sub(ticks[2 * i]) as f64 * self.period as f64;
                record(name, ns / 1e6);
            }
            Ok(())
        }
    }
}

/// Where the driver's pipeline cache is kept between processes, for the
/// devices made after this; else ZOPRO_PIPELINE_CACHE. An Adreno 830
/// compiles the kernels for 26 s the first time and loads them from the
/// cache in half a second after; without one every process compiles.
static PIPELINE_CACHE: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();

pub fn set_pipeline_cache(path: std::path::PathBuf) {
    let _ = PIPELINE_CACHE.set(path);
}

/// Qualcomm's PCI vendor id, which wgpu reports for an Adreno.
pub const QUALCOMM: u32 = 0x5143;

pub struct Gpu {
    /// Whether the MSM kernels come pre-compiled as SPIR-V (glsl/): on
    /// Vulkan with passthrough, by default on Android (ZOPRO_SPIRV=0|1
    /// overrides). Adreno's compiler loses the device on naga's SPIR-V of
    /// the WGSL kernels and links glslc's of the GLSL ones.
    pub spirv: bool,
    /// The adapter's PCI vendor id (Qualcomm 0x5143, ARM 0x13b5, ...).
    pub vendor: u32,
    /// Nanoseconds per timestamp tick, when profiling (`profile`).
    pub(crate) timestamp_period: Option<f32>,
    /// The driver's pipeline cache, persisted at ZOPRO_PIPELINE_CACHE when
    /// set: a phone compiles a kernel in tens of seconds the first time.
    cache: Option<wgpu::PipelineCache>,
    cache_path: Option<std::path::PathBuf>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub name: String,
    /// Set by the device's uncaptured-error handler: wgpu would otherwise
    /// panic, and a driver that refuses a kernel (a Galaxy S25's did) must
    /// only cost the GPU.
    failed: std::sync::Arc<std::sync::OnceLock<String>>,
    /// The kernels built on this device, one per entry point, and the WGSL
    /// modules they come from: a key's four G1 MSMs, its G2 MSM and its
    /// quotient share them rather than each compiling its own (a page load
    /// in Chrome compiled the G1 module four times, 10 s on an M1 VM), and
    /// a browser builds them at init_gpu, before the key downloads.
    kernels: std::sync::Mutex<std::collections::HashMap<String, wgpu::ComputePipeline>>,
    modules: std::sync::Mutex<std::collections::HashMap<&'static str, wgpu::ShaderModule>>,
}

/// An error scope's verdict: waited for natively; on the web, where nothing
/// may block, left to the device's uncaptured-error flag.
pub(crate) fn popped(scope: wgpu::ErrorScopeGuard) -> Option<wgpu::Error> {
    #[cfg(not(target_arch = "wasm32"))]
    return pollster::block_on(scope.pop());
    #[cfg(target_arch = "wasm32")]
    {
        drop(scope);
        None
    }
}

/// Waits for a buffer's read mapping: natively by polling the device, on
/// the web by yielding to the page until the browser maps it.
/// Maps a staging buffer once its submission is done: natively, waiting for
/// that submission only (later ones may run on), or for the whole queue.
pub(crate) async fn mapped(
    gpu: &Gpu,
    buffer: &wgpu::Buffer,
    submission: Option<wgpu::SubmissionIndex>,
) -> Result<(), String> {
    let (tx, rx) = futures_channel::oneshot::channel();
    buffer
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            let _ = tx.send(result);
        });
    #[cfg(not(target_arch = "wasm32"))]
    gpu.device
        .poll(wgpu::PollType::Wait {
            submission_index: submission,
            timeout: None,
        })
        .map_err(|e| format!("gpu: {e}"))?;
    #[cfg(target_arch = "wasm32")]
    let _ = submission;
    rx.await
        .map_err(|_| "the mapping was dropped".to_string())?
        .map_err(|e| format!("map: {e}"))?;
    if let Some(failure) = gpu.failure() {
        buffer.unmap();
        return Err(format!("the device reported an error: {failure}"));
    }
    Ok(())
}

impl Gpu {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Result<Self, String> {
        pollster::block_on(Self::request())
    }

    /// The high-performance adapter's device with all its limits (a wide
    /// key's buffers pass WebGPU's defaults).
    pub async fn request() -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .map_err(|e| format!("no adapter: {e}"))?;
        let info = adapter.get_info();
        // On Android only an Adreno, where the kernels beat the cores (a
        // Seeker's Mali-G615 runs them 3-4 times slower than its cores after
        // minutes of compiling); ZOPRO_GPU=1 takes any GPU. Decided before
        // the device is made, which costs a phone half a second.
        if cfg!(target_os = "android")
            && info.vendor != QUALCOMM
            && std::env::var("ZOPRO_GPU").as_deref() != Ok("1")
        {
            return Err(format!("{} is not an Adreno", info.name));
        }
        let passthrough = info.backend == wgpu::Backend::Vulkan
            && adapter
                .features()
                .contains(wgpu::Features::PASSTHROUGH_SHADERS);
        let spirv = passthrough
            && match std::env::var("ZOPRO_SPIRV").as_deref() {
                Ok("1") => true,
                Ok(_) => false,
                Err(_) => cfg!(target_os = "android"),
            };
        let cache_path = PIPELINE_CACHE
            .get()
            .cloned()
            .or_else(|| std::env::var_os("ZOPRO_PIPELINE_CACHE").map(std::path::PathBuf::from));
        let caching =
            cache_path.is_some() && adapter.features().contains(wgpu::Features::PIPELINE_CACHE);
        let mut features = wgpu::Features::empty();
        if spirv {
            features |= wgpu::Features::PASSTHROUGH_SHADERS;
        }
        if caching {
            features |= wgpu::Features::PIPELINE_CACHE;
        }
        let timing =
            profile::wanted() && adapter.features().contains(wgpu::Features::TIMESTAMP_QUERY);
        if timing {
            features |= wgpu::Features::TIMESTAMP_QUERY;
        }
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("zopro"),
                required_features: features,
                required_limits: adapter.limits(),
                ..Default::default()
            })
            .await
            .map_err(|e| format!("no device: {e}"))?;
        let failed = std::sync::Arc::new(std::sync::OnceLock::new());
        let flag = failed.clone();
        device.on_uncaptured_error(std::sync::Arc::new(move |e| {
            log::error!("zopro wgpu: {e}");
            let _ = flag.set(e.to_string());
        }));
        // A driver crash (a phone's shader compiler) loses the device with
        // no error in any scope: later buffers are silently invalid.
        let flag = failed.clone();
        device.set_device_lost_callback(Box::new(move |reason, message| {
            log::error!("zopro wgpu: device lost ({reason:?}): {message}");
            let _ = flag.set(format!("device lost ({reason:?}): {message}"));
        }));
        let cache = caching.then(|| {
            let data = cache_path
                .as_ref()
                .and_then(|path| std::fs::read(path).ok());
            // SAFETY: the data is what get_data() of this driver's cache
            // wrote, or absent; the driver validates it (fallback on a
            // mismatch).
            unsafe {
                device.create_pipeline_cache(&wgpu::PipelineCacheDescriptor {
                    label: Some("zopro"),
                    data: data.as_deref(),
                    fallback: true,
                })
            }
        });
        let timestamp_period = timing.then(|| queue.get_timestamp_period());
        Ok(Self {
            device,
            queue,
            name: format!("{} ({:?})", info.name, info.backend),
            failed,
            spirv,
            vendor: info.vendor,
            timestamp_period,
            cache,
            cache_path,
            kernels: Default::default(),
            modules: Default::default(),
        })
    }

    /// A WGSL module, compiled once per device.
    pub(crate) fn module(&self, label: &'static str, source: &'static str) -> wgpu::ShaderModule {
        self.modules
            .lock()
            .unwrap()
            .entry(label)
            .or_insert_with(|| {
                self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(label),
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                })
            })
            .clone()
    }

    /// The kernel of an entry point, built by `build` the first time.
    pub(crate) fn kernel(
        &self,
        entry: &str,
        build: impl FnOnce() -> wgpu::ComputePipeline,
    ) -> wgpu::ComputePipeline {
        if let Some(found) = self.kernels.lock().unwrap().get(entry) {
            return found.clone();
        }
        let pipeline = build();
        self.kernels
            .lock()
            .unwrap()
            .insert(entry.to_string(), pipeline.clone());
        pipeline
    }

    /// A WGSL kernel from a module, both cached.
    pub(crate) fn wgsl_kernel(
        &self,
        label: &'static str,
        source: &'static str,
        entry: &str,
    ) -> wgpu::ComputePipeline {
        self.kernel(entry, || {
            let module = self.module(label, source);
            self.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry),
                    layout: None,
                    module: &module,
                    entry_point: Some(entry),
                    compilation_options: Default::default(),
                    cache: self.cache(),
                })
        })
    }

    /// Builds every WGSL kernel now (the pre-compiled kernels need no
    /// warming): a browser calls this at init_gpu, so that the compilation
    /// runs while the key downloads and decodes rather than at its first
    /// proof.
    pub fn warm_kernels(&self) {
        if self.spirv {
            return;
        }
        for (label, source, entries) in [
            (G1_LABEL, G1_SHADER, <G1Projective as Curve>::ENTRIES),
            (G2_LABEL, G2_SHADER, <G2Projective as Curve>::ENTRIES),
        ] {
            for entry in entries {
                self.wgsl_kernel(label, source, entry);
            }
        }
        for (entry, source) in SORT_WGSL {
            self.wgsl_kernel(entry, source, entry);
        }
        for entry in quotient::ENTRIES {
            self.wgsl_kernel(quotient::LABEL, quotient::FR_SHADER, entry);
        }
    }

    /// The driver's pipeline cache, for pipeline descriptors.
    pub(crate) fn cache(&self) -> Option<&wgpu::PipelineCache> {
        self.cache.as_ref()
    }

    /// Writes the pipeline cache to its file, after kernels were built.
    pub(crate) fn save_cache(&self) {
        if let (Some(cache), Some(path)) = (&self.cache, &self.cache_path) {
            if let Some(data) = cache.get_data() {
                if let Err(e) = std::fs::write(path, data) {
                    log::warn!("zopro wgpu: pipeline cache not saved: {e}");
                }
            }
        }
    }

    /// Whether the device reported an error outside an error scope.
    pub fn failed(&self) -> bool {
        self.failed.get().is_some()
    }

    /// The first error the device reported outside an error scope.
    pub fn failure(&self) -> Option<&str> {
        self.failed.get().map(String::as_str)
    }
}

pub struct Msm<C: Curve> {
    /// Running the pre-compiled kernels, whose bindings are one contiguous
    /// set each (see `spirv_pipeline`).
    spirv: bool,
    /// The reduction leaves (V, S) per thread to the host (SPIRV_PAIRS).
    pairs: bool,
    gpu: std::sync::Arc<Gpu>,
    _curve: std::marker::PhantomData<C>,
    width: usize,
    half: usize,
    windows: usize,
    n: usize,
    first: wgpu::ComputePipeline,
    tree_pipeline: wgpu::ComputePipeline,
    reduce_blocks: wgpu::ComputePipeline,
    /// Buckets per reduction thread and threads per window.
    reduce_shape: [usize; 2],
    bases: wgpu::Buffer,
    sorted: wgpu::Buffer,
    /// Windows per chunk: the shard trees are sized for one chunk and
    /// reused, which bounds the GPU memory a wide key takes.
    chunk_windows: usize,
    /// Per chunk and level: shard starts, counts, destinations, uniform.
    levels: Vec<Vec<[wgpu::Buffer; 4]>>,
    tree: [wgpu::Buffer; 2],
    buckets: wgpu::Buffer,
    /// The reduction's partial sums, a workgroup's each.
    sums: wgpu::Buffer,
    staging: wgpu::Buffer,
    reduce_uniform: wgpu::Buffer,
    counts: Vec<u32>,
    starts: Vec<u32>,
    most: usize,
    /// The sort on the GPU, with the pre-compiled kernels.
    gpu_sort: Option<GpuSort>,
}

/// A browser frees a GPU buffer when the collector takes its JavaScript
/// object, not when it is dropped: the buffers are destroyed here, so that a
/// key let go frees its hundreds of megabytes at once.
impl<C: Curve> Drop for Msm<C> {
    fn drop(&mut self) {
        let fixed = [
            &self.bases,
            &self.sorted,
            &self.tree[0],
            &self.tree[1],
            &self.buckets,
            &self.sums,
            &self.staging,
            &self.reduce_uniform,
        ];
        let levels = self.levels.iter().flatten().flatten();
        let sort = self.gpu_sort.iter().flat_map(|s| {
            [
                &s.scalars,
                &s.counts,
                &s.counts_staging,
                &s.cursors,
            ]
            .into_iter()
            .chain(s.slices.iter())
        });
        for buffer in fixed.into_iter().chain(levels).chain(sort) {
            buffer.destroy();
        }
    }
}

/// Per-pass GPU times of one MSM (ZOPRO_WGPU_STAGES).
#[derive(Default)]
struct Stages(Vec<(String, f64)>);

impl Stages {
    /// Submits what the encoder holds, waits for it, records its time and
    /// hands back a fresh encoder.
    #[cfg(not(target_arch = "wasm32"))]
    fn cut(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: wgpu::CommandEncoder,
        name: String,
    ) -> wgpu::CommandEncoder {
        let started = Instant::now();
        let submission = queue.submit([encoder.finish()]);
        let _ = device.poll(wgpu::PollType::Wait {
            submission_index: Some(submission),
            timeout: None,
        });
        self.0
            .push((name, started.elapsed().as_secs_f64() * 1000.0));
        device.create_command_encoder(&Default::default())
    }
    #[cfg(target_arch = "wasm32")]
    fn cut(
        &mut self,
        device: &wgpu::Device,
        _: &wgpu::Queue,
        encoder: wgpu::CommandEncoder,
        _: String,
    ) -> wgpu::CommandEncoder {
        let _ = encoder;
        device.create_command_encoder(&Default::default())
    }
    fn report(&self) -> String {
        let total: f64 = self.0.iter().map(|(_, ms)| ms).sum();
        let stages: Vec<String> = self
            .0
            .iter()
            .map(|(name, ms)| format!("{name}={ms:.1}"))
            .collect();
        format!("total={total:.1} {}", stages.join(" "))
    }
}

/// The digits counted and the entries scattered on the GPU: the host only
/// reads the bucket counts back to plan the shards. (On a phone the host's
/// recoding and counting sort held the cores the CPU's G2 MSM needs.)
struct GpuSort {
    count: wgpu::ComputePipeline,
    scatter: wgpu::ComputePipeline,
    scalars: wgpu::Buffer,
    counts: wgpu::Buffer,
    counts_staging: wgpu::Buffer,
    cursors: wgpu::Buffer,
    /// One uniform per slice of SORT_SLICE entries: n, width, windows,
    /// half, the slice's first entry.
    slices: Vec<wgpu::Buffer>,
    /// The submitted counting, which `start` reads back.
    counted: Option<wgpu::SubmissionIndex>,
}

/// A binding of a pre-compiled kernel's one set: a storage buffer (read
/// only or not) or a uniform.
#[derive(Clone, Copy)]
pub(crate) enum Slot {
    Read,
    Write,
    Uniform,
}

/// A pre-compiled kernel's pipeline, its bind group layout declared (a
/// passthrough module cannot be reflected): binding i is `slots[i]`, in
/// one contiguous set 0 — Adreno's driver loses the device on binding gaps
/// or a second set.
pub(crate) fn spirv_pipeline(
    device: &wgpu::Device,
    cache: Option<&wgpu::PipelineCache>,
    entry: &str,
    spirv: &[u8],
    slots: &[Slot],
) -> wgpu::ComputePipeline {
    let words: Vec<u32> = spirv
        .as_chunks::<4>()
        .0
        .iter()
        .map(|c| u32::from_le_bytes(*c))
        .collect();
    // SAFETY: glslc's output for the compute shaders in glsl/, whose
    // bindings the slots declare.
    let module = unsafe {
        device.create_shader_module_passthrough(wgpu::ShaderModuleDescriptorPassthrough {
            label: Some(entry),
            entry_points: std::borrow::Cow::Borrowed(&[wgpu::PassthroughShaderEntryPoint {
                name: std::borrow::Cow::Borrowed("main"),
                workgroup_size: (WORKGROUP, 1, 1),
            }]),
            spirv: Some(std::borrow::Cow::Owned(words)),
            ..Default::default()
        })
    };
    let entries: Vec<wgpu::BindGroupLayoutEntry> = slots
        .iter()
        .enumerate()
        .map(|(binding, slot)| wgpu::BindGroupLayoutEntry {
            binding: binding as u32,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: match slot {
                    Slot::Read => wgpu::BufferBindingType::Storage { read_only: true },
                    Slot::Write => wgpu::BufferBindingType::Storage { read_only: false },
                    Slot::Uniform => wgpu::BufferBindingType::Uniform,
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect();
    let group = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &entries,
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(entry),
        bind_group_layouts: &[Some(&group)],
        immediate_size: 0,
    });
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(entry),
        layout: Some(&pipeline_layout),
        module: &module,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache,
    })
}

const G1_LABEL: &str = "zopro msm g1";
const G2_LABEL: &str = "zopro msm g2";

/// The smallest MSM the GPU sorts by default.
const GPU_SORT_MIN: usize = 1 << 16;

/// The GPU sort's threshold: GPU_SORT_MIN, or in a browser the build's
/// ZOPRO_WEB_GPU_SORT_MIN.
fn gpu_sort_min() -> usize {
    #[cfg(target_arch = "wasm32")]
    if let Some(min) = option_env!("ZOPRO_WEB_GPU_SORT_MIN").and_then(|v| v.parse().ok()) {
        return min;
    }
    GPU_SORT_MIN
}
/// The same kernels in WGSL, for the browser: count and scatter in
/// modules of their own, as their bindings differ.
const SORT_WGSL: [(&str, &str); 2] = [
    (
        "msm_count",
        concat!(
            include_str!("../shaders/fr.wgsl"),
            "\n",
            include_str!("../shaders/sort_digits.wgsl"),
            "\n",
            include_str!("../shaders/sort_count.wgsl")
        ),
    ),
    (
        "msm_scatter",
        concat!(
            include_str!("../shaders/fr.wgsl"),
            "\n",
            include_str!("../shaders/sort_digits.wgsl"),
            "\n",
            include_str!("../shaders/sort_scatter.wgsl")
        ),
    ),
];

const SORT_KERNELS: [(&str, &[u8]); 2] = [
    ("msm_count", include_bytes!("../glsl/out/msm_count.spv")),
    ("msm_scatter", include_bytes!("../glsl/out/msm_scatter.spv")),
];

/// The MSM kernels' slots, by index: shards_first is bases, sorted, start,
/// count, dst, out and the Shards uniform; shards_tree is tree, start,
/// count, dst, out and the uniform; the reduction is buckets, partials and
/// the Reduce uniform.
fn msm_slots(index: usize) -> &'static [Slot] {
    use Slot::*;
    match index {
        0 => &[Read, Read, Read, Read, Read, Write, Uniform],
        1 => &[Read, Read, Read, Read, Write, Uniform],
        _ => &[Read, Write, Uniform],
    }
}

pub(crate) fn storage(device: &wgpu::Device, label: &str, bytes: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes.max(16) as u64,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

/// A uniform buffer with its values queued, never mapped: wgpu's
/// `create_buffer_init` panics on a device that has failed.
pub(crate) fn uniform(gpu: &Gpu, values: [u32; 4]) -> wgpu::Buffer {
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("uniform"),
        size: 16,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    gpu.queue.write_buffer(&buffer, 0, bytemuck_cast(&values));
    buffer
}

/// A uniform of any number of words.
pub(crate) fn uniform_words(gpu: &Gpu, values: &[u32]) -> wgpu::Buffer {
    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("uniform"),
        size: (values.len() * 4) as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    gpu.queue.write_buffer(&buffer, 0, bytemuck_cast(values));
    buffer
}

/// Entries per count and scatter dispatch. The work a dispatch or a
/// submission holds is bounded: a OnePlus 13 warmed through a 54 → 1
/// merge ran Z's 2^21-entry scatter for 2.3 s in one dispatch and its MSM
/// for 5.6 s in one submission, and the next proof lost the device.
pub(crate) const SORT_SLICE: usize = 1 << 18;

/// The entries of slice `slice` of n.
pub(crate) fn slice_len(n: usize, slice: usize) -> u32 {
    (n - slice * SORT_SLICE).min(SORT_SLICE) as u32
}

/// What `encoder` holds, submitted; a fresh encoder for the rest.
pub(crate) fn flush(gpu: &Gpu, encoder: wgpu::CommandEncoder) -> wgpu::CommandEncoder {
    gpu.queue.submit([encoder.finish()]);
    gpu.device.create_command_encoder(&Default::default())
}

pub(crate) fn bytemuck_cast(values: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding; the slice's bytes are what the GPU reads.
    unsafe { std::slice::from_raw_parts(values.as_ptr() as *const u8, values.len() * 4) }
}

impl<C: Curve> Msm<C> {
    pub fn new(gpu: std::sync::Arc<Gpu>, bases: &[C::Affine]) -> Result<(Self, f64), String> {
        let device = &gpu.device;
        let started = Instant::now();
        // A driver that cannot build a kernel (a phone's refused to link one)
        // or a key that does not fit are errors here, not a lost device.
        let validation = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let internal = device.push_error_scope(wgpu::ErrorFilter::Internal);
        let spirv = if gpu.spirv { C::SPIRV } else { None };
        // After each kernel: a driver that refuses one leaves the device
        // failed, and nothing else must be asked of it.
        let mut pipelines = Vec::with_capacity(4);
        for (index, entry) in C::ENTRIES.into_iter().enumerate() {
            let started = Instant::now();
            pipelines.push(match spirv {
                Some(kernels) => {
                    {
                        // ZOPRO_SPV_DIR/<entry>.spv replaces a built-in kernel
                        // (kernel experiments on a device).
                        let file = std::env::var("ZOPRO_SPV_DIR")
                            .ok()
                            .and_then(|dir| std::fs::read(format!("{dir}/{entry}.spv")).ok());
                        if file.is_some() {
                            log::info!("ZOPRO_TRACE kernel {entry} from ZOPRO_SPV_DIR");
                        }
                        let kernel = file.as_deref().unwrap_or(kernels[index]);
                        gpu.kernel(entry, || {
                            spirv_pipeline(device, gpu.cache(), entry, kernel, msm_slots(index))
                        })
                    }
                }
                None => gpu.wgsl_kernel(C::LABEL, C::SHADER, entry),
            });
            // Delivers a device-lost callback the build caused.
            let _ = device.poll(wgpu::PollType::Poll);
            log::debug!(
                "zopro wgpu: kernel {entry} built in {:.0} ms",
                started.elapsed().as_secs_f64() * 1000.0
            );
            if gpu.failed() {
                return Err(format!("kernel {entry}: the device reported an error"));
            }
        }
        let [first, tree_pipeline, reduce_blocks]: [wgpu::ComputePipeline; 3] =
            pipelines.try_into().expect("three kernels");
        for scope in [internal, validation] {
            if let Some(e) = popped(scope) {
                return Err(format!("kernels: {e}"));
            }
        }
        if gpu.failed() {
            return Err("kernels: the device reported an error".into());
        }
        gpu.save_cache();
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let mut raw = Vec::with_capacity(bases.len() * C::AFFINE_BYTES);
        for p in bases {
            C::encode(p, &mut raw);
        }
        let n = bases.len();
        let width = width_for(n);
        let half = 1 << (width - 1);
        let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
        let buckets = windows * half;
        let per_window = n.div_ceil(first_shard(spirv.is_some())) + half;
        let chunk_windows = ((64 << 20) / (per_window * C::XYZZ_BYTES)).clamp(1, windows);
        let most = chunk_windows * per_window;
        let per_window = half
            .min(
                std::env::var("ZOPRO_REDUCE_PER_WINDOW")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(REDUCE_THREADS_PER_WINDOW),
            )
            .max(WORKGROUP as usize);
        let pairs = spirv.is_some() && C::SPIRV_PAIRS;
        // The host weighs the pairs: fewer threads per window keep that
        // cheap (ZOPRO_REDUCE_PER_WINDOW still overrides).
        let per_window = if pairs && std::env::var("ZOPRO_REDUCE_PER_WINDOW").is_err() {
            per_window.min(PAIRS_PER_WINDOW)
        } else {
            per_window
        };
        let reduce_shape = [half / per_window, per_window];
        let partials = if pairs {
            windows * per_window * 2
        } else {
            windows * per_window / WORKGROUP as usize
        };
        // The sort on the GPU for MSMs of 2^16 points and more: with the
        // pre-compiled kernels on a OnePlus 13 it freed the cores the CPU's
        // G2 MSM needs at 8 -> 2 (658 against 681 ms) and lost at 1 -> 2
        // (208 against 174), where the GPU round trips weigh more; in a
        // browser it frees the WebAssembly threads for G2 (the WGSL
        // kernels). ZOPRO_GPU_SORT=0|1 decides for every size natively.
        let gpu_sort = ((spirv.is_some() || cfg!(target_arch = "wasm32"))
            && match std::env::var("ZOPRO_GPU_SORT").as_deref() {
                Ok("1") => true,
                Ok(_) => false,
                Err(_) => n >= gpu_sort_min(),
            }
            || std::env::var("ZOPRO_GPU_SORT").as_deref() == Ok("1"))
        .then(|| {
            use Slot::*;
            let [(count_entry, count), (scatter_entry, scatter)] = SORT_KERNELS;
            let (count, scatter) = if spirv.is_some() {
                (
                    gpu.kernel(count_entry, || {
                        spirv_pipeline(device, gpu.cache(), count_entry, count, &[Read, Write, Uniform])
                    }),
                    gpu.kernel(scatter_entry, || {
                        spirv_pipeline(
                            device,
                            gpu.cache(),
                            scatter_entry,
                            scatter,
                            &[Read, Write, Write, Uniform],
                        )
                    }),
                )
            } else {
                let [(count_entry, count_source), (scatter_entry, scatter_source)] = SORT_WGSL;
                (
                    gpu.wgsl_kernel(count_entry, count_source, count_entry),
                    gpu.wgsl_kernel(scatter_entry, scatter_source, scatter_entry),
                )
            };
            GpuSort {
                count,
                scatter,
                scalars: storage(device, "scalars", n * 32),
                counts: storage(device, "counts", buckets * 4),
                counts_staging: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("counts staging"),
                    size: (buckets * 4) as u64,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                cursors: storage(device, "cursors", buckets * 4),
                slices: (0..n.div_ceil(SORT_SLICE))
                    .map(|slice| {
                        uniform_words(
                            &gpu,
                            &[
                                n as u32,
                                width as u32,
                                windows as u32,
                                half as u32,
                                (slice * SORT_SLICE) as u32,
                                0,
                                0,
                                0,
                            ],
                        )
                    })
                    .collect(),
                counted: None,
            }
        });
        if gpu_sort.is_some() {
            let _ = device.poll(wgpu::PollType::Poll);
            if gpu.failed() {
                return Err("sort kernels: the device reported an error".into());
            }
            gpu.save_cache();
        }
        let memory = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let bases_buffer = storage(device, "bases", raw.len());
        gpu.queue.write_buffer(&bases_buffer, 0, &raw);
        let msm = Self {
            gpu_sort,
            spirv: spirv.is_some(),
            pairs,
            width,
            half,
            windows,
            n,
            first,
            tree_pipeline,
            reduce_blocks,
            reduce_shape,
            bases: bases_buffer,
            sorted: storage(device, "sorted", n * windows * 4),
            chunk_windows,
            levels: Vec::new(),
            tree: [
                storage(device, "tree a", most * C::XYZZ_BYTES),
                storage(device, "tree b", most * C::XYZZ_BYTES),
            ],
            buckets: storage(device, "buckets", buckets * C::XYZZ_BYTES),
            sums: storage(device, "sums", partials * C::XYZZ_BYTES),
            staging: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("staging"),
                size: (partials * C::XYZZ_BYTES) as u64,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            reduce_uniform: uniform(
                &gpu,
                [half as u32, reduce_shape[0] as u32, per_window as u32, 0],
            ),
            counts: vec![0; buckets],
            starts: vec![0; buckets],
            most,
            gpu,
            _curve: std::marker::PhantomData,
        };
        if let Some(e) = popped(memory) {
            return Err(format!("buffers: {e}"));
        }
        if msm.gpu.failed() {
            return Err("buffers: the device reported an error".into());
        }
        Ok((msm, compile_ms))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn msm(&mut self, scalars: &[Fr]) -> Result<(C, Timings), String> {
        let pending = self.start(scalars);
        self.finish(pending)
    }

    /// Sorts the scalars and submits the MSM; the host is free until finish.
    pub fn start(&mut self, scalars: &[Fr]) -> Pending {
        assert_eq!(scalars.len(), self.n);
        self.start_with(Some(scalars))
    }

    /// The MSM whose scalars `count_from` counted on the GPU.
    pub fn start_counted(&mut self) -> Pending {
        assert!(
            self.gpu_sort
                .as_ref()
                .is_some_and(|sort| sort.counted.is_some()),
            "counted on the GPU"
        );
        self.start_with(None)
    }

    fn start_with(&mut self, scalars: Option<&[Fr]>) -> Pending {
        let started = Instant::now();
        if self.gpu_sort.is_some() {
            self.count_once(scalars);
            #[cfg(not(target_arch = "wasm32"))]
            self.read_counts();
        } else {
            self.sort_on_host(scalars);
        }
        self.submit_sorted(started)
    }

    /// `start` in a browser: the GPU's counts are awaited, not waited for.
    pub async fn start_async(&mut self, scalars: &[Fr]) -> Pending {
        assert_eq!(scalars.len(), self.n);
        self.start_with_async(Some(scalars)).await
    }

    /// `start_counted` in a browser.
    pub async fn start_counted_async(&mut self) -> Pending {
        self.start_with_async(None).await
    }

    async fn start_with_async(&mut self, scalars: Option<&[Fr]>) -> Pending {
        let started = Instant::now();
        if self.gpu_sort.is_some() {
            self.count_once(scalars);
            self.read_counts_async().await;
        } else {
            self.sort_on_host(scalars);
        }
        self.submit_sorted(started)
    }

    /// Submits the counting unless `count`/`count_from` already did.
    fn count_once(&mut self, scalars: Option<&[Fr]>) {
        if self
            .gpu_sort
            .as_ref()
            .is_some_and(|sort| sort.counted.is_none())
        {
            self.count(scalars.expect("scalars to count"));
        }
    }

    fn sort_on_host(&mut self, scalars: Option<&[Fr]>) {
        let sorted = self.sort(scalars.expect("scalars to sort"));
        self.gpu
            .queue
            .write_buffer(&self.sorted, 0, bytemuck_cast(&sorted));
    }

    /// The MSM's passes over sorted entries (the GPU scatters them first
    /// when it sorts), submitted.
    fn submit_sorted(&mut self, started: Instant) -> Pending {
        let mut timings = Timings::default();
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        // ZOPRO_WGPU_STAGES=1: each pass submitted and waited for alone,
        // its time traced (a profile, not a proof's timing).
        let mut profile = std::env::var("ZOPRO_WGPU_STAGES")
            .is_ok_and(|v| v == "1")
            .then(Stages::default);
        let chunks: Vec<Vec<usize>> = (0..self.windows.div_ceil(self.chunk_windows))
            .map(|chunk| {
                let first = chunk * self.chunk_windows;
                self.plan(chunk, first..(first + self.chunk_windows).min(self.windows))
            })
            .collect();
        let passes = self.gpu_sort.as_ref().map_or(0, |s| s.slices.len())
            + chunks.iter().map(Vec::len).sum::<usize>()
            + 1;
        let mut timer = profile::Timer::new(&self.gpu, passes);
        if let Some(sort) = &self.gpu_sort {
            self.gpu
                .queue
                .write_buffer(&sort.cursors, 0, bytemuck_cast(&self.starts));
            for (slice, uniform) in sort.slices.iter().enumerate() {
                let group = self
                    .gpu
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &sort.scatter.get_bind_group_layout(0),
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: sort.scalars.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: sort.cursors.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: self.sorted.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: uniform.as_entire_binding(),
                            },
                        ],
                    });
                let mut pass = match timer.as_mut() {
                    Some(timer) => encoder.begin_compute_pass(&timer.pass("msm_scatter")),
                    None => encoder.begin_compute_pass(&Default::default()),
                };
                pass.set_pipeline(&sort.scatter);
                pass.set_bind_group(0, &group, &[]);
                pass.dispatch_workgroups(slice_len(self.n, slice).div_ceil(WORKGROUP), 1, 1);
                drop(pass);
                encoder = match profile.as_mut() {
                    Some(stages) => {
                        stages.cut(&self.gpu.device, &self.gpu.queue, encoder, "scatter".into())
                    }
                    None => flush(&self.gpu, encoder),
                };
            }
        }
        timings.sort_ms = started.elapsed().as_secs_f64() * 1000.0;

        let started = Instant::now();
        let device = &self.gpu.device;
        let queue = &self.gpu.queue;
        encoder.clear_buffer(&self.buckets, 0, None);
        for (chunk, levels) in chunks.iter().enumerate() {
            let last = levels.len() - 1;
            for (level, shards) in levels.iter().enumerate() {
                let pipeline = if level == 0 {
                    &self.first
                } else {
                    &self.tree_pipeline
                };
                let [start, count, dst, params] = &self.levels[chunk][level];
                let out = if level == last {
                    &self.buckets
                } else {
                    &self.tree[level % 2]
                };
                // A layout derived from the entry point holds only the bindings
                // it reads: the tree levels have no bases or sorted indices.
                let mut entries = vec![
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: start.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: count.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: dst.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 5,
                        resource: out.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 6,
                        resource: params.as_entire_binding(),
                    },
                ];
                if level == 0 {
                    entries.push(wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.bases.as_entire_binding(),
                    });
                    entries.push(wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.sorted.as_entire_binding(),
                    });
                } else if self.spirv {
                    // tree, start, count, dst, out, shards in one set.
                    entries.insert(
                        0,
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.tree[(level - 1) % 2].as_entire_binding(),
                        },
                    );
                    for (i, entry) in entries.iter_mut().enumerate() {
                        entry.binding = i as u32;
                    }
                }
                let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &entries,
                });
                let group1 = (level > 0 && !self.spirv).then(|| {
                    device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &pipeline.get_bind_group_layout(1),
                        entries: &[wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.tree[(level - 1) % 2].as_entire_binding(),
                        }],
                    })
                });
                let kernel = C::ENTRIES[if level == 0 { 0 } else { 1 }];
                let mut pass = match timer.as_mut() {
                    Some(timer) => encoder.begin_compute_pass(&timer.pass(kernel)),
                    None => encoder.begin_compute_pass(&Default::default()),
                };
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &group0, &[]);
                if let Some(group1) = &group1 {
                    pass.set_bind_group(1, group1, &[]);
                }
                let group = if level == 0 { first_group() } else { WORKGROUP };
                pass.dispatch_workgroups((*shards as u32).div_ceil(group), 1, 1);
                drop(pass);
                if let Some(stages) = profile.as_mut() {
                    encoder =
                        stages.cut(device, queue, encoder, format!("c{chunk}l{level}x{shards}"));
                }
            }
            // A submission per chunk: no submission holds the whole MSM.
            if profile.is_none() {
                encoder = flush(&self.gpu, encoder);
            }
        }
        {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.reduce_blocks.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.buckets.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.sums.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: if self.spirv { 2 } else { 3 },
                        resource: self.reduce_uniform.as_entire_binding(),
                    },
                ],
            });
            let mut pass = match timer.as_mut() {
                Some(timer) => encoder.begin_compute_pass(&timer.pass(C::ENTRIES[2])),
                None => encoder.begin_compute_pass(&Default::default()),
            };
            pass.set_pipeline(&self.reduce_blocks);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(
                (self.windows * self.reduce_shape[1] / WORKGROUP as usize) as u32,
                1,
                1,
            );
        }
        if let Some(mut stages) = profile.take() {
            encoder = stages.cut(device, queue, encoder, "reduce".into());
            log::info!("ZOPRO_TRACE wgpu-stages n={} {}", self.n, stages.report());
        }
        encoder.copy_buffer_to_buffer(&self.sums, 0, &self.staging, 0, self.sums.size());
        if let Some(timer) = &timer {
            timer.resolve(&mut encoder);
        }
        let submission = queue.submit([encoder.finish()]);
        Pending {
            submitted: started,
            timings,
            submission,
            timer,
        }
    }

    /// Waits for a submitted MSM and combines its window sums.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn finish(&mut self, pending: Pending) -> Result<(C, Timings), String> {
        pollster::block_on(self.finish_async(pending))
    }

    /// As `finish`, yielding while the GPU works: the browser's way.
    pub async fn finish_async(&mut self, pending: Pending) -> Result<(C, Timings), String> {
        let mut timings = pending.timings;
        mapped(&self.gpu, &self.staging, Some(pending.submission.clone())).await?;
        if let Some(timer) = pending.timer {
            timer.read(&self.gpu, pending.submission.clone()).await?;
        }
        let data = self
            .staging
            .slice(..)
            .get_mapped_range()
            .map_err(|e| format!("sums: {e}"))?
            .to_vec();
        self.staging.unmap();
        timings.gpu_ms = pending.submitted.elapsed().as_secs_f64() * 1000.0;

        let started = Instant::now();
        if self.pairs {
            let result = self.weigh_pairs(&data);
            timings.combine_ms = started.elapsed().as_secs_f64() * 1000.0;
            return Ok((result, timings));
        }
        let mut result = C::IDENTITY;
        for window in (0..self.windows).rev() {
            for _ in 0..self.width {
                result = result.double();
            }
            let groups = self.reduce_shape[1] / WORKGROUP as usize;
            for group in 0..groups {
                let at = (window * groups + group) * C::XYZZ_BYTES;
                result += C::xyzz_to_projective(&data[at..at + C::XYZZ_BYTES]);
            }
        }
        timings.combine_ms = started.elapsed().as_secs_f64() * 1000.0;
        Ok((result, timings))
    }

    /// The MSM from the reduction's (V, S) pairs: thread t of a window
    /// covers buckets from t * leaf, so the window's sum is the sum of V_t
    /// plus leaf * sum t * S_t, the latter by running sums over t.
    fn weigh_pairs(&self, data: &[u8]) -> C {
        let [leaf, threads] = self.reduce_shape;
        let point = |index: usize| {
            C::xyzz_to_projective(&data[index * C::XYZZ_BYTES..(index + 1) * C::XYZZ_BYTES])
        };
        let sums: Vec<C> = (0..self.windows)
            .into_par_iter()
            .map(|window| {
                let pair = |t: usize| 2 * (window * threads + t);
                let mut vs = C::IDENTITY;
                let mut running = C::IDENTITY;
                let mut weighted = C::IDENTITY;
                for t in (0..threads).rev() {
                    vs += point(pair(t));
                    if t > 0 {
                        running += point(pair(t) + 1);
                        weighted += running;
                    }
                }
                for _ in 0..leaf.trailing_zeros() {
                    weighted = weighted.double();
                }
                vs + weighted
            })
            .collect();
        let mut result = C::IDENTITY;
        for sum in sums.iter().rev() {
            for _ in 0..self.width {
                result = result.double();
            }
            result += *sum;
        }
        result
    }

    /// Levels of shards over the sorted runs of a chunk of windows, into
    /// the chunk's level buffers; the last level writes to the buckets.
    fn plan(&mut self, chunk: usize, windows: std::ops::Range<usize>) -> Vec<usize> {
        let first_bucket = windows.start * self.half;
        let buckets = windows.len() * self.half;
        let mut runs: Vec<(u32, u32)> = self.starts[first_bucket..first_bucket + buckets]
            .iter()
            .zip(&self.counts[first_bucket..first_bucket + buckets])
            .map(|(s, c)| (*s, *c))
            .collect();
        let mut shard_counts = Vec::new();
        let mut level = 0;
        if self.levels.len() <= chunk {
            self.levels.push(Vec::new());
        }
        loop {
            let size = if level == 0 {
                first_shard(self.spirv)
            } else {
                shard()
            } as u32;
            let last = runs.iter().all(|(_, c)| *c <= size);
            let (mut start, mut count, mut dst) = (Vec::new(), Vec::new(), Vec::new());
            let mut next = Vec::with_capacity(buckets);
            for (b, (run_start, run_count)) in runs.iter().enumerate() {
                let first = start.len() as u32;
                let mut at = 0;
                while at < *run_count {
                    let len = size.min(run_count - at);
                    start.push(run_start + at);
                    count.push(len);
                    dst.push(if last {
                        (first_bucket + b) as u32
                    } else {
                        first + (at / size)
                    });
                    at += len;
                }
                next.push((first, start.len() as u32 - first));
            }
            if self.levels[chunk].len() <= level {
                let device = &self.gpu.device;
                self.levels[chunk].push([
                    storage(device, "start", self.most * 4),
                    storage(device, "count", self.most * 4),
                    storage(device, "dst", self.most * 4),
                    uniform(&self.gpu, [0; 4]),
                ]);
            }
            let queue = &self.gpu.queue;
            let [bs, bc, bd, bp] = &self.levels[chunk][level];
            queue.write_buffer(bs, 0, bytemuck_cast(&start));
            queue.write_buffer(bc, 0, bytemuck_cast(&count));
            queue.write_buffer(bd, 0, bytemuck_cast(&dst));
            queue.write_buffer(bp, 0, bytemuck_cast(&[start.len() as u32, 0, 0, 0]));
            shard_counts.push(start.len());
            if last {
                return shard_counts;
            }
            runs = next;
            level += 1;
        }
    }

    /// Digits and the counting sort per window; returns the sorted encoded
    /// base indices, window segments of n.
    /// Whether the GPU sorts this MSM's entries (`count` may then go ahead
    /// of `start`).
    pub fn sorts_on_gpu(&self) -> bool {
        self.gpu_sort.is_some()
    }

    /// Submits the counting of the scalars' digits per bucket, for a
    /// `start` with the same scalars to read back: queued ahead of other
    /// work, the counts are back by the time the host plans.
    pub fn count(&mut self, scalars: &[Fr]) {
        assert_eq!(scalars.len(), self.n);
        let sort = self.gpu_sort.as_ref().expect("the GPU sort");
        // SAFETY: Fr is four u64 limbs with no padding, the kernels' encoding.
        let bytes = unsafe {
            std::slice::from_raw_parts(scalars.as_ptr() as *const u8, scalars.len() * 32)
        };
        self.gpu.queue.write_buffer(&sort.scalars, 0, bytes);
        let encoder = self.gpu.device.create_command_encoder(&Default::default());
        self.submit_count(encoder);
    }

    /// As `count`, the scalars being the first n Fr of a GPU buffer (the
    /// quotient's output for Z's MSM): copied on the GPU, never through
    /// the host. `start_counted` then runs the MSM.
    pub fn count_from(&mut self, source: &wgpu::Buffer) {
        let sort = self.gpu_sort.as_ref().expect("the GPU sort");
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(source, 0, &sort.scalars, 0, (self.n * 32) as u64);
        self.submit_count(encoder);
    }

    /// The counting passes after what `encoder` holds, submitted a slice
    /// at a time.
    fn submit_count(&mut self, mut encoder: wgpu::CommandEncoder) {
        let sort = self.gpu_sort.as_mut().expect("the GPU sort");
        let (device, queue) = (&self.gpu.device, &self.gpu.queue);
        encoder.clear_buffer(&sort.counts, 0, None);
        for (slice, uniform) in sort.slices.iter().enumerate() {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &sort.count.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: sort.scalars.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: sort.counts.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: uniform.as_entire_binding(),
                    },
                ],
            });
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&sort.count);
                pass.set_bind_group(0, &group, &[]);
                pass.dispatch_workgroups(slice_len(self.n, slice).div_ceil(WORKGROUP), 1, 1);
            }
            encoder = flush(&self.gpu, encoder);
        }
        encoder.copy_buffer_to_buffer(&sort.counts, 0, &sort.counts_staging, 0, sort.counts.size());
        sort.counted = Some(queue.submit([encoder.finish()]));
    }

    /// The submitted counts read back into `counts` and `starts` (each
    /// window's entries from `w * n`).
    #[cfg(not(target_arch = "wasm32"))]
    fn read_counts(&mut self) {
        pollster::block_on(self.read_counts_async());
    }

    /// As `read_counts`, yielding while the GPU counts: the browser's way.
    async fn read_counts_async(&mut self) {
        let sort = self.gpu_sort.as_mut().expect("the GPU sort");
        let submission = sort.counted.take().expect("counts submitted");
        let staging = &sort.counts_staging;
        // A failed device leaves the counts as they were: the MSM's finish
        // reports it.
        if mapped(&self.gpu, staging, Some(submission)).await.is_ok() {
            let data = staging
                .slice(..)
                .get_mapped_range()
                .expect("counts mapped")
                .to_vec();
            for (count, bytes) in self.counts.iter_mut().zip(data.as_chunks::<4>().0) {
                *count = u32::from_le_bytes(*bytes);
            }
            staging.unmap();
        }
        let (n, half) = (self.n, self.half);
        for (w, (starts, counts)) in self
            .starts
            .chunks_mut(half)
            .zip(self.counts.chunks(half))
            .enumerate()
        {
            let mut at = (w * n) as u32;
            for (start, count) in starts.iter_mut().zip(counts) {
                *start = at;
                at += count;
            }
        }
    }

    fn sort(&mut self, scalars: &[Fr]) -> Vec<u32> {
        let (width, windows, half, n) = (self.width, self.windows, self.half, self.n);
        let radix = 1u64 << width;
        let mask = radix - 1;
        let mut digits = vec![0i16; n * windows];
        digits
            .par_chunks_mut(windows)
            .zip(scalars)
            .for_each(|(row, s)| {
                let words = s.to_canonical();
                let mut carry = 0u64;
                for (w, digit) in row.iter_mut().enumerate() {
                    let offset = w * width;
                    let (word, shift) = (offset / 64, offset % 64);
                    let mut value = if word < 4 { words[word] >> shift } else { 0 };
                    if shift + width > 64 && word < 3 {
                        value |= words[word + 1] << (64 - shift);
                    }
                    let value = (value & mask) + carry;
                    carry = (value + radix / 2) >> width;
                    *digit = (value as i64 - (carry * radix) as i64) as i16;
                }
            });
        let mut sorted = vec![0u32; n * windows];
        sorted
            .par_chunks_mut(n)
            .zip(self.starts.par_chunks_mut(half))
            .zip(self.counts.par_chunks_mut(half))
            .enumerate()
            .for_each(|(w, ((sorted, starts), counts))| {
                counts.fill(0);
                for i in 0..n {
                    let d = digits[i * windows + w];
                    if d != 0 {
                        counts[d.unsigned_abs() as usize - 1] += 1;
                    }
                }
                let base = (w * n) as u32;
                let mut at = base;
                for (start, count) in starts.iter_mut().zip(counts.iter()) {
                    *start = at;
                    at += count;
                }
                let mut positions: Vec<u32> = starts.iter().map(|s| s - base).collect();
                for i in 0..n {
                    let d = digits[i * windows + w];
                    if d != 0 {
                        let b = d.unsigned_abs() as usize - 1;
                        sorted[positions[b] as usize] =
                            i as u32 | if d < 0 { 0x8000_0000 } else { 0 };
                        positions[b] += 1;
                    }
                }
            });
        sorted
    }
}

fn fq(bytes: &[u8]) -> Fq {
    Fq::from_montgomery(core::array::from_fn(|i| {
        u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap())
    }))
}

/// The tests' logger: wgpu's messages on stderr (RUST_LOG, default debug).
#[cfg(test)]
fn init_logger() {
    let _ = env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("zopro_wgpu=debug"),
    )
    .is_test(false)
    .try_init();
}

/// A seeded source of field elements and points for the tests (splitmix64).
#[cfg(test)]
pub(crate) mod test_rng {
    use zopro_bn254::{Affine, CurveConfig, Fq, Fq2, Fr, Projective};

    pub(crate) struct StdRng(u64);

    pub(crate) trait SeedableRng {
        fn seed_from_u64(seed: u64) -> Self;
    }
    impl SeedableRng for StdRng {
        fn seed_from_u64(seed: u64) -> Self {
            Self(seed)
        }
    }

    impl StdRng {
        fn bytes(&mut self) -> [u8; 64] {
            let mut bytes = [0u8; 64];
            for chunk in bytes.chunks_mut(8) {
                self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let mut z = self.0;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                chunk.copy_from_slice(&(z ^ (z >> 31)).to_le_bytes());
            }
            bytes
        }

        pub(crate) fn point<C: CurveConfig>(&mut self) -> Affine<C> {
            Projective::<C>::generator().mul(&Fr::sample(self)).to_affine()
        }
    }

    /// A uniform element.
    pub(crate) trait Sample {
        fn sample(rng: &mut StdRng) -> Self;
    }
    impl Sample for Fq {
        fn sample(rng: &mut StdRng) -> Self {
            Fq::from_random_bytes(&rng.bytes())
        }
    }
    impl Sample for Fr {
        fn sample(rng: &mut StdRng) -> Self {
            Fr::from_random_bytes(&rng.bytes())
        }
    }
    impl Sample for Fq2 {
        fn sample(rng: &mut StdRng) -> Self {
            Fq2::new(Fq::sample(rng), Fq::sample(rng))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_rng::{Sample, SeedableRng, StdRng};
    use zopro_bn254::{Affine, CurveConfig, G1Config, G2Config, Projective};

    /// Distinct points without a scalar multiplication each: a random
    /// start, then repeated addition of a random step.
    pub(super) fn points<C: CurveConfig>(n: usize, rng: &mut StdRng) -> Vec<Affine<C>> {
        let step = rng.point::<C>().to_projective();
        let mut p = rng.point::<C>().to_projective();
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(p);
            p += step;
        }
        Projective::<C>::batch_to_affine(&out)
    }

    /// Scalars as witnesses have them: zeros, ones, small values, minus
    /// one, and random ones.
    fn scalars(n: usize, skewed: bool, rng: &mut StdRng) -> Vec<Fr> {
        (0..n)
            .map(|i| match (skewed, i % 5) {
                (true, 0) => Fr::from(0u64),
                (true, 1) => Fr::from(1u64),
                (true, 2) => Fr::from((i % 300) as u64),
                (true, 3) => -Fr::from(1u64),
                _ => Fr::sample(rng),
            })
            .collect()
    }

    fn check<C: CurveConfig>(sizes: &[usize])
    where
        Projective<C>: Curve<Affine = Affine<C>>,
    {
        let Some(mut make) = backend::<Projective<C>>() else {
            eprintln!("no GPU: skipped");
            return;
        };
        let mut rng = StdRng::seed_from_u64(7);
        for &n in sizes {
            let bases = points::<C>(n, &mut rng);
            let mut msm = make(&bases);
            for skewed in [false, true] {
                let scalars = scalars(n, skewed, &mut rng);
                let expected = zopro_bn254::curve::msm(&bases, &scalars);
                assert_eq!(
                    msm(&scalars).to_affine(),
                    expected.to_affine(),
                    "{n} points, skewed {skewed}"
                );
            }
            let zeros = vec![Fr::from(0u64); n];
            assert!(msm(&zeros).is_identity(), "{n} points, all zero");
        }
    }

    #[allow(clippy::type_complexity)]
    fn backend<C: Curve>() -> Option<impl FnMut(&[C::Affine]) -> Box<dyn FnMut(&[Fr]) -> C>> {
        init_logger();
        let gpu = std::sync::Arc::new(Gpu::new().ok()?);
        Some(move |bases: &[C::Affine]| -> Box<dyn FnMut(&[Fr]) -> C> {
            let mut msm = Msm::<C>::new(gpu.clone(), bases).expect("msm").0;
            Box::new(move |scalars: &[Fr]| msm.msm(scalars).expect("gpu msm").0)
        })
    }

    #[test]
    fn g1_msms_match_the_reference() {
        check::<G1Config>(&[1, 7, 100, 3000, 20000]);
    }

    #[test]
    fn g2_msms_match_the_reference() {
        check::<G2Config>(&[1, 100, 3000]);
    }
}

#[cfg(test)]
mod field_tests {
    use super::*;
    use crate::test_rng::{Sample, SeedableRng, StdRng};
    use zopro_bn254::{Affine, CurveConfig, Field, G1Config, G2Config, Projective};

    /// One kernel computing a + b, a - b, a * b, -a and a^2 per pair, for a field
    /// file's prefix and type, against the CPU's.
    fn check<F: Field + Sample>(source: &str, prefix: &str, ty: &str) {
        let k = std::mem::size_of::<F>() / 4;
        init_logger();
        let Ok(gpu) = Gpu::new() else {
            eprintln!("no GPU: skipped");
            return;
        };
        let kernel = format!(
            "{source}\n@group(0) @binding(0) var<storage, read_write> data: array<{ty}>;\n\
             @compute @workgroup_size(64) fn ops(@builtin(global_invocation_id) gid: vec3<u32>) {{\n\
             let i = gid.x; if (i >= arrayLength(&data) / 7u) {{ return; }}\n\
             let a = data[7u * i]; let b = data[7u * i + 1u];\n\
             data[7u * i + 2u] = {prefix}_add(a, b); data[7u * i + 3u] = {prefix}_sub(a, b);\n\
             data[7u * i + 4u] = {prefix}_mul(a, b); data[7u * i + 5u] = {prefix}_neg(a);\n\
             data[7u * i + 6u] = {prefix}_square(a); }}\n"
        );
        let mut rng = StdRng::seed_from_u64(3);
        let n = 4096;
        let mut values: Vec<(F, F)> = (0..n)
            .map(|_| (F::sample(&mut rng), F::sample(&mut rng)))
            .collect();
        // Edge cases: zero, one, minus one, the same and opposite values.
        values[0] = (F::ZERO, F::ZERO);
        values[1] = (F::ONE, -F::ONE);
        values[2] = (-F::ONE, -F::ONE);
        values[3] = (values[3].0, values[3].0);
        values[4] = (values[4].0, -values[4].0);
        let words = |f: &F| -> Vec<u32> {
            // Montgomery form, as the kernels hold it, as the CPU's.
            let repr = unsafe { std::slice::from_raw_parts(f as *const F as *const u32, k) };
            repr.to_vec()
        };
        let mut input: Vec<u32> = Vec::new();
        for (a, b) in &values {
            input.extend(words(a));
            input.extend(words(b));
            input.extend(vec![0u32; 5 * k]);
        }
        let device = &gpu.device;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(kernel.into()),
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("ops"),
            compilation_options: Default::default(),
            cache: None,
        });
        let buffer = storage(device, "data", input.len() * 4);
        gpu.queue.write_buffer(&buffer, 0, bytemuck_cast(&input));
        let staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (input.len() * 4) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups((n as u32).div_ceil(64), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&buffer, 0, &staging, 0, (input.len() * 4) as u64);
        gpu.queue.submit([encoder.finish()]);
        pollster::block_on(mapped(&gpu, &staging, None)).expect("map");
        let out: Vec<u32> = staging
            .slice(..)
            .get_mapped_range()
            .unwrap()
            .chunks(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        let back = |w: &[u32]| -> F { unsafe { *(w.as_ptr() as *const F) } };
        for (i, (a, b)) in values.iter().enumerate() {
            let o = &out[i * 7 * k..(i + 1) * 7 * k];
            assert_eq!(back(&o[2 * k..]), *a + *b, "{prefix} add {i}");
            assert_eq!(back(&o[3 * k..]), *a - *b, "{prefix} sub {i}");
            assert_eq!(back(&o[4 * k..]), *a * *b, "{prefix} mul {i}");
            assert_eq!(back(&o[5 * k..]), -*a, "{prefix} neg {i}");
            assert_eq!(back(&o[6 * k..]), a.square(), "{prefix} square {i}");
        }
        let _ = F::ONE.square();
    }

    #[test]
    fn fq_ops_match_the_cpu() {
        check::<Fq>(include_str!("../shaders/field.wgsl"), "fp", "Fp");
    }

    #[test]
    fn fr_ops_match_the_cpu() {
        check::<Fr>(include_str!("../shaders/fr.wgsl"), "fr", "Fr");
    }

    /// The point formulas per pair of affine points p, q: p + q (both
    /// lifted), p + q mixed, the pair add, 2p, and p + p mixed, against
    /// the CPU's.
    fn check_points<C: CurveConfig>(prefix: &str, ty: &str, affine: &str)
    where
        Projective<C>: Curve<Affine = Affine<C>>,
    {
        let Ok(gpu) = Gpu::new() else {
            eprintln!("no GPU: skipped");
            return;
        };
        let source = <Projective<C> as Curve>::SHADER.split("@group").next().unwrap();
        let kernel = format!(
            "{source}\n@group(0) @binding(0) var<storage, read> input: array<{affine}>;\n\
             @group(0) @binding(1) var<storage, read_write> output: array<{ty}>;\n\
             @compute @workgroup_size(64) fn ops(@builtin(global_invocation_id) gid: vec3<u32>) {{\n\
             let i = gid.x; if (i >= arrayLength(&input) / 2u) {{ return; }}\n\
             let p = input[2u * i]; let q = input[2u * i + 1u];\n\
             let lp = {prefix}_from_affine(p);\n\
             output[5u * i + 3u] = {prefix}_double(lp);\n\
             output[5u * i] = {prefix}_add(lp, {prefix}_from_affine(q));\n\
             output[5u * i + 1u] = {prefix}_add_mixed(lp, q);\n\
             output[5u * i + 2u] = {prefix}_add_affine_pair(lp, q);\n\
             output[5u * i + 4u] = {prefix}_add_mixed(lp, p); }}\n"
        );
        let mut rng = StdRng::seed_from_u64(4);
        let n = 256;
        let mut points: Vec<Affine<C>> = (0..2 * n).map(|_| rng.point::<C>()).collect();
        points[1] = points[0];
        points[3] = -points[2];
        points[4] = Affine::<C>::IDENTITY;
        points[7] = Affine::<C>::IDENTITY;
        let mut input = Vec::new();
        for p in &points {
            <Projective<C> as Curve>::encode(p, &mut input);
        }
        let device = &gpu.device;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(kernel.into()),
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("ops"),
            compilation_options: Default::default(),
            cache: None,
        });
        let in_buffer = storage(device, "input", input.len());
        gpu.queue.write_buffer(&in_buffer, 0, &input);
        let out_len = (5 * n * <Projective<C> as Curve>::XYZZ_BYTES) as u64;
        let out_buffer = storage(device, "output", out_len as usize);
        let staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: out_len,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: in_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: out_buffer.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups((n as u32).div_ceil(64), 1, 1);
        }
        encoder.copy_buffer_to_buffer(&out_buffer, 0, &staging, 0, out_len);
        gpu.queue.submit([encoder.finish()]);
        pollster::block_on(mapped(&gpu, &staging, None)).expect("map");
        let out = staging.slice(..).get_mapped_range().unwrap().to_vec();
        let bytes = <Projective<C> as Curve>::XYZZ_BYTES;
        let at = |k: usize| <Projective<C> as Curve>::xyzz_to_projective(&out[k * bytes..(k + 1) * bytes]);
        let names = [
            "add",
            "add_mixed",
            "add_affine_pair",
            "double",
            "add_mixed self",
        ];
        let mut wrong = Vec::new();
        for i in 0..n {
            let (p, q) = (points[2 * i].to_projective(), points[2 * i + 1].to_projective());
            for (k, expected) in [p + q, p + q, p + q, p.double(), p.double()]
                .into_iter()
                .enumerate()
            {
                if at(5 * i + k) != expected {
                    wrong.push(format!("{} {i}", names[k]));
                }
            }
        }
        assert!(
            wrong.is_empty(),
            "{prefix}: {} wrong: {:?}",
            wrong.len(),
            &wrong[..wrong.len().min(12)]
        );
    }

    /// Which kernels a driver links: each probe builds one small kernel on
    /// a fresh device and reports whether the device survived (a phone's
    /// driver loses the device instead of reporting a link error). Prints
    /// PROBE lines; run it with --ignored on a new GPU.
    #[test]
    #[ignore]
    fn kernel_probes() {
        init_logger();
        let g1 = G1_SHADER.split("@group").next().unwrap();
        let g2 = G2_SHADER.split("@group").next().unwrap();
        let g1_io = "@group(0) @binding(0) var<storage, read> tree: array<Xyzz>;\n\
            @group(0) @binding(1) var<storage, read> bases: array<Affine>;\n\
            @group(0) @binding(2) var<storage, read_write> out: array<Xyzz>;\n";
        let g2_io = "@group(0) @binding(0) var<storage, read> tree: array<Xyzz2>;\n\
            @group(0) @binding(1) var<storage, read> bases: array<Affine2>;\n\
            @group(0) @binding(2) var<storage, read_write> out: array<Xyzz2>;\n\
            @group(0) @binding(3) var<storage, read_write> data: array<Fp2>;\n";
        let probes: [(&str, &str, &str); 12] = [
            ("g1 tree loop (shards_tree)", g1, "var acc = tree[0]; for (var i = 1u; i < arrayLength(&tree); i++) { acc = xyzz_add(acc, tree[i]); } out[0] = acc;"),
            ("g1 single add", g1, "out[0] = xyzz_add(tree[0], tree[1]);"),
            ("g1 mixed loop (shards_first)", g1, "var acc = xyzz_from_affine(bases[0]); for (var i = 1u; i < arrayLength(&bases); i++) { acc = xyzz_add_mixed(acc, bases[i]); } out[0] = acc;"),
            ("g1 single mixed", g1, "out[0] = xyzz_add_mixed(tree[0], bases[0]);"),
            ("g1 double", g1, "out[0] = xyzz_double(tree[0]);"),
            ("g1 add unrolled x4", g1, "out[0] = xyzz_add(xyzz_add(tree[0], tree[1]), xyzz_add(tree[2], tree[3]));"),
            ("fp2 add", g2, "data[1] = fp2_add(data[0], data[1]);"),
            ("fp2 mul", g2, "data[1] = fp2_mul(data[0], data[1]);"),
            ("fp2 square", g2, "data[1] = fp2_square(data[0]);"),
            ("fp2 mul then add", g2, "data[2] = fp2_add(fp2_mul(data[0], data[1]), data[0]);"),
            ("g2 single add", g2, "out[0] = xyzz2_add(tree[0], tree[1]);"),
            ("g2 single mixed", g2, "out[0] = xyzz2_add_mixed(tree[0], bases[0]);"),
        ];
        for (name, prefix, body) in probes {
            let io = if std::ptr::eq(prefix, g1) {
                g1_io
            } else {
                g2_io
            };
            let source = format!("{prefix}\n{io}@compute @workgroup_size(64) fn probe(@builtin(global_invocation_id) gid: vec3<u32>) {{ if (gid.x != 0u) {{ return; }} {body} }}\n");
            let started = Instant::now();
            let Ok(gpu) = Gpu::new() else {
                eprintln!("PROBE no GPU");
                return;
            };
            let module = gpu
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some(name),
                    source: wgpu::ShaderSource::Wgsl(source.into()),
                });
            let _pipeline = gpu
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(name),
                    layout: None,
                    module: &module,
                    entry_point: Some("probe"),
                    compilation_options: Default::default(),
                    cache: None,
                });
            let _ = gpu.device.poll(wgpu::PollType::Poll);
            eprintln!(
                "PROBE {name}: {} in {:.0} ms",
                if gpu.failed() { "LOST" } else { "ok" },
                started.elapsed().as_secs_f64() * 1000.0
            );
        }
    }

    /// Which of the pre-compiled SPIR-V kernels a driver links, each on a
    /// fresh device (ZOPRO_SPIRV=1 on a Vulkan host). Prints PROBE lines;
    /// run it with --ignored on a new GPU.
    #[test]
    #[ignore]
    fn spirv_kernel_probes() {
        init_logger();
        let g1 = <G1Projective as Curve>::SPIRV.unwrap();
        let g2 = <G2Projective as Curve>::SPIRV.unwrap();
        let names = ["shards_first", "shards_tree", "reduce_blocks"];
        // ZOPRO_PROBE selects probes by a substring of "<curve> <kernel>".
        let filter = std::env::var("ZOPRO_PROBE").unwrap_or_default();
        for (curve, kernels) in [("g1", g1), ("g2", g2)] {
            for (index, bytes) in kernels.iter().enumerate() {
                if !format!("{curve} {}", names[index]).contains(&filter) {
                    continue;
                }
                let Ok(gpu) = Gpu::new() else {
                    eprintln!("PROBE no GPU");
                    return;
                };
                if !gpu.spirv {
                    eprintln!("PROBE no SPIR-V passthrough on {}", gpu.name);
                    return;
                }
                // ZOPRO_SPV_SUFFIX probes `$ZOPRO_SPV_DIR/<kernel><suffix>.spv`
                // files (another glslc build) in place of the built-in ones.
                let file = std::env::var("ZOPRO_SPV_SUFFIX").ok().map(|suffix| {
                    let dir = std::env::var("ZOPRO_SPV_DIR").unwrap_or_else(|_| "spv".into());
                    let g2 = if curve == "g2" { "_g2" } else { "" };
                    std::fs::read(format!("{dir}/{}{g2}{suffix}.spv", names[index]))
                        .expect("spv file")
                });
                let bytes: &[u8] = file.as_deref().unwrap_or(bytes);
                let started = Instant::now();
                let _pipeline = spirv_pipeline(
                    &gpu.device,
                    gpu.cache(),
                    names[index],
                    bytes,
                    msm_slots(index),
                );
                let _ = gpu.device.poll(wgpu::PollType::Poll);
                eprintln!(
                    "PROBE spirv {curve} {}: {} in {:.0} ms",
                    names[index],
                    if gpu.failed() { "LOST" } else { "ok" },
                    started.elapsed().as_secs_f64() * 1000.0
                );
                if !gpu.failed() {
                    gpu.save_cache();
                }
            }
        }
    }

    #[test]
    fn g1_points_match_the_reference() {
        check_points::<G1Config>("xyzz", "Xyzz", "Affine");
    }

    #[test]
    fn g2_points_match_the_reference() {
        check_points::<G2Config>("xyzz2", "Xyzz2", "Affine2");
    }

    #[test]
    fn fq2_ops_match_the_cpu() {
        let source = concat!(
            include_str!("../shaders/field.wgsl"),
            "\n",
            include_str!("../shaders/fp2.wgsl")
        );
        check::<Fq2>(source, "fp2", "Fp2");
    }
}
