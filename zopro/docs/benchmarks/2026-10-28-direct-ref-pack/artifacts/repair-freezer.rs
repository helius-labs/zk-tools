use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(format!("{R}/{p}")).unwrap()).unwrap()}
fn save(p:&str,v:&Value){fs::write(format!("{R}/{p}"),serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn compile(source:&str,binary:&str){let dep="/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";let td="/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";let o=Command::new("rustc").args(["--edition=2021","-O",&format!("{R}/{source}"),"-o",&format!("{R}/{binary}"),"-L",&format!("dependency={dep}"),"-L",&format!("dependency={td}"),"--extern",&format!("serde_json={dep}/libserde_json-9c41682dbe4e057f.rlib"),"--extern",&format!("sha2={dep}/libsha2-b36128d04ec91f93.rlib"),"--extern",&format!("rustc_demangle={td}/librustc_demangle-3e2a96f78b3a16e4.rlib")]).output().unwrap();fs::write(format!("{R}/{binary}-build-stdout.log"),o.stdout).unwrap();fs::write(format!("{R}/{binary}-build-stderr.log"),o.stderr).unwrap();assert!(o.status.success());}
fn main(){
    assert_eq!(read("native/failure.json")["completed"],json!(["frozen-core-tests","frozen-build"]));
    let mut s=fs::read_to_string(format!("{R}/native.rs")).unwrap();let anchor="let suffix=stem.strip_prefix(&format!(\"lib{name}-\")).unwrap();";assert_eq!(s.matches(anchor).count(),1);
    let start=s.find("                let stem=p.file_stem()").unwrap();let end=s[start..].find("                save(r(format!(\"native/{label}-{name}-fingerprint.json\")),&fingerprint);").unwrap()+start;
    s.replace_range(start..end,r#"                let stem=p.file_stem().unwrap().to_str().unwrap();
                let cargo_name=name.replace('_',"-");
                let paths=if let Some(suffix)=stem.strip_prefix(&format!("lib{name}-")) {
                    vec![r(format!("target-{label}/release/.fingerprint/{cargo_name}-{suffix}/lib-{name}.json"))]
                }else{
                    assert_eq!(stem,format!("lib{name}"));
                    let dir=r(format!("target-{label}/release/.fingerprint"));
                    fs::read_dir(dir).unwrap().map(|e|e.unwrap()).filter(|e|e.file_name().to_string_lossy().starts_with(&format!("{cargo_name}-"))).map(|e|e.path().join(format!("lib-{name}.json"))).filter(|p|p.is_file()).collect::<Vec<_>>()
                };
                assert!(!paths.is_empty());let mut fingerprint=json!([]);
                for path in paths {let f=read(&path);assert_eq!(f["rustflags"],json!(flags.split_whitespace().collect::<Vec<_>>()));fingerprint.as_array_mut().unwrap().push(json!({"path":path,"receipt":f}));}
"#);
    let anchor="r(format!(\"native/{label}-build/stdout.log\"))";assert_eq!(s.matches(anchor).count(),1);s=s.replace(anchor,"r(format!(\"{}/{label}-build/stdout.log\",if label==\"frozen\"{\"native\"}else{\"native-rest\"}))");
    fs::write(format!("{R}/native-v3.rs"),s).unwrap();compile("native-v3.rs","native-helper-v3");
    let mut s=fs::read_to_string(format!("{R}/experiment-v2.rs")).unwrap().replace("build-pins-v2.json","build-pins-v3.json").replace("build-plan-v2.json","build-plan-v3.json").replace("native/result.json","native-rest/result.json");
    s=s.replace("native/{label}-field-tests/stdout.log","native-rest/{label}-field-tests/stdout.log");
    s=s.replace("format!(\"native/{label}-core-tests/stdout.log\")","format!(\"{}/{label}-core-tests/stdout.log\", if label==\"frozen\"{\"native\"}else{\"native-rest\"})");
    fs::write(format!("{R}/experiment-v3.rs"),s).unwrap();compile("experiment-v3.rs","experiment-v3");
    save("freezer-repair.json",&json!({"failure":"core cdylib+rlib artifact has un-hashed libzopro_core.rlib filename; fingerprint audit assumed hashed name","scope":"metadata/freezer only; ordinary core rlib copied immediately after same isolated build, no intervening build","already_passed_G2_test_and_frozen_build_repeated":false,"frozen_freezer_only_failed_step_retried":true,"source_kernel_changed":false,"new_workflow":"native-rest","original_failure":"native/failure.json","timing_started":false}));
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read("build-pins-v2.json")).unwrap();for f in["native-v3.rs","native-helper-v3","experiment-v3.rs","experiment-v3","repair-freezer.rs","repair-freezer","freezer-repair.json","native/failure.json","native/frozen-build/stdout.log","native/frozen-core-tests/stdout.log"]{pins.insert(format!("{R}/{f}"),hash(&format!("{R}/{f}")));}save("build-pins-v3.json",&json!(pins));
    let mut plan=read("build-plan-v2.json");plan["steps"]=json!(plan["steps"].as_array().unwrap()[2..].to_vec());for step in plan["steps"].as_array_mut().unwrap(){if step["name"].as_str().unwrap().ends_with("-freeze"){step["argv"][0]=json!(format!("{R}/native-helper-v3"));}}plan["source_pins"]=json!(format!("{R}/build-pins-v3.json"));save("build-plan-v3.json",&plan);println!("Freezer handles un-hashed core artifact; preserved successful G2test/build; no source/timingchange");
}
