//! The referee: arkworks' Groth16 verifier, given Xark's verifying key and
//! a zopro proof as arkworks reads it from its bytes.
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, VerifyingKey, prepare_verifying_key};
use ark_serialize::CanonicalDeserialize;

/// Whether arkworks accepts the proof (Xark's `proof.bin` bytes) of
/// `public` with the verifying key (Xark's `vk.bin` bytes).
pub fn ark_verifies(vk: &[u8], proof: &[u8], public: &[Fr]) -> bool {
    let vk = VerifyingKey::<Bn254>::deserialize_compressed(vk).expect("arkworks reads vk.bin");
    let proof = Proof::<Bn254>::deserialize_compressed(proof).expect("arkworks reads the proof");
    Groth16::<Bn254>::verify_proof(&prepare_verifying_key(&vk), &proof, public)
        .expect("arkworks verifies")
}
