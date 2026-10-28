//! Checks of the arithmetic at key load. A phone's GPU may compile the
//! kernels and still compute them wrong (two older iPhones did where a
//! newer one did not), so each unit is compared with another
//! implementation on a small input and reported as a ZOPRO_TRACE selftest
//! line; a GPU unit that disagrees is not used.
use super::*;

/// Points of each vector kept back for the checks.
pub(crate) const HEAD: usize = 1024;

#[cfg(gpu_any)]
pub(crate) struct Verdict {
    pub msm: bool,
    pub quotient: bool,
}

fn report(unit: &str, ok: bool, started: crate::time::Instant) -> bool {
    log::info!(
        "ZOPRO_TRACE selftest {unit} {} {:.1} ms",
        if ok { "ok" } else { "MISMATCH" },
        started.elapsed().as_secs_f64() * 1000.0
    );
    ok
}

/// The base field multiply (an assembly routine on AArch64) against a
/// plain CIOS multiply in u128 arithmetic.
pub(crate) fn field() -> bool {
    let started = crate::time::Instant::now();
    let ok = (0..4096).all(|_| {
        let (a, b) = (crate::random_fq(), crate::random_fq());
        (a * b).0 == cios(a.0, b.0)
    });
    report("field", ok, started)
}

fn cios(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let q = Fq::MODULUS;
    let inv = <zopro_bn254::FqConfig as zopro_bn254::FpConfig>::INV;
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
    let r = [t[0], t[1], t[2], t[3]];
    let mut d = [0u64; 4];
    let mut borrow = false;
    for i in 0..4 {
        let (x, b1) = r[i].overflowing_sub(q[i]);
        let (x, b2) = x.overflowing_sub(borrow as u64);
        d[i] = x;
        borrow = b1 | b2;
    }
    if t[4] != 0 || !borrow {
        d
    } else {
        r
    }
}

/// The GPU's G1 MSM and quotient against the CPU's, and the CPU's G2 MSM
/// against the lane-form MSM's; with Metal on a Mac, the GPU's G2 MSM too.
#[cfg(gpu_any)]
pub(crate) fn gpu(
    gpu: &mut Gpu,
    plan: &fft::Plan,
    a_head: &[G1Affine],
    b2_head: &[G2Affine],
    n_a: usize,
    n_b: usize,
) -> Verdict {
    let mut scalars = vec![Fr::ZERO; n_a.max(n_b)];
    for s in scalars.iter_mut().take(HEAD) {
        *s = crate::random_fr();
    }
    let head = a_head.len().min(HEAD);

    // Three inputs that tell the kernels apart: one point (a shard and the
    // reduction), one bucket of 64 equal digits (shards and their tree),
    // and random scalars over the head (every window and sign).
    let g1 = |name: &str, scalars: &[Fr]| {
        let started = crate::time::Instant::now();
        let cpu = msm_g1(&a_head[..head], &scalars[..head]);
        let got = gpu.a.lock().unwrap().msm(scalars);
        report(name, got.is_ok_and(|(got, _)| got == cpu), started)
    };
    let mut one = vec![Fr::ZERO; n_a];
    one[0] = Fr::ONE;
    let mut bucket = vec![Fr::ZERO; n_a];
    for s in bucket.iter_mut().take(head.min(64)) {
        *s = Fr::from(5u64);
    }
    #[cfg_attr(not(gpu), allow(unused_mut))]
    let mut msm = g1("gpu-msm-g1-one", &one)
        & g1("gpu-msm-g1-bucket", &bucket)
        & g1("gpu-msm-g1-random", &scalars[..n_a]);

    let started = crate::time::Instant::now();
    let head2 = b2_head.len().min(HEAD);
    let cpu2 = msm_g2(b2_head, &scalars[..head2]);
    // The lane-form MSM, another algorithm.
    let reference = crate::msm::msm(&b2_head[..head2], &scalars[..head2]);
    report("cpu-msm-g2", cpu2 == reference, started);
    #[cfg(gpu)]
    if let Some(b2) = &gpu.b2 {
        let started = crate::time::Instant::now();
        let got2 = b2.lock().unwrap().msm(&scalars[..n_b]);
        msm &= report(
            "gpu-msm-g2",
            got2.is_ok_and(|(got2, _)| got2 == cpu2),
            started,
        );
    }

    // The quotient only with the fullcheck feature: it is a full-size
    // transform on the CPU too, seconds and a few hundred MB at the widest
    // key, and no GPU has got it wrong yet.
    #[cfg(feature = "fullcheck")]
    let quotient = {
        let started = crate::time::Instant::now();
        let n = plan.size();
        let (a, b, c): (Vec<Fr>, Vec<Fr>, Vec<Fr>) = (
            (0..n).map(|_| crate::random_fr()).collect(),
            (0..n).map(|_| crate::random_fr()).collect(),
            (0..n).map(|_| crate::random_fr()).collect(),
        );
        let cpu = plan.quotient(a.clone(), b.clone(), c.clone());
        let got = gpu.quotient.lock().unwrap().quotient(&a, &b, &c);
        report("gpu-quotient", got.is_ok_and(|got| got == cpu), started)
    };
    #[cfg(not(feature = "fullcheck"))]
    let quotient = {
        let _ = plan;
        true
    };
    Verdict { msm, quotient }
}

/// A GPU G1 MSM over the whole vector with random scalars against the CPU's
/// (the fullcheck feature: a wrong proof on a Galaxy A56 at the widest key
/// passed every head check).
#[cfg(all(gpu_any, feature = "fullcheck"))]
pub(crate) fn full_g1(
    name: &str,
    bases: &[G1Affine],
    mut gpu: impl FnMut(&[Fr]) -> Result<G1Projective, String>,
) -> bool {
    let started = crate::time::Instant::now();
    let scalars: Vec<Fr> = (0..bases.len()).map(|_| crate::random_fr()).collect();
    let cpu = msm_g1(bases, &scalars);
    let got = gpu(&scalars);
    report(
        &format!("full-msm-{name}"),
        got.is_ok_and(|got| got == cpu),
        started,
    )
}

/// The CPU G2 MSM over the whole vector against the lane-form MSM's.
#[cfg(feature = "fullcheck")]
pub(crate) fn full_g2(bases: &[G2Affine]) -> bool {
    let started = crate::time::Instant::now();
    let scalars: Vec<Fr> = (0..bases.len()).map(|_| crate::random_fr()).collect();
    let ours = msm_g2(bases, &scalars);
    let reference = crate::msm::msm(bases, &scalars);
    report("full-cpu-msm-b2", ours == reference, started)
}
