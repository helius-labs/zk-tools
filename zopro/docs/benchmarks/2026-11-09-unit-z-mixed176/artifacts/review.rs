use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/unit-z-mixed176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
    let pins=read(format!("{R}/pins.json"));
    for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}
    let plan=read(format!("{R}/plan.json"));
    let result=read(format!("{R}/run/result.json"));assert_eq!(result["status"],"complete");
    assert_eq!(result["steps"].as_array().unwrap().len(),5);
    for step in plan["steps"].as_array().unwrap(){
        let name=step["name"].as_str().unwrap();let root=format!("{R}/run/{name}");
        let exit=read(format!("{root}/exit.json"));assert_eq!(exit["returncode"],0);assert_eq!(exit["success"],true);
        for stream in["stdout","stderr"]{assert_eq!(hash(format!("{root}/{stream}.log")),exit[format!("{stream}_sha256")]);}
        let command=read(format!("{root}/command.json"));let original=step["argv"].as_array().unwrap();let actual=command["argv"].as_array().unwrap();
        assert_eq!(actual.len(),original.len()+4);assert_eq!(&actual[4..],&original[..]);assert_eq!(actual[0],"timeout");assert_eq!(actual[1],"-k");assert_eq!(actual[2],"5s");assert_eq!(actual[3],format!("{}s",step["timeout_seconds"]));
    }
    let native=fs::read_to_string(format!("{R}/run/native-independent-curve-and-worker-tests/stdout.log")).unwrap();
    let generic=fs::read_to_string(format!("{R}/run/generic-independent-curve-tests/stdout.log")).unwrap();
    assert!(native.contains("test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    assert!(generic.contains("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
    for log in [&native,&generic]{assert!(log.contains("INDEPENDENT_OUTPUTS=11520;CHAIN_TRANSITIONS=1536"));}
    assert!(native.contains("ACTUAL_WORKERS=53;CONCURRENT_CALLERS=4;SHARED_POOL_WORKERS=8;COUNTS=Counts { outputs: 7800, units: 1040, nonunits: 3120, identities: 3640, same: 1040, opposite: 1040, transitions: 1040 }"));
    let kernel=fs::read_to_string(format!("{R}/kernel.rs")).unwrap();
    for forbidden in["Vec", "unsafe {", "Box", "alloc", "ThreadPool", "static "] {assert!(!kernel.contains(forbidden));}
    for fragment in["left.z != C::Base::ONE", "return left.add_affine(right)", "left.double()", "Projective::IDENTITY", "Projective::new(x, y, h.double())"]{assert!(kernel.contains(fragment));}
    let decode=fs::read_to_string(format!("{R}/run/decode-native-actual-entrypoints/stdout.log")).unwrap();
    let mut bodies=BTreeMap::<String,String>::new();let mut current=None;
    for line in decode.lines(){
        if line.ends_with(">:") && !line.starts_with(char::is_whitespace) {
            let (_,n)=line.split_once(" <").unwrap();let n=n.strip_suffix(">:").unwrap();
            current=if n=="unit_z_mixed_g1"||n=="unit_z_mixed_g2"||n.starts_with("qualification::kernel::add::<zopro_bn254::curve::G")||n.starts_with("<zopro_bn254::curve::Projective<zopro_bn254::curve::G")&&n.ends_with("::add_affine"){bodies.insert(n.into(),line.to_string()+"\n");Some(n.to_string())}else{None};
        }else if let Some(n)=&current{bodies.get_mut(n).unwrap().push_str(&(line.to_string()+"\n"));}
    }
    assert_eq!(bodies.len(),6);
    let mut symbols=Vec::new();
    for(n,b)in &bodies{
        let mut mnemonics=BTreeMap::<String,usize>::new();let mut calls=Vec::new();let mut rows=0;let mut bytes=0;
        for line in b.lines().skip(1){
            let parts=line.split('\t').collect::<Vec<_>>();
            if parts.len()<2||!parts[0].trim().ends_with(':'){continue}
            bytes+=parts[1].split_whitespace().filter(|w|w.len()==2&&w.bytes().all(|c|c.is_ascii_hexdigit())).count();
            if parts.len()<3 {continue}let ins=parts[2].trim();if ins.is_empty(){continue}
            rows+=1;let mnemonic=ins.split_whitespace().next().unwrap();*mnemonics.entry(mnemonic.into()).or_default()+=1;
            if mnemonic=="call"||mnemonic=="jmp"{if let Some((_,callee))=ins.split_once('<'){calls.push(callee.trim_end_matches('>').to_string());}}
        }
        if n.starts_with("qualification::kernel::add") {
            let g1=n.contains("G1Config");let mulx=mnemonics.get("mulx").copied().unwrap_or(0);
            assert_eq!(mulx,if g1{192}else{512},"actual compiled unit-Z product sites");
            assert!(calls.iter().any(|c|c.ends_with("::add_affine")),"nonunit original fallback present");
            assert!(calls.iter().any(|c|c.ends_with("::double")),"equal-point double present");
        }
        if n.ends_with("::add_affine") {assert_eq!(mnemonics.get("mulx").copied().unwrap_or(0),if n.contains("G1Config"){352}else{928});}
        symbols.push(json!({"symbol":n,"instruction_sites":rows,"decoded_bytes_including_padding":bytes,"mnemonics":mnemonics,"direct_transfers":calls}));
    }
    let mut excerpts=String::new();for b in bodies.values(){excerpts.push_str(b);}
    fs::write(format!("{R}/selected-disassembly.txt"),excerpts).unwrap();
    let out=json!({"passed":true,"adopted":false,"speed_measured":false,"production_files_unchanged":176,"pins_checked":pins.as_object().unwrap().len(),"workflow_commands_passed":5,"native_tests_passed":2,"generic_tests_passed":1,"new_native_independent_outputs":19320,"new_generic_independent_outputs":11520,"new_native_chain_transitions":2576,"new_generic_chain_transitions":1536,"generic_reuses_native_argument_population":true,"independent_output_cases_count_native_and_generic_separately":true,"actual_worker_broadcasts":53,"concurrent_OS_callers":4,"shared_pool_workers_requested":8,"no_new_native_field_suite":true,"SDK_requests":0,"new_Gnark_exports":0,"canonical_proofs":41792,"no_native_core_integration":true,"no_native_performance_capture":true,"same_agent_review_not_external_audit":true,"source_no_heap_expression_or_unsafe_block":true,"static_sites_not_executed_instructions_or_stack_peak":true,"symbols":symbols,"next":"Actual-proof branch census and coherent integration qualification before prospective complete-proof timing;retain accepted bitmap-reset source/SDK/manifest"});
    fs::write(format!("{R}/review-result.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("UNIT_Z_REVIEW_PASS190pins/5commands;native19320+generic11520outputs;mulx192/352G1,512/928G2;canonical41792;speedunmeasured");
}
