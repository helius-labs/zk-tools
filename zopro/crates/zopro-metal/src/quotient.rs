//! The quotient polynomial's coefficients on the GPU: the three evaluation
//! vectors A and B are moved to the domain's coset (inverse DIF, scale,
//! forward DIT), multiplied pointwise, and brought back. C needs only its
//! inverse DIF because subtraction is linear. The final pass applies the
//! denominator and subtracts C in coefficient form. The tables are the CPU
//! plan's: half the roots of each direction,
//! and the bit-reversed coset weights in two small tables each (Weights).
use std::{ffi::c_void, time::Instant};

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
    /// The DIF passes, widest gaps first; the DIT passes run them reversed.
    passes: Vec<Pass>,
    scale: ComputePipelineState,
    product: ComputePipelineState,
    finish_quotient: ComputePipelineState,
    encode_digits: Option<ComputePipelineState>,
    sort_digits: Option<[ComputePipelineState; 2]>,
    n: usize,
    roots: Buffer,
    inverse_roots: Buffer,
    /// Each direction's weights: lows, then chunks.
    forward_weights: [Buffer; 2],
    reverse_weights: [Buffer; 2],
    low_bits: u32,
    abc: Buffer,
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
            Self::new_in_pool(n, roots, inverse_roots, forward_weights, reverse_weights)
        })
    }

    fn new_in_pool(
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        forward_weights: Weights,
        reverse_weights: Weights,
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
        let encode_digits = cfg!(all(target_os = "macos", target_arch = "aarch64"))
            .then(|| pipeline("fr_quotient_digits").ok())
            .flatten();
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
        let derive_c = if cfg!(all(target_os = "macos", target_arch = "aarch64"))
            && n >= (1 << 18)
            && std::env::var("ZOPRO_METAL_DERIVE_C").as_deref() != Ok("0")
        {
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
                passes,
                scale,
                product,
                finish_quotient,
                encode_digits,
                sort_digits,
                n,
                roots: upload(&device, roots),
                inverse_roots: upload(&device, inverse_roots),
                forward_weights: [
                    upload(&device, forward_weights.lows),
                    upload(&device, forward_weights.chunks),
                ],
                reverse_weights: [
                    upload(&device, reverse_weights.lows),
                    upload(&device, reverse_weights.chunks),
                ],
                low_bits: forward_weights.low_bits,
                abc: device.new_buffer((3 * n * 32) as u64, MTLResourceOptions::StorageModeShared),
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

    pub fn can_recode_msm(&self) -> bool {
        let max_windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(7);
        self.encode_digits.is_some() && self.n <= u32::MAX as usize / max_windows
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
        Ok(self.copy_coefficients())
    }

    /// Collect the recoded quotient and optionally copy field coefficients for profiling.
    pub fn finish_msm_digits(
        &mut self,
        pending: PendingQuotient,
        copy_coefficients: bool,
    ) -> Result<(QuotientDigits, Option<Vec<Fr>>), String> {
        objc::rc::autoreleasepool(|| crate::completed(&pending.command))?;
        let digits = pending.digits.ok_or("no quotient digits were submitted")?;
        Ok((digits, copy_coefficients.then(|| self.copy_coefficients())))
    }

    fn copy_coefficients(&self) -> Vec<Fr> {
        // SAFETY: the GPU is done with abc.
        let values =
            unsafe { std::slice::from_raw_parts(self.abc.contents() as *const Fr, self.n) };
        values[..self.n - 1].to_vec()
    }

    fn start_in_pool(
        &mut self,
        a: &[Fr],
        b: &[Fr],
        c: Option<&[Fr]>,
        width: Option<usize>,
    ) -> PendingQuotient {
        let n = self.n;
        // SAFETY: abc holds 3n Fr in shared storage; the GPU is idle on it.
        let values =
            unsafe { std::slice::from_raw_parts_mut(self.abc.contents() as *mut Fr, 3 * n) };
        for (slot, source) in values.chunks_exact_mut(n).zip([Some(a), Some(b), c]) {
            if let Some(source) = source {
                slot[..source.len()].copy_from_slice(source);
                slot[source.len()..].fill(Fr::ZERO);
            }
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
        let encoder = command.new_compute_command_encoder();
        let c_offset = (2 * n * 32) as u64;
        if c.is_none() {
            let kernel = self.derive_c.as_ref().expect("quotient C derivation");
            let (first, rest) = self.passes.split_first().expect("derive C transforms");
            self.pass(encoder, kernel, 0, &self.inverse_roots, first);
            for pass in rest {
                self.pass(encoder, &self.dif, c_offset, &self.inverse_roots, pass);
            }
        }
        for vector in 0..2 {
            let offset = (vector * n * 32) as u64;
            self.dif_all(encoder, offset, &self.inverse_roots);
            self.scale_by(encoder, offset, &self.forward_weights, &self.scale);
            self.dit_all(encoder, offset, &self.roots);
        }
        if c.is_some() {
            self.dif_all(encoder, c_offset, &self.inverse_roots);
        }
        encoder.set_compute_pipeline_state(&self.product);
        encoder.set_buffer(0, Some(&self.abc), 0);
        let count = n as u32;
        encoder.set_bytes(2, 4, &count as *const u32 as *const c_void);
        self.dispatch(encoder, &self.product, n);
        self.dif_all(encoder, 0, &self.inverse_roots);
        self.scale_by(encoder, 0, &self.reverse_weights, &self.finish_quotient);
        let digits = width.map(|width| {
            let n = n - 1;
            let windows = (Fr::MODULUS_BITS as usize + 1).div_ceil(width);
            let count = u32::try_from(n * windows).expect("quotient digits fit GPU indexing");
            let bytes = u64::from(count) * std::mem::size_of::<i16>() as u64;
            let buffer = self.abc.device().new_buffer(bytes, MTLResourceOptions::StorageModeShared);
            let args = [n as u32, width as u32, windows as u32, crate::DIGIT_CHUNK as u32];
            let pipeline = self.encode_digits.as_ref().expect("quotient digit pipeline");
            encoder.set_compute_pipeline_state(pipeline);
            encoder.set_buffer(0, Some(&self.abc), 0);
            encoder.set_buffer(1, Some(&buffer), 0);
            encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
            self.dispatch(encoder, pipeline, n);
            let digits = QuotientDigits {
                buffer,
                scatter,
                n,
                width,
                windows,
            };
            if let (Some(scatter), Some([pipeline, _])) = (&digits.scatter, &self.sort_digits) {
                let args = digits.scatter_args();
                encoder.set_compute_pipeline_state(pipeline);
                encoder.set_buffer(0, Some(&digits.buffer), 0);
                encoder.set_buffer(1, Some(&scatter.counts), 0);
                encoder.set_bytes(2, 16, args.as_ptr().cast());
                digits.dispatch(encoder, pipeline);
            }
            digits
        });
        encoder.end_encoding();
        command.commit();
        PendingQuotient {
            command: command.to_owned(),
            digits,
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
    fn dif_all(&self, encoder: &ComputeCommandEncoderRef, offset: u64, roots: &Buffer) {
        for pass in &self.passes {
            self.pass(encoder, &self.dif, offset, roots, pass);
        }
    }

    /// DIT from gap one to the widest: bit-reversed in, natural order out.
    fn dit_all(&self, encoder: &ComputeCommandEncoderRef, offset: u64, roots: &Buffer) {
        for pass in self.passes.iter().rev() {
            self.pass(encoder, &self.dit, offset, roots, pass);
        }
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
