//! G1 and G2 MSMs on an Apple GPU. The host recodes the scalars into signed
//! 15-bit digits and counting-sorts the (base, window) pairs into bucket
//! runs, in parallel over windows; the GPU sums each run into its bucket
//! and reduces each window to its weighted sum; the host combines the
//! windows. Bases are uploaded once, in the kernel's memory layout (eight
//! little-endian words per coordinate, Montgomery form), which is also the
//! shader's.
use std::{ffi::c_void, time::Instant};

use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, PrimeField, Zero};
#[cfg(not(metallib))]
use metal::CompileOptions;
use metal::{Buffer, CommandQueue, ComputePipelineState, Device, MTLResourceOptions, MTLSize};
use rayon::prelude::*;

pub mod quotient;
pub use quotient::Quotient;

/// The shaders as a library: the metallib build.rs compiled, else compiled
/// from source now (the G2 kernels take about a minute).
pub(crate) fn library(device: &Device) -> Result<metal::Library, String> {
    #[cfg(metallib)]
    {
        let bytes: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ringil.metallib"));
        return device.new_library_with_data(bytes).map_err(|e| format!("metallib: {e}"));
    }
    #[cfg(not(metallib))]
    device.new_library_with_source(SHADER, &CompileOptions::new()).map_err(|e| format!("shader: {e}"))
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
    match n {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        _ => 15,
    }
}
const MOST_REDUCE_THREADS: usize = 1024;
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
    /// Per level: shard starts, counts and destinations.
    levels: Vec<[Buffer; 3]>,
    tree: [Buffer; 2],
    buckets: Buffer,
    sums: Buffer,
    /// Host copies of the bucket runs, for planning the levels.
    counts: Vec<u32>,
    starts: Vec<u32>,
    _curve: std::marker::PhantomData<C>,
}

fn shared(device: &Device, bytes: usize) -> Buffer {
    device.new_buffer(bytes.max(16) as u64, MTLResourceOptions::StorageModeShared)
}

impl<C: Curve> Msm<C> {
    /// Compiles the kernels and uploads the bases; `compile_ms` is how long
    /// the shader compilation took.
    pub fn new(bases: &[C::Affine]) -> Result<(Self, f64), String> {
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let library = library(&device)?;
        let pipeline = |name: &str| {
            let function = library.get_function(name, None).map_err(|e| format!("{name}: {e}"))?;
            device
                .new_compute_pipeline_state_with_function(&function)
                .map_err(|e| format!("{name}: {e}"))
        };
        let first = pipeline(C::FIRST)?;
        let tree_pipeline = pipeline(C::TREE)?;
        let reduce = pipeline(C::REDUCE)?;
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let mut raw = Vec::with_capacity(bases.len() * C::AFFINE_BYTES);
        for p in bases {
            C::encode(p, &mut raw);
        }
        let bases_buffer = device.new_buffer_with_data(
            raw.as_ptr() as *const c_void,
            raw.len() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        let n = bases.len();
        let width = width_for(n);
        let half = 1 << (width - 1);
        let windows = (Fr::MODULUS_BIT_SIZE as usize + 1).div_ceil(width);
        let buckets = windows * half;
        // The most shards a level can have: every pair in its own first
        // shard, or every bucket's run split, whichever is more.
        let most = (n * windows).div_ceil(FIRST_SHARD) + buckets;
        Ok((
            Self {
                width,
                half,
                windows,
                queue: device.new_command_queue(),
                sorted: shared(&device, n * windows * 4),
                levels: Vec::new(),
                tree: [shared(&device, most * C::XYZZ_BYTES), shared(&device, most * C::XYZZ_BYTES)],
                buckets: shared(&device, buckets * C::XYZZ_BYTES),
                sums: shared(&device, windows * C::XYZZ_BYTES),
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

    pub fn msm(&mut self, scalars: &[Fr]) -> (C, Timings) {
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
        let levels = self.plan();
        let sort_ms = elapsed(started);
        let command = self.queue.new_command_buffer();
        // Empty buckets stay at infinity.
        let blit = command.new_blit_command_encoder();
        blit.fill_buffer(&self.buckets, metal::NSRange::new(0, self.buckets.length()), 0);
        blit.end_encoding();
        let encoder = command.new_compute_command_encoder();
        let last = levels.len() - 1;
        for (level, shards) in levels.iter().enumerate() {
            let pipeline = if level == 0 { &self.first } else { &self.tree_pipeline };
            encoder.set_compute_pipeline_state(pipeline);
            if level == 0 {
                encoder.set_buffer(0, Some(&self.bases), 0);
                encoder.set_buffer(1, Some(&self.sorted), 0);
            } else {
                encoder.set_buffer(0, Some(&self.tree[(level - 1) % 2]), 0);
            }
            let [start, count, dst] = &self.levels[level];
            encoder.set_buffer(2, Some(start), 0);
            encoder.set_buffer(3, Some(count), 0);
            encoder.set_buffer(4, Some(dst), 0);
            let out = if level == last { &self.buckets } else { &self.tree[level % 2] };
            encoder.set_buffer(5, Some(out), 0);
            let n = *shards as u32;
            encoder.set_bytes(6, 4, &n as *const u32 as *const c_void);
            let group = pipeline.thread_execution_width().min(n as u64).max(1);
            encoder.dispatch_threads(MTLSize::new(n as u64, 1, 1), MTLSize::new(group, 1, 1));
        }
        // One threadgroup per window, a thread per `leaf` buckets, at least
        // a full SIMD group.
        let threads = self.half.min(MOST_REDUCE_THREADS).max(32);
        let leaf = self.half / threads;
        let args = [self.half as u32, leaf as u32, leaf.trailing_zeros(), 0u32];
        encoder.set_compute_pipeline_state(&self.reduce);
        encoder.set_buffer(0, Some(&self.buckets), 0);
        encoder.set_buffer(1, Some(&self.sums), 0);
        encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
        encoder.dispatch_thread_groups(MTLSize::new(self.windows as u64, 1, 1), MTLSize::new(threads as u64, 1, 1));
        encoder.end_encoding();
        command.commit();
        Pending {
            command: command.to_owned(),
            submitted: Instant::now(),
            timings: Timings { sort_ms, ..Timings::default() },
        }
    }

    /// Waits for a submitted MSM and combines its window sums.
    pub fn finish(&mut self, pending: Pending) -> (C, Timings) {
        let mut timings = pending.timings;
        pending.command.wait_until_completed();
        timings.gpu_ms = elapsed(pending.submitted);
        let started = Instant::now();
        let sums = self.sums.contents() as *const u8;
        let mut result = C::ZERO;
        for window in (0..self.windows).rev() {
            for _ in 0..self.width {
                result.double_in_place();
            }
            // SAFETY: `sums` holds `windows` xyzz points, written by the
            // completed reduce dispatch.
            let point = unsafe { std::slice::from_raw_parts(sums.add(window * C::XYZZ_BYTES), C::XYZZ_BYTES) };
            result += C::xyzz_to_projective(point);
        }
        timings.combine_ms = elapsed(started);
        (result, timings)
    }

    /// The levels of shards over the sorted runs, written to the level
    /// buffers; returns each level's shard count. Level 0 splits bucket runs
    /// of base points into FIRST_SHARD; each later level splits the previous
    /// level's contiguous shard ranges into SHARD, until every bucket has one.
    fn plan(&mut self) -> Vec<usize> {
        let buckets = self.windows * self.half;
        let mut runs: Vec<(u32, u32)> = self.starts.iter().zip(&self.counts).map(|(s, c)| (*s, *c)).collect();
        let mut shard_counts = Vec::new();
        let mut level = 0;
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
                    dst.push(if last { b as u32 } else { first + (at / size) });
                    at += len;
                }
                next.push((first, start.len() as u32 - first));
            }
            if self.levels.len() <= level {
                let most = (self.n * self.windows).div_ceil(FIRST_SHARD) + buckets;
                self.levels.push([shared(&self.device, most * 4), shared(&self.device, most * 4), shared(&self.device, most * 4)]);
            }
            for (buffer, data) in self.levels[level].iter().zip([&start, &count, &dst]) {
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

    fn run(&self, encode: impl FnOnce(&metal::ComputeCommandEncoderRef)) {
        let command = self.queue.new_command_buffer();
        let encoder = command.new_compute_command_encoder();
        encode(encoder);
        encoder.end_encoding();
        command.commit();
        command.wait_until_completed();
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

