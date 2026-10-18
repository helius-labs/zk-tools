//! G1 and G2 MSMs through wgpu: one WGSL kernel set for Vulkan (Android), WebGPU
//! (the browser) and Metal. As ringil-metal: the host recodes the scalars
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

use ark_bn254::{Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, PrimeField, Zero};
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
pub trait Curve: CurveGroup {
    const AFFINE_BYTES: usize;
    const XYZZ_BYTES: usize;
    const SHADER: &'static str;
    /// shards_first, shards_tree, reduce_blocks.
    const ENTRIES: [&'static str; 3];
    fn encode(point: &Self::Affine, out: &mut Vec<u8>);
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
    const SHADER: &'static str = G1_SHADER;
    const ENTRIES: [&'static str; 3] = ["shards_first", "shards_tree", "reduce_blocks"];
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
    const SHADER: &'static str = G2_SHADER;
    const ENTRIES: [&'static str; 3] = ["shards_first_g2", "shards_tree_g2", "reduce_blocks_g2"];
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

pub type G1Msm = Msm<ark_bn254::G1Projective>;
pub type G2Msm = Msm<ark_bn254::G2Projective>;
const WORKGROUP: u32 = 64;
const FIRST_SHARD: usize = 32;
const SHARD: usize = 32;
/// Buckets per thread in the first reduction stage.
/// Threads per window in the reduction (as ringil-metal's, measured best).
const REDUCE_THREADS_PER_WINDOW: usize = 512;

fn width_for(n: usize) -> usize {
    let width = match n {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        _ => 15,
    };
    // As the Metal driver's: capped at 13 bits (RINGIL_GPU_WIDTH overrides).
    let cap = std::env::var("RINGIL_GPU_WIDTH").ok().and_then(|v| v.parse::<usize>().ok()).unwrap_or(13).clamp(7, 15);
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
}

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub name: String,
    /// Set by the device's uncaptured-error handler: wgpu would otherwise
    /// panic, and a driver that refuses a kernel (a Galaxy S25's did) must
    /// only cost the GPU.
    failed: std::sync::Arc<std::sync::atomic::AtomicBool>,
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
pub(crate) async fn mapped(gpu: &Gpu, buffer: &wgpu::Buffer) -> Result<(), String> {
    let (tx, rx) = futures_channel::oneshot::channel();
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    #[cfg(not(target_arch = "wasm32"))]
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| format!("gpu: {e}"))?;
    rx.await.map_err(|_| "the mapping was dropped".to_string())?.map_err(|e| format!("map: {e}"))?;
    if gpu.failed() {
        buffer.unmap();
        return Err("the device reported an error".into());
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
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ringil"),
                required_limits: adapter.limits(),
                ..Default::default()
            })
            .await
            .map_err(|e| format!("no device: {e}"))?;
        let failed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let flag = failed.clone();
        device.on_uncaptured_error(std::sync::Arc::new(move |e| {
            log::error!("ringil wgpu: {e}");
            flag.store(true, std::sync::atomic::Ordering::SeqCst);
        }));
        Ok(Self { device, queue, name: format!("{} ({:?})", info.name, info.backend), failed })
    }

    /// Whether the device reported an error outside an error scope.
    pub fn failed(&self) -> bool {
        self.failed.load(std::sync::atomic::Ordering::SeqCst)
    }
}

pub struct Msm<C: Curve> {
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
}

pub(crate) fn storage(device: &wgpu::Device, label: &str, bytes: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes.max(16) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
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
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ringil msm"),
            source: wgpu::ShaderSource::Wgsl(C::SHADER.into()),
        });
        let pipeline = |entry: &str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: None,
                module: &module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        // After each kernel: a driver that refuses one leaves the device
        // failed, and nothing else must be asked of it.
        let mut pipelines = Vec::with_capacity(4);
        for entry in C::ENTRIES {
            pipelines.push(pipeline(entry));
            if gpu.failed() {
                return Err(format!("kernel {entry}: the device reported an error"));
            }
        }
        let [first, tree_pipeline, reduce_blocks]: [wgpu::ComputePipeline; 3] = pipelines.try_into().ok().expect("three kernels");
        for scope in [internal, validation] {
            if let Some(e) = popped(scope) {
                return Err(format!("kernels: {e}"));
            }
        }
        if gpu.failed() {
            return Err("kernels: the device reported an error".into());
        }
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let mut raw = Vec::with_capacity(bases.len() * C::AFFINE_BYTES);
        for p in bases {
            C::encode(p, &mut raw);
        }
        let n = bases.len();
        let width = width_for(n);
        let half = 1 << (width - 1);
        let windows = (Fr::MODULUS_BIT_SIZE as usize + 1).div_ceil(width);
        let buckets = windows * half;
        let per_window = n.div_ceil(FIRST_SHARD) + half;
        let chunk_windows = ((64 << 20) / (per_window * C::XYZZ_BYTES)).clamp(1, windows);
        let most = chunk_windows * per_window;
        let per_window = half.min(REDUCE_THREADS_PER_WINDOW);
        let reduce_shape = [half / per_window, per_window];
        let partials = windows * per_window / WORKGROUP as usize;
        let memory = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let bases_buffer = storage(device, "bases", raw.len());
        gpu.queue.write_buffer(&bases_buffer, 0, &raw);
        let msm = Self {
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
                tree: [storage(device, "tree a", most * C::XYZZ_BYTES), storage(device, "tree b", most * C::XYZZ_BYTES)],
                buckets: storage(device, "buckets", buckets * C::XYZZ_BYTES),
                sums: storage(device, "sums", partials * C::XYZZ_BYTES),
                staging: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("staging"),
                    size: (partials * C::XYZZ_BYTES) as u64,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                reduce_uniform: uniform(&gpu, [half as u32, reduce_shape[0] as u32, per_window as u32, 0]),
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
        let mut timings = Timings::default();
        let started = Instant::now();
        let sorted = self.sort(scalars);
        let chunks: Vec<Vec<usize>> = (0..self.windows.div_ceil(self.chunk_windows))
            .map(|chunk| {
                let first = chunk * self.chunk_windows;
                self.plan(chunk, first..(first + self.chunk_windows).min(self.windows))
            })
            .collect();
        timings.sort_ms = started.elapsed().as_secs_f64() * 1000.0;

        let started = Instant::now();
        let device = &self.gpu.device;
        let queue = &self.gpu.queue;
        queue.write_buffer(&self.sorted, 0, bytemuck_cast(&sorted));
        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.clear_buffer(&self.buckets, 0, None);
        for (chunk, levels) in chunks.iter().enumerate() {
        let last = levels.len() - 1;
        for (level, shards) in levels.iter().enumerate() {
            let pipeline = if level == 0 { &self.first } else { &self.tree_pipeline };
            let [start, count, dst, params] = &self.levels[chunk][level];
            let out = if level == last { &self.buckets } else { &self.tree[level % 2] };
            // A layout derived from the entry point holds only the bindings
            // it reads: the tree levels have no bases or sorted indices.
            let mut entries = vec![
                wgpu::BindGroupEntry { binding: 2, resource: start.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: count.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: dst.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 5, resource: out.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 6, resource: params.as_entire_binding() },
            ];
            if level == 0 {
                entries.push(wgpu::BindGroupEntry { binding: 0, resource: self.bases.as_entire_binding() });
                entries.push(wgpu::BindGroupEntry { binding: 1, resource: self.sorted.as_entire_binding() });
            }
            let group0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &entries,
            });
            let group1 = (level > 0).then(|| {
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &pipeline.get_bind_group_layout(1),
                    entries: &[wgpu::BindGroupEntry { binding: 0, resource: self.tree[(level - 1) % 2].as_entire_binding() }],
                })
            });
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &group0, &[]);
            if let Some(group1) = &group1 {
                pass.set_bind_group(1, group1, &[]);
            }
            pass.dispatch_workgroups((*shards as u32).div_ceil(WORKGROUP), 1, 1);
        }
        }
        {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.reduce_blocks.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: self.buckets.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: self.sums.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 3, resource: self.reduce_uniform.as_entire_binding() },
                ],
            });
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.reduce_blocks);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups((self.windows * self.reduce_shape[1] / WORKGROUP as usize) as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&self.sums, 0, &self.staging, 0, self.sums.size());
        queue.submit([encoder.finish()]);
        Pending { submitted: started, timings }
    }

    /// Waits for a submitted MSM and combines its window sums.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn finish(&mut self, pending: Pending) -> Result<(C, Timings), String> {
        pollster::block_on(self.finish_async(pending))
    }

    /// As `finish`, yielding while the GPU works: the browser's way.
    pub async fn finish_async(&mut self, pending: Pending) -> Result<(C, Timings), String> {
        let mut timings = pending.timings;
        mapped(&self.gpu, &self.staging).await?;
        let data = self.staging.slice(..).get_mapped_range().map_err(|e| format!("sums: {e}"))?.to_vec();
        self.staging.unmap();
        timings.gpu_ms = pending.submitted.elapsed().as_secs_f64() * 1000.0;

        let started = Instant::now();
        let mut result = C::ZERO;
        for window in (0..self.windows).rev() {
            for _ in 0..self.width {
                result.double_in_place();
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
            let size = if level == 0 { FIRST_SHARD } else { SHARD } as u32;
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
                    dst.push(if last { (first_bucket + b) as u32 } else { first + (at / size) });
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
    fn sort(&mut self, scalars: &[Fr]) -> Vec<u32> {
        let (width, windows, half, n) = (self.width, self.windows, self.half, self.n);
        let radix = 1u64 << width;
        let mask = radix - 1;
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
                        sorted[positions[b] as usize] = i as u32 | if d < 0 { 0x8000_0000 } else { 0 };
                        positions[b] += 1;
                    }
                }
            });
        sorted
    }
}

fn fq(bytes: &[u8]) -> Fq {
    Fq::new_unchecked(BigInt(core::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))))
}
