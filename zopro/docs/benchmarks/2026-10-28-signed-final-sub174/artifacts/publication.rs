use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/signed-final-sub174";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-signed-final-sub174";
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn main(){
 let root=Path::new(R);let dest=Path::new(D);let analysis=read(root.join("analysis.json"));assert_eq!(analysis["component_gate"],true);
 for(p,h)in read(root.join("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let mut index:BTreeMap<String,String>=serde_json::from_value(read(dest.join("artifact-index.json"))).unwrap();for(p,h)in &index{assert_eq!(&hash(dest.join(p)),h);}
 assert_eq!(index.keys().filter(|s|s.starts_with("artifacts/source/")).count(),176);
 let t=fs::read_to_string(root.join("staged-whitespace.log")).unwrap();let diagnostics:Vec<_>=t.lines().filter(|s|s.starts_with("zopro/")).collect();assert_eq!(diagnostics.len(),15);
 assert!(diagnostics.iter().all(|s|(s.contains("/native/zopro-")||s.contains("/native-initial/zopro-"))&&s.contains("/stdout.log:")||s.contains("/runs/decode-")||s.contains("/source/crates/zopro-metal/Cargo.toml:")));
 save(dest.join("publication.json"),&json!({"status":"complete component; SDK trial permitted; not adopted","component_gate":true,"source_files":176,"originals_exact":170,"reverse174_exact":true,
  "capture_steps":400,"latency_processes":372,"preflights":12,"profiles":4,"profile_samples":4615,"lost_records":0,"all_capture_pins_match":true,
  "native_unique_successful_steps":10,"initial_OFF_link_failed_sha2_omission":true,"successful_tests_or_captures_repeated":false,
  "OFF_field_tests":33,"ON_field_tests":35,"core_tests_each":125,"new_field_Arkworks_lane_outputs":185760,"SDK_requests":0,"new_proof_exports":0,"independent_proof_count":15553,
  "timed_allocation_calls":0,"capacities_match":true,"production_changed":false,"adopted":false,"goal_achieved":false,"goal_status":"active/unmet0of8",
  "captured_whitespace_diagnostics":15,"authored_whitespace_diagnostics":0,"captured_bytes_preserved":true,"raw_perf_local_sha256":analysis["raw_perf_sha256"],"raw_perf_hex_local_sha256":analysis["raw_loss_dump_sha256"],
  "accepted_primary_ms":8.366931,"accepted_primary_peak_RSS_max_MiB":135.0,"new_peakRSS_or_pressure_claim":false}));
 fs::copy(root.join("publication.rs"),dest.join("artifacts/publication.rs")).unwrap();
 for p in ["publication.json","README.md","artifacts/publication.rs"]{index.insert(p.into(),hash(dest.join(p)));}save(dest.join("artifact-index.json"),&json!(index));
 let mut sums=index.clone();sums.insert("artifact-index.json".into(),hash(dest.join("artifact-index.json")));fs::write(dest.join("SHA256SUMS"),sums.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();for(p,h)in &sums{assert_eq!(&hash(dest.join(p)),h);}
 println!("Verified{}hashes/176complete source;15captured0authored whitespace;component qualified only",sums.len());
}
