//! The verifier and gnark's proof encodings against fixtures: the 2 → 2
//! transfer key's verifying key (gnark's raw encoding, from a key image
//! head), a proof the kernel made that gnark verified, its public input,
//! (in the kernel's encoding, four little-endian Montgomery limbs), and the
//! mobile SDK's response fixture.
use zopro_bn254::Fr;
use zopro_core::verify::{compress_g1, compress_g2, public_witness, Proof, VerifyingKey};

const VK: &[u8] = include_bytes!("fixtures/transfer-2x2.vk");
const PROOF: &[u8] = include_bytes!("fixtures/transfer-2x2.proof");
const PUBLIC: &[u8] = include_bytes!("fixtures/transfer-2x2.public");

fn key() -> VerifyingKey {
    VerifyingKey::decode_raw(VK).expect("the fixture's verifying key")
}

#[test]
fn verifies_the_kernels_proof() {
    let key = key();
    assert_eq!(key.public_inputs(), 1);
    let proof = Proof::from_raw(PROOF).unwrap();
    let public = zopro_core::decode_frs(PUBLIC)[0];
    assert!(key.verify(&proof, &[public]));
    assert!(!key.verify(&proof, &[public + Fr::from(1u64)]));
    assert!(!key.verify(&proof, &[]));
    for damaged in [
        Proof {
            ar: (proof.ar.to_projective() + zopro_bn254::G1Affine::GENERATOR).to_affine(),
            ..proof
        },
        Proof {
            krs: (proof.krs.to_projective() + zopro_bn254::G1Affine::GENERATOR).to_affine(),
            ..proof
        },
        Proof {
            bs: (proof.bs.to_projective() + zopro_bn254::G2Affine::GENERATOR).to_affine(),
            ..proof
        },
    ] {
        assert!(!key.verify(&damaged, &[public]));
    }

    let mut damaged = PROOF.to_vec();
    damaged[200] ^= 1;
    if let Ok(proof) = Proof::from_raw(&damaged) {
        assert!(!key.verify(&proof, &[public]));
    }
}

#[test]
fn gnark_encodings_round_trip() {
    let proof = Proof::from_raw(PROOF).unwrap();
    assert_eq!(proof.to_raw(), PROOF);
    let compressed = proof.to_gnark();
    assert_eq!(compressed.len(), 164);
    assert_eq!(&compressed[128..132], &[0, 0, 0, 0]);
    assert_eq!(compressed[132], 0b01 << 6);
    assert_eq!(
        zopro_core::decompress_g1(&compress_g1(&proof.ar)).unwrap(),
        PROOF[..64]
    );
    assert_eq!(
        zopro_core::decompress_g1(&compress_g1(&proof.krs)).unwrap(),
        PROOF[64..128]
    );
    assert_eq!(
        zopro_core::decompress_g2(&compress_g2(&proof.bs)).unwrap(),
        PROOF[128..]
    );
    let public = zopro_core::decode_frs(PUBLIC)[0];
    let witness = public_witness(&[public]);
    assert_eq!(witness.len(), 44);
    assert_eq!(&witness[..12], &[0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(witness[12..], public.to_be_bytes());
}
