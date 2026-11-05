use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/late-m3-query176/corrected";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn pin(p: &mut BTreeMap<String, String>, file: impl AsRef<Path>) {
    let f = file.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn step(name: &str, argv: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":600})
}
include!("sampling.rs");
fn loss(data: &str) {
    let mut c = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lost = vec![];
    let mut lines = 0;
    for l in BufReader::new(c.stdout.take().unwrap()).lines() {
        let l = l.unwrap();
        lines += 1;
        if l.contains("PERF_RECORD_LOST") {
            lost.push(l);
        }
    }
    assert!(c.wait().unwrap().success());
    println!(
        "{}",
        json!({"zero_lost":lost.is_empty(),"records":lost,"lines":lines})
    );
}
fn add(
    c: &Value,
    label: &str,
    block: usize,
    panel: &str,
    env: &Value,
    ids: &Value,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
) {
    let name = if panel == "latency" {
        format!("b{block}-{}-{label}", c["name"].as_str().unwrap())
    } else {
        format!("{panel}-{}-{label}", c["name"].as_str().unwrap())
    };
    let rounds = match panel {
        "preflight" => 1,
        "latency" => 31,
        _ => 200,
    };
    let workers = c["workers"].as_u64().unwrap();
    let affinity = if workers == 1 {
        "2".into()
    } else {
        format!("0-{}", workers - 1)
    };
    let mut environment = env.clone();
    environment["RAYON_NUM_THREADS"] = json!(workers.to_string());
    environment["ZOPRO_CPU_LATE_M3"] = json!(if label=="ON" {"1"} else {"0"});
    let mut argv = vec![];
    let mut data = Value::Null;
    if panel == "stat" || panel == "cycles" {
        environment["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
        environment["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
        argv.extend(["sudo".to_string(), "-n".into(), "env".into()]);
        for (k, v) in environment.as_object().unwrap() {
            argv.push(format!("{k}={}", v.as_str().unwrap()));
        }
        argv.push("perf".into());
        if panel == "stat" {
            let file = r("stat").join(format!("{name}.csv"));
            data = json!(file);
            argv.extend([
                "stat".into(),
                "--no-big-num".into(),
                "-x".into(),
                ";".into(),
                "-e".into(),
                "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                "-o".into(),
                file.display().to_string(),
            ]);
        } else {
            let file = r("perf").join(format!("{name}.data"));
            data = json!(file);
            argv.extend([
                "record".into(),
                "--no-buildid-cache".into(),
                "--sample-cpu".into(),
                "--timestamp".into(),
                "-P".into(),
                "-e".into(),
                "cycles:u".into(),
                "-c".into(),
                "16000000".into(),
                "-m".into(),
                "512".into(),
                "-o".into(),
                file.display().to_string(),
            ]);
        }
        argv.extend([
            "--delay=-1".into(),
            format!(
                "--control=fifo:{},{}",
                r("ctl").display(),
                r("ack").display()
            ),
            "--".into(),
        ]);
    }
    argv.extend([
        "taskset".into(),
        "-c".into(),
        affinity,
        ids[label]["binary"].as_str().unwrap().into(),
        c["query"].as_str().unwrap().into(),
        c["fixture"].as_str().unwrap().into(),
        c["oracle"].as_str().unwrap().into(),
        workers.to_string(),
        rounds.to_string(),
    ]);
    let mut s = step(&name, json!(argv));
    if panel == "latency" || panel == "preflight" {
        s["environment"] = environment;
    }
    steps.push(s);
    jobs.push(json!({"name":name,"case":c,"variant":label,"block":block,"panel":panel,"rounds":rounds,"data":data,"binary":ids[label]["binary"]}));
}
fn prepare(){
 assert!(!r("capture-plan.json").exists());assert_eq!(read(r("consumers/result.json"))["status"],"complete");
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("capture-base-pins.json"))).unwrap();for(p,h)in&pins{assert_eq!(hash(p),*h);}
 let ids=read(r("identities.json"));for label in["frozen","OFF","ON"]{assert_eq!(hash(ids[label]["binary"].as_str().unwrap()),ids[label]["binary_sha256"]);}
 let cs=read(r("cases.json"));let cases=cs.as_array().unwrap();assert_eq!(cases.len(),6);
 let mut env=read(r("native-plan.json"))["steps"][0]["environment"].clone();for k in["RUSTFLAGS","CARGO_TARGET_DIR","CARGO_BUILD_JOBS"]{env.as_object_mut().unwrap().remove(k);}
 fs::create_dir(r("perf")).unwrap();for f in["ctl","ack"]{assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success());}
 let(mut jobs,mut steps)=(vec![],vec![]);
 for c in cases {for label in["frozen","OFF","ON"]{add(c,label,0,"preflight",&env,&ids,&mut jobs,&mut steps);}}
 steps.push(step("preflight-check",json!([r("capture"),"preflight"])));
 for block in 0..15 {let mut order=(0..6).collect::<Vec<_>>();order.rotate_left(block%6);if block%2==1{order.reverse();}let mut labels=["frozen","OFF","ON"];labels.rotate_left(block%3);if block%2==1{labels.reverse();}for i in order{for label in labels{add(&cases[i],label,block,"latency",&env,&ids,&mut jobs,&mut steps);}}}
 for workers in[16,32]{let c=cases.iter().find(|v|v["shape"]=="1x2"&&v["workers"]==workers).unwrap();for label in["OFF","ON"]{add(c,label,0,"cycles",&env,&ids,&mut jobs,&mut steps);}}
 for j in jobs.iter().filter(|v|v["panel"]=="cycles"){
  let name=j["name"].as_str().unwrap();let file=j["data"].as_str().unwrap();
  for(kind,argv)in[("access",json!(["sudo","-n","chown","ubuntu:ubuntu",file])),("events",json!(["perf","evlist","-v","-i",file])),("loss",json!([r("capture"),"loss",file])),("decode",json!(["perf","script","--ns","-i",file,"-F","pid,tid,cpu,time,period,ip,sym"])),("metadata",json!(["perf","script","--header","--show-mmap-events","--show-task-events","--ns","-i",file,"-F","pid,tid,cpu,time,period,ip,sym"]))]{steps.push(step(&format!("{kind}-{name}"),argv));}
 }
 assert_eq!(jobs.len(),292);assert_eq!(steps.len(),313);save(r("jobs.json"),&json!(jobs));
 for f in["capture.rs","capture","sampling.rs","design.md","identities.json","cases.json","jobs.json","consumer-plan.json","native-plan.json"]{pin(&mut pins,r(f));}
 save(r("capture-pins.json"),&json!(pins));save(r("capture-plan.json"),&json!({"version":1,"source_pins":r("capture-pins.json"),"steps":steps}));
 println!("313steps/292callers;18preflights+check/270latency/4cyclesprofiles;15fixedblocks;frozen+OFF+ON;alllatencybeforeprofiles");
}
fn caller(j:&Value)->Value{
 let n=j["name"].as_str().unwrap();let raw=fs::read_to_string(r(format!("runs/{n}/stdout.log"))).unwrap();let lines=raw.lines().filter_map(|l|l.strip_prefix("CALLER ")).collect::<Vec<_>>();assert_eq!(lines.len(),1);let a:Value=serde_json::from_str(lines[0]).unwrap();
 for(key,expected)in[("workers",j["case"]["workers"].clone()),("input_rows",j["case"]["points"].clone()),("rounds",j["rounds"].clone()),("query",json!("B")),("new_proofs",json!(0)),("priming_passes",json!(3)),("postcheck_passes",json!(1))]{assert_eq!(a[key],expected,"{n}/{key}");}
 for k in["all_timed_outputs_checked","every_output_matches_independent_full_MSM","point_inputs_unchanged","normalization_inside_timer","B_word_reset_inside_timer"]{assert_eq!(a[k],true);}
 let candidate=j["variant"]=="ON"&&j["case"]["shape"]=="1x2";
 assert_eq!(a["metadata"]["route"],if candidate {313} else {0},"{n} actual route");
 assert_eq!(a["metadata"]["retained_payload_bytes"],if candidate {40*a["input_rows"].as_u64().unwrap()} else {0});
 assert_eq!(a["samples_ns"].as_array().unwrap().len()as u64,j["rounds"].as_u64().unwrap());
 assert_eq!(a["worker_rows"].as_array().unwrap().len(),a["workers"].as_u64().unwrap()as usize);
 for wr in a["worker_rows"].as_array().unwrap(){assert_eq!(wr["cpu"],if a["workers"]==1{json!(2)}else{wr["worker"].clone()});}
 a
}
fn preflight(){let jobs=read(r("jobs.json"));let mut n=0;for j in jobs.as_array().unwrap().iter().filter(|v|v["panel"]=="preflight"){let _=caller(j);n+=1;}assert_eq!(n,18);println!("18preflights:fullindependentoracle/affinity/exactactualroute/digitpayloadPASS");}
fn stats(v:&[f64])->Value{assert_eq!(v.len(),15);assert!(v.iter().all(|v|v.is_finite()&&*v>0.));let mut s=v.to_vec();s.sort_by(f64::total_cmp);json!({"median":s[7],"lower":s[3],"upper":s[11],"wins":v.iter().filter(|v|**v<1.).count(),"blocks":15,"values":v})}
fn finish(){
 assert!(!r("analysis.json").exists());assert_eq!(read(r("runs/result.json"))["status"],"complete");
 for(p,h)in read(r("capture-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let protocol=read(r("capture-plan.json"));for step in protocol["steps"].as_array().unwrap(){let n=step["name"].as_str().unwrap();let e=read(r(format!("runs/{n}/exit.json")));assert_eq!(e["returncode"],0);assert_eq!(e["success"],true);for stream in["stdout","stderr"]{assert_eq!(hash(r(format!("runs/{n}/{stream}.log"))),e[format!("{stream}_sha256")]);}}
 let jobs=read(r("jobs.json"));let mut values=BTreeMap::<(String,String,usize),Value>::new();let mut rows=Vec::new();let mut profiles=BTreeMap::new();let mut symbols=BTreeMap::new();let mut passes=0;
 for j in jobs.as_array().unwrap(){let n=j["name"].as_str().unwrap();let a=caller(j);passes+=j["rounds"].as_u64().unwrap()+4;
  if j["panel"]=="latency"{let key=(j["case"]["name"].as_str().unwrap().into(),j["variant"].as_str().unwrap().into(),j["block"].as_u64().unwrap()as usize);assert!(values.insert(key,a.clone()).is_none());}
  if j["panel"]=="cycles"{
   let allowed=a["worker_rows"].as_array().unwrap().iter().map(|v|v["tid"].as_u64().unwrap()as u32).chain(std::iter::once(a["pid"].as_u64().unwrap()as u32)).collect::<BTreeSet<_>>();
   let attributes=fs::read_to_string(r(format!("runs/events-{n}/stdout.log"))).unwrap();for needle in["cycles:u","16000000","CPU","TIME","PERIOD"]{assert!(attributes.contains(needle));}assert!(!attributes.contains("freq: 1"));assert_eq!(read(r(format!("runs/loss-{n}/stdout.log")))["zero_lost"],true);
   let metadata=fs::read_to_string(r(format!("runs/metadata-{n}/stdout.log"))).unwrap();assert!(metadata.contains("PERF_RECORD_FORK")&&metadata.contains("PERF_RECORD_COMM"));
   let pid=a["pid"].as_u64().unwrap()as u32;let mut sym=BTreeMap::<String,u64>::new();let mut categories=BTreeMap::<String,u64>::new();let mut tids=BTreeSet::new();let mut count=0u64;
   for(p,t,cpu,period,name)in fs::read_to_string(r(format!("runs/decode-{n}/stdout.log"))).unwrap().lines().filter_map(sample){assert_eq!(p,pid);assert!(allowed.contains(&t));assert!(cpu<j["case"]["workers"].as_u64().unwrap()as u32);assert_eq!(period,16_000_000);tids.insert(t);*categories.entry(category(&name).into()).or_default()+=1;*sym.entry(name).or_default()+=1;count+=1;}
   assert!(count>0);assert!(metadata.lines().any(|l|l.contains("PERF_RECORD_MMAP")&&l.contains(j["binary"].as_str().unwrap())&&l.contains(&format!("{pid}/{pid}"))));
   let unknown=sym.iter().filter(|(s,_)|s.starts_with("[unknown]")).map(|(_,v)|*v).sum::<u64>()as f64/count as f64;
   let mut sorted=sym.into_iter().map(|(s,v)|json!({"symbol":s,"samples":v,"share":v as f64/count as f64})).collect::<Vec<_>>();sorted.sort_by_key(|v|std::cmp::Reverse(v["samples"].as_u64().unwrap()));symbols.insert(n.to_string(),sorted);
   profiles.insert(n.to_string(),json!({"samples":count,"sampled_period":count*16_000_000,"sampled_TIDs":tids,"own_PID":pid,"zero_lost":true,"unknown_share":unknown,"period16M":true,"binary_MMAP_PID_matches":true,"affinity_bounds_pass":true,"qualified":count>=1000&&unknown<=0.05,"categories":categories,"absolute_period_coverage_not_validated":true}));
  }
  rows.push(json!({"job":j,"receipt":a}));
 }
 let(mut calibration,mut gains,mut fallback)=(true,true,true);let mut cases=BTreeMap::new();
 for c in read(r("cases.json")).as_array().unwrap(){let name=c["name"].as_str().unwrap();let get=|label:&str,metric:&str|->Vec<f64>{(0..15).map(|b|{let v=&values[&(name.into(),label.into(),b)];match metric{"ns"=>v["elapsed_ns"].as_u64().unwrap()as f64/31.,"cpu"=>(v["usage"]["user_cpu_ms"].as_f64().unwrap()+v["usage"]["system_cpu_ms"].as_f64().unwrap())/31.,_=>unreachable!()}}).collect()};
  let frozen=get("frozen","ns");let off=get("OFF","ns");let on=get("ON","ns");let cal=stats(&off.iter().zip(&frozen).map(|(a,b)|a/b).collect::<Vec<_>>());let ratio=stats(&on.iter().zip(&off).map(|(a,b)|a/b).collect::<Vec<_>>());
  let calpass=cal["lower"].as_f64().unwrap()>=0.97&&cal["upper"].as_f64().unwrap()<=1.03;calibration&=calpass;
  if c["shape"]=="1x2"&&c["workers"]!=1{gains&=ratio["upper"].as_f64().unwrap()<1.;}if c["shape"]=="2x2"{fallback&=ratio["upper"].as_f64().unwrap()<=1.03;}
  let rss=|label:&str|(0..15).map(|b|values[&(name.into(),label.into(),b)]["usage"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0f64,f64::max);
  cases.insert(name.to_string(),json!({"frozen_ns":stats(&frozen),"OFF_ns":stats(&off),"ON_ns":stats(&on),"OFF_over_frozen":cal,"calibration_passed":calpass,"ON_over_OFF":ratio,"OFF_CPU_ms":stats(&get("OFF","cpu")),"ON_CPU_ms":stats(&get("ON","cpu")),"OFF_max_process_RSS_MiB":rss("OFF"),"ON_max_process_RSS_MiB":rss("ON")}));
 }
 assert_eq!(rows.len(),292);assert_eq!(passes,10356);save(r("symbols.json"),&json!(symbols));let gate=calibration&&gains&&fallback;
 save(r("analysis.json"),&json!({"complete":true,"cases":cases,"calibration_all6_passed":calibration,"primary16_32_both_upper_below1":gains,"grouped_fallback_safety_passed":fallback,"caller_gate_passed":gate,"profiles":profiles,"rows":rows,"workflow_steps":313,"caller_processes":292,"complete_query_passes":passes,"all_outputs_independent_complete_MSM":true,"new_proofs":0,"proof_ledger":29461,"adopted":false,"production_changed":false}));println!("CompleteG2query gate{gate};calibration{calibration};gains{gains};fallback{fallback};292callers/10356checkedqueries");
}
fn main(){let a=std::env::args().collect::<Vec<_>>();match a.get(1).map(String::as_str){Some("prepare")=>prepare(),Some("preflight")=>preflight(),Some("finish")=>finish(),Some("loss")=>loss(&a[2]),_=>panic!("prepare/preflight/finish/loss")}}
