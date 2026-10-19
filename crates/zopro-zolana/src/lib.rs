//! Zolana's proof requests on zopro: the SDKs' `/prove` request (the JSON
//! gnark's Go prover took) and a zopro key built from Zolana's gnark key
//! give the proof the prover server returns, verified before it leaves.
//!
//! The client does only what the Go glue did: decode and check the request
//! and place its values on the circuit's inputs by name; `zopro-prover`
//! solves, proves and verifies. A Zolana key's metadata (`Metadata`) gives
//! its shape and the sha256 of the gnark key it was built from, which proofs
//! report and the SDKs check against their pins.
use zopro_bn254::Fr;
use zopro_prover::{
    Assignment,
    zopro_core::{
        artifact::Artifact,
        verify::{Proof, public_witness},
    },
};

mod merge;
pub mod proof_json;
pub mod request;
mod transfer;

pub use request::Error;
/// The generic prover and the kernel, for hosts that read key files and
/// cache solver images.
pub use zopro_prover::{self, zopro_core};

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

/// What a Zolana key keeps in its metadata, as JSON: `{"inputs": 2,
/// "outputs": 2, "keySha256": "<64 hex digits>"}`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub inputs: u32,
    pub outputs: u32,
    /// The sha256 of the gnark key file the zopro key was built from.
    pub key_sha256: String,
}

impl Metadata {
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        let json: serde_json::Value =
            serde_json::from_slice(bytes).map_err(|_| "a Zolana key's metadata is JSON")?;
        let count = |field: &str| {
            json[field]
                .as_u64()
                .and_then(|v| u32::try_from(v).ok())
                .ok_or_else(|| format!("the metadata's {field}"))
        };
        let key_sha256 = json["keySha256"]
            .as_str()
            .filter(|s| {
                s.len() == 64
                    && s.bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            })
            .ok_or("the metadata's keySha256")?
            .to_owned();
        Ok(Self {
            inputs: count("inputs")?,
            outputs: count("outputs")?,
            key_sha256,
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::json!({
            "inputs": self.inputs,
            "outputs": self.outputs,
            "keySha256": self.key_sha256,
        })
        .to_string()
    }
}

/// A proof as the SDKs return it.
#[derive(Clone, Debug)]
pub struct ProofResult {
    /// gnark's compressed proof (`Proof.WriteTo`).
    pub proof: Vec<u8>,
    /// gnark's binary public witness.
    pub public_inputs: Vec<u8>,
    /// The proof as the prover server returns it (`proof_json`).
    pub proof_json: serde_json::Value,
    /// Decoding the request and placing its values.
    pub witness_ms: u64,
    /// Solving, the MSMs and the assembly.
    pub prove_ms: u64,
    /// Checking the proof.
    pub verify_ms: u64,
}

/// A loaded Zolana key, ready to prove requests for its circuit.
pub struct Prover {
    prover: zopro_prover::Prover,
    circuit: Circuit,
    metadata: Metadata,
}

impl Prover {
    /// From a zopro key file.
    pub fn load(bytes: &[u8]) -> Result<Self, String> {
        Self::new(zopro_prover::Prover::load(bytes)?)
    }

    /// From a parsed key, as `zopro_prover::Prover::from_artifact`.
    pub fn from_artifact(
        artifact: &Artifact,
        vectors: &[Vec<u8>; 5],
        solver_image: Option<&[u8]>,
    ) -> Result<Self, String> {
        Self::new(zopro_prover::Prover::from_artifact(
            artifact,
            vectors,
            solver_image,
        )?)
    }

    /// Over a loaded key that a Zolana gnark key was packed into.
    pub fn new(prover: zopro_prover::Prover) -> Result<Self, String> {
        let circuit = Circuit::from_key_name(prover.name())
            .ok_or_else(|| format!("no circuit for a key named {}", prover.name()))?;
        let metadata = Metadata::parse(prover.metadata())?;
        Ok(Self {
            prover,
            circuit,
            metadata,
        })
    }

    pub fn name(&self) -> &str {
        self.prover.name()
    }

    pub fn circuit(&self) -> Circuit {
        self.circuit
    }

    /// The circuit's shape: inputs and outputs.
    pub fn shape(&self) -> (u32, u32) {
        (self.metadata.inputs, self.metadata.outputs)
    }

    /// The sha256 of the gnark key the zopro key was built from.
    pub fn key_sha256(&self) -> &str {
        &self.metadata.key_sha256
    }

    /// The kernel's solver image, for a host to cache beside the key.
    pub fn solver_image(&self) -> Option<Vec<u8>> {
        self.prover.solver_image()
    }

    pub fn prover(&self) -> &zopro_prover::Prover {
        &self.prover
    }

    pub fn prover_mut(&mut self) -> &mut zopro_prover::Prover {
        &mut self.prover
    }

    /// The witness the request assigns, in the key's order.
    pub fn witness(&self, request: &str) -> Result<Vec<Fr>, Error> {
        let json = request::parse(request)?;
        let circuit_type = json["circuitType"].as_str().ok_or(Error::Request)?;
        let circuit = Circuit::from_type(circuit_type).ok_or(Error::Unsupported)?;
        if circuit != self.circuit {
            return Err(Error::Shape);
        }
        let (inputs, outputs) = self.shape();
        let assignment: Assignment = match circuit {
            Circuit::TransferConfidential
            | Circuit::TransferRing
            | Circuit::TransferRingAuthority => {
                transfer::assign(&json, circuit, inputs, outputs, self.prover.names())?
            }
            Circuit::Merge | Circuit::MergeRing => merge::assign(&json, circuit, inputs)?,
        };
        self.prover.witness(&assignment).map_err(|_| Error::Shape)
    }

    /// Proves `request`: the proof is verified before it is returned, a GPU
    /// proof that fails is proved again on the CPU.
    pub fn prove(&mut self, request: &str) -> Result<ProofResult, Error> {
        let started = zopro_prover::zopro_core::time::Instant::now();
        let witness = self.witness(request)?;
        let witness_ms = started.elapsed().as_millis() as u64;
        let proved = self
            .prover
            .prove(&witness)
            .map_err(|e| Error::Prove(e.to_string()))?;
        Ok(self.result(proved, witness_ms))
    }

    /// The SDKs' result for a proof of this key.
    pub fn result(&self, proved: zopro_prover::Proved, witness_ms: u64) -> ProofResult {
        ProofResult {
            proof: proved.proof.to_gnark(),
            public_inputs: public_witness(&proved.public),
            proof_json: proof_json::to_json(&proved.proof, Some(&self.metadata.key_sha256)),
            witness_ms,
            prove_ms: proved.prove_ms,
            verify_ms: proved.verify_ms,
        }
    }

    /// Whether `proof` proves the public input `public`.
    pub fn verify(&self, proof: &Proof, public: &[Fr]) -> bool {
        self.prover.verify(proof, public)
    }
}

#[cfg(target_arch = "wasm32")]
pub mod web;
