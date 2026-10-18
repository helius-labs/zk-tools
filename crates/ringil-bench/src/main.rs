//! Proves one shape from the files TestRingilDump wrote (head, a, b, k, z,
//! b2, program, coefficients, witness) and prints the stage times, then
//! writes the proof as Ar, Krs, Bs in gnark's raw big-endian encoding for
//! the Go referee.
//!
//!   RAYON_NUM_THREADS=4 ringil-bench <dump dir> [rounds]
use std::{env, fs, path::Path, time::Instant};

use ark_bn254::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, Field, PrimeField, UniformRand};
use ringil_core::{decode_g1, decode_g2, Key};

fn read(dir: &Path, name: &str) -> Vec<u8> {
    fs::read(dir.join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn main() {
    let dir = env::args().nth(1).expect("the dump directory");
    if dir == "field" {
        return field();
    }
    if dir == "gpu" {
        return gpu(Path::new(&env::args().nth(2).expect("the dump directory")));
    }
    let dir = Path::new(&dir);
    let rounds: usize = env::args().nth(2).map(|v| v.parse().unwrap()).unwrap_or(5);
    let head = read(dir, "head");
    assert_eq!(&head[..16], b"zolana-key-head1", "key image head");
    // After the magic and eleven u32 counts: digest, alpha, beta, delta,
    // beta2, delta2, generator, coset generator.
    let alpha = decode_g1(&head[92..156])[0];
    let beta = decode_g1(&head[156..220])[0];
    let delta = decode_g1(&head[220..284])[0];
    let beta2 = decode_g2(&head[284..412])[0];
    let delta2 = decode_g2(&head[412..540])[0];
    let domain = &head[540..604];
    let (a, b, k, z, b2) = (read(dir, "a"), read(dir, "b"), read(dir, "k"), read(dir, "z"), read(dir, "b2"));

    let started = Instant::now();
    let mut key = Key::new(&a, &b, &k, &z, &b2, domain).expect("key");
    let key_ms = started.elapsed().as_millis();
    let started = Instant::now();
    assert!(key.set_solver(&read(dir, "program"), &read(dir, "coefficients")), "solver plan");
    let plan_ms = started.elapsed().as_millis();
    let witness = read(dir, "witness");

    let mut prove = Vec::new();
    let mut proof = Vec::new();
    for _ in 0..rounds {
        let started = Instant::now();
        let parts = key.solve_parts(&witness).expect("solve and parts");
        let (ar, bs1, kr, hz) = (
            decode_g1(&parts[..64])[0],
            decode_g1(&parts[64..128])[0],
            decode_g1(&parts[128..192])[0],
            decode_g1(&parts[192..256])[0],
        );
        let bs2 = decode_g2(&parts[256..384])[0];
        proof = assemble(ar, bs1, kr, hz, bs2, alpha, beta, delta, beta2, delta2);
        prove.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    fs::write(dir.join("proof"), &proof).unwrap();
    let mut sorted = prove.clone();
    sorted.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let threads = env::var("RAYON_NUM_THREADS").unwrap_or_default();
    println!(
        "RINGIL rust {} threads={threads} key_ms={key_ms} plan_ms={plan_ms} prove_ms_min={:.0} prove_ms_median={:.0} rounds={rounds}",
        dir.file_name().unwrap().to_string_lossy(),
        sorted[0],
        sorted[sorted.len() / 2]
    );
}

/// gnark's Groth16 assembly over the kernel's parts, with fresh blinding.
#[allow(clippy::too_many_arguments)]
fn assemble(
    a: G1Affine,
    b: G1Affine,
    k: G1Affine,
    z: G1Affine,
    b2: G2Affine,
    alpha: G1Affine,
    beta: G1Affine,
    delta: G1Affine,
    beta2: G2Affine,
    delta2: G2Affine,
) -> Vec<u8> {
    let mut rng = rand::rngs::OsRng;
    let (r, s) = (Fr::rand(&mut rng), Fr::rand(&mut rng));
    let delta: G1Projective = delta.into_group();
    let ar = a.into_group() + alpha.into_group() + delta * r;
    let bs1 = b.into_group() + beta.into_group() + delta * s;
    let bs: G2Projective = b2.into_group() + beta2.into_group() + delta2.into_group() * s;
    let krs = k.into_group() + z.into_group() + delta * (-(r * s)) + ar * s + bs1 * r;
    let (ar, krs, bs) = (ar.into_affine(), krs.into_affine(), bs.into_affine());
    let mut out = Vec::with_capacity(256);
    for v in [ar.x, ar.y, krs.x, krs.y] {
        out.extend_from_slice(&v.into_bigint().to_bytes_be());
    }
    for v in [bs.x.c1, bs.x.c0, bs.y.c1, bs.y.c0] {
        out.extend_from_slice(&v.into_bigint().to_bytes_be());
    }
    out
}

/// Nanoseconds per Fq multiplication, squaring, and Fq2 multiplication on
/// one thread: a dependent chain of ten million.
fn field() {
    use ark_bn254::{Fq, Fq2};
    let mut rng = rand::rngs::OsRng;
    let (a, b) = (Fq::rand(&mut rng), Fq::rand(&mut rng));
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
        y.square_in_place();
    }
    let square = started.elapsed().as_nanos() as f64 / n as f64;
    let (a2, b2) = (Fq2::rand(&mut rng), Fq2::rand(&mut rng));
    let started = Instant::now();
    let mut z = a2;
    for _ in 0..n {
        z *= b2;
    }
    let mul2 = started.elapsed().as_nanos() as f64 / n as f64;
    println!("RINGIL field fq_mul_ns={mul:.2} fq_square_ns={square:.2} fq2_mul_ns={mul2:.2} {}", (x + y).0 .0[0] ^ z.c0.0 .0[0]);
}

/// The G1 MSMs of a dumped shape's Z and B vectors with random scalars, on
/// the CPU (RAYON_NUM_THREADS) and on the GPU, checked against each other.
#[cfg(any(target_os = "macos", target_os = "ios"))]
fn gpu(dir: &Path) {
    let mut rng = rand::rngs::OsRng;
    // Witness scalars are skewed: many wires are 0, 1 or small, so a few
    // buckets receive most points. "skewed" is a quarter ones, a quarter
    // small, the rest random.
    let skewed = |n: usize, rng: &mut rand::rngs::OsRng| -> Vec<Fr> {
        (0..n)
            .map(|i| match i % 4 {
                0 => Fr::from(1u64),
                1 => Fr::from((i % 65536) as u64),
                _ => Fr::rand(rng),
            })
            .collect()
    };
    for name in ["b", "b skewed", "z", "b2", "b2 skewed"] {
        let file = name.split(' ').next().unwrap();
        let skew = name.ends_with("skewed");
        if file == "b2" {
            let bases = decode_g2(&read(dir, file));
            let scalars = if skew { skewed(bases.len(), &mut rng) } else { (0..bases.len()).map(|_| Fr::rand(&mut rng)).collect() };
            gpu_one(name, &bases, &scalars, ringil_core::msm_g2);
        } else {
            let bases = decode_g1(&read(dir, file));
            let scalars = if skew { skewed(bases.len(), &mut rng) } else { (0..bases.len()).map(|_| Fr::rand(&mut rng)).collect() };
            gpu_one(name, &bases, &scalars, ringil_core::msm_g1);
        }
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn gpu_one<C: ringil_metal::Curve>(name: &str, bases: &[C::Affine], scalars: &[Fr], cpu_msm: fn(&[C::Affine], &[Fr]) -> C) {
    {
        let started = Instant::now();
        let (mut msm, compile_ms) = ringil_metal::Msm::<C>::new(bases).expect("gpu msm");
        let upload_ms = started.elapsed().as_secs_f64() * 1000.0 - compile_ms;
        let started = Instant::now();
        let cpu = cpu_msm(bases, scalars);
        let cpu_ms = started.elapsed().as_secs_f64() * 1000.0;
        // Warm, then timed.
        let _ = msm.msm(scalars);
        let mut best: Option<(ringil_metal::Timings, f64)> = None;
        let mut result = cpu;
        for _ in 0..3 {
            let started = Instant::now();
            let (point, timings) = msm.msm(scalars);
            let total = started.elapsed().as_secs_f64() * 1000.0;
            result = point;
            if best.is_none_or(|(_, t)| total < t) {
                best = Some((timings, total));
            }
        }
        let (t, total) = best.unwrap();
        assert_eq!(result.into_affine(), cpu.into_affine(), "{name}: the GPU MSM differs from the CPU");
        println!(
            "RINGIL gpu \"{name}\" points={} device=\"{}\" compile_ms={compile_ms:.0} upload_ms={upload_ms:.0} cpu_ms={cpu_ms:.0} gpu_ms={total:.0} sort_ms={:.0} accumulate_ms={:.0} reduce_ms={:.0} combine_ms={:.1}",
            bases.len(),
            msm.device_name(),
            t.sort_ms,
            t.accumulate_ms,
            t.reduce_ms,
            t.combine_ms
        );
    }
}
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn gpu(_: &Path) {
    panic!("the GPU bench needs Metal");
}
