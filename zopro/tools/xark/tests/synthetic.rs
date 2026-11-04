//! A small circuit end to end, with nothing outside this repository: an
//! R1CS written here, a key from Xark's setup (arkworks' trusted-dealer
//! setup, point for point), the zopro key converted from it, and zopro's
//! proofs checked by arkworks' verifier, for each of Xark's key encodings.
mod support;

use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::{CanonicalSerialize, Compress};
use num_bigint::BigInt;
use rand::SeedableRng;
use xark_groth16::{ConstraintSource, DomainKind, ProgramSource};
use xark_ir::{
    FieldConst, FieldSpec, LinearCombination, R1csConstraint, R1csProgram, Term, Variable,
    Visibility,
};

fn var(id: u32) -> LinearCombination {
    LinearCombination {
        constant: FieldConst::zero(),
        terms: vec![Term {
            coeff: FieldConst::from_i64(1),
            var: id,
        }],
    }
}

fn plus(mut lc: LinearCombination, constant: i64) -> LinearCombination {
    lc.constant = FieldConst::from_i64(constant);
    lc
}

fn row(
    id: usize,
    a: LinearCombination,
    b: LinearCombination,
    c: LinearCombination,
) -> R1csConstraint {
    R1csConstraint {
        id: id as u32,
        a,
        b,
        c,
        debug: None,
    }
}

/// x_{i+1} = x_i^2 + i for `steps` steps from a private x_0, the last one
/// public as `out`; a public y with (y + 2)(x_0 + 1) = w for a private w
/// that only C names (its A and B points are at infinity); and a row of a
/// large negative coefficient. Ids are out of order: public `out` is the
/// largest, so the layout must reorder them.
fn circuit(steps: usize) -> (R1csProgram, Vec<(u32, Fr)>) {
    let x = |i: usize| i as u32 + 10;
    let (y, w, out) = (1u32, 2u32, 10_000u32);
    let mut variables = vec![
        Variable {
            id: y,
            name: "y".into(),
            visibility: Visibility::Public,
        },
        Variable {
            id: w,
            name: "w".into(),
            visibility: Visibility::Internal,
        },
        Variable {
            id: out,
            name: "out".into(),
            visibility: Visibility::Public,
        },
    ];
    let mut values = vec![(y, Fr::from(41u64))];
    let mut constraints = Vec::new();
    let mut value = Fr::from(3u64);
    for i in 0..=steps {
        variables.push(Variable {
            id: x(i),
            name: format!("x{i}"),
            visibility: Visibility::Private,
        });
        values.push((x(i), value));
        if i < steps {
            // x_i * x_i = x_{i+1} - i
            constraints.push(row(
                i,
                var(x(i)),
                var(x(i)),
                plus(var(x(i + 1)), -(i as i64)),
            ));
            value = value * value + Fr::from(i as u64);
        }
    }
    values.push((out, value));
    values.push((
        w,
        (Fr::from(41u64) + Fr::from(2u64)) * (Fr::from(3u64) + Fr::from(1u64)),
    ));
    // (x_steps - out) * 1 = 0
    let mut difference = var(x(steps));
    difference.terms.push(Term {
        coeff: FieldConst::from_i64(-1),
        var: out,
    });
    constraints.push(row(
        constraints.len(),
        difference,
        plus(LinearCombination::zero(), 1),
        LinearCombination::zero(),
    ));
    // (y + 2) * (x_0 + 1) = w
    constraints.push(row(
        constraints.len(),
        plus(var(y), 2),
        plus(var(x(0)), 1),
        var(w),
    ));
    // (-(r - 1) / 2 * x_0) * 2 = x_0: the coefficient's signed bytes are long.
    let half = -(BigInt::from_bytes_le(
        num_bigint::Sign::Plus,
        &Fr::MODULUS_MINUS_ONE_DIV_TWO.to_bytes_le(),
    ));
    let mut scaled = LinearCombination::zero();
    scaled.terms.push(Term {
        coeff: FieldConst::from_bigint(half),
        var: x(0),
    });
    constraints.push(row(
        constraints.len(),
        scaled,
        plus(LinearCombination::zero(), 2),
        var(x(0)),
    ));
    let program = R1csProgram {
        field: FieldSpec::bn254(),
        variables,
        constraints,
    };
    (program, values)
}

#[test]
fn zopro_proves_xark_keys_that_arkworks_verifies() {
    for steps in [3, 300] {
        let (program, values) = circuit(steps);
        let source = ProgramSource::new(&program);
        let layout = source.layout();
        let z = layout.assignment(|id| {
            values
                .iter()
                .find(|(v, _)| *v == id)
                .map(|(_, value)| *value)
        });
        assert_eq!(
            xark_groth16::first_unsatisfied(&source, &z),
            None,
            "the witness satisfies the rows"
        );
        let public = z
            .get(1..layout.num_instance())
            .expect("the instance")
            .to_vec();
        let rows = zopro_xark::row_file(&source, "synthetic");
        for compress in [Compress::Yes, Compress::No] {
            let mut pk = Vec::new();
            let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(steps as u64);
            let vk = xark_groth16::setup::dev_setup(
                &source,
                &mut rng,
                &mut pk,
                compress,
                DomainKind::Radix2,
            )
            .expect("Xark's setup");
            let mut vk_bytes = Vec::new();
            vk.serialize_with_mode(&mut vk_bytes, compress).unwrap();
            let mut keys = vec![pk.clone()];
            if compress == Compress::Yes {
                // A legacy key: arkworks' bytes without Xark's header.
                keys.push(pk.get(14..).expect("a v2 header").to_vec());
            }
            for pk in keys {
                let key = zopro_core::xark::pack(&pk, &vk_bytes, &rows, "synthetic")
                    .expect("a zopro key");
                let mut prover = zopro_prover::Prover::load(&key).expect("the key loads");
                assert_eq!(prover.public_inputs(), public.len());
                let witness = zopro_xark::zopro_witness(&z).unwrap();
                let first = prover.prove(&witness).expect("zopro proves");
                let second = prover.prove(&witness).expect("zopro proves");
                assert_ne!(first.proof, second.proof, "each proof is blinded afresh");
                let mut vk_compressed = Vec::new();
                vk.serialize_compressed(&mut vk_compressed).unwrap();
                for proved in [&first, &second] {
                    let proof = zopro_xark::xark_proof(&proved.proof).unwrap();
                    assert!(
                        support::ark_verifies(&vk_compressed, &proof, &public),
                        "{steps} steps, compressed: {}",
                        compress == Compress::Yes
                    );
                    let mut wrong = public.clone();
                    *wrong.first_mut().expect("a public input") += Fr::from(1u64);
                    assert!(!support::ark_verifies(&vk_compressed, &proof, &wrong));
                }
                // A witness that breaks a row is refused before any proof.
                let mut broken = witness.clone();
                *broken
                    .get_mut(layout.num_instance())
                    .expect("a private value") += zopro_xark::Fr::ONE;
                assert!(prover.prove(&broken).is_err());
            }
        }
    }
}

#[test]
fn a_key_and_rows_of_different_circuits_are_refused() {
    let (small, _) = circuit(3);
    let (large, _) = circuit(40);
    let mut pk = Vec::new();
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(1);
    let source = ProgramSource::new(&small);
    let vk = xark_groth16::setup::dev_setup(
        &source,
        &mut rng,
        &mut pk,
        Compress::Yes,
        DomainKind::Radix2,
    )
    .unwrap();
    let mut vk_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_bytes).unwrap();
    let rows = zopro_xark::row_file(&ProgramSource::new(&large), "large");
    assert!(zopro_core::xark::pack(&pk, &vk_bytes, &rows, "mismatch").is_err());
    let mut other_vk = Vec::new();
    let mut pk2 = Vec::new();
    xark_groth16::setup::dev_setup(
        &source,
        &mut rng,
        &mut pk2,
        Compress::Yes,
        DomainKind::Radix2,
    )
    .unwrap()
    .serialize_compressed(&mut other_vk)
    .unwrap();
    let rows = zopro_xark::row_file(&source, "small");
    assert!(zopro_core::xark::pack(&pk, &other_vk, &rows, "mismatch").is_err());
    assert!(zopro_core::xark::pack(&pk, &vk_bytes, &rows, "small").is_ok());
}
