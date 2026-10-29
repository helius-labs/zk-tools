use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-product-sub175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        if let Some(tail) = f.strip_prefix("crates/zopro-bn254/") {
            let a = Path::new(Z).join(f);
            assert_eq!(hash(&a), h.as_str().unwrap());
            let b = Path::new(R).join("bn254").join(tail);
            fs::create_dir_all(b.parent().unwrap()).unwrap();
            fs::copy(a, &b).unwrap();
            pins.insert(b.display().to_string(), hash(b));
        }
    }
    assert_eq!(pins.len(), 17);
    fs::write(
        format!("{R}/field-original-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let s = fs::read_to_string(format!("{R}/bn254/src/ifma8.rs")).unwrap();
    let a = s
        .find("    #[inline(always)]\n    fn product_cios<const FUSED: bool>")
        .unwrap();
    let b = a + s[a..].find("    /// Inputs must each be below p.").unwrap();
    let body = &s[a..b];
    let norm="            for j in 0..4 {\n                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));\n                t[j] = _mm512_and_si512(t[j], mask);\n            }";
    assert_eq!(body.matches(norm).count(), 1);
    let new=body.replace("fn product_cios<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self)","fn product_sub_canonical<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self, sub: &Self)").replace(norm,"            let modulus = split(C::P).map(Self::splat);\n            let mut signed_carry = zero;\n            for j in 0..5 {\n                let value = _mm512_add_epi64(_mm512_sub_epi64(_mm512_add_epi64(t[j], modulus[j]), sub.0[j]), signed_carry);\n                t[j] = _mm512_and_si512(value,mask);\n                signed_carry = _mm512_srai_epi64::<52>(value);\n            }");
    let ordinary = s.find("    pub fn difference2(&self,").unwrap();
    let end = ordinary
        + s[ordinary..]
            .find("    #[inline(always)]\n    fn product<")
            .unwrap();
    let diff = s[ordinary..end]
        .replace(
            "pub fn difference2(&self, b: &Self, c: &Self, d: &Self)",
            "pub fn difference2_sub_canonical(&self, b: &Self, c: &Self, d: &Self, sub: &Self)",
        )
        .replace(
            "self.product::<true>(b, &negative, d)",
            "self.product_sub_canonical::<true>(b, &negative, d, sub)",
        );
    let wrappers=format!("impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{{\n#[inline(always)]pub fn sum2_sub_canonical(&self,b:&Self,c:&Self,d:&Self,sub:&Self)->Self{{assert!(MIXED);self.product_sub_canonical::<true>(b,c,d,sub)}}\n#[inline(always)]{diff}\n{new}\n}}\n");
    fs::write(format!("{R}/kernel.rs"), wrappers).unwrap();
    println!("Prepared17acceptedfieldfiles; newfusedCIOSfinalsignedpass; no sourceproductionedits");
}
