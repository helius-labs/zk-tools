use std::{fs,path::{Path,PathBuf}};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
fn hash(p:&Path)->String {format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
struct Editor { root:PathBuf, edits:Vec<Value> }
impl Editor {
 fn change(&mut self, rel:&str, old:&str, new:&str) {
  let p=self.root.join(rel); let s=fs::read_to_string(&p).unwrap();
  assert_eq!(s.matches(old).count(),1,"anchor {rel}: {old}");
  let backup=Path::new("/tmp/zopro-stage-costs/originals").join(rel);
  if !backup.exists(){fs::create_dir_all(backup.parent().unwrap()).unwrap();fs::copy(&p,&backup).unwrap();}
  fs::write(&p,s.replacen(old,new,1)).unwrap(); self.edits.push(json!({"path":rel,"old":old,"new":new}));
 }
}
fn span(id:u64,n:&str)->String {format!("#[cfg(zopro_stage_costs)] let _stage=zopro_core::stage_costs::Span::enter({id},{n});")}
fn main(){
 let root=PathBuf::from("/tmp/zopro-stage-costs"); let src=root.join("candidate"); let mut e=Editor{root:src.clone(),edits:Vec::new()};
 fs::copy(root.join("stage_costs.rs.txt"),src.join("crates/zopro-core/src/stage_costs.rs")).unwrap();
 e.change("crates/zopro-core/src/lib.rs","pub mod alloc;","pub mod alloc;\n#[cfg(all(zopro_stage_costs,target_os=\"linux\"))]\npub mod stage_costs;");
 let core="crates/zopro-core/src/lib.rs";
 e.change(core,"fn solve_inner(&self, witness: &[u8], omit_b: bool) -> Result<Vec<u8>, String> {","fn solve_inner(&self, witness: &[u8], omit_b: bool) -> Result<Vec<u8>, String> {\n        #[cfg(zopro_stage_costs)] let _entry=crate::stage_costs::Span::enter(if rayon::current_thread_index().is_none(){6}else{7},witness.len());");
 e.change(core,"let solver::Solution{mut w,a,b,c}=plan.solve_in_with_slots(witness,self.domain,&folded.slots,materialize_c)?;","let solver::Solution{mut w,a,b,c}={ #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(8,self.domain); plan.solve_in_with_slots(witness,self.domain,&folded.slots,materialize_c)? }; ");
 for (old,new) in [
 ("let correction=folded.public_term(&w[..plan.n_public]);","let correction={ #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(9,plan.n_public); folded.public_term(&w[..plan.n_public]) };"),
 ("let words=canonical(&mut w);","let words={ #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(10,w.len()); canonical(&mut w) };"),
 ("let mut product=queries.product_values(&self.plan,a,b);","let mut product={ #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(12,self.domain); queries.product_values(&self.plan,a,b) };"),
 ("let ((ar,kh),bs2)=rayon::join(","#[cfg(zopro_stage_costs)] let _queries=crate::stage_costs::Span::enter(13,k.len());\n            let ((ar,kh),bs2)=rayon::join("),
 ("            let mut out=Vec::with_capacity(384);\n            for point in [ar,G1Projective::IDENTITY,kh+correction,G1Projective::IDENTITY]", "            #[cfg(zopro_stage_costs)] drop(_queries);\n            #[cfg(zopro_stage_costs)] let _encode=crate::stage_costs::Span::enter(14,4);\n            let mut out=Vec::with_capacity(384);\n            for point in [ar,G1Projective::IDENTITY,kh+correction,G1Projective::IDENTITY]"),
 ("if let Some(proof) = fixed.assemble_rerandomized(&parts) {","if let Some(proof) = { #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(15,parts.len()); fixed.assemble_rerandomized(&parts) } {")
 ] {e.change(core,old,new);}
 let gather="let (sa_vec,mut sb_vec):(Vec<_>,Vec<_>)=rayon::join(||plan.a_indices.par_iter().map(|&i|words[i as usize]).collect(),||plan.b_indices.par_iter().map(|&i|words[i as usize]).collect());";
 e.change(core,gather,"let (sa_vec,mut sb_vec):(Vec<_>,Vec<_>)={ #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(11,words.len()); rayon::join(||plan.a_indices.par_iter().map(|&i|words[i as usize]).collect(),||plan.b_indices.par_iter().map(|&i|words[i as usize]).collect()) };");
 let sdk="crates/zopro-zolana/src/lib.rs";
 e.change(sdk,"let witness = self.witness(request)?;","let witness = {\n            #[cfg(zopro_stage_costs)] let _stage=zopro_prover::zopro_core::stage_costs::Span::enter(1,request.len());\n            self.witness(request)?\n        };");
 e.change(sdk,"pub fn result(&self, proved: zopro_prover::Proved, witness_ms: u64) -> ProofResult {","pub fn result(&self, proved: zopro_prover::Proved, witness_ms: u64) -> ProofResult {\n        #[cfg(zopro_stage_costs)] let _stage=zopro_prover::zopro_core::stage_costs::Span::enter(2,proved.public.len());");
 let prover="crates/zopro-prover/src/lib.rs";
 e.change(prover,"pub fn prove(&mut self, witness: &[Fr]) -> Result<Proved, Error> {\n        let encoded = self.encode(witness)?;",&format!("pub fn prove(&mut self, witness: &[Fr]) -> Result<Proved, Error> {{\n        let encoded = {{ {} self.encode(witness)? }};",span(3,"witness.len()")));
 let raw="let raw = self\n                    .key\n                    .solve_rerandomized(&encoded, &self.fixed)\n                    .map_err(Error::Prove)?;";
 e.change(prover,raw,&format!("let raw = {{ {} self.key.solve_rerandomized(&encoded,&self.fixed).map_err(Error::Prove)? }};",span(4,"encoded.len()")));
 e.change(prover,"let verify_started = zopro_core::time::Instant::now();\n                let public",&format!("let verify_started = zopro_core::time::Instant::now();\n                {}\n                let public",span(5,"self.public")));
 e.change("crates/zopro-core/src/msm_affine.rs","    assert_eq!(bases.len(), words.len());\n    let groups = crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\")","    #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(16,words.len());\n    assert_eq!(bases.len(), words.len());\n    let groups = crate::simd::cpu_option(\"ZOPRO_CPU_DEDUP\")");
 let grouping="crates/zopro-core/src/msm_affine/grouping.rs";
 e.change(grouping,"fn grouping_sample(scalars: &[[u64; 4]], zero_aware: bool) -> Option<(GroupSample<'_>, usize)> {","fn grouping_sample(scalars: &[[u64; 4]], zero_aware: bool) -> Option<(GroupSample<'_>, usize)> {\n    #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(18,scalars.len());");
 e.change(grouping,"fn scalar_groups_policy(scalars: &[[u64; 4]], zero_aware: bool) -> Option<ScalarGroups> {","fn scalar_groups_policy(scalars: &[[u64; 4]], zero_aware: bool) -> Option<ScalarGroups> {\n    #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(17,scalars.len());");
 e.change(grouping,"    let mut groups =\n        std::collections::HashMap", "    #[cfg(zopro_stage_costs)] let _map=crate::stage_costs::Span::enter(19,scalars.len());\n    let mut groups =\n        std::collections::HashMap");
 e.change(grouping,"    drop(groups);\n    if values.len()", "    drop(groups);\n    #[cfg(zopro_stage_costs)] drop(_map);\n    if values.len()");
 e.change(grouping,"    let mut starts = Vec::with_capacity(counts.len() + 1);", "    #[cfg(zopro_stage_costs)] let _pack=crate::stage_costs::Span::enter(20,scalars.len());\n    let mut starts = Vec::with_capacity(counts.len() + 1);");
 e.change("crates/zopro-core/src/msm_affine/g2_phi.rs","    short.par_iter_mut().for_each(|s| *s = phi_split(*s));","    { #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(21,short.len()); short.par_iter_mut().for_each(|s| *s = phi_split(*s)); }\n    #[cfg(zopro_stage_costs)] let _stage=crate::stage_costs::Span::enter(22,short.len());");
 let example=src.join("crates/zopro-bench/examples"); fs::create_dir_all(example.join("support")).unwrap();
 let driver=fs::read_to_string("/tmp/zopro-ready-arithmetic/candidate/crates/zopro-bench/examples/cpu-sdk.rs").unwrap()
 .replace("task_ready","stage_costs").replace("ZOPRO_READY_OUTPUT","ZOPRO_STAGE_OUTPUT");
 let start=driver.find("    if let Some(path)=trace {").unwrap(); let end=driver[start..].find("\n    for (round, ms)").unwrap()+start;
 let export="    if let Some(path)=trace {let records:Vec<_>=observations.into_iter().enumerate().map(|(round,events)|serde_json::json!({\"round\":round,\"events\":events.into_iter().map(|e|serde_json::json!({\"id\":e.id,\"label\":zopro_prover::zopro_core::stage_costs::label(e.label),\"n\":e.n,\"tid\":e.tid,\"wall0\":e.wall0,\"wall1\":e.wall1,\"cpu0\":e.cpu0,\"cpu1\":e.cpu1})).collect::<Vec<_>>()})).collect();fs::write(path,serde_json::to_vec(&records)?)?;}";
 fs::write(example.join("cpu-sdk.rs"),format!("{}{}{}",&driver[..start],export,&driver[end..])).unwrap();
 fs::copy("/tmp/zopro-ready-arithmetic/candidate/crates/zopro-bench/examples/support/perf.rs",example.join("support/perf.rs")).unwrap();
 // Every accepted source must be recoverable, including files with several edits.
 let manifest:Value=serde_json::from_slice(&fs::read("/tmp/zopro-grouping-module/production-source-manifest.json").unwrap()).unwrap();
 let sources=manifest["source_files_sha256"].as_object().unwrap();
 for(rel,wanted)in sources{let p=src.join(rel);let mut s=fs::read(&p).unwrap();for edit in e.edits.iter().rev().filter(|x|x["path"]==*rel){let x=String::from_utf8(s).unwrap();let new=edit["new"].as_str().unwrap();assert_eq!(x.matches(new).count(),1);s=x.replacen(new,edit["old"].as_str().unwrap(),1).into_bytes();}assert_eq!(format!("{:x}",Sha256::digest(&s)),wanted.as_str().unwrap(),"{rel}");}
 fs::write(root.join("edits.json"),serde_json::to_vec_pretty(&e.edits).unwrap()).unwrap();
 let mut pins=serde_json::Map::new(); for rel in sources.keys(){let p=src.join(rel);pins.insert(p.display().to_string(),json!(hash(&p)));}
 for rel in ["crates/zopro-core/src/stage_costs.rs","crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{let p=src.join(rel);pins.insert(p.display().to_string(),json!(hash(&p)));}
 for rel in ["prepare-build.rs","stage_costs.rs.txt","design.md"]{let p=root.join(rel);pins.insert(p.display().to_string(),json!(hash(&p)));}
 fs::write(root.join("source-pins.json"),serde_json::to_vec_pretty(&json!({"files_sha256":pins})).unwrap()).unwrap();
 let mut plan:Value=serde_json::from_slice(&fs::read("/tmp/zopro-ready-arithmetic/native-plan.json").unwrap()).unwrap();
 let rendered=serde_json::to_string(&plan).unwrap().replace("/tmp/zopro-ready-arithmetic","/tmp/zopro-stage-costs").replace("zopro_ready_arithmetic","zopro_stage_costs");
 plan=serde_json::from_str(&rendered).unwrap();
 plan["steps"][2]["required_tests"]=json!(["stage_costs::tests::nested_and_parallel_spans_are_complete","stage_costs::tests::exhaustion_is_reported","stage_costs::tests::unfinished_span_prevents_reuse_until_dropped","stage_costs::tests::panic_disarms_and_allows_reuse"]);
 fs::write(root.join("native-plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
 println!("recovered {} originals; {} edits; {} pins",sources.len(),e.edits.len(),pins.len());
}
