use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,io::Read,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/bounded-coalesce176";
fn p(s:&str)->PathBuf{Path::new(R).join(s)}
fn read(x:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(x).unwrap()).unwrap()}
fn save(x:impl AsRef<Path>,v:&Value){fs::write(x,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(x:impl AsRef<Path>)->String{let mut f=fs::File::open(x).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pins(file:&str){for(x,h)in read(p(file)).as_object().unwrap(){assert_eq!(hash(x),h.as_str().unwrap());}}
fn setup(){
 pins("pins.json");assert!(!p("plan-fixed.json").exists());
 let s=fs::read_to_string(p("model.rs")).unwrap();let needle="n in[0,1,7,1023";assert_eq!(s.matches(needle).count(),1);
 fs::write(p("model-fixed.rs"),s.replacen(needle,"n in[0usize,1,7,1023",1)).unwrap();
 let s=fs::read_to_string(p("probe.rs")).unwrap();assert!(s.starts_with("mod model;"));fs::write(p("probe-fixed.rs"),s.replacen("mod model;","#[path=\"model-fixed.rs\"]mod model;",1)).unwrap();
 let mut plan=read(p("plan.json"));
 for step in plan["steps"].as_array_mut().unwrap(){if step["kind"]=="link-sdk"{step["driver"]=json!(p("probe-fixed.rs"));step["additional_sources"][0]=json!(p("model-fixed.rs"));}if let Some(a)=step.get_mut("argv"){for x in a.as_array_mut().unwrap(){if let Some(s)=x.as_str(){*x=json!(s.replace("/runs/","/runs-fixed/"));}}}}
 let mut m=read(p("pins.json")).as_object().unwrap().clone();for name in["model-fixed.rs","probe-fixed.rs","repair.rs","repair-helper"]{let q=p(name);m.insert(q.display().to_string(),json!(hash(q)));}
 save(p("pins-fixed.json"),&json!(m));plan["source_pins"]=json!(p("pins-fixed.json"));save(p("plan-fixed.json"),&plan);
 println!("Preserved initial E0689 test-build failure; only usize test-length annotation repaired; no mathematical body or successful execution changed");
}
fn finish(){
 assert_eq!(read(p("runs-fixed/result.json"))["status"],"complete");pins("pins.json");pins("pins-fixed.json");
 let tests=read(p("runs-fixed/model-tests-run/tests.json"));assert_eq!(tests["unfiltered"],true);assert_eq!(tests["passed_tests"].as_array().unwrap().len(),3);
 let mut cases=serde_json::Map::new();let(mut eligible,mut positive)=(0,0);
 for(name,_)in read(p("inputs.json")).as_object().unwrap(){let v=read(p(&format!("{name}.json")));if v["bounded_path_selected"]==true{eligible+=1;positive+=usize::from(v["necessary_opportunity_positive"]==true);assert_eq!(v["two_independent_token_streams"],true);}cases.insert(name.clone(),v);}
 save(p("analysis.json"),&json!({"complete":true,"cases":cases,"eligible_populations":eligible,"positive_populations":positive,"necessary_opportunity_passed":eligible>0&&positive==eligible,"performance_claim":false,"tests":3,"new_witness_solves":0,"new_proofs":0,"new_timing_samples":0,"proof_ledger":27353,"production_changed":false,"accepted_primary_ms":7.928660,"primary_target_ms":3.7,"CPU10x_goal_rows_achieved":0,"CPU10x_goal_rows_total":8,"initial_compile_failure_preserved":true,"mathematical_implementation_unchanged_by_repair":true}));
 println!("Bounded coalescer opportunity: eligible={eligible} positive={positive}; production/ledger/timings unchanged");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("setup")=>setup(),Some("finish")=>finish(),_=>panic!("setup/finish")}}
