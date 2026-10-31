use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::{BTreeMap,BTreeSet},fs,io::{BufRead,BufReader,Read},path::{Path,PathBuf},process::{Command,Stdio},os::unix::fs::MetadataExt};
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";
const X:&str="/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";
const ACCEPTED:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk";
const VERIFY:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const ARMS:[&str;3]=["accepted","OFF","ON"];
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut h=Sha256::new();let mut f=fs::File::open(p).unwrap();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pin(p:&mut BTreeMap<String,String>,file:impl AsRef<Path>){p.insert(file.as_ref().display().to_string(),hash(&file));}
fn step(name:impl Into<String>,argv:Vec<String>)->Value{json!({"kind":"command","name":name.into(),"argv":argv,"timeout_seconds":1200})}
fn add(c:&Value,arm:&str,block:usize,panel:&str,base:&Value,ids:&Value,jobs:&mut Vec<Value>,steps:&mut Vec<Value>){
 let name=if panel=="latency"{format!("b{block:02}-{}-{arm}",c["name"].as_str().unwrap())}else{format!("{panel}-{}-{arm}",c["name"].as_str().unwrap())};
 let rounds=match panel{"preflight"=>1,"latency"=>101,"stat"|"cycles"=>251,_=>panic!()};let dir=r(format!("proofs/{name}"));
 let mut env=base.clone();env["RAYON_NUM_THREADS"]=json!(c["workers"].as_u64().unwrap().to_string());env["ZOPRO_CPU_DEDUP"]=json!("1");
 let mut argv=vec![];let mut data=None;
 if panel=="stat"||panel=="cycles" {
  env["ZOPRO_BENCH_PERF_CTL"]=json!(r("ctl"));env["ZOPRO_BENCH_PERF_ACK"]=json!(r("ack"));
  argv.extend(["sudo".into(),"-n".into(),"env".into()]);for(k,v)in env.as_object().unwrap(){argv.push(format!("{k}={}",v.as_str().unwrap()));}argv.push("perf".into());
  let file=if panel=="stat"{r(format!("stat/{name}.csv"))}else{r(format!("perf/{name}.data"))};
  if panel=="stat" {argv.extend(["stat".into(),"--no-big-num".into(),"-x".into(),";".into(),"-e".into(),"{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),"-o".into(),file.display().to_string()]);}
  else {argv.extend(["record".into(),"--no-buildid-cache".into(),"--sample-cpu".into(),"--timestamp".into(),"-P".into(),"-e".into(),"cycles:u".into(),"-c".into(),"16000000".into(),"-m".into(),"512".into(),"-o".into(),file.display().to_string()]);}
  data=Some(file);argv.extend(["--delay=-1".into(),format!("--control=fifo:{R}/ctl,{R}/ack"),"--".into()]);
 }
 argv.extend(["taskset".into(),"-c".into(),c["cpu_affinity"].as_str().unwrap().into(),ids[arm]["binary"].as_str().unwrap().into(),c["image"].as_str().unwrap().into(),c["request"].as_str().unwrap().into(),rounds.to_string(),dir.display().to_string()]);
 let mut s=step(&name,argv);if panel=="latency"||panel=="preflight"{s["environment"]=env;}steps.push(s);
 jobs.push(json!({"name":name,"case":c,"variant":arm,"block":block,"panel":panel,"rounds":rounds,"directory":dir,"data":data,"binary":ids[arm]["binary"]}));
}
fn verify(j:&Value)->Value{let c=&j["case"];let dir=Path::new(j["directory"].as_str().unwrap());step(format!("verify-{}",j["name"].as_str().unwrap()),vec![VERIFY.into(),c["image"].as_str().unwrap().into(),c["image_sha256"].as_str().unwrap().into(),dir.join("0.proof").display().to_string(),dir.join("0.public").display().to_string(),c["public_sha256"].as_str().unwrap().into()])}
fn routes(j:&Value,_text:&str)->Value{json!({"dispatch":"compile-time native8 mixed radix","arm":j["variant"]})}
fn prepare(){
 assert!(!r("plan.json").exists());assert_eq!(read(r("build/result.json"))["status"],"complete");
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
 assert_eq!(hash(ACCEPTED),"6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661");pin(&mut pins,ACCEPTED);
 let mut ids=json!({"accepted":{"binary":ACCEPTED,"sha256":hash(ACCEPTED)}});
 for arm in["OFF","ON"]{let linked=read(r(format!("build/sdk-{arm}/artifacts.json")));let binary=linked["binary"].as_str().unwrap();assert_eq!(hash(binary),linked["binary_sha256"]);for(p,h)in linked["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}pin(&mut pins,binary);
 let cmd=read(r(format!("build/sdk-{arm}/command.json")));for flag in["-O","target-cpu=native","lto=fat","codegen-units=1"]{assert!(cmd["argv"].as_array().unwrap().iter().any(|x|x==flag));}
 let frozen=read(r(format!("frozen-{arm}.json")));let f=frozen["rustflags"].as_str().unwrap();assert_eq!(f.contains("--cfg zopro_ifma_logjumps"),arm=="ON");assert_eq!(f,fs::read_to_string(r(format!("{arm}-flags.txt"))).unwrap());ids[arm]=json!({"binary":binary,"sha256":hash(binary),"native_cfg":arm=="ON"});}
 save(r("identities.json"),&ids);
 let old=read("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/cases.json");let mut cases=vec![];
 for shape in["1x2","2x2"]{let c=old.as_array().unwrap().iter().find(|c|c["shape"]==shape&&c["workers"]==32).unwrap();for w in[16,32]{let mut c=c.clone();c["workers"]=json!(w);c["name"]=json!(format!("{shape}-w{w}"));c["cpu_affinity"]=json!(format!("0-{}",w-1));cases.push(c);}}
 save(r("cases.json"),&json!(cases));let prior="/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176/proof-sha256-after.json";let ledger=read(prior);assert_eq!(ledger.as_array().unwrap().len(),29051);save(r("prior-proof-sha256.json"),&ledger);pin(&mut pins,prior);
 let base=read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json");for name in["proofs","stat","perf"]{fs::create_dir_all(r(name)).unwrap();}for name in["ctl","ack"]{assert!(Command::new("mkfifo").arg(r(name)).status().unwrap().success());}
 let(mut jobs,mut steps)=(vec![],vec![]);
 for c in &cases{for arm in ARMS{add(c,arm,0,"preflight",&base,&ids,&mut jobs,&mut steps);steps.push(verify(jobs.last().unwrap()));}}
 steps.push(step("preflight-routes",vec![r("campaign").display().to_string(),"preflight".into()]));
 for b in 0..15{let mut order=(0..cases.len()).collect::<Vec<_>>();let len=order.len();order.rotate_left(b%len);if b%2==1{order.reverse();}let mut arms=ARMS;arms.rotate_left(b%3);if b%2==1{arms.reverse();}for i in order{for arm in arms{add(&cases[i],arm,b,"latency",&base,&ids,&mut jobs,&mut steps);}}}
 for panel in["stat","cycles"]{for c in cases.iter().filter(|c|c["shape"]=="1x2"){for arm in ARMS{add(c,arm,0,panel,&base,&ids,&mut jobs,&mut steps);}}}
 for j in jobs.iter().filter(|j|j["panel"]!="preflight"){steps.push(verify(j));}
 for j in jobs.iter().filter(|j|j["panel"]=="cycles"){
  let n=j["name"].as_str().unwrap();let file=j["data"].as_str().unwrap();steps.push(step(format!("own-{n}"),vec!["sudo".into(),"-n".into(),r("campaign").display().to_string(),"own".into(),file.into()]));
  for meta in[false,true]{let mut argv=vec!["perf".into(),"script".into(),"--header".into(),"--ns".into(),"-i".into(),file.into(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()];if meta{argv.extend(["--show-mmap-events".into(),"--show-task-events".into(),"--show-lost-events".into()]);}steps.push(step(format!("{}-{n}",if meta{"metadata"}else{"decode"}),argv));}
  steps.push(step(format!("events-{n}"),vec!["perf".into(),"evlist".into(),"-v".into(),"-i".into(),file.into()]));steps.push(step(format!("loss-{n}"),vec![r("campaign").display().to_string(),"loss".into(),file.into()]));
 }
 assert_eq!(jobs.len(),204);assert_eq!(steps.len(),439);assert_eq!(jobs.iter().map(|j|j["rounds"].as_u64().unwrap()+3).sum::<u64>(),21816);save(r("jobs.json"),&json!(jobs));
 for c in &cases{for key in["image","request","raw","key"]{pin(&mut pins,c[key].as_str().unwrap());}assert_eq!(hash(c["image"].as_str().unwrap()),c["image_sha256"]);}
 for e in fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap(){pin(&mut pins,e.unwrap().path());}
 for p in[VERIFY,X,"/usr/bin/perf","/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json"]{pin(&mut pins,p);}
 for f in["campaign.rs","campaign","finish.rs","sampling.rs","counter.rs","metadata.rs","design.md","identities.json","cases.json","jobs.json","prior-proof-sha256.json","source-pins.json","source-audit.json","source-review.json","audit-source.rs","audit-source","audit-source.log","model.json","prepare-campaign.rs"]{pin(&mut pins,r(f));}
 save(r("pins.json"),&json!(pins));save(r("plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));println!("Prepared439commands/204Gnarkexports/21816SDKrequests;4contexts3arms15blocks101rounds; no measurement performed yet");
}
fn preflight(){let js=read(r("jobs.json"));let mut rows=vec![];for j in js.as_array().unwrap().iter().filter(|j|j["panel"]=="preflight"){let n=j["name"].as_str().unwrap();let v=routes(j,&fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap());assert!(fs::read_to_string(r(format!("runs/verify-{n}/stdout.log"))).unwrap().contains("VERIFIED "));rows.push(json!({"name":n,"counts":v}));}assert_eq!(rows.len(),12);save(r("preflight-routes.json"),&json!(rows));println!("All12SDKpreflights verified and compile-time sources/flags checked before timing");}
fn own(data:&str){assert!(Path::new(data).starts_with(r("perf")));assert!(data.ends_with(".data"));let before=hash(data);let m=fs::metadata(r("campaign")).unwrap();assert!(Command::new("chown").arg(format!("{}:{}",m.uid(),m.gid())).arg(data).status().unwrap().success());assert_eq!(hash(data),before);println!("{}",json!({"sha256_before":before,"sha256_after":hash(data)}));}
fn loss(data:&str){let mut child=Command::new("perf").args(["script","-D","-i",data]).stdout(Stdio::piped()).spawn().unwrap();let mut lost=vec![];for l in BufReader::new(child.stdout.take().unwrap()).lines(){let l=l.unwrap();if l.contains("PERF_RECORD_LOST"){lost.push(l);}}assert!(child.wait().unwrap().success());println!("{}",json!({"zero_lost":lost.is_empty(),"records":lost}));}
include!("sampling.rs");include!("counter.rs");include!("metadata.rs");include!("finish.rs");
fn main(){let a=std::env::args().collect::<Vec<_>>();match a.get(1).map(String::as_str){Some("prepare")=>prepare(),Some("preflight")=>preflight(),Some("own")=>own(&a[2]),Some("loss")=>loss(&a[2]),Some("finish")=>finish(),_=>panic!("mode")}}
