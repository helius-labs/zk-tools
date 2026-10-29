use serde_json::Value;use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};const R:&str="/home/ubuntu/src/zopro-perf-work/row-quotient175";const OLD:&str="/home/ubuntu/src/zopro-perf-work/triple-cancel175";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}fn once(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replace(a,b);}
fn main(){let r=Path::new(R);let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();let mut pins=BTreeMap::new();for(f,h)in m["source_files_sha256"].as_object().unwrap(){if let Some(tail)=f.strip_prefix("crates/zopro-bn254/"){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());let to=r.join("bn254").join(tail);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(&p,&to).unwrap();pins.insert(to.display().to_string(),hash(to));}}assert_eq!(pins.len(),17);fs::write(r.join("field-original-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
let original=fs::read_to_string(r.join("bn254/src/ifma8.rs")).unwrap();let start=original.find("    #[inline(always)]\n    fn product_cios<const FUSED: bool>").unwrap();let end=start+original[start..].find("    /// Inputs must each be below p.").unwrap();let mut body=original[start..end].replace("fn product_cios<","fn product_rowq<");
once(&mut body,"let mut t = [zero; 6];",r#"let mut t = [zero; 6];
            let a0i=_mm512_madd52lo_epu64(zero,self.0[0],inv);
            let c0i=if FUSED{_mm512_madd52lo_epu64(zero,c.0[0],inv)}else{zero};
            let rowq:[__m512i;5]=core::array::from_fn(|i|{
                let q=_mm512_madd52lo_epu64(zero,a0i,b.0[i]);
                if FUSED{_mm512_madd52lo_epu64(q,c0i,d.0[i])}else{q}
            });"#);
once(&mut body,"($i:literal) => {{\n                    macro_rules! product",r#"($i:literal) => {{
                    let m=if $i==0{_mm512_and_si512(rowq[$i],mask)}else{
                        _mm512_and_si512(_mm512_madd52lo_epu64(rowq[$i],t[0],inv),mask)
                    };
                    macro_rules! product"#);
once(&mut body,"                    let m = _mm512_and_si512(_mm512_madd52lo_epu64(zero, t[0], inv), mask);\n","");
once(&mut body,"            if MIXED {\n                // Four radix-52",r#"            if MIXED {
                let m=_mm512_and_si512(_mm512_madd52lo_epu64(rowq[4],t[0],inv),Self::splat((1u64<<48)-1));
                // Four radix-52"#);
once(&mut body,"                let m = _mm512_and_si512(\n                    _mm512_madd52lo_epu64(zero, t[0], inv),\n                    Self::splat((1 << 48) - 1),\n                );\n","");
let prefix="impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{\n#[inline(always)]pub fn mul_rowq(&self,b:&Self)->Self{self.product_rowq::<false>(b,self,b)}\n#[inline(always)]pub fn sum2_rowq(&self,b:&Self,c:&Self,d:&Self)->Self{self.product_rowq::<true>(b,c,d)}\n";fs::write(r.join("kernel.rs"),format!("{prefix}{body}\n}}\n")).unwrap();
let probe=fs::read_to_string(Path::new(OLD).join("src/bin/probe.rs")).unwrap().replace("triple_cancel_kernel","row_quotient_kernel").replace("triple","rowq");fs::write(r.join("src/bin/probe.rs"),probe).unwrap();let mut test=fs::read_to_string(Path::new(OLD).join("test-field.rs")).unwrap();let a=test.find("        let inverse = if C::P").unwrap();let b=a+test[a..].find("        let vals = [").unwrap();test.replace_range(a..b,"");test=test.replace("triple","rowq").replace("TRIPLE_ARKWORKS","ROWQ_ARKWORKS");fs::write(r.join("test-field.rs"),test).unwrap();
let cargo=fs::read_to_string(Path::new(OLD).join("Cargo.toml")).unwrap().replace("zopro-triple-cancel-probe","zopro-row-quotient-probe").replace("sha2 = \"=0.10.9\"","sha2 = \"=0.10.9\"\nnum-bigint=\"0.4\"\nnum-traits=\"0.2\"");fs::write(r.join("Cargo.toml"),cargo).unwrap();
let mut model=fs::read_to_string(Path::new(OLD).join("model.rs")).unwrap();let a=model.find("    let mut wide = [0u128; 8];").unwrap();let b=model.find("    for j in 0..4 {\n        t[j + 1]").unwrap();let suffix_start=model.find("        let low = cancel(&mut t, 0, m, &pd, bits);").unwrap();let suffix_end=b;let suffix=model[suffix_start..suffix_end].to_string();let replacement=format!(r#"    let a0i=(a[0]*iv[0])&MASK;let c0i=(c[0]*iv[0])&MASK;
    let partials:[u128;5]=core::array::from_fn(|i|((a0i*b[i])&MASK)+if fused{{(c0i*d[i])&MASK}}else{{0}});
    let mut t=[0u128;6];
    for i in 0..5{{
        let bits=if i==4&&mixed{{48}}else{{52}};let digit_mask=(1u128<<bits)-1;
        let qraw=((t[0]*iv[0])&MASK)+partials[i];assert!(qraw<3*B);let m=qraw&digit_mask;
        stats.max_q1=stats.max_q1.max(*partials.iter().max().unwrap());stats.max_q2=stats.max_q2.max(qraw);
        let before=integer(&t);let row= input[0]*N::from(b[i])+if fused{{input[2]*N::from(d[i])}}else{{N::zero()}};
        for j in 0..5{{pair(&mut t,j,a[j],b[i]);if fused{{pair(&mut t,j,c[j],d[i]);}}}}
        bound(&t,stats);assert_eq!(m,(t[0]*iv[0])&digit_mask);stats.prefix_checks+=1;
        assert_eq!(integer(&t),before+row);
{suffix}"#);model.replace_range(a..b,&replacement);model=model.replace("inverse156","negative_inverse_low156_reference").replace("everyprefixquotient andpostprefixinteger;","everydistributedrowquotient andrawrowinteger;");fs::write(r.join("src/bin/model.rs"),model).unwrap();println!("Prepared17fieldfiles/private105/161IFMArowquotient kernels; model60168cases/test345088lanes; zero production edits");}
