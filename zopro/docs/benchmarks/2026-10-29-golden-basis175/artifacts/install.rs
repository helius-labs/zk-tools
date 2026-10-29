use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/golden-basis175";
fn hash(b:&[u8])->String {format!("{:x}",Sha256::digest(b))}
fn main() {
    let manifest:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json").unwrap()).unwrap();
    let mut pins=serde_json::Map::new();
    for (p,h) in manifest["source_files_sha256"].as_object().unwrap() {
        if let Some(t)=p.strip_prefix("crates/zopro-bn254/") {
            let bytes=fs::read(Path::new(R).join("bn254").join(t)).unwrap();assert_eq!(hash(&bytes),h.as_str().unwrap());pins.insert(t.into(),h.clone());
        }
    } assert_eq!(pins.len(),17);
    assert!(Path::new(R).join("frozen-probe").exists());
    fs::write(Path::new(R).join("field-original-pins.json"),serde_json::to_vec_pretty(&json!(pins)).unwrap()).unwrap();
    let old="/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/source/crates/zopro-bn254/src/nonnegative_gauss.rs";
    let old_bytes=fs::read(old).unwrap();fs::write(Path::new(R).join("parent-gauss.rs"),&old_bytes).unwrap();
    let mut s=String::from_utf8(old_bytes).unwrap();let start=s.find("    #[cfg(zopro_gauss_positive)]").unwrap();
    s=format!("impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{{\n{}",&s[start..]);
    s=s.replace("#[cfg(zopro_gauss_positive)]","#[cfg(golden_basis)]").replace("complex_product_gauss","golden_product");
    let replacements=[
        ("            let mut n = [zero; 5];\n", ""),
        ("            let mut cn = zero;\n", ""),
        ("                let nv = _mm512_add_epi64(_mm512_sub_epi64(two_p[j], a1.0[j]), cn);\n                cn = _mm512_srai_epi64::<52>(nv);\n                n[j] = _mm512_and_si512(nv, mask);\n", ""),
        ("            let mut ct = zero;\n", ""),
        ("                    let tv = _mm512_add_epi64(_mm512_add_epi64(b0.0[$i], b1.0[$i]), ct);\n                    ct = _mm512_srli_epi64::<52>(tv);\n                    let ti = _mm512_and_si512(tv, mask);\n", ""),
        ("_mm512_sub_epi64(_mm512_add_epi64(b1.0[$i], two_p[$i]), b0.0[$i])", "_mm512_sub_epi64(_mm512_add_epi64(b0.0[$i], two_p[$i]), b1.0[$i])"),
    ];
    for (a,b) in replacements {assert_eq!(s.matches(a).count(),1,"missing {a}");s=s.replace(a,b);}
    for kind in ["lo","hi"] {
        let slot=if kind=="lo"{"$j"}else{"$k"};
        for (a,b) in [
            (format!("_mm512_madd52{kind}_epu64(zero, s[$j], b0.0[$i])"),format!("_mm512_madd52{kind}_epu64(zero, s[$j], b1.0[$i])")),
            (format!("_mm512_madd52{kind}_epu64(x[{slot}], n[$j], ti)"),format!("_mm512_madd52{kind}_epu64(x[{slot}], a0.0[$j], di)")),
            (format!("_mm512_madd52{kind}_epu64(y[{slot}], a0.0[$j], di)"),format!("_mm512_madd52{kind}_epu64(y[{slot}], a1.0[$j], b0.0[$i])")),
        ] {assert_eq!(s.matches(&a).count(),1);s=s.replace(&a,&b);}
    }
    s.push_str(&fs::read_to_string(Path::new(R).join("division.rs")).unwrap());
    fs::write(Path::new(R).join("kernel.rs"),s).unwrap();
    let p=Path::new(R).join("bn254/src/ifma8.rs");let mut s=fs::read_to_string(&p).unwrap();
    s+="\n#[cfg(golden_basis)] include!(\"../../kernel.rs\");\n#[cfg(all(test,golden_basis))] include!(\"../../test-field.rs\");\n";
    fs::write(p,s).unwrap();println!("17 exact baseline field sources verified; frozen baseline retained; one include suffix and private kernel installed");
}
