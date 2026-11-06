use std::{fs,process::{Command,exit}};
use serde_json::{Value,json};
const R:&str="/home/ubuntu/src/zopro-perf-work/calibrated-compiler176";
fn main(){let arm=std::env::args().nth(1).unwrap();assert!(["OFF","ON"].contains(&arm.as_str()));
 let v:Value=serde_json::from_slice(&fs::read(format!("{R}/original-link.json")).unwrap()).unwrap();
 let mut argv=v["argv"].as_array().unwrap().iter().map(|s|s.as_str().unwrap().to_owned()).collect::<Vec<_>>();
 let objs=argv.iter().enumerate().filter(|(_,s)|s.ends_with(".rcgu.o")).map(|(i,s)|(i,s.clone())).collect::<Vec<_>>();assert_eq!(objs.len(),1);
 let object=format!("{R}/native/{arm}.o");assert!(fs::metadata(&object).unwrap().len()>0);argv[objs[0].0]=object;
 let out=format!("{R}/native/caller-{arm}");let oldout=format!("{R}/native/original-rustc");let index=argv.iter().position(|s|s=="-o").unwrap();assert_eq!(argv[index+1],oldout);argv[index+1]=out;
 assert!(std::path::Path::new(v["cwd"].as_str().unwrap()).is_dir());
 let receipt=format!("{R}/link-{arm}.json");assert!(!std::path::Path::new(&receipt).exists());
 fs::write(&receipt,serde_json::to_vec_pretty(&json!({"program":v["program"],"cwd":v["cwd"],"argv":argv,"replaced_original_object":objs[0].1,"original_ir":"original-rustc.bc","no_arithmetic_source_change":true})).unwrap()).unwrap();
 let status=Command::new(v["program"].as_str().unwrap()).args(&argv).current_dir(v["cwd"].as_str().unwrap()).status().unwrap();
 fs::write(format!("{R}/link-{arm}-exit.json"),serde_json::to_vec_pretty(&json!({"success":status.success(),"code":status.code()})).unwrap()).unwrap();exit(status.code().unwrap_or(128));
}
