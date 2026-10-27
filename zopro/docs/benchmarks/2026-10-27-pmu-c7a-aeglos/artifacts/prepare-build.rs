use std::{fs,path::Path};
fn main(){
 let r=Path::new("/tmp/zopro-pmu-c7a-20261027");
 let old="/tmp/zopro-current-c7a-20261027";
 let s=fs::read_to_string(format!("{old}/prepare-native.rs")).unwrap().replace(old,r.to_str().unwrap()).replace("/tmp/zopro-composed-adoption/production-source-manifest.json","/tmp/zopro-clean-fft-adoption/production-source-manifest.json").replace("171","172");
 fs::write(r.join("prepare-native.rs"),s).unwrap();
}
