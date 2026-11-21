//! A Groth16 prover over BN254 for gnark circuits, without gnark: a zopro
//! key (`zopro_core::artifact`, built offline from gnark's constraint
//! system and keys by `tools/gnark`) and the circuit's inputs by gnark's
//! names give a proof in gnark's formats, verified with the key's verifying
//! key before it is returned.
//!
//! **Experimental.** New and unaudited: checked against gnark and arkworks
//! on the devices we have, every proof verified before it is returned, no
//! independent review yet. Do not rely on it where a wrong or leaked proof
//! would cost anything.
//!
//! ```ignore
//! let mut prover = Prover::load(&std::fs::read("cubic.zopro")?)?;
//! let mut assignment = Assignment::default();
//! assignment.set("X".into(), Fr::from(3u64));
//! assignment.set("Y".into(), Fr::from(35u64));
//! let proved = prover.prove(&prover.witness(&assignment)?)?;
//! let (proof, public) = (proved.proof.to_gnark(), proved.public_witness());
//! ```
use zopro_bn254::Fr;
use zopro_core::{
    Key, Points,
    artifact::Artifact,
    groth16::PreparedFixedPoints,
    verify::{Proof, VerifyingKey, public_witness},
};

pub mod raw;
pub mod witness;

pub use witness::Assignment;
/// The kernel, for hosts that read key files and cache solver images.
pub use zopro_core;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// The assignment does not give the key's inputs.
    Witness(String),
    /// The kernel could not prove (an unsatisfied constraint, a failed GPU).
    Prove(String),
    /// The proof did not verify with the key's verifying key.
    Unverified,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Witness(message) | Self::Prove(message) => f.write_str(message),
            Self::Unverified => f.write_str("the proof did not verify"),
        }
    }
}

impl std::error::Error for Error {}

/// A verified proof.
#[derive(Clone, Debug)]
pub struct Proved {
    pub proof: Proof,
    /// The public inputs it proves, the constant one apart.
    pub public: Vec<Fr>,
    /// Solving, the MSMs and the assembly.
    pub prove_ms: u64,
    /// Checking the proof.
    pub verify_ms: u64,
}

impl Proved {
    /// gnark's binary public witness.
    pub fn public_witness(&self) -> Vec<u8> {
        public_witness(&self.public)
    }
}

/// A loaded zopro key.
pub struct Prover {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    overlap_verification: bool,
    key: Key,
    verifying_key: VerifyingKey,
    fixed: PreparedFixedPoints,
    name: String,
    metadata: Vec<u8>,
    /// The inputs' names in the key's order, public then secret, the
    /// constant one apart.
    names: Vec<String>,
    public: usize,
}

impl Prover {
    /// Opt into packed cyclotomic squaring during proof verification on
    /// native AVX-512 IFMA builds. This uses no extra key tables.
    pub fn set_packed_final_exponentiation(&mut self, enabled: bool) {
        self.verifying_key.set_packed_final_exponentiation(enabled);
    }

    /// Share a smaller Rayon pool across provers for native IFMA canonical
    /// witness solving and FFT preparation. With `ZOPRO_CPU_SOLVER_FRONTEND=1`,
    /// general CPU solver instructions also use this pool. With
    /// `ZOPRO_CPU_FFT_PRODUCT_FRONTEND=1`, folded packed Euler products
    /// use this pool and allocate their temporary output before dispatch.
    /// Pass `None` to detach it.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_frontend_pool(
        &mut self,
        pool: Option<std::sync::Arc<zopro_core::FrontendThreadPool>>,
    ) {
        self.key.set_frontend_pool(pool);
    }

    /// Allow final grouped G2 windows to reuse the attached frontend pool on
    /// native IFMA builds with resource scheduling. The prover checks scratch
    /// space at dispatch. Enable per key when the two pools use separate cores.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_grouped_g2_on_frontend(&mut self, enabled: bool) {
        self.key.set_grouped_g2_on_frontend(enabled);
    }

    /// Run the native IFMA fused K/H path's raw G1 A query on a smaller
    /// caller-attached frontend pool when additional scratch is available.
    /// Disabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_raw_g1_a_on_frontend(&mut self, enabled: bool) {
        self.key.set_raw_g1_a_on_frontend(enabled);
    }

    /// Fuse inverse, Euler-weight and forward work within native CPU FFT leaves.
    /// Disabled by default. Portable and other quotient routes keep their ordinary path.
    pub fn set_euler_fft_leaf_fusion(&mut self, enabled: bool) {
        self.key.set_euler_fft_leaf_fusion(enabled);
    }

    /// Attach a disjoint worker pool for a small exclusive G2 query.
    /// Enable quotient overlap to start G2 before the product FFT.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_exclusive_g2_pool(
        &mut self,
        pool: Option<std::sync::Arc<zopro_core::FrontendThreadPool>>,
        overlap_quotient: bool,
    ) {
        self.key.set_exclusive_g2_pool(pool, overlap_quotient);
    }

    /// Set a caller-owned pool for CPU witness instructions.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_solver_pool(
        &mut self,
        pool: Option<std::sync::Arc<zopro_core::FrontendThreadPool>>,
    ) {
        self.key.set_solver_pool(pool);
    }

    /// Reduce eligible grouped query points within the CPU scratch budget.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_grouped_point_tiling(&mut self, enabled: bool) {
        self.key.set_grouped_point_tiling(enabled);
    }

    /// Run the large native IFMA G1 K query on the attached frontend pool.
    /// Enable per key when the caller uses disjoint cores. Dispatch checks
    /// the workspace budget. Disabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_g1_k_on_frontend(&mut self, enabled: bool) {
        self.key.set_g1_k_on_frontend(enabled);
    }

    /// Use serial scalar preparation and a larger initial table for medium grouped B2 queries.
    /// The workspace policy still governs admission. Disabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_grouped_b2_serial_preparation(&mut self, enabled: bool) {
        self.key.set_grouped_b2_serial_preparation(enabled);
    }

    /// Reuse solver-derived Poseidon origins during native grouped B2 proofs.
    /// The prover checks scalar equality before reusing any class.
    /// Disabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_grouped_b2_poseidon_origins(&mut self, enabled: bool) {
        self.key.set_grouped_b2_poseidon_origins(enabled);
    }

    /// Control scalar grouping for the native exclusive G2 evaluator.
    /// Grouping is enabled by default.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_exclusive_g2_grouping(&mut self, enabled: bool) {
        self.key.set_exclusive_g2_grouping(enabled);
    }

    /// Compute the public-input pairing alongside CPU proof generation for
    /// keys with up to sixteen public inputs. Uses the existing Rayon pool.
    /// The default is sequential verification after generation.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    pub fn set_overlap_verification(&mut self, enabled: bool) {
        self.overlap_verification = enabled;
    }

    /// Use optional public wire offsets with folded CPU proof queries for this key.
    ///
    /// The little-endian file starts with `ZPWO0001`, the SHA-256 of its B2
    /// points (x.c0, x.c1, y.c0, y.c1 as Montgomery limbs), a u32 B2 count,
    /// and a u32 record count. Each record has an increasing u32 B2 index
    /// followed by a nonzero canonical 32-byte scalar. Supply public values.
    ///
    /// Preparation derives correction points from the loaded proving key. The
    /// caller can release the input bytes after this call. Unsupported routes
    /// and insufficient optional memory return `Ok(false)`.
    pub fn prepare_public_wire_offsets(&mut self, bytes: &[u8]) -> Result<bool, &'static str> {
        self.key.prepare_public_wire_offsets(bytes)
    }

    /// From a zopro key file.
    pub fn load(bytes: &[u8]) -> Result<Self, String> {
        let artifact = Artifact::parse(bytes)?;
        Self::from_artifact(&artifact, artifact.points(), None)
    }

    /// From a parsed key, its points (`Artifact::points`, or a host's
    /// cache of them decompressed) and a cached solver image when the host
    /// kept one (`solver_image`).
    pub fn from_artifact(
        artifact: &Artifact,
        points: Points,
        solver_image: Option<&[u8]>,
    ) -> Result<Self, String> {
        let names: Vec<String> = artifact
            .input_names()
            .iter()
            .map(|n| (*n).to_owned())
            .collect();
        let public = artifact.public as usize - 1;
        let verifying_key = artifact.verifying_key()?;
        if verifying_key.public_inputs() != public
            || names.len() != public + artifact.secret as usize
        {
            return Err("the key's inputs do not match its verifying key".into());
        }
        Ok(Self {
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            overlap_verification: false,
            key: artifact.key(points, solver_image)?,
            verifying_key,
            fixed: PreparedFixedPoints::new(artifact.fixed_points()?),
            name: artifact.name.to_owned(),
            metadata: artifact.metadata.to_vec(),
            names,
            public,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// What the application that built the key keeps in it.
    pub fn metadata(&self) -> &[u8] {
        &self.metadata
    }

    /// The inputs' names in the key's order: the public ones
    /// (`public_inputs` of them), then the secret ones.
    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn public_inputs(&self) -> usize {
        self.public
    }

    /// The kernel's solver image, for a host to cache beside the key.
    pub fn solver_image(&self) -> Option<Vec<u8>> {
        self.key.solver_image()
    }

    /// Whether proofs run on a GPU.
    pub fn has_gpu(&self) -> bool {
        self.key.has_gpu()
    }

    /// Proves on the CPU from now on, with the points in the key's image
    /// (`load_image`), or in a browser with the CPU's own copy. Natively a
    /// key loaded from bytes has its points on the GPU alone: it keeps its
    /// GPU and fails, and `drop_gpu_with` gives it the points again.
    pub fn drop_gpu(&mut self) -> Result<(), String> {
        self.key.drop_gpu().map_err(str::to_owned)
    }

    /// `drop_gpu` with the points the key was loaded from
    /// (`Artifact::points` of its key file, or its raw form's vectors), for
    /// a key loaded from bytes. On an error the prover is as it was.
    pub fn drop_gpu_with(&mut self, points: Points) -> Result<(), String> {
        self.key.drop_gpu_with(points).map_err(str::to_owned)
    }

    /// The witness vector of `assignment`, in the key's order.
    pub fn witness(&self, assignment: &Assignment) -> Result<Vec<Fr>, Error> {
        witness::order(assignment, &self.names)
    }

    /// Proves `witness` (the key's order): the proof is verified before it
    /// is returned; a GPU proof that fails, or fails to verify, is proved
    /// again on the CPU, and the GPU is let go (`drop_gpu`). A key whose
    /// points are on the GPU alone keeps it, and the proof fails.
    pub fn prove(&mut self, witness: &[Fr]) -> Result<Proved, Error> {
        let encoded = self.encode(witness)?;
        loop {
            if self.key.supports_rerandomized_solve() {
                let started = zopro_core::time::Instant::now();
                let public = &witness[..self.public];
                let mut prove_ms = 0;
                let mut verify_started = started;
                let mut generate = || {
                    let raw = self
                        .key
                        .solve_rerandomized(&encoded, &self.fixed)
                        .map_err(Error::Prove)?;
                    prove_ms = started.elapsed().as_millis() as u64;
                    verify_started = zopro_core::time::Instant::now();
                    Proof::from_raw(&raw).map_err(|_| Error::Unverified)
                };
                #[cfg(any(
                    all(target_os = "macos", target_arch = "aarch64"),
                    all(target_os = "linux", target_arch = "x86_64")
                ))]
                let verified = {
                    // The verifier can compute its public term before the proof is ready.
                    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
                    let overlap = self.public <= 16
                        && std::env::var("ZOPRO_CPU_VERIFY_OVERLAP").as_deref() != Ok("0")
                        && self.key.prefer_verification_overlap();
                    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
                    let overlap = self.overlap_verification && self.public <= 16;
                    if overlap {
                        self.verifying_key.verify_during_generation(public, generate)
                    } else {
                        generate()
                            .map(|proof| self.verifying_key.verify(&proof, public).then_some(proof))
                    }
                };
                #[cfg(not(any(
                    all(target_os = "macos", target_arch = "aarch64"),
                    all(target_os = "linux", target_arch = "x86_64")
                )))]
                let verified = generate()
                    .map(|proof| self.verifying_key.verify(&proof, public).then_some(proof));
                let proof = match verified {
                    Ok(Some(proof)) => proof,
                    Err(Error::Prove(error)) if self.key.has_gpu() => {
                        self.fall_back(&format!("the GPU failed ({error})"))?;
                        continue;
                    }
                    Ok(None) | Err(Error::Unverified) if self.key.has_gpu() => {
                        self.fall_back("a GPU proof did not verify")?;
                        continue;
                    }
                    Ok(None) => return Err(Error::Unverified),
                    Err(error) => return Err(error),
                };
                return Ok(Proved {
                    proof,
                    public: public.to_vec(),
                    prove_ms,
                    verify_ms: verify_started.elapsed().as_millis() as u64,
                });
            }
            let started = zopro_core::time::Instant::now();
            let parts = match self.key.solve(&encoded) {
                Ok(parts) => parts,
                Err(error) if self.key.has_gpu() => {
                    self.fall_back(&format!("the GPU failed ({error})"))?;
                    continue;
                }
                Err(error) => return Err(Error::Prove(error)),
            };
            let prove_ms = started.elapsed().as_millis() as u64;
            match finish(
                &parts,
                witness,
                self.public,
                &self.fixed,
                &self.verifying_key,
                prove_ms,
            ) {
                Err(_) if self.key.has_gpu() => self.fall_back("a GPU proof did not verify")?,
                result => return result,
            }
        }
    }

    /// Lets a GPU that failed go, for the CPU to prove, or says why the CPU
    /// cannot.
    fn fall_back(&mut self, failure: &str) -> Result<(), Error> {
        match self.key.drop_gpu() {
            Ok(()) => {
                log::warn!("zopro: {failure}; proving on the CPU from now on");
                Ok(())
            }
            Err(error) => {
                log::warn!("zopro: {failure}; {error}");
                Err(Error::Prove(format!("{failure}; {error}")))
            }
        }
    }

    /// `prove` in a browser: the GPU's work is awaited. A proof that does
    /// not verify is `Error::Unverified`, the GPU kept: the caller lets it
    /// go (`drop_gpu`) and proves again.
    #[cfg(target_arch = "wasm32")]
    pub fn prove_async(
        &self,
        witness: &[Fr],
    ) -> Result<std::pin::Pin<Box<dyn std::future::Future<Output = Result<Proved, Error>>>>, Error>
    {
        let encoded = self.encode(witness)?;
        let started = zopro_core::time::Instant::now();
        #[cfg(feature = "wgpu")]
        if self.key.has_gpu() {
            let parts = self
                .key
                .solve_parts_gpu(&encoded)
                .map_err(|_| Error::Prove("the GPU could not start".into()))?;
            let (witness, public, fixed) = (witness.to_vec(), self.public, self.fixed.clone());
            let verifying_key = self.verifying_key.clone();
            return Ok(Box::pin(async move {
                let parts = wasm_bindgen_futures::JsFuture::from(parts)
                    .await
                    .map_err(|e| Error::Prove(format!("the GPU failed: {e:?}")))?;
                let parts = js_sys::Uint8Array::new(&parts).to_vec();
                let prove_ms = started.elapsed().as_millis() as u64;
                finish(&parts, &witness, public, &fixed, &verifying_key, prove_ms)
            }));
        }
        let parts = self.key.solve(&encoded).map_err(Error::Prove)?;
        let prove_ms = started.elapsed().as_millis() as u64;
        let result = finish(
            &parts,
            witness,
            self.public,
            &self.fixed,
            &self.verifying_key,
            prove_ms,
        );
        Ok(Box::pin(async move { result }))
    }

    /// Whether `proof` proves `public` (the constant one apart).
    pub fn verify(&self, proof: &Proof, public: &[Fr]) -> bool {
        self.verifying_key.verify(proof, public)
    }

    fn encode(&self, witness: &[Fr]) -> Result<Vec<u8>, Error> {
        if witness.len() != self.names.len() {
            return Err(Error::Witness(format!(
                "the key takes {} inputs, not {}",
                self.names.len(),
                witness.len()
            )));
        }
        Ok(witness::encode(witness))
    }
}

/// The proof from the kernel's parts, verified.
fn finish(
    parts: &[u8],
    witness: &[Fr],
    public: usize,
    fixed: &PreparedFixedPoints,
    verifying_key: &VerifyingKey,
    prove_ms: u64,
) -> Result<Proved, Error> {
    let started = zopro_core::time::Instant::now();
    let public = witness[..public].to_vec();
    let proof = Proof::from_raw(&fixed.assemble(parts)).map_err(|_| Error::Unverified)?;
    if !verifying_key.verify(&proof, &public) {
        return Err(Error::Unverified);
    }
    Ok(Proved {
        proof,
        public,
        prove_ms,
        verify_ms: started.elapsed().as_millis() as u64,
    })
}

#[cfg(target_arch = "wasm32")]
pub mod web;
