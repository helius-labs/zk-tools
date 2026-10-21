#[cfg(feature = "client")]
use core::marker::PhantomData;
#[cfg(any(feature = "setup", not(target_arch = "wasm32")))]
use std::path::Path;
#[cfg(feature = "client")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "client")]
use ark_bn254::Fr;
use ark_bn254::{Bn254, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
#[cfg(feature = "client")]
use ark_std::{rand::rngs::OsRng, UniformRand};
use groth16_solana::groth16::Groth16Verifyingkey;

#[cfg(feature = "setup")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetupKind {
    /// A verified imported ceremony key.
    Production,
    /// Fresh randomness sampled locally for a single-party setup.
    SingleParty,
    /// Reproducible setup whose secret randomness is public.
    InsecureTest,
}

#[cfg(feature = "setup")]
impl SetupKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Production => "imported-ceremony",
            Self::SingleParty => "single-party",
            Self::InsecureTest => "insecure-test",
        }
    }

    fn verifier_kind(self) -> groth16_solana::vk::setup::SetupKind {
        match self {
            Self::Production | Self::SingleParty => {
                groth16_solana::vk::setup::SetupKind::Production
            }
            Self::InsecureTest => groth16_solana::vk::setup::SetupKind::InsecureTest,
        }
    }
}

#[cfg(feature = "client")]
use super::proof_inputs::public_inputs;
#[cfg(feature = "setup")]
use super::r1cs::R1cs;
#[cfg(feature = "client")]
use super::{
    proof::create_proof,
    proof_inputs::ProofInputs,
    synthesis::{ArkworksCircuit, CircuitMatrices, CircuitShape, R1csMatrices},
    zkey::Zkey,
};
use crate::{conversion::be_bytes, ProverError, ProverErrorKind};
#[cfg(feature = "client")]
use crate::{CompressedProof, ZkCircuit};

pub type ProvingKey = ark_groth16::ProvingKey<Bn254>;
pub type VerifyingKey = ark_groth16::VerifyingKey<Bn254>;
pub type Proof = ark_groth16::Proof<Bn254>;

pub struct Groth16Keys {
    proving_key: ProvingKey,
    verifying_key: SolanaVerifyingKey,
    imported_zkey: bool,
    single_party: bool,
    circuit_digest: [u8; 32],
}

impl From<ProvingKey> for Groth16Keys {
    fn from(proving_key: ProvingKey) -> Self {
        Self {
            verifying_key: SolanaVerifyingKey::from(&proving_key.vk),
            proving_key,
            imported_zkey: false,
            single_party: false,
            circuit_digest: [0u8; 32],
        }
    }
}

impl Groth16Keys {
    pub fn verifying_key(&self) -> &SolanaVerifyingKey {
        &self.verifying_key
    }

    pub fn proving_key(&self) -> &ProvingKey {
        &self.proving_key
    }

    pub fn circuit_digest(&self) -> &[u8; 32] {
        &self.circuit_digest
    }

    #[cfg(feature = "build")]
    pub(crate) fn is_imported_zkey(&self) -> bool {
        self.imported_zkey
    }

    #[cfg(feature = "client")]
    fn circuit_shape(&self) -> CircuitShape {
        CircuitShape {
            instance_variables: self.proving_key.vk.gamma_abc_g1.len(),
            witness_variables: self.proving_key.l_query.len(),
        }
    }

    #[cfg(feature = "setup")]
    pub fn setup_kind(&self) -> SetupKind {
        if self.imported_zkey {
            SetupKind::Production
        } else if self.single_party {
            SetupKind::SingleParty
        } else {
            SetupKind::InsecureTest
        }
    }

    /// Serializes the setup provenance, proving key and its circuit digest.
    /// Tags 0 and 1 preserve the previous test/imported bool encoding.
    pub fn to_bytes(&self) -> Result<Vec<u8>, ProverError> {
        let mut bytes = Vec::new();
        let provenance: u8 = if self.imported_zkey {
            1
        } else if self.single_party {
            2
        } else {
            0
        };
        provenance
            .serialize_uncompressed(&mut bytes)
            .map_err(ProverErrorKind::KeySerialization)?;
        self.proving_key
            .serialize_uncompressed(&mut bytes)
            .map_err(ProverErrorKind::KeySerialization)?;
        bytes.extend_from_slice(&self.circuit_digest);
        Ok(bytes)
    }

    /// Reads canonical key bytes, validating curve and subgroup membership.
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, ProverError> {
        let provenance =
            u8::deserialize_uncompressed(&mut bytes).map_err(ProverErrorKind::KeySerialization)?;
        if provenance > 2 {
            return Err(ProverErrorKind::KeySerialization(
                ark_serialize::SerializationError::InvalidData,
            )
            .into());
        }
        let imported_zkey = provenance == 1;
        let proving_key = ProvingKey::deserialize_uncompressed(&mut bytes)
            .map_err(ProverErrorKind::KeySerialization)?;
        let Ok(circuit_digest) = <[u8; 32]>::try_from(bytes) else {
            return Err(ProverErrorKind::KeySerialization(
                ark_serialize::SerializationError::InvalidData,
            )
            .into());
        };
        Ok(Self {
            imported_zkey,
            single_party: provenance == 2,
            circuit_digest,
            ..Self::from(proving_key)
        })
    }

    /// Checks a key against a trusted SHA-256 before deserializing it.
    pub fn from_bytes_checked(bytes: &[u8], checksum: &[u8; 32]) -> Result<Self, ProverError> {
        use sha2::{Digest, Sha256};

        if Sha256::digest(bytes).as_slice() != checksum {
            return Err(ProverErrorKind::KeyChecksumMismatch.into());
        }
        Self::from_bytes(bytes)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_checked(path: &Path, checksum: &[u8; 32]) -> Result<Self, ProverError> {
        Self::from_bytes_checked(
            &std::fs::read(path).map_err(|error| key_file(path, error))?,
            checksum,
        )
    }

    #[cfg(feature = "client")]
    pub fn from_zkey_bytes<P: ZkCircuit + 'static>(bytes: &[u8]) -> Result<Self, ProverError> {
        Self::from_zkey_for(bytes, circuit_matrices::<P>()?.matrices())
    }

    #[cfg(all(feature = "client", not(target_arch = "wasm32")))]
    pub fn load_zkey<P: ZkCircuit + 'static>(path: &Path) -> Result<Self, ProverError> {
        Self::from_zkey_bytes::<P>(&std::fs::read(path).map_err(|error| key_file(path, error))?)
    }

    #[cfg(feature = "setup")]
    pub fn from_zkey_for_r1cs(bytes: &[u8], r1cs: &R1cs) -> Result<Self, ProverError> {
        Self::from_zkey_for(bytes, r1cs.matrices())
    }

    #[cfg(feature = "client")]
    fn from_zkey_for(bytes: &[u8], matrices: &R1csMatrices) -> Result<Self, ProverError> {
        let zkey = Zkey::read(bytes)?;
        zkey.check_circuit(matrices)?;
        let keys = Self::from(zkey.proving_key);
        let keys = Self {
            imported_zkey: true,
            circuit_digest: matrices.digest(),
            ..keys
        };
        Ok(keys)
    }

    /// Generates a single-party setup using fresh cryptographic randomness.
    #[cfg(feature = "setup")]
    pub fn new_with_random_setup(r1cs: &R1cs) -> Result<Self, ProverError> {
        Ok(Self {
            single_party: true,
            ..Self::setup_with_rng(r1cs.matrices(), &mut OsRng)?
        })
    }

    #[cfg(feature = "setup")]
    pub fn new_with_test_setup(r1cs: &R1cs) -> Result<Self, ProverError> {
        Self::test_setup(r1cs.matrices())
    }

    #[cfg(feature = "setup")]
    fn test_setup(matrices: &R1csMatrices) -> Result<Self, ProverError> {
        use ark_std::rand::{rngs::StdRng, SeedableRng};
        Self::setup_with_rng(matrices, &mut StdRng::seed_from_u64(TEST_SETUP_SEED))
    }

    #[cfg(feature = "setup")]
    fn setup_with_rng(
        matrices: &R1csMatrices,
        rng: &mut (impl ark_std::rand::RngCore + ark_std::rand::CryptoRng),
    ) -> Result<Self, ProverError> {
        use super::{reduction::CircomReduction, synthesis::Replay};
        use ark_groth16::Groth16;

        Ok(Self {
            circuit_digest: matrices.digest(),
            ..Self::from(
                Groth16::<Bn254, CircomReduction>::generate_random_parameters_with_reduction(
                    Replay(matrices),
                    rng,
                )?,
            )
        })
    }

    #[cfg(feature = "setup")]
    pub fn gnark_verifying_key(&self) -> Result<Vec<u8>, ProverError> {
        let vk = &self.proving_key.vk;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&g1_bytes(&vk.alpha_g1));
        bytes.extend_from_slice(&g1_bytes(&self.proving_key.beta_g1));
        bytes.extend_from_slice(&g2_bytes(&vk.beta_g2));
        bytes.extend_from_slice(&g2_bytes(&vk.gamma_g2));
        bytes.extend_from_slice(&g1_bytes(&self.proving_key.delta_g1));
        bytes.extend_from_slice(&g2_bytes(&vk.delta_g2));
        let ic_len = u32::try_from(vk.gamma_abc_g1.len()).map_err(|_| {
            ProverErrorKind::ExportTooLarge("the verifying key has too many points")
        })?;
        bytes.extend_from_slice(&ic_len.to_be_bytes());
        for point in &vk.gamma_abc_g1 {
            bytes.extend_from_slice(&g1_bytes(point));
        }
        bytes.extend_from_slice(&0u32.to_be_bytes());
        bytes.extend_from_slice(&0u32.to_be_bytes());
        Ok(bytes)
    }

    #[cfg(feature = "setup")]
    pub fn export_verifying_key(&self, export: &VerifyingKeyExport<'_>) -> Result<(), ProverError> {
        use groth16_solana::vk::circom::parse_vk_json_to_rust_string;
        use sha2::{Digest, Sha256};

        if export.setup != self.setup_kind() {
            return Err(ProverErrorKind::SetupKindMismatch.into());
        }
        if matches!(export.setup, SetupKind::Production | SetupKind::SingleParty) {
            if matches!(export.setup, SetupKind::Production) && !self.imported_zkey {
                return Err(ProverErrorKind::ProductionKeyRequired.into());
            }
            if self.circuit_digest == [0u8; 32] {
                return Err(ProverErrorKind::ProductionCircuitDigestRequired.into());
            }
        }
        let checksum: [u8; 32] = Sha256::digest(self.to_bytes()?).into();
        let mut source = parse_vk_json_to_rust_string(
            &circom_vk_json(&self.proving_key.vk).to_string(),
            export.setup.verifier_kind(),
            &checksum,
        )
        .map_err(ProverErrorKind::VerifyingKeyExport)?;
        source.push_str(&format!(
            "\npub const ZOLANA_SETUP_KIND: &str = \"{}\";\n",
            export.setup.label()
        ));
        source.push_str(&format!(
            "\npub const ZOLANA_CIRCUIT_SHA256: &str = \"{}\";\n",
            hex::encode(self.circuit_digest)
        ));
        std::fs::create_dir_all(export.output_dir)
            .map_err(|error| key_file(export.output_dir, error))?;
        let output = export.output_dir.join(export.output_filename);
        Ok(std::fs::write(&output, source).map_err(|error| key_file(&output, error))?)
    }
}

#[cfg(feature = "setup")]
pub struct VerifyingKeyExport<'a> {
    pub output_dir: &'a Path,
    pub output_filename: &'a str,
    pub setup: SetupKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolanaVerifyingKey {
    pub alpha_g1: [u8; 64],
    pub beta_g2: [u8; 128],
    pub gamma_g2: [u8; 128],
    pub delta_g2: [u8; 128],
    pub ic: Vec<[u8; 64]>,
}

impl From<&VerifyingKey> for SolanaVerifyingKey {
    fn from(vk: &VerifyingKey) -> Self {
        Self {
            alpha_g1: g1_bytes(&vk.alpha_g1),
            beta_g2: g2_bytes(&vk.beta_g2),
            gamma_g2: g2_bytes(&vk.gamma_g2),
            delta_g2: g2_bytes(&vk.delta_g2),
            ic: vk.gamma_abc_g1.iter().map(g1_bytes).collect(),
        }
    }
}

impl<'a> From<&'a SolanaVerifyingKey> for Groth16Verifyingkey<'a> {
    fn from(verifying_key: &'a SolanaVerifyingKey) -> Self {
        Self {
            nr_pubinputs: verifying_key.ic.len().saturating_sub(1),
            vk_alpha_g1: verifying_key.alpha_g1,
            vk_beta_g2: verifying_key.beta_g2,
            vk_gamma_g2: verifying_key.gamma_g2,
            vk_delta_g2: verifying_key.delta_g2,
            vk_ic: &verifying_key.ic,
            vk_commitment: None,
        }
    }
}

#[cfg(feature = "client")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SolanaProof {
    pub a: [u8; 64],
    pub b: [u8; 128],
    pub c: [u8; 64],
}

#[cfg(feature = "client")]
impl From<&Proof> for SolanaProof {
    fn from(proof: &Proof) -> Self {
        let negated_a: G1Affine = (-proof.a.into_group()).into();
        Self {
            a: g1_bytes(&negated_a),
            b: g2_bytes(&proof.b),
            c: g1_bytes(&proof.c),
        }
    }
}

#[cfg(feature = "client")]
impl SolanaProof {
    pub fn verify(
        &self,
        verifying_key: &SolanaVerifyingKey,
        public_inputs: &[[u8; 32]],
    ) -> Result<(), ProverError> {
        verify_groth16(
            self,
            public_inputs,
            &Groth16Verifyingkey::from(verifying_key),
        )
    }
}

/// The most public inputs a proof verified off-chain may carry. The
/// groth16-solana verifier takes its input count as a const generic, so each
/// count up to this bound is one monomorphized instance.
#[cfg(feature = "client")]
pub const MAX_PUBLIC_INPUTS: usize = 32;

/// Verifies an uncompressed proof whose public input count is only known at
/// run time, such as one handed in from JavaScript.
#[cfg(feature = "client")]
pub(crate) fn verify_groth16(
    proof: &SolanaProof,
    public_inputs: &[[u8; 32]],
    verifying_key: &Groth16Verifyingkey,
) -> Result<(), ProverError> {
    macro_rules! dispatch {
        ($($count:literal)*) => {
            match public_inputs.len() {
                $($count => verify_count::<$count>(proof, public_inputs, verifying_key),)*
                count => Err(ProverErrorKind::TooManyPublicInputs {
                    count,
                    max: MAX_PUBLIC_INPUTS,
                }
                .into()),
            }
        };
    }
    dispatch!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32)
}

#[cfg(feature = "client")]
fn verify_count<const N: usize>(
    proof: &SolanaProof,
    public_inputs: &[[u8; 32]],
    verifying_key: &Groth16Verifyingkey,
) -> Result<(), ProverError> {
    use groth16_solana::groth16::Groth16Verifier;

    let public_inputs = <&[[u8; 32]; N]>::try_from(public_inputs)
        .map_err(|_| ProverErrorKind::WrongPublicInputCount)?;
    Ok(
        Groth16Verifier::new(&proof.a, &proof.b, &proof.c, public_inputs, verifying_key)
            .and_then(|mut verifier| verifier.verify())
            .map_err(|_| ProverErrorKind::ProofRejected)?,
    )
}

#[cfg(feature = "client")]
impl TryFrom<&SolanaProof> for CompressedProof {
    type Error = ProverError;

    fn try_from(proof: &SolanaProof) -> Result<Self, ProverError> {
        use solana_bn254::compression::prelude::{
            alt_bn128_g1_compress_be, alt_bn128_g2_compress_be,
        };

        let invalid = |_| ProverErrorKind::CorruptProof;
        Ok(Self {
            a: alt_bn128_g1_compress_be(&proof.a).map_err(invalid)?,
            b: alt_bn128_g2_compress_be(&proof.b).map_err(invalid)?,
            c: alt_bn128_g1_compress_be(&proof.c).map_err(invalid)?,
        })
    }
}

#[cfg(feature = "client")]
impl CompressedProof {
    pub fn verify(
        &self,
        verifying_key: &SolanaVerifyingKey,
        public_inputs: &[[u8; 32]],
    ) -> Result<(), ProverError> {
        use solana_bn254::compression::prelude::{
            alt_bn128_g1_decompress_be, alt_bn128_g2_decompress_be,
        };

        let invalid = |_| ProverErrorKind::CorruptProof;
        SolanaProof {
            a: alt_bn128_g1_decompress_be(&self.a).map_err(invalid)?,
            b: alt_bn128_g2_decompress_be(&self.b).map_err(invalid)?,
            c: alt_bn128_g1_decompress_be(&self.c).map_err(invalid)?,
        }
        .verify(verifying_key, public_inputs)
    }
}

#[cfg(feature = "client")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofResult {
    pub proof: SolanaProof,
    /// The proof's public inputs in order, as 32-byte big-endian values.
    pub public_inputs: Vec<[u8; 32]>,
}

#[cfg(feature = "client")]
impl ProofResult {
    pub fn compressed(&self) -> Result<CompressedProof, ProverError> {
        CompressedProof::try_from(&self.proof)
    }
}

#[cfg(feature = "client")]
pub struct Groth16Prover<P> {
    keys: Groth16Keys,
    matrices: Arc<CircuitMatrices>,
    program: PhantomData<fn() -> P>,
}

#[cfg(feature = "client")]
impl<P: ZkCircuit + 'static> Groth16Prover<P> {
    pub fn new(keys: Groth16Keys) -> Result<Self, ProverError> {
        Self::with_matrices(keys, circuit_matrices::<P>()?)
    }

    pub fn from_zkey_bytes(zkey: &[u8]) -> Result<Self, ProverError> {
        let matrices = circuit_matrices::<P>()?;
        let keys = Groth16Keys::from_zkey_for(zkey, matrices.matrices())?;
        Self::with_matrices(keys, matrices)
    }

    fn with_matrices(
        keys: Groth16Keys,
        matrices: Arc<CircuitMatrices>,
    ) -> Result<Self, ProverError> {
        if matrices.shape() != keys.circuit_shape()
            || matrices.matrices().digest() != keys.circuit_digest
        {
            return Err(ProverErrorKind::KeysForAnotherCircuit.into());
        }
        Ok(Self {
            keys,
            matrices,
            program: PhantomData,
        })
    }

    #[cfg(feature = "setup")]
    pub fn new_with_test_setup() -> Result<Self, ProverError> {
        let matrices = circuit_matrices::<P>()?;
        let keys = Groth16Keys::test_setup(matrices.matrices())?;
        Self::with_matrices(keys, matrices)
    }

    pub fn keys(&self) -> &Groth16Keys {
        &self.keys
    }

    pub fn constraint_count(&self) -> usize {
        self.matrices.constraint_count()
    }

    pub fn prove(&self, proof_inputs: &P) -> Result<ProofResult, ProverError> {
        self.prove_inputs(&ArkworksCircuit::new(proof_inputs)?.proof_inputs()?)
    }

    pub fn prove_inputs(&self, proof_inputs: &ProofInputs) -> Result<ProofResult, ProverError> {
        let assignment = proof_inputs.values();
        self.matrices.check(assignment)?;
        let proof = SolanaProof::from(&create_proof(
            &self.keys.proving_key,
            Fr::rand(&mut OsRng),
            Fr::rand(&mut OsRng),
            self.matrices.matrices(),
            assignment,
        )?);
        // The circuit matrices establish the public input count and `check`
        // above validates the assignment against them. WTNS alone has no such
        // layout.
        let public_inputs = public_inputs(assignment, self.matrices.public_inputs())?;
        proof.verify(&self.keys.verifying_key, &public_inputs)?;
        Ok(ProofResult {
            proof,
            public_inputs,
        })
    }

    pub fn verify(&self, result: &ProofResult) -> Result<(), ProverError> {
        CompressedProof::try_from(&result.proof)?
            .verify(&self.keys.verifying_key, &result.public_inputs)
    }
}

#[cfg(feature = "setup")]
const TEST_SETUP_SEED: u64 = 0;

#[cfg(feature = "client")]
fn circuit_matrices<P: ZkCircuit + 'static>() -> Result<Arc<CircuitMatrices>, ProverError> {
    use std::{any::TypeId, collections::HashMap, sync::OnceLock};

    // The matrices are deterministic per circuit, so synthesize them once
    // per process: constraint-system finalization is the load path's
    // dominant cost (~80 ms native, ~135 ms in the browser) and repeated
    // prover constructions of the same program get it for free.
    static CACHE: OnceLock<Mutex<HashMap<TypeId, Arc<CircuitMatrices>>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let key = TypeId::of::<P>();
    if let Some(matrices) = cache.lock().expect("matrices cache poisoned").get(&key) {
        return Ok(Arc::clone(matrices));
    }
    let placeholder = P::placeholder()?;
    let matrices = Arc::new(ArkworksCircuit::for_setup(&placeholder).matrices()?);
    cache
        .lock()
        .expect("matrices cache poisoned")
        .insert(key, Arc::clone(&matrices));
    Ok(matrices)
}

#[cfg(any(feature = "setup", not(target_arch = "wasm32")))]
fn key_file(path: &Path, error: std::io::Error) -> ProverErrorKind {
    ProverErrorKind::KeyFile {
        path: path.to_path_buf(),
        error,
    }
}

/// The snarkjs JSON form `parse_vk_json_to_rust_string` reads. Its parser drops
/// the trailing z coordinate and copies x and y verbatim, so the point at
/// infinity must stay `(0, 0, 1)`: that becomes the all-zero bytes alt_bn128
/// and [`g1_bytes`] use for it. snarkjs's own `(0, 1, 0)` would come out as
/// `(0, 1)`, a point off the curve.
#[cfg(feature = "setup")]
fn circom_vk_json(vk: &VerifyingKey) -> serde_json::Value {
    use serde_json::json;

    let g1 = |point: &G1Affine| {
        let (x, y) = point.xy().unwrap_or_default();
        json!([x.to_string(), y.to_string(), "1"])
    };
    let g2 = |point: &G2Affine| {
        let (x, y) = point.xy().unwrap_or_default();
        json!([
            [x.c0.to_string(), x.c1.to_string()],
            [y.c0.to_string(), y.c1.to_string()],
            ["1", "0"]
        ])
    };
    json!({
        "vk_alpha_1": g1(&vk.alpha_g1),
        "vk_beta_2": g2(&vk.beta_g2),
        "vk_gamma_2": g2(&vk.gamma_g2),
        "vk_delta_2": g2(&vk.delta_g2),
        "IC": vk.gamma_abc_g1.iter().map(g1).collect::<Vec<_>>(),
    })
}

fn g1_bytes(point: &G1Affine) -> [u8; 64] {
    let mut bytes = [0u8; 64];
    if let Some((x, y)) = point.xy() {
        let (x_bytes, y_bytes) = bytes.split_at_mut(32);
        x_bytes.copy_from_slice(&be_bytes(&x));
        y_bytes.copy_from_slice(&be_bytes(&y));
    }
    bytes
}

fn g2_bytes(point: &G2Affine) -> [u8; 128] {
    let mut bytes = [0u8; 128];
    if let Some((x, y)) = point.xy() {
        for (chunk, coordinate) in bytes
            .as_chunks_mut::<32>()
            .0
            .iter_mut()
            .zip([x.c1, x.c0, y.c1, y.c0].iter())
        {
            *chunk = be_bytes(coordinate);
        }
    }
    bytes
}

#[cfg(all(test, feature = "setup"))]
mod tests {
    use groth16_solana::vk::circom::parse_vk_json_to_rust_string;

    use super::*;

    fn keys() -> Groth16Keys {
        let g1 = G1Affine::generator();
        let g2 = G2Affine::generator();
        Groth16Keys::from(ProvingKey {
            vk: VerifyingKey {
                alpha_g1: g1,
                beta_g2: g2,
                gamma_g2: g2,
                delta_g2: g2,
                gamma_abc_g1: vec![g1; 2],
            },
            beta_g1: g1,
            delta_g1: g1,
            a_query: vec![g1; 2],
            b_g1_query: vec![g1; 2],
            b_g2_query: vec![g2; 2],
            h_query: vec![g1],
            l_query: vec![g1],
        })
    }

    #[test]
    fn canonical_keys_preserve_points_and_setup_provenance() {
        for imported_zkey in [false, true] {
            let keys = Groth16Keys {
                imported_zkey,
                ..keys()
            };
            let bytes = keys.to_bytes().unwrap();
            let reloaded = Groth16Keys::from_bytes(&bytes).unwrap();
            assert_eq!(reloaded.proving_key, keys.proving_key);
            assert_eq!(reloaded.verifying_key, keys.verifying_key);
            assert_eq!(reloaded.imported_zkey, imported_zkey);
        }
    }

    #[test]
    fn canonical_keys_reject_invalid_points_and_incomplete_encodings() {
        let mut keys = keys();
        let bytes = keys.to_bytes().unwrap();
        let (_, truncated) = bytes.split_last().unwrap();
        assert!(Groth16Keys::from_bytes(truncated).is_err());
        let mut trailing = bytes;
        trailing.push(0);
        assert!(Groth16Keys::from_bytes(&trailing).is_err());

        keys.proving_key.vk.alpha_g1 =
            G1Affine::new_unchecked(ark_bn254::Fq::from(1u64), ark_bn254::Fq::from(1u64));
        assert!(Groth16Keys::from_bytes(&keys.to_bytes().unwrap()).is_err());
    }

    #[test]
    fn canonical_keys_check_the_pinned_checksum_before_parsing() {
        use sha2::{Digest, Sha256};

        let bytes = keys().to_bytes().unwrap();
        let checksum = Sha256::digest(&bytes).into();
        assert!(Groth16Keys::from_bytes_checked(&bytes, &checksum).is_ok());
        let error = Groth16Keys::from_bytes_checked(b"corrupt key", &checksum)
            .err()
            .unwrap();
        assert!(matches!(error.kind(), ProverErrorKind::KeyChecksumMismatch));
    }

    /// A verifying key point at infinity exports as the zero bytes the verifier
    /// and `SolanaVerifyingKey` use for it.
    #[test]
    fn a_point_at_infinity_exports_as_zero_bytes() {
        let vk = VerifyingKey {
            alpha_g1: G1Affine::zero(),
            beta_g2: G2Affine::zero(),
            gamma_g2: G2Affine::zero(),
            delta_g2: G2Affine::zero(),
            gamma_abc_g1: vec![G1Affine::zero(); 2],
        };
        let source = parse_vk_json_to_rust_string(
            &circom_vk_json(&vk).to_string(),
            SetupKind::InsecureTest.verifier_kind(),
            &[0; 32],
        )
        .expect("verifying key source");
        let array = |bytes: &[u8]| {
            let bytes = bytes.iter().map(|byte| format!("{byte}u8"));
            format!("[{}]", bytes.collect::<Vec<_>>().join(", "))
        };
        let g1 = array(&g1_bytes(&G1Affine::zero()));
        let g2 = array(&g2_bytes(&G2Affine::zero()));

        assert_eq!(
            (
                source.contains(&format!("vk_alpha_g1: {g1},")),
                source.contains(&format!("vk_beta_g2: {g2},")),
                source.contains(&format!("vk_gamma_g2: {g2},")),
                source.contains(&format!("vk_delta_g2: {g2},")),
                source.matches(&format!("\t\t{g1},\n")).count(),
                g1_bytes(&G1Affine::zero()),
            ),
            (true, true, true, true, 2, [0u8; 64])
        );
    }
}
