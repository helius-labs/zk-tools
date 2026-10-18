#![cfg_attr(
    all(target_arch = "wasm32", target_feature = "simd128"),
    feature(asm_experimental_arch)
)]
use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, Field};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use wasm_bindgen::prelude::*;

/// The clock: std's natively, the page's in a browser, where std has none.
pub(crate) mod time {
    #[cfg(not(target_arch = "wasm32"))]
    pub use std::time::Instant;
    #[cfg(target_arch = "wasm32")]
    pub use web_time::Instant;
}
pub use wasm_bindgen_rayon::init_thread_pool;
mod decompress;
pub mod fft;
pub mod groth16;
mod msm;
#[cfg(not(target_arch = "wasm32"))]
mod msm_affine;
#[cfg(not(target_arch = "wasm32"))]
mod selftest;
mod simd;
mod solver;

// Stage times as RINGIL_TRACE log lines (`--features trace`): the native
// bench prints them, a phone's bench puts them in the device log.
fn trace(label: &str, started: crate::time::Instant) -> crate::time::Instant {
    #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
    log::info!(
        "RINGIL_TRACE {label} {:.1} ms{}",
        started.elapsed().as_secs_f64() * 1000.0,
        footprint_note()
    );
    #[cfg(not(all(feature = "trace", not(target_arch = "wasm32"))))]
    let _ = (label, started);
    crate::time::Instant::now()
}
/// The process's physical footprint, as a trace line's suffix (Apple; what
/// iOS's memory limit counts), and its peak so far.
#[cfg(all(feature = "trace", target_vendor = "apple"))]
fn footprint_note() -> String {
    let mut info: libc::rusage_info_v4 = unsafe { std::mem::zeroed() };
    // SAFETY: the buffer is a rusage_info_v4, as RUSAGE_INFO_V4 asks for.
    let status = unsafe {
        libc::proc_pid_rusage(
            std::process::id() as i32,
            libc::RUSAGE_INFO_V4,
            &mut info as *mut _ as *mut libc::rusage_info_t,
        )
    };
    if status != 0 {
        return String::new();
    }
    format!(
        " fp {} peak {} MB",
        info.ri_phys_footprint >> 20,
        info.ri_lifetime_max_phys_footprint >> 20
    )
}
#[cfg(all(
    feature = "trace",
    not(target_vendor = "apple"),
    not(target_arch = "wasm32")
))]
fn footprint_note() -> String {
    String::new()
}

pub use decompress::{decompress_g1, decompress_g2};

/// A key's GPU. In a browser its WebGPU objects belong to the worker's
/// thread, and the key, shared with the thread pool by the CPU path, must
/// be Send and Sync.
#[cfg(gpu_any)]
#[derive(Clone)]
struct GpuHold(std::sync::Arc<Gpu>);
// SAFETY: in a browser only the worker's thread touches the GPU objects
// (solve_parts_gpu and the key's construction and drop, all on that
// thread); the pool's threads get the key for its CPU vectors alone.
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
unsafe impl Send for GpuHold {}
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
unsafe impl Sync for GpuHold {}
#[cfg(gpu_any)]
impl std::ops::Deref for GpuHold {
    type Target = Gpu;
    fn deref(&self) -> &Gpu {
        &self.0
    }
}

/// The self-test's verdict in a browser, where it does not run.
#[cfg(all(gpu_any, target_arch = "wasm32"))]
mod selftest_web {
    pub struct Verdict {
        pub msm: bool,
        pub quotient: bool,
    }
}

fn words(data: &[u8]) -> BigInt<4> {
    BigInt(core::array::from_fn(|i| {
        u64::from_le_bytes(data[i * 8..i * 8 + 8].try_into().unwrap())
    }))
}
fn fq(data: &[u8]) -> Fq {
    Fq::new_unchecked(words(data))
}
fn frs(data: &[u8]) -> Vec<Fr> {
    data.as_chunks::<32>()
        .0
        .iter()
        .map(|x| Fr::new_unchecked(words(x)))
        .collect()
}
fn g1s(data: &[u8]) -> Vec<G1Affine> {
    data.as_chunks::<64>()
        .0
        .iter()
        .map(|x| {
            let px = fq(x);
            let py = fq(&x[32..]);
            if px == Fq::ZERO && py == Fq::ZERO {
                G1Affine::identity()
            } else {
                G1Affine::new_unchecked(px, py)
            }
        })
        .collect()
}
fn g2s(data: &[u8]) -> Vec<G2Affine> {
    data.as_chunks::<128>()
        .0
        .iter()
        .map(|x| {
            let px = Fq2::new(fq(x), fq(&x[32..]));
            let py = Fq2::new(fq(&x[64..]), fq(&x[96..]));
            if px == Fq2::ZERO && py == Fq2::ZERO {
                G2Affine::identity()
            } else {
                G2Affine::new_unchecked(px, py)
            }
        })
        .collect()
}
fn output_fq(out: &mut Vec<u8>, v: Fq) {
    for w in v.0 .0 {
        out.extend_from_slice(&w.to_le_bytes())
    }
}
fn output_g1(out: &mut Vec<u8>, p: G1Projective) {
    let p = p.into_affine();
    if p.infinity {
        out.extend_from_slice(&[0; 64]);
    } else {
        output_fq(out, p.x);
        output_fq(out, p.y);
    }
}
fn output_g2(out: &mut Vec<u8>, p: G2Projective) {
    let p = p.into_affine();
    if p.infinity {
        out.extend_from_slice(&[0; 128]);
    } else {
        output_fq(out, p.x.c0);
        output_fq(out, p.x.c1);
        output_fq(out, p.y.c0);
        output_fq(out, p.y.c1);
    }
}

// The version of the solver images Key::solver_image writes and
// Key::set_solver_image reads, for a host to tell its images apart.
#[wasm_bindgen]
/// What the GPU has allocated for this process, in bytes (Metal only; zero
/// elsewhere): the bench's stress mode watches it for leaks.
pub fn gpu_allocated_bytes() -> u64 {
    #[cfg(gpu)]
    return ringil_metal::allocated_bytes();
    #[cfg(not(gpu))]
    0
}

/// Compiles the GPU kernels ahead of the first key load (Metal; nothing
/// elsewhere). False when there is no GPU or it could not compile them.
pub fn gpu_warm() -> bool {
    #[cfg(gpu)]
    return ringil_metal::warm().is_ok();
    #[cfg(not(gpu))]
    false
}

pub fn solver_image_version() -> u32 {
    solver::IMAGE_VERSION as u32
}

/// The share of the G2 MSM's points the GPU takes when there is one; the
/// CPU takes the rest at the same time. RINGIL_G2_GPU_SHARE sets it
/// (RINGIL_G2=gpu|cpu: all or none); else 0.8 on a Mac, whose GPU is far
/// faster than its cores at it (the idle cores' fifth saves 4–6 %), a quarter on an Apple9 phone (A19), whose
/// GPU was idle for about 200 ms of a 535 ms 8 → 2 proof while the CPU's G2
/// MSM took 443, and none on older phones, whose GPUs are busy and slower
/// at G2 than their cores.
#[cfg(gpu)]
fn g2_gpu_share() -> f64 {
    if let Some(share) = std::env::var("RINGIL_G2_GPU_SHARE")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
    {
        return share.clamp(0.0, 1.0);
    }
    match std::env::var("RINGIL_G2").as_deref() {
        Ok("gpu") => 1.0,
        Ok("cpu") => 0.0,
        _ if cfg!(target_os = "macos") => 0.8,
        _ if ringil_metal::fast_gpu() => 0.25,
        _ => 0.0,
    }
}

#[cfg(gpu)]
struct Gpu {
    a: std::sync::Mutex<ringil_metal::G1Msm>,
    b: std::sync::Mutex<ringil_metal::G1Msm>,
    k: std::sync::Mutex<ringil_metal::G1Msm>,
    z: std::sync::Mutex<ringil_metal::G1Msm>,
    b2: Option<std::sync::Mutex<ringil_metal::G2Msm>>,
    /// The G2 points the GPU takes: those before this index; the CPU takes
    /// the rest, which is all the key keeps of the vector.
    b2_cut: usize,
    quotient: std::sync::Mutex<ringil_metal::Quotient>,
}
#[cfg(gpu_wgpu)]
struct Gpu {
    a: std::sync::Mutex<ringil_wgpu::G1Msm>,
    b: std::sync::Mutex<ringil_wgpu::G1Msm>,
    k: std::sync::Mutex<ringil_wgpu::G1Msm>,
    z: std::sync::Mutex<ringil_wgpu::G1Msm>,
    b2: Option<std::sync::Mutex<ringil_wgpu::G2Msm>>,
    b2_cut: usize,
    quotient: std::sync::Mutex<ringil_wgpu::Quotient>,
}

/// Threads for a GPU MSM's sort while the global pool computes the CPU's
/// G2 MSM (RINGIL_SORT_THREADS, default 2).
#[cfg(all(gpu_wgpu, not(target_arch = "wasm32")))]
fn sort_pool() -> &'static rayon::ThreadPool {
    static POOL: std::sync::OnceLock<rayon::ThreadPool> = std::sync::OnceLock::new();
    POOL.get_or_init(|| {
        let threads = std::env::var("RINGIL_SORT_THREADS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2);
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .thread_name(|i| format!("ringil-sort-{i}"))
            .build()
            .expect("sort pool")
    })
}

/// The share of the G2 MSM's points a wgpu GPU takes (RINGIL_G2_GPU_SHARE,
/// else measured best: on an M5 Pro through wgpu, and none with the
/// pre-compiled kernels, where the OnePlus 13's Adreno is the critical path
/// already); the CPU takes the rest.
#[cfg(gpu_wgpu)]
fn g2_gpu_share(spirv: bool) -> f64 {
    // A browser has no environment: the build's RINGIL_WEB_G2_SHARE, else
    // the default.
    #[cfg(target_arch = "wasm32")]
    return option_env!("RINGIL_WEB_G2_SHARE")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(WEB_G2_SHARE)
        .clamp(0.0, 1.0);
    #[allow(unreachable_code)]
    std::env::var("RINGIL_G2_GPU_SHARE")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(if spirv { 0.0 } else { WGPU_G2_SHARE })
        .clamp(0.0, 1.0)
}
/// In a browser, where the CPU's share runs as WebAssembly: measured best
/// in Chrome on an M5 Pro at 4 threads (8 → 2 288 ms, 24 → 2 636 ms, where
/// 0.25 gave 348 and 935).
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
const WEB_G2_SHARE: f64 = 0.75;
#[cfg(gpu_wgpu)]
const WGPU_G2_SHARE: f64 = 0.25;

#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
thread_local! {
    /// The page's WebGPU device, once init_gpu has it.
    static WEB_GPU: std::cell::RefCell<Option<std::sync::Arc<ringil_wgpu::Gpu>>> = const { std::cell::RefCell::new(None) };
}

/// Requests the page's WebGPU device for the keys built after it: their
/// MSMs and quotient go to the GPU through solve_parts_gpu. False without
/// WebGPU (the keys then prove on the CPU).
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
#[wasm_bindgen]
pub async fn init_gpu() -> bool {
    match ringil_wgpu::Gpu::request().await {
        Ok(gpu) => {
            WEB_GPU.with(|device| *device.borrow_mut() = Some(std::sync::Arc::new(gpu)));
            true
        }
        Err(_) => false,
    }
}

/// A GPU taking the key's vectors one at a time, so that the key keeps no
/// copy of what the GPU holds.
#[cfg(gpu_wgpu)]
struct GpuBuilder {
    started: crate::time::Instant,
    gpu: std::sync::Arc<ringil_wgpu::Gpu>,
    quotient: ringil_wgpu::Quotient,
    g1: [Option<ringil_wgpu::G1Msm>; 4],
    b2: Option<ringil_wgpu::G2Msm>,
    b2_cut: usize,
}
#[cfg(gpu_wgpu)]
impl GpuBuilder {
    /// None with RINGIL_GPU=0 or without a GPU; on Android only with
    /// RINGIL_GPU=1: measured on a Galaxy A16 (Mali-G57), A56 (Xclipse 940)
    /// and S25 (Adreno 830), the WGSL MSMs took minutes to compile and ran
    /// 3–5× slower than the phone's own cores, or did not link at all.
    fn open(plan: &fft::Plan) -> Option<Self> {
        let wanted = std::env::var("RINGIL_GPU").ok();
        if wanted.as_deref() == Some("0")
            || (cfg!(target_os = "android") && wanted.as_deref() != Some("1"))
        {
            return None;
        }
        let started = crate::time::Instant::now();
        #[cfg(not(target_arch = "wasm32"))]
        let gpu = std::sync::Arc::new(ringil_wgpu::Gpu::new().ok()?);
        // In a browser the device is requested ahead, asynchronously
        // (init_gpu); building on it is synchronous.
        #[cfg(target_arch = "wasm32")]
        let gpu = WEB_GPU.with(|device| device.borrow().clone())?;
        let (roots, inverse_roots, forward, reverse) = plan.tables()?;
        let quotient = ringil_wgpu::Quotient::new(
            gpu.clone(),
            plan.size(),
            &roots,
            &inverse_roots,
            &forward,
            &reverse,
        )
        .ok()?
        .0;
        Some(Self {
            started,
            gpu,
            quotient,
            g1: [None, None, None, None],
            b2: None,
            b2_cut: 0,
        })
    }
    fn g1(&mut self, which: usize, bases: &[G1Affine]) -> Result<(), String> {
        self.g1[which] = Some(ringil_wgpu::G1Msm::new(self.gpu.clone(), bases)?.0);
        Ok(())
    }
    /// How many of the G2 points (the first) the GPU took.
    fn g2(&mut self, bases: &[G2Affine]) -> Result<usize, String> {
        let cut = (bases.len() as f64 * g2_gpu_share(self.gpu.spirv)).round() as usize;
        if cut == 0 {
            return Ok(0);
        }
        self.b2 = Some(ringil_wgpu::G2Msm::new(self.gpu.clone(), &bases[..cut])?.0);
        self.b2_cut = cut;
        Ok(cut)
    }
    fn finish(self) -> Gpu {
        let [a, b, k, z] = self
            .g1
            .map(|msm| std::sync::Mutex::new(msm.expect("every G1 vector uploaded")));
        trace("gpu-upload", self.started);
        Gpu {
            a,
            b,
            k,
            z,
            b2: self.b2.map(std::sync::Mutex::new),
            b2_cut: self.b2_cut,
            quotient: std::sync::Mutex::new(self.quotient),
        }
    }
}

/// Hands a decoded vector to the GPU and keeps nothing of it, or keeps it
/// when there is no GPU. A GPU that cannot take the key fails the build:
/// the caller then proves another way.
#[cfg(gpu_any)]
fn upload_g1(
    gpu: &mut Option<GpuBuilder>,
    which: usize,
    bases: Vec<G1Affine>,
) -> Result<Vec<G1Affine>, &'static str> {
    match gpu {
        Some(builder) => {
            builder
                .g1(which, &bases)
                .map_err(|_| "the GPU could not take the key")?;
            #[cfg(feature = "fullcheck")]
            {
                let msm = builder.g1[which].as_mut().expect("uploaded");
                selftest::full_g1(["a", "b", "k", "z"][which], &bases, |scalars| {
                    msm.msm(scalars).map(|r| r.0)
                });
            }
            // In a browser the CPU keeps its copy: solve_parts proves there.
            #[cfg(target_arch = "wasm32")]
            return Ok(bases);
            #[allow(unreachable_code)]
            Ok(Vec::new())
        }
        None => Ok(bases),
    }
}
#[cfg(gpu_any)]
fn upload_g2(
    gpu: &mut Option<GpuBuilder>,
    bases: Vec<G2Affine>,
) -> Result<Vec<G2Affine>, &'static str> {
    #[cfg(feature = "fullcheck")]
    selftest::full_g2(&bases);
    match gpu {
        Some(builder) => {
            // The CPU keeps the points after those the GPU took; in a
            // browser it keeps them all, for solve_parts on the CPU.
            let cut = builder
                .g2(&bases)
                .map_err(|_| "the GPU could not take the key")?;
            if cfg!(target_arch = "wasm32") || cut == 0 {
                Ok(bases)
            } else {
                Ok(bases[cut..].to_vec())
            }
        }
        None => Ok(bases),
    }
}
#[cfg(all(gpu_any, not(target_arch = "wasm32")))]
fn gpu_trace(name: &str, sort_ms: f64, gpu_ms: f64) {
    #[cfg(feature = "trace")]
    log::info!("RINGIL_TRACE gpu-msm-{name} sort {sort_ms:.1} gpu {gpu_ms:.1} ms");
    #[cfg(not(feature = "trace"))]
    let _ = (name, sort_ms, gpu_ms);
}

/// A GPU taking the key's vectors one at a time, so that the key keeps no
/// copy of what the GPU holds: a phone has no room for two.
#[cfg(gpu)]
struct GpuBuilder {
    started: crate::time::Instant,
    quotient: ringil_metal::Quotient,
    g1: [Option<ringil_metal::G1Msm>; 4],
    b2: Option<ringil_metal::G2Msm>,
    b2_cut: usize,
    /// The trees and buckets all the key's MSMs share.
    scratch: std::sync::Arc<ringil_metal::Scratch>,
}
#[cfg(gpu)]
impl GpuBuilder {
    /// None with RINGIL_GPU=0 or without a GPU.
    fn open(plan: &fft::Plan) -> Option<Self> {
        if std::env::var("RINGIL_GPU").is_ok_and(|v| v == "0") {
            return None;
        }
        let started = crate::time::Instant::now();
        let (roots, inverse_roots, forward, reverse) = plan.tables()?;
        let quotient =
            ringil_metal::Quotient::new(plan.size(), &roots, &inverse_roots, &forward, &reverse)
                .ok()?
                .0;
        let scratch = ringil_metal::Scratch::new().ok()?;
        Some(Self {
            started,
            quotient,
            g1: [None, None, None, None],
            b2: None,
            b2_cut: 0,
            scratch,
        })
    }
    fn g1(&mut self, which: usize, bases: &[G1Affine]) -> Result<(), String> {
        self.g1[which] = Some(ringil_metal::G1Msm::new_in(bases, self.scratch.clone())?.0);
        Ok(())
    }
    /// Whether the GPU took G2: a Mac's does, a phone's leaves it to the CPU.
    /// How many of the G2 points (the first) the GPU took.
    fn g2(&mut self, bases: &[G2Affine]) -> Result<usize, String> {
        let cut = (bases.len() as f64 * g2_gpu_share()).round() as usize;
        if cut == 0 {
            return Ok(0);
        }
        self.b2 = Some(ringil_metal::G2Msm::new_in(bases, self.scratch.clone())?.0);
        self.b2_cut = cut;
        Ok(cut)
    }
    fn finish(self) -> Gpu {
        let [a, b, k, z] = self
            .g1
            .map(|msm| std::sync::Mutex::new(msm.expect("every G1 vector uploaded")));
        trace("gpu-upload", self.started);
        #[cfg(feature = "trace")]
        log::info!(
            "RINGIL_TRACE gpu-allocated {} MB",
            ringil_metal::allocated_bytes() >> 20
        );
        Gpu {
            a,
            b,
            k,
            z,
            b2: self.b2.map(std::sync::Mutex::new),
            b2_cut: self.b2_cut,
            quotient: std::sync::Mutex::new(self.quotient),
        }
    }
}

#[wasm_bindgen]
pub struct Key {
    #[cfg(gpu_any)]
    gpu: Option<GpuHold>,
    /// Whether the GPU's quotient agreed with the CPU's at load.
    #[cfg(gpu_any)]
    gpu_quotient: bool,
    /// Whether the trace build has profiled this key's GPU stages (Metal).
    #[cfg(all(feature = "trace", gpu))]
    profiled: std::sync::atomic::AtomicBool,
    /// The A, B and K vectors' lengths: the vectors themselves are empty
    /// when a GPU holds them.
    lengths: [usize; 3],
    a: Vec<G1Affine>,
    b: Vec<G1Affine>,
    k: Vec<G1Affine>,
    z: Vec<G1Affine>,
    b2: Vec<G2Affine>,
    domain: Radix2EvaluationDomain<Fr>,
    plan: fft::Plan,
    commitments: Vec<(Vec<G1Affine>, Vec<G1Affine>)>,
    solver: Option<solver::Plan>,
}
#[wasm_bindgen]
impl Key {
    #[wasm_bindgen(constructor)]
    pub fn new(
        a: &[u8],
        b: &[u8],
        k: &[u8],
        z: &[u8],
        b2: &[u8],
        domain_params: &[u8],
    ) -> Result<Key, JsError> {
        Self::build(a, b, k, z, b2, domain_params).map_err(JsError::new)
    }
    /// `new` for hosts outside JavaScript, with a plain error.
    pub fn build(
        a: &[u8],
        b: &[u8],
        k: &[u8],
        z: &[u8],
        b2: &[u8],
        domain_params: &[u8],
    ) -> Result<Key, &'static str> {
        if [a, b, k, z].iter().any(|v| !v.len().is_multiple_of(64))
            || !b2.len().is_multiple_of(128)
            || b.len() / 64 != b2.len() / 128
            || domain_params.len() != 64
        {
            return Err("invalid gnark key dimensions");
        }
        let n = z.len() / 64 + 1;
        if !n.is_power_of_two() {
            return Err("invalid gnark FFT domain");
        }
        let params = frs(domain_params);
        let mut domain = Radix2EvaluationDomain::<Fr>::new(n).ok_or("unsupported FFT domain")?;
        domain.group_gen = params[0];
        domain.group_gen_inv = params[0].inverse().ok_or("invalid FFT generator")?;
        let _coset = domain.get_coset(params[1]).ok_or("invalid FFT coset")?;
        let den = (params[1].pow([n as u64]) - Fr::ONE)
            .inverse()
            .ok_or("invalid quotient denominator")?;
        let started = crate::time::Instant::now();
        let plan = fft::Plan::new(n, params[0], params[1], den);
        trace("plan", started);
        let lengths = [a.len() / 64, b.len() / 64, k.len() / 64];
        // One vector at a time: decoded, handed to the GPU, dropped. A GPU
        // that cannot take the key (it failed to build a kernel, it ran out
        // of memory) is let go, and the key is decoded again for the CPU.
        let started = crate::time::Instant::now();
        #[cfg(gpu_any)]
        let mut gpu = GpuBuilder::open(&plan);
        #[cfg(not(gpu_any))]
        let mut gpu: Option<()> = None;
        trace("gpu-open", started);
        #[cfg(not(gpu_any))]
        let (upload_g1, upload_g2) = (
            |_: &mut Option<()>, _: usize, v| Ok::<_, &'static str>(v),
            |_: &mut Option<()>, v| Ok::<_, &'static str>(v),
        );
        #[cfg(not(target_arch = "wasm32"))]
        let head = |n: usize| n.min(selftest::HEAD);
        #[cfg(target_arch = "wasm32")]
        let head = |_: usize| 0;
        let decode = |gpu: &mut Option<_>| -> Result<_, &'static str> {
            let started = crate::time::Instant::now();
            let a = g1s(a);
            let a_head = a[..head(a.len())].to_vec();
            let started = trace("decode-a", started);
            let a = upload_g1(gpu, 0, a)?;
            let started = trace("upload-a", started);
            let b = upload_g1(gpu, 1, g1s(b))?;
            let started = trace("decode+upload-b", started);
            let k = upload_g1(gpu, 2, g1s(k))?;
            let started = trace("decode+upload-k", started);
            let z = upload_g1(gpu, 3, g1s(z))?;
            let started = trace("decode+upload-z", started);
            let b2 = g2s(b2);
            let b2_head = b2[..head(b2.len())].to_vec();
            let started = trace("decode-b2", started);
            let b2 = upload_g2(gpu, b2)?;
            trace("upload-b2", started);
            Ok((a, b, k, z, b2, a_head, b2_head))
        };
        #[cfg(not(target_arch = "wasm32"))]
        selftest::field();
        #[cfg(gpu_any)]
        let mut gpu_quotient = true;
        #[cfg(gpu_any)]
        let mut built_gpu: Option<Gpu> = None;
        let (a, b, k, z, b2) = loop {
            match decode(&mut gpu) {
                Ok((a, b, k, z, b2, a_head, b2_head)) => {
                    // A GPU whose MSM disagrees with the CPU is let go, and the
                    // key is decoded again for the CPU; a quotient that
                    // disagrees stays on the CPU.
                    #[cfg(gpu_any)]
                    if let Some(builder) = gpu.take() {
                        let mut finished = builder.finish();
                        #[cfg(not(target_arch = "wasm32"))]
                        let verdict = selftest::gpu(
                            &mut finished,
                            &plan,
                            &a_head,
                            &b2_head,
                            lengths[0],
                            lengths[1],
                        );
                        #[cfg(target_arch = "wasm32")]
                        let verdict = {
                            let _ = &mut finished;
                            selftest_web::Verdict {
                                msm: true,
                                quotient: true,
                            }
                        };
                        if !verdict.msm {
                            drop(finished);
                            // With Metal, once more with the shaders compiled at
                            // the size level, where aeglos found the default
                            // level's compiler at fault.
                            #[cfg(gpu)]
                            if ringil_metal::prefer_smaller_code() {
                                trace("gpu-retry-smaller-code", crate::time::Instant::now());
                                gpu = GpuBuilder::open(&plan);
                            }
                            continue;
                        }
                        gpu_quotient = verdict.quotient
                            && !std::env::var("RINGIL_GPU_QUOTIENT").is_ok_and(|v| v == "0");
                        built_gpu = Some(finished);
                    }
                    let _ = (a_head, b2_head);
                    break (a, b, k, z, b2);
                }
                Err(_) if gpu.is_some() => gpu = None,
                Err(e) => return Err(e),
            }
        };
        // The GPU holds the quotient's tables: the CPU's go, unless the CPU
        // is to compute the quotient.
        #[cfg(gpu_any)]
        let mut plan = plan;
        #[cfg(gpu_any)]
        if built_gpu.is_some() && gpu_quotient {
            plan.release_tables();
        }
        Ok(Self {
            #[cfg(gpu_any)]
            gpu: built_gpu.map(|gpu| GpuHold(std::sync::Arc::new(gpu))),
            #[cfg(gpu_any)]
            gpu_quotient,
            #[cfg(all(feature = "trace", gpu))]
            profiled: Default::default(),
            lengths,
            a,
            b,
            k,
            z,
            b2,
            domain,
            plan,
            commitments: Vec::new(),
            solver: None,
        })
    }
    pub fn add_commitment(&mut self, basis: &[u8], sigma: &[u8]) -> Result<(), JsError> {
        if !basis.len().is_multiple_of(64) || basis.len() != sigma.len() {
            return Err(JsError::new("invalid commitment basis"));
        }
        self.commitments.push((g1s(basis), g1s(sigma)));
        self.solver = None;
        Ok(())
    }
    // Unsupported plans retain gnark's solver. A runtime witness failure is
    // returned to the caller and never retried with a different solver.
    pub fn set_solver(&mut self, program: &[u8], coefficients: &[u8]) -> bool {
        self.solver = None;
        if !self.commitments.is_empty() {
            return false;
        }
        let started = crate::time::Instant::now();
        let parsed = solver::Plan::new(program, coefficients);
        trace("solver-plan", started);
        if let Ok(plan) = parsed {
            if plan.matches(
                self.lengths[0],
                self.lengths[1],
                self.lengths[2],
                self.domain.size(),
            ) {
                self.solver = Some(plan);
                return true;
            }
        }
        false
    }
    // The prepared solver as an image a host keeps, and loads again with
    // set_solver_image instead of building the plan from the program.
    pub fn solver_image(&self) -> Option<Vec<u8>> {
        self.solver.as_ref().map(solver::Plan::image)
    }
    // A solver from solver_image's bytes, if they are an image of this
    // kernel's version for a key of these dimensions.
    pub fn set_solver_image(&mut self, image: &[u8]) -> bool {
        let started = crate::time::Instant::now();
        self.solver = None;
        if !self.commitments.is_empty() {
            return false;
        }
        let parsed = solver::Plan::from_image(image);
        trace("solver-image", started);
        #[cfg(feature = "trace")]
        if let Ok(plan) = &parsed {
            log::info!("RINGIL_TRACE solver-plan-size {}", plan.footprint());
        }
        if let Ok(plan) = parsed {
            if plan.matches(
                self.lengths[0],
                self.lengths[1],
                self.lengths[2],
                self.domain.size(),
            ) {
                self.solver = Some(plan);
                return true;
            }
        }
        false
    }
    pub fn solve_parts(&self, witness: &[u8]) -> Result<Vec<u8>, JsError> {
        self.solve(witness).map_err(|error| JsError::new(&error))
    }
    /// Whether the key holds the page's GPU: solve_parts_gpu proves with it.
    pub fn has_gpu(&self) -> bool {
        #[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
        return self.gpu.is_some();
        #[allow(unreachable_code)]
        false
    }
    /// solve_parts with the GPU, in a browser: a Promise of the parts. The
    /// solve, the GPU's submissions and the CPU's share of the G2 MSM run
    /// now; the GPU's results are awaited, the quotient's first so that Z's
    /// MSM follows the others.
    #[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
    pub fn solve_parts_gpu(&self, witness: &[u8]) -> Result<js_sys::Promise, JsError> {
        let gpu = self
            .gpu
            .clone()
            .ok_or_else(|| JsError::new("the key has no GPU"))?;
        let plan = self
            .solver
            .as_ref()
            .ok_or_else(|| JsError::new("no prepared solver"))?;
        if !witness.len().is_multiple_of(32) {
            return Err(JsError::new("invalid witness encoding"));
        }
        let solution = plan.solve(&frs(witness)).map_err(|e| JsError::new(&e))?;
        let sa: Vec<_> = plan.a_indices.iter().map(|i| solution.w[*i]).collect();
        let sb: Vec<_> = plan.b_indices.iter().map(|i| solution.w[*i]).collect();
        let sk = &solution.w[plan.n_public..];
        let cut = if gpu.b2.is_some() { gpu.b2_cut } else { 0 };
        let (a, b, c) = (solution.a, solution.b, solution.c);
        let (h_from_cpu, quotient_on_gpu) = if self.gpu_quotient {
            gpu.quotient.lock().unwrap().start(&a, &b, &c);
            (None, true)
        } else {
            (Some(self.quotient(a, b, c)), false)
        };
        let pa = gpu.a.lock().unwrap().start(&sa);
        let pb = gpu.b.lock().unwrap().start(&sb);
        let pk = gpu.k.lock().unwrap().start(sk);
        let pb2 = gpu
            .b2
            .as_ref()
            .map(|b2| b2.lock().unwrap().start(&sb[..cut]));
        // The CPU's share of G2 while the GPU works.
        let cpu_b2 = (cut < sb.len()).then(|| msm_g2(&self.b2[cut..], &sb[cut..]));
        Ok(wasm_bindgen_futures::future_to_promise(async move {
            let fail = |e: String| JsValue::from(JsError::new(&e));
            let h = match h_from_cpu {
                Some(h) => h,
                None => {
                    debug_assert!(quotient_on_gpu);
                    let mut quotient = gpu.quotient.lock().unwrap();
                    quotient.finish_async().await.map_err(fail)?
                }
            };
            let pz = gpu.z.lock().unwrap().start(&h);
            let (ar, _) = gpu.a.lock().unwrap().finish_async(pa).await.map_err(fail)?;
            let (bs1, _) = gpu.b.lock().unwrap().finish_async(pb).await.map_err(fail)?;
            let (kr, _) = gpu.k.lock().unwrap().finish_async(pk).await.map_err(fail)?;
            let gpu_b2 = match (&gpu.b2, pb2) {
                (Some(b2), Some(pb2)) => {
                    Some(b2.lock().unwrap().finish_async(pb2).await.map_err(fail)?.0)
                }
                _ => None,
            };
            let (hz, _) = gpu.z.lock().unwrap().finish_async(pz).await.map_err(fail)?;
            let bs2 = match (gpu_b2, cpu_b2) {
                (Some(g), Some(c)) => g + c,
                (Some(g), None) => g,
                (None, Some(c)) => c,
                (None, None) => G2Projective::ZERO,
            };
            // A test build's fault (RINGIL_WEB_FAULT) checks that the host
            // catches a GPU that miscomputes.
            let ar = if option_env!("RINGIL_WEB_FAULT").is_some() {
                ar + <G1Projective as ark_ec::PrimeGroup>::generator()
            } else {
                ar
            };
            let mut out = Vec::with_capacity(384);
            for p in [ar, bs1, kr, hz] {
                output_g1(&mut out, p)
            }
            output_g2(&mut out, bs2);
            Ok(js_sys::Uint8Array::from(out.as_slice()).into())
        }))
    }
    /// `solve_parts` for hosts outside JavaScript, with a plain error.
    pub fn solve(&self, witness: &[u8]) -> Result<Vec<u8>, String> {
        let plan = self.solver.as_ref().ok_or("no prepared solver")?;
        if !witness.len().is_multiple_of(32) {
            return Err("invalid witness encoding".into());
        }
        let started = crate::time::Instant::now();
        let solution = plan.solve(&frs(witness))?;
        trace("solve", started);
        let sa: Vec<_> = plan.a_indices.iter().map(|i| solution.w[*i]).collect();
        let sb: Vec<_> = plan.b_indices.iter().map(|i| solution.w[*i]).collect();
        self.compute_parts(
            &sa,
            &sb,
            &solution.w[plan.n_public..],
            solution.a,
            solution.b,
            solution.c,
        )
    }
    pub fn commitment(
        &self,
        index: usize,
        knowledge: bool,
        values: &[u8],
    ) -> Result<Vec<u8>, JsError> {
        let key = self
            .commitments
            .get(index)
            .ok_or_else(|| JsError::new("unknown commitment key"))?;
        let basis = if knowledge { &key.1 } else { &key.0 };
        if values.len() != basis.len() * 32 {
            return Err(JsError::new("commitment witness length mismatch"));
        }
        let point = msm_g1(basis, &frs(values));
        let mut out = Vec::with_capacity(64);
        output_g1(&mut out, point);
        Ok(out)
    }

    pub fn parts(
        &self,
        sa: &[u8],
        sb: &[u8],
        sk: &[u8],
        a: &[u8],
        b: &[u8],
        c: &[u8],
    ) -> Result<Vec<u8>, JsError> {
        if sa.len() != self.lengths[0] * 32
            || sb.len() != self.lengths[1] * 32
            || sk.len() != self.lengths[2] * 32
            || a.len() != b.len()
            || a.len() != c.len()
            || !a.len().is_multiple_of(32)
            || a.len() / 32 > self.domain.size()
        {
            return Err(JsError::new("witness does not match prepared gnark key"));
        }
        let sa = frs(sa);
        let sb = frs(sb);
        let sk = frs(sk);
        self.compute_parts(&sa, &sb, &sk, frs(a), frs(b), frs(c))
            .map_err(|e| JsError::new(&e))
    }
}
impl Key {
    /// The A, B and K MSM scalars of a witness, for the bench's profile of
    /// how skewed they are.
    #[cfg(feature = "trace")]
    #[doc(hidden)]
    pub fn msm_scalars(&self, witness: &[u8]) -> Result<[Vec<Fr>; 3], String> {
        let plan = self.solver.as_ref().ok_or("no prepared solver")?;
        let solution = plan.solve(&frs(witness))?;
        Ok([
            plan.a_indices.iter().map(|i| solution.w[*i]).collect(),
            plan.b_indices.iter().map(|i| solution.w[*i]).collect(),
            solution.w[plan.n_public..].to_vec(),
        ])
    }
    fn compute_parts(
        &self,
        sa: &[Fr],
        sb: &[Fr],
        sk: &[Fr],
        a: Vec<Fr>,
        b: Vec<Fr>,
        c: Vec<Fr>,
    ) -> Result<Vec<u8>, String> {
        #[cfg(gpu)]
        if let Some(gpu) = &self.gpu {
            // The MSMs whose scalars are ready run on the GPU, driven from a
            // thread of their own so that their waits hold no rayon worker,
            // while the GPU also takes the quotient and the pool computes
            // G2 when it is not on the GPU; Z's MSM follows the quotient.
            // The sorts run on the pool, so a CPU G2 MSM starts only once
            // they are done: otherwise it starves them for its whole run.
            let (sorted_tx, sorted_rx) = std::sync::mpsc::channel::<()>();
            // The quotient comes back too, for the trace build's profile.
            let (h, (ar, bs1, kr, bs2, hz)) = std::thread::scope(|scope| -> Result<_, String> {
                let msms = scope.spawn(move || -> Result<_, String> {
                    // Each MSM is submitted as soon as it is sorted, so the
                    // host sorts the next while the GPU runs it; G2 B shares
                    // G1 B's scalars, and so its sorted runs.
                    let (mut a, mut b, mut k) = (
                        gpu.a.lock().unwrap(),
                        gpu.b.lock().unwrap(),
                        gpu.k.lock().unwrap(),
                    );
                    let pa = a.start(sa);
                    let pb = b.start(sb);
                    let mut b2 = gpu.b2.as_ref().map(|b2| b2.lock().unwrap());
                    let pb2 = b2.as_mut().map(|b2| {
                        if gpu.b2_cut == sb.len() {
                            b2.start_sorted_as(&*b)
                        } else {
                            // The CPU's points weigh nothing here.
                            let mut mine = sb.to_vec();
                            mine[gpu.b2_cut..].fill(Fr::ZERO);
                            b2.start(&mine)
                        }
                    });
                    let pk = k.start(sk);
                    drop(sorted_tx);
                    let (ar, t) = a.finish(pa)?;
                    gpu_trace("a", t.sort_ms, t.gpu_ms);
                    let (bs1, t) = b.finish(pb)?;
                    gpu_trace("b", t.sort_ms, t.gpu_ms);
                    let bs2 = match (b2.as_mut(), pb2) {
                        (Some(b2), Some(pb2)) => {
                            let (bs2, t) = b2.finish(pb2)?;
                            gpu_trace("b2", t.sort_ms, t.gpu_ms);
                            Some(bs2)
                        }
                        _ => None,
                    };
                    let (kr, t) = k.finish(pk)?;
                    gpu_trace("k", t.sort_ms, t.gpu_ms);
                    Ok((ar, bs1, kr, bs2))
                });
                let cut = if gpu.b2.is_some() { gpu.b2_cut } else { 0 };
                let cpu_b2 = (cut < sb.len()).then(|| {
                    scope.spawn(move || {
                        let _ = sorted_rx.recv();
                        let started = crate::time::Instant::now();
                        let bs2 = msm_g2(&self.b2, &sb[cut..]);
                        trace("cpu-msm-b2", started);
                        bs2
                    })
                });
                let started = crate::time::Instant::now();
                let h = if !self.gpu_quotient {
                    self.quotient(a, b, c)
                } else {
                    gpu.quotient.lock().unwrap().quotient(&a, &b, &c)?
                };
                trace("quotient", started);
                // Z's MSM is sorted and queued now, behind the others still
                // on the GPU, rather than after them.
                let mut z = gpu.z.lock().unwrap();
                let pz = z.start(&h);
                let (ar, bs1, kr, bs2) = msms.join().expect("gpu msms")?;
                let cpu = cpu_b2.map(|cpu| cpu.join().expect("cpu msm"));
                let bs2 = match (bs2, cpu) {
                    (Some(gpu), Some(cpu)) => gpu + cpu,
                    (Some(gpu), None) => gpu,
                    (None, Some(cpu)) => cpu,
                    (None, None) => unreachable!("a G2 MSM"),
                };
                let started = trace("quotient+gpu-msms", started);
                let (hz, t) = z.finish(pz)?;
                gpu_trace("z", t.sort_ms, t.gpu_ms);
                trace("gpu-msm-z", started);
                Ok((h, (ar, bs1, kr, bs2, hz)))
            })?;
            // The trace build profiles the GPU's stages once per key, with
            // this proof's scalars, after the proof (the next one pays it).
            #[cfg(not(feature = "trace"))]
            drop(h);
            #[cfg(feature = "trace")]
            if !self
                .profiled
                .swap(true, std::sync::atomic::Ordering::Relaxed)
            {
                for (name, msm, scalars) in [
                    ("a", &gpu.a, sa),
                    ("b", &gpu.b, sb),
                    ("k", &gpu.k, sk),
                    ("z", &gpu.z, &h[..]),
                ] {
                    log::info!(
                        "RINGIL_TRACE gpu-profile {name} {}",
                        msm.lock().unwrap().profile(scalars)
                    );
                }
            }
            let mut out = Vec::with_capacity(384);
            for p in [ar, bs1, kr, hz] {
                output_g1(&mut out, p)
            }
            output_g2(&mut out, bs2);
            return Ok(out);
        }
        #[cfg(all(gpu_wgpu, not(target_arch = "wasm32")))]
        if let Some(gpu) = &self.gpu {
            // On a thread of their own: the quotient submitted to the GPU,
            // the MSMs whose scalars are out sorted and queued behind it
            // while it runs (and the GPU's share of G2), then Z's once the
            // quotient is back; the pool computes the CPU's share of G2 as
            // soon as the first sorts are done. (Waiting for the quotient
            // before sorting left the phone's cores idle for its whole run:
            // 116 ms of an 8 → 2 proof on a OnePlus 13.)
            let started = crate::time::Instant::now();
            let (sorted_tx, sorted_rx) = std::sync::mpsc::channel::<()>();
            let cut = if gpu.b2.is_some() { gpu.b2_cut } else { 0 };
            let ((ar, bs1, kr, hz, gpu_b2), cpu_b2) =
                std::thread::scope(|scope| -> Result<_, String> {
                    let gpu_work = scope.spawn(move || -> Result<_, String> {
                        let started = crate::time::Instant::now();
                        let mut quotient = self.gpu_quotient.then(|| gpu.quotient.lock().unwrap());
                        if let Some(quotient) = quotient.as_mut() {
                            quotient.start(&a, &b, &c);
                        }
                        let (mut ma, mut mb, mut mk, mut mz) = (
                            gpu.a.lock().unwrap(),
                            gpu.b.lock().unwrap(),
                            gpu.k.lock().unwrap(),
                            gpu.z.lock().unwrap(),
                        );
                        let (pa, pb, pk) = (ma.start(sa), mb.start(sb), mk.start(sk));
                        let mut b2 = gpu.b2.as_ref().map(|b2| b2.lock().unwrap());
                        let pb2 = b2.as_mut().map(|b2| b2.start(&sb[..cut]));
                        drop(sorted_tx);
                        let h = match quotient.as_mut() {
                            Some(quotient) => quotient.finish()?,
                            None => self.quotient(a, b, c),
                        };
                        drop(quotient);
                        trace("quotient", started);
                        // By now the pool runs the CPU's G2 MSM, whose tasks
                        // Z's sort would queue behind (550 ms on a OnePlus
                        // 13): it gets threads of its own, which the GPU,
                        // busy with A, B and K, does not wait for.
                        let z = &mut *mz;
                        let pz = sort_pool().install(|| z.start(&h));
                        let (ar, t) = ma.finish(pa)?;
                        gpu_trace("a", t.sort_ms, t.gpu_ms);
                        let (bs1, t) = mb.finish(pb)?;
                        gpu_trace("b", t.sort_ms, t.gpu_ms);
                        let (kr, t) = mk.finish(pk)?;
                        gpu_trace("k", t.sort_ms, t.gpu_ms);
                        let gpu_b2 = match (b2.as_mut(), pb2) {
                            (Some(b2), Some(pb2)) => {
                                let (bs2, t) = b2.finish(pb2)?;
                                gpu_trace("b2", t.sort_ms, t.gpu_ms);
                                Some(bs2)
                            }
                            _ => None,
                        };
                        let (hz, t) = mz.finish(pz)?;
                        gpu_trace("z", t.sort_ms, t.gpu_ms);
                        Ok((ar, bs1, kr, hz, gpu_b2))
                    });
                    let _ = sorted_rx.recv();
                    let started = crate::time::Instant::now();
                    let cpu_b2 = (cut < sb.len()).then(|| msm_g2(&self.b2, &sb[cut..]));
                    trace("cpu-msm-b2", started);
                    Ok((gpu_work.join().expect("gpu work")?, cpu_b2))
                })?;
            trace("quotient+gpu-msms", started);
            let bs2 = match (gpu_b2, cpu_b2) {
                (Some(gpu), Some(cpu)) => gpu + cpu,
                (Some(gpu), None) => gpu,
                (None, Some(cpu)) => cpu,
                (None, None) => unreachable!("a G2 MSM"),
            };
            let mut out = Vec::with_capacity(384);
            for p in [ar, bs1, kr, hz] {
                output_g1(&mut out, p)
            }
            output_g2(&mut out, bs2);
            return Ok(out);
        }
        let started = crate::time::Instant::now();
        let h = self.quotient(a, b, c);
        let started = trace("quotient", started);
        let ((ar, bs1), (kr, (hz, bs2))) = rayon::join(
            || rayon::join(|| msm_g1(&self.a, sa), || msm_g1(&self.b, sb)),
            || {
                rayon::join(
                    || msm_g1(&self.k, sk),
                    || rayon::join(|| msm_g1(&self.z, &h), || msm_g2(&self.b2, sb)),
                )
            },
        );
        trace("msms", started);
        // Each MSM alone, on the whole pool, for its own share: this adds
        // to the proof time, so only with RINGIL_TRACE_MSMS=1.
        #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
        if std::env::var("RINGIL_TRACE_MSMS").is_ok_and(|v| v == "1") {
            for (label, bases, scalars) in [
                ("msm-a", &self.a, sa),
                ("msm-b", &self.b, sb),
                ("msm-k", &self.k, sk),
                ("msm-z", &self.z, &h[..]),
            ] {
                let started = crate::time::Instant::now();
                let _ = msm_g1(bases, scalars);
                trace(label, started);
            }
            let started = crate::time::Instant::now();
            let _ = msm_g2(&self.b2, sb);
            trace("msm-b2", started);
        }
        let mut out = Vec::with_capacity(384);
        for p in [ar, bs1, kr, hz] {
            output_g1(&mut out, p)
        }
        output_g2(&mut out, bs2);
        Ok(out)
    }
    fn quotient(&self, a: Vec<Fr>, b: Vec<Fr>, c: Vec<Fr>) -> Vec<Fr> {
        self.plan.quotient(a, b, c)
    }
}

pub fn msm_g1(bases: &[G1Affine], scalars: &[Fr]) -> G1Projective {
    if bases.is_empty() {
        G1Projective::ZERO
    } else if affine_msm() {
        #[cfg(not(target_arch = "wasm32"))]
        return msm_affine::msm(bases, scalars);
        #[allow(unreachable_code)]
        msm::msm(bases, scalars)
    } else {
        msm::msm(bases, scalars)
    }
}
pub fn msm_g2(bases: &[G2Affine], scalars: &[Fr]) -> G2Projective {
    if bases.is_empty() {
        G2Projective::ZERO
    } else if affine_msm() {
        #[cfg(not(target_arch = "wasm32"))]
        return msm_affine::msm(bases, scalars);
        #[allow(unreachable_code)]
        msm::msm(bases, scalars)
    } else {
        msm::msm(bases, scalars)
    }
}

/// Which CPU MSM runs: natively the batch-affine one (on a Galaxy-class
/// phone's Cortex-A78 it proved 2 → 2 in 623 ms against the lane form's
/// 988, an M5 Pro the same), unless RINGIL_MSM=lanes; WebAssembly the lane
/// form, which its SIMD lanes suit.
fn affine_msm() -> bool {
    !cfg!(target_arch = "wasm32") && !std::env::var("RINGIL_MSM").is_ok_and(|v| v == "lanes")
}

// The kernel's little-endian Montgomery encodings, as the Go bridge and the
// key image carry them, for hosts of this crate.
pub fn decode_g1(data: &[u8]) -> Vec<G1Affine> {
    g1s(data)
}
pub fn decode_g2(data: &[u8]) -> Vec<G2Affine> {
    g2s(data)
}
pub fn decode_frs(data: &[u8]) -> Vec<Fr> {
    frs(data)
}

#[cfg(all(test, gpu_any))]
mod gpu_tests {
    use super::*;
    use ark_ec::CurveGroup;
    use ark_ff::UniformRand;
    use rand::{rngs::StdRng, SeedableRng};

    fn limbs(out: &mut Vec<u8>, x: &Fq) {
        for word in x.0 .0 {
            out.extend_from_slice(&word.to_le_bytes());
        }
    }
    fn g1_bytes(points: &[G1Affine]) -> Vec<u8> {
        let mut out = Vec::new();
        for p in points {
            limbs(&mut out, &p.x);
            limbs(&mut out, &p.y);
        }
        out
    }
    fn g2_bytes(points: &[G2Affine]) -> Vec<u8> {
        let mut out = Vec::new();
        for p in points {
            for x in [&p.x.c0, &p.x.c1, &p.y.c0, &p.y.c1] {
                limbs(&mut out, x);
            }
        }
        out
    }
    fn fr_bytes(values: &[Fr]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|v| v.0 .0.iter().flat_map(|w| w.to_le_bytes()))
            .collect()
    }
    fn g1_points(n: usize, rng: &mut StdRng) -> Vec<G1Affine> {
        let (mut p, step) = (G1Projective::rand(rng), G1Projective::rand(rng));
        let all: Vec<_> = (0..n)
            .map(|_| {
                p += step;
                p
            })
            .collect();
        G1Projective::normalize_batch(&all)
    }
    fn g2_points(n: usize, rng: &mut StdRng) -> Vec<G2Affine> {
        let (mut p, step) = (G2Projective::rand(rng), G2Projective::rand(rng));
        let all: Vec<_> = (0..n)
            .map(|_| {
                p += step;
                p
            })
            .collect();
        G2Projective::normalize_batch(&all)
    }

    /// A key on the GPU proves the parts a key on the CPU proves: the MSMs
    /// (G2 split between the two where the defaults say), the quotient, and
    /// the window width and ordering of the GPU path.
    #[test]
    fn gpu_parts_match_cpu() {
        let mut rng = StdRng::seed_from_u64(11);
        let (n, domain) = (3000, 4096);
        let (a, b, k) = (
            g1_points(n, &mut rng),
            g1_points(n, &mut rng),
            g1_points(n, &mut rng),
        );
        let z = g1_points(domain - 1, &mut rng);
        let b2 = g2_points(n, &mut rng);
        let generator = Radix2EvaluationDomain::<Fr>::new(domain).unwrap().group_gen;
        let params = fr_bytes(&[generator, Fr::from(5u64)]);
        let bytes = [
            g1_bytes(&a),
            g1_bytes(&b),
            g1_bytes(&k),
            g1_bytes(&z),
            g2_bytes(&b2),
        ];
        let build = || {
            Key::build(
                &bytes[0], &bytes[1], &bytes[2], &bytes[3], &bytes[4], &params,
            )
            .expect("key")
        };
        // SAFETY: no other test reads RINGIL_GPU.
        unsafe { std::env::set_var("RINGIL_GPU", "0") };
        let cpu = build();
        unsafe { std::env::remove_var("RINGIL_GPU") };
        let gpu = build();
        if gpu.gpu.is_none() {
            eprintln!("no GPU: skipped");
            return;
        }
        for round in 0..2 {
            let random = |len: usize, rng: &mut StdRng| -> Vec<u8> {
                fr_bytes(
                    &(0..len)
                        .map(|i| {
                            if i % 3 == 0 {
                                Fr::from(1u64)
                            } else {
                                Fr::rand(rng)
                            }
                        })
                        .collect::<Vec<_>>(),
                )
            };
            let (sa, sb, sk) = (
                random(n, &mut rng),
                random(n, &mut rng),
                random(n, &mut rng),
            );
            let (ea, eb, ec) = (
                random(domain, &mut rng),
                random(domain, &mut rng),
                random(domain, &mut rng),
            );
            let expected = cpu
                .parts(&sa, &sb, &sk, &ea, &eb, &ec)
                .unwrap_or_else(|_| panic!("cpu parts"));
            let got = gpu
                .parts(&sa, &sb, &sk, &ea, &eb, &ec)
                .unwrap_or_else(|_| panic!("gpu parts"));
            assert_eq!(got, expected, "round {round}");
        }
    }
}
