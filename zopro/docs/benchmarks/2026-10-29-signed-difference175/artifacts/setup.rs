use serde_json::{json,Value};use sha2::{Sha256,Digest};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/signed-difference175";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:&str)->PathBuf{Path::new(R).join(p)}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){if std::env::args().nth(1).as_deref()==Some("install"){
let p=r("bn254/src/ifma8.rs");let mut s=fs::read_to_string(&p).unwrap();assert!(!s.contains("../../kernel.rs"));s+="\ninclude!(\"../../kernel.rs\");\ninclude!(\"../../test-field.rs\");\n";fs::write(p,s).unwrap();return;
}
let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();let mut pins=BTreeMap::new();for(f,h)in m["source_files_sha256"].as_object().unwrap(){if let Some(t)=f.strip_prefix("crates/zopro-bn254/"){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());let q=r("bn254").join(t);fs::create_dir_all(q.parent().unwrap()).unwrap();fs::copy(p,&q).unwrap();pins.insert(q.display().to_string(),hash(q));}}assert_eq!(pins.len(),17);save(r("field-original-pins.json"),&json!(pins));
let s=fs::read_to_string(r("bn254/src/ifma8.rs")).unwrap();let start=s.find("    #[inline(always)]\n    fn product_cios<const FUSED: bool>").unwrap();let end=start+s[start..].find("    /// Inputs must each be below p.").unwrap();let mut body=s[start..end].to_string();
body=body.replace("fn product_cios<const FUSED: bool>","pub fn difference2_signed").replace("if FUSED {","{");
for (kind,slot) in [("lo","$j"),("hi","$k")] {for row in ["$i","4"] {
let before=format!("t[{slot}] = _mm512_madd52{kind}_epu64(t[{slot}], c.0[$j], d.0[{row}]);");let after=format!("t[{slot}] = _mm512_sub_epi64(t[{slot}], _mm512_madd52{kind}_epu64(zero, c.0[$j], d.0[{row}]));");assert_eq!(body.matches(&before).count(),1);body=body.replace(&before,&after);
}}
// The accepted cancelled-low identity ceil(t0/2^k) also holds for signed t0.
// Always select it in this private helper; every shift of a signed coefficient
// is arithmetic. No unsigned-complement/correction or inter-output Gauss path.
body=body.replace("if cfg!(zopro_cios_cancel_carry)","if true").replace("_mm512_srli_epi64", "_mm512_srai_epi64");
let original="            if FUSED && MIXED {\n                Self::reduce_fused(result)\n            } else {\n                Self(result, PhantomData)\n            }";
let finish=r#"            let negative = _mm512_srai_epi64::<63>(t[4]);
            for j in 0..5 { t[j] = _mm512_add_epi64(t[j], _mm512_and_si512(p[j],negative)); }
            for j in 0..4 {
                t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srai_epi64::<52>(t[j]));
                t[j]=_mm512_and_si512(t[j],mask);
            }
            Self([t[0],t[1],t[2],t[3],t[4]],PhantomData)"#;
assert_eq!(body.matches(original).count(),1);body=body.replace(original,finish).replace("            let result = [t[0], t[1], t[2], t[3], t[4]];\n","");assert!(!body.contains("FUSED"));fs::write(r("kernel.rs"),format!("impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{{\n{body}\n}}\n")).unwrap();
let old=Path::new("/home/ubuntu/src/zopro-perf-work/complement-difference175");for file in["test-field.rs","perf.rs","Cargo.toml","Cargo.lock"]{
let text=fs::read_to_string(old.join(file)).unwrap().replace("complement-difference","signed-difference").replace("complement_difference","signed_difference").replace("difference2_complement","difference2_signed").replace("COMPLEMENT_NATIVE","SIGNED_NATIVE");fs::write(r(file),text).unwrap();}
let probe=fs::read_to_string(old.join("src/bin/probe.rs")).unwrap().replace("complement_difference_kernel","signed_difference_kernel").replace("difference2_complement","difference2_signed");fs::write(r("src/bin/probe.rs"),probe).unwrap();println!("Prepared17exactfieldoriginals, direct signed subtract-CIOS helper and complete-expression harness; no production change");
}
