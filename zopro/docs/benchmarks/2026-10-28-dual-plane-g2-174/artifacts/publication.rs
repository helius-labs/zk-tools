#![recursion_limit="256"]
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/dual-plane-g2-174";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-dual-plane-g2-174";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn main(){
 let r=Path::new(R);let d=Path::new(D);let a=read(r.join("analysis.json"));
 assert_eq!(a["gate_passed"],false);assert_eq!(a["SDK_requests"],7806);assert_eq!(a["independent_proof_count"],15811);
 assert_eq!(read(r.join("runs/result.json"))["status"],"complete");
 for(p,h)in read(r.join("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 assert_eq!(read(r.join("proof-inventory.json")).as_object().unwrap().len(),108);
 let t=fs::read_to_string(r.join("staged-whitespace.log")).unwrap();let lines:Vec<_>=t.lines().filter(|s|s.starts_with("zopro/")).collect();assert_eq!(lines.len(),15);
 assert!(lines.iter().all(|s|s.contains("/native/zopro-")&&s.contains("/stdout.log:")||s.contains("/runs/decode-cycles-")||s.contains("/source/crates/zopro-metal/Cargo.toml:")));
 let profiles=a["diagnostics"]["profiles"].as_object().unwrap();assert_eq!(profiles.len(),3);
 let samples:u64=profiles.values().map(|p|p["samples"].as_u64().unwrap()).sum();assert_eq!(samples,28054);
 assert_eq!(a["diagnostics"]["all_events_running_at_least_99_9_percent"],true);
 assert_eq!(a["diagnostics"]["profile_one_PID_each_zero_lost"],true);
 let p=json!({"status":"complete;primary latency gate failed;not adopted","source_files":175,"originals_exact":172,"reverse174_exact":true,"production_changed":false,"adopted":false,"gate_passed":false,"failed_checks":a["failed_checks"],"all_capture_pins_match":true,"capture_pin_count":read(r.join("pins.json")).as_object().unwrap().len(),"capture_steps":117,"latency_processes":45,"preflight_exports":3,"diagnostic_processes":6,"SDK_requests":7806,"fresh_independent_Gnark_exports":54,"proof_public_members":108,"independent_proof_count_before":15757,"independent_proof_count":15811,"current_core_tests":{"OFF":122,"ON":124},"current_prover_tests_each":8,"field_tests_inherited_not_rerun":33,"pre_query_admission_draft_tests_preserved_no_performance_capture":true,"PMU_running_percent_min":100.0,"cycles_profiles":3,"profile_samples":samples,"one_PID_each":true,"lost_records":0,"profile_timings_in_latency_gate":false,"dynamic_dual_plane_route_not_established_by_named_profile_symbols":true,"successful_tests_or_captures_repeated":false,"threshold_or_protocol_changed_after_capture":false,"captured_whitespace_diagnostics":15,"authored_whitespace_diagnostics":0,"captured_bytes_preserved":true,"all175source_files_and108proof_members_archived":true,"raw_perf_local_sha256":read(d.join("artifacts/local-perf-data.json")),"accepted_qualified_primary_ms":8.366931,"contemporaneous_accepted_primary_ms":8.076584,"goal_achieved":false,"original8_CPU10x_achieved":0,"goal_status":"active and unmet","owned_jobs_cloud_probes":0});
 fs::write(d.join("publication.json"),serde_json::to_vec_pretty(&p).unwrap()).unwrap();
 fs::copy(r.join("publication.rs"),d.join("artifacts/publication.rs")).unwrap();
 fs::write(d.join("artifacts/staged-whitespace.log"),format!("{}\n",lines.join("\n"))).unwrap();
 println!("Publication checked:54freshGnark;allclosingpins;15captured/0authored whitespace;noadoption");
}
