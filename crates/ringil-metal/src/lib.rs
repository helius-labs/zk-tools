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
use std::{ffi::c_void, time::Instant};

use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, PrimeField, Zero};
#[cfg(not(metallib))]
use metal::CompileOptions;
use objc::rc::autoreleasepool;
use objc::{msg_send, sel, sel_impl};
use metal::{Buffer, CommandQueue, ComputePipelineState, Device, MTLResourceOptions, MTLSize};
use rayon::prelude::*;

pub mod quotient;
pub use quotient::Quotient;

/// The shaders as a library: the metallib build.rs compiled, else compiled
/// from source now. The G2 kernels are instantiated only when asked for:
/// they take a minute to compile from source (six on an iPhone), and a
/// phone proves G2 on its CPU.
/// Compiled once per process and kernel set: a phone does not cache source
/// compilations the way a Mac does, and a key makes five or six objects
/// from the same library. One system device is assumed.
pub(crate) fn library(device: &Device, g2: bool) -> Result<metal::Library, String> {
    static LIBRARIES: std::sync::Mutex<[Option<Shared<metal::Library>>; 4]> = std::sync::Mutex::new([None, None, None, None]);
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
pub(crate) fn pipeline(device: &Device, g2: bool, name: &str) -> Result<metal::ComputePipelineState, String> {
    type Pipelines = std::collections::HashMap<(String, bool, bool), Shared<metal::ComputePipelineState>>;
    static PIPELINES: std::sync::Mutex<Option<Pipelines>> = std::sync::Mutex::new(None);
    let key = (name.to_string(), g2, smaller_code());
    if let Some(found) = PIPELINES.lock().unwrap().as_ref().and_then(|cache| cache.get(&key)) {
        return Ok(found.0.clone());
    }
    let library = library(device, g2)?;
    let function = library.get_function(name, None).map_err(|e| format!("{name}: {e}"))?;
    let state = device.new_compute_pipeline_state_with_function(&function).map_err(|e| format!("{name}: {e}"))?;
    PIPELINES.lock().unwrap().get_or_insert_with(Default::default).insert(key, Shared(state.clone()));
    Ok(state)
}

/// Compiles the library and builds the G1 MSM and quotient pipelines, so
/// that the first key load does not: an app can call it at start.
pub fn warm() -> Result<(), String> {
    autoreleasepool(|| {
        let device = Device::system_default().ok_or("no Metal device")?;
        for name in [G1Projective::FIRST, G1Projective::TREE, G1Projective::REDUCE, "fr_dif_stage", "fr_dit_stage", "fr_scale", "fr_pointwise"] {
            pipeline(&device, false, name)?;
        }
        Ok(())
    })
}

/// Whether the shaders compile at Metal's size optimization level rather
/// than the default: aeglos found the default level corrupting a kernel,
/// and two older iPhones' GPU MSMs disagreed with the CPU where a newer
/// one's agreed. A host switches after such a mismatch; RINGIL_METAL_SIZE=1
/// switches from the start. Libraries are cached per level.
static SMALLER_CODE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Switches to the size level for the libraries compiled from now on; false
/// when it was already on, so a host does not retry forever.
pub fn prefer_smaller_code() -> bool {
    !SMALLER_CODE.swap(true, std::sync::atomic::Ordering::SeqCst)
}

pub fn smaller_code() -> bool {
    SMALLER_CODE.load(std::sync::atomic::Ordering::SeqCst) || std::env::var("RINGIL_METAL_SIZE").is_ok_and(|v| v == "1")
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
    autoreleasepool(|| Device::system_default().is_some_and(|device| device.supports_family(metal::MTLGPUFamily::Apple9)))
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
    #[cfg(metallib)]
    {
        let _ = g2;
        let bytes: &[u8] = if smaller {
            include_bytes!(concat!(env!("OUT_DIR"), "/ringil-size.metallib"))
        } else {
            include_bytes!(concat!(env!("OUT_DIR"), "/ringil.metallib"))
        };
        device.new_library_with_data(bytes).map_err(|e| format!("metallib: {e}"))
    }
    #[cfg(not(metallib))]
    {
        let source = if g2 { format!("{SHADER}\nBUCKET_KERNELS(fp2_t, g2)\n") } else { SHADER.to_string() };
        let options = CompileOptions::new();
        if smaller {
            // MTLLibraryOptimizationLevelSize; metal-rs has no setter for it.
            let _: () = unsafe { msg_send![&*options, setOptimizationLevel: 1i64] };
        }
        device.new_library_with_source(&source, &options).map_err(|e| format!("shader: {e}"))
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
/// Window width by point count, as the CPU MSM's: wider windows mean fewer
/// bucket additions and more buckets to reduce (and, here, to zero).
fn width_for(n: usize) -> usize {
    let width = match n {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        _ => 15,
    };
    // Capped at 13 bits (RINGIL_GPU_WIDTH overrides, at most 15: digits are
    // i16): a quarter of 15 bits' buckets to reduce and sort for 18 % more
    // additions. On an M5 Pro whole proofs went 202 → 154 ms at 8 → 2 and
    // 444 → 395 at 24 → 2; on an iPhone 12 and 15 the 15-bit reduction was
    // 28 % of the GPU's MSM time.
    let cap = std::env::var("RINGIL_GPU_WIDTH").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(13).clamp(7, 15);
    width.min(cap)
}
/// Threads per window in the reduction: each takes half / this many buckets.
const REDUCE_THREADS_PER_WINDOW: usize = 512;
/// Threads per threadgroup in the reduction; each threadgroup leaves one
/// partial sum for the host to add.
const REDUCE_GROUP: usize = 256;
/// Shard length. Witness scalars put most points in a few buckets, whose
/// long runs are what the shards split; long shards there starve the GPU of
/// threads (128 over base points left 24x2's G2 MSM at 227 ms, 32 at 151).
const FIRST_SHARD: usize = 32;
const SHARD: usize = 32;

/// Stage times of one MSM, in milliseconds: the host sort and planning,
/// the GPU from submission to completion, the host combination.
#[derive(Debug, Clone, Copy, Default)]
pub struct Timings {
    pub sort_ms: f64,
    pub gpu_ms: f64,
    pub combine_ms: f64,
}

/// An MSM submitted to the GPU and not yet collected.
pub struct Pending {
    command: metal::CommandBuffer,
    submitted: Instant,
    timings: Timings,
}

/// A curve the shaders have kernels for, with its point encodings.
pub trait Curve: CurveGroup {
    const AFFINE_BYTES: usize;
    const XYZZ_BYTES: usize;
    const G2: bool;
    const FIRST: &'static str;
    const TREE: &'static str;
    const REDUCE: &'static str;
    /// The affine point as the kernel holds it, the identity as zeros.
    fn encode(point: &Self::Affine, out: &mut Vec<u8>);
    /// An xyzz point (x, y, zz, zzz) as arkworks' Jacobian (x zz, y zzz, zz).
    fn xyzz_to_projective(bytes: &[u8]) -> Self;
}

fn fq_bytes(v: &Fq, out: &mut Vec<u8>) {
    for w in v.0 .0 {
        out.extend_from_slice(&w.to_le_bytes());
    }
}

impl Curve for ark_ec::short_weierstrass::Projective<ark_bn254::g1::Config> {
    const AFFINE_BYTES: usize = 64;
    const XYZZ_BYTES: usize = 128;
    const G2: bool = false;
    const FIRST: &'static str = "shards_first_g1";
    const TREE: &'static str = "shards_tree_g1";
    const REDUCE: &'static str = "reduce_g1";
    fn encode(point: &G1Affine, out: &mut Vec<u8>) {
        if point.infinity {
            out.extend_from_slice(&[0u8; 64]);
        } else {
            fq_bytes(&point.x, out);
            fq_bytes(&point.y, out);
        }
    }
    fn xyzz_to_projective(bytes: &[u8]) -> Self {
        let (x, y, zz, zzz) = (fq(&bytes[..32]), fq(&bytes[32..64]), fq(&bytes[64..96]), fq(&bytes[96..128]));
        if zz.is_zero() {
            return Self::ZERO;
        }
        Self::new_unchecked(x * zz, y * zzz, zz)
    }
}

impl Curve for ark_ec::short_weierstrass::Projective<ark_bn254::g2::Config> {
    const AFFINE_BYTES: usize = 128;
    const XYZZ_BYTES: usize = 256;
    const G2: bool = true;
    const FIRST: &'static str = "shards_first_g2";
    const TREE: &'static str = "shards_tree_g2";
    const REDUCE: &'static str = "reduce_g2";
    fn encode(point: &G2Affine, out: &mut Vec<u8>) {
        if point.infinity {
            out.extend_from_slice(&[0u8; 128]);
        } else {
            for v in [point.x.c0, point.x.c1, point.y.c0, point.y.c1] {
                fq_bytes(&v, out);
            }
        }
    }
    fn xyzz_to_projective(bytes: &[u8]) -> Self {
        let fq2 = |at: usize| Fq2::new(fq(&bytes[at..at + 32]), fq(&bytes[at + 32..at + 64]));
        let (x, y, zz, zzz) = (fq2(0), fq2(64), fq2(128), fq2(192));
        if zz.is_zero() {
            return Self::ZERO;
        }
        Self::new_unchecked(x * zz, y * zzz, zz)
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
    tree_pipeline: ComputePipelineState,
    reduce: ComputePipelineState,
    bases: Buffer,
    n: usize,
    sorted: Buffer,
    /// Windows per chunk: the shard trees are sized for one chunk and
    /// reused, which bounds the GPU memory a wide key takes.
    chunk_windows: usize,
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

fn shared(device: &Device, bytes: usize) -> Buffer {
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
        let buffers = ScratchBuffers { tree: [shared(&device, 16), shared(&device, 16)], buckets: shared(&device, 16) };
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
            self.levels.iter().flatten().flatten().for_each(discard);
        })
    }
}

impl<C: Curve> Msm<C> {
    /// Compiles the kernels and uploads the bases; `compile_ms` is how long
    /// the shader compilation took.
    pub fn new(bases: &[C::Affine]) -> Result<(Self, f64), String> {
        Self::new_in(bases, Scratch::new()?)
    }

    /// As `new`, with the trees and buckets shared with other MSMs.
    pub fn new_in(bases: &[C::Affine], scratch: std::sync::Arc<Scratch>) -> Result<(Self, f64), String> {
        // Building a pipeline autoreleases objects that hold on to the
        // device's memory; a thread without a pool leaked them per key.
        autoreleasepool(|| Self::new_in_pool(bases, scratch))
    }

    fn new_in_pool(bases: &[C::Affine], scratch: std::sync::Arc<Scratch>) -> Result<(Self, f64), String> {
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let pipeline = |name: &str| pipeline(&device, C::G2, name);
        let first = pipeline(C::FIRST)?;
        let tree_pipeline = pipeline(C::TREE)?;
        let reduce = pipeline(C::REDUCE)?;
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        // Encoded straight into the buffer, a block of points at a time: a
        // whole host copy of the vector per key load left the allocator
        // holding hundreds of MB.
        let bases_buffer = shared(&device, bases.len() * C::AFFINE_BYTES);
        let mut raw = Vec::with_capacity(4096 * C::AFFINE_BYTES);
        for (block, points) in bases.chunks(4096).enumerate() {
            raw.clear();
            for p in points {
                C::encode(p, &mut raw);
            }
            // SAFETY: the shared buffer is the vector's size and no GPU work
            // uses it yet; each block lands at its own offset.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    raw.as_ptr(),
                    (bases_buffer.contents() as *mut u8).add(block * 4096 * C::AFFINE_BYTES),
                    raw.len(),
                );
            }
        }
        let n = bases.len();
        let width = width_for(n);
        let half = 1 << (width - 1);
        let windows = (Fr::MODULUS_BIT_SIZE as usize + 1).div_ceil(width);
        let buckets = windows * half;
        // The most shards a level can have for one window: every pair in
        // its own first shard, or every bucket's run split, whichever is
        // more. Chunks of windows keep each tree under about 64 MB.
        let per_window = n.div_ceil(FIRST_SHARD) + half;
        let chunk_windows = ((64 << 20) / (per_window * C::XYZZ_BYTES)).clamp(1, windows);
        let most = chunk_windows * per_window;
        scratch.reserve(&device, most * C::XYZZ_BYTES, buckets * C::XYZZ_BYTES);
        let reduce_shape = {
            let per_window = half.min(REDUCE_THREADS_PER_WINDOW);
            let allowed = reduce.max_total_threads_per_threadgroup() as usize;
            let allowed = 1usize << (usize::BITS - 1 - allowed.max(1).leading_zeros());
            let group = per_window.min(REDUCE_GROUP).min(allowed);
            [half / per_window, per_window, group, per_window / group]
        };
        Ok((
            Self {
                width,
                half,
                windows,
                chunk_windows,
                queue: queue(&device),
                sorted: shared(&device, n * windows * 4),
                levels: Vec::new(),
                scratch,
                bucket_bytes: buckets * C::XYZZ_BYTES,
                sums: shared(&device, windows * reduce_shape[3] * C::XYZZ_BYTES),
                reduce_shape,
                counts: vec![0; buckets],
                starts: vec![0; buckets],
                _curve: std::marker::PhantomData,
                first,
                tree_pipeline,
                reduce,
                bases: bases_buffer,
                n,
                device,
            },
            compile_ms,
        ))
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

    /// The MSM over the scalars another MSM of the same length just sorted
    /// (G1 B and G2 B share theirs): its sorted runs, no sort.
    pub fn start_sorted_as<D: Curve>(&mut self, other: &Msm<D>) -> Pending {
        assert_eq!((other.n, other.width), (self.n, self.width));
        let started = Instant::now();
        self.sorted = other.sorted.clone();
        self.counts.clone_from(&other.counts);
        self.starts.clone_from(&other.starts);
        self.submit(started)
    }

    fn submit(&mut self, started: Instant) -> Pending {
        // Command buffers and encoders are autoreleased, and a Rust thread
        // has no pool: without one each proof leaked them, and with them
        // every buffer of the key, until an iPhone's GPU ran out.
        autoreleasepool(|| self.submit_in_pool(started))
    }

    fn submit_in_pool(&mut self, started: Instant) -> Pending {
        let chunks: Vec<Vec<usize>> = (0..self.windows.div_ceil(self.chunk_windows))
            .map(|chunk| {
                let first = chunk * self.chunk_windows;
                self.plan(chunk, first..(first + self.chunk_windows).min(self.windows))
            })
            .collect();
        let sort_ms = elapsed(started);
        let command = self.queue.new_command_buffer();
        let scratch = self.scratch.0.lock().unwrap();
        // Empty buckets stay at infinity.
        let blit = command.new_blit_command_encoder();
        blit.fill_buffer(&scratch.buckets, metal::NSRange::new(0, self.bucket_bytes as u64), 0);
        blit.end_encoding();
        let encoder = command.new_compute_command_encoder();
        // Chunks run in order and share the trees; the dispatches of one
        // are complete before the next begins.
        for (chunk, levels) in chunks.iter().enumerate() {
            let last = levels.len() - 1;
            for (level, shards) in levels.iter().enumerate() {
                let pipeline = if level == 0 { &self.first } else { &self.tree_pipeline };
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
                let out = if level == last { &scratch.buckets } else { &scratch.tree[level % 2] };
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
        self.encode_reduce(encoder, &scratch.buckets);
        encoder.end_encoding();
        command.commit();
        Pending {
            command: command.to_owned(),
            submitted: Instant::now(),
            timings: Timings { sort_ms, ..Timings::default() },
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
            let run = |encode: &dyn Fn(&metal::ComputeCommandEncoderRef)| -> f64 {
                let command = self.queue.new_command_buffer();
                let encoder = command.new_compute_command_encoder();
                encode(encoder);
                encoder.end_encoding();
                command.commit();
                command.wait_until_completed();
                // SAFETY: GPUStartTime/GPUEndTime are CFTimeInterval
                // properties of a completed command buffer.
                let (start, end): (f64, f64) = unsafe { (msg_send![command, GPUStartTime], msg_send![command, GPUEndTime]) };
                (end - start) * 1000.0
            };
            let (mut first_ms, mut tree_ms, mut first_shards, mut tree_shards) = (0.0, 0.0, 0usize, 0usize);
            for (chunk, levels) in chunks.iter().enumerate() {
                let last = levels.len() - 1;
                for (level, shards) in levels.iter().enumerate() {
                    let pipeline = if level == 0 { &self.first } else { &self.tree_pipeline };
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
                        let out = if level == last { &scratch.buckets } else { &scratch.tree[level % 2] };
                        encoder.set_buffer(5, Some(out), 0);
                        let n = *shards as u32;
                        encoder.set_bytes(6, 4, &n as *const u32 as *const c_void);
                        let group = pipeline.thread_execution_width().min(n as u64).max(1);
                        encoder.dispatch_threads(MTLSize::new(n as u64, 1, 1), MTLSize::new(group, 1, 1));
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
            let reduce_ms = run(&|encoder| self.encode_reduce(encoder, &scratch.buckets));
            let threads = self.reduce_shape[1];
            format!(
                "first {first_ms:.1} ms ({first_shards} shards, limit {}) tree {tree_ms:.1} ms ({tree_shards} shards, limit {}) reduce {reduce_ms:.1} ms ({threads} threads/window, limit {}) width {} windows {}",
                self.first.max_total_threads_per_threadgroup(),
                self.tree_pipeline.max_total_threads_per_threadgroup(),
                self.reduce.max_total_threads_per_threadgroup(),
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
        encoder.set_compute_pipeline_state(&self.reduce);
        encoder.set_buffer(0, Some(buckets), 0);
        encoder.set_buffer(1, Some(&self.sums), 0);
        encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
        encoder.dispatch_threads(MTLSize::new((self.windows * per_window) as u64, 1, 1), MTLSize::new(group as u64, 1, 1));
    }

    /// Waits for a submitted MSM and combines its window sums.
    pub fn finish(&mut self, pending: Pending) -> Result<(C, Timings), String> {
        let mut timings = pending.timings;
        autoreleasepool(|| completed(&pending.command))?;
        timings.gpu_ms = elapsed(pending.submitted);
        let started = Instant::now();
        let sums = self.sums.contents() as *const u8;
        let mut result = C::ZERO;
        for window in (0..self.windows).rev() {
            for _ in 0..self.width {
                result.double_in_place();
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

    /// The levels of shards over the sorted runs of a chunk of windows,
    /// written to the chunk's level buffers; returns each level's shard
    /// count. Level 0 splits bucket runs of base points into FIRST_SHARD;
    /// each later level splits the previous level's contiguous shard ranges
    /// into SHARD, until every bucket has one, which the last level writes
    /// to the bucket's place.
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
            let size = if level == 0 { FIRST_SHARD } else { SHARD } as u32;
            let last = runs.iter().all(|(_, c)| *c <= size);
            let mut start = Vec::new();
            let mut count = Vec::new();
            let mut dst = Vec::new();
            let mut next = Vec::with_capacity(buckets);
            for (b, (run_start, run_count)) in runs.iter().enumerate() {
                let first = start.len() as u32;
                let mut at = 0;
                while at < *run_count {
                    let len = size.min(run_count - at);
                    start.push(run_start + at);
                    count.push(len);
                    dst.push(if last { (first_bucket + b) as u32 } else { first + (at / size) });
                    at += len;
                }
                next.push((first, start.len() as u32 - first));
            }
            if self.levels[chunk].len() <= level {
                let most = self.chunk_windows * (self.n.div_ceil(FIRST_SHARD) + self.half);
                self.levels[chunk].push([shared(&self.device, most * 4), shared(&self.device, most * 4), shared(&self.device, most * 4)]);
            }
            for (buffer, data) in self.levels[chunk][level].iter().zip([&start, &count, &dst]) {
                // SAFETY: each level buffer holds `most` u32s, at least data.len().
                unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), buffer.contents() as *mut u32, data.len()) };
            }
            shard_counts.push(start.len());
            if last {
                return shard_counts;
            }
            runs = next;
            level += 1;
        }
    }

    /// Digits, counts and the scatter: digits in parallel over scalars,
    /// then one window per task writing its own segment of the buffers.
    fn sort(&mut self, scalars: &[Fr]) {
        let (width, windows, half) = (self.width, self.windows, self.half);
        let radix = 1u64 << width;
        let mask = radix - 1;
        let n = self.n;
        // digits[i * windows + w].
        let mut digits = vec![0i16; n * windows];
        digits.par_chunks_mut(windows).zip(scalars).for_each(|(row, s)| {
            let BigInt(words) = s.into_bigint();
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
        // SAFETY: the buffer holds n * windows u32s, shared storage, and
        // nothing else reads it until the dispatch.
        let sorted = unsafe { std::slice::from_raw_parts_mut(self.sorted.contents() as *mut u32, n * windows) };
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
                        sorted[positions[b] as usize] = i as u32 | if d < 0 { 0x8000_0000 } else { 0 };
                        positions[b] += 1;
                    }
                }
            });
    }


    pub fn device_name(&self) -> String {
        self.device.name().to_string()
    }
}

fn elapsed(started: Instant) -> f64 {
    started.elapsed().as_secs_f64() * 1000.0
}

fn fq(bytes: &[u8]) -> Fq {
    Fq::new_unchecked(BigInt(core::array::from_fn(|i| {
        u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap())
    })))
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::{AffineRepr, CurveGroup, VariableBaseMSM};
    use ark_ff::{PrimeField, UniformRand};
    use rand::{rngs::StdRng, SeedableRng};

    /// Distinct points without a scalar multiplication each: a random
    /// start, then repeated addition of a random step.
    fn points<C: CurveGroup>(n: usize, rng: &mut StdRng) -> Vec<C::Affine> {
        let step = C::rand(rng);
        let mut p = C::rand(rng);
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            out.push(p);
            p += step;
        }
        C::normalize_batch(&out)
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
                _ => Fr::rand(rng),
            })
            .collect()
    }

    fn check<C: Curve + VariableBaseMSM<ScalarField = Fr>>(sizes: &[usize]) {
        let Some(mut make) = backend::<C>() else {
            eprintln!("no GPU: skipped");
            return;
        };
        let mut rng = StdRng::seed_from_u64(7);
        for &n in sizes {
            let bases = points::<C>(n, &mut rng);
            let mut msm = make(&bases);
            for skewed in [false, true] {
                let scalars = scalars(n, skewed, &mut rng);
                let expected = C::msm_unchecked(&bases, &scalars);
                assert_eq!(msm(&scalars).into_affine(), expected.into_affine(), "{n} points, skewed {skewed}");
            }
            let zeros = vec![Fr::from(0u64); n];
            assert!(msm(&zeros).into_affine().is_zero(), "{n} points, all zero");
            let _ = Fr::MODULUS;
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

    #[test]
    fn g1_msms_match_arkworks() {
        check::<G1Projective>(&[1, 7, 100, 3000, 20000]);
    }

    #[test]
    fn g2_msms_match_arkworks() {
        check::<G2Projective>(&[1, 100, 3000]);
    }
}
