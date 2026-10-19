//! Zolana's Groth16 prover in Rust alone: a zopro key (`zopro_core::artifact`)
//! and a proof request as the SDKs send them (the same JSON gnark's Go
//! prover took) give a proof, verified before it leaves, in gnark's formats
//! for the chain and gnark's verifier.
//!
//! gnark stays the reference offline — the circuits are gnark circuits and
//! the keys come from them — so the client does only what the Go glue did:
//! decode and check the request, place its values on the circuit's inputs
//! by name, solve, prove, verify, encode.
use ark_bn254::Fr;
use zopro_core::{
    Key,
    artifact::Artifact,
    groth16::{self, FixedPoints},
    verify::{Proof, VerifyingKey, public_witness},
};

mod merge;
pub mod request;
mod transfer;
pub mod witness;

pub use request::Error;
/// The kernel, for hosts that cache its solver images and read key files.
pub use zopro_core;

/// The circuits' assignments, for tests against the Go prover's witnesses.
#[doc(hidden)]
pub mod test_support {
    pub use crate::merge::assign as merge;
    pub use crate::transfer::assign as transfer;
}

/// The circuits the SDKs prove with, by the request's `circuitType` and
/// the key's name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Circuit {
    TransferConfidential,
    TransferRing,
    TransferRingAuthority,
    Merge,
    MergeRing,
}

impl Circuit {
    pub fn from_type(circuit_type: &str) -> Option<Self> {
        Some(match circuit_type {
            "transfer-confidential" => Self::TransferConfidential,
            "transfer-ring" => Self::TransferRing,
            "transfer-ring-authority" => Self::TransferRingAuthority,
            "merge" => Self::Merge,
            "merge-ring" => Self::MergeRing,
            _ => return None,
        })
    }

    /// From a key's name: `transfer_confidential_2_2`, `merge_24_1`,
    /// `transfer_ring_authority_2_2`, ...
    pub fn from_key_name(name: &str) -> Option<Self> {
        Some(if name.starts_with("transfer_confidential_") {
            Self::TransferConfidential
        } else if name.starts_with("transfer_ring_authority_") {
            Self::TransferRingAuthority
        } else if name.starts_with("transfer_ring_") {
            Self::TransferRing
        } else if name.starts_with("merge_ring_") {
            Self::MergeRing
        } else if name.starts_with("merge_") {
            Self::Merge
        } else {
            return None;
        })
    }
}

/// A proof as the SDKs return it.
#[derive(Clone, Debug)]
pub struct ProofResult {
    /// gnark's compressed proof (`Proof.WriteTo`).
    pub proof: Vec<u8>,
    /// gnark's binary public witness.
    pub public_inputs: Vec<u8>,
    /// The proof's coordinates in the SDKs' JSON.
    pub proof_json: serde_json::Value,
    /// Decoding the request and placing its values.
    pub witness_ms: u64,
    /// Solving, the MSMs and the assembly.
    pub prove_ms: u64,
    /// Checking the proof.
    pub verify_ms: u64,
}

/// A loaded zopro key, ready to prove requests for its circuit.
pub struct Prover {
    key: Key,
    verifying_key: VerifyingKey,
    fixed: FixedPoints,
    circuit: Circuit,
    name: String,
    inputs: u32,
    outputs: u32,
    /// The inputs' names in the key's order, the constant one apart.
    names: Vec<String>,
    /// The key's digest, 64 hex digits, for the proof JSON.
    digest: String,
}

impl Prover {
    /// From a zopro key file.
    pub fn load(bytes: &[u8]) -> Result<Self, String> {
        let artifact = Artifact::parse(bytes)?;
        let vectors = artifact.vectors()?;
        Self::from_artifact(&artifact, &vectors, None)
    }

    /// From a parsed key, its vectors decompressed (by `Artifact::vectors`
    /// or a host's cache of them) and a cached solver image when the host
    /// kept one (`solver_image`).
    pub fn from_artifact(
        artifact: &Artifact,
        vectors: &[Vec<u8>; 5],
        solver_image: Option<&[u8]>,
    ) -> Result<Self, String> {
        let circuit = Circuit::from_key_name(artifact.name)
            .ok_or_else(|| format!("no circuit for a key named {}", artifact.name))?;
        let key = artifact.key(vectors, solver_image)?;
        Ok(Self {
            key,
            verifying_key: artifact.verifying_key()?,
            fixed: artifact.fixed_points()?,
            circuit,
            name: artifact.name.to_owned(),
            inputs: artifact.inputs,
            outputs: artifact.outputs,
            names: artifact
                .input_names()
                .iter()
                .map(|n| (*n).to_owned())
                .collect(),
            digest: zopro_core::verify::hex_of(&artifact.digest),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn circuit(&self) -> Circuit {
        self.circuit
    }

    /// The circuit's shape: inputs and outputs.
    pub fn shape(&self) -> (u32, u32) {
        (self.inputs, self.outputs)
    }

    /// The kernel's solver image, for a host to cache beside the key.
    pub fn solver_image(&self) -> Option<Vec<u8>> {
        self.key.solver_image()
    }

    /// The witness the request assigns, in the key's order: for tests and
    /// tools; `prove` builds it itself.
    pub fn witness(&self, request: &str) -> Result<Vec<Fr>, Error> {
        let assignment = self.assignment(request)?;
        witness::order(&assignment, &self.names)
    }

    fn assignment(&self, request: &str) -> Result<witness::Assignment, Error> {
        let json = request::parse(request)?;
        let circuit_type = json["circuitType"].as_str().ok_or(Error::Request)?;
        let circuit = Circuit::from_type(circuit_type).ok_or(Error::Unsupported)?;
        if circuit != self.circuit {
            return Err(Error::Shape);
        }
        match circuit {
            Circuit::TransferConfidential
            | Circuit::TransferRing
            | Circuit::TransferRingAuthority => {
                transfer::assign(&json, circuit, self.inputs, self.outputs, &self.names)
            }
            Circuit::Merge | Circuit::MergeRing => merge::assign(&json, circuit, self.inputs),
        }
    }

    /// The request's witness, encoded for the kernel, with its public
    /// values; the first step of a proof.
    fn prepare(&self, request: &str) -> Result<Prepared, Error> {
        let started = zopro_core::time::Instant::now();
        let values = self.witness(request)?;
        let public: Vec<Fr> = values[..self.names.len() - secret_count(&self.names)].to_vec();
        let encoded = witness::encode(&values);
        Ok(Prepared {
            public,
            encoded,
            witness_ms: started.elapsed().as_millis() as u64,
        })
    }

    /// Proves `request`: the proof is verified with the key's verifying key
    /// before it is returned; a GPU proof that fails to verify is proved
    /// again on the CPU, and the GPU is let go.
    pub fn prove(&mut self, request: &str) -> Result<ProofResult, Error> {
        let prepared = self.prepare(request)?;
        let (mut prove_ms, mut verify_ms) = (0, 0);
        loop {
            let started = zopro_core::time::Instant::now();
            let parts = self.key.solve(&prepared.encoded).map_err(Error::Prove)?;
            prove_ms += started.elapsed().as_millis() as u64;
            match self.finish(&parts, &prepared, prove_ms, verify_ms) {
                Ok(result) => return Ok(result),
                Err(Unverified { verify_ms: ms }) if self.key.has_gpu() => {
                    verify_ms = ms;
                    log::warn!("zopro: a GPU proof did not verify; proving on the CPU from now on");
                    self.key.drop_gpu();
                }
                Err(_) => return Err(Error::Prove("the proof did not verify".into())),
            }
        }
    }

    /// The proof from the kernel's parts, verified and encoded.
    fn finish(
        &self,
        parts: &[u8],
        prepared: &Prepared,
        prove_ms: u64,
        verify_ms: u64,
    ) -> Result<ProofResult, Unverified> {
        finish(
            parts,
            prepared,
            &self.fixed,
            &self.verifying_key,
            &self.digest,
            prove_ms,
            verify_ms,
        )
    }

    /// Whether a proof (gnark's compressed encoding or the SDKs' JSON) proves
    /// the request's public input.
    pub fn verify(&self, proof: &Proof, public: &[Fr]) -> bool {
        self.verifying_key.verify(proof, public)
    }
}

/// How many of the names are secret: every circuit has one public input,
/// its hash, first.
fn secret_count(names: &[String]) -> usize {
    names.len() - 1
}

/// A request's witness, ready for the kernel.
struct Prepared {
    public: Vec<Fr>,
    encoded: Vec<u8>,
    witness_ms: u64,
}

/// A proof that did not verify, with the time the check took.
struct Unverified {
    verify_ms: u64,
}

/// The proof from the kernel's parts, verified and encoded.
fn finish(
    parts: &[u8],
    prepared: &Prepared,
    fixed: &FixedPoints,
    verifying_key: &VerifyingKey,
    digest: &str,
    prove_ms: u64,
    verify_ms: u64,
) -> Result<ProofResult, Unverified> {
    let started = zopro_core::time::Instant::now();
    let raw = groth16::assemble(parts, fixed);
    let proof = Proof::from_raw(&raw).ok();
    let verified = proof.is_some_and(|proof| verifying_key.verify(&proof, &prepared.public));
    let verify_ms = verify_ms + started.elapsed().as_millis() as u64;
    match (verified, proof) {
        (true, Some(proof)) => Ok(ProofResult {
            proof: proof.to_gnark(),
            public_inputs: public_witness(&prepared.public),
            proof_json: proof.to_json(Some(digest)),
            witness_ms: prepared.witness_ms,
            prove_ms,
            verify_ms,
        }),
        _ => Err(Unverified { verify_ms }),
    }
}

#[cfg(target_arch = "wasm32")]
pub mod web;
