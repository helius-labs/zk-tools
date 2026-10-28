use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf},collections::{BTreeMap,BTreeSet}};
const R:&str="/home/ubuntu/src/zopro-perf-work/scalar-square-sdk";
const P:&str="/home/ubuntu/src/zopro-perf-work/scalar-symmetric-square";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn setup(){
    assert!(!r("build-plan.json").exists());assert_eq!(read(format!("{P}/analysis.json"))["component_gate_passed"],true);
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read(format!("{P}/pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
    for p in["design.md","native.rs","native-helper"]{pins.insert(r(p).display().to_string(),hash(r(p)));}let mut steps=vec![];let flags=fs::read_to_string(format!("{P}/native-flags.txt")).unwrap();
    for label in["off","on"]{let full=if label=="on"{format!("{flags} --cfg zopro_scalar_symmetric_square")}else{flags.clone()};let env=json!({"RUSTFLAGS":full,"CARGO_TARGET_DIR":r(format!("target-{label}")),"CARGO_BUILD_JOBS":"16"});let cargo=format!("{P}/source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":["solver::tests::mapped_prefixes_validate_roundtrip_and_keep_the_owner_alive","solver::tests::refined_components_match_sequential_execution","solver::tests::solves_and_rejects_invalid_witnesses"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("raw-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","raw::tests::"],"environment":env,"required_tests":["raw::tests::rejects_overflowing_and_truncated_lengths","raw::tests::sparse_images_cross_four_gib_without_truncation"],"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("cubic-{label}"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--test","cubic"],"environment":env,"required_tests":["parses_values","proves_by_name","writes_dump_directly_without_intermediate_point_copies","loads_from_its_raw_form","proves_from_its_image_in_place","reads_legacy_key_and_raw_image"],"unfiltered":true,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),PathBuf::from("freeze"),PathBuf::from(label)],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared12pure-native qualification/build/freeze/linksteps");
}
fn freeze(label:&str){
    assert!(["off","on"].contains(&label));let flags=std::env::var("RUSTFLAGS").unwrap();let base=fs::read_to_string(format!("{P}/native-flags.txt")).unwrap();assert_eq!(flags,if label=="on"{format!("{base} --cfg zopro_scalar_symmetric_square")}else{base});
    let log=fs::read_to_string(r(format!("native/libraries-{label}/stdout.log"))).unwrap();let root=r(format!("frozen/{label}"));fs::create_dir_all(&root).unwrap();let mut libs:BTreeMap<String,BTreeSet<String>>=BTreeMap::new();let mut pins=BTreeMap::new();
    for line in log.lines(){let v:Value=serde_json::from_str(line).unwrap();if v["reason"]!="compiler-artifact"{continue}let name=v["target"]["name"].as_str().unwrap();let files=v["filenames"].as_array().unwrap();let metas:Vec<_>=files.iter().filter_map(Value::as_str).filter(|p|p.ends_with(".rmeta")&&p.contains("/deps/")).collect();for file in files.iter().filter_map(Value::as_str).filter(|p|p.ends_with(".rlib")||p.ends_with(".so")){let p=if file.ends_with(".rlib")&&!file.contains("/deps/")&&metas.len()==1{Path::new(metas[0]).with_extension("rlib")}else{PathBuf::from(file)};let h=hash(&p);let ext=p.extension().unwrap().to_str().unwrap();let dest=root.join(format!("lib{name}-{h}.{ext}"));if dest.exists(){assert_eq!(hash(&dest),h)}else{fs::copy(&p,&dest).unwrap();}assert_eq!(hash(&dest),h);pins.insert(dest.display().to_string(),h);if ext=="rlib"{libs.entry(name.into()).or_default().insert(dest.display().to_string());}}}
    let mut selected=BTreeMap::new();let mut lines=String::new();for name in["zopro_core","zopro_bn254","zopro_prover","zopro_zolana","memmap2","serde_json","libc","rayon"]{let found=libs.get(name).unwrap();assert_eq!(found.len(),1,"{name}");let p=found.first().unwrap();selected.insert(name,p.clone());lines.push_str(&serde_json::to_string(&json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[p]})).unwrap());lines.push('\n');}
    fs::write(r(format!("frozen-{label}-cargo.jsonl")),lines).unwrap();save(r(format!("frozen-{label}.json")),&json!({"label":label,"rustflags":flags,"libraries":selected,"all_libraries":libs,"input_pins":pins,"proc_macro_shared_objects_frozen":true,"cargo_artifacts_sha256":format!("{:x}",Sha256::digest(log.as_bytes()))}));println!("Froze{label}{}rlib/sofiles;8requiredlibraries",pins.len());
}
fn main(){let a:Vec<_>=std::env::args().collect();match a[1].as_str(){"setup"=>setup(),"freeze"=>freeze(&a[2]),_=>panic!("mode")}}
