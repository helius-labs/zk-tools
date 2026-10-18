//! G1 MSMs through wgpu: one WGSL kernel set for Vulkan (Android), WebGPU
//! (the browser) and Metal. As ringil-metal: the host recodes the scalars
//! into signed digits and counting-sorts the pairs into bucket runs; the
//! GPU sums each run in shards, level by level, reduces each window in two
//! stages, and the host combines the windows. Native hosts wait on the
//! device; the browser's asynchronous path is to come.
use std::time::Instant;

use ark_bn254::{Fq, Fr, G1Affine, G1Projective};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, PrimeField, Zero};
use rayon::prelude::*;
use wgpu::util::DeviceExt;

const MSM_SHADER: &str = concat!(
    include_str!("../shaders/field.wgsl"),
    "\n",
    include_str!("../shaders/xyzz.wgsl"),
    "\n",
    include_str!("../shaders/buckets.wgsl"),
);
const WORKGROUP: u32 = 64;
const FIRST_SHARD: usize = 32;
const SHARD: usize = 32;
/// Buckets per thread in the first reduction stage.
const REDUCE_BLOCK: usize = 64;

fn width_for(n: usize) -> usize {
    match n {
        0..=1023 => 7,
        1024..=4095 => 9,
        4096..=16383 => 10,
        16384..=65535 => 12,
        _ => 15,
    }
}

/// Stage times of one MSM, in milliseconds.
#[derive(Debug, Clone, Copy, Default)]
pub struct Timings {
    pub sort_ms: f64,
    pub gpu_ms: f64,
    pub combine_ms: f64,
}

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub name: String,
}

impl Gpu {
    pub fn new() -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }))
        .map_err(|e| format!("no adapter: {e}"))?;
        let info = adapter.get_info();
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("ringil"),
            required_limits: adapter.limits(),
            ..Default::default()
        }))
        .map_err(|e| format!("no device: {e}"))?;
        Ok(Self { device, queue, name: format!("{} ({:?})", info.name, info.backend) })
    }
}

pub struct G1Msm {
    gpu: std::sync::Arc<Gpu>,
    width: usize,
    half: usize,
    windows: usize,
    n: usize,
    first: wgpu::ComputePipeline,
    tree_pipeline: wgpu::ComputePipeline,
    reduce_blocks: wgpu::ComputePipeline,
    reduce_windows: wgpu::ComputePipeline,
    bases: wgpu::Buffer,
    sorted: wgpu::Buffer,
    levels: Vec<[wgpu::Buffer; 4]>,
    tree: [wgpu::Buffer; 2],
    buckets: wgpu::Buffer,
    pairs: wgpu::Buffer,
    sums: wgpu::Buffer,
    staging: wgpu::Buffer,
    reduce_uniform: wgpu::Buffer,
    counts: Vec<u32>,
    starts: Vec<u32>,
    most: usize,
}

fn storage(device: &wgpu::Device, label: &str, bytes: usize) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: bytes.max(16) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

fn uniform(device: &wgpu::Device, values: [u32; 4]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("uniform"),
        contents: bytemuck_cast(&values),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

fn bytemuck_cast(values: &[u32]) -> &[u8] {
    // SAFETY: u32 has no padding; the slice's bytes are what the GPU reads.
    unsafe { std::slice::from_raw_parts(values.as_ptr() as *const u8, values.len() * 4) }
}

impl G1Msm {
    pub fn new(gpu: std::sync::Arc<Gpu>, bases: &[G1Affine]) -> Result<(Self, f64), String> {
        let device = &gpu.device;
        let started = Instant::now();
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("ringil msm"),
            source: wgpu::ShaderSource::Wgsl(MSM_SHADER.into()),
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
        let (first, tree_pipeline, reduce_blocks, reduce_windows) =
            (pipeline("shards_first"), pipeline("shards_tree"), pipeline("reduce_blocks"), pipeline("reduce_windows"));
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let mut raw = Vec::with_capacity(bases.len() * 64);
        for p in bases {
            if p.infinity {
                raw.extend_from_slice(&[0u8; 64]);
            } else {
                for v in [p.x, p.y] {
                    for w in v.0 .0 {
                        raw.extend_from_slice(&w.to_le_bytes());
                    }
                }
            }
        }
        let n = bases.len();
        let width = width_for(n);
        let half = 1 << (width - 1);
        let windows = (Fr::MODULUS_BIT_SIZE as usize + 1).div_ceil(width);
        let buckets = windows * half;
        let most = (n * windows).div_ceil(FIRST_SHARD) + buckets;
        let blocks = half.div_ceil(REDUCE_BLOCK);
        let bases_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("bases"),
            contents: &raw,
            usage: wgpu::BufferUsages::STORAGE,
        });
        Ok((
            Self {
                width,
                half,
                windows,
                n,
                first,
                tree_pipeline,
                reduce_blocks,
                reduce_windows,
                bases: bases_buffer,
                sorted: storage(device, "sorted", n * windows * 4),
                levels: Vec::new(),
                tree: [storage(device, "tree a", most * 128), storage(device, "tree b", most * 128)],
                buckets: storage(device, "buckets", buckets * 128),
                pairs: storage(device, "pairs", windows * blocks * 2 * 128),
                sums: storage(device, "sums", windows * 128),
                staging: device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("staging"),
                    size: (windows * 128) as u64,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                }),
                reduce_uniform: uniform(device, [half as u32, REDUCE_BLOCK as u32, REDUCE_BLOCK.trailing_zeros(), blocks as u32]),
                counts: vec![0; buckets],
                starts: vec![0; buckets],
                most,
                gpu,
            },
            compile_ms,
        ))
    }

    pub fn msm(&mut self, scalars: &[Fr]) -> (G1Projective, Timings) {
        assert_eq!(scalars.len(), self.n);
        let mut timings = Timings::default();
        let started = Instant::now();
        let sorted = self.sort(scalars);
        let levels = self.plan();
        timings.sort_ms = started.elapsed().as_secs_f64() * 1000.0;

        let started = Instant::now();
        let device = &self.gpu.device;
        let queue = &self.gpu.queue;
        queue.write_buffer(&self.sorted, 0, bytemuck_cast(&sorted));
        let mut encoder = device.create_command_encoder(&Default::default());
        encoder.clear_buffer(&self.buckets, 0, None);
        let last = levels.len() - 1;
        for (level, shards) in levels.iter().enumerate() {
            let pipeline = if level == 0 { &self.first } else { &self.tree_pipeline };
            let [start, count, dst, params] = &self.levels[level];
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
        let blocks = self.half.div_ceil(REDUCE_BLOCK);
        for (pipeline, threads) in [(&self.reduce_blocks, self.windows * blocks), (&self.reduce_windows, self.windows)] {
            let blocks_stage = std::ptr::eq(pipeline, &self.reduce_blocks);
            let mut entries = vec![
                wgpu::BindGroupEntry { binding: 1, resource: self.pairs.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: self.reduce_uniform.as_entire_binding() },
            ];
            entries.push(if blocks_stage {
                wgpu::BindGroupEntry { binding: 0, resource: self.buckets.as_entire_binding() }
            } else {
                wgpu::BindGroupEntry { binding: 2, resource: self.sums.as_entire_binding() }
            });
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &entries,
            });
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &group, &[]);
            let workgroup = if std::ptr::eq(pipeline, &self.reduce_windows) { 1 } else { WORKGROUP as usize };
            pass.dispatch_workgroups(threads.div_ceil(workgroup) as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&self.sums, 0, &self.staging, 0, (self.windows * 128) as u64);
        queue.submit([encoder.finish()]);
        let slice = self.staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |_| {});
        device.poll(wgpu::PollType::wait_indefinitely()).expect("gpu");
        let data = slice.get_mapped_range().expect("mapped sums").to_vec();
        self.staging.unmap();
        timings.gpu_ms = started.elapsed().as_secs_f64() * 1000.0;

        let started = Instant::now();
        let mut result = G1Projective::ZERO;
        for window in (0..self.windows).rev() {
            for _ in 0..self.width {
                result.double_in_place();
            }
            result += xyzz_to_projective(&data[window * 128..window * 128 + 128]);
        }
        timings.combine_ms = started.elapsed().as_secs_f64() * 1000.0;
        (result, timings)
    }

    /// Levels of shards over the sorted runs, into the level buffers.
    fn plan(&mut self) -> Vec<usize> {
        let buckets = self.windows * self.half;
        let mut runs: Vec<(u32, u32)> = self.starts.iter().zip(&self.counts).map(|(s, c)| (*s, *c)).collect();
        let mut shard_counts = Vec::new();
        let mut level = 0;
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
                    dst.push(if last { b as u32 } else { first + (at / size) });
                    at += len;
                }
                next.push((first, start.len() as u32 - first));
            }
            if self.levels.len() <= level {
                let device = &self.gpu.device;
                self.levels.push([
                    storage(device, "start", self.most * 4),
                    storage(device, "count", self.most * 4),
                    storage(device, "dst", self.most * 4),
                    uniform(device, [0; 4]),
                ]);
            }
            let queue = &self.gpu.queue;
            let [bs, bc, bd, bp] = &self.levels[level];
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

fn xyzz_to_projective(bytes: &[u8]) -> G1Projective {
    let (x, y, zz, zzz) = (fq(&bytes[..32]), fq(&bytes[32..64]), fq(&bytes[64..96]), fq(&bytes[96..128]));
    if zz.is_zero() {
        return G1Projective::ZERO;
    }
    G1Projective::new_unchecked(x * zz, y * zzz, zz)
}
