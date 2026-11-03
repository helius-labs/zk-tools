//! Every proof is blinded with a fresh r and s. A proof that skips or
//! reuses them still verifies, so only a test can tell, and it would leak
//! the witness. The cubic circuit's witness (zopro-prover's fixture, X³ +
//! X + 5 = Y) proved twice with one key: the kernel's parts are the same
//! both times, the two proofs verify, A, B and C each differ between them,
//! and neither is the unblinded proof (r = s = 0), which verifies too. On
//! the CPU, and with `--features metal` or `--features wgpu` on the GPU
//! (skipped without one).
use std::path::Path;

use zopro_bn254::Fr;
use zopro_core::{
    artifact::{pack, Artifact},
    decode_frs, decode_g1, decode_g2,
    dump::Dump,
    groth16::{assemble, FixedPoints},
    verify::{Proof, VerifyingKey},
    Key,
};

struct Cubic {
    key_file: Vec<u8>,
    verifying_key: VerifyingKey,
    fixed: FixedPoints,
    witness: Vec<u8>,
    public: Vec<Fr>,
}

impl Cubic {
    fn read() -> Self {
        let dir =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../zopro-prover/tests/fixtures/cubic");
        let dump = Dump::read(&dir).expect("the cubic dump");
        let read = |name: &str| std::fs::read(dir.join(name)).expect(name);
        let key_file = pack(&dump.parts("cubic")).expect("pack");
        let artifact = Artifact::parse(&key_file).expect("parse");
        Self {
            verifying_key: artifact.verifying_key().expect("verifying key"),
            fixed: artifact.fixed_points().expect("fixed points"),
            key_file,
            witness: read("witness"),
            public: decode_frs(&read("public")),
        }
    }

    /// The key; with a GPU feature, on the CPU (ZOPRO_GPU=0) unless `gpu`.
    fn key(&self, gpu: bool) -> Key {
        // SAFETY: this binary's only test, the only reader of ZOPRO_GPU.
        if !gpu {
            unsafe { std::env::set_var("ZOPRO_GPU", "0") };
        }
        let artifact = Artifact::parse(&self.key_file).expect("parse");
        let key = artifact.key(artifact.points(), None).expect("key");
        unsafe { std::env::remove_var("ZOPRO_GPU") };
        key
    }

    fn check(&self, key: &Key, on: &str) {
        let (verifying_key, fixed) = (&self.verifying_key, &self.fixed);
        let parts = key.solve(&self.witness).expect("solve");
        assert_eq!(
            key.solve(&self.witness).expect("solve"),
            parts,
            "{on}: the same parts each time, so only the blinding differs"
        );
        let first = Proof::from_raw(&assemble(&parts, fixed)).expect("proof");
        let second = Proof::from_raw(&assemble(&parts, fixed)).expect("proof");
        assert!(
            verifying_key.verify(&first, &self.public),
            "{on}: the first proof verifies"
        );
        assert!(
            verifying_key.verify(&second, &self.public),
            "{on}: the second proof verifies"
        );
        assert_ne!(first.ar, second.ar, "{on}: A, a fresh r");
        assert_ne!(first.bs, second.bs, "{on}: B, a fresh s");
        assert_ne!(first.krs, second.krs, "{on}: C");

        // r = s = 0: Ar = A + alpha, Bs = B2 + beta2, Krs = K + Z.
        let g1 = |i: usize| decode_g1(&parts[64 * i..64 * i + 64])[0].to_projective();
        let unblinded = Proof {
            ar: (g1(0) + fixed.alpha.to_projective()).to_affine(),
            bs: (decode_g2(&parts[256..384])[0].to_projective() + fixed.beta2.to_projective())
                .to_affine(),
            krs: (g1(2) + g1(3)).to_affine(),
        };
        assert!(
            verifying_key.verify(&unblinded, &self.public),
            "the unblinded proof verifies: the verifier cannot catch it"
        );
        for proof in [&first, &second] {
            assert_ne!(proof.ar, unblinded.ar, "{on}: A is blinded");
            assert_ne!(proof.bs, unblinded.bs, "{on}: B is blinded");
            assert_ne!(proof.krs, unblinded.krs, "{on}: C is blinded");
        }
    }
}

#[test]
fn every_proof_is_blinded_afresh() {
    let cubic = Cubic::read();
    let cpu = cubic.key(false);
    assert!(!cpu.has_gpu());
    cubic.check(&cpu, "CPU");
    #[cfg(gpu_any)]
    {
        let gpu = cubic.key(true);
        if !gpu.has_gpu() {
            eprintln!("no GPU: skipped");
            return;
        }
        cubic.check(&gpu, "GPU");
    }
}
