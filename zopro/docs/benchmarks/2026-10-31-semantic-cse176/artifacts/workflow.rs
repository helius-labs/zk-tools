use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf}};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/semantic-cse176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const BASE:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const OLD:&str="/home/ubuntu/src/zopro-perf-work/wire-readiness175";
fn p(s:&str)->PathBuf{Path::new(ROOT).join(s)}
fn read(x:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(x).unwrap()).unwrap()}
fn save(x:impl AsRef<Path>,v:&Value){fs::write(x,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(x:impl AsRef<Path>)->String{let mut f=fs::File::open(x).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pin(m:&mut BTreeMap<String,String>,x:impl AsRef<Path>){m.insert(x.as_ref().display().to_string(),hash(&x));}
fn check_pins(){for(x,h)in read(p("pins.json")).as_object().unwrap(){assert_eq!(hash(x),h.as_str().unwrap(),"{x}");}}
fn setup(){
    assert!(!p("plan.json").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let m=read(&manifest);
    let frozen=Path::new(BASE).join("frozen-candidate.json");
    let f=read(&frozen);
    assert_eq!(m["rustflags"],f["rustflags"]);
    let mut pins=BTreeMap::new();
    for(x,h)in m["source_files_sha256"].as_object().unwrap(){let q=Path::new(Z).join(x);assert_eq!(hash(&q),h.as_str().unwrap());pin(&mut pins,q);}
    assert_eq!(m["source_files_sha256"].as_object().unwrap().len(),176);
    for(x,h)in f["input_pins"].as_object().unwrap(){assert_eq!(hash(x),h.as_str().unwrap());pins.insert(x.clone(),h.as_str().unwrap().to_string());}
    let artifacts=Path::new(BASE).join("frozen-candidate-cargo.jsonl");
    for q in [&manifest,&frozen,&artifacts]{pin(&mut pins,q);}
    let mut inputs=BTreeMap::new();
    for shape in ["1x2","2x2"]{
        let file=Path::new(OLD).join(format!("{shape}.solver"));
        let old=read(Path::new(OLD).join("analysis.json"));
        assert_eq!(hash(&file),old["parsed_programs"][shape]["solver_sha256"]);
        pin(&mut pins,&file);
        inputs.insert(shape,json!({"file":file,"sha256":hash(&file),"prior_metadata":old["parsed_programs"][shape]["census"]}));
    }
    save(p("inputs.json"),&json!(inputs));
    let mut steps=vec![
      json!({"kind":"link-sdk","name":"model-tests","artifacts":artifacts,"driver":p("census.rs"),"additional_sources":[p("model.rs"),p("semantic.rs"),p("semantic-tests.rs")],"rustc_flags":["--test","-O","-C","target-cpu=native"],"timeout_seconds":600}),
      json!({"kind":"command","name":"model-tests-run","argv":[p("runs/model-tests-sdk"),"--test-threads=1"],"required_tests":["model::tests::affine_forms_match_independent_field_evaluation","model::tests::unknown_divisor_is_not_cancelled_and_zero_branch_is_preserved","model::tests::parser_propagates_stored_cancellation_and_rejects_corruption","model::semantic_tests::monic_commutative_products_match_field_valuations","model::semantic_tests::deterministic_hints_scaled_inverse_involution_and_bit_prefix","model::semantic_tests::boolean_rules_include_zero_complements_and_scalars","model::semantic_tests::guarded_division_does_not_cancel_zero_case","model::semantic_tests::complete_parser_finds_known_aliases_and_rejects_corruption"],"unfiltered":true,"timeout_seconds":120}),
      json!({"kind":"link-sdk","name":"census","artifacts":artifacts,"driver":p("census.rs"),"additional_sources":[p("model.rs"),p("semantic.rs"),p("semantic-tests.rs")],"timeout_seconds":600})];
    for shape in ["1x2","2x2"]{steps.push(json!({"kind":"command","name":format!("analyze-{shape}"),"argv":[p("runs/census-sdk"),inputs[shape]["file"],p(&format!("{shape}.json"))],"timeout_seconds":600}));}
    for name in ["model.rs","semantic.rs","semantic-tests.rs","census.rs","workflow.rs","workflow-helper","design.md","inputs.json","setup-model.rs","setup-model","setup-workflow.rs","setup-workflow"]{pin(&mut pins,p(name));}
    pin(&mut pins,"/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask");
    pin(&mut pins,Path::new(OLD).join("analysis.json"));
    pin(&mut pins,Path::new(Z).join("crates/zopro-core/src/solver.rs"));
    save(p("pins.json"),&json!(pins));
    save(p("plan.json"),&json!({"version":1,"source_pins":p("pins.json"),"steps":steps}));
    println!("Prepared 5 steps, 8 structural/semantic tests, 2 cached-program semantic censuses; production source176 exact; no solves/proofs/timings");
}
fn finish(){
    assert_eq!(read(p("runs/result.json"))["status"],"complete");check_pins();
    let mut cases=BTreeMap::new();let mut opportunity=false;
    let tests=read(p("runs/model-tests-run/tests.json"));
    assert_eq!(tests["unfiltered"],true);assert_eq!(tests["passed_tests"].as_array().unwrap().len(),8);
    for shape in ["1x2","2x2"]{
        let v=read(p(&format!("{shape}.json")));let input=read(p("inputs.json"));let old=&input[shape]["prior_metadata"];
        for key in ["wires","constraints","ops","hints","stored_slots"]{if !old[key].is_null(){assert_eq!(v[key],old[key],"{shape} {key}");}}
        assert_eq!(v["complete"],true);
        assert_eq!(v["assignments"].as_u64().unwrap(),v["expanded_affine_assignments"].as_u64().unwrap()+v["expanded_constant_assignments"].as_u64().unwrap()+v["opaque_nonlinear_assignments"].as_u64().unwrap());
        for(_,q)in v["semantic_query_classes"].as_object().unwrap(){opportunity|=q["potential_terms_removed_proportional_or_zero"].as_u64().unwrap()>0;}
        cases.insert(shape,v);
    }
    save(p("analysis.json"),&json!({"complete":true,"opportunity":opportunity,"cases":cases,"tests":8,"production_source_files":176,"accepted_primary_ms":7.928660,"primary_target_ms":3.7,"goal_achieved":false,"CPU_10x_targets_achieved":0,"CPU_10x_targets_total":8,"proof_ledger":28847,"new_witness_solves":0,"new_proofs":0,"new_timing_samples":0,"production_changed":false}));
    println!("Closed semantic-CSE census; opportunity={opportunity}; all pins exact; production/ledger/timings unchanged");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("setup")=>setup(),Some("finish")=>finish(),_=>panic!("setup/finish")}}
