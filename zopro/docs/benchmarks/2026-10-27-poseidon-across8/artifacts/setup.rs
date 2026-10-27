use std::{fs,path::Path,collections::BTreeMap};use serde_json::{Value,json};use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-poseidon-across8";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);let m=read("/tmp/zopro-grouping-module/production-source-manifest.json");let mut pins=BTreeMap::new();let mut changed=Vec::new();let mut removed=0;
 for(rel,h)in m["source_files_sha256"].as_object().unwrap(){let orig=Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel);assert_eq!(hash(&orig),h.as_str().unwrap());let candidate=r.join("candidate").join(rel);let actual=hash(&candidate);if actual!=h.as_str().unwrap(){changed.push(rel.clone());let new=fs::read_to_string(&candidate).unwrap();let recovered:String=new.split_inclusive('\n').filter(|line|{let drop=line.starts_with("#[cfg(all(zopro_resource_schedule,zopro_poseidon_across8,");if drop{removed+=1;}!drop}).collect();assert_eq!(recovered,fs::read_to_string(orig).unwrap());}pins.insert(candidate.display().to_string(),actual);}
 assert_eq!(changed,vec!["crates/zopro-core/src/lib.rs","crates/zopro-core/src/solver.rs","crates/zopro-core/src/solver/sparse_poseidon.rs","crates/zopro-core/src/solver/sparse_poseidon/tests.rs"]);assert_eq!(removed,5);
 for rel in ["candidate/crates/zopro-core/src/solver/sparse_poseidon/across8.rs","candidate/crates/zopro-core/src/solver/sparse_poseidon/component.rs","candidate/crates/zopro-core/src/solver/sparse_poseidon/tests/across_refs.rs","probe.rs","support/perf.rs","setup.rs","design.md","prebuild-note.md"]{let p=r.join(rel);pins.insert(p.display().to_string(),hash(p));}
 assert_eq!(hash(r.join("support/perf.rs")),hash("/home/ubuntu/src/zk-tools/zopro/crates/zopro-bench/examples/support/perf.rs"));save(r.join("source-pins.json"),&json!(pins));save(r.join("source-audit.json"),&json!({"original_files":170,"unchanged_originals":166,"changed_originals":changed,"all_original_bytes_recovered_after_removing5_component_cfg_lines":true,"production_dispatch_unchanged":true,"field_source_unchanged":true,"original_scalar_bodies_unchanged":true,"new_modules":["across8.rs","component.rs","tests/across_refs.rs"],"production_changed":false,"new_key_table_or_execution_plan":false}));
 let text=serde_json::to_string(&read("/tmp/zopro-poseidon-frontier/native-plan.json")).unwrap().replace("/tmp/zopro-poseidon-frontier",R).replace("zopro_poseidon_census","zopro_poseidon_across8");let mut plan:Value=serde_json::from_str(&text).unwrap();
 for step in plan["steps"].as_array_mut().unwrap(){if step["name"]=="core"{step["required_tests"]=json!([
 "solver::sparse_poseidon::tests::across_refs::complete_combined_layout_matches_original_fixture",
 "solver::sparse_poseidon::tests::across_refs::eight_lanes_all_wires_rows_prefixes_and_ragged_chains",
 "solver::sparse_poseidon::tests::across_refs::untagged_and_unsupported_width_fall_back"]);}if step["name"]=="on"{step["name"]=json!("probe");step["driver"]=json!(r.join("probe.rs"));step["additional_sources"]=json!([r.join("support/perf.rs")]);}}
 save(r.join("native-plan.json"),&plan);println!("Across8 complete-write component frozen; native plan ready; noSDK/Gnark expected");
}
