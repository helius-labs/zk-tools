use std::{fs,path::Path};
fn main(){let root=Path::new("/tmp/zopro-serial-fft-leaves");let old=fs::read_to_string("/tmp/zopro-fft-pass-grain/make-protocol.rs").unwrap();let mut new=old.replacen("let root=Path::new(\"/tmp/zopro-fft-pass-grain\")","let root=Path::new(\"/tmp/zopro-serial-fft-leaves\")",1).replace("ZOPRO_CPU_FFT_PASS_GRAIN","ZOPRO_CPU_SERIAL_FFT_LEAVES").replace("root.join(\"native/result.json\")","root.join(\"native-v2/result.json\")").replace("root.join(\"candidate/.xtask/source-pins.json\")","root.join(\"candidate-v2/.xtask/source-pins.json\")").replace("root.join(\"native/candidate/artifacts.json\")","root.join(\"native-v2/candidate/artifacts.json\")").replace("root.join(\"native/candidate-sdk\")","root.join(\"native-v2/candidate-sdk\")").replace("\"native/libraries/stdout.log\"","\"native-v2/libraries/stdout.log\"").replace("\"native/candidate/artifacts.json\"","\"native-v2/candidate/artifacts.json\"").replace("\"prepare-source-check.json\"","\"prepare-v2.json\",\"native-plan-v2.json\",\"source-audit.rs\",\"source-audit.json\",\"repair-test.rs\",\"failure-v1/review.json\",\"failure-v1/core-tests.sha256\",\"failure-v1/core-tests\"");let marker="proofs(Path::new(\"/tmp/zopro-bounded-steal-search/aws/local-output/preflight-proofs\"),&mut prior);";assert!(new.contains(marker));new=new.replace(marker,&format!("{marker}\ninventory(\"/tmp/zopro-fft-pass-grain/proof-inventory.json\",&mut prior);"));let marker="perf[\"checks\"]=json!([check]);save(root.join(\"perf-template.json\"),perf.clone());";assert!(new.contains(marker));let injected=r#"perf["checks"]=json!([check]);
perf["cases"]=read("/tmp/zopro-bounded-steal-search/all-nine-template.json")["cases"].clone();
for case in perf["cases"].as_array_mut().unwrap() {
    case["rounds"]=json!(31);
    case["profile_rounds"]=json!(match case["name"].as_str().unwrap(){"1x2-w32"=>101,"12x2-w32"=>51,_=>9});
}
let mut physical=perf["cases"][0].clone();physical["name"]=json!("1x2-w16");physical["workers"]=json!(16);physical["rounds"]=json!(101);physical["cpu_affinity"]=json!("0-15");physical["reference"]=Value::Null;
perf["cases"].as_array_mut().unwrap().push(physical);
perf["profile_cases"]=json!(["1x2-w32","12x2-w32","merge-54x1-w32"]);
assert_eq!(perf["cases"].as_array().unwrap().len(),10);
let calls=perf["cases"].as_array().unwrap().iter().map(|c|c["rounds"].as_u64().unwrap()+3).sum::<u64>()*7*3;
assert_eq!(calls,8610);
save(root.join("perf-template.json"),perf.clone());"#;new=new.replace(marker,injected);assert!(new.contains("root.join(\"native-v2/result.json\")"));fs::write(root.join("make-protocol.rs"),new).unwrap();}
