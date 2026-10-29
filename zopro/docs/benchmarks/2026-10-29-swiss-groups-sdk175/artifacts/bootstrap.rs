use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/swiss-groups-sdk175";
fn main(){let r=Path::new(R);let old=Path::new("/home/ubuntu/src/zopro-perf-work/early-x-sdk175");assert!(!r.join("native.rs").exists());
fs::write(r.join("native.rs"),fs::read_to_string(old.join("native.rs")).unwrap().replace("early-x-sdk175","swiss-groups-sdk175")).unwrap();
let mut s=fs::read_to_string(old.join("capture.rs")).unwrap().replace("early-x-sdk175","swiss-groups-sdk175").replace("/home/ubuntu/src/zopro-perf-work/g2-part-ready175/proof-sha256-after.json","/home/ubuntu/src/zopro-perf-work/early-x-sdk175/proof-sha256-after.json").replace("20881","21007").replace("20755","20881").replace("completed early canonical X standardSDK primary31blockpilot; no adoption","completed index-only grouped-probing standardSDK primary31blockpilot; no adoption");
s=s.replace("\"make-capture.rs\",\"capture-setup.rs\",","");
fs::write(r.join("capture.rs"),s).unwrap();
}
