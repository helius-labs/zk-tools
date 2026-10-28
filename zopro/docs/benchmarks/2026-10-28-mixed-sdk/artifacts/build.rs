use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/mixed-sdk";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){let r=Path::new(R);assert!(!r.join("build-pins.json").exists());let mut pins=BTreeMap::new();let manifest:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    for(f,h)in manifest["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p.display().to_string(),hash(&p));for dir in ["source","control-source","diagnostic-source"]{let p=r.join(dir).join(f);if dir=="control-source"{assert_eq!(hash(&p),h.as_str().unwrap());}pins.insert(p.display().to_string(),hash(&p));}}
    for p in ["source/crates/zopro-core/src/msm/mixed_g1.rs","diagnostic-source/crates/zopro-core/src/msm/mixed_g1.rs","diagnostic-source/crates/zopro-core/src/msm_geometry.rs","admission.rs","setup.rs","setup","mixed_g1.rs","inverse-edits.json","source-audit.json","diagnostic-setup.rs","diagnostic-setup","diagnostic-audit.json","observer.rs","diagnostic-driver.rs","native-flags.txt","design.md","build.rs","build"]{let p=r.join(p);pins.insert(p.display().to_string(),hash(&p));}
    for p in [format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),format!("{Z}/crates/zopro-bench/examples/support/perf.rs")]{pins.insert(p.clone(),hash(p));}
    save(r.join("build-pins.json"),&json!(pins));
    for label in ["candidate","control","diagnostic"] {let dir=match label{"candidate"=>"source","control"=>"control-source",_=>"diagnostic-source"};let target=match label{"candidate"=>"target","control"=>"target-control",_=>"target-diagnostic"};let out=r.join(format!("native-{label}"));
        let base=manifest["rustflags"].as_str().unwrap();let flags=match label{"candidate"=>format!("{base} --cfg zopro_mixed_g1"),"control"=>base.into(),_=>format!("{base} --cfg zopro_mixed_g1 --cfg zopro_msm_geometry")};
        let driver=if label=="diagnostic"{r.join("diagnostic-driver.rs")}else{Path::new(Z).join("crates/zopro-bench/examples/cpu-sdk.rs")};
        let sources=if label=="diagnostic"{Vec::<String>::new()}else{vec![format!("{Z}/crates/zopro-bench/examples/support/perf.rs")]};
        save(r.join(format!("build-{label}.json")),&json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":{"RUSTFLAGS":flags,"CARGO_TARGET_DIR":r.join(target),"CARGO_BUILD_JOBS":"16"},"steps":[
            {"kind":"command","name":"libraries","argv":["cargo","build","--manifest-path",r.join(dir).join("Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"timeout_seconds":1200},
            {"kind":"link-sdk","name":label,"artifacts":out.join("libraries/stdout.log"),"driver":driver,"additional_sources":sources,"timeout_seconds":600}
        ]}));
    }
}
