fn setup(){
assert!(!r("build-plan.json").exists());let parent=Path::new("/home/ubuntu/src/zopro-perf-work/swiss-groups175");let m=read(MANIFEST);assert_eq!(read(parent.join("analysis.json"))["component_gate"],true);
let mut pins=BTreeMap::new();for(f,h)in read(parent.join("pins.json")).as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());pins.insert(f.clone(),h.as_str().unwrap().to_string());}
for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());let dest=r("source").join(f);fs::create_dir_all(dest.parent().unwrap()).unwrap();fs::copy(p,dest).unwrap();}
let file="crates/zopro-core/src/msm_affine/grouping.rs";let path=r("source").join(file);let old=fs::read_to_string(&path).unwrap();
let map="    let mut groups =\n        std::collections::HashMap::with_capacity_and_hasher(capacity, ahash::RandomState::new());";
let entry="        let index = *groups.entry(*scalar).or_insert_with(|| {\n            values.push(*scalar);\n            counts.push(0);\n            (values.len() - 1) as u32\n        });";
assert_eq!(old.matches(map).count(),1);assert_eq!(old.matches(entry).count(),1);
let new_map=format!("    #[cfg(zopro_swiss_groups)]\n    let mut groups = index_map::IndexMap::new(capacity, ahash::RandomState::new());\n    #[cfg(not(zopro_swiss_groups))]\n{map}");
let new_entry=format!("        #[cfg(not(zopro_swiss_groups))]\n{entry}\n        #[cfg(zopro_swiss_groups)]\n        let index = {{ let (id, fresh) = groups.insert(scalar, &mut values); if fresh {{ counts.push(0); }} id }};");
let suffix="\n#[cfg(zopro_swiss_groups)]\nmod index_map;\n";let new=old.replace(map,&new_map).replace(entry,&new_entry)+suffix;
let inverse=new.strip_suffix(suffix).unwrap().replace(&new_map,map).replace(&new_entry,entry);assert_eq!(inverse,old);fs::write(&path,new).unwrap();
let child="crates/zopro-core/src/msm_affine/grouping/index_map.rs";let runtime=fs::read_to_string(parent.join("index_map.rs")).unwrap();let test=r#"
#[cfg(test)]
mod tests {
 use super::*;
 use std::hash::Hasher;
 struct Constant;
 struct ConstantHasher;
 impl Hasher for ConstantHasher { fn write(&mut self,_:&[u8]){} fn finish(&self)->u64{u64::MAX} }
 impl BuildHasher for Constant { type Hasher=ConstantHasher; fn build_hasher(&self)->ConstantHasher{ConstantHasher} }
 #[test] fn forced_collision_growth_and_complete_key_equality() {
  for capacity in [0,1,3,17,128] {
   let mut table=IndexMap::new(capacity,Constant);let mut values=Vec::new();
   for i in 0..1025u64 { let v=[7,i/3,i%3,9];assert_eq!(table.insert(&v,&mut values),(i as u32,true));assert_eq!(table.insert(&v,&mut values),(i as u32,false)); }
   for i in (0..1025u64).rev(){assert_eq!(table.insert(&[7,i/3,i%3,9],&mut values),(i as u32,false));}
   assert_eq!(values.len(),1025);
  }
 }
}
"#;
fs::write(r("source").join(child),format!("{runtime}{test}")).unwrap();
let cargo="crates/zopro-core/Cargo.toml";let lock="Cargo.lock";let cp=r("source").join(cargo);let c=fs::read_to_string(&cp).unwrap();let line="hashbrown = { version = \"=0.16.1\", default-features = false, features = [\"inline-more\"] }\n";let needle="[target.'cfg(not(target_arch = \"wasm32\"))'.dependencies]\n";assert_eq!(c.matches(needle).count(),1);let cnew=c.replace(needle,&format!("{needle}{line}"));assert_eq!(cnew.replace(line,""),c);fs::write(cp,cnew).unwrap();
let lp=r("source").join(lock);let l=fs::read_to_string(&lp).unwrap();let needle="name = \"zopro-core\"\nversion = \"0.1.0\"\ndependencies = [\n \"ahash\",\n \"futures-channel\",\n \"getrandom 0.2.17\",\n";assert_eq!(l.matches(needle).count(),1);let added=format!("{needle} \"hashbrown 0.16.1\",\n");let lnew=l.replace(needle,&added);assert_eq!(lnew.replace(&added,needle),l);fs::write(lp,lnew).unwrap();
let mut exact=0;let mut field=0;for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=r("source").join(f);if hash(&p)==h.as_str().unwrap(){exact+=1;}else{assert!([file,cargo,lock].contains(&f.as_str()));}if f.starts_with("crates/zopro-bn254/"){assert_eq!(hash(&p),h.as_str().unwrap());field+=1;}pins.insert(p.display().to_string(),hash(p));}assert_eq!(exact,172);assert_eq!(field,17);pins.insert(r("source").join(child).display().to_string(),hash(r("source").join(child)));
let mut steps=vec![];for(label,on)in[("control",false),("candidate",true)]{let flags=if on{format!("{} --cfg zopro_swiss_groups",m["rustflags"].as_str().unwrap())}else{m["rustflags"].as_str().unwrap().into()};fs::write(r(format!("native-flags-{label}.txt")),&flags).unwrap();let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(flags);env["CARGO_TARGET_DIR"]=json!(r(format!("target-{label}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");steps.push(json!({"kind":"command","name":format!("core-{label}"),"argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));steps.push(json!({"kind":"command","name":format!("prover-{label}"),"argv":["cargo","test","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));}
save(r("source-audit.json"),&json!({"parent":"swiss-groups175","parent_component_gate":true,"originals":175,"originals_exact":172,"private_files":176,"modified_originals":[file,cargo,lock],"new_child":child,"field_original_files_exact":17,"field_tests_inherited":33,"not_arithmetic_binary_identity_claim":true,"parent_runtime_prefix_exact":true,"edits":{file:{"original_map":map,"candidate_map":new_map,"original_entry":entry,"candidate_entry":new_entry,"suffix":suffix},cargo:{"inserted_line":line},lock:{"added_core_dependency":"hashbrown 0.16.1"}},"production_changed":false}));
for f in["bootstrap.rs","setup.rs","native.rs","native-helper","design.md","source-audit.json","native-flags-control.txt","native-flags-candidate.txt"]{pins.insert(r(f).display().to_string(),hash(r(f)));}pins.insert(parent.join("analysis.json").display().to_string(),hash(parent.join("analysis.json")));pins.insert(MANIFEST.into(),hash(MANIFEST));save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared10 native SDK steps;176privatefiles172originalsexact, field17 unchanged;parentcomponentpositive");
}
