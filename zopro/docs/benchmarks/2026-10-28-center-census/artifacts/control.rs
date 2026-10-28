use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/center-census";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const IMAGE:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-25-worker-groups-request-entry/qualification/warm-default-w32-single-request/input/transfer_confidential_1_2.image";
const IH:&str="16a3e2a686372d531879d4ff99e63eab0ca374d686f05a9d7343d2bf534dd041";
const PH:&str="3f0d846c3905d11595d77dd988df829ddc51325eda2b93e4db8f0e17a5928e5b";
const V:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pin(m:&mut BTreeMap<String,String>,p:impl AsRef<Path>){let p=p.as_ref();m.insert(p.display().to_string(),hash(p));}
fn manifest()->Value{read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"))}
fn setup(){
    let r=Path::new(R);assert!(!r.join("source").exists());let m=manifest();
    for(file,h)in m["source_files_sha256"].as_object().unwrap(){let from=Path::new(Z).join(file);assert_eq!(hash(&from),h.as_str().unwrap());let to=r.join("source").join(file);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
    let lib="crates/zopro-core/src/lib.rs";let pending="crates/zopro-core/src/msm_affine/borrowed_pending.rs";
    let mut edits=BTreeMap::<String,Vec<(String,String)>>::new();
    edits.insert(lib.into(),vec![(String::new(),"\n#[cfg(zopro_msm_geometry)]\npub mod center_census;\n".into())]);
    edits.insert(pending.into(),vec![
        ("    assert_eq!(k.len(), sk.len());\n".into(),"    #[cfg(zopro_msm_geometry)]\n    crate::center_census::fused(k,sk,h,sh,bits);\n    assert_eq!(k.len(), sk.len());\n".into()),
        ("    assert_eq!(bases.len(), scalars.len());\n".into(),"    #[cfg(zopro_msm_geometry)]\n    crate::center_census::ordinary(bases,scalars,parts,width);\n    assert_eq!(bases.len(), scalars.len());\n".into())]);
    for(file,list)in &edits{let path=r.join("source").join(file);let original=fs::read_to_string(&path).unwrap();let mut next=original.clone();for(a,b)in list{if a.is_empty(){next.push_str(b);}else{assert_eq!(next.matches(a).count(),1);next=next.replacen(a,b,1);}}
        let mut back=next.clone();for(a,b)in list.iter().rev(){assert_eq!(back.matches(b).count(),1);back=back.replacen(b,a,1);}assert_eq!(back,original);fs::write(path,next).unwrap();}
    fs::copy(r.join("observer.rs"),r.join("source/crates/zopro-core/src/center_census.rs")).unwrap();save(r.join("inverse-edits.json"),&json!(edits));
    let source=fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm_affine.rs")).unwrap();let observer=fs::read_to_string(r.join("observer.rs")).unwrap();let start=source.find("fn booth_digit(").unwrap();let end=source[start..].find("\n}\n").unwrap()+start+3;assert!(observer.contains(&source[start..end]));
    let pins=source_pins();save(r.join("build-pins.json"),&json!(pins));
    let flags=format!("{} --cfg zopro_msm_geometry",m["rustflags"].as_str().unwrap());
    let plan=json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":{"CARGO_BUILD_JOBS":"16","CARGO_TARGET_DIR":r.join("target"),"RUSTFLAGS":flags},"steps":[
        {"kind":"command","name":"libraries","argv":["cargo","build","--manifest-path",r.join("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"timeout_seconds":1200},
        {"kind":"link-sdk","name":"observer","artifacts":r.join("native/libraries/stdout.log"),"driver":r.join("driver.rs"),"timeout_seconds":600}]});
    save(r.join("build-plan.json"),&plan);println!("Prepared:172 production exact,170 private originals unchanged,2 reversible files plus observer");
}
fn source_pins()->BTreeMap<String,String>{
    let r=Path::new(R);let m=manifest();let edits=read(r.join("inverse-edits.json"));let mut pins=BTreeMap::new();let mut same=0;
    for(file,h)in m["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(file);let private=r.join("source").join(file);assert_eq!(hash(&p),h.as_str().unwrap());pin(&mut pins,&p);pin(&mut pins,&private);
        if let Some(list)=edits.get(file){let mut back=fs::read_to_string(&private).unwrap();for edit in list.as_array().unwrap().iter().rev(){let a=edit[0].as_str().unwrap();let b=edit[1].as_str().unwrap();assert_eq!(back.matches(b).count(),1);back=back.replacen(b,a,1);}assert_eq!(back,fs::read_to_string(&p).unwrap());}else{assert_eq!(hash(&private),h.as_str().unwrap());same+=1;}}
    assert_eq!(same,170);assert_eq!(hash(r.join("observer.rs")),hash(r.join("source/crates/zopro-core/src/center_census.rs")));
    for f in ["observer.rs","control.rs","driver.rs","design.md"]{pin(&mut pins,r.join(f));}pin(&mut pins,X);pin(&mut pins,r.join("source/crates/zopro-core/src/center_census.rs"));pins
}
fn prepare(){
    let r=Path::new(R);let mut pins=source_pins();let a=read(r.join("native/observer/artifacts.json"));assert_eq!(hash(r.join("native/observer-sdk")),a["binary_sha256"].as_str().unwrap());
    for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}pin(&mut pins,r.join("native/observer-sdk"));pin(&mut pins,r.join("native/observer/artifacts.json"));
    assert_eq!(hash(IMAGE),IH);pin(&mut pins,IMAGE);let fixture=format!("{Z}/tools/zolana/fixtures/transfer-1x2.json");pin(&mut pins,&fixture);pin(&mut pins,V);
    let old=read("/home/ubuntu/src/zopro-perf-work/msm-geometry/run-plan.json");let env=old["environment"].clone();let mut steps=vec![];
    for i in 0..3{steps.push(json!({"kind":"command","name":format!("observe-{i}"),"argv":["taskset","-c","0-31",r.join("native/observer-sdk").to_str().unwrap(),IMAGE,&fixture,r.join(format!("proofs/{i}")).to_str().unwrap()],"timeout_seconds":180}));}
    for i in 0..3{steps.push(json!({"kind":"command","name":format!("verify-{i}"),"argv":[V,IMAGE,IH,r.join(format!("proofs/{i}/0.proof")).to_str().unwrap(),r.join(format!("proofs/{i}/0.public")).to_str().unwrap(),PH],"timeout_seconds":120}));}
    let mut prior:BTreeSet<String>=read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/prior-proof-sha256.json").as_array().unwrap().iter().map(|v|v.as_str().unwrap().into()).collect();
    let inventory=read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/proof-inventory.json");for(p,h)in inventory.as_object().unwrap(){if p.ends_with(".proof"){assert_eq!(hash(p),h.as_str().unwrap());assert!(prior.insert(h.as_str().unwrap().into()));}}
    assert_eq!(prior.len(),12580);save(r.join("prior-proof-sha256.json"),&json!(prior));pin(&mut pins,r.join("prior-proof-sha256.json"));
    pin(&mut pins,r.join("observer-tests"));pin(&mut pins,r.join("observer-tests.log"));save(r.join("pins.json"),&json!(pins));save(r.join("run-plan.json"),&json!({"version":1,"source_pins":r.join("pins.json"),"environment":env,"steps":steps}));println!("Prepared3 observed proofs after3primes each, then3independent verifications;prior12580");
}
fn finish(){
    let r=Path::new(R);let pins=read(r.join("pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let completed=read(r.join("runs/result.json"));assert_eq!(completed["status"],"complete");assert_eq!(completed["steps"].as_array().unwrap().len(),6);
    let mut maps=vec![];let mut proof_hashes:BTreeSet<String>=read(r.join("prior-proof-sha256.json")).as_array().unwrap().iter().map(|v|v.as_str().unwrap().into()).collect();let mut inventory=BTreeMap::new();
    for i in 0..3{let stdout=fs::read_to_string(r.join(format!("runs/observe-{i}/stdout.log"))).unwrap();let census=stdout.lines().find_map(|s|s.strip_prefix("CENSUS ")).unwrap();let map:BTreeMap<String,u64>=serde_json::from_str(census).unwrap();assert!(!map.is_empty());maps.push(map);
        let summary:Value=serde_json::from_str(stdout.lines().find_map(|s|s.strip_prefix("SUMMARY ")).unwrap()).unwrap();assert_eq!(summary["SDK_requests"],4);assert_eq!(summary["internally_verified"],true);assert_eq!(summary["workers"],32);
        for ext in ["proof","public"]{let p=r.join(format!("proofs/{i}/0.{ext}"));let h=hash(&p);if ext=="proof"{assert_eq!(fs::metadata(&p).unwrap().len(),164);assert!(proof_hashes.insert(h.clone()));}inventory.insert(p.display().to_string(),h);}}
    assert_eq!(maps[0],maps[1]);assert_eq!(maps[0],maps[2]);save(r.join("census.json"),&json!(maps[0]));save(r.join("proof-inventory.json"),&json!(inventory));
    let prefixes:BTreeSet<_>=maps[0].keys().filter_map(|k|k.strip_suffix("/calls")).collect();let mut rows=vec![];
    for prefix in prefixes{let m=&maps[0];assert_eq!(m[&format!("{prefix}/calls")],1);let mut row=json!({"stream":prefix,"inputs":m[&format!("{prefix}/inputs")],"active_inputs":m[&format!("{prefix}/active_inputs")],"centered_signs":m[&format!("{prefix}/centered_signs")],"small_original":m[&format!("{prefix}/small_original")],"small_centered":m[&format!("{prefix}/small_centered")]});
        for kind in ["original","centered"]{let mut totals=BTreeMap::<String,u64>::new();for(k,v)in m{if !k.starts_with(&format!("{prefix}/w=")){continue;}let parts:Vec<_>=k.split('/').collect();if parts[parts.len()-2]!=kind{continue;}let metric=parts.last().unwrap();*totals.entry((*metric).into()).or_default()+=v;}
            assert_eq!(totals["ideal_insert"]+totals["occupied"],totals["terms"]);row[kind]=json!(totals);}rows.push(row);}
    assert_eq!(rows.len(),4,"expected ordinary plusK,H,combined");
    // Term counts are additive across the fused segment boundary; occupancy is not.
    let get=|name:&str|rows.iter().find(|v|v["stream"].as_str().unwrap().starts_with(name)).unwrap();
    for kind in ["original","centered"]{for metric in ["terms","negative"]{assert_eq!(get("fused-k/")[kind][metric].as_u64().unwrap()+get("fused-h/")[kind][metric].as_u64().unwrap(),get("fused-combined/")[kind][metric].as_u64().unwrap());}}
    let out=json!({"status":"complete verified actual scalar census","production_changed":false,"adopted":false,"rows":rows,"independent_proof_count":12583,"new_independent_exports":3,"SDK_requests":12,"all_three_censuses_identical":true,"latency_measured":false,"RSS_measured":false,"goal_status":"active","original8_CPU10x_rows":0});save(r.join("analysis.json"),&out);
    println!("{}",serde_json::to_string_pretty(&out).unwrap());
}
fn main(){match std::env::args().nth(1).as_deref(){Some("setup")=>setup(),Some("repin-build")=>save(Path::new(R).join("build-pins.json"),&json!(source_pins())),Some("prepare")=>prepare(),Some("finish")=>finish(),_=>panic!("setup/repin-build/prepare/finish")}}
