//! The quotient polynomial's coefficients through wgpu, as ringil-metal's:
//! inverse DIF, scale, forward DIT per vector, the pointwise product, and
//! the inverse DIF with the denominator's weights, in radix-2 stages over
//! global memory.
use crate::time::Instant;
use std::sync::Arc;

use ark_bn254::Fr;

use crate::{spirv_pipeline, storage, Gpu, Slot};

const FR_SHADER: &str = concat!(
    include_str!("../shaders/fr.wgsl"),
    "\n",
    include_str!("../shaders/quotient.wgsl")
);
const WORKGROUP: u32 = 64;
/// The kernels pre-compiled for Vulkan (glsl/), where the MSMs' are: the
/// Adreno 830 got naga's SPIR-V of quotient.wgsl wrong.
const SPIRV: [(&str, &[u8]); 4] = [
    (
        "fr_dif_stage",
        include_bytes!("../glsl/out/fr_dif_stage.spv"),
    ),
    (
        "fr_dit_stage",
        include_bytes!("../glsl/out/fr_dit_stage.spv"),
    ),
    ("fr_scale", include_bytes!("../glsl/out/fr_scale.spv")),
    (
        "fr_pointwise",
        include_bytes!("../glsl/out/fr_pointwise.spv"),
    ),
];

pub struct Quotient {
    gpu: Arc<Gpu>,
    /// Running the pre-compiled kernels: the pointwise kernel's abc is
    /// binding 0 of set 0, its uniform binding 1.
    spirv: bool,
    /// The submitted transform's, for waiting on it alone.
    submission: Option<wgpu::SubmissionIndex>,
    dif: wgpu::ComputePipeline,
    dit: wgpu::ComputePipeline,
    scale: wgpu::ComputePipeline,
    pointwise: wgpu::ComputePipeline,
    n: usize,
    log_n: u32,
    roots: wgpu::Buffer,
    inverse_roots: wgpu::Buffer,
    forward_weights: wgpu::Buffer,
    reverse_weights: wgpu::Buffer,
    abc: wgpu::Buffer,
    staging: wgpu::Buffer,
    /// One uniform per distinct (level, kind) of stage: DIF/DIT levels and
    /// the whole-vector passes.
    stages: Vec<wgpu::Buffer>,
    whole: wgpu::Buffer,
}

fn fr_bytes(values: &[Fr]) -> &[u8] {
    // SAFETY: Fr is four u64 limbs with no padding, the kernel's encoding.
    unsafe { std::slice::from_raw_parts(values.as_ptr() as *const u8, values.len() * 32) }
}

impl Quotient {
    pub fn new(
        gpu: Arc<Gpu>,
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        forward_weights: &[Fr],
        reverse_weights: &[Fr],
    ) -> Result<(Self, f64), String> {
        assert!(n.is_power_of_two() && n >= 8);
        let device = &gpu.device;
        let started = Instant::now();
        let validation = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let internal = device.push_error_scope(wgpu::ErrorFilter::Internal);
        let spirv = gpu.spirv;
        let module = (!spirv).then(|| {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("ringil quotient"),
                source: wgpu::ShaderSource::Wgsl(FR_SHADER.into()),
            })
        });
        let pipeline = |index: usize| {
            let (entry, kernel) = SPIRV[index];
            if spirv {
                use Slot::*;
                let slots: &[Slot] = if index == 3 {
                    &[Write, Uniform]
                } else {
                    &[Write, Read, Uniform]
                };
                return spirv_pipeline(device, gpu.cache(), entry, kernel, slots);
            }
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: None,
                module: module.as_ref().expect("WGSL module"),
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: gpu.cache(),
            })
        };
        let mut pipelines = Vec::with_capacity(4);
        for (index, (entry, _)) in SPIRV.iter().enumerate() {
            pipelines.push(pipeline(index));
            if gpu.failed() {
                return Err(format!("kernel {entry}: the device reported an error"));
            }
        }
        let [dif, dit, scale, pointwise]: [wgpu::ComputePipeline; 4] =
            pipelines.try_into().expect("four kernels");
        for scope in [internal, validation] {
            if let Some(e) = crate::popped(scope) {
                return Err(format!("kernels: {e}"));
            }
        }
        if gpu.failed() {
            return Err("kernels: the device reported an error".into());
        }
        gpu.save_cache();
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let memory = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let upload = |label: &str, values: &[Fr]| {
            let buffer = storage(device, label, values.len() * 32);
            gpu.queue.write_buffer(&buffer, 0, fr_bytes(values));
            buffer
        };
        let log_n = n.trailing_zeros();
        let stages = (0..log_n)
            .map(|level| crate::uniform(&gpu, [n as u32, level, log_n - 1 - level, 0]))
            .collect();
        let quotient = Self {
            spirv,
            submission: None,
            dif,
            dit,
            scale,
            pointwise,
            n,
            log_n,
            roots: upload("roots", roots),
            inverse_roots: upload("inverse roots", inverse_roots),
            forward_weights: upload("forward weights", forward_weights),
            reverse_weights: upload("reverse weights", reverse_weights),
            abc: storage(device, "abc", 3 * n * 32),
            staging: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("quotient staging"),
                size: (n * 32) as u64,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            stages,
            whole: crate::uniform(&gpu, [n as u32, 0, 0, 0]),
            gpu,
        };
        if let Some(e) = crate::popped(memory) {
            return Err(format!("buffers: {e}"));
        }
        if quotient.gpu.failed() {
            return Err("buffers: the device reported an error".into());
        }
        Ok((quotient, compile_ms))
    }

    /// The quotient's coefficients times the denominator, bit-reversed, the
    /// first n - 1, from evaluations of a, b and c (padded with zeros to n).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn quotient(&mut self, a: &[Fr], b: &[Fr], c: &[Fr]) -> Result<Vec<Fr>, String> {
        self.start(a, b, c);
        self.finish()
    }

    /// The submitted quotient, waiting for the device.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn finish(&mut self) -> Result<Vec<Fr>, String> {
        pollster::block_on(self.finish_async())
    }

    /// Submits the quotient's transforms; `finish_async` collects them.
    pub fn start(&mut self, a: &[Fr], b: &[Fr], c: &[Fr]) {
        let n = self.n;
        let (device, queue) = (&self.gpu.device, &self.gpu.queue);
        let zero = Fr::from(0u64);
        for (vector, source) in [a, b, c].into_iter().enumerate() {
            let mut padded = Vec::with_capacity(n);
            padded.extend_from_slice(source);
            padded.resize(n, zero);
            queue.write_buffer(&self.abc, (vector * n * 32) as u64, fr_bytes(&padded));
        }
        let mut encoder = device.create_command_encoder(&Default::default());
        for vector in 0..3 {
            let offset = (vector * n * 32) as u64;
            self.transform(&mut encoder, offset, &self.inverse_roots, true);
            self.scale_by(&mut encoder, offset, &self.forward_weights);
            self.transform(&mut encoder, offset, &self.roots, false);
        }
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&self.pointwise);
            if self.spirv {
                let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &self.pointwise.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.abc.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: self.whole.as_entire_binding(),
                        },
                    ],
                });
                pass.set_bind_group(0, &group, &[]);
            } else {
                let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &self.pointwise.get_bind_group_layout(0),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.whole.as_entire_binding(),
                    }],
                });
                let group1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &self.pointwise.get_bind_group_layout(1),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.abc.as_entire_binding(),
                    }],
                });
                pass.set_bind_group(0, &group, &[]);
                pass.set_bind_group(1, &group1, &[]);
            }
            pass.dispatch_workgroups((n as u32).div_ceil(WORKGROUP), 1, 1);
        }
        self.transform(&mut encoder, 0, &self.inverse_roots, true);
        self.scale_by(&mut encoder, 0, &self.reverse_weights);
        encoder.copy_buffer_to_buffer(&self.abc, 0, &self.staging, 0, (n * 32) as u64);
        self.submission = Some(queue.submit([encoder.finish()]));
    }

    /// The submitted quotient, the first n - 1 coefficients.
    pub async fn finish_async(&mut self) -> Result<Vec<Fr>, String> {
        let n = self.n;
        crate::mapped(&self.gpu, &self.staging, self.submission.take()).await?;
        let data = self
            .staging
            .slice(..)
            .get_mapped_range()
            .map_err(|e| format!("quotient: {e}"))?
            .to_vec();
        self.staging.unmap();
        // SAFETY: n Fr in the kernel's encoding, four u64 limbs each.
        let values: &[Fr] = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const Fr, n) };
        Ok(values[..n - 1].to_vec())
    }

    fn vector_binding(&self, offset: u64) -> wgpu::BindingResource<'_> {
        wgpu::BindingResource::Buffer(wgpu::BufferBinding {
            buffer: &self.abc,
            offset,
            size: Some(std::num::NonZeroU64::new((self.n * 32) as u64).unwrap()),
        })
    }

    /// DIF from the widest gap to one (bit-reversed out) when `dif`, else
    /// DIT from gap one to the widest (bit-reversed in).
    fn transform(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        offset: u64,
        roots: &wgpu::Buffer,
        dif: bool,
    ) {
        let pipeline = if dif { &self.dif } else { &self.dit };
        let levels: Vec<u32> = if dif {
            (0..self.log_n).collect()
        } else {
            (0..self.log_n).rev().collect()
        };
        for level in levels {
            let group = self
                .gpu
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: self.vector_binding(offset),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: roots.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: self.stages[level as usize].as_entire_binding(),
                        },
                    ],
                });
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups((self.n as u32 / 2).div_ceil(WORKGROUP), 1, 1);
        }
    }

    fn scale_by(&self, encoder: &mut wgpu::CommandEncoder, offset: u64, weights: &wgpu::Buffer) {
        let group = self
            .gpu
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.scale.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.vector_binding(offset),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: weights.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: self.whole.as_entire_binding(),
                    },
                ],
            });
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&self.scale);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups((self.n as u32).div_ceil(WORKGROUP), 1, 1);
    }
}
