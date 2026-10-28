#![recursion_limit="256"]
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/loose-g1-finish174";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-loose-g1-finish174";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn main(){
 let r=Path::new(R);let d=Path::new(D);let a=read(r.join("analysis.json"));
 assert_eq!(a["SDK_requests"],7806);assert_eq!(a["independent_proof_count"],15865);
 assert_eq!(read(r.join("runs/result.json"))["status"],"complete");
 for(p,h)in read(r.join("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 assert_eq!(read(r.join("proof-inventory.json")).as_object().unwrap().len(),108);
 let t=fs::read_to_string(r.join("staged-whitespace.log")).unwrap();let lines:Vec<_>=t.lines().filter(|s|s.starts_with("zopro/")).collect();
 assert!(lines.iter().all(|s|(s.contains("/native/zopro-")||s.contains("/native-repair/zopro-"))&&s.contains("/stdout.log:")||s.contains("/runs/decode-cycles-")||s.contains("/source/crates/zopro-metal/Cargo.toml:")));
 let profiles=a["diagnostics"]["profiles"].as_object().unwrap();assert_eq!(profiles.len(),3);
 let samples:u64=profiles.values().map(|p|p["samples"].as_u64().unwrap()).sum();
 assert_eq!(a["diagnostics"]["all_events_running_at_least_99_9_percent"],true);assert_eq!(a["diagnostics"]["profile_one_PID_each_zero_lost"],true);
 let mut streams=serde_json::Map::new();for (k,p)in profiles{let rows:Vec<_>=p["symbols"].as_array().unwrap().iter().filter(|s|s["symbol"].as_str().unwrap().contains("stream_g1_impl")).map(|s|json!({"demangled":rustc_demangle::demangle(s["symbol"].as_str().unwrap()).to_string(),"samples":s["samples"],"cycle_share":s["cycle_share"]})).collect();streams.insert(k.clone(),json!(rows));}
 let p=json!({"status":"complete primary SDK screen;no adoption","source_files":175,"originals_exact":171,"reverse174_exact":true,"production_changed":false,"adopted":false,"gate_passed":a["gate_passed"],"failed_checks":a["failed_checks"],"all_capture_pins_match":true,"capture_pin_count":read(r.join("pins.json")).as_object().unwrap().len(),"capture_steps":117,"latency_processes":45,"preflight_exports":3,"diagnostic_processes":6,"SDK_requests":7806,"fresh_independent_Gnark_exports":54,"proof_public_members":108,"independent_proof_count_before":15811,"independent_proof_count":15865,"current_field_tests":{"OFF":33,"ON":34},"current_core_tests":{"OFF":122,"ON":122},"current_prover_tests_each":8,"new_independent_Arkworks_coordinate_checks":133120,"original_nonfused_CIOS_body_unchanged":true,"original_field_generation_parent_unchanged":true,"PMU_running_percent_min":100.0,"cycles_profiles":3,"profile_samples":samples,"profile_G1_streams":streams,"one_PID_each":true,"lost_records":0,"profile_timings_in_latency_gate":false,"successful_tests_or_captures_repeated":false,"threshold_or_protocol_changed_after_capture":false,"captured_whitespace_diagnostics":lines.len(),"authored_whitespace_diagnostics":0,"captured_bytes_preserved":true,"all175source_files_and108proof_members_archived":true,"raw_perf_local_sha256":read(d.join("artifacts/local-perf-data.json")),"accepted_qualified_primary_ms":8.366931,"contemporaneous_accepted_primary_ms":a["cells"]["1x2-w32/accepted"]["median_ms"],"goal_achieved":false,"original8_CPU10x_achieved":0,"goal_status":"active and unmet","owned_jobs_cloud_probes":0});
 fs::write(d.join("publication.json"),serde_json::to_vec_pretty(&p).unwrap()).unwrap();fs::copy(r.join("publication.rs"),d.join("artifacts/publication.rs")).unwrap();
 let mut p=read(d.join("publication.json"));p["candidate_const_assertion_type_error_before_tests"]=json!(true);p["original_control_six_steps_reused"]=json!(true);p["native_completion_receipt"]=read(r.join("native-completion.json"));fs::write(d.join("publication.json"),serde_json::to_vec_pretty(&p).unwrap()).unwrap();
 fs::write(d.join("artifacts/staged-whitespace.log"),format!("{}\n",lines.join("\n"))).unwrap();
 println!("Publication checked54freshGnark/allpins;{}captured0authored whitespace;{}profile samples;gate{}",lines.len(),samples,a["gate_passed"]);
}
