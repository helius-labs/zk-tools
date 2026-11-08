use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/unreduced-gcd-native176";
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn main(){
 assert!(!Path::new(&format!("{ROOT}/review.json")).exists());
 let mut counts=Vec::new();
 for name in["qualification-pins.json","qualification-pins-fixed.json","qualification-pins-nonlto.json"]{
  let pins=read(&format!("{ROOT}/{name}"));for(p,h)in pins.as_object().unwrap(){let actual=if name=="qualification-pins.json" && p==&format!("{ROOT}/native.rs"){format!("{ROOT}/native-initial.rs")}else{p.clone()};assert_eq!(hash(&actual),h.as_str().unwrap());}counts.push(pins.as_object().unwrap().len());
 }
 assert_eq!(counts,[189,191,194]);
 let old=fs::read_to_string(format!("{ROOT}/native-initial.rs")).unwrap();let new=fs::read_to_string(format!("{ROOT}/native.rs")).unwrap();assert_eq!(old.replace("a.len.max(b.len())","a.len.max(b.len)"),new);
 let original=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/inverse.rs").unwrap();
 let start=original.find("#[inline(always)]\nfn bit_len").unwrap();let end=original.find("/// (f*a+g*b)/2^SHIFT").unwrap();assert_eq!(fs::read_to_string(format!("{ROOT}/integer-helpers.rs.inc")).unwrap(),&original[start..end]);
 let start=original.find("#[inline(always)]\npub(crate) fn decode_row(e:u64)").unwrap();assert_eq!(fs::read_to_string(format!("{ROOT}/accepted-matrix.rs.inc")).unwrap(),&original[start..]);
 let mut receipts=Vec::new();
 for(dir,steps,success)in[("qualification-runs",vec!["build-fixed-limb-qualification"],false),("qualification-runs-fixed",vec!["build-fixed-limb-qualification"],false),("qualification-runs-nonlto",vec!["build-fixed-limb-qualification","fixed-limb-independent-oracles"],true)]{
  for step in steps{let base=format!("{ROOT}/{dir}/{step}");let exit=read(&format!("{base}/exit.json"));assert_eq!(exit["success"],success);for stream in["stdout","stderr"]{assert_eq!(hash(&format!("{base}/{stream}.log")),exit[format!("{stream}_sha256")].as_str().unwrap());}receipts.push(exit);}
 }
 let argv=read(&format!("{ROOT}/qualification-runs-nonlto/build-fixed-limb-qualification/command.json"));let args=argv["argv"].as_array().unwrap();for flag in["-O","target-cpu=native","lto=off","codegen-units=1","overflow-checks=yes"]{assert!(args.iter().any(|x|x==flag));}assert!(!args.iter().any(|x|x=="lto=fat"));
 let q=read(&format!("{ROOT}/qualification.json"));assert_eq!(q["native_arithmetic_qualification_passed"],true);assert_eq!(q["wide_linear_checks"],3927);assert_eq!(q["native_consumer_codegen_reviewed"],false);assert_eq!(q["timings_measured"],false);
 assert_eq!(q["fields"].as_array().unwrap().len(),2);for f in q["fields"].as_array().unwrap(){assert_eq!(f["complete_inverse_checks"],8956);assert_eq!(f["threaded_inverse_checks"],4*8956);assert_eq!(f["correction_checks"],72);assert_eq!(f["worker_counts"],json!([1,4,16,32]));assert_eq!(f["worker_stack_bytes"],524288);assert_eq!(f["raw_zero_none"],true);}
 let review=json!({"review_passed":true,"source_pin_populations":counts,"command_receipts":receipts.len(),"failed_build_receipts":2,"successful_receipts":2,"initial_source_preserved":true,"sole_source_compile_fix":"b.len() -> b.len","integer_and_matrix_helpers_byte_exact":true,"qualification_argv":{"opt_level":3,"target_cpu":"native","lto":"off","codegen_units":1,"overflow_checks":true},"wide_linear_checks":3927,"complete_inverse_case_rows":17912,"threaded_inverse_checks":71648,"late_correction_checks":144,"zero_checks":2,"qualification_sha256":hash(&format!("{ROOT}/qualification.json")),"native_arithmetic_qualification_passed":true,"production_fat_LTO_consumer_qualified":false,"speedup_measured":false,"new_proofs":0,"canonical_unchanged":35181,"goal_complete":false,"limits":"Offline reader verifies source/receipt/flag/schema/count bindings, not a second execution or independent proof of native wide arithmetic. Exact BigUint and accepted inverse comparisons execute in the pinned qualification driver. Original GCD termination and full branch coverage are not newly proved."});
 fs::write(format!("{ROOT}/review.json"),serde_json::to_vec_pretty(&review).unwrap()).unwrap();println!("Native reader PASS:189/191/194 pins,4 receipts incl2 retained failed builds;17912+71648 inverse checks; arithmetic only, no fat-LTO/time claim");
}
