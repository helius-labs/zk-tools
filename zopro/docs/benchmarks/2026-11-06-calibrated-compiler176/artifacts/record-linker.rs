use std::{env,fs,process::{Command,exit}};
use serde_json::json;
fn main(){
 let argv=env::args().skip(1).collect::<Vec<_>>();
 let root="/home/ubuntu/src/zopro-perf-work/calibrated-compiler176";
 let out=format!("{root}/original-link.json");assert!(!std::path::Path::new(&out).exists(),"link invocation already captured");
 let cwd=env::current_dir().unwrap();
 fs::write(&out,serde_json::to_vec_pretty(&json!({"program":"/usr/bin/cc","argv":argv,"cwd":cwd,"purpose":"one original accepted-Rust final-link invocation; no benchmark"})).unwrap()).unwrap();
 let status=Command::new("/usr/bin/cc").args(&argv).status().unwrap();
 fs::write(format!("{root}/original-link-exit.json"),serde_json::to_vec_pretty(&json!({"success":status.success(),"code":status.code()})).unwrap()).unwrap();
 exit(status.code().unwrap_or(128));
}
