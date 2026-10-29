//! Zolana's prover in a browser: `ZolanaProver` takes the SDKs' `/prove`
//! request and gives the prover server's proof JSON, beside the generic
//! `WebProver`, `raw_format`, the kernel's thread pool and WebGPU.
use js_sys::{Object, Promise, Uint8Array};
use wasm_bindgen::prelude::*;
use zopro_prover::{
    raw,
    web::{promise, set},
};

use crate::{Prover, proof_json};

#[wasm_bindgen]
pub struct ZolanaProver {
    prover: Prover,
    source: Option<raw::Source>,
}

#[wasm_bindgen]
impl ZolanaProver {
    /// Loads a zopro key file, or the raw form an earlier load exported.
    #[wasm_bindgen(constructor)]
    pub fn new(key: &[u8]) -> Result<ZolanaProver, JsError> {
        let (prover, source) =
            zopro_prover::Prover::load_cacheable(key).map_err(|e| JsError::new(&e))?;
        let prover = Prover::new(prover).map_err(|e| JsError::new(&e))?;
        Ok(Self { prover, source })
    }

    /// The raw form to cache, once per load of a key file; empty after.
    pub fn export_raw(&mut self) -> Uint8Array {
        match self.source.take() {
            Some(source) => Uint8Array::from(source.export(self.prover.prover()).as_slice()),
            None => Uint8Array::new_with_length(0),
        }
    }

    pub fn name(&self) -> String {
        self.prover.name().to_owned()
    }

    /// The sha256 of the gnark key the zopro key was built from, hex.
    pub fn digest(&self) -> String {
        self.prover.key_sha256().to_owned()
    }

    pub fn inputs(&self) -> u32 {
        self.prover.shape().0
    }

    pub fn outputs(&self) -> u32 {
        self.prover.shape().1
    }

    pub fn has_gpu(&self) -> bool {
        self.prover.prover().has_gpu()
    }

    /// Proves on the CPU from now on.
    pub fn drop_gpu(&mut self) {
        self.prover.prover_mut().drop_gpu();
    }

    /// A Promise of `{proof, witnessMs, proveMs, verifyMs}`, `proof` the
    /// JSON string the prover server returns. It rejects with "unverified"
    /// when the proof did not verify (a GPU at fault: call `drop_gpu` and
    /// prove again), else with the request's error.
    pub fn prove(&self, request: &str) -> Result<Promise, JsError> {
        let started = zopro_prover::zopro_core::time::Instant::now();
        let witness = self
            .prover
            .witness(request)
            .map_err(|e| JsError::new(&e.to_string()))?;
        let witness_ms = started.elapsed().as_millis() as f64;
        let digest = self.prover.key_sha256().to_owned();
        Ok(promise(self.prover.prover(), &witness, move |proved| {
            let object = Object::new();
            let json = proof_json::to_json(&proved.proof, Some(&digest));
            set(&object, "proof", JsValue::from_str(&json.to_string()));
            set(&object, "witnessMs", JsValue::from_f64(witness_ms));
            set(
                &object,
                "proveMs",
                JsValue::from_f64(proved.prove_ms as f64),
            );
            set(
                &object,
                "verifyMs",
                JsValue::from_f64(proved.verify_ms as f64),
            );
            object.into()
        }))
    }

    /// Whether `proof_json` (the SDKs' proof JSON) proves `request`'s public
    /// input with this key.
    pub fn verify(&self, request: &str, proof_json: &str) -> Result<bool, JsError> {
        let witness = self
            .prover
            .witness(request)
            .map_err(|e| JsError::new(&e.to_string()))?;
        let json: serde_json::Value =
            serde_json::from_str(proof_json).map_err(|_| JsError::new("proof JSON"))?;
        let proof = proof_json::from_json(&json).map_err(JsError::new)?;
        let public = &witness[..self.prover.prover().public_inputs()];
        Ok(self.prover.verify(&proof, public))
    }
}
