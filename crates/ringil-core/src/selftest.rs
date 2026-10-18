//! Checks of the arithmetic at key load. A phone's GPU may compile the
//! kernels and still compute them wrong (two older iPhones did where a
//! newer one did not), so each unit is compared with another
//! implementation on a small input and reported as a RINGIL_TRACE selftest
//! line; a GPU unit that disagrees is not used.
use super::*;
use ark_ff::{BigInteger, MontConfig, PrimeField, UniformRand};

/// Points of each vector kept back for the checks.
pub(crate) const HEAD: usize = 1024;

pub(crate) struct Verdict {
    pub msm: bool,
    pub quotient: bool,
}

fn report(unit: &str, ok: bool, started: std::time::Instant) -> bool {
    log::info!(
        "RINGIL_TRACE selftest {unit} {} {:.1} ms",
        if ok { "ok" } else { "MISMATCH" },
        started.elapsed().as_secs_f64() * 1000.0
    );
    ok
}

/// The base field multiply (an assembly routine on AArch64) against a
/// plain CIOS multiply in u128 arithmetic.
pub(crate) fn field() -> bool {
    let started = std::time::Instant::now();
    let mut rng = rand::thread_rng();
    let ok = (0..4096).all(|_| {
        let (a, b) = (Fq::rand(&mut rng), Fq::rand(&mut rng));
        (a * b).0 == BigInt(cios(a.0 .0, b.0 .0))
    });
    report("field", ok, started)
}

fn cios(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let q = Fq::MODULUS.0;
    let inv = <ark_bn254::FqConfig as MontConfig<4>>::INV;
    let mut t = [0u64; 6];
    for &bi in &b {
        let mut carry = 0u128;
        for j in 0..4 {
            let s = t[j] as u128 + a[j] as u128 * bi as u128 + carry;
            t[j] = s as u64;
            carry = s >> 64;
        }
        let s = t[4] as u128 + carry;
        t[4] = s as u64;
        t[5] = (s >> 64) as u64;
        let m = t[0].wrapping_mul(inv);
        let s = t[0] as u128 + m as u128 * q[0] as u128;
        let mut carry = s >> 64;
        for j in 1..4 {
            let s = t[j] as u128 + m as u128 * q[j] as u128 + carry;
            t[j - 1] = s as u64;
            carry = s >> 64;
        }
        let s = t[4] as u128 + carry;
        t[3] = s as u64;
        t[4] = t[5] + (s >> 64) as u64;
    }
    let mut r = BigInt([t[0], t[1], t[2], t[3]]);
    if t[4] != 0 || r >= BigInt(q) {
        r.sub_with_borrow(&BigInt(q));
    }
    r.0
}

/// The GPU's G1 MSM and quotient against the CPU's, and the CPU's G2 MSM
/// against arkworks'; with Metal on a Mac, the GPU's G2 MSM too.
#[cfg(gpu_any)]
pub(crate) fn gpu(gpu: &mut Gpu, plan: &fft::Plan, a_head: &[G1Affine], b2_head: &[G2Affine], n_a: usize, n_b: usize) -> Verdict {
    let mut rng = rand::thread_rng();
    let mut scalars = vec![Fr::ZERO; n_a.max(n_b)];
    for s in scalars.iter_mut().take(HEAD) {
        *s = Fr::rand(&mut rng);
    }
    let head = a_head.len().min(HEAD);

    // Three inputs that tell the kernels apart: one point (a shard and the
    // reduction), one bucket of 64 equal digits (shards and their tree),
    // and random scalars over the head (every window and sign).
    let mut g1 = |name: &str, scalars: &[Fr]| {
        let started = std::time::Instant::now();
        let cpu = msm_g1(&a_head[..head], &scalars[..head]);
        let (got, _) = gpu.a.lock().unwrap().msm(scalars);
        report(name, got == cpu, started)
    };
    let mut one = vec![Fr::ZERO; n_a];
    one[0] = Fr::ONE;
    let mut bucket = vec![Fr::ZERO; n_a];
    for s in bucket.iter_mut().take(head.min(64)) {
        *s = Fr::from(5u64);
    }
    #[cfg_attr(not(gpu), allow(unused_mut))]
    let mut msm = g1("gpu-msm-g1-one", &one) & g1("gpu-msm-g1-bucket", &bucket) & g1("gpu-msm-g1-random", &scalars[..n_a]);

    let started = std::time::Instant::now();
    let head2 = b2_head.len().min(HEAD);
    let cpu2 = msm_g2(b2_head, &scalars[..head2]);
    let reference = <G2Projective as ark_ec::VariableBaseMSM>::msm_unchecked(&b2_head[..head2], &scalars[..head2]);
    report("cpu-msm-g2", cpu2 == reference, started);
    #[cfg(gpu)]
    if let Some(b2) = &gpu.b2 {
        let started = std::time::Instant::now();
        let (got2, _) = b2.lock().unwrap().msm(&scalars[..n_b]);
        msm &= report("gpu-msm-g2", got2 == cpu2, started);
    }

    let started = std::time::Instant::now();
    let n = plan.size();
    let (a, b, c): (Vec<Fr>, Vec<Fr>, Vec<Fr>) = (
        (0..n).map(|_| Fr::rand(&mut rng)).collect(),
        (0..n).map(|_| Fr::rand(&mut rng)).collect(),
        (0..n).map(|_| Fr::rand(&mut rng)).collect(),
    );
    let cpu = plan.quotient(a.clone(), b.clone(), c.clone());
    let got = gpu.quotient.lock().unwrap().quotient(&a, &b, &c);
    let quotient = report("gpu-quotient", got == cpu, started);
    Verdict { msm, quotient }
}
