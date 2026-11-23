//! G1 and G2 MSMs on an Apple GPU. The host recodes the scalars into signed
//! 15-bit digits and counting-sorts the (base, window) pairs into bucket
//! runs, in parallel over windows; the GPU sums each run into its bucket
//! and reduces each window to its weighted sum; the host combines the
//! windows. Bases are uploaded once, in the kernel's memory layout (eight
//! little-endian words per coordinate, Montgomery form), which is also the
//! shader's.
// objc's msg_send! checks a cfg (feature = "cargo-clippy") that cargo
// does not declare.
#![allow(unexpected_cfgs)]
// Empty elsewhere: Metal is Apple's, and a workspace build on Linux still
// builds this crate.
#![cfg(any(target_os = "macos", target_os = "ios"))]
use std::{ffi::c_void, time::Instant};

use zopro_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
#[cfg(not(metallib))]
use metal::CompileOptions;
use metal::{Buffer, CommandQueue, ComputePipelineState, Device, MTLResourceOptions, MTLSize};
use objc::rc::autoreleasepool;
use objc::{msg_send, sel, sel_impl};
use rayon::prelude::*;

pub mod quotient;
mod shard_order;
mod tensor_fft;
pub use quotient::{Quotient, QuotientDigits, Weights};

// Matching block layout for the host and GPU digit producers.
const DIGIT_CHUNK: usize = 4096;

/// The shaders as a library: the metallib build.rs compiled, else compiled
/// from source now. The G2 kernels are instantiated only when asked for:
/// they take a minute to compile from source (six on an iPhone), and a
/// phone proves G2 on its CPU.
/// Compiled once per process and kernel set: a phone does not cache source
/// compilations the way a Mac does, and a key makes five or six objects
/// from the same library. One system device is assumed.
pub(crate) fn library(device: &Device, g2: bool) -> Result<metal::Library, String> {
    static LIBRARIES: std::sync::Mutex<[Option<Shared<metal::Library>>; 4]> =
        std::sync::Mutex::new([None, None, None, None]);
    let smaller = smaller_code();
    let slot = g2 as usize + 2 * smaller as usize;
    let mut cache = LIBRARIES.lock().unwrap();
    if let Some(library) = &cache[slot] {
        return Ok(library.0.clone());
    }
    let library = compile(device, g2, smaller)?;
    cache[slot] = Some(Shared(library.clone()));
    Ok(library)
}

/// A kernel's pipeline state, built once per process, library and level:
/// building one per key left the driver's copies mapped into the process,
/// tens of MB more with every key load.
pub(crate) fn pipeline(
    device: &Device,
    g2: bool,
    name: &str,
) -> Result<metal::ComputePipelineState, String> {
    type Pipelines =
        std::collections::HashMap<(String, bool, bool), Shared<metal::ComputePipelineState>>;
    static PIPELINES: std::sync::Mutex<Option<Pipelines>> = std::sync::Mutex::new(None);
    let key = (name.to_string(), g2, smaller_code());
    if let Some(found) = PIPELINES
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|cache| cache.get(&key))
    {
        return Ok(found.0.clone());
    }
    let library = library(device, g2)?;
    let function = library
        .get_function(name, None)
        .map_err(|e| format!("{name}: {e}"))?;
    let state = device
        .new_compute_pipeline_state_with_function(&function)
        .map_err(|e| format!("{name}: {e}"))?;
    PIPELINES
        .lock()
        .unwrap()
        .get_or_insert_with(Default::default)
        .insert(key, Shared(state.clone()));
    Ok(state)
}

fn fused_g1_enabled() -> bool {
    cfg!(all(target_os = "macos", target_arch = "aarch64"))
        && std::env::var("ZOPRO_METAL_FUSED_MIXED").as_deref() != Ok("0")
}

/// Compiles the library and builds the G1 MSM and quotient pipelines, so
/// that the first key load does not: an app can call it at start.
pub fn warm() -> Result<(), String> {
    autoreleasepool(|| {
        let device = Device::system_default().ok_or("no Metal device")?;
        if fused_g1_enabled() {
            let _ = pipeline(&device, false, "shards_first_g1_fused");
        }
        for name in [
            G1Projective::FIRST,
            G1Projective::TREE,
            G1Projective::REDUCE,
            "fr_dif_pass",
            "fr_dit_pass",
            "fr_scale",
            "fr_product",
        ] {
            pipeline(&device, false, name)?;
        }
        Ok(())
    })
}

/// Whether the shaders compile at Metal's size optimization level rather
/// than the default: aeglos found the default level corrupting a kernel,
/// and two older iPhones' GPU MSMs disagreed with the CPU where a newer
/// one's agreed. A host switches after such a mismatch; ZOPRO_METAL_SIZE=1
/// switches from the start. Libraries are cached per level.
static SMALLER_CODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Switches to the size level for the libraries compiled from now on; false
/// when it was already on, so a host does not retry forever.
pub fn prefer_smaller_code() -> bool {
    !SMALLER_CODE.swap(true, std::sync::atomic::Ordering::SeqCst)
}

pub fn smaller_code() -> bool {
    SMALLER_CODE.load(std::sync::atomic::Ordering::SeqCst)
        || std::env::var("ZOPRO_METAL_SIZE").is_ok_and(|v| v == "1")
}

/// One command queue per process, shared by every MSM and quotient of
/// every key: its command buffers execute in order, and an iPhone warned
/// of forty open queues when each object had its own.
pub(crate) fn queue(device: &Device) -> CommandQueue {
    static QUEUE: std::sync::Mutex<Option<Shared<CommandQueue>>> = std::sync::Mutex::new(None);
    let mut cache = QUEUE.lock().unwrap();
    if let Some(queue) = &*cache {
        return queue.0.clone();
    }
    let queue = device.new_command_queue();
    *cache = Some(Shared(queue.clone()));
    queue
}

/// A completed command buffer, or the error that aborted it: an iPhone 15
/// under memory pressure aborted ours ("Internal Error") and the proof
/// built from the stale sums was wrong.
pub(crate) fn completed(command: &metal::CommandBufferRef) -> Result<(), String> {
    command.wait_until_completed();
    match command.status() {
        metal::MTLCommandBufferStatus::Completed => Ok(()),
        status => Err(format!("GPU command buffer {status:?}")),
    }
}

/// Whether the GPU is of Apple's ninth family or later (A19, M-series):
/// there its G2 MSM beats the CPU's by several times, where an A14's or
/// A16's GPU does not.
pub fn fast_gpu() -> bool {
    autoreleasepool(|| {
        Device::system_default()
            .is_some_and(|device| device.supports_family(metal::MTLGPUFamily::Apple9))
    })
}

/// What the Metal device has allocated for this process, in bytes.
pub fn allocated_bytes() -> u64 {
    autoreleasepool(|| Device::system_default().map_or(0, |device| device.current_allocated_size()))
}

/// A Metal object held across threads: Metal's objects are thread-safe,
/// metal-rs just does not say so.
struct Shared<T>(T);
unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}

fn compile(device: &Device, g2: bool, smaller: bool) -> Result<metal::Library, String> {
    // ZOPRO_METAL_SHADERS=<dir>: the shaders from that directory, compiled
    // now, for trying kernel changes without a build (tools/kernel-lab).
    if let Some(dir) = std::env::var_os("ZOPRO_METAL_SHADERS") {
        let dir = std::path::Path::new(&dir);
        let mut source = String::new();
        for name in ["field", "curve", "buckets", "quotient"] {
            let file = dir.join(format!("{name}.metal"));
            source.push_str(
                &std::fs::read_to_string(&file)
                    .map_err(|e| format!("{}: {e}", file.display()))?,
            );
            source.push('\n');
        }
        if g2 {
            source.push_str("BUCKET_KERNELS(fp2_t, g2)\n");
        }
        let options = metal::CompileOptions::new();
        if smaller {
            let _: () = unsafe { objc::msg_send![&*options, setOptimizationLevel: 1i64] };
        }
        return device
            .new_library_with_source(&source, &options)
            .map_err(|e| format!("shader: {e}"));
    }
    #[cfg(metallib)]
    {
        let _ = g2;
        let bytes: &[u8] = if smaller {
            include_bytes!(concat!(env!("OUT_DIR"), "/zopro-size.metallib"))
        } else {
            include_bytes!(concat!(env!("OUT_DIR"), "/zopro.metallib"))
        };
        device
            .new_library_with_data(bytes)
            .map_err(|e| format!("metallib: {e}"))
    }
    #[cfg(not(metallib))]
    {
        let mut source = SHADER.to_string();
        if !smaller {
            source.push_str(RADIX29_SHADERS);
        }
        if g2 {
            source.push_str("\nBUCKET_KERNELS(fp2_t, g2)\n");
        }
        let options = CompileOptions::new();
        if smaller {
            // MTLLibraryOptimizationLevelSize; metal-rs has no setter for it.
            let _: () = unsafe { msg_send![&*options, setOptimizationLevel: 1i64] };
        }
        device
            .new_library_with_source(&source, &options)
            .map_err(|e| format!("shader: {e}"))
    }
}

#[cfg_attr(metallib, allow(dead_code))]
pub(crate) const SHADER: &str = concat!(
    include_str!("../shaders/field.metal"),
    "\n",
    include_str!("../shaders/curve.metal"),
    "\n",
    include_str!("../shaders/buckets.metal"),
    "\n",
    include_str!("../shaders/quotient.metal"),
);

#[cfg(not(metallib))]
const RADIX29_SHADERS: &str = concat!(
    "\n",
    include_str!("../shaders/radix29.metal"),
    "\n",
    include_str!("../shaders/radix29_points.metal"),
);
/// Window width by point count, as the CPU MSM's: wider windows mean fewer
/// bucket additions and more buckets to reduce (and, here, to zero).
fn width_for(n: usize, default_cap: usize) -> usize {
    let width = match n {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        _ => 15,
    };
    // Capped at 13 bits (ZOPRO_GPU_WIDTH overrides, at most 15: digits are
    // i16): a quarter of 15 bits' buckets to reduce and sort for 18 % more
    // additions. On an M5 Pro whole proofs went 202 → 154 ms at 8 → 2 and
    // 444 → 395 at 24 → 2; on an iPhone 12 and 15 the 15-bit reduction was
    // 28 % of the GPU's MSM time.
    let cap = std::env::var("ZOPRO_GPU_WIDTH")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default_cap)
        .clamp(7, 15);
    width.min(cap)
}
/// Threads per window in the reduction: each takes half / this many buckets.
const REDUCE_THREADS_PER_WINDOW: usize = 512;
/// Threads per threadgroup in the reduction; each threadgroup leaves one
/// partial sum for the host to add.
const REDUCE_GROUP: usize = 256;
/// The threadgroup memory reduction's most threads per group
/// (`REDUCE_TG` in buckets.metal).
const REDUCE_TG_GROUP: usize = 64;
/// Shard length. Witness scalars put most points in a few buckets, whose
/// long runs are what the shards split; long shards there starve the GPU of
/// threads (128 over base points left 24x2's G2 MSM at 227 ms, 32 at 151).
const FIRST_SHARD: usize = 32;
const SHARD: usize = 32;
/// The most a shard tree takes, in MB (ZOPRO_METAL_TREE_MB sets it): the
/// windows go through it in chunks, and the key's two trees count against an
/// iPhone's memory limit. Chunks under 8 MB rather than 64 cost no time on an
/// iPad Pro (A12Z) or an M5 Pro and took 16 -> 2's peak from 765 to 673 MB on
/// the iPad, 24 -> 2's from 1362 to 1226 on the M5 Pro.
const TREE_MB: usize = 8;

/// Stage times of one MSM, in milliseconds: the host sort and planning,
/// the GPU from submission to completion, the host combination.
#[derive(Debug, Clone, Copy, Default)]
pub struct Timings {
    pub sort_ms: f64,
    pub gpu_ms: f64,
    pub combine_ms: f64,
}

/// An MSM's scalars sorted on the host, not yet submitted.
pub struct Sorted {
    started: Instant,
    bases: Option<Buffer>,
}

/// An MSM submitted to the GPU and not yet collected.
pub struct Pending {
    command: metal::CommandBuffer,
    host_prefix: Option<(metal::CommandBuffer, usize)>,
    submitted: Instant,
    timings: Timings,
}

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
    const G2: bool;
    const FIRST: &'static str;
    const TREE: &'static str;
    const REDUCE: &'static str;
    /// The reduction without SIMD-group functions (`reduce_tg` in
    /// buckets.metal), for GPUs without them (an A12's).
    const REDUCE_TG: &'static str;
    /// The affine point as the kernel holds it, the identity as zeros.
    fn encode(point: &Self::Affine, out: &mut Vec<u8>);
    /// An xyzz point (x, y, zz, zzz) as a Jacobian one (x zz, y zzz, zz).
    fn xyzz_to_projective(bytes: &[u8]) -> Self;
    #[doc(hidden)]
    fn reduce_host_window(buckets: &[u8]) -> Self {
        reduce_host_projective::<Self>(buckets)
    }

}

fn reduce_host_projective<C: Curve>(buckets: &[u8]) -> C {
    let mut running = C::IDENTITY;
    let mut sum = C::IDENTITY;
    for point in buckets.chunks_exact(C::XYZZ_BYTES).rev() {
        running += C::xyzz_to_projective(point);
        sum += running;
    }
    sum
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
    const G2: bool = false;
    const FIRST: &'static str = "shards_first_g1";
    const TREE: &'static str = "shards_tree_g1";
    const REDUCE: &'static str = "reduce_g1";
    const REDUCE_TG: &'static str = "reduce_tg_g1";
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

    #[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "aarch64"))]
    fn reduce_host_window(buckets: &[u8]) -> Self {
        #[cfg(target_os = "ios")]
        {
            static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
            if !*ENABLED.get_or_init(fast_gpu) {
                return reduce_host_projective::<Self>(buckets);
            }
        }
        // Keep the GPU's coordinates through the weighted sum.
        let mut running = HostBucket::ZERO;
        let mut sum = HostBucket::ZERO;
        for bytes in buckets.chunks_exact(Self::XYZZ_BYTES).rev() {
            let point = HostBucket {
                x: fq(&bytes[..32]),
                y: fq(&bytes[32..64]),
                zz: fq(&bytes[64..96]),
                zzz: fq(&bytes[96..]),
            };
            running = running.add(point);
            sum = sum.add(running);
        }
        sum.projective()
    }
}

#[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "aarch64"))]
#[derive(Clone, Copy)]
struct HostBucket {
    x: Fq,
    y: Fq,
    zz: Fq,
    zzz: Fq,
}

#[cfg(all(any(target_os = "macos", target_os = "ios"), target_arch = "aarch64"))]
impl HostBucket {
    const ZERO: Self = Self {
        x: Fq::ZERO,
        y: Fq::ZERO,
        zz: Fq::ZERO,
        zzz: Fq::ZERO,
    };

    fn projective(self) -> G1Projective {
        G1Projective::new(self.x * self.zz, self.y * self.zzz, self.zz)
    }

    fn add(self, other: Self) -> Self {
        if self.zz.is_zero() {
            return other;
        }
        if other.zz.is_zero() {
            return self;
        }
        let u = self.x * other.zz;
        let s = self.y * other.zzz;
        let dx = other.x * self.zz - u;
        let dy = other.y * self.zzz - s;
        if dx.is_zero() {
            if !dy.is_zero() {
                return Self::ZERO;
            }
            let d = self.projective().double();
            let zz = d.z.square();
            return Self {
                x: d.x,
                y: d.y,
                zz,
                zzz: zz * d.z,
            };
        }
        let pp = dx.square();
        let ppp = dx * pp;
        let q = u * pp;
        let x = dy.square() - ppp - q - q;
        Self {
            x,
            y: (q - x) * dy - s * ppp,
            zz: (self.zz * other.zz) * pp,
            zzz: (self.zzz * other.zzz) * ppp,
        }
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
    const G2: bool = true;
    const FIRST: &'static str = "shards_first_g2";
    const TREE: &'static str = "shards_tree_g2";
    const REDUCE: &'static str = "reduce_g2";
    const REDUCE_TG: &'static str = "reduce_tg_g2";
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

pub struct Msm<C: Curve> {
    width: usize,
    half: usize,
    windows: usize,
    device: Device,
    queue: CommandQueue,
    first: ComputePipelineState,
    shard_order: Option<shard_order::ShardOrder>,
    tree_pipeline: ComputePipelineState,
    /// The window reduction on the GPU; none where neither kernel builds
    /// (G2 on an A12Z exceeds the shader stack), and the host reduces
    /// `host_buckets`, the buckets this MSM writes directly.
    reduce: Option<ComputePipelineState>,
    host_buckets: Option<Buffer>,
    host_tail: Option<HostBucketTail>,
    bases: Buffer,
    n: usize,
    sorted: Buffer,
    sorted_for_original_bases: bool,
    /// Windows per chunk: the shard trees are sized for one chunk and
    /// reused, which bounds the GPU memory a wide key takes.
    chunk_windows: usize,
    first_shard: usize,
    /// Per chunk and level: shard starts, counts and destinations.
    levels: Vec<Vec<[Buffer; 3]>>,
    /// The shard trees and the buckets, shared with the key's other MSMs.
    scratch: std::sync::Arc<Scratch>,
    bucket_bytes: usize,
    /// The reduction's partial sums, `groups` per window.
    sums: Buffer,
    /// The reduction's shape: buckets per thread, threads per window, threads
    /// per threadgroup, threadgroups per window.
    reduce_shape: [usize; 4],
    /// Host copies of the bucket runs, for planning the levels.
    counts: Vec<u32>,
    starts: Vec<u32>,
    _curve: std::marker::PhantomData<C>,
}

struct HostBucketTail {
    first_window: usize,
    buffer: Buffer,
}

fn shared(device: &metal::DeviceRef, bytes: usize) -> Buffer {
    device.new_buffer(bytes.max(16) as u64, MTLResourceOptions::StorageModeShared)
}

/// Gives a buffer's memory back as it goes: released alone, the driver kept
/// it pooled and charged to the process (1.2 GB on a Mac after a few key
/// loads), which an iPhone counts against its memory limit.
pub(crate) fn discard(buffer: &Buffer) {
    buffer.set_purgeable_state(metal::MTLPurgeableState::Empty);
}

/// Buffers a key's MSMs share: the shard trees and the buckets. The host
/// never writes them and command buffers on the one queue use them in
/// turn, so one set per key does instead of one per MSM (four G1 MSMs'
/// trees were 512 MB at the widest key, too much for a 4 GB phone).
pub struct Scratch(std::sync::Mutex<ScratchBuffers>);
struct ScratchBuffers {
    tree: [Buffer; 2],
    buckets: Buffer,
}
impl Scratch {
    pub fn new() -> Result<std::sync::Arc<Self>, String> {
        autoreleasepool(Self::new_in_pool)
    }

    fn new_in_pool() -> Result<std::sync::Arc<Self>, String> {
        let device = Device::system_default().ok_or("no Metal device")?;
        let buffers = ScratchBuffers {
            tree: [shared(&device, 16), shared(&device, 16)],
            buckets: shared(&device, 16),
        };
        Ok(std::sync::Arc::new(Self(std::sync::Mutex::new(buffers))))
    }
    /// Room for an MSM's trees and buckets. A command buffer in flight keeps
    /// the buffers it was encoded with, so growing them is safe.
    fn reserve(&self, device: &Device, tree_bytes: usize, bucket_bytes: usize) {
        let mut buffers = self.0.lock().unwrap();
        for tree in buffers.tree.iter_mut() {
            if (tree.length() as usize) < tree_bytes {
                discard(tree);
                *tree = shared(device, tree_bytes);
            }
        }
        if (buffers.buckets.length() as usize) < bucket_bytes {
            discard(&buffers.buckets);
            buffers.buckets = shared(device, bucket_bytes);
        }
    }
}

impl Drop for ScratchBuffers {
    fn drop(&mut self) {
        autoreleasepool(|| {
            self.tree.iter().for_each(discard);
            discard(&self.buckets);
        })
    }
}

impl<C: Curve> Drop for Msm<C> {
    fn drop(&mut self) {
        autoreleasepool(|| {
            for buffer in [&self.bases, &self.sorted, &self.sums] {
                discard(buffer);
            }
            if let Some(host) = &self.host_buckets {
                discard(host);
            }
            if let Some(tail) = &self.host_tail {
                discard(&tail.buffer);
            }
            self.levels.iter().flatten().flatten().for_each(discard);
        })
    }
}

impl G1Msm {
    /// Overlap weighted bucket sums on the CPU with the final GPU windows.
    #[doc(hidden)]
    pub fn overlap_host_reduction(&mut self) {
        if cfg!(target_os = "ios")
            && self.n >= 1 << 20
            && self.width == 15
            && self.device.supports_family(metal::MTLGPUFamily::Apple9)
            && std::env::var("ZOPRO_METAL_EULER_OVERLAP_REDUCE").as_deref() != Ok("0")
        {
            self.split_host_buckets();
        }
    }

    /// Use radix-29 arithmetic for the first level of dense G1 shard sums.
    /// Keep the selected pipeline when the alternate shader is unavailable.
    pub fn select_radix29_shards(&mut self) -> bool {
        if smaller_code() {
            return false;
        }
        let Ok(first) = pipeline(&self.device, false, "shards_first_g1_radix29") else {
            return false;
        };
        self.first = first;
        if cfg!(target_os = "ios")
            && ((1 << 20)..=(1 << 21)).contains(&self.n)
            && self.device.supports_family(metal::MTLGPUFamily::Apple9)
        {
            let capacity = self.chunk_windows * (self.n.div_ceil(self.first_shard) + self.half);
            self.shard_order = shard_order::ShardOrder::new(&self.device, self.n, capacity).ok();
        }
        true
    }
}

impl<C: Curve> Msm<C> {
    /// Compiles the kernels and uploads the bases; `compile_ms` is how long
    /// the shader compilation took.
    pub fn new(bases: &[C::Affine]) -> Result<(Self, f64), String> {
        Self::new_in(bases, Scratch::new()?)
    }

    /// As `new`, with the trees and buckets shared with other MSMs.
    pub fn new_in(
        bases: &[C::Affine],
        scratch: std::sync::Arc<Scratch>,
    ) -> Result<(Self, f64), String> {
        Self::new_in_with_width_cap(bases, scratch, 13)
    }

    /// As `new_in`, with a caller-specified, overridable default window cap.
    #[doc(hidden)]
    pub fn new_in_with_width_cap(
        bases: &[C::Affine],
        scratch: std::sync::Arc<Scratch>,
        default_cap: usize,
    ) -> Result<(Self, f64), String> {
        Self::new_in_with_tree_budget(bases, scratch, default_cap, None)
    }

    /// Override the default scratch budget per tree in MiB, up to 32 MiB.
    #[doc(hidden)]
    pub fn new_in_with_tree_budget(
        bases: &[C::Affine],
        scratch: std::sync::Arc<Scratch>,
        default_cap: usize,
        default_tree_mib: Option<usize>,
    ) -> Result<(Self, f64), String> {
        // Building a pipeline autoreleases objects that hold on to the
        // device's memory; a thread without a pool leaked them per key.
        autoreleasepool(|| Self::new_in_pool(bases, scratch, default_cap, default_tree_mib))
    }

    fn new_in_pool(
        bases: &[C::Affine],
        scratch: std::sync::Arc<Scratch>,
        default_cap: usize,
        default_tree_mib: Option<usize>,
    ) -> Result<(Self, f64), String> {
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let pipeline = |name: &str| pipeline(&device, C::G2, name);
        let first = if bases.len() >= (1 << 15) && C::FIRST == G1Projective::FIRST
            && fused_g1_enabled()
        {
            pipeline("shards_first_g1_fused").or_else(|_| pipeline(C::FIRST))?
        } else {
            pipeline(C::FIRST)?
        };
        let tree_pipeline = pipeline(C::TREE)?;
        // The windows reduce on the host on iOS and Apple silicon Macs.
        // A GPU reduction kernel holds spilled registers in memory the driver charges
        // to the app from its first run. With the four-query prover on M5
        // Pro, host reduction cut both latency and footprint on 2x2 and 12x2.
        // ZOPRO_METAL_HOST_REDUCE=1 or 0 says where anywhere.
        let host = std::env::var("ZOPRO_METAL_HOST_REDUCE").map_or(
            cfg!(any(
                target_os = "ios",
                all(target_os = "macos", target_arch = "aarch64")
            )),
            |v| v == "1",
        );
        // SIMD-group shuffles need an A13's GPU or later
        // (ZOPRO_METAL_TG_REDUCE=1 takes the fallback anywhere).
        let forced = std::env::var("ZOPRO_METAL_TG_REDUCE").is_ok_and(|v| v == "1");
        let shuffles = if host {
            Err("host reduction".to_string())
        } else if forced {
            Err("ZOPRO_METAL_TG_REDUCE=1".to_string())
        } else {
            pipeline(C::REDUCE)
        };
        let (reduce, tree_reduce) = match shuffles {
            _ if host => (None, false),
            Ok(reduce) => (Some(reduce), false),
            Err(e) => match pipeline(C::REDUCE_TG) {
                Ok(reduce) => {
                    log::info!("zopro metal: {e}; reducing through threadgroup memory");
                    (Some(reduce), true)
                }
                Err(e) => {
                    log::info!("zopro metal: {e}; reducing on the host");
                    (None, false)
                }
            },
        };
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let bases_buffer = Self::upload_bases(&device, bases);
        let n = bases.len();
        let width = width_for(n, default_cap);
        let half = 1 << (width - 1);
        let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
        let buckets = windows * half;
        // The most shards a level can have for one window: every pair in
        // its own first shard, or every bucket's run split, whichever is
        // more. Chunks of windows keep each tree under TREE_MB.
        let dense = cfg!(all(target_os = "macos", target_arch = "aarch64"))
            && !C::G2 && n >= (1 << 18);
        // Shorter first shards let more threads cooperate on a large
        // G1 bucket. The tree budget still bounds the resulting scratch.
        let first_shard = if dense { 22 } else { FIRST_SHARD };
        let per_window = n.div_ceil(first_shard) + half;
        let default_tree_mb = default_tree_mib.map_or(
            if dense { 24 } else { TREE_MB },
            |budget| budget.clamp(TREE_MB, 32),
        );
        let tree_cap = std::env::var("ZOPRO_METAL_TREE_MB")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_tree_mb)
            << 20;
        let chunk_windows = (tree_cap / (per_window * C::XYZZ_BYTES)).clamp(1, windows);
        let most = chunk_windows * per_window;
        let bucket_bytes = buckets * C::XYZZ_BYTES;
        let shared_bucket_bytes = if reduce.is_some() { bucket_bytes } else { 0 };
        scratch.reserve(&device, most * C::XYZZ_BYTES, shared_bucket_bytes);
        let reduce_shape = {
            let per_window = half.min(REDUCE_THREADS_PER_WINDOW);
            let allowed = reduce.as_ref().map_or(REDUCE_GROUP, |r| {
                r.max_total_threads_per_threadgroup() as usize
            });
            let allowed = 1usize << (usize::BITS - 1 - allowed.max(1).leading_zeros());
            let most = if tree_reduce {
                REDUCE_TG_GROUP
            } else {
                REDUCE_GROUP
            };
            let group = per_window.min(most).min(allowed);
            [half / per_window, per_window, group, per_window / group]
        };
        Ok((
            Self {
                width,
                half,
                windows,
                chunk_windows,
                first_shard,
                queue: queue(&device),
                sorted: shared(&device, n * windows * 4),
                sorted_for_original_bases: false,
                levels: Vec::new(),
                scratch,
                bucket_bytes,
                sums: shared(&device, windows * reduce_shape[3] * C::XYZZ_BYTES),
                reduce_shape,
                counts: vec![0; buckets],
                starts: vec![0; buckets],
                _curve: std::marker::PhantomData,
                first,
                shard_order: None,
                tree_pipeline,
                host_buckets: reduce.is_none().then(|| shared(&device, bucket_bytes)),
                host_tail: None,
                reduce,
                bases: bases_buffer,
                n,
                device,
            },
            compile_ms,
        ))
    }

    fn split_host_buckets(&mut self) {
        if self.host_tail.is_some() {
            return;
        }
        let Some(host) = &mut self.host_buckets else {
            return;
        };
        let chunks = self.windows.div_ceil(self.chunk_windows);
        let first_window = chunks.saturating_sub(4) * self.chunk_windows;
        if first_window == 0 {
            return;
        }
        let window_bytes = self.half * C::XYZZ_BYTES;
        discard(host);
        *host = shared(&self.device, first_window * window_bytes);
        self.host_tail = Some(HostBucketTail {
            first_window,
            buffer: shared(&self.device, (self.windows - first_window) * window_bytes),
        });
    }

    pub fn msm(&mut self, scalars: &[Fr]) -> Result<(C, Timings), String> {
        let pending = self.start(scalars);
        self.finish(pending)
    }

    /// Sorts, plans and submits the GPU work, without waiting: the host can
    /// sort the next MSM while this one runs.
    pub fn start(&mut self, scalars: &[Fr]) -> Pending {
        assert_eq!(scalars.len(), self.n);
        let started = Instant::now();
        self.sort(scalars);
        self.submit(started)
    }

    pub fn scalar_window_width(&self) -> usize {
        self.width
    }

    /// Plan the MSM from its GPU histogram, or sort the recoded digits on
    /// the host if the quotient only produced digits.
    pub fn start_quotient_digits(&mut self, digits: &QuotientDigits) -> Result<Pending, String> {
        assert_eq!((digits.n, digits.width), (self.n, self.width));
        let started = Instant::now();
        if let Some(scatter) = &digits.scatter {
            // SAFETY: the quotient command finished before returning these
            // digits. The histogram contains one counter per bucket and a
            // final counter for invalid digits.
            let counts = unsafe {
                std::slice::from_raw_parts(
                    scatter.counts.contents().cast::<u32>(),
                    self.counts.len() + 1,
                )
            };
            let (&invalid, counts) = counts.split_last().unwrap();
            if invalid != 0 {
                return Err("MSM digit exceeds its window".into());
            }
            self.counts.copy_from_slice(counts);
            for (w, (starts, counts)) in self
                .starts
                .chunks_mut(self.half)
                .zip(counts.chunks(self.half))
                .enumerate()
            {
                let mut at = 0usize;
                for (start, &count) in starts.iter_mut().zip(counts) {
                    *start = (w * self.n + at) as u32;
                    at = at.checked_add(count as usize).ok_or("invalid MSM histogram")?;
                    if at > self.n {
                        return Err("invalid MSM histogram".into());
                    }
                }
            }
            self.sorted_for_original_bases = true;
            return Ok(autoreleasepool(|| self.submit_in_pool(started, None, Some(digits))));
        }
        self.scatter_digits(self.n, digits.as_slice(), DIGIT_CHUNK)?;
        self.sorted_for_original_bases = true;
        Ok(self.submit(started))
    }

    /// The first half of [`Msm::start`]: the scalars sorted, for
    /// [`Msm::submit_sorted`] once the GPU should take them.
    pub fn sort_now(&mut self, scalars: &[Fr]) -> Sorted {
        assert_eq!(scalars.len(), self.n);
        let started = Instant::now();
        self.sort(scalars);
        Sorted { started, bases: None }
    }

    /// Start an MSM with a compact list of points and canonical scalars.
    /// The original bases and the size of this MSM's scratch stay intact.
    pub fn start_grouped(&mut self, bases: &[C::Affine], scalars: &[[u64; 4]]) -> Pending {
        let sorted = self.sort_grouped(bases, scalars);
        self.submit_sorted(sorted)
    }

    /// Prepare compact bases and scalars for submission by this MSM.
    pub fn sort_grouped(&mut self, bases: &[C::Affine], scalars: &[[u64; 4]]) -> Sorted {
        let n = bases.len();
        assert!(n != 0 && n <= self.n);
        assert_eq!(n, scalars.len());
        let started = Instant::now();
        let buffer = Self::upload_bases(&self.device, bases);
        self.sort_words(scalars, |words| *words);
        self.sorted_for_original_bases = false;
        Sorted { started, bases: Some(buffer) }
    }

    fn upload_bases(device: &Device, bases: &[C::Affine]) -> Buffer {
        // Encode a block at a time so loading a key needs no full host copy.
        let buffer = shared(device, bases.len() * C::AFFINE_BYTES);
        let mut bytes = Vec::with_capacity(4096 * C::AFFINE_BYTES);
        for (i, block) in bases.chunks(4096).enumerate() {
            bytes.clear();
            for point in block {
                C::encode(point, &mut bytes);
            }
            // SAFETY: this is a fresh shared buffer with space for all points.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    (buffer.contents() as *mut u8).add(i * 4096 * C::AFFINE_BYTES),
                    bytes.len(),
                );
            }
        }
        buffer
    }

    pub fn submit_sorted(&mut self, sorted: Sorted) -> Pending {
        if let Some(bases) = sorted.bases {
            // Metal's retained command buffer owns its input until completion.
            autoreleasepool(|| self.submit_in_pool(sorted.started, Some(&bases), None))
        } else {
            self.submit(sorted.started)
        }
    }

    /// The MSM over the scalars another MSM of the same length just sorted
    /// (G1 B and G2 B share theirs): its sorted runs, no sort.
    pub fn start_sorted_as<D: Curve>(&mut self, other: &Msm<D>) -> Pending {
        assert_eq!((other.n, other.width), (self.n, self.width));
        assert!(other.sorted_for_original_bases);
        let started = Instant::now();
        self.sorted = other.sorted.clone();
        self.counts.clone_from(&other.counts);
        self.starts.clone_from(&other.starts);
        self.sorted_for_original_bases = true;
        self.submit(started)
    }

    fn submit(&mut self, started: Instant) -> Pending {
        // Command buffers and encoders are autoreleased, and a Rust thread
        // has no pool: without one each proof leaked them, and with them
        // every buffer of the key, until an iPhone's GPU ran out.
        autoreleasepool(|| self.submit_in_pool(started, None, None))
    }

    fn submit_in_pool(
        &mut self,
        started: Instant,
        bases: Option<&Buffer>,
        digits: Option<&QuotientDigits>,
    ) -> Pending {
        let chunks: Vec<Vec<usize>> = (0..self.windows.div_ceil(self.chunk_windows))
            .map(|chunk| {
                let first = chunk * self.chunk_windows;
                self.plan(chunk, first..(first + self.chunk_windows).min(self.windows))
            })
            .collect();
        let sort_ms = elapsed(started);
        let mut command = self.queue.new_command_buffer();
        let scratch = self.scratch.0.lock().unwrap();
        // The host reads its own buckets after completion, while the next
        // command may already be using the shared trees.
        let buckets = self.host_buckets.as_ref().unwrap_or(&scratch.buckets);
        let mut output_buckets = buckets;
        // Empty buckets stay at infinity.
        let blit = command.new_blit_command_encoder();
        let bytes = self.host_buckets.as_ref().map_or(self.bucket_bytes as u64, |host| host.length());
        blit.fill_buffer(buckets, metal::NSRange::new(0, bytes), 0);
        let scatter = digits.and_then(|digits| {
            let scatter = digits.scatter.as_ref()?;
            let bytes = self.starts.len() * 4;
            let starts = self.device.new_buffer_with_data(
                self.starts.as_ptr().cast(),
                bytes as u64,
                MTLResourceOptions::StorageModeShared,
            );
            let positions = shared(&self.device, bytes);
            blit.fill_buffer(&positions, metal::NSRange::new(0, bytes as u64), 0);
            Some((scatter, digits, starts, positions))
        });
        blit.end_encoding();
        let mut encoder = command.new_compute_command_encoder();
        if let Some((scatter, digits, starts, positions)) = scatter {
            scatter.encode(encoder, digits, &starts, &positions, &self.sorted);
        }
        if self.shard_order.is_some() {
            log::info!("ZOPRO_TRACE metal-ordered-shards {}", shard_order::BAND_SHIFT);
        }
        // Chunks run in order and share the trees; the dispatches of one
        // are complete before the next begins.
        let mut host_prefix = None;
        for (chunk, levels) in chunks.iter().enumerate() {
            if let Some(tail) = self.host_tail.as_ref().filter(|tail| chunk * self.chunk_windows == tail.first_window) {
                encoder.end_encoding();
                // Only the completed prefix buffer is exposed to the CPU.
                // The tail uses a separate shared Metal resource.
                command.commit();
                host_prefix = Some((command.to_owned(), tail.first_window));
                command = self.queue.new_command_buffer();
                output_buckets = &tail.buffer;
                let blit = command.new_blit_command_encoder();
                blit.fill_buffer(output_buckets, metal::NSRange::new(0, output_buckets.length()), 0);
                blit.end_encoding();
                encoder = command.new_compute_command_encoder();
            }
            let last = levels.len() - 1;
            for (level, shards) in levels.iter().enumerate() {
                if *shards == 0 {
                    continue;
                }
                let [start, count, dst] = &self.levels[chunk][level];
                let pipeline = if level == 0 {
                    if let Some(order) = &self.shard_order {
                        // Reorder only first-level work. Later levels still
                        // consume those sums at their original destinations.
                        let indices = order.encode(encoder, &self.sorted, start, *shards);
                        encoder.set_buffer(7, Some(indices), 0);
                        &order.first
                    } else {
                        &self.first
                    }
                } else {
                    &self.tree_pipeline
                };
                encoder.set_compute_pipeline_state(pipeline);
                if level == 0 {
                    encoder.set_buffer(0, Some(bases.unwrap_or(&self.bases)), 0);
                    encoder.set_buffer(1, Some(&self.sorted), 0);
                } else {
                    encoder.set_buffer(0, Some(&scratch.tree[(level - 1) % 2]), 0);
                }
                encoder.set_buffer(2, Some(start), 0);
                encoder.set_buffer(3, Some(count), 0);
                encoder.set_buffer(4, Some(dst), 0);
                let out = if level == last {
                    output_buckets
                } else {
                    &scratch.tree[level % 2]
                };
                encoder.set_buffer(5, Some(out), 0);
                let n = *shards as u32;
                encoder.set_bytes(6, 4, &n as *const u32 as *const c_void);
                let group = pipeline.thread_execution_width().min(n as u64).max(1);
                encoder.dispatch_threads(MTLSize::new(n as u64, 1, 1), MTLSize::new(group, 1, 1));
            }
        }
        // One threadgroup per window, a thread per `leaf` buckets, at least
        // a full SIMD group and no more than the kernel's registers allow on
        // this GPU (a power of two, since `leaf` is a shift).
        if self.reduce.is_some() {
            self.encode_reduce(encoder, buckets);
        }
        encoder.end_encoding();
        command.commit();
        Pending {
            command: command.to_owned(),
            host_prefix,
            submitted: Instant::now(),
            timings: Timings {
                sort_ms,
                ..Timings::default()
            },
        }
    }

    /// The GPU time of each stage of an MSM, in ms, each dispatch in a
    /// command buffer of its own (the bench's profile): the first level of
    /// shards (over the bases), the tree levels, and the window reduction;
    /// plus each pipeline's thread limit, which falls as registers grow.
    pub fn profile(&mut self, scalars: &[Fr]) -> String {
        autoreleasepool(|| {
            self.sort(scalars);
            let chunks: Vec<Vec<usize>> = (0..self.windows.div_ceil(self.chunk_windows))
                .map(|chunk| {
                    let first = chunk * self.chunk_windows;
                    self.plan(chunk, first..(first + self.chunk_windows).min(self.windows))
                })
                .collect();
            let scratch = self.scratch.0.lock().unwrap();
            let buckets = self.host_buckets.as_ref().unwrap_or(&scratch.buckets);
            let run = |encode: &dyn Fn(&metal::ComputeCommandEncoderRef)| -> f64 {
                let command = self.queue.new_command_buffer();
                let encoder = command.new_compute_command_encoder();
                encode(encoder);
                encoder.end_encoding();
                command.commit();
                command.wait_until_completed();
                // SAFETY: GPUStartTime/GPUEndTime are CFTimeInterval
                // properties of a completed command buffer.
                let (start, end): (f64, f64) = unsafe {
                    (
                        msg_send![command, GPUStartTime],
                        msg_send![command, GPUEndTime],
                    )
                };
                (end - start) * 1000.0
            };
            let (mut first_ms, mut tree_ms, mut first_shards, mut tree_shards) =
                (0.0, 0.0, 0usize, 0usize);
            for (chunk, levels) in chunks.iter().enumerate() {
                let output_buckets = self.host_tail.as_ref()
                    .filter(|tail| chunk * self.chunk_windows >= tail.first_window)
                    .map_or(buckets, |tail| &tail.buffer);
                let last = levels.len() - 1;
                for (level, shards) in levels.iter().enumerate() {
                    let pipeline = if level == 0 {
                        &self.first
                    } else {
                        &self.tree_pipeline
                    };
                    let ms = run(&|encoder| {
                        encoder.set_compute_pipeline_state(pipeline);
                        if level == 0 {
                            encoder.set_buffer(0, Some(&self.bases), 0);
                            encoder.set_buffer(1, Some(&self.sorted), 0);
                        } else {
                            encoder.set_buffer(0, Some(&scratch.tree[(level - 1) % 2]), 0);
                        }
                        let [start, count, dst] = &self.levels[chunk][level];
                        encoder.set_buffer(2, Some(start), 0);
                        encoder.set_buffer(3, Some(count), 0);
                        encoder.set_buffer(4, Some(dst), 0);
                        let out = if level == last {
                            output_buckets
                        } else {
                            &scratch.tree[level % 2]
                        };
                        encoder.set_buffer(5, Some(out), 0);
                        let n = *shards as u32;
                        encoder.set_bytes(6, 4, &n as *const u32 as *const c_void);
                        let group = pipeline.thread_execution_width().min(n as u64).max(1);
                        encoder.dispatch_threads(
                            MTLSize::new(n as u64, 1, 1),
                            MTLSize::new(group, 1, 1),
                        );
                    });
                    if level == 0 {
                        first_ms += ms;
                        first_shards += shards;
                    } else {
                        tree_ms += ms;
                        tree_shards += shards;
                    }
                }
            }
            let Some(reduce) = &self.reduce else {
                return format!(
                    "first {first_ms:.1} ms ({first_shards} shards) tree {tree_ms:.1} ms ({tree_shards} shards) reduce on the host width {} windows {}",
                    self.width, self.windows
                );
            };
            let reduce_ms = run(&|encoder| self.encode_reduce(encoder, buckets));
            let threads = self.reduce_shape[1];
            format!(
                "first {first_ms:.1} ms ({first_shards} shards, limit {}) tree {tree_ms:.1} ms ({tree_shards} shards, limit {}) reduce {reduce_ms:.1} ms ({threads} threads/window, limit {}) width {} windows {}",
                self.first.max_total_threads_per_threadgroup(),
                self.tree_pipeline.max_total_threads_per_threadgroup(),
                reduce.max_total_threads_per_threadgroup(),
                self.width,
                self.windows
            )
        })
    }

    /// The window reduction: a thread per block of buckets, a partial sum
    /// per threadgroup.
    fn encode_reduce(&self, encoder: &metal::ComputeCommandEncoderRef, buckets: &Buffer) {
        let [leaf, per_window, group, _] = self.reduce_shape;
        let args = [self.half as u32, leaf as u32, per_window as u32, 0u32];
        encoder.set_compute_pipeline_state(self.reduce.as_ref().expect("a GPU reduction"));
        encoder.set_buffer(0, Some(buckets), 0);
        encoder.set_buffer(1, Some(&self.sums), 0);
        encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
        encoder.dispatch_threads(
            MTLSize::new((self.windows * per_window) as u64, 1, 1),
            MTLSize::new(group as u64, 1, 1),
        );
    }

    /// Waits for a submitted MSM and combines its window sums.
    pub fn finish(&mut self, pending: Pending) -> Result<(C, Timings), String> {
        let mut timings = pending.timings;
        let (mut host_sums, prefix_combine_ms) = if let Some((command, windows)) = &pending.host_prefix {
            if let Err(error) = autoreleasepool(|| completed(command)) {
                // A fallback may release the shared workspace after this return.
                let _ = autoreleasepool(|| completed(&pending.command));
                return Err(error);
            }
            let started = Instant::now();
            let host = self.host_buckets.as_ref().expect("host bucket workspace");
            let sums = self.reduce_host_windows(host, 0..*windows);
            log::info!("ZOPRO_TRACE metal-overlap-host-reduce {windows}");
            (sums, elapsed(started))
        } else {
            (Vec::new(), 0.0)
        };
        autoreleasepool(|| completed(&pending.command))?;
        timings.gpu_ms = elapsed(pending.submitted);
        let started = Instant::now();
        let mut result = C::IDENTITY;
        if let Some(host) = &self.host_buckets {
            let (buffer, count) = self.host_tail.as_ref().map_or((host, self.windows), |tail| {
                (&tail.buffer, self.windows - tail.first_window)
            });
            host_sums.extend(self.reduce_host_windows(buffer, 0..count));
            for sum in host_sums.into_iter().rev() {
                for _ in 0..self.width {
                    result = result.double();
                }
                result += sum;
            }
            timings.combine_ms = prefix_combine_ms + elapsed(started);
            return Ok((result, timings));
        }
        let sums = self.sums.contents() as *const u8;
        for window in (0..self.windows).rev() {
            for _ in 0..self.width {
                result = result.double();
            }
            let groups = self.reduce_shape[3];
            for group in 0..groups {
                // SAFETY: `sums` holds `groups` xyzz points per window,
                // written by the completed reduce dispatch.
                let at = (window * groups + group) * C::XYZZ_BYTES;
                let point = unsafe { std::slice::from_raw_parts(sums.add(at), C::XYZZ_BYTES) };
                result += C::xyzz_to_projective(point);
            }
        }
        timings.combine_ms = elapsed(started);
        Ok((result, timings))
    }

    /// Each window's sum of m * B_m by running sums, windows in parallel.
    fn reduce_host_windows(&self, host: &Buffer, windows: std::ops::Range<usize>) -> Vec<C> {
        let buckets = host.contents() as usize;
        let window_bytes = self.half * C::XYZZ_BYTES;
        windows
            .into_par_iter()
            .map(|window| {
                let at = window * window_bytes;
                // SAFETY: the command that writes this buffer has completed.
                let values = unsafe {
                    std::slice::from_raw_parts((buckets + at) as *const u8, window_bytes)
                };
                C::reduce_host_window(values)
            })
            .collect()
    }

    /// The levels of shards over the sorted runs of a chunk of windows,
    /// written to the chunk's level buffers; returns each level's shard
    /// count. Level 0 splits bucket runs of base points into first_shard;
    /// each later level splits the previous level's contiguous shard ranges
    /// into SHARD, until every bucket has one, which the last level writes
    /// to the bucket's place.
    fn plan(&mut self, chunk: usize, windows: std::ops::Range<usize>) -> Vec<usize> {
        let first_bucket = windows.start * self.half;
        let bucket_origin = self.host_tail.as_ref().filter(|tail| windows.start >= tail.first_window)
            .map_or(0, |tail| tail.first_window * self.half);
        let buckets = windows.len() * self.half;
        let mut runs: Vec<(u32, u32)> = self.starts[first_bucket..first_bucket + buckets]
            .iter()
            .zip(&self.counts[first_bucket..first_bucket + buckets])
            .map(|(s, c)| (*s, *c))
            .collect();
        let mut shard_counts = Vec::new();
        let mut next = Vec::new();
        let mut level = 0;
        if self.levels.len() <= chunk {
            self.levels.push(Vec::new());
        }
        let most = self.chunk_windows * (self.n.div_ceil(self.first_shard) + self.half);
        loop {
            let size = if level == 0 { self.first_shard } else { SHARD } as u32;
            let last = runs.iter().all(|(_, c)| *c <= size);
            if self.levels[chunk].len() <= level {
                self.levels[chunk].push([
                    shared(&self.device, most * 4),
                    shared(&self.device, most * 4),
                    shared(&self.device, most * 4),
                ]);
            }
            let [starts, counts, destinations] = self.levels[chunk][level].each_ref().map(|buffer| {
                // SAFETY: each buffer independently holds most u32 slots.
                // This MSM has finished using the buffers from its previous
                // execution before the next plan writes them.
                unsafe {
                    std::slice::from_raw_parts_mut(
                        buffer.contents().cast::<std::mem::MaybeUninit<u32>>(),
                        most,
                    )
                }
            });
            if !last {
                next.reserve(buckets);
            }
            let mut cursor = 0;
            for (b, &(run_start, run_count)) in runs.iter().enumerate() {
                let first = cursor as u32;
                let mut at = 0;
                while at < run_count {
                    let len = size.min(run_count - at);
                    starts[cursor].write(run_start + at);
                    counts[cursor].write(len);
                    destinations[cursor].write(if last {
                        (first_bucket - bucket_origin + b) as u32
                    } else {
                        cursor as u32
                    });
                    cursor += 1;
                    at += len;
                }
                if !last {
                    next.push((first, cursor as u32 - first));
                }
            }
            shard_counts.push(cursor);
            if last {
                return shard_counts;
            }
            std::mem::swap(&mut runs, &mut next);
            next.clear();
            level += 1;
        }
    }

    /// Digits, counts and the scatter: digits in parallel over scalars,
    /// then one window per task writing its own segment of the buffers.
    fn sort(&mut self, scalars: &[Fr]) {
        assert_eq!(scalars.len(), self.n);
        self.sort_words(scalars, Fr::to_canonical);
        self.sorted_for_original_bases = true;
    }

    fn sort_words<S: Sync>(&mut self, scalars: &[S], words: impl Fn(&S) -> [u64; 4] + Sync) {
        let (width, windows) = (self.width, self.windows);
        let radix = 1u64 << width;
        let mask = radix - 1;
        let n = scalars.len();
        // Keep more of the narrow G1 recoding blocks in flight on wide Macs.
        let digit_chunk = if cfg!(all(target_os = "macos", target_arch = "aarch64"))
            && !C::G2
            && (8..=9).contains(&width)
            && (16384..65536).contains(&n)
            && rayon::current_num_threads() >= 16
        {
            1024
        } else {
            DIGIT_CHUNK
        };
        // In chunks of scalars, window-major within each: a window's pass
        // reads its runs rather than every row (a 2^21-point Z sort took
        // 60-110 ms reading all of them).
        let mut digits = vec![0i16; n * windows];
        digits
            .par_chunks_mut(digit_chunk * windows)
            .zip(scalars.par_chunks(digit_chunk))
            .for_each(|(block, chunk)| {
                let len = chunk.len();
                for (i, s) in chunk.iter().enumerate() {
                    let words = words(s);
                    let mut carry = 0u64;
                    for w in 0..windows {
                        let offset = w * width;
                        let (word, shift) = (offset / 64, offset % 64);
                        let mut value = if word < 4 { words[word] >> shift } else { 0 };
                        if shift + width > 64 && word < 3 {
                            value |= words[word + 1] << (64 - shift);
                        }
                        let value = (value & mask) + carry;
                        carry = (value + radix / 2) >> width;
                        block[w * len + i] = (value as i64 - (carry * radix) as i64) as i16;
                    }
                }
            });
        self.scatter_digits(n, &digits, digit_chunk).expect("host recoding");
    }

    fn scatter_digits(
        &mut self, n: usize, digits: &[i16], digit_chunk: usize,
    ) -> Result<(), &'static str> {
        let (windows, half) = (self.windows, self.half);
        // Window w's digits, chunk by chunk, with the chunk's first index.
        let runs = |w: usize| {
            digits
                .chunks(digit_chunk * windows)
                .enumerate()
                .map(move |(c, block)| {
                    let len = block.len() / windows;
                    (c * digit_chunk, &block[w * len..(w + 1) * len])
                })
        };
        // SAFETY: the buffer holds n * windows u32s, shared storage, and
        // nothing else reads it until the dispatch.
        let sorted = unsafe {
            std::slice::from_raw_parts_mut(self.sorted.contents() as *mut u32, n * windows)
        };
        sorted
            .par_chunks_mut(n)
            .zip(self.starts.par_chunks_mut(half))
            .zip(self.counts.par_chunks_mut(half))
            .enumerate()
            .try_for_each(|(w, ((sorted, starts), counts))| {
                counts.fill(0);
                for (_, run) in runs(w) {
                    for &d in run {
                        if d != 0 {
                            let count = counts.get_mut(d.unsigned_abs() as usize - 1)
                                .ok_or("MSM digit exceeds its window")?;
                            *count += 1;
                        }
                    }
                }
                let base = (w * n) as u32;
                let mut at = base;
                for (start, count) in starts.iter_mut().zip(counts.iter()) {
                    *start = at;
                    at += count;
                }
                let mut positions: Vec<u32> = starts.iter().map(|s| s - base).collect();
                for (first, run) in runs(w) {
                    for (j, &d) in run.iter().enumerate() {
                        if d != 0 {
                            let b = d.unsigned_abs() as usize - 1;
                            sorted[positions[b] as usize] =
                                (first + j) as u32 | if d < 0 { 0x8000_0000 } else { 0 };
                            positions[b] += 1;
                        }
                    }
                }
                Ok(())
            })
    }

    pub fn device_name(&self) -> String {
        self.device.name().to_string()
    }
}

fn elapsed(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

fn fq(bytes: &[u8]) -> Fq {
    Fq::from_montgomery(core::array::from_fn(|i| {
        u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{Affine, CurveConfig, G1Config, G2Config, Projective};

    /// A seeded source of scalars (splitmix64).
    struct StdRng(u64);
    impl StdRng {
        fn seed_from_u64(seed: u64) -> Self {
            Self(seed)
        }
        fn fr(&mut self) -> Fr {
            let mut bytes = [0u8; 64];
            for chunk in bytes.chunks_mut(8) {
                self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
                let mut z = self.0;
                z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                chunk.copy_from_slice(&(z ^ (z >> 31)).to_le_bytes());
            }
            Fr::from_random_bytes(&bytes)
        }
    }

    /// Distinct points without a scalar multiplication each: a random
    /// start, then repeated addition of a random step.
    fn points<C: CurveConfig>(n: usize, rng: &mut StdRng) -> Vec<Affine<C>> {
        let step = Projective::<C>::generator().mul(&rng.fr());
        let mut p = Projective::<C>::generator().mul(&rng.fr());
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
                _ => rng.fr(),
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
        metal::Device::system_default()?;
        Some(|bases: &[C::Affine]| -> Box<dyn FnMut(&[Fr]) -> C> {
            let mut msm = Msm::<C>::new(bases).expect("msm").0;
            Box::new(move |scalars: &[Fr]| msm.msm(scalars).expect("gpu msm").0)
        })
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[test]
    fn host_bucket_sum_handles_rescaled_points_and_cancellation() {
        let g = G1Affine::GENERATOR;
        let points = [G1Affine::IDENTITY, g, -g, -g, g, g];
        let mut buckets = Vec::new();
        for (i, p) in points.iter().enumerate() {
            let z = if p.is_identity() { Fq::ZERO } else { Fq::from(i as u64 + 2) };
            let zz = z.square();
            let zzz = zz * z;
            for v in [p.x * zz, p.y * zzz, zz, zzz] {
                fq_bytes(&v, &mut buckets);
            }
        }
        let weights: [Fr; 6] = core::array::from_fn(|i| Fr::from(i as u64 + 1));
        assert_eq!(
            G1Projective::reduce_host_window(&buckets),
            zopro_bn254::curve::msm(&points, &weights),
        );
    }

    #[test]
    fn g1_msms_match_the_reference() {
        check::<G1Config>(&[1, 7, 100, 3000, 20000]);
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        check::<G1Config>(&[32768]);
    }

    #[test]
    fn radix29_shards_preserve_signed_and_exceptional_g1_additions() {
        if metal::Device::system_default().is_none() || smaller_code() {
            return;
        }
        let n = (1 << 16) + 1;
        let mut rng = StdRng::seed_from_u64(15);
        let mut bases = points::<G1Config>(n, &mut rng);
        let mut values = scalars(n, false, &mut rng);
        let g = G1Affine::GENERATOR;
        bases[..7].copy_from_slice(&[G1Affine::IDENTITY, g, g, -g, g, -g, g]);
        values[..7].fill(Fr::from(11u64));
        let expected = zopro_bn254::curve::msm(&bases, &values);
        for width in [13, 15] {
            let scratch = Scratch::new().unwrap();
            let (mut msm, _) = G1Msm::new_in_with_width_cap(&bases, scratch, width).unwrap();
            assert!(msm.select_radix29_shards());
            assert_eq!(msm.msm(&values).unwrap().0, expected);
            let capacity = msm.chunk_windows * (n.div_ceil(msm.first_shard) + msm.half);
            msm.shard_order = Some(shard_order::ShardOrder::new(&msm.device, n, capacity).unwrap());
            assert_eq!(msm.msm(&values).unwrap().0, expected);
            assert_eq!(msm.msm(&vec![Fr::ZERO; n]).unwrap().0, G1Projective::IDENTITY);
        }
    }

    #[test]
    fn g2_msms_match_the_reference() {
        check::<G2Config>(&[1, 100, 3000]);
    }

    #[test]
    fn host_reduction_overlaps_disjoint_gpu_windows() {
        if metal::Device::system_default().is_none() {
            return;
        }
        let mut rng = StdRng::seed_from_u64(1743);
        let bases = points::<G1Config>(1025, &mut rng);
        let values = scalars(bases.len(), true, &mut rng);
        let expected = zopro_bn254::curve::msm(&bases, &values);
        let (mut msm, _) = G1Msm::new(&bases).unwrap();
        if msm.host_buckets.is_none() {
            return;
        }
        msm.chunk_windows = 1;
        msm.split_host_buckets();
        let pending = msm.start(&values);
        assert!(pending.host_prefix.is_some());
        assert_eq!(msm.finish(pending).unwrap().0, expected);
        let pending = msm.start(&vec![Fr::ZERO; bases.len()]);
        assert_eq!(msm.finish(pending).unwrap().0, G1Projective::IDENTITY);
    }

    #[test]
    fn grouped_msm_preserves_the_original_bases() {
        if metal::Device::system_default().is_none() {
            eprintln!("no GPU: skipped");
            return;
        }
        let mut rng = StdRng::seed_from_u64(19);
        let bases = points::<G1Config>(257, &mut rng);
        let compact = points::<G1Config>(63, &mut rng);
        let weights = scalars(compact.len(), true, &mut rng);
        let words: Vec<_> = weights.iter().map(Fr::to_canonical).collect();
        let (mut msm, _) = G1Msm::new(&bases).unwrap();
        let pending = msm.start_grouped(&compact, &words);
        let (got, _) = msm.finish(pending).unwrap();
        assert_eq!(got, zopro_bn254::curve::msm(&compact, &weights));

        let weights = scalars(bases.len(), false, &mut rng);
        let (got, _) = msm.msm(&weights).unwrap();
        assert_eq!(got, zopro_bn254::curve::msm(&bases, &weights));
    }

    #[test]
    fn invalid_gpu_digits_return_an_error() {
        if metal::Device::system_default().is_none() {
            eprintln!("no GPU: skipped");
            return;
        }
        let (mut msm, _) = G1Msm::new(&[G1Affine::GENERATOR]).unwrap();
        let mut digits = vec![0; msm.windows];
        digits[0] = i16::MAX;
        assert_eq!(msm.scatter_digits(1, &digits, DIGIT_CHUNK), Err("MSM digit exceeds its window"));
    }
}
