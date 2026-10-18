//! A G1 MSM on an Apple GPU. The host recodes the scalars into signed
//! 15-bit digits and counting-sorts the (base, window) pairs into bucket
//! runs, in parallel over windows; the GPU sums each run into its bucket
//! and reduces each window to its weighted sum; the host combines the
//! windows. Bases are uploaded once, in the kernel's memory layout (eight
//! little-endian words per coordinate, Montgomery form), which is also the
//! shader's.
use std::{ffi::c_void, time::Instant};

use ark_bn254::{Fq, Fr, G1Affine, G1Projective};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, PrimeField, Zero};
use metal::{
    Buffer, CommandQueue, CompileOptions, ComputePipelineState, Device, MTLResourceOptions, MTLSize,
};
use rayon::prelude::*;

const SHADER: &str = concat!(
    include_str!("../shaders/field.metal"),
    "\n",
    include_str!("../shaders/curve.metal"),
    "\n",
    include_str!("../shaders/buckets.metal"),
);
const WIDTH: usize = 15;
const HALF: usize = 1 << (WIDTH - 1);
const WINDOWS: usize = (Fr::MODULUS_BIT_SIZE as usize + 1).div_ceil(WIDTH);
const REDUCE_THREADS: usize = 256;

/// Stage times of one MSM, in milliseconds.
#[derive(Debug, Clone, Copy, Default)]
pub struct Timings {
    pub sort_ms: f64,
    pub accumulate_ms: f64,
    pub reduce_ms: f64,
    pub combine_ms: f64,
}

pub struct G1Msm {
    device: Device,
    queue: CommandQueue,
    accumulate: ComputePipelineState,
    reduce: ComputePipelineState,
    bases: Buffer,
    n: usize,
    sorted: Buffer,
    starts: Buffer,
    counts: Buffer,
    buckets: Buffer,
    sums: Buffer,
}

fn shared(device: &Device, bytes: usize) -> Buffer {
    device.new_buffer(bytes.max(16) as u64, MTLResourceOptions::StorageModeShared)
}

impl G1Msm {
    /// Compiles the kernels and uploads the bases; `compile_ms` is how long
    /// the shader compilation took.
    pub fn new(bases: &[G1Affine]) -> Result<(Self, f64), String> {
        let device = Device::system_default().ok_or("no Metal device")?;
        let started = Instant::now();
        let library = device
            .new_library_with_source(SHADER, &CompileOptions::new())
            .map_err(|e| format!("shader: {e}"))?;
        let pipeline = |name: &str| {
            let function = library.get_function(name, None).map_err(|e| format!("{name}: {e}"))?;
            device
                .new_compute_pipeline_state_with_function(&function)
                .map_err(|e| format!("{name}: {e}"))
        };
        let accumulate = pipeline("accumulate_g1")?;
        let reduce = pipeline("reduce_g1")?;
        let compile_ms = started.elapsed().as_secs_f64() * 1000.0;
        // Affine points as the kernel holds them: x, y, four limbs each, the
        // identity (0, 0).
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
        let bases_buffer = device.new_buffer_with_data(
            raw.as_ptr() as *const c_void,
            raw.len() as u64,
            MTLResourceOptions::StorageModeShared,
        );
        let n = bases.len();
        let buckets = WINDOWS * HALF;
        Ok((
            Self {
                queue: device.new_command_queue(),
                sorted: shared(&device, n * WINDOWS * 4),
                starts: shared(&device, buckets * 4),
                counts: shared(&device, buckets * 4),
                buckets: shared(&device, buckets * 128),
                sums: shared(&device, WINDOWS * 128),
                accumulate,
                reduce,
                bases: bases_buffer,
                n,
                device,
            },
            compile_ms,
        ))
    }

    pub fn msm(&self, scalars: &[Fr]) -> (G1Projective, Timings) {
        assert_eq!(scalars.len(), self.n);
        let mut timings = Timings::default();
        let started = Instant::now();
        self.sort(scalars);
        timings.sort_ms = elapsed(started);

        let started = Instant::now();
        let count = (WINDOWS * HALF) as u32;
        self.run(|encoder| {
            encoder.set_compute_pipeline_state(&self.accumulate);
            encoder.set_buffer(0, Some(&self.bases), 0);
            encoder.set_buffer(1, Some(&self.sorted), 0);
            encoder.set_buffer(2, Some(&self.starts), 0);
            encoder.set_buffer(3, Some(&self.counts), 0);
            encoder.set_buffer(4, Some(&self.buckets), 0);
            encoder.set_bytes(5, 4, &count as *const u32 as *const c_void);
            let group = self.accumulate.thread_execution_width().min(count as u64);
            encoder.dispatch_threads(MTLSize::new(count as u64, 1, 1), MTLSize::new(group, 1, 1));
        });
        timings.accumulate_ms = elapsed(started);

        let started = Instant::now();
        let leaf = HALF / REDUCE_THREADS;
        let args = [HALF as u32, leaf as u32, leaf.trailing_zeros(), 0u32];
        self.run(|encoder| {
            encoder.set_compute_pipeline_state(&self.reduce);
            encoder.set_buffer(0, Some(&self.buckets), 0);
            encoder.set_buffer(1, Some(&self.sums), 0);
            encoder.set_bytes(2, 16, args.as_ptr() as *const c_void);
            encoder.dispatch_thread_groups(
                MTLSize::new(WINDOWS as u64, 1, 1),
                MTLSize::new(REDUCE_THREADS as u64, 1, 1),
            );
        });
        timings.reduce_ms = elapsed(started);

        let started = Instant::now();
        let sums = self.sums.contents() as *const u8;
        let mut result = G1Projective::ZERO;
        for window in (0..WINDOWS).rev() {
            for _ in 0..WIDTH {
                result.double_in_place();
            }
            // SAFETY: `sums` holds WINDOWS xyzz points of 128 bytes, written
            // by the completed reduce dispatch.
            let point = unsafe { std::slice::from_raw_parts(sums.add(window * 128), 128) };
            result += xyzz_to_projective(point);
        }
        timings.combine_ms = elapsed(started);
        (result, timings)
    }

    /// Digits, counts and the scatter: digits in parallel over scalars,
    /// then one window per task writing its own segment of the buffers.
    fn sort(&self, scalars: &[Fr]) {
        let radix = 1u64 << WIDTH;
        let mask = radix - 1;
        let n = self.n;
        // digits[i * WINDOWS + w].
        let mut digits = vec![0i16; n * WINDOWS];
        digits.par_chunks_mut(WINDOWS).zip(scalars).for_each(|(row, s)| {
            let BigInt(words) = s.into_bigint();
            let mut carry = 0u64;
            for (w, digit) in row.iter_mut().enumerate() {
                let offset = w * WIDTH;
                let (word, shift) = (offset / 64, offset % 64);
                let mut value = if word < 4 { words[word] >> shift } else { 0 };
                if shift + WIDTH > 64 && word < 3 {
                    value |= words[word + 1] << (64 - shift);
                }
                let value = (value & mask) + carry;
                carry = (value + radix / 2) >> WIDTH;
                *digit = (value as i64 - (carry * radix) as i64) as i16;
            }
        });
        // SAFETY: the buffers hold n * WINDOWS, WINDOWS * HALF and WINDOWS *
        // HALF u32s, shared storage, and nothing else reads them until the
        // dispatch.
        let sorted = unsafe { std::slice::from_raw_parts_mut(self.sorted.contents() as *mut u32, n * WINDOWS) };
        let starts = unsafe { std::slice::from_raw_parts_mut(self.starts.contents() as *mut u32, WINDOWS * HALF) };
        let all_counts = unsafe { std::slice::from_raw_parts_mut(self.counts.contents() as *mut u32, WINDOWS * HALF) };
        sorted
            .par_chunks_mut(n)
            .zip(starts.par_chunks_mut(HALF))
            .zip(all_counts.par_chunks_mut(HALF))
            .enumerate()
            .for_each(|(w, ((sorted, starts), counts))| {
                counts.fill(0);
                for i in 0..n {
                    let d = digits[i * WINDOWS + w];
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
                    let d = digits[i * WINDOWS + w];
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

/// An xyzz point (x, y, zz, zzz) to arkworks' Jacobian (x zz, y zzz, zz).
fn xyzz_to_projective(bytes: &[u8]) -> G1Projective {
    let (x, y, zz, zzz) = (fq(&bytes[..32]), fq(&bytes[32..64]), fq(&bytes[64..96]), fq(&bytes[96..128]));
    if zz.is_zero() {
        return G1Projective::ZERO;
    }
    G1Projective::new_unchecked(x * zz, y * zzz, zz)
}

/// The kernel's result as affine, for checks.
pub fn affine(p: G1Projective) -> G1Affine {
    p.into_affine()
}
