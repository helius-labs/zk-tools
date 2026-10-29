use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pin(pins:&mut BTreeMap<String,String>,p:impl AsRef<Path>){let p=p.as_ref();pins.insert(p.display().to_string(),hash(p));}
fn verify(pins:&BTreeMap<String,String>){for(p,h)in pins{assert_eq!(hash(p),*h,"{p}");}}
fn label(m:usize)->&'static str{["frozen","off","on"][m]}
fn contexts()->Vec<(&'static str,usize,&'static str)>{let mut v=vec![];for(k,n)in[("g2-borrowed-signed",200),("g2-borrowed-signed",352),("g1-borrowed-signed",200)]{for t in["solo","physical16","logical32"]{v.push((k,n,t));}}v}
fn env()->Value{json!({"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1","ZOPRO_CPU_G2_PREFIX_RECORDS":"1","ZOPRO_CPU_DIRECT":"1","ZOPRO_CPU_CANONICAL_BATCH":"1","ZOPRO_CPU_VECTOR_INVERSE":"1","ZOPRO_CPU_G1_SCRATCH_STORE":"2","ZOPRO_CPU_PACKED":"1"})}
fn link(){
 assert_eq!(read(r("native-on/result.json"))["status"],"complete");assert!(!r("link-plan.json").exists());
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("repair-pins.json"))).unwrap();verify(&pins);
 for f in["driver.rs","driver","prepare-files.rs","prepare-files","setup-failure.txt","build-plan.json","native-on/result.json"]{pin(&mut pins,r(f));}
 let mut steps=vec![];let mut tests=BTreeMap::new();
 for name in["frozen","off","on"]{
  let log=fs::read_to_string(r(format!("{}/core-{name}/stdout.log",if name=="on"{"native-on"}else{"native"}))).unwrap();
  let line=log.lines().find(|s|s.starts_with("test result: ok.")).unwrap();assert!(line.contains("0 failed;"));
  assert!(log.contains("borrowed_signed_g2_generator_oracle_tails_and_reuse ... ok"));
  let n:usize=line.split_whitespace().nth(3).unwrap().parse().unwrap();assert!(n>=128);tests.insert(name,n);
  pin(&mut pins,r(format!("{}/core-{name}/stdout.log",if name=="on"{"native-on"}else{"native"})));
  let a=read(r(format!("artifacts-{name}.json")));assert_eq!(a["rustflags"].as_str().unwrap().contains("--cfg zopro_gauss_positive"),name=="on");
  for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
  pin(&mut pins,r(format!("artifacts-{name}.json")));
  let mut argv=vec!["rustc".to_owned()];argv.extend(a["rustflags"].as_str().unwrap().split_whitespace().map(str::to_owned));
  argv.extend(["--edition=2021".into(),"-O".into(),r("probe.rs").display().to_string(),"-o".into(),r(format!("{name}-probe")).display().to_string(),"-L".into(),format!("dependency={}",r(format!("frozen/{name}")).display())]);
  for lib in["zopro_core","zopro_bn254","rayon","libc","serde_json","sha2"]{argv.extend(["--extern".into(),format!("{lib}={}",a["libraries"][lib].as_str().unwrap())]);}
  steps.push(json!({"kind":"command","name":format!("link-{name}"),"argv":argv,"timeout_seconds":300}));
 }
 assert_eq!(tests["frozen"],tests["off"]);assert_eq!(tests["off"],tests["on"]);
 save(r("native-tests.json"),&json!(tests));pin(&mut pins,r("native-tests.json"));
 save(r("link-pins.json"),&json!(pins));save(r("link-plan.json"),&json!({"version":1,"source_pins":r("link-pins.json"),"steps":steps}));println!("Prepared 3 coherent probe links; native tests {tests:?}");
}
fn prepare(){
 assert_eq!(read(r("linked/result.json"))["status"],"complete");assert!(!r("run-plan.json").exists());
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("link-pins.json"))).unwrap();verify(&pins);
 for f in["link-plan.json","link-pins.json","linked/result.json"]{pin(&mut pins,r(f));}pin(&mut pins,X);
 let mut code=vec![];
 for n in["frozen","off","on"]{
  let binary=r(format!("{n}-probe"));pin(&mut pins,&binary);
  let o=Command::new("nm").args(["--defined-only"]).arg(&binary).output().unwrap();assert!(o.status.success());let symbols=String::from_utf8(o.stdout).unwrap();let mut found=0;
  for(i,line)in symbols.lines().filter(|l|l.contains("g2_borrowed")&&l.contains("apply")).enumerate(){let symbol=line.split_whitespace().last().unwrap();let o=Command::new("objdump").args(["-d","--no-show-raw-insn"]).arg(format!("--disassemble={symbol}")).arg(&binary).output().unwrap();assert!(o.status.success());let text=String::from_utf8(o.stdout).unwrap();let count=text.matches("vpmadd52").count();if count==0{continue}let p=r(format!("codegen-{n}-{i}.asm.txt"));fs::write(&p,&text).unwrap();pin(&mut pins,&p);code.push(json!({"variant":n,"symbol":symbol,"ifma_sites":count,"bytes":text.len(),"path":p,"sha256":hash(&p)}));found+=1;}
  assert!(found>0,"no actual IFMA borrowedG2 symbol in {n}");
 }
 save(r("codegen.json"),&json!({"actual_borrowed_G2":code,"static_sites_not_executed_work":true}));pin(&mut pins,r("codegen.json"));
 let mut topology=BTreeMap::new();for cpu in 0..32{let p=format!("/sys/devices/system/cpu/cpu{cpu}/topology");topology.insert(cpu,json!({"core_id":fs::read_to_string(format!("{p}/core_id")).unwrap().trim(),"siblings":fs::read_to_string(format!("{p}/thread_siblings_list")).unwrap().trim()}));}
 assert_eq!(topology[&2]["siblings"],"2,18");assert_eq!((0..16).map(|n|topology[&n]["core_id"].as_str().unwrap()).collect::<BTreeSet<_>>().len(),16);
 save(r("topology.json"),&json!(topology));pin(&mut pins,r("topology.json"));
 for(name,argv)in[("host",vec!["ps","-eo","pid,comm,pcpu,pmem,etimes","--sort=-pcpu"]),("compiler",vec!["rustc","-vV"]),("machine",vec!["lscpu"])]{let o=Command::new(argv[0]).args(&argv[1..]).output().unwrap();assert!(o.status.success());fs::write(r(format!("{name}.txt")),o.stdout).unwrap();pin(&mut pins,r(format!("{name}.txt")));}
 let o=Command::new("rustup").args(["which","rustc"]).output().unwrap();assert!(o.status.success());pin(&mut pins,String::from_utf8(o.stdout).unwrap().trim());
 let mut steps=vec![];let mut jobs=vec![];let mut profiles=vec![];
 let mut add=|name:String,phase:&str,block:usize,k:&str,n:usize,t:&str,m:usize,rounds:usize|{
  let normal=vec![r(format!("{}-probe",label(m))).display().to_string(),k.into(),n.to_string(),m.to_string(),t.into(),rounds.to_string()];
  let mut environment=env();let argv=if phase=="profile"{
   let d=r("profiles").join(&name);fs::create_dir_all(&d).unwrap();assert!(Command::new("mkfifo").arg(d.join("ctl")).arg(d.join("ack")).status().unwrap().success());
   environment["ZOPRO_BENCH_PERF_CTL"]=json!(d.join("ctl"));environment["ZOPRO_BENCH_PERF_ACK"]=json!(d.join("ack"));
   let mut a:Vec<String>=vec!["sudo".into(),"-n".into(),"env".into()];for(k,v)in environment.as_object().unwrap(){a.push(format!("{k}={}",v.as_str().unwrap()));}
   a.extend(["perf".into(),"record".into(),"--no-buildid-cache".into(),"-e".into(),"cycles:u".into(),"-F".into(),"199".into(),"-m".into(),"512".into(),"-o".into(),d.join("perf.data").display().to_string(),"--delay=-1".into(),format!("--control=fifo:{},{}",d.join("ctl").display(),d.join("ack").display()),"--".into()]);a.extend(normal);a
  }else{normal};
  steps.push(json!({"kind":"command","name":name,"argv":argv,"environment":environment,"timeout_seconds":300}));
  jobs.push(json!({"name":name,"phase":phase,"block":block,"kind":k,"size":n,"topology":t,"mode":m,"rounds":rounds}));
 };
 for(k,n,t)in contexts(){for m in 0..3{add(format!("pre-{k}-{n}-{t}-m{m}"),"preflight",0,k,n,t,m,32);}}
 for b in 0..31{let mut c=contexts();c.rotate_left(b%9);if b%2!=0{c.reverse();}for(i,(k,n,t))in c.into_iter().enumerate(){let mut modes=vec![0,1,2];modes.rotate_left((b+i)%3);if b%2!=0{modes.reverse();}for m in modes{add(format!("b{b}-{k}-{n}-{t}-m{m}"),"latency",b,k,n,t,m,8192);}}}
 for t in["solo","logical32"]{for m in[1,2]{let name=format!("profile-g2-200-{t}-m{m}");add(name.clone(),"profile",0,"g2-borrowed-signed",200,t,m,262144);profiles.push(name);}}
 drop(add);assert_eq!(jobs.len(),868);
 for name in profiles{let p=r("profiles").join(&name).join("perf.data");steps.push(json!({"kind":"command","name":format!("owner-{name}"),"argv":["sudo","-n","chown","ubuntu:ubuntu",p],"timeout_seconds":120}));steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["perf","script","--header","--show-lost-events","-i",p,"-F","pid,tid,period,ip,sym"],"timeout_seconds":120}));steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":["perf","script","-D","-i",p],"timeout_seconds":120}));}
 assert_eq!(steps.len(),880);save(r("jobs.json"),&json!(jobs));pin(&mut pins,r("jobs.json"));save(r("pins.json"),&json!(pins));save(r("run-plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));println!("Prepared 27 preflights,837 latency processes,4 warm profiles,12 postcapture steps; {} pins",pins.len());
}
fn bounds(v:&[f64])->Value{assert_eq!(v.len(),31);let mut x=v.to_vec();x.sort_by(f64::total_cmp);json!({"median":x[15],"low":x[9],"high":x[21],"wins":v.iter().filter(|v|**v<1.).count(),"ratios":v})}
fn collect()->(BTreeMap<String,Value>,Value){
 assert_eq!(read(r("runs/result.json"))["status"],"complete");let jobs=read(r("jobs.json"));let mut rows=BTreeMap::new();let mut seen=BTreeMap::new();let mut profiles=vec![];
 for j in jobs.as_array().unwrap(){let name=j["name"].as_str().unwrap();let out=fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();let parsed:Vec<_>=out.lines().filter_map(|l|l.strip_prefix("RESULT ")).collect();assert_eq!(parsed.len(),1);let v:Value=serde_json::from_str(parsed[0]).unwrap();
  assert_eq!(v["internally_verified"],true);assert_eq!(v["timed_allocations"],0);for key in["kind","size","topology","mode"]{assert_eq!(v[key],j[key]);}assert_eq!(v["iterations_per_worker"],j["rounds"]);assert_eq!(v["SDK_requests"],0);
  let key=format!("{}/{}/{}/{}",j["kind"],j["size"],j["rounds"],j["topology"]);let signature=json!([v["output_sha256"],v["capacity_bytes_per_worker"],v["fixture_reference_bytes_per_worker"]]);if let Some(old)=seen.insert(key,signature.clone()){assert_eq!(old,signature);}
  if j["phase"]=="profile"{
   let p=r("profiles").join(name).join("perf.data");let text=fs::read_to_string(r(format!("runs/decode-{name}/stdout.log"))).unwrap();let loss=fs::read_to_string(r(format!("runs/loss-{name}/stdout.log"))).unwrap();assert!(!loss.contains("PERF_RECORD_LOST"));let mut symbols:BTreeMap<String,(u64,u64)>=BTreeMap::new();let mut pids=BTreeSet::new();let mut samples=0u64;
   for l in text.lines().filter(|l|!l.trim().is_empty()&&!l.trim_start().starts_with('#')){let c:Vec<_>=l.split_whitespace().collect();assert!(c.len()>=4);let(pid,tid)=c[0].split_once('/').unwrap();let pid:u32=pid.parse().unwrap();let _tid:u32=tid.parse().unwrap();let period:u64=c[1].parse().unwrap();pids.insert(pid);samples+=1;let key=c[3..].join(" ");let entry=symbols.entry(key).or_default();entry.0+=1;entry.1+=period;}
   assert!(samples>0);assert_eq!(pids.len(),1);let mut hot:Vec<_>=symbols.into_iter().map(|(s,(n,p))|json!({"symbol":s,"samples":n,"period":p})).collect();hot.sort_by_key(|v|std::cmp::Reverse(v["period"].as_u64().unwrap()));
   profiles.push(json!({"name":name,"path":p,"sha256":hash(&p),"bytes":fs::metadata(&p).unwrap().len(),"samples":samples,"lost":0,"sampled_PIDs":pids,"top_symbols":hot.into_iter().take(25).collect::<Vec<_>>(),"FIFO_gated_after_priming_before_oracle":true}));
  }rows.insert(name.into(),v);
 }assert_eq!(rows.len(),868);(rows,json!(profiles))
}
fn analyze(){
 let(rows,profiles)=collect();let mut cells=BTreeMap::new();let mut pairs=BTreeMap::new();let(mut calibration,mut nonregression,mut individual_gain)=(true,true,true);let mut capacities=BTreeMap::new();
 for(k,n,t)in contexts(){for m in 0..3{let mut metrics=BTreeMap::new();for metric in["wall_ns_per_wave","throughput_ns_per_call","mean_worker_ns_per_batch","thread_cpu_ns_per_call"]{let v:Vec<_>=(0..31).map(|b|rows[&format!("b{b}-{k}-{n}-{t}-m{m}")][metric].as_f64().unwrap()).collect();let mut x=v.clone();x.sort_by(f64::total_cmp);metrics.insert(metric,json!({"median":x[15],"values":v}));}cells.insert(format!("{k}/{n}/{t}/m{m}"),json!(metrics));let v=&rows[&format!("b0-{k}-{n}-{t}-m{m}")];capacities.insert(format!("{k}/{n}/{t}/m{m}"),json!({"scratch_bytes_per_worker":v["capacity_bytes_per_worker"],"fixture_reference_bytes_per_worker":v["fixture_reference_bytes_per_worker"]}));}
  for(a,b)in[(1,0),(2,0),(2,1)]{let v:Vec<_>=(0..31).map(|block|rows[&format!("b{block}-{k}-{n}-{t}-m{a}")]["wall_ns_per_wave"].as_f64().unwrap()/rows[&format!("b{block}-{k}-{n}-{t}-m{b}")]["wall_ns_per_wave"].as_f64().unwrap()).collect();let p=bounds(&v);let high=p["high"].as_f64().unwrap();if a==1{calibration &=p["low"].as_f64().unwrap()>=1./1.03&&high<=1.03;}else{nonregression &=high<=1.03;if k=="g2-borrowed-signed"&&t=="logical32"{individual_gain &=high<1.&&p["wins"].as_u64().unwrap()>=21;}}pairs.insert(format!("{k}/{n}/{t}/{a}-over-{b}"),p);}
 }
 let mut aggregate=BTreeMap::new();let mut aggregate_gain=true;for base in[0,1]{let v:Vec<_>=(0..31).map(|b|{let sum:f64=[200,352].into_iter().map(|n|(rows[&format!("b{b}-g2-borrowed-signed-{n}-logical32-m2")]["wall_ns_per_wave"].as_f64().unwrap()/rows[&format!("b{b}-g2-borrowed-signed-{n}-logical32-m{base}")]["wall_ns_per_wave"].as_f64().unwrap()).ln()).sum();(sum/2.).exp()}).collect();let p=bounds(&v);aggregate_gain &=p["high"].as_f64().unwrap()<1.&&p["wins"].as_u64().unwrap()>=21;aggregate.insert(format!("on-over-{}",label(base)),p);}
 let pins:BTreeMap<String,String>=serde_json::from_value(read(r("pins.json"))).unwrap();verify(&pins);let passed=calibration&&nonregression&&individual_gain&&aggregate_gain;
 let v=json!({"scope":"current175 native actual borrowed caller; not SDK latency","gates":{"calibration":calibration,"nonregression":nonregression,"individual_logical32_G2_gain":individual_gain,"aggregate_logical32_G2_gain":aggregate_gain,"caller_passed":passed},"cells":cells,"pairs":pairs,"logical32_G2_aggregate":aggregate,"capacities":capacities,"profiles":profiles,"captures":868,"preflight_processes":27,"latency_processes":837,"profile_processes":4,"workflow_steps":880,"native_core_tests":read(r("native-tests.json")),"inherited_field_qualification":read(r("inherited-field-qualification.json")),"timed_allocations_all_zero":true,"all_output_capacity_signatures_equal":true,"final_pins_verified":pins.len(),"SDK_requests":0,"independent_proof_ledger":17811,"adopted":false,"production_changed":false,"goal_active_unmet":true,"goal_original8_achieved":0});
 save(r("analysis.json"),&v);save(r("observations.json"),&json!(rows));println!("{}",v["gates"]);for(k,v)in aggregate{println!("{k} median={} bounds=[{},{}] wins={}",v["median"],v["low"],v["high"],v["wins"]);}
}
fn archive(){
 let a=read(r("analysis.json"));let report=Path::new(Z).join("docs/benchmarks/2026-10-29-positive-gauss-caller175");let d=report.join("artifacts");fs::create_dir_all(&d).unwrap();
 let mut receipts=vec![];for(directory,plan)in[("native","build-plan.json"),("native-on","repair-plan.json"),("linked","link-plan.json"),("runs","run-plan.json")]{let p=read(r(plan));for step in p["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let base=r(directory).join(name);if !base.exists(){continue}let mut streams=BTreeMap::new();for f in["stdout.log","stderr.log","command.json","exit.json"]{let path=base.join(f);if path.exists(){let bytes=fs::read(&path).unwrap();let text=String::from_utf8(bytes.clone()).unwrap();assert_eq!(text.as_bytes(),bytes);streams.insert(f,json!({"text":text,"sha256":hash(&path),"bytes":bytes.len()}));}}receipts.push(json!({"workflow":directory,"name":name,"step":step,"files":streams}));}}
 save(d.join("run-receipts.json"),&json!(receipts));
 for f in["design.md","history-audit.md","prepare-files.rs","setup.rs","bootstrap.rs","native.rs","driver.rs","caller_probe.rs","probe.rs","perf.rs","method-original.rs.txt","inherited-field-qualification.json","source-audit.json","native-tests.json","analysis.json","observations.json","jobs.json","pins.json","build-pins.json","link-pins.json","build-plan.json","link-plan.json","run-plan.json","setup-failure.txt","host.txt","machine.txt","compiler.txt","native-flags.txt","artifacts-frozen.json","artifacts-off.json","artifacts-on.json","topology.json"]{fs::copy(r(f),d.join(f)).unwrap();}
 for f in["crates/zopro-bn254/src/nonnegative_gauss.rs","crates/zopro-core/src/simd.rs","crates/zopro-core/src/msm/g2_borrowed.rs"]{let to=d.join("candidate").join(f);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(r("source").join(f),to).unwrap();}
 let readme=format!("# Nonnegative Gauss in the borrowed G2 caller\n\nCaller gate passed: **{}**. No production adoption or new SDK/proof timing.\n\nThe exact earlier qualified three-raw-product/two-reduction method is integrated into the two complex products in the current borrowed G2 reverse pass. Norms, inversion, forward records, batch geometry and scratch remain unchanged. Ordinary denominator conjugation is included. No new heap or table is added. See [design](artifacts/design.md) and [history](artifacts/history-audit.md).\n\nRyzen9950X:31 fixed rotated/reversed blocks at one thread,16 physical cores and32 logical CPUs. Frozen current, fresh OFF andON use coherent private native libraries. Each worker performs8192 measured batches after three1024-batch primes; common wall completion is the primary statistic. All837 latency processes,27 preflights and4 gated profiles complete; no exclusions or reruns. All outputs match scalar-generator oracles; all timed intervals allocate zero times and keep identical caller capacities.\n\n",a["gates"]["caller_passed"]);
 fs::write(report.join("README.md"),readme).unwrap();save(d.join("archive-summary.json"),&json!({"report":report,"receipts":receipts.len(),"SDK_requests":0,"production_changed":false,"independent_proofs":17811,"no_adoption":true}));println!("Archived {} receipts to {}",receipts.len(),report.display());
}
fn seal(){
 let report=Path::new(Z).join("docs/benchmarks/2026-10-29-positive-gauss-caller175");fn walk(p:&Path,base:&Path,out:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let p=e.unwrap().path();if p.is_dir(){walk(&p,base,out)}else{let rel=p.strip_prefix(base).unwrap().display().to_string();if rel!="SHA256SUMS"&&rel!="artifact-index.json"{out.insert(rel,hash(p));}}}}
 let mut files=BTreeMap::new();walk(&report,&report,&mut files);let sums=files.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>();fs::write(report.join("SHA256SUMS"),sums).unwrap();for(p,h)in &files{assert_eq!(hash(report.join(p)),*h);}save(report.join("artifact-index.json"),&json!({"files":files,"strict_hashes_verified":true}));println!("Sealed {} files",files.len());
}
fn main(){let args:Vec<_>=std::env::args().collect();match args[1].as_str(){"link"=>link(),"prepare"=>prepare(),"analyze"=>analyze(),"archive"=>archive(),"seal"=>seal(),_=>panic!("mode")}}
