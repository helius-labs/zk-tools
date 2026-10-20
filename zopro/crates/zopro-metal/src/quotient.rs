//! The quotient polynomial's coefficients on the GPU: the three evaluation
//! vectors are moved to the domain's coset (inverse DIF, scale, forward
//! DIT), multiplied pointwise, and brought back (inverse DIF, scale by the
//! denominator), in passes of several radix-2 stages through threadgroup
//! memory. The tables are the CPU plan's: half the roots of each direction,
//! and the bit-reversed coset weights.
use std::{ffi::c_void, time::Instant};

use zopro_bn254::Fr;
use metal::{
    Buffer, CommandQueue, ComputeCommandEncoderRef, ComputePipelineState, Device,
    MTLResourceOptions, MTLSize,
};

/// A quotient submitted to the GPU and not yet collected.
pub struct PendingQuotient {
    command: metal::CommandBuffer,
}

pub struct Quotient {
    queue: CommandQueue,
    dif: ComputePipelineState,
    dit: ComputePipelineState,
    /// The DIF passes, widest gaps first; the DIT passes run them reversed.
    passes: Vec<Pass>,
    scale: ComputePipelineState,
    pointwise: ComputePipelineState,
    n: usize,
    roots: Buffer,
    inverse_roots: Buffer,
    forward_weights: Buffer,
    reverse_weights: Buffer,
    abc: Buffer,
}

impl Drop for Quotient {
    fn drop(&mut self) {
        objc::rc::autoreleasepool(|| {
            for buffer in [
                &self.roots,
                &self.inverse_roots,
                &self.forward_weights,
                &self.reverse_weights,
                &self.abc,
            ] {
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
        forward_weights: &[Fr],
        reverse_weights: &[Fr],
    ) -> Result<(Self, f64), String> {
        objc::rc::autoreleasepool(|| {
            Self::new_in_pool(n, roots, inverse_roots, forward_weights, reverse_weights)
        })
    }

    fn new_in_pool(
        n: usize,
        roots: &[Fr],
        inverse_roots: &[Fr],
        forward_weights: &[Fr],
        reverse_weights: &[Fr],
    ) -> Result<(Self, f64), String> {
        assert!(n.is_power_of_two() && n >= 8);
        assert_eq!(roots.len(), n / 2);
        assert_eq!(forward_weights.len(), n);
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let pipeline = |name: &str| crate::pipeline(&device, false, name);
        let (dif, dit, scale, pointwise) = (
            pipeline("fr_dif_pass")?,
            pipeline("fr_dit_pass")?,
            pipeline("fr_scale")?,
            pipeline("fr_pointwise")?,
        );
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        let log_n = n.trailing_zeros();
        let limit = dif
            .max_total_threads_per_threadgroup()
            .min(dit.max_total_threads_per_threadgroup());
        let passes = plan_passes(log_n, limit);
        Ok((
            Self {
                queue: crate::queue(&device),
                dif,
                dit,
                passes,
                scale,
                pointwise,
                n,
                roots: upload(&device, roots),
                inverse_roots: upload(&device, inverse_roots),
                forward_weights: upload(&device, forward_weights),
                reverse_weights: upload(&device, reverse_weights),
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
        objc::rc::autoreleasepool(|| self.start_in_pool(a, b, c))
    }

    pub fn finish(&mut self, pending: PendingQuotient) -> Result<Vec<Fr>, String> {
        objc::rc::autoreleasepool(|| crate::completed(&pending.command))?;
        // SAFETY: the GPU is done with abc.
        let values =
            unsafe { std::slice::from_raw_parts(self.abc.contents() as *const Fr, self.n) };
        Ok(values[..self.n - 1].to_vec())
    }

    fn start_in_pool(&mut self, a: &[Fr], b: &[Fr], c: &[Fr]) -> PendingQuotient {
        let n = self.n;
        // SAFETY: abc holds 3n Fr in shared storage; the GPU is idle on it.
        let values =
            unsafe { std::slice::from_raw_parts_mut(self.abc.contents() as *mut Fr, 3 * n) };
        for (slot, source) in values.chunks_exact_mut(n).zip([a, b, c]) {
            slot[..source.len()].copy_from_slice(source);
            slot[source.len()..].fill(Fr::from(0u64));
        }
        let command = self.queue.new_command_buffer();
        let encoder = command.new_compute_command_encoder();
        for vector in 0..3 {
            let offset = (vector * n * 32) as u64;
            self.dif_all(encoder, offset, &self.inverse_roots);
            self.scale_by(encoder, offset, &self.forward_weights);
            self.dit_all(encoder, offset, &self.roots);
        }
        encoder.set_compute_pipeline_state(&self.pointwise);
        encoder.set_buffer(0, Some(&self.abc), 0);
        let count = n as u32;
        encoder.set_bytes(2, 4, &count as *const u32 as *const c_void);
        self.dispatch(encoder, &self.pointwise, n);
        self.dif_all(encoder, 0, &self.inverse_roots);
        self.scale_by(encoder, 0, &self.reverse_weights);
        encoder.end_encoding();
        command.commit();
        PendingQuotient {
            command: command.to_owned(),
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

    fn scale_by(&self, encoder: &ComputeCommandEncoderRef, offset: u64, weights: &Buffer) {
        encoder.set_compute_pipeline_state(&self.scale);
        encoder.set_buffer(0, Some(&self.abc), offset);
        encoder.set_buffer(1, Some(weights), 0);
        let count = self.n as u32;
        encoder.set_bytes(2, 4, &count as *const u32 as *const c_void);
        self.dispatch(encoder, &self.scale, self.n);
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
