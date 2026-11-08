use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::{BTreeMap,BTreeSet},fs,path::Path,process::Command};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/unreduced-gcd-consumer176";
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(n:&str,v:Value){let p=format!("{ROOT}/{n}");assert!(!Path::new(&p).exists(),"would replay {n}");fs::write(p,serde_json::to_vec_pretty(&v).unwrap()).unwrap();}
fn pins(p:&str){for(p,h)in read(p).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}}
fn receipt(dir:&str,step:&Value)->Value{
 let base=format!("{ROOT}/{dir}/{}",step["name"].as_str().unwrap());let exit=read(&format!("{base}/exit.json"));assert_eq!(exit["success"],true);assert_eq!(exit["returncode"],0);
 let actual=read(&format!("{base}/command.json"));assert_eq!(&actual["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
 for stream in["stdout","stderr"]{assert_eq!(hash(&format!("{base}/{stream}.log")),exit[format!("{stream}_sha256")].as_str().unwrap());}exit
}
fn step(name:&str,argv:Vec<String>,timeout:u64)->Value{json!({"name":name,"kind":"command","argv":argv,"timeout_seconds":timeout})}
fn qualify(){
 pins(&format!("{ROOT}/native-pins.json"));let plan=read(&format!("{ROOT}/native-plan.json"));for s in plan["steps"].as_array().unwrap(){receipt("native-runs",s);}
 for field in["Fq","Fr"]{for arm in["OFF","ON"]{let text=fs::read_to_string(format!("{ROOT}/native-runs/qualify-{field}-{arm}/stdout.log")).unwrap();let v:Value=serde_json::from_str(text.trim().strip_prefix("QUALIFIED ").unwrap()).unwrap();assert_eq!(v["field"],field);assert_eq!(v["arm"],arm);assert_eq!(v["checks"],4096);}}
 let args=plan["steps"][3]["argv"].as_array().unwrap();for f in["-O","target-cpu=native","lto=fat","codegen-units=1"]{assert!(args.iter().any(|a|a==f));}assert!(!args.iter().any(|a|a.as_str().unwrap().starts_with("num_bigint=")));
 for n in["native.rs","integer-helpers.rs.inc","accepted-matrix.rs.inc"]{assert_eq!(hash(&format!("{ROOT}/{n}")),hash(&format!("/home/ubuntu/src/zopro-perf-work/unreduced-gcd-native176/{n}")));}
 for f in read(&format!("{ROOT}/fixtures.json")).as_array().unwrap(){assert_eq!(hash(f["path"].as_str().unwrap()),f["sha256"].as_str().unwrap());assert_eq!(f["independent_BigUint_Fermat"],true);}
 let mut p=read(&format!("{ROOT}/native-pins.json"));for n in["fixtures.json","fixtures/Fq.bin","fixtures/Fr.bin","native/libzopro_bn254.rlib","native/consumer","campaign.rs","campaign","source-audit.json"]{let name=format!("{ROOT}/{n}");p[&name]=hash(&name).into();}
 save("codegen-pins.json",p);let binary=format!("{ROOT}/native/consumer");save("codegen-plan.json",json!({"version":1,"source_pins":format!("{ROOT}/codegen-pins.json"),"steps":[step("consumer-nm",vec!["nm".into(),"-S".into(),"--defined-only".into(),binary.clone()],120),step("consumer-objdump",vec!["objdump".into(),"-d".into(),"--no-show-raw-insn".into(),"-Mintel".into(),binary],120),step("inspect-saved-consumer",vec![format!("{ROOT}/campaign"),"inspect".into()],120)]}));
 println!("Native8receipts/366pins/16384 actual fat-LTO output checks PASS; one nm and one full objdump prepared, no clocks");
}
fn inspect(){
 let plan=read(&format!("{ROOT}/codegen-plan.json"));for s in &plan["steps"].as_array().unwrap()[..2]{receipt("codegen-runs",s);}
 let nm=fs::read_to_string(format!("{ROOT}/codegen-runs/consumer-nm/stdout.log")).unwrap();let asm=fs::read_to_string(format!("{ROOT}/codegen-runs/consumer-objdump/stdout.log")).unwrap();
 let mut symbols=BTreeMap::<String,(u64,u64)>::new();for l in nm.lines(){let c=l.split_whitespace().collect::<Vec<_>>();if c.len()!=4{continue;}symbols.insert(c[3].into(),(u64::from_str_radix(c[0],16).unwrap(),u64::from_str_radix(c[1],16).unwrap()));}
 let mut bodies=BTreeMap::<String,Vec<String>>::new();let mut current=None;
 for l in asm.lines(){if let Some((_,name))=l.trim().split_once(" <"){if let Some(n)=name.strip_suffix(">:"){current=Some(n.to_string());continue;}}if let Some(n)=current.as_ref(){bodies.entry(n.clone()).or_default().push(l.to_string());}}
 let mut rows=Vec::new();for(name,(address,size))in &symbols{if !(name.starts_with("complete_f")||name.contains("6native")||name.contains("7inverse7inverse")){continue;}let body=bodies.get(name).unwrap();let mut mn=BTreeMap::<String,u64>::new();let mut calls=Vec::new();let mut stack=0;
  for l in body{let Some((a,b))=l.trim().split_once(':')else{continue};if a.is_empty()||!a.chars().all(|c|c.is_ascii_hexdigit()){continue;}let Some(op)=b.split_whitespace().next()else{continue};*mn.entry(op.into()).or_default()+=1;if op=="call"{calls.push(l.clone());}if b.contains("rsp")||b.contains("rbp"){stack+=1;}}
  rows.push(json!({"symbol":name,"demangled":rustc_demangle::demangle(name).to_string(),"address":address,"bytes":size,"static_instructions":mn.values().sum::<u64>(),"mnemonics":mn,"static_stack_operand_rows":stack,"calls":calls}));
 }
 for n in["complete_fq_original","complete_fq_candidate","complete_fr_original","complete_fr_candidate"]{assert!(rows.iter().any(|v|v["symbol"]==n));}
 save("codegen.json",json!({"review_passed":true,"one_full_decoder_invocation":true,"actual_consumer_sha256":hash(&format!("{ROOT}/native/consumer")),"native_field_sha256":hash(&format!("{ROOT}/native/libzopro_bn254.rlib")),"symbols":rows,"limits":"Actual optimized complete inverse entries and emitted native helpers inspected. Static rows/calls/stack operands are not dynamic work, stack bounds, spills, cycles or proof performance."}));println!("Saved actual optimized complete inverse/native helpers; static code only, no clocks");
}
fn prepare(){
 assert_eq!(read(&format!("{ROOT}/codegen.json"))["review_passed"],true);pins(&format!("{ROOT}/codegen-pins.json"));let codegen=read(&format!("{ROOT}/codegen-plan.json"));for s in codegen["steps"].as_array().unwrap(){receipt("codegen-runs",s);}
 let mut steps=Vec::new();let mut jobs=Vec::new();let contexts=[("Fq",1), ("Fq",16),("Fq",32),("Fr",1),("Fr",16),("Fr",32)];
 for block in 0..31{let mut cases=contexts.into_iter().flat_map(|(f,w)|["OFF","ON"].map(|a|(f,w,a))).collect::<Vec<_>>();cases.rotate_left(block%12);if block/12%2==1{cases.reverse();}
  for(field,w,arm)in cases{let name=format!("b{block:02}-{field}-w{w}-{arm}");let cpus=if w==1{"2".to_string()}else{format!("0-{}",w-1)};
   steps.push(step(&name,vec!["taskset".into(),"-c".into(),cpus,format!("{ROOT}/native/consumer"),"bench".into(),field.into(),format!("{ROOT}/fixtures/{field}.bin"),arm.into(),w.to_string(),"16".into()],180));jobs.push(json!({"name":name,"kind":"latency","block":block,"field":field,"workers":w,"arm":arm,"passes":16}));
  }
 }
 fs::create_dir(format!("{ROOT}/diag")).unwrap();for leaf in["ctl","ack"]{assert!(Command::new("mkfifo").arg(format!("{ROOT}/diag/{leaf}")).status().unwrap().success());}
 let mut offline=Vec::new();for kind in["stat","cycles"]{for field in["Fq","Fr"]{for arm in["OFF","ON"]{
  let name=format!("{kind}-{field}-{arm}");let passes=if kind=="stat"{64}else{256};let path=format!("{ROOT}/diag/{name}.{}",if kind=="stat"{"csv"}else{"data"});let ctl=format!("{ROOT}/diag/ctl");let ack=format!("{ROOT}/diag/ack");
  let mut argv=vec!["sudo".into(),"-n".into(),"env".into(),format!("ZOPRO_BENCH_PERF_CTL={ctl}"),format!("ZOPRO_BENCH_PERF_ACK={ack}"),"perf".into()];
  if kind=="stat"{argv.extend(["stat".into(),"--no-big-num".into(),"-x".into(),";".into(),"-e".into(),"{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),"-o".into(),path.clone()]);}
  else{argv.extend(["record".into(),"--no-buildid-cache".into(),"--sample-cpu".into(),"--timestamp".into(),"-P".into(),"-e".into(),"cycles:u".into(),"-c".into(),"16000000".into(),"-m".into(),"512".into(),"-o".into(),path.clone()]);}
  argv.extend(["--delay=-1".into(),format!("--control=fifo:{ctl},{ack}"),"--".into(),"taskset".into(),"-c".into(),"0-31".into(),format!("{ROOT}/native/consumer"),"bench".into(),field.into(),format!("{ROOT}/fixtures/{field}.bin"),arm.into(),"32".into(),passes.to_string()]);
  steps.push(step(&name,argv,600));jobs.push(json!({"name":name,"kind":kind,"field":field,"arm":arm,"workers":32,"passes":passes,"path":path}));
  if kind=="cycles"{for(prefix,args)in[("own",vec!["sudo".into(),"-n".into(),"chown".into(),"ubuntu:ubuntu".into(),path.clone()]),("decode",vec!["perf".into(),"script".into(),"--header".into(),"--ns".into(),"-i".into(),path.clone(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()]),("events",vec!["perf".into(),"evlist".into(),"-v".into(),"-i".into(),path.clone()]),("metadata",vec!["perf".into(),"script".into(),"--header".into(),"--show-mmap-events".into(),"--show-task-events".into(),"--ns".into(),"-i".into(),path.clone(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()]),("loss",vec!["/home/ubuntu/src/zopro-perf-work/window-batch-proof-capture176/campaign".into(),"loss".into(),path.clone()])]{offline.push(step(&format!("{prefix}-{name}"),args,180));}}
 }}}
 assert_eq!(steps.len(),380);steps.extend(offline);steps.push(step("analyze-completed-campaign",vec![format!("{ROOT}/campaign"),"analyze".into()],180));
 save("jobs.json",json!(jobs));save("protocol.json",json!({"blocks":31,"contexts":contexts,"arms":["OFF","ON"],"same_executable":true,"fixture_inputs_per_field":4096,"passes_latency":16,"passes_stat":64,"passes_profiles":256,"worker_stack_bytes":524288,"rank_bounds_one_based":[10,22],"selection":"all6 nonregupper<=1.02; Fq1,Fq32,6-contextequal-log aggregate upper<1,wins>=21/31; no gain floor/reroll/subset/extension","profile_period":16000000,"profile_min_samples":1000,"profile_max_unknown_percent":5,"counter_min_running_percent":99.9,"profile_only_consumers_differ":false,"production_unchanged":true,"new_proofs":0,"goal_complete":false}));
 let mut p=read(&format!("{ROOT}/codegen-pins.json"));for n in["codegen.json","jobs.json","protocol.json","design.md","campaign.rs","campaign"]{let path=format!("{ROOT}/{n}");p[&path]=hash(&path).into();}for path in["/home/ubuntu/src/zopro-perf-work/window-batch-proof-capture176/campaign","/home/ubuntu/src/zopro-perf-work/window-batch-proof-capture176/campaign.rs"]{p[path]=hash(path).into();}
 save("capture-pins.json",p);save("capture-plan.json",json!({"version":1,"source_pins":format!("{ROOT}/capture-pins.json"),"steps":steps}));println!("Prepared372 paired latency+8 gated diagnostics+20 offline+analysis, all helpers ready; no clocks launched");
}
fn estimate(v:&[f64])->Value{assert_eq!(v.len(),31);let mut s=v.to_vec();s.sort_by(f64::total_cmp);json!({"median":s[15],"lower":s[9],"upper":s[21],"wins":v.iter().filter(|x|**x<1.).count(),"blocks":31})}
fn output(name:&str)->Value{let s=fs::read_to_string(format!("{ROOT}/capture-runs/{name}/stdout.log")).unwrap();let rows=s.lines().filter_map(|l|l.strip_prefix("INVERSE ")).collect::<Vec<_>>();assert_eq!(rows.len(),1);serde_json::from_str(rows[0]).unwrap()}
fn analyze(){
 pins(&format!("{ROOT}/capture-pins.json"));let plan=read(&format!("{ROOT}/capture-plan.json"));for s in plan["steps"].as_array().unwrap().iter().filter(|s|s["name"]!="analyze-completed-campaign"){receipt("capture-runs",s);}
 let jobs=read(&format!("{ROOT}/jobs.json"));let mut rows=BTreeMap::new();for j in jobs.as_array().unwrap(){let n=j["name"].as_str().unwrap();let o=output(n);assert_eq!(o["field"],j["field"]);assert_eq!(o["arm"],j["arm"]);assert_eq!(o["workers"],j["workers"]);assert_eq!(o["passes"],j["passes"]);assert_eq!(o["all_pre_and_post_outputs_checked"],true);
  let workers=j["workers"].as_u64().unwrap() as usize;let ws=o["worker_rows"].as_array().unwrap();assert_eq!(ws.len(),workers);let mut tids=BTreeSet::new();for(i,w)in ws.iter().enumerate(){assert_eq!(w["worker"],i);assert_eq!(w["cpu"],if workers==1{2}else{i});assert!(tids.insert(w["tid"].as_u64().unwrap()));assert_eq!(w["checksum"],o["checksum"]);}rows.insert(n.to_string(),o);
 }
 let mut comparisons=Vec::new();let mut all=BTreeMap::<usize,Vec<f64>>::new();let mut selected=true;
 for field in["Fq","Fr"]{for workers in[1,16,32]{let mut ratios=Vec::new();let mut off=Vec::new();let mut on=Vec::new();for b in 0..31{let a=&rows[&format!("b{b:02}-{field}-w{workers}-OFF")];let c=&rows[&format!("b{b:02}-{field}-w{workers}-ON")];assert_eq!(a["checksum"],c["checksum"]);let av=a["wall_ns_per_inverse_per_worker"].as_f64().unwrap();let cv=c["wall_ns_per_inverse_per_worker"].as_f64().unwrap();ratios.push(cv/av);off.push(av);on.push(cv);all.entry(b).or_default().push(cv/av);}
  off.sort_by(f64::total_cmp);on.sort_by(f64::total_cmp);let e=estimate(&ratios);let nonreg=e["upper"].as_f64().unwrap()<=1.02;let required=field=="Fq" && [1,32].contains(&workers);let gain=e["upper"].as_f64().unwrap()<1.&&e["wins"].as_u64().unwrap()>=21;selected&=nonreg&&(!required||gain);
  comparisons.push(json!({"field":field,"workers":workers,"OFF_ns":off[15],"ON_ns":on[15],"ratio":e,"nonregression_passed":nonreg,"gain_required":required,"gain_passed":gain}));
 }}
 let aggregate=estimate(&all.values().map(|v|(v.iter().map(|x|x.ln()).sum::<f64>()/6.).exp()).collect::<Vec<_>>());let aggpass=aggregate["upper"].as_f64().unwrap()<1.&&aggregate["wins"].as_u64().unwrap()>=21;selected&=aggpass;
 let(mut panels,mut profiles)=(Vec::new(),Vec::new());let mut quality=true;
 for j in jobs.as_array().unwrap().iter().filter(|j|j["kind"]!="latency"){
  let n=j["name"].as_str().unwrap();let o=&rows[n];let total=o["operations_per_worker"].as_u64().unwrap()*32;
  if j["kind"]=="stat"{let csv=fs::read_to_string(j["path"].as_str().unwrap()).unwrap();let mut ev=Vec::new();for l in csv.lines(){let c=l.split(';').collect::<Vec<_>>();if c.len()<5||!["cycles:u","instructions:u","cache-misses:u","branch-misses:u"].contains(&c[2]){continue;}let value=c[0].trim().parse::<u64>().unwrap();let running=c[4].trim().parse::<f64>().unwrap();quality&=running>=99.9;ev.push(json!({"event":c[2],"count":value,"per_inverse":value as f64/total as f64,"running_percent":running}));}assert_eq!(ev.len(),4);panels.push(json!({"name":n,"events":ev}));}
  else{let text=fs::read_to_string(format!("{ROOT}/capture-runs/decode-{n}/stdout.log")).unwrap();let meta=fs::read_to_string(format!("{ROOT}/capture-runs/metadata-{n}/stdout.log")).unwrap();let events=fs::read_to_string(format!("{ROOT}/capture-runs/events-{n}/stdout.log")).unwrap();let loss=read(&format!("{ROOT}/capture-runs/loss-{n}/stdout.log"));let mut symbols=BTreeMap::<String,u64>::new();let(mut samples,mut unknown,mut ownership)=(0u64,0u64,true);
   let tids=o["worker_rows"].as_array().unwrap().iter().map(|w|(w["tid"].as_u64().unwrap(),w["cpu"].as_u64().unwrap())).collect::<BTreeMap<_,_>>();let pid=o["pid"].as_u64().unwrap();let mut worker_samples=0;
   for l in text.lines().filter(|l|!l.starts_with('#')&&!l.trim().is_empty()){let c=l.split_whitespace().collect::<Vec<_>>();if c.len()<6||!c[0].contains('/'){continue;}let(p,t)=c[0].split_once('/').unwrap();let p=p.parse::<u64>().unwrap();let t=t.parse::<u64>().unwrap();let cpu=c[1].trim_matches(['[',']']).parse::<u64>().unwrap();samples+=1;ownership&=p==pid&&c[3]=="16000000";if let Some(wcpu)=tids.get(&t){ownership&=cpu==*wcpu;worker_samples+=1;}else{ownership&=t==pid;}let sym=c[5..].join(" ");if sym.contains("unknown"){unknown+=1;}*symbols.entry(sym).or_default()+=1;}
   let percent=unknown as f64*100./samples.max(1) as f64;let passed=samples>=1000&&percent<=5.&&ownership&&worker_samples as f64/samples.max(1) as f64>=0.99&&meta.contains("PERF_RECORD_MMAP")&&meta.contains("native/consumer")&&events.contains("16000000")&&loss["zero_lost"]==true;quality&=passed;
   let mut top=symbols.into_iter().collect::<Vec<_>>();top.sort_by_key(|v|std::cmp::Reverse(v.1));profiles.push(json!({"name":n,"samples":samples,"unknown":unknown,"unknown_percent":percent,"worker_samples":worker_samples,"ownership_period_geometry":ownership,"quality_passed":passed,"loss":loss,"top_symbols":top,"data_sha256":hash(j["path"].as_str().unwrap())}));
  }
 }
 selected&=quality;save("analysis.json",json!({"comparisons":comparisons,"equal_log_aggregate":aggregate,"aggregate_gain_passed":aggpass,"diagnostic_quality_passed":quality,"selected_for_G1_G2_callers":selected,"production_adopted":false,"stat_panels":panels,"profiles":profiles,"all_raw_rows":rows,"new_proofs":0,"new_SDK_requests":0,"canonical_unchanged":35181,"goal_complete":false,"limits":"Complete scalar inverse in fixed standalone workload, not batch/SDK proof latency. Native checksums checked after timer; individual timed outputs not retained; full pre/post outputs match independent expected fixtures. Static/PMU/flat profile evidence not exclusive critical-path attribution."}));
 println!("Complete inverse analysis selected_for_G1_G2_callers={selected},diagnostic_quality={quality}; production unchanged");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("qualify")=>qualify(),Some("inspect")=>inspect(),Some("prepare")=>prepare(),Some("analyze")=>analyze(),_=>panic!("qualify|inspect|prepare|analyze")}}
