use std::{fs,path::Path};
fn main(){let root=Path::new("/tmp/zopro-retained-deque-buffers");
let old=fs::read_to_string("/tmp/zopro-serial-fft-leaves/make-protocol.rs").unwrap();let mut source=old.replace("/tmp/zopro-serial-fft-leaves","/tmp/zopro-retained-deque-buffers").replace("ZOPRO_CPU_SERIAL_FFT_LEAVES","ZOPRO_CPU_RETAINED_DEQUE_BUFFERS");
let start=source.find("for name in [\"experiment.json\"").unwrap();let end=source[start..].find("\nadd_file(\"/home/ubuntu").unwrap()+start;
source.replace_range(start..end,r#"for name in ["experiment.json","prepare.json","prepare-v2.json","native-plan-v2.json","source-audit.rs","source-audit.json","repair-build.rs","failure-v1.json","native-plan.json","implement.rs","make-protocol.rs","analyze.rs","analyze","analyze-tests","analyze-tests.log","inventory-preflight.sh","validate-preflight.jq","build-input-pins-v2.json","prepare-build.rs","prepare-build-v2.rs","upstream-pins.json","lock-audit.json","trace-build-plan.json","prepare-protocol-source.rs","registry-audit.rs","registry-audit.json","registry-input-pins.json"]{add_file(root.join(name),&mut pins)}
for (name,digest) in read(root.join("build-input-pins-v2.json")).as_object().unwrap(){assert_eq!(hash(name),digest.as_str().unwrap());pins.insert(name.clone(),digest.as_str().unwrap().into());}
for (name,digest) in read(root.join("registry-input-pins.json")).as_object().unwrap(){assert_eq!(hash(name),digest.as_str().unwrap());pins.insert(name.clone(),digest.as_str().unwrap().into());}
// Pin the exact scheduler/collector libraries present in each actual Cargo
// artifact stream, in addition to the eight directly linked driver libraries.
let mut linked=serde_json::Map::new();
for (variant,path) in [("accepted","/tmp/zopro-grouping-module/native/libraries/stdout.log"),("candidate","/tmp/zopro-retained-deque-buffers/native-v2/libraries/stdout.log")] {
 let text=fs::read_to_string(path).unwrap();let mut found=BTreeMap::<String,String>::new();
 for line in text.lines(){if let Ok(v)=serde_json::from_str::<Value>(line){if v["reason"]=="compiler-artifact" {if let Some(name)=v["target"]["name"].as_str(){if ["crossbeam_deque","crossbeam_epoch","crossbeam_utils","rayon_core"].contains(&name){for f in v["filenames"].as_array().unwrap(){let p=f.as_str().unwrap();if p.ends_with(".rlib"){assert!(found.insert(name.into(),p.into()).is_none());add_file(p,&mut pins);}}}}}}}
 assert_eq!(found.len(),4); linked.insert(variant.into(),json!(found));add_file(path,&mut pins);
}
save(root.join("linked-scheduler-libraries.json"),json!(linked));add_file(root.join("linked-scheduler-libraries.json"),&mut pins);"#);
let marker="inventory(\"/tmp/zopro-fft-pass-grain/proof-inventory.json\",&mut prior);";
assert_eq!(source.matches(marker).count(),1);source=source.replace(marker,&format!("{marker}\ninventory(\"/tmp/zopro-serial-fft-leaves/proof-inventory.json\",&mut prior);"));
fs::write(root.join("make-protocol.rs"),source).unwrap();
for name in ["analyze.rs","inventory-preflight.sh","validate-preflight.jq"] {let old=fs::read_to_string(Path::new("/tmp/zopro-serial-fft-leaves").join(name)).unwrap();let new=old.replace("/tmp/zopro-serial-fft-leaves","/tmp/zopro-retained-deque-buffers");fs::write(root.join(name),new).unwrap();}
let old=fs::read_to_string(root.join("native-plan-v2.json")).unwrap();let mut trace:serde_json::Value=serde_json::from_str(&old).unwrap();trace["steps"].as_array_mut().unwrap().retain(|s|["compiler","libraries","candidate"].contains(&s["name"].as_str().unwrap()));trace["environment"]["RUSTFLAGS"]=serde_json::json!(format!("{} --cfg zopro_deque_trace",trace["environment"]["RUSTFLAGS"].as_str().unwrap()));trace["environment"]["CARGO_TARGET_DIR"]=serde_json::json!(root.join("target-trace"));let text=serde_json::to_string_pretty(&trace).unwrap().replace("/native-v2/","/trace-build/");fs::write(root.join("trace-build-plan.json"),format!("{text}\n")).unwrap();
println!("Prepared frozen all-nine protocol sources and separate post-perf capacity build plan");}
