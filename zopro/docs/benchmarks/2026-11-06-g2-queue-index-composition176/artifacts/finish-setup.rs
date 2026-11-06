use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-queue-index-composition176";
const P:&str="/home/ubuntu/src/zopro-perf-work/g2-contained-index176";
const Q:&str="/home/ubuntu/src/zopro-perf-work/queue-drain175";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn load(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn walk(p:&Path,pins:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let p=e.unwrap().path();if p.is_dir(){walk(&p,pins)}else{pins.insert(p.display().to_string(),hash(&fs::read(p).unwrap()));}}}
fn edit(edits:&mut Vec<Value>,p:&str,old:&str,new:&str){let path=r("source").join(p);let s=fs::read_to_string(&path).unwrap();assert_eq!(s.matches(old).count(),1,"{p}:{old}");fs::write(path,s.replacen(old,new,1)).unwrap();edits.push(json!({"file":p,"old":old,"new":new}));}
fn append(edits:&mut Vec<Value>,p:&str,suffix:&str){let path=r("source").join(p);let s=fs::read_to_string(&path).unwrap();fs::write(path,format!("{s}{suffix}")).unwrap();edits.push(json!({"file":p,"suffix":suffix}));}
fn main(){
 assert!(r("source").exists());assert!(!r("driver").exists());
 let manifest=load(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
 let parent_audit=load(format!("{P}/source-audit.json"));
 assert_eq!(parent_audit["all176_originals_recovered"],true);
 let parent_pins=load(format!("{P}/build-pins.json"));
 let mut parent_files=BTreeMap::new();walk(&Path::new(P).join("source"),&mut parent_files);assert_eq!(parent_files.len(),178);
 for(path,h)in &parent_files{assert_eq!(parent_pins[path].as_str().unwrap(),h);}
 let mut edits=vec![];
 let suffix=fs::read_to_string(format!("{Q}/g2-suffix.rs")).unwrap().replace("cfg!(zopro_queue_drain)","(cfg!(zopro_queue_drain)&&crate::queue_index_composition::drain_enabled())");
 edits.push(json!({"file":"crates/zopro-core/src/msm_affine/g2_borrowed.rs","old":"    fn flush_queue(&mut self) {","new":"    fn flush_queue(&mut self) {#[cfg(zopro_queue_drain)]if crate::queue_index_composition::drain_enabled(){self.drain_queued_batches();}"}));
 edits.push(json!({"file":"crates/zopro-core/src/msm_affine/g2_borrowed.rs","suffix":suffix}));
 edits.push(json!({"file":"crates/zopro-core/src/msm_affine.rs","suffix":"\n#[cfg(any(test,zopro_queue_drain))]mod queue_drain;\n"}));
 edits.push(json!({"file":"crates/zopro-core/src/lib.rs","suffix":"\n#[cfg(zopro_queue_index_composition)]pub mod queue_index_composition;\n"}));
 assert_eq!(fs::read(format!("{Q}/queue_drain.rs")).unwrap(),fs::read(r("source/crates/zopro-core/src/msm_affine/queue_drain.rs")).unwrap());
 let expected_selector=r#"//! Independent process selectors for the diagnostic composition.
use std::sync::OnceLock;
static DRAIN:OnceLock<bool>=OnceLock::new();
static PACK:OnceLock<bool>=OnceLock::new();
#[inline]pub fn drain_enabled()->bool{*DRAIN.get_or_init(||std::env::var("ZOPRO_CPU_QUERY_DRAIN").as_deref()==Ok("1"))}
#[inline]pub fn pack_enabled()->bool{*PACK.get_or_init(||std::env::var("ZOPRO_CPU_QUERY_PACK").as_deref()==Ok("1"))}
"#;
 assert_eq!(fs::read_to_string(r("source/crates/zopro-core/src/queue_index_composition.rs")).unwrap(),expected_selector);
 let mut exact=0;
 for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){
   let original=fs::read(Path::new(Z).join(p)).unwrap();assert_eq!(hash(&original),h.as_str().unwrap());
   let b=fs::read(r("source").join(p)).unwrap();if hash(&b)==h.as_str().unwrap(){exact+=1;}
   if !edits.iter().any(|e|e["file"]==p.as_str())&&parent_audit["suffixes"][p].is_null(){assert_eq!(hash(&b),h.as_str().unwrap());assert_eq!(hash(&b),parent_pins[Path::new(P).join("source").join(p).display().to_string()].as_str().unwrap());continue;}
   let mut s=String::from_utf8(b).unwrap();
   for e in edits.iter().rev().filter(|e|e["file"]==p.as_str()) {if let Some(t)=e["suffix"].as_str(){assert!(s.ends_with(t));s.truncate(s.len()-t.len());}else{let new=e["new"].as_str().unwrap();assert_eq!(s.matches(new).count(),1);s=s.replacen(new,e["old"].as_str().unwrap(),1);}}
   assert_eq!(hash(s.as_bytes()),parent_pins[Path::new(P).join("source").join(p).display().to_string()].as_str().unwrap());
   if let Some(t)=parent_audit["suffixes"][p].as_str(){assert!(s.ends_with(t));s.truncate(s.len()-t.len());}
   assert_eq!(hash(s.as_bytes()),h.as_str().unwrap(),"production restore {p}");
 }
 assert_eq!(exact,170);
 let parent_driver=fs::read_to_string(format!("{P}/driver/src/main.rs")).unwrap();
 let mut driver=parent_driver.clone();let mut de=vec![];
 for(old,new)in [
  ("let query=&a[1];", "let query=&a[1];let pack=zopro_core::queue_index_composition::pack_enabled();let drain=zopro_core::queue_index_composition::drain_enabled();if query!=\"A\"{assert_eq!(query==\"M\",pack);}"),
  ("\"new_proofs\":0", "\"new_proofs\":0,\"composition\":{\"pack_enabled\":pack,\"drain_enabled\":drain}"),
 ]{assert_eq!(driver.matches(old).count(),1);driver=driver.replacen(old,new,1);de.push(json!({"old":old,"new":new}));}
 let mut inverse=driver.clone();for e in de.iter().rev(){inverse=inverse.replacen(e["new"].as_str().unwrap(),e["old"].as_str().unwrap(),1);}assert_eq!(inverse,parent_driver);
 fs::create_dir_all(r("driver/src")).unwrap();fs::write(r("driver/src/main.rs"),&driver).unwrap();fs::write(r("driver.rs"),driver).unwrap();
 fs::copy(format!("{P}/driver/src/perf_support.rs"),r("driver/src/perf_support.rs")).unwrap();
 let cargo=fs::read_to_string(format!("{P}/driver/Cargo.toml")).unwrap().replace("name=\"zopro-contained-index\"","name=\"zopro-queue-index-composition\"");fs::write(r("driver/Cargo.toml"),cargo).unwrap();
 fs::copy(format!("{P}/fixtures.json"),r("fixtures.json")).unwrap();
 let mut env=load(format!("{P}/environment.json"));env["RUSTFLAGS"]=json!(format!("{} --cfg zopro_queue_drain --cfg zopro_queue_index_composition",env["RUSTFLAGS"].as_str().unwrap()));env["CARGO_TARGET_DIR"]=json!(r("target"));env["ZOPRO_CPU_QUERY_DRAIN"]=json!("0");env["ZOPRO_CPU_QUERY_PACK"]=json!("0");
 fs::write(r("environment.json"),serde_json::to_vec_pretty(&env).unwrap()).unwrap();
 fs::write(r("source-audit.json"),serde_json::to_vec_pretty(&json!({"snapshot_files":180,"originals_exact":exact,"all176_originals_recovered":true,"parent178_exact_after_reversal":true,"parent_source_pins":parent_files,"edits":edits,"queue_method_exact_except_runtime_selector_test_expectations":true,"queue_eligibility_exact":true,"contained_children_exact":true,"field_and_G1_runtime_exact":true,"original_Window_fields_and_capacity_unchanged":true,"extra_runtime_heap_buffers":0,"new_native_tests":0,"reused_contained_tests":3,"reused_queue_tests":2,"production_changed":false})).unwrap()).unwrap();
 fs::write(r("driver-audit.json"),serde_json::to_vec_pretty(&json!({"parent":format!("{P}/driver/src/main.rs"),"parent_sha256":hash(parent_driver.as_bytes()),"edits":de,"inverse_recovery_exact":true,"same_executable_all_four_arms":true,"selector_read_before_pool_or_timing":true,"selected_configuration_reported_after_timer":true})).unwrap()).unwrap();
 let c="/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/cargo";let mf=r("source/Cargo.toml");
 let mut steps=vec![];for(drain,tag)in[("0","OFF"),("1","ON")]{let mut env=env.clone();env["ZOPRO_CPU_QUERY_DRAIN"]=json!(drain);steps.push(json!({"kind":"command","name":format!("core-drain-{tag}"),"argv":[c,"test","--locked","--offline","--release","--manifest-path",mf,"-p","zopro-core","--features","ifma8","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["msm_affine::queue_drain::tests::simd_width_pending_and_signed_destination_guards","msm_affine::g2_borrowed::queue_drain_tests::drain_and_fallback_match_independent_buckets","msm::contained_index::contained_tests::contained_complete_queries_groups_boundaries_and_explicit_budget_fallback"],"timeout_seconds":1800}));}
 steps.push(json!({"kind":"command","name":"driver-lock","argv":[c,"generate-lockfile","--offline","--manifest-path",r("driver/Cargo.toml")],"environment":env,"timeout_seconds":120}));
 steps.push(json!({"kind":"command","name":"driver-build","argv":[c,"build","--locked","--offline","--release","-vv","--message-format=json","--manifest-path",r("driver/Cargo.toml")],"environment":env,"timeout_seconds":1800}));
 let mut pins=BTreeMap::new();walk(&r("source"),&mut pins);assert_eq!(pins.len(),180);walk(&r("driver"),&mut pins);
 for f in ["setup.rs","finish-setup.rs","setup-first-failure.txt","design.md","history-audit.md","source-audit.json","driver-audit.json","driver.rs","environment.json","fixtures.json"]{pins.insert(r(f).display().to_string(),hash(&fs::read(r(f)).unwrap()));}
 for p in [format!("{P}/source-audit.json"),format!("{P}/build-pins.json"),format!("{P}/review.json"),format!("{Q}/g2-suffix.rs"),format!("{Q}/queue_drain.rs")]{pins.insert(p.clone(),hash(&fs::read(p).unwrap()));}
 fs::write(r("build-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(r("build-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps})).unwrap()).unwrap();
 println!("Prepared180source/170originalexact/all176recover;both queue modes run130core tests incl paired complete-query/pressure fallback/1,4,16,32 small-stackworkers;4native workflow commands;no capture.");
}
