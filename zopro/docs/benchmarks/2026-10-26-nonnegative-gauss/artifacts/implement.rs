use std::{fs,path::Path,process::Command};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let r=Path::new("/tmp/zopro-nonnegative-gauss");
 let old=fs::read_to_string("/tmp/zopro-streamed-complex-b/candidate/crates/zopro-bn254/src/ifma.rs").unwrap();
 let start=old.find("    // BEGIN STREAMED_COMPLEX_B\n").unwrap();let end=old[start..].find("    // END STREAMED_COMPLEX_B\n").unwrap()+start+"    // END STREAMED_COMPLEX_B\n".len();
 let control=old[start..end].replace("zopro_streamed_complex_b","zopro_gauss_positive");
 let cancel_start=control.find("                    macro_rules! cancel {").unwrap();let cancel_end=control.find("                    cancel!(x);").unwrap();
 let cancel=control[cancel_start..cancel_end].replace("_mm256_srai_epi64","_mm256_srli_epi64");
 let mut body=fs::read_to_string(r.join("positive-prefix.rs.txt")).unwrap();body.push_str(&cancel);body.push_str(&fs::read_to_string(r.join("positive-suffix.rs.txt")).unwrap());
 let additions=control+"\n    // BEGIN NONNEGATIVE_GAUSS\n"+&body+"    // END NONNEGATIVE_GAUSS\n\n";
 let p=r.join("candidate/crates/zopro-bn254/src/ifma.rs");let base=fs::read_to_string(&p).unwrap();let anchor="    #[inline(always)]\n    pub fn mul(&self, b: &Self) -> Self {";assert_eq!(base.matches(anchor).count(),1);fs::write(&p,base.replacen(anchor,&(additions.clone()+anchor),1)).unwrap();fs::write(r.join("field-additions.rs.txt"),additions).unwrap();
 let out=Command::new("python3").arg(r.join("candidate/scripts/ifma8.py")).output().unwrap();fs::write(r.join("generator.stdout.log"),out.stdout).unwrap();fs::write(r.join("generator.stderr.log"),out.stderr).unwrap();assert!(out.status.success());
 let manifest:Value=serde_json::from_slice(&fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap()).unwrap();let mut changes=vec![];
 for (name,digest) in manifest["source_files_sha256"].as_object().unwrap(){let p=r.join("candidate").join(name);if hash(&p)!=digest.as_str().unwrap(){assert!(name=="crates/zopro-bn254/src/ifma.rs"||name=="crates/zopro-bn254/src/ifma8.rs");let mut s=fs::read_to_string(&p).unwrap();for tag in ["STREAMED_COMPLEX_B","NONNEGATIVE_GAUSS"]{let a=s.find(&format!("    // BEGIN {tag}\n")).unwrap();let marker=format!("    // END {tag}\n");let b=s.find(&marker).unwrap()+marker.len();s.replace_range(a..b,"");}let original=fs::read_to_string(Path::new("/home/ubuntu/src/zk-tools/zopro").join(name)).unwrap();assert!(s.contains("\n\n\n    #[inline(always)]\n    pub fn mul("));s=s.replacen("\n\n\n    #[inline(always)]\n    pub fn mul(","\n    #[inline(always)]\n    pub fn mul(",1);assert!(s==original,"whole-original inverse audit failed");changes.push(json!({"path":name,"sha256":hash(&p),"original_sha256":digest}));}}
 assert_eq!(changes.len(),2);fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"changes":changes,"other_168_files_unchanged":true,"originals_recovered_except_blank_lines":true,"no_production_changes":true})).unwrap()).unwrap();
 let probe=fs::read_to_string("/tmp/zopro-streamed-complex-b/probe.rs").unwrap().replace("zopro_streamed_complex_b","zopro_gauss_positive").replace("complex_product_streamed","complex_product_gauss").replace("complex_product_shared","complex_product_streamed").replace("complex_streamed_codegen","complex_gauss_codegen").replace("complex_shared_codegen","complex_streamed_codegen").replace("\"streamed\"","\"gauss\"").replace("\"shared\"","\"streamed\"").replace("old|shared|streamed","old|streamed|gauss");assert!(probe.contains("include!(\"probe-tests.rs\")"));fs::write(r.join("probe.rs"),probe).unwrap();
 fs::create_dir(r.join("oracle")).unwrap();for name in ["Cargo.toml","Cargo.lock"]{let s=fs::read_to_string(Path::new("/tmp/zopro-streamed-complex-b/oracle").join(name)).unwrap().replace("streamed-complex-probe","nonnegative-gauss-probe");fs::write(r.join("oracle").join(name),s).unwrap();}
 println!("Added positive Gauss method and exact old streamed diagnostic to two field sources; other168 files unchanged");
}
