use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf}};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/bounded-coalesce176";const Z:&str="/home/ubuntu/src/zk-tools/zopro";const BASE:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
fn p(s:impl AsRef<Path>)->PathBuf{Path::new(ROOT).join(s)}
fn read(x:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(x).unwrap()).unwrap()}
fn save(x:impl AsRef<Path>,v:&Value){fs::write(x,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(x:impl AsRef<Path>)->String{let mut f=fs::File::open(x).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pin(m:&mut BTreeMap<String,String>,x:impl AsRef<Path>){m.insert(x.as_ref().display().to_string(),hash(&x));}
fn body<'a>(s:&'a str,name:&str)->&'a str{let start=s.find(name).unwrap();let open=start+s[start..].find('{').unwrap();let mut n=0;for(i,b)in s.as_bytes()[open..].iter().enumerate(){if *b==b'{'{n+=1}if *b==b'}'{n-=1;if n==0{return &s[start..open+i+1]}}}panic!("unclosed body")}
fn normalize(s:&str)->String{s.chars().filter(|c|!c.is_whitespace()).collect()}
fn setup(){
 assert!(!p("plan.json").exists());let mut pins=BTreeMap::new();
 let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");let m=read(&manifest);
 let frozen=Path::new(BASE).join("frozen-candidate.json");let f=read(&frozen);assert_eq!(m["rustflags"],f["rustflags"]);
 for(x,h)in m["source_files_sha256"].as_object().unwrap(){let q=Path::new(Z).join(x);assert_eq!(hash(&q),h.as_str().unwrap());pin(&mut pins,q);}assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),176);
 for(x,h)in f["input_pins"].as_object().unwrap(){assert_eq!(hash(x),h.as_str().unwrap());pins.insert(x.clone(),h.as_str().unwrap().to_string());}
 let reused=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs").unwrap();let phi=fs::read_to_string(Path::new(Z).join("crates/zopro-core/src/msm_affine/g2_phi.rs")).unwrap();
 let start=phi.find("const C1:").unwrap();let end=phi.find("use zopro_bn254::{Fq,").unwrap();assert_eq!(normalize(&phi[start..end]),normalize(&reused[..reused.find("fn booth_digit").unwrap()]));
 let msm=fs::read_to_string(Path::new(Z).join("crates/zopro-core/src/msm_affine.rs")).unwrap();assert_eq!(normalize(body(&msm,"fn booth_digit(")),normalize(body(&reused,"fn booth_digit(")));
 save(p("source-review.json"),&json!({"complete":true,"production_files":176,"Phi_constants_and_functions_exact_ignoring_whitespace":true,"Booth_function_exact_ignoring_whitespace":true,"new_prover_source":false}));
 let artifacts=Path::new(BASE).join("frozen-candidate-cargo.jsonl");for q in [&manifest,&frozen,&artifacts]{pin(&mut pins,q);}
 let mut inputs=BTreeMap::new();let old=read("/home/ubuntu/src/zopro-perf-work/g1-column-count176/pins-segmented.json");
 for shape in["1x2","2x2"]{for query in["A","B"]{
  let file=if query=="A"{format!("/home/ubuntu/src/zopro-perf-work/g1-column-count176/{shape}-raw-a.words")}else{format!("/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/inputs/{shape}.b2q")};
  let receipt=if query=="A"{"/home/ubuntu/src/zopro-perf-work/g1-column-count176/pins-segmented.json".to_string()}else{format!("/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/oracle-fixed-runs/oracle-{shape}/stdout.log")};
  if query=="A"{assert_eq!(hash(&file),old[&file]);}else{assert_eq!(hash(&file),read(&receipt)["fixture_sha256"]);}
  pin(&mut pins,&file);pin(&mut pins,&receipt);inputs.insert(format!("{shape}-{query}"),json!({"shape":shape,"query":query,"file":file,"sha256":hash(&file),"provenance_receipt":receipt}));
 }}
 save(p("inputs.json"),&json!(inputs));
 let mut steps=vec![json!({"kind":"link-sdk","name":"model-tests","artifacts":artifacts,"driver":p("probe.rs"),"additional_sources":[p("model.rs"),"/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs"],"rustc_flags":["--test","-O","-C","target-cpu=native"],"timeout_seconds":600}),
 json!({"kind":"command","name":"model-tests-run","argv":[p("runs/model-tests-sdk"),"--test-threads=1"],"required_tests":["model::tests::digit_reconstruction_full_width_and_boundaries","model::tests::coalescing_is_bounded_and_charges_only_useful_classes","model::tests::every_bucket_matches_for_skewed_and_boundary_tiles"],"unfiltered":true,"timeout_seconds":600}),
 json!({"kind":"link-sdk","name":"census","artifacts":artifacts,"driver":p("probe.rs"),"additional_sources":[p("model.rs"),"/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs"],"timeout_seconds":600})];
 for(name,v)in &inputs{steps.push(json!({"kind":"command","name":format!("count-{name}"),"argv":[p("runs/census-sdk"),v["query"],v["file"],p(format!("{name}.json"))],"timeout_seconds":600}));}
 for name in["model.rs","probe.rs","workflow.rs","workflow-helper","design.md","inputs.json","source-review.json"]{pin(&mut pins,p(name));}
 pin(&mut pins,"/home/ubuntu/src/zopro-perf-work/g2-balanced-count175/split.rs");pin(&mut pins,"/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask");
 save(p("pins.json"),&json!(pins));save(p("plan.json"),&json!({"version":1,"source_pins":p("pins.json"),"steps":steps}));
 println!("Prepared7steps,3structural tests,4cached populations; exact original digit/Phi bodies; no proof/timing run");
}
fn finish(){
 assert_eq!(read(p("runs/result.json"))["status"],"complete");for(x,h)in read(p("pins.json")).as_object().unwrap(){assert_eq!(hash(x),h.as_str().unwrap());}
 let tests=read(p("runs/model-tests-run/tests.json"));assert_eq!(tests["unfiltered"],true);assert_eq!(tests["passed_tests"].as_array().unwrap().len(),3);
 let mut cases=BTreeMap::new();let mut positive=0;let mut eligible=0;
 for(name,_)in read(p("inputs.json")).as_object().unwrap(){let v=read(p(format!("{name}.json")));if v["bounded_path_selected"]==true{eligible+=1;positive+=usize::from(v["necessary_opportunity_positive"]==true);assert_eq!(v["two_independent_token_streams"],true);}cases.insert(name.clone(),v);}
 save(p("analysis.json"),&json!({"complete":true,"cases":cases,"eligible_populations":eligible,"positive_populations":positive,"necessary_opportunity_passed":eligible>0&&positive==eligible,"performance_claim":false,"tests":3,"new_witness_solves":0,"new_proofs":0,"new_timing_samples":0,"proof_ledger":27353,"production_changed":false,"accepted_primary_ms":7.928660,"primary_target_ms":3.7,"CPU10x_goal_rows_achieved":0,"CPU10x_goal_rows_total":8}));
 println!("Bounded coalescer opportunity:eligible={eligible},positive={positive}; no prover change or speed claim");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("setup")=>setup(),Some("finish")=>finish(),_=>panic!("setup/finish")}}
