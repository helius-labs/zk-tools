use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/late-m3-query176";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}fn load(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn append(p:&str,s:&str){let q=r("source").join(p);let mut b=fs::read(q.clone()).unwrap();b.extend_from_slice(s.as_bytes());fs::write(q,b).unwrap();}
fn main(){
 assert!(!r("source").exists());let m=load(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
 for(p,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(p);assert_eq!(hash(&from),h.as_str().unwrap());let to=r("source").join(p);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
 fs::create_dir_all(r("source/crates/zopro-core/src/msm_affine/g2_borrowed")).unwrap();fs::copy(r("late_m3.rs"),r("source/crates/zopro-core/src/msm_affine/g2_borrowed/late_m3.rs")).unwrap();
 append("crates/zopro-core/src/msm_affine/g2_borrowed.rs","\n#[cfg(zopro_late_m3)]mod late_m3;\n#[cfg(zopro_late_m3)]pub(super)fn try_late_m3(bases:&[G2Affine],words:&[[u64;4]])->Option<(G2Projective,usize)>{late_m3::try_query(bases,words)}\n");
 append("crates/zopro-core/src/msm_affine/workspace.rs",&format!("\n{}",fs::read_to_string(r("workspace-suffix.rs")).unwrap()));
 let p="crates/zopro-core/src/msm_affine.rs";let original=fs::read_to_string(r("source").join(p)).unwrap();
 let anchor="        return g2_phi::raw_exclusive(bases, words);";assert_eq!(original.matches(anchor).count(),1);
 let hook="        #[cfg(zopro_late_m3)]\n        if std::env::var(\"ZOPRO_CPU_LATE_M3\").as_deref()==Ok(\"1\") {\n            if let Some((result,_peak))=g2_borrowed::try_late_m3(bases,words) {\n                crate::coalesce_query_probe::record(40*bases.len());\n                return result;\n            }\n        }\n";
 fs::write(r("source").join(p),original.replace(anchor,&format!("{hook}{anchor}"))).unwrap();
 let export=r#"
#[cfg(all(zopro_coalesce_probe,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
pub mod coalesce_query_probe {
 use zopro_bn254::{G1Affine,G2Affine,G1Projective,G2Projective};
 #[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct Metadata{pub route:u32,pub classes:usize,pub skips:usize,pub payload:usize}
 std::thread_local!{static META:std::cell::Cell<Metadata>=std::cell::Cell::new(Metadata::default());}
 pub(crate)fn record(payload:usize){META.with(|m|m.set(Metadata{route:313,payload,..Metadata::default()}));}
 pub fn a(bases:&[G1Affine],words:&[[u64;4]])->(G1Projective,Metadata){(crate::msm_affine::msm_canonical(bases,words),Metadata::default())}
 pub fn b(bases:&[G2Affine],words:&mut[[u64;4]])->(G2Projective,Metadata){META.with(|m|m.set(Metadata::default()));let p=crate::msm_affine::g2_query_exclusive(bases,words);(p,META.with(|m|m.get()))}
}
"#;
 fs::write(r("probe-export.rs"),export).unwrap();append("crates/zopro-core/src/lib.rs",export);
 let mut recovery=BTreeMap::new();let mut exact=0;
 for(p,h)in m["source_files_sha256"].as_object().unwrap(){let mut s=fs::read(r("source").join(p)).unwrap();let old=fs::read(Path::new(Z).join(p)).unwrap();if hash(r("source").join(p))==h.as_str().unwrap(){exact+=1;}else if p=="crates/zopro-core/src/msm_affine.rs"{s=String::from_utf8(s).unwrap().replace(hook,"").into_bytes();}else{s.truncate(old.len());}assert_eq!(s,old);recovery.insert(p.clone(),true);}
 assert_eq!(exact,172);let audit=json!({"originals":176,"snapshot_files":177,"originals_exact":exact,"all_original_bytes_recover":recovery,"production_changed":false,"field_and_original_kernels_exact":true,"grouping_and_fallback_body_unchanged":true,"dispatch_hook":hook});fs::write(r("source-audit.json"),serde_json::to_vec_pretty(&audit).unwrap()).unwrap();
 let flags=format!("{} --cfg zopro_coalesce_probe --cfg zopro_late_m3",m["rustflags"].as_str().unwrap());fs::write(r("candidate-flags.txt"),&flags).unwrap();fs::write(r("native-flags.txt"),&flags).unwrap();
 let old="/home/ubuntu/src/zopro-perf-work/coalesce-query176";
 for f in["native.rs","driver.rs","perf_support.rs","fixtures.json"]{let s=fs::read_to_string(format!("{old}/{f}")).unwrap();fs::write(r(f),if f=="native.rs"{s.replace(old,R)}else{s}).unwrap();}
 println!("Prepared177snapshotfiles/172originalexact/176recover;no field or production edit");
}
