use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-operation-census176";const P:&str="/home/ubuntu/src/zopro-perf-work/late-m3-query176";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}fn load(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn edit(all:&mut Vec<Value>,file:&str,old:&str,new:&str){let p=r("source").join(file);let s=fs::read_to_string(&p).unwrap();assert_eq!(s.matches(old).count(),1,"{file}:{old}");fs::write(p,s.replace(old,new)).unwrap();all.push(json!({"file":file,"old":old,"new":new}));}
fn main(){assert!(!r("source").exists());let pins=load(format!("{P}/build-pins.json"));let mut source_pins=BTreeMap::new();for(p,h)in pins.as_object().unwrap(){if let Ok(relative)=Path::new(p).strip_prefix(format!("{P}/source")){assert_eq!(hash(p),h.as_str().unwrap());let to=r("source").join(relative);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(p,&to).unwrap();source_pins.insert(relative.display().to_string(),h.clone());}}assert_eq!(source_pins.len(),177);
let mut edits=Vec::new();let b="crates/zopro-core/src/msm_affine/g2_borrowed.rs";
edit(&mut edits,b,"    packed: BatchAdd<G2Config>,\n","    packed: BatchAdd<G2Config>,\n    census:census::Row,\n");
edit(&mut edits,b,"            packed,\n","            packed,\n            census:census::Row::new(half,size),\n");
edit(&mut edits,b,"        let i = self.projective_indices[b];","        self.census.event(9,1);\n        let i = self.projective_indices[b];");
edit(&mut edits,b,"        if i == u32::MAX {","        if i == u32::MAX {\n            self.census.event(10,1);");
edit(&mut edits,b,"            self.projective[i as usize] += p;","            self.census.event(11,1);\n            self.projective[i as usize] += p;");
edit(&mut edits,b,"        if a.is_identity() {","        if a.is_identity() {\n            self.census.event(1,1);");
edit(&mut edits,b,"            if a.y == signed.y {","            if a.y == signed.y {\n                self.census.event(12,1);");
edit(&mut edits,b,"                *a = G2Affine::IDENTITY","                self.census.event(13,1);\n                *a = G2Affine::IDENTITY");
edit(&mut edits,b,"        self.batch.push((encoded, p));","        self.batch.push((encoded, p));\n        self.census.maximum(18,self.batch.len());");
edit(&mut edits,b,"        self.packed\n            .apply_signed_borrowed_g2","        let len=self.batch.len();self.census.event(2,len);self.census.event(3,1);self.census.event(4,usize::from(len==self.census.size));self.census.event(14,len%8);self.census.event(15,len/8*8);self.census.histogram[self.census.phase][len]+=1;\n        self.packed\n            .apply_signed_borrowed_g2");
edit(&mut edits,b,"        while let Some(&(encoded, p)) = self.queue.last() {","        while let Some(&(encoded, p)) = self.queue.last() {\n            self.census.event(16,1);");
edit(&mut edits,b,"            self.queue.pop();","            self.queue.pop();\n            self.census.event(6,1);");
edit(&mut edits,b,"        for i in 0..self.queue.len() {","        self.census.event(7,self.queue.len());self.census.event(8,usize::from(!self.queue.is_empty()));\n        for i in 0..self.queue.len() {");
edit(&mut edits,b,"        for (p, s) in bases.iter().zip(short) {","        self.census.feed(w,0,usize::from(!high),bases.len());\n        for (p, s) in bases.iter().zip(short) {");
edit(&mut edits,b,"            let b = d.unsigned_abs() as usize - 1;","            self.census.event(0,1);\n            let b = d.unsigned_abs() as usize - 1;");
edit(&mut edits,b,"                self.queue.push((encoded, p));","                self.queue.push((encoded, p));\n                self.census.event(5,1);self.census.maximum(19,self.queue.len());");
edit(&mut edits,b,"                if self.queue.len() == size - 1 {","                if self.queue.len() == size - 1 {\n                    self.census.event(17,1);");
edit(&mut edits,b,"    fn finish(mut self) -> Vec<G2Affine> {","    fn finish(mut self) -> Vec<G2Affine> {\n        self.census.normalized=self.projective.len();");
edit(&mut edits,b,"        self.affine\n    }","        census::emit(self.census);\n        self.affine\n    }");
let orig=fs::read_to_string(r("source").join(b)).unwrap();fs::write(r("source").join(b),format!("{orig}\n#[cfg(zopro_operation_census)]pub(crate)mod census;\n")).unwrap();edits.push(json!({"file":b,"suffix":"\n#[cfg(zopro_operation_census)]pub(crate)mod census;\n"}));
let child="crates/zopro-core/src/msm_affine/g2_borrowed/late_m3.rs";
edit(&mut edits,child,"        for (p,s) in bases.iter().zip(codes) {","        self.census.feed(col,if m3{3}else{1},comp,bases.len());\n        for (p,s) in bases.iter().zip(codes) {");
edit(&mut edits,child,"            let b=(code&MAG)as usize-1;","            self.census.event(0,1);\n            let b=(code&MAG)as usize-1;");
edit(&mut edits,child,"                self.queue.push((encoded,p));","                self.queue.push((encoded,p));\n                self.census.event(5,1);self.census.maximum(19,self.queue.len());");
edit(&mut edits,child,"                if self.queue.len()==size-1 {self.flush_queue();}","                if self.queue.len()==size-1 {self.census.event(17,1);self.flush_queue();}");
let ma="crates/zopro-core/src/msm_affine.rs";let original=fs::read_to_string(r("source").join(ma)).unwrap();let suffix="\n#[cfg(zopro_operation_census)]pub(crate)use g2_borrowed::census;\n";fs::write(r("source").join(ma),format!("{original}{suffix}")).unwrap();edits.push(json!({"file":ma,"suffix":suffix}));
let lib="crates/zopro-core/src/lib.rs";let original=fs::read_to_string(r("source").join(lib)).unwrap();let suffix="\n#[cfg(zopro_operation_census)]pub mod g2_operation_census{pub use crate::msm_affine::census::{begin,finish,Row,LABELS,GLOBAL_LABELS};}\n";fs::write(r("source").join(lib),format!("{original}{suffix}")).unwrap();edits.push(json!({"file":lib,"suffix":suffix}));
let msm="crates/zopro-core/src/msm.rs";
let a="    let tile = 2048 / (core::mem::size_of::<C::Base>() / 32);";
edit(&mut edits,msm,a,&format!("    if core::mem::size_of::<C::Base>()==64 {{crate::msm_affine::census::indexed(order.len(),starts.len()-1);}}\n{a}"));
let a="    const GROUP: usize = 256;\n    let partials: Vec<_> = buckets";
edit(&mut edits,msm,a,&format!("    if core::mem::size_of::<C::Base>()==64 {{crate::msm_affine::census::weighted(buckets.len(),buckets.iter().filter(|p|!p.is_identity()).count());}}\n{a}"));
let a="    let m = s.pairs.len();";
edit(&mut edits,msm,a,&format!("{a}\n    if core::mem::size_of::<C::Base>()==64 {{crate::msm_affine::census::layer(m,counts.iter().map(|n|n/2).sum());}}"));
fs::copy(r("census.rs"),r("source/crates/zopro-core/src/msm_affine/g2_borrowed/census.rs")).unwrap();
// Recover the exact already-qualified parent snapshot by reversing every observer edit.
for(p,h)in&source_pins{let mut bytes=fs::read(r("source").join(p)).unwrap();if edits.iter().any(|e|e["file"]==p.as_str()){let mut s=String::from_utf8(bytes).unwrap();for e in edits.iter().rev().filter(|e|e["file"]==p.as_str()){if let Some(suffix)=e["suffix"].as_str(){assert!(s.ends_with(suffix));s.truncate(s.len()-suffix.len());}else{let new=e["new"].as_str().unwrap();assert_eq!(s.matches(new).count(),1);s=s.replacen(new,e["old"].as_str().unwrap(),1);}}bytes=s.into_bytes();}assert_eq!(format!("{:x}",Sha256::digest(bytes)),h.as_str().unwrap());}
fs::write(r("observer-edits.json"),serde_json::to_vec_pretty(&edits).unwrap()).unwrap();fs::write(r("source-audit.json"),serde_json::to_vec_pretty(&json!({"parent_snapshot_files":177,"new_snapshot_files":178,"all177parent_bytes_recover":true,"parent_snapshot_pins":source_pins,"field_and_original_point_formulas_exact":true,"counter_only_changes":true})).unwrap()).unwrap();
for f in["native.rs","candidate-flags.txt","native-flags.txt"]{let s=fs::read_to_string(format!("{P}/{f}")).unwrap();fs::write(r(f),if f=="native.rs"{s.replace(P,R)}else{format!("{s} --cfg zopro_operation_census")}).unwrap();}
println!("Prepared178files;all177parentbytesrecover;counteronly/no productionchanges");}
