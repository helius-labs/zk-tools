//! Prove assembled SPR transfer and merge inputs with zopro. The proof is
//! verified before returning. JSON entry points support the existing wire
//! format for fixture and Wasm consumers.
//!
//! **Experimental.** New and unaudited: checked against gnark and arkworks
//! on the devices we have, every proof verified before it is returned, no
//! independent review yet. Do not rely on it where a wrong or leaked proof
//! would cost anything.
//!
//! The client does only what the Go glue did: decode and check the request
//! and place its values on the circuit's inputs by name; `zopro-prover`
//! solves, proves and verifies. A Zolana key's metadata (`Metadata`) gives
//! its shape and the sha256 of the gnark key it was built from, which proofs
//! report and the SDKs check against their pins.
pub use zopro_bn254::Fr;
use zopro_prover::{
    Assignment,
    zopro_core::{
        Points,
        artifact::Artifact,
        verify::{Proof, public_witness},
    },
};

mod merge;
pub mod proof_json;
pub mod request;
mod transfer;

pub use merge::{MergeInput, MergeInputs};
pub use request::Error;
pub use transfer::{
    CacheInputs, TransferInput, TransferInputs, TransferKind, TransferOutput, TreeSlot, Utxo,
};
/// The generic prover and the kernel, for hosts that read key files and
/// cache solver images.
pub use zopro_prover::{self, zopro_core};

/// The circuits' assignments, for tests against the Go prover's witnesses.
#[doc(hidden)]
pub mod test_support {
    use crate::{Assignment, Circuit, Error, TransferKind};

    pub fn transfer(
        json: &serde_json::Value,
        circuit: Circuit,
        inputs: u32,
        outputs: u32,
        names: &[String],
    ) -> Result<Assignment, Error> {
        crate::transfer::assign(
            &crate::transfer::decode(json, TransferKind::from_circuit(circuit)?)?,
            inputs,
            outputs,
            names,
        )
    }

    pub fn merge(
        json: &serde_json::Value,
        circuit: Circuit,
        inputs: u32,
    ) -> Result<Assignment, Error> {
        crate::merge::assign(&crate::merge::decode(json, circuit)?, inputs)
    }
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

/// A circuit's assembled native inputs. These fields can contain secrets.
pub enum ProvingInput {
    Transfer(TransferInputs),
    Merge(MergeInputs),
}

impl ProvingInput {
    /// Decode the existing wire format at an HTTP, Wasm, or fixture boundary.
    pub fn from_json(request: &str) -> Result<Self, Error> {
        let json = request::parse(request)?;
        Self::decode(&json, Self::json_circuit(&json)?)
    }

    fn json_circuit(json: &serde_json::Value) -> Result<Circuit, Error> {
        let kind = json["circuitType"].as_str().ok_or(Error::Request)?;
        Circuit::from_type(kind).ok_or(Error::Unsupported)
    }

    fn decode(json: &serde_json::Value, circuit: Circuit) -> Result<Self, Error> {
        match circuit {
            Circuit::Merge | Circuit::MergeRing => Ok(Self::Merge(merge::decode(json, circuit)?)),
            _ => Ok(Self::Transfer(transfer::decode(
                json,
                TransferKind::from_circuit(circuit)?,
            )?)),
        }
    }

    pub fn circuit(&self) -> Circuit {
        match self {
            Self::Transfer(input) => input.kind.circuit(),
            Self::Merge(input) => input.circuit(),
        }
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

    /// Load an immutable key and cache its solver image and GPU pipelines
    /// beside it. See [`zopro_prover::Prover::load_cached`].
    ///
    /// # Safety
    ///
    /// The key and cache directory must be trusted, and no mapped file may be
    /// modified in place or truncated while it is mapped.
    #[cfg(all(feature = "disk-cache", not(target_arch = "wasm32")))]
    pub unsafe fn load_cached(path: &std::path::Path) -> Result<Self, String> {
        // SAFETY: the caller guarantees the mapped files remain immutable.
        Self::new(unsafe { zopro_prover::Prover::load_cached(path) }?)
    }

    /// From a parsed key, as `zopro_prover::Prover::from_artifact`.
    pub fn from_artifact(
        artifact: &Artifact,
        points: Points,
        solver_image: Option<&[u8]>,
    ) -> Result<Self, String> {
        Self::new(zopro_prover::Prover::from_artifact(
            artifact,
            points,
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

    /// Compatibility entry point for a request in the wire format.
    pub fn witness(&self, request: &str) -> Result<Vec<Fr>, Error> {
        let json = request::parse(request)?;
        let circuit = ProvingInput::json_circuit(&json)?;
        if circuit != self.circuit {
            return Err(Error::Shape);
        }
        self.witness_input(&ProvingInput::decode(&json, circuit)?)
    }

    /// The witness for assembled native inputs, in the loaded key's order.
    pub fn witness_input(&self, input: &ProvingInput) -> Result<Vec<Fr>, Error> {
        if input.circuit() != self.circuit {
            return Err(Error::Shape);
        }
        let (inputs, outputs) = self.shape();
        let assignment = match input {
            ProvingInput::Transfer(input) => {
                transfer::assign(input, inputs, outputs, self.prover.names())?
            }
            ProvingInput::Merge(input) => merge::assign(input, inputs)?,
        };
        self.prover.witness(&assignment).map_err(|_| Error::Shape)
    }

    /// Compatibility entry point for a request in the wire format.
    pub fn prove(&mut self, request: &str) -> Result<ProofResult, Error> {
        let started = zopro_prover::zopro_core::time::Instant::now();
        let witness = self.witness(request)?;
        self.prove_witness(&witness, started.elapsed().as_millis() as u64)
    }

    /// Prove assembled native inputs and verify the proof before returning it.
    pub fn prove_input(&mut self, input: &ProvingInput) -> Result<ProofResult, Error> {
        let started = zopro_prover::zopro_core::time::Instant::now();
        let witness = self.witness_input(input)?;
        self.prove_witness(&witness, started.elapsed().as_millis() as u64)
    }

    fn prove_witness(&mut self, witness: &[Fr], witness_ms: u64) -> Result<ProofResult, Error> {
        let proved = self
            .prover
            .prove(witness)
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
