use std::{fs,path::Path};use serde_json::{json,Value};
const R:&str="/tmp/zopro-prefix-stages";
fn edit(rel:&str,old:&str,new:&str){let p=Path::new(R).join("candidate").join(rel);let s=fs::read_to_string(&p).unwrap();let start=if s.matches(old).count()!=1&&rel=="crates/zopro-core/src/lib.rs"{s.find("    fn solve_canonical_wires(").unwrap()}else{0};assert_eq!(s[start..].matches(old).count(),1,"{rel}: {old}");let mut out=s[..start].to_owned();out.push_str(&s[start..].replacen(old,new,1));fs::write(p,out).unwrap();}
fn wrap(rel:&str,old:&str,label:u64,n:&str){edit(rel,old,&format!("{{ let _stage=crate::stage_costs::Span::enter({label},{n}); {old} }}"));}
fn main(){
 let lib="crates/zopro-core/src/lib.rs";
 edit(lib,"mod preparation;","mod preparation;\n#[cfg(all(zopro_stage_costs,target_os=\"linux\"))]\npub mod stage_costs;");
 edit(lib,"    fn solve_inner(&self, witness: &[u8], omit_b: bool) -> Result<Vec<u8>, String> {","    fn solve_inner(&self, witness: &[u8], omit_b: bool) -> Result<Vec<u8>, String> {\n        let _entry=crate::stage_costs::Span::enter(if rayon::current_thread_index().is_none(){6}else{7},witness.len());");
 wrap(lib,"plan.solve_in_with_slots(witness,self.domain,&folded.slots,materialize_c)?",8,"self.domain");
 wrap(lib,"folded.public_term(&w[..plan.n_public])",9,"plan.n_public");
 wrap(lib,"canonical(&mut w)",10,"w.len()");
 wrap(lib,"rayon::join(||plan.a_indices.par_iter().map(|&i|words[i as usize]).collect(),||plan.b_indices.par_iter().map(|&i|words[i as usize]).collect())",11,"words.len()");
 wrap(lib,"queries.product_values(&self.plan,a,b)",12,"self.domain");
 edit(lib,"            let ((ar,kh),bs2)=rayon::join(","            let query_span=crate::stage_costs::Span::enter(13,k.len());\n            let ((ar,kh),bs2)=rayon::join(");
 wrap(lib,"msm(&self.a[..],sa)",27,"sa.len()");
 wrap(lib,"msm_affine::fused_kh::fused(k,sk,z,canonical(h),13)",28,"sk.len()");
 {
 let p=Path::new(R).join("candidate").join(lib);let s=fs::read_to_string(&p).unwrap();let start=s.find("    fn solve_canonical_wires(").unwrap();let old="msm_affine::g2_query_exclusive(&self.b2,sb)";assert_eq!(s[start..].matches(old).count(),2);let mut out=s[..start].to_owned();out.push_str(&s[start..].replace(old,"{ let _stage=crate::stage_costs::Span::enter(29,sb.len()); msm_affine::g2_query_exclusive(&self.b2,sb) }"));fs::write(p,out).unwrap();
 }
 edit(lib,"            let mut out=Vec::with_capacity(384);","            drop(query_span);\n            let _encode=crate::stage_costs::Span::enter(14,4);\n            let mut out=Vec::with_capacity(384);");
 edit(lib,"        let (((ar,kr),hz),(bs1,bs2))={","        let ordinary_query_span=crate::stage_costs::Span::enter(13,k.len());\n        let (((ar,kr),hz),(bs1,bs2))={");
 edit(lib,"        let mut out=Vec::with_capacity(384);for point in [ar,bs1,kr+correction,queries.finish_product(hz)]", "        drop(ordinary_query_span);\n        let _encode=crate::stage_costs::Span::enter(14,4);\n        let mut out=Vec::with_capacity(384);for point in [ar,bs1,kr+correction,queries.finish_product(hz)]");
 wrap(lib,"fixed.assemble_rerandomized(&parts)",15,"parts.len()");
 let pool="crates/zopro-core/src/prefix_pool.rs";
 edit(pool,"    let prepared = pool.workers.install(|| {","    let install_span=crate::stage_costs::Span::enter(25,key.domain);\n    let prepared = pool.workers.install(|| {\n        let _worker=crate::stage_costs::Span::enter(23,key.domain);");
 wrap(pool,"plan.solve_in_with_slots(witness,key.domain,&folded.slots,materialize_c)?",8,"key.domain");
 wrap(pool,"folded.public_term(&w[..plan.n_public])",9,"plan.n_public");
 wrap(pool,"msm_affine::canonical(&mut w)",10,"w.len()");
 edit(pool,"    let (a,b) = rayon::join(","    let gather_span=crate::stage_costs::Span::enter(11,words.len());\n    let (a,b) = rayon::join(");
 edit(pool,"    (w,a,b)","    drop(gather_span);\n    (w,a,b)");
 wrap(pool,"queries.product_values(&key.plan,a,b)",12,"key.domain");
 edit(pool,"    })?;","    })?;\n    drop(install_span);");
 edit(pool,"    rayon::join(|| {","    let _math_join=crate::stage_costs::Span::enter(26,key.domain);\n    rayon::join(|| {\n        let _worker=crate::stage_costs::Span::enter(24,key.domain);");
 edit(pool,"        let ((ar,kh),bs2) = rayon::join(","        let query_span=crate::stage_costs::Span::enter(13,folded.private.len());\n        let ((ar,kh),bs2) = rayon::join(");
 wrap(pool,"msm_affine::msm_canonical(&key.a,&a)",27,"a.len()");
 wrap(pool,"msm_affine::fused_kh::fused(&folded.private,sk,z,msm_affine::canonical(&mut product),13)",28,"sk.len()");
 wrap(pool,"msm_affine::g2_query_exclusive(&key.b2,&mut b)",29,"b.len()");
 edit(pool,"        let mut out = Vec::with_capacity(384);","        drop(query_span);\n        let _encode=crate::stage_costs::Span::enter(14,4);\n        let mut out = Vec::with_capacity(384);");
 let recorder="crates/zopro-core/src/stage_costs.rs";
 edit(recorder,"22=>\"g2.arithmetic\", _=>\"test\"", "22=>\"g2.arithmetic\", 23=>\"core.prefix_worker\", 24=>\"core.math_worker\", 25=>\"core.prefix_install\", 26=>\"core.math_join\", 27=>\"core.A_query\", 28=>\"core.KH_query\", 29=>\"core.B2_query\", _=>\"test\"");
 let mut plan:Value=serde_json::from_slice(&fs::read("/tmp/zopro-prefix-pool/native-plan.json").unwrap()).unwrap();
 let s=serde_json::to_string(&plan).unwrap().replace("/tmp/zopro-prefix-pool",R).replace("--cfg zopro_prefix_pool","--cfg zopro_prefix_pool --cfg zopro_stage_costs");plan=serde_json::from_str(&s).unwrap();plan["source_pins"]=json!(format!("{R}/source-pins.json"));fs::write(format!("{R}/native-plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
}
