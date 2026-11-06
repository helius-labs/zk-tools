use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/g2-joint-ring-screen176";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){
    assert!(!Path::new(&format!("{ROOT}/plan.json")).exists());
    let z="/home/ubuntu/src/zk-tools/zopro";
    let manifest=format!("{z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let mut pins=BTreeMap::new();
    for(p,h)in read(&manifest)["source_files_sha256"].as_object().unwrap(){let p=format!("{z}/{p}");assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p,h.as_str().unwrap().to_string());}
    assert_eq!(pins.len(),176);
    let parent="/home/ubuntu/src/zopro-perf-work/cross-component-alias176";
    let inputs=read(format!("{parent}/inputs.json"));
    for i in inputs.as_object().unwrap().values(){let file=i["file"].as_str().unwrap();assert_eq!(hash(file),i["sha256"].as_str().unwrap());let oracle=i["provenance_receipt"].as_str().unwrap();let v=read(oracle);assert_eq!(v["independent_Arkworks_complete_MSM"],true);assert_eq!(v["fixture_sha256"],i["sha256"]);for p in[file,oracle]{pins.insert(p.to_string(),hash(p));}}
    fs::write(format!("{ROOT}/inputs.json"),serde_json::to_vec_pretty(&inputs).unwrap()).unwrap();
    let split=fs::read_to_string(format!("{parent}/split.rs")).unwrap();
    let phi=fs::read_to_string(format!("{z}/crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
    let prefix=phi.split_once("const C1:").unwrap().1.split_once("use zopro_bn254::{Fq,").unwrap().0;
    assert!(split.starts_with(&format!("const C1:{prefix}")));fs::write(format!("{ROOT}/split.rs"),split).unwrap();
    let compiler="/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc";
    let field="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
    assert_eq!(hash(field),"1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713");
    let deps="/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
    let serde=format!("{deps}/libserde_json-cdebb9338c2784ad.rlib");
    for p in[manifest,format!("{parent}/inputs.json"),format!("{parent}/split.rs"),compiler.into(),field.into(),serde.clone()]{pins.insert(p.clone(),hash(&p));}
    for report in ["2026-10-22-universal-binary-ring-native","2026-10-22-universal-hexagonal-native","2026-10-21-ryzen-research/preconverted-r260-g2-query","2026-10-21-ryzen-research/preconverted-r260-concurrent-phase","2026-10-21-ryzen-research/raw260-ordinary-buckets","2026-10-28-scaled-r260-const"]{let p=format!("{z}/docs/benchmarks/{report}/README.md");pins.insert(p.clone(),hash(&p));}
    for n in["setup.rs","setup","study.rs","split.rs","inputs.json","design.md"]{let p=format!("{ROOT}/{n}");pins.insert(p.clone(),hash(&p));}
    let compile=|test:bool|{let mut a=vec![compiler.to_string(),"--edition=2021".into(),"-O".into(),"-C".into(),"target-cpu=native".into(),format!("{ROOT}/study.rs"),"-L".into(),format!("dependency={deps}"),"-L".into(),format!("dependency={}",Path::new(field).parent().unwrap().display()),"--extern".into(),format!("serde_json={serde}"),"--extern".into(),format!("zopro_bn254={field}")];if test{a.push("--test".into());}a.extend(["-o".into(),format!("{ROOT}/{}",if test{"study-tests"}else{"study"})]);a};
    let step=|name:&str,argv:Vec<String>,unfiltered:bool|json!({"kind":"command","name":name,"argv":argv,"environment":{},"unfiltered":unfiltered,"timeout_seconds":300});
    let steps=vec![step("compile-tests",compile(true),false),step("ring-model-tests",vec![format!("{ROOT}/study-tests"),"--test-threads=1".into()],true),step("compile-study",compile(false),false),step("real-ring-counts",vec![format!("{ROOT}/study")],false)];
    fs::write(format!("{ROOT}/pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    fs::write(format!("{ROOT}/plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{ROOT}/pins.json"),"steps":steps})).unwrap()).unwrap();
    println!("Prepared four-step G2 ring screen, {} pins, all176 production exact",pins.len());
}
