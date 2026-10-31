use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn once(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replace(a,b);}
fn main(){
 let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json")).unwrap()).unwrap();
 let originals=m["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),176);
 let mut pins=BTreeMap::new();let mut changed=BTreeMap::new();
 for(path,h)in originals{
  let bytes=fs::read(format!("{Z}/{path}")).unwrap();assert_eq!(hash(&bytes),h.as_str().unwrap());let mut out=bytes.clone();
  if path=="crates/zopro-core/src/solver.rs" {
   let mut s=String::from_utf8(out).unwrap();
   once(&mut s,"    Bits,\n}","    Bits,\n    // Transient metadata: image encoding remains InvZero.\n    InverseAlias { source: u32, factor: [u32; 8] },\n}");
   assert_eq!(s.matches("        plan.compact_coefficients();").count(),2);
   s=s.replace("        plan.compact_coefficients();","        plan.compact_coefficients();\n        plan.prepare_inverse_aliases();");
   once(&mut s,"                        hint.kind as usize,","                        match hint.kind { HintKind::Bits=>1, HintKind::InvZero|HintKind::InverseAlias{..}=>0 },");
   once(&mut s,"                    if self.terms[hint.input()].iter().any(|t| !ready[t.wire()]) {","                    if matches!(hint.kind,HintKind::InverseAlias{source,..} if !ready[source as usize]) || self.terms[hint.input()].iter().any(|t| !ready[t.wire()]) {");
   once(&mut s,"            Op::Hint(hint) => out.extend(self.terms[hint.input()].iter().map(|t| t.wire())),","            Op::Hint(hint) => {\n                out.extend(self.terms[hint.input()].iter().map(|t| t.wire()));\n                if let HintKind::InverseAlias{source,..}=hint.kind {out.push(source as usize);}\n            },");
   once(&mut s,"            Op::Hint(hint) => {\n                let mut value = Fr::ZERO;","            Op::Hint(hint) => {\n                if let HintKind::InverseAlias{source,factor}=hint.kind {\n                    let scale=inverse_aliases::field(factor);\n                    let value=get(source as usize);\n                    set(hint.outputs().start,if scale==Fr::ONE {value}else{value*scale});\n                    return Ok(());\n                }\n                let mut value = Fr::ZERO;");
   once(&mut s,"                    HintKind::InvZero => {\n                        set(hint.outputs().start, value.inverse().unwrap_or(Fr::ZERO))\n                    }","                    HintKind::InvZero => {\n                        set(hint.outputs().start, value.inverse().unwrap_or(Fr::ZERO))\n                    }\n                    HintKind::InverseAlias{..}=>unreachable!(),");
   once(&mut s,"                                    HintKind::Bits => 1,","                                    HintKind::Bits => 1,\n                                    HintKind::InverseAlias{..}=>2,");
   once(&mut s,"                            for t in &self.terms[h.input()] {","                            if let HintKind::InverseAlias{source,factor}=h.kind {key.push(source);key.extend(factor);}\n                            for t in &self.terms[h.input()] {");
   for name in ["encode_words","small","with_hints"] {once(&mut s,&format!("    fn {name}("),&format!("    pub(super) fn {name}("));}
   s.push_str("\nmod inverse_aliases;\npub(crate) fn inverse_hint_alias_snapshot()->[usize;4]{inverse_aliases::snapshot()}\n");
   out=s.into_bytes();
  } else if path=="crates/zopro-core/src/solver/bounded.rs" {
   let mut s=String::from_utf8(out).unwrap();
   once(&mut s,"            Op::Hint(h) => h.input().len(),","            Op::Hint(h) => h.input().len().checked_add(usize::from(matches!(h.kind,HintKind::InverseAlias{..})))?,");out=s.into_bytes();
  } else if path=="crates/zopro-core/src/lib.rs" {
   out.extend_from_slice(b"\npub fn inverse_hint_alias_snapshot()->[usize;4]{solver::inverse_hint_alias_snapshot()}\n");
  }
  if out!=bytes {changed.insert(path.clone(),json!({"original_sha256":hash(&bytes),"candidate_sha256":hash(&out)}));}
  let dst=format!("{R}/source/{path}");assert!(!Path::new(&dst).exists());fs::create_dir_all(Path::new(&dst).parent().unwrap()).unwrap();fs::write(&dst,&out).unwrap();pins.insert(dst,hash(&out));
 }
 let child=format!("{R}/source/crates/zopro-core/src/solver/inverse_aliases.rs");let code=fs::read(format!("{R}/aliases.rs.in")).unwrap();fs::write(&child,&code).unwrap();pins.insert(child,hash(&code));assert_eq!(pins.len(),177);
 fs::write(format!("{R}/source-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 fs::write(format!("{R}/source-audit.json"),serde_json::to_vec_pretty(&json!({"originals":176,"unchanged":173,"changed":changed,"new_child":1,"field_files_changed":0})).unwrap()).unwrap();
 fs::write(format!("{R}/native-flags.txt"),m["rustflags"].as_str().unwrap()).unwrap();
 let mut env:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json").unwrap()).unwrap();
 env["RUSTFLAGS"]=m["rustflags"].clone();env["CARGO_BUILD_JOBS"]=json!("16");env["CARGO_TARGET_DIR"]=json!(format!("{R}/target-native"));env["RAYON_NUM_THREADS"]=json!("4");env["ZOPRO_CPU_INVERSE_ALIASES"]=json!("0");
 for name in ["setup.rs","setup","design.md","aliases.rs.in","source-audit.json","native-flags.txt"] {let p=format!("{R}/{name}");pins.insert(p.clone(),hash(&fs::read(p).unwrap()));}
 let tool="/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";pins.insert(tool.into(),hash(&fs::read(tool).unwrap()));
 fs::write(format!("{R}/qualification-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 let mut on=env.clone();on["ZOPRO_CPU_INVERSE_ALIASES"]=json!("1");
 let manifest=format!("{R}/source/Cargo.toml");
 fs::write(format!("{R}/qualification-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{R}/qualification-pins.json"),"steps":[
 {"kind":"command","name":"core","argv":["cargo","test","--offline","--locked","--release","--manifest-path",manifest,"-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"required_tests":["solver::inverse_aliases::tests::inline_layout_zero_scaling_cancellation_and_images","solver::inverse_aliases::tests::budget_invalid_and_cached_schedule_keep_original_ops","solver::inverse_aliases::tests::scheduled_caller_preserves_rows_and_source_readiness"],"timeout_seconds":1800},
 {"kind":"command","name":"prover-ON","argv":["cargo","test","--offline","--locked","--release","--manifest-path",manifest,"-p","zopro-prover","--lib","--","--test-threads=1"],"environment":on,"unfiltered":true,"timeout_seconds":1800}
 ]})).unwrap()).unwrap();
 println!("Prepared private177-file solver candidate;173 originals and all field files exact; core/prover-ON qualification pinned");
}
