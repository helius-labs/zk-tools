use std::{fs,path::Path,collections::BTreeMap};use serde_json::{json,Value};use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-borrowed-g1";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);
 let mut head=fs::read_to_string("/tmp/zopro-borrowed-pending/experiment-head.rs").unwrap().replace("/tmp/zopro-borrowed-pending",R)
  .replace("6850","6886").replace("112","114").replace("\"off\"","\"owned\"").replace("\"on\"","\"all\"");
 head=head.replace("\"/tmp/zopro-pairing-clean/proof-inventory.json\"]","\"/tmp/zopro-pairing-clean/proof-inventory.json\",\"/tmp/zopro-borrowed-pending/proof-inventory.json\"]");
 let start=head.find(" c[\"variants\"]=json!([").unwrap();let end=start+head[start..].find(" c[\"pins\"]").unwrap();
 let variants=r#" c["variants"]=json!([
 {"name":"accepted","binary":"/tmp/zopro-grouping-module/native/candidate-sdk","environment":{"ZOPRO_CPU_BORROWED_PENDING":"0","ZOPRO_CPU_BORROWED_ORDINARY":"0"}},
 {"name":"owned","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"0","ZOPRO_CPU_BORROWED_ORDINARY":"0"}},
 {"name":"kh","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"1","ZOPRO_CPU_BORROWED_ORDINARY":"0"}},
 {"name":"all","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"1","ZOPRO_CPU_BORROWED_ORDINARY":"1"}}]);
"#;
 head.replace_range(start..end,variants);
 let needle="env[\"ZOPRO_CPU_BORROWED_PENDING\"]=json!(if variant==\"all\"{\"1\"}else{\"0\"});";
 assert!(head.contains(needle));head=head.replace(needle,&format!("{needle}env[\"ZOPRO_CPU_BORROWED_ORDINARY\"]=json!(if variant==\"all\"{{\"1\"}}else{{\"0\"}});"));
 let parent=fs::read_to_string("/tmp/zopro-borrowed-pending/experiment.rs").unwrap();let start=parent.find("fn qualify()").unwrap();let end=parent.find("fn metrics(").unwrap();
 let qualify=parent[start..end].replace("6868","6904").replace("6886","6928");
 let eval=fs::read_to_string(r.join("evaluate.rs")).unwrap();
 let main="fn main(){match std::env::args().nth(1).as_deref(){Some(\"prepare\")=>prepare(),Some(\"qualify\")=>qualify(),Some(\"screen\")=>evaluate(\"screen\"),Some(\"full\")=>evaluate(\"performance\"),_=>panic!(\"mode\")}}\n";
 fs::write(r.join("experiment.rs"),format!("{head}\n{qualify}\n{eval}\n{main}")).unwrap();
 let m=read("/tmp/zopro-grouping-module/production-source-manifest.json");let mut pins=BTreeMap::new();let mut changed=Vec::new();
 for(rel,h)in m["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel)),h.as_str().unwrap());let p=r.join("candidate").join(rel);let actual=hash(&p);if actual!=h.as_str().unwrap(){changed.push(rel.clone());}pins.insert(p.display().to_string(),actual);}
 assert_eq!(changed,vec!["crates/zopro-core/src/msm.rs","crates/zopro-core/src/msm_affine.rs","crates/zopro-core/src/msm_affine/fused_kh.rs"]);
 for rel in ["candidate/crates/zopro-core/src/msm_affine/borrowed_pending.rs","candidate/crates/zopro-bench/examples/cpu-sdk.rs","candidate/crates/zopro-bench/examples/support/perf.rs","adapt.rs","ordinary.rs","setup.rs","evaluate.rs","experiment.rs","design.md"]{let p=r.join(rel);pins.insert(p.display().to_string(),hash(p));}
 save(r.join("source-pins.json"),&json!(pins));save(r.join("implementation-audit.json"),&json!({"changed_originals":changed,"new_module":"msm_affine/borrowed_pending.rs","production_unchanged":170,"new_unsafe_code":false,"new_lookup_table":false,"ordinary_hook":"default-none BatchInverse; Fq native override","candidate_modes":["owned","kh","all"],"former_conservative_forecast_unchanged":true,"grouped_point_lifetimes_synchronous":true,"original_owned_geometry_retained":true}));
 let text=serde_json::to_string(&read("/tmp/zopro-borrowed-pending/native-plan.json")).unwrap().replace("/tmp/zopro-borrowed-pending",R);let mut plan:Value=serde_json::from_str(&text).unwrap();plan["environment"]["ZOPRO_CPU_BORROWED_ORDINARY"]=json!("1");
 for step in plan["steps"].as_array_mut().unwrap(){if step["name"]=="core"{let tests=step["required_tests"].as_array_mut().unwrap();tests.push(json!("msm_affine::borrowed_pending::tests::ordinary_queries_generator_oracle_and_owned_control"));tests.push(json!("msm_affine::borrowed_pending::tests::ordinary_trait_dispatch_and_fallback"));}}
 save(r.join("native-plan.json"),&plan);println!("Ordinary G1 extension pinned; native plan ready;4 SDK variants declared");
}
