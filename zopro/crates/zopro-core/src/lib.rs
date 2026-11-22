#![cfg_attr(
    all(target_arch = "wasm32", target_feature = "simd128"),
    feature(asm_experimental_arch)
)]
use wasm_bindgen::prelude::*;
use zopro_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

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
#[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
mod lagrange;
mod msm;
#[cfg(not(target_arch = "wasm32"))]
mod msm_affine;
#[cfg(all(
    zopro_g2_horner_buckets,
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
use msm_affine::{g2_query_grouped as msm_grouped_b2, g2_query_raw as msm_raw_b2};
#[cfg(all(
    not(target_arch = "wasm32"),
    not(all(
        zopro_g2_horner_buckets,
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        feature = "ifma8",
        not(no_ifma)
    ))
))]
use msm_affine::{msm_grouped as msm_grouped_b2, msm_raw as msm_raw_b2};
mod preparation;
#[cfg(not(target_arch = "wasm32"))]
mod selftest;
mod simd;
mod solver;
#[cfg(test)]
mod testing;
pub mod verify;

/// Initialize fresh field storage in parallel, including first page touches.
/// Capacity beyond `len` remains uninitialized for the quotient's padding.
pub(crate) fn zero_frs(len: usize, capacity: usize) -> Vec<Fr> {
    let mut values = Vec::with_capacity(capacity.max(len));
    if len < 16384 || !crate::simd::cpu_option("ZOPRO_CPU_PARALLEL_ALLOC") {
        values.resize(len, Fr::ZERO);
        return values;
    }
    use rayon::prelude::*;
    values.spare_capacity_mut()[..len]
        .par_chunks_mut(1024)
        .for_each(|chunk| {
            for value in chunk {
                value.write(Fr::ZERO);
            }
        });
    // SAFETY: every element below len was initialized by the completed tasks.
    unsafe { values.set_len(len) };
    values
}

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
    format!(
        " fp {} peak {} MB",
        kb("RssAnon:") >> 10,
        kb("VmHWM:") >> 10
    )
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

#[cfg(all(
    zopro_resource_schedule,
    feature = "ifma8",
    target_arch = "x86_64",
    target_os = "linux",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
mod public_wire_offsets;

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
fn fast_ios_metal() -> bool {
    #[cfg(all(target_os = "ios", target_arch = "aarch64"))]
    {
        static FAST: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        *FAST.get_or_init(zopro_metal::fast_gpu)
    }
    #[cfg(not(all(target_os = "ios", target_arch = "aarch64")))]
    false
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
    derive_c: bool,
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

/// A separate pool for the final MSM's sort while the global pool evaluates
/// G2. On large Metal queries the GPU recodes the digits for a four-thread
/// scatter. The ordinary sort uses two threads. ZOPRO_SORT_THREADS overrides both.
#[cfg(all(any(gpu_wgpu, gpu), not(target_arch = "wasm32")))]
fn sort_pool(gpu_digits: bool) -> &'static rayon::ThreadPool {
    static POOL: std::sync::OnceLock<rayon::ThreadPool> = std::sync::OnceLock::new();
    static DIGITS_POOL: std::sync::OnceLock<rayon::ThreadPool> = std::sync::OnceLock::new();
    let (pool, default_threads) = if gpu_digits { (&DIGITS_POOL, 4) } else { (&POOL, 2) };
    pool.get_or_init(|| {
        let threads = std::env::var("ZOPRO_SORT_THREADS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default_threads);
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .thread_name(|i| format!("zopro-sort-{i}"))
            .build()
            .expect("sort pool")
    })
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const APPLE_SMALL_PAIRED_G1_WORKERS: usize = 11;

/// Pool sizes used by the overlapping CPU and Metal MSM schedules.
#[cfg(any(gpu, all(target_os = "macos", target_arch = "aarch64")))]
fn apple_msm_pool(threads: usize) -> Option<&'static rayon::ThreadPool> {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    static FIVE: std::sync::OnceLock<Option<rayon::ThreadPool>> = std::sync::OnceLock::new();
    static SIX: std::sync::OnceLock<Option<rayon::ThreadPool>> = std::sync::OnceLock::new();
    static TWELVE: std::sync::OnceLock<Option<rayon::ThreadPool>> = std::sync::OnceLock::new();
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    static SEVEN: std::sync::OnceLock<Option<rayon::ThreadPool>> = std::sync::OnceLock::new();
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    static ELEVEN: std::sync::OnceLock<Option<rayon::ThreadPool>> = std::sync::OnceLock::new();
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    static THIRTEEN: std::sync::OnceLock<Option<rayon::ThreadPool>> = std::sync::OnceLock::new();
    let pool = match threads {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        5 => &FIVE,
        6 => &SIX,
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        7 => &SEVEN,
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        11 => &ELEVEN,
        12 => &TWELVE,
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        13 => &THIRTEEN,
        _ => unreachable!("Apple MSM pool size"),
    };
    pool.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .thread_name(move |i| format!("zopro-msm-{threads}-{i}"))
            .build()
            .ok()
    })
    .as_ref()
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
    fn open(plan: &fft::Plan, _b2_points: usize) -> Option<Self> {
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
    domain: usize,
    b2_points: usize,
    b2: Option<zopro_metal::G2Msm>,
    b2_cut: usize,
    /// The trees and buckets all the key's MSMs share.
    scratch: std::sync::Arc<zopro_metal::Scratch>,
}
#[cfg(gpu)]
impl GpuBuilder {
    /// None with ZOPRO_GPU=0 or without a GPU.
    fn open(plan: &fft::Plan, b2_points: usize) -> Option<Self> {
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
            domain: plan.size(),
            b2_points,
            b2: None,
            b2_cut: 0,
            scratch,
        })
    }
    fn g1(&mut self, which: usize, bases: &[G1Affine]) -> Result<(), String> {
        let wide_apple = cfg!(all(target_os = "macos", target_arch = "aarch64"))
            && rayon::current_num_threads() >= 16;
        let short_witness = wide_apple
            && self.domain == (1 << 17)
            && self.b2_points < 90112
            && matches!(which, 0 | 2);
        let short_quotient_with_large_g2 = wide_apple
            && self.domain == (1 << 16)
            && (57344..65536).contains(&self.b2_points);
        let narrow_tree = wide_apple
            && self.domain == (1 << 16)
            && (32768..49152).contains(&self.b2_points)
            && matches!(which, 0 | 2 | 3);
        let shorter_million_point_query = wide_apple
            && self.domain == (1 << 20)
            && self.b2_points < 589824
            && which == 3;
        // B1 and B2 can reuse the same sorted runs and must share a width.
        // On wide Apple silicon hosts, fewer buckets make small G1 queries
        // cheaper to plan and combine on the CPU.
        let width = if self.domain <= (1 << 16) && matches!(which, 0 | 2 | 3) && fast_ios_metal() {
            // Smaller bucket tables reduce host work on short phone proofs.
            Some(8)
        } else if wide_apple {
            match (self.domain, which) {
                (32768, 0 | 2 | 3) => Some(9),
                (65536, 0 | 2) if short_quotient_with_large_g2 => Some(9),
                (65536, 0 | 2 | 3) => Some(8),
                // The dense quotient benefits from fewer passes over its points.
                (262144, 3) => Some(11),
                (131072, 0 | 2 | 3) | (262144, 0 | 2) => Some(10),
                _ => None,
            }
        } else {
            None
        };
        let scratch = self.scratch.clone();
        let (msm, _) = if narrow_tree || short_witness || (short_quotient_with_large_g2 && which == 3) {
            // Process more short-window groups in the shared tree buffers.
            zopro_metal::G1Msm::new_in_with_tree_budget(bases, scratch, 8, Some(16))?
        } else if shorter_million_point_query {
            zopro_metal::G1Msm::new_in_with_tree_budget(bases, scratch, 13, Some(32))?
        } else if let Some(cap) = width {
            zopro_metal::G1Msm::new_in_with_width_cap(bases, scratch, cap)?
        } else {
            zopro_metal::G1Msm::new_in(bases, scratch)?
        };
        self.g1[which] = Some(msm);
        Ok(())
    }
    /// Upload the GPU's prefix of the G2 query and return its length.
    fn g2(&mut self, bases: &[G2Affine]) -> Result<usize, String> {
        let cut = (bases.len() as f64 * g2_gpu_share()).round() as usize;
        if cut == 0 {
            return Ok(0);
        }
        self.b2 = Some(zopro_metal::G2Msm::new_in(&bases[..cut], self.scratch.clone())?.0);
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
            derive_c: self.quotient.can_derive_c(),
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
#[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
pub mod basis;
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
const _: () =
    assert!(std::mem::size_of::<G1Affine>() == 64 && std::mem::size_of::<G2Affine>() == 128);
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
impl<T: Clone> Store<T> {
    /// Copy only when an optional rewrite needs mutable storage.
    fn make_mut(&mut self) -> &mut [T] {
        if matches!(self, Self::Mapped { .. }) {
            *self = Self::Owned(self.to_vec());
        }
        let Self::Owned(values) = self else {
            unreachable!()
        };
        values
    }
}
impl<T> Store<T> {
    pub(crate) fn owned_bytes(&self) -> Option<usize> {
        match self {
            Self::Owned(v) => v.capacity().checked_mul(core::mem::size_of::<T>()),
            Self::Mapped { .. } => Some(0),
        }
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

    /// Validate a key's shape and FFT parameters without decoding its points.
    fn validate_shape(&self, domain_params: &[u8]) -> Result<([usize; 5], [Fr; 3]), &'static str> {
        let counts = self.counts()?;
        if counts[1] != counts[4] || domain_params.len() != 64 {
            return Err("invalid gnark key dimensions");
        }
        let domain = counts[3] + 1;
        if !domain.is_power_of_two() {
            return Err("invalid gnark FFT domain");
        }
        if domain.trailing_zeros() > Fr::TWO_ADICITY {
            return Err("unsupported FFT domain");
        }
        let params = frs(domain_params);
        let (generator, coset) = (params[0], params[1]);
        if generator.is_zero() {
            return Err("invalid FFT generator");
        }
        if coset.is_zero() {
            return Err("invalid FFT coset");
        }
        let denominator = fft::denominator(domain, coset).ok_or("invalid quotient denominator")?;
        Ok((counts, [generator, coset, denominator]))
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

/// A caller-owned Rayon pool for canonical witness and FFT preparation.
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub use rayon::ThreadPool as FrontendThreadPool;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn select_solver_pool<'a>(
    solver: Option<&'a FrontendThreadPool>,
    frontend: Option<&'a FrontendThreadPool>,
    use_frontend: impl FnOnce() -> bool,
) -> Option<&'a FrontendThreadPool> {
    let smaller =
        |pool: &&FrontendThreadPool| pool.current_num_threads() < rayon::current_num_threads();
    solver
        .filter(smaller)
        .or_else(|| frontend.filter(smaller).filter(|_| use_frontend()))
}

#[cfg(all(test, target_os = "linux", target_arch = "x86_64"))]
#[test]
fn explicit_solver_pool_takes_precedence_over_frontend() {
    let pool = |threads| rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
    let current = pool(4);
    let solver = pool(1);
    let frontend = pool(2);
    let selected = |solver, use_frontend| {
        current.install(|| {
            select_solver_pool(solver, Some(&frontend), || use_frontend)
                .map(FrontendThreadPool::current_num_threads)
        })
    };
    assert_eq!(selected(Some(&solver), false), Some(1));
    assert_eq!(selected(Some(&solver), true), Some(1));
    assert_eq!(selected(Some(&current), true), Some(2));
    assert_eq!(selected(Some(&current), false), None);
    assert_eq!(selected(None, false), None);
    current.install(|| {
        let selected = select_solver_pool(Some(&solver), Some(&frontend), || {
            panic!("explicit solver selection must skip the frontend flag")
        });
        assert_eq!(selected.map(FrontendThreadPool::current_num_threads), Some(1));
    });
}

#[cfg(all(
    zopro_resource_schedule,
    zopro_g2_horner_buckets,
    feature = "ifma8",
    target_os = "linux",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
struct ExclusiveG2Pool {
    workers: std::sync::Arc<FrontendThreadPool>,
    scratch_bytes: usize,
    overlap_quotient: bool,
}

#[wasm_bindgen]
pub struct Key {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    frontend_pool: Option<std::sync::Arc<FrontendThreadPool>>,
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    solver_pool: Option<std::sync::Arc<FrontendThreadPool>>,
    #[cfg(all(zopro_resource_schedule, zopro_g2_horner_buckets, feature = "ifma8", target_os = "linux", target_arch = "x86_64", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", not(no_ifma)))]
    exclusive_g2_pool: Option<ExclusiveG2Pool>,
    #[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
    grouped_point_tiling: bool,
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    grouped_g2_on_frontend: bool,
    exclusive_g2_grouping: bool,
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    raw_g1_a_on_frontend: bool,
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    g1_k_on_frontend: bool,
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    grouped_b2_serial_preparation: bool,
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    grouped_b2_poseidon_origins: bool,
    basis_only: bool,
    // Audited native workspace, or conservative prompt release when unknown.
    proof_workspace: Option<usize>,
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
    #[cfg(all(zopro_resource_schedule, feature="ifma8", target_arch="x86_64", target_os="linux", target_feature="avx512ifma", target_feature="avx512vl", target_feature="avx512dq", not(no_ifma)))]
    wire_offsets: Option<Box<public_wire_offsets::Offsets>>,
    #[cfg(all(target_arch = "x86_64", target_os = "linux", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
    canonical_wires: bool,
    /// The FFT domain's size.
    domain: usize,
    plan: fft::Plan,
    commitments: Vec<(Vec<G1Affine>, Vec<G1Affine>)>,
    solver: Option<solver::Plan>,
    // A replacement query needs its original C metadata even if solving is
    // disabled. Move the existing plan here on invalidation, never clone it.
    basis_fallback: Option<Box<solver::Plan>>,
    #[cfg(not(target_arch = "wasm32"))]
    equal_witness: Option<msm_affine::PreparedEqualWitness>,
    #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
    lagrange: Option<lagrange::Queries>,
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
        self.invalidate_solver();
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.equal_witness = None;
        }
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
        self.set_solver_image_with_owner(image, None)
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
        let solution = plan
            .solve_in(&frs(witness), self.domain)
            .map_err(|e| JsError::new(&e))?;
        let sa: Vec<_> = plan
            .a_indices
            .iter()
            .map(|&i| solution.w[i as usize])
            .collect();
        let sb: Vec<_> = plan
            .b_indices
            .iter()
            .map(|&i| solution.w[i as usize])
            .collect();
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
        self.solve_inner(witness, false)
    }
    fn solve_inner(&self, witness: &[u8], omit_b: bool) -> Result<Vec<u8>, String> {
        #[cfg(all(
            zopro_small_core_entry,
            zopro_real_kh,
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            feature = "ifma8",
            not(no_ifma)
        ))]
        if self.solver.is_some()
            && self.canonical_wires
            && !self.has_gpu()
            && affine_msm()
            && self.equal_witness.is_none()
            && rayon::current_thread_index().is_none()
            && crate::simd::cpu_option("ZOPRO_CPU_SMALL_CORE_ENTRY")
            && self.lagrange.as_ref().is_some_and(|q| {
                q.folded.as_ref().is_some_and(|f| {
                    msm_affine::fused_kh::eligible(
                        omit_b,
                        q.product_scale,
                        f.private.len(),
                        self.plan.size(),
                    )
                })
            })
        {
            // Reuse the existing worker pool; create the proof scope only once,
            // in the recursive worker call. Larger query paths retain their entry.
            if std::env::var_os("ZOPRO_SMALL_CORE_ENTRY_TRACE").is_some() {
                eprintln!("SMALL_CORE_ENTRY entered");
            }
            return rayon::join(
                || {
                    assert!(rayon::current_thread_index().is_some());
                    self.solve_inner(witness, omit_b)
                },
                || (),
            )
            .0;
        }
        let _reclaim = (!self.has_gpu()).then(|| alloc::ProofScope::enter(self.proof_workspace));
        let plan = self.solver.as_ref().ok_or("no prepared solver")?;
        if !witness.len().is_multiple_of(32) {
            return Err("invalid witness encoding".into());
        }
        let started = crate::time::Instant::now();
        let slots: &[usize] = &[];
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        let slots = self
            .lagrange
            .as_ref()
            .and_then(|q| q.folded.as_ref())
            .map_or(slots, |folded| &folded.slots);
        let materialize_c = true;
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        let materialize_c = materialize_c
            && !(self.lagrange.as_ref().is_some_and(|q| q.folded.is_some())
                && crate::simd::cpu_option("ZOPRO_CPU_FOLD_SKIP_C"));
        #[cfg(all(
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            feature = "ifma8",
            not(no_ifma)
        ))]
        if self.canonical_wires && !self.has_gpu() && affine_msm() && self.equal_witness.is_none() {
            if let Some(queries) = &self.lagrange {
                if let Some(folded) = &queries.folded {
                    return self.solve_canonical_wires(
                        plan,
                        &frs(witness),
                        materialize_c,
                        queries,
                        folded,
                        omit_b,
                        _reclaim.as_ref(),
                    );
                }
            }
        }
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        let instruction_pool = if self.has_gpu() {
            None
        } else {
            select_solver_pool(
                self.solver_pool.as_deref(),
                self.frontend_pool.as_deref(),
                || std::env::var_os("ZOPRO_CPU_SOLVER_FRONTEND").is_some_and(|value| value == "1"),
            )
        };
        #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
        let instruction_pool = None;
        #[cfg(gpu)]
        let derive_c = self.gpu_quotient && self.gpu.as_ref().is_some_and(|gpu| gpu.derive_c);
        #[cfg(not(gpu))]
        let derive_c = false;
        let solver::Solution { mut w, a, b, c } = plan.solve_with_pool(
            &frs(witness),
            self.domain,
            slots,
            materialize_c && !derive_c,
            instruction_pool,
        )?;
        if let Some(scope) = &_reclaim {
            scope.release_unused(&mut w);
        }
        let started = trace("solve", started);
        use rayon::prelude::*;
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            let paired = materialize_c
                && !self.has_gpu()
                && omit_b
                && self.apple_small_msm_overlap()
                && self.lagrange.as_ref().is_none_or(|queries| {
                    queries.folded.is_none() && queries.constraints.is_empty()
                })
                && self.plan.has_apple_small_quotient()
                && msm_affine::apple_fused_kh_enabled(self.k.len(), self.domain - 1)
                && msm_affine::apple_g2_three_parts_enabled()
                && std::env::var("ZOPRO_CPU_WITNESS_GROUP_PREP").as_deref() != Ok("0");
            if paired {
                // Give more of the pool to the quotient and G1 MSMs when
                // the G2 query can finish on fewer workers.
                let g1 = match self.domain {
                    65536 if self.a.len() < 24576 => 12,
                    131072 => match self.b2.len() {
                        0..65536 | 90112..122880 => 13,
                        65536..90112 => 12,
                        _ => APPLE_SMALL_PAIRED_G1_WORKERS,
                    },
                    _ => APPLE_SMALL_PAIRED_G1_WORKERS,
                };
                if let Some(pools) = apple_msm_pool(g1).zip(apple_msm_pool(18 - g1)) {
                    return Ok(self.apple_small_paired_cpu_parts(plan, &w, (a, b, c), pools));
                }
            }
        }
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        let offsets = self
            .wire_offsets
            .as_deref()
            .filter(|_| omit_b && !self.canonical_wires);
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if let Some(offsets) = offsets {
            offsets.apply(&mut w);
        }
        let (mut sa, mut sb): (Vec<_>, Vec<_>) = rayon::join(
            || plan.a_indices.par_iter().map(|&i| w[i as usize]).collect(),
            || plan.b_indices.par_iter().map(|&i| w[i as usize]).collect(),
        );
        trace("extract-scalars", started);
        let (public, private) = w.split_at_mut(plan.n_public);
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if let Some(offsets) = offsets {
            // The folded public term still needs the original public values.
            offsets.restore_public(public);
        }
        let c = (!derive_c).then_some(c);
        self.compute_parts(&mut sa, &mut sb, private, a, b, c, Some(public), omit_b)
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
        let _reclaim = (!self.has_gpu()).then(|| alloc::ProofScope::enter(self.proof_workspace));
        let (mut sa, mut sb, mut sk) = (frs(sa), frs(sb), frs(sk));
        self.compute_parts(
            &mut sa, &mut sb, &mut sk, frs(a), frs(b), Some(frs(c)), None, false,
        )
            .map_err(|e| JsError::new(&e))
    }
}
impl Key {
    pub(crate) fn set_solver_image_with_owner(
        &mut self,
        image: &[u8],
        owner: Option<&Image>,
    ) -> bool {
        let started = crate::time::Instant::now();
        self.invalidate_solver();
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.equal_witness = None;
        }
        if !self.commitments.is_empty() {
            return false;
        }
        let parsed = solver::Plan::from_image_with_owner(image, owner);
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
                if !self.basis_plan_matches(&plan) {
                    return false;
                }
                self.basis_fallback = None;
                self.prepare_plan(&plan);
                self.solver = Some(plan);
                return true;
            }
        }
        false
    }
    fn prepare_plan(&mut self, plan: &solver::Plan) {
        self.proof_workspace = None;
        #[cfg(zopro_live_preparation_charge)]
        {
            self.prepare_solver_queries(plan);
            #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
            if std::env::var("ZOPRO_CPU_LAGRANGE_FOLD").as_deref() == Ok("1")
                && self.lagrange.as_ref().is_some_and(|q| q.folded.is_none())
            {
                // Without folding, the extra constraint MSM costs more than
                // these optional queries save. Return their credit first.
                self.lagrange = None;
            }
        }
        let started = crate::time::Instant::now();
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            use msm_affine::workspace::{forecast, Geometry};
            let mut k = self.k.len();
            let mut h = self.z.len();
            let mut correction = 0;
            if let Some(q) = &self.lagrange {
                if let Some(f) = &q.folded {
                    k = k.max(f.private.len());
                }
                h = h.max(q.product.len());
                correction = q.constraints.len();
            }
            let geometry = Geometry {
                wires: plan.value_count() - plan.stored_slot_count(),
                slots: plan.stored_slot_count(),
                inputs: plan.input_count(),
                public: plan.n_public,
                domain: self.domain,
                a: self.a.len(),
                b: self.b.len(),
                k,
                h,
                correction,
            };
            let estimate = if self.has_gpu() || self.equal_witness.is_some() {
                None
            } else {
                forecast(geometry)
            };
            let estimate = estimate.and_then(|mut value| {
                if self.lagrange.as_ref().is_some_and(|q| q.euler) {
                    let extra = self
                        .domain
                        .checked_mul(core::mem::size_of::<Fr>())?
                        .checked_add(256 * core::mem::size_of::<fft::Quad>())?
                        .checked_add(4096)?;
                    value.fields = value.fields.checked_add(extra)?;
                    value.total = value.total.checked_add(extra)?;
                }
                Some(value)
            });
            if std::env::var_os("ZOPRO_CPU_WORKSPACE_TRACE").is_some() {
                eprintln!("TYPED_WORKSPACE geometry={geometry:?} estimate={estimate:?}");
            }
            self.proof_workspace = estimate.map(|v| v.total);
            plan.prepare_with_workspace(estimate.map(|v| v.total), true);
        }
        #[cfg(not(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        plan.prepare();
        trace("solver-schedule", started);
        #[cfg(not(zopro_live_preparation_charge))]
        self.prepare_solver_queries(plan);
    }

    fn prepare_solver_queries(&mut self, plan: &solver::Plan) {
        if self.basis_only {
            return;
        }
        let _ = plan;
        #[cfg(all(
            zopro_folded_query_reclaim,
            not(target_arch = "wasm32"),
            target_endian = "little"
        ))]
        if self
            .lagrange
            .as_ref()
            .is_some_and(|q| q.constraints.is_empty())
        {
            // A second solver installation needs a fresh linear-C correction.
            // Reserve its full builder peak before replacing the old queries;
            // if it cannot fit, keep the existing allocation-free fallback.
            self.lagrange = lagrange::Queries::load_or_prepare(&self.plan, &self.z);
        }
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        if let Some(queries) = &mut self.lagrange {
            queries.folded = None;
            if std::env::var("ZOPRO_CPU_LAGRANGE_FOLD").is_ok_and(|v| v == "1") {
                queries.fold(plan, &self.k);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.equal_witness = None;
            let mode = std::env::var("ZOPRO_CPU_PREPARED_EQUAL")
                .ok()
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(if crate::simd::cpu_option("ZOPRO_CPU_PREPARED_EQUAL") {
                    2
                } else {
                    0
                })
                .min(2);
            if !self.has_gpu() && self.a.len().max(self.b.len()) >= 32768 && mode > 0 {
                let started = crate::time::Instant::now();
                let Some(reservation) =
                    preparation::Reservation::acquire(self.domain, plan.preparation_peak())
                else {
                    return;
                };
                let groups = plan.equal_wire_groups();
                let mut prepared = msm_affine::PreparedEqualWitness::new(
                    &self.a,
                    &self.b,
                    &self.b2,
                    &plan.a_indices,
                    &plan.b_indices,
                    &groups,
                    plan.value_count(),
                );
                if mode == 2 {
                    let (k, indices, folded) =
                        (&self.k[..], plan.private_value_indices(&[]), false);
                    #[cfg(target_endian = "little")]
                    let (k, indices, folded) = self
                        .lagrange
                        .as_ref()
                        .and_then(|q| q.folded.as_ref())
                        .map_or((k, indices, folded), |q| {
                            (&q.private[..], plan.private_value_indices(&q.slots), true)
                        });
                    if let Some(prepared) = &mut prepared {
                        prepared.prepare_k(k, &indices, &groups, plan.value_count(), folded);
                    }
                }
                if let Some(prepared) = &mut prepared {
                    prepared.reservation = Some(reservation);
                }
                self.equal_witness = prepared;
                trace("prepare-equal-witness", started);
            }
        }
    }

    /// A built solver plan, when it matches the key's dimensions.
    pub(crate) fn set_solver_plan(&mut self, parsed: Result<solver::Plan, &'static str>) -> bool {
        self.invalidate_solver();
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.equal_witness = None;
        }
        if !self.commitments.is_empty() {
            return false;
        }
        let Ok(plan) = parsed else { return false };
        if !plan.matches(
            self.lengths[0],
            self.lengths[1],
            self.lengths[2],
            self.domain,
        ) {
            return false;
        }
        if !self.basis_plan_matches(&plan) {
            return false;
        }
        self.basis_fallback = None;
        self.prepare_plan(&plan);
        self.solver = Some(plan);
        true
    }
    /// `new` for hosts outside JavaScript, from either encoding of the
    /// points, with a plain error.
    pub fn build(points: Points, domain_params: &[u8]) -> Result<Key, &'static str> {
        Self::build_impl(points, domain_params, false)
    }
    fn build_impl(
        points: Points,
        domain_params: &[u8],
        basis_only: bool,
    ) -> Result<Key, &'static str> {
        let ([a, b, k, z, _b2_points], [generator, coset, denominator]) =
            points.validate_shape(domain_params)?;
        let domain = z + 1;
        let started = crate::time::Instant::now();
        let plan = fft::Plan::new(domain, generator, coset, denominator);
        trace("plan", started);
        let lengths = [a, b, k];
        // One vector at a time: decoded, handed to the GPU, dropped. A GPU
        // that cannot take the key (it failed to build a kernel, it ran out
        // of memory) is let go, and the key is decoded again for the CPU.
        let started = crate::time::Instant::now();
        #[cfg(gpu_any)]
        let mut gpu = if basis_only {
            None
        } else {
            GpuBuilder::open(&plan, _b2_points)
        };
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
                                gpu = GpuBuilder::open(&plan, _b2_points);
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
        let mut key = Self {
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            frontend_pool: None,
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            solver_pool: None,
            #[cfg(all(zopro_resource_schedule, zopro_g2_horner_buckets, feature = "ifma8", target_os = "linux", target_arch = "x86_64", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", not(no_ifma)))]
            exclusive_g2_pool: None,
            #[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
            grouped_point_tiling: false,
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            grouped_g2_on_frontend: false,
            exclusive_g2_grouping: true,
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            raw_g1_a_on_frontend: false,
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            g1_k_on_frontend: false,
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            grouped_b2_serial_preparation: false,
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            grouped_b2_poseidon_origins: false,
            basis_only,
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
            #[cfg(all(zopro_resource_schedule, feature="ifma8", target_arch="x86_64", target_os="linux", target_feature="avx512ifma", target_feature="avx512vl", target_feature="avx512dq", not(no_ifma)))]
            wire_offsets: None,
            #[cfg(all(target_arch = "x86_64", target_os = "linux", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
            canonical_wires: !cfg!(target_arch="wasm32") && lengths.iter().copied().max().unwrap_or(0)<(1<<16) && std::env::var("ZOPRO_CPU_CANONICAL_WIRES").as_deref()==Ok("1"),
            domain,
            plan,
            commitments: Vec::new(),
            proof_workspace: None,
            solver: None,
            basis_fallback: None,
            #[cfg(not(target_arch = "wasm32"))]
            equal_witness: None,
            #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
            lagrange: None,
        };
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        if !basis_only && !key.has_gpu() {
            let small_overlap = key.apple_small_msm_overlap();
            key.plan.prepare_apple_quotient(small_overlap);
        }
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        if !basis_only
            && key.domain >= 8
            && !key.has_gpu()
            && std::env::var("ZOPRO_CPU_LAGRANGE").is_ok_and(|v| v == "1")
        {
            key.lagrange = lagrange::Queries::load_or_prepare(&key.plan, &key.z);
        }

        Ok(key)
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
            plan.a_indices
                .iter()
                .map(|&i| solution.w[i as usize])
                .collect(),
            plan.b_indices
                .iter()
                .map(|&i| solution.w[i as usize])
                .collect(),
            solution.w[plan.n_public..].to_vec(),
        ])
    }
    /// The proof's parts from the solve's MSM scalars, which the CPU's MSMs
    /// take over (natively, as canonical limbs in place), and its a, b, c.
    /// For a validated solver's inputs, Metal can reconstruct missing c.
    fn compute_parts(
        &self,
        sa: &mut [Fr],
        sb: &mut [Fr],
        sk: &mut [Fr],
        a: Vec<Fr>,
        b: Vec<Fr>,
        c: Option<Vec<Fr>>,
        public: Option<&[Fr]>,
        omit_b: bool,
    ) -> Result<Vec<u8>, String> {
        #[cfg(gpu)]
        if let Some(gpu) = &self.gpu {
            let (sa, sb, sk) = (&*sa, &*sb, &*sk);
            // The MSMs whose scalars are ready run on the GPU, driven from a
            // thread of their own so that their waits hold no rayon worker,
            // while the GPU also takes the quotient and the pool computes
            // G2 when it is not on the GPU; Z's MSM follows the quotient.
            // The CPU G2 MSM normally starts after the sorts have finished
            // on their shared pool. Compact witness queries can use a
            // separate pool so their compaction overlaps G2.
            let (sorted_tx, sorted_rx) = std::sync::mpsc::channel::<()>();
            // The quotient has the GPU to itself: Z's sort and MSM wait for
            // it, the others for nothing, so they are submitted once it is
            // back (sharing the GPU with them it came back after 179 ms
            // rather than 45 on an M5 Pro, 54 → 1 merge).
            let started = crate::time::Instant::now();
            let mut quotient = self.gpu_quotient.then(|| gpu.quotient.lock().unwrap());
            let phone_metal = fast_ios_metal();
            let large_digits = self.domain >= (1 << 18);
            // Reuse device recoding for the smaller quotient domains when
            // their G1 MSMs use the narrower Apple silicon window caps.
            let wide_apple = cfg!(all(target_os = "macos", target_arch = "aarch64"))
                && rayon::current_num_threads() >= 16;
            let medium_apple = wide_apple && self.domain == (1 << 17);
            let narrow_digits = wide_apple && omit_b && self.domain == (1 << 16);
            let medium_digits = medium_apple && omit_b;
            let digit_width = ((large_digits || medium_digits || narrow_digits)
                && quotient.as_ref().is_some_and(|q| q.can_recode_msm())
                && std::env::var("ZOPRO_METAL_QUOTIENT_DIGITS").as_deref() != Ok("0"))
            .then(|| gpu.z.lock().unwrap().scalar_window_width())
            .filter(|&width| large_digits || width <= if medium_digits { 10 } else { 8 });
            // An admitted digit buffer stays charged through completion of
            // the final MSM. Without live headroom, read back coefficients.
            let _digit_workspace = if phone_metal && omit_b {
                digit_width.zip(quotient.as_ref()).and_then(|(width, quotient)| {
                    preparation::Reservation::analysis(quotient.digit_scatter_workspace(width))
                })
            } else {
                None
            };
            let digit_width = digit_width.filter(|_| !phone_metal || _digit_workspace.is_some());
            #[cfg(all(feature = "trace", target_os = "ios"))]
            if digit_width.is_some() {
                log::info!("ZOPRO_TRACE ios-quotient-digits");
            }
            #[cfg(feature = "trace")]
            if narrow_digits && digit_width.is_some() {
                log::info!("ZOPRO_TRACE metal-small-quotient-digits");
            }
            let pending = quotient.as_mut().map(|q| match (c.as_deref(), digit_width) {
                (None, width) => {
                    #[cfg(feature = "trace")]
                    log::info!("ZOPRO_TRACE metal-quotient-derive-c");
                    q.start_derived(&a, &b, width)
                }
                (Some(c), Some(width)) => q.start_for_msm(&a, &b, c, width),
                (Some(c), None) => q.start(&a, &b, c),
            });
            // The GPU has its copy of a, b and c: the CPU's go now rather
            // than after the MSMs (48 MB at 16 -> 2).
            let abc = if pending.is_some() {
                drop((a, b, c));
                None
            } else {
                Some((a, b, c.ok_or("CPU quotient requires C evaluations")?))
            };
            let (quotient_tx, quotient_rx) = std::sync::mpsc::channel::<()>();
            // A quotient on the CPU holds nothing up.
            let quotient_tx = pending.is_some().then_some(quotient_tx);
            #[cfg(feature = "trace")]
            let profile = log::log_enabled!(log::Level::Info)
                && !self.profiled.swap(true, std::sync::atomic::Ordering::Relaxed);
            #[cfg(not(feature = "trace"))]
            let profile = false;
            let ios_group = self.gpu_quotient && phone_metal;
            let group = omit_b
                && (cfg!(all(target_os = "macos", target_arch = "aarch64")) || ios_group)
                && std::env::var("ZOPRO_METAL_GROUP_WITNESS").as_deref() != Ok("0");
            // A mapped key retains the full G2 query. Keep its usual split
            // unless this proof can shrink all of G2 enough to run on the CPU.
            let grouped_b2 = (group
                && ios_group
                && gpu.b2.is_some()
                && matches!(&*self.b2, Store::Mapped { .. })
                && affine_msm()
                && std::env::var_os("ZOPRO_G2_GPU_SHARE").is_none()
                && std::env::var_os("ZOPRO_G2").is_none())
            .then(|| msm_affine::compact_phone_g2(&self.b2, sb))
            .flatten();
            let gpu_b2 = gpu.b2.as_ref().filter(|_| grouped_b2.is_none());
            #[cfg(feature = "trace")]
            if let Some((points, _, _)) = &grouped_b2 {
                log::info!(
                    "ZOPRO_TRACE ios-metal-compact-g2 {} {}",
                    sb.len(),
                    points.len()
                );
            }
            // For narrow queries the witness MSMs finish sooner when their
            // command buffers are queued before the quotient MSM.
            let witness_first = group
                && wide_apple
                && self.gpu_quotient
                && gpu.b2.is_none()
                && match self.domain {
                    65536 => (32768..49152).contains(&sb.len()) || (57344..65536).contains(&sb.len()),
                    131072 => (73728..90112).contains(&sb.len()),
                    _ => false,
                };
            let (witness_submitted_tx, witness_submitted_rx) =
                witness_first.then(std::sync::mpsc::channel::<()>).unzip();
            // These small witness sorts can both finish while Metal builds H.
            let parallel_small_sort = narrow_digits
                && self.gpu_quotient
                && gpu.b2.is_none()
                && ((40960..49152).contains(&sb.len()) || (57344..65536).contains(&sb.len()));
            // Below 64K B2 points the CPU can compact G2 on its own pool
            // while A and K prepare on the sort pool.
            let compact_pool = (group
                && (self.domain == (1 << 16) || (medium_apple && sb.len() < (1 << 16)))
                && self.gpu_quotient
                && gpu.b2.is_none()
                && rayon::current_num_threads() >= 18
                && crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                && std::env::var("ZOPRO_METAL_COMPACT_OVERLAP").as_deref() != Ok("0")
                && (medium_apple || msm_affine::metal_compact_candidate(sa)))
            .then(|| apple_msm_pool(12))
            .flatten();
            let compact_overlap = compact_pool.is_some();
            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            let borrow_large_g2 = group
                && wide_apple
                && self.domain == (1 << 16)
                && self.gpu_quotient
                && gpu.b2.is_none()
                && (53248..65536).contains(&sb.len())
                && msm_affine::apple_g2_three_parts_enabled();
            let prepare_g2 = group
                && self.domain == (1 << 15)
                && self.gpu_quotient
                && gpu.b2.is_none()
                && (1024..65536).contains(&sb.len())
                && affine_msm()
                && rayon::current_num_threads() >= 16
                && std::env::var("ZOPRO_METAL_PREPARE_G2").as_deref() != Ok("0");
            #[cfg(feature = "trace")]
            if compact_overlap {
                log::info!("ZOPRO_TRACE metal-compact-overlap");
            }
            let (h, (ar, bs1, kr, bs2, hz)) = std::thread::scope(|scope| -> Result<_, String> {
                let msms = scope.spawn(move || -> Result<_, String> {
                    // Each MSM is submitted as soon as it is sorted, so the
                    // host sorts the next while the GPU runs it. G2 B can
                    // reuse the sorted runs when G1 B is present.
                    let host_grouped = group
                        && gpu_b2.is_none()
                        && self.gpu_quotient
                        && (ios_group
                            || (self.domain >= (1 << 18) && rayon::current_num_threads() >= 16))
                        && std::env::var("ZOPRO_METAL_HOST_GROUPED").as_deref() != Ok("0");
                    let small_grouping = compact_overlap || ios_group;
                    let group_a = || {
                        group
                            .then(|| msm_affine::group_for_metal(&self.a, sa, small_grouping))
                            .flatten()
                    };
                    let group_k = || {
                        group
                            .then(|| msm_affine::group_for_metal(&self.k, sk, small_grouping))
                            .flatten()
                    };
                    let (grouped_a, grouped_k) = if compact_overlap {
                        sort_pool(true).install(|| rayon::join(group_a, group_k))
                    } else if host_grouped && !(ios_group && self.domain >= (1 << 20)) {
                        rayon::join(group_a, group_k)
                    } else {
                        // At the largest phone domains reuse the first query's
                        // scratch headroom when starting the second query.
                        (group_a(), group_k())
                    };
                    // Small compact queries can run alongside the quotient
                    // and Z MSM on Metal, reducing work on the GPU queue.
                    if let (true, Some((a, a_words, _)), Some((k, k_words, _))) =
                        (host_grouped, &grouped_a, &grouped_k)
                    {
                        if a.len() <= self.a.len() / 4 && k.len() <= self.k.len() / 4 {
                            #[cfg(all(feature = "trace", target_os = "ios"))]
                            log::info!("ZOPRO_TRACE ios-metal-host-a-k");
                            drop(sorted_tx);
                            let (ar, kr) = rayon::join(
                                || msm_affine::msm_raw(a, a_words),
                                || msm_affine::msm_raw(k, k_words),
                            );
                            return Ok((ar, G1Projective::IDENTITY, kr, None));
                        }
                    }
                    let (mut a, mut b, mut k) = (
                        gpu.a.lock().unwrap(),
                        gpu.b.lock().unwrap(),
                        gpu.k.lock().unwrap(),
                    );
                    let (a_msm, b_msm, k_msm) = (&mut *a, &mut *b, &mut *k);
                    let mut sort = || {
                        // Upload and sort compact inputs while the quotient
                        // runs. Larger witness queries can delay the GPU
                        // queue if both sorts must finish before submission.
                        if host_grouped
                            && self.domain == (1 << 18)
                            && sb.len() <= self.domain / 8 * 5
                            && (grouped_a.is_some() || grouped_k.is_some())
                            && std::env::var("ZOPRO_METAL_PRESORT_GROUPED").as_deref() != Ok("0")
                        {
                            #[cfg(feature = "trace")]
                            log::info!("ZOPRO_TRACE metal-presort-grouped");
                            let (sorted_a, sorted_k) = rayon::join(
                                || match &grouped_a {
                                    Some((points, words, _)) => a_msm.sort_grouped(points, words),
                                    None => a_msm.sort_now(sa),
                                },
                                || match &grouped_k {
                                    Some((points, words, _)) => k_msm.sort_grouped(points, words),
                                    None => k_msm.sort_now(sk),
                                },
                            );
                            return (Some(sorted_a), None, Some(sorted_k));
                        }
                        // Rerandomization omits B here. Sort the two independent
                        // witness queries concurrently while the quotient runs.
                        if group
                            && (self.domain == (1 << 15) || medium_apple || parallel_small_sort)
                            && rayon::current_num_threads() >= 16
                            && std::env::var("ZOPRO_METAL_PARALLEL_WITNESS_SORT").as_deref() != Ok("0")
                        {
                            #[cfg(feature = "trace")]
                            log::info!("ZOPRO_TRACE metal-parallel-witness-sort");
                            let (sorted_a, sorted_k) = rayon::join(
                                || grouped_a.is_none().then(|| a_msm.sort_now(sa)),
                                || grouped_k.is_none().then(|| k_msm.sort_now(sk)),
                            );
                            return (sorted_a, None, sorted_k);
                        }
                        let sorted_a = grouped_a.is_none().then(|| a_msm.sort_now(sa));
                        let sorted_b = (!omit_b).then(|| b_msm.sort_now(sb));
                        let sorted_k = grouped_k.is_none().then(|| k_msm.sort_now(sk));
                        (sorted_a, sorted_b, sorted_k)
                    };
                    let (sorted_a, sorted_b, sorted_k) = if compact_overlap {
                        sort_pool(true).install(sort)
                    } else {
                        sort()
                    };
                    drop(sorted_tx);
                    let _ = quotient_rx.recv();
                    let pa = match sorted_a {
                        Some(sorted) => a.submit_sorted(sorted),
                        None => {
                            let (points, words, _charge) = grouped_a.as_ref().unwrap();
                            a.start_grouped(points, words)
                        }
                    };
                    let pb = sorted_b.map(|sorted| b.submit_sorted(sorted));
                    let mut b2 = gpu_b2.map(|b2| b2.lock().unwrap());
                    let pb2 = b2.as_mut().map(|b2| {
                        if gpu.b2_cut == sb.len() && pb.is_some() {
                            b2.start_sorted_as(&*b)
                        } else {
                            b2.start(&sb[..gpu.b2_cut])
                        }
                    });
                    let pk = match sorted_k {
                        Some(sorted) => k.submit_sorted(sorted),
                        None => {
                            let (points, words, _charge) = grouped_k.as_ref().unwrap();
                            k.start_grouped(points, words)
                        }
                    };
                    drop(witness_submitted_tx);
                    let (ar, t) = a.finish(pa)?;
                    gpu_trace("a", t.sort_ms, t.gpu_ms);
                    let bs1 = if let Some(pb) = pb {
                        let (bs1, t) = b.finish(pb)?;
                        gpu_trace("b", t.sort_ms, t.gpu_ms);
                        bs1
                    } else {
                        G1Projective::IDENTITY
                    };
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
                let cut = if gpu_b2.is_some() { gpu.b2_cut } else { 0 };
                let cpu_b2 = (cut < sb.len()).then(|| {
                    scope.spawn(move || {
                        // Decode the G2 scalars while the host sorts A and K.
                        // Wait for both sorts before occupying the pool.
                        let words = prepare_g2.then(|| {
                            sb[cut..].iter().map(Fr::to_canonical).collect::<Vec<_>>()
                        });
                        if !compact_overlap {
                            let _ = sorted_rx.recv();
                        }
                        let started = crate::time::Instant::now();
                        let g2 = || {
                            let bases = self.cpu_b2(cut);
                            if let Some((points, words, _reservation)) = grouped_b2 {
                                msm_affine::msm_raw(&points, &words)
                            } else if let Some(mut words) = words {
                                #[cfg(feature = "trace")]
                                log::info!("ZOPRO_TRACE metal-prepared-g2");
                                msm_affine::g2_query_exclusive(
                                    bases,
                                    &mut words,
                                    self.exclusive_g2_grouping,
                                )
                            } else {
                                #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
                                if (compact_overlap || borrow_large_g2)
                                    && self.exclusive_g2_grouping
                                    && affine_msm()
                                {
                                    if let Some(sum) =
                                        msm_affine::try_apple_compact_g2(
                                            bases, &sb[cut..], borrow_large_g2,
                                        )
                                    {
                                        return sum;
                                    }
                                }
                                msm_g2(bases, &sb[cut..])
                            }
                        };
                        let bs2 = if let Some(pool) = compact_pool {
                            pool.install(g2)
                        } else {
                            g2()
                        };
                        trace("cpu-msm-b2", started);
                        bs2
                    })
                });
                let (h, digits) = match (quotient.as_mut(), pending, abc) {
                    (Some(q), Some(pending), _) if digit_width.is_some() => {
                        let (digits, h) = q.finish_msm_digits(pending, profile)?;
                        (h, Some(digits))
                    }
                    (Some(q), Some(pending), _) => (Some(q.finish(pending)?), None),
                    (_, _, Some((a, b, c))) => (Some(self.quotient(a, b, c)), None),
                    _ => unreachable!("the quotient on the GPU or on the CPU"),
                };
                drop((quotient, quotient_tx));
                trace("quotient", started);
                if let Some(submitted) = witness_submitted_rx {
                    // Wait for command submission before queuing Z.
                    let _ = submitted.recv();
                    #[cfg(feature = "trace")]
                    log::info!("ZOPRO_TRACE metal-witness-submit-first");
                }
                // Z's MSM is sorted and queued now, behind the others still
                // on the GPU, rather than after them; on a pool of its own,
                // since on the global one it queued behind the CPU's G2 MSM
                // (365 ms instead of 10 on an iPad Pro's A12Z at 8x2).
                let mut z = gpu.z.lock().unwrap();
                let msm_z = &mut *z;
                let pz = sort_pool(digits.is_some()).install(|| match &digits {
                    Some(digits) => msm_z.start_quotient_digits(digits),
                    None => Ok(msm_z.start(h.as_deref().expect("quotient coefficients"))),
                })?;
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
            if profile {
                let h = h.expect("quotient coefficients for profiling");
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
        let c = c.ok_or("derived C requires a Metal quotient")?;
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
                                sort_pool(false).install(|| z.start(&h))
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

        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        if self.basis_only && public.is_none() {
            return self.basis_parts(sa, sb, sk, a, b, c, omit_b);
        }
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        if let Some(queries) = self.lagrange.as_ref().filter(|q| {
            // A folded-only query has no independent constraint correction;
            // the generic public=None API retains the full original quotient.
            public.is_some() && q.folded.is_some() || !q.constraints.is_empty()
        }) {
            if let (Some(folded), Some(public)) = (&queries.folded, public) {
                drop(c);
                let started = crate::time::Instant::now();
                #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
                let pool = self.frontend_pool.as_deref().filter(|pool| {
                    pool.current_num_threads() < rayon::current_num_threads()
                        && std::env::var("ZOPRO_CPU_FFT_PRODUCT_FRONTEND").as_deref() == Ok("1")
                });
                #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
                let pool = None;
                let mut product = queries.product_values(&self.plan, a, b, pool);
                let started = trace("quotient-product", started);
                let product_points = &queries.product[..product.len()];
                let (((ar, bs1), (kr, (hz, bs2))), correction) = rayon::join(
                    || {
                        self.cpu_msms(
                            sa,
                            sb,
                            sk,
                            &mut product,
                            product_points,
                            &folded.private,
                            true,
                            omit_b,
                        )
                    },
                    || folded.public_term(public),
                );
                #[cfg(all(
                    zopro_resource_schedule,
                    feature = "ifma8",
                    target_arch = "x86_64",
                    target_os = "linux",
                    target_feature = "avx512ifma",
                    target_feature = "avx512vl",
                    target_feature = "avx512dq",
                    not(no_ifma)
                ))]
                let (ar, kr, bs2) = match self.wire_offsets.as_deref() {
                    Some(offsets) if omit_b && !self.canonical_wires => {
                        (ar + offsets.a, kr + offsets.k, bs2 + offsets.b2)
                    }
                    _ => (ar, kr, bs2),
                };
                trace("msms", started);
                let mut out = Vec::with_capacity(384);
                for point in [ar, bs1, kr + correction, queries.finish_product(hz)] {
                    output_g1(&mut out, point);
                }
                output_g2(&mut out, bs2);
                return Ok(out);
            }
            let started = crate::time::Instant::now();
            let mut product = queries.product_values(&self.plan, a, b, None);
            let started = trace("quotient-product", started);
            let (((ar, bs1), (kr, (positive, bs2))), negative) = rayon::join(
                || {
                    self.cpu_msms(
                        sa,
                        sb,
                        sk,
                        &mut product,
                        &queries.product,
                        &self.k,
                        false,
                        omit_b,
                    )
                },
                || msm_g1(&queries.constraints[..c.len()], &c),
            );
            trace("msms", started);
            let mut out = Vec::with_capacity(384);
            for point in [ar, bs1, kr, positive - negative] {
                output_g1(&mut out, point);
            }
            output_g2(&mut out, bs2);
            return Ok(out);
        }

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        if omit_b && self.apple_large_msm_overlap() {
            // Account for the three quotient arrays that can remain live while
            // the witness MSMs fill their buckets.
            let overlap_bytes = self.domain.checked_mul(3 * std::mem::size_of::<Fr>());
            if let Some(_reservation) = preparation::Reservation::scalar_grouping(overlap_bytes) {
                return Ok(self.apple_large_cpu_parts(sa, sb, sk, (a, b, c)));
            }
        }

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let apple_groups = matches!(self.domain, 32768 | 65536)
            && std::env::var("ZOPRO_CPU_WITNESS_GROUP_PREP").as_deref() != Ok("0");
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            // Overlap the G2 query with the quotient and G1 queries while
            // keeping eighteen workers in total on wide Apple silicon Macs.
            let pools = (omit_b && self.apple_small_msm_overlap())
                .then(|| apple_msm_pool(12).zip(apple_msm_pool(6)))
                .flatten();
            if let Some((g1, g2)) = pools {
                use msm_affine::{
                    canonical, canonical_exclusive, g2_query_exclusive, msm_canonical,
                };
                let ((ar, kr, hz), bs2) = std::thread::scope(|scope| {
                    let b2 = scope.spawn(|| {
                        g2.install(|| {
                            if apple_groups {
                                return msm_affine::apple_b2_from_fr(
                                    &self.b2,
                                    sb,
                                    self.exclusive_g2_grouping,
                                    false,
                                );
                            }
                            g2_query_exclusive(
                                &self.b2,
                                canonical_exclusive(sb),
                                self.exclusive_g2_grouping,
                            )
                        })
                    });
                    let grouped = g1.install(|| {
                        let started = crate::time::Instant::now();
                        let a_query = || {
                            if apple_groups {
                                msm_affine::apple_a_from_fr(&self.a, sa, false)
                            } else {
                                msm_canonical(&self.a, canonical(sa))
                            }
                        };
                        if msm_affine::apple_fused_kh_enabled(self.k.len(), self.domain - 1) {
                            // The fused sum needs H, but K's grouping depends only on the witness.
                            let (mut h, prepared) = rayon::join(
                                || self.quotient(a, b, c),
                                || msm_affine::prepare_apple_k(&self.k, sk, 0..usize::MAX, apple_groups),
                            );
                            let started = trace("cpu-quotient-and-k-prepare", started);
                            let (ar, kh) = rayon::join(
                                a_query,
                                || {
                                    msm_affine::apple_fused_kh(&self.k, sk, &self.z, &mut h, prepared, false)
                                },
                            );
                            trace("cpu-g1-msms", started);
                            return (ar, kh, G1Projective::IDENTITY);
                        }
                        let mut h = self.quotient(a, b, c);
                        let started = trace("quotient", started);
                        let ((ar, kr), hz) = rayon::join(
                            || rayon::join(a_query, || msm_canonical(&self.k, canonical(sk))),
                            || msm_canonical(&self.z, canonical(&mut h)),
                        );
                        trace("cpu-g1-msms", started);
                        (ar, kr, hz)
                    });
                    (grouped, b2.join().expect("CPU G2 MSM"))
                });
                let mut out = Vec::with_capacity(384);
                for point in [ar, G1Projective::IDENTITY, kr, hz] {
                    output_g1(&mut out, point);
                }
                output_g2(&mut out, bs2);
                return Ok(out);
            }
        }

        let started = crate::time::Instant::now();
        let quotient = || self.quotient(a, b, c);
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let (mut h, prepared) = {
            let fuse = omit_b
                && self.domain == (1 << 16)
                && self.equal_witness.is_none()
                && sa.len().max(sb.len()).max(sk.len()) < (1 << 16)
                && affine_msm()
                && rayon::current_num_threads() >= 18
                && crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                && msm_affine::apple_fused_kh_enabled(self.k.len(), self.domain - 1)
                && std::env::var("ZOPRO_CPU_WITNESS_G1_ENDO").as_deref() != Ok("0")
                && std::env::var("ZOPRO_TRACE_MSMS").as_deref() != Ok("1");
            if fuse {
                // K can be compacted before the quotient supplies H.
                rayon::join(quotient, || {
                    msm_affine::prepare_apple_k(&self.k, sk, 1024..32768, apple_groups)
                })
            } else {
                (quotient(), None)
            }
        };
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        let mut h = quotient();
        let started = trace("quotient", started);
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        if let Some(prepared) = prepared {
            use msm_affine::{canonical, canonical_exclusive, g2_query_exclusive, msm_g1_witness};
            let ((ar, kh), bs2) = rayon::join(
                || rayon::join(
                    || {
                        if apple_groups {
                            msm_affine::apple_a_from_fr(&self.a, sa, false)
                        } else {
                            msm_g1_witness(&self.a, canonical(sa))
                        }
                    },
                    || msm_affine::apple_fused_kh(&self.k, sk, &self.z, &mut h, Some(prepared), false),
                ),
                || {
                    if apple_groups {
                        msm_affine::apple_b2_from_fr(&self.b2, sb, self.exclusive_g2_grouping, false)
                    } else {
                        g2_query_exclusive(&self.b2, canonical_exclusive(sb), self.exclusive_g2_grouping)
                    }
                },
            );
            trace("msms", started);
            let mut out = Vec::with_capacity(384);
            for point in [ar, G1Projective::IDENTITY, kh, G1Projective::IDENTITY] {
                output_g1(&mut out, point);
            }
            output_g2(&mut out, bs2);
            return Ok(out);
        }
        // Diagnostic replays need the original field values: cpu_msms consumes
        // scalar storage, including the exclusive G2 decomposition.
        #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
        let trace_scalars = std::env::var("ZOPRO_TRACE_MSMS")
            .is_ok_and(|v| v == "1")
            .then(|| (sa.to_vec(), sb.to_vec(), sk.to_vec(), h.to_vec()));
        let ((ar, bs1), (kr, (hz, bs2))) =
            self.cpu_msms(sa, sb, sk, &mut h, &self.z, &self.k, false, omit_b);
        trace("msms", started);
        // Each MSM alone, on the whole pool, for its own share: this adds
        // to the proof time, so only with ZOPRO_TRACE_MSMS=1.
        #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
        if let Some((sa, sb, sk, h)) = trace_scalars {
            for (label, bases, scalars) in [
                ("msm-a", &self.a, &sa[..]),
                ("msm-b", &self.b, &sb[..]),
                ("msm-k", &self.k, &sk[..]),
                ("msm-z", &self.z, &h[..]),
            ] {
                let started = crate::time::Instant::now();
                let _ = msm_g1(bases, scalars);
                trace(label, started);
            }
            let started = crate::time::Instant::now();
            let _ = msm_g2(&self.b2, &sb);
            trace("msm-b2", started);
        }
        let mut out = Vec::with_capacity(384);
        for p in [ar, bs1, kr, hz] {
            output_g1(&mut out, p)
        }
        output_g2(&mut out, bs2);
        Ok(out)
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn apple_small_paired_cpu_parts(
        &self,
        plan: &solver::Plan,
        witness: &[Fr],
        (a, b, c): (Vec<Fr>, Vec<Fr>, Vec<Fr>),
        (g1, g2): (&rayon::ThreadPool, &rayon::ThreadPool),
    ) -> Vec<u8> {
        use rayon::prelude::*;
        #[cfg(feature = "trace")]
        log::info!("ZOPRO_TRACE cpu-small-paired-quotient-schedule");

        let private = &witness[plan.n_public..];
        let gather = |indices: &[u32]| -> Vec<Fr> {
            indices
                .par_iter()
                .with_min_len(2048)
                .map(|&i| witness[i as usize])
                .collect()
        };
        // K preparation can share the B2 pool while the G1 pool computes
        // the quotient and gathers A. Send K as soon as it is ready.
        let ((ar, kh), bs2) = std::thread::scope(|scope| {
            let (send_k, receive_k) = std::sync::mpsc::sync_channel(1);
            let b2 = scope.spawn(move || {
                g2.install(move || {
                    let (_, bs2) = rayon::join(
                        move || {
                            let prepared = msm_affine::prepare_apple_k(
                                &self.k, private, 0..usize::MAX, true,
                            );
                            let _ = send_k.send(prepared);
                        },
                        || {
                            let mut sb = gather(&plan.b_indices);
                            msm_affine::apple_b2_from_fr(
                                &self.b2, &mut sb, self.exclusive_g2_grouping, true,
                            )
                        },
                    );
                    bs2
                })
            });
            let ak = g1.install(move || {
                let started = crate::time::Instant::now();
                let (mut h, mut sa) = rayon::join(
                    || self.plan.apple_small_quotient(a, b, c),
                    || gather(&plan.a_indices),
                );
                let prepared = receive_k.recv().expect("K preparation result");
                let started = trace("cpu-quotient-and-k-prepare", started);
                let ak = rayon::join(
                    || msm_affine::apple_a_from_fr(&self.a, &mut sa, true),
                    || {
                        // The ungrouped route needs exclusive scalar storage.
                        let mut sk = if prepared.is_none() { private.to_vec() } else { Vec::new() };
                        msm_affine::apple_fused_kh(&self.k, &mut sk, &self.z, &mut h, prepared, true)
                    },
                );
                trace("cpu-g1-msms", started);
                ak
            });
            (ak, b2.join().expect("CPU G2 MSM"))
        });
        let mut out = Vec::with_capacity(384);
        for point in [ar, G1Projective::IDENTITY, kh, G1Projective::IDENTITY] {
            output_g1(&mut out, point);
        }
        output_g2(&mut out, bs2);
        out
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn apple_small_msm_overlap(&self) -> bool {
        let limit = match self.domain {
            32768 | 65536 => 65536,
            131072 => 131072,
            _ => return false,
        };
        ((1 << 14)..limit).contains(&self.b2.len())
            && affine_msm()
            && self.equal_witness.is_none()
            && rayon::current_num_threads() >= 18
            && std::env::var("ZOPRO_CPU_MSM_OVERLAP").as_deref() != Ok("0")
            && std::env::var("ZOPRO_TRACE_MSMS").as_deref() != Ok("1")
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn apple_large_msm_overlap(&self) -> bool {
        self.domain == (1 << 20)
            && self.b2.len() >= (1 << 16)
            && affine_msm()
            && self.equal_witness.is_none()
            && crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
            && rayon::current_num_threads() >= 18
            && std::env::var("ZOPRO_CPU_MSM_OVERLAP").as_deref() != Ok("0")
            && std::env::var("ZOPRO_TRACE_MSMS").as_deref() != Ok("1")
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    fn apple_large_cpu_parts(
        &self,
        sa: &mut [Fr],
        sb: &mut [Fr],
        sk: &mut [Fr],
        (a, b, c): (Vec<Fr>, Vec<Fr>, Vec<Fr>),
    ) -> Vec<u8> {
        use msm_affine::{msm_in_place, msm_montgomery};

        let started = crate::time::Instant::now();
        let ((ar, kr, hz), bs2) = rayon::join(
            || {
                let (mut h, (ar, kr)) = rayon::join(
                    || self.quotient(a, b, c),
                    || rayon::join(|| msm_in_place(&self.a, sa), || msm_in_place(&self.k, sk)),
                );
                let hz = if crate::simd::cpu_option("ZOPRO_CPU_MONTGOMERY_H") {
                    msm_montgomery(&self.z, &h)
                } else {
                    msm_in_place(&self.z, &mut h)
                };
                (ar, kr, hz)
            },
            || msm_in_place(&self.b2, sb),
        );
        trace("cpu-large-quotient-and-msms", started);

        let mut out = Vec::with_capacity(384);
        for point in [ar, G1Projective::IDENTITY, kr, hz] {
            output_g1(&mut out, point);
        }
        output_g2(&mut out, bs2);
        out
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
        z: &[G1Affine],
        k: &[G1Affine],
        folded_k: bool,
        omit_b: bool,
    ) -> (
        (G1Projective, G1Projective),
        (G1Projective, (G1Projective, G2Projective)),
    ) {
        let _ = folded_k;
        #[cfg(not(target_arch = "wasm32"))]
        if affine_msm() {
            let montgomery_h =
                h.len() >= (1 << 18) && crate::simd::cpu_option("ZOPRO_CPU_MONTGOMERY_H");
            if let Some(prepared) = &self.equal_witness {
                let (((ar, kr), hz), (bs1, bs2)) = rayon::join(
                    || {
                        rayon::join(
                            || {
                                rayon::join(
                                    || prepared.msm_a(&self.a, sa),
                                    || prepared.msm_k(k, sk, folded_k),
                                )
                            },
                            || {
                                if montgomery_h {
                                    msm_affine::msm_montgomery(z, h)
                                } else {
                                    msm_affine::msm_in_place(z, h)
                                }
                            },
                        )
                    },
                    || if omit_b { (G1Projective::IDENTITY, msm_affine::msm_in_place(&self.b2, sb)) } else { prepared.msms_b(&self.b, &self.b2, sb) },
                );
                return ((ar, bs1), (kr, (hz, bs2)));
            }
            if sa.len().max(sb.len()).max(sk.len()) < (1 << 16)
                || !crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
            {
                use msm_affine::{
                    canonical, msm_canonical as msm, msm_grouped, msm_raw, scalar_groups,
                };
                let (sa, sb, sk) = (canonical(sa), msm_affine::canonical_exclusive(sb), canonical(sk));
                #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
                let endomorphism = omit_b
                    && !folded_k
                    && self.domain == (1 << 16)
                    && rayon::current_num_threads() >= 16
                    && std::env::var("ZOPRO_CPU_WITNESS_G1_ENDO").as_deref() != Ok("0");
                let g1_query = |bases: &[G1Affine], scalars| {
                    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
                    if endomorphism {
                        return msm_affine::msm_g1_witness(bases, scalars);
                    }
                    msm(bases, scalars)
                };
                let (((ar, kr), hz), (bs1, bs2)) = rayon::join(
                    || {
                        rayon::join(
                            || rayon::join(|| g1_query(&self.a, sa), || g1_query(k, sk)),
                            || {
                                if montgomery_h {
                                    msm_affine::msm_montgomery(z, h)
                                } else {
                                    msm_affine::msm_h_in_place(z, h)
                                }
                            },
                        )
                    },
                    || {
                        if omit_b && (1024..65536).contains(&self.b2.len()) {
                            return (G1Projective::IDENTITY,msm_affine::g2_query_exclusive(&self.b2, sb, self.exclusive_g2_grouping));
                        }
                        let groups = crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                            .then(|| scalar_groups(sb))
                            .flatten();
                        rayon::join(
                            || {
                                if omit_b { return G1Projective::IDENTITY; }
                                groups.as_ref().map_or_else(
                                    || msm_raw(&self.b[..], sb),
                                    |g| msm_grouped(&self.b[..], g),
                                )
                            },
                            || {
                                groups.as_ref().map_or_else(
                                    || msm_raw_b2(&self.b2[..], sb),
                                    |g| msm_grouped_b2(&self.b2[..], g),
                                )
                            },
                        )
                    },
                );
                return ((ar, bs1), (kr, (hz, bs2)));
            }
            use msm_affine::{canonical, group_witness, msm_grouped, msm_in_place as msm, msm_raw};
            let compute_a = || {
                #[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
                if self.grouped_point_tiling {
                    if let Some(sum) = msm_affine::tiled_grouped_msm(&self.a, sa, 32) {
                        return sum;
                    }
                }
                msm(&self.a, sa)
            };
            let compute_k = || {
                #[cfg(all(
                    zopro_resource_schedule,
                    feature = "ifma8",
                    target_arch = "x86_64",
                    target_os = "linux",
                    target_feature = "avx512ifma",
                    target_feature = "avx512vl",
                    target_feature = "avx512dq",
                    not(no_ifma)
                ))]
                if self.g1_k_on_frontend {
                    if let Some(pool) = self.frontend_pool.as_ref().filter(|pool| {
                        pool.current_num_threads() < rayon::current_num_threads()
                    }) {
                        let result = pool.install(|| {
                            let workers = rayon::current_num_threads();
                            let _scratch = crate::preparation::Reservation::analysis(
                                msm_affine::workspace::frontend_g1_bytes(sk.len(), workers, true),
                            )?;
                            if std::env::var_os("ZOPRO_K_FRONTEND_TRACE").is_some() {
                                eprintln!("K_FRONTEND {} {workers}", sk.len());
                            }
                            if self.grouped_point_tiling {
                                if let Some(sum) = msm_affine::tiled_grouped_msm(k, sk, 16) {
                                    return Some(sum);
                                }
                            }
                            Some(msm(k, &mut *sk))
                        });
                        if let Some(result) = result {
                            return result;
                        }
                    }
                }
                msm(k, sk)
            };
            let (((ar, kr), hz), (bs1, bs2)) = rayon::join(
                || {
                    rayon::join(
                        || rayon::join(compute_a, compute_k),
                        || {
                            if montgomery_h {
                                msm_affine::msm_montgomery(z, h)
                            } else {
                                msm(z, h)
                            }
                        },
                    )
                },
                || {
                    if omit_b && (1024..65536).contains(&self.b2.len()) {
                        let words=msm_affine::canonical_exclusive(sb);
                        return (G1Projective::IDENTITY,msm_affine::g2_query_exclusive(&self.b2, words, self.exclusive_g2_grouping));
                    }
                    #[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
                    if self.grouped_point_tiling && omit_b && !self.grouped_b2_poseidon_origins
                        && std::env::var("ZOPRO_CPU_GROUPED_G2_PHI").as_deref() != Ok("1")
                    {
                        if let Some(sum) = msm_affine::tiled_grouped_msm(&self.b2, sb, 16) {
                            return (G1Projective::IDENTITY, sum);
                        }
                    }
                    let serial = {
                        #[cfg(all(
                            target_os = "linux", target_arch = "x86_64", feature = "ifma8",
                            target_feature = "avx512ifma", target_feature = "avx512vl",
                            target_feature = "avx512dq", not(no_ifma),
                        ))]
                        { self.grouped_b2_serial_preparation && omit_b && (65_536..262_144).contains(&sb.len()) }
                        #[cfg(not(all(
                            target_os = "linux", target_arch = "x86_64", feature = "ifma8",
                            target_feature = "avx512ifma", target_feature = "avx512vl",
                            target_feature = "avx512dq", not(no_ifma),
                        )))]
                        { false }
                    };
                    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
                    let ordinary = || if serial { msm_affine::group_witness_serial_b2(sb) } else { group_witness(sb) };
                    #[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
                    let groups = if omit_b && (65_536..262_144).contains(&sb.len())
                        && self.grouped_b2_poseidon_origins
                    {
                        self.solver.as_ref().and_then(solver::Plan::b2_origins)
                            .and_then(|origins| msm_affine::group_witness_b2_origins(sb, origins, serial))
                            .or_else(ordinary)
                    } else {
                        ordinary()
                    };
                    #[cfg(all(not(zopro_resource_schedule), target_os = "linux", target_arch = "x86_64"))]
                    let groups = ordinary();
                    #[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
                    let groups = {
                        let _ = serial;
                        if omit_b {
                            msm_affine::group_witness_for::<zopro_bn254::G2Config>(sb)
                        } else {
                            group_witness(sb)
                        }
                    };
                    if let Some(groups) = groups {
                        #[cfg(all(zopro_g2_horner_buckets, target_arch = "x86_64", target_feature = "avx512ifma", target_feature = "avx512vl", target_feature = "avx512dq", feature = "ifma8", not(no_ifma)))]
                        if omit_b
                            && self.b2.len() >= 65_536
                            && std::env::var("ZOPRO_CPU_GROUPED_G2_PHI").as_deref() == Ok("1")
                        {
                            #[cfg(target_os = "linux")]
                            let pool = self.frontend_pool.as_deref()
                                .filter(|_| self.grouped_g2_on_frontend);
                            #[cfg(not(target_os = "linux"))]
                            let pool = None;
                            let sum = msm_affine::owned_grouped_phi(&self.b2, groups, pool);
                            return (G1Projective::IDENTITY, sum);
                        }
                        rayon::join(
                            || if omit_b { G1Projective::IDENTITY } else { msm_grouped(&self.b[..], &groups) },
                            || msm_grouped_b2(&self.b2[..], &groups),
                        )
                    } else {
                        let sb = canonical(sb);
                        rayon::join(|| if omit_b { G1Projective::IDENTITY } else { msm_raw(&self.b[..], sb) }, || msm_raw_b2(&self.b2[..], sb))
                    }
                },
            );
            return ((ar, bs1), (kr, (hz, bs2)));
        }
        rayon::join(
            || rayon::join(|| msm_g1(&self.a, sa), || if omit_b { G1Projective::IDENTITY } else { msm_g1(&self.b, sb) }),
            || {
                rayon::join(
                    || msm_g1(k, sk),
                    || rayon::join(|| msm_g1(z, h), || msm_g2(&self.b2, sb)),
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
                        .map(|i| if i % 3 == 0 { Fr::from(1u64) } else { rng.fr() })
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
            #[cfg(gpu)]
            if round == 0 {
                let metal = gpu.gpu.as_ref().unwrap();
                let mut quotient = metal.quotient.lock().unwrap();
                if quotient.can_recode_msm() {
                    let (a, b, c) = (frs(&ea), frs(&eb), frs(&ec));
                    let mut msm = metal.z.lock().unwrap();
                    let pending = quotient.start_for_msm(&a, &b, &c, msm.scalar_window_width());
                    let (digits, _) = quotient.finish_msm_digits(pending, false).unwrap();
                    let pending = msm.start_quotient_digits(&digits).unwrap();
                    let (got, _) = msm.finish(pending).unwrap();
                    assert_eq!(got, msm_g1(&z, &cpu.quotient(a, b, c)));
                }
            }
        }
    }
}

#[cfg(all(
    target_arch = "x86_64",
    target_os = "linux",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
impl Key {
    fn solve_canonical_wires(
        &self,
        plan: &solver::Plan,
        witness: &[Fr],
        materialize_c: bool,
        queries: &lagrange::Queries,
        folded: &lagrange::Folded,
        omit_b: bool,
        reclaim: Option<&alloc::ProofScope>,
    ) -> Result<Vec<u8>, String> {
        use msm_affine::{canonical, msm_canonical as msm, msm_grouped, msm_raw, scalar_groups};
        use rayon::prelude::*;
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        let offsets = self.wire_offsets.as_ref().filter(|_| omit_b);
        let prepare = || -> Result<_, String> {
            let solver::Solution { mut w, a, b, c } =
                plan.solve_in_with_slots(witness, self.domain, &folded.slots, materialize_c)?;
            if let Some(scope) = reclaim {
                scope.release_unused(&mut w);
            }
            drop(c);
            // The correction consumes Montgomery public values before the one
            // in-place conversion. All later accesses to w use canonical words.
            let correction = folded.public_term(&w[..plan.n_public]);
            #[cfg(all(
                zopro_resource_schedule,
                feature = "ifma8",
                target_arch = "x86_64",
                target_os = "linux",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            ))]
            if let Some(offsets) = offsets {
                offsets.apply(&mut w);
            }
            let (sa_vec, sb_vec): (Vec<_>, Vec<_>) =
                if crate::simd::cpu_option("ZOPRO_CPU_FUSE_SCALAR_PREPARATION") {
                    fused_canonical_gather::prepare(&mut w, &plan.a_indices, &plan.b_indices)
                } else {
                    let words = canonical(&mut w);
                    rayon::join(
                        || {
                            plan.a_indices
                                .par_iter()
                                .map(|&i| words[i as usize])
                                .collect()
                        },
                        || {
                            plan.b_indices
                                .par_iter()
                                .map(|&i| words[i as usize])
                                .collect()
                        },
                    )
                };
            Ok((w, sa_vec, sb_vec, a, b, correction))
        };
        let exclusive_g2 = |words: &mut [[u64; 4]]| {
            #[cfg(all(
                zopro_resource_schedule,
                zopro_g2_horner_buckets,
                feature = "ifma8",
                target_os = "linux",
                target_arch = "x86_64",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            ))]
            if let Some(placement) = &self.exclusive_g2_pool {
                if let Some(_scratch) =
                    preparation::Reservation::analysis(Some(placement.scratch_bytes))
                {
                    return placement.workers.install(|| self.exclusive_g2(words));
                }
            }
            self.exclusive_g2(words)
        };
        let finish = |w: Vec<Fr>,
                      sa_vec: Vec<[u64; 4]>,
                      mut sb_vec: Vec<[u64; 4]>,
                      mut product: Vec<Fr>,
                      correction,
                      defer_g2: bool|
         -> Vec<u8> {
            // SAFETY: both branches converted every Fr in place to canonical limbs.
            // Fr is repr(transparent) over four u64 words; the shared view ends before w drops.
            let words =
                unsafe { std::slice::from_raw_parts(w.as_ptr().cast::<[u64; 4]>(), w.len()) };
            let (sa, sb, sk) = (&sa_vec[..], &mut sb_vec[..], &words[plan.n_public..]);
            let count = product.len();
            let (h, z, k) = (
                &mut product[..],
                &queries.product[..count],
                &folded.private[..],
            );
            #[cfg(all(zopro_real_kh, zopro_resource_schedule))]
            // A single weighted sum is valid when B1 is omitted and H has unit scale.
            let medium = if omit_b
                && queries.product_scale == Fr::ONE
                && (1024..65536).contains(&k.len())
                && (1024..65536).contains(&z.len())
                && k.len() + z.len() >= 65536
                && std::env::var("ZOPRO_CPU_MEDIUM_KH").as_deref() == Ok("1")
            {
                crate::preparation::Reservation::analysis(msm_affine::workspace::medium_kh_bytes(
                    k.len(),
                    z.len(),
                ))
            } else {
                None
            };
            #[cfg(all(zopro_real_kh, not(zopro_resource_schedule)))]
            let medium: Option<()> = None;
            #[cfg(all(
                zopro_real_kh,
                feature = "ifma8",
                target_arch = "x86_64",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            ))]
            if msm_affine::fused_kh::eligible(omit_b, queries.product_scale, k.len(), z.len())
                || medium.is_some()
            {
                if std::env::var_os("ZOPRO_FUSED_KH_TRACE").is_some() {
                    eprintln!("FUSED_KH k={} h={} width=13", k.len(), z.len());
                }
                let a = || {
                    #[cfg(zopro_resource_schedule)]
                    if self.raw_g1_a_on_frontend && !defer_g2 {
                        if let Some(pool) = self.frontend_pool.as_ref().filter(|pool| {
                            pool.current_num_threads() < rayon::current_num_threads()
                        }) {
                            if let Some(_scratch) = crate::preparation::Reservation::analysis(
                                msm_affine::workspace::frontend_g1_bytes(
                                    sa.len(),
                                    pool.current_num_threads(),
                                    false,
                                ),
                            ) {
                                return pool.install(|| msm_raw(&self.a[..], sa));
                            }
                        }
                    }
                    msm(&self.a[..], sa)
                };
                let ((ar, kh), bs2) = rayon::join(
                    || {
                        rayon::join(a, || {
                            msm_affine::fused_kh::fused(k, sk, z, canonical(h), 13)
                        })
                    },
                    || {
                        if defer_g2 {
                            G2Projective::IDENTITY
                        } else if (1024..65536).contains(&self.b2.len()) {
                            exclusive_g2(sb)
                        } else {
                            let groups = crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                                .then(|| scalar_groups(sb))
                                .flatten();
                            groups.as_ref().map_or_else(
                                || msm_raw_b2(&self.b2, sb),
                                |g| msm_grouped_b2(&self.b2, g),
                            )
                        }
                    },
                );
                #[cfg(all(
                    zopro_resource_schedule,
                    feature = "ifma8",
                    target_arch = "x86_64",
                    target_os = "linux",
                    target_feature = "avx512ifma",
                    target_feature = "avx512vl",
                    target_feature = "avx512dq",
                    not(no_ifma)
                ))]
                let (ar, kh, bs2) =
                    offsets.map_or((ar, kh, bs2), |o| (ar + o.a, kh + o.k, bs2 + o.b2));
                let mut out = Vec::with_capacity(384);
                for point in [
                    ar,
                    G1Projective::IDENTITY,
                    kh + correction,
                    G1Projective::IDENTITY,
                ] {
                    output_g1(&mut out, point);
                }
                if !defer_g2 {
                    output_g2(&mut out, bs2);
                }
                return out;
            }
            let (((ar, kr), hz), (bs1, bs2)) = {
                let montgomery_h =
                    h.len() >= (1 << 18) && crate::simd::cpu_option("ZOPRO_CPU_MONTGOMERY_H");
                rayon::join(
                    || {
                        rayon::join(
                            || rayon::join(|| msm(&self.a[..], sa), || msm(k, sk)),
                            || {
                                if montgomery_h {
                                    msm_affine::msm_montgomery(z, h)
                                } else {
                                    msm(z, canonical(h))
                                }
                            },
                        )
                    },
                    || {
                        if omit_b && (1024..65536).contains(&self.b2.len()) {
                            return (
                                G1Projective::IDENTITY,
                                if defer_g2 {
                                    G2Projective::IDENTITY
                                } else {
                                    exclusive_g2(sb)
                                },
                            );
                        }
                        let groups = crate::simd::cpu_option("ZOPRO_CPU_DEDUP")
                            .then(|| scalar_groups(sb))
                            .flatten();
                        rayon::join(
                            || {
                                if omit_b {
                                    return G1Projective::IDENTITY;
                                }
                                groups.as_ref().map_or_else(
                                    || msm_raw(&self.b[..], sb),
                                    |g| msm_grouped(&self.b[..], g),
                                )
                            },
                            || {
                                groups.as_ref().map_or_else(
                                    || msm_raw_b2(&self.b2[..], sb),
                                    |g| msm_grouped_b2(&self.b2[..], g),
                                )
                            },
                        )
                    },
                )
            };

            #[cfg(all(
                zopro_resource_schedule,
                feature = "ifma8",
                target_arch = "x86_64",
                target_os = "linux",
                target_feature = "avx512ifma",
                target_feature = "avx512vl",
                target_feature = "avx512dq",
                not(no_ifma)
            ))]
            let (ar, kr, bs2) = offsets.map_or((ar, kr, bs2), |o| (ar + o.a, kr + o.k, bs2 + o.b2));
            let mut out = Vec::with_capacity(384);
            for point in [ar, bs1, kr + correction, queries.finish_product(hz)] {
                output_g1(&mut out, point);
            }
            if !defer_g2 {
                output_g2(&mut out, bs2);
            }
            out
        };

        let frontend = self
            .frontend_pool
            .as_ref()
            .filter(|pool| pool.current_num_threads() < rayon::current_num_threads());

        #[cfg(all(
            zopro_resource_schedule,
            zopro_g2_horner_buckets,
            feature = "ifma8",
            target_os = "linux",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        if omit_b {
            if let Some(placement) = self
                .exclusive_g2_pool
                .as_ref()
                .filter(|placement| placement.overlap_quotient)
            {
                if let Some(_scratch) =
                    preparation::Reservation::analysis(Some(placement.scratch_bytes))
                {
                    let prepared = match frontend {
                        Some(pool) => pool.install(prepare),
                        None => prepare(),
                    };
                    let (w, sa, mut sb, a, b, correction) = prepared?;
                    let mut g2 = G2Projective::IDENTITY;
                    let mut out = placement.workers.in_place_scope(|scope| {
                        let g2 = &mut g2;
                        scope.spawn(move |_| *g2 = self.exclusive_g2(&mut sb));
                        let product = queries.product_values(&self.plan, a, b, None);
                        finish(w, sa, Vec::new(), product, correction, true)
                    });
                    if let Some(offsets) = offsets {
                        g2 += offsets.b2;
                    }
                    output_g2(&mut out, g2);
                    return Ok(out);
                }
            }
        }

        let prepare_with_quotient = || {
            let (w, sa, sb, a, b, correction) = prepare()?;
            let product = queries.product_values(&self.plan, a, b, None);
            Ok::<_, String>((w, sa, sb, product, correction))
        };
        let (w, sa, sb, product, correction) = match frontend {
            Some(pool) => pool.install(prepare_with_quotient),
            None => prepare_with_quotient(),
        }?;
        Ok(finish(w, sa, sb, product, correction, false))
    }

    fn exclusive_g2(&self, words: &mut [[u64; 4]]) -> G2Projective {
        msm_affine::g2_query_exclusive(&self.b2, words, self.exclusive_g2_grouping)
    }
}

impl Key {
    /// Fuse inverse, Euler-weight and forward work within native CPU FFT leaves.
    /// Disabled by default. Portable and other quotient routes keep their ordinary path.
    pub fn set_euler_fft_leaf_fusion(&mut self, enabled: bool) {
        self.plan.set_euler_fft_leaf_fusion(enabled);
    }

    /// Whether the active backend supports four-query proving.
    pub fn supports_rerandomized_solve(&self) -> bool {
        if !self.has_gpu() {
            return true;
        }
        #[cfg(gpu)]
        if fast_ios_metal() {
            return true;
        }
        cfg!(all(gpu, target_os = "macos", target_arch = "aarch64"))
    }

    // Large CPU proofs benefit from keeping the shared pool focused on
    // proving. Metal and smaller CPU proofs benefit from starting verification early.
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[doc(hidden)]
    pub fn prefer_verification_overlap(&self) -> bool {
        (self.has_gpu() || self.domain <= (1 << 16)) && rayon::current_num_threads() > 1
    }

    /// Return a freshly randomized raw Groth16 proof on a supported backend.
    /// The ordinary five-part solving APIs retain their complete outputs.
    pub fn solve_rerandomized(
        &self,
        witness: &[u8],
        fixed: &groth16::PreparedFixedPoints,
    ) -> Result<Vec<u8>, String> {
        if !self.supports_rerandomized_solve() {
            return Err("the active backend cannot solve a randomized proof".into());
        }
        if !fixed.rerandomization_key_valid() {
            return Err("zero delta in proving key".into());
        }
        let parts = self.solve_inner(witness, true)?;
        if let Some(proof) = fixed.assemble_rerandomized(&parts) {
            return Ok(proof);
        }
        // A0 = identity: restore the complete ordinary prover and condition
        // its fresh output on A != identity, matching the fast distribution.
        let parts = self.solve_inner(witness, false)?;
        Ok(fixed.assemble_nonzero(&parts))
    }
}

impl Key {
    fn invalidate_solver(&mut self) {
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            self.wire_offsets = None;
        }
        self.proof_workspace = None;
        if self.basis_only {
            if let Some(plan) = self.solver.take() {
                self.basis_fallback = Some(Box::new(plan));
            }
        } else {
            self.solver = None;
        }
    }
    fn basis_plan_matches(&self, plan: &solver::Plan) -> bool {
        #[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
        if self.basis_only {
            return self
                .lagrange
                .as_ref()
                .is_some_and(|q| q.basis == Some(plan.basis_c_digest()));
        }
        let _ = plan;
        !self.basis_only
    }
}
#[cfg(all(not(target_arch = "wasm32"), target_endian = "little"))]
impl Key {
    pub fn build_cpu_basis(
        points: Points,
        domain_params: &[u8],
        image: &Image,
        product: (usize, usize),
        private: (usize, usize),
        public: (usize, usize),
        slots: Vec<usize>,
        stored_scale: Fr,
        digest: [u8; 32],
        solver_image: &[u8],
    ) -> Result<Self, String> {
        Self::build_cpu_basis_kind(
            points,
            domain_params,
            image,
            product,
            private,
            public,
            slots,
            stored_scale,
            digest,
            solver_image,
            false,
        )
    }
    pub fn build_cpu_euler_basis(
        points: Points,
        domain_params: &[u8],
        image: &Image,
        product: (usize, usize),
        private: (usize, usize),
        public: (usize, usize),
        slots: Vec<usize>,
        stored_scale: Fr,
        digest: [u8; 32],
        solver_image: &[u8],
    ) -> Result<Self, String> {
        Self::build_cpu_basis_kind(
            points,
            domain_params,
            image,
            product,
            private,
            public,
            slots,
            stored_scale,
            digest,
            solver_image,
            true,
        )
    }
    fn build_cpu_basis_kind(
        points: Points,
        domain_params: &[u8],
        image: &Image,
        product: (usize, usize),
        private: (usize, usize),
        public: (usize, usize),
        slots: Vec<usize>,
        stored_scale: Fr,
        digest: [u8; 32],
        solver_image: &[u8],
        euler: bool,
    ) -> Result<Self, String> {
        let mut key = Self::build_impl(points, domain_params, true)?;
        if stored_scale.is_zero() || key.domain < 8 {
            return Err("invalid CPU basis scale/domain".into());
        }
        if euler {
            key.plan.prepare_euler_basis();
        }
        let scale = if euler {
            stored_scale.inverse().unwrap()
        } else {
            key.plan.basis_query_scale() * stored_scale.inverse().unwrap()
        };
        let mut queries =
            lagrange::Queries::mapped_basis(image, product, private, public, slots, scale, digest)?;
        queries.euler = euler;
        let folded = queries.folded.as_ref().unwrap();
        let plan = solver::Plan::from_image_with_owner(solver_image, Some(image))?;
        if !plan.matches(key.lengths[0], key.lengths[1], key.lengths[2], key.domain)
            || product.1 / 64 != key.domain
            || folded.private.len() != key.lengths[2] + folded.slots.len()
            || public.1 / 64 != plan.n_public
            || plan.basis_c_digest() != digest
            || folded.slots.windows(2).any(|w| w[0] >= w[1])
            || folded
                .slots
                .iter()
                .any(|&i| i < plan.basis_wires() || i >= plan.value_count())
        {
            return Err("CPU basis dimensions or C representation mismatch".into());
        }
        let mut selected = std::collections::BTreeSet::new();
        plan.for_each_c(|_, wire, _| {
            if wire >= plan.basis_wires() {
                selected.insert(wire);
            }
        });
        if !selected.into_iter().eq(folded.slots.iter().copied()) {
            return Err("CPU basis selected slots mismatch".into());
        }
        key.k = Store::Owned(Vec::new());
        key.z = Store::Owned(Vec::new());
        key.lagrange = Some(queries);
        key.prepare_plan(&plan);
        key.solver = Some(plan);
        Ok(key)
    }
    fn basis_parts(
        &self,
        sa: &mut [Fr],
        sb: &mut [Fr],
        sk: &mut [Fr],
        a: Vec<Fr>,
        b: Vec<Fr>,
        c: Vec<Fr>,
        omit_b: bool,
    ) -> Result<Vec<u8>, String> {
        use rayon::prelude::*;
        let plan = self
            .solver
            .as_ref()
            .or(self.basis_fallback.as_deref())
            .ok_or("CPU basis requires its C metadata")?;
        let q = self.lagrange.as_ref().ok_or("missing CPU basis queries")?;
        let f = q.folded.as_ref().ok_or("missing folded query")?;
        // The arbitrary-parts API does not promise that C is the solver's C.
        // Restore K and H independently; do not assume a satisfied witness.
        let cp = plan.basis_c_private(sk);
        let correction_values = if q.euler {
            self.plan
                .euler_product_values(cp, vec![Fr::ONE; self.domain], None)
        } else {
            self.plan.coset_product(cp, vec![Fr::ONE; self.domain])
        };
        let correction = q.finish_product(msm_g1(&q.product, &correction_values));
        drop(correction_values);
        let mut product = if q.euler {
            self.plan
                .original_h_euler_values(self.plan.quotient(a, b, c))
        } else {
            let mut product = self.plan.coset_product(a, b);
            let c_coset = self.plan.coset_product(c, vec![Fr::ONE; self.domain]);
            product
                .par_iter_mut()
                .zip(c_coset.par_iter())
                .for_each(|(p, c)| *p -= *c);
            product
        };
        let ((ar, bs1), (kr, (hz, bs2))) = self.cpu_msms(
            sa,
            sb,
            sk,
            &mut product,
            &q.product,
            &f.private[..self.lengths[2]],
            false,
            omit_b,
        );
        let mut out = Vec::with_capacity(384);
        for p in [ar, bs1, kr + correction, q.finish_product(hz)] {
            output_g1(&mut out, p);
        }
        output_g2(&mut out, bs2);
        Ok(out)
    }
}

#[cfg(all(
    target_arch = "x86_64",
    target_os = "linux",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    feature = "ifma8",
    not(no_ifma)
))]
mod fused_canonical_gather;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
impl Key {
    /// Use a shared pool for the native IFMA canonical witness and FFT stages.
    /// The pool is used when it has fewer workers than the current Rayon pool.
    /// With `ZOPRO_CPU_SOLVER_FRONTEND=1`, the general CPU witness interpreter
    /// also uses this pool. `ZOPRO_CPU_FFT_PRODUCT_FRONTEND=1` dispatches the
    /// folded packed Euler product to this pool on native IFMA builds. The
    /// caller allocates its temporary output before dispatch. Passing `None`
    /// restores execution in the current pool.
    pub fn set_frontend_pool(&mut self, pool: Option<std::sync::Arc<FrontendThreadPool>>) {
        self.frontend_pool = pool;
    }

    /// Allow the final windows of an owned, grouped G2 query to use the
    /// attached frontend pool. Dispatch requires a smaller pool and sufficient
    /// additional proof workspace. Disabled by default.
    pub fn set_grouped_g2_on_frontend(&mut self, enabled: bool) {
        self.grouped_g2_on_frontend = enabled;
    }

    /// Run the native fused K/H path's G1 A query directly on the attached
    /// frontend pool when it is smaller and extra scratch is admitted.
    /// Disabled by default.
    pub fn set_raw_g1_a_on_frontend(&mut self, enabled: bool) {
        self.raw_g1_a_on_frontend = enabled;
    }

    /// Attach a caller-owned pool on disjoint cores for the small exclusive G2 query.
    /// With quotient overlap enabled, G2 starts once the canonical scalars are ready.
    /// The prover reserves scratch for each request before dispatching.
    pub fn set_exclusive_g2_pool(
        &mut self,
        pool: Option<std::sync::Arc<FrontendThreadPool>>,
        overlap_quotient: bool,
    ) {
        #[cfg(all(
            zopro_resource_schedule,
            zopro_g2_horner_buckets,
            feature = "ifma8",
            target_os = "linux",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            self.exclusive_g2_pool = pool.and_then(|workers| {
                let scratch_bytes = workers.install(|| {
                    msm_affine::workspace::auxiliary_g2(self.b2.len(), rayon::current_num_threads())
                })?;
                Some(ExclusiveG2Pool {
                    workers,
                    scratch_bytes,
                    overlap_quotient,
                })
            });
        }
        #[cfg(not(all(
            zopro_resource_schedule,
            zopro_g2_horner_buckets,
            feature = "ifma8",
            target_os = "linux",
            target_arch = "x86_64",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        let _ = (pool, overlap_quotient);
    }

    /// Set a caller-owned CPU solver pool independently of the frontend.
    pub fn set_solver_pool(&mut self, pool: Option<std::sync::Arc<FrontendThreadPool>>) {
        self.solver_pool = pool;
    }

    /// Reduce eligible grouped query points in parallel tiles under the CPU scratch budget.
    pub fn set_grouped_point_tiling(&mut self, enabled: bool) {
        #[cfg(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64"))]
        {
            self.grouped_point_tiling = enabled;
        }
        #[cfg(not(all(zopro_resource_schedule, target_os = "linux", target_arch = "x86_64")))]
        let _ = enabled;
    }

    /// Use the smaller caller-attached pool for the large native G1 K query.
    /// Requires sufficient scratch under the proof workspace policy.
    /// Disabled by default.
    pub fn set_g1_k_on_frontend(&mut self, enabled: bool) {
        self.g1_k_on_frontend = enabled;
    }

    /// Prepare distinct scalars on the current worker for medium grouped B2 queries.
    /// This also reserves initial table space for one class per three B2 terms.
    /// The workspace policy still governs admission. Disabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_grouped_b2_serial_preparation(&mut self, enabled: bool) {
        self.grouped_b2_serial_preparation = enabled;
    }

    /// Use exact solver-derived Poseidon origins in medium native grouped B2.
    /// Retains a bounded index plan under the workspace policy. Each proof
    /// checks equality before reusing a scalar class. Disabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_grouped_b2_poseidon_origins(&mut self, enabled: bool) {
        self.grouped_b2_poseidon_origins = enabled;
        #[cfg(zopro_resource_schedule)]
        if !enabled {
            if let Some(plan) = self.solver.as_mut() {
                plan.clear_b2_origins();
            }
        }
    }

    /// Control scalar grouping for the native exclusive G2 evaluator.
    /// Grouping is enabled by default.
    pub fn set_exclusive_g2_grouping(&mut self, enabled: bool) {
        self.exclusive_g2_grouping = enabled;
    }
}

impl Key {
    /// Prepare corrections from public offsets for this CPU key.
    /// An unsupported key or insufficient preparation budget disables the
    /// option and returns `Ok(false)`. An invalid file returns an error.
    pub fn prepare_public_wire_offsets(&mut self, bytes: &[u8]) -> Result<bool, &'static str> {
        #[cfg(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        ))]
        {
            let prepared = public_wire_offsets::prepare(self, bytes)?;
            let enabled = prepared.is_some();
            self.wire_offsets = prepared.map(Box::new);
            Ok(enabled)
        }
        #[cfg(not(all(
            zopro_resource_schedule,
            feature = "ifma8",
            target_arch = "x86_64",
            target_os = "linux",
            target_feature = "avx512ifma",
            target_feature = "avx512vl",
            target_feature = "avx512dq",
            not(no_ifma)
        )))]
        {
            let _ = bytes;
            Ok(false)
        }
    }
}
