use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf},collections::{BTreeMap,BTreeSet}};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-borrowed-merge176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn receipt(p:&Path)->Value{
 let e=read(p.join("exit.json"));assert_eq!(e["returncode"],0,"{}",p.display());assert_eq!(e["success"],true);
 for stream in["stdout","stderr"]{assert_eq!(hash(p.join(format!("{stream}.log"))),e[format!("{stream}_sha256")].as_str().unwrap());}e
}
fn med(a:&Value)->f64{let mut v=a["samples_ns"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()).collect::<Vec<_>>();assert_eq!(v.len(),17);v.sort();v[8]as f64}
fn stat(v:&[f64])->Value{assert_eq!(v.len(),15);let mut s=v.to_vec();s.sort_by(f64::total_cmp);json!({"median":s[7],"lower":s[3],"upper":s[11],"wins":v.iter().filter(|&&x|x<1.).count(),"values":v})}
fn same(a:&Value,b:&Value){for key in["median","lower","upper"]{let x=a[key].as_f64().unwrap();let y=b[key].as_f64().unwrap();assert!((x-y).abs()<=x.abs().max(y.abs()).max(1.)*1e-12,"{key}: {x} vs {y}");}assert_eq!(a["wins"],b["wins"]);}
fn main(){
 let mut pin_occurrences=0;
 for name in["build-pins.json","consumer-pins.json","capture-pins.json"]{for(p,h)in read(r(name)).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");pin_occurrences+=1;}}
 let manifest=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));let audit=read(r("source-audit.json"));
 let mut exact=0;let mut recovered=0;
 for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){assert_eq!(hash(Path::new(Z).join(p)),h.as_str().unwrap());let target=r("source").join(p);let bytes=fs::read(&target).unwrap();if hash(&target)==h.as_str().unwrap(){exact+=1;}
  let restored=if p=="crates/zopro-core/src/lib.rs"{let s=String::from_utf8(bytes).unwrap();s.strip_suffix(audit["lib_export_suffix"].as_str().unwrap()).unwrap().as_bytes().to_vec()}
  else if p=="crates/zopro-core/src/msm/g2_borrowed.rs"{let s=String::from_utf8(bytes).unwrap();s.strip_suffix(audit["g2_borrowed_suffix"].as_str().unwrap()).unwrap().as_bytes().to_vec()}
  else if let Some(edits)=audit["edits"].get(target.display().to_string()){let mut s=String::from_utf8(bytes).unwrap();for e in edits.as_array().unwrap().iter().rev(){let new=e["new"].as_str().unwrap();assert_eq!(s.matches(new).count(),1);s=s.replace(new,e["old"].as_str().unwrap());}s.into_bytes()}
  else{bytes};assert_eq!(restored,fs::read(Path::new(Z).join(p)).unwrap(),"{p}");recovered+=1;
 }
 assert_eq!(exact,172);assert_eq!(recovered,176);
 let mut commands=0;
 for(plan,out)in[("native-plan.json","native"),("consumer-plan.json","consumers"),("capture-plan.json","runs")]{assert_eq!(read(r(format!("{out}/result.json")))["status"],"complete");let p=read(r(plan));for step in p["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let dir=r(out).join(name);receipt(&dir);let command=read(dir.join("command.json"));if step["kind"]=="command"{let mut expected=vec![json!("timeout"),json!("-k"),json!("5s"),json!(format!("{}s",step["timeout_seconds"].as_u64().unwrap()))];expected.extend(step["argv"].as_array().unwrap().iter().cloned());assert_eq!(command["argv"],json!(expected));}else{let argv=command["argv"].as_array().unwrap();assert!(argv.iter().any(|v|v=="-O"));for flag in["target-cpu=native","lto=fat","codegen-units=1"]{assert!(argv.windows(2).any(|w|w[0]=="-C"&&w[1]==flag));}}commands+=1;}}
 for(label,n)in[("control",125),("candidate",129)]{let s=fs::read_to_string(r(format!("native/full-core-{label}/stdout.log"))).unwrap();assert!(s.contains(&format!("test result: ok. {n} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out")));if label=="candidate"{for name in["record_layout_and_requested_heap_bound","ordinary_tails_exceptional_pairs_and_reuse","disabled_typed_dispatch_does_not_mutate_inputs","public_merge_chunk_boundaries_and_small_stack_pools"]{assert!(s.contains(&format!("{name} ... ok")));}}}
 let jobs=read(r("jobs.json"));let analysis=read(r("analysis.json"));let mut values=BTreeMap::<(String,String,usize),Value>::new();let mut rows=BTreeMap::new();let mut reused=0;let mut queries=0;
 for j in jobs.as_array().unwrap(){let name=j["name"].as_str().unwrap();let dir=j["reused_from"].as_str().map(PathBuf::from).unwrap_or_else(||r("runs").join(name));if j["reused_from"].is_string(){reused+=1;}receipt(&dir);let s=fs::read_to_string(dir.join("stdout.log")).unwrap();let a:Value=serde_json::from_str(s.lines().find_map(|l|l.strip_prefix("CALLER ")).unwrap()).unwrap();
  assert_eq!(a["workers"],j["case"]["workers"]);assert_eq!(a["query"],j["case"]["query"]);assert_eq!(a["rounds"],j["rounds"]);assert_eq!(a["metadata"]["route"],0);assert_eq!(a["metadata"]["retained_payload_bytes"],0);
  for k in["all_timed_outputs_checked","every_output_matches_independent_full_MSM","point_inputs_unchanged","normalization_inside_timer"]{assert_eq!(a[k],true);}assert_eq!(a["B_word_reset_inside_timer"],a["query"]=="B");
  let workers=a["workers"].as_u64().unwrap();let mut tids=BTreeSet::new();for w in a["worker_rows"].as_array().unwrap(){assert!(tids.insert(w["tid"].as_u64().unwrap()));assert_eq!(w["cpu"].as_u64().unwrap(),if workers==1{2}else{w["worker"].as_u64().unwrap()});}assert_eq!(tids.len(),workers as usize);
  queries+=j["rounds"].as_u64().unwrap()+4;
  if j["panel"]=="latency"{assert!(values.insert((j["case"]["name"].as_str().unwrap().into(),j["variant"].as_str().unwrap().into(),j["block"].as_u64().unwrap()as usize),a.clone()).is_none());}
  rows.insert(name.to_string(),a);
 }
 assert_eq!(values.len(),315);assert_eq!(reused,7);assert_eq!(rows.len(),348);
 let mut ratios=BTreeMap::new();let mut nonregression=true;let mut calibration=true;
 for c in read(r("cases.json")).as_array().unwrap(){let name=c["name"].as_str().unwrap();let times=|label:&str|(0..15).map(|b|med(&values[&(name.into(),label.into(),b)])).collect::<Vec<_>>();let off=times("OFF");let on=times("ON");let frozen=times("FROZEN");let stored=&analysis["cases"][name];for(label,t)in[("OFF",&off),("ON",&on),("FROZEN",&frozen)]{same(&stat(t),&stored[format!("{label}_ns")]);let peak=(0..15).map(|b|values[&(name.into(),label.into(),b)]["usage"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0f64,f64::max);assert_eq!(peak,stored[format!("{label}_max_RSS_MiB")].as_f64().unwrap());}
  for(label,control)in[("OFF",&off),("FROZEN",&frozen)]{let v=on.iter().zip(control).map(|(a,b)|a/b).collect::<Vec<_>>();let st=stat(&v);same(&st,&stored["ON_comparisons"][label]);if label=="OFF"{nonregression&=st["upper"].as_f64().unwrap()<=1.02;}ratios.insert((name.to_string(),label.to_string()),v);}
  let cal=stat(&off.iter().zip(&frozen).map(|(a,b)|a/b).collect::<Vec<_>>());same(&cal,&stored["OFF_over_FROZEN"]);calibration&=cal["lower"].as_f64().unwrap()>=1./1.03&&cal["upper"].as_f64().unwrap()<=1.03;
 }
 let mut gains=true;for label in["OFF","FROZEN"]{let primary=stat(&ratios[&("1x2-B-w32".into(),label.into())]);let v=(0..15).map(|b|(ratios[&("1x2-B-w32".into(),label.into())][b].ln()/2.+ratios[&("2x2-B-w32".into(),label.into())][b].ln()/2.).exp()).collect::<Vec<_>>();let aggregate=stat(&v);same(&aggregate,&analysis["two_shape_32_aggregate"][label]);if label=="OFF"{gains&=primary["upper"].as_f64().unwrap()<1.&&primary["wins"].as_u64().unwrap()>=10&&aggregate["upper"].as_f64().unwrap()<1.&&aggregate["wins"].as_u64().unwrap()>=10;}}
 // Independent profile ownership/period/CPU/loss/MMAP review. Keep symbol parsing in the capture helper.
 let mut profile_samples=0;
 for j in jobs.as_array().unwrap().iter().filter(|j|j["panel"]=="cycles"){let name=j["name"].as_str().unwrap();let a=&rows[name];let pid=a["pid"].as_u64().unwrap();let workers=a["workers"].as_u64().unwrap();let mut allowed=BTreeMap::new();for w in a["worker_rows"].as_array().unwrap(){allowed.insert(w["tid"].as_u64().unwrap(),w["cpu"].as_u64().unwrap());}
  let mut count=0;for l in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap().lines(){let s=l.trim();if s.is_empty()||s.starts_with('#'){continue;}let f=s.split_whitespace().collect::<Vec<_>>();let(p,t)=f[0].split_once('/').unwrap();assert_eq!(p.parse::<u64>().unwrap(),pid);let tid=t.parse::<u64>().unwrap();let cpu=f[1].trim_matches(['[',']']).parse::<u64>().unwrap();assert!(cpu<workers);if tid!=pid{assert_eq!(allowed[&tid],cpu);}assert_eq!(f[3].parse::<u64>().unwrap(),16000000);count+=1;}
  assert_eq!(count,analysis["profiles"][name]["samples"].as_u64().unwrap());profile_samples+=count;
  assert_eq!(read(r(format!("runs/loss-{name}/stdout.log")))["zero_lost"],true);
  let attrs=fs::read_to_string(r(format!("runs/events-{name}/stdout.log"))).unwrap();for x in["cycles:u","16000000","CPU","TIME","PERIOD"]{assert!(attrs.contains(x));}assert!(!attrs.contains("freq: 1"));
  let meta=fs::read_to_string(r(format!("runs/metadata-{name}/stdout.log"))).unwrap();assert!(meta.lines().any(|l|l.contains("PERF_RECORD_MMAP")&&l.contains(j["binary"].as_str().unwrap())&&l.contains(&format!("{pid}/{pid}"))));
 }
 assert_eq!(analysis["calibration_passed"],calibration);assert_eq!(analysis["nonregression_passed"],nonregression);assert_eq!(analysis["gain_passed"],gains);
 let diagnostic=analysis["profiles"].as_object().unwrap().values().all(|v|v["qualified"]==true)&&analysis["counters"].as_object().unwrap().values().all(|v|v["qualified"]==true);assert_eq!(analysis["diagnostics_qualified"],diagnostic);assert_eq!(analysis["complete_query_gate_passed"],nonregression&&gains&&diagnostic);
 let out=json!({"passed":true,"pin_occurrences":pin_occurrences,"successful_commands":commands,"source_originals_exact":exact,"originals_recovered":recovered,"new_tests":4,"native_test_counts":[125,129],"query_rows":rows.len(),"query_invocations":queries,"reused_preflights":reused,"independently_reconstructed_performance_gates":true,"profile_samples_with_independently_verified_ownership_period_cpu":profile_samples,"raw_PMU_counts_not_independently_reparsed":true,"accepted_production_unchanged":true,"goal_achieved":false});
 fs::write(r("review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("{out}");
}
