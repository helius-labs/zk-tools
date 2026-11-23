//! The quotient polynomial's coefficients on the GPU: the three evaluation
//! vectors A and B are moved to the domain's coset (inverse DIF, scale,
//! forward DIT), multiplied pointwise, and brought back. C needs only its
//! inverse DIF because subtraction is linear. The final pass applies the
//! denominator and subtracts C in coefficient form. The tables are the CPU
//! plan's: half the roots of each direction,
//! and the bit-reversed coset weights in two small tables each (Weights).
use std::{ffi::c_void, mem::MaybeUninit, time::Instant};

use zopro_bn254::Fr;
use metal::{
    Buffer, CommandQueue, ComputeCommandEncoderRef, ComputePipelineState, Device,
    MTLResourceOptions, MTLSize,
};

/// One direction's bit-reversed coset weights, as the CPU plan keeps them:
/// element i's is chunks[i >> low_bits] * lows[i & (2^low_bits - 1)], so the
/// GPU holds two small tables rather than n elements (32 MB for both
/// directions at 2^19).
pub struct Weights<'a> {
    pub lows: &'a [Fr],
    pub chunks: &'a [Fr],
    pub low_bits: u32,
}

/// A quotient submitted to the GPU and not yet collected.
pub struct PendingQuotient {
    command: metal::CommandBuffer,
    digits: Option<QuotientDigits>,
    entries: usize,
    canonical: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum QuotientC {
    Supplied,
    Derived,
    EulerProduct,
}

/// Signed MSM digits produced by a completed quotient command.
pub struct QuotientDigits {
    buffer: Buffer,
    pub(super) scatter: Option<DigitScatter>,
    pub(super) n: usize,
    pub(super) width: usize,
    windows: usize,
}

pub(super) struct DigitScatter {
    pub(super) counts: Buffer,
    pipeline: ComputePipelineState,
}

impl DigitScatter {
    pub(super) fn encode(
        &self,
        encoder: &ComputeCommandEncoderRef,
        digits: &QuotientDigits,
        starts: &Buffer,
        positions: &Buffer,
        sorted: &Buffer,
    ) {
        let args = digits.scatter_args();
        encoder.set_compute_pipeline_state(&self.pipeline);
        encoder.set_buffer(0, Some(&digits.buffer), 0);
        encoder.set_buffer(1, Some(starts), 0);
        encoder.set_buffer(2, Some(&self.counts), 0);
        encoder.set_buffer(3, Some(positions), 0);
        encoder.set_buffer(4, Some(sorted), 0);
        encoder.set_bytes(5, 16, args.as_ptr().cast());
        digits.dispatch(encoder, &self.pipeline);
    }
}

impl QuotientDigits {
    fn scatter_args(&self) -> [u32; 4] {
        [
            self.n as u32,
            1 << (self.width - 1),
            self.windows as u32,
            crate::DIGIT_CHUNK as u32,
        ]
    }

    fn dispatch(&self, encoder: &ComputeCommandEncoderRef, pipeline: &ComputePipelineState) {
        let group = (pipeline.thread_execution_width() * 4)
            .min(pipeline.max_total_threads_per_threadgroup())
            .min(self.n as u64);
        encoder.dispatch_threads(
            MTLSize::new(self.n as u64, self.windows as u64, 1),
            MTLSize::new(group, 1, 1),
        );
    }

    pub(super) fn as_slice(&self) -> &[i16] {
        // SAFETY: the buffer was uniquely allocated for this command, which
        // has completed before the digits can be read.
        unsafe {
            std::slice::from_raw_parts(
                self.buffer.contents() as *const i16,
                self.n * self.windows,
            )
        }
    }
}

pub struct Quotient {
    queue: CommandQueue,
    dif: ComputePipelineState,
    dit: ComputePipelineState,
    derive_c: Option<ComputePipelineState>,
    tensor_fft: Option<crate::tensor_fft::TensorFft>,
    /// The DIF passes, widest gaps first; the DIT passes run them reversed.
    passes: Vec<Pass>,
    scale: ComputePipelineState,
    product: ComputePipelineState,
    finish_quotient: ComputePipelineState,
    encode_digits: Option<ComputePipelineState>,
    euler: Option<EulerProduct>,
    sort_digits: Option<[ComputePipelineState; 2]>,
    n: usize,
    roots: Buffer,
    inverse_roots: Buffer,
    /// Each direction's weights: lows, then chunks.
    forward_weights: [Buffer; 2],
    reverse_weights: [Buffer; 2],
    low_bits: u32,
    abc: Buffer,
    euler_workspace: bool,
}

struct EulerProduct {
    weight: ComputePipelineState,
    product: ComputePipelineState,
    weights: [Buffer; 2],
    rows: Buffer,
    active: usize,
    fused: Option<FusedEuler>,
}

struct FusedEuler {
    inverse: ComputePipelineState,
    digits: ComputePipelineState,
    canonical_weights: [Buffer; 2],
    loose: Option<[ComputePipelineState; 2]>,
    final_pass: Option<ComputePipelineState>,
}

impl Drop for EulerProduct {
    fn drop(&mut self) {
        self.weights.iter().chain([&self.rows]).for_each(crate::discard);
        if let Some(fused) = &self.fused {
            fused.canonical_weights.iter().for_each(crate::discard);
        }
    }
}

impl Drop for Quotient {
    fn drop(&mut self) {
        objc::rc::autoreleasepool(|| {
            for buffer in [&self.roots, &self.inverse_roots, &self.abc]
                .into_iter()
                .chain(&self.forward_weights)
                .chain(&self.reverse_weights)
            {
                crate::discard(buffer);
            }
        })
    }
}

fn upload(device: &Device, values: &[Fr]) -> Buffer {
    device.new_buffer_with_data(
        values.as_ptr() as *const c_void,
        (values.len() * 32).max(16) as u64,
        MTLResourceOptions::StorageModeShared,
    )
}

impl Quotient {
    /// `roots` and `inverse_roots` are the first n/2 powers of the domain
    /// generator and its inverse; the weights are the plan's, bit-reversed.
    pub fn new(
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        forward_weights: Weights,
        reverse_weights: Weights,
    ) -> Result<(Self, f64), String> {
        objc::rc::autoreleasepool(|| {
            Self::new_in_pool(n, roots, inverse_roots, forward_weights, reverse_weights, false)
        })
    }

    #[doc(hidden)]
    pub fn new_euler_workspace(
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        forward_weights: Weights,
        reverse_weights: Weights,
    ) -> Result<(Self, f64), String> {
        objc::rc::autoreleasepool(|| {
            Self::new_in_pool(n, roots, inverse_roots, forward_weights, reverse_weights, true)
        })
    }

    fn new_in_pool(
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        forward_weights: Weights,
        reverse_weights: Weights,
        euler_workspace: bool,
    ) -> Result<(Self, f64), String> {
        assert!(n.is_power_of_two() && n >= 8);
        assert_eq!(roots.len(), n / 2);
        for weights in [&forward_weights, &reverse_weights] {
            assert_eq!(weights.lows.len(), 1 << weights.low_bits);
            assert_eq!(weights.lows.len() * weights.chunks.len(), n);
        }
        assert_eq!(forward_weights.low_bits, reverse_weights.low_bits);
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let pipeline = |name: &str| crate::pipeline(&device, false, name);
        let (dif, dit, scale, product, finish_quotient) = (
            pipeline("fr_dif_pass")?,
            pipeline("fr_dit_pass")?,
            pipeline("fr_scale")?,
            pipeline("fr_product")?,
            pipeline("fr_finish_quotient")?,
        );
        let apple_silicon = cfg!(all(target_os = "macos", target_arch = "aarch64"));
        let recent_phone = cfg!(all(target_os = "ios", target_arch = "aarch64"))
            && device.supports_family(metal::MTLGPUFamily::Apple9)
            && ((1 << 18)..=(1 << 21)).contains(&n);
        let encode_digits = (apple_silicon || recent_phone)
            .then(|| pipeline("fr_quotient_digits").ok())
            .flatten();
        let forward = [
            upload(&device, forward_weights.lows),
            upload(&device, forward_weights.chunks),
        ];
        let sort_digits = encode_digits.as_ref().and_then(|_| {
            if std::env::var("ZOPRO_METAL_GPU_SCATTER").as_deref() == Ok("0") {
                return None;
            }
            Some([
                pipeline("fr_digit_counts").ok()?,
                pipeline("fr_digit_scatter").ok()?,
            ])
        });
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let log_n = n.trailing_zeros();
        let limit = dif
            .max_total_threads_per_threadgroup()
            .min(dit.max_total_threads_per_threadgroup());
        let passes = plan_passes(log_n, limit);
        let tensor_fft = passes
            .last()
            .filter(|p| p.levels == 9)
            .and_then(|_| crate::tensor_fft::TensorFft::new(&device, n, [inverse_roots, roots]));
        let derive_c_override = std::env::var("ZOPRO_METAL_DERIVE_C");
        let derive_c_requested = (apple_silicon && n >= (1 << 18)
            && derive_c_override.as_deref() != Ok("0"))
            || (recent_phone && derive_c_override.as_deref() == Ok("1"));
        let derive_c = if derive_c_requested && !euler_workspace {
            passes.first().and_then(|pass| {
                let kernel = pipeline("fr_dif_derive_c").ok()?;
                let threads = 1 << (pass.levels + pass.cols_log - 1);
                (threads <= kernel.max_total_threads_per_threadgroup()).then_some(kernel)
            })
        } else {
            None
        };
        Ok((
            Self {
                queue: crate::queue(&device),
                dif,
                dit,
                derive_c,
                tensor_fft,
                passes,
                scale,
                product,
                finish_quotient,
                encode_digits,
                euler: None,
                sort_digits,
                n,
                roots: upload(&device, roots),
                inverse_roots: upload(&device, inverse_roots),
                forward_weights: forward,
                reverse_weights: [
                    upload(&device, reverse_weights.lows),
                    upload(&device, reverse_weights.chunks),
                ],
                low_bits: forward_weights.low_bits,
                abc: device.new_buffer(
                    ((3 - usize::from(euler_workspace)) * n * 32) as u64,
                    MTLResourceOptions::StorageModeShared,
                ),
                euler_workspace,
            },
            compile_ms,
        ))
    }

    /// The quotient's coefficients times the denominator, bit-reversed, the
    /// first n - 1 of them (what the Z key has bases for), from evaluations
    /// of a, b and c over the domain (padded with zeros to n).
    pub fn quotient(&mut self, a: &[Fr], b: &[Fr], c: &[Fr]) -> Result<Vec<Fr>, String> {
        let pending = self.start(a, b, c);
        self.finish(pending)
    }

    /// The quotient submitted to the GPU, for [`Quotient::finish`]: what is
    /// queued after it runs after it.
    pub fn start(&mut self, a: &[Fr], b: &[Fr], c: &[Fr]) -> PendingQuotient {
        objc::rc::autoreleasepool(|| self.start_in_pool(a, b, Some(c), None))
    }

    /// Retain only the originally populated rows for the Euler product.
    #[doc(hidden)]
    pub fn prepare_euler_product(&mut self, active: usize) -> Result<(), String> {
        if !self.euler_workspace || !(1..=self.n).contains(&active) {
            return Err("invalid Euler quotient workspace".into());
        }
        let device = self.abc.device().to_owned();
        let low = 1usize << self.low_bits;
        let montgomery_degree = |i: usize| {
            Fr::from((i.reverse_bits() >> (usize::BITS - self.n.trailing_zeros())) as u64)
        };
        let lows: Vec<_> = (0..low).map(montgomery_degree).collect();
        let highs: Vec<_> = (0..self.n / low).map(|i| montgomery_degree(i * low)).collect();
        let fused = if std::env::var("ZOPRO_METAL_EULER_FUSED").as_deref() != Ok("0") {
            let loose_enabled = std::env::var("ZOPRO_METAL_EULER_LOOSE_FFT").as_deref() != Ok("0");
            let inverse_name = if loose_enabled { "fr_dif_from_active_loose" } else { "fr_dif_from_active" };
            let inverse = crate::pipeline(&device, false, inverse_name)?;
            let first = self.passes.first().expect("Euler inverse transform");
            let threads = 1 << (first.levels + first.cols_log - 1);
            if threads > inverse.max_total_threads_per_threadgroup() {
                return Err("compact Euler inverse exceeds Metal threads".into());
            }
            let canonical = |value: &Fr| Fr::from_montgomery(value.to_canonical());
            let canonical_weights = [&lows, &highs].map(|table| {
                let table: Vec<_> = table.iter().map(canonical).collect();
                upload(&device, &table)
            });
            let loose = if loose_enabled {
                let passes = [crate::pipeline(&device, false, "fr_dif_euler_loose")?,
                              crate::pipeline(&device, false, "fr_dit_euler_loose")?];
                for pass in &self.passes {
                    let threads = 1 << (pass.levels + pass.cols_log - 1);
                    if passes.iter().any(|pipeline| threads > pipeline.max_total_threads_per_threadgroup()) {
                        return Err("loose Euler pass exceeds Metal threads".into());
                    }
                }
                Some(passes)
            } else {
                None
            };
            let final_pass = if crate::DIGIT_CHUNK.is_power_of_two()
                && std::env::var("ZOPRO_METAL_EULER_FINAL_PRODUCT").as_deref() != Ok("0")
            {
                let pipeline = crate::pipeline(&device, false, "fr_dit_euler_final")?;
                if threads > pipeline.max_total_threads_per_threadgroup() {
                    return Err("the final Euler pipeline exceeds Metal threads".into());
                }
                Some(pipeline)
            } else {
                None
            };
            Some(FusedEuler {
                inverse,
                loose,
                final_pass,
                digits: crate::pipeline(&device, false, "fr_euler_product_digits")?,
                canonical_weights,
            })
        } else {
            None
        };
        let storage = if fused.is_some() {
            MTLResourceOptions::StorageModeShared
        } else {
            MTLResourceOptions::StorageModePrivate
        };
        self.euler = Some(EulerProduct {
            weight: crate::pipeline(&device, false, "fr_euler_weight")?,
            product: crate::pipeline(&device, false, "fr_euler_product")?,
            weights: [upload(&device, &lows), upload(&device, &highs)],
            rows: device.new_buffer((2 * active * 32) as u64, storage),
            active,
            fused,
        });
        Ok(())
    }

    #[doc(hidden)]
    pub fn start_euler_product(
        &mut self, a: &[Fr], b: &[Fr], width: Option<usize>,
    ) -> PendingQuotient {
        assert!(a.len() <= self.euler.as_ref().expect("Euler workspace").active);
        assert!(b.len() <= self.euler.as_ref().expect("Euler workspace").active);
        objc::rc::autoreleasepool(|| {
            let euler = self.euler.as_ref().unwrap();
            if euler.fused.is_some() {
                // SAFETY: the prior proof completed. The shared buffer is unique here.
                let rows = unsafe {
                    std::slice::from_raw_parts_mut(euler.rows.contents().cast::<Fr>(), 2 * euler.active)
                };
                for (dest, source) in rows.chunks_exact_mut(euler.active).zip([a, b]) {
                    dest[..source.len()].copy_from_slice(source);
                    dest[source.len()..].fill(Fr::ZERO);
                }
            } else {
                self.upload_inputs(a, b, None);
            }
            self.enqueue(QuotientC::EulerProduct, width)
        })
    }

    /// # Safety
    /// Earlier GPU writes must be finished and all of A and B initialized.
    #[doc(hidden)]
    pub unsafe fn start_preloaded_euler_product(&mut self, width: Option<usize>) -> PendingQuotient {
        objc::rc::autoreleasepool(|| self.enqueue(QuotientC::EulerProduct, width))
    }

    pub fn can_recode_msm(&self) -> bool {
        let max_windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(7);
        self.encode_digits.is_some() && self.n <= u32::MAX as usize / max_windows
    }

    /// Temporary Metal buffers needed to recode and scatter a quotient.
    #[doc(hidden)]
    pub fn digit_scatter_workspace(&self, width: usize) -> Option<usize> {
        self.digit_scatter_workspace_for(width, self.n - 1)
    }

    #[doc(hidden)]
    pub fn digit_scatter_workspace_for(&self, width: usize, entries: usize) -> Option<usize> {
        if !self.can_recode_msm() || !(7..=15).contains(&width) {
            return None;
        }
        self.sort_digits.as_ref()?;
        let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
        let buckets = windows.checked_mul(1 << (width - 1))?;
        let digit_bytes = entries.checked_mul(windows)?.checked_mul(2)?;
        // Counters have an extra slot for invalid digits. Starts and offsets
        // are separate buffers retained by the final MSM command.
        let counters = buckets.checked_add(1)?.checked_mul(4)?;
        let starts = buckets.checked_mul(4)?;
        let page = 16 * 1024;
        digit_bytes
            .checked_next_multiple_of(page)?
            .checked_add(counters.checked_next_multiple_of(page)?)?
            .checked_add(starts.checked_next_multiple_of(page)?.checked_mul(2)?)
    }

    /// Queue the quotient followed by recoding for the MSM's window width.
    pub fn start_for_msm(&mut self, a: &[Fr], b: &[Fr], c: &[Fr], width: usize) -> PendingQuotient {
        assert!(self.can_recode_msm() && (7..=15).contains(&width));
        objc::rc::autoreleasepool(|| self.start_in_pool(a, b, Some(c), Some(width)))
    }

    pub fn can_derive_c(&self) -> bool {
        self.derive_c.is_some()
    }

    /// For a validated solver's row values, derive C = A * B on the GPU.
    pub fn start_derived(&mut self, a: &[Fr], b: &[Fr], width: Option<usize>) -> PendingQuotient {
        assert!(self.can_derive_c());
        assert!(width.is_none_or(|w| self.can_recode_msm() && (7..=15).contains(&w)));
        objc::rc::autoreleasepool(|| self.start_in_pool(a, b, None, width))
    }

    pub fn finish(&mut self, pending: PendingQuotient) -> Result<Vec<Fr>, String> {
        objc::rc::autoreleasepool(|| crate::completed(&pending.command))?;
        Self::log_euler_device_time(&pending);
        Ok(self.copy_coefficients(pending.entries, pending.canonical))
    }

    fn log_euler_device_time(pending: &PendingQuotient) {
        if !log::log_enabled!(log::Level::Info)
            || std::env::var("ZOPRO_CATALOG_PAIRED_FOLD").as_deref() != Ok("1")
        {
            return;
        }
        use objc::{msg_send, sel, sel_impl};

        // SAFETY: the completed Metal command reports these timestamps in seconds.
        let (start, end): (f64, f64) = unsafe {
            (
                msg_send![&*pending.command, GPUStartTime],
                msg_send![&*pending.command, GPUEndTime],
            )
        };
        if start > 0.0 && end >= start {
            log::info!("ZOPRO_TRACE metal-folded-c-quotient {:.3}", (end - start) * 1000.0);
        }
    }

    /// Collect the recoded quotient and optionally copy field coefficients for profiling.
    pub fn finish_msm_digits(
        &mut self,
        pending: PendingQuotient,
        copy_coefficients: bool,
    ) -> Result<(QuotientDigits, Option<Vec<Fr>>), String> {
        objc::rc::autoreleasepool(|| crate::completed(&pending.command))?;
        Self::log_euler_device_time(&pending);
        let digits = pending.digits.ok_or("no quotient digits were submitted")?;
        Ok((
            digits,
            copy_coefficients.then(|| self.copy_coefficients(pending.entries, pending.canonical)),
        ))
    }

    fn copy_coefficients(&self, entries: usize, canonical: bool) -> Vec<Fr> {
        // SAFETY: the GPU is done with abc.
        let values =
            unsafe { std::slice::from_raw_parts(self.abc.contents() as *const Fr, self.n) };
        let values = &values[..entries];
        if canonical {
            use rayon::prelude::*;
            values
                .par_iter()
                .map(|value| {
                    Fr::from_canonical(value.0).expect("reduced canonical Euler coefficient")
                })
                .collect()
        } else {
            values.to_vec()
        }
    }

    fn start_in_pool(
        &mut self,
        a: &[Fr],
        b: &[Fr],
        c: Option<&[Fr]>,
        width: Option<usize>,
    ) -> PendingQuotient {
        self.upload_inputs(a, b, c);
        let mode = if c.is_none() { QuotientC::Derived } else { QuotientC::Supplied };
        self.enqueue(mode, width)
    }

    fn upload_inputs(&mut self, a: &[Fr], b: &[Fr], c: Option<&[Fr]>) {
        let n = self.n;
        assert!(c.is_none() || !self.euler_workspace);
        let entries = (3 - usize::from(self.euler_workspace)) * n;
        // SAFETY: abc holds the selected row workspace and the GPU is idle on it.
        let values =
            unsafe { std::slice::from_raw_parts_mut(self.abc.contents() as *mut Fr, entries) };
        for (slot, source) in values.chunks_exact_mut(n).zip([Some(a), Some(b), c]) {
            if let Some(source) = source {
                slot[..source.len()].copy_from_slice(source);
                slot[source.len()..].fill(Fr::ZERO);
            }
        }
    }

    /// Give the witness solver exclusive access to the three shared vectors.
    ///
    /// # Safety
    /// No previously queued command may still access the vectors.
    #[doc(hidden)]
    pub unsafe fn write_rows<T>(
        &mut self,
        write: impl FnOnce([&mut [MaybeUninit<Fr>]; 3]) -> T,
    ) -> T {
        if let Some(euler) = self.euler.as_ref().filter(|euler| euler.fused.is_some()) {
            // SAFETY: this compact shared buffer is idle and holds exactly two active prefixes.
            let values = unsafe { std::slice::from_raw_parts_mut(euler.rows.contents().cast(), 2 * euler.active) };
            let (a, b) = values.split_at_mut(euler.active);
            return write([a, b, &mut []]);
        }
        // SAFETY: the buffers have one field element per 32 bytes and the
        // caller owns the workspace and has waited for any earlier commands.
        let entries = (3 - usize::from(self.euler_workspace)) * self.n;
        let values = unsafe { std::slice::from_raw_parts_mut(self.abc.contents().cast(), entries) };
        let (a, rest) = values.split_at_mut(self.n);
        let (b, c) = rest.split_at_mut(self.n);
        write([a, b, c])
    }

    /// Submit a quotient for evaluations written in the shared vectors.
    ///
    /// # Safety
    /// Earlier GPU writes must be finished. The caller must initialize all
    /// elements of A and B, including padding. The same applies to C unless
    /// the GPU derives it.
    #[doc(hidden)]
    pub unsafe fn start_preloaded(
        &mut self,
        derive_c: bool,
        width: Option<usize>,
    ) -> PendingQuotient {
        assert!(!derive_c || self.can_derive_c());
        assert!(width.is_none_or(|w| self.can_recode_msm() && (7..=15).contains(&w)));
        let mode = if derive_c { QuotientC::Derived } else { QuotientC::Supplied };
        objc::rc::autoreleasepool(|| self.enqueue(mode, width))
    }

    fn enqueue(&mut self, mode: QuotientC, width: Option<usize>) -> PendingQuotient {
        assert!(width.is_none_or(|w| self.can_recode_msm() && (7..=15).contains(&w)));
        assert!(!self.euler_workspace || mode == QuotientC::EulerProduct);
        let euler = (mode == QuotientC::EulerProduct).then(|| self.euler.as_ref().expect("Euler workspace"));
        let n = self.n;
        let fused_euler = euler.and_then(|euler| euler.fused.as_ref());
        let euler_digits = fused_euler.filter(|_| width.is_some());
        let loose = fused_euler.and_then(|fused| fused.loose.as_ref());
        if loose.is_some() {
            log::info!("ZOPRO_TRACE metal-euler-loose-fft 1");
        }
        let euler_center = self.tensor_fft.as_ref().filter(|tensor| {
            fused_euler.is_some()
                && tensor.supports_euler_center()
                && std::env::var("ZOPRO_METAL_EULER_TENSOR_CENTER").as_deref() != Ok("0")
        });
        if euler_center.is_some() {
            log::info!("ZOPRO_TRACE metal-euler-tensor-center 1");
        }
        let command = self.queue.new_command_buffer();
        let scatter = width
            .zip(self.sort_digits.as_ref())
            .map(|(width, [_, pipeline])| {
                let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
                // The last counter records out-of-range digits.
                let count = windows * (1usize << (width - 1)) + 1;
                let counts = crate::shared(self.abc.device(), count * 4);
                let blit = command.new_blit_command_encoder();
                blit.fill_buffer(&counts, metal::NSRange::new(0, counts.length()), 0);
                blit.end_encoding();
                DigitScatter {
                    counts,
                    pipeline: pipeline.clone(),
                }
            });
        let entries = euler.map_or(n - 1, |euler| euler.active);
        let digits = width.map(|width| {
            let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
            let count = u32::try_from(entries * windows).expect("quotient digits fit GPU indexing");
            let bytes = u64::from(count) * std::mem::size_of::<i16>() as u64;
            let buffer = self.abc.device().new_buffer(bytes, MTLResourceOptions::StorageModeShared);
            QuotientDigits { buffer, scatter, n: entries, width, windows }
        });
        let final_product = euler_digits.zip(euler).zip(digits.as_ref()).and_then(|((fused, euler), digits)| {
            if digits.width != 15 {
                return None;
            }
            Some((fused.final_pass.as_ref()?, euler, digits))
        });
        if final_product.is_some() {
            log::info!("ZOPRO_TRACE metal-euler-final-product 1");
            log::info!("ZOPRO_TRACE metal-euler-fixed-digits 1");
        }
        if let Some(euler) = euler.filter(|euler| euler.fused.is_none()) {
            let blit = command.new_blit_command_encoder();
            let bytes = (euler.active * 32) as u64;
            blit.copy_from_buffer(&self.abc, 0, &euler.rows, 0, bytes);
            blit.copy_from_buffer(&self.abc, (n * 32) as u64, &euler.rows, bytes, bytes);
            blit.end_encoding();
        }
        let encoder = command.new_compute_command_encoder();
        if self.tensor_fft.is_some() {
            log::info!("ZOPRO_TRACE metal-tensor-fft 1");
        }
        let c_offset = (2 * n * 32) as u64;
        if mode == QuotientC::Derived {
            let kernel = self.derive_c.as_ref().expect("quotient C derivation");
            let (first, rest) = self.passes.split_first().expect("derive C transforms");
            self.pass(encoder, kernel, 0, &self.inverse_roots, first);
            for pass in rest {
                self.transform_pass(encoder, c_offset, pass, false);
            }
        }
        for vector in 0..2 {
            let offset = (vector * n * 32) as u64;
            if let Some((fused, euler)) = fused_euler.zip(euler) {
                let (first, rest) = self.passes.split_first().expect("Euler inverse transform");
                let active = euler.active as u32;
                encoder.set_buffer(3, Some(&euler.rows), (vector * euler.active * 32) as u64);
                encoder.set_bytes(4, 4, (&active as *const u32).cast());
                self.pass(encoder, &fused.inverse, offset, &self.inverse_roots, first);
                if let Some(tensor) = euler_center {
                    let (_, outer) = rest.split_last().expect("Euler tensor leaf");
                    for pass in outer {
                        self.euler_transform_pass(encoder, offset, pass, false, loose);
                    }
                    tensor.encode_euler_center(
                        encoder, &self.abc, offset, n,
                        &fused.canonical_weights, self.low_bits, euler_digits.is_some(),
                    );
                    self.euler_forward(encoder, vector, &self.passes[..self.passes.len() - 1], loose, final_product);
                    continue;
                }
                for pass in rest {
                    self.euler_transform_pass(encoder, offset, pass, false, loose);
                }
            } else {
                self.dif_all(encoder, offset);
            }
            let (weights, pipeline) = euler.map_or((&self.forward_weights, &self.scale), |product| {
                (euler_digits.map_or(&product.weights, |fused| &fused.canonical_weights), &product.weight)
            });
            self.scale_by(encoder, offset, weights, pipeline);
            self.euler_forward(encoder, vector, &self.passes, loose, final_product);
        }
        if mode == QuotientC::Supplied {
            self.dif_all(encoder, c_offset);
        }
        if let Some(euler) = euler.filter(|_| euler_digits.is_none()) {
            encoder.set_compute_pipeline_state(&euler.product);
            encoder.set_buffer(0, Some(&self.abc), 0);
            encoder.set_buffer(1, Some(&euler.rows), 0);
            let args = [n as u32, euler.active as u32];
            encoder.set_bytes(2, 8, args.as_ptr().cast());
            self.dispatch(encoder, &euler.product, euler.active);
        } else if euler_digits.is_none() {
            encoder.set_compute_pipeline_state(&self.product);
            encoder.set_buffer(0, Some(&self.abc), 0);
            let count = n as u32;
            encoder.set_bytes(2, 4, &count as *const u32 as *const c_void);
            self.dispatch(encoder, &self.product, n);
        } else {
            log::info!("ZOPRO_TRACE metal-euler-fused 1");
        }
        if euler.is_none() {
            self.dif_all(encoder, 0);
            self.scale_by(encoder, 0, &self.reverse_weights, &self.finish_quotient);
        }
        if let Some(digits) = &digits {
            if final_product.is_none() {
                let args = [digits.n as u32, digits.width as u32, digits.windows as u32, crate::DIGIT_CHUNK as u32];
                let pipeline = if let Some((fused, euler)) = euler_digits.zip(euler) {
                    let domain = self.n as u32;
                    encoder.set_buffer(3, Some(&euler.rows), 0);
                    encoder.set_bytes(4, 4, (&domain as *const u32).cast());
                    &fused.digits
                } else {
                    self.encode_digits.as_ref().expect("quotient digit pipeline")
                };
                encoder.set_compute_pipeline_state(pipeline);
                encoder.set_buffer(0, Some(&self.abc), 0);
                encoder.set_buffer(1, Some(&digits.buffer), 0);
                encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
                self.dispatch(encoder, pipeline, digits.n);
            }
            if let (Some(scatter), Some([pipeline, _])) = (&digits.scatter, &self.sort_digits) {
                let args = digits.scatter_args();
                encoder.set_compute_pipeline_state(pipeline);
                encoder.set_buffer(0, Some(&digits.buffer), 0);
                encoder.set_buffer(1, Some(&scatter.counts), 0);
                encoder.set_bytes(2, 16, args.as_ptr().cast());
                digits.dispatch(encoder, pipeline);
            }
        }
        encoder.end_encoding();
        command.commit();
        PendingQuotient {
            command: command.to_owned(),
            digits,
            entries,
            canonical: euler_digits.is_some(),
        }
    }

    fn dispatch(
        &self,
        encoder: &ComputeCommandEncoderRef,
        pipeline: &ComputePipelineState,
        threads: usize,
    ) {
        let group = pipeline.thread_execution_width().min(threads as u64).max(1);
        encoder.dispatch_threads(
            MTLSize::new(threads as u64, 1, 1),
            MTLSize::new(group, 1, 1),
        );
    }

    fn pass(
        &self,
        encoder: &ComputeCommandEncoderRef,
        pipeline: &ComputePipelineState,
        offset: u64,
        roots: &Buffer,
        pass: &Pass,
    ) {
        let args = [
            self.n as u32,
            pass.first,
            pass.levels,
            pass.stride_log,
            pass.cols_log,
            0,
            0,
            0,
        ];
        let tile = 1u64 << (pass.levels + pass.cols_log);
        encoder.set_compute_pipeline_state(pipeline);
        encoder.set_buffer(0, Some(&self.abc), offset);
        encoder.set_buffer(1, Some(roots), 0);
        encoder.set_bytes(2, 32, args.as_ptr() as *const c_void);
        encoder.set_threadgroup_memory_length(0, tile * 32);
        encoder.dispatch_thread_groups(
            MTLSize::new(self.n as u64 / tile, 1, 1),
            MTLSize::new(tile / 2, 1, 1),
        );
    }

    /// DIF from the widest gap to one: natural order in, bit-reversed out.
    fn dif_all(&self, encoder: &ComputeCommandEncoderRef, offset: u64) {
        for pass in &self.passes {
            self.transform_pass(encoder, offset, pass, false);
        }
    }

    fn euler_forward(
        &self,
        encoder: &ComputeCommandEncoderRef,
        vector: usize,
        passes: &[Pass],
        loose: Option<&[ComputePipelineState; 2]>,
        final_product: Option<(&ComputePipelineState, &EulerProduct, &QuotientDigits)>,
    ) {
        let offset = (vector * self.n * 32) as u64;
        for pass in passes.iter().rev() {
            if let Some((pipeline, euler, digits)) = final_product.filter(|_| pass.first == 0) {
                let output = [euler.active as u32, crate::DIGIT_CHUNK as u32, vector as u32, 0];
                encoder.set_buffer(3, Some(&euler.rows), 0);
                encoder.set_buffer(4, Some(&digits.buffer), 0);
                encoder.set_bytes(5, 16, output.as_ptr().cast());
                self.pass(encoder, pipeline, 0, &self.roots, pass);
            } else {
                self.euler_transform_pass(encoder, offset, pass, true, loose);
            }
        }
    }

    fn euler_transform_pass(
        &self, encoder: &ComputeCommandEncoderRef, offset: u64, pass: &Pass, dit: bool,
        loose: Option<&[ComputePipelineState; 2]>,
    ) {
        let uses_tensor = self.tensor_fft.is_some()
            && pass.levels == 9 && pass.stride_log == 0 && pass.cols_log == 0;
        if let Some(pipelines) = loose.filter(|_| !uses_tensor) {
            let roots = if dit { &self.roots } else { &self.inverse_roots };
            self.pass(encoder, &pipelines[usize::from(dit)], offset, roots, pass);
        } else {
            self.transform_pass(encoder, offset, pass, dit);
        }
    }

    fn transform_pass(&self, encoder: &ComputeCommandEncoderRef, offset: u64, pass: &Pass, dit: bool) {
        if let Some(tensor) = self.tensor_fft.as_ref()
            .filter(|_| pass.levels == 9 && pass.stride_log == 0 && pass.cols_log == 0)
        {
            tensor.encode(encoder, &self.abc, offset, self.n, dit);
            return;
        }
        let (pipeline, roots) = if dit {
            (&self.dit, &self.roots)
        } else {
            (&self.dif, &self.inverse_roots)
        };
        self.pass(encoder, pipeline, offset, roots, pass);
    }

    fn scale_by(
        &self,
        encoder: &ComputeCommandEncoderRef,
        offset: u64,
        weights: &[Buffer; 2],
        pipeline: &ComputePipelineState,
    ) {
        encoder.set_compute_pipeline_state(pipeline);
        encoder.set_buffer(0, Some(&self.abc), offset);
        encoder.set_buffer(1, Some(&weights[0]), 0);
        let count = self.n as u32;
        encoder.set_bytes(2, 4, &count as *const u32 as *const c_void);
        encoder.set_buffer(3, Some(&weights[1]), 0);
        encoder.set_bytes(4, 4, &self.low_bits as *const u32 as *const c_void);
        self.dispatch(encoder, pipeline, self.n);
    }
}

/// One pass of a transform: `levels` stages from global level `first`, on
/// tiles of 2^levels rows 2^stride_log apart by 2^cols_log columns.
#[derive(Debug, Clone, Copy)]
struct Pass {
    first: u32,
    levels: u32,
    stride_log: u32,
    cols_log: u32,
}

/// The passes over a 2^log_n domain, widest gaps first: tiles of 2^9
/// elements (16 KB of threadgroup memory, two threadgroups to a core), no
/// more than `limit` / 2 pairs, and eight columns where the stride allows,
/// so that a tile's loads are 256-byte runs rather than scattered words
/// (more in the last pass when it has fewer stages).
/// ZOPRO_METAL_NTT_TILE and ZOPRO_METAL_NTT_COLS set the logs.
fn plan_passes(log_n: u32, limit: u64) -> Vec<Pass> {
    let env = |name: &str, default: u32| {
        std::env::var(name)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    };
    let fit = (2 * limit.max(2)).ilog2();
    let tile_log = env("ZOPRO_METAL_NTT_TILE", 9).clamp(1, 10).min(fit).min(log_n);
    let cols = env("ZOPRO_METAL_NTT_COLS", 3);
    let mut passes = Vec::new();
    let mut covered = 0;
    while covered < log_n {
        let mut cols_log = cols.min(covered).min(tile_log - 1);
        let levels = (tile_log - cols_log).min(log_n - covered);
        cols_log = (tile_log - levels).min(covered);
        passes.push(Pass {
            first: log_n - covered - levels,
            levels,
            stride_log: covered,
            cols_log,
        });
        covered += levels;
    }
    passes.reverse();
    passes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_product_preserves_the_active_domain_prefix() {
        if Device::system_default().is_none() {
            return;
        }
        let n = 64;
        let active = 37;
        let root = Fr::root_of_unity(n).unwrap();
        let inverse = root.inverse().unwrap();
        let powers = |base: Fr| {
            (0..n).map(|i| base.pow(&[i as u64])).collect::<Vec<_>>()
        };
        let roots = powers(root);
        let inverse_roots = powers(inverse);
        let weights = || Weights { lows: &[Fr::ONE], chunks: &[Fr::ONE; 64], low_bits: 0 };
        let (mut quotient, _) = Quotient::new_euler_workspace(
            n, &roots[..n/2], &inverse_roots[..n/2], weights(), weights(),
        ).unwrap();
        quotient.prepare_euler_product(active).unwrap();
        let a: Vec<_> = (0..active).map(|i| Fr::from((i * i + 7) as u64)).collect();
        let b: Vec<_> = (0..active).map(|i| Fr::from((3 * i + 5) as u64)).collect();
        let euler = |rows: &[Fr], i: usize| {
            (1..n).map(|k| {
                let coefficient: Fr = rows.iter().enumerate()
                    .fold(Fr::ZERO, |sum, (j, value)| sum + *value * inverse_roots[(j * k) % n]);
                coefficient * Fr::from(k as u64) * roots[(k * i) % n]
            }).fold(Fr::ZERO, |sum, term| sum + term)
        };
        let expected: Vec<_> = (0..active)
            .map(|i| euler(&a, i) * b[i] + a[i] * euler(&b, i)).collect();
        let pending = quotient.start_euler_product(&a, &b, None);
        assert_eq!(quotient.finish(pending).unwrap(), expected);
        if quotient.can_recode_msm() {
            for width in [9, 15] {
                let pending = quotient.start_euler_product(&a, &b, Some(width));
                let (digits, values) = quotient.finish_msm_digits(pending, true).unwrap();
                assert_eq!(values.unwrap(), expected);
                assert_eq!(digits.n, active);
                let radix = Fr::from(1u64 << digits.width);
                for (i, value) in expected.iter().copied().enumerate() {
                    let decoded = (0..digits.windows).rev().fold(Fr::ZERO, |sum, window| {
                        let digit = digits.as_slice()[window * active + i];
                        let magnitude = Fr::from(u64::from(digit.unsigned_abs()));
                        sum * radix + if digit >= 0 { magnitude } else { -magnitude }
                    });
                    assert_eq!(decoded, value);
                }
            }
        }
    }
}
