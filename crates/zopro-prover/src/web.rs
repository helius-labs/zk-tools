//! The prover in a browser, over wasm-bindgen, beside the kernel's thread
//! pool (`initThreadPool`) and WebGPU (`init_gpu`) that zopro-core exports.
//!
//! ```js
//! const prover = new WebProver(keyFileOrRawForm);
//! const { proof, publicWitness } = await prover.prove({ X: "3", Y: "35" });
//! ```
use js_sys::{Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;

use crate::{Assignment, Error, Proved, Prover, raw, witness};

/// The raw form `export_raw` writes, for a page's cache to check.
#[wasm_bindgen]
pub fn raw_format() -> String {
    raw::FORMAT.into()
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
    pub fn drop_gpu(&mut self) {
        self.prover.drop_gpu();
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
