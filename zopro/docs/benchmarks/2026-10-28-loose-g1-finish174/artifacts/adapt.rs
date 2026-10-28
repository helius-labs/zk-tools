use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/loose-g1-finish174";
fn main(){
 let old="/home/ubuntu/src/zopro-perf-work/dual-plane-g2-174";
 let s=fs::read_to_string(format!("{old}/native.rs")).unwrap().replace(old,R).replace("zopro_g2_dual_plane","zopro_loose_g1_finish");
 fs::write(format!("{R}/native.rs"),s).unwrap();
 let s=fs::read_to_string(format!("{old}/capture.rs")).unwrap().replace(old,R)
  .replace("const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174\";",&format!("const OLD: &str = \"{old}\";"))
  .replace("zopro_g2_dual_plane","zopro_loose_g1_finish").replace("dual_plane_compile_cfg","loose_g1_finish_compile_cfg")
  .replace("prior.len(), 15553","prior.len(), 15757")
  .replace("prior.len(), 15757","prior.len(), 15811")
  .replace("prior.len(), 15811);\n    for", "prior.len(), 15757);\n    for")
  .replace("prior.len(), 15811);\n    save(r(\"proof-inventory.json\")", "prior.len(), 15865);\n    save(r(\"proof-inventory.json\")")
  .replace("ledger 15757 -> 15811","ledger 15811 -> 15865")
  .replace("ledger15811","ledger15865")
  .replace("if label==\"control\"{122}else{124}","122")
  .replace("completed joint two-plane G2 primary SDK screen;no adoption","completed bounded loose G1 finish primary SDK screen;no adoption")
  .replace("\"independent_proof_count\":15811","\"independent_proof_count\":15865")
  .replace("\"native_field_tests\":\"field source exact;inherited accepted33test qualification;not rerun\"","\"native_field_tests\":{\"OFF\":33,\"ON\":34}")
  .replace("\"native_core_tests\":{\"OFF\":122,\"ON\":124}","\"native_core_tests\":{\"OFF\":122,\"ON\":122}")
  .replace("joint G2 high-low bucket planes;15blocks primary32 only;one query resource reservation;not larger/all8/clean/resource qualification","bounded loose G1 coordinate finish;15blocks primary32 only;not larger/all8/clean/resource qualification");
 assert!(s.contains(&format!("const OLD: &str = \"{old}\";")));
 fs::write(format!("{R}/capture.rs"),s).unwrap();
 let s=fs::read_to_string(format!("{old}/close.rs")).unwrap().replace(old,R)
 .replace("2026-10-28-dual-plane-g2-174","2026-10-28-loose-g1-finish174").replace("],15811", "],15865");
 fs::write(format!("{R}/close.rs"),s).unwrap();
 for f in ["native.rs","capture.rs","close.rs"]{assert!(Path::new(R).join(f).exists());}
}
