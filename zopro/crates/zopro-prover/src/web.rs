//! The prover in a browser, over wasm-bindgen, beside the kernel's thread
//! pool (`initThreadPool`, in a build with shared memory) and WebGPU
//! (`init_gpu`) that zopro-core exports.
//!
//! ```js
//! const prover = new WebProver(keyFileOrRawForm);
//! const { proof, publicWitness } = await prover.prove({ X: "3", Y: "35" });
//! ```
use js_sys::{Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::{
    Assignment, Error, Proved, Prover, raw, witness,
    zopro_core::artifact::{self, Artifact},
};

/// The raw form `export_raw` writes, for a page's cache to check.
#[wasm_bindgen]
pub fn raw_format() -> String {
    raw::FORMAT.into()
}

/// Where a key file's vectors of compressed points lie: byte offset and
/// length of a, b, k, z (G1, 32 bytes a point) and b2 (G2, 64), in turn.
/// A page without a thread pool splits each into runs of whole points,
/// decompresses them in workers (`kernel_points`) and loads the key from
/// the results (`from_vectors`).
#[wasm_bindgen]
pub fn key_vectors(key: &[u8]) -> Result<Vec<u32>, JsError> {
    let artifact = Artifact::parse(key).map_err(JsError::new)?;
    Ok(artifact
        .vector_ranges(key)
        .iter()
        .flat_map(|&(at, len)| [at as u32, len as u32])
        .collect())
}

/// Compressed points of a key file's vector (G2 for b2) in the kernel's
/// encoding.
#[wasm_bindgen]
pub fn kernel_points(compressed: &[u8], g2: bool) -> Result<Vec<u8>, JsError> {
    artifact::kernel_points(compressed, g2).map_err(JsError::new)
}


#[wasm_bindgen]
pub struct WebProver {
    prover: Prover,
    source: Option<raw::Source>,
}

#[wasm_bindgen]
impl WebProver {
    /// Loads a zopro key file, or the raw form an earlier load exported.
    #[wasm_bindgen(constructor)]
    pub fn new(key: &[u8]) -> Result<WebProver, JsError> {
        let (prover, source) = Prover::load_cacheable(key).map_err(|e| JsError::new(&e))?;
        Ok(Self { prover, source })
    }

    /// Loads a zopro key file whose vectors a page decompressed
    /// (`key_vectors`, `kernel_points`), each joined in order.
    pub fn from_vectors(
        key: &[u8],
        a: Vec<u8>,
        b: Vec<u8>,
        k: Vec<u8>,
        z: Vec<u8>,
        b2: Vec<u8>,
    ) -> Result<WebProver, JsError> {
        let (prover, source) =
            Prover::load_decompressed(key, [a, b, k, z, b2]).map_err(|e| JsError::new(&e))?;
        Ok(Self {
            prover,
            source: Some(source),
        })
    }

    /// The raw form to cache, once per load of a key file; empty after, and
    /// after a load of the raw form.
    pub fn export_raw(&mut self) -> Uint8Array {
        match self.source.take() {
            Some(source) => Uint8Array::from(source.export(&self.prover).as_slice()),
            None => Uint8Array::new_with_length(0),
        }
    }

    pub fn name(&self) -> String {
        self.prover.name().to_owned()
    }

    pub fn metadata(&self) -> Uint8Array {
        Uint8Array::from(self.prover.metadata())
    }

    /// The inputs' names, public then secret.
    pub fn names(&self) -> Vec<String> {
        self.prover.names().to_vec()
    }

    pub fn public_inputs(&self) -> usize {
        self.prover.public_inputs()
    }

    pub fn has_gpu(&self) -> bool {
        self.prover.has_gpu()
    }

    /// Proves on the CPU from now on.
    pub fn drop_gpu(&mut self) -> Result<(), JsError> {
        self.prover.drop_gpu().map_err(|e| JsError::new(&e))
    }

    /// A Promise of `{proof, publicWitness, proveMs, verifyMs}` for an
    /// object of the inputs' values by name (decimal, or `0x` and hex):
    /// gnark's compressed proof and binary public witness. It rejects with
    /// "unverified" when the proof did not verify (a GPU at fault: call
    /// `drop_gpu` and prove again).
    pub fn prove(&self, assignment: JsValue) -> Result<Promise, JsError> {
        let mut values = Assignment::default();
        for entry in Object::entries(
            &assignment
                .dyn_into::<Object>()
                .map_err(|_| JsError::new("an object of values"))?,
        ) {
            let pair = js_sys::Array::from(&entry);
            let (name, value) = (pair.get(0).as_string(), pair.get(1).as_string());
            let (Some(name), Some(value)) = (name, value) else {
                return Err(JsError::new("values are strings"));
            };
            let value = witness::parse_value(&value)
                .ok_or_else(|| JsError::new(&format!("{name} is not a field element")))?;
            values.set(name, value);
        }
        let witness = self
            .prover
            .witness(&values)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(promise(&self.prover, &witness, |proved| {
            let object = Object::new();
            set(
                &object,
                "proof",
                Uint8Array::from(proved.proof.to_gnark().as_slice()).into(),
            );
            set(
                &object,
                "publicWitness",
                Uint8Array::from(proved.public_witness().as_slice()).into(),
            );
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
}

/// A Promise of `prover`'s proof of `witness`, made into a JS value by
/// `value`; it rejects with "unverified" or the kernel's message.
pub fn promise(
    prover: &Prover,
    witness: &[zopro_bn254::Fr],
    value: impl FnOnce(Proved) -> JsValue + 'static,
) -> Promise {
    let rejection = |error: Error| match error {
        Error::Unverified => JsValue::from_str("unverified"),
        error => JsValue::from(JsError::new(&error.to_string())),
    };
    match prover.prove_async(witness) {
        Ok(future) => wasm_bindgen_futures::future_to_promise(async move {
            future.await.map(value).map_err(rejection)
        }),
        Err(error) => Promise::reject(&rejection(error)),
    }
}

pub fn set(object: &Object, name: &str, value: JsValue) {
    let _ = Reflect::set(object, &JsValue::from_str(name), &value);
}
