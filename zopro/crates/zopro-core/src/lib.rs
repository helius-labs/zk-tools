#![cfg_attr(
    all(target_arch = "wasm32", target_feature = "simd128"),
    feature(asm_experimental_arch)
)]
use zopro_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use wasm_bindgen::prelude::*;

/// The clock: std's natively, the page's in a browser, where std has none.
pub mod time {
    #[cfg(not(target_arch = "wasm32"))]
    pub use std::time::Instant;
    #[cfg(target_arch = "wasm32")]
    pub use web_time::Instant;
}
/// The thread pool, in a browser build with shared memory; a build without
/// it (no SharedArrayBuffer, a page that is not cross-origin isolated) runs
/// on the calling thread alone.
#[cfg(all(target_arch = "wasm32", target_feature = "atomics"))]
pub use wasm_bindgen_rayon::init_thread_pool;
pub mod alloc;
pub mod artifact;
mod decompress;
#[cfg(not(target_arch = "wasm32"))]
pub mod dump;
pub mod fft;
pub mod groth16;
mod msm;
#[cfg(not(target_arch = "wasm32"))]
mod msm_affine;
#[cfg(not(target_arch = "wasm32"))]
mod selftest;
mod simd;
mod solver;
#[cfg(test)]
mod testing;
pub mod verify;

// Stage times as ZOPRO_TRACE log lines (`--features trace`): the native
// bench prints them, a phone's bench puts them in the device log.
pub(crate) fn trace(label: &str, started: crate::time::Instant) -> crate::time::Instant {
    #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
    log::info!(
        "ZOPRO_TRACE {label} {:.1} ms{}",
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
    // libSystem's, on iOS too; the libc crate declares it for macOS only.
    extern "C" {
        fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut libc::rusage_info_t) -> i32;
    }
    let mut info: libc::rusage_info_v4 = unsafe { std::mem::zeroed() };
    // SAFETY: the buffer is a rusage_info_v4, as RUSAGE_INFO_V4 asks for.
    let status = unsafe {
        proc_pid_rusage(
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
/// The anonymous resident memory (what Android's low-memory killer weighs)
/// and the peak resident set, file-backed pages included (Linux, Android).
#[cfg(all(feature = "trace", any(target_os = "linux", target_os = "android")))]
fn footprint_note() -> String {
    let Ok(status) = std::fs::read_to_string("/proc/self/status") else {
        return String::new();
    };
    let kb = |field: &str| -> u64 {
        status
            .lines()
            .find_map(|l| l.strip_prefix(field))
            .and_then(|v| v.trim().trim_end_matches("kB").trim().parse().ok())
            .unwrap_or(0)
    };
    format!(" fp {} peak {} MB", kb("RssAnon:") >> 10, kb("VmHWM:") >> 10)
}
#[cfg(all(
    feature = "trace",
    not(any(target_vendor = "apple", target_os = "linux", target_os = "android")),
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

fn words(data: &[u8]) -> [u64; 4] {
    core::array::from_fn(|i| u64::from_le_bytes(data[i * 8..i * 8 + 8].try_into().unwrap()))
}
fn fq(data: &[u8]) -> Fq {
    Fq::from_montgomery(words(data))
}
fn random_bytes() -> [u8; 64] {
    let mut bytes = [0u8; 64];
    getrandom::getrandom(&mut bytes).expect("the system's randomness");
    bytes
}
/// A uniform scalar from the system's randomness.
pub(crate) fn random_fr() -> Fr {
    Fr::from_random_bytes(&random_bytes())
}
/// A uniform base field element from the system's randomness.
pub(crate) fn random_fq() -> Fq {
    Fq::from_random_bytes(&random_bytes())
}
fn frs(data: &[u8]) -> Vec<Fr> {
    data.as_chunks::<32>()
        .0
        .iter()
        .map(|x| Fr::from_montgomery(words(x)))
        .collect()
}
fn g1s(data: &[u8]) -> Vec<G1Affine> {
    data.as_chunks::<64>()
        .0
        .iter()
        .map(|x| G1Affine::new(fq(x), fq(&x[32..])))
        .collect()
}
fn g2s(data: &[u8]) -> Vec<G2Affine> {
    data.as_chunks::<128>()
        .0
        .iter()
        .map(|x| {
            G2Affine::new(
                Fq2::new(fq(x), fq(&x[32..])),
                Fq2::new(fq(&x[64..]), fq(&x[96..])),
            )
        })
        .collect()
}
fn output_fq(out: &mut Vec<u8>, v: Fq) {
    for w in v.0 {
        out.extend_from_slice(&w.to_le_bytes())
    }
}
fn output_g1(out: &mut Vec<u8>, p: G1Projective) {
    let p = p.to_affine();
    output_fq(out, p.x);
    output_fq(out, p.y);
}
fn output_g2(out: &mut Vec<u8>, p: G2Projective) {
    let p = p.to_affine();
    output_fq(out, p.x.c0);
    output_fq(out, p.x.c1);
    output_fq(out, p.y.c0);
    output_fq(out, p.y.c1);
}

// The version of the solver images Key::solver_image writes and
// Key::set_solver_image reads, for a host to tell its images apart.
#[wasm_bindgen]
/// What the GPU has allocated for this process, in bytes (Metal only; zero
/// elsewhere): the bench's stress mode watches it for leaks.
pub fn gpu_allocated_bytes() -> u64 {
    #[cfg(gpu)]
    return zopro_metal::allocated_bytes();
    #[cfg(not(gpu))]
    0
}

/// Compiles the GPU kernels ahead of the first key load (Metal; nothing
/// elsewhere). False when there is no GPU or it could not compile them.
pub fn gpu_warm() -> bool {
    #[cfg(gpu)]
    return zopro_metal::warm().is_ok();
    #[cfg(not(gpu))]
    false
}

pub fn solver_image_version() -> u32 {
    solver::IMAGE_VERSION as u32
}

/// The share of the G2 MSM's points the GPU takes when there is one; the
/// CPU takes the rest at the same time. ZOPRO_G2_GPU_SHARE sets it
/// (ZOPRO_G2=gpu|cpu: all or none); else none on a Mac, whose GPU is the
/// bottleneck of the proof (M5 Pro, 24 → 2: 209 ms at 0, 233 at 0.2; 2 → 2:
/// 41 and 49; a 54 → 1 merge, 423 at 0 and 399 at 0.1), a quarter on an
/// Apple9 phone (A19), whose GPU was idle for about 200 ms of a 535 ms 8 → 2 proof while the CPU's G2
/// MSM took 443, and none on older phones, whose GPUs are busy and slower
/// at G2 than their cores.
#[cfg(gpu)]
fn g2_gpu_share() -> f64 {
    if let Some(share) = std::env::var("ZOPRO_G2_GPU_SHARE")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
    {
        return share.clamp(0.0, 1.0);
    }
    match std::env::var("ZOPRO_G2").as_deref() {
        Ok("gpu") => 1.0,
        Ok("cpu") => 0.0,
        _ if cfg!(target_os = "macos") => 0.0,
        _ if zopro_metal::fast_gpu() => 0.25,
        _ => 0.0,
    }
}

#[cfg(gpu)]
struct Gpu {
    a: std::sync::Mutex<zopro_metal::G1Msm>,
    b: std::sync::Mutex<zopro_metal::G1Msm>,
    k: std::sync::Mutex<zopro_metal::G1Msm>,
    z: std::sync::Mutex<zopro_metal::G1Msm>,
    b2: Option<std::sync::Mutex<zopro_metal::G2Msm>>,
    /// The G2 points the GPU takes: those before this index; the CPU takes
    /// the rest, which is all the key keeps of the vector.
    b2_cut: usize,
    quotient: std::sync::Mutex<zopro_metal::Quotient>,
}
#[cfg(gpu_wgpu)]
struct Gpu {
    a: std::sync::Mutex<zopro_wgpu::G1Msm>,
    b: std::sync::Mutex<zopro_wgpu::G1Msm>,
    k: std::sync::Mutex<zopro_wgpu::G1Msm>,
    z: std::sync::Mutex<zopro_wgpu::G1Msm>,
    b2: Option<std::sync::Mutex<zopro_wgpu::G2Msm>>,
    b2_cut: usize,
    quotient: std::sync::Mutex<zopro_wgpu::Quotient>,
}

/// Threads for a GPU MSM's sort while the global pool computes the CPU's
/// G2 MSM (ZOPRO_SORT_THREADS, default 2).
#[cfg(all(any(gpu_wgpu, gpu), not(target_arch = "wasm32")))]
fn sort_pool() -> &'static rayon::ThreadPool {
    static POOL: std::sync::OnceLock<rayon::ThreadPool> = std::sync::OnceLock::new();
    POOL.get_or_init(|| {
        let threads = std::env::var("ZOPRO_SORT_THREADS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2);
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .thread_name(|i| format!("zopro-sort-{i}"))
            .build()
            .expect("sort pool")
    })
}

/// The share of the G2 MSM's points a wgpu GPU takes (ZOPRO_G2_GPU_SHARE,
/// else measured best: on an M5 Pro through wgpu, and none with the
/// pre-compiled kernels, where the OnePlus 13's Adreno is the critical path
/// already); the CPU takes the rest.
#[cfg(gpu_wgpu)]
fn g2_gpu_share(spirv: bool) -> f64 {
    // A browser has no environment and no pre-compiled kernels: the build's
    // ZOPRO_WEB_G2_SHARE, else by the pool's size.
    #[cfg(target_arch = "wasm32")]
    let _ = spirv;
    #[cfg(target_arch = "wasm32")]
    return option_env!("ZOPRO_WEB_G2_SHARE")
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or_else(|| web_g2_share(rayon::current_num_threads()))
        .clamp(0.0, 1.0);
    #[allow(unreachable_code)]
    std::env::var("ZOPRO_G2_GPU_SHARE")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(if spirv { 0.0 } else { WGPU_G2_SHARE })
        .clamp(0.0, 1.0)
}
/// In a browser, where the CPU's share runs as WebAssembly on the pool
/// while the GPU sorts and sums the rest: the CPU's speed grows with the
/// pool, the GPU's does not. Measured best in Chrome on an M5 Pro: 0.55 at
/// 4 threads, 0.3 at 8 and 0.1 at 16 (a 54 → 1 merge at 16 threads: 702 ms
/// at 0.1, 735 at 0.3, 864 at 0.55; at 4: 984 at 0.55, 1380 at 0.3).
/// Without shared memory the CPU's share would take the one thread that
/// plans and awaits the GPU, so the GPU takes all of G2 (8x2 there: 234
/// ms, against 382 with a quarter on the CPU).
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
fn web_g2_share(threads: usize) -> f64 {
    if cfg!(not(target_feature = "atomics")) {
        return 1.0;
    }
    (2.2 / threads.max(1) as f64).min(0.75)
}
#[cfg(gpu_wgpu)]
const WGPU_G2_SHARE: f64 = 0.25;

#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
thread_local! {
    /// The page's WebGPU device, once init_gpu has it.
    static WEB_GPU: std::cell::RefCell<Option<std::sync::Arc<zopro_wgpu::Gpu>>> = const { std::cell::RefCell::new(None) };
}

/// Per-kernel GPU times from timestamp queries for the proofs after the
/// next `init_gpu` (`gpu_profile_report` gives them): a profile, not for
/// production proofs.
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn gpu_profile(on: bool) {
    zopro_wgpu::profile::set_profiling(on);
}

/// The kernels' GPU times since the last report, a line each:
/// `kernel total_ms ms passes passes`.
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
#[wasm_bindgen]
pub fn gpu_profile_report() -> String {
    zopro_wgpu::profile::report()
}

/// Where a wgpu device keeps its driver's pipeline cache between
/// processes, for the keys built after this: an Adreno compiles the
/// kernels for about 26 s the first time and loads them in half a second
/// from the cache. Nothing where there is no wgpu device.
pub fn set_gpu_pipeline_cache(path: &std::path::Path) {
    #[cfg(all(gpu_wgpu, not(target_arch = "wasm32")))]
    zopro_wgpu::set_pipeline_cache(path.to_path_buf());
    #[cfg(not(all(gpu_wgpu, not(target_arch = "wasm32"))))]
    let _ = path;
}

/// Requests the page's WebGPU device for the keys built after it: their
/// MSMs and quotient go to the GPU through solve_parts_gpu. False without
/// WebGPU (the keys then prove on the CPU).
#[cfg(all(gpu_wgpu, target_arch = "wasm32"))]
#[wasm_bindgen]
pub async fn init_gpu() -> bool {
    match zopro_wgpu::Gpu::request().await {
        Ok(gpu) => {
            // The kernels compile from now, while the key downloads.
            gpu.warm_kernels();
            WEB_GPU.with(|device| *device.borrow_mut() = Some(std::sync::Arc::new(gpu)));
            true
        }
        Err(_) => false,
    }
}

/// The largest FFT domain an Android GPU takes (see `GpuBuilder::open`).
#[cfg(gpu_wgpu)]
const ANDROID_GPU_DOMAIN: usize = 1 << 20;

/// A GPU taking the key's vectors one at a time, so that the key keeps no
/// copy of what the GPU holds.
#[cfg(gpu_wgpu)]
struct GpuBuilder {
    started: crate::time::Instant,
    gpu: std::sync::Arc<zopro_wgpu::Gpu>,
    quotient: zopro_wgpu::Quotient,
    g1: [Option<zopro_wgpu::G1Msm>; 4],
    b2: Option<zopro_wgpu::G2Msm>,
    b2_cut: usize,
}
#[cfg(gpu_wgpu)]
impl GpuBuilder {
    /// None with ZOPRO_GPU=0 or without a GPU usable here.
    fn open(plan: &fft::Plan) -> Option<Self> {
        let wanted = std::env::var("ZOPRO_GPU").ok();
        if wanted.as_deref() == Some("0") {
            return None;
        }
        // A phone's GPU throttles within a big proof: a OnePlus 13 ran a
        // 54 → 1 merge (a 2^21 domain) in 8.5 s on its Adreno, every pass
        // of Z's MSM four to five times slower than B's minutes earlier,
        // and in 6.5 s on its cores; 24 → 2 (2^20) in 1.7 s against 3.2.
        // ZOPRO_GPU=1 takes the GPU for every size.
        if cfg!(target_os = "android")
            && plan.size() > ANDROID_GPU_DOMAIN
            && wanted.as_deref() != Some("1")
        {
            return None;
        }
        let started = crate::time::Instant::now();
        #[cfg(not(target_arch = "wasm32"))]
        let gpu = std::sync::Arc::new(zopro_wgpu::Gpu::new().ok()?);
        // On Android only an Adreno (zopro_wgpu::Gpu::request decides) and
        // only with the pre-compiled kernels (Vulkan with SPIR-V
        // passthrough): the WGSL kernels through naga lost an Adreno's
        // device. ZOPRO_GPU=1 tries them anyway.
        #[cfg(all(target_os = "android", not(target_arch = "wasm32")))]
        if !gpu.spirv && wanted.as_deref() != Some("1") {
            return None;
        }
        // In a browser the device is requested ahead, asynchronously
        // (init_gpu); building on it is synchronous.
        #[cfg(target_arch = "wasm32")]
        let gpu = WEB_GPU.with(|device| device.borrow().clone())?;
        let (roots, inverse_roots, forward, reverse) = plan.tables()?;
        let quotient = zopro_wgpu::Quotient::new(
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
        self.g1[which] = Some(zopro_wgpu::G1Msm::new(self.gpu.clone(), bases)?.0);
        Ok(())
    }
    /// How many of the G2 points (the first) the GPU took.
    fn g2(&mut self, bases: &[G2Affine]) -> Result<usize, String> {
        let cut = (bases.len() as f64 * g2_gpu_share(self.gpu.spirv)).round() as usize;
        if cut == 0 {
            return Ok(0);
        }
        self.b2 = Some(zopro_wgpu::G2Msm::new(self.gpu.clone(), &bases[..cut])?.0);
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
///
/// The CPU keeps points read in place from a host's image: they cost the
/// process nothing (the image stays mapped for the CPU's share of G2, and
/// the system may drop their pages), and they are what the CPU proves with
/// once the GPU is let go (`Key::drop_gpu`).
#[cfg(gpu_any)]
fn upload_g1(
    gpu: &mut Option<GpuBuilder>,
    which: usize,
    bases: Store<G1Affine>,
) -> Result<Store<G1Affine>, &'static str> {
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
            match bases {
                Store::Mapped { .. } => Ok(bases),
                Store::Owned(_) => Ok(Store::Owned(Vec::new())),
            }
        }
        None => Ok(bases),
    }
}
#[cfg(gpu_any)]
fn upload_g2(
    gpu: &mut Option<GpuBuilder>,
    bases: Store<G2Affine>,
) -> Result<Store<G2Affine>, &'static str> {
    #[cfg(feature = "fullcheck")]
    selftest::full_g2(&bases);
    match gpu {
        Some(builder) => {
            // The CPU keeps the points after those the GPU took; all of
            // them in an image (see upload_g1) and in a browser, for
            // solve_parts on the CPU.
            let cut = builder
                .g2(&bases)
                .map_err(|_| "the GPU could not take the key")?;
            match bases {
                Store::Owned(points) if cut > 0 && !cfg!(target_arch = "wasm32") => {
                    Ok(Store::Owned(points[cut..].to_vec()))
                }
                bases => Ok(bases),
            }
        }
        None => Ok(bases),
    }
}
#[cfg(all(gpu_any, not(target_arch = "wasm32")))]
fn gpu_trace(name: &str, sort_ms: f64, gpu_ms: f64) {
    #[cfg(feature = "trace")]
    log::info!("ZOPRO_TRACE gpu-msm-{name} sort {sort_ms:.1} gpu {gpu_ms:.1} ms");
    #[cfg(not(feature = "trace"))]
    let _ = (name, sort_ms, gpu_ms);
}

/// A GPU taking the key's vectors one at a time, so that the key keeps no
/// copy of what the GPU holds: a phone has no room for two.
#[cfg(gpu)]
struct GpuBuilder {
    started: crate::time::Instant,
    quotient: zopro_metal::Quotient,
    g1: [Option<zopro_metal::G1Msm>; 4],
    b2: Option<zopro_metal::G2Msm>,
    b2_cut: usize,
    /// The trees and buckets all the key's MSMs share.
    scratch: std::sync::Arc<zopro_metal::Scratch>,
}
#[cfg(gpu)]
impl GpuBuilder {
    /// None with ZOPRO_GPU=0 or without a GPU.
    fn open(plan: &fft::Plan) -> Option<Self> {
        if std::env::var("ZOPRO_GPU").is_ok_and(|v| v == "0") {
            return None;
        }
        let started = crate::time::Instant::now();
        let (roots, inverse_roots, forward, reverse) = plan.compact_tables()?;
        fn weights(w: &fft::WeightTables) -> zopro_metal::Weights<'_> {
            zopro_metal::Weights {
                lows: &w.lows,
                chunks: &w.chunks,
                low_bits: w.low_bits,
            }
        }
        let quotient = zopro_metal::Quotient::new(
            plan.size(),
            &roots,
            &inverse_roots,
            weights(&forward),
            weights(&reverse),
        )
        .ok()?
        .0;
        let scratch = zopro_metal::Scratch::new().ok()?;
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
        self.g1[which] = Some(zopro_metal::G1Msm::new_in(bases, self.scratch.clone())?.0);
        Ok(())
    }
    /// Whether the GPU took G2: a Mac's does, a phone's leaves it to the CPU.
    /// How many of the G2 points (the first) the GPU took.
    fn g2(&mut self, bases: &[G2Affine]) -> Result<usize, String> {
        let cut = (bases.len() as f64 * g2_gpu_share()).round() as usize;
        if cut == 0 {
            return Ok(0);
        }
        self.b2 = Some(zopro_metal::G2Msm::new_in(bases, self.scratch.clone())?.0);
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
            "ZOPRO_TRACE gpu-allocated {} MB",
            zopro_metal::allocated_bytes() >> 20
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

/// A key's five point vectors, A, B, K, Z (G1) and B2 (G2), as a host
/// has them. The key decodes one vector at a time, so a load holds the
/// host's bytes and the points decoded so far, not a second encoding.
#[derive(Clone, Copy)]
pub enum Points<'a> {
    /// The kernel's encoding: each coordinate's Montgomery limbs, little
    /// endian, 64 bytes a G1 point and 128 a G2 point.
    Kernel([&'a [u8]; 5]),
    /// gnark's compressed encoding, as a key file stores them (32 and 64
    /// bytes): decompressed into points with nothing between.
    Compressed([&'a [u8]; 5]),
    /// The kernel's encoding inside a host's image of the key (its raw
    /// form, mapped), at these byte offsets and lengths: on a little-endian
    /// device that is the points' memory layout, so the key reads them in
    /// place and the system may drop their pages under memory pressure, to
    /// read them again, instead of the process holding them.
    Mapped(&'a Image, [(usize, usize); 5]),
}

/// A host's image of a key's points (`Points::Mapped`): a mapped file.
pub type Image = std::sync::Arc<dyn AsRef<[u8]> + Send + Sync>;

/// A key's points: decoded into the process's memory, or read in place
/// from a host's image.
pub(crate) enum Store<T> {
    Owned(Vec<T>),
    Mapped {
        image: Image,
        offset: usize,
        len: usize,
        points: std::marker::PhantomData<T>,
    },
}
/// The points an image may hold in place: their memory is the kernel's
/// encoding on a little-endian device.
pub(crate) trait InPlace: Sized {
    const SIZE: usize;
}
impl InPlace for G1Affine {
    const SIZE: usize = 64;
}
impl InPlace for G2Affine {
    const SIZE: usize = 128;
}
const _: () = assert!(std::mem::size_of::<G1Affine>() == 64 && std::mem::size_of::<G2Affine>() == 128);
impl<T: InPlace> Store<T> {
    /// `bytes` bytes at `offset` in `image`, when they lie in it and are
    /// aligned for the points.
    fn mapped(image: &Image, offset: usize, bytes: usize) -> Result<Self, &'static str> {
        let data = (**image).as_ref();
        let in_image = offset
            .checked_add(bytes)
            .is_some_and(|end| end <= data.len());
        if !cfg!(target_endian = "little")
            || !in_image
            || !bytes.is_multiple_of(T::SIZE)
            || !(data.as_ptr() as usize + offset).is_multiple_of(std::mem::align_of::<T>())
        {
            return Err("a key image that does not fit its points");
        }
        Ok(Store::Mapped {
            image: image.clone(),
            offset,
            len: bytes / T::SIZE,
            points: std::marker::PhantomData,
        })
    }
}
impl<T> std::ops::Deref for Store<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        match self {
            Store::Owned(v) => v,
            Store::Mapped {
                image, offset, len, ..
            } => {
                let data = &(**image).as_ref()[*offset..];
                // SAFETY: `mapped` checked that the bytes lie in the image
                // and are aligned, and T is a point whose memory is the
                // kernel's encoding (InPlace); the image is immutable and
                // the store keeps it alive.
                unsafe { std::slice::from_raw_parts(data.as_ptr().cast::<T>(), *len) }
            }
        }
    }
}
impl Points<'_> {
    /// The vectors' lengths in points.
    fn counts(&self) -> Result<[usize; 5], &'static str> {
        let (lengths, g1, g2) = match self {
            Points::Kernel(v) => (v.map(<[u8]>::len), 64, 128),
            Points::Compressed(v) => (v.map(<[u8]>::len), 32, 64),
            Points::Mapped(_, ranges) => (ranges.map(|(_, len)| len), 64, 128),
        };
        let mut counts = [0; 5];
        for (i, len) in lengths.into_iter().enumerate() {
            let size = if i == 4 { g2 } else { g1 };
            if !len.is_multiple_of(size) {
                return Err("invalid gnark key dimensions");
            }
            counts[i] = len / size;
        }
        Ok(counts)
    }
    fn g1(&self, i: usize) -> Result<Store<G1Affine>, &'static str> {
        match self {
            Points::Kernel(v) => Ok(Store::Owned(g1s(v[i]))),
            Points::Compressed(v) => decompress::g1_affine(v[i]).map(Store::Owned),
            Points::Mapped(image, ranges) => Store::mapped(image, ranges[i].0, ranges[i].1),
        }
    }
    fn g2(&self) -> Result<Store<G2Affine>, &'static str> {
        match self {
            Points::Kernel(v) => Ok(Store::Owned(g2s(v[4]))),
            Points::Compressed(v) => decompress::g2_affine(v[4]).map(Store::Owned),
            Points::Mapped(image, ranges) => Store::mapped(image, ranges[4].0, ranges[4].1),
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
    /// when a GPU holds the only copy of them (points decoded natively from
    /// a host's bytes).
    lengths: [usize; 3],
    a: Store<G1Affine>,
    b: Store<G1Affine>,
    k: Store<G1Affine>,
    z: Store<G1Affine>,
    /// All of G2, or the points after those a GPU took when it holds the
    /// only copy of them (`cpu_b2`). Shared, for a browser's CPU share of
    /// the G2 MSM on the pool while the proof's future awaits the GPU.
    b2: std::sync::Arc<Store<G2Affine>>,
    /// The FFT domain's size.
    domain: usize,
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
        Self::build(Points::Kernel([a, b, k, z, b2]), domain_params).map_err(JsError::new)
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
        let started = crate::time::Instant::now();
        let parsed = solver::Plan::new(program, coefficients);
        trace("solver-plan", started);
        self.set_solver_plan(parsed)
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
            log::info!("ZOPRO_TRACE solver-plan-size {}", plan.footprint());
        }
        if let Ok(plan) = parsed {
            if plan.matches(
                self.lengths[0],
                self.lengths[1],
                self.lengths[2],
                self.domain,
            ) {
                let started = crate::time::Instant::now();
                plan.prepare();
                trace("solver-schedule", started);
                self.solver = Some(plan);
                return true;
            }
        }
        false
    }
    pub fn solve_parts(&self, witness: &[u8]) -> Result<Vec<u8>, JsError> {
        self.solve(witness).map_err(|error| JsError::new(&error))
    }
    /// Proves on the CPU from now on: a host whose GPU failed, or whose GPU
    /// proof did not verify, lets the GPU go. The CPU proves with the
    /// points in the key's image (`Points::Mapped`), or in a browser with
    /// its own copy. Natively, points decoded from a host's bytes have
    /// their only copy on the GPU: the key keeps its GPU and fails, and
    /// `drop_gpu_with` takes the points again.
    pub fn drop_gpu(&mut self) -> Result<(), &'static str> {
        #[cfg(gpu_any)]
        if self.gpu.is_some() {
            if !self.cpu_points() {
                return Err("the key's points are on the GPU alone (drop_gpu_with)");
            }
            self.gpu = None;
            self.plan.restore_tables();
        }
        Ok(())
    }
    /// Whether the key holds a GPU (in a browser, the page's: solve_parts_gpu
    /// proves with it).
    pub fn has_gpu(&self) -> bool {
        #[cfg(gpu_any)]
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
        // CPU stages into the GPU profile when profiling (gpu_profile).
        let mut clock = crate::time::Instant::now();
        let mut stage = |name: &'static str| {
            zopro_wgpu::profile::record_cpu(name, clock.elapsed().as_secs_f64() * 1000.0);
            clock = crate::time::Instant::now();
        };
        let solution = plan.solve_in(&frs(witness), self.domain).map_err(|e| JsError::new(&e))?;
        let sa: Vec<_> = plan.a_indices.iter().map(|&i| solution.w[i as usize]).collect();
        let sb: Vec<_> = plan.b_indices.iter().map(|&i| solution.w[i as usize]).collect();
        let sk: Vec<Fr> = solution.w[plan.n_public..].to_vec();
        let cut = if gpu.b2.is_some() { gpu.b2_cut } else { 0 };
        stage("cpu solve");
        let (a, b, c) = (solution.a, solution.b, solution.c);
        let h_from_cpu = if self.gpu_quotient {
            gpu.quotient.lock().unwrap().start(&a, &b, &c);
            None
        } else {
            Some(self.quotient(a, b, c))
        };
        stage("cpu quotient");
        // The GPU counts the scalars of the MSMs it sorts behind the
        // quotient, Z's from the quotient's output without a read back;
        // the host plans each MSM from its counts as they come.
        let sb2 = sb[..cut].to_vec();
        for (msm, scalars) in [(&gpu.a, &sa), (&gpu.b, &sb), (&gpu.k, &sk)] {
            let mut msm = msm.lock().unwrap();
            if msm.sorts_on_gpu() {
                msm.count(scalars);
            }
        }
        if let Some(b2) = &gpu.b2 {
            let mut b2 = b2.lock().unwrap();
            if b2.sorts_on_gpu() {
                b2.count(&sb2);
            }
        }
        let z_counted = h_from_cpu.is_none() && {
            let mut z = gpu.z.lock().unwrap();
            z.sorts_on_gpu() && {
                z.count_from(gpu.quotient.lock().unwrap().output());
                true
            }
        };
        // The MSMs the host sorts start now, before the CPU's G2 share takes
        // the pool.
        let host_start = |msm: &std::sync::Mutex<zopro_wgpu::G1Msm>, scalars: &[Fr]| {
            let mut msm = msm.lock().unwrap();
            (!msm.sorts_on_gpu()).then(|| msm.start(scalars))
        };
        let (ha, hb, hk) = (
            host_start(&gpu.a, &sa),
            host_start(&gpu.b, &sb),
            host_start(&gpu.k, &sk),
        );
        let hb2 = gpu.b2.as_ref().and_then(|b2| {
            let mut b2 = b2.lock().unwrap();
            (!b2.sorts_on_gpu()).then(|| b2.start(&sb2))
        });
        stage("cpu counts submit, host sorts");
        // The CPU's share of G2: on the pool while this thread awaits the
        // GPU, or, in a build without shared memory, where the pool is this
        // thread and never runs a spawned job, on this thread once the GPU
        // has every MSM.
        let cpu_b2 = (cut < sb.len()).then(|| {
            let (bases, scalars) = (self.b2.clone(), sb[cut..].to_vec());
            move || {
                let started = crate::time::Instant::now();
                let sum = msm_g2(&bases[cut..], &scalars);
                zopro_wgpu::profile::record_cpu(
                    "cpu g2 share",
                    started.elapsed().as_secs_f64() * 1000.0,
                );
                sum
            }
        });
        #[cfg(target_feature = "atomics")]
        let cpu_b2 = cpu_b2.map(|share| {
            let (tx, rx) = futures_channel::oneshot::channel();
            rayon::spawn(move || {
                let _ = tx.send(share());
            });
            rx
        });
        Ok(wasm_bindgen_futures::future_to_promise(async move {
            let mut clock = crate::time::Instant::now();
            let mut stage = |name: &'static str| {
                zopro_wgpu::profile::record_cpu(name, clock.elapsed().as_secs_f64() * 1000.0);
                clock = crate::time::Instant::now();
            };
            let fail = |e: String| JsValue::from(JsError::new(&e));
            // The GPU-sorted MSMs start as their counts come back.
            async fn begin(
                msm: &std::sync::Mutex<zopro_wgpu::G1Msm>,
                started: Option<zopro_wgpu::Pending>,
            ) -> zopro_wgpu::Pending {
                match started {
                    Some(pending) => pending,
                    None => msm.lock().unwrap().start_counted_async().await,
                }
            }
            let pa = begin(&gpu.a, ha).await;
            let pb = begin(&gpu.b, hb).await;
            let pk = begin(&gpu.k, hk).await;
            let pb2 = match (&gpu.b2, hb2) {
                (Some(_), Some(pending)) => Some(pending),
                (Some(b2), None) => Some(b2.lock().unwrap().start_counted_async().await),
                (None, _) => None,
            };
            stage("plan a b k b2");
            let pz = if z_counted {
                gpu.z.lock().unwrap().start_counted_async().await
            } else {
                let h = match h_from_cpu {
                    Some(h) => h,
                    None => {
                        let mut quotient = gpu.quotient.lock().unwrap();
                        quotient.finish_async().await.map_err(fail)?
                    }
                };
                gpu.z.lock().unwrap().start(&h)
            };
            stage("plan z");
            #[cfg(not(target_feature = "atomics"))]
            let cpu_b2 = cpu_b2.map(|share| share());
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
            stage("wait msms");
            #[cfg(target_feature = "atomics")]
            let cpu_b2 = match cpu_b2 {
                Some(rx) => Some(rx.await.map_err(|_| fail("the CPU's G2 share".into()))?),
                None => None,
            };
            stage("wait cpu g2");
            let bs2 = match (gpu_b2, cpu_b2) {
                (Some(g), Some(c)) => g + c,
                (Some(g), None) => g,
                (None, Some(c)) => c,
                (None, None) => G2Projective::IDENTITY,
            };
            // A test build's fault (ZOPRO_WEB_FAULT) checks that the host
            // catches a GPU that miscomputes.
            let ar = if option_env!("ZOPRO_WEB_FAULT").is_some() {
                ar + G1Projective::generator()
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
        let solver::Solution { mut w, a, b, c } = plan.solve_in(&frs(witness), self.domain)?;
        trace("solve", started);
        let mut sa: Vec<_> = plan.a_indices.iter().map(|&i| w[i as usize]).collect();
        let mut sb: Vec<_> = plan.b_indices.iter().map(|&i| w[i as usize]).collect();
        self.compute_parts(&mut sa, &mut sb, &mut w[plan.n_public..], a, b, c)
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
            || a.len() / 32 > self.domain
        {
            return Err(JsError::new("witness does not match prepared gnark key"));
        }
        let (mut sa, mut sb, mut sk) = (frs(sa), frs(sb), frs(sk));
        self.compute_parts(&mut sa, &mut sb, &mut sk, frs(a), frs(b), frs(c))
            .map_err(|e| JsError::new(&e))
    }
}
impl Key {
    /// A built solver plan, when it matches the key's dimensions.
    pub(crate) fn set_solver_plan(&mut self, parsed: Result<solver::Plan, &'static str>) -> bool {
        self.solver = None;
        if !self.commitments.is_empty() {
            return false;
        }
        let Ok(plan) = parsed else { return false };
        if !plan.matches(self.lengths[0], self.lengths[1], self.lengths[2], self.domain) {
            return false;
        }
        let started = crate::time::Instant::now();
        plan.prepare();
        trace("solver-schedule", started);
        self.solver = Some(plan);
        true
    }
    /// `new` for hosts outside JavaScript, from either encoding of the
    /// points, with a plain error.
    pub fn build(points: Points, domain_params: &[u8]) -> Result<Key, &'static str> {
        let [a, b, k, z, b2] = points.counts()?;
        if b != b2 || domain_params.len() != 64 {
            return Err("invalid gnark key dimensions");
        }
        let n = z + 1;
        if !n.is_power_of_two() {
            return Err("invalid gnark FFT domain");
        }
        let params = frs(domain_params);
        if n.trailing_zeros() > Fr::TWO_ADICITY {
            return Err("unsupported FFT domain");
        }
        if params[0].is_zero() {
            return Err("invalid FFT generator");
        }
        if params[1].is_zero() {
            return Err("invalid FFT coset");
        }
        let domain = n;
        let den = (params[1].pow(&[n as u64]) - Fr::ONE)
            .inverse()
            .ok_or("invalid quotient denominator")?;
        let started = crate::time::Instant::now();
        let plan = fft::Plan::new(n, params[0], params[1], den);
        trace("plan", started);
        let lengths = [a, b, k];
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
            let a = points.g1(0)?;
            let a_head = a[..head(a.len())].to_vec();
            let started = trace("decode-a", started);
            let a = upload_g1(gpu, 0, a)?;
            let started = trace("upload-a", started);
            let b = upload_g1(gpu, 1, points.g1(1)?)?;
            let started = trace("decode+upload-b", started);
            let k = upload_g1(gpu, 2, points.g1(2)?)?;
            let started = trace("decode+upload-k", started);
            let z = upload_g1(gpu, 3, points.g1(3)?)?;
            let started = trace("decode+upload-z", started);
            let b2 = points.g2()?;
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
                            if zopro_metal::prefer_smaller_code() {
                                trace("gpu-retry-smaller-code", crate::time::Instant::now());
                                gpu = GpuBuilder::open(&plan);
                            }
                            continue;
                        }
                        gpu_quotient = verdict.quotient
                            && !std::env::var("ZOPRO_GPU_QUOTIENT").is_ok_and(|v| v == "0");
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
            b2: std::sync::Arc::new(b2),
            domain,
            plan,
            commitments: Vec::new(),
            solver: None,
        })
    }
    /// `drop_gpu` for a key whose points are on the GPU alone: the CPU
    /// takes them from `points`, those the key was built from
    /// (`Artifact::points` of its key file, or a host's cache of them),
    /// decoded before the GPU goes. On an error the key is as it was.
    pub fn drop_gpu_with(&mut self, points: Points) -> Result<(), &'static str> {
        #[cfg(gpu_any)]
        if self.gpu.is_some() && !self.cpu_points() {
            let [a, b, k] = self.lengths;
            if points.counts()? != [a, b, k, self.domain - 1, b] {
                return Err("the points of another key");
            }
            let (a, b, k, z, b2) = (
                points.g1(0)?,
                points.g1(1)?,
                points.g1(2)?,
                points.g1(3)?,
                points.g2()?,
            );
            (self.a, self.b, self.k, self.z) = (a, b, k, z);
            self.b2 = std::sync::Arc::new(b2);
        }
        #[cfg(not(gpu_any))]
        let _ = points;
        self.drop_gpu()
    }
    /// The A, B and K MSM scalars of a witness, for the bench's profile of
    /// how skewed they are.
    #[doc(hidden)]
    pub fn msm_scalars(&self, witness: &[u8]) -> Result<[Vec<Fr>; 3], String> {
        let plan = self.solver.as_ref().ok_or("no prepared solver")?;
        let solution = plan.solve_in(&frs(witness), self.domain)?;
        Ok([
            plan.a_indices.iter().map(|&i| solution.w[i as usize]).collect(),
            plan.b_indices.iter().map(|&i| solution.w[i as usize]).collect(),
            solution.w[plan.n_public..].to_vec(),
        ])
    }
    /// The proof's parts from the solve's MSM scalars, which the CPU's MSMs
    /// take over (natively, as canonical limbs in place), and its a, b, c.
    fn compute_parts(
        &self,
        sa: &mut [Fr],
        sb: &mut [Fr],
        sk: &mut [Fr],
        a: Vec<Fr>,
        b: Vec<Fr>,
        c: Vec<Fr>,
    ) -> Result<Vec<u8>, String> {
        #[cfg(gpu)]
        if let Some(gpu) = &self.gpu {
            let (sa, sb, sk) = (&*sa, &*sb, &*sk);
            // The MSMs whose scalars are ready run on the GPU, driven from a
            // thread of their own so that their waits hold no rayon worker,
            // while the GPU also takes the quotient and the pool computes
            // G2 when it is not on the GPU; Z's MSM follows the quotient.
            // The sorts run on the pool, so a CPU G2 MSM starts only once
            // they are done: otherwise it starves them for its whole run.
            let (sorted_tx, sorted_rx) = std::sync::mpsc::channel::<()>();
            // The quotient has the GPU to itself: Z's sort and MSM wait for
            // it, the others for nothing, so they are submitted once it is
            // back (sharing the GPU with them it came back after 179 ms
            // rather than 45 on an M5 Pro, 54 → 1 merge).
            let started = crate::time::Instant::now();
            let mut quotient = self.gpu_quotient.then(|| gpu.quotient.lock().unwrap());
            let pending = quotient.as_mut().map(|q| q.start(&a, &b, &c));
            // The GPU has its copy of a, b and c: the CPU's go now rather
            // than after the MSMs (48 MB at 16 -> 2).
            let abc = if pending.is_some() {
                drop((a, b, c));
                None
            } else {
                Some((a, b, c))
            };
            let (quotient_tx, quotient_rx) = std::sync::mpsc::channel::<()>();
            // A quotient on the CPU holds nothing up.
            let quotient_tx = pending.is_some().then_some(quotient_tx);
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
                    let (sorted_a, sorted_b, sorted_k) =
                        (a.sort_now(sa), b.sort_now(sb), k.sort_now(sk));
                    drop(sorted_tx);
                    let _ = quotient_rx.recv();
                    let pa = a.submit_sorted(sorted_a);
                    let pb = b.submit_sorted(sorted_b);
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
                    let pk = k.submit_sorted(sorted_k);
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
                        let bs2 = msm_g2(self.cpu_b2(cut), &sb[cut..]);
                        trace("cpu-msm-b2", started);
                        bs2
                    })
                });
                let h = match (quotient.as_mut(), pending, abc) {
                    (Some(q), Some(pending), _) => q.finish(pending)?,
                    (_, _, Some((a, b, c))) => self.quotient(a, b, c),
                    _ => unreachable!("the quotient on the GPU or on the CPU"),
                };
                drop((quotient, quotient_tx));
                trace("quotient", started);
                // Z's MSM is sorted and queued now, behind the others still
                // on the GPU, rather than after them; on a pool of its own,
                // since on the global one it queued behind the CPU's G2 MSM
                // (365 ms instead of 10 on an iPad Pro's A12Z at 8x2).
                let mut z = gpu.z.lock().unwrap();
                let msm_z = &mut *z;
                let pz = sort_pool().install(|| msm_z.start(&h));
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
                        "ZOPRO_TRACE gpu-profile {name} {}",
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
            let (sa, sb, sk) = (&*sa, &*sb, &*sk);
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
                        let (mut ma, mut mb, mut mk, mut mz) = (
                            gpu.a.lock().unwrap(),
                            gpu.b.lock().unwrap(),
                            gpu.k.lock().unwrap(),
                            gpu.z.lock().unwrap(),
                        );
                        // An MSM the GPU sorts is counted first, ahead of
                        // all else on the GPU's queue. When every G1 MSM is,
                        // the CPU's G2 MSM waits for no sort; when Z's is
                        // and the quotient is on the GPU too, Z's scalars
                        // never leave it: the GPU counts them from the
                        // quotient's output, ahead of the other MSMs.
                        for (msm, scalars) in [(&mut ma, sa), (&mut mb, sb), (&mut mk, sk)] {
                            if msm.sorts_on_gpu() {
                                msm.count(scalars);
                            }
                        }
                        let host_sorts =
                            !(ma.sorts_on_gpu() && mb.sorts_on_gpu() && mk.sorts_on_gpu());
                        let mut quotient = self.gpu_quotient.then(|| gpu.quotient.lock().unwrap());
                        let z_on_gpu = mz.sorts_on_gpu() && quotient.is_some();
                        if let Some(q) = quotient.as_mut() {
                            q.start(&a, &b, &c);
                            if z_on_gpu {
                                mz.count_from(q.output());
                            }
                        }
                        let mut sorted_tx = Some(sorted_tx);
                        if !host_sorts {
                            sorted_tx = None;
                        }
                        let (pa, pb, pk) = (ma.start(sa), mb.start(sb), mk.start(sk));
                        let mut b2 = gpu.b2.as_ref().map(|b2| b2.lock().unwrap());
                        let pb2 = b2.as_mut().map(|b2| b2.start(&sb[..cut]));
                        drop(sorted_tx);
                        let pz = if z_on_gpu {
                            mz.start_counted()
                        } else {
                            let h = match quotient.as_mut() {
                                Some(quotient) => quotient.finish()?,
                                None => self.quotient(a, b, c),
                            };
                            trace("quotient", started);
                            // By now the pool runs the CPU's G2 MSM, whose
                            // tasks a host sort of Z would queue behind (550
                            // ms on a OnePlus 13): it gets threads of its own,
                            // which the GPU, busy with A, B and K, does not
                            // wait for.
                            let z = &mut *mz;
                            if z.sorts_on_gpu() {
                                z.start(&h)
                            } else {
                                sort_pool().install(|| z.start(&h))
                            }
                        };
                        drop(quotient);
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
                    let cpu_b2 = (cut < sb.len()).then(|| msm_g2(self.cpu_b2(cut), &sb[cut..]));
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
        let mut h = self.quotient(a, b, c);
        let started = trace("quotient", started);
        let ((ar, bs1), (kr, (hz, bs2))) = self.cpu_msms(sa, sb, sk, &mut h);
        trace("msms", started);
        // Each MSM alone, on the whole pool, for its own share: this adds
        // to the proof time, so only with ZOPRO_TRACE_MSMS=1.
        #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
        if std::env::var("ZOPRO_TRACE_MSMS").is_ok_and(|v| v == "1") {
            for (label, bases, scalars) in [
                ("msm-a", &self.a, &*sa),
                ("msm-b", &self.b, &*sb),
                ("msm-k", &self.k, &*sk),
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
    /// Whether the CPU holds every point, as it must to prove without a
    /// GPU.
    #[cfg(gpu_any)]
    fn cpu_points(&self) -> bool {
        let [a, b, k] = self.lengths;
        self.a.len() == a
            && self.b.len() == b
            && self.k.len() == k
            && self.z.len() == self.domain - 1
            && self.b2.len() == b
    }
    /// The G2 points the CPU takes when a GPU took the first `cut`: the
    /// last of those it keeps, all or those after the GPU's.
    #[cfg(all(gpu_any, not(target_arch = "wasm32")))]
    fn cpu_b2(&self, cut: usize) -> &[G2Affine] {
        &self.b2[self.b2.len() + cut - self.lengths[1]..]
    }
    /// The CPU's five MSMs at once. Natively the batch-affine ones take the
    /// scalars as canonical limbs, converted in place: a proof holds no
    /// copy of them and no digits (63 MB at 16 -> 2).
    #[allow(clippy::type_complexity)]
    fn cpu_msms(
        &self,
        sa: &mut [Fr],
        sb: &mut [Fr],
        sk: &mut [Fr],
        h: &mut [Fr],
    ) -> (
        (G1Projective, G1Projective),
        (G1Projective, (G1Projective, G2Projective)),
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        if affine_msm() {
            use msm_affine::{canonical, msm_canonical as msm};
            let (sa, sb, sk, h) = (canonical(sa), canonical(sb), canonical(sk), canonical(h));
            return rayon::join(
                || rayon::join(|| msm(&self.a[..], sa), || msm(&self.b[..], sb)),
                || {
                    rayon::join(
                        || msm(&self.k[..], sk),
                        || rayon::join(|| msm(&self.z[..], h), || msm(&self.b2[..], sb)),
                    )
                },
            );
        }
        rayon::join(
            || rayon::join(|| msm_g1(&self.a, sa), || msm_g1(&self.b, sb)),
            || {
                rayon::join(
                    || msm_g1(&self.k, sk),
                    || rayon::join(|| msm_g1(&self.z, h), || msm_g2(&self.b2, sb)),
                )
            },
        )
    }
}

pub fn msm_g1(bases: &[G1Affine], scalars: &[Fr]) -> G1Projective {
    if bases.is_empty() {
        G1Projective::IDENTITY
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
        G2Projective::IDENTITY
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
/// 988, an M5 Pro the same), unless ZOPRO_MSM=lanes; WebAssembly the lane
/// form, which its SIMD lanes suit.
fn affine_msm() -> bool {
    !cfg!(target_arch = "wasm32") && !std::env::var("ZOPRO_MSM").is_ok_and(|v| v == "lanes")
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
    use crate::testing::Seeded as StdRng;

    fn limbs(out: &mut Vec<u8>, x: &Fq) {
        for word in x.0 {
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
            .flat_map(|v| v.0.iter().flat_map(|w| w.to_le_bytes()))
            .collect()
    }
    fn g1_points(n: usize, rng: &mut StdRng) -> Vec<G1Affine> {
        let g = G1Projective::generator();
        let (mut p, step) = (g.mul(&rng.fr()), g.mul(&rng.fr()));
        let all: Vec<_> = (0..n)
            .map(|_| {
                p += step;
                p
            })
            .collect();
        G1Projective::batch_to_affine(&all)
    }
    fn g2_points(n: usize, rng: &mut StdRng) -> Vec<G2Affine> {
        let g = G2Projective::generator();
        let (mut p, step) = (g.mul(&rng.fr()), g.mul(&rng.fr()));
        let all: Vec<_> = (0..n)
            .map(|_| {
                p += step;
                p
            })
            .collect();
        G2Projective::batch_to_affine(&all)
    }

    /// A key on the GPU proves the parts a key on the CPU proves: the MSMs
    /// (G2 split between the two where the defaults say), the quotient, and
    /// the window width and ordering of the GPU path.
    #[test]
    fn gpu_parts_match_cpu() {
        let mut rng = StdRng::new(11);
        let (n, domain) = (3000, 4096);
        let (a, b, k) = (
            g1_points(n, &mut rng),
            g1_points(n, &mut rng),
            g1_points(n, &mut rng),
        );
        let z = g1_points(domain - 1, &mut rng);
        let b2 = g2_points(n, &mut rng);
        let generator = Fr::root_of_unity(domain).unwrap();
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
                Points::Kernel([&bytes[0], &bytes[1], &bytes[2], &bytes[3], &bytes[4]]),
                &params,
            )
            .expect("key")
        };
        // SAFETY: no other test reads ZOPRO_GPU.
        unsafe { std::env::set_var("ZOPRO_GPU", "0") };
        let cpu = build();
        unsafe { std::env::remove_var("ZOPRO_GPU") };
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
                                rng.fr()
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
