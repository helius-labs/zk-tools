use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/msm-geometry";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String {format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main() {
    let r=Path::new(R);let source=r.join("source");assert!(!r.join("source-audit.json").exists());
    let mut edits=BTreeMap::<String,Vec<(String,String)>>::new();
    edits.insert("crates/zopro-core/src/lib.rs".into(),vec![(String::new(),"\n#[cfg(zopro_msm_geometry)]\npub mod msm_geometry;\n".into())]);
    edits.insert("crates/zopro-core/src/msm.rs".into(),vec![("    let m = s.pairs.len();\n".into(),"    let m = s.pairs.len();\n    #[cfg(zopro_msm_geometry)]\n    crate::msm_geometry::tree::<C>(counts,m);\n".into())]);
    for (file,path) in [("crates/zopro-core/src/msm_affine.rs","owned"),("crates/zopro-core/src/msm_affine/borrowed_pending.rs","borrowed")] {
        let list=edits.entry(file.into()).or_default();
        let field=if path=="owned" {"    affine: Vec<Affine<C>>,\n"}else{"    affine: Vec<Affine<C>>,\n"};
        list.push((field.into(),format!("    #[cfg(zopro_msm_geometry)]\n    geometry: crate::msm_geometry::Stream,\n{field}")));
        let init="            affine: vec![Affine::<C>::IDENTITY; half],\n";
        list.push((init.into(),format!("            #[cfg(zopro_msm_geometry)]\n            geometry: crate::msm_geometry::Stream::new::<C>(\"{path}\",half,size),\n{init}")));
        let execute="    fn execute(&mut self) {\n        if self.batch.is_empty() {\n            return;\n        }\n";
        list.push((execute.into(),format!("{execute}        #[cfg(zopro_msm_geometry)]\n        self.geometry.execute(&self.batch,&self.queue);\n")));
        let flush="    fn flush_queue(&mut self) {\n";
        list.push((flush.into(),format!("{flush}        #[cfg(zopro_msm_geometry)]\n        self.geometry.flush(&self.batch,&self.queue);\n")));
        let proj=if path=="owned" {"    fn add_projective(&mut self, bucket: usize, point: Affine<C>) {\n"}else{"    fn projective(&mut self, bucket: usize, point: Affine<C>) {\n"};
        list.push((proj.into(),format!("{proj}        #[cfg(zopro_msm_geometry)]\n        self.geometry.projective();\n")));
        let empty="        if current.is_identity() {\n";
        list.push((empty.into(),format!("{empty}            #[cfg(zopro_msm_geometry)]\n            self.geometry.empty();\n")));
        let push="        self.batch.push((encoded, point));\n";
        list.push((push.into(),format!("        #[cfg(zopro_msm_geometry)]\n        self.geometry.pushed();\n{push}")));
        let queued=if path=="owned" {"            w.queue.push((encoded, point));\n"}else{"                self.queue.push((encoded, point));\n"};
        let hook=if path=="owned" {"            #[cfg(zopro_msm_geometry)]\n            w.geometry.queued();\n"}else{"                #[cfg(zopro_msm_geometry)]\n                self.geometry.queued();\n"};
        list.push((queued.into(),format!("{hook}{queued}")));
        let blocked=if path=="owned" {"            if self.busy((bucket&0x7fffffff)as usize) {\n"}else{"            if self.busy((encoded & 0x7fffffff) as usize) {\n"};
        list.push((blocked.into(),format!("{blocked}                #[cfg(zopro_msm_geometry)]\n                self.geometry.blocked();\n")));
    }
    let mut changed=Vec::new();
    for (file,list) in &edits {
        let original=fs::read_to_string(Path::new(Z).join(file)).unwrap();let mut next=original.clone();
        for (before,after) in list {
            if before.is_empty() {next.push_str(after);}else{assert_eq!(next.matches(before).count(),1,"{file}: {before}");next=next.replacen(before,after,1);}
        }
        let mut recovered=next.clone();
        for (before,after) in list.iter().rev() {assert_eq!(recovered.matches(after).count(),1);recovered=recovered.replacen(after,before,1);}
        assert_eq!(recovered,original);fs::write(source.join(file),next).unwrap();changed.push(file);
    }
    fs::copy(r.join("observer.rs"),source.join("crates/zopro-core/src/msm_geometry.rs")).unwrap();
    let manifest:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut same=0;for (file,h) in manifest["source_files_sha256"].as_object().unwrap() {assert_eq!(hash(Path::new(Z).join(file)),h.as_str().unwrap());if !edits.contains_key(file) {assert_eq!(hash(source.join(file)),h.as_str().unwrap());same+=1;}}
    assert_eq!(same,168);
    fs::write(r.join("native-flags.txt"),format!("{} --cfg zopro_msm_geometry\n",manifest["rustflags"].as_str().unwrap())).unwrap();
    fs::write(r.join("inverse-edits.json"),serde_json::to_vec_pretty(&edits).unwrap()).unwrap();
    fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"production_files_unchanged":172,"private_original_files_unchanged":same,"changed_original_files":changed,"all_inverse_edits_recover_original_bytes":true,"new_module":"crates/zopro-core/src/msm_geometry.rs","production_changed":false})).unwrap()).unwrap();
    println!("Prepared observer hooks:172production unchanged,168private originals unchanged,4inverse edits recover");
}
