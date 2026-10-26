//! Exact-container component workflow. All builds finish before this executable runs.
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,path::Path,process::{Command,Stdio}};
const PERF:&str="/usr/lib/linux-tools/6.8.0-147-generic/perf";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn state()->Value{let mut v=BTreeMap::new();for p in ["/proc/loadavg","/proc/swaps","/sys/fs/cgroup/cpu.stat","/sys/fs/cgroup/cpu.max","/sys/fs/cgroup/memory.max","/sys/fs/cgroup/memory.current","/sys/fs/cgroup/memory.peak","/sys/fs/cgroup/memory.events","/sys/fs/cgroup/memory.swap.current","/sys/fs/cgroup/memory.swap.events","/sys/fs/cgroup/memory.swap.max"]{if let Ok(s)=fs::read_to_string(p){v.insert(p,s);}}json!(v)}
fn med(mut xs:Vec<f64>)->f64{assert!(!xs.is_empty());xs.sort_by(f64::total_cmp);let n=xs.len();if n%2==1{xs[n/2]}else{(xs[n/2-1]+xs[n/2])/2.}}
fn gm(xs:&[f64])->f64{assert!(!xs.is_empty());(xs.iter().map(|x|x.ln()).sum::<f64>()/xs.len()as f64).exp()}
fn command(args:&[String],out:&Path){let f=fs::File::create(out).unwrap();assert!(Command::new(&args[0]).args(&args[1..]).stdout(Stdio::from(f.try_clone().unwrap())).stderr(Stdio::from(f)).status().unwrap().success(),"command failed; see {}",out.display());}
fn analyze(dir:&Path,phase:&str,jobs:&[Value]){
 assert_eq!(read(dir.join("workflow/result.json"))["status"],"complete");
 let mut rows=Vec::new();let mut calls=0;let mut hints:BTreeMap<String,BTreeSet<String>>=BTreeMap::new();
 for j in jobs{let log=fs::read_to_string(dir.join("workflow").join(j["name"].as_str().unwrap()).join("stdout.log")).unwrap();
  let summary:Value=serde_json::from_str(log.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();
  let samples:Vec<Value>=log.lines().filter_map(|l|l.strip_prefix("SAMPLE ")).map(|l|serde_json::from_str(l).unwrap()).collect();
  for k in ["rounds","workers","shape"]{assert_eq!(summary[k],j[k]);}
  assert_eq!(summary["priming"],2);assert_eq!(summary["reference_setup_calls"],1);assert_eq!(summary["mode"],if j["mode"]=="frozen"{json!("stream")}else{j["mode"].clone()});
  assert_eq!(samples.len(),j["rounds"].as_u64().unwrap()as usize);
  for(i,s)in samples.iter().enumerate(){assert_eq!(s["round"],i);assert!(s["ms"].as_f64().unwrap()>0.);}
  let peak=samples.iter().map(|s|s["peak_live_requested_above_inputs_bytes"].as_u64().unwrap()).max().unwrap();
  assert!(peak<=summary["forecast_requested_bytes"].as_u64().unwrap().checked_add(summary["points"].as_u64().unwrap()*32).unwrap());
  hints.entry(j["shape"].as_str().unwrap().into()).or_default().insert(summary["output_sha_hint"].as_str().unwrap().into());
  calls+=samples.len()+3;
  rows.push(json!({"job":j,"median_ms":med(samples.iter().map(|s|s["ms"].as_f64().unwrap()).collect()),"max_requested_above_inputs_bytes":peak,"summary":summary,"samples":samples}));
 }
 assert_eq!(hints.len(),2);for hs in hints.values(){assert_eq!(hs.len(),1);}
 let mut comparisons=BTreeMap::new();let mut calibration=Vec::new();let mut failures:BTreeMap<&str,Vec<String>>=BTreeMap::from([("tree",Vec::new()),("prefix",Vec::new())]);let mut profiles=BTreeMap::new();
 if phase=="measure"{
  assert_eq!(jobs.len(),230);assert_eq!(calls,4882);
  for shape in ["1x2","2x2"]{for workers in [16,32,64,96]{let mut modes=BTreeMap::new();let mut pairs=BTreeMap::new();
   for mode in ["frozen","stream","tree","prefix"]{let rs:Vec<_>=rows.iter().filter(|r|r["job"]["phase"]=="latency"&&r["job"]["shape"]==shape&&r["job"]["workers"]==workers&&r["job"]["mode"]==mode).collect();assert_eq!(rs.len(),7);
    modes.insert(mode,json!({"request_ms":med(rs.iter().map(|r|r["median_ms"].as_f64().unwrap()).collect()),"max_requested_above_inputs_bytes":rs.iter().map(|r|r["max_requested_above_inputs_bytes"].as_u64().unwrap()).max().unwrap(),"max_process_RSS_mib":rs.iter().map(|r|r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max),"forecast_bytes":rs[0]["summary"]["forecast_requested_bytes"]}));
   }
   for(a,b)in[("stream","frozen"),("tree","stream"),("tree","frozen"),("prefix","stream"),("prefix","frozen"),("prefix","tree")]{let name=format!("{a}_{b}");let mut ratios=Vec::new();
    for block in 0..7{let ms=|m:&str|rows.iter().find(|r|r["job"]["phase"]=="latency"&&r["job"]["shape"]==shape&&r["job"]["workers"]==workers&&r["job"]["mode"]==m&&r["job"]["block"]==block).unwrap()["median_ms"].as_f64().unwrap();let ratio=ms(a)/ms(b);ratios.push(ratio);if a=="stream"&&!(1./1.05..=1.05).contains(&ratio){calibration.push(format!("{shape}/w{workers}/b{block} stream/frozen {ratio}"));}}
    let ratio=gm(&ratios);let median=med(ratios.clone());let wins=ratios.iter().filter(|x|**x<1.).count();
    if a=="stream"&&!(1./1.03..=1.03).contains(&median){calibration.push(format!("{shape}/w{workers} paired stream/frozen {median}"));}
    if workers>=64&&a!="stream"&&["stream","frozen"].contains(&b){if(shape=="1x2"&&(ratio>=1.||wins<4))||(shape=="2x2"&&ratio>1.02){failures.get_mut(a).unwrap().push(format!("{shape}/w{workers} {name} ratio {ratio}, wins {wins}/7"));}}
    pairs.insert(name,json!({"paired_ratios":ratios,"paired_geomean":ratio,"paired_median":median,"wins":wins}));
   }
   comparisons.insert(format!("{shape}-w{workers}"),json!({"modes":modes,"pairs":pairs}));
  }}
  for workers in [32,96]{for mode in ["stream","tree","prefix"]{let name=format!("w{workers}-{mode}");let script=fs::read_to_string(dir.join(format!("workflow/samples-{name}/stdout.log"))).unwrap();let report=fs::read_to_string(dir.join(format!("workflow/report-{name}/stdout.log"))).unwrap();assert!(!script.contains("PERF_RECORD_LOST"));assert!(report.contains("Total Lost Samples: 0"));let mut pids=BTreeSet::new();let mut count=0;
   for l in script.lines().filter(|l|l.contains("cycles:u:")){let pieces:Vec<_>=l.split_whitespace().collect();pids.insert(pieces[1].split('/').next().unwrap().parse::<u32>().unwrap());count+=1;}assert!(count>0);assert_eq!(pids.len(),1);
   profiles.insert(name.clone(),json!({"samples":count,"sampled_pids":pids,"lost_records":0,"raw_sha256":hash(dir.join(format!("profiles/{name}/perf.data")))}));
  }}
 }else{assert_eq!(jobs.len(),8);assert_eq!(calls,32);}
 save(dir.join("analysis.json"),&json!({"status":"complete","raw_query_calls":calls,"reference_setup_calls":jobs.len(),"calibration_passed":calibration.is_empty(),"calibration_failures":calibration,"generic_gate_passed":phase=="measure"&&calibration.is_empty()&&failures["tree"].is_empty(),"prefix_gate_passed":phase=="measure"&&calibration.is_empty()&&failures["prefix"].is_empty(),"speed_failures":failures,"comparisons":comparisons,"profiles":profiles,"rows":rows,"output_hints":hints,"SDK_proofs":0,"Gnark_exports":0,"prior_proof_count":5958,"adopted":false,"goal_achieved":false}));
 save(dir.join("complete.json"),&json!({"status":"complete","raw_query_calls":calls,"processes":jobs.len(),"oracle_checks_passed":true,"SDK_proofs":0,"Gnark_exports":0,"analysis_sha256":hash(dir.join("analysis.json")),"after":state()}));
}
fn main(){let args:Vec<_>=std::env::args().collect();if args.get(1).map(String::as_str)==Some("snapshot"){save(&args[2],&state());return;}
 let phase=args.get(1).expect("phase");assert!(["preflight","measure"].contains(&phase.as_str()));let dir=Path::new("/output").join(phase);fs::create_dir(&dir).unwrap();let m=read("/gt/manifest.json");let mut pins:BTreeMap<String,String>=serde_json::from_value(m["container_pins"].clone()).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h,"container pin {p}");}pins.insert("/gt/manifest.json".into(),hash("/gt/manifest.json"));pins.insert("/gt/cases.json".into(),hash("/gt/cases.json"));
 if phase=="measure"{let receipt=read("/gt/preflight-complete.json");assert_eq!(receipt["raw_query_calls"],32);assert_eq!(receipt["oracle_checks_passed"],true);pins.insert("/gt/preflight-complete.json".into(),hash("/gt/preflight-complete.json"));command(&[PERF.into(),"stat".into(),"-e".into(),"cycles:u".into(),"--".into(),"/bin/true".into()],&dir.join("perf-availability.log"));}
 for(name,a)in[("lscpu.txt",vec![]),("topology.txt",vec!["-e=CPU,CORE,SOCKET,NODE"]) ]{let o=Command::new("lscpu").args(a).output().unwrap();assert!(o.status.success());fs::write(dir.join(name),o.stdout).unwrap();}
 save(dir.join("before.json"),&state());let mut env=m["environment"].clone();env["ZOPRO_CPU_LAGRANGE_DIR"]=json!("/output/lagrange-cache");env["ZOPRO_BASIS_KEY_DIR"]=json!("/gt-basis");let cases=read("/gt/cases.json");let mut jobs=Vec::new();let mut steps=Vec::new();
 for block in 0..if phase=="preflight"{1}else{7}{let mut cs=cases.as_array().unwrap().clone();let mut ws=if phase=="preflight"{vec![4]}else{vec![16,32,64,96]};let nw=ws.len();ws.rotate_left(block%nw);let mut modes=vec!["frozen","stream","tree","prefix"];modes.rotate_left(block%4);if block%2==1{cs.reverse();ws.reverse();modes.reverse();}
  for c in cs{for &workers in &ws{for mode in &modes{let name=format!("b{block}-{}-w{workers}-{mode}",c["shape"].as_str().unwrap());let binary=if *mode=="frozen"{"/usr/local/bin/gt-baseline"}else{"/usr/local/bin/gt-candidate"};let cli_mode=if *mode=="frozen"{"stream"}else{*mode};let rounds=if phase=="preflight"{1}else{17};let mut e=env.clone();e["RAYON_NUM_THREADS"]=json!(workers.to_string());steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",format!("0-{}",workers-1),binary,"/gt/cases.json",c["shape"],cli_mode,workers.to_string(),rounds.to_string()],"environment":e,"timeout_seconds":180}));jobs.push(json!({"phase":"latency","name":name,"shape":c["shape"],"workers":workers,"mode":mode,"rounds":rounds,"block":block}));}}}
 }
 if phase=="measure"{assert_eq!(jobs.len(),224);steps.push(json!({"kind":"command","name":"after-latency","argv":["/usr/local/bin/gt-runner","snapshot",dir.join("after-latency.json")]}));
  for workers in [32,96]{for mode in ["stream","tree","prefix"]{let name=format!("w{workers}-{mode}");let p=dir.join("profiles").join(&name);fs::create_dir_all(&p).unwrap();let ctl=p.join("ctl");let ack=p.join("ack");assert!(Command::new("mkfifo").args(["-m","600"]).arg(&ctl).arg(&ack).status().unwrap().success());let data=p.join("perf.data");let mut e=env.clone();e["RAYON_NUM_THREADS"]=json!(workers.to_string());e["ZOPRO_BENCH_PERF_CTL"]=json!(ctl);e["ZOPRO_BENCH_PERF_ACK"]=json!(ack);
   steps.push(json!({"kind":"command","name":format!("profile-{name}"),"argv":[PERF,"record","--no-buildid-cache","--delay=-1",format!("--control=fifo:{},{}",ctl.display(),ack.display()),"-e","cycles:u","-F","199","-o",data,"--","taskset","-c",format!("0-{}",workers-1),"/usr/local/bin/gt-candidate","/gt/cases.json","1x2",mode,workers.to_string(),"64"],"environment":e,"timeout_seconds":180}));jobs.push(json!({"phase":"profile","name":format!("profile-{name}"),"shape":"1x2","workers":workers,"mode":mode,"rounds":64}));
   steps.push(json!({"kind":"command","name":format!("report-{name}"),"argv":[PERF,"report","--stdio","--no-children","--sort","symbol","--percent-limit","1","-i",data],"timeout_seconds":120}));
   steps.push(json!({"kind":"command","name":format!("samples-{name}"),"argv":[PERF,"script","--show-lost-events","-i",data,"-F","comm,pid,tid,time,period,event,ip,sym"],"timeout_seconds":120}));
  }}
  steps.push(json!({"kind":"command","name":"after-profiles","argv":["/usr/local/bin/gt-runner","snapshot",dir.join("after-profiles.json")]}));
 }
 save(dir.join("pins.json"),&json!(pins));save(dir.join("jobs.json"),&json!(jobs));save(dir.join("plan.json"),&json!({"version":1,"source_pins":dir.join("pins.json"),"environment":{},"steps":steps}));
 command(&["/usr/local/bin/gt-xtask".into(),"workflow".into(),dir.join("plan.json").display().to_string(),dir.join("workflow").display().to_string()],&dir.join("workflow.log"));
 for(p,h)in &pins{assert_eq!(hash(p),*h);}analyze(&dir,phase,&jobs);println!("{phase} complete");
}
