#![cfg_attr(
    all(target_arch = "wasm32", target_feature = "simd128"),
    feature(asm_experimental_arch)
)]
use ark_bn254::{Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AdditiveGroup, CurveGroup};
use ark_ff::{BigInt, Field};
use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
mod decompress;
pub mod fft;
mod msm;
#[cfg(not(target_arch = "wasm32"))]
mod selftest;
mod simd;
mod solver;
pub mod groth16;

// Stage times as RINGIL_TRACE log lines (`--features trace`): the native
// bench prints them, a phone's bench puts them in the device log.
fn trace(label: &str, started: std::time::Instant) -> std::time::Instant {
    #[cfg(all(feature = "trace", not(target_arch = "wasm32")))]
    log::info!("RINGIL_TRACE {label} {:.1} ms", started.elapsed().as_secs_f64() * 1000.0);
    #[cfg(not(all(feature = "trace", not(target_arch = "wasm32"))))]
    let _ = (label, started);
    std::time::Instant::now()
}
pub use decompress::{decompress_g1, decompress_g2};

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
pub fn solver_image_version() -> u32 {
    solver::IMAGE_VERSION as u32
}

/// Where the G2 MSM runs when there is a GPU: RINGIL_G2=gpu|cpu, else the
/// GPU where it is of Apple's ninth family or later (a Mac's, an A19's:
/// several times faster than the cores at it) and the CPU on older phones,
/// whose GPUs are slower at G2 than their cores and whose G2 kernels take
/// minutes to compile.
#[cfg(gpu)]
fn g2_on_gpu() -> bool {
    match std::env::var("RINGIL_G2").as_deref() {
        Ok("gpu") => true,
        Ok("cpu") => false,
        _ => ringil_metal::fast_gpu(),
    }
}

#[cfg(gpu)]
struct Gpu {
    a: std::sync::Mutex<ringil_metal::G1Msm>,
    b: std::sync::Mutex<ringil_metal::G1Msm>,
    k: std::sync::Mutex<ringil_metal::G1Msm>,
    z: std::sync::Mutex<ringil_metal::G1Msm>,
    b2: Option<std::sync::Mutex<ringil_metal::G2Msm>>,
    quotient: std::sync::Mutex<ringil_metal::Quotient>,
}
#[cfg(gpu_wgpu)]
struct Gpu {
    a: std::sync::Mutex<ringil_wgpu::G1Msm>,
    b: std::sync::Mutex<ringil_wgpu::G1Msm>,
    k: std::sync::Mutex<ringil_wgpu::G1Msm>,
    z: std::sync::Mutex<ringil_wgpu::G1Msm>,
    quotient: std::sync::Mutex<ringil_wgpu::Quotient>,
}
/// A GPU taking the key's G1 vectors one at a time, so that the key keeps
/// no copy of what the GPU holds; G2 stays on the CPU, where WGSL's field
/// arithmetic cannot beat it.
#[cfg(gpu_wgpu)]
struct GpuBuilder {
    started: std::time::Instant,
    gpu: std::sync::Arc<ringil_wgpu::Gpu>,
    quotient: ringil_wgpu::Quotient,
    g1: [Option<ringil_wgpu::G1Msm>; 4],
}
#[cfg(gpu_wgpu)]
impl GpuBuilder {
    /// None with RINGIL_GPU=0 or without a GPU; on Android only with
    /// RINGIL_GPU=1: measured on a Galaxy A16 (Mali-G57), A56 (Xclipse 940)
    /// and S25 (Adreno 830), the WGSL MSMs took minutes to compile and ran
    /// 3–5× slower than the phone's own cores, or did not link at all.
    fn open(plan: &fft::Plan) -> Option<Self> {
        let wanted = std::env::var("RINGIL_GPU").ok();
        if wanted.as_deref() == Some("0") || (cfg!(target_os = "android") && wanted.as_deref() != Some("1")) {
            return None;
        }
        let started = std::time::Instant::now();
        let gpu = std::sync::Arc::new(ringil_wgpu::Gpu::new().ok()?);
        let (roots, inverse_roots, forward, reverse) = plan.tables()?;
        let quotient = ringil_wgpu::Quotient::new(gpu.clone(), plan.size(), &roots, &inverse_roots, &forward, &reverse).ok()?.0;
        Some(Self { started, gpu, quotient, g1: [None, None, None, None] })
    }
    fn g1(&mut self, which: usize, bases: &[G1Affine]) -> Result<(), String> {
        self.g1[which] = Some(ringil_wgpu::G1Msm::new(self.gpu.clone(), bases)?.0);
        Ok(())
    }
    fn g2(&mut self, _bases: &[G2Affine]) -> Result<bool, String> {
        Ok(false)
    }
    fn finish(self) -> Gpu {
        let [a, b, k, z] = self.g1.map(|msm| std::sync::Mutex::new(msm.expect("every G1 vector uploaded")));
        trace("gpu-upload", self.started);
        Gpu { a, b, k, z, quotient: std::sync::Mutex::new(self.quotient) }
    }
}

/// Hands a decoded vector to the GPU and keeps nothing of it, or keeps it
/// when there is no GPU. A GPU that cannot take the key fails the build:
/// the caller then proves another way.
#[cfg(gpu_any)]
fn upload_g1(gpu: &mut Option<GpuBuilder>, which: usize, bases: Vec<G1Affine>) -> Result<Vec<G1Affine>, &'static str> {
    match gpu {
        Some(builder) => {
            builder.g1(which, &bases).map_err(|_| "the GPU could not take the key")?;
            #[cfg(feature = "fullcheck")]
            {
                let msm = builder.g1[which].as_mut().expect("uploaded");
                selftest::full_g1(["a", "b", "k", "z"][which], &bases, |scalars| msm.msm(scalars).map(|r| r.0));
            }
            Ok(Vec::new())
        }
        None => Ok(bases),
    }
}
#[cfg(gpu_any)]
fn upload_g2(gpu: &mut Option<GpuBuilder>, bases: Vec<G2Affine>) -> Result<Vec<G2Affine>, &'static str> {
    #[cfg(feature = "fullcheck")]
    selftest::full_g2(&bases);
    match gpu {
        Some(builder) => {
            let taken = builder.g2(&bases).map_err(|_| "the GPU could not take the key")?;
            Ok(if taken { Vec::new() } else { bases })
        }
        None => Ok(bases),
    }
}
#[cfg(gpu_any)]
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
    started: std::time::Instant,
    quotient: ringil_metal::Quotient,
    g1: [Option<ringil_metal::G1Msm>; 4],
    b2: Option<ringil_metal::G2Msm>,
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
        let started = std::time::Instant::now();
        let (roots, inverse_roots, forward, reverse) = plan.tables()?;
        let quotient = ringil_metal::Quotient::new(plan.size(), &roots, &inverse_roots, &forward, &reverse).ok()?.0;
        let scratch = ringil_metal::Scratch::new().ok()?;
        Some(Self { started, quotient, g1: [None, None, None, None], b2: None, scratch })
    }
    fn g1(&mut self, which: usize, bases: &[G1Affine]) -> Result<(), String> {
        self.g1[which] = Some(ringil_metal::G1Msm::new_in(bases, self.scratch.clone())?.0);
        Ok(())
    }
    /// Whether the GPU took G2: a Mac's does, a phone's leaves it to the CPU.
    fn g2(&mut self, bases: &[G2Affine]) -> Result<bool, String> {
        if !g2_on_gpu() {
            return Ok(false);
        }
        self.b2 = Some(ringil_metal::G2Msm::new_in(bases, self.scratch.clone())?.0);
        Ok(true)
    }
    fn finish(self) -> Gpu {
        let [a, b, k, z] = self.g1.map(|msm| std::sync::Mutex::new(msm.expect("every G1 vector uploaded")));
        trace("gpu-upload", self.started);
        #[cfg(feature = "trace")]
        log::info!("RINGIL_TRACE gpu-allocated {} MB", ringil_metal::allocated_bytes() >> 20);
        Gpu { a, b, k, z, b2: self.b2.map(std::sync::Mutex::new), quotient: std::sync::Mutex::new(self.quotient) }
    }
}

#[wasm_bindgen]
pub struct Key {
    #[cfg(gpu_any)]
    gpu: Option<Gpu>,
    /// Whether the GPU's quotient agreed with the CPU's at load.
    #[cfg(gpu_any)]
    gpu_quotient: bool,
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
        let started = std::time::Instant::now();
        let plan = fft::Plan::new(n, params[0], params[1], den);
        trace("plan", started);
        let lengths = [a.len() / 64, b.len() / 64, k.len() / 64];
        // One vector at a time: decoded, handed to the GPU, dropped. A GPU
        // that cannot take the key (it failed to build a kernel, it ran out
        // of memory) is let go, and the key is decoded again for the CPU.
        let started = std::time::Instant::now();
        #[cfg(gpu_any)]
        let mut gpu = GpuBuilder::open(&plan);
        #[cfg(not(gpu_any))]
        let mut gpu: Option<()> = None;
        trace("gpu-open", started);
        #[cfg(not(gpu_any))]
        let (upload_g1, upload_g2) = (|_: &mut Option<()>, _: usize, v| Ok::<_, &'static str>(v), |_: &mut Option<()>, v| Ok::<_, &'static str>(v));
        #[cfg(not(target_arch = "wasm32"))]
        let head = |n: usize| n.min(selftest::HEAD);
        #[cfg(target_arch = "wasm32")]
        let head = |_: usize| 0;
        let decode = |gpu: &mut Option<_>| -> Result<_, &'static str> {
            let started = std::time::Instant::now();
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
                        let verdict = selftest::gpu(&mut finished, &plan, &a_head, &b2_head, lengths[0], lengths[1]);
                        if !verdict.msm {
                            drop(finished);
                            // With Metal, once more with the shaders compiled at
                            // the size level, where aeglos found the default
                            // level's compiler at fault.
                            #[cfg(gpu)]
                            if ringil_metal::prefer_smaller_code() {
                                trace("gpu-retry-smaller-code", std::time::Instant::now());
                                gpu = GpuBuilder::open(&plan);
                            }
                            continue;
                        }
                        gpu_quotient = verdict.quotient;
                        built_gpu = Some(finished);
                    }
                    let _ = (a_head, b2_head);
                    break (a, b, k, z, b2);
                }
                Err(_) if gpu.is_some() => gpu = None,
                Err(e) => return Err(e),
            }
        };
        Ok(Self {
            #[cfg(gpu_any)]
            gpu: built_gpu,
            #[cfg(gpu_any)]
            gpu_quotient,
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
        let started = std::time::Instant::now();
        let parsed = solver::Plan::new(program, coefficients);
        trace("solver-plan", started);
        if let Ok(plan) = parsed {
            if plan.matches(self.lengths[0], self.lengths[1], self.lengths[2], self.domain.size()) {
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
        let started = std::time::Instant::now();
        self.solver = None;
        if !self.commitments.is_empty() {
            return false;
        }
        let parsed = solver::Plan::from_image(image);
        trace("solver-image", started);
        if let Ok(plan) = parsed {
            if plan.matches(self.lengths[0], self.lengths[1], self.lengths[2], self.domain.size()) {
                self.solver = Some(plan);
                return true;
            }
        }
        false
    }
    pub fn solve_parts(&self, witness: &[u8]) -> Result<Vec<u8>, JsError> {
        self.solve(witness).map_err(|error| JsError::new(&error))
    }
    /// `solve_parts` for hosts outside JavaScript, with a plain error.
    pub fn solve(&self, witness: &[u8]) -> Result<Vec<u8>, String> {
        let plan = self.solver.as_ref().ok_or("no prepared solver")?;
        if !witness.len().is_multiple_of(32) {
            return Err("invalid witness encoding".into());
        }
        let started = std::time::Instant::now();
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
        self.compute_parts(&sa, &sb, &sk, frs(a), frs(b), frs(c)).map_err(|e| JsError::new(&e))
    }
}
impl Key {
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
            let started = std::time::Instant::now();
            // The sorts run on the pool, so a CPU G2 MSM starts only once
            // they are done: otherwise it starves them for its whole run.
            let (sorted_tx, sorted_rx) = std::sync::mpsc::channel::<()>();
            let (h, (ar, bs1, kr, bs2)) = std::thread::scope(|scope| -> Result<_, String> {
                let msms = scope.spawn(move || -> Result<_, String> {
                    // Each MSM is submitted as soon as it is sorted, so the
                    // host sorts the next while the GPU runs it; G2 B shares
                    // G1 B's scalars, and so its sorted runs.
                    let (mut a, mut b, mut k) = (gpu.a.lock().unwrap(), gpu.b.lock().unwrap(), gpu.k.lock().unwrap());
                    let pa = a.start(sa);
                    let pb = b.start(sb);
                    let mut b2 = gpu.b2.as_ref().map(|b2| b2.lock().unwrap());
                    let pb2 = b2.as_mut().map(|b2| b2.start_sorted_as(&*b));
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
                let cpu_b2 = gpu.b2.is_none().then(|| {
                    scope.spawn(move || {
                        let _ = sorted_rx.recv();
                        let started = std::time::Instant::now();
                        let bs2 = msm_g2(&self.b2, sb);
                        trace("cpu-msm-b2", started);
                        bs2
                    })
                });
                let started = std::time::Instant::now();
                let h = if !self.gpu_quotient || std::env::var("RINGIL_GPU_QUOTIENT").is_ok_and(|v| v == "0") {
                    self.quotient(a, b, c)
                } else {
                    gpu.quotient.lock().unwrap().quotient(&a, &b, &c)?
                };
                trace("quotient", started);
                let (ar, bs1, kr, bs2) = msms.join().expect("gpu msms")?;
                let bs2 = bs2.unwrap_or_else(|| cpu_b2.expect("a G2 MSM").join().expect("cpu msm"));
                Ok((h, (ar, bs1, kr, bs2)))
            })?;
            let started = trace("quotient+gpu-msms", started);
            let (hz, t) = gpu.z.lock().unwrap().msm(&h)?;
            gpu_trace("z", t.sort_ms, t.gpu_ms);
            trace("gpu-msm-z", started);
            let mut out = Vec::with_capacity(384);
            for p in [ar, bs1, kr, hz] {
                output_g1(&mut out, p)
            }
            output_g2(&mut out, bs2);
            return Ok(out);
        }
        #[cfg(gpu_wgpu)]
        if let Some(gpu) = &self.gpu {
            // The G1 MSMs and the quotient on the GPU, one after another on
            // a thread of their own; the pool computes G2 meanwhile.
            let started = std::time::Instant::now();
            let (sorted_tx, sorted_rx) = std::sync::mpsc::channel::<()>();
            let ((ar, bs1, kr, h), bs2) = std::thread::scope(|scope| -> Result<_, String> {
                let gpu_work = scope.spawn(move || -> Result<_, String> {
                    // The three MSMs are sorted and submitted back to back,
                    // and only then does the CPU G2 MSM take the pool.
                    let (mut ma, mut mb, mut mk) = (gpu.a.lock().unwrap(), gpu.b.lock().unwrap(), gpu.k.lock().unwrap());
                    let (pa, pb, pk) = (ma.start(sa), mb.start(sb), mk.start(sk));
                    drop(sorted_tx);
                    let (ar, t) = ma.finish(pa)?;
                    gpu_trace("a", t.sort_ms, t.gpu_ms);
                    let (bs1, t) = mb.finish(pb)?;
                    gpu_trace("b", t.sort_ms, t.gpu_ms);
                    let (kr, t) = mk.finish(pk)?;
                    gpu_trace("k", t.sort_ms, t.gpu_ms);
                    let started = std::time::Instant::now();
                    let h = if self.gpu_quotient { gpu.quotient.lock().unwrap().quotient(&a, &b, &c)? } else { self.quotient(a, b, c) };
                    trace("quotient", started);
                    Ok((ar, bs1, kr, h))
                });
                let _ = sorted_rx.recv();
                let started = std::time::Instant::now();
                let bs2 = msm_g2(&self.b2, sb);
                trace("cpu-msm-b2", started);
                Ok((gpu_work.join().expect("gpu work")?, bs2))
            })?;
            let started = trace("quotient+gpu-msms", started);
            let (hz, t) = gpu.z.lock().unwrap().msm(&h)?;
            gpu_trace("z", t.sort_ms, t.gpu_ms);
            trace("gpu-msm-z", started);
            let mut out = Vec::with_capacity(384);
            for p in [ar, bs1, kr, hz] {
                output_g1(&mut out, p)
            }
            output_g2(&mut out, bs2);
            return Ok(out);
        }
        let started = std::time::Instant::now();
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
            for (label, bases, scalars) in [("msm-a", &self.a, sa), ("msm-b", &self.b, sb), ("msm-k", &self.k, sk), ("msm-z", &self.z, &h[..])] {
                let started = std::time::Instant::now();
                let _ = msm_g1(bases, scalars);
                trace(label, started);
            }
            let started = std::time::Instant::now();
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
    } else {
        msm::msm(bases, scalars)
    }
}
pub fn msm_g2(bases: &[G2Affine], scalars: &[Fr]) -> G2Projective {
    if bases.is_empty() {
        G2Projective::ZERO
    } else {
        msm::msm(bases, scalars)
    }
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
