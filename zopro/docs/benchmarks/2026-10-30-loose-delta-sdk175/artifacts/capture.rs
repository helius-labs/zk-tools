use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,io::{Read,BufRead,BufReader},path::{Path,PathBuf},process::{Command,Stdio}};
const R:&str="/home/ubuntu/src/zopro-perf-work/loose-delta-sdk175";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const ACCEPTED:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
const VERIFY:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const VS:[&str;3]=["accepted","OFF","ON"];
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut file=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut bytes=[0;65536];loop{let n=file.read(&mut bytes).unwrap();if n==0{break}h.update(&bytes[..n]);}format!("{:x}",h.finalize())}
fn pin(pins:&mut BTreeMap<String,String>,p:impl AsRef<Path>){pins.insert(p.as_ref().display().to_string(),hash(p));}
fn step(name:impl Into<String>,argv:Vec<String>)->Value{json!({"kind":"command","name":name.into(),"argv":argv,"timeout_seconds":900})}
fn verify_step(j:&Value)->Value{let c=&j["case"];let dir=Path::new(j["directory"].as_str().unwrap());step(format!("verify-{}",j["name"].as_str().unwrap()),vec![VERIFY.into(),c["image"].as_str().unwrap().into(),c["image_sha256"].as_str().unwrap().into(),dir.join("0.proof").display().to_string(),dir.join("0.public").display().to_string(),c["public_sha256"].as_str().unwrap().into()])}
fn add_job(c:&Value,v:&str,block:usize,panel:&str,base:&Value,ids:&Value,jobs:&mut Vec<Value>,steps:&mut Vec<Value>){
    let name=if panel=="latency"{format!("b{block}-{}-{v}",c["name"].as_str().unwrap())}else{format!("{panel}-{}-{v}",c["name"].as_str().unwrap())};
    let rounds=match panel{"preflight"=>1,"latency"=>if c["shape"]=="1x2"{151}else{31},"stat"=>31,"cycles"=>251,_=>panic!("panel")};
    let dir=r(format!("proofs/{name}"));let mut env=base.clone();env["RAYON_NUM_THREADS"]=json!(c["workers"].as_u64().unwrap().to_string());env["ZOPRO_CPU_DEDUP"]=json!("1");
    let mut argv=Vec::new();let mut data=None;
    if panel=="stat"||panel=="cycles"{
        env["ZOPRO_BENCH_PERF_CTL"]=json!(r("ctl"));env["ZOPRO_BENCH_PERF_ACK"]=json!(r("ack"));
        argv.extend(["sudo".into(),"-n".into(),"env".into()]);for(k,value)in env.as_object().unwrap(){argv.push(format!("{k}={}",value.as_str().unwrap()));}argv.push("perf".into());
        if panel=="stat"{let file=r(format!("stat/{name}.csv"));argv.extend(["stat".into(),"--no-big-num".into(),"-x".into(),";".into(),"-e".into(),"{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),"-o".into(),file.display().to_string()]);data=Some(file);}
        else{let file=r(format!("perf/{name}.data"));argv.extend(["record".into(),"--no-buildid-cache".into(),"--sample-cpu".into(),"--timestamp".into(),"-P".into(),"-e".into(),"cycles:u".into(),"-c".into(),"16000000".into(),"-m".into(),"512".into(),"-o".into(),file.display().to_string()]);data=Some(file);}
        argv.extend(["--delay=-1".into(),format!("--control=fifo:{},{}",r("ctl").display(),r("ack").display()),"--".into()]);
    }
    argv.extend(["taskset".into(),"-c".into(),c["cpu_affinity"].as_str().unwrap().into(),ids[v]["binary"].as_str().unwrap().into(),c["image"].as_str().unwrap().into(),c["request"].as_str().unwrap().into(),rounds.to_string(),dir.display().to_string()]);
    let mut command=step(&name,argv);if panel=="latency"||panel=="preflight"{command["environment"]=env;}steps.push(command);
    jobs.push(json!({"name":name,"case":c,"variant":v,"block":block,"panel":panel,"rounds":rounds,"directory":dir,"data":data,"binary":ids[v]["binary"]}));
}
fn prepare(){
    assert!(!r("plan.json").exists());assert_eq!(read(r("native/result.json"))["status"],"complete");let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
    let mut ids=serde_json::Map::new();ids.insert("accepted".into(),json!({"binary":ACCEPTED,"sha256":hash(ACCEPTED)}));assert_eq!(hash(ACCEPTED),"20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f");pin(&mut pins,ACCEPTED);
    for(label,v)in[("control","OFF"),("candidate","ON")]{let tests=read(r(format!("native/{label}-core/tests.json")));let passed=tests["passed_tests"].as_array().unwrap();assert_eq!(passed.len(),124);assert_eq!(tests["unfiltered"],true);assert_eq!(read(r(format!("native/{label}-prover/tests.json")))["passed_tests"].as_array().unwrap().len(),8);
        let receipt=read(r(format!("native/{label}/artifacts.json")));let binary=receipt["binary"].as_str().unwrap();assert_eq!(hash(binary),receipt["binary_sha256"]);assert_eq!(receipt["libraries"].as_object().unwrap().len(),8);for(p,h)in receipt["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}pin(&mut pins,binary);ids.insert(v.into(),json!({"binary":binary,"sha256":hash(binary)}));
        let frozen=read(r(format!("frozen-{label}.json")));assert_eq!(frozen["rustflags"],fs::read_to_string(r(if label=="candidate"{"candidate-flags.txt"}else{"native-flags.txt"})).unwrap());for(p,h)in frozen["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    }

    for(suite,count)in[("core",101),("prover",8)]{let file=r(format!("native/portable-{suite}/tests.json"));let tests=read(&file);assert_eq!(tests["unfiltered"],true);assert_eq!(tests["passed_tests"].as_array().unwrap().len(),count);pin(&mut pins,file);}
    let field=read("/home/ubuntu/src/zopro-perf-work/loose-delta175/native/field-on/tests.json");assert_eq!(field["unfiltered"],true);assert_eq!(field["passed_tests"].as_array().unwrap().len(),35);
    let ids=Value::Object(ids);save(r("identities.json"),&ids);
    let prior_path="/home/ubuntu/src/zopro-perf-work/cached-drain175/proof-sha256-after.json";let prior=read(prior_path);assert_eq!(prior.as_array().unwrap().len(),22990);pin(&mut pins,prior_path);save(r("prior-proof-sha256.json"),&prior);
    let all=read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");assert_eq!(all.as_array().unwrap().len(),9);let mut physical=all[0].clone();physical["name"]=json!("1x2-w16");physical["workers"]=json!(16);physical["cpu_affinity"]=json!("0-15");let mut cases=all.as_array().unwrap().clone();cases.push(physical.clone());save(r("cases.json"),&json!(cases));
    let manifest=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));let base=&manifest["environment"];
    for dir in["proofs","stat","perf"]{fs::create_dir_all(r(dir)).unwrap();}for file in["ctl","ack"]{if r(file).exists(){use std::os::unix::fs::FileTypeExt;assert!(fs::symlink_metadata(r(file)).unwrap().file_type().is_fifo());}else{assert!(Command::new("mkfifo").arg(r(file)).status().unwrap().success());}}
    let mut jobs=Vec::new();let mut steps=Vec::new();let mut pre=all.as_array().unwrap().clone();pre.push(physical);
    for c in &pre{for v in VS{add_job(c,v,0,"preflight",base,&ids,&mut jobs,&mut steps);steps.push(verify_step(jobs.last().unwrap()));}}
    assert_eq!(steps.len(),60);
    for b in 0..31{let mut order=(0usize..10).collect::<Vec<_>>();order.rotate_left(b%10);if b%2!=0{order.reverse();}let mut variants=VS;variants.rotate_left(b%3);if b%2!=0{variants.reverse();}for i in order{for v in variants{add_job(&cases[i],v,b,"latency",base,&ids,&mut jobs,&mut steps);}}}
    for panel in["stat","cycles"]{for c in cases.iter().filter(|c|c["shape"]=="1x2"){for v in VS{add_job(c,v,0,panel,base,&ids,&mut jobs,&mut steps);}}}
    assert_eq!(jobs.len(),972);for j in jobs.iter().filter(|j|j["panel"]!="preflight"){steps.push(verify_step(j));}
    for j in jobs.iter().filter(|j|j["panel"]=="cycles"){let name=j["name"].as_str().unwrap();let data=j["data"].as_str().unwrap().to_string();steps.push(step(format!("own-{name}"),vec!["sudo".into(),"-n".into(),"chown".into(),"ubuntu:ubuntu".into(),data.clone()]));
        for(kind,argv)in[("decode",vec!["perf".into(),"script".into(),"--header".into(),"--ns".into(),"-i".into(),data.clone(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()]),("events",vec!["perf".into(),"evlist".into(),"-v".into(),"-i".into(),data.clone()]),("loss",vec![r("capture-helper").display().to_string(),"loss".into(),data.clone()]),("metadata",vec!["perf".into(),"script".into(),"--header".into(),"--show-mmap-events".into(),"--show-task-events".into(),"--ns".into(),"-i".into(),data,"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()])]{steps.push(step(format!("{kind}-{name}"),argv));}}
    assert_eq!(steps.len(),1974);assert_eq!(jobs.iter().map(|j|j["rounds"].as_u64().unwrap()+3).sum::<u64>(),55788);save(r("jobs.json"),&json!(jobs));
    for c in &pre{for key in["image","request"]{pin(&mut pins,c[key].as_str().unwrap());}assert_eq!(hash(c["image"].as_str().unwrap()),c["image_sha256"]);}
    for e in fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap(){pin(&mut pins,e.unwrap().path());}
    let raw=Command::new("sudo").args(["-n","cat","/sys/kernel/tracing/uprobe_events"]).output().unwrap();assert!(raw.status.success());assert!(!String::from_utf8_lossy(&raw.stdout).contains("candidate-sdk"));save(r("no-sdk-probes.json"),&json!({"none":true,"raw":String::from_utf8_lossy(&raw.stdout)}));
    for p in[VERIFY,X,"/usr/bin/perf"]{pin(&mut pins,p);}for p in["capture.rs","analysis.rs","sampling.rs","counter.rs","capture-helper","publish.rs","publication.rs","publish","design.md","history-audit.md","source-audit.json","final-review.rs","final-review","final-source-review.json","jobs.json","cases.json","identities.json","prior-proof-sha256.json","no-sdk-probes.json","native/result.json","native/control/artifacts.json","native/candidate/artifacts.json","frozen-control.json","frozen-candidate.json"]{pin(&mut pins,r(p));}
    for file in ["summary-generator.rs","summary-generator","prepare-capture.rs"] {pin(&mut pins,r(file));}
    save(r("pins.json"),&json!(pins));save(r("plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));println!("Prepared1974steps/55788SDK/972newGnark;31blocks all9w32+primary16; production unchanged");
}
fn loss(data:&str){let mut child=Command::new("perf").args(["script","-D","-i",data]).stdout(Stdio::piped()).spawn().unwrap();let mut records=Vec::new();let mut lines=0;for line in BufReader::new(child.stdout.take().unwrap()).lines(){let line=line.unwrap();lines+=1;if line.contains("PERF_RECORD_LOST"){records.push(line);}}assert!(child.wait().unwrap().success());println!("{}",json!({"lines":lines,"zero_lost":records.is_empty(),"records":records}));}
include!("sampling.rs");include!("counter.rs");include!("analysis.rs");
fn main(){let args=std::env::args().collect::<Vec<_>>();match args.get(1).map(String::as_str){Some("prepare")=>prepare(),Some("finish")=>finish(),Some("loss")=>loss(&args[2]),_=>panic!("mode")}}
