//! Xark's side of a zopro key of an Xark circuit: what needs Xark itself.
//!
//! * The rows a proving key was made for, as Xark's row file
//!   (`r1cs.stream`), which `zopro-bench pack-xark` packs with `pk.bin` and
//!   `vk.bin` into a zopro key (`zopro_core::xark`, without Xark or
//!   arkworks). Each variable's id in the file is its position in Xark's
//!   assignment.
//! * The witness: Xark solves the circuit from its inputs, and the zopro
//!   key takes the whole assignment but its constant one, in Xark's order.
//!   zopro solves nothing and only proves.
//!
//! ```ignore
//! let circuit = zopro_xark::Circuit::load(&xbc, &pk, None)?;
//! let mut prover = zopro_prover::Prover::load(&std::fs::read("settle_dvp.zopro")?)?;
//! let proved = prover.prove(&circuit.witness(inputs)?)?;
//! let proof = zopro_xark::xark_proof(&proved.proof); // Xark's proof.bin bytes
//! ```
use std::collections::BTreeMap;

use ark_bn254::Fr as ArkFr;
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::CanonicalSerialize;
use num_bigint::{BigInt, BigUint};
use xark_groth16::r1cs::RowChunk;
use xark_groth16::{ConstraintSource, PkSource, SerializedPk};
use xark_ir::primitive::PrimitiveProgram;
use xark_ir::{
    FieldConst, FieldSpec, LinearCombination, R1csConstraint, R1csProgram, Term, Variable,
    Visibility,
};

/// zopro's scalar field.
pub use zopro_bn254::Fr;

/// An Xark circuit and the rows of one of its proving keys.
pub struct Circuit {
    primitive: PrimitiveProgram,
    rows: Box<dyn ConstraintSource>,
    fingerprint: String,
}

impl Circuit {
    /// From `circuit.xbc` and the proving key (`pk.bin`) whose rows to
    /// take: the circuit as built, or its minimized form for a key made of
    /// that (`cached`: the `r1cs.stream` `xark setup` wrote beside such a
    /// key; without it the circuit is minimized again), as Xark's prover
    /// picks them.
    pub fn load(xbc: &[u8], pk: &[u8], cached: Option<Vec<u8>>) -> Result<Self, String> {
        let key = SerializedPk::from_bytes(pk).map_err(|e| e.to_string())?;
        let rows = xark_prover::rows_for_key(xbc, key.header().a_len, cached)?;
        let primitive = xark_ir::function_decode::expand_function_blob(xbc)
            .map_err(|e| format!("parsing circuit.xbc: {e}"))?
            .to_primitive();
        Ok(Self {
            primitive,
            rows,
            fingerprint: xark_ir::function_decode::stream_digest_hex(xbc),
        })
    }

    /// The rows the key was made for.
    pub fn source(&self) -> &dyn ConstraintSource {
        &*self.rows
    }

    /// The rows as Xark's row file (`row_file`), for `zopro-bench
    /// pack-xark`.
    pub fn rows(&self) -> Vec<u8> {
        row_file(&*self.rows, &self.fingerprint)
    }

    /// Xark's assignment z of the circuit's inputs by leaf name (decimal
    /// or `0x` hex values, as Xark's clients encode them): the constant
    /// one, the public inputs, then the other variables, solved and checked
    /// by Xark's solver (zopro checks the key's rows again as it proves).
    pub fn assignment(
        &self,
        inputs: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Vec<ArkFr>, String> {
        let inputs: BTreeMap<String, String> = inputs.into_iter().collect();
        let ids = xark_prover::resolve_input_ids(&self.primitive.vars, &inputs)?;
        let solved = xark_ir::solver::solve_and_check(&self.primitive, &ids)
            .map_err(|e| format!("the inputs do not satisfy the circuit: {e:?}"))?;
        Ok(self.rows.layout().assignment(|id| {
            solved.get(&id).map(|value| {
                value
                    .as_bn254_fr()
                    .unwrap_or_else(|| xark_prover::fr_from_decimal(&value.to_decimal()))
            })
        }))
    }

    /// The zopro key's witness of the inputs: Xark's assignment without
    /// its constant one, for `zopro_prover::Prover::prove`.
    pub fn witness(
        &self,
        inputs: impl IntoIterator<Item = (String, String)>,
    ) -> Result<Vec<Fr>, String> {
        zopro_witness(&self.assignment(inputs)?)
    }
}

/// The rows of `source` as Xark's row file (`r1cs.stream`), each
/// variable's id its position in Xark's assignment, so that the file's
/// layout is the source's.
pub fn row_file(source: &dyn ConstraintSource, fingerprint: &str) -> Vec<u8> {
    let layout = source.layout();
    let instance = layout.num_instance();
    let variables = (1..layout.num_vars())
        .map(|position| Variable {
            id: position as u32,
            name: String::new(),
            visibility: if position < instance {
                Visibility::Public
            } else {
                Visibility::Internal
            },
        })
        .collect();
    let mut constraints = Vec::with_capacity(source.num_constraints());
    let mut chunk = RowChunk::default();
    for window in source.windows(1 << 14) {
        source.expand(&window, &mut chunk);
        for row in 0..chunk.rows() {
            let [a, b, c] = [0, 1, 2].map(|side| {
                let (columns, coefficients) = chunk.terms(row, side);
                linear_combination(columns, coefficients)
            });
            constraints.push(R1csConstraint {
                id: constraints.len() as u32,
                a,
                b,
                c,
                debug: None,
            });
        }
    }
    let program = R1csProgram {
        field: FieldSpec::bn254(),
        variables,
        constraints,
    };
    xark_ir::r1cs_stream::encode(fingerprint, &program)
}

/// A row's linear combination from Xark's expanded terms: column 0 is the
/// constant one.
fn linear_combination(columns: &[u32], coefficients: &[ArkFr]) -> LinearCombination {
    let mut constant = ArkFr::from(0u64);
    let mut terms = Vec::with_capacity(columns.len());
    for (&column, coefficient) in columns.iter().zip(coefficients) {
        if column == 0 {
            constant += coefficient;
        } else {
            terms.push(Term {
                coeff: field_const(coefficient),
                var: column,
            });
        }
    }
    LinearCombination {
        constant: field_const(&constant),
        terms,
    }
}

fn field_const(value: &ArkFr) -> FieldConst {
    FieldConst::from_bigint(BigInt::from(BigUint::from_bytes_le(
        &value.into_bigint().to_bytes_le(),
    )))
}

/// Xark's assignment as the zopro key's witness: z[1..], in zopro's field.
pub fn zopro_witness(z: &[ArkFr]) -> Result<Vec<Fr>, String> {
    match z.split_first() {
        Some((one, rest)) if *one == ArkFr::from(1u64) => rest
            .iter()
            .map(|value| {
                Fr::from_canonical(value.into_bigint().0)
                    .ok_or_else(|| "a value past r".to_string())
            })
            .collect(),
        _ => Err("an assignment starts with the constant one".into()),
    }
}

/// A zopro proof as Xark's (and arkworks') proof, compressed: the bytes of
/// Xark's `proof.bin`, which its verifiers and clients read.
pub fn xark_proof(proof: &zopro_prover::zopro_core::verify::Proof) -> Result<Vec<u8>, String> {
    let proof = ark_proof(proof)?;
    let mut out = Vec::new();
    proof
        .serialize_compressed(&mut out)
        .map_err(|e| e.to_string())?;
    Ok(out)
}

/// A zopro proof as arkworks' points.
fn ark_proof(
    proof: &zopro_prover::zopro_core::verify::Proof,
) -> Result<xark_groth16::Proof<ark_bn254::Bn254>, String> {
    use ark_bn254::{Fq, Fq2, G1Affine, G2Affine};
    let fq = |v: &zopro_bn254::Fq| Fq::from_be_bytes_mod_order(&v.to_be_bytes());
    let g1 = |p: &zopro_bn254::G1Affine| -> Result<G1Affine, String> {
        if p.is_identity() {
            return Ok(G1Affine::identity());
        }
        let point = G1Affine::new_unchecked(fq(&p.x), fq(&p.y));
        point
            .is_on_curve()
            .then_some(point)
            .ok_or_else(|| "a G1 point off the curve".into())
    };
    let g2 = |p: &zopro_bn254::G2Affine| -> Result<G2Affine, String> {
        if p.is_identity() {
            return Ok(G2Affine::identity());
        }
        let point = G2Affine::new_unchecked(
            Fq2::new(fq(&p.x.c0), fq(&p.x.c1)),
            Fq2::new(fq(&p.y.c0), fq(&p.y.c1)),
        );
        point
            .is_on_curve()
            .then_some(point)
            .ok_or_else(|| "a G2 point off the twist".into())
    };
    Ok(xark_groth16::Proof {
        a: g1(&proof.ar)?,
        b: g2(&proof.bs)?,
        c: g1(&proof.krs)?,
    })
}

/// A flat inputs file: one JSON object of leaf names and values, as Xark's
/// clients encode them (`ProgramProofInputs::to_json`).
pub fn read_inputs(json: &str) -> Result<Vec<(String, String)>, String> {
    let value: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    let object = value.as_object().ok_or("the inputs are one JSON object")?;
    object
        .iter()
        .map(|(name, value)| match value {
            serde_json::Value::String(text) => Ok((name.clone(), text.clone())),
            serde_json::Value::Number(number) => Ok((name.clone(), number.to_string())),
            serde_json::Value::Bool(flag) => Ok((name.clone(), u8::from(*flag).to_string())),
            _ => Err(format!("input {name} is not a scalar")),
        })
        .collect()
}
