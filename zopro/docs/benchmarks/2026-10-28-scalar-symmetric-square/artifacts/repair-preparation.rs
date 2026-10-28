use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,process::Command,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/scalar-symmetric-square";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(format!("{R}/{p}")).unwrap()).unwrap()}
fn save(p:&str,v:&Value){fs::write(format!("{R}/{p}"),serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){
    let old=fs::read_to_string(format!("{R}/scalar_square-v1.rs")).unwrap();let new=fs::read_to_string(format!("{R}/scalar_square.rs")).unwrap();
    assert_eq!(old.split("#[cfg(test)]").next(),new.split("#[cfg(test)]").next());
    assert!(read("native/failure.json")["completed"].as_array().unwrap().is_empty());
    fs::rename(format!("{R}/native"),format!("{R}/native-first")).unwrap();
    fs::copy(format!("{R}/scalar_square.rs"),format!("{R}/source/crates/zopro-bn254/src/scalar_square.rs")).unwrap();
    let driver=fs::read_to_string(format!("{R}/experiment.rs")).unwrap().replace("build-pins.json","build-pins-v2.json").replace("build-plan.json","build-plan-v2.json");
    fs::write(format!("{R}/experiment-v2.rs"),driver).unwrap();
    let dep="/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";
    let o=Command::new("rustc").args(["--edition=2021","-O",&format!("{R}/experiment-v2.rs"),"-o",&format!("{R}/experiment-v2"),"-L",&format!("dependency={dep}"),"--extern",&format!("serde_json={dep}/libserde_json-9c41682dbe4e057f.rlib"),"--extern",&format!("sha2={dep}/libsha2-b36128d04ec91f93.rlib")]).output().unwrap();fs::write(format!("{R}/repair-driver-stdout.log"),o.stdout).unwrap();fs::write(format!("{R}/repair-driver-stderr.log"),o.stderr).unwrap();assert!(o.status.success());
    save("preparation-repair.json",&json!({"failure":"E0283 ambiguous generic field constructor in new test","all_successful_tests_or_captures_repeated":false,"native_compile_failed_before_tests":true,"production_kernel_byte_identical":true,"correction":"explicit Fp::<config> in two test-only constructor expressions; rename intermediate y2","failed_workflow":"native-first/failure.json","original_design_unchanged":true}));
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read("build-pins.json")).unwrap();
    for f in["scalar_square.rs","source/crates/zopro-bn254/src/scalar_square.rs","scalar_square-v1.rs","repair-preparation.rs","repair-preparation","experiment-v2.rs","experiment-v2","preparation-repair.json"]{pins.insert(format!("{R}/{f}"),hash(format!("{R}/{f}")));}
    save("build-pins-v2.json",&json!(pins));let mut plan=read("build-plan.json");plan["source_pins"]=json!(format!("{R}/build-pins-v2.json"));save("build-plan-v2.json",&plan);println!("Repaired test-only inference; original kernel unchanged; failed compilation preserved");
}
