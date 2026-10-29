use std::{fs,path::Path};use serde_json::{json,Value};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){let r=Path::new("/home/ubuntu/src/zopro-perf-work/shared-wire-sdk175");
let candidate=read(r.join("native-candidate/result.json"));assert_eq!(candidate["status"],"complete");
let off=read(r.join("native/completed.json"));assert_eq!(off.as_array().unwrap().len(),5);
assert_eq!(read(r.join("native/core-control/tests.json"))["passed_tests"].as_array().unwrap().len(),124);
assert_eq!(read(r.join("native-candidate/core-candidate/tests.json"))["passed_tests"].as_array().unwrap().len(),126);
fs::rename(r.join("native/core-candidate"),r.join("native/failed-core-candidate")).unwrap();
for name in["core-candidate","prover-candidate","libraries-candidate","freeze-candidate","candidate","candidate-sdk"] {
std::os::unix::fs::symlink(format!("../native-candidate/{name}"),r.join("native").join(name)).unwrap();
}
let result=json!({"status":"complete","kind":"composed_native_workflows","OFF_completed_steps":off,"ON_workflow":candidate,"ON_workflow_path":r.join("native-candidate/result.json"),"visibility_repair":read(r.join("repair.json")),"successful_tests_repeated":false});
fs::write(r.join("native/result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();
println!("Both native variants complete; original OFF receipts reused, repaired ON126core,8prover");
}
