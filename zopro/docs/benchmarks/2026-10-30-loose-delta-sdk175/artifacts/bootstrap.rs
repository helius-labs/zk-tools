use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/loose-delta-sdk175";const P:&str="/home/ubuntu/src/zopro-perf-work/queue-drain175";
fn main(){let root=Path::new(R);assert!(!root.join("native.rs").exists());let s=fs::read_to_string(Path::new(P).join("native.rs")).unwrap().replace(P,R);fs::write(root.join("native.rs"),s).unwrap();}
