use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/mixed-sdk";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);let diag=r.join("diagnostic-source");assert!(!r.join("diagnostic-audit.json").exists());
    let audit:Value=serde_json::from_slice(&fs::read(r.join("source-audit.json")).unwrap()).unwrap();
    for f in audit["changed_original_files"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).chain(["crates/zopro-core/src/msm/mixed_g1.rs"]){fs::copy(r.join("source").join(f),diag.join(f)).unwrap();}
    let edits:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/msm-geometry/inverse-edits-extended.json").unwrap()).unwrap();let mut before=BTreeMap::new();
    for(file,list)in edits.as_object().unwrap(){let p=diag.join(file);let original=fs::read_to_string(&p).unwrap();before.insert(file.clone(),hash(&p));let mut next=original.clone();
        for e in list.as_array().unwrap(){let a=e[0].as_str().unwrap();let b=e[1].as_str().unwrap();if a.is_empty(){next.push_str(b);}else{assert_eq!(next.matches(a).count(),1,"{file}:{a}");next=next.replacen(a,b,1);}}
        let mut recovered=next.clone();for e in list.as_array().unwrap().iter().rev(){recovered=recovered.replacen(e[1].as_str().unwrap(),e[0].as_str().unwrap(),1);}assert_eq!(recovered,original);fs::write(p,next).unwrap();
    }
    let file=diag.join("crates/zopro-core/src/msm_affine/borrowed_pending.rs");let mut s=fs::read_to_string(&file).unwrap();
    let needle="            if state.apply::<C,_>(&mut self.affine,&self.batch,&mut self.queue) {\n";assert_eq!(s.matches(needle).count(),1);
    s=s.replacen(needle,&format!("            let before_queue=self.queue.len();\n{needle}                self.geometry.mixed(before_queue-self.queue.len());\n"),1);fs::write(file,s).unwrap();
    let mut observer=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/msm-geometry/observer.rs").unwrap();let needle="    pub(crate) fn pushed(&mut self)";assert_eq!(observer.matches(needle).count(),1);
    observer=observer.replacen(needle,"    pub(crate) fn mixed(&mut self,n:usize) {if self.active {add(&mut self.local,\"mixed_calls\",1);add(&mut self.local,\"mixed_consumed\",n as u64);}}\n    pub(crate) fn pushed(&mut self)",1);fs::write(diag.join("crates/zopro-core/src/msm_geometry.rs"),&observer).unwrap();fs::write(r.join("observer.rs"),observer).unwrap();
    fs::copy("/home/ubuntu/src/zopro-perf-work/msm-geometry/driver.rs",r.join("diagnostic-driver.rs")).unwrap();
    fs::write(r.join("diagnostic-audit.json"),serde_json::to_vec_pretty(&json!({"base_main_files_before_observer":before,"observer_edits_recover_base":true,"additional_counter":"mixed_calls/mixed_consumed after successful queue consumption","main_arithmetic_module_sha256":hash(r.join("source/crates/zopro-core/src/msm/mixed_g1.rs")),"diagnostic_arithmetic_module_sha256":hash(diag.join("crates/zopro-core/src/msm/mixed_g1.rs"))})).unwrap()).unwrap();
}
