use serde_json::{json,Value}; use sha2::{Digest,Sha256}; use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-native-buckets176"; const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:&str)->PathBuf{Path::new(R).join(p)} fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){
 assert!(!r("source-audit.json").exists());
 let manifest:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json")).unwrap()).unwrap();
 let originals=manifest["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),176);
 let old=fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm/g2_borrowed.rs")).unwrap();
 let a=old.find("pub(super) fn apply<").unwrap();let b=old.find("\nimpl<C: CurveConfig<Base = Fq2>> BatchAdd<C>").unwrap();let original=&old[a..b];
 let mut candidate=original.to_owned();let mut edits=vec![];
 for (from,to) in [
 ("pub(super) fn apply<","fn apply_native<"),
 ("buckets: &mut [Affine<C>]","buckets: &mut NativeBuckets<C>"),
 ("        let ax = fq2_pack(core::array::from_fn(|i| {\n            buckets[(batch[k + i].0 & 0x7fffffff) as usize].x\n        }));","        let forward_ids=core::array::from_fn(|i|batch[k+i].0&0x7fffffff);\n        let ax=buckets.x(forward_ids);"),
 ("                let a = buckets[index as usize];","                let a = buckets.point(index as usize);"),
 ("                buckets[(batch[k].0 & 0x7fffffff) as usize] = Affine::new(x, y);","                buckets.set((batch[k].0 & 0x7fffffff) as usize, Affine::new(x,y));"),
 ("        let a: [Affine<C>; LANES] =\n            core::array::from_fn(|i| buckets[(batch[k + i].0 & 0x7fffffff) as usize]);","        let ids=core::array::from_fn(|i|batch[k+i].0&0x7fffffff);"),
 ("        let ax = fq2_pack(a.map(|p| p.x));\n        let ay = fq2_pack(a.map(|p| p.y));","        let ax=buckets.x(ids);\n        let ay=buckets.y(ids);"),
 ("        let (x, y) = (fq2_unpack(x), fq2_unpack(y));\n        for i in 0..LANES {\n            buckets[(batch[k + i].0 & 0x7fffffff) as usize] = Affine::new(x[i], y[i]);\n        }","        buckets.put(ids,x,y);")
 ]{assert_eq!(candidate.matches(from).count(),1,"{from}");candidate=candidate.replacen(from,to,1);edits.push(json!([from,to]));}
 let mut recovered=candidate.clone();for pair in edits.iter().rev(){recovered=recovered.replacen(pair[1].as_str().unwrap(),pair[0].as_str().unwrap(),1);}assert_eq!(recovered,original);
 fs::write(r("native-apply.rs"),&candidate).unwrap();
 let mut g1=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-denominator-tree176/g1-caller.rs").unwrap();g1=g1.replace("pub struct G1Caller","pub struct G1Caller");
 let mut g2=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-denominator-tree176/g2-caller.rs").unwrap();
 let mut caller_edits=vec![];
 for(from,to)in[
 ("    source:Vec<G2Affine>, buckets:Vec<G2Affine>, incoming:Vec<G2Affine>,","    source:Vec<G2Affine>, buckets:Vec<G2Affine>, native:NativeBuckets<G2Config>, incoming:Vec<G2Affine>, active_native:bool,"),
 ("        let size=n.max(1).next_power_of_two();","        let size=2048;"),
 ("        Self{buckets:source.clone(),source,incoming,owned,unsigned,size,seed,scratch:","        Self{native:NativeBuckets::new(&source),active_native:false,buckets:source.clone(),source,incoming,owned,unsigned,size,seed,scratch:"),
 ("    pub fn reset(&mut self){self.buckets.copy_from_slice(&self.source);}","    pub fn reset(&mut self){self.buckets.copy_from_slice(&self.source);for (i,&p)in self.source.iter().enumerate(){self.native.set(i,p);}}"),
 ("        if borrowed {","        self.active_native=cfg!(zopro_native_buckets)&&tree;\n        if borrowed {"),
 ("                #[cfg(zopro_denominator_tree)] if tree {apply::<G2Config,true>(&mut self.buckets,refs,&mut self.scratch);continue;}","                #[cfg(zopro_native_buckets)] if tree {apply_native::<G2Config,true>(&mut self.native,refs,&mut self.scratch);continue;}"),
 ("assert_eq!(self.buckets,expected);","assert_eq!(if self.active_native{self.native.points()}else{self.buckets.clone()},expected);"),
 ("for p in &self.buckets{","let points=if self.active_native{self.native.points()}else{self.buckets.clone()};for p in &points{")
 ]{assert_eq!(g2.matches(from).count(),1,"{from}");g2=g2.replacen(from,to,1);caller_edits.push(json!([from,to]));}
 g2.push_str("\nimpl G2Caller{pub fn state_payload_bytes(&self)->[usize;2]{[self.size*128,self.size*160]}}\n");
 fs::write(r("g1-caller.rs"),&g1).unwrap();fs::write(r("g2-caller.rs"),&g2).unwrap();
 let storepath=format!("{Z}/docs/benchmarks/2026-10-22-universal-native-bucket52/source/crates/zopro-bn254/src/bucket52.rs");let store=fs::read(&storepath).unwrap();fs::write(r("store.rs"),&store).unwrap();
 let cfg="#[cfg(all(feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
 let suffixes=BTreeMap::from([
 ("crates/zopro-bn254/src/lib.rs".to_owned(),format!("\n{}\n#[doc(hidden)]pub mod native_bucket_store;\n",cfg.replacen("feature=\"ifma8\",","",1))),
 ("crates/zopro-core/src/msm.rs".to_owned(),format!("\n{cfg}\npub mod native_buckets;\n")),
 ("crates/zopro-core/src/lib.rs".to_owned(),format!("\n{cfg}\n#[doc(hidden)]pub use msm::native_buckets::{{G1Caller,G2Caller}};\n")),
 ("crates/zopro-core/src/simd.rs".to_owned(),format!("\n{cfg}\nimpl FqBatch{{#[inline(always)]pub(crate) fn native_gather(s:&zopro_bn254::native_bucket_store::Store,ids:[u32;8])->Self{{Self(s.gather(ids))}}#[inline(always)]pub(crate) fn native_scatter(&self,s:&mut zopro_bn254::native_bucket_store::Store,ids:[u32;8]){{s.scatter(ids,&self.0,255);}}}}\n"))
 ]);
 for(f,h)in originals{let bytes=fs::read(format!("{Z}/{f}")).unwrap();assert_eq!(hash(&bytes),h.as_str().unwrap());let p=r("source").join(f);fs::create_dir_all(p.parent().unwrap()).unwrap();let mut bytes=bytes;if let Some(s)=suffixes.get(f){bytes.extend_from_slice(s.as_bytes());}fs::write(p,bytes).unwrap();}
 fs::write(r("source/crates/zopro-bn254/src/native_bucket_store.rs"),&store).unwrap();
 let mut child=String::from("use super::*;use zopro_bn254::{G1Affine,G1Config,G1Projective,G2Affine,G2Config,G2Projective};\n");for p in ["native-state.rs","native-apply.rs","g1-caller.rs","g2-caller.rs","tests.rs"]{child.push_str(&fs::read_to_string(r(p)).unwrap());child.push('\n');}fs::write(r("source/crates/zopro-core/src/msm/native_buckets.rs"),child).unwrap();
 save(r("source-audit.json"),&json!({"accepted_originals":176,"suffixes":suffixes,"apply_clone":{"original":original,"candidate":candidate,"edits":edits,"inverse_recovered":true},"caller_edits":caller_edits,"store_original":storepath,"store_sha256":hash(&store),"field_arithmetic_unchanged":true}));
 let cargo=format!("[package]\nname=\"zopro-native-buckets\"\nversion=\"0.0.0\"\nedition=\"2021\"\npublish=false\n[workspace]\n[dependencies]\nzopro-core={{path=\"../source/crates/zopro-core\"}}\nzopro-bn254={{path=\"../source/crates/zopro-bn254\"}}\nserde_json=\"=1.0.149\"\nsha2=\"=0.10.9\"\nrayon=\"1.10\"\nlibc=\"0.2\"\n[profile.release]\nopt-level=3\nlto=true\ncodegen-units=1\n");
 fs::create_dir_all(r("driver/src")).unwrap();fs::write(r("driver/Cargo.toml"),cargo).unwrap();fs::copy("/home/ubuntu/src/zopro-perf-work/g2-denominator-tree176/driver.rs",r("driver/src/main.rs")).unwrap();fs::copy(format!("{Z}/crates/zopro-bench/examples/support/perf.rs"),r("driver/src/perf.rs")).unwrap();
 let mut env:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/environment.json").unwrap()).unwrap();env["RUSTFLAGS"]=json!(format!("{} --cfg zopro_native_buckets",manifest["rustflags"].as_str().unwrap()));env["CARGO_TARGET_DIR"]=json!(r("target"));env["RAYON_NUM_THREADS"]=json!("4");save(r("environment.json"),&env);
 println!("Prepared178source/172exactoriginals/4appendonlyparents/all176recover; oldStoreexact/currentG2arithmeticinverseaudit.");
}
