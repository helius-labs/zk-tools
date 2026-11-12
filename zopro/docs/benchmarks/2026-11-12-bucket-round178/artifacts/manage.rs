use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/bucket-round178";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(n:&str,v:Value){fs::write(format!("{R}/{n}"),serde_json::to_vec_pretty(&v).unwrap()).unwrap()}
fn pin(paths:Vec<String>)->Value{json!(paths.into_iter().map(|p|{let h=hash(&p);(p,h)}).collect::<BTreeMap<_,_>>())}
fn check_pins(n:&str){for(p,h)in read(format!("{R}/{n}")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}}
fn command(name:&str,argv:Vec<String>,env:Value)->Value{json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":1800})}
fn capture(){
 check_pins("build-pins.json");assert_eq!(read(format!("{R}/build-run/result.json"))["status"],"complete");
 let tests=fs::read_to_string(format!("{R}/build-run/native-tests/stdout.log")).unwrap();assert!(tests.contains("5 passed; 0 failed"));
 let binary=format!("{R}/target/release/zopro-ternary-g2-caller");let mut paths=vec![binary.clone(),format!("{R}/protocol.md"),format!("{R}/manage.rs")];
 fs::create_dir_all(format!("{R}/bin")).unwrap();for copy in 0..3{let p=format!("{R}/bin/copy{copy}");fs::copy(&binary,&p).unwrap();assert_eq!(hash(&p),hash(&binary));paths.push(p);}
 let cases=read("/home/ubuntu/src/zopro-perf-work/ordinary-elision178/cases.json");save("cases.json",cases.clone());paths.push(format!("{R}/cases.json"));for c in cases.as_array().unwrap(){paths.push(c["fixture"].as_str().unwrap().into());paths.push(c["oracle"].as_str().unwrap().into());}
 let env=read("/home/ubuntu/src/zopro-perf-work/ordinary-elision178/environment.json");save("environment.json",env.clone());paths.push(format!("{R}/environment.json"));
 let mut main=vec![];let mut pre=vec![];let mut jobs=vec![];
 for block in 0..3{let mut order=(0..6usize).map(|i|(i+block)%6).collect::<Vec<_>>();if block%2==1{order.reverse();}
  for cidx in order{let c=&cases[cidx];let mut modes=(0..3).map(|i|(i+cidx+block)%3).collect::<Vec<_>>();if block%2==1{modes.reverse();}
   for mode in modes{let name=format!("b{block}-{}-m{mode}",c["name"].as_str().unwrap());let mut e=env.clone();e["ZOPRO_DIAGNOSTIC_PAIR_MODE"]=mode.to_string().into();let workers=c["workers"].as_u64().unwrap();let argv=vec!["/usr/bin/taskset".into(),"-c".into(),if workers==1{"2".into()}else{format!("0-{}",workers-1)},format!("{R}/bin/copy{block}"),"B".into(),c["fixture"].as_str().unwrap().into(),c["oracle"].as_str().unwrap().into(),workers.to_string(),"17".into()];
    main.push(command(&name,argv.clone(),e.clone()));jobs.push(json!({"name":name,"block":block,"mode":mode,"case":c["name"],"workers":workers,"shape":c["shape"]}));
    if block==0{let mut a=argv;a.last_mut().map(|x|*x="1".into());pre.push(command(&name,a,e));}
  }}
 }
 let mut denial=pre.iter().find(|s|s["name"]=="b0-1x2-B-w32-m2").unwrap().clone();denial["name"]="denied-1x2-B-w32-m2".into();denial["environment"]["ZOPRO_CPU_PREPARE_MAX_BYTES"]="0".into();pre.push(denial);
 save("jobs.json",json!(jobs));paths.push(format!("{R}/jobs.json"));save("capture-pins.json",pin(paths));
 for(n,steps)in[("preflight",pre),("main",main)]{save(&format!("{n}-plan.json"),json!({"version":1,"source_pins":format!("{R}/capture-pins.json"),"steps":steps}));}println!("Frozen19 preflight +54 main processes; three modes/backings.");
}
fn rows(kind:&str)->Vec<Value>{
 check_pins("capture-pins.json");let root=format!("{R}/{kind}-run");assert_eq!(read(format!("{root}/result.json"))["status"],"complete");let plan=read(format!("{R}/{kind}-plan.json"));let mut out=vec![];
 for step in plan["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let dir=format!("{root}/{name}");let e=read(format!("{dir}/exit.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(format!("{dir}/{stream}.log")),e[format!("{stream}_sha256")]);}let c=read(format!("{dir}/command.json"));assert_eq!(&c["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
  let text=fs::read_to_string(format!("{dir}/stdout.log")).unwrap();let mut row:Value=serde_json::from_str(text.trim().strip_prefix("CALLER ").unwrap()).unwrap();assert_eq!(row["every_output_matches_independent_full_MSM"],true);assert_eq!(row["point_inputs_unchanged"],true);let rounds=row["rounds"].as_u64().unwrap();assert_eq!(rounds,if kind=="preflight"{1}else{17});let mode=row["mode"].as_u64().unwrap();assert_eq!(mode.to_string(),step["environment"]["ZOPRO_DIAGNOSTIC_PAIR_MODE"]);let selected=if step["environment"]["ZOPRO_CPU_PREPARE_MAX_BYTES"]=="0"{0}else{mode};assert_eq!(row["selected_mode"],selected);let mut routes=[0;3];routes[selected as usize]=rounds+4;assert_eq!(row["route_status"],json!(routes));let samples=row["samples_ns"].as_array().unwrap();assert_eq!(samples.len(),rounds as usize);row["sample_mean_ns"]=json!(samples.iter().map(|x|x.as_u64().unwrap()as f64).sum::<f64>()/rounds as f64);row["name"]=name.into();out.push(row);
 }out
}
fn finish(kind:&str){
 let rows=rows(kind);save(&format!("{kind}-rows.json"),json!(rows));if kind=="preflight"{assert_eq!(rows.len(),19);println!("All19 preflights: independent MSM oracle, input immutability and admitted/denied routes pass.");return}
 assert_eq!(rows.len(),54);let jobs=read(format!("{R}/jobs.json"));let mut summary=vec![];let mut advance=true;
 for case in read(format!("{R}/cases.json")).as_array().unwrap(){let mut blocks=vec![];for block in 0..3{let get=|mode|{let job=jobs.as_array().unwrap().iter().find(|j|j["case"]==case["name"]&&j["block"]==block&&j["mode"]==mode).unwrap();rows.iter().find(|r|r["name"]==job["name"]).unwrap()};let ms=(0..3).map(|m|get(m)["sample_mean_ns"].as_f64().unwrap()/1e6).collect::<Vec<_>>();let usage=(0..3).map(|m|get(m)["usage"].clone()).collect::<Vec<_>>();let ratio=ms[2]/ms[0];if case["name"]=="1x2-B-w32"{advance&=ratio<1.;}if case["workers"]!=1{advance&=ratio<=1.03;}
  blocks.push(json!({"block":block,"ms":ms,"usage":usage,"new_vs_stream":ratio,"new_vs_staged":ms[2]/ms[1],"staged_vs_stream":ms[1]/ms[0]}));}summary.push(json!({"case":case["name"],"blocks":blocks}));}
 save("result.json",json!({"rows":summary,"advance_to_full_proof_qualification":advance,"new_proofs":0,"SDK_calls":0,"canonical_count":55028,"adopted":false,"goal_complete":false,"scope":"local complete-query directional screen","checked_queries":19*5+54*21}));println!("1229 queries checked; advance_to_full_proof_qualification={advance}");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("capture")=>capture(),Some("preflight")=>finish("preflight"),Some("finish")=>finish("main"),_=>panic!("capture/preflight/finish")}}
