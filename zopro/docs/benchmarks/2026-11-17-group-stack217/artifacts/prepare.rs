use std::{fs,path::Path,collections::BTreeMap};use serde_json::{Value,json};use sha2::{Sha256,Digest};
const R:&str="/home/ubuntu/src/zopro-perf-work/group-stack217";const OLD:&str="/home/ubuntu/src/zopro-perf-work/group-profile216";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);let old=Path::new(OLD);let sdk=old.join("bin/sdk0");assert_eq!(hash(&sdk),"6d9ae8f89e09b4bec8ac7c6a6f795bfbf04594623e4abf39802ada7b4d5d254c");let mut pins:BTreeMap<String,String>=serde_json::from_value(read(old.join("capture-pins.json"))).unwrap();let plan=read(old.join("main-plan.json"));let jobs=read(old.join("jobs.json"));let mut steps=vec![];let mut verifies=vec![];let mut records=vec![];
for b in[false,true]{let arm=if b{"ON"}else{"OFF"};let oldjob=jobs.as_array().unwrap().iter().find(|j|j["smoke"]==false&&j["boolean"]==b&&j["sampled"]==false&&j["block"]==0).unwrap();let oldstep=plan["steps"].as_array().unwrap().iter().find(|s|s["name"]==oldjob["name"]).unwrap();let src=oldstep["argv"].as_array().unwrap().iter().map(|s|s.as_str().unwrap()).collect::<Vec<_>>();let a=src.iter().position(|&s|s==sdk.to_str().unwrap()).unwrap();let image=src[a+1];let req=src[a+2];assert_eq!(hash(image),oldjob["image_sha256"]);let pd=r.join(format!("proofs/{arm}"));let script=r.join(format!("{arm}.gdb"));let text=r#"set pagination off
set confirm off
set print thread-events off
set debuginfod enabled off
set auto-load off
set language c
starti
set $hits = 0
set $function = (unsigned long)&'zopro_core::msm_affine::grouping::scalar_groups'
break *($function+17) if $rdx == 30089
commands
silent
set $hits = $hits + 1
set $frame = (unsigned long)$rsp
set $entry = $frame + 472
set $spill = $frame + 240
set $ret = *(unsigned long*)$entry
printf "STACK_ENTRY hit=%d n=%ld thread=%d entry=0x%lx frame=0x%lx spill=0x%lx return=0x%lx function=0x%lx scalar=0x%lx\n", $hits,$rdx,$_thread,$entry,$frame,$spill,$ret,$function,(unsigned long)$rsi
continue
end
continue
printf "STACK_ENTRIES %d\n",$hits
if $hits != 4
quit 7
end
quit
"#;fs::write(&script,text).unwrap();let mut argv=src[..a].iter().map(|s|s.to_string()).collect::<Vec<_>>();let sudo_env=argv.iter().position(|s|s=="/usr/bin/taskset").unwrap();assert_eq!(&argv[sudo_env..],["/usr/bin/taskset","-c","0-31"]);argv.extend(["/usr/bin/gdb".into(),"--quiet".into(),"--batch".into(),"--return-child-result".into(),"-x".into(),script.display().to_string(),"--args".into(),sdk.display().to_string(),image.into(),req.into(),"1".into(),pd.display().to_string()]);let mut env=oldstep["environment"].clone();if env.is_null(){env=json!({});}env["DEBUGINFOD_URLS"]=json!("");argv.insert(sudo_env,"DEBUGINFOD_URLS=".into());steps.push(json!({"kind":"command","name":format!("audit-{arm}"),"argv":argv,"environment":env,"timeout_seconds":300}));let verifier=old.join("bin/verify");verifies.push(json!({"kind":"command","name":format!("verify-{arm}"),"argv":[verifier,Path::new(image).to_path_buf(),oldjob["image_sha256"].as_str().unwrap(),pd.join("0.proof"),pd.join("0.public"),oldjob["public_sha256"].as_str().unwrap()],"timeout_seconds":300}));records.push(json!({"arm":arm,"boolean":b,"image":image,"request":req,"proof_dir":pd,"public_sha256":oldjob["public_sha256"]}));for p in[script,Path::new(req).to_path_buf(),Path::new(image).to_path_buf()]{pins.insert(p.display().to_string(),hash(p));}}
steps.extend(verifies);let seed=old.join("proof-sha256-after-main.json");assert_eq!(hash(&seed),"3d8d234da2237304a40b1658a1b5a2d47a06a2c6245a464a3ec87ca9a39756ad");for p in[seed,r.join("protocol.md"),r.join("prepare.rs"),r.join("review.rs"),r.join("review"),Path::new("/usr/bin/gdb").to_path_buf()]{pins.insert(p.display().to_string(),hash(p));}fs::write(r.join("jobs.json"),serde_json::to_vec_pretty(&records).unwrap()).unwrap();fs::write(r.join("pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(r.join("plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps})).unwrap()).unwrap();}
