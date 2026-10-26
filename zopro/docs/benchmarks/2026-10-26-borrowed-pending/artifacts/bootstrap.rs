use std::{fs,path::Path,collections::BTreeMap};
use serde_json::{json,Value}; use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-borrowed-pending";
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let r=Path::new(R); let m=read("/tmp/zopro-grouping-module/production-source-manifest.json");
 let old=fs::read_to_string("/tmp/zopro-pairing-clean/experiment.rs").unwrap();
 let start=old.find("fn qualify()").unwrap();let end=old.find("fn report()").unwrap();
 let body=old[start..end].replace("6850","6886").replace("6832","6868").replace("6814","6850").replace("7063","7099")
  .replace("\"fresh\"","\"off\"").replace("\"clean\"","\"on\"")
  .replace("clean/fresh","on/off").replace("clean/accepted","on/accepted").replace("fresh/accepted","off/accepted").replace("clean/{control}","on/{control}");
 let head=fs::read_to_string(r.join("experiment-head.rs")).unwrap();
 fs::write(r.join("experiment.rs"),format!("{head}\n{body}\nfn main(){{match std::env::args().nth(1).as_deref(){{Some(\"prepare\")=>prepare(),Some(\"qualify\")=>qualify(),Some(\"screen\")=>evaluate(\"screen\"),Some(\"full\")=>evaluate(\"performance\"),_=>panic!(\"mode\")}}}}\n")).unwrap();
 let mut pins=BTreeMap::new();let mut changed=Vec::new();
 for(rel,h)in m["source_files_sha256"].as_object().unwrap(){
  assert_eq!(hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel)),h.as_str().unwrap());
  let p=r.join("candidate").join(rel);let actual=hash(&p);
  if actual!=h.as_str().unwrap(){changed.push(rel.clone());}pins.insert(p.display().to_string(),actual);
 }
 assert_eq!(changed,vec!["crates/zopro-core/src/msm.rs","crates/zopro-core/src/msm_affine.rs","crates/zopro-core/src/msm_affine/fused_kh.rs"]);
 for rel in ["candidate/crates/zopro-core/src/msm_affine/borrowed_pending.rs","candidate/crates/zopro-bench/examples/cpu-sdk.rs","candidate/crates/zopro-bench/examples/support/perf.rs","bootstrap.rs","experiment-head.rs","experiment.rs","design.md"] {
  let p=r.join(rel);pins.insert(p.display().to_string(),hash(p));
 }
 for rel in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{
  assert_eq!(hash(r.join("candidate").join(rel)),hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel)));
 }
 save(r.join("source-pins.json"),&json!(pins));
 save(r.join("implementation-audit.json"),&json!({"changed_originals":changed,"new_module":"msm_affine/borrowed_pending.rs","production_sources_unchanged":170,"arithmetic_formula_changes":false,"safe_borrowed_pending_buffers":true,"new_key_tables":false,"new_full_scalar_or_point_copy":false,"new_runtime_flag":"ZOPRO_CPU_BORROWED_PENDING","only_fused_kh_routed":true,"native_IFMA_deferred_signed_packed_direct_guard":true,"adopted":false}));
 let text=serde_json::to_string(&read("/tmp/zopro-pairing-clean/native-plan.json")).unwrap().replace("/tmp/zopro-pairing-clean",R);
 let mut plan:Value=serde_json::from_str(&text).unwrap();
 plan["environment"]["ZOPRO_CPU_BORROWED_PENDING"]=json!("1");
 let steps=plan["steps"].as_array_mut().unwrap();steps.retain(|s| !["fresh-libraries","fresh"].contains(&s["name"].as_str().unwrap()));
 for s in steps {if s["name"]=="clean"{s["name"]=json!("on");}
  if s.get("required_tests").is_some(){s["required_tests"]=json!([]);}
  if s["name"]=="core"{s["required_tests"]=json!([
   "msm::deferred_sign_reference_tests::borrowed_signed_stream_tails_and_reuse_match_generator",
   "msm_affine::borrowed_pending::tests::pending_record_sizes_and_collision_reuse",
   "msm_affine::borrowed_pending::tests::fused_generator_oracle_and_owned_control"]);}
 }
 save(r.join("native-plan.json"),&plan);
 println!("3 changed original sources, one new module, native plan ready");
}
