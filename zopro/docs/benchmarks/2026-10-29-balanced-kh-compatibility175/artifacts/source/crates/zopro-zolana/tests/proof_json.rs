//! The prover server's proof JSON against the mobile SDK's response
//! fixture and the 2 -> 2 key's verifying key (gnark's raw encoding).
use zopro_bn254::Fr;
use zopro_zolana::{proof_json, zopro_core::verify::VerifyingKey};

const VK: &[u8] = include_bytes!("fixtures/transfer-2x2.vk");
const RESPONSE: &str = include_str!("fixtures/transfer-2x2.response.json");

#[test]
fn verifies_the_sdks_response() {
    let response: serde_json::Value = serde_json::from_str(RESPONSE).unwrap();
    let proof = proof_json::from_json(&response["proof"]).unwrap();
    let hash = response["publicInputHash"].as_str().unwrap();
    let bytes: Vec<u8> = (0..32)
        .map(|i| u8::from_str_radix(&hash[2 + 2 * i..4 + 2 * i], 16).unwrap())
        .collect();
    let public = Fr::from_be_bytes_mod_order(&bytes);
    let key = VerifyingKey::decode_raw(VK).unwrap();
    assert!(key.verify(&proof, &[public]));
    assert!(!key.verify(&proof, &[public + Fr::from(1u64)]));
    // The JSON round trip is exact.
    assert_eq!(proof_json::to_json(&proof, None), response["proof"]);
}

#[test]
fn reads_the_keys_metadata() {
    let metadata = zopro_zolana::Metadata {
        inputs: 2,
        outputs: 2,
        key_sha256: "ab".repeat(32),
    };
    assert_eq!(
        zopro_zolana::Metadata::parse(metadata.to_json().as_bytes()),
        Ok(metadata)
    );
    assert!(
        zopro_zolana::Metadata::parse(br#"{"inputs":2,"outputs":2,"keySha256":"00"}"#).is_err()
    );
    assert!(zopro_zolana::Metadata::parse(b"").is_err());
}
