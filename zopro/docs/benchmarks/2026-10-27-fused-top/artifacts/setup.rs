use std::{fs,path::Path};use serde_json::{Value,json};use sha2::{Digest,Sha256};
const R:&str="/tmp/zopro-fused-top";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){
 let r=Path::new(R);let base=Path::new("/tmp/zopro-poseidon-fused-dot/baseline");
 let manifest=read("/tmp/zopro-grouping-module/production-source-manifest.json");
 for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(base.join(p)),h.as_str().unwrap());assert_eq!(hash(r.join("candidate").join(p)),h.as_str().unwrap());}
 let mut edits=Vec::new();
 let rel="crates/zopro-core/src/msm.rs";let p=r.join("candidate").join(rel);let mut s=fs::read_to_string(&p).unwrap();
 let start=s.find("pub(super) fn reduce_indexed_buckets<C: CurveConfig>(").unwrap();let end=s[start..].find("/// Reduces already accumulated").unwrap()+start;let old=s[start..end].to_string();
 let mut new=old.clone();let pivot=new.find("    let tile =").unwrap();
 new.insert_str(pivot,"    reduce_indexed_buckets_by(|index| bases[index as usize], order, starts)\n}\n\n/// Reduces indexed points supplied by a borrowed virtual input.\npub(super) fn reduce_indexed_buckets_by<C: CurveConfig, F>(\n    get: F, order: &[u32], starts: &[usize],\n) -> Vec<Affine<C>>\nwhere C::Base: BatchInverse, F: Fn(u32) -> Affine<C> + Sync,\n{\n");
 assert_eq!(new.matches("limbs::<C>(bases[i as usize])").count(),1);new=new.replace("limbs::<C>(bases[i as usize])","limbs::<C>(get(i))");
 s.replace_range(start..end,&new);fs::write(&p,s).unwrap();edits.push(json!({"file":rel,"old":old,"new":new}));
 let rel="crates/zopro-core/src/msm_affine/fused_kh.rs";let p=r.join("candidate").join(rel);let mut s=fs::read_to_string(&p).unwrap();
 let old="   let(a,b)=rayon::join(||top_window::<G1Config,true>(k,sk,w,bits),||top_window::<G1Config,true>(h,sh,w,bits));\n   if let(Some(a),Some(b))=(a,b){return a+b;}";
 let new="   if let Some(sum)=top(k,sk,h,sh,w,bits){return sum;}";
 assert_eq!(s.matches(old).count(),1);s=s.replace(old,new);edits.push(json!({"file":rel,"old":old,"new":new}));
 let old="// Only internal rerandomized solving consumes K+H together.";let new=format!("{}\n{old}",fs::read_to_string(r.join("top.rs")).unwrap());assert_eq!(s.matches(old).count(),1);s=s.replace(old,&new);edits.push(json!({"file":rel,"old":old,"new":new}));fs::write(p,s).unwrap();save(r.join("edits.json"),&json!(edits));
 let mut changed=Vec::new();for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){let file=r.join("candidate").join(p);if hash(&file)!=h.as_str().unwrap(){changed.push(p);let mut s=fs::read_to_string(file).unwrap();for e in edits.iter().rev().filter(|e|e["file"]==*p){let new=e["new"].as_str().unwrap();assert_eq!(s.matches(new).count(),1);s=s.replace(new,e["old"].as_str().unwrap());}assert_eq!(s,fs::read_to_string(base.join(p)).unwrap());}}
 assert_eq!(changed,vec!["crates/zopro-core/src/msm.rs","crates/zopro-core/src/msm_affine/fused_kh.rs"]);
 save(r.join("implementation-audit.json"),&json!({"original_files":170,"unchanged_candidate_originals":168,"changed":changed,"original_bytes_recovered":true,"new_module":false,"field_sources_unchanged":true,"new_arithmetic_primitive":false,"new_table_or_plan":false,"new_pool_or_scheduler":false,"production_changed":false,"combined_index_order_instead_of_two":true,"borrowed_points_not_copied":true,"fresh_control_reused_unchanged":true}));
 let mut plan:Value=serde_json::from_str(&serde_json::to_string(&read("/tmp/zopro-poseidon-fused-dot/native-plan.json")).unwrap().replace("/tmp/zopro-poseidon-fused-dot",R)).unwrap();let steps=plan["steps"].as_array_mut().unwrap();steps.retain(|x|!["fresh","fresh-libraries"].contains(&x["name"].as_str().unwrap()));for x in steps {if x["name"]=="core"{x["required_tests"]=json!(["msm_affine::fused_kh::top_tests::combined_top_generator_oracle_and_fallbacks","msm_affine::fused_kh::tests::combined_queries_generator_oracle"]);}if x["name"]=="fused"{x["name"]=json!("combined");}}
 save(r.join("native-plan.json"),&plan);
 let mut e=fs::read_to_string("/tmp/zopro-poseidon-direct-dot/experiment.rs").unwrap().replace("/tmp/zopro-poseidon-direct-dot",R).replace("direct","combined");
 let a=e.find("fn setup()").unwrap();let b=e.find("fn prepare()").unwrap();
 e.replace_range(a..b,r#"fn setup(){let r=Path::new(R);let manifest=read("/tmp/zopro-grouping-module/production-source-manifest.json");let mut pins=BTreeMap::new();for(rel,h)in manifest["source_files_sha256"].as_object().unwrap(){let original=Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel);assert_eq!(hash(&original),h.as_str().unwrap());add(&original,&mut pins);let baseline=Path::new("/tmp/zopro-poseidon-fused-dot/baseline").join(rel);assert_eq!(hash(&baseline),h.as_str().unwrap());add(baseline,&mut pins);add(r.join("candidate").join(rel),&mut pins);}let a=read(r.join("implementation-audit.json"));assert_eq!(a["original_bytes_recovered"],true);let control="/tmp/zopro-poseidon-fused-dot/native/fresh/artifacts.json";let receipt=read(control);assert_eq!(hash(receipt["binary"].as_str().unwrap()),receipt["binary_sha256"].as_str().unwrap());for(p,h)in receipt["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}add(control,&mut pins);add(receipt["binary"].as_str().unwrap(),&mut pins);save(r.join("fresh-control-artifacts.json"),&receipt);add(r.join("fresh-control-artifacts.json"),&mut pins);for rel in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{let p=r.join("candidate").join(rel);assert_eq!(hash(&p),hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel)));add(p,&mut pins);}for name in ["setup.rs","experiment.rs","design.md","top.rs","edits.json","implementation-audit.json","native-plan.json"]{add(r.join(name),&mut pins);}add(TOOL,&mut pins);add("/usr/bin/perf",&mut pins);save(r.join("source-pins.json"),&json!(pins));println!("Frozen combined K/H top,168originals exact; unchanged field; native110expected");}
"#);
 e=e.replace("\"/tmp/zopro-poseidon-fused-dot/proof-inventory.json\"]","\"/tmp/zopro-poseidon-fused-dot/proof-inventory.json\",\"/tmp/zopro-poseidon-direct-dot/proof-inventory.json\",\"/tmp/zopro-sdk-paired-control/proof-inventory.json\",\"/tmp/zopro-arithmetic-leaves-corrected/combined-proof-inventory.json\"]");
 e=e.replace("6982","7233").replace("6991","7242").replace("7009","7260").replace("7222","7473");
 e=e.replace("        109\n","        110\n").replace("    add(r.join(\"codegen-audit.json\"),&mut pins);\n","");
 assert!(!e.contains("codegen-audit"));save(r.join("adapt-audit.json"),&json!({"protocol_source":"/tmp/zopro-poseidon-direct-dot/experiment.rs","original_protocol_sha256":hash("/tmp/zopro-poseidon-direct-dot/experiment.rs"),"prior":7233,"preflight_next":7242,"screen_next":7260,"full_next":7473}));
 fs::write(r.join("experiment.rs"),e).unwrap();println!("Two-source combined-top overlay and native/experiment plans generated");
}
