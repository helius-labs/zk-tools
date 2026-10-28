use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(format!("{R}/{p}")).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:&str,v:&Value){fs::write(format!("{R}/{p}"),serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){
    let old=fs::read_to_string(format!("{R}/native/frozen-core-tests/stdout.log")).unwrap();for name in["owned_g1_generator_oracle_tails_and_reuse","borrowed_signed_g1_generator_oracle_tails_and_reuse"]{assert!(old.contains(&format!("test msm::caller_probe::tests::{name} ... ok")));}assert!(old.contains("2 passed; 1 failed"));assert!(read("native/failure.json")["completed"].as_array().unwrap().is_empty());
    fs::rename(format!("{R}/native"),format!("{R}/native-first")).unwrap();let mut plan=read("build-plan.json");let first=&mut plan["steps"][0];let args=first["argv"].as_array_mut().unwrap();let pos=args.iter().position(|v|v=="msm::caller_probe::tests::").unwrap();args[pos]=json!("msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse");first["required_tests"]=json!(["msm::caller_probe::tests::owned_g2_prefix_generator_oracle_tails_and_reuse"]);
    plan["environment"]=json!({"ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","RAYON_NUM_THREADS":"4"});
    let mut s=fs::read_to_string(format!("{R}/experiment.rs")).unwrap().replace("build-pins.json","build-pins-v2.json").replace("build-plan.json","build-plan-v2.json");
    let anchor="let n = if label == \"frozen\" { 3 } else { 122 };";assert!(s.contains(anchor));s=s.replace(anchor,"let n = if label == \"frozen\" { 1 } else { 122 };");
    let anchor="            assert!(t.contains(&format!(\"test msm::caller_probe::tests::{name} ... ok\")));";assert_eq!(s.matches(anchor).count(),1);s=s.replace(anchor,"            if label != \"frozen\" || name == \"owned_g2_prefix_generator_oracle_tails_and_reuse\" { assert!(t.contains(&format!(\"test msm::caller_probe::tests::{name} ... ok\"))); }");
    let anchor="    let ids = identities();\n    for (_, a) in ids.as_object().unwrap() {";assert_eq!(s.matches(anchor).count(),1);
    s=s.replace(anchor,r#"    let before=fs::read_to_string(r.join("native-first/frozen-core-tests/stdout.log")).unwrap();
    for name in["owned_g1_generator_oracle_tails_and_reuse","borrowed_signed_g1_generator_oracle_tails_and_reuse"]{assert!(before.contains(&format!("test msm::caller_probe::tests::{name} ... ok")));}
    let ids = identities();
    for (_, a) in ids.as_object().unwrap() {"#);
    fs::write(format!("{R}/experiment-v2.rs"),s).unwrap();let dep="/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps";let td="/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
    let out=Command::new("rustc").args(["--edition=2021","-O",&format!("{R}/experiment-v2.rs"),"-o",&format!("{R}/experiment-v2"),"-L",&format!("dependency={dep}"),"-L",&format!("dependency={td}"),"--extern",&format!("serde_json={dep}/libserde_json-9c41682dbe4e057f.rlib"),"--extern",&format!("sha2={dep}/libsha2-b36128d04ec91f93.rlib"),"--extern",&format!("rustc_demangle={td}/librustc_demangle-3e2a96f78b3a16e4.rlib")]).output().unwrap();fs::write(format!("{R}/repair-driver-stdout.log"),out.stdout).unwrap();fs::write(format!("{R}/repair-driver-stderr.log"),out.stderr).unwrap();assert!(out.status.success());
    save("preparation-repair.json",&json!({"failure":"Frozen G2 adapter requires prefix-records runtime flag absent from native test environment","correction":"restore same accepted runtime test options as parent harness; timing env already had these options","source_kernel_changed":false,"G1_successful_tests_repeated":false,"only_failed_frozen_G2_test_retried":true,"original_failed_workflow":"native-first/failure.json","timing_started":false}));
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read("build-pins.json")).unwrap();for f in["repair-preparation.rs","repair-preparation","preparation-repair.json","experiment-v2.rs","experiment-v2","native-first/frozen-core-tests/stdout.log","native-first/frozen-core-tests/stderr.log","native-first/failure.json"]{pins.insert(format!("{R}/{f}"),hash(&format!("{R}/{f}")));}
    save("build-pins-v2.json",&json!(pins));plan["source_pins"]=json!(format!("{R}/build-pins-v2.json"));save("build-plan-v2.json",&plan);println!("Restoredrequiredtestenv;preserved2G1successes,retryonlyfailedG2thenunrunsteps");
}
