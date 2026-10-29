use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/mask-carry175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let manifest:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut originals = BTreeMap::new();
    for (f, h) in manifest["source_files_sha256"].as_object().unwrap() {
        if let Some(tail) = f.strip_prefix("crates/zopro-bn254/") {
            let from = Path::new(Z).join(f);
            assert_eq!(hash(&from), h.as_str().unwrap());
            let to = Path::new(R).join("bn254").join(tail);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(&from, &to).unwrap();
            originals.insert(to.display().to_string(), hash(to));
        }
    }
    assert_eq!(originals.len(), 17);
    fs::write(
        format!("{R}/field-original-pins.json"),
        serde_json::to_vec_pretty(&originals).unwrap(),
    )
    .unwrap();
    let template = fs::read_to_string(format!("{R}/bn254/src/ifma8.rs")).unwrap();
    let start = template
        .find("    #[inline(always)]\n    fn product_cios<const FUSED: bool>")
        .unwrap();
    let end = start
        + template[start..]
            .find("    /// Inputs must each be below p.")
            .unwrap();
    let body = &template[start..end];
    let old="            for j in 0..4 {\n                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));\n                t[j] = _mm512_and_si512(t[j], mask);\n            }";
    assert_eq!(body.matches(old).count(), 1);
    let body=body.replace("fn product_cios<","fn product_masked<").replace(old,"            let out = Self::normalize_masked([t[0], t[1], t[2], t[3], t[4]]);\n            t = [out[0],out[1],out[2],out[3],out[4],zero];");
    let normalize = fs::read_to_string(format!("{R}/normalize.rs")).unwrap();
    let tests = fs::read_to_string(format!("{R}/native-tests.rs")).unwrap();
    fs::write(format!("{R}/kernel.rs"),format!("impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{{\n#[inline(always)]pub fn mul_masked(&self,b:&Self)->Self{{self.product_masked::<false>(b,self,b)}}\n#[inline(always)]pub fn sum2_masked(&self,b:&Self,c:&Self,d:&Self)->Self{{self.product_masked::<true>(b,c,d)}}\n{normalize}\n{body}\n}}\n{tests}")).unwrap();
    let probe =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/paired-cancel/src/bin/probe.rs")
            .unwrap()
            .replace("paired_kernel", "mask_carry_kernel")
            .replace("mul_lookahead", "mul_masked")
            .replace("sum2_lookahead", "sum2_masked")
            .replace("\"paired\"", "\"masked\"");
    fs::write(format!("{R}/src/bin/probe.rs"), probe).unwrap();
    println!("Copied17acceptedfieldfiles; generatedCIOSbodywithonefinalnormalizationreplacement");
}
