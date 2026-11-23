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
    mapped_upload: bool,
    degrees: wgpu::Buffer,
    size: wgpu::Buffer,
    weight: wgpu::ComputePipeline,
    product: wgpu::ComputePipeline,
    fused: Option<FusedEulerTiles>,
    paired: Option<PairedEulerTiles>,
}

#[cfg(target_os = "android")]
struct PairedEulerTiles {
    both: Option<(wgpu::ComputePipeline, wgpu::ComputePipeline)>,
    last: wgpu::ComputePipeline,
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
    #[cfg(target_os = "android")]
    direction: Option<[wgpu::ComputePipeline; 2]>,
    #[cfg(target_os = "android")]
    large: bool,
    #[cfg(target_os = "android")]
    packed_roots: bool,
}

struct TilePass {
    arguments: [wgpu::Buffer; 2],
    groups: [u32; 2],
    #[cfg(target_os = "android")]
    geometry: [u32; 5],
}

#[cfg(target_os = "android")]
struct FusedEulerTiles {
    pipeline: wgpu::ComputePipeline,
    midpoint: Option<wgpu::ComputePipeline>,
    arguments: [[wgpu::Buffer; 2]; 4],
}

#[cfg(target_os = "android")]
impl FusedEulerTiles {
    fn new(gpu: &Gpu, tiled: &TiledFft, active: u32, low: u32) -> Self {
        use Slot::{Read, Uniform, Write};
        let pipeline = quotient_kernel(
            gpu, "fr_fft_tile_euler", include_bytes!("../glsl/out/fr_fft_tile_euler.spv"),
            &[Write, Read, Uniform, Read, Read],
        );
        let midpoint = gpu.device.features().contains(wgpu::Features::SUBGROUP)
            .then(|| std::env::var_os("ZOPRO_VULKAN_EULER_MIDPOINT_SPV"))
            .flatten()
            .and_then(|path| std::fs::read(path).ok())
            .map(|kernel| {
                log::info!("ZOPRO_TRACE vulkan-euler subgroup midpoint");
                gpu.kernel("fr_euler_midpoint", || {
                    spirv_pipeline(&gpu.device, gpu.cache(), "fr_euler_midpoint", &kernel,
                        &[Write, Read, Uniform, Read, Read])
                })
            });
        let arguments = [0, 1, 2, 3].map(|mode| {
            let [n, first, levels, stride, cols] = if mode == 1 || mode == 3 {
                tiled.passes.last().expect("Euler tile").geometry
            } else {
                tiled.passes[0].geometry
            };
            [0, 1].map(|vector| crate::uniform_words(gpu, &[
                n, first, levels, stride, cols, u32::from(mode != 0), active, low, vector, mode, 0, 0,
            ]))
        });
        Self { pipeline, midpoint, arguments }
    }
}

#[cfg(target_os = "android")]
impl Drop for FusedEulerTiles {
    fn drop(&mut self) {
        for buffer in self.arguments.iter().flatten() {
            buffer.destroy();
        }
    }
}

fn quotient_kernel(gpu: &Gpu, entry: &'static str, builtin: &[u8], slots: &[Slot]) -> wgpu::ComputePipeline {
    gpu.kernel(entry, || {
        #[cfg(target_os = "android")]
        let builtin: &[u8] = if std::env::var("ZOPRO_VULKAN_WIDE_FR").as_deref() == Ok("1") {
            match entry {
                "fr_fft_tile" => include_bytes!("../glsl/out/fr_fft_tile_wide.spv"),
                "fr_fft_tile_large" => include_bytes!("../glsl/out/fr_fft_tile_large_wide.spv"),
                "fr_euler_degree" => include_bytes!("../glsl/out/fr_euler_degree_wide.spv"),
                "fr_euler_product" => include_bytes!("../glsl/out/fr_euler_product_wide.spv"),
                _ => builtin,
            }
        } else {
            builtin
        };
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

        let large = cfg!(target_os = "android")
            && std::env::var("ZOPRO_VULKAN_FFT_LARGE_TILE").as_deref() == Ok("1");
        let packed_roots = cfg!(target_os = "android")
            && std::env::var("ZOPRO_VULKAN_FFT_PACKED_ROOTS").as_deref() == Ok("1")
            && std::env::var("ZOPRO_VULKAN_WIDE_FR").as_deref() == Ok("1");
        let (name, kernel): (_, &[u8]) = match (large, packed_roots) {
            (true, true) => ("fr_fft_tile_large_roots_packed", include_bytes!("../glsl/out/fr_fft_tile_large_roots_packed.spv")),
            (false, true) => ("fr_fft_tile_roots_packed", include_bytes!("../glsl/out/fr_fft_tile_roots_packed.spv")),
            (true, false) => ("fr_fft_tile_large", include_bytes!("../glsl/out/fr_fft_tile_large.spv")),
            (false, false) => ("fr_fft_tile", include_bytes!("../glsl/out/fr_fft_tile.spv")),
        };
        let pipeline = quotient_kernel(gpu, name, kernel, &[Write, Read, Uniform]);
        #[cfg(target_os = "android")]
        let direction = if !large && !packed_roots && gpu.device.features().contains(wgpu::Features::SUBGROUP) {
            std::env::var_os("ZOPRO_ANDROID_FFT_DIRECTION_DIR").and_then(|path| {
                let path = std::path::PathBuf::from(path);
                let names = ["fr_fft_tile_dif", "fr_fft_tile_dit"];
                let inverse = std::fs::read(path.join(format!("{}.spv", names[0]))).ok()?;
                let forward = std::fs::read(path.join(format!("{}.spv", names[1]))).ok()?;
                log::info!("ZOPRO_TRACE quotient direction-specific FFT kernels");
                Some(std::array::from_fn(|index| {
                    let bytes = if index == 0 { &inverse } else { &forward };
                    gpu.kernel(names[index], || {
                        spirv_pipeline(&gpu.device, gpu.cache(), names[index], bytes,
                            &[Write, Read, Uniform])
                    })
                }))
            })
        } else {
            None
        };
        let max_levels = if large { 7 } else { 6 };
        let tile_bits = if large { 8 } else { 7 };
        let log_n = n.trailing_zeros();
        let layout = if cfg!(target_os = "android") {
            std::env::var("ZOPRO_VULKAN_FFT_LEVELS").ok().and_then(|value| {
                let levels = value.split(',').map(str::parse::<u32>).collect::<Result<Vec<_>, _>>().ok()?;
                let total = levels.iter().try_fold(0u32, |sum, &level| sum.checked_add(level))?;
                (total == log_n && levels.iter().all(|level| (1..=tile_bits).contains(level)))
                    .then_some(levels)
            })
        } else {
            None
        };
        let mut first = 0;
        let mut passes = Vec::new();
        while first < log_n {
            let remaining = log_n - first;
            let levels = layout.as_ref().map_or_else(
                || remaining.div_ceil(remaining.div_ceil(max_levels)),
                |levels| levels[passes.len()],
            );
            let stride_log = remaining - levels;
            let cols_log = (tile_bits - levels).min(stride_log);
            let groups = n >> (levels + cols_log);
            let x = groups.min(16384);
            let arguments = [0, 1].map(|forward| {
                crate::uniform_words(gpu, &[n, first, levels, stride_log, cols_log, forward, 0, 0])
            });
            passes.push(TilePass {
                arguments, groups: [x, groups / x],
                #[cfg(target_os = "android")]
                geometry: [n, first, levels, stride_log, cols_log],
            });
            first += levels;
        }
        Self {
            pipeline, passes,
            #[cfg(target_os = "android")]
            direction,
            #[cfg(target_os = "android")]
            large,
            #[cfg(target_os = "android")]
            packed_roots,
        }
    }

    /// Store each stage's roots in the order a tile's rows and columns use.
    /// The stages need n/2 + n/4 + ... + 1 values in either direction.
    #[cfg(target_os = "android")]
    fn pack_roots(&self, roots: &[Fr]) -> Vec<Fr> {
        let mut packed = Vec::with_capacity(2 * roots.len() - 1);
        for pass in &self.passes {
            let [n, first, levels, stride_log, _] = pass.geometry;
            for local in 0..levels {
                let level = first + local;
                let row_bits = levels - 1 - local;
                let row_mask = (1 << row_bits) - 1;
                let gap = n >> (level + 1);
                packed.extend((0..gap).map(|index| {
                    let phase = (index >> row_bits) | ((index & row_mask) << stride_log);
                    roots[(phase << level) as usize]
                }));
            }
        }
        packed
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
        let upload_roots = |label: &str, values: &[Fr]| {
            #[cfg(target_os = "android")]
            if let Some(tiled) = tiled.as_ref().filter(|tiled| tiled.packed_roots) {
                return upload(label, &tiled.pack_roots(values));
            }
            upload(label, values)
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
            roots: upload_roots("roots", roots),
            inverse_roots: upload_roots("inverse roots", inverse_roots),
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
        let fused = self.tiled.as_ref().filter(|tiled| {
            !tiled.large && !tiled.packed_roots && tiled.passes.len() > 1
                && std::env::var("ZOPRO_VULKAN_EULER_FUSED").as_deref() == Ok("1")
        }).map(|tiled| FusedEulerTiles::new(&self.gpu, tiled, active as u32, low as u32));
        let load_paired = || -> Result<_, String> {
            let Some(directory) = std::env::var_os("ZOPRO_ANDROID_VULKAN_PAIRED_EULER_DIR") else {
                return Ok(None);
            };
            let directory = std::path::PathBuf::from(directory);
            use Slot::{Read, Uniform, Write};
            let tiled = self.tiled.as_ref().ok_or("paired Euler requires a tiled transform")?;
            if fused.is_some() || tiled.packed_roots || tiled.large
                || self.abc.size() > device.limits().max_storage_buffer_binding_size as u64
            {
                return Err("paired Euler is incompatible with this quotient workspace".into());
            }
            let load = |name, slots: &[Slot]| crate::resident::load_pipeline(&self.gpu, &directory, name, slots);
            let last = load("fr_euler_last_product", &[Write, Read, Uniform, Read, Uniform])?;
            let both = if std::env::var("ZOPRO_ANDROID_VULKAN_PAIRED_EULER").as_deref() == Ok("1") {
                let tile = load("fr_euler_pair_tile", &[Write, Read, Uniform])?;
                let degree = load("fr_euler_pair_degree", &[Write, Read, Uniform])?;
                Some((tile, degree))
            } else {
                None
            };
            log::info!("ZOPRO_TRACE vulkan-euler last tile product paired={}", both.is_some());
            Ok(Some(PairedEulerTiles { both, last }))
        };
        let paired = load_paired();
        let mapped_upload = device.features().contains(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS);
        let rows = if mapped_upload {
            log::info!("ZOPRO_TRACE vulkan-euler mapped input rows");
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("mapped Euler input rows"),
                size: (2 * active * 32) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC
                    | wgpu::BufferUsages::MAP_WRITE,
                mapped_at_creation: false,
            })
        } else {
            storage(device, "Euler input rows", 2 * active * 32)
        };
        let workspace = EulerProduct {
            active,
            rows,
            mapped_upload,
            degrees,
            size: crate::uniform(&self.gpu, [self.n as u32, active as u32, low as u32, 0]),
            weight,
            product,
            fused,
            paired: None,
        };
        let errors = [memory, internal, validation].map(crate::popped);
        if let Some(error) = errors.into_iter().flatten().next() {
            return Err(format!("Euler quotient: {error}"));
        }
        if self.gpu.failed() {
            return Err("Euler quotient: the device reported an error".into());
        }
        let mut workspace = workspace;
        workspace.paired = paired?;
        self.euler = Some(workspace);
        self.gpu.save_cache();
        Ok(())
    }

    #[cfg(target_os = "android")]
    pub fn has_mapped_euler_input(&self) -> bool {
        self.euler.as_ref().is_some_and(|euler| euler.mapped_upload)
    }

    #[cfg(target_os = "android")]
    fn with_mapped_euler_input<R>(
        &self,
        write: impl FnOnce(&mut wgpu::BufferViewMut) -> Result<R, String>,
    ) -> Result<R, String> {
        struct Unmap<'a>(&'a wgpu::Buffer);
        impl Drop for Unmap<'_> {
            fn drop(&mut self) {
                self.0.unmap();
            }
        }
        let euler = self.euler.as_ref().expect("prepared Euler workspace");
        assert!(euler.mapped_upload);
        pollster::block_on(crate::map_buffer(&self.gpu, &euler.rows, wgpu::MapMode::Write, None))?;
        let _unmap = Unmap(&euler.rows);
        let mut view = euler.rows.slice(..).get_mapped_range_mut()
            .map_err(|error| format!("Euler input map: {error}"))?;
        write(&mut view)
    }

    /// Writes the two active evaluation rows directly into the Vulkan input.
    ///
    /// # Safety
    /// The callback may only write the initialized limbs and must fill both
    /// rows on success. It must neither read through nor retain the pointers.
    #[cfg(target_os = "android")]
    pub unsafe fn write_euler_input<R>(
        &mut self,
        write: impl FnOnce([*mut Fr; 2], usize) -> Result<R, String>,
    ) -> Result<R, String> {
        let rows = self.euler.as_ref().expect("prepared Euler workspace").active;
        self.with_mapped_euler_input(|view| {
            let length = view.len();
            let mut bytes = view.slice(..);
            let first = bytes.as_raw_element_ptr().cast::<Fr>().as_ptr();
            if !first.is_aligned() || length != 2 * rows * size_of::<Fr>() {
                return Err("invalid Vulkan input mapping alignment or length".into());
            }
            // SAFETY: first points into the live writable mapping with room
            // for two complete active vectors; neither pointer escapes write.
            let second = unsafe { first.add(rows) };
            write([first, second], rows)
        })
    }

    #[cfg(target_os = "android")]
    pub fn start_euler(
        &mut self,
        a: &[Fr],
        b: &[Fr],
        download_from: Option<usize>,
    ) -> Result<(), String> {
        let euler = self.euler.as_ref().expect("prepared Euler workspace");
        let active = euler.active;
        assert_eq!((a.len(), b.len()), (active, active));
        let active_bytes = active * size_of::<Fr>();
        if euler.mapped_upload {
            self.with_mapped_euler_input(|view| {
                view.slice(..active_bytes).copy_from_slice(fr_bytes(a));
                view.slice(active_bytes..).copy_from_slice(fr_bytes(b));
                Ok(())
            })?;
        } else {
            self.gpu.queue.write_buffer(&euler.rows, 0, fr_bytes(a));
            self.gpu.queue.write_buffer(&euler.rows, active_bytes as u64, fr_bytes(b));
        }
        self.start_prepared_euler(download_from);
        Ok(())
    }

    /// Dispatches transforms of the two evaluation vectors already uploaded.
    #[cfg(target_os = "android")]
    pub fn start_prepared_euler(&mut self, download_from: Option<usize>) {
        let euler = self.euler.as_ref().expect("prepared Euler workspace");
        let active = euler.active;
        assert!(download_from.is_none_or(|first| first < active));
        self.result_len = active - download_from.unwrap_or(active);
        let (device, queue) = (&self.gpu.device, &self.gpu.queue);
        let active_bytes = (active * 32) as u64;
        let domain_bytes = (self.n * 32) as u64;
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
        if let Some(PairedEulerTiles { both: Some((tile, degree)), .. }) = &euler.paired {
            let tiled = self.tiled.as_ref().expect("paired quotient tiles");
            for vector in 0..2u64 {
                let offset = vector * domain_bytes;
                if active_bytes < domain_bytes {
                    encoder.clear_buffer(&self.abc, offset + active_bytes, Some(domain_bytes - active_bytes));
                }
                encoder.copy_buffer_to_buffer(&euler.rows, vector * active_bytes, &self.abc, offset, active_bytes);
            }
            for stage in &tiled.passes {
                self.dispatch_euler_pair(&mut encoder, tile, stage, true);
            }
            encoder = crate::flush(&self.gpu, encoder);
            crate::resident::dispatch_grid(
                device, &mut encoder, degree, &[&self.abc, &euler.degrees, &euler.size],
                [(self.n as u32).div_ceil(WORKGROUP), 1, 2],
                &mut *self.timer.borrow_mut(), "fr_euler_pair_degree",
            );
            for stage in tiled.passes.iter().skip(1).rev() {
                self.dispatch_euler_pair(&mut encoder, tile, stage, false);
            }
            self.dispatch_tile(&mut encoder, 0, &self.roots, false, tiled, &tiled.passes[0]);
            self.dispatch_euler_last(&mut encoder, euler, &tiled.passes[0]);
        } else {
            for vector in 0..2u64 {
                if let Some(fused) = euler.fused.as_ref() {
                    self.transform_euler_tiles(&mut encoder, euler, fused, vector as usize, true);
                    encoder = crate::flush(&self.gpu, encoder);
                    self.transform_euler_tiles(&mut encoder, euler, fused, vector as usize, false);
                    encoder = crate::flush(&self.gpu, encoder);
                    continue;
                }
                let offset = vector * domain_bytes;
                if active_bytes < domain_bytes {
                    encoder.clear_buffer(&self.abc, offset + active_bytes, Some(domain_bytes - active_bytes));
                }
                encoder.copy_buffer_to_buffer(&euler.rows, vector * active_bytes, &self.abc, offset, active_bytes);
                self.transform(&mut encoder, offset, &self.inverse_roots, true);
                encoder = crate::flush(&self.gpu, encoder);
                dispatch(&mut encoder, "fr_euler_degree", &euler.weight, self.vector_binding(offset), &euler.degrees, self.n);
                if vector == 1 && euler.paired.is_some() {
                    let tiled = self.tiled.as_ref().expect("last quotient tile");
                    for stage in tiled.passes.iter().skip(1).rev() {
                        self.dispatch_tile(&mut encoder, offset, &self.roots, false, tiled, stage);
                    }
                    self.dispatch_euler_last(&mut encoder, euler, &tiled.passes[0]);
                } else {
                    self.transform(&mut encoder, offset, &self.roots, false);
                }
                encoder = crate::flush(&self.gpu, encoder);
            }
            if euler.fused.is_none() && euler.paired.is_none() {
                dispatch(&mut encoder, "fr_euler_product", &euler.product, self.abc.as_entire_binding(), &euler.rows, active);
            }
        }
        if let Some(first) = download_from {
            encoder.copy_buffer_to_buffer(&self.abc, (first * 32) as u64, &self.staging, 0, (self.result_len * 32) as u64);
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
        let field_bytes = std::mem::size_of::<Fr>();
        let bytes = n * field_bytes;
        let data = self
            .staging
            .slice(..bytes as u64)
            .get_mapped_range()
            .map_err(|e| format!("quotient: {e}"))?;
        let mut values = Vec::<Fr>::with_capacity(n);
        // SAFETY: the kernel encodes each Fr as four native u64 limbs. Copying
        // bytes needs no alignment from the mapped view. The destination is an
        // aligned allocation for n Fr, initialized by the copy before set_len.
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), values.as_mut_ptr().cast(), bytes);
            values.set_len(n);
        }
        drop(data);
        self.staging.unmap();
        Ok(values)
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
            self.dispatch_tile(encoder, offset, roots, dif, tiled, &tiled.passes[index]);
        }
    }

    fn dispatch_tile(
        &self, encoder: &mut wgpu::CommandEncoder, offset: u64, roots: &wgpu::Buffer,
        dif: bool, tiled: &TiledFft, stage: &TilePass,
    ) {
        let pipeline = &tiled.pipeline;
        #[cfg(target_os = "android")]
        let pipeline = tiled.direction.as_ref().map_or(pipeline, |directions| {
            &directions[usize::from(!dif)]
        });
        let group = self.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.get_bind_group_layout(0),
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
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.dispatch_workgroups(stage.groups[0], stage.groups[1], 1);
    }

    #[cfg(target_os = "android")]
    fn dispatch_euler_pair(
        &self, encoder: &mut wgpu::CommandEncoder, pipeline: &wgpu::ComputePipeline,
        stage: &TilePass, dif: bool,
    ) {
        let roots = if dif { &self.inverse_roots } else { &self.roots };
        crate::resident::dispatch_grid(
            &self.gpu.device, encoder, pipeline,
            &[&self.abc, roots, &stage.arguments[usize::from(!dif)]],
            [stage.groups[0], stage.groups[1], 2],
            &mut *self.timer.borrow_mut(), "fr_euler_pair_tile",
        );
    }

    #[cfg(target_os = "android")]
    fn dispatch_euler_last(
        &self, encoder: &mut wgpu::CommandEncoder, euler: &EulerProduct, stage: &TilePass,
    ) {
        let pipeline = &euler.paired.as_ref().expect("last quotient tile").last;
        crate::resident::dispatch_grid(
            &self.gpu.device, encoder, pipeline,
            &[&self.abc, &self.roots, &stage.arguments[1], &euler.rows, &euler.size],
            [stage.groups[0], stage.groups[1], 1],
            &mut *self.timer.borrow_mut(), "fr_euler_last_product",
        );
    }

    #[cfg(target_os = "android")]
    fn transform_euler_tiles(
        &self, encoder: &mut wgpu::CommandEncoder, euler: &EulerProduct,
        fused: &FusedEulerTiles, vector: usize, dif: bool,
    ) {
        let tiled = self.tiled.as_ref().expect("Euler transform tiles");
        let last = tiled.passes.len() - 1;
        let roots = if dif { &self.inverse_roots } else { &self.roots };
        let offset = (vector * self.n * 32) as u64;
        for step in 0..=last {
            let index = if dif { step } else { last - step };
            let stage = &tiled.passes[index];
            if !dif && step == 0 && fused.midpoint.is_some() {
                continue;
            }
            let mode = match (dif, step, vector) {
                (true, step, _) if step == last && fused.midpoint.is_some() => 3,
                (true, 0, _) => 0,
                (false, 0, _) => 1,
                (false, step, 1) if step == last => 2,
                _ => {
                    self.dispatch_tile(encoder, offset, roots, dif, tiled, stage);
                    continue;
                }
            };
            let (pipeline, auxiliary, label) = if mode == 3 {
                (fused.midpoint.as_ref().expect("Euler midpoint pipeline"),
                 &self.roots, "fr_euler_midpoint")
            } else {
                (&fused.pipeline, &euler.rows, "fr_fft_tile_euler")
            };
            let group = self.gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: self.abc.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: roots.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 2, resource: fused.arguments[mode][vector].as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 3, resource: auxiliary.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 4, resource: euler.degrees.as_entire_binding() },
                ],
            });
            let mut pass = self.begin(encoder, label);
            pass.set_pipeline(pipeline);
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
