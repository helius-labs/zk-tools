//! The quotient polynomial's coefficients through wgpu, as zopro-metal's:
//! inverse DIF, scale, forward DIT per vector, the pointwise product, and
//! the inverse DIF with the denominator's weights, in radix-2 stages over
//! global memory.
use crate::time::Instant;
use std::sync::Arc;

use zopro_bn254::Fr;

use crate::{spirv_pipeline, storage, Gpu, Slot};

pub(crate) const LABEL: &str = "zopro quotient";
pub(crate) const ENTRIES: [&str; 4] = ["fr_dif_stage", "fr_dit_stage", "fr_scale", "fr_pointwise"];
pub(crate) const FR_SHADER: &str = concat!(
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
    monomial_weights: Option<[wgpu::Buffer; 2]>,
    abc: wgpu::Buffer,
    staging: wgpu::Buffer,
    result_len: usize,
    /// One uniform per distinct (level, kind) of stage: DIF/DIT levels and
    /// the whole-vector passes.
    stages: Vec<wgpu::Buffer>,
    tiled: Option<TiledFft>,
    #[cfg(target_os = "android")]
    euler: Option<EulerProduct>,
    whole: wgpu::Buffer,
    /// The submitted transform's pass times, when profiling.
    timer: std::cell::RefCell<Option<crate::profile::Timer>>,
}

#[cfg(target_os = "android")]
struct EulerProduct {
    active: usize,
    rows: wgpu::Buffer,
    degrees: wgpu::Buffer,
    size: wgpu::Buffer,
    weight: wgpu::ComputePipeline,
    product: wgpu::ComputePipeline,
}

#[cfg(target_os = "android")]
impl Drop for EulerProduct {
    fn drop(&mut self) {
        for buffer in [&self.rows, &self.degrees, &self.size] {
            buffer.destroy();
        }
    }
}

struct TiledFft {
    pipeline: wgpu::ComputePipeline,
    passes: Vec<TilePass>,
}

struct TilePass {
    arguments: [wgpu::Buffer; 2],
    groups: [u32; 2],
}

fn quotient_kernel(gpu: &Gpu, entry: &'static str, builtin: &[u8], slots: &[Slot]) -> wgpu::ComputePipeline {
    gpu.kernel(entry, || {
        #[cfg(target_os = "android")]
        let candidate = std::env::var_os("ZOPRO_VULKAN_QUOTIENT_SPV_DIR")
            .and_then(|directory| std::fs::read(std::path::PathBuf::from(directory).join(format!("{entry}.spv"))).ok());
        #[cfg(target_os = "android")]
        let builtin = match candidate.as_deref() {
            Some(bytes) => {
                log::info!("ZOPRO_TRACE quotient override {entry}");
                bytes
            }
            None => builtin,
        };
        spirv_pipeline(&gpu.device, gpu.cache(), entry, builtin, slots)
    })
}

impl TiledFft {
    fn new(gpu: &Gpu, n: u32) -> Self {
        use Slot::{Read, Uniform, Write};

        let kernel = include_bytes!("../glsl/out/fr_fft_tile.spv");
        let pipeline = quotient_kernel(gpu, "fr_fft_tile", kernel, &[Write, Read, Uniform]);
        let log_n = n.trailing_zeros();
        let mut first = 0;
        let mut passes = Vec::new();
        while first < log_n {
            let remaining = log_n - first;
            let levels = remaining.div_ceil(remaining.div_ceil(6));
            let stride_log = remaining - levels;
            let cols_log = (7 - levels).min(stride_log);
            let groups = n >> (levels + cols_log);
            let x = groups.min(16384);
            let arguments = [0, 1].map(|forward| {
                crate::uniform_words(gpu, &[n, first, levels, stride_log, cols_log, forward, 0, 0])
            });
            passes.push(TilePass { arguments, groups: [x, groups / x] });
            first += levels;
        }
        Self { pipeline, passes }
    }
}

impl Drop for TiledFft {
    fn drop(&mut self) {
        for pass in &self.passes {
            for buffer in &pass.arguments {
                buffer.destroy();
            }
        }
    }
}

/// As the MSMs' buffers: destroyed, not left to a browser's collector.
impl Drop for Quotient {
    fn drop(&mut self) {
        let uniforms = self.stages.iter().chain([&self.whole]);
        for buffer in [
            &self.roots,
            &self.inverse_roots,
            &self.abc,
            &self.staging,
        ]
        .into_iter()
        .chain(self.monomial_weights.iter().flatten())
        .chain(uniforms)
        {
            buffer.destroy();
        }
    }
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
        Self::create(gpu, n, roots, inverse_roots, Some([forward_weights, reverse_weights]))
    }

    #[cfg(target_os = "android")]
    pub fn new_euler(
        gpu: Arc<Gpu>,
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        active: usize,
    ) -> Result<Self, String> {
        let (mut quotient, _) = Self::create(gpu, n, roots, inverse_roots, None)?;
        quotient.prepare_euler_product(active)?;
        Ok(quotient)
    }

    fn create(
        gpu: Arc<Gpu>,
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        monomial_weights: Option<[&[Fr]; 2]>,
    ) -> Result<(Self, f64), String> {
        assert!(n.is_power_of_two() && n >= 8);
        let device = &gpu.device;
        let started = Instant::now();
        let validation = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let internal = device.push_error_scope(wgpu::ErrorFilter::Internal);
        let spirv = gpu.spirv;
        let pipeline = |index: usize| {
            let (entry, kernel) = SPIRV[index];
            if spirv {
                use Slot::*;
                let slots: &[Slot] = if index == 3 {
                    &[Write, Uniform]
                } else {
                    &[Write, Read, Uniform]
                };
                return quotient_kernel(&gpu, entry, kernel, slots);
            }
            gpu.wgsl_kernel(LABEL, FR_SHADER, entry)
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
        let tiled = if spirv && std::env::var("ZOPRO_VULKAN_FFT_TILE").as_deref() == Ok("1") {
            Some(TiledFft::new(&gpu, n as u32))
        } else {
            None
        };
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
            monomial_weights: monomial_weights.map(|[forward, reverse]| {
                [upload("forward weights", forward), upload("reverse weights", reverse)]
            }),
            abc: storage(device, "quotient vectors", (2 + usize::from(monomial_weights.is_some())) * n * 32),
            staging: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("quotient staging"),
                size: (n * 32) as u64,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            result_len: n - 1,
            stages,
            tiled,
            #[cfg(target_os = "android")]
            euler: None,
            whole: crate::uniform(&gpu, [n as u32, 0, 0, 0]),
            timer: std::cell::RefCell::new(None),
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
        self.result_len = n - 1;
        let [forward_weights, reverse_weights] = self.monomial_weights.as_ref().expect("monomial quotient");
        let (device, queue) = (&self.gpu.device, &self.gpu.queue);
        let zero = Fr::from(0u64);
        for (vector, source) in [a, b, c].into_iter().enumerate() {
            let mut padded = Vec::with_capacity(n);
            padded.extend_from_slice(source);
            padded.resize(n, zero);
            queue.write_buffer(&self.abc, (vector * n * 32) as u64, fr_bytes(&padded));
        }
        // Four transforms of log n passes, two scalings and the pointwise.
        *self.timer.borrow_mut() =
            crate::profile::Timer::new(&self.gpu, 4 * 3 * self.log_n as usize + 8);
        // A submission per transform: no submission holds the whole
        // quotient (see SORT_SLICE).
        let mut encoder = device.create_command_encoder(&Default::default());
        for vector in 0..3 {
            let offset = (vector * n * 32) as u64;
            self.transform(&mut encoder, offset, &self.inverse_roots, true);
            encoder = crate::flush(&self.gpu, encoder);
            self.scale_by(&mut encoder, offset, forward_weights);
            self.transform(&mut encoder, offset, &self.roots, false);
            encoder = crate::flush(&self.gpu, encoder);
        }
        {
            let mut pass = self.begin(&mut encoder, "fr_pointwise");
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
        encoder = crate::flush(&self.gpu, encoder);
        self.transform(&mut encoder, 0, &self.inverse_roots, true);
        self.scale_by(&mut encoder, 0, reverse_weights);
        encoder.copy_buffer_to_buffer(&self.abc, 0, &self.staging, 0, (n * 32) as u64);
        if let Some(timer) = self.timer.borrow().as_ref() {
            timer.resolve(&mut encoder);
        }
        self.submission = Some(queue.submit([encoder.finish()]));
    }

    /// Use the original domain's Euler derivative and keep only active rows.
    #[cfg(target_os = "android")]
    pub fn prepare_euler_product(&mut self, active: usize) -> Result<(), String> {
        if !self.spirv || !(1..=self.n).contains(&active) {
            return Err("Euler product requires Vulkan and valid active rows".into());
        }
        let (device, queue) = (&self.gpu.device, &self.gpu.queue);
        let validation = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let internal = device.push_error_scope(wgpu::ErrorFilter::Internal);
        let memory = device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let pipeline = |entry, kernel| {
            use Slot::{Read, Uniform, Write};
            quotient_kernel(&self.gpu, entry, kernel, &[Write, Read, Uniform])
        };
        let weight = pipeline("fr_euler_degree", include_bytes!("../glsl/out/fr_euler_degree.spv"));
        let product = pipeline("fr_euler_product", include_bytes!("../glsl/out/fr_euler_product.spv"));
        // Bit reversal distributes over the low and high index slices.
        let low = 1 << (self.log_n / 2);
        let degree = |i: usize| Fr::from((i.reverse_bits() >> (usize::BITS - self.log_n)) as u64);
        let table: Vec<_> = (0..low).map(degree).chain((0..self.n / low).map(|i| degree(i * low))).collect();
        let degrees = storage(device, "Euler degrees", table.len() * 32);
        queue.write_buffer(&degrees, 0, fr_bytes(&table));
        let workspace = EulerProduct {
            active,
            rows: storage(device, "Euler input rows", 2 * active * 32),
            degrees,
            size: crate::uniform(&self.gpu, [self.n as u32, active as u32, low as u32, 0]),
            weight,
            product,
        };
        let errors = [memory, internal, validation].map(crate::popped);
        if let Some(error) = errors.into_iter().flatten().next() {
            return Err(format!("Euler quotient: {error}"));
        }
        if self.gpu.failed() {
            return Err("Euler quotient: the device reported an error".into());
        }
        self.euler = Some(workspace);
        self.gpu.save_cache();
        Ok(())
    }

    #[cfg(target_os = "android")]
    pub fn start_euler(&mut self, a: &[Fr], b: &[Fr], download: bool) {
        let euler = self.euler.as_ref().expect("prepared Euler workspace");
        let active = euler.active;
        assert_eq!((a.len(), b.len()), (active, active));
        self.result_len = active;
        let (device, queue) = (&self.gpu.device, &self.gpu.queue);
        let active_bytes = (active * 32) as u64;
        let domain_bytes = (self.n * 32) as u64;
        queue.write_buffer(&euler.rows, 0, fr_bytes(a));
        queue.write_buffer(&euler.rows, active_bytes, fr_bytes(b));
        *self.timer.borrow_mut() = crate::profile::Timer::new(&self.gpu, 4 * self.log_n as usize + 4);
        let mut encoder = device.create_command_encoder(&Default::default());
        let dispatch = |encoder: &mut wgpu::CommandEncoder, label, pipeline: &wgpu::ComputePipeline,
                        values, input: &wgpu::Buffer, count: usize| {
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: values },
                    wgpu::BindGroupEntry { binding: 1, resource: input.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 2, resource: euler.size.as_entire_binding() },
                ],
            });
            let mut pass = self.begin(encoder, label);
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups((count as u32).div_ceil(WORKGROUP), 1, 1);
        };
        for vector in 0..2 {
            let offset = vector * domain_bytes;
            if active_bytes < domain_bytes {
                encoder.clear_buffer(&self.abc, offset + active_bytes, Some(domain_bytes - active_bytes));
            }
            encoder.copy_buffer_to_buffer(&euler.rows, vector * active_bytes, &self.abc, offset, active_bytes);
            self.transform(&mut encoder, offset, &self.inverse_roots, true);
            encoder = crate::flush(&self.gpu, encoder);
            dispatch(&mut encoder, "fr_euler_degree", &euler.weight, self.vector_binding(offset), &euler.degrees, self.n);
            self.transform(&mut encoder, offset, &self.roots, false);
            encoder = crate::flush(&self.gpu, encoder);
        }
        dispatch(&mut encoder, "fr_euler_product", &euler.product, self.abc.as_entire_binding(), &euler.rows, active);
        if download {
            encoder.copy_buffer_to_buffer(&self.abc, 0, &self.staging, 0, active_bytes);
        }
        if let Some(timer) = self.timer.borrow().as_ref() {
            timer.resolve(&mut encoder);
        }
        self.submission = Some(queue.submit([encoder.finish()]));
    }

    /// The buffer whose first n - 1 Fr are the submitted quotient's
    /// coefficients (for a GPU consumer that skips the read back).
    pub fn output(&self) -> &wgpu::Buffer {
        &self.abc
    }

    /// Read timestamps without downloading the quotient used by a GPU MSM.
    #[cfg(target_os = "android")]
    pub fn finish_device_profile(&mut self) -> Result<(), String> {
        if let Some(timer) = self.timer.get_mut().take() {
            let submission = self.submission.clone().expect("submitted quotient timestamps");
            pollster::block_on(timer.read(&self.gpu, submission))?;
        }
        Ok(())
    }

    /// The submitted quotient, the first n - 1 coefficients.
    pub async fn finish_async(&mut self) -> Result<Vec<Fr>, String> {
        let n = self.result_len;
        let submission = self.submission.take();
        crate::mapped(&self.gpu, &self.staging, submission.clone()).await?;
        let timer = self.timer.borrow_mut().take();
        if let (Some(timer), Some(submission)) = (timer, submission) {
            timer.read(&self.gpu, submission).await?;
        }
        let data = self
            .staging
            .slice(..(n * 32) as u64)
            .get_mapped_range()
            .map_err(|e| format!("quotient: {e}"))?
            .to_vec();
        self.staging.unmap();
        // SAFETY: n Fr in the kernel's encoding, four u64 limbs each.
        let values: &[Fr] = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const Fr, n) };
        Ok(values.to_vec())
    }

    /// A compute pass, timed as `kernel`'s when profiling.
    fn begin<'e>(
        &self,
        encoder: &'e mut wgpu::CommandEncoder,
        kernel: &'static str,
    ) -> wgpu::ComputePass<'e> {
        match self.timer.borrow_mut().as_mut() {
            Some(timer) => encoder.begin_compute_pass(&timer.pass(kernel)),
            None => encoder.begin_compute_pass(&Default::default()),
        }
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
        if let Some(tiled) = &self.tiled {
            self.transform_tiled(encoder, offset, roots, dif, tiled);
            return;
        }
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
            let mut pass = self.begin(encoder, if dif { "fr_dif_stage" } else { "fr_dit_stage" });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups((self.n as u32 / 2).div_ceil(WORKGROUP), 1, 1);
        }
    }

    fn transform_tiled(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        offset: u64,
        roots: &wgpu::Buffer,
        dif: bool,
        tiled: &TiledFft,
    ) {
        for pass in 0..tiled.passes.len() {
            let index = if dif { pass } else { tiled.passes.len() - 1 - pass };
            let stage = &tiled.passes[index];
            let group = self.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &tiled.pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: self.vector_binding(offset) },
                    wgpu::BindGroupEntry { binding: 1, resource: roots.as_entire_binding() },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: stage.arguments[usize::from(!dif)].as_entire_binding(),
                    },
                ],
            });
            let mut pass = self.begin(encoder, "fr_fft_tile");
            pass.set_pipeline(&tiled.pipeline);
            pass.set_bind_group(0, &group, &[]);
            pass.dispatch_workgroups(stage.groups[0], stage.groups[1], 1);
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
        let mut pass = self.begin(encoder, "fr_scale");
        pass.set_pipeline(&self.scale);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups((self.n as u32).div_ceil(WORKGROUP), 1, 1);
    }
}
