//! Proves one shape from the files TestRingilDump wrote (head, a, b, k, z,
//! b2, program, coefficients, witness) and prints the stage times, then
//! writes the proof as Ar, Krs, Bs in gnark's raw big-endian encoding for
//! the Go referee.
//!
//!   RAYON_NUM_THREADS=4 ringil-bench <dump dir> [rounds]
use std::{env, fs, path::Path, time::Instant};

use ark_bn254::{Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField, UniformRand};
use ringil_core::{decode_g1, decode_g2, Key};

fn read(dir: &Path, name: &str) -> Vec<u8> {
    fs::read(dir.join(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn main() {
    let dir = env::args().nth(1).expect("the dump directory");
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
