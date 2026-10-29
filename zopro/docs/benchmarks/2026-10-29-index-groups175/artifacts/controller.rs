use serde_json::{json,Value};
use sha2::{Sha256,Digest};
use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/index-groups175";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const P:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-production";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn setup(){
assert!(!r("plan.json").exists());let manifest=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));let frozen=read(format!("{P}/frozen-candidate.json"));let mut pins=BTreeMap::<String,String>::new();
for(f,h)in manifest["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p.display().to_string(),h.as_str().unwrap().into());}
for(f,h)in frozen["input_pins"].as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());pins.insert(f.clone(),h.as_str().unwrap().into());}
let source=fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm_affine/grouping.rs")).unwrap();
let start=source.find("    let mut groups =\n").unwrap();let end=source.find("\n#[cfg(test)]\nmod tests;").unwrap();let body=&source[start..end];
assert!(body.ends_with("}\n"));
let original=body.replace("ScalarGroups {","Groups {").replace("            _reservation: reservation,\n", "").replace("        _reservation: reservation,\n", "").replace("ahash::RandomState::new()","state");
let mut candidate=original.clone();let old="    let mut groups =\n        std::collections::HashMap::with_capacity_and_hasher(capacity, state);";
assert_eq!(candidate.matches(old).count(),1);candidate=candidate.replace(old,"    let mut groups = super::index_map::IndexMap::new(capacity, state);");
let old="        let index = *groups.entry(*scalar).or_insert_with(|| {\n            values.push(*scalar);\n            counts.push(0);\n            (values.len() - 1) as u32\n        });";
assert_eq!(candidate.matches(old).count(),1);candidate=candidate.replace(old,"        let (index, fresh) = groups.insert(scalar, &mut values);\n        if fresh { counts.push(0); }");
let header="#[derive(Debug,PartialEq,Eq)]\npub struct Groups { pub len:usize,pub values:Vec<[u64;4]>,pub order:Vec<u32>,pub starts:Vec<usize>,pub counts:Vec<usize> }\n";
fs::write(r("builders.rs"),format!("{header}\npub fn original(scalars:&[[u64;4]],capacity:usize,percent:usize,state:ahash::RandomState)->Option<Groups>{{\n{original}\npub fn candidate(scalars:&[[u64;4]],capacity:usize,percent:usize,state:ahash::RandomState)->Option<Groups>{{\n{candidate}")).unwrap();
save(r("extraction-audit.json"),&json!({"production_source_sha256":hash(format!("{Z}/crates/zopro-core/src/msm_affine/grouping.rs")),"source_start":start,"source_end":end,"original_adaptations":["constructor signature","Groups result type/reservation field omitted","caller supplies seeded AHash RandomState"],"candidate_adaptations":["IndexMap instead of HashMap","dictionary insertion populates existing values then counts"],"sample_and_admission_outside_constructor_timer":true,"original_input_eligibility_inherited":true,"production_changed":false}));
fs::copy(format!("{Z}/crates/zopro-bench/examples/support/perf.rs"),r("perf.rs")).unwrap();fs::copy(format!("{P}/cases.json"),r("cases.json")).unwrap();
let cases=read(r("cases.json"));for case in cases.as_array().unwrap(){for name in ["key","request"]{let p=case[name].as_str().unwrap();pins.insert(p.into(),hash(p));}}
for f in ["controller.rs","controller","index_map.rs","probe.rs","builders.rs","perf.rs","design.md","cases.json","extraction-audit.json"]{pins.insert(r(f).display().to_string(),hash(r(f)));}
pins.insert(format!("{P}/frozen-candidate.json"),hash(format!("{P}/frozen-candidate.json")));
save(r("pins-before.json"),&json!(pins));let base=Path::new(frozen["libraries"]["zopro_core"].as_str().unwrap()).parent().unwrap();let ahash=fs::read_dir(base).unwrap().map(|e|e.unwrap().path()).find(|p|p.file_name().unwrap().to_str().unwrap().starts_with("libahash-")&&p.extension().unwrap()=="rlib").unwrap();
let mut args=vec!["rustc".to_string(),"--edition=2021".into(),"-O".into(),"-C".into(),"target-cpu=native".into(),"-C".into(),"lto=fat".into(),"-C".into(),"codegen-units=1".into(),"-L".into(),format!("dependency={}",base.display())];
for(n,p)in frozen["libraries"].as_object().unwrap(){args.extend(["--extern".into(),format!("{n}={}",p.as_str().unwrap())]);}args.extend(["--extern".into(),format!("ahash={}",ahash.display()),r("probe.rs").display().to_string()]);
let mut test=args.clone();test.extend(["--test".into(),"-o".into(),r("probe-tests").display().to_string()]);let mut bench=args;bench.extend(["-o".into(),r("probe").display().to_string()]);
let steps=json!([
{"name":"compile-tests","kind":"command","argv":test,"timeout_seconds":900},
{"name":"run-tests","kind":"command","argv":[r("probe-tests"),"--test-threads=1"],"unfiltered":true,"timeout_seconds":600},
{"name":"compile-probe","kind":"command","argv":bench,"timeout_seconds":900}
]);save(r("plan.json"),&json!({"version":1,"source_pins":r("pins-before.json"),"steps":steps}));println!("Prepared three native component steps, {} pins; original175 unchanged",pins.len());
}
fn main(){setup()}
