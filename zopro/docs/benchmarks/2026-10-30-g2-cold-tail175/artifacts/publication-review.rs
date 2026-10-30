use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::Path,process::Command};
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let z=Path::new("/home/ubuntu/src/zk-tools/zopro");let r=Path::new("/home/ubuntu/src/zopro-perf-work/g2-cold-tail175");
 let d=z.join("docs/benchmarks/2026-10-30-g2-cold-tail175");let s=z.join("docs/benchmarks/2026-10-29-current-cpu-summary");
 let pins=read(r.join("pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let supporting=read(s.join("source-hashes.json"));for(p,h)in supporting.as_object().unwrap(){assert_eq!(hash(z.join(p)),h.as_str().unwrap());}
 let manifest=read(z.join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));let production=&manifest["source_files_sha256"];assert_eq!(production.as_object().unwrap().len(),175);for(p,h)in production.as_object().unwrap(){assert_eq!(hash(z.join(p)),h.as_str().unwrap());}
 let old=Command::new("git").args(["show","241f02692e:zopro/docs/benchmarks/2026-10-29-current-cpu-summary/data.json"]).output().unwrap();assert!(old.status.success());let old:Value=serde_json::from_slice(&old.stdout).unwrap();let current=read(s.join("data.json"));assert_eq!(old["rows"],current["rows"]);assert_eq!(current["rows"].as_array().unwrap().len(),9);assert_eq!(current["accepted_original8_CPU10x"],0);assert_eq!(current["historical_distinct_Gnark_ledger"],23971);
 let a=read(r.join("analysis.json"));assert_eq!(a,read(d.join("artifacts/analysis.json")));assert_eq!(a["caller_gate"],false);assert_eq!(a["failed_checks"].as_array().unwrap().len(),6);assert_eq!(a["cells"],current["latest_G2_cold_tail_caller"]["cells"]);assert_eq!(a["calibration_checks"],current["latest_G2_cold_tail_caller"]["calibration_checks"]);assert_eq!(a["kernel_calls_including_primes_and_profiles"],1059202848u64);assert_eq!(read(r.join("runs/result.json"))["status"],"complete");assert_eq!(read(r.join("runs/result.json"))["steps"].as_array().unwrap().len(),2129);assert_eq!(read(r.join("rows.json")).as_array().unwrap().len(),2089);
 assert!(a["profiles"].as_object().unwrap().values().all(|v|v["attribution_qualified"]==true&&v["zero_lost"]==true));assert!(a["counters"].as_object().unwrap().values().all(|v|v["qualified"]==true));
 let mut links=0;for base in[&d,&s]{assert!(Command::new("sha256sum").args(["-c","SHA256SUMS"]).current_dir(base).status().unwrap().success());let md=fs::read_to_string(base.join("README.md")).unwrap();for tail in md.split("](").skip(1){let link=tail.split(')').next().unwrap();assert!(base.join(link).is_file(),"missing {link}");links+=1;}}
 println!("{}",json!({"original_pins":pins.as_object().unwrap().len(),"summary_pins":supporting.as_object().unwrap().len(),"production_source_hashes":175,"accepted_local_AWS_rows_unchanged":9,"all_CPU_Metal_references_unchanged":8,"links":links,"caller_gate":false,"production_changed":false,"proof_ledger":23971}));
}
