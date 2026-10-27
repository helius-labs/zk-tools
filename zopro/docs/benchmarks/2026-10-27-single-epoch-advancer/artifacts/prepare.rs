use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R:&str="/tmp/zopro-single-epoch-advancer";
const P:&str="/home/ubuntu/src/zk-tools/zopro";
const M:&str="/tmp/zopro-clean-fft-adoption/production-source-manifest.json";
const X:&str="/tmp/zopro-xtask-fast-screen-target/release/xtask";
const E:&str="/home/ubuntu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-epoch-0.9.21";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1)}
fn copy(a:&Path,b:&Path){fs::create_dir_all(b).unwrap();for e in fs::read_dir(a).unwrap(){let e=e.unwrap();let name=e.file_name();let s=name.to_string_lossy();if s=="target"||s==".git"||s==".cargo-ok"||s==".cargo-checksum.json"{continue}let p=e.path();if e.file_type().unwrap().is_dir(){copy(&p,&b.join(name))}else{fs::copy(p,b.join(name)).unwrap();}}}
fn pins(a:&Path,out:&mut BTreeMap<String,String>){for e in fs::read_dir(a).unwrap(){let e=e.unwrap();if e.file_type().unwrap().is_dir(){pins(&e.path(),out)}else{out.insert(e.path().display().to_string(),hash(e.path()));}}}
fn main(){
 let r=Path::new(R);assert!(!r.join("native-plan.json").exists());
 let m=read(M);let originals=m["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),172);
 for(f,h)in originals{assert_eq!(hash(Path::new(P).join(f)),h.as_str().unwrap());}
 let overlay=r.join("overlay");let epoch=overlay.join("vendor/crossbeam-epoch");copy(Path::new(E),&epoch);
 let mut cm=fs::read_to_string(epoch.join("Cargo.toml")).unwrap();cm.push_str("\n[workspace]\n");fs::write(epoch.join("Cargo.toml"),cm).unwrap();
 fs::copy("/tmp/zopro-empty-epoch-collection/workspace/vendor/crossbeam-epoch/Cargo.lock",epoch.join("Cargo.lock")).unwrap();
 fs::copy(r.join("advance_gate.rs"),epoch.join("src/advance_gate.rs")).unwrap();
 let f=epoch.join("src/internal.rs");let old=fs::read_to_string(&f).unwrap();let mut s=old.clone();
 replace(&mut s,"pub(crate) struct Global {","pub(crate) struct Global {\n    #[cfg(zopro_single_epoch_advancer)]\n    pub(crate) advance_gate: advance_gate::AdvanceGate,");
 replace(&mut s,"            locals: List::new(),","            #[cfg(zopro_single_epoch_advancer)]\n            advance_gate: advance_gate::AdvanceGate::configured(),\n            locals: List::new(),");
 let gate="        #[cfg(zopro_single_epoch_advancer)]\n        let _permit = if self.advance_gate.enabled {\n            match self.advance_gate.try_enter() {\n                Some(permit) => Some(permit),\n                None => {\n                    let current = self.epoch.load(if cfg!(crossbeam_sanitize_thread) { Ordering::Acquire } else { Ordering::Relaxed });\n                    atomic::fence(Ordering::SeqCst);\n                    return current;\n                }\n            }\n        } else { None };\n";
 replace(&mut s,"    pub(crate) fn try_advance(&self, guard: &Guard) -> Epoch {\n",&format!("    pub(crate) fn try_advance(&self, guard: &Guard) -> Epoch {{\n{gate}"));
 let mut recovered=s.clone();replace(&mut recovered,"pub(crate) struct Global {\n    #[cfg(zopro_single_epoch_advancer)]\n    pub(crate) advance_gate: advance_gate::AdvanceGate,","pub(crate) struct Global {");replace(&mut recovered,"            #[cfg(zopro_single_epoch_advancer)]\n            advance_gate: advance_gate::AdvanceGate::configured(),\n","");replace(&mut recovered,gate,"");assert_eq!(recovered,old);
 s.push_str("\n#[cfg(zopro_single_epoch_advancer)]\n#[path = \"advance_gate.rs\"]\npub(crate) mod advance_gate;\n");fs::write(&f,s).unwrap();
 let mut cargo=fs::read_to_string(Path::new(P).join("Cargo.toml")).unwrap();cargo.push_str("\n[patch.crates-io]\ncrossbeam-epoch = { path = \"vendor/crossbeam-epoch\" }\n");fs::write(overlay.join("Cargo.toml"),cargo).unwrap();
 let lock=fs::read_to_string(Path::new(P).join("Cargo.lock")).unwrap();let mut chunks:Vec<String>=lock.split("[[package]]").map(str::to_owned).collect();let mut changed=0;
 for c in &mut chunks{if c.starts_with("\nname = \"crossbeam-epoch\"\n"){let body=c.clone();*c=c.lines().filter(|l|!l.starts_with("source = ")&&!l.starts_with("checksum = ")).collect::<Vec<_>>().join("\n");c.push_str("\n\n");assert_ne!(c,&body);changed+=1;}}
 assert_eq!(changed,1);fs::write(overlay.join("Cargo.lock"),chunks.join("[[package]]")).unwrap();
 let output=Command::new(X).args(["prepare",M,P]).arg(r.join("candidate")).arg(&overlay).output().unwrap();fs::write(r.join("prepare.log"),output.stdout).unwrap();fs::write(r.join("prepare.stderr.log"),output.stderr).unwrap();assert!(output.status.success());
 let c=r.join("candidate");for(f,h)in originals{if f!="Cargo.toml"&&f!="Cargo.lock"{assert_eq!(hash(c.join(f)),h.as_str().unwrap());}}
 let mut source_pins=BTreeMap::new();pins(&c,&mut source_pins);
 for p in [r.join("design.md"),r.join("prepare.rs"),r.join("advance_gate.rs"),Path::new(M).to_owned(),Path::new(X).to_owned()]{source_pins.insert(p.display().to_string(),hash(p));}
 save(r.join("source-audit.json"),&json!({"production_changed":false,"production_originals":172,"candidate_originals_identical":170,"changed_originals":["Cargo.toml","Cargo.lock"],"lock_change":"only epoch registry source/checksum removed; all versions and runtime edges retained","epoch_version":"0.9.21","epoch_original_internal_sha256":hash(Path::new(E).join("src/internal.rs")),"original_internal_recovered_exactly_after_removing_insertions":true,"queue_pin_collection_expiration_arithmetic_unchanged":true,"private_dependency":true}));
 source_pins.insert(r.join("source-audit.json").display().to_string(),hash(r.join("source-audit.json")));save(r.join("source-pins.json"),&json!(source_pins));
 let mut plan=read("/tmp/zopro-clean-fft-leaves/native-plan.json");let flags=format!("{} --cfg zopro_single_epoch_advancer",m["rustflags"].as_str().or_else(||m["native_rustflags"].as_str()).unwrap());
 let mut steps=vec![json!({"kind":"command","name":"epoch-tests","argv":["cargo","test","--manifest-path",c.join("vendor/crossbeam-epoch/Cargo.toml"),"--offline","--locked","--release","--lib","--","--test-threads=1"],"environment":{"CARGO_TARGET_DIR":r.join("target-epoch-tests"),"RUSTFLAGS":flags,"ZOPRO_CPU_SINGLE_EPOCH_ADVANCER":"1"},"unfiltered":true,"required_tests":["internal::advance_gate::tests::nonblocking_permit_excludes_and_releases_on_unwind","internal::advance_gate::tests::held_permit_skips_advance_and_expired_collection_still_completes","internal::advance_gate::tests::pinned_reader_prevents_early_reclamation_then_releases","internal::advance_gate::tests::concurrent_retirements_are_eventually_reclaimed_once"],"timeout_seconds":1200}),json!({"kind":"command","name":"epoch-alloc-check","argv":["cargo","check","--manifest-path",c.join("vendor/crossbeam-epoch/Cargo.toml"),"--offline","--locked","--release","--no-default-features","--features","alloc","--lib"],"environment":{"CARGO_TARGET_DIR":r.join("target-epoch-tests"),"RUSTFLAGS":flags},"timeout_seconds":1200})];
 for step in plan["steps"].as_array().unwrap(){if !step["name"].as_str().unwrap().starts_with("candidate"){continue}let raw=serde_json::to_string(step).unwrap().replace("/tmp/zopro-clean-fft-leaves",R);let mut step:Value=serde_json::from_str(&raw).unwrap();if step["kind"]=="command"{step["environment"]["RUSTFLAGS"]=json!(flags);step["environment"]["ZOPRO_CPU_SINGLE_EPOCH_ADVANCER"]=json!("1");}steps.push(step);}
 plan["steps"]=json!(steps);plan["source_pins"]=json!(r.join("source-pins.json"));plan["environment"]["ZOPRO_CPU_SINGLE_EPOCH_ADVANCER"]=json!("1");save(r.join("native-plan.json"),&plan);
 println!("Prepared current172 epoch-only private candidate; original advancement body and runtime versions retained");
}
