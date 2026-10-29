use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/complement-difference175";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:&str)->PathBuf{Path::new(R).join(p)}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){let install=std::env::args().nth(1).as_deref()==Some("install");if install{let p=r("bn254/src/ifma8.rs");let mut s=fs::read_to_string(&p).unwrap();assert!(!s.contains("../../kernel.rs"));s+="\ninclude!(\"../../kernel.rs\");\ninclude!(\"../../test-field.rs\");\n";fs::write(p,s).unwrap();println!("Installed private helper; original17recoverable by removing suffix");return}
let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();let mut pins=BTreeMap::new();for(f,h)in m["source_files_sha256"].as_object().unwrap(){if let Some(t)=f.strip_prefix("crates/zopro-bn254/"){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());let q=r("bn254").join(t);fs::create_dir_all(q.parent().unwrap()).unwrap();fs::copy(p,&q).unwrap();pins.insert(q.display().to_string(),hash(q));}}assert_eq!(pins.len(),17);save(r("field-original-pins.json"),&json!(pins));
let s=fs::read_to_string(r("bn254/src/ifma8.rs")).unwrap();let start=s.find("    #[inline(always)]\n    fn product_cios<const FUSED: bool>").unwrap();let end=start+s[start..].find("    /// Inputs must each be below p.").unwrap();let body=&s[start..end];
let norm="            for j in 0..4 {\n                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));\n                t[j] = _mm512_and_si512(t[j], mask);\n            }";assert_eq!(body.matches(norm).count(),1);
let seed=r#"            // n=R-1-c: each digit is complemented independently.
            let negative = Self(core::array::from_fn(|j| _mm512_xor_si512(c.0[j], Self::splat(if MIXED && j == 4 {(1u64<<48)-1} else {MASK}))), PhantomData);
            let c = &negative;
            // X=ab+(R-1-c)d+d=ab-cd+Rd is nonnegative.
            let mut t = [d.0[0],d.0[1],d.0[2],d.0[3],d.0[4],zero];"#;
let normalize=r#"            // REDC(X)+p-d=(ab-cd+qp)/R+p is strictly between0and3p.
            // Inject the correction before the original final carry pass.
            let mut carry = zero;
            for j in 0..5 {
                let value = _mm512_add_epi64(_mm512_sub_epi64(_mm512_add_epi64(t[j],p[j]),d.0[j]),carry);
                carry = _mm512_srai_epi64::<52>(value);
                t[j] = _mm512_and_si512(value,mask);
            }"#;
let finalold="            if FUSED && MIXED {\n                Self::reduce_fused(result)\n            } else {\n                Self(result, PhantomData)\n            }";assert_eq!(body.matches(finalold).count(),1);
let new=body.replace("fn product_cios<const FUSED: bool>","pub fn difference2_complement").replace("if FUSED {","{").replace("            let mut t = [zero; 6];",seed).replace(norm,normalize).replace(finalold,"            Self::reduce_fused(result)");assert!(!new.contains("FUSED"));fs::write(r("kernel.rs"),format!("impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{{\n{new}\n}}\n")).unwrap();
fs::create_dir_all(r("src/bin")).unwrap();let mut probe=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-product-sub175/src/bin/probe.rs").unwrap();probe=probe.replace("Field, FpConfig, ","");let start=probe.find("#[inline(always)]\nfn mul(").unwrap();let end=start+probe[start..].find("#[inline(always)]\nfn square").unwrap();probe.replace_range(start..end,r#"#[inline(always)]
fn difference<const ALT:bool>(a:V,b:V,c:V,d:V)->V {
    #[cfg(complement_difference_kernel)]
    if ALT {return a.difference2_complement(&b,&c,&d);}
    a.difference2(&b,&c,&d)
}
#[inline(always)]
fn mul<const ALT:bool>(a:Pair,b:Pair)->Pair {
    [difference::<ALT>(a[0],b[0],a[1],b[1]),a[0].sum2(&b[1],&a[1],&b[0])]
}
"#);
let start=probe.find("#[inline(always)]\nfn product_sub<").unwrap();let end=start+probe[start..].find("#[inline(never)]").unwrap();probe.replace_range(start..end,"#[inline(always)]\nfn product_sub<const ALT:bool>(a:Pair,b:Pair,c:Pair)->Pair{sub(mul::<ALT>(a,b),c)}\n");probe=probe.replace("dy[1].difference2(&dx[0], &dy[0], &dx[1])","difference::<ALT>(dy[1],dx[0],dy[0],dx[1])");probe=probe.replace("use std::{hint::black_box, time::Instant};","use std::{hint::black_box, time::Instant};\n#[path=\"../../perf.rs\"]mod perf;");probe=probe.replace("    let start = Instant::now();","    let mut control=perf::Control::from_env().unwrap();\n    if let Some(c)=control.as_mut(){c.enable().unwrap();}\n    let start = Instant::now();");probe=probe.replace("    let elapsed = start.elapsed();","    let elapsed = start.elapsed();\n    if let Some(c)=control.as_mut(){c.disable().unwrap();}");fs::write(r("src/bin/probe.rs"),probe).unwrap();fs::copy("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/perf.rs",r("perf.rs")).unwrap();
fs::write(r("Cargo.toml"),r#"[package]
name="zopro-complement-difference-probe"
version="0.0.0"
edition="2021"
publish=false
[workspace]
members=["bn254"]
[dependencies]
zopro-bn254={path="bn254"}
serde_json="=1.0.149"
sha2="=0.10.9"
num-bigint="=0.4.8"
num-traits="=0.2.19"
libc="=0.2.191"
[profile.release]
opt-level=3
lto=true
codegen-units=1
"#).unwrap();println!("Prepared17exactfields/privatecomplementhelper/probe; install only after frozen build");}
