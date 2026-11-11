use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/ibs-backing177";
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
    assert!(!Path::new(&format!("{R}/post-plan.json")).exists());
    let mut p:Value=serde_json::from_slice(&fs::read(format!("{R}/plan.json")).unwrap()).unwrap();
    let all=p["steps"].as_array().unwrap();let first=all.iter().position(|s|s["name"]=="raw-b0-op-c1").unwrap();
    assert!(!Path::new(&format!("{R}/run/{}/exit.json",all[first+1]["name"].as_str().unwrap())).exists());
    let mut steps=all[first..].to_vec();
    for s in &mut steps{let name=s["name"].as_str().unwrap().to_string();let a=s["argv"].as_array_mut().unwrap();
        if name.starts_with("raw-")||name.starts_with("script-")||name.starts_with("metadata-"){assert_eq!(&a[..2],&[json!("sudo"),json!("-n")]);a.drain(..2);}
        if name.starts_with("decode-"){for v in a{let x=v.as_str().unwrap();if x.starts_with(&format!("{R}/run/raw-"))||x.starts_with(&format!("{R}/run/script-")){*v=json!(x.replacen(&format!("{R}/run/"),&format!("{R}/postrun/"),1));}}}
    }
    p["steps"]=json!(steps);fs::write(format!("{R}/post-plan.json"),serde_json::to_vec_pretty(&p).unwrap()).unwrap();
    let s=fs::read_to_string(format!("{R}/flow.rs")).unwrap();
    let a="assert_eq!(read(r(\"run/result.json\"))[\"status\"], \"complete\");";assert_eq!(s.matches(a).count(),1);
    let s=s.replace(a,"assert_eq!(read(r(\"postrun/result.json\"))[\"status\"], \"complete\");");
    let a="for s in read(r(\"plan.json\"))[\"steps\"].as_array().unwrap() {\n        receipt(\"run\", s);\n    }";assert_eq!(s.matches(a).count(),1);
    let s=s.replace(a,"for s in read(r(\"plan.json\"))[\"steps\"].as_array().unwrap() { if s[\"name\"]==\"raw-b0-op-c1\" {break;} receipt(\"run\",s); }\n    for s in read(r(\"post-plan.json\"))[\"steps\"].as_array().unwrap() {receipt(\"postrun\",s);}");
    let s=s.replace("run/metadata-{name}/stdout.log","postrun/metadata-{name}/stdout.log");
    fs::write(format!("{R}/close.rs"),s).unwrap();
    fs::write(format!("{R}/repair.json"),serde_json::to_vec_pretty(&json!({"original_plan_sha256":hash(&format!("{R}/plan.json")),"post_plan_sha256":hash(&format!("{R}/post-plan.json")),"original_flow_sha256":hash(&format!("{R}/flow.rs")),"close_source_sha256":hash(&format!("{R}/close.rs")),"reason":"After chown to ubuntu, privileged perf rejects file ownership. Decode as owning normal user. No profile bytes, samples, decoder semantics, proof verification or gates changed.","completed_original_steps":first,"pending_offline_steps":p["steps"].as_array().unwrap().len()})).unwrap()).unwrap();
    println!("Prepared offline-only recovery and closeout reader; preserved original failed command");
}
