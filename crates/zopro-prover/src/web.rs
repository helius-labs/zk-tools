//! The prover for a browser: `WebProver` over wasm-bindgen, beside the
//! kernel's thread pool, WebGPU (`init_gpu`) and point decompression that
//! zopro-core exports. A GPU proof is awaited; the worker that holds the
//! prover lets the GPU go (`drop_gpu`) when one fails to verify and proves
//! again on the CPU.
use js_sys::{Object, Promise, Reflect, Uint8Array};
use wasm_bindgen::prelude::*;
use zopro_core::artifact::Artifact;

use crate::{Error, ProofResult, Prover, Unverified, finish};

/// The raw form a page caches in place of the key file, so a later load
/// skips point decompression and the solver's build: a magic, the key
/// file's head (everything before its vectors, length-prefixed), the five
/// vectors decompressed into the kernel's encoding, and the kernel's solver
/// image (length-prefixed). `RAW_FORMAT` names the layout; a page keeps a
/// raw form only beside the format that wrote it.
pub const RAW_FORMAT: &str = "zopro-raw-1";
const RAW_MAGIC: &[u8; 16] = b"zopro-raw1\0\0\0\0\0\0";

#[wasm_bindgen]
pub fn raw_format() -> String {
    RAW_FORMAT.into()
}

#[wasm_bindgen]
pub struct WebProver {
    prover: Prover,
    /// What `export_raw` writes, from a load of the key file; none after
    /// the export or a load of the raw form.
    raw: Option<(Vec<u8>, [Vec<u8>; 5])>,
}

#[wasm_bindgen]
impl WebProver {
    /// Loads `key`: a zopro key file, or the raw form an earlier load
    /// exported.
    #[wasm_bindgen(constructor)]
    pub fn new(key: &[u8]) -> Result<WebProver, JsError> {
        if key.starts_with(RAW_MAGIC) {
            let (artifact, vectors, image) =
                parse_raw(key).ok_or_else(|| JsError::new("not a raw key"))?;
            let prover = Prover::from_artifact(&artifact, &vectors, Some(&image))
                .map_err(|e| JsError::new(&e))?;
            return Ok(Self { prover, raw: None });
        }
        let (_, at) = Artifact::head(key).map_err(JsError::new)?;
        let artifact = Artifact::parse(key).map_err(JsError::new)?;
        let vectors = artifact.vectors().map_err(JsError::new)?;
        let prover =
            Prover::from_artifact(&artifact, &vectors, None).map_err(|e| JsError::new(&e))?;
        Ok(Self {
            prover,
            raw: Some((key[..at].to_vec(), vectors)),
        })
    }

    /// The raw form to cache, once per load of the key file; empty after,
    /// and after a load of the raw form.
    pub fn export_raw(&mut self) -> Uint8Array {
        let Some((head, vectors)) = self.raw.take() else {
            return Uint8Array::new_with_length(0);
        };
        let image = self.prover.solver_image().unwrap_or_default();
        let size = 24 + head.len() + vectors.iter().map(Vec::len).sum::<usize>() + image.len();
        let mut out = Vec::with_capacity(size);
        out.extend_from_slice(RAW_MAGIC);
        out.extend_from_slice(&(head.len() as u32).to_le_bytes());
        out.extend_from_slice(&head);
        for vector in &vectors {
            out.extend_from_slice(vector);
        }
        out.extend_from_slice(&(image.len() as u32).to_le_bytes());
        out.extend_from_slice(&image);
        Uint8Array::from(out.as_slice())
    }

    pub fn name(&self) -> String {
        self.prover.name().to_owned()
    }

    /// The gnark proving key's sha256, hex: what the key file was built
    /// from, and what a proof reports.
    pub fn digest(&self) -> String {
        self.prover.digest.clone()
    }

    pub fn inputs(&self) -> u32 {
        self.prover.shape().0
    }

    pub fn outputs(&self) -> u32 {
        self.prover.shape().1
    }

    pub fn has_gpu(&self) -> bool {
        self.prover.key.has_gpu()
    }

    /// Proves on the CPU from now on.
    pub fn drop_gpu(&mut self) {
        self.prover.key.drop_gpu();
    }

    /// A Promise of the proof: `{proof, witnessMs, proveMs, verifyMs}`,
    /// `proof` the JSON string the prover server returns. It rejects with
    /// "unverified" when the proof did not verify (a GPU at fault: call
    /// `drop_gpu` and prove again), else with the request's error.
    pub fn prove(&mut self, request: &str) -> Result<Promise, JsError> {
        let prepared = self
            .prover
            .prepare(request)
            .map_err(|e| JsError::new(&e.to_string()))?;
        let key = &self.prover.key;
        let started = zopro_core::time::Instant::now();
        if key.has_gpu() {
            let parts = key.solve_parts_gpu(&prepared.encoded)?;
            let (fixed, digest) = (self.prover.fixed, self.prover.digest.clone());
            let verifying_key = self.prover.verifying_key.clone();
            return Ok(wasm_bindgen_futures::future_to_promise(async move {
                let parts = wasm_bindgen_futures::JsFuture::from(parts).await?;
                let parts = Uint8Array::new(&parts).to_vec();
                let prove_ms = started.elapsed().as_millis() as u64;
                finish(
                    &parts,
                    &prepared,
                    &fixed,
                    &verifying_key,
                    &digest,
                    prove_ms,
                    0,
                )
                .map(result_object)
                .map_err(unverified)
            }));
        }
        let parts = key.solve(&prepared.encoded).map_err(|e| JsError::new(&e))?;
        let prove_ms = started.elapsed().as_millis() as u64;
        Ok(
            match self
                .prover
                .finish(&parts, &prepared, prove_ms, 0)
                .map(result_object)
                .map_err(unverified)
            {
                Ok(value) => Promise::resolve(&value),
                Err(error) => Promise::reject(&error),
            },
        )
    }

    /// Whether `proof_json` (the SDKs' proof JSON) proves `request`'s public
    /// input with this key.
    pub fn verify(&self, request: &str, proof_json: &str) -> Result<bool, JsError> {
        let prepared = self
            .prover
            .prepare(request)
            .map_err(|e| JsError::new(&e.to_string()))?;
        let json: serde_json::Value =
            serde_json::from_str(proof_json).map_err(|_| JsError::new("proof JSON"))?;
        let proof = zopro_core::verify::Proof::from_json(&json).map_err(JsError::new)?;
        Ok(self.prover.verify(&proof, &prepared.public))
    }
}

fn unverified(Unverified { .. }: Unverified) -> JsValue {
    JsValue::from_str("unverified")
}

fn result_object(result: ProofResult) -> JsValue {
    let object = Object::new();
    let set = |name: &str, value: JsValue| {
        let _ = Reflect::set(&object, &JsValue::from_str(name), &value);
    };
    set("proof", JsValue::from_str(&result.proof_json.to_string()));
    set("witnessMs", JsValue::from_f64(result.witness_ms as f64));
    set("proveMs", JsValue::from_f64(result.prove_ms as f64));
    set("verifyMs", JsValue::from_f64(result.verify_ms as f64));
    object.into()
}

/// The raw form's key head, vectors and solver image, when it is whole.
fn parse_raw(raw: &[u8]) -> Option<(Artifact<'_>, [Vec<u8>; 5], Vec<u8>)> {
    let mut reader = Reader { raw, at: 16 };
    let head_len = reader.length()?;
    let head = reader.take(head_len)?;
    let (artifact, end) = Artifact::head(head).ok()?;
    if end != head.len() {
        return None;
    }
    let [a, b, k, z, b2] = artifact.points.map(|n| n as usize);
    let vectors = [
        reader.take(a * 64)?.to_vec(),
        reader.take(b * 64)?.to_vec(),
        reader.take(k * 64)?.to_vec(),
        reader.take(z * 64)?.to_vec(),
        reader.take(b2 * 128)?.to_vec(),
    ];
    let image_len = reader.length()?;
    let image = reader.take(image_len)?.to_vec();
    (reader.at == raw.len()).then_some((artifact, vectors, image))
}

struct Reader<'a> {
    raw: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        let out = self.raw.get(self.at..self.at.checked_add(len)?)?;
        self.at += len;
        Some(out)
    }

    fn length(&mut self) -> Option<usize> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?) as usize)
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        JsValue::from_str(&error.to_string())
    }
}
