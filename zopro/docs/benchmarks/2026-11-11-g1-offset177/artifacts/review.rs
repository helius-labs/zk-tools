use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs};
const R:&str="/home/ubuntu/src/zopro-perf-work/g1-offset177";
const F:&str="/home/ubuntu/src/zopro-perf-work/affine-redc-offset-mask176";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let audit=read(&format!("{R}/source-audit.json"));
 let manifest_path=audit["base_manifest"].as_str().unwrap();
 assert_eq!(hash(manifest_path),audit["base_manifest_sha256"]);
 let manifest=read(manifest_path);let base=manifest["source_files_sha256"].as_object().unwrap();
 let mut changed=BTreeMap::new();
 for(rel,h)in base{let p=format!("{R}/source/{rel}");let actual=hash(&p);if actual!=h.as_str().unwrap(){changed.insert(rel.clone(),actual);}}
 assert_eq!(changed.keys().map(String::as_str).collect::<Vec<_>>(),vec!["crates/zopro-bn254/src/ifma8.rs","crates/zopro-core/src/msm.rs","crates/zopro-core/src/simd.rs"]);
 let mut msm=fs::read_to_string(format!("{R}/source/crates/zopro-core/src/msm.rs")).unwrap();
 let test_suffix="\n#[cfg(all(test,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nmod affine_offset_caller;\n";
 msm=msm.strip_suffix(test_suffix).unwrap().to_owned();
 for edit in audit["G1_edits"].as_array().unwrap().iter().rev(){let new=edit["new"].as_str().unwrap();assert_eq!(msm.matches(new).count(),1);msm=msm.replace(new,edit["old"].as_str().unwrap());}
 assert_eq!(format!("{:x}",Sha256::digest(msm.as_bytes())),base["crates/zopro-core/src/msm.rs"]);
 let inherited=read(&format!("{F}/native-fixed-pins.json"));let mut field_inputs=0;
 for(p,h)in inherited.as_object().unwrap(){
  if let Some(rel)=p.strip_prefix(&format!("{F}/source-fixed/")){
   if rel.starts_with("crates/zopro-bn254/"){
    assert_eq!(hash(p),h.as_str().unwrap());
    if !rel.ends_with("/ifma8.rs"){assert_eq!(hash(&format!("{R}/source/{rel}")),h.as_str().unwrap());}
    field_inputs+=1;
   }
  }
 }
 assert!(field_inputs>=10);
 let field_body=fs::read_to_string(format!("{R}/source/crates/zopro-bn254/src/ifma8.rs")).unwrap();
 let field_body=field_body.strip_suffix("\n#[cfg(all(zopro_g1_affine_offset, zopro_cios_cancel_carry))]\nmod affine_offset;\n").unwrap();
 assert_eq!(format!("{:x}",Sha256::digest(field_body.as_bytes())),base["crates/zopro-bn254/src/ifma8.rs"]);
 let pins=read(&format!("{R}/build-pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let ledger="/home/ubuntu/src/zopro-perf-work/gather-backing177/proof-sha256-after-capture.json";
 assert_eq!(hash(ledger),"7069c3c56ce4619e024753ef25eccd24b8d36ac1211929b923fdd2767e1e76ca");
 let value=json!({"passed":true,"base_files":base.len(),"exact_original_files":base.len()-changed.len(),"changed_files":changed,"G1_edits_reversed_to_accepted_hash":true,"G2_sources_exact":true,"field_inputs_verified_against_qualified_build_pins":field_inputs,"field_arithmetic_module_exact":true,"new_source_pins_checked":pins.as_object().unwrap().len(),"native_helper_sha256":hash(&format!("{R}/native-helper")),"canonical_ledger_sha256":hash(ledger),"SDK_calls":0,"new_exports":0,"adopted":false,"performance_tested":false});
 fs::write(format!("{R}/source-review.json"),serde_json::to_vec_pretty(&value).unwrap()).unwrap();
 println!("Source review passed; {field_inputs} inherited field inputs checked; no SDK execution.");
}
