use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/inverse-seed-scale176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
 let m:Value=serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();let sources=m["source_files_sha256"].as_object().unwrap();assert_eq!(sources.len(),176);
 let mut pins=BTreeMap::new();for(n,h)in sources{let p=format!("{Z}/{n}");assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p, h.as_str().unwrap().to_string());}
 let original=fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/inverse.rs")).unwrap();
 let changes=[
 ("    let (mut u, mut v) = (C::R2, [0; 4]);", "    let (mut u, mut v) = (const { portable::power_of_two(576, C::P) }, [0; 4]);"),
 ("        return None;", "        #[cfg(zopro_inverse_seed_observe)] crate::observe::hit(0);\n        return None;"),
 ("    let result = Fp::from_montgomery(v);", "    #[cfg(zopro_inverse_seed_observe)] crate::observe::hit(3);\n    let result = Fp::from_montgomery(v) * Fp::from_montgomery(const { portable::power_of_two(192, C::P) });"),
 ("        return Fp::from_montgomery(v);", "        #[cfg(zopro_inverse_seed_observe)] crate::observe::hit(2);\n        return Fp::from_montgomery(v) * Fp::from_montgomery(const { portable::power_of_two(192, C::P) });"),
 ("    let scale = Fp::<C>::from_montgomery(const { portable::power_of_two(320, C::P) });\n    Fp::from_montgomery(result) * scale", "    #[cfg(zopro_inverse_seed_observe)] crate::observe::hit(1);\n    Fp::from_montgomery(result)")];
 let mut candidate=original.clone();for(old,new)in changes{assert_eq!(candidate.matches(old).count(),1,"{old}");candidate=candidate.replace(old,new);}
 let suffix="\npub(crate) fn probe_finish<C:FpConfig>(a:u64,b:u64,u:[u64;4],v:[u64;4])->Fp<C>{finish::<C>(a,b,u,v)}\n";
 candidate.push_str(suffix);let mut recovered=candidate.strip_suffix(suffix).unwrap().to_owned();for(old,new)in changes.into_iter().rev(){assert_eq!(recovered.matches(new).count(),1);recovered=recovered.replace(new,old);}assert_eq!(recovered,original);
 let dest=format!("{R}/candidate.rs");assert!(!Path::new(&dest).exists());fs::write(&dest,candidate).unwrap();pins.insert(dest.clone(),hash(&dest));
 let audit=json!({"production_sources_checked":176,"production_unchanged":true,"original_inverse_sha256":hash(format!("{Z}/crates/zopro-bn254/src/inverse.rs")),"candidate_sha256":hash(&dest),"inverse_recovered_exact":true,"changes":changes,"probe_suffix":suffix,"no_new_production_allocation":true});fs::write(format!("{R}/source-audit.json"),serde_json::to_vec_pretty(&audit).unwrap()).unwrap();
 let old:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/euclid-float-quotient176/relink-plan.json").unwrap()).unwrap();let mut build=old["steps"][0].clone();build["name"]="build-new-seed-scale-qualification".into();let args=build["argv"].as_array_mut().unwrap();for a in args.iter_mut(){let s=a.as_str().unwrap();if s.ends_with("/euclid-float-quotient176/driver2.rs"){*a=format!("{R}/qualify.rs").into();}else if s.ends_with("/euclid-float-quotient176/native/frozen-consumer"){*a=format!("{R}/qualify").into();}}
 args.extend(["--cfg".into(),"zopro_inverse_seed_observe".into(),"-C".into(),"overflow-checks=yes".into(),"-C".into(),"debug-assertions=yes".into()]);
 for a in args.iter(){let s=a.as_str().unwrap();if let Some((_,p))=s.split_once('='){if p.ends_with(".rlib"){pins.insert(p.to_owned(),hash(p));}}}
 let exe=args[0].as_str().unwrap();pins.insert(exe.to_string(),hash(exe));
 for n in ["design.md","prepare.rs","qualify.rs","source-audit.json"]{let p=format!("{R}/{n}");pins.insert(p.clone(),hash(&p));}pins.insert(manifest.clone(),hash(&manifest));
 fs::write(format!("{R}/pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();let plan=json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":[build,{"kind":"command","name":"qualify-new-seed-scale-invariant","argv":[format!("{R}/qualify")],"timeout_seconds":300}]});fs::write(format!("{R}/plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();println!("Prepared exact inverse transformation,176 accepted hashes, new qualification plan; no clocks/proofs");
}
