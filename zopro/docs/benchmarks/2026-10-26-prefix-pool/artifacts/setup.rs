use std::{fs,path::Path,collections::BTreeMap};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-prefix-pool";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let mut pipeline=fs::read_to_string("/tmp/zopro-isolated-grouping/pipeline.rs").unwrap()
     .replace("/tmp/zopro-isolated-grouping",R)
     .replace("codegen-review.json","implementation-audit.json")
     .replace("original_grouping_structure_preserved","verified")
     .replace(".len(),115)",".len(),112)")
     .replace("/tmp/zopro-g2-owned-finish/perf-plan.json","/tmp/zopro-identical-control/corrected-run/perf-plan.json")
     .replace("ZOPRO_CPU_GROUPING_LOWER_BOUND","ZOPRO_CPU_PREFIX_POOL")
     .replace("lower_bound","prefix_pool")
     .replace("\"audit-current.rs\",\"audit-current/results.json\",", "")
     .replace("\"prepare-pipeline.rs\",\"pipeline-adaptation.json\",", "\"setup.rs\",\"closeout.rs\",\"closeout\",\"design.md\",")
     .replace("5612","5826").replace("5630","5844").replace("5714","5928");
 let start=pipeline.find(" let old=read(").unwrap();let end=pipeline[start..].find("\n let cases:").unwrap()+start;
 pipeline.replace_range(start..end," let old=read(\"/tmp/zopro-identical-control/corrected-run/perf-plan.json\");let mut prior:BTreeSet<String>=serde_json::from_value(old[\"prior_proof_sha256\"].clone()).unwrap();assert_eq!(prior.len(),5770);for(p,h)in read(\"/tmp/zopro-identical-control/corrected-run/proof-inventory.json\").as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());if p.ends_with(\".proof\"){assert!(prior.insert(h.as_str().unwrap().into()));}}assert_eq!(prior.len(),5826);");
 pipeline=pipeline.replace("env[\"ZOPRO_CPU_PREFIX_POOL\"]=json!(if mode==\"on\"{\"1\"}else{\"0\"});", "env[\"ZOPRO_CPU_PREFIX_POOL\"]=json!(if mode==\"on\"{\"1\"}else{\"0\"});env[\"ZOPRO_PREFIX_POOL_TRACE\"]=json!(\"1\");");
 fs::write(Path::new(R).join("pipeline.rs"),pipeline).unwrap();
 let r=Path::new(R);let src=r.join("candidate");let manifest=read("/tmp/zopro-grouping-module/production-source-manifest.json");let mut pins=BTreeMap::new();let mut changed=Vec::new();for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){let path=src.join(p);let actual=hash(&path);if actual!=h.as_str().unwrap(){changed.push(p.clone());}pins.insert(path.display().to_string(),actual);}assert_eq!(changed,vec!["crates/zopro-core/src/lib.rs","crates/zopro-core/src/msm_affine.rs"]);
 for rel in ["crates/zopro-core/src/prefix_pool.rs","crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{let p=src.join(rel);pins.insert(p.display().to_string(),hash(&p));}
 for rel in ["design.md","setup.rs","pipeline.rs","closeout.rs"]{let p=r.join(rel);pins.insert(p.display().to_string(),hash(&p));}
 save(r.join("source-pins.json"),&json!(pins));save(r.join("implementation-audit.json"),&json!({"verified":true,"accepted_manifest":hash("/tmp/zopro-grouping-module/production-source-manifest.json"),"original_sources":170,"unchanged_originals":168,"changed":changed,"new_core_module":"crates/zopro-core/src/prefix_pool.rs","production_changed":false}));
 let mut plan=read("/tmp/zopro-isolated-grouping/native-plan.json");let s=serde_json::to_string(&plan).unwrap().replace("/tmp/zopro-isolated-grouping",R).replace("--cfg zopro_isolated_grouping","--cfg zopro_prefix_pool");plan=serde_json::from_str(&s).unwrap();plan["environment"].as_object_mut().unwrap().remove("ZOPRO_CPU_GROUPING_LOWER_BOUND");plan["environment"]["ZOPRO_CPU_PREFIX_POOL"]=json!("1");
 for step in plan["steps"].as_array_mut().unwrap(){if step["name"]=="on-core"{step["required_tests"]=json!(["prefix_pool::tests::routing_preserves_low_core_custom_pool_and_concurrency_fallbacks","prefix_pool::tests::canonical_storage_moves_without_conversion_or_buffer_copy","prefix_pool::tests::proof_scope_can_cover_both_pools"]);}}
 save(r.join("native-plan.json"),&plan);
}
