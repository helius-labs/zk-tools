use std::{fs,path::Path,collections::BTreeMap};use serde_json::{Value,json};use sha2::{Sha256,Digest};
const R:&str="/home/ubuntu/src/zopro-perf-work/group-input212";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);let old=Path::new("/home/ubuntu/src/zopro-perf-work/group-cpu211");let mut pins:BTreeMap<String,String>=serde_json::from_value(read(old.join("build-pins.json"))).unwrap();let jobs=read(old.join("jobs.json"));let plan=read(old.join("smoke-plan.json"));let sdk=old.join("build-run/candidate-sdk");assert_eq!(hash(&sdk),"84a472c6cd1a1eca40c9010d330aeea870ceb8e54381384bd163c2b626b3a6fe");let mut steps=vec![];let mut records=vec![];
 for b in[false,true]{let oldjob=jobs.as_array().unwrap().iter().find(|j|j["smoke"]==true&&j["boolean"]==b&&j["trace"]==true).unwrap();let oldstep=plan["steps"].as_array().unwrap().iter().find(|s|s["name"]==oldjob["name"]).unwrap();let arm=if b{"ON"}else{"OFF"};let cmdfile=r.join(format!("{arm}.gdb"));let pd=r.join(format!("proofs/{arm}"));fs::create_dir_all(r.join(format!("dumps/{arm}"))).unwrap();let text=format!(r#"set pagination off
set confirm off
set print thread-events off
set debuginfod enabled off
set auto-load off
set language c
starti
set $hits = 0
break *'zopro_core::msm_affine::grouping::scalar_groups' if $rdx == 30089
commands
silent
set $hits = $hits + 1
set $begin = $rsi
set $end = $rsi + 32*$rdx
eval "dump binary memory {R}/dumps/{arm}/scalar-%d.bin %p %p", $hits, $begin, $end
printf "INPUT_DUMP hit=%d n=%ld bytes=%ld alignment64=%ld\n", $hits, $rdx, $end-$begin, ((unsigned long)$begin)%64
continue
end
continue
printf "INPUT_DUMPS %d\n", $hits
if $hits != 4
quit 7
end
quit
"#);fs::write(&cmdfile,text).unwrap();let mut env=oldstep["environment"].clone();env["RAYON_NUM_THREADS"]=json!("32");env["DEBUGINFOD_URLS"]=json!("");let req=oldstep["argv"][4].as_str().unwrap();let image=oldjob["image"].as_str().unwrap();let job=json!({"arm":arm,"boolean":b,"proof_dir":pd,"image":image,"image_sha256":oldjob["image_sha256"],"request":req,"public_sha256":oldjob["public_sha256"]});
 steps.push(json!({"kind":"command","name":format!("dump-{arm}"),"argv":["/usr/bin/taskset","-c","0-31","/usr/bin/gdb","--quiet","--batch","--return-child-result","-x",cmdfile,"--args",sdk,image,req,"1",pd],"environment":env,"timeout_seconds":300}));steps.push(json!({"kind":"command","name":format!("verify-{arm}"),"argv":[old.join("bin/verify"),Path::new(image),Path::new(oldjob["image_sha256"].as_str().unwrap()),pd.join("0.proof"),pd.join("0.public"),Path::new(oldjob["public_sha256"].as_str().unwrap())],"timeout_seconds":300}));records.push(job);
 for p in[cmdfile,Path::new(image).to_path_buf(),Path::new(req).to_path_buf()]{pins.insert(p.display().to_string(),hash(p));}
 }
 for p in[sdk,old.join("bin/verify"),r.join("protocol.md"),r.join("prepare.rs"),Path::new("/usr/bin/gdb").to_path_buf(),Path::new("/usr/bin/taskset").to_path_buf()]{pins.insert(p.display().to_string(),hash(p));}let seed=old.join("proof-sha256-after-main.json");assert_eq!(hash(&seed),"25f3486018e2702859911f4e047fb105d57020648a30aa46d2579e6262ad6e2b");pins.insert(seed.display().to_string(),hash(seed));
 fs::write(r.join("jobs.json"),serde_json::to_vec_pretty(&records).unwrap()).unwrap();fs::write(r.join("pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(r.join("plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("pins.json"),"steps":steps})).unwrap()).unwrap();println!("TwoexactSDK GDBdataaudits+Gnark;8privateinputdumps expected;no timingswillbeinterpreted");}
