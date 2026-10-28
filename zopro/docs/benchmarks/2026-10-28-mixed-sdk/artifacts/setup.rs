use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/mixed-sdk";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const CFG:&str="all(zopro_mixed_g1,zopro_live_preparation_charge,zopro_resource_schedule,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma))";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let r=Path::new(R);assert!(!r.join("source-audit.json").exists());let s=r.join("source");
    let prior=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/mixed-callers/mixed_probe.rs").unwrap();
    let prefix=prior.split("fn baseline").next().unwrap();
    let mut body=prefix.replace("//! Mixed signed G1 flush diagnostic. Production source is unchanged.","//! Shared inversion for ordinary and sparse triple signed additions.")
        .replace("use std::{borrow::Borrow, hint::black_box};","use std::borrow::Borrow;")
        .replace("use zopro_bn254::{G1Affine,G1Config,G1Projective};\n","")
        .replace("fn signed<P:Borrow<G1Affine>>","fn signed<C:CurveConfig<Base=Fq>,P:Borrow<Affine<C>>>")
        .replace("fn complete<P:Copy+Borrow<G1Affine>>","fn complete<C:CurveConfig<Base=Fq>,P:Copy+Borrow<Affine<C>>>")
        .replace("fn mixed<P:Copy+Borrow<G1Affine>>","fn mixed<C:CurveConfig<Base=Fq>,P:Copy+Borrow<Affine<C>>>")
        .replace("G1Affine","Affine<C>")
        .replace("Affine<C>::","Affine::<C>::")
        .replace("signed(e)","signed::<C,_>(e)")
        .replace("signed(&q[","signed::<C,_>(&q[")
        .replace("signed(&queue[","signed::<C,_>(&queue[")
        .replace("complete(b,q,queue,s)","complete::<C,_>(b,q,queue,s)");
    let uncapped="if j>0 {self.triples.push([i,j-1]);}";
    assert_eq!(body.matches(uncapped).count(),1);
    body=body.replacen(uncapped,"if j>0 {if self.triples.len()==MATCHES {self.triples.clear();return;}self.triples.push([i,j-1]);}",1);
    body.push_str(&fs::read_to_string(r.join("admission.rs")).unwrap().replacen("//! Optional mixed-flush storage. The query owns one shared peak reservation.","// Optional mixed-flush storage. Query owns one shared peak reservation.",1));
    fs::write(r.join("mixed_g1.rs"),&body).unwrap();fs::write(s.join("crates/zopro-core/src/msm/mixed_g1.rs"),&body).unwrap();
    let mut edits=BTreeMap::<String,Vec<(String,String)>>::new();
    edits.insert("crates/zopro-core/src/msm.rs".into(),vec![(String::new(),format!("\n#[cfg({CFG})]\npub(super) mod mixed_g1;\n"))]);
    let list=edits.entry("crates/zopro-core/src/msm_affine/borrowed_pending.rs".into()).or_default();
    list.push(("    packed: BatchAdd<C>,\n".into(),format!("    packed: BatchAdd<C>,\n    #[cfg({CFG})]\n    mixed: Vec<crate::msm::mixed_g1::Admitted>,\n")));
    list.push(("            packed: BatchAdd::default(),\n".into(),format!("            packed: BatchAdd::default(),\n            #[cfg({CFG})]\n            mixed: Vec::new(),\n")));
    let marker="    fn projective(&mut self, bucket: usize, point: Affine<C>) {\n";
    list.push((marker.into(),format!("    #[cfg({CFG})]\n    fn enable_mixed(&mut self,owner:Option<&std::sync::Arc<crate::msm::mixed_g1::Admission>>) {{\n        if let Some(owner)=owner {{if let Some(state)=owner.window() {{self.mixed=state;}}}}\n    }}\n{marker}")));
    let execute="        self.packed\n            .apply_signed_borrowed(&mut self.affine, &self.batch);\n";
    list.push((execute.into(),format!("        #[cfg({CFG})]\n        if let Some(state)=self.mixed.first_mut() {{\n            if state.apply::<C,_>(&mut self.affine,&self.batch,&mut self.queue) {{\n                for(index,_)in &self.batch {{self.busy[(index&0x7fffffff)as usize/64]=0;}}\n                self.batch.clear();return;\n            }}\n        }}\n{execute}")));
    let needle="    let size = batch_size(bits).max(32);\n";
    list.push((needle.into(),format!("{needle}    #[cfg({CFG})]\n    let mixed_admission=crate::msm::mixed_g1::admit(size,windows);\n")));
    let needle="    let size = batch_size(width).max(32);\n";
    list.push((needle.into(),format!("{needle}    #[cfg({CFG})]\n    let mixed_admission=crate::msm::mixed_g1::admit(size,windows*parts);\n")));
    for needle in ["            let mut state = Window::new(1usize << (bits - 1), size);\n","                    let mut state = Window::new(1usize << (width - 1), size);\n"] {let indent=needle.len()-needle.trim_start().len();list.push((needle.into(),format!("{needle}{}#[cfg({CFG})]\n{}state.enable_mixed(mixed_admission.as_ref());\n"," ".repeat(indent)," ".repeat(indent))));}
    for (file,list) in &edits {let original=fs::read_to_string(Path::new(Z).join(file)).unwrap();let mut next=original.clone();for(before,after)in list {if before.is_empty(){next.push_str(after);}else{assert_eq!(next.matches(before).count(),1,"{file}:{before}");next=next.replacen(before,after,1);}}
        let mut recovered=next.clone();for(before,after)in list.iter().rev(){assert_eq!(recovered.matches(after).count(),1);recovered=recovered.replacen(after,before,1);}assert_eq!(recovered,original);fs::write(s.join(file),next).unwrap();}
    let manifest:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();let mut same=0;
    for(f,h)in manifest["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(Path::new(Z).join(f)),h.as_str().unwrap());if !edits.contains_key(f){assert_eq!(hash(s.join(f)),h.as_str().unwrap());same+=1;}}
    assert_eq!(same,170);fs::write(r.join("native-flags.txt"),format!("{} --cfg zopro_mixed_g1\n",manifest["rustflags"].as_str().unwrap())).unwrap();
    fs::write(r.join("inverse-edits.json"),serde_json::to_vec_pretty(&edits).unwrap()).unwrap();fs::write(r.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"production_unchanged":172,"private_originals_unchanged":same,"changed_original_files":edits.keys().collect::<Vec<_>>(),"all_inverse_edits_recover":true,"prior_arithmetic_source_sha256":hash("/home/ubuntu/src/zopro-perf-work/mixed-callers/mixed_probe.rs"),"translation":"qualified prefix through mixed body only; G1Affine replaced with generic Affine<C>, explicit C type arguments; fixture/baseline/devdeps excluded","new_module_sha256":hash(r.join("mixed_g1.rs"))})).unwrap()).unwrap();
}
