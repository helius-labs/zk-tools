//! Xark circuits built elsewhere, end to end, when their artifacts are
//! given: `ZOPRO_XARK_KEYS=<dir>` holds `<circuit>/{circuit.xbc,pk.bin,vk.bin}`
//! (a DvP program's `target/zk/dvp-program`), and `ZOPRO_XARK_INPUTS=<dir>`
//! each circuit's inputs as `<circuit>.json`, one object of leaf names and
//! values (`ProgramProofInputs::to_json`). For each circuit with both: the
//! key converted to a zopro key, Xark's witness of the inputs, two zopro
//! proofs, each checked by arkworks' verifier against the circuit's
//! `vk.bin`, and refused for another public input.
mod support;

use std::path::Path;

use ark_bn254::Fr;

#[test]
fn zopro_proves_xark_circuits_that_arkworks_verifies() {
    let (Ok(keys), Ok(inputs)) = (
        std::env::var("ZOPRO_XARK_KEYS"),
        std::env::var("ZOPRO_XARK_INPUTS"),
    ) else {
        eprintln!("ZOPRO_XARK_KEYS and ZOPRO_XARK_INPUTS are not set: skipped");
        return;
    };
    let mut proved = 0;
    for entry in std::fs::read_dir(&inputs).expect("the inputs directory") {
        let path = entry.expect("an inputs entry").path();
        let Some(name) = path
            .file_stem()
            .and_then(|n| n.to_str())
            .filter(|_| path.extension().is_some_and(|e| e == "json"))
        else {
            continue;
        };
        let dir = Path::new(&keys).join(name);
        let read = |file: &str| {
            std::fs::read(dir.join(file)).unwrap_or_else(|e| panic!("{name}/{file}: {e}"))
        };
        let (xbc, pk, vk) = (read("circuit.xbc"), read("pk.bin"), read("vk.bin"));
        let circuit =
            zopro_xark::Circuit::load(&xbc, &pk, std::fs::read(dir.join("r1cs.stream")).ok())
                .expect("the circuit");
        let key = zopro_core::xark::pack(&pk, &vk, &circuit.rows(), name).expect("a zopro key");
        let mut prover = zopro_prover::Prover::load(&key).expect("the key loads");
        let values =
            zopro_xark::read_inputs(&std::fs::read_to_string(&path).expect("the inputs")).unwrap();
        let z = circuit.assignment(values).expect("Xark solves the inputs");
        let instance = circuit.source().layout().num_instance();
        let public: Vec<Fr> = z.get(1..instance).expect("the instance").to_vec();
        let witness = zopro_xark::zopro_witness(&z).unwrap();
        let (first, second) = (
            prover.prove(&witness).expect("zopro proves"),
            prover.prove(&witness).expect("zopro proves"),
        );
        assert_ne!(
            first.proof, second.proof,
            "{name}: each proof is blinded afresh"
        );
        for proof in [&first.proof, &second.proof] {
            let proof = zopro_xark::xark_proof(proof).unwrap();
            assert!(
                support::ark_verifies(&vk, &proof, &public),
                "{name}: arkworks rejects zopro's proof"
            );
            let mut wrong = public.clone();
            *wrong.first_mut().expect("a public input") += Fr::from(1u64);
            assert!(
                !support::ark_verifies(&vk, &proof, &wrong),
                "{name}: a proof of another input"
            );
        }
        eprintln!(
            "{name}: proved in {} ms, verified by arkworks",
            first.prove_ms
        );
        proved += 1;
    }
    assert!(proved > 0, "no circuit with inputs in {inputs}");
}
