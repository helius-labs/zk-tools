use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/partial-windows175";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn function(s:&str,needle:&str)->String{let start=s.find(needle).unwrap();let brace=s[start..].find('{').unwrap()+start;let mut depth=0;for(i,b)in s.as_bytes().iter().enumerate().skip(brace){if *b==b'{'{depth+=1}if *b==b'}'{depth-=1;if depth==0{return s[start..=i].to_string()}}}panic!()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){
    let r=Path::new(R);let z=Path::new(Z);assert!(!r.join("plan.json").exists());
    let mut pins=BTreeMap::new();
    let manifest=z.join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let m=read(&manifest);let files=m["source_files_sha256"].as_object().unwrap();assert_eq!(files.len(),175);
    pins.insert(manifest.display().to_string(),hash(&manifest));
    let old=Path::new("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/source-frozen");
    let suffixes=BTreeMap::from([
        ("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_gauss_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_gauss_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n"),
        ("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_gauss_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n")]);
    for(f,h)in files {
        let p=z.join(f);assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p.display().to_string(),hash(p));
        let p=old.join(f);let mut source=fs::read(&p).unwrap();
        if let Some(suffix)=suffixes.get(f.as_str()){assert!(source.ends_with(suffix.as_bytes()));source.truncate(source.len()-suffix.len());}
        assert_eq!(format!("{:x}",Sha256::digest(&source)),h.as_str().unwrap());
        pins.insert(p.display().to_string(),hash(p));
    }
    let fpath="/home/ubuntu/src/zopro-perf-work/early-x175/artifacts-frozen.json";let frozen=read(fpath);
    assert_eq!(frozen["libraries"].as_object().unwrap().len(),9);
    for(p,h)in frozen["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.clone(),h.as_str().unwrap().to_string());}
    pins.insert(fpath.to_string(),hash(fpath));
    let prior="/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/probe.rs";
    let now=fs::read_to_string(r.join("probe.rs")).unwrap();
    assert_eq!(function(&fs::read_to_string(prior).unwrap(),"fn groups("),function(&now,"fn groups("));
    pins.insert(prior.to_string(),hash(prior));
    let prior="/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs";
    assert_eq!(hash(prior),hash(r.join("split.rs")));pins.insert(prior.to_string(),hash(prior));
    let split=fs::read_to_string(r.join("split.rs")).unwrap();
    let original=fs::read_to_string(z.join("crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
    for name in ["fn product<","fn sign_magnitude(","fn phi_split("] {assert_eq!(function(&split,name),function(&original,name));}
    assert_eq!(function(&split,"fn booth_digit("),function(&fs::read_to_string(z.join("crates/zopro-core/src/msm_affine.rs")).unwrap(),"fn booth_digit("));
    let prodplan=Path::new("/home/ubuntu/src/zopro-perf-work/balanced-kh-production/run-plan.json");
    let env=read(&prodplan)["steps"][0]["environment"].clone();assert_eq!(env["ZOPRO_CPU_ZERO_AWARE_GROUPS"],"1");assert_eq!(env["ZOPRO_CPU_DEDUP"],"1");
    pins.insert(prodplan.display().to_string(),hash(&prodplan));
    for shape in ["1x2","2x2"] {
        let fixture=format!("/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/inputs/{shape}.b2q");
        let oracle=format!("/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/oracle-fixed-runs/oracle-{shape}/stdout.log");
        assert_eq!(hash(&fixture),read(&oracle)["fixture_sha256"]);pins.insert(fixture.clone(),hash(fixture));pins.insert(oracle.clone(),hash(oracle));
    }
    for p in ["setup.rs","setup","probe.rs","split.rs","design.md","initial-setup/setup.rs","initial-setup/setup","initial-setup/setup.log"]{let p=r.join(p);pins.insert(p.display().to_string(),hash(p));}
    let xtask="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";pins.insert(xtask.to_string(),hash(xtask));
    let dir=Path::new(frozen["libraries"]["zopro_bn254"].as_str().unwrap()).parent().unwrap();
    let mut args=vec!["rustc".to_string(),"--edition=2021".into(),"-O".into(),"-C".into(),"target-cpu=native".into(),"-C".into(),"lto=fat".into(),"-C".into(),"codegen-units=1".into(),"-L".into(),format!("dependency={}",dir.display())];
    for(n,p)in frozen["libraries"].as_object().unwrap(){args.extend(["--extern".into(),format!("{n}={}",p.as_str().unwrap())]);}
    args.push(r.join("probe.rs").display().to_string());
    let mut tests=args.clone();tests.extend(["--test".into(),"-o".into(),r.join("tests").display().to_string()]);
    args.extend(["-o".into(),r.join("probe").display().to_string()]);
    let step=|name:&str,argv:Vec<String>|json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":900});
    save(r.join("pins.json"),&json!(pins));
    save(r.join("plan.json"),&json!({"version":1,"source_pins":r.join("pins.json"),"steps":[step("compile-tests",tests),step("test-sharing-model",vec![r.join("tests").display().to_string(),"--test-threads=1".into()]),step("compile-probe",args),step("count-two-fixtures",vec![r.join("probe").display().to_string()])]}));
    save(r.join("setup-audit.json"),&json!({"production_files_verified":175,"frozen_files_recovered":175,"group_function_byte_exact":true,"split_functions_byte_exact":true,"Booth_function_byte_exact":true,"accepted_environment":env,"pin_count":pins.len()}));
    println!("Prepared four count-only steps, {} pins; no production change",pins.len());
}
