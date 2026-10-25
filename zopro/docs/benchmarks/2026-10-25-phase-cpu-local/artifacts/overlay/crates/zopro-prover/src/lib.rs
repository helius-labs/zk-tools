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
    artifact::Artifact,
    groth16::PreparedFixedPoints,
    verify::{public_witness, Proof, VerifyingKey},
    Key, Points,
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

    /// Proves on the CPU from now on.
    pub fn drop_gpu(&mut self) {
        self.key.drop_gpu();
    }

    /// The witness vector of `assignment`, in the key's order.
    pub fn witness(&self, assignment: &Assignment) -> Result<Vec<Fr>, Error> {
        witness::order(assignment, &self.names)
    }

    /// Proves `witness` (the key's order): the proof is verified before it
    /// is returned; a GPU proof that fails, or fails to verify, is proved
    /// again on the CPU, and the GPU is let go.
    pub fn prove(&mut self, witness: &[Fr]) -> Result<Proved, Error> {
        #[cfg(zopro_phase_cpu)] zopro_core::phase_cpu::mark("encode");
        let encoded = self.encode(witness)?;
        loop {
            if !self.key.has_gpu() {
                let started = zopro_core::time::Instant::now();
                let raw = self
                    .key
                    .solve_rerandomized(&encoded, &self.fixed)
                    .map_err(Error::Prove)?;
                let prove_ms = started.elapsed().as_millis() as u64;
                let verify_started = zopro_core::time::Instant::now();
                #[cfg(zopro_phase_cpu)] zopro_core::phase_cpu::mark("verify");
                let public = witness[..self.public].to_vec();
                let proof = Proof::from_raw(&raw).map_err(|_| Error::Unverified)?;
                if !self.verifying_key.verify(&proof, &public) {
                    return Err(Error::Unverified);
                }
                #[cfg(zopro_phase_cpu)] zopro_core::phase_cpu::mark("sdk_return");
                return Ok(Proved {
                    proof,
                    public,
                    prove_ms,
                    verify_ms: verify_started.elapsed().as_millis() as u64,
                });
            }
            let started = zopro_core::time::Instant::now();
            let parts = match self.key.solve(&encoded) {
                Ok(parts) => parts,
                Err(error) if self.key.has_gpu() => {
                    log::warn!("zopro: the GPU failed ({error}); proving on the CPU from now on");
                    self.key.drop_gpu();
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
                Err(_) if self.key.has_gpu() => {
                    log::warn!("zopro: a GPU proof did not verify; proving on the CPU from now on");
                    self.key.drop_gpu();
                }
                result => return result,
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
