//! Proves one shape from the files TestZoproDump wrote (head, a, b, k, z,
//! b2, program, coefficients, witness) and prints the stage times, then
//! writes the proof as Ar, Krs, Bs in gnark's raw big-endian encoding for
//! the Go referee.
//!
//!   RAYON_NUM_THREADS=4 zopro-bench <dump dir> [rounds]
//!   zopro-bench stress <cycles> <dump dir>...
use std::{env, fs, path::Path, time::Instant};

use zopro_bn254::Fr;
use zopro_core::{decode_g1, decode_g2, Key};

fn read(dir: &Path, name: &str) -> Vec<u8> {
    fs::read(dir.join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

/// The kernel's traces, to stderr as TRACE lines.
struct Stderr;
impl log::Log for Stderr {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        eprintln!(
            "{}",
            record.args().to_string().replace("ZOPRO_TRACE", "TRACE")
        );
    }
    fn flush(&self) {}
}

/// The bench, by its arguments (`zopro-bench <dump dir> [rounds]`, or a mode
/// and its arguments).
pub fn run() {
    let _ = log::set_logger(&Stderr).map(|()| log::set_max_level(log::LevelFilter::Info));
    let dir = env::args().nth(1).expect("the dump directory");
    if dir == "field" {
        return field();
    }
    if dir == "gpu" {
        return gpu(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    if dir == "wgpu" {
        return wgpu_bench(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    if dir == "pack" {
        let args: Vec<String> = env::args().collect();
        return pack_dump(Path::new(&args[2]), &args[3], Path::new(&args[4]));
    }
    if dir == "cpu-msm" {
        return cpu_msm(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    if dir == "parts-diff" {
        return parts_diff(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    if dir == "scalars" {
        return scalar_profile(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    if dir == "stress" {
        let cycles = env::args().nth(2).expect("cycles").parse().expect("cycles");
        let dirs: Vec<String> = env::args().skip(3).collect();
        return stress(cycles, &dirs);
    }
    #[cfg(target_vendor = "apple")]
    if dir == "metal-quotient" {
        return metal_quotient(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    if dir == "wgpu-quotient" {
        return wgpu_quotient(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    let dir = Path::new(&dir);
    let rounds: usize = env::args().nth(2).map(|v| v.parse().unwrap()).unwrap_or(5);
    let head = zopro_core::dump::Head::read(dir).expect("dump head");
    let fixed = head.fixed_points();
    let domain = &head.domain[..];
    let (a, b, k, z, b2) = (
        read(dir, "a"),
        read(dir, "b"),
        read(dir, "k"),
        read(dir, "z"),
        read(dir, "b2"),
    );
    // What the host holds anyway (gnark's vectors, which the app hands over
    // borrowed): the rest of the peak is the kernel's.
    let inputs_mb = footprint_mb();

    let started = Instant::now();
    let mut key = Key::new(&a, &b, &k, &z, &b2, domain).expect("key");
    let key_ms = started.elapsed().as_millis();
    let started = Instant::now();
    // As the mobile SDK does: the solver from its cached image beside the
    // dump when there is one (`solver-image`), else parsed and cached.
    let cached = dir.join("solver-image");
    match fs::read(&cached) {
        Ok(image) => assert!(key.set_solver_image(&image), "solver image"),
        Err(_) => {
            assert!(
                key.set_solver(&read(dir, "program"), &read(dir, "coefficients")),
                "solver plan"
            );
            if env::var("ZOPRO_CACHE_SOLVER").is_ok_and(|v| v == "1") {
                fs::write(&cached, key.solver_image().expect("solver image")).unwrap();
            }
        }
    }
    let plan_ms = started.elapsed().as_millis();
    let witness = read(dir, "witness");

    let mut prove = Vec::new();
    let mut proof = Vec::new();
    for _ in 0..rounds {
        let started = Instant::now();
        let parts = key.solve(&witness).expect("solve and parts");
        proof = zopro_core::groth16::assemble(&parts, &fixed);
        prove.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    fs::write(dir.join("proof"), &proof).unwrap();
    let mut sorted = prove.clone();
    sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let threads = env::var("RAYON_NUM_THREADS").unwrap_or_default();
    println!(
        "ZOPRO rust {} threads={threads} key_ms={key_ms} plan_ms={plan_ms} prove_ms_min={:.0} prove_ms_median={:.0} rounds={rounds} inputs_mb={inputs_mb} peak_footprint_mb={}",
        dir.file_name().unwrap().to_string_lossy(),
        sorted[0],
        sorted[sorted.len() / 2],
        peak_footprint_mb()
    );
}

/// Nanoseconds per Fq multiplication, squaring, and Fq2 multiplication on
/// one thread: a dependent chain of ten million.
/// Uniform elements from the system's randomness.
trait Random: Sized {
    fn random() -> Self;
}
fn random_bytes() -> [u8; 64] {
    let mut bytes = [0u8; 64];
    getrandom::getrandom(&mut bytes).expect("the system's randomness");
    bytes
}
impl Random for Fr {
    fn random() -> Self {
        Fr::from_random_bytes(&random_bytes())
    }
}
impl Random for zopro_bn254::Fq {
    fn random() -> Self {
        zopro_bn254::Fq::from_random_bytes(&random_bytes())
    }
}
impl Random for zopro_bn254::Fq2 {
    fn random() -> Self {
        zopro_bn254::Fq2::new(Random::random(), Random::random())
    }
}

fn field() {
    use zopro_bn254::{Fq, Fq2};
    let (a, b) = (Fq::random(), Fq::random());
    let n = 10_000_000u32;
    let started = Instant::now();
    let mut x = a;
    for _ in 0..n {
        x *= b;
    }
    let mul = started.elapsed().as_nanos() as f64 / n as f64;
    let started = Instant::now();
    let mut y = a;
    for _ in 0..n {
        y = y.square();
    }
    let square = started.elapsed().as_nanos() as f64 / n as f64;
    let (a2, b2) = (Fq2::random(), Fq2::random());
    let started = Instant::now();
    let mut z = a2;
    for _ in 0..n {
        z *= b2;
    }
    let mul2 = started.elapsed().as_nanos() as f64 / n as f64;
    // Four independent chains: the throughput an out-of-order core reaches
    // when the products do not wait on each other.
    let started = Instant::now();
    let mut w = [a2, b2, a2 + b2, a2 - b2];
    for _ in 0..n / 4 {
        for v in w.iter_mut() {
            *v *= b2;
        }
    }
    let mul2_independent = started.elapsed().as_nanos() as f64 / n as f64;
    let started = Instant::now();
    let mut u = [a, b, a + b, a - b];
    for _ in 0..n / 4 {
        for v in u.iter_mut() {
            *v *= b;
        }
    }
    let mul_independent = started.elapsed().as_nanos() as f64 / n as f64;
    println!(
        "ZOPRO field fq_mul_ns={mul:.2} fq_square_ns={square:.2} fq2_mul_ns={mul2:.2} independent: fq_mul_ns={mul_independent:.2} fq2_mul_ns={mul2_independent:.2} {}",
        (x + y + u[0]).0[0] ^ (z + w[0]).c0.0[0]
    );
}

/// The G1 MSMs of a dumped shape's Z and B vectors with random scalars, on
/// the CPU (RAYON_NUM_THREADS) and on the GPU, checked against each other.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn gpu(dir: &Path) {
    // Witness scalars are skewed: many wires are 0, 1 or small, so a few
    // buckets receive most points. "skewed" is a quarter ones, a quarter
    // small, the rest random.
    let skewed = |n: usize| -> Vec<Fr> {
        (0..n)
            .map(|i| match i % 4 {
                0 => Fr::from(1u64),
                1 => Fr::from((i % 65536) as u64),
                _ => Fr::random(),
            })
            .collect()
    };
    for name in ["b", "b skewed", "z", "b2", "b2 skewed"] {
        let file = name.split(' ').next().unwrap();
        let skew = name.ends_with("skewed");
        if file == "b2" {
            let bases = decode_g2(&read(dir, file));
            let scalars = if skew {
                skewed(bases.len())
            } else {
                (0..bases.len()).map(|_| Fr::random()).collect()
            };
            gpu_one(name, &bases, &scalars, zopro_core::msm_g2);
        } else {
            let bases = decode_g1(&read(dir, file));
            let scalars = if skew {
                skewed(bases.len())
            } else {
                (0..bases.len()).map(|_| Fr::random()).collect()
            };
            gpu_one(name, &bases, &scalars, zopro_core::msm_g1);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn gpu_one<C: zopro_metal::Curve>(
    name: &str,
    bases: &[C::Affine],
    scalars: &[Fr],
    cpu_msm: fn(&[C::Affine], &[Fr]) -> C,
) {
    {
        let started = Instant::now();
        let (mut msm, compile_ms) = zopro_metal::Msm::<C>::new(bases).expect("gpu msm");
        let upload_ms = started.elapsed().as_secs_f64() * 1000.0 - compile_ms;
        let started = Instant::now();
        let cpu = cpu_msm(bases, scalars);
        let cpu_ms = started.elapsed().as_secs_f64() * 1000.0;
        // Warm, then timed.
        let _ = msm.msm(scalars);
        let mut best: Option<(zopro_metal::Timings, f64)> = None;
        let mut result = cpu;
        for _ in 0..3 {
            let started = Instant::now();
            let (point, timings) = msm.msm(scalars).expect("gpu msm");
            let total = started.elapsed().as_secs_f64() * 1000.0;
            result = point;
            if best.is_none_or(|(_, t)| total < t) {
                best = Some((timings, total));
            }
        }
        let (t, total) = best.unwrap();
        println!("ZOPRO gpu-profile \"{name}\" {}", msm.profile(scalars));
        assert_eq!(
            result,
            cpu,
            "{name}: the GPU MSM differs from the CPU"
        );
        println!(
            "ZOPRO gpu \"{name}\" points={} device=\"{}\" compile_ms={compile_ms:.0} upload_ms={upload_ms:.0} cpu_ms={cpu_ms:.0} gpu_ms={total:.0} sort_ms={:.0} device_ms={:.0} combine_ms={:.1}",
            bases.len(),
            msm.device_name(),
            t.sort_ms,
            t.gpu_ms,
            t.combine_ms
        );
    }
}
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn gpu(_: &Path) {
    panic!("the GPU bench needs Metal");
}

/// The MSMs of a dumped shape's B, Z and G2 B through wgpu, random and
/// skewed scalars, checked against the CPU.
fn wgpu_bench(dir: &Path) {
    let gpu = std::sync::Arc::new(zopro_wgpu::Gpu::new().expect("wgpu"));
    let scalars_for = |n: usize, skewed: bool| -> Vec<Fr> {
        (0..n)
            .map(|i| match (skewed, i % 4) {
                (true, 0) => Fr::from(1u64),
                (true, 1) => Fr::from((i % 65536) as u64),
                _ => Fr::random(),
            })
            .collect()
    };
    for name in ["b", "b skewed", "z", "b2", "b2 skewed"] {
        let file = name.split(' ').next().unwrap();
        let skewed = name.ends_with("skewed");
        if file == "b2" {
            let bases = decode_g2(&read(dir, file));
            let scalars = scalars_for(bases.len(), skewed);
            wgpu_one(&gpu, name, &bases, &scalars, zopro_core::msm_g2);
        } else {
            let bases = decode_g1(&read(dir, file));
            let scalars = scalars_for(bases.len(), skewed);
            wgpu_one(&gpu, name, &bases, &scalars, zopro_core::msm_g1);
        }
    }
}

fn wgpu_one<C: zopro_wgpu::Curve>(
    gpu: &std::sync::Arc<zopro_wgpu::Gpu>,
    name: &str,
    bases: &[C::Affine],
    scalars: &[Fr],
    cpu_msm: fn(&[C::Affine], &[Fr]) -> C,
) {
    let started = Instant::now();
    let (mut msm, compile_ms) = zopro_wgpu::Msm::<C>::new(gpu.clone(), bases).expect("wgpu msm");
    let upload_ms = started.elapsed().as_secs_f64() * 1000.0 - compile_ms;
    let started = Instant::now();
    let cpu = cpu_msm(bases, scalars);
    let cpu_ms = started.elapsed().as_secs_f64() * 1000.0;
    let _ = msm.msm(scalars);
    // Only the timed runs' kernel times (ZOPRO_WGPU_PROFILE=1).
    let _ = zopro_wgpu::profile::report();
    let mut best: Option<(zopro_wgpu::Timings, f64, C)> = None;
    for _ in 0..3 {
        let started = Instant::now();
        let (point, timings) = msm.msm(scalars).expect("gpu msm");
        let total = started.elapsed().as_secs_f64() * 1000.0;
        if best.as_ref().is_none_or(|(_, t, _)| total < *t) {
            best = Some((timings, total, point));
        }
    }
    let (t, total, point) = best.unwrap();
    assert_eq!(
        point,
        cpu,
        "{name}: the wgpu MSM differs from the CPU"
    );
    println!(
        "ZOPRO wgpu \"{name}\" points={} device=\"{}\" compile_ms={compile_ms:.0} upload_ms={upload_ms:.0} cpu_ms={cpu_ms:.0} gpu_ms={total:.0} sort_ms={:.0} device_ms={:.0} combine_ms={:.1}",
        bases.len(),
        gpu.name,
        t.sort_ms,
        t.gpu_ms,
        t.combine_ms
    );
    for line in zopro_wgpu::profile::report().lines() {
        println!("ZOPRO wgpu-profile \"{name}\" {line} (3 runs)");
    }
}

/// The quotient of random evaluations through Metal against the CPU plan's,
/// on the dumped shape's domain.
#[cfg(target_vendor = "apple")]
fn metal_quotient(dir: &Path) {
    let head = zopro_core::dump::Head::read(dir).expect("dump head");
    let params = zopro_core::decode_frs(&head.domain);
    let (generator, coset) = (params[0], params[1]);
    let n = read(dir, "z").len() / 64 + 1;
    let den = zopro_core::fft::denominator(n, coset).expect("denominator");
    let plan = zopro_core::fft::Plan::new(n, generator, coset, den);
    let (roots, inverse_roots, forward, reverse) = plan.tables().expect("tables");
    let (mut quotient, _) =
        zopro_metal::Quotient::new(n, &roots, &inverse_roots, &forward, &reverse)
            .expect("quotient");
    let m = n - n / 16;
    let (a, b, c): (Vec<Fr>, Vec<Fr>, Vec<Fr>) = (
        (0..m).map(|_| Fr::random()).collect(),
        (0..m).map(|_| Fr::random()).collect(),
        (0..m).map(|_| Fr::random()).collect(),
    );
    let expected = plan.quotient(a.clone(), b.clone(), c.clone());
    let _ = quotient.quotient(&a, &b, &c);
    let mut times = Vec::new();
    let mut actual = Vec::new();
    for _ in 0..5 {
        let started = Instant::now();
        actual = quotient.quotient(&a, &b, &c).expect("gpu quotient");
        times.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    times.sort_by(f64::total_cmp);
    let exact = actual == expected;
    println!(
        "ZOPRO metal-quotient n={n} gpu_ms_median={:.1} gpu_ms_min={:.1} exact={exact}",
        times[2], times[0]
    );
    assert!(exact, "the Metal quotient differs from the CPU plan");
}

/// The quotient of random evaluations through wgpu against the CPU plan's,
/// on the dumped shape's domain.
fn wgpu_quotient(dir: &Path) {
    let head = zopro_core::dump::Head::read(dir).expect("dump head");
    let params = zopro_core::decode_frs(&head.domain);
    let (generator, coset) = (params[0], params[1]);
    let n = read(dir, "z").len() / 64 + 1;
    let den = zopro_core::fft::denominator(n, coset).expect("denominator");
    let plan = zopro_core::fft::Plan::new(n, generator, coset, den);
    let (roots, inverse_roots, forward, reverse) = plan.tables().expect("tables");
    let gpu = std::sync::Arc::new(zopro_wgpu::Gpu::new().expect("wgpu"));
    let started = Instant::now();
    let (mut quotient, compile_ms) =
        zopro_wgpu::Quotient::new(gpu.clone(), n, &roots, &inverse_roots, &forward, &reverse)
            .expect("quotient");
    let upload_ms = started.elapsed().as_secs_f64() * 1000.0 - compile_ms;
    let m = n - n / 16;
    let (a, b, c): (Vec<Fr>, Vec<Fr>, Vec<Fr>) = (
        (0..m).map(|_| Fr::random()).collect(),
        (0..m).map(|_| Fr::random()).collect(),
        (0..m).map(|_| Fr::random()).collect(),
    );
    let started = Instant::now();
    let expected = plan.quotient(a.clone(), b.clone(), c.clone());
    let cpu_ms = started.elapsed().as_secs_f64() * 1000.0;
    let _ = quotient.quotient(&a, &b, &c);
    let started = Instant::now();
    let actual = quotient.quotient(&a, &b, &c).expect("gpu quotient");
    let gpu_ms = started.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(actual.len(), expected.len());
    let first_mismatch = actual.iter().zip(&expected).position(|(x, y)| x != y);
    assert!(
        first_mismatch.is_none(),
        "the wgpu quotient differs from the CPU plan at {first_mismatch:?}"
    );
    println!(
        "ZOPRO wgpu-quotient n={n} device=\"{}\" compile_ms={compile_ms:.0} upload_ms={upload_ms:.0} cpu_ms={cpu_ms:.0} gpu_ms={gpu_ms:.0} exact=true",
        gpu.name
    );
}

/// A key's inputs as a dump holds them.
struct Dump {
    name: String,
    a: Vec<u8>,
    b: Vec<u8>,
    k: Vec<u8>,
    z: Vec<u8>,
    b2: Vec<u8>,
    domain: Vec<u8>,
    program: Vec<u8>,
    coefficients: Vec<u8>,
    witness: Vec<u8>,
}

impl Dump {
    fn read(dir: &Path) -> Self {
        let head = zopro_core::dump::Head::read(dir).expect("dump head");
        Self {
            name: dir.file_name().unwrap().to_string_lossy().into_owned(),
            a: read(dir, "a"),
            b: read(dir, "b"),
            k: read(dir, "k"),
            z: read(dir, "z"),
            b2: read(dir, "b2"),
            domain: head.domain,
            program: read(dir, "program"),
            coefficients: read(dir, "coefficients"),
            witness: read(dir, "witness"),
        }
    }

    fn key(&self) -> Key {
        let mut key =
            Key::build(&self.a, &self.b, &self.k, &self.z, &self.b2, &self.domain).expect("key");
        assert!(
            key.set_solver(&self.program, &self.coefficients),
            "solver plan"
        );
        key
    }
}

/// The process's physical footprint in MB: what iOS's memory limit counts
/// (RSS also counts clean pages the system can drop).
#[cfg(target_vendor = "apple")]
fn footprint_mb() -> u64 {
    let mut info: libc::rusage_info_v2 = unsafe { std::mem::zeroed() };
    // SAFETY: the buffer is a rusage_info_v2, as RUSAGE_INFO_V2 asks for.
    let status = unsafe {
        proc_pid_rusage(
            std::process::id() as i32,
            libc::RUSAGE_INFO_V2,
            &mut info as *mut _ as *mut libc::rusage_info_t,
        )
    };
    if status == 0 {
        info.ri_phys_footprint >> 20
    } else {
        0
    }
}
#[cfg(not(target_vendor = "apple"))]
fn footprint_mb() -> u64 {
    0
}

/// The process's largest physical footprint so far, in MB.
#[cfg(target_vendor = "apple")]
fn peak_footprint_mb() -> u64 {
    let mut info: libc::rusage_info_v4 = unsafe { std::mem::zeroed() };
    // SAFETY: the buffer is a rusage_info_v4, as RUSAGE_INFO_V4 asks for.
    let status = unsafe {
        proc_pid_rusage(
            std::process::id() as i32,
            libc::RUSAGE_INFO_V4,
            &mut info as *mut _ as *mut libc::rusage_info_t,
        )
    };
    if status == 0 {
        info.ri_lifetime_max_phys_footprint >> 20
    } else {
        0
    }
}
#[cfg(not(target_vendor = "apple"))]
fn peak_footprint_mb() -> u64 {
    0
}

/// Loads every dump's key in turn, `cycles` times over, proving twice with
/// each, as an app switching shapes does. The proof's MSM results are
/// deterministic (the assembly adds the randomness), so each is compared
/// with a CPU-only key's; the GPU's allocation and the process's physical
/// footprint are printed per cycle. Exits non-zero on any mismatch, or when either rose on
/// each of the last four cycles (a leak; warming allocators settle).
fn stress(cycles: usize, dirs: &[String]) {
    let dumps: Vec<Dump> = dirs.iter().map(|d| Dump::read(Path::new(d))).collect();
    println!("STRESS dumps read: footprint_mb={}", footprint_mb());
    // SAFETY: single-threaded here; the variable is read at key build.
    unsafe { env::set_var("ZOPRO_GPU", "0") };
    let references: Vec<Vec<u8>> = dumps
        .iter()
        .map(|d| d.key().solve(&d.witness).expect("cpu parts"))
        .collect();
    unsafe { env::remove_var("ZOPRO_GPU") };
    println!("STRESS references: footprint_mb={}", footprint_mb());
    if env::var("ZOPRO_STRESS_DETAIL").is_ok_and(|v| v == "1") {
        let started = Instant::now();
        let warmed = zopro_core::gpu_warm();
        println!(
            "STRESS gpu warm {warmed} in {} ms: footprint_mb={}",
            started.elapsed().as_millis(),
            footprint_mb()
        );
    }
    let mut samples: Vec<(u64, u64)> = Vec::new();
    let mut failed = false;
    for cycle in 0..cycles {
        let started = Instant::now();
        let detail = env::var("ZOPRO_STRESS_DETAIL").is_ok_and(|v| v == "1");
        for (dump, reference) in dumps.iter().zip(&references) {
            let key = dump.key();
            if detail {
                println!(
                    "STRESS   {} built: footprint_mb={} gpu_mb={}",
                    dump.name,
                    footprint_mb(),
                    zopro_core::gpu_allocated_bytes() >> 20
                );
                if let Some(seconds) = env::var("ZOPRO_STRESS_PAUSE")
                    .ok()
                    .and_then(|v| v.parse().ok())
                {
                    std::thread::sleep(std::time::Duration::from_secs(seconds));
                }
            }
            for round in 0..2 {
                match key.solve(&dump.witness) {
                    Ok(parts) if &parts == reference => {}
                    Ok(_) => {
                        println!(
                            "STRESS cycle {cycle} {} round {round}: MISMATCH with the CPU",
                            dump.name
                        );
                        failed = true;
                    }
                    Err(e) => {
                        println!(
                            "STRESS cycle {cycle} {} round {round}: error {e}",
                            dump.name
                        );
                        failed = true;
                    }
                }
            }
            if detail {
                println!(
                    "STRESS   {} proved: footprint_mb={}",
                    dump.name,
                    footprint_mb()
                );
                drop(key);
                println!(
                    "STRESS   {} dropped: footprint_mb={} gpu_mb={}",
                    dump.name,
                    footprint_mb(),
                    zopro_core::gpu_allocated_bytes() >> 20
                );
            }
        }
        let (gpu_mb, rss) = (zopro_core::gpu_allocated_bytes() >> 20, footprint_mb());
        println!(
            "STRESS cycle {cycle} gpu_mb={gpu_mb} footprint_mb={rss} ms={}",
            started.elapsed().as_millis()
        );
        samples.push((gpu_mb, rss));
    }
    // A leak grows every cycle; allocators warming up grow for a few and
    // settle. So: growth on each of the last four cycles, by more than 2 %
    // in all, of the GPU's allocation or of the RSS.
    if samples.len() >= 5 {
        let tail = &samples[samples.len() - 5..];
        for (name, at) in [("gpu_mb", 0usize), ("footprint_mb", 1)] {
            let value = |s: &(u64, u64)| if at == 0 { s.0 } else { s.1 };
            let rising = tail.windows(2).all(|w| value(&w[1]) > value(&w[0]));
            let (from, to) = (value(&tail[0]), value(&tail[4]));
            if rising && to > from + from / 50 {
                println!("STRESS {name} rose every cycle, {from} to {to}: a leak");
                failed = true;
            }
        }
    } else {
        println!("STRESS too few cycles to judge memory (5 at least)");
    }
    println!("STRESS {}", if failed { "FAILED" } else { "passed" });
    if failed {
        std::process::exit(1);
    }
}

/// How skewed a shape's MSM scalars are: per vector, the share of zeros,
/// ones, minus ones, and of the rest by bit length (small negatives, p - s
/// with s short, counted by s's length).
fn scalar_profile(dir: &Path) {
    let dump = Dump::read(dir);
    let key = dump.key();
    let scalars = key.msm_scalars(&dump.witness).expect("solve");
    let classes = [
        "0", "1", "-1", "<=8b", "<=16b", "<=32b", "<=64b", "<=128b", "full", "-<=64b", "-<=128b",
    ];
    for (name, values) in ["a", "b", "k"].iter().zip(&scalars) {
        let mut counts = [0usize; 11];
        for v in values {
            let bits = v.num_bits();
            let neg_bits = (-*v).num_bits();
            let class = if v.is_zero() {
                0
            } else if *v == Fr::from(1u64) {
                1
            } else if *v == -Fr::from(1u64) {
                2
            } else if bits <= 8 {
                3
            } else if bits <= 16 {
                4
            } else if bits <= 32 {
                5
            } else if bits <= 64 {
                6
            } else if bits <= 128 {
                7
            } else if neg_bits <= 64 {
                9
            } else if neg_bits <= 128 {
                10
            } else {
                8
            };
            counts[class] += 1;
        }
        let n = values.len().max(1);
        let line: Vec<String> = classes
            .iter()
            .zip(counts)
            .map(|(c, k)| format!("{c}={:.1}%", 100.0 * k as f64 / n as f64))
            .collect();
        println!("SCALARS {} n={} {}", name, values.len(), line.join(" "));
    }
}

/// The proof parts of one witness from a GPU key and from a CPU-only key
/// (they are deterministic; the proof's randomness comes after): which of
/// A, B1, K, H·Z (G1) and B2 (G2) differ.
fn parts_diff(dir: &Path) {
    let dump = Dump::read(dir);
    // SAFETY: single-threaded here; the keys read ZOPRO_GPU when built.
    unsafe { env::set_var("ZOPRO_GPU", "0") };
    let cpu = dump.key().solve(&dump.witness).expect("cpu parts");
    unsafe { env::set_var("ZOPRO_GPU", "1") };
    let key = dump.key();
    for round in 0..2 {
        let gpu = key.solve(&dump.witness).expect("gpu parts");
        let names = ["a", "b1", "k", "hz", "b2"];
        let sizes = [64, 64, 64, 64, 128];
        let mut at = 0;
        let mut line = Vec::new();
        for (name, size) in names.iter().zip(sizes) {
            let same = cpu.get(at..at + size) == gpu.get(at..at + size);
            line.push(format!("{name}={}", if same { "same" } else { "DIFFERS" }));
            at += size;
        }
        println!(
            "PARTS round {round} len cpu={} gpu={} {}",
            cpu.len(),
            gpu.len(),
            line.join(" ")
        );
    }
}

/// The CPU MSMs of a dumped shape's B, Z and B2 with random scalars, best of
/// three (RAYON_NUM_THREADS, ZOPRO_CPU_WIDTH, ZOPRO_CPU_SPLIT).
fn cpu_msm(dir: &Path) {
    let time = |f: &mut dyn FnMut()| -> f64 {
        f();
        (0..3)
            .map(|_| {
                let started = Instant::now();
                f();
                started.elapsed().as_secs_f64() * 1000.0
            })
            .fold(f64::MAX, f64::min)
    };
    let only = env::var("ZOPRO_CPU_VECTORS").unwrap_or_else(|_| "b,z,b2".into());
    for name in only.split(',') {
        let ms = if name == "b2" {
            let bases = decode_g2(&read(dir, name));
            let scalars: Vec<Fr> = (0..bases.len()).map(|_| Fr::random()).collect();
            time(&mut || {
                let _ = std::hint::black_box(zopro_core::msm_g2(&bases, &scalars));
            })
        } else {
            let bases = decode_g1(&read(dir, name));
            let scalars: Vec<Fr> = (0..bases.len()).map(|_| Fr::random()).collect();
            time(&mut || {
                let _ = std::hint::black_box(zopro_core::msm_g1(&bases, &scalars));
            })
        };
        println!(
            "ZOPRO cpu-msm {name} threads={} width={} split={} ms={ms:.0}",
            env::var("RAYON_NUM_THREADS").unwrap_or_default(),
            env::var("ZOPRO_CPU_WIDTH").unwrap_or_else(|_| "auto".into()),
            env::var("ZOPRO_CPU_SPLIT").unwrap_or_else(|_| "1".into()),
        );
    }
}

/// `pack <dump dir> <key name> <out file>`: the zopro key file of a dumped
/// key (see `zopro_core::dump`; `tools/gnark` writes dumps).
fn pack_dump(dir: &Path, name: &str, out: &Path) {
    let dump = zopro_core::dump::Dump::read(dir).expect("dump");
    let parts = dump.parts(name);
    let started = Instant::now();
    let bytes = zopro_core::artifact::pack(&parts).expect("pack");
    fs::write(out, &bytes).expect("write the key");
    println!(
        "ZOPRO pack {name} bytes={} points={} ms={}",
        bytes.len(),
        dump.vectors[..4].iter().map(Vec::len).sum::<usize>() / 64 + dump.vectors[4].len() / 128,
        started.elapsed().as_millis()
    );
}

/// The iOS bench app's entry (`tools/ios-bench`): the arguments the app was
/// launched with, the dumps under its Documents directory.
#[no_mangle]
pub extern "C" fn zopro_bench_main() {
    if let Ok(home) = env::var("HOME") {
        let _ = env::set_current_dir(Path::new(&home).join("Documents"));
    }
    run();
}

// libSystem's, on iOS too; the libc crate declares it for macOS only.
#[cfg(target_vendor = "apple")]
extern "C" {
    fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut libc::rusage_info_t) -> i32;
}
