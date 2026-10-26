use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,collections::BTreeMap};
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(b:impl AsRef<[u8]>)->String {format!("{:x}",Sha256::digest(b.as_ref()))}
fn edit(s:&mut String,old:&str,new:&str,file:&str,edits:&mut Vec<Value>) {assert_eq!(s.matches(old).count(),1,"anchor {file}: {old}");*s=s.replacen(old,new,1);edits.push(json!({"file":file,"before":old,"after":new}));}
fn main(){
 let root=Path::new("/tmp/zopro-ready-arithmetic");let dir=root.join("candidate");
 let m=read("/tmp/zopro-grouping-module/production-source-manifest.json");
 for(p,h)in m["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(fs::read(dir.join(p)).unwrap()),h.as_str().unwrap());assert_eq!(hash(fs::read(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)).unwrap()),h.as_str().unwrap());}
 let mut edits=Vec::new();let cfg="#[cfg(zopro_ready_arithmetic)]";
 let f="crates/zopro-core/src/lib.rs";let mut s=fs::read_to_string(dir.join(f)).unwrap();
 edit(&mut s,"pub mod alloc;",&format!("pub mod alloc;\n{cfg}\npub mod task_ready;"),f,&mut edits);fs::write(dir.join(f),s).unwrap();
 let f="crates/zopro-core/src/msm_affine.rs";let mut s=fs::read_to_string(dir.join(f)).unwrap();
 edit(&mut s,"    let pieces: Vec<Projective<C>> = (0..windows * parts)",&format!("    {cfg}\n    let ready = crate::task_ready::Batch::new(crate::task_ready::family::<C>(1,3),windows*parts);\n    let pieces: Vec<Projective<C>> = (0..windows * parts)"),f,&mut edits);
 edit(&mut s,"            {\n                return sum;\n            }\n            window::<C>(",&format!("            {{\n                {cfg} ready.cancel(task);\n                return sum;\n            }}\n            window::<C>("),f,&mut edits);
 edit(&mut s,"                canonical[range].iter().map(|s| digit(s, w, width)),\n                width,\n            )",&format!("                canonical[range].iter().map(|s| digit(s, w, width)),\n                width,\n                {cfg} (&ready,task),\n            )"),f,&mut edits);
 edit(&mut s,"fn window<C: CurveConfig>(\n    bases: &[Affine<C>],\n    digits: impl Iterator<Item = i16>,\n    width: usize,\n)",&format!("fn window<C: CurveConfig>(\n    bases: &[Affine<C>],\n    digits: impl Iterator<Item = i16>,\n    width: usize,\n    {cfg} ready: (&crate::task_ready::Batch,usize),\n)"),f,&mut edits);
 edit(&mut s,"    let half = 1usize << (width - 1);\n    let size = batch_size(width);",&format!("    {cfg} let leaf=ready.0.start(ready.1);\n    let half = 1usize << (width - 1);\n    let size = batch_size(width);"),f,&mut edits);
 edit(&mut s,"    let w = fill_window::<C>(bases, digits, width, size);",&format!("    let w = fill_window::<C>(bases, digits, width, size);\n    {cfg} drop(leaf);"),f,&mut edits);
 let start=s.find("fn merge_split_width<").unwrap();let mut tail=s[start..].to_owned();let mut local=Vec::new();
 edit(&mut tail,"    let sums: Vec<_> = (0..windows)",&format!("    {cfg}\n    let ready=crate::task_ready::Batch::new(crate::task_ready::family::<C>(1,3),windows*parts);\n    let sums: Vec<_> = (0..windows)"),f,&mut local);
 edit(&mut tail,"            if let Some(sum) = top_window::<C, true>(bases, scalars, w, width) {\n                return sum;",&format!("            if let Some(sum) = top_window::<C, true>(bases, scalars, w, width) {{\n                {cfg} for p in 0..parts {{ready.cancel(w*parts+p);}}\n                return sum;"),f,&mut local);
 edit(&mut tail,"                .map(|p| {\n                    let range",&format!("                .map(|p| {{\n                    {cfg} let _leaf=ready.start(w*parts+p);\n                    let range"),f,&mut local);
 let oldtail=s[start..].to_owned();s.replace_range(start..,&tail);edits.push(json!({"file":f,"before":oldtail,"after":tail}));fs::write(dir.join(f),s).unwrap();
 let f="crates/zopro-core/src/msm_affine/fused_kh.rs";let mut s=fs::read_to_string(dir.join(f)).unwrap();
 edit(&mut s," let sums:Vec<_>=(0..windows)",&format!(" {cfg} let ready=crate::task_ready::Batch::new(2,windows);\n let sums:Vec<_>=(0..windows)"),f,&mut edits);
 edit(&mut s,"   if let(Some(a),Some(b))=(a,b){return a+b;}",&format!("   if let(Some(a),Some(b))=(a,b){{{cfg} ready.cancel(w);return a+b;}}"),f,&mut edits);
 edit(&mut s,"  let mut state=Window::<G1Config>::new",&format!("  {cfg} let leaf=ready.start(w);\n  let mut state=Window::<G1Config>::new"),f,&mut edits);
 edit(&mut s,"  crate::msm::weighted_affine(&affine_buckets(state))",&format!("  let buckets=affine_buckets(state);\n  {cfg} drop(leaf);\n  crate::msm::weighted_affine(&buckets)"),f,&mut edits);fs::write(dir.join(f),s).unwrap();
 let f="crates/zopro-core/src/msm_affine/g2_phi.rs";let mut s=fs::read_to_string(dir.join(f)).unwrap();
 edit(&mut s,"    let windows = 127usize.div_ceil(bits);\n    let sums:",&format!("    let windows = 127usize.div_ceil(bits);\n    {cfg} let ready=crate::task_ready::Batch::new(3,windows*parts);\n    let sums:"),f,&mut edits);
 edit(&mut s,"                .map(|part| {\n                    let start",&format!("                .map(|part| {{\n                    {cfg} let _leaf=if w+1==windows && 126-w*bits<=8 {{ready.cancel(w*parts+part);None}} else {{Some(ready.start(w*parts+part))}};\n                    let start"),f,&mut edits);fs::write(dir.join(f),s).unwrap();
 let f="crates/zopro-core/src/msm.rs";let mut s=fs::read_to_string(dir.join(f)).unwrap();
 edit(&mut s,"    result.spare_capacity_mut()[..n]\n        .par_chunks_mut(GROUP)",&format!("    {cfg} let ready=crate::task_ready::Batch::new(crate::task_ready::family::<C>(8,9),n.div_ceil(GROUP));\n    result.spare_capacity_mut()[..n]\n        .par_chunks_mut(GROUP)"),f,&mut edits);
 edit(&mut s,"            |(points, scratch), (i, output)| {\n                let first",&format!("            |(points, scratch), (i, output)| {{\n                {cfg} let _leaf=ready.start(i);\n                let first"),f,&mut edits);
 edit(&mut s,"    let partials: Vec<Affine<C>> = tasks\n        .into_par_iter()\n        .map(|(first, end)| {",&format!("    {cfg} let ready=crate::task_ready::Batch::new(crate::task_ready::family::<C>(6,7),tasks.len());\n    let partials: Vec<Affine<C>> = tasks\n        .into_par_iter()\n        .enumerate()\n        .map(|(task,(first, end))| {{\n            {cfg} let _leaf=ready.start(task);"),f,&mut edits);
 edit(&mut s,"    let partials: Vec<_> = buckets\n        .par_chunks(GROUP)\n        .map_init(",&format!("    {cfg} let ready=crate::task_ready::Batch::new(crate::task_ready::family::<C>(4,5),buckets.len().div_ceil(GROUP));\n    let partials: Vec<_> = buckets\n        .par_chunks(GROUP)\n        .enumerate()\n        .map_init("),f,&mut edits);
 edit(&mut s,"            |(tree, scratch), buckets| {\n                let n",&format!("            |(tree, scratch), (task,buckets)| {{\n                {cfg} let _leaf=ready.start(task);\n                let n"),f,&mut edits);fs::write(dir.join(f),s).unwrap();
 fs::copy(root.join("task_ready.rs.txt"),dir.join("crates/zopro-core/src/task_ready.rs")).unwrap();
 fs::create_dir_all(dir.join("crates/zopro-bench/examples/support")).unwrap();
 fs::copy("/tmp/zopro-g2-owned-finish/candidate/crates/zopro-bench/examples/support/perf.rs",dir.join("crates/zopro-bench/examples/support/perf.rs")).unwrap();
 let mut sdk=fs::read_to_string("/tmp/zopro-g2-owned-finish/candidate/crates/zopro-bench/examples/cpu-sdk.rs").unwrap();
 sdk=sdk.replace("    for _ in 0..3 {","    zopro_prover::zopro_core::task_ready::initialize();\n    let trace=std::env::var_os(\"ZOPRO_READY_OUTPUT\");\n    let mut observations=Vec::with_capacity(rounds);\n    for _ in 0..3 {");
 sdk=sdk.replace("    for _ in 0..rounds {\n        let started", "    for _ in 0..rounds {\n        let session=trace.as_ref().map(|_|zopro_prover::zopro_core::task_ready::Session::begin());\n        let started");
 sdk=sdk.replace("        last = Some(proof);", "        if let Some(session)=session {observations.push(session.finish().map_err(std::io::Error::other)?);}\n        last = Some(proof);");
 sdk=sdk.replace("    for (round, ms) in samples.into_iter().enumerate() {", "    if let Some(path)=trace {let records:Vec<_>=observations.into_iter().enumerate().map(|(round,events)|serde_json::json!({\"round\":round,\"events\":events.into_iter().map(|e|serde_json::json!({\"id\":e.id,\"ready\":e.ready,\"start\":e.start,\"end\":e.end,\"kind\":e.kind,\"worker\":e.worker,\"state\":e.state,\"batch\":e.batch,\"ordinal\":e.ordinal})).collect::<Vec<_>>()})).collect();fs::write(path,serde_json::to_vec(&records)?)?;}\n    for (round, ms) in samples.into_iter().enumerate() {");
 sdk=sdk.replace("    summary[\"priming\"] = 3.into();", "    summary[\"priming\"] = 3.into();\n    summary[\"recorder_storage_bytes\"] = zopro_prover::zopro_core::task_ready::storage_bytes().into();");
 fs::write(dir.join("crates/zopro-bench/examples/cpu-sdk.rs"),sdk).unwrap();
 // Recover every original file byte-for-byte, not just individual anchors.
 for(p,h)in m["source_files_sha256"].as_object().unwrap(){let mut bytes=fs::read_to_string(dir.join(p)).ok();if let Some(s)=bytes.as_mut(){for e in edits.iter().rev().filter(|e|e["file"]==*p){let a=e["after"].as_str().unwrap();assert_eq!(s.matches(a).count(),1);*s=s.replacen(a,e["before"].as_str().unwrap(),1);}assert_eq!(hash(s.as_bytes()),h.as_str().unwrap(),"inverse {p}");}else{assert_eq!(hash(fs::read(dir.join(p)).unwrap()),h.as_str().unwrap());}}
 fs::write(root.join("edits.json"),serde_json::to_vec_pretty(&edits).unwrap()).unwrap();
 let mut pins=BTreeMap::new();for(p,_)in m["source_files_sha256"].as_object().unwrap(){pins.insert(dir.join(p).to_str().unwrap().to_owned(),hash(fs::read(dir.join(p)).unwrap()));}
 for p in ["crates/zopro-core/src/task_ready.rs","crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{pins.insert(dir.join(p).to_str().unwrap().to_owned(),hash(fs::read(dir.join(p)).unwrap()));}
 for p in ["design.md","implement.rs","task_ready.rs.txt","edits.json"] {pins.insert(root.join(p).to_str().unwrap().to_owned(),hash(fs::read(root.join(p)).unwrap()));}
 fs::write(root.join("source-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 let parent=read("/tmp/zopro-g2-owned-finish/native-plan.json");let mut plan=parent.clone();
 plan["source_pins"]=json!(root.join("source-pins.json"));
 let mut steps=Vec::new();for mut step in parent["steps"].as_array().unwrap().iter().filter(|s|!s["name"].as_str().unwrap().starts_with("off")).cloned(){
    step=serde_json::from_str(&serde_json::to_string(&step).unwrap().replace("/tmp/zopro-g2-owned-finish","/tmp/zopro-ready-arithmetic").replace("zopro_g2_owned_finish","zopro_ready_arithmetic")).unwrap();
    if step["name"]=="on-core" {step["required_tests"]=json!(["task_ready::tests::parallel_units_are_unique_and_ordered","task_ready::tests::cancellation_is_distinct_from_execution","task_ready::tests::exhaustion_is_reported_without_out_of_bounds_writes","task_ready::tests::panic_disarms_and_allows_reuse"]);}
    steps.push(step);
 }
 plan["steps"]=json!(steps);
 fs::write(root.join("native-plan.json"),serde_json::to_vec_pretty(&plan).unwrap()).unwrap();
 println!("Prepared private arithmetic-readiness recorder; 170 originals recover exactly; {} pins",pins.len());
}
