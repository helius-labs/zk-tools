use std::{fs,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/cached-drain175";
const Q:&str="/home/ubuntu/src/zopro-perf-work/queue-drain175";
fn r(p:&str)->PathBuf{Path::new(R).join(p)}
fn change(s:&mut String,a:&str,b:&str){assert!(s.contains(a),"missing {a}");*s=s.replace(a,b);}
fn main(){
    assert!(!r("native.rs").exists());
    for file in["native.rs","sampling.rs","counter.rs"]{let source=fs::read_to_string(Path::new(Q).join(file)).unwrap();fs::write(r(file),source.replace(Q,R)).unwrap();}
    let mut setup=fs::read_to_string(Path::new(Q).join("setup.rs")).unwrap().replace(Q,R);
    let a=setup.find("    let parent=").unwrap();let b=setup.find("    let flags=").unwrap();
    setup.replace_range(a..b,r#"
    let mut changed=BTreeMap::new();let mut children=Vec::new();
    for (parent,files,new) in [
      ("/home/ubuntu/src/zopro-perf-work/cached-hash-sdk175",vec!["crates/zopro-core/src/msm_affine/grouping.rs","crates/zopro-core/Cargo.toml","Cargo.lock"],"crates/zopro-core/src/msm_affine/grouping/representative.rs"),
      ("/home/ubuntu/src/zopro-perf-work/queue-drain175",vec!["crates/zopro-core/src/msm_affine.rs","crates/zopro-core/src/msm_affine/borrowed_pending.rs","crates/zopro-core/src/msm_affine/g2_borrowed.rs"],"crates/zopro-core/src/msm_affine/queue_drain.rs")
    ]{
      let parent=Path::new(parent);let oldpins=read(parent.join("build-pins.json"));
      for p in files.into_iter().chain([new]){let from=parent.join("source").join(p);assert_eq!(hash(&from),oldpins[from.to_str().unwrap()].as_str().unwrap());pin(&mut pins,&from);fs::create_dir_all(r("source").join(p).parent().unwrap()).unwrap();fs::copy(&from,r("source").join(p)).unwrap();if m["source_files_sha256"].get(p).is_some(){assert!(changed.insert(p.to_string(),json!({"qualified_parent_source":from,"parent_hash":hash(&from)})).is_none());}else{children.push(p.to_string());}}
      for file in["source-audit.json","analysis.json"]{pin(&mut pins,parent.join(file));}
    }
    let(mut exact,mut fields)=(0,0);for(p,h)in m["source_files_sha256"].as_object().unwrap(){let file=r("source").join(p);if hash(&file)==h.as_str().unwrap(){exact+=1;}else{assert!(changed.contains_key(p));}if p.starts_with("crates/zopro-bn254/"){fields+=1;assert_eq!(hash(&file),h.as_str().unwrap());}pin(&mut pins,file);}assert_eq!(exact,169);assert_eq!(fields,17);assert_eq!(changed.len(),6);assert_eq!(children.len(),2);for p in &children{pin(&mut pins,r("source").join(p));}
    save(r("source-audit.json"),&json!({"source_files":177,"original_files":175,"originals_exact":169,"all17field_exact":true,"workspace_and_Window_fields_unchanged":true,"copied_parent_edits":changed,"new_children":children,"composition_only":true,"new_heap_or_key_state_vs_components":false,"parent_gates_remain_failed":true}));
"#);
    change(&mut setup,"{flags} --cfg zopro_queue_drain","{flags} --cfg zopro_queue_drain --cfg zopro_cached_hash_groups");
    change(&mut setup,"\"design.md\",\"history-audit.md\",\"queue_drain.rs\",\"g1-suffix.rs\",\"g2-suffix.rs\",\"setup.rs\"","\"design.md\",\"history-audit.md\",\"bootstrap.rs\",\"setup.rs\"");
    change(&mut setup,"Prepared10native steps;176private/172exact/reverse175;3newcoretests;field/workspace unchanged","Prepared10native steps;177private/169exact;127OFF/131ON core tests;exact composition;field/workspace unchanged");
    fs::write(r("setup.rs"),setup).unwrap();
    let mut capture=fs::read_to_string(Path::new(Q).join("capture.rs")).unwrap().replace(Q,R);
    change(&mut capture,"passed.len(),127","passed.len(),if label==\"control\"{127}else{131}");
    change(&mut capture,"/home/ubuntu/src/zopro-perf-work/component-partition175/proof-sha256-after.json","/home/ubuntu/src/zopro-perf-work/queue-drain175/proof-sha256-after.json");
    change(&mut capture,"22366","22498");
    change(&mut capture,"let cases=vec![all[0].clone(),physical.clone()];","let mut cases=all.as_array().unwrap().clone();cases.push(physical.clone());");
    change(&mut capture,"\"preflight\"=>1,\"latency\"=>151","\"preflight\"=>1,\"latency\"=>if c[\"shape\"]==\"1x2\"{151}else{31}");
    change(&mut capture,"let mut order=[0usize,1];order.rotate_left(b%2);","let mut order=(0usize..10).collect::<Vec<_>>();order.rotate_left(b%10);");
    change(&mut capture,"for c in &cases{for v in VS{add_job(c,v,0,panel","for c in cases.iter().filter(|c|c[\"shape\"]==\"1x2\"){for v in VS{add_job(c,v,0,panel");
    change(&mut capture,"jobs.len(),132","jobs.len(),492");change(&mut capture,"steps.len(),294","steps.len(),1014");change(&mut capture,"15708","27828");
    change(&mut capture,"\"summary-generator.rs\",\"summary-generator\",","");
    change(&mut capture,"Prepared294steps/27828SDK/132newGnark;15blocks two topologies","Prepared1014steps/27828SDK/492newGnark;15blocks all9w32+primary16");
    fs::write(r("capture.rs"),capture).unwrap();
    let mut analysis=fs::read_to_string(Path::new(Q).join("analysis.rs")).unwrap();change(&mut analysis,"22498","22990");change(&mut analysis,"22366","22498");change(&mut analysis,"15708","27828");change(&mut analysis,"\"new_independent_exports\":132","\"new_independent_exports\":492");
    change(&mut analysis,"else if c[\"workers\"]==16","else if c[\"name\"]!=\"1x2-w32\"");
    let at=analysis.find("    save(r(\"analysis.json\")").unwrap();
    analysis.insert_str(at,r#"
    let case_data=read(r("cases.json"));let original8=case_data.as_array().unwrap().iter().filter(|c|c["workers"]==32&&!c["reference"].is_null()).collect::<Vec<_>>();assert_eq!(original8.len(),8);let mut aggregates=BTreeMap::new();let mut comparisons=BTreeMap::new();
    for control in["accepted","OFF"]{let values=(0..15).map(|block|(original8.iter().map(|c|pairs[&format!("{}/ON/{control}",c["name"].as_str().unwrap())]["statistics"]["ratios"][block].as_f64().unwrap().ln()).sum::<f64>()/8.).exp()).collect::<Vec<_>>();let stats=interval(&values);let passed=stats["high"].as_f64().unwrap()<1.&&stats["wins"].as_u64().unwrap()>=10;if !passed{failed.push(format!("original8/ON/{control}/gain"));}aggregates.insert(control,json!({"statistics":stats,"passed":passed}));}
    for c in &original8{let name=c["name"].as_str().unwrap();let mut by_variant=BTreeMap::new();for v in VS{let ms=cells[&format!("{name}/{v}")]["median_ms"].as_f64().unwrap();let cpu=c["reference"]["cpu_ms"].as_f64().unwrap();let metal=c["reference"]["metal_ms"].as_f64().unwrap();by_variant.insert(v,json!({"ms":ms,"CPU_speedup":cpu/ms,"Metal_speedup":metal/ms,"CPU10x":ms<=cpu/10.}));}comparisons.insert(name,json!({"reference":c["reference"],"variants":by_variant}));}
"#);
    change(&mut analysis,"\"native_core_tests_each\":127","\"native_core_tests\":{\"OFF\":127,\"ON\":131}");
    change(&mut analysis,"\"source_files\":176,\"originals_exact\":172","\"source_files\":177,\"originals_exact\":169");
    change(&mut analysis,"\"cells\":cells","\"original8_aggregates\":aggregates,\"new_reference_comparisons\":comparisons,\"cells\":cells");
    change(&mut analysis,"Pilot only; all9 latency/resource/portable qualification still required on success","All9w32 andprimary16 latency;resource/portable/finalartifact qualification still required on success");
    change(&mut analysis,"Analyzed pilot; ledger22990","Analyzed combined all9+physical16 SDK; ledger22990");
    fs::write(r("analysis.rs"),analysis).unwrap();
    println!("Generated exact two-component composition and prospective 1014-step capture; no source change outside six disjoint parent edits plus two children");
}
