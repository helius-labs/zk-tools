use std::{fs,path::Path};
fn main(){let r="/home/ubuntu/src/zopro-perf-work/g1-control-codegen176";assert!(!Path::new(r).join("compare-scalar.rs").exists());let mut s=fs::read_to_string(format!("{r}/compare-helpers.rs")).unwrap();
for(a,b)in[
 ("join(\"helper-comparison.json\").exists()","join(\"scalar-comparison.json\").exists()"),
 ("fs::read(format!(\"{R}/comparison.json\"))","fs::read(format!(\"{R}/helper-comparison.json\"))"),
 ("[(\"invert_lanes\",\"zopro_core::msm::invert_lanes\"),(\"inverse_nonzero\",\"<zopro_core::simd::FqBatch>::inverse_nonzero\")]","[(\"scalar_inverse\",\"zopro_bn254::inverse::inverse::<zopro_bn254::FqConfig>\")]"),
 ("previous[\"rows\"][label][\"external_calls\"]","previous[\"rows\"][format!(\"{label}/inverse_nonzero\")][\"external_calls\"]"),
 ("[\"invert_lanes\",\"inverse_nonzero\"]","[\"scalar_inverse\"]"),
 ("fs::write(format!(\"{R}/helper-comparison.json\"),","fs::write(format!(\"{R}/scalar-comparison.json\"),"),
 ("one-level hot inversion callee comparison; no new capture or causal claim","remaining scalar inverse arithmetic leaf comparison; no new capture or causal claim")
]{assert_eq!(s.matches(a).count(),1,"{a}");s=s.replacen(a,b,1)}
fs::write(format!("{r}/compare-scalar.rs"),s).unwrap();println!("Prepared last scalar inversion leaf comparison; stop at panic/allocator leaves");}
