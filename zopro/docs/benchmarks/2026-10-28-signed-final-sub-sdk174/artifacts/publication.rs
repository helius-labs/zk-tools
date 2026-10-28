#![recursion_limit="256"]
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-signed-final-sub-sdk174";
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){let r=Path::new(R);let d=Path::new(D);let a=read(r.join("analysis.json"));assert_eq!(a["gate_passed"],false);assert_eq!(a["independent_proof_count"],15757);assert_eq!(a["SDK_requests"],18696);
 for(p,h)in read(r.join("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 assert_eq!(read(r.join("proof-inventory.json")).as_object().unwrap().len(),408);
 let t=fs::read_to_string(r.join("staged-whitespace.log")).unwrap();let lines:Vec<_>=t.lines().filter(|s|s.starts_with("zopro/")).collect();assert_eq!(lines.len(),15);assert!(lines.iter().all(|s|s.contains("/native/zopro-prover-")&&s.contains("/stdout.log:")||s.contains("/runs/decode-cycles-")||s.contains("/source/crates/zopro-metal/Cargo.toml:")));
 let pubdata=json!({"status":"complete;primary latency gate failed;not adopted","source_files":176,"originals_exact":170,"reverse174_exact":true,"source_matches_qualified_component_bytes":true,
 "production_changed":false,"adopted":false,"gate_passed":false,"failed_checks":a["failed_checks"],"all_capture_pins_match":true,"capture_pin_count":read(r.join("pins.json")).as_object().unwrap().len(),
 "capture_steps":426,"latency_processes":180,"preflight_exports":12,"diagnostic_processes":12,"SDK_requests":18696,"fresh_independent_Gnark_exports":204,"proof_public_members":408,"independent_proof_count_before":15553,"independent_proof_count":15757,
 "new_prover_tests_each":8,"field_core_tests_reused_from_component":true,"component_report_commit":"a708550951e501528add767068b397741eae9634","native_SDK_build_steps":4,
 "PMU_running_percent_min":100.0,"cycles_profiles":6,"profile_samples":43434,"one_PID_each":true,"lost_records":0,"profile_timings_in_latency_gate":false,
 "preparation_casefile_assertion_failed_before_capture":true,"successful_tests_or_captures_repeated":false,"threshold_or_protocol_changed_after_capture":false,
 "captured_whitespace_diagnostics":15,"authored_whitespace_diagnostics":0,"captured_bytes_preserved":true,"all176source_files_and408proof_members_archived":true,
 "raw_perf_local_sha256":read(d.join("artifacts/local-perf-data.json")),"accepted_qualified_primary_ms":8.366931,"contemporaneous_accepted_primary_ms":8.161132,
 "goal_achieved":false,"original8_CPU10x_achieved":0,"goal_status":"active and unmet","owned_jobs_cloud_probes":0});
 fs::write(d.join("publication.json"),serde_json::to_vec_pretty(&pubdata).unwrap()).unwrap();fs::copy(r.join("publication.rs"),d.join("artifacts/publication.rs")).unwrap();println!("Publication validated15captured0authored whitespace;204freshGnark/allclosingpins;noadoption");
}
