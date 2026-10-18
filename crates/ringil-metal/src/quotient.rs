//! The quotient polynomial's coefficients on the GPU: the three evaluation
//! vectors are moved to the domain's coset (inverse DIF, scale, forward
//! DIT), multiplied pointwise, and brought back (inverse DIF, scale by the
//! denominator), in radix-2 stages over global memory. The tables are the
//! CPU plan's: half the roots of each direction, and the bit-reversed coset
//! weights.
use std::{ffi::c_void, time::Instant};

use ark_bn254::Fr;
use metal::{Buffer, CommandQueue, ComputeCommandEncoderRef, ComputePipelineState, Device, MTLResourceOptions, MTLSize};


pub struct Quotient {
    queue: CommandQueue,
    dif: ComputePipelineState,
    dit: ComputePipelineState,
    scale: ComputePipelineState,
    pointwise: ComputePipelineState,
    n: usize,
    log_n: u32,
    roots: Buffer,
    inverse_roots: Buffer,
    forward_weights: Buffer,
    reverse_weights: Buffer,
    abc: Buffer,
}

impl Drop for Quotient {
    fn drop(&mut self) {
        objc::rc::autoreleasepool(|| {
            for buffer in [&self.roots, &self.inverse_roots, &self.forward_weights, &self.reverse_weights, &self.abc] {
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
    pub fn new(n: usize, roots: &[Fr], inverse_roots: &[Fr], forward_weights: &[Fr], reverse_weights: &[Fr]) -> Result<(Self, f64), String> {
        objc::rc::autoreleasepool(|| Self::new_in_pool(n, roots, inverse_roots, forward_weights, reverse_weights))
    }

    fn new_in_pool(n: usize, roots: &[Fr], inverse_roots: &[Fr], forward_weights: &[Fr], reverse_weights: &[Fr]) -> Result<(Self, f64), String> {
        assert!(n.is_power_of_two() && n >= 8);
        assert_eq!(roots.len(), n / 2);
        assert_eq!(forward_weights.len(), n);
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let pipeline = |name: &str| crate::pipeline(&device, false, name);
        let (dif, dit, scale, pointwise) =
            (pipeline("fr_dif_stage")?, pipeline("fr_dit_stage")?, pipeline("fr_scale")?, pipeline("fr_pointwise")?);
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        Ok((
            Self {
                queue: crate::queue(&device),
                dif,
                dit,
                scale,
                pointwise,
                n,
                log_n: n.trailing_zeros(),
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
        objc::rc::autoreleasepool(|| self.quotient_in_pool(a, b, c))
    }

    fn quotient_in_pool(&mut self, a: &[Fr], b: &[Fr], c: &[Fr]) -> Result<Vec<Fr>, String> {
        let n = self.n;
        // SAFETY: abc holds 3n Fr in shared storage; the GPU is idle on it.
        let values = unsafe { std::slice::from_raw_parts_mut(self.abc.contents() as *mut Fr, 3 * n) };
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
        crate::completed(command)?;
        Ok(values[..n - 1].to_vec())
    }

    fn dispatch(&self, encoder: &ComputeCommandEncoderRef, pipeline: &ComputePipelineState, threads: usize) {
        let group = pipeline.thread_execution_width().min(threads as u64).max(1);
        encoder.dispatch_threads(MTLSize::new(threads as u64, 1, 1), MTLSize::new(group, 1, 1));
    }

    fn stage(&self, encoder: &ComputeCommandEncoderRef, pipeline: &ComputePipelineState, offset: u64, roots: &Buffer, level: u32) {
        let args = [self.n as u32, level, self.log_n - 1 - level, 0];
        encoder.set_compute_pipeline_state(pipeline);
        encoder.set_buffer(0, Some(&self.abc), offset);
        encoder.set_buffer(1, Some(roots), 0);
        encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
        self.dispatch(encoder, pipeline, self.n / 2);
    }

    /// DIF from the widest gap to one: natural order in, bit-reversed out.
    fn dif_all(&self, encoder: &ComputeCommandEncoderRef, offset: u64, roots: &Buffer) {
        for level in 0..self.log_n {
            self.stage(encoder, &self.dif, offset, roots, level);
        }
    }

    /// DIT from gap one to the widest: bit-reversed in, natural order out.
    fn dit_all(&self, encoder: &ComputeCommandEncoderRef, offset: u64, roots: &Buffer) {
        for level in (0..self.log_n).rev() {
            self.stage(encoder, &self.dit, offset, roots, level);
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
